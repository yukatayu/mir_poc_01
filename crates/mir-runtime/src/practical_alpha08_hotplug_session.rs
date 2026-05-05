use std::{
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::practical_alpha1::{PracticalAlpha1Package, load_practical_alpha1_package_path};
use serde::Serialize;

use crate::{
    alpha_layer_insertion_runtime::LayerRuntimePreview,
    alpha_local_runtime::{EventDagEdge, EventDagNode},
    practical_alpha1_hotplug::{
        PracticalAlpha1HotPlugError, PracticalAlpha1ObjectAttachPreview,
        attach_practical_alpha1_package,
    },
    practical_alpha05_session::{
        PRACTICAL_ALPHA05_SESSION_SCOPE, PracticalAlpha05ObserverSafeExport,
        PracticalAlpha05SessionHotPlugLifecycleEntry, PracticalAlpha05SessionObjectPreviewState,
        PracticalAlpha05SessionReport, observe_practical_alpha05_session,
    },
};

pub const PRACTICAL_ALPHA08_HOTPLUG_SESSION_SCOPE: &str = "practical-alpha08-session-hotplug-floor";
pub const PRACTICAL_ALPHA08_HOTPLUG_SESSION_SURFACE_KIND: &str =
    "practical_alpha08_nonfinal_session_hotplug_report";
pub const PRACTICAL_ALPHA08_HOTPLUG_SESSION_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA08_HOTPLUG_SESSION_RETAINED_LATER_REFS: &[&str] = &[
    "alpha09_live_devtools_viewer",
    "distributed_durable_save_load",
    "final_public_hotplug_runtime_api",
];

const PRACTICAL_ALPHA08_HOTPLUG_SESSION_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-0.8 same-session hot-plug carrier as distributed durable save/load completion",
    "do not treat the practical alpha-0.8 same-session hot-plug carrier as alpha-0.9 live viewer or final public devtools ABI completion",
    "do not treat practical avatar/object preview visibility as native runtime execution or product runtime completion",
];

const PRACTICAL_ALPHA08_HOTPLUG_SESSION_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical alpha-0.8 same-session hot-plug floor only",
    "rejected attaches stay report-visible but do not mutate live session state",
    "no distributed durable save/load, alpha-0.9 live viewer, or final public ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha08SessionHotPlugErrorKind {
    FrontDoor,
    SessionScope,
    HotPlug,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha08SessionHotPlugError {
    pub kind: PracticalAlpha08SessionHotPlugErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha08SessionHotPlugError {
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

impl std::error::Error for PracticalAlpha08SessionHotPlugError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha08SessionHotPlugReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "hotplug_scope")]
    pub hotplug_scope: String,
    pub session_scope: String,
    pub session_id: String,
    pub source_hotplug_scope: String,
    pub source_hotplug_surface_kind: String,
    pub sample_id: String,
    pub package_id: String,
    pub package_kind: String,
    pub attach_profile: String,
    pub operation_kind: String,
    pub request_event_id: String,
    pub verdict_event_id: String,
    #[serde(default)]
    pub activation_cut_ref: Option<String>,
    #[serde(default)]
    pub detach_boundary_ref: Option<String>,
    pub terminal_outcome: String,
    #[serde(default)]
    pub reason_family: Option<String>,
    #[serde(default)]
    pub rejection_reason_refs: Vec<String>,
    pub session_mutated: bool,
    #[serde(default)]
    pub active_layers_before: Vec<String>,
    #[serde(default)]
    pub active_layers_after: Vec<String>,
    #[serde(default)]
    pub object_previews_after: Vec<String>,
    #[serde(default)]
    pub runtime_behavior_delta: Vec<String>,
    #[serde(default)]
    pub devtools_delta: Vec<String>,
    #[serde(default)]
    pub runtime_preview: Option<LayerRuntimePreview>,
    #[serde(default)]
    pub object_attach_preview: Option<PracticalAlpha1ObjectAttachPreview>,
    pub observer_safe_export_after: PracticalAlpha05ObserverSafeExport,
    pub hotplug_event_ids_after: Vec<String>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

pub fn run_practical_alpha08_hotplug_path(
    session: &PracticalAlpha05SessionReport,
    path: impl AsRef<Path>,
) -> Result<
    (
        PracticalAlpha05SessionReport,
        PracticalAlpha08SessionHotPlugReport,
    ),
    PracticalAlpha08SessionHotPlugError,
> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha08SessionHotPlugError {
            kind: PracticalAlpha08SessionHotPlugErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    run_practical_alpha08_hotplug(session, &package)
        .map_err(|error| PracticalAlpha08SessionHotPlugError { path, ..error })
}

