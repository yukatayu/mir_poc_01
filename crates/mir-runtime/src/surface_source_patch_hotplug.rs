use std::{collections::BTreeSet, io, path::Path};

use mir_ast::{
    surface_alpha::{
        SurfaceMirParseReport, SurfaceModule, SurfacePlaceItem, SurfaceRawStmt, SurfaceStmt,
        parse_surface_mir_report, parse_surface_mir_report_path,
    },
    textual_alpha::{SourceSpan, TextualMirDiagnostic},
};
use mir_semantics::surface_to_core_elaboration::{
    SurfaceCoreIr, SurfaceToCoreElaborationReport, elaborate_surface_to_core_source,
};
use mirrorea_core::{HotPlugRequest, HotPlugVerdict};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SurfaceSourcePatchReport {
    pub surface_kind: String,
    pub accepted: bool,
    pub session_id: String,
    pub module_path: Option<String>,
    pub stage_summaries: Vec<SurfaceSourcePatchStage>,
    pub compatibility: SurfaceSourcePatchCompatibility,
    pub core_ir: SurfaceCoreIr,
    pub hotplug_request: Option<HotPlugRequest>,
    pub hotplug_verdict: Option<HotPlugVerdict>,
    pub activation_cut: Option<SurfacePatchActivationCut>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub diagnostic_codes: Vec<String>,
    pub direct_eval_performed: bool,
    pub runtime_mutation_applied: bool,
    pub source_authority: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SurfaceSourcePatchStage {
    pub stage: String,
    pub accepted: bool,
    pub diagnostic_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct SurfaceSourcePatchCompatibility {
    pub provided_surfaces: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
    pub observation_policy: String,
    pub redaction_policy: String,
    pub retention_policy: String,
    pub state_additions: Vec<SurfacePatchStateAddition>,
    pub state_migrations: Vec<String>,
    pub save_load_interaction: String,
    pub rollback_replay_cut_policy: String,
    pub checked_membership_epoch: String,
    pub checked_member_incarnations: Vec<String>,
    pub required_membership_witness_refs: Vec<String>,
    pub required_capability_witness_refs: Vec<String>,
    pub core_ir_diff: SurfacePatchCoreIrDiff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SurfacePatchStateAddition {
    pub owner_locus: String,
    pub state_name: String,
    pub keyspace_type: Option<String>,
    pub value_type: String,
    pub visible_fields: Vec<String>,
    pub initializer_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct SurfacePatchCoreIrDiff {
    pub transition_count: usize,
    pub remote_request_count: usize,
    pub message_envelope_count: usize,
    pub publication_count: usize,
    pub observation_count: usize,
    pub generated_edge_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SurfacePatchActivationCut {
    pub cut_id: String,
    pub cut_kind: String,
    pub request_ref: String,
    pub verdict_ref: String,
    pub membership_frontier_ref: String,
    pub capability_frontier_ref: String,
    pub activated_state_additions: Vec<String>,
    pub source_authority: String,
}

pub fn check_surface_source_patch_path(
    path: impl AsRef<Path>,
    session_id: &str,
) -> Result<SurfaceSourcePatchReport, io::Error> {
    let source = std::fs::read_to_string(path)?;
    Ok(check_surface_source_patch_source(&source, session_id))
}

pub fn inspect_surface_source_patch_path(
    path: impl AsRef<Path>,
    session_id: &str,
) -> Result<SurfaceSourcePatchReport, io::Error> {
    let source = std::fs::read_to_string(path)?;
    Ok(inspect_surface_source_patch_source(&source, session_id))
}

pub fn inspect_surface_source_patch_source(
    source: &str,
    session_id: &str,
) -> SurfaceSourcePatchReport {
    let parse_report = parse_surface_mir_report(source);
    build_report(source, session_id, parse_report, false)
}

pub fn check_surface_source_patch_source(
    source: &str,
    session_id: &str,
) -> SurfaceSourcePatchReport {
    let parse_report = parse_surface_mir_report(source);
    build_report(source, session_id, parse_report, true)
}

pub fn parse_surface_source_patch_path(path: impl AsRef<Path>) -> SurfaceMirParseReport {
    parse_surface_mir_report_path(path)
}

fn build_report(
    source: &str,
    session_id: &str,
    parse_report: SurfaceMirParseReport,
    activate: bool,
) -> SurfaceSourcePatchReport {
    let mut diagnostics = parse_report.diagnostics.clone();
    let mut stages = vec![stage(
        "parse",
        parse_report.accepted,
        diagnostic_codes_from(&parse_report.diagnostics),
    )];

    let module = parse_report.module.clone();
    let typecheck_diagnostics = module
        .as_ref()
        .map(typecheck_patch_module)
        .unwrap_or_default();
    diagnostics.extend(typecheck_diagnostics.clone());
    let typecheck_accepted = parse_report.accepted && typecheck_diagnostics.is_empty();
    stages.push(stage(
        "typecheck",
        typecheck_accepted,
        diagnostic_codes_from(&typecheck_diagnostics),
    ));

    let elaboration_report = if typecheck_accepted {
        Some(elaborate_surface_to_core_source(source))
    } else {
        None
    };
    let elaboration_accepted = elaboration_report
        .as_ref()
        .map(|report| report.accepted)
        .unwrap_or(false);
    let elaboration_diagnostics = elaboration_report
        .as_ref()
        .map(|report| report.diagnostics.clone())
        .unwrap_or_default();
    diagnostics.extend(elaboration_diagnostics.clone());
    stages.push(stage(
        "elaborate",
        elaboration_accepted,
        diagnostic_codes_from(&elaboration_diagnostics),
    ));

    let compatibility_diagnostics = module
        .as_ref()
        .map(compatibility_diagnostics)
        .unwrap_or_default();
    diagnostics.extend(compatibility_diagnostics.clone());

    let mut compatibility = module
        .as_ref()
        .map(|module| compatibility_from(module, session_id, elaboration_report.as_ref()))
        .unwrap_or_else(|| compatibility_without_module(session_id));
    let diagnostic_codes = diagnostic_codes_from(&diagnostics);
    let compatibility_accepted = parse_report.accepted
        && typecheck_accepted
        && elaboration_accepted
        && diagnostic_codes.is_empty();
    stages.push(stage(
        "compatibility",
        compatibility_accepted,
        diagnostic_codes_from(&compatibility_diagnostics),
    ));

    let module_path = module.as_ref().map(|module| module.module_path.clone());
    let request = module_path
        .as_ref()
        .map(|module_path| hotplug_request(session_id, module_path, &compatibility));
    let admission_accepted = compatibility_accepted
        && request
            .as_ref()
            .map(|request| {
                request.validate().is_ok()
                    && request_satisfies_required_capabilities(request, &compatibility)
            })
            .unwrap_or(false);
    stages.push(stage("admission", admission_accepted, Vec::new()));

    let verdict = request
        .as_ref()
        .map(|request| hotplug_verdict(request, admission_accepted, &diagnostic_codes));
    let accepted = admission_accepted;
    let activation_cut = if accepted && activate {
        request
            .as_ref()
            .zip(verdict.as_ref())
            .map(|(request, verdict)| activation_cut(request, verdict, &compatibility))
    } else {
        None
    };
    let runtime_mutation_applied = activation_cut.is_some();

    if !accepted && compatibility.state_additions.is_empty() {
        compatibility.state_additions = Vec::new();
    }

    SurfaceSourcePatchReport {
        surface_kind: "surface_source_patch_hotplug_report".to_string(),
        accepted,
        session_id: session_id.to_string(),
        module_path,
        stage_summaries: stages,
        compatibility,
        core_ir: elaboration_report
            .map(|report| report.core_ir)
            .unwrap_or_default(),
        hotplug_request: request,
        hotplug_verdict: verdict,
        activation_cut,
        diagnostics,
        diagnostic_codes,
        direct_eval_performed: false,
        runtime_mutation_applied,
        source_authority: ".mir".to_string(),
        final_public_api_frozen: false,
    }
}

impl SurfaceSourcePatchReport {
    pub fn stage_status(&self, stage_name: &str) -> Option<bool> {
        self.stage_summaries
            .iter()
            .find(|stage| stage.stage == stage_name)
            .map(|stage| stage.accepted)
    }

    pub fn diagnostic_codes(&self) -> Vec<String> {
        self.diagnostic_codes.clone()
    }
}

fn typecheck_patch_module(module: &SurfaceModule) -> Vec<TextualMirDiagnostic> {
    let mut diagnostics = Vec::new();
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::State(state) = item else {
                continue;
            };
            let Some(index) = &state.index else {
                continue;
            };
            if index.key_type_text != "Participant" {
                diagnostics.push(diagnostic(
                    "unsupported_patch_indexed_state_keyspace",
                    "source patch alpha indexed state additions require Participant keyspace",
                    index.span.clone(),
                ));
            }
        }
    }
    diagnostics
}

fn compatibility_diagnostics(module: &SurfaceModule) -> Vec<TextualMirDiagnostic> {
    let mut diagnostics = Vec::new();
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::When(when) = item else {
                continue;
            };
            scan_statements_for_self_grant(&when.body, &mut diagnostics);
        }
    }
    for block in &module.role_instance_blocks {
        for when in &block.whens {
            scan_statements_for_self_grant(&when.body, &mut diagnostics);
        }
    }
    diagnostics
}

fn scan_statements_for_self_grant(
    statements: &[SurfaceStmt],
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) {
    for stmt in statements {
        match stmt {
            SurfaceStmt::Grant(raw) if is_server_authority_self_grant(raw) => {
                diagnostics.push(diagnostic(
                    "patch_self_grant_server_authority_rejected",
                    "source patches cannot grant ServerAuthority to themselves",
                    raw.span.clone(),
                ));
            }
            SurfaceStmt::NestedPlaceBlock(block) => {
                scan_statements_for_self_grant(&block.body, diagnostics);
            }
            _ => {}
        }
    }
}

fn is_server_authority_self_grant(raw: &SurfaceRawStmt) -> bool {
    raw.text.contains("ServerAuthority") && raw.text.contains("self")
}

fn compatibility_from(
    module: &SurfaceModule,
    session_id: &str,
    elaboration_report: Option<&SurfaceToCoreElaborationReport>,
) -> SurfaceSourcePatchCompatibility {
    let state_additions = state_additions(module);
    let core_ir_diff = elaboration_report
        .map(|report| core_ir_diff(&report.core_ir))
        .unwrap_or_default();
    let provided_surfaces = state_additions
        .iter()
        .map(|state| format!("state:{}::{}", state.owner_locus, state.state_name))
        .collect::<Vec<_>>();
    let required_capabilities = required_capabilities(&state_additions);
    SurfaceSourcePatchCompatibility {
        provided_surfaces,
        required_capabilities,
        effect_row: vec!["state_addition".to_string(), "activation_cut".to_string()],
        failure_row: vec![
            "ParseRejected".to_string(),
            "TypecheckRejected".to_string(),
            "ElaborationRejected".to_string(),
            "CompatibilityRejected".to_string(),
            "AdmissionRejected".to_string(),
        ],
        observation_policy: "observer_safe_generated_rows_only".to_string(),
        redaction_policy: "observer_safe".to_string(),
        retention_policy: "source_patch_lifecycle_report".to_string(),
        state_additions,
        state_migrations: Vec::new(),
        save_load_interaction: "activation_cut_must_be_saved_with_patch_frontier".to_string(),
        rollback_replay_cut_policy: "no_direct_eval_replay_activation_cut_only".to_string(),
        checked_membership_epoch: format!("membership-frontier:{session_id}:epoch-0001"),
        checked_member_incarnations: vec![format!("{session_id}:incarnation-0001")],
        required_membership_witness_refs: vec![format!("{session_id}:membership-witness")],
        required_capability_witness_refs: vec![format!("{session_id}:capability-witness")],
        core_ir_diff,
    }
}

fn compatibility_without_module(session_id: &str) -> SurfaceSourcePatchCompatibility {
    SurfaceSourcePatchCompatibility {
        observation_policy: "observer_safe_generated_rows_only".to_string(),
        redaction_policy: "observer_safe".to_string(),
        retention_policy: "source_patch_lifecycle_report".to_string(),
        save_load_interaction: "activation_cut_must_be_saved_with_patch_frontier".to_string(),
        rollback_replay_cut_policy: "no_direct_eval_replay_activation_cut_only".to_string(),
        checked_membership_epoch: format!("membership-frontier:{session_id}:epoch-0001"),
        checked_member_incarnations: vec![format!("{session_id}:incarnation-0001")],
        required_membership_witness_refs: vec![format!("{session_id}:membership-witness")],
        required_capability_witness_refs: vec![format!("{session_id}:capability-witness")],
        ..SurfaceSourcePatchCompatibility::default()
    }
}

fn state_additions(module: &SurfaceModule) -> Vec<SurfacePatchStateAddition> {
    let mut additions = Vec::new();
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::State(state) = item else {
                continue;
            };
            additions.push(SurfacePatchStateAddition {
                owner_locus: block.place_ref.clone(),
                state_name: state.state_name.clone(),
                keyspace_type: state
                    .index
                    .as_ref()
                    .map(|index| index.key_type_text.clone()),
                value_type: state.value_type_text.clone(),
                visible_fields: state
                    .visible
                    .as_ref()
                    .map(|visible| visible.fields.clone())
                    .unwrap_or_default(),
                initializer_present: state.initial_value.is_some(),
            });
        }
    }
    additions
}

