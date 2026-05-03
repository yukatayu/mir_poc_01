use std::collections::BTreeSet;

use mirrorea_core::{
    HotPlugRequest, HotPlugVerdict, LayerSignature, LogicalPlaceRuntimeShell,
    LogicalPlaceRuntimeSnapshot, MirroreaCoreError, PrincipalClaim, layer_signature_lanes,
};
use serde::Serialize;

use crate::{
    alpha_local_runtime::build_local_sugoroku_runtime_report,
    hotplug_runtime::{HotPlugRuntimeSkeletonReport, assemble_hotplug_runtime_skeleton_report},
};

const WORLD_PLACE: &str = "WorldPlace[AlphaRoom#1]";
const WORLD_KIND: &str = "WorldPlace";
const GAME_PLACE: &str = "GamePlace[SugorokuGame#1]";
const GAME_KIND: &str = "SugorokuGamePlace";
const ATTACHPOINT_REF: &str = "AttachPoint[AlphaRoom#1::MessageDispatch]";
const ATTACHPOINT_KIND: &str = "AttachPoint";
const ADMIN_PRINCIPAL: &str = "ExampleAdmin";
const NON_ADMIN_PRINCIPAL: &str = "Mallory";

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayerKind {
    Debug,
    Auth,
    RateLimit,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttachPoint {
    MessageDispatch,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractMode {
    TransparentOverlay,
    ExplicitContractUpdate,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RedactionLevel {
    None,
    SubjectRedacted,
    SubjectAndPayloadRedacted,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TraceMode {
    Off,
    OnDemand,
    AlwaysOn,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerContract {
    pub layer_name: String,
    pub layer_kind: LayerKind,
    pub attach_point: AttachPoint,
    pub contract_mode: ContractMode,
    pub input_type: String,
    pub output_type: String,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub provided_capabilities: Vec<String>,
    pub provided_interface: Vec<String>,
    pub observation_labels: Vec<String>,
    pub redaction_level: RedactionLevel,
    pub retention_scope: String,
    pub trace_mode: TraceMode,
    pub attach_capability: String,
    pub notes: Vec<String>,
}

impl LayerContract {
    fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("LayerContract", "layer_name", &self.layer_name)?;
        require_non_empty("LayerContract", "input_type", &self.input_type)?;
        require_non_empty("LayerContract", "output_type", &self.output_type)?;
        require_non_empty_items("LayerContract", "preconditions", &self.preconditions)?;
        require_non_empty_items("LayerContract", "postconditions", &self.postconditions)?;
        require_non_empty_items(
            "LayerContract",
            "required_capabilities",
            &self.required_capabilities,
        )?;
        require_non_empty_items(
            "LayerContract",
            "provided_capabilities",
            &self.provided_capabilities,
        )?;
        require_non_empty_items(
            "LayerContract",
            "provided_interface",
            &self.provided_interface,
        )?;
        require_non_empty("LayerContract", "retention_scope", &self.retention_scope)?;
        require_non_empty(
            "LayerContract",
            "attach_capability",
            &self.attach_capability,
        )?;
        require_non_empty_items("LayerContract", "notes", &self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AttachPointPolicy {
    pub attachpoint_ref: String,
    pub attachpoint_kind: String,
    pub attach_point: AttachPoint,
    pub allowed_layer_kinds: Vec<LayerKind>,
    pub base_contract_name: String,
    pub input_type: String,
    pub output_type: String,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub allowed_effect_row: Vec<String>,
    pub allowed_failure_row: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub provided_capabilities: Vec<String>,
    pub provided_interface: Vec<String>,
    pub allowed_observation_labels: Vec<String>,
    pub minimum_redaction_level: RedactionLevel,
    pub allowed_retention_scopes: Vec<String>,
    pub allowed_trace_modes: Vec<TraceMode>,
    pub notes: Vec<String>,
}

impl AttachPointPolicy {
    fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty(
            "AttachPointPolicy",
            "attachpoint_ref",
            &self.attachpoint_ref,
        )?;
        require_non_empty(
            "AttachPointPolicy",
            "attachpoint_kind",
            &self.attachpoint_kind,
        )?;
        require_non_empty(
            "AttachPointPolicy",
            "base_contract_name",
            &self.base_contract_name,
        )?;
        require_non_empty("AttachPointPolicy", "input_type", &self.input_type)?;
        require_non_empty("AttachPointPolicy", "output_type", &self.output_type)?;
        require_non_empty_items(
            "AttachPointPolicy",
            "allowed_layer_kinds",
            &self.allowed_layer_kinds,
        )?;
        require_non_empty_items("AttachPointPolicy", "preconditions", &self.preconditions)?;
        require_non_empty_items("AttachPointPolicy", "postconditions", &self.postconditions)?;
        require_non_empty_items(
            "AttachPointPolicy",
            "required_capabilities",
            &self.required_capabilities,
        )?;
        require_non_empty_items(
            "AttachPointPolicy",
            "provided_capabilities",
            &self.provided_capabilities,
        )?;
        require_non_empty_items(
            "AttachPointPolicy",
            "provided_interface",
            &self.provided_interface,
        )?;
        require_non_empty_items(
            "AttachPointPolicy",
            "allowed_retention_scopes",
            &self.allowed_retention_scopes,
        )?;
        require_non_empty_items(
            "AttachPointPolicy",
            "allowed_trace_modes",
            &self.allowed_trace_modes,
        )?;
        require_non_empty_items("AttachPointPolicy", "notes", &self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerAttachRequest {
    pub request_id: String,
    pub sample_id: String,
    pub attachpoint_ref: String,
    pub principal_claim: PrincipalClaim,
    pub capability_refs: Vec<String>,
    pub membership_epoch: u64,
    pub member_incarnation: u64,
    pub witness_refs: Vec<String>,
    pub layer_signature: LayerSignature,
    pub requested_layer: LayerContract,
    pub contract_update_ref: Option<String>,
    pub notes: Vec<String>,
}

impl LayerAttachRequest {
    fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("LayerAttachRequest", "request_id", &self.request_id)?;
        require_non_empty("LayerAttachRequest", "sample_id", &self.sample_id)?;
        require_non_empty(
            "LayerAttachRequest",
            "attachpoint_ref",
            &self.attachpoint_ref,
        )?;
        self.principal_claim.validate()?;
        require_non_empty_items(
            "LayerAttachRequest",
            "capability_refs",
            &self.capability_refs,
        )?;
        require_non_empty_items("LayerAttachRequest", "witness_refs", &self.witness_refs)?;
        self.layer_signature.validate()?;
        self.requested_layer.validate()?;
        if let Some(contract_update_ref) = &self.contract_update_ref {
            require_non_empty(
                "LayerAttachRequest",
                "contract_update_ref",
                contract_update_ref,
            )?;
        }
        require_non_empty_items("LayerAttachRequest", "notes", &self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerContractCheck {
    pub check_name: String,
    pub result: String,
    pub reason_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerCompatibilityReport {
    pub contract_mode: ContractMode,
    pub accepted_path: String,
    pub checks: Vec<LayerContractCheck>,
    pub passed_reason_refs: Vec<String>,
    pub failed_reason_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerTraceRow {
    pub row_id: String,
    pub label: String,
    pub authority: String,
    pub redaction_level: RedactionLevel,
    pub retention_scope: String,
    pub summary: String,
    pub source_ref: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerRuntimePreview {
    pub preview_kind: String,
    pub terminal_outcome: String,
    pub reason_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerInsertionReport {
    pub runtime_scope: String,
    pub sample_id: String,
    pub layer_signature_lanes: Vec<String>,
    pub attach_request: LayerAttachRequest,
    pub attachpoint_policy: AttachPointPolicy,
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub hotplug_runtime_report: HotPlugRuntimeSkeletonReport,
    pub compatibility: LayerCompatibilityReport,
    pub active_layers_before: Vec<String>,
    pub active_layers_after: Vec<String>,
    pub pre_attach_trace_rows: Vec<LayerTraceRow>,
    pub post_attach_trace_rows: Vec<LayerTraceRow>,
    pub runtime_preview: Option<LayerRuntimePreview>,
    pub source_runtime_sample_ref: Option<String>,
    pub terminal_outcome: String,
    pub retained_later_refs: Vec<String>,
    pub notes: Vec<String>,
}

pub fn build_debug_layer_attach_report() -> Result<LayerInsertionReport, MirroreaCoreError> {
    let shell = bootstrap_shell()?;
    let policy = message_dispatch_attachpoint_policy();
    let request = layer_attach_request(
        "LI-01",
        ADMIN_PRINCIPAL,
        &[
            "AttachDebugLayer(AlphaRoom#1)",
            "ManageAttachPoint(AlphaRoom#1::MessageDispatch)",
        ],
        debug_layer_signature(),
        debug_layer_contract(),
        None,
        &["accepted attach must open debug trace only after activation cut"],
    )?;
    build_report(
        &shell,
        policy,
        request,
        Some(debug_trace_rows()?),
        None,
        Some("LR-01".to_string()),
    )
}

pub fn build_debug_layer_non_admin_rejection_report()
-> Result<LayerInsertionReport, MirroreaCoreError> {
    let shell = bootstrap_shell()?;
    let policy = message_dispatch_attachpoint_policy();
    let request = layer_attach_request(
        "LI-02",
        NON_ADMIN_PRINCIPAL,
        &["ObserveRoom(AlphaRoom#1)"],
        debug_layer_signature(),
        debug_layer_contract(),
        None,
        &["rejected attach must not activate debug trace"],
    )?;
    build_report(&shell, policy, request, None, None, None)
}

pub fn build_auth_layer_contract_update_report() -> Result<LayerInsertionReport, MirroreaCoreError>
{
    let shell = bootstrap_shell()?;
    let policy = message_dispatch_attachpoint_policy();
    let request = layer_attach_request(
        "LI-03",
        ADMIN_PRINCIPAL,
        &[
            "AttachAuthLayer(AlphaRoom#1)",
            "ManageAttachPoint(AlphaRoom#1::MessageDispatch)",
        ],
        auth_layer_signature(),
        auth_layer_contract(),
        Some("activation_cut#auth_contract_update"),
        &["auth layer is admitted only as explicit contract-update path"],
    )?;
    build_report(
        &shell,
        policy,
        request,
        None,
        Some(LayerRuntimePreview {
            preview_kind: "auth_contract_update_preview".to_string(),
            terminal_outcome: "accepted_contract_update".to_string(),
            reason_refs: vec![
                "contract_update_activation_cut_present".to_string(),
                "auth_failed_declared_on_new_contract_version".to_string(),
            ],
            notes: vec![
                "transparent overlay is intentionally not claimed for auth".to_string(),
                "runtime auth session semantics remain later".to_string(),
            ],
        }),
        None,
    )
}

pub fn build_declared_ratelimit_report() -> Result<LayerInsertionReport, MirroreaCoreError> {
    let shell = bootstrap_shell()?;
    let policy = message_dispatch_attachpoint_policy();
    let request = layer_attach_request(
        "LI-04",
        ADMIN_PRINCIPAL,
        &[
            "AttachRateLimitLayer(AlphaRoom#1)",
            "ManageAttachPoint(AlphaRoom#1::MessageDispatch)",
        ],
        rate_limit_layer_signature(),
        rate_limit_layer_contract(),
        None,
        &["rate-limit remains transparent only because RateLimited is already declared"],
    )?;
    build_report(
        &shell,
        policy,
        request,
        None,
        Some(LayerRuntimePreview {
            preview_kind: "rate_limit_dispatch_preview".to_string(),
            terminal_outcome: "rejected".to_string(),
            reason_refs: vec![
                "failure_row_within_declared_budget".to_string(),
                "rate_limit_budget_exhausted".to_string(),
                "RateLimited".to_string(),
            ],
            notes: vec![
                "preview proves declared-failure discipline only".to_string(),
                "real token-bucket runtime accounting remains later".to_string(),
            ],
        }),
        None,
    )
}

pub fn build_incompatible_patch_rejection_report() -> Result<LayerInsertionReport, MirroreaCoreError>
{
    let shell = bootstrap_shell()?;
    let policy = message_dispatch_attachpoint_policy();
    let request = layer_attach_request(
        "LI-05",
        ADMIN_PRINCIPAL,
        &[
            "AttachDebugLayer(AlphaRoom#1)",
            "ManageAttachPoint(AlphaRoom#1::MessageDispatch)",
        ],
        incompatible_patch_signature(),
        incompatible_patch_contract(),
        None,
        &["incompatible patch must be rejected before activation cut"],
    )?;
    build_report(&shell, policy, request, None, None, None)
}

pub fn build_closeout_reports() -> Result<Vec<LayerInsertionReport>, MirroreaCoreError> {
    Ok(vec![
        build_debug_layer_attach_report()?,
        build_debug_layer_non_admin_rejection_report()?,
        build_auth_layer_contract_update_report()?,
        build_declared_ratelimit_report()?,
        build_incompatible_patch_rejection_report()?,
    ])
}

fn build_report(
    shell: &LogicalPlaceRuntimeShell,
    policy: AttachPointPolicy,
    request: LayerAttachRequest,
    accepted_trace_rows: Option<Vec<LayerTraceRow>>,
    runtime_preview: Option<LayerRuntimePreview>,
    source_runtime_sample_ref: Option<String>,
) -> Result<LayerInsertionReport, MirroreaCoreError> {
    policy.validate()?;
    request.validate()?;

    let runtime_snapshot = shell.snapshot();
    let member = runtime_snapshot
        .membership
        .members
        .get(&request.principal_claim.principal)
        .ok_or_else(|| {
            MirroreaCoreError::new(format!(
                "layer insertion runtime requires active membership for requesting principal {}",
                request.principal_claim.principal
            ))
        })?;
    if !member.active {
        return Err(MirroreaCoreError::new(format!(
            "layer insertion runtime requires requesting principal {} to be active",
            request.principal_claim.principal
        )));
    }
    if member.place != request.principal_claim.participant_place {
        return Err(MirroreaCoreError::new(
            "layer insertion runtime principal claim place drifted from runtime snapshot",
        ));
    }
    if runtime_snapshot
        .place_catalog
        .places
        .get(&policy.attachpoint_ref)
        .map(String::as_str)
        != Some(ATTACHPOINT_KIND)
    {
        return Err(MirroreaCoreError::new(
            "layer insertion runtime requires a registered AttachPoint place",
        ));
    }

    let compatibility = evaluate_compatibility(
        &policy,
        &request.requested_layer,
        request.contract_update_ref.as_deref(),
    );
    let attach_authorized = request
        .capability_refs
        .contains(&request.requested_layer.attach_capability);

    let authorization_reason_refs = if attach_authorized {
        vec![
            "attach_capability_present".to_string(),
            "admin_attach_authority_confirmed".to_string(),
        ]
    } else {
        vec![
            "attach_capability_missing".to_string(),
            "activation_cut_not_opened".to_string(),
        ]
    };
    let membership_reason_refs = vec![
        "membership_frontier_verified".to_string(),
        "membership_epoch_current".to_string(),
    ];

    let accepted = compatibility.failed_reason_refs.is_empty() && attach_authorized;
    let terminal_outcome = if accepted && compatibility.accepted_path == "explicit_contract_update"
    {
        "accepted_contract_update".to_string()
    } else if accepted {
        "accepted".to_string()
    } else {
        "rejected".to_string()
    };

    let hotplug_request = HotPlugRequest {
        request_id: request.request_id.clone(),
        attachpoint_ref: request.attachpoint_ref.clone(),
        patch_ref: format!("LayerPatch[{}]", request.requested_layer.layer_name),
        operation_kind: "attach".to_string(),
        requesting_principal: request.principal_claim.principal.clone(),
        requesting_participant_place: request.principal_claim.participant_place.clone(),
        message_envelope_ref: format!("layer_attach_envelope#{}", request.sample_id),
        auth_evidence_ref: request.contract_update_ref.clone(),
        capability_refs: request.capability_refs.clone(),
        witness_refs: request.witness_refs.clone(),
        notes: vec![
            format!("contract_mode={}", compatibility.accepted_path),
            "verdict is computed from current attach-time compatibility checks".to_string(),
        ],
    };
    let hotplug_verdict = HotPlugVerdict {
        request_ref: hotplug_request.request_id.clone(),
        verdict_kind: if accepted {
            "accepted".to_string()
        } else {
            "rejected".to_string()
        },
        compatibility_reason_refs: verdict_compatibility_reasons(&compatibility),
        authorization_reason_refs,
        membership_freshness_reason_refs: membership_reason_refs,
        notes: vec![
            "verdict is not imported as a pre-admitted carrier in this floor".to_string(),
            "completed hot-plug lifecycle still remains later".to_string(),
        ],
    };
    let hotplug_runtime_report =
        assemble_hotplug_runtime_skeleton_report(shell, hotplug_request, hotplug_verdict)?;

    Ok(LayerInsertionReport {
        runtime_scope: "alpha_layer_insertion_runtime_stage_d_narrow".to_string(),
        sample_id: request.sample_id.clone(),
        layer_signature_lanes: layer_signature_lanes(),
        attach_request: request.clone(),
        attachpoint_policy: policy,
        runtime_snapshot,
        hotplug_runtime_report,
        compatibility,
        active_layers_before: Vec::new(),
        active_layers_after: if accepted {
            vec![request.requested_layer.layer_name.clone()]
        } else {
            Vec::new()
        },
        pre_attach_trace_rows: Vec::new(),
        post_attach_trace_rows: if accepted {
            accepted_trace_rows.unwrap_or_default()
        } else {
            Vec::new()
        },
        runtime_preview,
        source_runtime_sample_ref,
        terminal_outcome,
        retained_later_refs: retained_later_refs(),
        notes: vec![
            "non-public layer insertion runtime floor over local dispatch and hot-plug carrier substrate"
                .to_string(),
            "current cut proves attach authority, contract comparison, and trace visibility after accepted attach only"
                .to_string(),
            "does not claim completed lifecycle, detach runtime, rollback, durable migration, distributed ordering, parser integration, network transport, runtime package admission, avatar runtime, or final public layer attachment ABI"
                .to_string(),
        ],
    })
}

pub(crate) fn build_layer_insertion_report_over_default_attachpoint(
    request: LayerAttachRequest,
    accepted_trace_rows: Option<Vec<LayerTraceRow>>,
    runtime_preview: Option<LayerRuntimePreview>,
    source_runtime_sample_ref: Option<String>,
) -> Result<LayerInsertionReport, MirroreaCoreError> {
    let shell = bootstrap_shell()?;
    let policy = message_dispatch_attachpoint_policy();
    build_report(
        &shell,
        policy,
        request,
        accepted_trace_rows,
        runtime_preview,
        source_runtime_sample_ref,
    )
}

fn evaluate_compatibility(
    policy: &AttachPointPolicy,
    layer: &LayerContract,
    contract_update_ref: Option<&str>,
) -> LayerCompatibilityReport {
    let mut checks = Vec::new();
    let mut passed_reason_refs = Vec::new();
    let mut failed_reason_refs = Vec::new();

    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "layer_kind_allowed",
        policy.allowed_layer_kinds.contains(&layer.layer_kind),
        "layer_kind_allowed".to_string(),
        "attachpoint_disallows_layer_kind".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "attach_point_matches",
        layer.attach_point == policy.attach_point,
        "attach_point_matches_policy".to_string(),
        "attach_point_mismatch".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "input_contravariance",
        policy.input_type == layer.input_type,
        "input_type_contravariant_or_equal".to_string(),
        "input_type_contravariance_failed".to_string(),
        &["current floor uses exact type-name equality for attach-time comparison".to_string()],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "output_covariance",
        layer.output_type == policy.output_type,
        "output_type_covariant_or_equal".to_string(),
        "output_type_covariance_failed".to_string(),
        &["current floor uses exact type-name equality for attach-time comparison".to_string()],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "precondition_weakening",
        subset(&layer.preconditions, &policy.preconditions),
        "precondition_weakened_or_equal".to_string(),
        "precondition_strengthened".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "postcondition_strengthening",
        subset(&policy.postconditions, &layer.postconditions),
        "postcondition_strengthened_or_equal".to_string(),
        "postcondition_weakened".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "effect_row_containment",
        subset(&layer.effect_row, &policy.allowed_effect_row),
        "effect_row_within_declared_budget".to_string(),
        "effect_row_widened".to_string(),
        &[],
    );

    let failure_subset = subset(&layer.failure_row, &policy.allowed_failure_row);
    match layer.contract_mode {
        ContractMode::TransparentOverlay => {
            push_check(
                &mut checks,
                &mut passed_reason_refs,
                &mut failed_reason_refs,
                "failure_row_containment",
                failure_subset,
                "failure_row_within_declared_budget".to_string(),
                "failure_row_requires_contract_update".to_string(),
                &[],
            );
        }
        ContractMode::ExplicitContractUpdate => {
            let update_present = contract_update_ref.is_some();
            checks.push(LayerContractCheck {
                check_name: "failure_row_containment".to_string(),
                result: "skipped".to_string(),
                reason_refs: vec!["failure_row_change_guarded_by_contract_update".to_string()],
                notes: vec![
                    "transparent containment is not claimed for auth/current explicit-update rows"
                        .to_string(),
                ],
            });
            passed_reason_refs.push("failure_row_change_guarded_by_contract_update".to_string());
            push_check(
                &mut checks,
                &mut passed_reason_refs,
                &mut failed_reason_refs,
                "contract_update_activation_cut",
                update_present,
                "contract_update_activation_cut_present".to_string(),
                "contract_update_activation_cut_missing".to_string(),
                &[],
            );
        }
    }

    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "capability_monotonicity",
        subset(&layer.required_capabilities, &policy.required_capabilities),
        "call_capability_not_strengthened".to_string(),
        "call_capability_strengthened".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "provided_capabilities_preserved",
        subset(&policy.provided_capabilities, &layer.provided_capabilities),
        "provided_capabilities_preserved".to_string(),
        "provided_capabilities_shrunk".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "provided_interface_preserved",
        subset(&policy.provided_interface, &layer.provided_interface),
        "provided_interface_preserved".to_string(),
        "provided_interface_shrunk".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "observation_label_containment",
        subset(
            &layer.observation_labels,
            &policy.allowed_observation_labels,
        ),
        "observation_labels_contained".to_string(),
        "observation_labels_widened".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "redaction_monotonicity",
        layer.redaction_level >= policy.minimum_redaction_level,
        "redaction_monotone".to_string(),
        "redaction_weakened".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "retention_scope_monotonicity",
        policy
            .allowed_retention_scopes
            .contains(&layer.retention_scope),
        "retention_scope_allowed".to_string(),
        "retention_scope_widened".to_string(),
        &[],
    );
    push_check(
        &mut checks,
        &mut passed_reason_refs,
        &mut failed_reason_refs,
        "trace_mode_policy",
        policy.allowed_trace_modes.contains(&layer.trace_mode),
        "trace_mode_allowed".to_string(),
        "trace_mode_would_overobserve".to_string(),
        &[],
    );

    LayerCompatibilityReport {
        contract_mode: layer.contract_mode,
        accepted_path: if failed_reason_refs.is_empty() {
            match layer.contract_mode {
                ContractMode::TransparentOverlay => "transparent_overlay".to_string(),
                ContractMode::ExplicitContractUpdate => "explicit_contract_update".to_string(),
            }
        } else {
            "rejected".to_string()
        },
        checks,
        passed_reason_refs,
        failed_reason_refs,
        notes: vec![
            "current floor compares exact type names plus finite set containment over declared rows"
                .to_string(),
            "full theorem-backed implication discharge and final contract algebra remain later".to_string(),
        ],
    }
}

fn push_check(
    checks: &mut Vec<LayerContractCheck>,
    passed_reason_refs: &mut Vec<String>,
    failed_reason_refs: &mut Vec<String>,
    check_name: &str,
    passed: bool,
    pass_reason: String,
    fail_reason: String,
    notes: &[String],
) {
    let reason_ref = if passed {
        pass_reason.clone()
    } else {
        fail_reason.clone()
    };
    checks.push(LayerContractCheck {
        check_name: check_name.to_string(),
        result: if passed {
            "passed".to_string()
        } else {
            "failed".to_string()
        },
        reason_refs: vec![reason_ref.clone()],
        notes: notes.to_vec(),
    });
    if passed {
        passed_reason_refs.push(pass_reason);
    } else {
        failed_reason_refs.push(fail_reason);
    }
}

fn verdict_compatibility_reasons(report: &LayerCompatibilityReport) -> Vec<String> {
    if report.failed_reason_refs.is_empty() {
        report
            .passed_reason_refs
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        report.failed_reason_refs.clone()
    }
}

fn bootstrap_shell() -> Result<LogicalPlaceRuntimeShell, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place(WORLD_PLACE, WORLD_KIND)?;
    shell.register_place(GAME_PLACE, GAME_KIND)?;
    shell.register_place(ATTACHPOINT_REF, ATTACHPOINT_KIND)?;
    shell.add_initial_participant("Alice")?;
    shell.add_initial_participant("Bob")?;
    shell.add_initial_participant(ADMIN_PRINCIPAL)?;
    shell.add_initial_participant(NON_ADMIN_PRINCIPAL)?;
    Ok(shell)
}

fn message_dispatch_attachpoint_policy() -> AttachPointPolicy {
    AttachPointPolicy {
        attachpoint_ref: ATTACHPOINT_REF.to_string(),
        attachpoint_kind: ATTACHPOINT_KIND.to_string(),
        attach_point: AttachPoint::MessageDispatch,
        allowed_layer_kinds: vec![LayerKind::Debug, LayerKind::Auth, LayerKind::RateLimit],
        base_contract_name: "sugoroku_dispatch_base".to_string(),
        input_type: "SugorokuDispatchEnvelope".to_string(),
        output_type: "SugorokuDispatchResult".to_string(),
        preconditions: collect(&[
            "membership_current",
            "dice_owner_authority",
            "handoff_target_active",
        ]),
        postconditions: collect(&[
            "dispatch_result_visible_to_room",
            "witness_trace_explicit",
            "handoff_visible_after_publication",
        ]),
        allowed_effect_row: collect(&[
            "emit_typed_debug_trace",
            "emit_rate_limit_counter",
            "emit_auth_audit",
        ]),
        allowed_failure_row: collect(&["StaleMembership", "MissingWitness", "RateLimited"]),
        required_capabilities: collect(&["RollDice", "PublishRoll", "HandoffTurn"]),
        provided_capabilities: collect(&["RoomDispatch", "WitnessTrace"]),
        provided_interface: collect(&["roll", "publish", "handoff"]),
        allowed_observation_labels: collect(&[
            "room.dispatch.summary",
            "room.debug.trace.redacted",
        ]),
        minimum_redaction_level: RedactionLevel::SubjectAndPayloadRedacted,
        allowed_retention_scopes: collect(&["helper_local_ephemeral"]),
        allowed_trace_modes: vec![TraceMode::OnDemand],
        notes: collect(&[
            "attachpoint policy is runtime-private and non-public",
            "effect/failure/observation budgets are declared before attach",
        ]),
    }
}

fn layer_attach_request(
    sample_id: &str,
    principal: &str,
    capability_refs: &[&str],
    layer_signature: LayerSignature,
    requested_layer: LayerContract,
    contract_update_ref: Option<&str>,
    notes: &[&str],
) -> Result<LayerAttachRequest, MirroreaCoreError> {
    let participant_place = format!("ParticipantPlace[{principal}]");
    let principal_claim = PrincipalClaim {
        principal: principal.to_string(),
        participant_place,
        claimed_authority: "AttachPointAdministrator".to_string(),
        claimed_capabilities: collect(capability_refs),
    };
    principal_claim.validate()?;

    Ok(LayerAttachRequest {
        request_id: format!("layer_attach_request#{sample_id}"),
        sample_id: sample_id.to_string(),
        attachpoint_ref: ATTACHPOINT_REF.to_string(),
        principal_claim,
        capability_refs: collect(capability_refs),
        membership_epoch: 0,
        member_incarnation: 0,
        witness_refs: collect(&["membership_frontier_snapshot#layer_insertion"]),
        layer_signature,
        requested_layer,
        contract_update_ref: contract_update_ref.map(str::to_string),
        notes: collect(notes),
    })
}

pub(crate) fn debug_layer_contract() -> LayerContract {
    LayerContract {
        layer_name: "debug_trace_layer".to_string(),
        layer_kind: LayerKind::Debug,
        attach_point: AttachPoint::MessageDispatch,
        contract_mode: ContractMode::TransparentOverlay,
        input_type: "SugorokuDispatchEnvelope".to_string(),
        output_type: "SugorokuDispatchResult".to_string(),
        preconditions: collect(&[
            "membership_current",
            "dice_owner_authority",
            "handoff_target_active",
        ]),
        postconditions: collect(&[
            "dispatch_result_visible_to_room",
            "witness_trace_explicit",
            "handoff_visible_after_publication",
            "redacted_trace_available_after_attach",
        ]),
        effect_row: collect(&["emit_typed_debug_trace"]),
        failure_row: Vec::new(),
        required_capabilities: collect(&["RollDice", "PublishRoll", "HandoffTurn"]),
        provided_capabilities: collect(&["RoomDispatch", "WitnessTrace", "DebugTraceView"]),
        provided_interface: collect(&["roll", "publish", "handoff", "debug_trace"]),
        observation_labels: collect(&["room.debug.trace.redacted"]),
        redaction_level: RedactionLevel::SubjectAndPayloadRedacted,
        retention_scope: "helper_local_ephemeral".to_string(),
        trace_mode: TraceMode::OnDemand,
        attach_capability: "AttachDebugLayer(AlphaRoom#1)".to_string(),
        notes: collect(&[
            "trace appears only after accepted attach",
            "subject/payload stay redacted in this current cut",
        ]),
    }
}

pub(crate) fn auth_layer_contract() -> LayerContract {
    LayerContract {
        layer_name: "auth_gate_layer".to_string(),
        layer_kind: LayerKind::Auth,
        attach_point: AttachPoint::MessageDispatch,
        contract_mode: ContractMode::ExplicitContractUpdate,
        input_type: "SugorokuDispatchEnvelope".to_string(),
        output_type: "SugorokuDispatchResult".to_string(),
        preconditions: collect(&[
            "membership_current",
            "dice_owner_authority",
            "handoff_target_active",
        ]),
        postconditions: collect(&[
            "dispatch_result_visible_to_room",
            "witness_trace_explicit",
            "handoff_visible_after_publication",
        ]),
        effect_row: collect(&["emit_auth_audit"]),
        failure_row: collect(&["AuthFailed"]),
        required_capabilities: collect(&["RollDice", "PublishRoll", "HandoffTurn"]),
        provided_capabilities: collect(&["RoomDispatch", "WitnessTrace"]),
        provided_interface: collect(&["roll", "publish", "handoff"]),
        observation_labels: collect(&["room.dispatch.summary"]),
        redaction_level: RedactionLevel::SubjectAndPayloadRedacted,
        retention_scope: "helper_local_ephemeral".to_string(),
        trace_mode: TraceMode::OnDemand,
        attach_capability: "AttachAuthLayer(AlphaRoom#1)".to_string(),
        notes: collect(&[
            "auth row changes failure behavior and therefore uses explicit contract update",
            "transparent overlay is intentionally not claimed",
        ]),
    }
}

pub(crate) fn rate_limit_layer_contract() -> LayerContract {
    LayerContract {
        layer_name: "rate_limit_layer".to_string(),
        layer_kind: LayerKind::RateLimit,
        attach_point: AttachPoint::MessageDispatch,
        contract_mode: ContractMode::TransparentOverlay,
        input_type: "SugorokuDispatchEnvelope".to_string(),
        output_type: "SugorokuDispatchResult".to_string(),
        preconditions: collect(&[
            "membership_current",
            "dice_owner_authority",
            "handoff_target_active",
        ]),
        postconditions: collect(&[
            "dispatch_result_visible_to_room",
            "witness_trace_explicit",
            "handoff_visible_after_publication",
        ]),
        effect_row: collect(&["emit_rate_limit_counter"]),
        failure_row: collect(&["RateLimited"]),
        required_capabilities: collect(&["RollDice", "PublishRoll", "HandoffTurn"]),
        provided_capabilities: collect(&["RoomDispatch", "WitnessTrace"]),
        provided_interface: collect(&["roll", "publish", "handoff"]),
        observation_labels: collect(&["room.dispatch.summary"]),
        redaction_level: RedactionLevel::SubjectAndPayloadRedacted,
        retention_scope: "helper_local_ephemeral".to_string(),
        trace_mode: TraceMode::OnDemand,
        attach_capability: "AttachRateLimitLayer(AlphaRoom#1)".to_string(),
        notes: collect(&[
            "RateLimited is already declared in the base allowed failure budget",
            "runtime accounting remains preview-only in this cut",
        ]),
    }
}

pub(crate) fn incompatible_patch_contract() -> LayerContract {
    LayerContract {
        layer_name: "unsafe_debug_shadow_layer".to_string(),
        layer_kind: LayerKind::Debug,
        attach_point: AttachPoint::MessageDispatch,
        contract_mode: ContractMode::TransparentOverlay,
        input_type: "SugorokuDispatchEnvelope".to_string(),
        output_type: "SugorokuDispatchResult".to_string(),
        preconditions: collect(&[
            "membership_current",
            "dice_owner_authority",
            "handoff_target_active",
            "debug_subscriber_present",
        ]),
        postconditions: collect(&["dispatch_result_visible_to_room"]),
        effect_row: collect(&["emit_typed_debug_trace"]),
        failure_row: Vec::new(),
        required_capabilities: collect(&[
            "RollDice",
            "PublishRoll",
            "HandoffTurn",
            "AdminDebugSession",
        ]),
        provided_capabilities: collect(&["DebugTraceView"]),
        provided_interface: collect(&["debug_trace"]),
        observation_labels: collect(&["room.debug.trace.full"]),
        redaction_level: RedactionLevel::None,
        retention_scope: "session_cache".to_string(),
        trace_mode: TraceMode::AlwaysOn,
        attach_capability: "AttachDebugLayer(AlphaRoom#1)".to_string(),
        notes: collect(&[
            "this patch intentionally violates transparent-overlay compatibility rules",
            "used only for LI-05 negative evidence",
        ]),
    }
}

pub(crate) fn debug_layer_signature() -> LayerSignature {
    layer_signature(
        "debug_trace_layer",
        &["ManageAttachPoint(AlphaRoom#1::MessageDispatch)"],
        &["DebugTraceView", "RedactedDispatchSummary"],
        &["dispatch_history -> redacted_debug_trace"],
        &[
            "attach_authority",
            "observation_label_subset",
            "redaction_monotone",
            "retention_scope_allowed",
        ],
        &["typed_debug_trace"],
        &["no_observation_leak", "trace_only_after_attach"],
        &["transparent_overlay_preserves_dispatch_surface"],
    )
}

pub(crate) fn auth_layer_signature() -> LayerSignature {
    layer_signature(
        "auth_gate_layer",
        &["ManageAttachPoint(AlphaRoom#1::MessageDispatch)"],
        &["AuthAuditRow"],
        &["dispatch_guard -> contract_update_activation_cut"],
        &["activation_cut_required", "membership_freshness"],
        &["auth_audit"],
        &["signature_proves_provenance_only", "no_hidden_auth_upgrade"],
        &["explicit_contract_update_required"],
    )
}

pub(crate) fn rate_limit_layer_signature() -> LayerSignature {
    layer_signature(
        "rate_limit_layer",
        &["ManageAttachPoint(AlphaRoom#1::MessageDispatch)"],
        &["RateLimitCounter"],
        &["dispatch_admission -> rate_limit_gate"],
        &["failure_row_declared", "counter_visibility_redacted"],
        &["rate_limit_counter"],
        &["no_hidden_failure_widening"],
        &["transparent_only_if_ratelimited_declared"],
    )
}

pub(crate) fn incompatible_patch_signature() -> LayerSignature {
    layer_signature(
        "unsafe_debug_shadow_layer",
        &["ManageAttachPoint(AlphaRoom#1::MessageDispatch)"],
        &["DebugTraceView"],
        &["dispatch_history -> always_on_unredacted_trace"],
        &["shadowing_detected", "observer_widening_detected"],
        &["unsafe_debug_trace"],
        &["must_reject_before_activation"],
        &["no_silent_api_shadowing"],
    )
}

fn layer_signature(
    name: &str,
    requires: &[&str],
    provides: &[&str],
    transforms: &[&str],
    checks: &[&str],
    emits: &[&str],
    obligations: &[&str],
    laws: &[&str],
) -> LayerSignature {
    let layer = LayerSignature {
        name: name.to_string(),
        requires: collect(requires),
        provides: collect(provides),
        transforms: collect(transforms),
        checks: collect(checks),
        emits: collect(emits),
        obligations: collect(obligations),
        laws: collect(laws),
    };
    layer
        .validate()
        .expect("alpha layer insertion signatures should satisfy mirrorea-core invariants");
    layer
}

fn debug_trace_rows() -> Result<Vec<LayerTraceRow>, MirroreaCoreError> {
    let local_runtime = build_local_sugoroku_runtime_report()?;
    Ok(local_runtime
        .visible_history
        .iter()
        .enumerate()
        .map(|(index, entry)| LayerTraceRow {
            row_id: format!("debug_trace#{}", index + 1),
            label: "room.debug.trace.redacted".to_string(),
            authority: "SpaceOperator".to_string(),
            redaction_level: RedactionLevel::SubjectAndPayloadRedacted,
            retention_scope: "helper_local_ephemeral".to_string(),
            summary: redact_trace_summary(entry),
            source_ref: format!("{}:{}", local_runtime.sample_id, index + 1),
        })
        .collect())
}

fn redact_trace_summary(entry: &str) -> String {
    if entry.starts_with("publish roll_result") {
        "publish roll_result(subject:redacted, payload:redacted) witness=draw_pub#1".to_string()
    } else if entry.starts_with("handoff") {
        "handoff dice_owner subject:redacted -> subject:redacted using witness=draw_pub#1"
            .to_string()
    } else {
        format!("redacted_trace({entry})")
    }
}

fn retained_later_refs() -> Vec<String> {
    collect(&[
        "completed_hotplug_lifecycle",
        "detach_runtime",
        "rollback_protocol",
        "durable_migration",
        "distributed_activation_ordering",
        "network_docker_runtime",
        "distributed_save_load",
        "runtime_package_avatar_admission",
        "final_public_layer_attachment_abi",
    ])
}

fn collect(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn subset(lhs: &[String], rhs: &[String]) -> bool {
    let lhs = lhs.iter().cloned().collect::<BTreeSet<_>>();
    let rhs = rhs.iter().cloned().collect::<BTreeSet<_>>();
    lhs.is_subset(&rhs)
}

fn require_non_empty(entity: &str, field: &str, value: &str) -> Result<(), MirroreaCoreError> {
    if value.trim().is_empty() {
        return Err(MirroreaCoreError::new(format!(
            "{entity} field `{field}` must be non-empty"
        )));
    }
    Ok(())
}

fn require_non_empty_items<T: std::fmt::Debug>(
    entity: &str,
    field: &str,
    values: &[T],
) -> Result<(), MirroreaCoreError> {
    if values.is_empty() {
        return Err(MirroreaCoreError::new(format!(
            "{entity} field `{field}` must have at least one item"
        )));
    }
    Ok(())
}