pub fn run_practical_alpha08_hotplug(
    session: &PracticalAlpha05SessionReport,
    package: &PracticalAlpha1Package,
) -> Result<
    (
        PracticalAlpha05SessionReport,
        PracticalAlpha08SessionHotPlugReport,
    ),
    PracticalAlpha08SessionHotPlugError,
> {
    if session.session_scope != PRACTICAL_ALPHA05_SESSION_SCOPE {
        return Err(PracticalAlpha08SessionHotPlugError {
            kind: PracticalAlpha08SessionHotPlugErrorKind::SessionScope,
            path: PathBuf::from("<inline>"),
            detail: format!(
                "same-session hot-plug requires session_scope `{PRACTICAL_ALPHA05_SESSION_SCOPE}`, found `{}`",
                session.session_scope
            ),
        });
    }

    let source_report = attach_practical_alpha1_package(package)
        .map_err(|error| map_hotplug_error(error, Path::new("<inline>")))?;

    let sequence_id = session.hotplug_lifecycle.len() + 1;
    let request_event_id = format!("hotplug_request#{sequence_id}");
    let verdict_event_id = format!("hotplug_verdict#{sequence_id}");
    let operation_kind = source_report
        .hotplug_runtime_report
        .request
        .operation_kind
        .clone();
    let session_mutated = should_mutate_session(&source_report.terminal_outcome);
    let mut next = session.clone();
    let mut runtime_behavior_delta = runtime_behavior_delta(&source_report);
    let mut devtools_delta = devtools_delta(&source_report);

    if session_mutated {
        next.session_phase = "hotplug".to_string();
        next.same_session_hotplug_claimed = true;
        append_hotplug_events(
            &mut next,
            &request_event_id,
            &verdict_event_id,
            &source_report,
        );
        merge_runtime_behavior_markers(&mut next.runtime_behavior_markers, &runtime_behavior_delta);

        if !source_report.active_layers_after.is_empty() {
            merge_unique_strings(&mut next.active_layers, &source_report.active_layers_after);
        }
        if let Some(preview) = &source_report.object_attach_preview {
            next.object_preview_inventory
                .push(PracticalAlpha05SessionObjectPreviewState {
                    sample_id: source_report.sample_id.clone(),
                    package_id: source_report.package_id.clone(),
                    selected_representation: preview.selected_representation.clone(),
                    fallback_representation: preview.fallback_representation.clone(),
                    provided_roles: preview.provided_roles.clone(),
                    required_host_capabilities: preview.required_host_capabilities.clone(),
                    notes: vec![
                        "non-final object attach preview is visible in the live session carrier"
                            .to_string(),
                        "native execution remains later".to_string(),
                    ],
                });
        }

        next.hotplug_lifecycle
            .push(PracticalAlpha05SessionHotPlugLifecycleEntry {
                sequence_id,
                sample_id: source_report.sample_id.clone(),
                package_id: source_report.package_id.clone(),
                package_kind: source_report.package_kind.clone(),
                attach_profile: source_report.attach_profile.clone(),
                operation_kind: operation_kind.clone(),
                terminal_outcome: source_report.terminal_outcome.clone(),
                reason_family: source_report.reason_family.clone(),
                rejection_reason_refs: source_report.rejection_reason_refs.clone(),
                request_event_id: request_event_id.clone(),
                verdict_event_id: verdict_event_id.clone(),
                activation_cut_ref: source_report.activation_cut_ref.clone(),
                detach_boundary_ref: source_report.detach_boundary_ref.clone(),
                session_mutated: true,
                active_layers_after: next.active_layers.clone(),
                object_previews_after: object_preview_summaries(&next),
                runtime_behavior_delta: runtime_behavior_delta.clone(),
                devtools_delta: devtools_delta.clone(),
                notes: lifecycle_notes(&source_report),
            });
        next.observer_safe_export = observe_practical_alpha05_session(&next);
    }

    if !session_mutated {
        runtime_behavior_delta.clear();
        devtools_delta.clear();
    }

    let observer_safe_export_after = if session_mutated {
        next.observer_safe_export.clone()
    } else {
        observe_practical_alpha05_session(session)
    };
    let hotplug_event_ids_after = if session_mutated {
        next.event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect()
    } else {
        session
            .event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect()
    };
    let active_layers_after = if session_mutated {
        next.active_layers.clone()
    } else {
        session.active_layers.clone()
    };
    let object_previews_after = if session_mutated {
        object_preview_summaries(&next)
    } else {
        object_preview_summaries(session)
    };

    let report = PracticalAlpha08SessionHotPlugReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        hotplug_scope: hotplug_scope(),
        session_scope: session.session_scope.clone(),
        session_id: session.session_id.clone(),
        source_hotplug_scope: source_report.hotplug_scope.clone(),
        source_hotplug_surface_kind: source_report.surface_kind.clone(),
        sample_id: source_report.sample_id.clone(),
        package_id: source_report.package_id.clone(),
        package_kind: source_report.package_kind.clone(),
        attach_profile: source_report.attach_profile.clone(),
        operation_kind,
        request_event_id,
        verdict_event_id,
        activation_cut_ref: source_report.activation_cut_ref.clone(),
        detach_boundary_ref: source_report.detach_boundary_ref.clone(),
        terminal_outcome: source_report.terminal_outcome.clone(),
        reason_family: source_report.reason_family.clone(),
        rejection_reason_refs: source_report.rejection_reason_refs.clone(),
        session_mutated,
        active_layers_before: session.active_layers.clone(),
        active_layers_after,
        object_previews_after,
        runtime_behavior_delta,
        devtools_delta,
        runtime_preview: source_report.runtime_preview.clone(),
        object_attach_preview: source_report.object_attach_preview.clone(),
        observer_safe_export_after,
        hotplug_event_ids_after,
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    };

    Ok((next, report))
}

