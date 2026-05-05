use std::{
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::practical_alpha1::{
    PracticalAlpha1HostAdapterKind, PracticalAlpha1HostIoPayload, PracticalAlpha1Package,
    load_practical_alpha1_package_path,
};
use serde::{Deserialize, Serialize};

use crate::alpha_local_runtime::{EventDagEdge, EventDagNode};
use crate::practical_alpha05_session::{
    PRACTICAL_ALPHA05_SESSION_SCOPE, PracticalAlpha05HostIoHistoryEntry,
    PracticalAlpha05ObserverSafeExport, PracticalAlpha05SessionReport,
    observe_practical_alpha05_session,
};

pub const PRACTICAL_ALPHA05_HOST_IO_SCOPE: &str = "practical-alpha05-host-io-floor";
pub const PRACTICAL_ALPHA05_HOST_IO_SURFACE_KIND: &str =
    "practical_alpha05_nonfinal_host_io_report";
pub const PRACTICAL_ALPHA05_HOST_IO_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA05_HOST_IO_RETAINED_LATER_REFS: &[&str] = &[
    "same_session_hotplug_execution",
    "distributed_durable_save_load",
    "final_public_host_boundary_abi",
    "browser_network_vr_host_family_split",
    "final_public_runtime_api",
];

const PRACTICAL_ALPHA05_HOST_IO_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-0.5 host-I/O lane as Mir core stdio builtin completion",
    "do not treat the practical alpha-0.5 host-I/O lane as same-session hot-plug completion",
    "do not treat the practical alpha-0.5 host-I/O lane as a final public host/runtime/devtools ABI",
];

const PRACTICAL_ALPHA05_HOST_IO_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical alpha-0.5 typed host-I/O floor only",
    "one minimal typed adapter family only",
    "no same-session hot-plug, distributed durable save/load, or final public ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha05HostIoErrorKind {
    FrontDoor,
    MissingHostIoSection,
    SessionScope,
    ResponseMismatch,
    UnsupportedPayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha05HostIoError {
    pub kind: PracticalAlpha05HostIoErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha05HostIoError {
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

impl std::error::Error for PracticalAlpha05HostIoError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha05HostIoReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "host_io_scope")]
    pub host_io_scope: String,
    pub session_scope: String,
    pub session_id: String,
    pub sample_id: String,
    pub package_id: String,
    pub adapter_kind: String,
    pub adapter_entry: String,
    pub request_schema: String,
    pub response_schema: String,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
    pub authority: String,
    pub observation_policy: String,
    pub request_payload: PracticalAlpha1HostIoPayload,
    pub response_payload: PracticalAlpha1HostIoPayload,
    pub session_event_ids_after: Vec<String>,
    pub observer_safe_export_after: PracticalAlpha05ObserverSafeExport,
    pub terminal_outcome: String,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

pub fn run_practical_alpha05_host_io_path(
    session: &PracticalAlpha05SessionReport,
    path: impl AsRef<Path>,
) -> Result<
    (PracticalAlpha05SessionReport, PracticalAlpha05HostIoReport),
    PracticalAlpha05HostIoError,
> {
    let path = path.as_ref().to_path_buf();
    let package =
        load_practical_alpha1_package_path(&path).map_err(|error| PracticalAlpha05HostIoError {
            kind: PracticalAlpha05HostIoErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        })?;
    run_practical_alpha05_host_io(session, &package)
        .map_err(|error| PracticalAlpha05HostIoError { path, ..error })
}

pub fn run_practical_alpha05_host_io(
    session: &PracticalAlpha05SessionReport,
    package: &PracticalAlpha1Package,
) -> Result<
    (PracticalAlpha05SessionReport, PracticalAlpha05HostIoReport),
    PracticalAlpha05HostIoError,
