use std::{
    collections::VecDeque,
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::practical_alpha1::PracticalAlpha1LayerAttachment;
use mir_ast::practical_alpha1_runtime_plan::{
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE, PracticalAlpha1RuntimePlan,
    PracticalAlpha1RuntimePlanError, build_practical_alpha1_runtime_plan,
    build_practical_alpha1_runtime_plan_path,
};
use mirrorea_core::{
    AuthEvidence, LogicalPlaceRuntimeShell, LogicalPlaceRuntimeSnapshot, MessageEnvelope,
    MirroreaCoreError, PrincipalClaim, message_envelope_lanes,
};
use serde::{Deserialize, Serialize};

use crate::alpha_local_runtime::{
    EventDagEdge, EventDagExport, EventDagNode, LocalRuntimeDispatchRecord,
};

pub const PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE: &str = "practical-alpha1-local-runtime-floor";
pub const PRACTICAL_ALPHA1_LOCAL_RUNTIME_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_local_runtime_report";
pub const PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_LOCAL_RUNTIME_RETAINED_LATER_REFS: &[&str] = &[
    "docker_transport_execution",
    "package_hotplug_execution",
    "local_save_load_execution",
    "devtools_viewer_execution",
    "final_public_runtime_api",
];

const PRACTICAL_ALPHA1_LOCAL_RUNTIME_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 local runtime floor as Docker transport completion",
    "do not treat the practical alpha-1 local runtime floor as package/hot-plug completion",
    "do not treat the practical alpha-1 local runtime floor as local save/load completion",
    "do not treat the practical alpha-1 event DAG export hook as a final public devtools schema",
    "do not promote samples/practical-alpha1 to an active runnable root in the runtime package",
];

const PRACTICAL_ALPHA1_LOCAL_RUNTIME_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical local-runtime floor only",
    "limited RUN practical sample families only",
    "no Docker/local TCP transport, package hot-plug, local save/load command, or final public ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1LocalRuntimeErrorKind {
    RuntimePlan,
    Core,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1LocalRuntimeError {
    pub kind: PracticalAlpha1LocalRuntimeErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1LocalRuntimeError {
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

impl std::error::Error for PracticalAlpha1LocalRuntimeError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1LocalRuntimeReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "runtime_scope")]
    pub runtime_scope: String,
    #[serde(default = "runtime_plan_scope")]
    pub runtime_plan_scope: String,
    pub package_id: String,
    pub world_id: String,
    pub entry_place: String,
    pub queue_kind: String,
    #[serde(default)]
    pub requested_layers: Vec<PracticalAlpha1LayerAttachment>,
    pub message_envelope_lanes: Vec<String>,
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub message_envelopes: Vec<MessageEnvelope>,
    pub dispatch_records: Vec<LocalRuntimeDispatchRecord>,
    pub event_dag: EventDagExport,
    pub current_owner: String,
    pub visible_history: Vec<String>,
    pub terminal_outcome: String,
    #[serde(default)]
    pub reason_family: Option<String>,
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
    pub package_hotplug_claimed: bool,
    #[serde(default)]
    pub save_load_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DispatchEvaluation {
    dispatch_outcome: String,
    reason_refs: Vec<String>,
    checked_membership_epoch: u64,
    checked_member_incarnation: u64,
}

pub fn run_practical_alpha1_local_runtime_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1LocalRuntimeReport, PracticalAlpha1LocalRuntimeError> {
    let path = path.as_ref().to_path_buf();
    let plan = build_practical_alpha1_runtime_plan_path(&path)
        .map_err(|error| wrap_runtime_plan_error(&path, error))?;
    execute_practical_alpha1_runtime_plan_at_path(&plan, &path)
}

pub fn run_practical_alpha1_local_runtime(
    package: &mir_ast::practical_alpha1::PracticalAlpha1Package,
) -> Result<PracticalAlpha1LocalRuntimeReport, PracticalAlpha1LocalRuntimeError> {
    let plan = build_practical_alpha1_runtime_plan(package)
        .map_err(|error| wrap_runtime_plan_error(Path::new("<inline>"), error))?;
    execute_practical_alpha1_runtime_plan_at_path(&plan, Path::new("<inline>"))
}

pub fn execute_practical_alpha1_runtime_plan(
    plan: &PracticalAlpha1RuntimePlan,
) -> Result<PracticalAlpha1LocalRuntimeReport, MirroreaCoreError> {
    execute_practical_alpha1_runtime_plan_at_path(plan, Path::new("<inline>"))
        .map_err(|error| MirroreaCoreError::new(error.to_string()))
}