fn should_mutate_session(terminal_outcome: &str) -> bool {
    matches!(
        terminal_outcome,
        "accepted"
            | "accepted_contract_update"
            | "accepted_object_attach_preview"
            | "deferred_detach_minimal_contract"
    )
}

fn append_hotplug_events(
    session: &mut PracticalAlpha05SessionReport,
    request_event_id: &str,
    verdict_event_id: &str,
    source_report: &crate::practical_alpha1_hotplug::PracticalAlpha1HotPlugReport,
) {
    if let Some(previous_event_id) = session
        .event_dag
        .nodes
        .last()
        .map(|node| node.event_id.clone())
    {
        session.event_dag.edges.push(EventDagEdge {
            from_event: previous_event_id,
            to_event: request_event_id.to_string(),
            relation: "same_session_order".to_string(),
        });
    }
    session.event_dag.nodes.push(EventDagNode {
        event_id: request_event_id.to_string(),
        event_kind: "hotplug_request".to_string(),
        place_ref: session.entry_place.clone(),
        envelope_ref: Some(
            source_report
                .hotplug_runtime_report
                .request
                .message_envelope_ref
                .clone(),
        ),
        notes: vec![
            format!("sample_id={}", source_report.sample_id),
            format!("attach_profile={}", source_report.attach_profile),
        ],
    });
    session.event_dag.nodes.push(EventDagNode {
        event_id: verdict_event_id.to_string(),
        event_kind: "hotplug_verdict".to_string(),
        place_ref: session.entry_place.clone(),
        envelope_ref: Some(
            source_report
                .hotplug_runtime_report
                .request
                .request_id
                .clone(),
        ),
        notes: vec![
            format!("terminal_outcome={}", source_report.terminal_outcome),
            format!("package_id={}", source_report.package_id),
        ],
    });
    session.event_dag.edges.push(EventDagEdge {
        from_event: request_event_id.to_string(),
        to_event: verdict_event_id.to_string(),
        relation: "request_verdict_order".to_string(),
    });

    if let Some(activation_cut_ref) = &source_report.activation_cut_ref {
        session.event_dag.nodes.push(EventDagNode {
            event_id: activation_cut_ref.clone(),
            event_kind: "activation_cut".to_string(),
            place_ref: session.entry_place.clone(),
            envelope_ref: Some(
                source_report
                    .hotplug_runtime_report
                    .request
                    .request_id
                    .clone(),
            ),
            notes: vec![
                "accepted attach remains bounded to the same-session activation boundary"
                    .to_string(),
            ],
        });
        session.event_dag.edges.push(EventDagEdge {
            from_event: verdict_event_id.to_string(),
            to_event: activation_cut_ref.clone(),
            relation: "activation_cut_order".to_string(),
        });
    }

    if let Some(detach_boundary_ref) = &source_report.detach_boundary_ref {
        session.event_dag.nodes.push(EventDagNode {
            event_id: detach_boundary_ref.clone(),
            event_kind: "detach_boundary".to_string(),
            place_ref: session.entry_place.clone(),
            envelope_ref: Some(source_report.hotplug_runtime_report.request.request_id.clone()),
            notes: vec![
                "detach execution remains deferred and only the minimal contract boundary is visible"
                    .to_string(),
            ],
        });
        session.event_dag.edges.push(EventDagEdge {
            from_event: verdict_event_id.to_string(),
            to_event: detach_boundary_ref.clone(),
            relation: "detach_boundary_order".to_string(),
        });
    }

    if let Some(preview) = &source_report.object_attach_preview {
        let object_preview_event_id = format!(
            "object_attach_preview#{}",
            session.hotplug_lifecycle.len() + 1
        );
        session.event_dag.nodes.push(EventDagNode {
            event_id: object_preview_event_id.clone(),
            event_kind: "object_attach_preview".to_string(),
            place_ref: session.entry_place.clone(),
            envelope_ref: Some(
                source_report
                    .hotplug_runtime_report
                    .request
                    .request_id
                    .clone(),
            ),
            notes: vec![
                format!("representation={}", preview.selected_representation),
                "native execution remains later".to_string(),
            ],
        });
        session.event_dag.edges.push(EventDagEdge {
            from_event: verdict_event_id.to_string(),
            to_event: object_preview_event_id,
            relation: "object_preview_order".to_string(),
        });
    }
}

