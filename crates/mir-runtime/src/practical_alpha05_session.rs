use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::practical_alpha1::{PracticalAlpha1Package, load_practical_alpha1_package_path};
use mir_ast::practical_alpha1_checker::{
    PRACTICAL_ALPHA1_CHECKER_SCOPE, PRACTICAL_ALPHA1_CHECKER_SURFACE_KIND,
    PracticalAlpha1CheckError, check_practical_alpha1_package,
};
use mir_ast::practical_alpha1_runtime_plan::PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE;
use mirrorea_core::LogicalPlaceRuntimeSnapshot;
use serde::{Deserialize, Serialize};

use crate::alpha_local_runtime::{EventDagExport, LocalRuntimeDispatchRecord};
use crate::practical_alpha1_local_runtime::{
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE, PRACTICAL_ALPHA1_LOCAL_RUNTIME_SURFACE_KIND,
    PracticalAlpha1LocalRuntimeError, PracticalAlpha1LocalRuntimeReport,
    run_practical_alpha1_local_runtime,
};

pub const PRACTICAL_ALPHA05_SESSION_SCOPE: &str = "practical-alpha05-session-floor";
pub const PRACTICAL_ALPHA05_SESSION_SURFACE_KIND: &str =
    "practical_alpha05_nonfinal_session_report";
pub const PRACTICAL_ALPHA05_SESSION_SCOPE_KIND: &str = "alpha_local";
pub const PRACTICAL_ALPHA05_OBSERVER_VIEW_KIND: &str =
    "practical_alpha05_observer_safe_session_view";
pub const PRACTICAL_ALPHA05_SAVEPOINT_FORMAT: &str = "practical_alpha05_local_frontier_json";

const PRACTICAL_ALPHA05_SESSION_RETAINED_LATER_REFS: &[&str] = &[
    "typed_external_host_io_direct_execution",
    "same_session_hotplug_execution",
    "session_bound_route_trace",
    "distributed_durable_save_load",
    "final_public_runtime_api",
];

const PRACTICAL_ALPHA05_SESSION_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-0.5 session carrier as same-session hot-plug completion",
    "do not treat the practical alpha-0.5 session carrier as typed external host-I/O completion before that lane exists",
    "do not treat the practical alpha-0.5 session carrier as distributed durable save/load completion",
    "do not treat the practical alpha-0.5 session carrier as a final public runtime or devtools ABI",
];

