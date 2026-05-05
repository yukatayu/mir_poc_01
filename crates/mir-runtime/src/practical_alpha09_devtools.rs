use std::{collections::BTreeSet, fmt, path::PathBuf};

use serde::Serialize;

use crate::{
    alpha_local_runtime::EventDagExport,
    practical_alpha05_session::{
        PRACTICAL_ALPHA05_SESSION_SCOPE, PRACTICAL_ALPHA05_SESSION_SURFACE_KIND,
        PracticalAlpha05ObserverSafeExport, PracticalAlpha05SessionHotPlugLifecycleEntry,
        PracticalAlpha05SessionReport,
    },
};

pub const PRACTICAL_ALPHA09_DEVTOOLS_SCOPE: &str = "practical-alpha09-session-devtools-floor";
pub const PRACTICAL_ALPHA09_DEVTOOLS_SURFACE_KIND: &str =
    "practical_alpha09_nonfinal_session_devtools_export";
pub const PRACTICAL_ALPHA09_DEVTOOLS_SCOPE_KIND: &str = "alpha_local";
pub const PRACTICAL_ALPHA09_VIEWER_MODE: &str = "nonfinal_static_html_session_viewer";

const PRACTICAL_ALPHA09_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-0.9 session devtools export as a final public viewer or telemetry ABI",
    "do not treat session-local retention/on-demand trace as durable audit or remote retrieval completion",
    "do not treat local route trace as WAN/federation transport completion",
];