fn required_capabilities(state_additions: &[SurfacePatchStateAddition]) -> Vec<String> {
    let mut capabilities = BTreeSet::new();
    capabilities.insert("PatchSource".to_string());
    for state in state_additions {
        capabilities.insert(format!("AddState({})", state.owner_locus));
        if !state.visible_fields.is_empty() {
            capabilities.insert(format!("PublishVisible({})", state.owner_locus));
        }
    }
    capabilities.into_iter().collect()
}

fn core_ir_diff(core_ir: &SurfaceCoreIr) -> SurfacePatchCoreIrDiff {
    SurfacePatchCoreIrDiff {
        transition_count: core_ir.transitions.len(),
        remote_request_count: core_ir.remote_requests.len(),
        message_envelope_count: core_ir.message_envelopes.len(),
        publication_count: core_ir.publications.len(),
        observation_count: core_ir.observations.len(),
        generated_edge_count: core_ir.generated_edges.len(),
    }
}

fn hotplug_request(
    session_id: &str,
    module_path: &str,
    compatibility: &SurfaceSourcePatchCompatibility,
) -> HotPlugRequest {
    HotPlugRequest {
        request_id: format!("hotplug_request#{module_path}"),
        attachpoint_ref: session_id.to_string(),
        patch_ref: format!("SourcePatch[{module_path}]"),
        operation_kind: "source_patch".to_string(),
        requesting_principal: "surface_patch_author".to_string(),
        requesting_participant_place: "ParticipantPlace[surface_patch_author]".to_string(),
        message_envelope_ref: format!("message_envelope#{module_path}"),
        auth_evidence_ref: Some(format!("auth_evidence#{module_path}")),
        capability_refs: compatibility
            .required_capabilities
            .iter()
            .map(|capability| format!("capability#{capability}"))
            .collect(),
        witness_refs: vec![
            "membership_witness#source_patch".to_string(),
            "capability_witness#source_patch".to_string(),
        ],
        notes: vec!["source patch enters parse/typecheck/elaborate/admit pipeline".to_string()],
    }
}

