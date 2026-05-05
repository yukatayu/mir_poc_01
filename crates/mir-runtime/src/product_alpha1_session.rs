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
pub const PRODUCT_ALPHA1_SESSION_SURFACE_KIND: &str = "product_alpha1_session_carrier";

const PRODUCT_ALPHA1_RUNTIME_STOP_LINES: &[&str] = &[
    "product alpha-1 same-session runtime is alpha-stable only, not a final public runtime ABI",
    "local session store is same-process persistence, not product transport command behavior or WAN/federation",
    "save/load and quiescent-save execution remain later than P-A1-27",
    "message recovery state is carried but failure/retry execution remains later than P-A1-27",
    "native execution remains disabled; native launch bundle work is later",
];

const PRODUCT_ALPHA1_RUNTIME_LIMITATIONS: &[&str] = &[
    "controlled local product alpha-1 session carrier only",
    "one deterministic product demo runtime path with typed host-I/O add-one evidence",
    "debug-layer attach is same-session and observable; broader auth/rate-limit/object/avatar packages remain later",
    "no distributed durable save/load, WAN federation, final viewer ABI, or arbitrary native package execution",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductAlpha1SessionErrorKind {
    FrontDoor,
    Checker,
    Core,
    HostIo,
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
    pub residual_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1MessageRecoveryState {
    pub message_state_lane: Vec<ProductAlpha1MessageStateRecord>,
    pub handled_failures: Vec<String>,
    pub recovery_policy: String,
    pub modal_obligations: Vec<String>,
    pub runtime_recovery_claimed: bool,
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
    let event_dag = build_initial_event_dag(package, &envelope, &host_io_history);
    let route_graph = ProductAlpha1RouteGraph {
        routes: vec![ProductAlpha1RouteEntry {
            route_id: "route#product-demo-local-1".to_string(),
            envelope_id: envelope.envelope_id.clone(),
            from_place: envelope.from_place.clone(),
            to_place: envelope.to_place.clone(),
            transport_lane: envelope.transport_seam.clone(),
            auth_lane_preserved: true,
            membership_lane_preserved: true,
            witness_lane_preserved: true,
            capability_lane_preserved: true,
        }],
    };
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
        residual_reason:
            "P-A1-27 carries save/load state but does not execute ordinary save or quiescent-save"
                .to_string(),
    };
    let message_recovery_state = ProductAlpha1MessageRecoveryState {
        message_state_lane: vec![ProductAlpha1MessageStateRecord {
            envelope_id: envelope.envelope_id.clone(),
            state: "delivered".to_string(),
            failure_class: None,
            recovery_action: None,
        }],
        handled_failures: package.message_recovery_policy.handled_failures.clone(),
        recovery_policy: package.message_recovery_policy.recovery.clone(),
        modal_obligations: vec![
            "modal ○: retry/reject path remains operationally enabled for later failure injection"
                .to_string(),
            "modal □: accepted local delivery cannot erase declared recovery obligations"
                .to_string(),
        ],
        runtime_recovery_claimed: false,
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

fn build_initial_event_dag(
    package: &ProductAlpha1Package,
    envelope: &MessageEnvelope,
    host_io_history: &[ProductAlpha1HostIoEntry],
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

    ProductAlpha1EventDag { nodes, edges }
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
        "accepted" => ("delivered", None, None),
        "rejected" => ("rejected", Some("reject"), Some("reject")),
        _ => ("deferred", Some("deferred"), Some("defer_to_later_runtime")),
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
    let mut bindings: BTreeSet<String> = package
        .auth_policy
        .required_bindings
        .iter()
        .cloned()
        .collect();
    bindings.insert("admin_membership".to_string());
    ProductAlpha1AuthState {
        active_bindings: bindings.into_iter().collect(),
        active_membership_refs: bootstrap_membership(package),
    }
}

fn build_capability_state(package: &ProductAlpha1Package) -> ProductAlpha1CapabilityState {
    let mut capabilities: BTreeSet<String> = package.capabilities.iter().cloned().collect();
    capabilities.insert("ObserveDebugSummary".to_string());
    capabilities.insert("AttachDebugLayer".to_string());
    ProductAlpha1CapabilityState {
        granted_capabilities: capabilities.into_iter().collect(),
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
    let mut values: BTreeSet<String> = package.membership_requirements.iter().cloned().collect();
    values.insert("active_admin_participant".to_string());
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

fn session_surface_kind() -> String {
    PRODUCT_ALPHA1_SESSION_SURFACE_KIND.to_string()
}

fn runtime_plan_scope() -> String {
    PRODUCT_ALPHA1_RUNTIME_PLAN_SCOPE.to_string()
}

fn session_scope() -> String {
    PRODUCT_ALPHA1_SESSION_SCOPE.to_string()
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
