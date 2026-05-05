use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::product_alpha1::{
    ProductAlpha1CheckReport, ProductAlpha1Error, ProductAlpha1HostIoPayload,
    ProductAlpha1HostIoRuntimeInput, ProductAlpha1Package, check_product_alpha1_package_path,
    load_product_alpha1_package_path,
};
use mirrorea_core::{
    AuthEvidence, HotPlugRequest, HotPlugVerdict, LogicalPlaceRuntimeShell,
    LogicalPlaceRuntimeSnapshot, MessageEnvelope, MirroreaCoreError, PrincipalClaim,
};
use serde::{Deserialize, Serialize};

pub const PRODUCT_ALPHA1_RUNTIME_PLAN_SCOPE: &str = "product-alpha1-runtime-plan-v0";
pub const PRODUCT_ALPHA1_SESSION_SCOPE: &str = "product-alpha1-same-session-runtime-v0";
pub const PRODUCT_ALPHA1_RUN_LOCAL_SURFACE_KIND: &str = "product_alpha1_run_local_report";
pub const PRODUCT_ALPHA1_ATTACH_SURFACE_KIND: &str = "product_alpha1_attach_report";
pub const PRODUCT_ALPHA1_SAVE_SURFACE_KIND: &str = "product_alpha1_save_report";
pub const PRODUCT_ALPHA1_LOAD_SURFACE_KIND: &str = "product_alpha1_load_report";
pub const PRODUCT_ALPHA1_QUIESCENT_SAVE_SURFACE_KIND: &str = "product_alpha1_quiescent_save_report";
pub const PRODUCT_ALPHA1_SESSION_SURFACE_KIND: &str = "product_alpha1_session_carrier";

const PRODUCT_ALPHA1_RUNTIME_STOP_LINES: &[&str] = &[
    "product alpha-1 same-session runtime is alpha-stable only, not a final public runtime ABI",
    "local session store is same-process persistence, not product transport command behavior or WAN/federation",
    "local R0 save/load and bounded R2 quiescent-save are alpha first cuts only, not durable distributed save/load",
    "message recovery rows cover bounded timeout/retry/reject observations only, not WAN or arbitrary crash recovery",
    "native execution remains disabled; native output is a host launch bundle, not package-native execution",
];

const PRODUCT_ALPHA1_RUNTIME_LIMITATIONS: &[&str] = &[
    "controlled local product alpha-1 session carrier only",
    "one deterministic product demo runtime path with typed host-I/O add-one evidence",
    "debug/auth/rate-limit layer attach is same-session and observable; object/avatar-preview attach remains deferred boundary evidence",
    "local save/load and quiescent-save are bounded to one local session store",
    "no distributed durable save/load, WAN federation, final viewer ABI, or arbitrary native package execution",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductAlpha1SessionErrorKind {
    FrontDoor,
    Checker,
    Core,
    HostIo,
    LoadAdmissibility,
    MissingSavepoint,
    Serialize,
    Transport,
    UnsupportedPackage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductAlpha1SessionError {
    pub kind: ProductAlpha1SessionErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for ProductAlpha1SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} at {}: {}",
            self.kind,
            self.path.display(),
            self.detail
        )
    }
}