fn execute_practical_alpha1_runtime_plan_at_path(
    plan: &PracticalAlpha1RuntimePlan,
    path: &Path,
) -> Result<PracticalAlpha1LocalRuntimeReport, PracticalAlpha1LocalRuntimeError> {
    execute_plan(plan).map_err(|error| PracticalAlpha1LocalRuntimeError {
        kind: PracticalAlpha1LocalRuntimeErrorKind::Core,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })
}

fn execute_plan(
    plan: &PracticalAlpha1RuntimePlan,
) -> Result<PracticalAlpha1LocalRuntimeReport, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    for place in &plan.runtime_places {
        shell.register_place(&place.place_id, &place.kind)?;
    }
    for participant in &plan.bootstrap_participants {
        shell.add_initial_participant(&participant.principal)?;
    }
    for advance in &plan.pre_dispatch_membership_advances {
        match advance.kind {
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeMembershipAdvanceKind::AddParticipant => {
                shell.add_participant(&advance.principal)?;
            }
        }
    }

    let message_envelopes: Vec<MessageEnvelope> = plan
        .initial_envelopes
        .iter()
        .map(message_envelope_from_plan)
        .collect::<Result<_, _>>()?;
    let runtime_snapshot = shell.snapshot();
    let mut queue = VecDeque::from(message_envelopes.clone());
    let queued = queue
        .pop_front()
        .ok_or_else(|| MirroreaCoreError::new("runtime-plan floor requires one queued envelope"))?;
    let queue_depth_before = 1;
    let evaluation = evaluate_dispatch(&shell, &queued)?;
    let dispatch_record =
        build_dispatch_record(plan, &queued, &evaluation, queue_depth_before, queue.len());
    let (current_owner, visible_history, event_dag, reason_family) =
        build_runtime_observations(plan, &queued, &evaluation);
    let terminal_outcome = if evaluation.dispatch_outcome == "accepted" {
        "accepted".to_string()
    } else {
        "rejected".to_string()
    };

    Ok(PracticalAlpha1LocalRuntimeReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        runtime_scope: runtime_scope(),
        runtime_plan_scope: runtime_plan_scope(),
        package_id: plan.package_id.clone(),
        world_id: plan.world_id.clone(),
        entry_place: plan.entry_place.clone(),
        queue_kind: plan.queue_kind.clone(),
        requested_layers: plan.requested_layers.clone(),
        message_envelope_lanes: message_envelope_lanes(),
        runtime_snapshot,
        message_envelopes,
        dispatch_records: vec![dispatch_record],
        event_dag,
        current_owner,
        visible_history,
        terminal_outcome,
        reason_family,
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        public_cli_frozen: false,
        runtime_plan_emitted: false,
        run_local_claimed: true,
        run_docker_claimed: false,
        package_hotplug_claimed: false,
        save_load_claimed: false,
    })
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

fn build_dispatch_record(
    plan: &PracticalAlpha1RuntimePlan,
    envelope: &MessageEnvelope,
    evaluation: &DispatchEvaluation,
    queue_depth_before: usize,
    queue_depth_after: usize,
) -> LocalRuntimeDispatchRecord {
    let (generated_event_refs, notes) = match (&plan.dispatch_program, evaluation.dispatch_outcome.as_str()) {
        (
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeDispatchProgram::SugorokuRollHandoff { .. },
            "accepted",
        ) => (
            vec![
                "roll_commit#1".to_string(),
                "publish_roll#1".to_string(),
                "witness_draw_pub#1".to_string(),
                "handoff_turn#1".to_string(),
            ],
            vec![
                "practical checked package dispatches through the local queue/runtime-plan seam"
                    .to_string(),
                "publish/witness/handoff remain explicit runtime-side events".to_string(),
            ],
        ),
        _ => (
            vec!["reject_stale_membership#1".to_string()],
            vec![
                "dispatch is rejected before roll/publish/handoff because the membership frontier drifted"
                    .to_string(),
            ],
        ),
    };

    LocalRuntimeDispatchRecord {
        dispatch_order: 1,
        envelope_id: envelope.envelope_id.clone(),
        from_place: envelope.from_place.clone(),
        to_place: envelope.to_place.clone(),
        payload_kind: envelope.payload_kind.clone(),
        dispatch_outcome: evaluation.dispatch_outcome.clone(),
        checked_membership_epoch: evaluation.checked_membership_epoch,
        checked_member_incarnation: evaluation.checked_member_incarnation,
        queue_depth_before,
        queue_depth_after,
        reason_refs: evaluation.reason_refs.clone(),
        generated_event_refs,
        witness_refs: envelope.witness_refs.clone(),
        notes,
    }
}

