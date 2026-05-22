use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use mir_ast::textual_alpha::TextualMirDiagnostic;
use serde::{Deserialize, Serialize};

use super::{
    checker::analyze_textual_mir_program_path,
    typed_ir::{FullSystemV1Obligation, TypedBindValue, TypedMirModule, TypedStmt, TypedType},
};

const PROJECTION_REQUEST_SCHEMA_VERSION: &str = "full-system-v1-projection-request-v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionTargetIr {
    pub target_id: String,
    pub role: String,
    pub place_refs: Vec<String>,
    pub entry_transitions: Vec<String>,
    pub observation_policy: String,
    pub redaction_policy: String,
    pub retention_policy: String,
    pub provider_policy: String,
    pub save_load_authority: bool,
    pub prediction_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionBoundaryIr {
    pub boundary_ref: String,
    pub boundary_kind: String,
    pub effect_names: Vec<String>,
    pub failure_row: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub required_witnesses: Vec<String>,
    pub from_target: String,
    pub to_target: String,
    pub authority: String,
    pub packet_schema_ref: Option<String>,
    pub ffi_schema_ref: Option<String>,
    pub rollback_cut_compatible: bool,
    pub replay_compatible: bool,
    pub save_load_obligation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionSchemaField {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionBoundarySchema {
    pub schema_ref: String,
    pub schema_kind: String,
    pub boundary_ref: String,
    pub from_target: String,
    pub to_target: String,
    pub effect_names: Vec<String>,
    pub request_fields: Vec<ProjectionSchemaField>,
    pub response_fields: Vec<ProjectionSchemaField>,
    pub failure_row: Vec<String>,
    pub capability_row: Vec<String>,
    pub required_witnesses: Vec<String>,
    pub authority_policy: String,
    pub from_provider_policy: String,
    pub to_provider_policy: String,
    pub from_observation_policy: String,
    pub to_observation_policy: String,
    pub from_redaction_policy: String,
    pub to_redaction_policy: String,
    pub from_retention_policy: String,
    pub to_retention_policy: String,
    pub rollback_cut_compatible: bool,
    pub replay_compatible: bool,
    pub save_load_obligation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionIr {
    pub projection_id: String,
    pub source_module_refs: Vec<String>,
    pub targets: Vec<ProjectionTargetIr>,
    pub boundaries: Vec<ProjectionBoundaryIr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionTargetManifest {
    pub target_id: String,
    pub role: String,
    pub place_refs: Vec<String>,
    pub entry_transitions: Vec<String>,
    pub boundary_refs: Vec<String>,
    pub effect_names: Vec<String>,
    pub failure_row: Vec<String>,
    pub capability_row: Vec<String>,
    pub witness_requirements: Vec<String>,
    pub packet_schema_refs: Vec<String>,
    pub ffi_schema_refs: Vec<String>,
    pub observation_policy: String,
    pub redaction_policy: String,
    pub retention_policy: String,
    pub provider_policy: String,
    pub save_load_authority: bool,
    pub prediction_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionPreservationReport {
    pub source_refs: Vec<String>,
    pub typed_ir_refs: Vec<String>,
    pub projection_ir_refs: Vec<String>,
    pub target_manifest_refs: Vec<String>,
    pub packet_schema_refs: Vec<String>,
    pub ffi_schema_refs: Vec<String>,
    pub checked_effect_rows: Vec<String>,
    pub checked_failure_rows: Vec<String>,
    pub checked_capability_rows: Vec<String>,
    pub checked_authority_rows: Vec<String>,
    pub checked_observation_rows: Vec<String>,
    pub checked_provider_policy_rows: Vec<String>,
    pub checked_rollback_replay_cut_rows: Vec<String>,
    pub rejected_rows: Vec<String>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ProjectionReport {
    pub accepted: bool,
    pub projection_id: String,
    pub source_path: String,
    pub request_path: String,
    pub projection_ir: ProjectionIr,
    pub target_manifests: Vec<ProjectionTargetManifest>,
    pub packet_schemas: Vec<ProjectionBoundarySchema>,
    pub ffi_schemas: Vec<ProjectionBoundarySchema>,
    pub preservation_report: ProjectionPreservationReport,
    pub diagnostics: Vec<ProjectionDiagnostic>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectionRequest {
    schema_version: String,
    projection_id: String,
    #[serde(default)]
    targets: Vec<ProjectionTargetRequest>,
    #[serde(default)]
    boundaries: Vec<ProjectionBoundaryRequest>,
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectionTargetRequest {
    target_id: String,
    role: String,
    #[serde(default)]
    place_refs: Vec<String>,
    #[serde(default)]
    entry_transitions: Vec<String>,
    observation_policy: String,
    redaction_policy: String,
    retention_policy: String,
    provider_policy: String,
    save_load_authority: bool,
    prediction_allowed: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectionBoundaryRequest {
    boundary_ref: String,
    boundary_kind: String,
    #[serde(default)]
    effect_names: Vec<String>,
    from_target: String,
    to_target: String,
    authority: String,
    #[serde(default)]
    required_witnesses: Vec<String>,
    packet_schema_ref: Option<String>,
    ffi_schema_ref: Option<String>,
    rollback_cut_compatible: bool,
    replay_compatible: bool,
    save_load_obligation: String,
}

#[derive(Debug, Clone)]
struct SourceBoundarySummary {
    effect_names: BTreeSet<String>,
    failure_row: BTreeSet<String>,
    required_capabilities: BTreeSet<String>,
    transition_names: BTreeSet<String>,
    place_refs: BTreeSet<String>,
    effect_summaries: BTreeMap<String, SourceEffectSummary>,
}

#[derive(Debug, Clone)]
struct SourceTransitionSummary {
    place_ref: String,
}

#[derive(Debug, Clone)]
struct SourceEffectSummary {
    failure_row: BTreeSet<String>,
    required_capabilities: BTreeSet<String>,
    request_fields: Vec<ProjectionSchemaField>,
    response_fields: Vec<ProjectionSchemaField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryPayloadShape {
    request_fields: Vec<ProjectionSchemaField>,
    response_fields: Vec<ProjectionSchemaField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoundarySchemaMismatch {
    PayloadShape,
    EffectContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundarySchemaContract {
    request_fields: Vec<ProjectionSchemaField>,
    response_fields: Vec<ProjectionSchemaField>,
    failure_row: Vec<String>,
    capability_row: Vec<String>,
}

pub fn project_textual_mir_module_path(
    source_path: impl AsRef<Path>,
    request_path: impl AsRef<Path>,
) -> FullSystemV1ProjectionReport {
    let source_path = source_path.as_ref();
    let request_path = request_path.as_ref();
    let source_path_text = source_path.display().to_string();
    let request_path_text = request_path.display().to_string();

    let request = match load_request(request_path) {
        Ok(request) => request,
        Err(diagnostic) => {
            return rejected_report(
                source_path_text,
                request_path_text,
                request_projection_id(request_path).to_string(),
                vec![diagnostic],
                Vec::new(),
                Vec::new(),
                Vec::new(),
                empty_projection_ir(request_projection_id(request_path)),
                empty_preservation_report(),
            );
        }
    };

    let analysis = analyze_textual_mir_program_path(source_path);
    if !analysis.accepted {
        let diagnostics = analysis
            .diagnostics
            .iter()
            .map(projection_diagnostic_from_textual)
            .collect::<Vec<_>>();
        let projection_id = request.projection_id.clone();
        return rejected_report(
            source_path_text,
            request_path_text,
            projection_id.clone(),
            diagnostics,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_projection_ir(&projection_id),
            projection_preservation_report(
                &projection_id,
                &analysis.modules,
                &request,
                Vec::new(),
                Vec::new(),
            ),
        );
    }

    let projection_id = request.projection_id.clone();
    let mut diagnostics = Vec::new();
    let mut rejected_rows = Vec::new();
    let source_boundaries = collect_source_boundaries(&analysis.modules);
    let source_transitions = collect_source_transitions(&analysis.modules);
    let source_place_refs = source_transitions
        .values()
        .map(|transition| transition.place_ref.clone())
        .collect::<BTreeSet<_>>();
    let source_transition_names = source_transitions.keys().cloned().collect::<BTreeSet<_>>();

    validate_request_schema(&request, &mut diagnostics);
    let (target_roles, place_to_target) = validate_targets(
        &request,
        &source_place_refs,
        &source_transition_names,
        &source_transitions,
        &mut diagnostics,
    );
    let target_ids = request
        .targets
        .iter()
        .map(|target| target.target_id.clone())
        .collect::<BTreeSet<_>>();
    let target_index = request
        .targets
        .iter()
        .map(|target| (target.target_id.clone(), target))
        .collect::<BTreeMap<_, _>>();

    for boundary in &request.boundaries {
        validate_boundary(
            boundary,
            &target_roles,
            &target_ids,
            &source_boundaries,
            &source_transitions,
            &place_to_target,
            &mut diagnostics,
            &mut rejected_rows,
        );
    }

    for boundary_ref in source_boundaries.keys() {
        if !request
            .boundaries
            .iter()
            .any(|boundary| boundary.boundary_ref == *boundary_ref)
        {
            diagnostics.push(ProjectionDiagnostic {
                code: "missing_boundary_projection".to_string(),
                message: format!("projection request does not declare boundary `{boundary_ref}`"),
            });
            rejected_rows.push(format!("{boundary_ref}:missing_boundary_projection"));
        }
    }

    let (packet_schemas, ffi_schemas) = if diagnostics.is_empty() {
        build_boundary_schemas(
            &request,
            &source_boundaries,
            &target_index,
            &mut diagnostics,
            &mut rejected_rows,
        )
    } else {
        (Vec::new(), Vec::new())
    };

    let projection_ir = if diagnostics.is_empty() {
        build_projection_ir(
            &projection_id,
            &analysis.modules,
            &request,
            &source_boundaries,
        )
    } else {
        empty_projection_ir(&projection_id)
    };
    let target_manifests = if diagnostics.is_empty() {
        build_target_manifests(&request, &source_boundaries, &place_to_target)
    } else {
        Vec::new()
    };
    let preservation_report = projection_preservation_report(
        &projection_id,
        &analysis.modules,
        &request,
        source_boundaries.values().collect(),
        rejected_rows,
    );

    if diagnostics.is_empty() {
        FullSystemV1ProjectionReport {
            accepted: true,
            projection_id,
            source_path: source_path_text,
            request_path: request_path_text,
            projection_ir,
            target_manifests,
            packet_schemas,
            ffi_schemas,
            preservation_report: preservation_report.clone(),
            diagnostics,
            residual_obligations: preservation_report.residual_obligations.clone(),
            final_public_api_frozen: false,
        }
    } else {
        rejected_report(
            source_path_text,
            request_path_text,
            projection_id,
            diagnostics,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            projection_ir,
            preservation_report,
        )
    }
}

fn load_request(path: &Path) -> Result<ProjectionRequest, ProjectionDiagnostic> {
    let text = fs::read_to_string(path).map_err(|error| ProjectionDiagnostic {
        code: "projection_request_io".to_string(),
        message: format!(
            "could not read projection request `{}`: {error}",
            path.display()
        ),
    })?;
    serde_json::from_str(&text).map_err(|error| ProjectionDiagnostic {
        code: "projection_request_parse_error".to_string(),
        message: format!(
            "could not parse projection request `{}`: {error}",
            path.display()
        ),
    })
}

fn request_projection_id(path: &Path) -> &str {
    path.file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("projection")
}

fn projection_diagnostic_from_textual(diagnostic: &TextualMirDiagnostic) -> ProjectionDiagnostic {
    ProjectionDiagnostic {
        code: diagnostic.code.clone(),
        message: diagnostic.message.clone(),
    }
}

fn validate_request_schema(
    request: &ProjectionRequest,
    diagnostics: &mut Vec<ProjectionDiagnostic>,
) {
    if request.schema_version != PROJECTION_REQUEST_SCHEMA_VERSION {
        diagnostics.push(ProjectionDiagnostic {
            code: "projection_request_schema_mismatch".to_string(),
            message: format!(
                "expected `{PROJECTION_REQUEST_SCHEMA_VERSION}` but found `{}`",
                request.schema_version
            ),
        });
    }
}

fn validate_targets(
    request: &ProjectionRequest,
    source_place_refs: &BTreeSet<String>,
    source_transition_names: &BTreeSet<String>,
    source_transitions: &BTreeMap<String, SourceTransitionSummary>,
    diagnostics: &mut Vec<ProjectionDiagnostic>,
) -> (BTreeMap<String, String>, BTreeMap<String, String>) {
    let mut target_roles = BTreeMap::new();
    let mut assigned_places = BTreeMap::new();
    let mut server_has_save_load_authority = false;
    for target in &request.targets {
        if target_roles
            .insert(target.target_id.clone(), target.role.clone())
            .is_some()
        {
            diagnostics.push(ProjectionDiagnostic {
                code: "duplicate_target_id".to_string(),
                message: format!("duplicate target id `{}`", target.target_id),
            });
        }
        if !matches!(target.role.as_str(), "server" | "client" | "adapter") {
            diagnostics.push(ProjectionDiagnostic {
                code: "unknown_target_role".to_string(),
                message: format!(
                    "target `{}` uses unsupported role `{}`",
                    target.target_id, target.role
                ),
            });
        }
        if target.save_load_authority {
            if target.role == "server" {
                server_has_save_load_authority = true;
            } else {
                diagnostics.push(ProjectionDiagnostic {
                    code: "save_load_authority_requires_server_target".to_string(),
                    message: format!(
                        "target `{}` role `{}` cannot own save/load authority",
                        target.target_id, target.role
                    ),
                });
            }
        }
        for place_ref in &target.place_refs {
            if !source_place_refs.contains(place_ref) {
                diagnostics.push(ProjectionDiagnostic {
                    code: "unknown_place_ref".to_string(),
                    message: format!(
                        "target `{}` references unknown place `{place_ref}`",
                        target.target_id
                    ),
                });
                continue;
            }
            if let Some(previous_target) =
                assigned_places.insert(place_ref.clone(), target.target_id.clone())
            {
                diagnostics.push(ProjectionDiagnostic {
                    code: "duplicate_place_assignment".to_string(),
                    message: format!(
                        "place `{place_ref}` is assigned to both `{previous_target}` and `{}`",
                        target.target_id
                    ),
                });
            }
        }
        for transition_name in &target.entry_transitions {
            if !source_transition_names.contains(transition_name) {
                diagnostics.push(ProjectionDiagnostic {
                    code: "unknown_entry_transition".to_string(),
                    message: format!(
                        "target `{}` references unknown transition `{transition_name}`",
                        target.target_id
                    ),
                });
                continue;
            }
            let Some(summary) = source_transitions.get(transition_name) else {
                continue;
            };
            if !target.place_refs.is_empty() && !target.place_refs.contains(&summary.place_ref) {
                diagnostics.push(ProjectionDiagnostic {
                    code: "entry_transition_place_mismatch".to_string(),
                    message: format!(
                        "transition `{transition_name}` belongs to place `{}` not assigned to target `{}`",
                        summary.place_ref, target.target_id
                    ),
                });
            }
        }
    }
    for place_ref in source_place_refs {
        if !assigned_places.contains_key(place_ref) {
            diagnostics.push(ProjectionDiagnostic {
                code: "unassigned_place_ref".to_string(),
                message: format!("source place `{place_ref}` is not assigned to any target"),
            });
        }
    }
    if request.targets.iter().any(|target| target.role == "server")
        && !server_has_save_load_authority
    {
        diagnostics.push(ProjectionDiagnostic {
            code: "missing_server_save_load_authority".to_string(),
            message: "at least one server target must own save/load authority".to_string(),
        });
    }
    (target_roles, assigned_places)
}

fn validate_boundary(
    boundary: &ProjectionBoundaryRequest,
    target_roles: &BTreeMap<String, String>,
    target_ids: &BTreeSet<String>,
    source_boundaries: &BTreeMap<String, SourceBoundarySummary>,
    source_transitions: &BTreeMap<String, SourceTransitionSummary>,
    place_to_target: &BTreeMap<String, String>,
    diagnostics: &mut Vec<ProjectionDiagnostic>,
    rejected_rows: &mut Vec<String>,
) {
    if !target_ids.contains(&boundary.from_target) {
        diagnostics.push(ProjectionDiagnostic {
            code: "unknown_boundary_target".to_string(),
            message: format!(
                "boundary `{}` references unknown from_target `{}`",
                boundary.boundary_ref, boundary.from_target
            ),
        });
        rejected_rows.push(format!("{}:unknown_boundary_target", boundary.boundary_ref));
    }
    if !target_ids.contains(&boundary.to_target) {
        diagnostics.push(ProjectionDiagnostic {
            code: "unknown_boundary_target".to_string(),
            message: format!(
                "boundary `{}` references unknown to_target `{}`",
                boundary.boundary_ref, boundary.to_target
            ),
        });
        rejected_rows.push(format!("{}:unknown_boundary_target", boundary.boundary_ref));
    }

    let Some(source_summary) = source_boundaries.get(&boundary.boundary_ref) else {
        diagnostics.push(ProjectionDiagnostic {
            code: "unknown_boundary_ref".to_string(),
            message: format!(
                "projection request boundary `{}` is not present in source",
                boundary.boundary_ref
            ),
        });
        rejected_rows.push(format!("{}:unknown_boundary_ref", boundary.boundary_ref));
        return;
    };

    let request_effects = boundary
        .effect_names
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if request_effects != source_summary.effect_names {
        diagnostics.push(ProjectionDiagnostic {
            code: "boundary_effect_row_mismatch".to_string(),
            message: format!(
                "boundary `{}` effect rows {:?} do not match source {:?}",
                boundary.boundary_ref, request_effects, source_summary.effect_names
            ),
        });
        rejected_rows.push(format!(
            "{}:boundary_effect_row_mismatch",
            boundary.boundary_ref
        ));
    }

    match boundary.boundary_kind.as_str() {
        "packet" => {
            if boundary.packet_schema_ref.is_none() || boundary.ffi_schema_ref.is_some() {
                diagnostics.push(ProjectionDiagnostic {
                    code: "packet_schema_ref_required".to_string(),
                    message: format!(
                        "packet boundary `{}` must declare packet_schema_ref and no ffi_schema_ref",
                        boundary.boundary_ref
                    ),
                });
                rejected_rows.push(format!(
                    "{}:packet_schema_ref_required",
                    boundary.boundary_ref
                ));
            }
        }
        "ffi" => {
            if boundary.ffi_schema_ref.is_none() || boundary.packet_schema_ref.is_some() {
                diagnostics.push(ProjectionDiagnostic {
                    code: "ffi_schema_ref_required".to_string(),
                    message: format!(
                        "ffi boundary `{}` must declare ffi_schema_ref and no packet_schema_ref",
                        boundary.boundary_ref
                    ),
                });
                rejected_rows.push(format!("{}:ffi_schema_ref_required", boundary.boundary_ref));
            }
        }
        kind => {
            diagnostics.push(ProjectionDiagnostic {
                code: "unknown_boundary_kind".to_string(),
                message: format!(
                    "boundary `{}` uses unsupported kind `{kind}`",
                    boundary.boundary_ref
                ),
            });
            rejected_rows.push(format!("{}:unknown_boundary_kind", boundary.boundary_ref));
        }
    }

    for transition_name in &source_summary.transition_names {
        let Some(transition) = source_transitions.get(transition_name) else {
            continue;
        };
        if let Some(owner_target) = place_to_target.get(&transition.place_ref) {
            if owner_target != &boundary.from_target && owner_target != &boundary.to_target {
                diagnostics.push(ProjectionDiagnostic {
                    code: "transition_owner_detached_from_boundary".to_string(),
                    message: format!(
                        "transition `{transition_name}` place `{}` is not connected to boundary `{}` targets",
                        transition.place_ref, boundary.boundary_ref
                    ),
                });
                rejected_rows.push(format!(
                    "{}:transition_owner_detached_from_boundary",
                    boundary.boundary_ref
                ));
            }
        }
    }

    if role_for(target_roles, &boundary.from_target) == Some("client")
        && boundary_requests_authoritative_mutation(source_summary, &boundary.authority)
    {
        diagnostics.push(ProjectionDiagnostic {
            code: "client_write_authority_escalation".to_string(),
            message: format!(
                "client target `{}` cannot own world-write boundary `{}`",
                boundary.from_target, boundary.boundary_ref
            ),
        });
        rejected_rows.push(format!(
            "{}:client_write_authority_escalation",
            boundary.boundary_ref
        ));
    }

    if role_for(target_roles, &boundary.from_target) == Some("adapter")
        && boundary_requests_authoritative_mutation(source_summary, &boundary.authority)
    {
        diagnostics.push(ProjectionDiagnostic {
            code: "adapter_server_state_mutation".to_string(),
            message: format!(
                "adapter target `{}` cannot own world-write boundary `{}`",
                boundary.from_target, boundary.boundary_ref
            ),
        });
        rejected_rows.push(format!(
            "{}:adapter_server_state_mutation",
            boundary.boundary_ref
        ));
    }
}

fn role_for<'a>(target_roles: &'a BTreeMap<String, String>, target_id: &str) -> Option<&'a str> {
    target_roles.get(target_id).map(String::as_str)
}

fn boundary_requests_authoritative_mutation(
    summary: &SourceBoundarySummary,
    authority: &str,
) -> bool {
    if authority.contains("world_write") || authority.contains("server_state_write") {
        return true;
    }
    summary
        .required_capabilities
        .iter()
        .any(|capability| capability_requires_authoritative_mutation(capability))
}

fn capability_requires_authoritative_mutation(capability: &str) -> bool {
    matches!(
        capability,
        "HostWrite" | "Publisher" | "WitnessAuthority" | "CutAuthority"
    )
}

fn collect_source_transitions(
    modules: &[TypedMirModule],
) -> BTreeMap<String, SourceTransitionSummary> {
    let mut transitions = BTreeMap::new();
    for module in modules {
        for transition in &module.transitions {
            transitions.insert(
                transition.transition_name.clone(),
                SourceTransitionSummary {
                    place_ref: transition.place_ref.clone(),
                },
            );
        }
    }
    transitions
}

fn collect_source_boundaries(
    modules: &[TypedMirModule],
) -> BTreeMap<String, SourceBoundarySummary> {
    let effect_index = modules
        .iter()
        .flat_map(|module| module.effects.iter())
        .map(|effect| {
            (
                effect.effect_name.clone(),
                SourceEffectSummary {
                    failure_row: effect.failure_row.iter().cloned().collect::<BTreeSet<_>>(),
                    required_capabilities: effect
                        .required_capabilities
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                    request_fields: effect
                        .parameters
                        .iter()
                        .map(|param| projection_schema_field(&param.name, &param.param_type))
                        .collect(),
                    response_fields: effect
                        .output
                        .iter()
                        .map(|output| projection_schema_field(&output.name, &output.output_type))
                        .collect(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut boundaries = BTreeMap::new();
    for module in modules {
        for transition in &module.transitions {
            collect_stmts_into_boundaries(
                &transition.transition_name,
                &transition.place_ref,
                &transition.body,
                &effect_index,
                &mut boundaries,
            );
        }
    }
    boundaries
}

fn collect_stmts_into_boundaries(
    transition_name: &str,
    place_ref: &str,
    body: &[TypedStmt],
    effect_index: &BTreeMap<String, SourceEffectSummary>,
    boundaries: &mut BTreeMap<String, SourceBoundarySummary>,
) {
    for stmt in body {
        match stmt {
            TypedStmt::Bind { value, .. } => {
                if let TypedBindValue::Perform(call) = value {
                    collect_call(transition_name, place_ref, call, effect_index, boundaries);
                }
            }
            TypedStmt::Perform { call, .. } => {
                collect_call(transition_name, place_ref, call, effect_index, boundaries);
            }
            TypedStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_stmts_into_boundaries(
                    transition_name,
                    place_ref,
                    then_body,
                    effect_index,
                    boundaries,
                );
                collect_stmts_into_boundaries(
                    transition_name,
                    place_ref,
                    else_body,
                    effect_index,
                    boundaries,
                );
            }
            TypedStmt::While { body, .. } | TypedStmt::For { body, .. } => {
                collect_stmts_into_boundaries(
                    transition_name,
                    place_ref,
                    body,
                    effect_index,
                    boundaries,
                );
            }
            TypedStmt::Let { .. } | TypedStmt::Assign { .. } | TypedStmt::Return { .. } => {}
        }
    }
}

fn collect_call(
    transition_name: &str,
    place_ref: &str,
    call: &super::typed_ir::TypedPerformCall,
    effect_index: &BTreeMap<String, SourceEffectSummary>,
    boundaries: &mut BTreeMap<String, SourceBoundarySummary>,
) {
    let summary = boundaries
        .entry(call.boundary_ref.clone())
        .or_insert_with(|| SourceBoundarySummary {
            effect_names: BTreeSet::new(),
            failure_row: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            transition_names: BTreeSet::new(),
            place_refs: BTreeSet::new(),
            effect_summaries: BTreeMap::new(),
        });
    summary.effect_names.insert(call.effect_name.clone());
    summary.transition_names.insert(transition_name.to_string());
    summary.place_refs.insert(place_ref.to_string());

    let effect_summary = effect_index
        .get(&call.effect_name)
        .cloned()
        .unwrap_or_else(|| SourceEffectSummary {
            failure_row: call.failure_row.iter().cloned().collect(),
            required_capabilities: call.required_capabilities.iter().cloned().collect(),
            request_fields: call
                .arguments
                .iter()
                .enumerate()
                .map(|(index, argument)| {
                    projection_schema_field(&format!("arg{index}"), &argument.ty)
                })
                .collect(),
            response_fields: call
                .output_type
                .as_ref()
                .map(|output_type| vec![projection_schema_field("result", output_type)])
                .unwrap_or_default(),
        });
    summary
        .failure_row
        .extend(effect_summary.failure_row.iter().cloned());
    summary
        .required_capabilities
        .extend(effect_summary.required_capabilities.iter().cloned());
    summary
        .effect_summaries
        .entry(call.effect_name.clone())
        .or_insert(effect_summary);
}

fn projection_schema_field(name: &str, ty: &TypedType) -> ProjectionSchemaField {
    ProjectionSchemaField {
        name: name.to_string(),
        ty: ty.display_name(),
    }
}

fn build_boundary_schemas(
    request: &ProjectionRequest,
    source_boundaries: &BTreeMap<String, SourceBoundarySummary>,
    target_index: &BTreeMap<String, &ProjectionTargetRequest>,
    diagnostics: &mut Vec<ProjectionDiagnostic>,
    rejected_rows: &mut Vec<String>,
) -> (Vec<ProjectionBoundarySchema>, Vec<ProjectionBoundarySchema>) {
    let mut packet_schemas = Vec::new();
    let mut ffi_schemas = Vec::new();
    for boundary in &request.boundaries {
        let Some(source) = source_boundaries.get(&boundary.boundary_ref) else {
            continue;
        };
        let contract = match boundary_schema_contract(boundary, source) {
            Ok(contract) => contract,
            Err(BoundarySchemaMismatch::PayloadShape) => {
                diagnostics.push(ProjectionDiagnostic {
                    code: "boundary_payload_shape_mismatch".to_string(),
                    message: format!(
                        "boundary `{}` mixes incompatible request/response payload shapes across its effect row",
                        boundary.boundary_ref
                    ),
                });
                rejected_rows.push(format!(
                    "{}:boundary_payload_shape_mismatch",
                    boundary.boundary_ref
                ));
                continue;
            }
            Err(BoundarySchemaMismatch::EffectContract) => {
                diagnostics.push(ProjectionDiagnostic {
                    code: "boundary_effect_contract_mismatch".to_string(),
                    message: format!(
                        "boundary `{}` mixes same-shape effects with incompatible failure/capability contracts",
                        boundary.boundary_ref
                    ),
                });
                rejected_rows.push(format!(
                    "{}:boundary_effect_contract_mismatch",
                    boundary.boundary_ref
                ));
                continue;
            }
        };
        let from_target = target_index
            .get(&boundary.from_target)
            .expect("validated from_target should exist");
        let to_target = target_index
            .get(&boundary.to_target)
            .expect("validated to_target should exist");
        let schema = ProjectionBoundarySchema {
            schema_ref: boundary
                .packet_schema_ref
                .clone()
                .or_else(|| boundary.ffi_schema_ref.clone())
                .expect("validated boundary should have schema ref"),
            schema_kind: boundary.boundary_kind.clone(),
            boundary_ref: boundary.boundary_ref.clone(),
            from_target: boundary.from_target.clone(),
            to_target: boundary.to_target.clone(),
            effect_names: sorted_vec(source.effect_names.iter().cloned()),
            request_fields: contract.request_fields,
            response_fields: contract.response_fields,
            failure_row: contract.failure_row,
            capability_row: contract.capability_row,
            required_witnesses: boundary.required_witnesses.clone(),
            authority_policy: boundary.authority.clone(),
            from_provider_policy: from_target.provider_policy.clone(),
            to_provider_policy: to_target.provider_policy.clone(),
            from_observation_policy: from_target.observation_policy.clone(),
            to_observation_policy: to_target.observation_policy.clone(),
            from_redaction_policy: from_target.redaction_policy.clone(),
            to_redaction_policy: to_target.redaction_policy.clone(),
            from_retention_policy: from_target.retention_policy.clone(),
            to_retention_policy: to_target.retention_policy.clone(),
            rollback_cut_compatible: boundary.rollback_cut_compatible,
            replay_compatible: boundary.replay_compatible,
            save_load_obligation: boundary.save_load_obligation.clone(),
        };
        match boundary.boundary_kind.as_str() {
            "packet" => packet_schemas.push(schema),
            "ffi" => ffi_schemas.push(schema),
            _ => {}
        }
    }
    (packet_schemas, ffi_schemas)
}

fn boundary_schema_contract(
    boundary: &ProjectionBoundaryRequest,
    source: &SourceBoundarySummary,
) -> Result<BoundarySchemaContract, BoundarySchemaMismatch> {
    let mut summaries = boundary
        .effect_names
        .iter()
        .filter_map(|effect_name| source.effect_summaries.get(effect_name))
        .map(|summary| {
            (
                BoundaryPayloadShape {
                    request_fields: summary.request_fields.clone(),
                    response_fields: summary.response_fields.clone(),
                },
                summary.failure_row.clone(),
                summary.required_capabilities.clone(),
            )
        });
    let Some((first_shape, first_failure_row, first_capability_row)) = summaries.next() else {
        return Ok(BoundarySchemaContract {
            request_fields: Vec::new(),
            response_fields: Vec::new(),
            failure_row: Vec::new(),
            capability_row: Vec::new(),
        });
    };
    for (shape, failure_row, capability_row) in summaries {
        if shape != first_shape {
            return Err(BoundarySchemaMismatch::PayloadShape);
        }
        if failure_row != first_failure_row || capability_row != first_capability_row {
            return Err(BoundarySchemaMismatch::EffectContract);
        }
    }
    Ok(BoundarySchemaContract {
        request_fields: first_shape.request_fields,
        response_fields: first_shape.response_fields,
        failure_row: sorted_vec(first_failure_row.into_iter()),
        capability_row: sorted_vec(first_capability_row.into_iter()),
    })
}

fn build_projection_ir(
    projection_id: &str,
    modules: &[TypedMirModule],
    request: &ProjectionRequest,
    source_boundaries: &BTreeMap<String, SourceBoundarySummary>,
) -> ProjectionIr {
    let module_refs = modules
        .iter()
        .map(|module| module.module_path.clone())
        .collect::<Vec<_>>();
    let targets = request
        .targets
        .iter()
        .map(|target| ProjectionTargetIr {
            target_id: target.target_id.clone(),
            role: target.role.clone(),
            place_refs: target.place_refs.clone(),
            entry_transitions: target.entry_transitions.clone(),
            observation_policy: target.observation_policy.clone(),
            redaction_policy: target.redaction_policy.clone(),
            retention_policy: target.retention_policy.clone(),
            provider_policy: target.provider_policy.clone(),
            save_load_authority: target.save_load_authority,
            prediction_allowed: target.prediction_allowed,
        })
        .collect::<Vec<_>>();
    let boundaries = request
        .boundaries
        .iter()
        .map(|boundary| {
            let source = source_boundaries
                .get(&boundary.boundary_ref)
                .expect("validated boundary should exist");
            ProjectionBoundaryIr {
                boundary_ref: boundary.boundary_ref.clone(),
                boundary_kind: boundary.boundary_kind.clone(),
                effect_names: boundary.effect_names.clone(),
                failure_row: sorted_vec(source.failure_row.iter().cloned()),
                required_capabilities: sorted_vec(source.required_capabilities.iter().cloned()),
                required_witnesses: boundary.required_witnesses.clone(),
                from_target: boundary.from_target.clone(),
                to_target: boundary.to_target.clone(),
                authority: boundary.authority.clone(),
                packet_schema_ref: boundary.packet_schema_ref.clone(),
                ffi_schema_ref: boundary.ffi_schema_ref.clone(),
                rollback_cut_compatible: boundary.rollback_cut_compatible,
                replay_compatible: boundary.replay_compatible,
                save_load_obligation: boundary.save_load_obligation.clone(),
            }
        })
        .collect::<Vec<_>>();
    ProjectionIr {
        projection_id: projection_id.to_string(),
        source_module_refs: module_refs,
        targets,
        boundaries,
    }
}

fn build_target_manifests(
    request: &ProjectionRequest,
    source_boundaries: &BTreeMap<String, SourceBoundarySummary>,
    place_to_target: &BTreeMap<String, String>,
) -> Vec<ProjectionTargetManifest> {
    let mut manifests = Vec::new();
    for target in &request.targets {
        let related_boundaries = request
            .boundaries
            .iter()
            .filter(|boundary| {
                boundary.from_target == target.target_id || boundary.to_target == target.target_id
            })
            .collect::<Vec<_>>();
        let mut effect_names = BTreeSet::new();
        let mut failure_row = BTreeSet::new();
        let mut capability_row = BTreeSet::new();
        let mut witness_requirements = BTreeSet::new();
        let mut packet_schema_refs = BTreeSet::new();
        let mut ffi_schema_refs = BTreeSet::new();
        for boundary in &related_boundaries {
            if let Some(source) = source_boundaries.get(&boundary.boundary_ref) {
                effect_names.extend(source.effect_names.iter().cloned());
                if source_boundary_owner_targets(source, place_to_target)
                    .contains(&target.target_id)
                {
                    failure_row.extend(source.failure_row.iter().cloned());
                    capability_row.extend(source.required_capabilities.iter().cloned());
                }
            }
            witness_requirements.extend(boundary.required_witnesses.iter().cloned());
            if let Some(packet_schema_ref) = &boundary.packet_schema_ref {
                packet_schema_refs.insert(packet_schema_ref.clone());
            }
            if let Some(ffi_schema_ref) = &boundary.ffi_schema_ref {
                ffi_schema_refs.insert(ffi_schema_ref.clone());
            }
        }
        manifests.push(ProjectionTargetManifest {
            target_id: target.target_id.clone(),
            role: target.role.clone(),
            place_refs: target.place_refs.clone(),
            entry_transitions: target.entry_transitions.clone(),
            boundary_refs: related_boundaries
                .iter()
                .map(|boundary| boundary.boundary_ref.clone())
                .collect(),
            effect_names: sorted_vec(effect_names),
            failure_row: sorted_vec(failure_row),
            capability_row: sorted_vec(capability_row),
            witness_requirements: sorted_vec(witness_requirements),
            packet_schema_refs: sorted_vec(packet_schema_refs),
            ffi_schema_refs: sorted_vec(ffi_schema_refs),
            observation_policy: target.observation_policy.clone(),
            redaction_policy: target.redaction_policy.clone(),
            retention_policy: target.retention_policy.clone(),
            provider_policy: target.provider_policy.clone(),
            save_load_authority: target.save_load_authority,
            prediction_allowed: target.prediction_allowed,
        });
    }
    manifests
}

fn source_boundary_owner_targets(
    source: &SourceBoundarySummary,
    place_to_target: &BTreeMap<String, String>,
) -> BTreeSet<String> {
    source
        .place_refs
        .iter()
        .filter_map(|place_ref| place_to_target.get(place_ref).cloned())
        .collect()
}

fn projection_preservation_report(
    projection_id: &str,
    modules: &[TypedMirModule],
    request: &ProjectionRequest,
    source_boundaries: Vec<&SourceBoundarySummary>,
    rejected_rows: Vec<String>,
) -> ProjectionPreservationReport {
    let source_refs = modules
        .iter()
        .map(|module| module.module_path.clone())
        .collect::<Vec<_>>();
    let typed_ir_refs = source_refs.clone();
    let projection_ir_refs = request
        .targets
        .iter()
        .map(|target| format!("{projection_id}::target::{}", target.target_id))
        .chain(
            request
                .boundaries
                .iter()
                .map(|boundary| format!("{projection_id}::boundary::{}", boundary.boundary_ref)),
        )
        .collect::<Vec<_>>();
    let target_manifest_refs = request
        .targets
        .iter()
        .map(|target| target.target_id.clone())
        .collect::<Vec<_>>();
    let packet_schema_refs = request
        .boundaries
        .iter()
        .filter_map(|boundary| boundary.packet_schema_ref.clone())
        .collect::<BTreeSet<_>>();
    let ffi_schema_refs = request
        .boundaries
        .iter()
        .filter_map(|boundary| boundary.ffi_schema_ref.clone())
        .collect::<BTreeSet<_>>();
    let checked_effect_rows = source_boundaries
        .iter()
        .flat_map(|boundary| boundary.effect_names.iter().cloned())
        .collect::<BTreeSet<_>>();
    let checked_failure_rows = source_boundaries
        .iter()
        .flat_map(|boundary| boundary.failure_row.iter().cloned())
        .collect::<BTreeSet<_>>();
    let checked_capability_rows = source_boundaries
        .iter()
        .flat_map(|boundary| boundary.required_capabilities.iter().cloned())
        .collect::<BTreeSet<_>>();
    let checked_authority_rows = request
        .boundaries
        .iter()
        .map(|boundary| format!("{}:{}", boundary.boundary_ref, boundary.authority))
        .collect::<BTreeSet<_>>();
    let checked_observation_rows = request
        .targets
        .iter()
        .map(|target| format!("{}:{}", target.target_id, target.observation_policy))
        .collect::<BTreeSet<_>>();
    let checked_provider_policy_rows = request
        .targets
        .iter()
        .map(|target| format!("{}:{}", target.target_id, target.provider_policy))
        .collect::<BTreeSet<_>>();
    let checked_rollback_replay_cut_rows = request
        .boundaries
        .iter()
        .map(|boundary| {
            format!(
                "{}:{}:{}:{}",
                boundary.boundary_ref,
                boundary.rollback_cut_compatible,
                boundary.replay_compatible,
                boundary.save_load_obligation
            )
        })
        .collect::<BTreeSet<_>>();
    let residual_obligations = vec![
        FullSystemV1Obligation {
            code: "packet_ffi_transport_semantics_deferred".to_string(),
            message: "packet and FFI schemas are emitted, but executable transport/runtime semantics remain later work".to_string(),
        },
        FullSystemV1Obligation {
            code: "server_client_runtime_split_deferred".to_string(),
            message: "projection target roles are realized, but executable server/client role split remains later work".to_string(),
        },
        FullSystemV1Obligation {
            code: "provider_admission_deferred".to_string(),
            message: "provider policy is preserved in manifests, but provider admission enforcement remains later work".to_string(),
        },
    ];
    ProjectionPreservationReport {
        source_refs,
        typed_ir_refs,
        projection_ir_refs,
        target_manifest_refs,
        packet_schema_refs: sorted_vec(packet_schema_refs),
        ffi_schema_refs: sorted_vec(ffi_schema_refs),
        checked_effect_rows: sorted_vec(checked_effect_rows),
        checked_failure_rows: sorted_vec(checked_failure_rows),
        checked_capability_rows: sorted_vec(checked_capability_rows),
        checked_authority_rows: sorted_vec(checked_authority_rows),
        checked_observation_rows: sorted_vec(checked_observation_rows),
        checked_provider_policy_rows: sorted_vec(checked_provider_policy_rows),
        checked_rollback_replay_cut_rows: sorted_vec(checked_rollback_replay_cut_rows),
        rejected_rows,
        residual_obligations,
    }
}

fn rejected_report(
    source_path: String,
    request_path: String,
    projection_id: String,
    diagnostics: Vec<ProjectionDiagnostic>,
    target_manifests: Vec<ProjectionTargetManifest>,
    packet_schemas: Vec<ProjectionBoundarySchema>,
    ffi_schemas: Vec<ProjectionBoundarySchema>,
    projection_ir: ProjectionIr,
    preservation_report: ProjectionPreservationReport,
) -> FullSystemV1ProjectionReport {
    FullSystemV1ProjectionReport {
        accepted: false,
        projection_id,
        source_path,
        request_path,
        projection_ir,
        target_manifests,
        packet_schemas,
        ffi_schemas,
        residual_obligations: preservation_report.residual_obligations.clone(),
        preservation_report,
        diagnostics,
        final_public_api_frozen: false,
    }
}

fn empty_projection_ir(projection_id: &str) -> ProjectionIr {
    ProjectionIr {
        projection_id: projection_id.to_string(),
        source_module_refs: Vec::new(),
        targets: Vec::new(),
        boundaries: Vec::new(),
    }
}

fn empty_preservation_report() -> ProjectionPreservationReport {
    ProjectionPreservationReport {
        source_refs: Vec::new(),
        typed_ir_refs: Vec::new(),
        projection_ir_refs: Vec::new(),
        target_manifest_refs: Vec::new(),
        packet_schema_refs: Vec::new(),
        ffi_schema_refs: Vec::new(),
        checked_effect_rows: Vec::new(),
        checked_failure_rows: Vec::new(),
        checked_capability_rows: Vec::new(),
        checked_authority_rows: Vec::new(),
        checked_observation_rows: Vec::new(),
        checked_provider_policy_rows: Vec::new(),
        checked_rollback_replay_cut_rows: Vec::new(),
        rejected_rows: Vec::new(),
        residual_obligations: Vec::new(),
    }
}

fn sorted_vec(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values
}