const PRACTICAL_ALPHA09_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical alpha-0.9 session-bound devtools floor only",
    "export is derived from one practical session carrier and its session-carried observation journal",
    "admin/full debug view is explicitly kept-later",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha09DevtoolsErrorKind {
    SessionScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha09DevtoolsError {
    pub kind: PracticalAlpha09DevtoolsErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha09DevtoolsError {
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

impl std::error::Error for PracticalAlpha09DevtoolsError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09Panel {
    pub panel_id: String,
    pub panel_kind: String,
    pub label: String,
    pub authority: String,
    pub redaction: String,
    pub retention_scope: String,
    pub source_session_ref: String,
    pub focus_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09TelemetryRow {
    pub telemetry_id: String,
    pub telemetry_kind: String,
    pub label: String,
    pub authority: String,
    pub redaction: String,
    pub retention_scope: String,
    pub source_session_ref: String,
    pub channel: String,
    pub value_summary: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09RouteTraceEntry {
    pub route_id: String,
    pub hop_index: usize,
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub payload_kind: String,
    pub dispatch_outcome: String,
    pub source_kind: String,
    pub event_refs: Vec<String>,
    pub authority: String,
    pub redaction: String,
    pub retention_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09MembershipTimelineEntry {
    pub entry_id: String,
    pub source_kind: String,
    pub membership_epoch: u64,
    pub active_participants: Vec<String>,
    pub member_incarnations: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09WitnessRelationEntry {
    pub relation_id: String,
    pub envelope_id: String,
    pub witness_refs: Vec<String>,
    pub generated_event_refs: Vec<String>,
    pub visible_history_refs: Vec<String>,
    pub authority: String,
    pub redaction: String,
    pub retention_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09FallbackDegradationEntry {
    pub degradation_id: String,
    pub source_sample_id: String,
    pub source_package_id: String,
    pub source_terminal_outcome: String,
    pub selected_representation: String,
    pub fallback_representation: String,
    pub fallback_reason: String,
    pub native_execution_performed: bool,
    pub degraded_roles: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09SaveLoadTimelineEntry {
    pub entry_id: String,
    pub savepoint_id: String,
    pub saved_state_format: String,
    pub saved_membership_epoch: u64,
    pub saved_event_count: usize,
    pub state_roundtrip_equal: bool,
    pub saved_active_layers: Vec<String>,
    pub saved_hotplug_event_count: usize,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09RetentionQueryEntry {
    pub query_id: String,
    pub selector: String,
    pub query_mode: String,
    pub query_outcome: String,
    pub retention_scope: String,
    pub redaction: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09ExportSections {
    pub event_dag: EventDagExport,
    pub route_trace: Vec<PracticalAlpha09RouteTraceEntry>,
    pub membership_timeline: Vec<PracticalAlpha09MembershipTimelineEntry>,
    pub witness_relation: Vec<PracticalAlpha09WitnessRelationEntry>,
    pub hotplug_lifecycle: Vec<PracticalAlpha05SessionHotPlugLifecycleEntry>,
    pub fallback_degradation: Vec<PracticalAlpha09FallbackDegradationEntry>,
    pub save_load_timeline: Vec<PracticalAlpha09SaveLoadTimelineEntry>,
    pub redacted_observer_view: PracticalAlpha05ObserverSafeExport,
    pub retention_query_trace: Vec<PracticalAlpha09RetentionQueryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha09SessionDevtoolsReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "devtools_scope")]
    pub devtools_scope: String,
    pub session_scope: String,
    pub session_id: String,
    pub source_session_surface_kind: String,
    pub export_source_kind: String,
    pub viewer_mode: String,
    pub observer_authority: String,
    pub redaction_policy: String,
    pub retention_scope: String,
    pub on_demand_trace_mode: String,
    pub admin_debug_view_status: String,
    pub panels: Vec<PracticalAlpha09Panel>,
    pub telemetry_rows: Vec<PracticalAlpha09TelemetryRow>,
    pub panel_ids: Vec<String>,
    pub panel_kinds: Vec<String>,
    pub telemetry_ids: Vec<String>,
    pub telemetry_kinds: Vec<String>,
    pub export_sections: PracticalAlpha09ExportSections,
    pub what_it_proves: Vec<String>,
    pub what_it_does_not_prove: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
    pub session_bound_export_claimed: bool,
    pub operational_alpha09_export_ready: bool,
    pub final_public_viewer_frozen: bool,
    pub durable_audit_claimed: bool,
}

pub fn export_practical_alpha09_devtools(
    session: &PracticalAlpha05SessionReport,
) -> Result<PracticalAlpha09SessionDevtoolsReport, PracticalAlpha09DevtoolsError> {
    if session.session_scope != PRACTICAL_ALPHA05_SESSION_SCOPE {
        return Err(PracticalAlpha09DevtoolsError {
            kind: PracticalAlpha09DevtoolsErrorKind::SessionScope,
            path: PathBuf::from("<inline>"),
            detail: format!(
                "alpha-0.9 devtools export requires session_scope `{PRACTICAL_ALPHA05_SESSION_SCOPE}`, found `{}`",
                session.session_scope
            ),
        });
    }

    let source_session_ref = format!("{}:{}", session.session_scope, session.session_id);
    let route_trace = build_route_trace(session);
    let membership_timeline = build_membership_timeline(session);
    let witness_relation = build_witness_relation(session);
    let fallback_degradation = build_fallback_degradation(session);
    let save_load_timeline = build_save_load_timeline(session);
    let retention_query_trace = build_retention_query_trace(session);
    let export_sections = PracticalAlpha09ExportSections {
        event_dag: session.event_dag.clone(),
        route_trace,
        membership_timeline,
        witness_relation,
        hotplug_lifecycle: session.hotplug_lifecycle.clone(),
        fallback_degradation,
        save_load_timeline,
        redacted_observer_view: session.observer_safe_export.clone(),
        retention_query_trace,
    };
    let panels = build_panels(session, &source_session_ref, &export_sections);
    let telemetry_rows = build_telemetry_rows(session, &source_session_ref, &export_sections);
    let panel_ids = panels.iter().map(|panel| panel.panel_id.clone()).collect();
    let panel_kinds = sorted_unique(panels.iter().map(|panel| panel.panel_kind.clone()));
    let telemetry_ids = telemetry_rows
        .iter()
        .map(|row| row.telemetry_id.clone())
        .collect();
    let telemetry_kinds =
        sorted_unique(telemetry_rows.iter().map(|row| row.telemetry_kind.clone()));
    let operational_alpha09_export_ready = has_required_panels(&panels)
        && !export_sections.route_trace.is_empty()
        && !export_sections.witness_relation.is_empty()
        && !export_sections.save_load_timeline.is_empty()
        && !export_sections.retention_query_trace.is_empty();

    Ok(PracticalAlpha09SessionDevtoolsReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        devtools_scope: devtools_scope(),
        session_scope: session.session_scope.clone(),
        session_id: session.session_id.clone(),
        source_session_surface_kind: PRACTICAL_ALPHA05_SESSION_SURFACE_KIND.to_string(),
        export_source_kind: "same_runtime_session_carrier".to_string(),
        viewer_mode: PRACTICAL_ALPHA09_VIEWER_MODE.to_string(),
        observer_authority: format!("ObserveSessionDevtools({})", session.entry_place),
        redaction_policy: "observer_safe_session_summary".to_string(),
        retention_scope: "session_local_ephemeral".to_string(),
        on_demand_trace_mode: "explicit_session_local_query_trace".to_string(),
        admin_debug_view_status: "kept_later".to_string(),
        panels,
        telemetry_rows,
        panel_ids,
        panel_kinds,
        telemetry_ids,
        telemetry_kinds,
        export_sections,
        what_it_proves: vec![
            "one practical runtime session can emit event DAG, route, membership, witness, hot-plug, fallback, save/load, redacted observer, and retention trace panels".to_string(),
            "rejected hot-plug attempts remain session-carried observation entries without mutating active layer or object state".to_string(),
            "the export is session-bound rather than exact report recomposition".to_string(),
        ],
        what_it_does_not_prove: vec![
            "final public viewer or telemetry ABI".to_string(),
            "durable audit backend or remote retained-artifact retrieval".to_string(),
            "WAN/federation route tracing".to_string(),
            "native avatar execution or unsupported-runtime execution success".to_string(),
            "distributed durable save/load".to_string(),
        ],
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        session_bound_export_claimed: true,
        operational_alpha09_export_ready,
        final_public_viewer_frozen: false,
        durable_audit_claimed: false,
    })
}

fn build_route_trace(
    session: &PracticalAlpha05SessionReport,
) -> Vec<PracticalAlpha09RouteTraceEntry> {
    session
        .dispatch_records
        .iter()
        .map(|record| PracticalAlpha09RouteTraceEntry {
            route_id: format!("local_route#{}", record.dispatch_order),
            hop_index: record.dispatch_order,
            envelope_id: record.envelope_id.clone(),
            from_place: record.from_place.clone(),
            to_place: record.to_place.clone(),
            payload_kind: record.payload_kind.clone(),
            dispatch_outcome: record.dispatch_outcome.clone(),
            source_kind: "session_dispatch_record".to_string(),
            event_refs: record.generated_event_refs.clone(),
            authority: format!("ObserveLocalRoute({})", session.entry_place),
            redaction: "observer_safe_route_trace".to_string(),
            retention_scope: "session_local_ephemeral".to_string(),
        })
        .collect()
}

fn build_membership_timeline(
    session: &PracticalAlpha05SessionReport,
) -> Vec<PracticalAlpha09MembershipTimelineEntry> {
    let mut entries = vec![membership_entry(
        "current_membership_frontier",
        "current_session_runtime_snapshot",
        session.runtime_snapshot.membership.membership_epoch,
        session
            .runtime_snapshot
            .membership
            .members
            .iter()
            .filter_map(|(principal, member)| member.active.then_some(principal.clone()))
            .collect(),
        session
            .runtime_snapshot
            .membership
            .members
            .iter()
            .map(|(principal, member)| format!("{principal}:{}", member.incarnation))
            .collect(),
        vec!["current session membership frontier".to_string()],
    )];

    for savepoint in &session.savepoints {
        entries.push(membership_entry(
            &format!("saved_membership_frontier#{}", savepoint.savepoint_id),
            "session_savepoint",
            savepoint.saved_runtime_snapshot.membership.membership_epoch,
            savepoint
                .saved_runtime_snapshot
                .membership
                .members
                .iter()
                .filter_map(|(principal, member)| member.active.then_some(principal.clone()))
                .collect(),
            savepoint
                .saved_runtime_snapshot
                .membership
                .members
                .iter()
                .map(|(principal, member)| format!("{principal}:{}", member.incarnation))
                .collect(),
            vec![
                format!("savepoint_id={}", savepoint.savepoint_id),
                "local-only saved frontier; not distributed durable checkpoint".to_string(),
            ],
        ));
    }
    entries
}

fn membership_entry(
    entry_id: &str,
    source_kind: &str,
    membership_epoch: u64,
    active_participants: Vec<String>,
    member_incarnations: Vec<String>,
    notes: Vec<String>,
) -> PracticalAlpha09MembershipTimelineEntry {
    PracticalAlpha09MembershipTimelineEntry {
        entry_id: entry_id.to_string(),
        source_kind: source_kind.to_string(),
        membership_epoch,
        active_participants,
        member_incarnations,
        notes,
    }
}

fn build_witness_relation(
    session: &PracticalAlpha05SessionReport,
) -> Vec<PracticalAlpha09WitnessRelationEntry> {
    session
        .dispatch_records
        .iter()
        .filter(|record| !record.witness_refs.is_empty())
        .map(|record| PracticalAlpha09WitnessRelationEntry {
            relation_id: format!("witness_relation#{}", record.dispatch_order),
            envelope_id: record.envelope_id.clone(),
            witness_refs: record.witness_refs.clone(),
            generated_event_refs: record.generated_event_refs.clone(),
            visible_history_refs: session.visible_history.clone(),
            authority: format!("InspectWitnessRelation({})", session.entry_place),
            redaction: "witness_ref_summary".to_string(),
            retention_scope: "session_local_ephemeral".to_string(),
        })
        .collect()
}

fn build_fallback_degradation(
    session: &PracticalAlpha05SessionReport,
) -> Vec<PracticalAlpha09FallbackDegradationEntry> {
    let mut entries: Vec<PracticalAlpha09FallbackDegradationEntry> = session
        .object_preview_inventory
        .iter()
        .map(|preview| PracticalAlpha09FallbackDegradationEntry {
            degradation_id: format!("object_preview#{}", preview.sample_id),
            source_sample_id: preview.sample_id.clone(),
            source_package_id: preview.package_id.clone(),
            source_terminal_outcome: "accepted_object_attach_preview".to_string(),
            selected_representation: preview.selected_representation.clone(),
            fallback_representation: preview.fallback_representation.clone(),
            fallback_reason: "object_attach_preview_not_native_execution".to_string(),
            native_execution_performed: false,
            degraded_roles: preview.provided_roles.clone(),
            notes: preview.notes.clone(),
        })
        .collect();

    for entry in &session.hotplug_lifecycle {
        if entry.terminal_outcome != "rejected" {
            continue;
        }
        let fallback_reason = entry
            .rejection_reason_refs
            .iter()
            .find(|reason| reason.starts_with("missing_host_capability:"))
            .cloned()
            .unwrap_or_else(|| {
                entry
                    .rejection_reason_refs
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "rejected_runtime_request".to_string())
            });
        entries.push(PracticalAlpha09FallbackDegradationEntry {
            degradation_id: format!("rejected_fallback#{}", entry.sample_id),
            source_sample_id: entry.sample_id.clone(),
            source_package_id: entry.package_id.clone(),
            source_terminal_outcome: entry.terminal_outcome.clone(),
            selected_representation: "rejected_runtime_request".to_string(),
            fallback_representation: "static_capsule_placeholder".to_string(),
            fallback_reason,
            native_execution_performed: false,
            degraded_roles: vec![
                "Animatable".to_string(),
                "AttachmentProvider".to_string(),
                "ExpressionProvider".to_string(),
            ],
            notes: vec![
                "session-carried rejection is exported as visible fallback degradation evidence"
                    .to_string(),
                "unsupported runtime execution remains not performed".to_string(),
            ],
        });
    }
    entries
}

fn build_save_load_timeline(
    session: &PracticalAlpha05SessionReport,
) -> Vec<PracticalAlpha09SaveLoadTimelineEntry> {
    session
        .savepoints
        .iter()
        .map(|savepoint| PracticalAlpha09SaveLoadTimelineEntry {
            entry_id: format!("save_load#{}", savepoint.savepoint_id),
            savepoint_id: savepoint.savepoint_id.clone(),
            saved_state_format: savepoint.saved_state_format.clone(),
            saved_membership_epoch: savepoint.saved_runtime_snapshot.membership.membership_epoch,
            saved_event_count: savepoint.saved_event_dag.nodes.len(),
            state_roundtrip_equal: savepoint.state_roundtrip_equal,
            saved_active_layers: savepoint.saved_active_layers.clone(),
            saved_hotplug_event_count: savepoint.saved_hotplug_lifecycle.len(),
            notes: savepoint.notes.clone(),
        })
        .collect()
}

fn build_retention_query_trace(
    session: &PracticalAlpha05SessionReport,
) -> Vec<PracticalAlpha09RetentionQueryEntry> {
    vec![
        retention_query(
            "trace_query#event_dag",
            "export_sections.event_dag",
            "hit",
            "event_structure_only",
            vec![format!("event_count={}", session.event_dag.nodes.len())],
        ),
        retention_query(
            "trace_query#observer_safe_view",
            "export_sections.redacted_observer_view",
            "hit",
            "observer_safe_session_summary",
            vec![format!(
                "visible_history_count={}",
                session.observer_safe_export.visible_history.len()
            )],
        ),
        retention_query(
            "trace_query#raw_auth_payload",
            "raw_auth_payload",
            "miss",
            "raw_privileged_payload_redacted",
            vec![
                "raw auth evidence is not retained in the observer-safe session export".to_string(),
            ],
        ),
    ]
}

fn retention_query(
    query_id: &str,
    selector: &str,
    query_outcome: &str,
    redaction: &str,
    notes: Vec<String>,
) -> PracticalAlpha09RetentionQueryEntry {
    PracticalAlpha09RetentionQueryEntry {
        query_id: query_id.to_string(),
        selector: selector.to_string(),
        query_mode: "session_local_on_demand_trace".to_string(),
        query_outcome: query_outcome.to_string(),
        retention_scope: "session_local_ephemeral".to_string(),
        redaction: redaction.to_string(),
        notes,
    }
}

fn build_panels(
    session: &PracticalAlpha05SessionReport,
    source_session_ref: &str,
    sections: &PracticalAlpha09ExportSections,
) -> Vec<PracticalAlpha09Panel> {
    vec![
        panel(
            "event_dag_live_session",
            "event_dag",
            "alpha09:event-dag-live-session",
            format!("InspectEventDag({})", session.entry_place),
            "event_structure_only",
            source_session_ref,
            sections
                .event_dag
                .nodes
                .iter()
                .map(|node| node.event_id.clone())
                .collect(),
            vec!["live/session event DAG export; not exact report recomposition".to_string()],
        ),
        panel(
            "local_route_trace",
            "route_trace",
            "alpha09:local-route-trace",
            format!("ObserveLocalRoute({})", session.entry_place),
            "observer_safe_route_trace",
            source_session_ref,
            sections
                .route_trace
                .iter()
                .map(|entry| entry.envelope_id.clone())
                .collect(),
            vec![
                "local route trace is session-local and does not claim WAN/federation".to_string(),
            ],
        ),
        panel(
            "membership_timeline",
            "membership_timeline",
            "alpha09:membership-timeline",
            format!("InspectMembershipTimeline({})", session.entry_place),
            "membership_frontier_summary",
            source_session_ref,
            sections
                .membership_timeline
                .iter()
                .map(|entry| entry.entry_id.clone())
                .collect(),
            vec!["timeline contains current session and local savepoint frontiers".to_string()],
        ),
        panel(
            "witness_relation",
            "witness_relation",
            "alpha09:witness-relation",
            format!("InspectWitnessRelation({})", session.entry_place),
            "witness_ref_summary",
            source_session_ref,
            sections
                .witness_relation
                .iter()
                .flat_map(|entry| entry.witness_refs.clone())
                .collect(),
            vec!["witness payloads remain redacted to refs".to_string()],
        ),
        panel(
            "hotplug_lifecycle",
            "hotplug_lifecycle",
            "alpha09:hotplug-lifecycle",
            format!("InspectHotPlugLifecycle({})", session.entry_place),
            "hotplug_boundary_summary",
            source_session_ref,
            sections
                .hotplug_lifecycle
                .iter()
                .map(|entry| format!("{}:{}", entry.sample_id, entry.terminal_outcome))
                .collect(),
            vec![
                "accepted, rejected, and deferred hot-plug attempts are session-carried"
                    .to_string(),
            ],
        ),
        panel(
            "fallback_degradation",
            "fallback_degradation",
            "alpha09:fallback-degradation",
            format!("InspectFallbackDegradation({})", session.entry_place),
            "degraded_role_summary",
            source_session_ref,
            sections
                .fallback_degradation
                .iter()
                .map(|entry| entry.degradation_id.clone())
                .collect(),
            vec!["fallback visibility does not imply native execution".to_string()],
        ),
        panel(
            "save_load_timeline",
            "save_load_timeline",
            "alpha09:save-load-timeline",
            format!("InspectSaveLoadTimeline({})", session.entry_place),
            "local_frontier_summary",
            source_session_ref,
            sections
                .save_load_timeline
                .iter()
                .map(|entry| entry.savepoint_id.clone())
                .collect(),
            vec!["local-only save/load timeline; not distributed durable checkpoint".to_string()],
        ),
        panel(
            "observer_safe_redacted_view",
            "redacted_observer_view",
            "alpha09:observer-safe-redacted-view",
            format!("ObserveSession({})", session.entry_place),
            "observer_safe_session_summary",
            source_session_ref,
            sections.redacted_observer_view.event_ids.clone(),
            vec!["raw witness/auth payloads are not exported".to_string()],
        ),
        panel(
            "retention_on_demand_trace",
            "retention_on_demand_trace",
            "alpha09:retention-on-demand-trace",
            format!("QuerySessionTrace({})", session.entry_place),
            "session_local_trace_summary",
            source_session_ref,
            sections
                .retention_query_trace
                .iter()
                .map(|entry| entry.query_id.clone())
                .collect(),
            vec!["query trace is session-local and non-durable".to_string()],
        ),
    ]
}

fn panel(
    panel_id: &str,
    panel_kind: &str,
    label: &str,
    authority: String,
    redaction: &str,
    source_session_ref: &str,
    focus_refs: Vec<String>,
    notes: Vec<String>,
) -> PracticalAlpha09Panel {
    PracticalAlpha09Panel {
        panel_id: panel_id.to_string(),
        panel_kind: panel_kind.to_string(),
        label: label.to_string(),
        authority,
        redaction: redaction.to_string(),
        retention_scope: "session_local_ephemeral".to_string(),
        source_session_ref: source_session_ref.to_string(),
        focus_refs,
        notes,
    }
}

fn build_telemetry_rows(
    session: &PracticalAlpha05SessionReport,
    source_session_ref: &str,
    sections: &PracticalAlpha09ExportSections,
) -> Vec<PracticalAlpha09TelemetryRow> {
    let mut rows = Vec::new();
    for node in &sections.event_dag.nodes {
        rows.push(telemetry(
            &node.event_id,
            &node.event_kind,
            format!("alpha09:event:{}", node.event_kind),
            format!("InspectEventDag({})", session.entry_place),
            "event_structure_only",
            source_session_ref,
            node.place_ref.clone(),
            node.envelope_ref
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            node.notes.clone(),
        ));
    }
    for route in &sections.route_trace {
        rows.push(telemetry(
            &route.route_id,
            "local_route_hop",
            "alpha09:local-route-hop".to_string(),
            route.authority.clone(),
            &route.redaction,
            source_session_ref,
            format!("{} -> {}", route.from_place, route.to_place),
            route.dispatch_outcome.clone(),
            route.event_refs.clone(),
        ));
    }
    for lifecycle in &sections.hotplug_lifecycle {
        rows.push(telemetry(
            &format!("hotplug_lifecycle#{}", lifecycle.sequence_id),
            "hotplug_lifecycle",
            "alpha09:hotplug-lifecycle".to_string(),
            format!("InspectHotPlugLifecycle({})", session.entry_place),
            "hotplug_boundary_summary",
            source_session_ref,
            lifecycle.package_id.clone(),
            lifecycle.terminal_outcome.clone(),
            lifecycle.notes.clone(),
        ));
    }
    for savepoint in &sections.save_load_timeline {
        rows.push(telemetry(
            &savepoint.entry_id,
            "save_load_timeline",
            "alpha09:save-load-timeline".to_string(),
            format!("InspectSaveLoadTimeline({})", session.entry_place),
            "local_frontier_summary",
            source_session_ref,
            savepoint.savepoint_id.clone(),
            format!("roundtrip_equal={}", savepoint.state_roundtrip_equal),
            savepoint.notes.clone(),
        ));
    }
    for query in &sections.retention_query_trace {
        rows.push(telemetry(
            &query.query_id,
            "retention_query",
            "alpha09:retention-query".to_string(),
            format!("QuerySessionTrace({})", session.entry_place),
            &query.redaction,
            source_session_ref,
            query.selector.clone(),
            query.query_outcome.clone(),
            query.notes.clone(),
        ));
    }
    rows
}

fn telemetry(
    telemetry_id: &str,
    telemetry_kind: &str,
    label: String,
    authority: String,
    redaction: &str,
    source_session_ref: &str,
    channel: String,
    value_summary: String,
    notes: Vec<String>,
) -> PracticalAlpha09TelemetryRow {
    PracticalAlpha09TelemetryRow {
        telemetry_id: telemetry_id.to_string(),
        telemetry_kind: telemetry_kind.to_string(),
        label,
        authority,
        redaction: redaction.to_string(),
        retention_scope: "session_local_ephemeral".to_string(),
        source_session_ref: source_session_ref.to_string(),
        channel,
        value_summary,
        notes,
    }
}

fn has_required_panels(panels: &[PracticalAlpha09Panel]) -> bool {
    let ids: BTreeSet<_> = panels.iter().map(|panel| panel.panel_id.as_str()).collect();
    [
        "event_dag_live_session",
        "local_route_trace",
        "membership_timeline",
        "witness_relation",
        "hotplug_lifecycle",
        "fallback_degradation",
        "save_load_timeline",
        "observer_safe_redacted_view",
        "retention_on_demand_trace",
    ]
    .into_iter()
    .all(|id| ids.contains(id))
}

fn sorted_unique(values: impl Iterator<Item = String>) -> Vec<String> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA09_DEVTOOLS_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA09_DEVTOOLS_SCOPE_KIND.to_string()
}

fn devtools_scope() -> String {
    PRACTICAL_ALPHA09_DEVTOOLS_SCOPE.to_string()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA09_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA09_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
