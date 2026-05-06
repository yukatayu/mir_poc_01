use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::product_alpha1_session::{
    ProductAlpha1EventDag, ProductAlpha1EventEdge, ProductAlpha1EventNode,
    ProductAlpha1HotPlugLifecycleEntry, ProductAlpha1MessageRecoveryState,
    ProductAlpha1ObserverSafeExport, ProductAlpha1PlaceEdge, ProductAlpha1PlaceNode,
    ProductAlpha1RouteGraph, ProductAlpha1SaveLoadState, ProductAlpha1SessionCarrier,
    ProductAlpha1SessionError,
};

pub const PRODUCT_ALPHA1_DEVTOOLS_SURFACE_KIND: &str = "product_alpha1_devtools_export_report";
pub const PRODUCT_ALPHA1_VIEWER_MODE: &str = "product_alpha1_nonfinal_static_html_viewer";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1DevtoolsBundle {
    #[serde(default = "devtools_surface_kind")]
    pub surface_kind: String,
    pub session_id: String,
    pub viewer_mode: String,
    pub observer_authority: String,
    pub redaction_policy: String,
    pub retention_scope: String,
    pub admin_debug_view_status: String,
    pub panel_ids: Vec<String>,
    pub panels: ProductAlpha1DevtoolsPanels,
    pub product_alpha1_ready: bool,
    pub final_public_viewer_frozen: bool,
    pub durable_audit_claimed: bool,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1DevtoolsPanels {
    pub product_overview: ProductAlpha1ProductOverviewPanel,
    pub source_import_graph: ProductAlpha1SourceImportGraphPanel,
    pub package_dependency_graph: ProductAlpha1PackageDependencyGraphPanel,
    pub projection_target_graph: ProductAlpha1ProjectionTargetGraphPanel,
    pub place_graph: ProductAlpha1PlaceGraphPanel,
    pub server_client_process_graph: ProductAlpha1ServerClientProcessGraphPanel,
    pub event_dag: ProductAlpha1EventDag,
    pub message_route_graph: ProductAlpha1RouteGraph,
    pub membership_frontier_timeline: Vec<ProductAlpha1MembershipFrontierEntry>,
    pub witness_relation_timeline: ProductAlpha1WitnessRelationPanel,
    pub hotplug_lifecycle: Vec<ProductAlpha1HotPlugLifecycleEntry>,
    pub save_load_quiescent_timeline: ProductAlpha1SaveLoadState,
    pub contract_effect_failure_summary: ProductAlpha1ContractEffectFailureSummaryPanel,
    pub message_failure_recovery: ProductAlpha1MessageRecoveryState,
    pub fallback_degradation: Vec<String>,
    pub auth_capability_decision: ProductAlpha1AuthCapabilityPanel,
    pub redaction_toggle: ProductAlpha1RedactionTogglePanel,
    pub retention_trace: ProductAlpha1RetentionTracePanel,
    pub portal_graph_future: ProductAlpha1FutureBoundaryPanel,
    pub shard_map_future: ProductAlpha1FutureBoundaryPanel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1ProductOverviewPanel {
    pub package_id: String,
    pub package_version: String,
    pub phase: String,
    pub active_layers: Vec<String>,
    pub view_role: String,
    pub redaction_level: String,
    pub retention_scope: String,
    pub event_count: usize,
    pub route_count: usize,
    pub active_member_count: usize,
    pub latest_savepoint_ref: Option<String>,
    pub non_claims: Vec<String>,
    pub product_alpha1_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PlaceGraphPanel {
    pub nodes: Vec<ProductAlpha1PlaceNodeSummary>,
    pub edges: Vec<ProductAlpha1PlaceEdgeSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PlaceNodeSummary {
    pub observer_safe_place_ref: String,
    pub place_kind: String,
    pub private_place_ref_redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PlaceEdgeSummary {
    pub from_place: String,
    pub to_place: String,
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1SourceImportGraphPanel {
    pub executable_input_kind: String,
    pub representative_source_status: String,
    pub nodes: Vec<ProductAlpha1NamedGraphNode>,
    pub edges: Vec<ProductAlpha1NamedGraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1PackageDependencyGraphPanel {
    pub package_root: String,
    pub direct_dependency_count: usize,
    pub nodes: Vec<ProductAlpha1NamedGraphNode>,
    pub edges: Vec<ProductAlpha1NamedGraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1ProjectionTargetGraphPanel {
    pub projection_status: String,
    pub target_count: usize,
    pub packet_boundary_count: usize,
    pub ffi_boundary_count: usize,
    pub llvm_codegen_claimed: bool,
    pub nodes: Vec<ProductAlpha1NamedGraphNode>,
    pub edges: Vec<ProductAlpha1NamedGraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1ServerClientProcessGraphPanel {
    pub current_execution_mode: String,
    pub final_binary_split_claimed: bool,
    pub nodes: Vec<ProductAlpha1NamedGraphNode>,
    pub edges: Vec<ProductAlpha1NamedGraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1NamedGraphNode {
    pub node_id: String,
    pub node_kind: String,
    pub current_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1NamedGraphEdge {
    pub from_node: String,
    pub to_node: String,
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1MembershipFrontierEntry {
    pub frontier_id: String,
    pub membership_epoch: u64,
    pub config_epoch: u64,
    pub config_epoch_status: String,
    pub active_member_count: usize,
    pub required_membership_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1AuthCapabilityPanel {
    pub auth_decisions: Vec<ProductAlpha1AuthDecisionSummary>,
    pub capability_decisions: Vec<ProductAlpha1CapabilityDecisionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1WitnessRelationPanel {
    pub frontier_id: String,
    pub observer_safe_relation_refs: Vec<String>,
    pub hidden_private_witness_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1AuthDecisionSummary {
    pub decision_id: String,
    pub decision: String,
    pub auth_stack: Vec<String>,
    pub membership_requirement_count: usize,
    pub capability_requirement_count: usize,
    pub private_witness_requirement_count: usize,
    pub contract_update_path: String,
    pub overlay_transparency_claimed: bool,
    pub observer_safe_reason_families: Vec<String>,
    pub hidden_private_reason_ref_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1CapabilityDecisionSummary {
    pub decision_id: String,
    pub decision: String,
    pub requested_capability_count: usize,
    pub granted_capability_count: usize,
    pub missing_capability_count: usize,
    pub observer_safe_note_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RedactionTogglePanel {
    pub observer_safe_available: bool,
    pub admin_debug_status: String,
    pub redacted_fields_count: usize,
    pub raw_secret_values_rendered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1RetentionTracePanel {
    pub retention_scope: String,
    pub retained_artifact_classes: Vec<String>,
    pub on_demand_trace_available: bool,
    pub observer_safe_hit_count: usize,
    pub observer_safe_miss_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1ContractEffectFailureSummaryPanel {
    pub contract_ids: Vec<String>,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
    pub capability_requirement_count: usize,
    pub witness_requirement_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1FutureBoundaryPanel {
    pub current_status: String,
    pub nodes: Vec<ProductAlpha1NamedGraphNode>,
    pub edges: Vec<ProductAlpha1NamedGraphEdge>,
    pub non_claims: Vec<String>,
}

pub fn export_product_alpha1_devtools(
    session: &ProductAlpha1SessionCarrier,
) -> Result<ProductAlpha1DevtoolsBundle, ProductAlpha1SessionError> {
    let panel_ids = required_panel_ids();
    let panels = ProductAlpha1DevtoolsPanels {
        product_overview: ProductAlpha1ProductOverviewPanel {
            package_id: session.package_id.clone(),
            package_version: session.runtime_plan.package_version.clone(),
            phase: session.phase.clone(),
            active_layers: session.active_layers.clone(),
            view_role: session.observer_safe_export.view_role.clone(),
            redaction_level: session.observer_safe_export.redaction_level.clone(),
            retention_scope: session.observer_safe_export.retention_scope.clone(),
            event_count: session.event_dag.nodes.len(),
            route_count: session.route_graph.routes.len(),
            active_member_count: session.membership.active_members.len(),
            latest_savepoint_ref: session.save_load_state.local_savepoint_refs.last().cloned(),
            non_claims: vec![
                "not final public viewer API".to_string(),
                "not durable telemetry or audit backend".to_string(),
                "not raw admin/debug secret export".to_string(),
            ],
            product_alpha1_ready: false,
        },
        source_import_graph: source_import_graph_panel(session),
        package_dependency_graph: package_dependency_graph_panel(session),
        projection_target_graph: projection_target_graph_panel(session),
        place_graph: observer_safe_place_graph_panel(
            &session.runtime_plan.place_graph.nodes,
            &session.runtime_plan.place_graph.edges,
        ),
        server_client_process_graph: server_client_process_graph_panel(session),
        event_dag: session.event_dag.clone(),
        message_route_graph: session.route_graph.clone(),
        membership_frontier_timeline: vec![ProductAlpha1MembershipFrontierEntry {
            frontier_id: session.membership.frontier_id.clone(),
            membership_epoch: session.membership.membership_epoch,
            config_epoch: 0,
            config_epoch_status: "kept_later_manifest_only".to_string(),
            active_member_count: session.membership.active_members.len(),
            required_membership_count: session.membership.required_membership_refs.len(),
        }],
        witness_relation_timeline: ProductAlpha1WitnessRelationPanel {
            frontier_id: session.witness_state.frontier_id.clone(),
            observer_safe_relation_refs: session.witness_state.observer_safe_relation_refs.clone(),
            hidden_private_witness_count: session.witness_state.witness_refs.len(),
        },
        hotplug_lifecycle: session.hotplug_lifecycle.clone(),
        save_load_quiescent_timeline: session.save_load_state.clone(),
        contract_effect_failure_summary: ProductAlpha1ContractEffectFailureSummaryPanel {
            contract_ids: session.runtime_plan.declared_contract_ids.clone(),
            effect_row: session.runtime_plan.effect_row.clone(),
            failure_row: session.runtime_plan.failure_row.clone(),
            capability_requirement_count: session.runtime_plan.capability_requirements.len(),
            witness_requirement_count: session.runtime_plan.witness_requirements.len(),
        },
        message_failure_recovery: session.message_recovery_state.clone(),
        fallback_degradation: vec![
            "unsupported runtime fallback remains visible monotone degradation when encountered"
                .to_string(),
        ],
        auth_capability_decision: ProductAlpha1AuthCapabilityPanel {
            auth_decisions: session
                .auth_decisions
                .iter()
                .map(|decision| ProductAlpha1AuthDecisionSummary {
                    decision_id: decision.decision_id.clone(),
                    decision: decision.decision.clone(),
                    auth_stack: decision.auth_stack.clone(),
                    membership_requirement_count: decision.membership_requirements.len(),
                    capability_requirement_count: decision.capability_refs.len(),
                    private_witness_requirement_count: decision.witness_refs.len(),
                    contract_update_path: decision.contract_update_path.clone(),
                    overlay_transparency_claimed: decision.overlay_transparency_claimed,
                    observer_safe_reason_families: observer_safe_reason_families(
                        &decision.reason_refs,
                    ),
                    hidden_private_reason_ref_count: decision.reason_refs.len(),
                })
                .collect(),
            capability_decisions: session
                .capability_decisions
                .iter()
                .map(|decision| ProductAlpha1CapabilityDecisionSummary {
                    decision_id: decision.decision_id.clone(),
                    decision: decision.decision.clone(),
                    requested_capability_count: decision.requested_capabilities.len(),
                    granted_capability_count: decision.granted_capabilities.len(),
                    missing_capability_count: decision.missing_capabilities.len(),
                    observer_safe_note_count: decision.notes.len(),
                })
                .collect(),
        },
        redaction_toggle: ProductAlpha1RedactionTogglePanel {
            observer_safe_available: true,
            admin_debug_status: "kept_later".to_string(),
            redacted_fields_count: session.observer_safe_export.redacted_fields.len(),
            raw_secret_values_rendered: false,
        },
        retention_trace: ProductAlpha1RetentionTracePanel {
            retention_scope: session.observer_safe_export.retention_scope.clone(),
            retained_artifact_classes: vec![
                "checker_report".to_string(),
                "runtime_plan".to_string(),
                "observer_safe_event_ids".to_string(),
                "observer_safe_route_ids".to_string(),
                "devtools_export_refs".to_string(),
            ],
            on_demand_trace_available: true,
            observer_safe_hit_count: session.observer_safe_export.visible_event_ids.len()
                + session.observer_safe_export.visible_routes.len()
                + session.save_load_state.devtools_export_refs.len(),
            observer_safe_miss_count: 0,
        },
        portal_graph_future: portal_graph_future_panel(session),
        shard_map_future: shard_map_future_panel(session),
    };
    Ok(ProductAlpha1DevtoolsBundle {
        surface_kind: devtools_surface_kind(),
        session_id: session.session_id.clone(),
        viewer_mode: PRODUCT_ALPHA1_VIEWER_MODE.to_string(),
        observer_authority: "ObserveProductAlpha1Demo(observer_safe)".to_string(),
        redaction_policy: session.observer_safe_export.redaction_level.clone(),
        retention_scope: session.observer_safe_export.retention_scope.clone(),
        admin_debug_view_status: "kept_later".to_string(),
        panel_ids,
        panels,
        product_alpha1_ready: false,
        final_public_viewer_frozen: false,
        durable_audit_claimed: false,
        non_claims: vec![
            "not final public viewer API".to_string(),
            "not durable telemetry or audit backend".to_string(),
            "not raw admin/debug secret export".to_string(),
        ],
    })
}

pub fn export_product_alpha1_devtools_for_session(
    session: &ProductAlpha1SessionCarrier,
    export_ref: &str,
) -> Result<(ProductAlpha1SessionCarrier, ProductAlpha1DevtoolsBundle), ProductAlpha1SessionError> {
    let mut next = session.clone();
    next.phase = "devtools_exported".to_string();
    if !next
        .save_load_state
        .devtools_export_refs
        .iter()
        .any(|existing| existing == export_ref)
    {
        next.save_load_state
            .devtools_export_refs
            .push(export_ref.to_string());
    }
    append_devtools_export_event(&mut next, export_ref);
    refresh_observer_safe_devtools_export(&mut next);
    let bundle = export_product_alpha1_devtools(&next)?;
    Ok((next, bundle))
}

pub fn render_product_alpha1_viewer_html(bundle: &ProductAlpha1DevtoolsBundle) -> String {
    let mut body = String::new();
    body.push_str("<!doctype html><html><head><meta charset=\"utf-8\"><title>Mirrorea Product Alpha-1 Devtools</title>");
    body.push_str("<style>body{font-family:system-ui,sans-serif;margin:0;background:#f7f7f5;color:#1e2930}main{max-width:1180px;margin:auto;padding:24px}.panel{border:1px solid #ccd2d8;border-radius:8px;padding:14px;margin:10px 0;background:white}code{background:#edf2f7;padding:2px 4px;border-radius:4px}pre{white-space:pre-wrap;overflow:auto;background:#f3f6f8;border:1px solid #d9e1e7;border-radius:6px;padding:10px;font-size:13px;line-height:1.45}</style>");
    body.push_str("</head><body><main>");
    body.push_str("<h1>Mirrorea Product Alpha-1 Devtools</h1>");
    body.push_str(&format!(
        "<p>Session <code>{}</code> uses observer-safe redaction. Admin/debug view: <code>{}</code>. Role <code>{}</code>, redaction <code>{}</code>, retention <code>{}</code>.</p>",
        escape_html(&bundle.session_id),
        escape_html(&bundle.admin_debug_view_status),
        escape_html(&bundle.observer_authority),
        escape_html(&bundle.redaction_policy),
        escape_html(&bundle.retention_scope)
    ));
    for panel_id in &bundle.panel_ids {
        let panel_payload = panel_payload_json(bundle, panel_id);
        body.push_str(&format!(
            "<section class=\"panel\" id=\"{}\"><h2>{}</h2><p>{}</p><pre>{}</pre></section>",
            escape_html(panel_id),
            escape_html(panel_id),
            panel_summary(panel_id),
            escape_html(&panel_payload)
        ));
    }
    body.push_str("</main></body></html>");
    body
}

pub fn validate_product_alpha1_viewer_dir(path: impl Into<PathBuf>) -> bool {
    let path = path.into();
    path.join("bundle.json").is_file() && path.join("index.html").is_file()
}

fn append_devtools_export_event(session: &mut ProductAlpha1SessionCarrier, export_ref: &str) {
    let event_id = unique_event_id(session, "event#devtools-export");
    if let Some(previous) = session.event_dag.nodes.last() {
        session.event_dag.edges.push(ProductAlpha1EventEdge {
            from_event: previous.event_id.clone(),
            to_event: event_id.clone(),
            relation: "same_session_devtools_export_order".to_string(),
        });
    }
    session.event_dag.nodes.push(ProductAlpha1EventNode {
        event_id,
        event_kind: "devtools_export".to_string(),
        place_ref: session.runtime_plan.entry_place.clone(),
        envelope_ref: None,
        observer_safe_summary: format!("product alpha devtools export {export_ref}"),
    });
}

fn refresh_observer_safe_devtools_export(session: &mut ProductAlpha1SessionCarrier) {
    let existing = session.observer_safe_export.clone();
    session.observer_safe_export = ProductAlpha1ObserverSafeExport {
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
        ..existing
    };
}

fn unique_event_id(session: &ProductAlpha1SessionCarrier, prefix: &str) -> String {
    let mut index = session.event_dag.nodes.len() + 1;
    loop {
        let candidate = format!("{prefix}#{index}");
        if !session
            .event_dag
            .nodes
            .iter()
            .any(|node| node.event_id == candidate)
        {
            return candidate;
        }
        index += 1;
    }
}

fn required_panel_ids() -> Vec<String> {
    [
        "product_overview",
        "source_import_graph",
        "package_dependency_graph",
        "projection_target_graph",
        "place_graph",
        "server_client_process_graph",
        "event_dag",
        "message_route_graph",
        "membership_frontier_timeline",
        "witness_relation_timeline",
        "hotplug_lifecycle",
        "save_load_quiescent_timeline",
        "contract_effect_failure_summary",
        "message_failure_recovery",
        "fallback_degradation",
        "auth_capability_decision",
        "redaction_toggle",
        "retention_trace",
        "portal_graph_future",
        "shard_map_future",
    ]
    .iter()
    .map(|value| (*value).to_string())
    .collect()
}

fn panel_summary(panel_id: &str) -> &'static str {
    match panel_id {
        "source_import_graph" => {
            "Representative `.mir` imports are explanatory; current executable input remains versioned package manifests."
        }
        "package_dependency_graph" => {
            "Package dependencies remain explicit and versioned; compatibility-preserving overlays are not inserted silently."
        }
        "projection_target_graph" => {
            "Projection targets, packet boundaries, and FFI boundaries are intent-level inventory only in this alpha line."
        }
        "server_client_process_graph" => {
            "Current execution remains bounded local/same-session; final server/client binary split is kept later."
        }
        "redaction_toggle" => {
            "Observer-safe view is available; admin/debug is explicitly kept later."
        }
        "message_failure_recovery" => {
            "Bounded timeout/retry/reject observations are linked from the session DAG."
        }
        "save_load_quiescent_timeline" => {
            "Local R0 and bounded R2 save/load evidence is session-carried."
        }
        "message_route_graph" => {
            "Transport, auth, membership, capability, and witness lanes remain separate."
        }
        "contract_effect_failure_summary" => {
            "Effect/failure rows are explicit and remain separate from capability and witness authority."
        }
        "portal_graph_future" => {
            "Portal/world-link remains a near-term future boundary, not a current WAN or continuous sync claim."
        }
        "shard_map_future" => {
            "Spatial shard replication remains a future hard-boundary profile, not current infinite federation."
        }
        _ => "Panel is derived from the same product alpha-1 session carrier.",
    }
}

fn panel_payload_json(bundle: &ProductAlpha1DevtoolsBundle, panel_id: &str) -> String {
    let payload = match panel_id {
        "product_overview" => serde_json::to_value(&bundle.panels.product_overview),
        "source_import_graph" => serde_json::to_value(&bundle.panels.source_import_graph),
        "package_dependency_graph" => serde_json::to_value(&bundle.panels.package_dependency_graph),
        "projection_target_graph" => serde_json::to_value(&bundle.panels.projection_target_graph),
        "place_graph" => serde_json::to_value(&bundle.panels.place_graph),
        "server_client_process_graph" => {
            serde_json::to_value(&bundle.panels.server_client_process_graph)
        }
        "event_dag" => serde_json::to_value(&bundle.panels.event_dag),
        "message_route_graph" => serde_json::to_value(&bundle.panels.message_route_graph),
        "membership_frontier_timeline" => {
            serde_json::to_value(&bundle.panels.membership_frontier_timeline)
        }
        "witness_relation_timeline" => {
            serde_json::to_value(&bundle.panels.witness_relation_timeline)
        }
        "hotplug_lifecycle" => serde_json::to_value(&bundle.panels.hotplug_lifecycle),
        "save_load_quiescent_timeline" => {
            serde_json::to_value(&bundle.panels.save_load_quiescent_timeline)
        }
        "contract_effect_failure_summary" => {
            serde_json::to_value(&bundle.panels.contract_effect_failure_summary)
        }
        "message_failure_recovery" => serde_json::to_value(&bundle.panels.message_failure_recovery),
        "fallback_degradation" => serde_json::to_value(&bundle.panels.fallback_degradation),
        "auth_capability_decision" => serde_json::to_value(&bundle.panels.auth_capability_decision),
        "redaction_toggle" => serde_json::to_value(&bundle.panels.redaction_toggle),
        "retention_trace" => serde_json::to_value(&bundle.panels.retention_trace),
        "portal_graph_future" => serde_json::to_value(&bundle.panels.portal_graph_future),
        "shard_map_future" => serde_json::to_value(&bundle.panels.shard_map_future),
        _ => serde_json::to_value(panel_summary(panel_id)),
    };
    payload
        .and_then(|value| serde_json::to_string_pretty(&value))
        .unwrap_or_else(|error| format!("{{\"serialization_error\":\"{}\"}}", error))
}

fn source_import_graph_panel(
    session: &ProductAlpha1SessionCarrier,
) -> ProductAlpha1SourceImportGraphPanel {
    let (nodes, edges) = canonical_operational_chain(
        &session.runtime_plan.package_kind,
        &session.package_id,
        "source_module",
        "representative_source_only",
        "import_chain",
    );
    ProductAlpha1SourceImportGraphPanel {
        executable_input_kind: "versioned_package_mir_json".to_string(),
        representative_source_status: "textual_mir_representative_only".to_string(),
        nodes,
        edges,
    }
}

fn package_dependency_graph_panel(
    session: &ProductAlpha1SessionCarrier,
) -> ProductAlpha1PackageDependencyGraphPanel {
    let (mut nodes, mut edges) = canonical_operational_chain(
        &session.runtime_plan.package_kind,
        &session.package_id,
        "package",
        "checked_dependency",
        "package_dependency",
    );
    if nodes.len() == 1 {
        for dependency in &session.runtime_plan.declared_dependencies {
            let node_id = dependency_node_id(dependency);
            nodes.push(ProductAlpha1NamedGraphNode {
                node_id: node_id.clone(),
                node_kind: "package".to_string(),
                current_status: "declared_dependency".to_string(),
            });
            edges.push(ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: node_id,
                relation: "package_dependency".to_string(),
            });
        }
    }
    ProductAlpha1PackageDependencyGraphPanel {
        package_root: session.package_id.clone(),
        direct_dependency_count: session.runtime_plan.declared_dependencies.len(),
        nodes,
        edges,
    }
}

fn projection_target_graph_panel(
    session: &ProductAlpha1SessionCarrier,
) -> ProductAlpha1ProjectionTargetGraphPanel {
    let operational = is_operational_product_package(&session.runtime_plan.package_kind);
    let nodes = if operational {
        vec![
            ProductAlpha1NamedGraphNode {
                node_id: session.package_id.clone(),
                node_kind: "source_package".to_string(),
                current_status: "runnable_alpha_input".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "target#server-host".to_string(),
                node_kind: "projection_target".to_string(),
                current_status: "intent_only".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "target#participant-client".to_string(),
                node_kind: "projection_target".to_string(),
                current_status: "intent_only".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "target#native-host-launch-bundle".to_string(),
                node_kind: "host_launch_bundle".to_string(),
                current_status: "implemented_for_root_package".to_string(),
            },
        ]
    } else {
        vec![
            ProductAlpha1NamedGraphNode {
                node_id: session.package_id.clone(),
                node_kind: "source_package".to_string(),
                current_status: "runnable_alpha_input".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "target#same-session-local".to_string(),
                node_kind: "runtime_target".to_string(),
                current_status: "implemented".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "target#native-host-launch-bundle".to_string(),
                node_kind: "host_launch_bundle".to_string(),
                current_status: "implemented_for_root_package".to_string(),
            },
        ]
    };
    let edges = if operational {
        vec![
            ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: "target#server-host".to_string(),
                relation: "projection_intent".to_string(),
            },
            ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: "target#participant-client".to_string(),
                relation: "projection_intent".to_string(),
            },
            ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: "target#native-host-launch-bundle".to_string(),
                relation: "implemented_host_bundle".to_string(),
            },
        ]
    } else {
        vec![
            ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: "target#same-session-local".to_string(),
                relation: "current_runtime_path".to_string(),
            },
            ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: "target#native-host-launch-bundle".to_string(),
                relation: "implemented_host_bundle".to_string(),
            },
        ]
    };
    ProductAlpha1ProjectionTargetGraphPanel {
        projection_status: if operational {
            "future_projection_boundary".to_string()
        } else {
            "local_runtime_plus_host_bundle".to_string()
        },
        target_count: nodes.len().saturating_sub(1),
        packet_boundary_count: if operational { 2 } else { 1 },
        ffi_boundary_count: 1,
        llvm_codegen_claimed: false,
        nodes,
        edges,
    }
}

fn server_client_process_graph_panel(
    session: &ProductAlpha1SessionCarrier,
) -> ProductAlpha1ServerClientProcessGraphPanel {
    let operational = is_operational_product_package(&session.runtime_plan.package_kind);
    let mut nodes = vec![ProductAlpha1NamedGraphNode {
        node_id: "process#world-server".to_string(),
        node_kind: "server_process".to_string(),
        current_status: if operational {
            "same_session_now_projection_later".to_string()
        } else {
            "same_session_local_process".to_string()
        },
    }];
    let mut edges = vec![ProductAlpha1NamedGraphEdge {
        from_node: "process#world-server".to_string(),
        to_node: "process#host-adapter".to_string(),
        relation: "typed_external_boundary".to_string(),
    }];
    if operational {
        nodes.push(ProductAlpha1NamedGraphNode {
            node_id: "process#participant-client".to_string(),
            node_kind: "client_process".to_string(),
            current_status: "projection_intent_only".to_string(),
        });
        edges.push(ProductAlpha1NamedGraphEdge {
            from_node: "process#world-server".to_string(),
            to_node: "process#participant-client".to_string(),
            relation: "message_envelope_projection".to_string(),
        });
    }
    nodes.push(ProductAlpha1NamedGraphNode {
        node_id: "process#host-adapter".to_string(),
        node_kind: "host_adapter".to_string(),
        current_status: "implemented".to_string(),
    });
    ProductAlpha1ServerClientProcessGraphPanel {
        current_execution_mode: if operational {
            "local_same_session_runtime_with_projection_inventory".to_string()
        } else {
            "local_same_session_runtime".to_string()
        },
        final_binary_split_claimed: false,
        nodes,
        edges,
    }
}

fn portal_graph_future_panel(
    session: &ProductAlpha1SessionCarrier,
) -> ProductAlpha1FutureBoundaryPanel {
    ProductAlpha1FutureBoundaryPanel {
        current_status: if is_operational_product_package(&session.runtime_plan.package_kind) {
            "planned_manifest_only".to_string()
        } else {
            "kept_later".to_string()
        },
        nodes: vec![
            ProductAlpha1NamedGraphNode {
                node_id: session.package_id.clone(),
                node_kind: "current_world".to_string(),
                current_status: "current_root".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "portal#world-link".to_string(),
                node_kind: "portal_boundary".to_string(),
                current_status: "future_inventory".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "world#destination".to_string(),
                node_kind: "future_world".to_string(),
                current_status: "future_inventory".to_string(),
            },
        ],
        edges: vec![
            ProductAlpha1NamedGraphEdge {
                from_node: session.package_id.clone(),
                to_node: "portal#world-link".to_string(),
                relation: "discrete_handoff".to_string(),
            },
            ProductAlpha1NamedGraphEdge {
                from_node: "portal#world-link".to_string(),
                to_node: "world#destination".to_string(),
                relation: "future_world_link".to_string(),
            },
        ],
        non_claims: vec![
            "no WAN federation".to_string(),
            "no continuous spatial sync".to_string(),
            "no final portal ABI".to_string(),
        ],
    }
}

fn shard_map_future_panel(
    session: &ProductAlpha1SessionCarrier,
) -> ProductAlpha1FutureBoundaryPanel {
    ProductAlpha1FutureBoundaryPanel {
        current_status: if is_operational_product_package(&session.runtime_plan.package_kind) {
            "planned_only".to_string()
        } else {
            "kept_later".to_string()
        },
        nodes: vec![
            ProductAlpha1NamedGraphNode {
                node_id: "shard#A".to_string(),
                node_kind: "authority_shard".to_string(),
                current_status: "future_inventory".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "shard#B".to_string(),
                node_kind: "authority_shard".to_string(),
                current_status: "future_inventory".to_string(),
            },
            ProductAlpha1NamedGraphNode {
                node_id: "observer#gradient-view".to_string(),
                node_kind: "observation_profile".to_string(),
                current_status: "later_than_two_shard_cut".to_string(),
            },
        ],
        edges: vec![
            ProductAlpha1NamedGraphEdge {
                from_node: "shard#A".to_string(),
                to_node: "shard#B".to_string(),
                relation: "hard_boundary_handoff".to_string(),
            },
            ProductAlpha1NamedGraphEdge {
                from_node: "shard#A".to_string(),
                to_node: "observer#gradient-view".to_string(),
                relation: "future_gradient_observation".to_string(),
            },
        ],
        non_claims: vec![
            "no global participant vector clock default".to_string(),
            "no continuous infinite federation implementation".to_string(),
            "no distributed durable save/load".to_string(),
        ],
    }
}

fn canonical_operational_chain(
    package_kind: &str,
    package_id: &str,
    node_kind: &str,
    dependency_status: &str,
    relation: &str,
) -> (
    Vec<ProductAlpha1NamedGraphNode>,
    Vec<ProductAlpha1NamedGraphEdge>,
) {
    match package_kind {
        "world_core" => (
            vec![ProductAlpha1NamedGraphNode {
                node_id: "WorldCore".to_string(),
                node_kind: node_kind.to_string(),
                current_status: "implemented".to_string(),
            }],
            Vec::new(),
        ),
        "membership_chat" => (
            vec![
                ProductAlpha1NamedGraphNode {
                    node_id: "WorldCore".to_string(),
                    node_kind: node_kind.to_string(),
                    current_status: dependency_status.to_string(),
                },
                ProductAlpha1NamedGraphNode {
                    node_id: "MembershipChat".to_string(),
                    node_kind: node_kind.to_string(),
                    current_status: "implemented".to_string(),
                },
            ],
            vec![ProductAlpha1NamedGraphEdge {
                from_node: "WorldCore".to_string(),
                to_node: "MembershipChat".to_string(),
                relation: relation.to_string(),
            }],
        ),
        "sugoroku_world" => (
            vec![
                ProductAlpha1NamedGraphNode {
                    node_id: "WorldCore".to_string(),
                    node_kind: node_kind.to_string(),
                    current_status: dependency_status.to_string(),
                },
                ProductAlpha1NamedGraphNode {
                    node_id: "MembershipChat".to_string(),
                    node_kind: node_kind.to_string(),
                    current_status: dependency_status.to_string(),
                },
                ProductAlpha1NamedGraphNode {
                    node_id: "SugorokuWorld".to_string(),
                    node_kind: node_kind.to_string(),
                    current_status: "implemented".to_string(),
                },
            ],
            vec![
                ProductAlpha1NamedGraphEdge {
                    from_node: "WorldCore".to_string(),
                    to_node: "MembershipChat".to_string(),
                    relation: relation.to_string(),
                },
                ProductAlpha1NamedGraphEdge {
                    from_node: "MembershipChat".to_string(),
                    to_node: "SugorokuWorld".to_string(),
                    relation: relation.to_string(),
                },
            ],
        ),
        _ => (
            vec![ProductAlpha1NamedGraphNode {
                node_id: package_id.to_string(),
                node_kind: node_kind.to_string(),
                current_status: "implemented".to_string(),
            }],
            Vec::new(),
        ),
    }
}

fn dependency_node_id(dependency: &str) -> String {
    dependency
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or(dependency)
        .to_string()
}

fn is_operational_product_package(package_kind: &str) -> bool {
    matches!(
        package_kind,
        "world_core" | "membership_chat" | "sugoroku_world"
    )
}

fn observer_safe_place_graph_panel(
    nodes: &[ProductAlpha1PlaceNode],
    edges: &[ProductAlpha1PlaceEdge],
) -> ProductAlpha1PlaceGraphPanel {
    let node_summaries = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| ProductAlpha1PlaceNodeSummary {
            observer_safe_place_ref: observer_safe_place_ref(node, index),
            place_kind: node.place_kind.clone(),
            private_place_ref_redacted: is_private_place_ref(node),
        })
        .collect();
    let edge_summaries = edges
        .iter()
        .map(|edge| ProductAlpha1PlaceEdgeSummary {
            from_place: observer_safe_edge_place_ref(nodes, &edge.from_place),
            to_place: observer_safe_edge_place_ref(nodes, &edge.to_place),
            relation: edge.relation.clone(),
        })
        .collect();
    ProductAlpha1PlaceGraphPanel {
        nodes: node_summaries,
        edges: edge_summaries,
    }
}

fn observer_safe_edge_place_ref(nodes: &[ProductAlpha1PlaceNode], place_id: &str) -> String {
    nodes
        .iter()
        .enumerate()
        .find(|(_, node)| node.place_id == place_id)
        .map(|(index, node)| observer_safe_place_ref(node, index))
        .unwrap_or_else(|| "redacted_place#unknown".to_string())
}

fn observer_safe_place_ref(node: &ProductAlpha1PlaceNode, index: usize) -> String {
    if is_private_place_ref(node) {
        format!("participant_place#{}", index + 1)
    } else {
        node.place_id.clone()
    }
}

fn is_private_place_ref(node: &ProductAlpha1PlaceNode) -> bool {
    node.place_kind.contains("Participant") || node.place_id.contains("ParticipantPlace[")
}

fn observer_safe_reason_families(reason_refs: &[String]) -> Vec<String> {
    let mut families = reason_refs
        .iter()
        .map(|reason| {
            reason
                .split_once(':')
                .map(|(family, _)| family)
                .unwrap_or(reason.as_str())
                .to_string()
        })
        .collect::<Vec<_>>();
    families.sort();
    families.dedup();
    families
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn devtools_surface_kind() -> String {
    PRODUCT_ALPHA1_DEVTOOLS_SURFACE_KIND.to_string()
}