fn build_runtime_observations(
    plan: &PracticalAlpha1RuntimePlan,
    envelope: &MessageEnvelope,
    evaluation: &DispatchEvaluation,
) -> (String, Vec<String>, EventDagExport, Option<String>) {
    match (&plan.dispatch_program, evaluation.dispatch_outcome.as_str()) {
        (
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeDispatchProgram::SugorokuRollHandoff {
                roll_value,
                handoff_target,
                publication_witness_ref,
                ..
            },
            "accepted",
        ) => {
            let visible_history = vec![
                format!(
                    "publish roll_result({}, {}) witness={}",
                    envelope.emitter_principal, roll_value, publication_witness_ref
                ),
                format!(
                    "handoff dice_owner {} -> {} using witness={}",
                    envelope.emitter_principal, handoff_target, publication_witness_ref
                ),
            ];
            let event_dag = EventDagExport {
                scope: "practical_local_event_dag_export_hook".to_string(),
                nodes: vec![
                    EventDagNode {
                        event_id: "roll_commit#1".to_string(),
                        event_kind: "roll_commit".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec![format!(
                            "practical local runtime accepts {}'s roll request",
                            envelope.emitter_principal
                        )],
                    },
                    EventDagNode {
                        event_id: "publish_roll#1".to_string(),
                        event_kind: "publish".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec!["roll result is published into visible history".to_string()],
                    },
                    EventDagNode {
                        event_id: "witness_draw_pub#1".to_string(),
                        event_kind: "witness".to_string(),
                        place_ref: plan.entry_place.clone(),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec![format!(
                            "published roll creates witness {}",
                            publication_witness_ref
                        )],
                    },
                    EventDagNode {
                        event_id: "handoff_turn#1".to_string(),
                        event_kind: "handoff".to_string(),
                        place_ref: participant_place_for(handoff_target),
                        envelope_ref: Some(envelope.envelope_id.clone()),
                        notes: vec![format!(
                            "turn ownership moves from {} to {}",
                            envelope.emitter_principal, handoff_target
                        )],
                    },
                ],
                edges: vec![
                    EventDagEdge {
                        from_event: "roll_commit#1".to_string(),
                        to_event: "publish_roll#1".to_string(),
                        relation: "publication_order".to_string(),
                    },
                    EventDagEdge {
                        from_event: "publish_roll#1".to_string(),
                        to_event: "witness_draw_pub#1".to_string(),
                        relation: "witness_order".to_string(),
                    },
                    EventDagEdge {
                        from_event: "witness_draw_pub#1".to_string(),
                        to_event: "handoff_turn#1".to_string(),
                        relation: "handoff_order".to_string(),
                    },
                ],
                notes: vec![
                    "event DAG export remains a non-public report hook, not a final public devtools schema"
                        .to_string(),
                ],
            };
            (handoff_target.clone(), visible_history, event_dag, None)
        }
        _ => {
            let event_dag = EventDagExport {
                scope: "practical_local_event_dag_export_hook".to_string(),
                nodes: vec![EventDagNode {
                    event_id: "reject_stale_membership#1".to_string(),
                    event_kind: "dispatch_reject".to_string(),
                    place_ref: plan.entry_place.clone(),
                    envelope_ref: Some(envelope.envelope_id.clone()),
                    notes: vec![
                        "membership frontier drift is rejected before roll/publish/handoff"
                            .to_string(),
                    ],
                }],
                edges: Vec::new(),
                notes: vec![
                    "negative practical runtime row records rejection without mutating local game state"
                        .to_string(),
                ],
            };
            (
                envelope.emitter_principal.clone(),
                Vec::new(),
                event_dag,
                Some("membership_freshness".to_string()),
            )
        }
    }
}

fn participant_place_for(principal: &str) -> String {
    format!("ParticipantPlace[{principal}]")
}

fn wrap_runtime_plan_error(
    path: &Path,
    error: PracticalAlpha1RuntimePlanError,
) -> PracticalAlpha1LocalRuntimeError {
    PracticalAlpha1LocalRuntimeError {
        kind: PracticalAlpha1LocalRuntimeErrorKind::RuntimePlan,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE_KIND.to_string()
}

fn runtime_scope() -> String {
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_SCOPE.to_string()
}

fn runtime_plan_scope() -> String {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_LOCAL_RUNTIME_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