fn request_satisfies_required_capabilities(
    request: &HotPlugRequest,
    compatibility: &SurfaceSourcePatchCompatibility,
) -> bool {
    compatibility
        .required_capabilities
        .iter()
        .all(|capability| {
            request
                .capability_refs
                .contains(&format!("capability#{capability}"))
        })
}

fn hotplug_verdict(
    request: &HotPlugRequest,
    accepted: bool,
    diagnostic_codes: &[String],
) -> HotPlugVerdict {
    let verdict_kind = if accepted { "accepted" } else { "rejected" };
    let compatibility_reason_refs = if accepted {
        vec!["source_patch_compatible".to_string()]
    } else if diagnostic_codes.is_empty() {
        vec!["source_patch_rejected".to_string()]
    } else {
        diagnostic_codes
            .iter()
            .map(|code| format!("diagnostic#{code}"))
            .collect()
    };
    HotPlugVerdict {
        request_ref: request.request_id.clone(),
        verdict_kind: verdict_kind.to_string(),
        compatibility_reason_refs,
        authorization_reason_refs: vec![if accepted {
            "patch_capability_admitted".to_string()
        } else {
            "patch_capability_not_activated".to_string()
        }],
        membership_freshness_reason_refs: vec![if accepted {
            "membership_frontier_checked".to_string()
        } else {
            "membership_frontier_not_activated".to_string()
        }],
        witness_reason_refs: vec![if accepted {
            "source_patch_witness_refs_checked".to_string()
        } else {
            "source_patch_witness_refs_not_activated".to_string()
        }],
        notes: vec!["source patch verdict is report-level alpha evidence".to_string()],
    }
}

