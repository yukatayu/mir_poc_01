use std::{
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::practical_alpha1::{PracticalAlpha1LayerAttachment, PracticalAlpha1Package};
use mir_ast::practical_alpha1_runtime_plan::PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE;
use mir_ast::practical_alpha1_save_load_plan::{
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SCOPE, PracticalAlpha1SaveLoadPlan,
    PracticalAlpha1SaveLoadPlanError, build_practical_alpha1_save_load_plan,
};
use mirrorea_core::{
    AuthEvidence, LogicalPlaceRuntimeShell, LogicalPlaceRuntimeSnapshot, MessageEnvelope,
    MirroreaCoreError, PrincipalClaim, message_envelope_lanes,
};
use serde::{Deserialize, Serialize};

use crate::alpha_local_runtime::{
    EventDagEdge, EventDagExport, EventDagNode, LocalRuntimeDispatchRecord,
};
use crate::practical_alpha1_local_runtime::{
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE, PracticalAlpha1LocalRuntimeError,
    PracticalAlpha1LocalRuntimeReport, run_practical_alpha1_local_runtime,
};

pub const PRACTICAL_ALPHA1_SAVE_LOAD_SCOPE: &str = "practical-alpha1-save-load-floor";
pub const PRACTICAL_ALPHA1_SAVE_LOAD_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_save_load_report";
pub const PRACTICAL_ALPHA1_SAVE_LOAD_SCOPE_KIND: &str = "alpha_local";
pub const PRACTICAL_ALPHA1_SAVE_LOAD_RETENTION_SCOPE: &str = "report_local_inventory";

const PRACTICAL_ALPHA1_SAVE_LOAD_RETAINED_LATER_REFS: &[&str] = &[
    "distributed_durable_save_load",
    "stale_witness_nonresurrection",
    "stale_lease_nonresurrection",
    "docker_transport_save_load",
    "hotplug_lifecycle_persistence",
    "final_public_save_load_api",
];

const PRACTICAL_ALPHA1_SAVE_LOAD_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 save-load report as distributed or durable save-load completion",
    "do not treat the practical alpha-1 save-load report as queue/channel/transport persistence completion",
    "do not treat CHK-CUT-01 guard reuse as full consistent-cut or Z-cycle completion",
    "do not treat the practical alpha-1 save-load report as a final public CLI/API freeze",
];

const PRACTICAL_ALPHA1_SAVE_LOAD_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical save-load floor only",
    "limited SL practical sample families only",
    "save/load depends on one exact practical local-runtime frontier before serialization",
    "no distributed durable save/load, stale witness/lease completion, or final public ABI",
];