fn runtime_behavior_delta(
    source_report: &crate::practical_alpha1_hotplug::PracticalAlpha1HotPlugReport,
) -> Vec<String> {
    let mut values = Vec::new();
    match source_report.attach_profile.as_str() {
        "debug_trace_layer" if source_report.terminal_outcome == "accepted" => {
            values.push("debug_trace_layer_active".to_string());
        }
        "auth_gate_layer" if source_report.terminal_outcome == "accepted_contract_update" => {
            values.push("auth_contract_update_active".to_string());
        }
        "rate_limit_layer" if source_report.terminal_outcome == "accepted" => {
            values.push("declared_failure_preview:RateLimited".to_string());
        }
        _ => {}
    }
    if source_report.object_attach_preview.is_some() {
        values.push("object_preview_visible".to_string());
    }
    if source_report.terminal_outcome == "deferred_detach_minimal_contract" {
        values.push("detach_boundary_visible".to_string());
    }
    values
}

fn devtools_delta(
    source_report: &crate::practical_alpha1_hotplug::PracticalAlpha1HotPlugReport,
) -> Vec<String> {
    let mut values = vec!["hotplug_lifecycle_visible".to_string()];
    if source_report.activation_cut_ref.is_some() {
        values.push("activation_cut_visible".to_string());
    }
    if source_report.detach_boundary_ref.is_some() {
        values.push("detach_boundary_visible".to_string());
    }
    if source_report.object_attach_preview.is_some() {
        values.push("object_preview_inventory_visible".to_string());
    }
    values
}

fn lifecycle_notes(
    source_report: &crate::practical_alpha1_hotplug::PracticalAlpha1HotPlugReport,
) -> Vec<String> {
    let mut notes =
        vec!["same-session hot-plug lifecycle remains non-final and alpha-local".to_string()];
    if source_report.terminal_outcome == "deferred_detach_minimal_contract" {
        notes.push("detach execution is not performed at this floor".to_string());
    }
    if source_report.object_attach_preview.is_some() {
        notes.push(
            "object visibility is a preview only and does not imply native runtime execution"
                .to_string(),
        );
    }
    notes
}

fn object_preview_summaries(session: &PracticalAlpha05SessionReport) -> Vec<String> {
    session
        .object_preview_inventory
        .iter()
        .map(|entry| format!("{}:{}", entry.package_id, entry.selected_representation))
        .collect()
}

fn merge_unique_strings(target: &mut Vec<String>, values: &[String]) {
    for value in values {
        if !target.contains(value) {
            target.push(value.clone());
        }
    }
}

fn merge_runtime_behavior_markers(target: &mut Vec<String>, values: &[String]) {
    merge_unique_strings(target, values);
}

fn map_hotplug_error(
    error: PracticalAlpha1HotPlugError,
    path: &Path,
) -> PracticalAlpha08SessionHotPlugError {
    PracticalAlpha08SessionHotPlugError {
        kind: PracticalAlpha08SessionHotPlugErrorKind::HotPlug,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA08_HOTPLUG_SESSION_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA08_HOTPLUG_SESSION_SCOPE_KIND.to_string()
}

fn hotplug_scope() -> String {
    PRACTICAL_ALPHA08_HOTPLUG_SESSION_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA08_HOTPLUG_SESSION_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA08_HOTPLUG_SESSION_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA08_HOTPLUG_SESSION_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