fn activation_cut(
    request: &HotPlugRequest,
    _verdict: &HotPlugVerdict,
    compatibility: &SurfaceSourcePatchCompatibility,
) -> SurfacePatchActivationCut {
    SurfacePatchActivationCut {
        cut_id: format!("activation_cut#{}", request.patch_ref),
        cut_kind: "activation_cut".to_string(),
        request_ref: request.request_id.clone(),
        verdict_ref: format!("hotplug_verdict#{}", request.patch_ref),
        membership_frontier_ref: compatibility.checked_membership_epoch.clone(),
        capability_frontier_ref: compatibility
            .required_capability_witness_refs
            .first()
            .cloned()
            .unwrap_or_else(|| "capability_witness#missing".to_string()),
        activated_state_additions: compatibility
            .state_additions
            .iter()
            .map(|state| format!("{}::{}", state.owner_locus, state.state_name))
            .collect(),
        source_authority: ".mir".to_string(),
    }
}

fn stage(stage: &str, accepted: bool, diagnostic_codes: Vec<String>) -> SurfaceSourcePatchStage {
    SurfaceSourcePatchStage {
        stage: stage.to_string(),
        accepted,
        diagnostic_codes,
    }
}

fn diagnostic_codes_from(diagnostics: &[TextualMirDiagnostic]) -> Vec<String> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.clone())
        .collect()
}

fn diagnostic(code: &str, message: &str, span: SourceSpan) -> TextualMirDiagnostic {
    TextualMirDiagnostic {
        code: code.to_string(),
        message: message.to_string(),
        span,
    }
}