const PRACTICAL_ALPHA1_SAVE_LOAD_CHECKER_GUARD_REFS: &[&str] = &["CHK-CUT-01"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1SaveLoadErrorKind {
    FrontDoor,
    SaveLoadPlan,
    LocalRuntime,
    Core,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1SaveLoadError {
    pub kind: PracticalAlpha1SaveLoadErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1SaveLoadError {
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

impl std::error::Error for PracticalAlpha1SaveLoadError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1SavedLocalFrontier {
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub current_owner: String,
    pub visible_history: Vec<String>,
    pub pending_envelopes: Vec<MessageEnvelope>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1SaveLoadReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "save_load_scope")]
    pub save_load_scope: String,
    #[serde(default = "save_load_plan_scope")]
    pub save_load_plan_scope: String,
    #[serde(default = "runtime_plan_scope")]
    pub runtime_plan_scope: String,
    pub package_id: String,
    pub world_id: String,
    pub entry_place: String,
    pub queue_kind: String,
    #[serde(default)]
    pub requested_layers: Vec<PracticalAlpha1LayerAttachment>,
    pub message_envelope_lanes: Vec<String>,
    pub saved_state_format: String,
    pub saved_runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub restored_runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub pending_envelopes_at_save: Vec<MessageEnvelope>,
    pub restored_visible_history: Vec<String>,
    pub visible_history_after_resume: Vec<String>,
    pub resumed_dispatch_records: Vec<LocalRuntimeDispatchRecord>,
    pub resumed_event_dag: EventDagExport,
    pub saved_owner: String,
    pub resumed_owner: String,
    pub terminal_outcome: String,
    #[serde(default)]
    pub reason_family: Option<String>,
    pub state_roundtrip_equal: bool,
    pub serialized_state_bytes: usize,
    #[serde(default = "retention_scope_default")]
    pub retention_scope: String,
    #[serde(default = "checker_guard_refs_default")]
    pub checker_guard_refs: Vec<String>,
    #[serde(default)]
    pub retained_artifacts: Vec<PracticalAlpha1RetainedArtifact>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
    #[serde(default)]
    pub public_cli_frozen: bool,
    #[serde(default)]
    pub runtime_plan_emitted: bool,
    #[serde(default)]
    pub run_local_claimed: bool,
    #[serde(default)]
    pub run_docker_claimed: bool,
    #[serde(default)]
    pub save_load_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RetainedArtifact {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub fetch_selector: String,
    pub retention_scope: String,
    pub redaction: String,
    pub source_section: String,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DispatchEvaluation {
    dispatch_outcome: String,
    reason_refs: Vec<String>,
    checked_membership_epoch: u64,
    checked_member_incarnation: u64,
}

pub fn run_practical_alpha1_save_load_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1SaveLoadReport, PracticalAlpha1SaveLoadError> {
    let path = path.as_ref().to_path_buf();
    let package =
        mir_ast::practical_alpha1::load_practical_alpha1_package_path(&path).map_err(|error| {
            PracticalAlpha1SaveLoadError {
                kind: PracticalAlpha1SaveLoadErrorKind::FrontDoor,
                path: error.path,
                detail: error.detail,
            }
        })?;
    let plan = build_practical_alpha1_save_load_plan(&package)
        .map_err(|error| wrap_save_load_plan_error(&path, error))?;
    let base_runtime = run_practical_alpha1_local_runtime(&package)
        .map_err(|error| wrap_local_runtime_error(&path, error))?;
    execute_practical_alpha1_save_load_at_path(&package, &plan, &base_runtime, &path)
}

pub fn run_practical_alpha1_save_load(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1SaveLoadReport, PracticalAlpha1SaveLoadError> {
    let plan = build_practical_alpha1_save_load_plan(package)
        .map_err(|error| wrap_save_load_plan_error(Path::new("<inline>"), error))?;
    let base_runtime = run_practical_alpha1_local_runtime(package)
        .map_err(|error| wrap_local_runtime_error(Path::new("<inline>"), error))?;
    execute_practical_alpha1_save_load_at_path(package, &plan, &base_runtime, Path::new("<inline>"))
}

fn execute_practical_alpha1_save_load_at_path(
    package: &PracticalAlpha1Package,
    plan: &PracticalAlpha1SaveLoadPlan,
    base_runtime: &PracticalAlpha1LocalRuntimeReport,
    path: &Path,
) -> Result<PracticalAlpha1SaveLoadReport, PracticalAlpha1SaveLoadError> {
    execute_practical_alpha1_save_load_plan(package, plan, base_runtime).map_err(|error| {
        PracticalAlpha1SaveLoadError {
            kind: PracticalAlpha1SaveLoadErrorKind::Core,
            path: path.to_path_buf(),
            detail: error.to_string(),
        }
    })
}

fn execute_practical_alpha1_save_load_plan(
    package: &PracticalAlpha1Package,
    plan: &PracticalAlpha1SaveLoadPlan,
    base_runtime: &PracticalAlpha1LocalRuntimeReport,
) -> Result<PracticalAlpha1SaveLoadReport, MirroreaCoreError> {
    validate_base_runtime_frontier(plan, base_runtime)?;
    let saved_frontier = PracticalAlpha1SavedLocalFrontier {
        runtime_snapshot: base_runtime.runtime_snapshot.clone(),
        current_owner: base_runtime.current_owner.clone(),
        visible_history: base_runtime.visible_history.clone(),
        pending_envelopes: Vec::new(),
        notes: vec![
            "saved local frontier is derived from one exact practical local-runtime execution"
                .to_string(),
            "pending queue persistence remains out of scope for the first practical save-load floor"
                .to_string(),
        ],
    };

    let serialized_state = serde_json::to_string_pretty(&saved_frontier).map_err(|error| {
        MirroreaCoreError::new(format!(
            "failed to serialize practical local save frontier: {error}"
        ))
    })?;
    let restored_frontier: PracticalAlpha1SavedLocalFrontier =
        serde_json::from_str(&serialized_state).map_err(|error| {
            MirroreaCoreError::new(format!(
                "failed to deserialize practical local save frontier: {error}"
            ))
        })?;
    validate_saved_frontier(plan, &restored_frontier)?;
    let state_roundtrip_equal = saved_frontier == restored_frontier;

    let mut restored_shell =
        LogicalPlaceRuntimeShell::restore(&restored_frontier.runtime_snapshot)?;
    for advance in &plan.post_restore_membership_advances {
        match advance.kind {
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeMembershipAdvanceKind::AddParticipant => {
                restored_shell.add_participant(&advance.principal)?;
            }
        }
    }
    let restored_runtime_snapshot = restored_shell.snapshot();
    let resumed_envelope = message_envelope_from_plan(&plan.resumed_envelope)?;
    let resumed_evaluation = evaluate_dispatch(&restored_shell, &resumed_envelope)?;
    let resumed_dispatch_record = build_resumed_dispatch_record(
        plan,
        &resumed_envelope,
        &resumed_evaluation,
        !plan.post_restore_membership_advances.is_empty(),
    );
    let (resumed_owner, visible_history_after_resume, resumed_event_dag, reason_family) =
        build_save_load_observations(
            plan,
            &restored_frontier,
            &resumed_envelope,
            &resumed_evaluation,
            !plan.post_restore_membership_advances.is_empty(),
        );
    let retained_artifacts = build_retained_artifacts(plan, &resumed_evaluation);

    Ok(PracticalAlpha1SaveLoadReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        save_load_scope: save_load_scope(),
        save_load_plan_scope: save_load_plan_scope(),
        runtime_plan_scope: runtime_plan_scope(),
        package_id: package.package_id.clone(),
        world_id: plan.world_id.clone(),
        entry_place: plan.entry_place.clone(),
        queue_kind: plan.queue_kind.clone(),
        requested_layers: package.layers.clone(),
        message_envelope_lanes: message_envelope_lanes(),
        saved_state_format: "practical_local_frontier_json".to_string(),
        saved_runtime_snapshot: saved_frontier.runtime_snapshot,
        restored_runtime_snapshot,
        pending_envelopes_at_save: restored_frontier.pending_envelopes,
        restored_visible_history: restored_frontier.visible_history.clone(),
        visible_history_after_resume,
        resumed_dispatch_records: vec![resumed_dispatch_record],
        resumed_event_dag,
        saved_owner: restored_frontier.current_owner.clone(),
        resumed_owner,
        terminal_outcome: if resumed_evaluation.dispatch_outcome == "accepted" {
            "accepted".to_string()
        } else {
            "rejected".to_string()
        },
        reason_family,
        state_roundtrip_equal,
        serialized_state_bytes: serialized_state.len(),
        retention_scope: retention_scope_default(),
        checker_guard_refs: checker_guard_refs_default(),
        retained_artifacts,
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        public_cli_frozen: false,
        runtime_plan_emitted: false,
        run_local_claimed: false,
        run_docker_claimed: false,
        save_load_claimed: true,
    })
}

fn build_retained_artifacts(
    plan: &PracticalAlpha1SaveLoadPlan,
    evaluation: &DispatchEvaluation,
) -> Vec<PracticalAlpha1RetainedArtifact> {
    let sample_id = &plan.sample_id;
    let dispatch_artifact_id = match evaluation.dispatch_outcome.as_str() {
        "accepted" => format!("resumed_dispatch_accept#{sample_id}"),
        "rejected_stale_membership" => format!("stale_membership_reject#{sample_id}"),
        _ => format!("resumed_dispatch_record#{sample_id}"),
    };

    vec![
        PracticalAlpha1RetainedArtifact {
            artifact_id: format!("saved_membership_frontier#{sample_id}"),
            artifact_kind: "membership_frontier".to_string(),
            fetch_selector: "saved_runtime_snapshot.membership".to_string(),
            retention_scope: PRACTICAL_ALPHA1_SAVE_LOAD_RETENTION_SCOPE.to_string(),
            redaction: "membership_frontier_summary".to_string(),
            source_section: "saved_runtime_snapshot.membership".to_string(),
            notes: vec![
                "exact report-local retained artifact for the saved membership frontier"
                    .to_string(),
            ],
        },
        PracticalAlpha1RetainedArtifact {
            artifact_id: format!("restored_membership_frontier#{sample_id}"),
            artifact_kind: "membership_frontier".to_string(),
            fetch_selector: "restored_runtime_snapshot.membership".to_string(),
            retention_scope: PRACTICAL_ALPHA1_SAVE_LOAD_RETENTION_SCOPE.to_string(),
            redaction: "membership_frontier_summary".to_string(),
            source_section: "restored_runtime_snapshot.membership".to_string(),
            notes: vec![
                "exact report-local retained artifact for the restored membership frontier"
                    .to_string(),
            ],
        },
        PracticalAlpha1RetainedArtifact {
            artifact_id: dispatch_artifact_id,
            artifact_kind: "dispatch_record".to_string(),
            fetch_selector: "resumed_dispatch_records[0]".to_string(),
            retention_scope: PRACTICAL_ALPHA1_SAVE_LOAD_RETENTION_SCOPE.to_string(),
            redaction: "dispatch_reason_summary".to_string(),
            source_section: "resumed_dispatch_records[0]".to_string(),
            notes: vec![
                "exact report-local retained artifact for the resumed dispatch verdict".to_string(),
            ],
        },
        PracticalAlpha1RetainedArtifact {
            artifact_id: format!("resumed_event_dag#{sample_id}"),
            artifact_kind: "event_dag".to_string(),
            fetch_selector: "resumed_event_dag".to_string(),
            retention_scope: PRACTICAL_ALPHA1_SAVE_LOAD_RETENTION_SCOPE.to_string(),
            redaction: "event_structure_only".to_string(),
            source_section: "resumed_event_dag".to_string(),
            notes: vec![
                "exact report-local retained artifact for the resumed event DAG".to_string(),
            ],
        },
    ]
}

fn validate_base_runtime_frontier(
    plan: &PracticalAlpha1SaveLoadPlan,
    base_runtime: &PracticalAlpha1LocalRuntimeReport,
) -> Result<(), MirroreaCoreError> {
    if base_runtime.runtime_scope != PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE {
        return Err(MirroreaCoreError::new(format!(
            "base practical local-runtime scope must be `{PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE}`, found `{}`",
            base_runtime.runtime_scope
        )));
    }
    if base_runtime.runtime_plan_scope != PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE {
        return Err(MirroreaCoreError::new(format!(
            "base practical runtime plan scope must be `{PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE}`, found `{}`",
            base_runtime.runtime_plan_scope
        )));
    }
    if base_runtime.terminal_outcome != plan.required_base_terminal_outcome {
        return Err(MirroreaCoreError::new(format!(
            "base practical local-runtime terminal_outcome must be `{}`, found `{}`",
            plan.required_base_terminal_outcome, base_runtime.terminal_outcome
        )));
    }
    if base_runtime.current_owner != plan.required_saved_owner {
        return Err(MirroreaCoreError::new(format!(
            "base practical local-runtime current_owner must be `{}`, found `{}`",
            plan.required_saved_owner, base_runtime.current_owner
        )));
    }
    if !base_runtime
        .visible_history
        .iter()
        .any(|entry| entry == &plan.required_saved_publish_history_entry)
    {
        return Err(MirroreaCoreError::new(
            "base practical local-runtime visible_history is missing the required published frontier"
                .to_string(),
        ));
    }
    if base_runtime.visible_history.last().map(String::as_str)
        != Some(plan.required_saved_history_tail.as_str())
    {
        return Err(MirroreaCoreError::new(
            "base practical local-runtime visible_history tail does not match the required saved handoff frontier"
                .to_string(),
        ));
    }
    Ok(())
}

fn validate_saved_frontier(
    plan: &PracticalAlpha1SaveLoadPlan,
    saved_frontier: &PracticalAlpha1SavedLocalFrontier,
) -> Result<(), MirroreaCoreError> {
    if saved_frontier.current_owner.is_empty() {
        return Err(MirroreaCoreError::new(
            "PracticalAlpha1SavedLocalFrontier current_owner must be non-empty",
        ));
    }
    if !saved_frontier.pending_envelopes.is_empty() {
        return Err(MirroreaCoreError::new(
            "PracticalAlpha1SavedLocalFrontier first floor expects no persisted pending envelopes at save time",
        ));
    }
    if !saved_frontier
        .visible_history
        .iter()
        .any(|entry| entry == &plan.required_saved_publish_history_entry)
    {
        return Err(MirroreaCoreError::new(
            "PracticalAlpha1SavedLocalFrontier is missing the required published frontier",
        ));
    }
    if saved_frontier.visible_history.last().map(String::as_str)
        != Some(plan.required_saved_history_tail.as_str())
    {
        return Err(MirroreaCoreError::new(
            "PracticalAlpha1SavedLocalFrontier tail must match the required saved handoff frontier",
        ));
    }
    Ok(())
}

fn message_envelope_from_plan(
    envelope: &mir_ast::practical_alpha1::PracticalAlpha1RuntimeEnvelope,
) -> Result<MessageEnvelope, MirroreaCoreError> {
    let message = MessageEnvelope {
        envelope_id: envelope.envelope_id.clone(),
        from_place: envelope.from_place.clone(),
        to_place: envelope.to_place.clone(),
        transport: envelope.transport.clone(),
        transport_medium: envelope.transport_medium.clone(),
        transport_seam: envelope.transport_seam.clone(),
        payload_kind: envelope.payload_kind.clone(),
        payload_ref: envelope.payload_ref.clone(),
        principal_claim: PrincipalClaim {
            principal: envelope.principal_claim.principal.clone(),
            participant_place: envelope.principal_claim.participant_place.clone(),
            claimed_authority: envelope.principal_claim.claimed_authority.clone(),
            claimed_capabilities: envelope.principal_claim.claimed_capabilities.clone(),
        },
        auth_evidence: envelope
            .auth_evidence
            .as_ref()
            .map(|evidence| AuthEvidence {
                kind: evidence.kind.clone(),
                subject: evidence.subject.clone(),
                issuer: evidence.issuer.clone(),
                bindings: evidence.bindings.clone(),
                notes: evidence.notes.clone(),
            }),
        emitter_principal: envelope.emitter_principal.clone(),
        membership_epoch: envelope.membership_epoch,
        member_incarnation: envelope.member_incarnation,
        freshness_checks: envelope.freshness_checks.clone(),
        capability_requirements: envelope.capability_requirements.clone(),
        authorization_checks: envelope.authorization_checks.clone(),
        witness_refs: envelope.witness_refs.clone(),
        dispatch_outcome: envelope.dispatch_outcome.clone(),
        notes: envelope.notes.clone(),
    };
    message.validate()?;
    Ok(message)
}

fn evaluate_dispatch(
    shell: &LogicalPlaceRuntimeShell,
    envelope: &MessageEnvelope,
) -> Result<DispatchEvaluation, MirroreaCoreError> {
    envelope.validate()?;
    let snapshot = shell.snapshot();
    let Some(member) = snapshot.membership.members.get(&envelope.emitter_principal) else {
        return Ok(rejected_evaluation(
            "rejected_missing_membership",
            vec!["missing_emitter_membership".to_string()],
            snapshot.membership.membership_epoch,
            0,
        ));
    };
    if !member.active {
        return Ok(rejected_evaluation(
            "rejected_inactive_membership",
            vec!["inactive_emitter_membership".to_string()],
            snapshot.membership.membership_epoch,
            member.incarnation,
        ));
    }
    if member.place != envelope.from_place
        || member.place != envelope.principal_claim.participant_place
    {
        return Ok(rejected_evaluation(
            "rejected_place_drift",
            vec!["emitter_place_drift".to_string()],
            snapshot.membership.membership_epoch,
            member.incarnation,
        ));
    }
    if snapshot.membership.membership_epoch != envelope.membership_epoch {
        return Ok(rejected_evaluation(
            "rejected_stale_membership",
            vec!["membership_epoch_drift".to_string()],
            snapshot.membership.membership_epoch,
            member.incarnation,
        ));
    }
    if member.incarnation != envelope.member_incarnation {
        return Ok(rejected_evaluation(
            "rejected_stale_membership",
            vec!["member_incarnation_drift".to_string()],
            snapshot.membership.membership_epoch,
            member.incarnation,
        ));
    }
    if !snapshot
        .place_catalog
        .places
        .contains_key(&envelope.to_place)
    {
        return Ok(rejected_evaluation(
            "rejected_unknown_destination",
            vec!["unknown_destination_place".to_string()],
            snapshot.membership.membership_epoch,
            member.incarnation,
        ));
    }

    Ok(DispatchEvaluation {
        dispatch_outcome: "accepted".to_string(),
        reason_refs: vec![
            "membership_frontier_current".to_string(),
            "destination_registered".to_string(),
            "local_queue_dispatch_ready".to_string(),
        ],
        checked_membership_epoch: snapshot.membership.membership_epoch,
        checked_member_incarnation: member.incarnation,
    })
}

fn rejected_evaluation(
    dispatch_outcome: &str,
    reason_refs: Vec<String>,
    checked_membership_epoch: u64,
    checked_member_incarnation: u64,
) -> DispatchEvaluation {
    DispatchEvaluation {
        dispatch_outcome: dispatch_outcome.to_string(),
        reason_refs,
        checked_membership_epoch,
        checked_member_incarnation,
    }
}

fn build_resumed_dispatch_record(
    plan: &PracticalAlpha1SaveLoadPlan,
    envelope: &MessageEnvelope,
    evaluation: &DispatchEvaluation,
    membership_advanced_after_restore: bool,
) -> LocalRuntimeDispatchRecord {
    let (generated_event_refs, notes) = match (
        &plan.resumed_dispatch_program,
        evaluation.dispatch_outcome.as_str(),
        membership_advanced_after_restore,
    ) {
        (
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeDispatchProgram::SugorokuRollHandoff { .. },
            "accepted",
            _,
        ) => (
            vec![
                "load_resume#1".to_string(),
                "roll_commit#2".to_string(),
                "publish_roll#2".to_string(),
                "witness_draw_pub#2".to_string(),
                "handoff_turn#2".to_string(),
            ],
            vec![
                "restored local frontier resumes through the same local queue/runtime seam"
                    .to_string(),
                "saved owner marker and visible-history frontier are re-checked before resumed dispatch"
                    .to_string(),
                "no distributed durable save-load or queue persistence claim is made in this row"
                    .to_string(),
            ],
        ),
        (_, _, true) => (
            vec![
                "load_resume#1".to_string(),
                "membership_advance#1".to_string(),
                "reject_stale_membership#2".to_string(),
            ],
            vec![
                "restored local frontier is re-evaluated only after the live membership frontier advances"
                    .to_string(),
                "saved membership frontier is not resurrected into accepted resumed dispatch"
                    .to_string(),
                "stale witness and lease non-resurrection remain later rows".to_string(),
            ],
        ),
        _ => (
            vec!["load_resume#1".to_string(), "reject_stale_membership#2".to_string()],
            vec![
                "restored local frontier still rejects resumed dispatch without mutating saved history"
                    .to_string(),
            ],
        ),
    };

    let mut reason_refs = evaluation.reason_refs.clone();
    reason_refs.push("saved_local_frontier_restored".to_string());
    reason_refs.push("saved_owner_history_match".to_string());
    reason_refs.push("pending_envelopes_empty_at_save".to_string());
    reason_refs.push("local_only_save_load".to_string());
    if membership_advanced_after_restore {
        reason_refs.push("membership_frontier_advanced_after_restore".to_string());
    }
    if evaluation.dispatch_outcome == "rejected_stale_membership" {
        reason_refs.push("stale_membership_not_resurrected".to_string());
    }

    LocalRuntimeDispatchRecord {
        dispatch_order: 2,
        envelope_id: envelope.envelope_id.clone(),
        from_place: envelope.from_place.clone(),
        to_place: envelope.to_place.clone(),
        payload_kind: envelope.payload_kind.clone(),
        dispatch_outcome: evaluation.dispatch_outcome.clone(),
        checked_membership_epoch: evaluation.checked_membership_epoch,
        checked_member_incarnation: evaluation.checked_member_incarnation,
        queue_depth_before: 1,
        queue_depth_after: 0,
        reason_refs,
        generated_event_refs,
        witness_refs: envelope.witness_refs.clone(),
        notes,
    }
}

fn build_save_load_observations(
    plan: &PracticalAlpha1SaveLoadPlan,
    restored_frontier: &PracticalAlpha1SavedLocalFrontier,
    envelope: &MessageEnvelope,
    evaluation: &DispatchEvaluation,
    membership_advanced_after_restore: bool,
) -> (String, Vec<String>, EventDagExport, Option<String>) {
    match (
        &plan.resumed_dispatch_program,
        evaluation.dispatch_outcome.as_str(),
    ) {
        (
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeDispatchProgram::SugorokuRollHandoff {
                roll_value,
                handoff_target,
                publication_witness_ref,
                ..
            },
            "accepted",
        ) => {
            let mut visible_history_after_resume = restored_frontier.visible_history.clone();
            visible_history_after_resume.push(format!(
                "publish roll_result({}, {}) witness={}",
                envelope.emitter_principal, roll_value, publication_witness_ref
            ));
            visible_history_after_resume.push(format!(
                "handoff dice_owner {} -> {} using witness={}",
                envelope.emitter_principal, handoff_target, publication_witness_ref
            ));
            let event_dag = EventDagExport {
                scope: "practical_save_load_event_dag_export_hook".to_string(),
                nodes: vec![
                    EventDagNode {
                        event_id: "save_local_frontier#1".to_string(),
                        event_kind: "save".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: None,
                        notes: vec![
                            "one exact practical local-runtime frontier is serialized after the first handoff"
                                .to_string(),
                        ],
                    },
                    EventDagNode {
                        event_id: "load_resume#1".to_string(),
                        event_kind: "load".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: None,
                        notes: vec![
                            "saved runtime snapshot, owner marker, and visible-history frontier are restored before resumed local dispatch"
                                .to_string(),
                        ],
                    },
                    EventDagNode {
                        event_id: "roll_commit#2".to_string(),
                        event_kind: "roll_commit".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec![format!(
                            "resumed local dispatch accepts {}'s roll request",
                            envelope.emitter_principal
                        )],
                    },
                    EventDagNode {
                        event_id: "publish_roll#2".to_string(),
                        event_kind: "publish".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec!["resumed roll result is published after local restore".to_string()],
                    },
                    EventDagNode {
                        event_id: "witness_draw_pub#2".to_string(),
                        event_kind: "witness".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec![format!(
                            "published resumed roll creates witness {}",
                            publication_witness_ref
                        )],
                    },
                    EventDagNode {
                        event_id: "handoff_turn#2".to_string(),
                        event_kind: "handoff".to_string(),
                        place_ref: participant_place_for(handoff_target),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec![format!(
                            "turn ownership moves from {} to {} after resumed dispatch",
                            envelope.emitter_principal, handoff_target
                        )],
                    },
                ],
                edges: vec![
                    EventDagEdge {
                        from_event: "save_local_frontier#1".to_string(),
                        to_event: "load_resume#1".to_string(),
                        relation: "save_load_roundtrip".to_string(),
                    },
                    EventDagEdge {
                        from_event: "load_resume#1".to_string(),
                        to_event: "roll_commit#2".to_string(),
                        relation: "load_resume_order".to_string(),
                    },
                    EventDagEdge {
                        from_event: "roll_commit#2".to_string(),
                        to_event: "publish_roll#2".to_string(),
                        relation: "publication_order".to_string(),
                    },
                    EventDagEdge {
                        from_event: "publish_roll#2".to_string(),
                        to_event: "witness_draw_pub#2".to_string(),
                        relation: "witness_order".to_string(),
                    },
                    EventDagEdge {
                        from_event: "witness_draw_pub#2".to_string(),
                        to_event: "handoff_turn#2".to_string(),
                        relation: "handoff_order".to_string(),
                    },
                ],
                notes: vec![
                    "event DAG export remains a non-public report hook".to_string(),
                    "save/load edge is local-only and does not imply distributed checkpoint closure"
                        .to_string(),
                ],
            };
            (
                handoff_target.clone(),
                visible_history_after_resume,
                event_dag,
                None,
            )
        }
        _ => {
            let membership_node = if membership_advanced_after_restore {
                vec![EventDagNode {
                    event_id: "membership_advance#1".to_string(),
                    event_kind: "membership_update".to_string(),
                    place_ref: "WorldPlace[AlphaRoom#1]".to_string(),
                    envelope_ref: None,
                    notes: vec![
                        "a later join advances the live membership frontier after restore and before resumed dispatch"
                            .to_string(),
                    ],
                }]
            } else {
                Vec::new()
            };
            let mut nodes = vec![
                EventDagNode {
                    event_id: "save_local_frontier#1".to_string(),
                    event_kind: "save".to_string(),
                    place_ref: plan.entry_place.clone(),
                    envelope_ref: None,
                    notes: vec![
                        "one exact practical local-runtime frontier is serialized after the first handoff"
                            .to_string(),
                    ],
                },
                EventDagNode {
                    event_id: "load_resume#1".to_string(),
                    event_kind: "load".to_string(),
                    place_ref: plan.entry_place.clone(),
                    envelope_ref: None,
                    notes: vec![
                        "saved runtime snapshot, owner marker, and visible-history frontier are restored before resumed dispatch"
                            .to_string(),
                    ],
                },
            ];
            nodes.extend(membership_node);
            nodes.push(EventDagNode {
                event_id: "reject_stale_membership#2".to_string(),
                event_kind: "dispatch_reject".to_string(),
                place_ref: plan.entry_place.clone(),
                envelope_ref: Some(envelope.envelope_id.clone()),
                notes: vec![
                    "saved membership frontier is rejected instead of being treated as current"
                        .to_string(),
                ],
            });
            let mut edges = vec![EventDagEdge {
                from_event: "save_local_frontier#1".to_string(),
                to_event: "load_resume#1".to_string(),
                relation: "save_load_roundtrip".to_string(),
            }];
            if membership_advanced_after_restore {
                edges.push(EventDagEdge {
                    from_event: "load_resume#1".to_string(),
                    to_event: "membership_advance#1".to_string(),
                    relation: "load_restore_order".to_string(),
                });
                edges.push(EventDagEdge {
                    from_event: "membership_advance#1".to_string(),
                    to_event: "reject_stale_membership#2".to_string(),
                    relation: "membership_dependency_order".to_string(),
                });
            } else {
                edges.push(EventDagEdge {
                    from_event: "load_resume#1".to_string(),
                    to_event: "reject_stale_membership#2".to_string(),
                    relation: "load_resume_order".to_string(),
                });
            }
            let event_dag = EventDagExport {
                scope: "practical_save_load_event_dag_export_hook".to_string(),
                nodes,
                edges,
                notes: vec![
                    "event DAG export remains a non-public report hook".to_string(),
                    "stale-membership rejection does not imply distributed checkpoint repair or stale witness completion"
                        .to_string(),
                ],
            };
            (
                restored_frontier.current_owner.clone(),
                restored_frontier.visible_history.clone(),
                event_dag,
                Some("membership_freshness".to_string()),
            )
        }
    }
}

fn participant_place_for(principal: &str) -> String {
    format!("ParticipantPlace[{principal}]")
}

fn wrap_save_load_plan_error(
    path: &Path,
    error: PracticalAlpha1SaveLoadPlanError,
) -> PracticalAlpha1SaveLoadError {
    PracticalAlpha1SaveLoadError {
        kind: PracticalAlpha1SaveLoadErrorKind::SaveLoadPlan,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn wrap_local_runtime_error(
    path: &Path,
    error: PracticalAlpha1LocalRuntimeError,
) -> PracticalAlpha1SaveLoadError {
    PracticalAlpha1SaveLoadError {
        kind: PracticalAlpha1SaveLoadErrorKind::LocalRuntime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_SCOPE_KIND.to_string()
}

fn save_load_scope() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_SCOPE.to_string()
}

fn save_load_plan_scope() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SCOPE.to_string()
}

fn runtime_plan_scope() -> String {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE.to_string()
}

fn checker_guard_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_CHECKER_GUARD_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn retention_scope_default() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_RETENTION_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