impl std::error::Error for ProductAlpha1SessionError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RunLocalReport {
    #[serde(default = "run_local_surface_kind")]
    pub surface_kind: String,
    pub package_id: String,
    pub check_report: ProductAlpha1CheckReport,
    pub runtime_plan: ProductAlpha1RuntimePlan,
    pub session: ProductAlpha1SessionCarrier,
    pub runtime_plan_emitted: bool,
    pub local_transport_claimed: bool,
    pub local_session_store_claimed: bool,
    pub typed_host_io_claimed: bool,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RuntimePlan {
    #[serde(default = "runtime_plan_scope")]
    pub runtime_plan_scope: String,
    pub package_id: String,
    pub package_version: String,
    pub package_kind: String,
    pub entry_place: String,
    pub session_store_scope: String,
    pub place_graph: ProductAlpha1PlaceGraph,
    pub bootstrap_membership: Vec<String>,
    pub witness_requirements: Vec<String>,
    pub capability_requirements: Vec<String>,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
    pub message_recovery_policy: String,
    pub savepoint_classes: Vec<String>,
    pub quiescent_save_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PlaceGraph {
    pub nodes: Vec<ProductAlpha1PlaceNode>,
    pub edges: Vec<ProductAlpha1PlaceEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PlaceNode {
    pub place_id: String,
    pub place_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PlaceEdge {
    pub from_place: String,
    pub to_place: String,
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1SessionCarrier {
    #[serde(default = "session_surface_kind")]
    pub surface_kind: String,
    #[serde(default = "session_scope")]
    pub session_scope: String,
    pub session_id: String,
    pub phase: String,
    pub package_id: String,
    pub root_package_kind: String,
    pub checker_report_ref: String,
    pub runtime_plan_ref: String,
    pub runtime_plan: ProductAlpha1RuntimePlan,
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub event_dag: ProductAlpha1EventDag,
    pub membership: ProductAlpha1MembershipState,
    pub witness_state: ProductAlpha1WitnessState,
    pub route_graph: ProductAlpha1RouteGraph,
    pub active_layers: Vec<String>,
    pub hotplug_lifecycle: Vec<ProductAlpha1HotPlugLifecycleEntry>,
    pub host_io_history: Vec<ProductAlpha1HostIoEntry>,
    pub auth_state: ProductAlpha1AuthState,
    pub capability_state: ProductAlpha1CapabilityState,
    pub auth_decisions: Vec<ProductAlpha1AuthDecision>,
    pub capability_decisions: Vec<ProductAlpha1CapabilityDecision>,
    pub save_load_state: ProductAlpha1SaveLoadState,
    #[serde(default)]
    pub savepoints: Vec<ProductAlpha1SessionSavepoint>,
    pub message_recovery_state: ProductAlpha1MessageRecoveryState,
    pub observer_safe_export: ProductAlpha1ObserverSafeExport,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1EventDag {
    pub nodes: Vec<ProductAlpha1EventNode>,
    pub edges: Vec<ProductAlpha1EventEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1EventNode {
    pub event_id: String,
    pub event_kind: String,
    pub place_ref: String,
    pub envelope_ref: Option<String>,
    pub observer_safe_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1EventEdge {
    pub from_event: String,
    pub to_event: String,
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1MembershipState {
    pub frontier_id: String,
    pub membership_epoch: u64,
    pub active_members: Vec<String>,
    pub required_membership_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1WitnessState {
    pub frontier_id: String,
    pub witness_refs: Vec<String>,
    pub observer_safe_relation_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RouteGraph {
    pub routes: Vec<ProductAlpha1RouteEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RouteEntry {
    pub route_id: String,
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub transport_lane: String,
    pub auth_lane_preserved: bool,
    pub membership_lane_preserved: bool,
    pub witness_lane_preserved: bool,
    pub capability_lane_preserved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1HotPlugLifecycleEntry {
    pub sequence_id: usize,
    pub package_id: String,
    pub package_kind: String,
    pub operation_kind: String,
    pub terminal_outcome: String,
    pub request_event_id: String,
    pub verdict_event_id: String,
    pub activation_cut_ref: Option<String>,
    pub session_mutated: bool,
    pub active_runtime_mutated: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1HostIoEntry {
    pub adapter_kind: String,
    pub effect_ref: String,
    pub request_summary: String,
    pub response_summary: String,
    pub request_event_id: String,
    pub response_event_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1SaveLoadState {
    pub ordinary_save_ready: bool,
    pub quiescent_save_ready: bool,
    pub local_savepoint_refs: Vec<String>,
    pub devtools_export_refs: Vec<String>,
    pub declared_savepoint_classes: Vec<String>,
    pub required_quiescent_obligations: Vec<String>,
    #[serde(default = "quiescence_state_default")]
    pub quiescence_state: ProductAlpha1QuiescenceState,
    pub residual_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1SessionSavepoint {
    pub savepoint_id: String,
    pub savepoint_class: String,
    pub saved_package_id: String,
    pub saved_runtime_plan_ref: String,
    pub saved_state_format: String,
    pub saved_runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub saved_event_dag: ProductAlpha1EventDag,
    pub saved_membership: ProductAlpha1MembershipState,
    pub saved_witness_state: ProductAlpha1WitnessState,
    pub saved_route_graph: ProductAlpha1RouteGraph,
    pub saved_active_layers: Vec<String>,
    pub saved_hotplug_lifecycle: Vec<ProductAlpha1HotPlugLifecycleEntry>,
    pub saved_host_io_history: Vec<ProductAlpha1HostIoEntry>,
    pub saved_auth_state: ProductAlpha1AuthState,
    pub saved_capability_state: ProductAlpha1CapabilityState,
    pub saved_auth_decisions: Vec<ProductAlpha1AuthDecision>,
    pub saved_capability_decisions: Vec<ProductAlpha1CapabilityDecision>,
    pub saved_save_load_state: ProductAlpha1SaveLoadState,
    pub saved_message_recovery_state: ProductAlpha1MessageRecoveryState,
    pub state_roundtrip_equal: bool,
    pub no_inflight: bool,
    pub all_places_sealed: bool,
    pub no_post_cut_send: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1MessageRecoveryState {
    pub message_state_lane: Vec<ProductAlpha1MessageStateRecord>,
    pub handled_failures: Vec<String>,
    pub recovery_policy: String,
    #[serde(default)]
    pub transport_contracts: Vec<ProductAlpha1TransportContract>,
    #[serde(default)]
    pub recovery_policies: Vec<ProductAlpha1RecoveryPolicyRecord>,
    #[serde(default)]
    pub failure_observations: Vec<ProductAlpha1FailureObservation>,
    pub modal_obligations: Vec<String>,
    pub runtime_recovery_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1TransportContract {
    pub contract_id: String,
    pub contract_kind: String,
    pub guarantees: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RecoveryPolicyRecord {
    pub policy_id: String,
    pub policy_kind: String,
    pub handled_failures: Vec<String>,
    pub max_retries: u32,
    pub terminal_action: String,
    pub modal_obligations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1FailureObservation {
    pub envelope_id: String,
    pub failure_class: String,
    pub initial_state: String,
    pub recovery_action: String,
    pub terminal_state: String,
    pub retry_count: u32,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1MessageStateRecord {
    pub envelope_id: String,
    pub state: String,
    pub failure_class: Option<String>,
    pub recovery_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1ObserverSafeExport {
    pub view_role: String,
    pub redaction_level: String,
    pub retention_scope: String,
    pub redacted_fields: Vec<String>,
    pub visible_event_ids: Vec<String>,
    pub visible_routes: Vec<String>,
    pub visible_hotplug_events: Vec<String>,
    pub visible_host_io_events: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1AttachReport {
    #[serde(default = "attach_surface_kind")]
    pub surface_kind: String,
    pub session_id: String,
    pub package_id: String,
    pub package_kind: String,
    pub terminal_outcome: String,
    pub session_mutated: bool,
    pub request_event_id: String,
    pub verdict_event_id: String,
    pub activation_cut_ref: Option<String>,
    pub auth_decision: ProductAlpha1AuthDecision,
    pub capability_decision: ProductAlpha1CapabilityDecision,
    pub active_layers_after: Vec<String>,
    pub active_runtime_mutated: bool,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1SaveReport {
    #[serde(default = "save_surface_kind")]
    pub surface_kind: String,
    pub session_id: String,
    pub savepoint_id: String,
    pub savepoint_class: String,
    pub terminal_outcome: String,
    pub state_roundtrip_equal: bool,
    pub ordinary_save_ready: bool,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1LoadReport {
    #[serde(default = "load_surface_kind")]
    pub surface_kind: String,
    pub session_id: String,
    pub savepoint_id: String,
    pub terminal_outcome: String,
    pub loaded_savepoint_class: String,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1QuiescentSaveReport {
    #[serde(default = "quiescent_save_surface_kind")]
    pub surface_kind: String,
    pub session_id: String,
    pub savepoint_id: String,
    pub savepoint_class: String,
    pub terminal_outcome: String,
    pub state_roundtrip_equal: bool,
    pub no_inflight: bool,
    pub all_places_sealed: bool,
    pub no_post_cut_send: bool,
    pub drained_messages: Vec<String>,
    pub failed_messages: Vec<String>,
    pub rejected_post_cut_sends: Vec<ProductAlpha1PostCutSendRecord>,
    pub non_claims: Vec<String>,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PostCutSendRecord {
    pub savepoint_id: String,
    pub envelope_id: String,
    pub attempted_after_cut: bool,
    pub outcome: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1QuiescenceState {
    pub seal_protocol_enabled: bool,
    pub post_cut_send_guard_enabled: bool,
    pub sealed_place_refs: Vec<String>,
    pub rejected_post_cut_sends: Vec<ProductAlpha1PostCutSendRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1AuthDecision {
    pub decision_id: String,
    pub decision: String,
    pub auth_stack: Vec<String>,
    pub membership_requirements: Vec<String>,
    pub capability_refs: Vec<String>,
    pub witness_refs: Vec<String>,
    pub contract_update_path: String,
    pub overlay_transparency_claimed: bool,
    pub reason_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1CapabilityDecision {
    pub decision_id: String,
    pub decision: String,
    pub requested_capabilities: Vec<String>,
    pub granted_capabilities: Vec<String>,
    pub missing_capabilities: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1AuthState {
    pub active_bindings: Vec<String>,
    pub active_membership_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1CapabilityState {
    pub granted_capabilities: Vec<String>,
}

pub fn run_product_alpha1_local_session_path(
    path: impl AsRef<Path>,
) -> Result<ProductAlpha1RunLocalReport, ProductAlpha1SessionError> {
    let path = path.as_ref().to_path_buf();
    let check_report = check_product_alpha1_package_path(&path)
        .map_err(|error| map_product_error(ProductAlpha1SessionErrorKind::Checker, error))?;
    let package = load_product_alpha1_package_path(&path)
        .map_err(|error| map_product_error(ProductAlpha1SessionErrorKind::FrontDoor, error))?;
    if package.package_kind != "world" {
        return Err(ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::UnsupportedPackage,
            path,
            detail: format!(
                "run-local requires a product alpha-1 world package, found `{}`",
                package.package_kind
            ),
        });
    }
    let runtime_plan = build_runtime_plan(&package);
    let session = build_run_local_session(&package, runtime_plan.clone())?;
    let typed_host_io_claimed = !session.host_io_history.is_empty();

    Ok(ProductAlpha1RunLocalReport {
        surface_kind: run_local_surface_kind(),
        package_id: package.package_id,
        check_report,
        runtime_plan,
        session,
        runtime_plan_emitted: true,
        local_transport_claimed: false,
        local_session_store_claimed: true,
        typed_host_io_claimed,
        product_alpha1_ready: false,
        final_public_api_frozen: false,
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    })
}

pub fn attach_product_alpha1_package_to_session_path(
    session: &ProductAlpha1SessionCarrier,
    package_path: impl AsRef<Path>,
) -> Result<(ProductAlpha1SessionCarrier, ProductAlpha1AttachReport), ProductAlpha1SessionError> {
    let path = package_path.as_ref().to_path_buf();
    check_product_alpha1_package_path(&path)
        .map_err(|error| map_product_error(ProductAlpha1SessionErrorKind::Checker, error))?;
    let package = load_product_alpha1_package_path(&path)
        .map_err(|error| map_product_error(ProductAlpha1SessionErrorKind::FrontDoor, error))?;

    if !matches!(
        package.package_kind.as_str(),
        "layer" | "object" | "avatar_preview" | "adapter"
    ) {
        return Err(ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::UnsupportedPackage,
            path,
            detail: format!(
                "attach requires layer/object/avatar_preview/adapter package, found `{}`",
                package.package_kind
            ),
        });
    }

    let sequence_id = session.hotplug_lifecycle.len() + 1;
    let request_event_id = format!("product_hotplug_request#{sequence_id}");
    let verdict_event_id = format!("product_hotplug_verdict#{sequence_id}");
    let auth_decision = evaluate_auth_decision(session, &package, &request_event_id);
    let capability_decision = evaluate_capability_decision(session, &package, &request_event_id);
    let authorized =
        auth_decision.decision == "accepted" && capability_decision.decision == "accepted";
    let terminal_outcome = if !authorized {
        "rejected"
    } else if package.package_kind == "layer" {
        "accepted"
    } else {
        "deferred"
    }
    .to_string();
    let active_runtime_mutated = terminal_outcome == "accepted";
    let session_mutated = true;
    let mut next = session.clone();
    next.phase = if active_runtime_mutated {
        "attached".to_string()
    } else {
        "attach_observed".to_string()
    };
    if active_runtime_mutated && !next.active_layers.contains(&package.package_id) {
        next.active_layers.push(package.package_id.clone());
    }

    let activation_cut_ref =
        active_runtime_mutated.then(|| format!("activation_cut#{}", package.package_id));
    append_attach_events(
        &mut next,
        &package,
        &request_event_id,
        &verdict_event_id,
        &terminal_outcome,
        activation_cut_ref.as_deref(),
    );
    append_attach_route(&mut next, &package);
    append_attach_message_state(&mut next, &package, &terminal_outcome);
    next.hotplug_lifecycle
        .push(ProductAlpha1HotPlugLifecycleEntry {
            sequence_id,
            package_id: package.package_id.clone(),
            package_kind: package.package_kind.clone(),
            operation_kind: "attach".to_string(),
            terminal_outcome: terminal_outcome.clone(),
            request_event_id: request_event_id.clone(),
            verdict_event_id: verdict_event_id.clone(),
            activation_cut_ref: activation_cut_ref.clone(),
            session_mutated,
            active_runtime_mutated,
            notes: vec![
                "auth/capability/membership/witness lanes stay explicit on product alpha-1 attach"
                    .to_string(),
                "layer attach is not modeled as a transparent overlay".to_string(),
            ],
        });
    next.auth_decisions.push(auth_decision.clone());
    next.capability_decisions.push(capability_decision.clone());
    next.observer_safe_export = build_observer_safe_export(&next, &package);

    let hotplug_request = HotPlugRequest {
        request_id: request_event_id.clone(),
        attachpoint_ref: "AttachPoint[ProductDemoRoom::Layers]".to_string(),
        patch_ref: package.package_id.clone(),
        operation_kind: "attach".to_string(),
        requesting_principal: "active_admin_participant".to_string(),
        requesting_participant_place: "ParticipantPlace[active_admin_participant]".to_string(),
        message_envelope_ref: format!("envelope#attach#{}", package.package_id),
        auth_evidence_ref: Some(format!("auth_evidence#{}", package.auth_policy.policy_id)),
        capability_refs: package.capabilities.clone(),
        witness_refs: non_empty_witness_refs(&package),
        notes: vec!["product alpha-1 attach request keeps fabric lanes explicit".to_string()],
    };
    hotplug_request
        .validate()
        .map_err(|error| map_core_error(Path::new("<inline>"), error))?;
    let hotplug_verdict = HotPlugVerdict {
        request_ref: request_event_id.clone(),
        verdict_kind: if terminal_outcome == "accepted" {
            "accepted"
        } else if terminal_outcome == "rejected" {
            "rejected"
        } else {
            "deferred"
        }
        .to_string(),
        compatibility_reason_refs: vec!["product_alpha1_schema_version".to_string()],
        authorization_reason_refs: vec!["auth_stack_declared".to_string()],
        membership_freshness_reason_refs: package.membership_requirements.clone(),
        witness_reason_refs: non_empty_witness_refs(&package),
        notes: vec!["product alpha-1 attach verdict is carrier-validated".to_string()],
    };
    hotplug_verdict
        .validate()
        .map_err(|error| map_core_error(Path::new("<inline>"), error))?;

    let attach_report = ProductAlpha1AttachReport {
        surface_kind: attach_surface_kind(),
        session_id: session.session_id.clone(),
        package_id: package.package_id,
        package_kind: package.package_kind,
        terminal_outcome,
        session_mutated,
        request_event_id,
        verdict_event_id,
        activation_cut_ref,
        auth_decision,
        capability_decision,
        active_layers_after: next.active_layers.clone(),
        active_runtime_mutated,
        product_alpha1_ready: false,
        final_public_api_frozen: false,
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    };

    Ok((next, attach_report))
}

pub fn save_product_alpha1_session(
    session: &ProductAlpha1SessionCarrier,
    savepoint_id: &str,
) -> Result<(ProductAlpha1SessionCarrier, ProductAlpha1SaveReport), ProductAlpha1SessionError> {
    let mut next = session.clone();
    next.phase = "saved".to_string();
    next.save_load_state.ordinary_save_ready = true;
    next.save_load_state.residual_reason =
        "P-A1-28 implements local R0 save/load and bounded R2 quiescent-save; R3/R4 remain non-goals"
            .to_string();
    push_ref_once(
        &mut next.save_load_state.local_savepoint_refs,
        savepoint_id.to_string(),
    );
    append_unique_save_load_event(
        &mut next,
        format!("event#save#{savepoint_id}"),
        "local_save",
        None,
        format!("local R0 savepoint {savepoint_id} emitted"),
    );
    refresh_observer_safe_export(&mut next);
    let savepoint = build_session_savepoint(
        &next,
        savepoint_id,
        "R0_Local",
        no_inflight_messages(&next).is_empty(),
        false,
        false,
    )?;
    upsert_savepoint(&mut next, savepoint.clone());

    let report = ProductAlpha1SaveReport {
        surface_kind: save_surface_kind(),
        session_id: session.session_id.clone(),
        savepoint_id: savepoint_id.to_string(),
        savepoint_class: savepoint.savepoint_class,
        terminal_outcome: "saved".to_string(),
        state_roundtrip_equal: savepoint.state_roundtrip_equal,
        ordinary_save_ready: true,
        product_alpha1_ready: false,
        final_public_api_frozen: false,
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    };
    Ok((next, report))
}

pub fn load_product_alpha1_session(
    session: &ProductAlpha1SessionCarrier,
    savepoint_id: &str,
) -> Result<(ProductAlpha1SessionCarrier, ProductAlpha1LoadReport), ProductAlpha1SessionError> {
    let resolved_savepoint_id = if savepoint_id == "latest" {
        session
            .save_load_state
            .local_savepoint_refs
            .last()
            .map(String::as_str)
            .unwrap_or(savepoint_id)
    } else {
        savepoint_id
    };
    let savepoint = session
        .savepoints
        .iter()
        .find(|candidate| candidate.savepoint_id == resolved_savepoint_id)
        .cloned()
        .ok_or_else(|| ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::MissingSavepoint,
            path: PathBuf::from("<session.savepoints>"),
            detail: format!("product alpha-1 savepoint `{savepoint_id}` does not exist"),
        })?;
    validate_load_admissibility(session, &savepoint)?;

    let mut next = session.clone();
    next.phase = "loaded".to_string();
    next.runtime_snapshot = savepoint.saved_runtime_snapshot.clone();
    next.event_dag = savepoint.saved_event_dag.clone();
    next.membership = savepoint.saved_membership.clone();
    next.witness_state = savepoint.saved_witness_state.clone();
    next.route_graph = savepoint.saved_route_graph.clone();
    next.active_layers = savepoint.saved_active_layers.clone();
    next.hotplug_lifecycle = savepoint.saved_hotplug_lifecycle.clone();
    next.host_io_history = savepoint.saved_host_io_history.clone();
    next.auth_state = savepoint.saved_auth_state.clone();
    next.capability_state = savepoint.saved_capability_state.clone();
    next.auth_decisions = savepoint.saved_auth_decisions.clone();
    next.capability_decisions = savepoint.saved_capability_decisions.clone();
    next.message_recovery_state = savepoint.saved_message_recovery_state.clone();
    next.savepoints = session.savepoints.clone();
    next.save_load_state = savepoint.saved_save_load_state.clone();
    next.save_load_state.ordinary_save_ready = true;
    push_ref_once(
        &mut next.save_load_state.local_savepoint_refs,
        resolved_savepoint_id.to_string(),
    );
    append_unique_save_load_event(
        &mut next,
        format!("event#load#{resolved_savepoint_id}"),
        "local_load",
        None,
        format!("local savepoint {resolved_savepoint_id} restored"),
    );
    refresh_observer_safe_export(&mut next);

    let report = ProductAlpha1LoadReport {
        surface_kind: load_surface_kind(),
        session_id: session.session_id.clone(),
        savepoint_id: resolved_savepoint_id.to_string(),
        terminal_outcome: "loaded".to_string(),
        loaded_savepoint_class: savepoint.savepoint_class,
        product_alpha1_ready: false,
        final_public_api_frozen: false,
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    };
    Ok((next, report))
}

pub fn quiescent_save_product_alpha1_session(
    session: &ProductAlpha1SessionCarrier,
    savepoint_id: &str,
) -> Result<
    (
        ProductAlpha1SessionCarrier,
        ProductAlpha1QuiescentSaveReport,
    ),
    ProductAlpha1SessionError,
> {
    let failed_messages = no_inflight_messages(session);
    let no_inflight = failed_messages.is_empty();
    let r2_declared = session
        .save_load_state
        .declared_savepoint_classes
        .iter()
        .any(|class| class == "R2")
        && session
            .runtime_plan
            .savepoint_classes
            .iter()
            .any(|class| class == "R2")
        && session.runtime_plan.quiescent_save_requested;

    let mut next = session.clone();
    if r2_declared && next.save_load_state.quiescence_state.seal_protocol_enabled {
        for place in &next.runtime_plan.place_graph.nodes {
            push_ref_once(
                &mut next.save_load_state.quiescence_state.sealed_place_refs,
                place.place_id.clone(),
            );
        }
    }

    let post_cut_send_record = ProductAlpha1PostCutSendRecord {
        savepoint_id: savepoint_id.to_string(),
        envelope_id: format!("envelope#post-cut-send#{savepoint_id}"),
        attempted_after_cut: true,
        outcome: if r2_declared
            && next
                .save_load_state
                .quiescence_state
                .post_cut_send_guard_enabled
        {
            "rejected"
        } else {
            "accepted_without_guard"
        }
        .to_string(),
        reason: "NoPostCutSend sealed the local product session epoch".to_string(),
    };
    if post_cut_send_record.outcome == "rejected" {
        next.save_load_state
            .quiescence_state
            .rejected_post_cut_sends
            .push(post_cut_send_record.clone());
    }
    let expected_places: BTreeSet<String> = next
        .runtime_plan
        .place_graph
        .nodes
        .iter()
        .map(|node| node.place_id.clone())
        .collect();
    let sealed_places: BTreeSet<String> = next
        .save_load_state
        .quiescence_state
        .sealed_place_refs
        .iter()
        .cloned()
        .collect();
    let all_places_sealed = !expected_places.is_empty()
        && expected_places
            .iter()
            .all(|place| sealed_places.contains(place));
    let rejected_post_cut_sends: Vec<_> = next
        .save_load_state
        .quiescence_state
        .rejected_post_cut_sends
        .iter()
        .filter(|record| record.savepoint_id == savepoint_id)
        .cloned()
        .collect();
    let no_post_cut_send = rejected_post_cut_sends
        .iter()
        .any(|record| record.attempted_after_cut && record.outcome == "rejected");
    let terminal_outcome = if r2_declared && no_inflight && all_places_sealed && no_post_cut_send {
        "saved"
    } else {
        "rejected"
    };

    next.phase = if terminal_outcome == "saved" {
        "quiescent_saved".to_string()
    } else {
        "quiescent_save_rejected".to_string()
    };
    append_quiescent_protocol_events(
        &mut next,
        savepoint_id,
        terminal_outcome,
        &failed_messages,
        all_places_sealed,
        &post_cut_send_record,
    );

    let mut state_roundtrip_equal = false;
    if terminal_outcome == "saved" {
        next.save_load_state.ordinary_save_ready = true;
        next.save_load_state.quiescent_save_ready = true;
        next.save_load_state.residual_reason =
            "P-A1-28 implements bounded local R2 quiescent-save; durable distributed R3/R4 remain non-goals"
                .to_string();
        push_ref_once(
            &mut next.save_load_state.local_savepoint_refs,
            savepoint_id.to_string(),
        );
        let savepoint = build_session_savepoint(
            &next,
            savepoint_id,
            "R2_Quiescent",
            no_inflight,
            all_places_sealed,
            no_post_cut_send,
        )?;
        state_roundtrip_equal = savepoint.state_roundtrip_equal;
        upsert_savepoint(&mut next, savepoint);
    } else {
        next.save_load_state.quiescent_save_ready = false;
    }
    refresh_observer_safe_export(&mut next);

    let report = ProductAlpha1QuiescentSaveReport {
        surface_kind: quiescent_save_surface_kind(),
        session_id: session.session_id.clone(),
        savepoint_id: savepoint_id.to_string(),
        savepoint_class: "R2_Quiescent".to_string(),
        terminal_outcome: terminal_outcome.to_string(),
        state_roundtrip_equal,
        no_inflight,
        all_places_sealed,
        no_post_cut_send,
        drained_messages: Vec::new(),
        failed_messages,
        rejected_post_cut_sends,
        non_claims: vec![
            "not a durable distributed save/load implementation".to_string(),
            "not WAN/federation recovery".to_string(),
            "not arbitrary crash recovery or exactly-once transport".to_string(),
        ],
        product_alpha1_ready: false,
        final_public_api_frozen: false,
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    };
    Ok((next, report))
}

fn build_runtime_plan(package: &ProductAlpha1Package) -> ProductAlpha1RuntimePlan {
    ProductAlpha1RuntimePlan {
        runtime_plan_scope: runtime_plan_scope(),
        package_id: package.package_id.clone(),
        package_version: package.package_version.clone(),
        package_kind: package.package_kind.clone(),
        entry_place: package
            .runtime_input
            .entry_place
            .clone()
            .unwrap_or_else(|| "Place[ProductDemoRoom]".to_string()),
        session_store_scope: "local_process_session_store".to_string(),
        place_graph: ProductAlpha1PlaceGraph {
            nodes: vec![
                ProductAlpha1PlaceNode {
                    place_id: "Place[ProductDemoRoom]".to_string(),
                    place_kind: "shared_virtual_space".to_string(),
                },
                ProductAlpha1PlaceNode {
                    place_id: "Place[HostAdapter]".to_string(),
                    place_kind: "typed_external_host_boundary".to_string(),
                },
            ],
            edges: vec![ProductAlpha1PlaceEdge {
                from_place: "Place[ProductDemoRoom]".to_string(),
                to_place: "Place[HostAdapter]".to_string(),
                relation: "typed_host_io_adapter_route".to_string(),
            }],
        },
        bootstrap_membership: bootstrap_membership(package),
        witness_requirements: package.witness_requirements.clone(),
        capability_requirements: package.capabilities.clone(),
        effect_row: package.effects.clone(),
        failure_row: package.failures.clone(),
        message_recovery_policy: package.message_recovery_policy.recovery.clone(),
        savepoint_classes: package.savepoint_policy.classes.clone(),
        quiescent_save_requested: package.savepoint_policy.quiescent_required,
    }
}

fn build_run_local_session(
    package: &ProductAlpha1Package,
    runtime_plan: ProductAlpha1RuntimePlan,
) -> Result<ProductAlpha1SessionCarrier, ProductAlpha1SessionError> {
    let (runtime_snapshot, envelope) = build_core_runtime_snapshot(package, &runtime_plan)?;
    let host_io_history = build_host_io_history(package)?;
    let failure_observations = build_failure_observations(package);
    let event_dag =
        build_initial_event_dag(package, &envelope, &host_io_history, &failure_observations);
    let mut routes = vec![ProductAlpha1RouteEntry {
        route_id: "route#product-demo-local-1".to_string(),
        envelope_id: envelope.envelope_id.clone(),
        from_place: envelope.from_place.clone(),
        to_place: envelope.to_place.clone(),
        transport_lane: envelope.transport_seam.clone(),
        auth_lane_preserved: true,
        membership_lane_preserved: true,
        witness_lane_preserved: true,
        capability_lane_preserved: true,
    }];
    routes.extend(build_failure_routes(&envelope, &failure_observations));
    let route_graph = ProductAlpha1RouteGraph { routes };
    let membership = ProductAlpha1MembershipState {
        frontier_id: "membership_frontier#product-demo-local".to_string(),
        membership_epoch: runtime_snapshot.membership.membership_epoch,
        active_members: runtime_snapshot
            .membership
            .members
            .iter()
            .filter_map(|(principal, member)| member.active.then_some(principal.clone()))
            .collect(),
        required_membership_refs: package.membership_requirements.clone(),
    };
    let witness_state = ProductAlpha1WitnessState {
        frontier_id: "witness_frontier#product-demo-local".to_string(),
        witness_refs: non_empty_witness_refs(package),
        observer_safe_relation_refs: vec!["witness_relation#product-demo-local".to_string()],
    };
    let save_load_state = ProductAlpha1SaveLoadState {
        ordinary_save_ready: false,
        quiescent_save_ready: false,
        local_savepoint_refs: Vec::new(),
        devtools_export_refs: Vec::new(),
        declared_savepoint_classes: package.savepoint_policy.classes.clone(),
        required_quiescent_obligations: vec![
            "NoInFlight".to_string(),
            "AllPlacesSealed".to_string(),
            "NoPostCutSend".to_string(),
        ],
        quiescence_state: quiescence_state_default(),
        residual_reason:
            "P-A1-28 runtime can execute local R0 save/load and bounded R2 quiescent-save; durable distributed R3/R4 remain non-goals"
                .to_string(),
    };
    let mut message_state_lane = vec![ProductAlpha1MessageStateRecord {
        envelope_id: envelope.envelope_id.clone(),
        state: "Delivered".to_string(),
        failure_class: None,
        recovery_action: None,
    }];
    message_state_lane.extend(failure_observations.iter().map(|failure| {
        ProductAlpha1MessageStateRecord {
            envelope_id: failure.envelope_id.clone(),
            state: failure.terminal_state.clone(),
            failure_class: Some(failure.failure_class.clone()),
            recovery_action: Some(failure.recovery_action.clone()),
        }
    }));
    let message_recovery_state = ProductAlpha1MessageRecoveryState {
        message_state_lane,
        handled_failures: package.message_recovery_policy.handled_failures.clone(),
        recovery_policy: package.message_recovery_policy.recovery.clone(),
        transport_contracts: build_transport_contracts(package),
        recovery_policies: build_recovery_policies(package),
        failure_observations,
        modal_obligations: vec![
            "modal ○: timeout may advance to retry and then reject in the bounded local demo"
                .to_string(),
            "modal □: accepted local delivery cannot erase declared recovery obligations"
                .to_string(),
        ],
        runtime_recovery_claimed: true,
    };

    let mut session = ProductAlpha1SessionCarrier {
        surface_kind: session_surface_kind(),
        session_scope: session_scope(),
        session_id: format!("session#{}", package.package_id),
        phase: "run_local".to_string(),
        package_id: package.package_id.clone(),
        root_package_kind: package.package_kind.clone(),
        checker_report_ref: format!("checker_report#{}", package.package_id),
        runtime_plan_ref: format!("runtime_plan#{}", package.package_id),
        runtime_plan,
        runtime_snapshot,
        event_dag,
        membership,
        witness_state,
        route_graph,
        active_layers: Vec::new(),
        hotplug_lifecycle: Vec::new(),
        host_io_history,
        auth_state: build_auth_state(package),
        capability_state: build_capability_state(package),
        auth_decisions: vec![initial_auth_decision(package)],
        capability_decisions: vec![initial_capability_decision(package)],
        save_load_state,
        savepoints: Vec::new(),
        message_recovery_state,
        observer_safe_export: ProductAlpha1ObserverSafeExport {
            view_role: String::new(),
            redaction_level: String::new(),
            retention_scope: String::new(),
            redacted_fields: Vec::new(),
            visible_event_ids: Vec::new(),
            visible_routes: Vec::new(),
            visible_hotplug_events: Vec::new(),
            visible_host_io_events: Vec::new(),
            notes: Vec::new(),
        },
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        product_alpha1_ready: false,
        final_public_api_frozen: false,
    };
    session.observer_safe_export = build_observer_safe_export(&session, package);
    Ok(session)
}

fn build_session_savepoint(
    session: &ProductAlpha1SessionCarrier,
    savepoint_id: &str,
    savepoint_class: &str,
    no_inflight: bool,
    all_places_sealed: bool,
    no_post_cut_send: bool,
) -> Result<ProductAlpha1SessionSavepoint, ProductAlpha1SessionError> {
    let mut savepoint = ProductAlpha1SessionSavepoint {
        savepoint_id: savepoint_id.to_string(),
        savepoint_class: savepoint_class.to_string(),
        saved_package_id: session.package_id.clone(),
        saved_runtime_plan_ref: session.runtime_plan_ref.clone(),
        saved_state_format: "product_alpha1_local_session_frontier_json".to_string(),
        saved_runtime_snapshot: session.runtime_snapshot.clone(),
        saved_event_dag: session.event_dag.clone(),
        saved_membership: session.membership.clone(),
        saved_witness_state: session.witness_state.clone(),
        saved_route_graph: session.route_graph.clone(),
        saved_active_layers: session.active_layers.clone(),
        saved_hotplug_lifecycle: session.hotplug_lifecycle.clone(),
        saved_host_io_history: session.host_io_history.clone(),
        saved_auth_state: session.auth_state.clone(),
        saved_capability_state: session.capability_state.clone(),
        saved_auth_decisions: session.auth_decisions.clone(),
        saved_capability_decisions: session.capability_decisions.clone(),
        saved_save_load_state: session.save_load_state.clone(),
        saved_message_recovery_state: session.message_recovery_state.clone(),
        state_roundtrip_equal: true,
        no_inflight,
        all_places_sealed,
        no_post_cut_send,
        notes: vec![
            "savepoint is local/session scoped and non-durable".to_string(),
            "distributed durable save/load R3/R4 remains a product alpha non-goal".to_string(),
        ],
    };
    let serialized =
        serde_json::to_string_pretty(&savepoint).map_err(|error| ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::Serialize,
            path: PathBuf::from("<product-alpha1-savepoint>"),
            detail: format!("failed to serialize product alpha-1 savepoint: {error}"),
        })?;
    let restored: ProductAlpha1SessionSavepoint =
        serde_json::from_str(&serialized).map_err(|error| ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::Serialize,
            path: PathBuf::from("<product-alpha1-savepoint>"),
            detail: format!("failed to deserialize product alpha-1 savepoint: {error}"),
        })?;
    savepoint.state_roundtrip_equal = savepoint == restored;
    Ok(savepoint)
}

fn upsert_savepoint(
    session: &mut ProductAlpha1SessionCarrier,
    savepoint: ProductAlpha1SessionSavepoint,
) {
    if let Some(existing) = session
        .savepoints
        .iter_mut()
        .find(|candidate| candidate.savepoint_id == savepoint.savepoint_id)
    {
        *existing = savepoint;
    } else {
        session.savepoints.push(savepoint);
    }
}

fn validate_load_admissibility(
    session: &ProductAlpha1SessionCarrier,
    savepoint: &ProductAlpha1SessionSavepoint,
) -> Result<(), ProductAlpha1SessionError> {
    if savepoint.saved_package_id != session.package_id {
        return load_admissibility_error(format!(
            "savepoint package `{}` does not match current session package `{}`",
            savepoint.saved_package_id, session.package_id
        ));
    }
    if savepoint.saved_runtime_plan_ref != session.runtime_plan_ref {
        return load_admissibility_error(format!(
            "savepoint runtime plan `{}` does not match current session runtime plan `{}`",
            savepoint.saved_runtime_plan_ref, session.runtime_plan_ref
        ));
    }
    let current_activation_cuts = session
        .hotplug_lifecycle
        .iter()
        .filter(|entry| entry.activation_cut_ref.is_some())
        .count();
    let saved_activation_cuts = savepoint
        .saved_hotplug_lifecycle
        .iter()
        .filter(|entry| entry.activation_cut_ref.is_some())
        .count();
    if current_activation_cuts > saved_activation_cuts {
        return load_admissibility_error(format!(
            "load would rewind across accepted activation cuts: current={current_activation_cuts}, saved={saved_activation_cuts}"
        ));
    }
    if session.membership != savepoint.saved_membership {
        return load_admissibility_error(
            "load would resurrect stale membership frontier".to_string(),
        );
    }
    if session.witness_state != savepoint.saved_witness_state {
        return load_admissibility_error("load would resurrect stale witness frontier".to_string());
    }
    if session.auth_state != savepoint.saved_auth_state {
        return load_admissibility_error("load would resurrect stale auth state".to_string());
    }
    if session.capability_state != savepoint.saved_capability_state {
        return load_admissibility_error("load would resurrect stale capability state".to_string());
    }
    Ok(())
}

fn load_admissibility_error<T>(detail: String) -> Result<T, ProductAlpha1SessionError> {
    Err(ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::LoadAdmissibility,
        path: PathBuf::from("<session.savepoints>"),
        detail,
    })
}

fn push_ref_once(refs: &mut Vec<String>, value: String) {
    if !refs.contains(&value) {
        refs.push(value);
    }
}

fn no_inflight_messages(session: &ProductAlpha1SessionCarrier) -> Vec<String> {
    session
        .message_recovery_state
        .message_state_lane
        .iter()
        .filter(|record| {
            let state = record.state.replace('_', "").to_ascii_lowercase();
            state == "inflight"
        })
        .map(|record| record.envelope_id.clone())
        .collect()
}

fn append_save_load_event(
    session: &mut ProductAlpha1SessionCarrier,
    event_id: String,
    event_kind: &str,
    envelope_ref: Option<String>,
    observer_safe_summary: String,
) {
    if let Some(previous) = session.event_dag.nodes.last() {
        session.event_dag.edges.push(ProductAlpha1EventEdge {
            from_event: previous.event_id.clone(),
            to_event: event_id.clone(),
            relation: "same_session_save_load_order".to_string(),
        });
    }
    session.event_dag.nodes.push(ProductAlpha1EventNode {
        event_id,
        event_kind: event_kind.to_string(),
        place_ref: "Place[ProductDemoRoom]".to_string(),
        envelope_ref,
        observer_safe_summary,
    });
}

fn append_unique_save_load_event(
    session: &mut ProductAlpha1SessionCarrier,
    event_prefix: String,
    event_kind: &str,
    envelope_ref: Option<String>,
    observer_safe_summary: String,
) -> String {
    let event_id = unique_event_id(session, &event_prefix);
    append_save_load_event(
        session,
        event_id.clone(),
        event_kind,
        envelope_ref,
        observer_safe_summary,
    );
    event_id
}

fn unique_event_id(session: &ProductAlpha1SessionCarrier, event_prefix: &str) -> String {
    let mut counter = session.event_dag.nodes.len() + 1;
    loop {
        let event_id = format!("{event_prefix}#{counter}");
        if !session
            .event_dag
            .nodes
            .iter()
            .any(|node| node.event_id == event_id)
        {
            return event_id;
        }
        counter += 1;
    }
}

fn append_quiescent_protocol_events(
    session: &mut ProductAlpha1SessionCarrier,
    savepoint_id: &str,
    terminal_outcome: &str,
    failed_messages: &[String],
    all_places_sealed: bool,
    post_cut_send_record: &ProductAlpha1PostCutSendRecord,
) {
    append_unique_save_load_event(
        session,
        format!("event#begin-save#{savepoint_id}"),
        "begin_save",
        None,
        format!("begin bounded local quiescent save {savepoint_id}"),
    );
    append_unique_save_load_event(
        session,
        format!("event#seal-places#{savepoint_id}"),
        "seal_places",
        None,
        if all_places_sealed {
            "all product demo places entered sealed state for the save epoch".to_string()
        } else {
            "quiescent save preflight did not seal every product demo place".to_string()
        },
    );
    append_unique_save_load_event(
        session,
        format!("event#post-cut-send-rejected#{savepoint_id}"),
        if post_cut_send_record.outcome == "rejected" {
            "post_cut_send_rejected"
        } else {
            "post_cut_send_guard_missing"
        },
        Some(post_cut_send_record.envelope_id.clone()),
        if post_cut_send_record.outcome == "rejected" {
            "post-cut send rejected by NoPostCutSend".to_string()
        } else {
            "post-cut send guard was unavailable during quiescent-save preflight".to_string()
        },
    );
    append_unique_save_load_event(
        session,
        format!("event#quiescent-save#{savepoint_id}"),
        "quiescent_save",
        None,
        if terminal_outcome == "saved" {
            format!("R2 quiescent save {savepoint_id} emitted")
        } else {
            format!(
                "R2 quiescent save {savepoint_id} rejected with in-flight messages {:?}",
                failed_messages
            )
        },
    );
}

fn refresh_observer_safe_export(session: &mut ProductAlpha1SessionCarrier) {
    let existing = session.observer_safe_export.clone();
    session.observer_safe_export = ProductAlpha1ObserverSafeExport {
        view_role: if existing.view_role.is_empty() {
            "observer_safe".to_string()
        } else {
            existing.view_role
        },
        redaction_level: if existing.redaction_level.is_empty() {
            "observer_safe".to_string()
        } else {
            existing.redaction_level
        },
        retention_scope: if existing.retention_scope.is_empty() {
            "demo_session".to_string()
        } else {
            existing.retention_scope
        },
        redacted_fields: existing.redacted_fields,
        visible_event_ids: session
            .event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect(),
        visible_routes: session
            .route_graph
            .routes
            .iter()
            .map(|route| route.route_id.clone())
            .collect(),
        visible_hotplug_events: session
            .hotplug_lifecycle
            .iter()
            .map(|entry| format!("{}:{}", entry.package_id, entry.terminal_outcome))
            .collect(),
        visible_host_io_events: session
            .host_io_history
            .iter()
            .map(|entry| {
                format!(
                    "{}:{}->{}",
                    entry.adapter_kind, entry.request_summary, entry.response_summary
                )
            })
            .collect(),
        notes: existing.notes,
    };
}

fn build_initial_event_dag(
    package: &ProductAlpha1Package,
    envelope: &MessageEnvelope,
    host_io_history: &[ProductAlpha1HostIoEntry],
    failure_observations: &[ProductAlpha1FailureObservation],
) -> ProductAlpha1EventDag {
    let mut nodes = vec![
        ProductAlpha1EventNode {
            event_id: "event#session-started".to_string(),
            event_kind: "session_started".to_string(),
            place_ref: "Place[ProductDemoRoom]".to_string(),
            envelope_ref: None,
            observer_safe_summary: format!("started {}", package.package_id),
        },
        ProductAlpha1EventNode {
            event_id: "event#message-enqueued".to_string(),
            event_kind: "message_enqueued".to_string(),
            place_ref: envelope.from_place.clone(),
            envelope_ref: Some(envelope.envelope_id.clone()),
            observer_safe_summary: "local message envelope queued".to_string(),
        },
        ProductAlpha1EventNode {
            event_id: "event#message-delivered".to_string(),
            event_kind: "message_delivered".to_string(),
            place_ref: envelope.to_place.clone(),
            envelope_ref: Some(envelope.envelope_id.clone()),
            observer_safe_summary: "local message envelope delivered".to_string(),
        },
    ];
    let mut edges = vec![
        ProductAlpha1EventEdge {
            from_event: "event#session-started".to_string(),
            to_event: "event#message-enqueued".to_string(),
            relation: "same_session_order".to_string(),
        },
        ProductAlpha1EventEdge {
            from_event: "event#message-enqueued".to_string(),
            to_event: "event#message-delivered".to_string(),
            relation: "delivery_order".to_string(),
        },
    ];

    if let Some(host_io) = host_io_history.first() {
        nodes.push(ProductAlpha1EventNode {
            event_id: host_io.request_event_id.clone(),
            event_kind: "host_io_request".to_string(),
            place_ref: "Place[HostAdapter]".to_string(),
            envelope_ref: None,
            observer_safe_summary: host_io.request_summary.clone(),
        });
        nodes.push(ProductAlpha1EventNode {
            event_id: host_io.response_event_id.clone(),
            event_kind: "host_io_response".to_string(),
            place_ref: "Place[HostAdapter]".to_string(),
            envelope_ref: None,
            observer_safe_summary: host_io.response_summary.clone(),
        });
        edges.push(ProductAlpha1EventEdge {
            from_event: "event#message-delivered".to_string(),
            to_event: host_io.request_event_id.clone(),
            relation: "host_io_after_delivery".to_string(),
        });
        edges.push(ProductAlpha1EventEdge {
            from_event: host_io.request_event_id.clone(),
            to_event: host_io.response_event_id.clone(),
            relation: "typed_request_response_order".to_string(),
        });
    }

    let mut previous_event_id = nodes
        .last()
        .map(|node| node.event_id.clone())
        .unwrap_or_else(|| "event#session-started".to_string());
    for failure in failure_observations {
        let observed_event_id = format!("event#message-failure-observed#{}", failure.failure_class);
        let recovered_event_id = format!("event#message-recovery#{}", failure.failure_class);
        nodes.push(ProductAlpha1EventNode {
            event_id: observed_event_id.clone(),
            event_kind: "message_failure_observed".to_string(),
            place_ref: envelope.to_place.clone(),
            envelope_ref: Some(failure.envelope_id.clone()),
            observer_safe_summary: format!(
                "{} observed for bounded product alpha recovery",
                failure.failure_class
            ),
        });
        nodes.push(ProductAlpha1EventNode {
            event_id: recovered_event_id.clone(),
            event_kind: "message_recovery_transition".to_string(),
            place_ref: envelope.to_place.clone(),
            envelope_ref: Some(failure.envelope_id.clone()),
            observer_safe_summary: format!(
                "{} -> {}",
                failure.initial_state, failure.terminal_state
            ),
        });
        edges.push(ProductAlpha1EventEdge {
            from_event: previous_event_id,
            to_event: observed_event_id.clone(),
            relation: "bounded_failure_observation_order".to_string(),
        });
        edges.push(ProductAlpha1EventEdge {
            from_event: observed_event_id,
            to_event: recovered_event_id.clone(),
            relation: "bounded_recovery_order".to_string(),
        });
        previous_event_id = recovered_event_id;
    }

    ProductAlpha1EventDag { nodes, edges }
}

fn build_failure_routes(
    envelope: &MessageEnvelope,
    failure_observations: &[ProductAlpha1FailureObservation],
) -> Vec<ProductAlpha1RouteEntry> {
    failure_observations
        .iter()
        .enumerate()
        .map(|(index, failure)| ProductAlpha1RouteEntry {
            route_id: format!("route#product-demo-failure-{}", index + 1),
            envelope_id: failure.envelope_id.clone(),
            from_place: envelope.from_place.clone(),
            to_place: envelope.to_place.clone(),
            transport_lane: format!("{}#bounded_failure", envelope.transport_seam),
            auth_lane_preserved: true,
            membership_lane_preserved: true,
            witness_lane_preserved: true,
            capability_lane_preserved: true,
        })
        .collect()
}

fn build_core_runtime_snapshot(
    package: &ProductAlpha1Package,
    runtime_plan: &ProductAlpha1RuntimePlan,
) -> Result<(LogicalPlaceRuntimeSnapshot, MessageEnvelope), ProductAlpha1SessionError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    for node in &runtime_plan.place_graph.nodes {
        shell
            .register_place(&node.place_id, &node.place_kind)
            .map_err(|error| map_core_error(Path::new("<inline>"), error))?;
    }
    for principal in &runtime_plan.bootstrap_membership {
        shell
            .add_initial_participant(principal)
            .map_err(|error| map_core_error(Path::new("<inline>"), error))?;
    }
    let snapshot = shell.snapshot();
    let principal = runtime_plan
        .bootstrap_membership
        .first()
        .cloned()
        .unwrap_or_else(|| "active_admin_participant".to_string());
    let envelope = MessageEnvelope {
        envelope_id: "envelope#product-demo-local-1".to_string(),
        from_place: runtime_plan.entry_place.clone(),
        to_place: runtime_plan.entry_place.clone(),
        transport: "local_process".to_string(),
        transport_medium: Some("same_process_memory".to_string()),
        transport_seam: "local_process".to_string(),
        payload_kind: "product_demo_tick".to_string(),
        payload_ref: "payload#product-demo-local-1".to_string(),
        principal_claim: PrincipalClaim {
            principal: principal.clone(),
            participant_place: format!("ParticipantPlace[{principal}]"),
            claimed_authority: package.auth_policy.policy_id.clone(),
            claimed_capabilities: package.capabilities.clone(),
        },
        auth_evidence: Some(AuthEvidence {
            kind: "product_alpha1_declared_auth".to_string(),
            subject: principal.clone(),
            issuer: package.auth_policy.policy_id.clone(),
            bindings: package.auth_policy.required_bindings.clone(),
            notes: vec![
                "observer-safe reports redact raw auth evidence and expose only lane presence"
                    .to_string(),
            ],
        }),
        emitter_principal: principal,
        membership_epoch: snapshot.membership.membership_epoch,
        member_incarnation: 0,
        freshness_checks: vec!["membership_epoch_match".to_string()],
        capability_requirements: package.capabilities.clone(),
        authorization_checks: package.auth_stack.clone(),
        witness_refs: non_empty_witness_refs(package),
        dispatch_outcome: "accepted".to_string(),
        notes: vec!["product alpha-1 local run envelope validates core fabric lanes".to_string()],
    };
    envelope
        .validate()
        .map_err(|error| map_core_error(Path::new("<inline>"), error))?;
    Ok((snapshot, envelope))
}

fn build_host_io_history(
    package: &ProductAlpha1Package,
) -> Result<Vec<ProductAlpha1HostIoEntry>, ProductAlpha1SessionError> {
    let Some(host_io) = &package.runtime_input.host_io else {
        return Ok(Vec::new());
    };

    if !package.effects.contains(&host_io.effect_ref) {
        return Err(host_io_error(format!(
            "runtime_input.host_io.effect_ref `{}` is not declared in package effects",
            host_io.effect_ref
        )));
    }

    let response = execute_product_host_io(host_io)?;
    if response != host_io.expected_response {
        return Err(host_io_error(format!(
            "host-I/O adapter `{}` returned {}, expected {}",
            host_io.adapter_kind,
            payload_summary(&response),
            payload_summary(&host_io.expected_response)
        )));
    }

    Ok(vec![ProductAlpha1HostIoEntry {
        adapter_kind: host_io.adapter_kind.clone(),
        effect_ref: host_io.effect_ref.clone(),
        request_summary: payload_summary(&host_io.request_payload),
        response_summary: payload_summary(&response),
        request_event_id: "event#host-io-request-1".to_string(),
        response_event_id: "event#host-io-response-1".to_string(),
    }])
}

fn execute_product_host_io(
    host_io: &ProductAlpha1HostIoRuntimeInput,
) -> Result<ProductAlpha1HostIoPayload, ProductAlpha1SessionError> {
    match (host_io.adapter_kind.as_str(), &host_io.request_payload) {
        ("AddOne", ProductAlpha1HostIoPayload::Int { value }) => {
            Ok(ProductAlpha1HostIoPayload::Int { value: value + 1 })
        }
        ("AddOne", other) => Err(host_io_error(format!(
            "AddOne adapter only accepts integer payloads, found {}",
            payload_summary(other)
        ))),
        (adapter, _) => Err(host_io_error(format!(
            "unsupported product alpha-1 host-I/O adapter `{adapter}`"
        ))),
    }
}

fn payload_summary(payload: &ProductAlpha1HostIoPayload) -> String {
    match payload {
        ProductAlpha1HostIoPayload::Int { value } => format!("Int({value})"),
        ProductAlpha1HostIoPayload::Text { value } => format!("Text(len={})", value.len()),
    }
}

fn append_attach_events(
    session: &mut ProductAlpha1SessionCarrier,
    package: &ProductAlpha1Package,
    request_event_id: &str,
    verdict_event_id: &str,
    terminal_outcome: &str,
    activation_cut_ref: Option<&str>,
) {
    if let Some(previous) = session.event_dag.nodes.last() {
        session.event_dag.edges.push(ProductAlpha1EventEdge {
            from_event: previous.event_id.clone(),
            to_event: request_event_id.to_string(),
            relation: "same_session_hotplug_order".to_string(),
        });
    }
    session.event_dag.nodes.push(ProductAlpha1EventNode {
        event_id: request_event_id.to_string(),
        event_kind: "hotplug_request".to_string(),
        place_ref: "Place[ProductDemoRoom]".to_string(),
        envelope_ref: Some(format!("envelope#attach#{}", package.package_id)),
        observer_safe_summary: format!("attach request for {}", package.package_id),
    });
    session.event_dag.nodes.push(ProductAlpha1EventNode {
        event_id: verdict_event_id.to_string(),
        event_kind: "hotplug_verdict".to_string(),
        place_ref: "Place[ProductDemoRoom]".to_string(),
        envelope_ref: Some(format!("envelope#attach#{}", package.package_id)),
        observer_safe_summary: format!("attach verdict {terminal_outcome}"),
    });
    session.event_dag.edges.push(ProductAlpha1EventEdge {
        from_event: request_event_id.to_string(),
        to_event: verdict_event_id.to_string(),
        relation: "request_verdict_order".to_string(),
    });

    if let Some(activation_cut_ref) = activation_cut_ref {
        session.event_dag.nodes.push(ProductAlpha1EventNode {
            event_id: activation_cut_ref.to_string(),
            event_kind: "activation_cut".to_string(),
            place_ref: "Place[ProductDemoRoom]".to_string(),
            envelope_ref: Some(format!("envelope#attach#{}", package.package_id)),
            observer_safe_summary: format!("activation cut admitted for {}", package.package_id),
        });
        session.event_dag.edges.push(ProductAlpha1EventEdge {
            from_event: verdict_event_id.to_string(),
            to_event: activation_cut_ref.to_string(),
            relation: "accepted_attach_activation_cut".to_string(),
        });
    }
}

fn append_attach_route(session: &mut ProductAlpha1SessionCarrier, package: &ProductAlpha1Package) {
    session.route_graph.routes.push(ProductAlpha1RouteEntry {
        route_id: format!("route#attach#{}", package.package_id),
        envelope_id: format!("envelope#attach#{}", package.package_id),
        from_place: "Place[ProductDemoRoom]".to_string(),
        to_place: "AttachPoint[ProductDemoRoom::Layers]".to_string(),
        transport_lane: "same_session_attach_envelope".to_string(),
        auth_lane_preserved: true,
        membership_lane_preserved: true,
        witness_lane_preserved: true,
        capability_lane_preserved: true,
    });
}

fn append_attach_message_state(
    session: &mut ProductAlpha1SessionCarrier,
    package: &ProductAlpha1Package,
    terminal_outcome: &str,
) {
    let (state, failure_class, recovery_action) = match terminal_outcome {
        "accepted" => ("Delivered", None, None),
        "rejected" => ("Rejected", Some("reject"), Some("reject")),
        _ => ("Deferred", Some("deferred"), Some("defer_to_later_runtime")),
    };
    session
        .message_recovery_state
        .message_state_lane
        .push(ProductAlpha1MessageStateRecord {
            envelope_id: format!("envelope#attach#{}", package.package_id),
            state: state.to_string(),
            failure_class: failure_class.map(str::to_string),
            recovery_action: recovery_action.map(str::to_string),
        });
}

fn initial_auth_decision(package: &ProductAlpha1Package) -> ProductAlpha1AuthDecision {
    ProductAlpha1AuthDecision {
        decision_id: format!("auth_decision#run-local#{}", package.package_id),
        decision: "accepted".to_string(),
        auth_stack: package.auth_stack.clone(),
        membership_requirements: package.membership_requirements.clone(),
        capability_refs: package.capabilities.clone(),
        witness_refs: non_empty_witness_refs(package),
        contract_update_path: "root_package_admission".to_string(),
        overlay_transparency_claimed: false,
        reason_refs: vec![
            "checker_report#accepted_auth_policy".to_string(),
            "runtime_plan#bootstrap_membership".to_string(),
        ],
        notes: vec![
            "root package auth state is admitted by product alpha-1 checker evidence".to_string(),
        ],
    }
}

fn initial_capability_decision(package: &ProductAlpha1Package) -> ProductAlpha1CapabilityDecision {
    ProductAlpha1CapabilityDecision {
        decision_id: format!("capability_decision#run-local#{}", package.package_id),
        decision: "accepted".to_string(),
        requested_capabilities: package.capabilities.clone(),
        granted_capabilities: package.capabilities.clone(),
        missing_capabilities: Vec::new(),
        notes: vec![
            "root package capabilities are the initial product session capability frontier"
                .to_string(),
        ],
    }
}

fn evaluate_auth_decision(
    session: &ProductAlpha1SessionCarrier,
    package: &ProductAlpha1Package,
    request_event_id: &str,
) -> ProductAlpha1AuthDecision {
    let active_members: BTreeSet<_> = session.membership.active_members.iter().cloned().collect();
    let active_bindings: BTreeSet<_> = session.auth_state.active_bindings.iter().cloned().collect();
    let witness_refs: BTreeSet<_> = session.witness_state.witness_refs.iter().cloned().collect();

    let mut reason_refs = Vec::new();
    let mut notes = Vec::new();

    for member in &package.membership_requirements {
        if active_members.contains(member) {
            reason_refs.push(format!("membership_present:{member}"));
        } else {
            notes.push(format!("missing_membership:{member}"));
        }
    }
    for binding in &package.auth_policy.required_bindings {
        if active_bindings.contains(binding) {
            reason_refs.push(format!("auth_binding_present:{binding}"));
        } else {
            notes.push(format!("missing_auth_binding:{binding}"));
        }
    }
    for witness in &package.witness_requirements {
        if witness_refs.contains(witness) {
            reason_refs.push(format!("witness_present:{witness}"));
        } else {
            notes.push(format!("missing_witness:{witness}"));
        }
    }
    if package.auth_stack.is_empty() {
        notes.push("missing_auth_stack".to_string());
    }

    let decision = if notes.is_empty() {
        "accepted"
    } else {
        "rejected"
    };

    ProductAlpha1AuthDecision {
        decision_id: format!("auth_decision#{request_event_id}"),
        decision: decision.to_string(),
        auth_stack: package.auth_stack.clone(),
        membership_requirements: package.membership_requirements.clone(),
        capability_refs: package.capabilities.clone(),
        witness_refs: non_empty_witness_refs(package),
        contract_update_path: "explicit_attach_contract_update".to_string(),
        overlay_transparency_claimed: false,
        reason_refs,
        notes,
    }
}

fn evaluate_capability_decision(
    session: &ProductAlpha1SessionCarrier,
    package: &ProductAlpha1Package,
    request_event_id: &str,
) -> ProductAlpha1CapabilityDecision {
    let granted: BTreeSet<_> = session
        .capability_state
        .granted_capabilities
        .iter()
        .cloned()
        .collect();
    let missing_capabilities: Vec<String> = package
        .capabilities
        .iter()
        .filter(|capability| !granted.contains(*capability))
        .cloned()
        .collect();
    let decision = if missing_capabilities.is_empty() {
        "accepted"
    } else {
        "rejected"
    };
    let notes = if missing_capabilities.is_empty() {
        vec![
            "requested attach capabilities are present in the product session frontier".to_string(),
        ]
    } else {
        missing_capabilities
            .iter()
            .map(|capability| format!("missing_capability:{capability}"))
            .collect()
    };

    ProductAlpha1CapabilityDecision {
        decision_id: format!("capability_decision#{request_event_id}"),
        decision: decision.to_string(),
        requested_capabilities: package.capabilities.clone(),
        granted_capabilities: session.capability_state.granted_capabilities.clone(),
        missing_capabilities,
        notes,
    }
}

fn build_auth_state(package: &ProductAlpha1Package) -> ProductAlpha1AuthState {
    let bindings: BTreeSet<String> = package
        .auth_policy
        .required_bindings
        .iter()
        .cloned()
        .collect();
    ProductAlpha1AuthState {
        active_bindings: bindings.into_iter().collect(),
        active_membership_refs: bootstrap_membership(package),
    }
}

fn build_capability_state(package: &ProductAlpha1Package) -> ProductAlpha1CapabilityState {
    let capabilities: BTreeSet<String> = package.capabilities.iter().cloned().collect();
    ProductAlpha1CapabilityState {
        granted_capabilities: capabilities.into_iter().collect(),
    }
}

fn build_transport_contracts(
    package: &ProductAlpha1Package,
) -> Vec<ProductAlpha1TransportContract> {
    let recovery_kind = recovery_policy_kind(&package.message_recovery_policy.recovery);
    vec![
        ProductAlpha1TransportContract {
            contract_id: format!("transport_contract#{}#best-effort", package.package_id),
            contract_kind: "BestEffort".to_string(),
            guarantees: vec![
                "local same-session delivery is observable".to_string(),
                "failure classes remain explicit in MessageState".to_string(),
            ],
            non_claims: vec![
                "not durable outbox".to_string(),
                "not exactly-once transport".to_string(),
            ],
        },
        ProductAlpha1TransportContract {
            contract_id: format!("transport_contract#{}#timeout", package.package_id),
            contract_kind: "TimeoutBounded".to_string(),
            guarantees: vec![format!(
                "timeout failure is converted through declared bounded {recovery_kind} policy"
            )],
            non_claims: vec!["not WAN partition recovery".to_string()],
        },
    ]
}

fn build_recovery_policies(
    package: &ProductAlpha1Package,
) -> Vec<ProductAlpha1RecoveryPolicyRecord> {
    vec![ProductAlpha1RecoveryPolicyRecord {
        policy_id: format!("recovery_policy#{}", package.package_id),
        policy_kind: recovery_policy_kind(&package.message_recovery_policy.recovery).to_string(),
        handled_failures: package.message_recovery_policy.handled_failures.clone(),
        max_retries: if package.message_recovery_policy.recovery == "retry_then_reject" {
            1
        } else {
            0
        },
        terminal_action: if package.message_recovery_policy.recovery.contains("reject") {
            "Reject".to_string()
        } else {
            "FallbackOrRetry".to_string()
        },
        modal_obligations: vec![
            "○ timeout advances to retry observation".to_string(),
            "□ rejected messages do not mutate active runtime state".to_string(),
        ],
    }]
}

fn build_failure_observations(
    package: &ProductAlpha1Package,
) -> Vec<ProductAlpha1FailureObservation> {
    package
        .message_recovery_policy
        .handled_failures
        .iter()
        .map(|failure| ProductAlpha1FailureObservation {
            envelope_id: format!("envelope#failure#{}#{}", package.package_id, failure),
            failure_class: failure.clone(),
            initial_state: "InFlight".to_string(),
            recovery_action: recovery_policy_kind(&package.message_recovery_policy.recovery)
                .to_string(),
            terminal_state: recovery_terminal_state(&package.message_recovery_policy.recovery)
                .to_string(),
            retry_count: if package
                .message_recovery_policy
                .recovery
                .starts_with("retry")
            {
                1
            } else {
                0
            },
            notes: vec![
                format!(
                    "bounded product demo observes {failure} and applies declared {} recovery",
                    package.message_recovery_policy.recovery
                ),
                "no durable outbox, WAN replay, or exactly-once delivery is claimed".to_string(),
            ],
        })
        .collect()
}

fn recovery_terminal_state(policy: &str) -> &str {
    match policy {
        "retry_then_reject" | "reject" => "Rejected",
        "retry" => "Retried",
        "fallback" => "FallbackVisible",
        _ => "DeclaredTerminalState",
    }
}

fn recovery_policy_kind(policy: &str) -> &str {
    match policy {
        "retry_then_reject" => "RetryThenReject",
        "retry" => "Retry",
        "reject" => "Reject",
        "fallback" => "Fallback",
        _ => "DeclaredRecoveryPolicy",
    }
}

fn build_observer_safe_export(
    session: &ProductAlpha1SessionCarrier,
    package: &ProductAlpha1Package,
) -> ProductAlpha1ObserverSafeExport {
    let existing = &session.observer_safe_export;
    let view_role = if existing.view_role.is_empty() {
        package.observation_policy.view_role.clone()
    } else {
        existing.view_role.clone()
    };
    let redaction_level = if existing.redaction_level.is_empty() {
        package.redaction_policy.level.clone()
    } else {
        existing.redaction_level.clone()
    };
    let retention_scope = if existing.retention_scope.is_empty() {
        package.retention_policy.scope.clone()
    } else {
        existing.retention_scope.clone()
    };
    let redacted_fields: Vec<String> = existing
        .redacted_fields
        .iter()
        .chain(package.redaction_policy.redacted_fields.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    ProductAlpha1ObserverSafeExport {
        view_role,
        redaction_level,
        retention_scope,
        redacted_fields,
        visible_event_ids: session
            .event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect(),
        visible_routes: session
            .route_graph
            .routes
            .iter()
            .map(|route| route.route_id.clone())
            .collect(),
        visible_hotplug_events: session
            .hotplug_lifecycle
            .iter()
            .map(|entry| format!("{}:{}", entry.package_id, entry.terminal_outcome))
            .collect(),
        visible_host_io_events: session
            .host_io_history
            .iter()
            .map(|entry| {
                format!(
                    "{}:{}->{}",
                    entry.adapter_kind, entry.request_summary, entry.response_summary
                )
            })
            .collect(),
        notes: vec![
            "observer-safe export exposes summaries only".to_string(),
            "raw witness payloads and raw auth evidence remain redacted".to_string(),
        ],
    }
}

fn host_io_error(detail: String) -> ProductAlpha1SessionError {
    ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::HostIo,
        path: PathBuf::from("<runtime_input.host_io>"),
        detail,
    }
}

fn bootstrap_membership(package: &ProductAlpha1Package) -> Vec<String> {
    let values: BTreeSet<String> = package.membership_requirements.iter().cloned().collect();
    values.into_iter().collect()
}

fn non_empty_witness_refs(package: &ProductAlpha1Package) -> Vec<String> {
    if package.witness_requirements.is_empty() {
        vec!["witness#product-alpha1-local".to_string()]
    } else {
        package.witness_requirements.clone()
    }
}

fn map_product_error(
    kind: ProductAlpha1SessionErrorKind,
    error: ProductAlpha1Error,
) -> ProductAlpha1SessionError {
    ProductAlpha1SessionError {
        kind,
        path: error.path,
        detail: error.detail,
    }
}

fn map_core_error(path: &Path, error: MirroreaCoreError) -> ProductAlpha1SessionError {
    ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Core,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn run_local_surface_kind() -> String {
    PRODUCT_ALPHA1_RUN_LOCAL_SURFACE_KIND.to_string()
}

fn attach_surface_kind() -> String {
    PRODUCT_ALPHA1_ATTACH_SURFACE_KIND.to_string()
}

fn save_surface_kind() -> String {
    PRODUCT_ALPHA1_SAVE_SURFACE_KIND.to_string()
}

fn load_surface_kind() -> String {
    PRODUCT_ALPHA1_LOAD_SURFACE_KIND.to_string()
}

fn quiescent_save_surface_kind() -> String {
    PRODUCT_ALPHA1_QUIESCENT_SAVE_SURFACE_KIND.to_string()
}

fn session_surface_kind() -> String {
    PRODUCT_ALPHA1_SESSION_SURFACE_KIND.to_string()
}

fn runtime_plan_scope() -> String {
    PRODUCT_ALPHA1_RUNTIME_PLAN_SCOPE.to_string()
}

fn session_scope() -> String {
    PRODUCT_ALPHA1_SESSION_SCOPE.to_string()
}

fn quiescence_state_default() -> ProductAlpha1QuiescenceState {
    ProductAlpha1QuiescenceState {
        seal_protocol_enabled: true,
        post_cut_send_guard_enabled: true,
        sealed_place_refs: Vec::new(),
        rejected_post_cut_sends: Vec::new(),
    }
}

fn stop_lines_default() -> Vec<String> {
    PRODUCT_ALPHA1_RUNTIME_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRODUCT_ALPHA1_RUNTIME_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