const PRACTICAL_ALPHA05_SESSION_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical alpha-0.5 session floor only",
    "session carrier is bounded to the current local runtime / savepoint / observer-safe export seam",
    "no same-session hot-plug, typed external host-I/O lane, distributed durable save/load, or final public ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha05SessionErrorKind {
    FrontDoor,
    Checker,
    Runtime,
    MissingSavepoint,
    Serialize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha05SessionError {
    pub kind: PracticalAlpha05SessionErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha05SessionError {
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

impl std::error::Error for PracticalAlpha05SessionError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha05ObserverSafeExport {
    #[serde(default = "observer_view_kind")]
    pub view_kind: String,
    pub authority: String,
    pub redaction: String,
    pub retention_scope: String,
    pub membership_epoch: u64,
    pub active_participants: Vec<String>,
    pub current_owner: String,
    pub event_ids: Vec<String>,
    pub visible_history: Vec<String>,
    pub witness_inventory: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha05SessionSavepoint {
    pub savepoint_id: String,
    #[serde(default = "savepoint_format")]
    pub saved_state_format: String,
    pub saved_runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub saved_current_owner: String,
    pub saved_visible_history: Vec<String>,
    pub saved_event_dag: EventDagExport,
    pub saved_capability_inventory: Vec<String>,
    pub saved_witness_inventory: Vec<String>,
    pub state_roundtrip_equal: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha05SessionReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "session_scope")]
    pub session_scope: String,
    pub session_id: String,
    pub session_phase: String,
    pub package_id: String,
    pub world_id: String,
    pub entry_place: String,
    pub queue_kind: String,
    pub checker_scope: String,
    pub checker_surface_kind: String,
    pub checker_sample_id: String,
    pub checker_verdict: String,
    pub runtime_plan_scope: String,
    pub runtime_scope: String,
    pub runtime_surface_kind: String,
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub current_owner: String,
    pub visible_history: Vec<String>,
    pub dispatch_records: Vec<LocalRuntimeDispatchRecord>,
    pub event_dag: EventDagExport,
    pub capability_inventory: Vec<String>,
    pub witness_inventory: Vec<String>,
    pub observer_safe_export: PracticalAlpha05ObserverSafeExport,
    #[serde(default)]
    pub savepoints: Vec<PracticalAlpha05SessionSavepoint>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
    #[serde(default)]
    pub session_bound_observation_claimed: bool,
    #[serde(default)]
    pub same_session_hotplug_claimed: bool,
    #[serde(default)]
    pub typed_host_io_claimed: bool,
    #[serde(default)]
    pub distributed_save_load_claimed: bool,
    #[serde(default)]
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PracticalAlpha05SavedLocalFrontier {
    runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    current_owner: String,
    visible_history: Vec<String>,
    event_dag: EventDagExport,
    capability_inventory: Vec<String>,
    witness_inventory: Vec<String>,
    notes: Vec<String>,
}

pub fn start_practical_alpha05_session_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha05SessionReport, PracticalAlpha05SessionError> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha05SessionError {
            kind: PracticalAlpha05SessionErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    start_practical_alpha05_session_at_path(&package, &path)
}

pub fn start_practical_alpha05_session(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha05SessionReport, PracticalAlpha05SessionError> {
    start_practical_alpha05_session_at_path(package, Path::new("<inline>"))
}

pub fn save_practical_alpha05_session(
    session: &PracticalAlpha05SessionReport,
    savepoint_id: &str,
) -> Result<PracticalAlpha05SessionReport, PracticalAlpha05SessionError> {
    let frontier = PracticalAlpha05SavedLocalFrontier {
        runtime_snapshot: session.runtime_snapshot.clone(),
        current_owner: session.current_owner.clone(),
        visible_history: session.visible_history.clone(),
        event_dag: session.event_dag.clone(),
        capability_inventory: session.capability_inventory.clone(),
        witness_inventory: session.witness_inventory.clone(),
        notes: vec![
            "saved alpha-0.5 local frontier remains local-only and non-final".to_string(),
            "same-session hot-plug and typed external host-I/O remain later".to_string(),
        ],
    };
    let serialized =
        serde_json::to_string_pretty(&frontier).map_err(|error| PracticalAlpha05SessionError {
            kind: PracticalAlpha05SessionErrorKind::Serialize,
            path: PathBuf::from("<inline>"),
            detail: format!("failed to serialize session savepoint frontier: {error}"),
        })?;
    let restored: PracticalAlpha05SavedLocalFrontier =
        serde_json::from_str(&serialized).map_err(|error| PracticalAlpha05SessionError {
            kind: PracticalAlpha05SessionErrorKind::Serialize,
            path: PathBuf::from("<inline>"),
            detail: format!("failed to deserialize session savepoint frontier: {error}"),
        })?;
    let state_roundtrip_equal = frontier == restored;

    let mut next = session.clone();
    next.session_phase = "saved".to_string();
    next.savepoints.push(PracticalAlpha05SessionSavepoint {
        savepoint_id: savepoint_id.to_string(),
        saved_state_format: savepoint_format(),
        saved_runtime_snapshot: restored.runtime_snapshot,
        saved_current_owner: restored.current_owner,
        saved_visible_history: restored.visible_history,
        saved_event_dag: restored.event_dag,
        saved_capability_inventory: restored.capability_inventory,
        saved_witness_inventory: restored.witness_inventory,
        state_roundtrip_equal,
        notes: vec![
            "savepoint is a local frontier only".to_string(),
            "no distributed durable checkpoint semantics are implied".to_string(),
        ],
    });
    next.observer_safe_export = build_observer_safe_export(&next);
    Ok(next)
}

pub fn load_practical_alpha05_session(
    session: &PracticalAlpha05SessionReport,
    savepoint_id: &str,
) -> Result<PracticalAlpha05SessionReport, PracticalAlpha05SessionError> {
    let savepoint = session
        .savepoints
        .iter()
        .find(|candidate| candidate.savepoint_id == savepoint_id)
        .ok_or_else(|| PracticalAlpha05SessionError {
            kind: PracticalAlpha05SessionErrorKind::MissingSavepoint,
            path: PathBuf::from("<inline>"),
            detail: format!("session savepoint `{savepoint_id}` does not exist"),
        })?;

    let mut next = session.clone();
    next.session_phase = "loaded".to_string();
    next.runtime_snapshot = savepoint.saved_runtime_snapshot.clone();
    next.current_owner = savepoint.saved_current_owner.clone();
    next.visible_history = savepoint.saved_visible_history.clone();
    next.event_dag = savepoint.saved_event_dag.clone();
    next.capability_inventory = savepoint.saved_capability_inventory.clone();
    next.witness_inventory = savepoint.saved_witness_inventory.clone();
    next.observer_safe_export = build_observer_safe_export(&next);
    Ok(next)
}

pub fn observe_practical_alpha05_session(
    session: &PracticalAlpha05SessionReport,
) -> PracticalAlpha05ObserverSafeExport {
    build_observer_safe_export(session)
}

fn start_practical_alpha05_session_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha05SessionReport, PracticalAlpha05SessionError> {
    let checker =
        check_practical_alpha1_package(package).map_err(|error| wrap_checker_error(path, error))?;
    let runtime = run_practical_alpha1_local_runtime(package)
        .map_err(|error| wrap_runtime_error(path, error))?;

    let capability_inventory = derive_capability_inventory(&runtime);
    let witness_inventory = derive_witness_inventory(&runtime);
    let mut session = PracticalAlpha05SessionReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        session_scope: session_scope(),
        session_id: format!("session#{}", runtime.package_id),
        session_phase: "started".to_string(),
        package_id: runtime.package_id.clone(),
        world_id: runtime.world_id.clone(),
        entry_place: runtime.entry_place.clone(),
        queue_kind: runtime.queue_kind.clone(),
        checker_scope: PRACTICAL_ALPHA1_CHECKER_SCOPE.to_string(),
        checker_surface_kind: PRACTICAL_ALPHA1_CHECKER_SURFACE_KIND.to_string(),
        checker_sample_id: checker.sample_id.clone(),
        checker_verdict: checker.verdict.clone(),
        runtime_plan_scope: PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE.to_string(),
        runtime_scope: PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE.to_string(),
        runtime_surface_kind: PRACTICAL_ALPHA1_LOCAL_RUNTIME_SURFACE_KIND.to_string(),
        runtime_snapshot: runtime.runtime_snapshot.clone(),
        current_owner: runtime.current_owner.clone(),
        visible_history: runtime.visible_history.clone(),
        dispatch_records: runtime.dispatch_records.clone(),
        event_dag: runtime.event_dag.clone(),
        capability_inventory,
        witness_inventory,
        observer_safe_export: PracticalAlpha05ObserverSafeExport {
            view_kind: observer_view_kind(),
            authority: String::new(),
            redaction: String::new(),
            retention_scope: String::new(),
            membership_epoch: 0,
            active_participants: Vec::new(),
            current_owner: String::new(),
            event_ids: Vec::new(),
            visible_history: Vec::new(),
            witness_inventory: Vec::new(),
            notes: Vec::new(),
        },
        savepoints: Vec::new(),
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        session_bound_observation_claimed: true,
        same_session_hotplug_claimed: false,
        typed_host_io_claimed: false,
        distributed_save_load_claimed: false,
        final_public_api_frozen: false,
    };
    session.observer_safe_export = build_observer_safe_export(&session);
    Ok(session)
}

fn build_observer_safe_export(
    session: &PracticalAlpha05SessionReport,
) -> PracticalAlpha05ObserverSafeExport {
    PracticalAlpha05ObserverSafeExport {
        view_kind: observer_view_kind(),
        authority: format!("ObserveSession({})", session.entry_place),
        redaction: "observer_safe_session_summary".to_string(),
        retention_scope: "session_local_ephemeral".to_string(),
        membership_epoch: session.runtime_snapshot.membership.membership_epoch,
        active_participants: session
            .runtime_snapshot
            .membership
            .members
            .iter()
            .filter_map(|(principal, member)| member.active.then_some(principal.clone()))
            .collect(),
        current_owner: session.current_owner.clone(),
        event_ids: session
            .event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect(),
        visible_history: session.visible_history.clone(),
        witness_inventory: session.witness_inventory.clone(),
        notes: vec![
            "session-bound observer-safe export reuses live session state rather than exact report recomposition"
                .to_string(),
            "auth/raw transport lanes remain out of scope for this alpha-0.5 local session floor"
                .to_string(),
        ],
    }
}

fn derive_capability_inventory(runtime: &PracticalAlpha1LocalRuntimeReport) -> Vec<String> {
    let mut values = BTreeSet::new();
    for envelope in &runtime.message_envelopes {
        for capability in &envelope.principal_claim.claimed_capabilities {
            values.insert(capability.clone());
        }
    }
    values.into_iter().collect()
}

fn derive_witness_inventory(runtime: &PracticalAlpha1LocalRuntimeReport) -> Vec<String> {
    let mut values = BTreeSet::new();
    for envelope in &runtime.message_envelopes {
        for witness in &envelope.witness_refs {
            values.insert(witness.clone());
        }
    }
    for entry in &runtime.visible_history {
        if let Some((_, witness)) = entry.split_once("witness=") {
            values.insert(witness.trim().to_string());
        }
    }
    values.into_iter().collect()
}

fn wrap_checker_error(
    path: &Path,
    error: PracticalAlpha1CheckError,
) -> PracticalAlpha05SessionError {
    PracticalAlpha05SessionError {
        kind: PracticalAlpha05SessionErrorKind::Checker,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn wrap_runtime_error(
    path: &Path,
    error: PracticalAlpha1LocalRuntimeError,
) -> PracticalAlpha05SessionError {
    PracticalAlpha05SessionError {
        kind: PracticalAlpha05SessionErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA05_SESSION_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA05_SESSION_SCOPE_KIND.to_string()
}

fn session_scope() -> String {
    PRACTICAL_ALPHA05_SESSION_SCOPE.to_string()
}

fn observer_view_kind() -> String {
    PRACTICAL_ALPHA05_OBSERVER_VIEW_KIND.to_string()
}

fn savepoint_format() -> String {
    PRACTICAL_ALPHA05_SAVEPOINT_FORMAT.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA05_SESSION_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA05_SESSION_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA05_SESSION_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