> {
    if session.session_scope != PRACTICAL_ALPHA05_SESSION_SCOPE {
        return Err(PracticalAlpha05HostIoError {
            kind: PracticalAlpha05HostIoErrorKind::SessionScope,
            path: PathBuf::from("<inline>"),
            detail: format!(
                "host-I/O lane requires session_scope `{PRACTICAL_ALPHA05_SESSION_SCOPE}`, found `{}`",
                session.session_scope
            ),
        });
    }
    let host_io =
        package
            .alpha_local_host_io_input
            .as_ref()
            .ok_or_else(|| PracticalAlpha05HostIoError {
                kind: PracticalAlpha05HostIoErrorKind::MissingHostIoSection,
                path: PathBuf::from("<inline>"),
                detail: "package does not declare `alpha_local_host_io_input`".to_string(),
            })?;

    let response_payload = execute_adapter(&host_io.adapter_kind, &host_io.request_payload)?;
    if response_payload != host_io.expected_response {
        return Err(PracticalAlpha05HostIoError {
            kind: PracticalAlpha05HostIoErrorKind::ResponseMismatch,
            path: PathBuf::from("<inline>"),
            detail: format!(
                "host adapter response drifted: expected {:?}, observed {:?}",
                host_io.expected_response, response_payload
            ),
        });
    }

    let event_index = session.host_io_history.len() + 1;
    let request_event_id = format!("host_request#{event_index}");
    let response_event_id = format!("host_response#{event_index}");
    let mut next = session.clone();
    next.session_phase = "host_io".to_string();
    next.typed_host_io_claimed = true;
    next.event_dag.nodes.push(EventDagNode {
        event_id: request_event_id.clone(),
        event_kind: "host_request".to_string(),
        place_ref: session.entry_place.clone(),
        envelope_ref: None,
        notes: vec![format!(
            "typed host adapter {} receives one request",
            host_io.adapter_entry
        )],
    });
    next.event_dag.nodes.push(EventDagNode {
        event_id: response_event_id.clone(),
        event_kind: "host_response".to_string(),
        place_ref: session.entry_place.clone(),
        envelope_ref: None,
        notes: vec![format!(
            "typed host adapter {} emits one receipt",
            host_io.adapter_entry
        )],
    });
    next.event_dag.edges.push(EventDagEdge {
        from_event: request_event_id.clone(),
        to_event: response_event_id.clone(),
        relation: "request_response_order".to_string(),
    });
    next.host_io_history
        .push(PracticalAlpha05HostIoHistoryEntry {
            adapter_kind: adapter_kind_name(host_io.adapter_kind).to_string(),
            request_summary: payload_summary(&host_io.request_payload),
            response_summary: payload_summary(&response_payload),
            event_request_id: request_event_id,
            event_response_id: response_event_id,
        });
    next.observer_safe_export = observe_practical_alpha05_session(&next);

    let report = PracticalAlpha05HostIoReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        host_io_scope: host_io_scope(),
        session_scope: session.session_scope.clone(),
        session_id: session.session_id.clone(),
        sample_id: host_io.sample_id.clone(),
        package_id: package.package_id.clone(),
        adapter_kind: adapter_kind_ref(host_io.adapter_kind).to_string(),
        adapter_entry: host_io.adapter_entry.clone(),
        request_schema: host_io.request_schema.clone(),
        response_schema: host_io.response_schema.clone(),
        effect_row: host_io.effect_row.clone(),
        failure_row: host_io.failure_row.clone(),
        authority: host_io.authority.clone(),
        observation_policy: host_io.observation_policy.clone(),
        request_payload: host_io.request_payload.clone(),
        response_payload,
        session_event_ids_after: next
            .event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect(),
        observer_safe_export_after: next.observer_safe_export.clone(),
        terminal_outcome: "accepted".to_string(),
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    };

    Ok((next, report))
}

fn execute_adapter(
    adapter_kind: &PracticalAlpha1HostAdapterKind,
    request_payload: &PracticalAlpha1HostIoPayload,
) -> Result<PracticalAlpha1HostIoPayload, PracticalAlpha05HostIoError> {
    match (adapter_kind, request_payload) {
        (PracticalAlpha1HostAdapterKind::AddOne, PracticalAlpha1HostIoPayload::Int { value }) => {
            Ok(PracticalAlpha1HostIoPayload::Int { value: value + 1 })
        }
        (
            PracticalAlpha1HostAdapterKind::EchoText,
            PracticalAlpha1HostIoPayload::Text { value },
        ) => Ok(PracticalAlpha1HostIoPayload::Text {
            value: value.clone(),
        }),
        _ => Err(PracticalAlpha05HostIoError {
            kind: PracticalAlpha05HostIoErrorKind::UnsupportedPayload,
            path: PathBuf::from("<inline>"),
            detail: "adapter_kind and payload kind do not match".to_string(),
        }),
    }
}

fn payload_summary(payload: &PracticalAlpha1HostIoPayload) -> String {
    match payload {
        PracticalAlpha1HostIoPayload::Int { value } => value.to_string(),
        PracticalAlpha1HostIoPayload::Text { value } => value.clone(),
    }
}

fn adapter_kind_name(kind: PracticalAlpha1HostAdapterKind) -> &'static str {
    match kind {
        PracticalAlpha1HostAdapterKind::AddOne => "AddOne",
        PracticalAlpha1HostAdapterKind::EchoText => "EchoText",
    }
}

fn adapter_kind_ref(kind: PracticalAlpha1HostAdapterKind) -> &'static str {
    match kind {
        PracticalAlpha1HostAdapterKind::AddOne => "add_one",
        PracticalAlpha1HostAdapterKind::EchoText => "echo_text",
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA05_HOST_IO_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA05_HOST_IO_SCOPE_KIND.to_string()
}

fn host_io_scope() -> String {
    PRACTICAL_ALPHA05_HOST_IO_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA05_HOST_IO_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA05_HOST_IO_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA05_HOST_IO_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
