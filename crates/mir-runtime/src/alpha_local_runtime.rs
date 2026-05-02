use std::collections::VecDeque;

use mirrorea_core::{
    LogicalPlaceRuntimeShell, LogicalPlaceRuntimeSnapshot, MessageEnvelope, MirroreaCoreError,
    PrincipalClaim, message_envelope_lanes,
};
use serde::{Deserialize, Serialize};

const WORLD_PLACE: &str = "WorldPlace[AlphaRoom#1]";
const WORLD_KIND: &str = "WorldPlace";
const GAME_PLACE: &str = "GamePlace[SugorokuGame#1]";
const GAME_KIND: &str = "SugorokuGamePlace";
const ALICE_PLACE: &str = "ParticipantPlace[Alice]";
const BOB_PLACE: &str = "ParticipantPlace[Bob]";
const QUEUE_KIND: &str = "in_process_fifo";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LocalRuntimeDispatchRecord {
    pub dispatch_order: usize,
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub payload_kind: String,
    pub dispatch_outcome: String,
    pub checked_membership_epoch: u64,
    pub checked_member_incarnation: u64,
    pub queue_depth_before: usize,
    pub queue_depth_after: usize,
    pub reason_refs: Vec<String>,
    pub generated_event_refs: Vec<String>,
    pub witness_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventDagNode {
    pub event_id: String,
    pub event_kind: String,
    pub place_ref: String,
    pub envelope_ref: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventDagEdge {
    pub from_event: String,
    pub to_event: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EventDagExport {
    pub scope: String,
    pub nodes: Vec<EventDagNode>,
    pub edges: Vec<EventDagEdge>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LocalRuntimeReport {
    pub runtime_scope: String,
    pub sample_id: String,
    pub queue_kind: String,
    pub message_envelope_lanes: Vec<String>,
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub message_envelopes: Vec<MessageEnvelope>,
    pub dispatch_records: Vec<LocalRuntimeDispatchRecord>,
    pub event_dag: EventDagExport,
    pub current_owner: String,
    pub visible_history: Vec<String>,
    pub terminal_outcome: String,
    pub reason_family: Option<String>,
    pub retained_later_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalRuntimeSavePoint {
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub current_owner: String,
    pub visible_history: Vec<String>,
    pub pending_envelopes: Vec<MessageEnvelope>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LocalSaveLoadReport {
    pub runtime_scope: String,
    pub sample_id: String,
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
    pub state_roundtrip_equal: bool,
    pub serialized_state_bytes: usize,
    pub retained_later_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DispatchEvaluation {
    dispatch_outcome: String,
    reason_refs: Vec<String>,
    checked_membership_epoch: u64,
    checked_member_incarnation: u64,
}

pub fn build_local_sugoroku_runtime_report() -> Result<LocalRuntimeReport, MirroreaCoreError> {
    let shell = bootstrap_local_runtime_shell()?;
    let envelope = roll_request_envelope(0, 0, "accepted");
    let message_envelopes = vec![envelope.clone()];
    let mut queue = VecDeque::from([envelope.clone()]);
    let queue_depth_before = queue.len();
    let queued = queue
        .pop_front()
        .expect("queue contains exactly one local runtime envelope");
    let evaluation = evaluate_dispatch(&shell, &queued)?;
    let dispatch_record = LocalRuntimeDispatchRecord {
        dispatch_order: 1,
        envelope_id: queued.envelope_id.clone(),
        from_place: queued.from_place.clone(),
        to_place: queued.to_place.clone(),
        payload_kind: queued.payload_kind.clone(),
        dispatch_outcome: evaluation.dispatch_outcome.clone(),
        checked_membership_epoch: evaluation.checked_membership_epoch,
        checked_member_incarnation: evaluation.checked_member_incarnation,
        queue_depth_before,
        queue_depth_after: queue.len(),
        reason_refs: evaluation.reason_refs.clone(),
        generated_event_refs: vec![
            "roll_commit#1".to_string(),
            "publish_roll#1".to_string(),
            "witness_draw_pub#1".to_string(),
            "handoff_turn#1".to_string(),
        ],
        witness_refs: queued.witness_refs.clone(),
        notes: vec![
            "single-envelope local queue dispatch over mirrorea-core substrate".to_string(),
            "publish/witness/handoff remain explicit runtime-side events".to_string(),
        ],
    };

    Ok(LocalRuntimeReport {
        runtime_scope: "alpha_local_runtime_stage_b_narrow".to_string(),
        sample_id: "LR-01".to_string(),
        queue_kind: QUEUE_KIND.to_string(),
        message_envelope_lanes: message_envelope_lanes(),
        runtime_snapshot: shell.snapshot(),
        message_envelopes,
        dispatch_records: vec![dispatch_record],
        event_dag: positive_event_dag(),
        current_owner: "Bob".to_string(),
        visible_history: vec![
            "publish roll_result(Alice, 4) witness=draw_pub#1".to_string(),
            "handoff dice_owner Alice -> Bob using witness=draw_pub#1".to_string(),
        ],
        terminal_outcome: "accepted".to_string(),
        reason_family: None,
        retained_later_refs: retained_later_refs(),
        notes: vec![
            "non-public in-memory local runtime floor over mirrorea-core substrate".to_string(),
            "moves one narrow local queue/message/membership/event path out of Python-only helpers"
                .to_string(),
            "does not claim hot-plug lifecycle, runtime package admission, avatar runtime, network, or parser integration"
                .to_string(),
        ],
    })
}

pub fn build_stale_membership_rejection_report() -> Result<LocalRuntimeReport, MirroreaCoreError> {
    let mut shell = bootstrap_local_runtime_shell()?;
    shell.add_participant("Carol")?;

    let envelope = roll_request_envelope(0, 0, "rejected_stale_membership");
    let message_envelopes = vec![envelope.clone()];
    let mut queue = VecDeque::from([envelope.clone()]);
    let queue_depth_before = queue.len();
    let queued = queue
        .pop_front()
        .expect("queue contains exactly one stale-membership envelope");
    let evaluation = evaluate_dispatch(&shell, &queued)?;
    let dispatch_record = LocalRuntimeDispatchRecord {
        dispatch_order: 1,
        envelope_id: queued.envelope_id.clone(),
        from_place: queued.from_place.clone(),
        to_place: queued.to_place.clone(),
        payload_kind: queued.payload_kind.clone(),
        dispatch_outcome: evaluation.dispatch_outcome.clone(),
        checked_membership_epoch: evaluation.checked_membership_epoch,
        checked_member_incarnation: evaluation.checked_member_incarnation,
        queue_depth_before,
        queue_depth_after: queue.len(),
        reason_refs: evaluation.reason_refs.clone(),
        generated_event_refs: vec!["reject_stale_membership#1".to_string()],
        witness_refs: queued.witness_refs.clone(),
        notes: vec![
            "dispatch is rejected before roll/publish/handoff because the membership frontier drifted"
                .to_string(),
        ],
    };

    Ok(LocalRuntimeReport {
        runtime_scope: "alpha_local_runtime_stage_b_narrow".to_string(),
        sample_id: "LR-02".to_string(),
        queue_kind: QUEUE_KIND.to_string(),
        message_envelope_lanes: message_envelope_lanes(),
        runtime_snapshot: shell.snapshot(),
        message_envelopes,
        dispatch_records: vec![dispatch_record],
        event_dag: stale_membership_event_dag(),
        current_owner: "Alice".to_string(),
        visible_history: Vec::new(),
        terminal_outcome: "rejected".to_string(),
        reason_family: Some("membership_freshness".to_string()),
        retained_later_refs: retained_later_refs(),
        notes: vec![
            "non-public in-memory negative sample for membership freshness only".to_string(),
            "no state change is applied after stale frontier rejection".to_string(),
        ],
    })
}

pub fn build_local_save_load_resume_report() -> Result<LocalSaveLoadReport, MirroreaCoreError> {
    let base = build_local_sugoroku_runtime_report()?;
    let save_point = LocalRuntimeSavePoint {
        runtime_snapshot: base.runtime_snapshot.clone(),
        current_owner: base.current_owner.clone(),
        visible_history: base.visible_history.clone(),
        pending_envelopes: Vec::new(),
        notes: vec![
            "single-runtime local room savepoint only".to_string(),
            "savepoint is captured after the first handoff and before the resumed dispatch"
                .to_string(),
        ],
    };

    let serialized_state = serde_json::to_string_pretty(&save_point).map_err(|error| {
        MirroreaCoreError::new(format!("failed to serialize local savepoint: {error}"))
    })?;
    let restored_save_point: LocalRuntimeSavePoint = serde_json::from_str(&serialized_state)
        .map_err(|error| {
            MirroreaCoreError::new(format!("failed to deserialize local savepoint: {error}"))
        })?;
    validate_saved_turn_frontier(&restored_save_point)?;
    let restored_shell = LogicalPlaceRuntimeShell::restore(&restored_save_point.runtime_snapshot)?;
    let restored_member = restored_save_point
        .runtime_snapshot
        .membership
        .members
        .get(&restored_save_point.current_owner)
        .ok_or_else(|| {
            MirroreaCoreError::new(format!(
                "restored savepoint missing owner membership for `{}`",
                restored_save_point.current_owner
            ))
        })?;
    let resumed_envelope = roll_request_envelope_for(
        "roll_request#2",
        &restored_save_point.current_owner,
        "Alice",
        "roll(Bob,2)",
        restored_save_point
            .runtime_snapshot
            .membership
            .membership_epoch,
        restored_member.incarnation,
        "accepted",
    );
    let resumed_evaluation = evaluate_dispatch(&restored_shell, &resumed_envelope)?;
    let resumed_dispatch_record = LocalRuntimeDispatchRecord {
        dispatch_order: 2,
        envelope_id: resumed_envelope.envelope_id.clone(),
        from_place: resumed_envelope.from_place.clone(),
        to_place: resumed_envelope.to_place.clone(),
        payload_kind: resumed_envelope.payload_kind.clone(),
        dispatch_outcome: resumed_evaluation.dispatch_outcome.clone(),
        checked_membership_epoch: resumed_evaluation.checked_membership_epoch,
        checked_member_incarnation: resumed_evaluation.checked_member_incarnation,
        queue_depth_before: 1,
        queue_depth_after: 0,
        reason_refs: {
            let mut refs = resumed_evaluation.reason_refs.clone();
            refs.push("saved_local_state_restored".to_string());
            refs.push("saved_owner_history_match".to_string());
            refs.push("local_save_only".to_string());
            refs.push("distributed_save_load_deferred".to_string());
            refs
        },
        generated_event_refs: vec![
            "load_resume#1".to_string(),
            "roll_commit#2".to_string(),
            "publish_roll#2".to_string(),
            "witness_draw_pub#2".to_string(),
            "handoff_turn#2".to_string(),
        ],
        witness_refs: resumed_envelope.witness_refs.clone(),
        notes: vec![
            "restored local runtime savepoint resumes through the same local queue floor"
                .to_string(),
            "saved owner marker and visible-history handoff frontier are re-checked before resumed dispatch"
                .to_string(),
            "no multi-place consistent-cut or durable replay claim is made in this row"
                .to_string(),
        ],
    };

    let mut visible_history_after_resume = restored_save_point.visible_history.clone();
    visible_history_after_resume.push("publish roll_result(Bob, 2) witness=draw_pub#2".to_string());
    visible_history_after_resume
        .push("handoff dice_owner Bob -> Alice using witness=draw_pub#2".to_string());
    let state_roundtrip_equal = save_point == restored_save_point;

    Ok(LocalSaveLoadReport {
        runtime_scope: "alpha_local_save_load_stage_f_bridge".to_string(),
        sample_id: "CUT-04".to_string(),
        saved_state_format: "local_runtime_state_json".to_string(),
        saved_runtime_snapshot: save_point.runtime_snapshot,
        restored_runtime_snapshot: restored_save_point.runtime_snapshot,
        pending_envelopes_at_save: restored_save_point.pending_envelopes,
        restored_visible_history: restored_save_point.visible_history,
        visible_history_after_resume,
        resumed_dispatch_records: vec![resumed_dispatch_record],
        resumed_event_dag: save_load_event_dag(),
        saved_owner: save_point.current_owner,
        resumed_owner: "Alice".to_string(),
        terminal_outcome: "accepted".to_string(),
        state_roundtrip_equal,
        serialized_state_bytes: serialized_state.len(),
        retained_later_refs: retained_later_refs(),
        notes: vec![
            "non-public local save/load bridge over the Stage-B runtime floor".to_string(),
            "save/load is local-only and does not widen to distributed durable persistence"
                .to_string(),
            "serialized runtime snapshot, saved owner marker, and visible-history frontier are restored before resumed dispatch"
                .to_string(),
            "restore validates room-local membership/place coherence only; stale witness/membership non-resurrection split remains later"
                .to_string(),
        ],
    })
}

fn validate_saved_turn_frontier(
    save_point: &LocalRuntimeSavePoint,
) -> Result<(), MirroreaCoreError> {
    if save_point.current_owner.is_empty() {
        return Err(MirroreaCoreError::new(
            "LocalRuntimeSavePoint current_owner must be non-empty",
        ));
    }
    if !save_point.pending_envelopes.is_empty() {
        return Err(MirroreaCoreError::new(
            "LocalRuntimeSavePoint local-only bridge expects no persisted pending envelopes at save time",
        ));
    }

    let expected_publish = "publish roll_result(Alice, 4) witness=draw_pub#1";
    if !save_point
        .visible_history
        .iter()
        .any(|entry| entry == expected_publish)
    {
        return Err(MirroreaCoreError::new(
            "LocalRuntimeSavePoint missing the published roll frontier needed for resumed local dispatch",
        ));
    }

    let expected_handoff = format!(
        "handoff dice_owner Alice -> {} using witness=draw_pub#1",
        save_point.current_owner
    );
    if save_point.visible_history.last().map(String::as_str) != Some(expected_handoff.as_str()) {
        return Err(MirroreaCoreError::new(format!(
            "LocalRuntimeSavePoint last visible history entry must record handoff frontier `{expected_handoff}` before resumed local dispatch"
        )));
    }

    Ok(())
}

fn bootstrap_local_runtime_shell() -> Result<LogicalPlaceRuntimeShell, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place(WORLD_PLACE, WORLD_KIND)?;
    shell.register_place(GAME_PLACE, GAME_KIND)?;
    shell.add_initial_participant("Alice")?;
    shell.add_initial_participant("Bob")?;
    Ok(shell)
}

fn roll_request_envelope(
    membership_epoch: u64,
    member_incarnation: u64,
    dispatch_outcome: &str,
) -> MessageEnvelope {
    roll_request_envelope_for(
        "roll_request#1",
        "Alice",
        "Bob",
        "roll(Alice,4)",
        membership_epoch,
        member_incarnation,
        dispatch_outcome,
    )
}

fn roll_request_envelope_for(
    envelope_id: &str,
    emitter_principal: &str,
    handoff_target: &str,
    payload_ref: &str,
    membership_epoch: u64,
    member_incarnation: u64,
    dispatch_outcome: &str,
) -> MessageEnvelope {
    let participant_place = format!("ParticipantPlace[{emitter_principal}]");
    MessageEnvelope {
        envelope_id: envelope_id.to_string(),
        from_place: participant_place.clone(),
        to_place: GAME_PLACE.to_string(),
        transport: "local_queue".to_string(),
        transport_medium: Some("same_process_fifo".to_string()),
        transport_seam: "local_queue".to_string(),
        payload_kind: "sugoroku_roll_request".to_string(),
        payload_ref: payload_ref.to_string(),
        principal_claim: PrincipalClaim {
            principal: emitter_principal.to_string(),
            participant_place,
            claimed_authority: "SugorokuDiceOwner".to_string(),
            claimed_capabilities: vec![
                "RollDice".to_string(),
                "PublishRoll".to_string(),
                "HandoffTurn".to_string(),
            ],
        },
        auth_evidence: None,
        emitter_principal: emitter_principal.to_string(),
        membership_epoch,
        member_incarnation,
        freshness_checks: vec![
            "membership_epoch matches local runtime frontier".to_string(),
            "member_incarnation matches active participant".to_string(),
        ],
        capability_requirements: vec![
            "RollDice".to_string(),
            "PublishRoll".to_string(),
            "HandoffTurn".to_string(),
        ],
        authorization_checks: vec![
            format!("current dice owner is {emitter_principal}"),
            format!("handoff target {handoff_target} is active"),
        ],
        witness_refs: vec!["game_started_witness".to_string()],
        dispatch_outcome: dispatch_outcome.to_string(),
        notes: vec![
            "single-process local queue preview only".to_string(),
            "not a final public runtime API".to_string(),
        ],
    }
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

fn positive_event_dag() -> EventDagExport {
    EventDagExport {
        scope: "report_local_event_dag_export_hook".to_string(),
        nodes: vec![
            EventDagNode {
                event_id: "roll_commit#1".to_string(),
                event_kind: "roll_commit".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: Some("roll_request#1".to_string()),
                notes: vec!["local runtime accepts Alice's roll request".to_string()],
            },
            EventDagNode {
                event_id: "publish_roll#1".to_string(),
                event_kind: "publish".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: Some("roll_request#1".to_string()),
                notes: vec!["roll result is published into visible history".to_string()],
            },
            EventDagNode {
                event_id: "witness_draw_pub#1".to_string(),
                event_kind: "witness".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: Some("roll_request#1".to_string()),
                notes: vec!["published roll creates witness draw_pub#1".to_string()],
            },
            EventDagNode {
                event_id: "handoff_turn#1".to_string(),
                event_kind: "handoff".to_string(),
                place_ref: BOB_PLACE.to_string(),
                envelope_ref: Some("roll_request#1".to_string()),
                notes: vec!["turn ownership moves from Alice to Bob".to_string()],
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
            "event DAG export is a non-public report hook, not final devtools schema".to_string(),
        ],
    }
}

fn stale_membership_event_dag() -> EventDagExport {
    EventDagExport {
        scope: "report_local_event_dag_export_hook".to_string(),
        nodes: vec![EventDagNode {
            event_id: "reject_stale_membership#1".to_string(),
            event_kind: "dispatch_reject".to_string(),
            place_ref: GAME_PLACE.to_string(),
            envelope_ref: Some("roll_request#1".to_string()),
            notes: vec![
                "membership frontier drift is rejected before roll/publish/handoff".to_string(),
            ],
        }],
        edges: Vec::new(),
        notes: vec![
            "negative runtime row records rejection without mutating local game state".to_string(),
        ],
    }
}

fn save_load_event_dag() -> EventDagExport {
    EventDagExport {
        scope: "report_local_save_load_event_dag_export_hook".to_string(),
        nodes: vec![
            EventDagNode {
                event_id: "save_local_state#1".to_string(),
                event_kind: "save".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: None,
                notes: vec![
                    "single-runtime local room state is serialized after Alice -> Bob handoff"
                        .to_string(),
                ],
            },
            EventDagNode {
                event_id: "load_resume#1".to_string(),
                event_kind: "load".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: None,
                notes: vec![
                    "saved runtime snapshot, owner marker, and visible-history frontier are restored before resumed queue dispatch"
                        .to_string(),
                ],
            },
            EventDagNode {
                event_id: "roll_commit#2".to_string(),
                event_kind: "roll_commit".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: Some("roll_request#2".to_string()),
                notes: vec!["resumed local dispatch accepts Bob's roll request".to_string()],
            },
            EventDagNode {
                event_id: "publish_roll#2".to_string(),
                event_kind: "publish".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: Some("roll_request#2".to_string()),
                notes: vec!["resumed roll result is published after local load".to_string()],
            },
            EventDagNode {
                event_id: "witness_draw_pub#2".to_string(),
                event_kind: "witness".to_string(),
                place_ref: GAME_PLACE.to_string(),
                envelope_ref: Some("roll_request#2".to_string()),
                notes: vec!["published resumed roll creates witness draw_pub#2".to_string()],
            },
            EventDagNode {
                event_id: "handoff_turn#2".to_string(),
                event_kind: "handoff".to_string(),
                place_ref: ALICE_PLACE.to_string(),
                envelope_ref: Some("roll_request#2".to_string()),
                notes: vec!["turn ownership moves from Bob back to Alice after resumed dispatch".to_string()],
            },
        ],
        edges: vec![
            EventDagEdge {
                from_event: "save_local_state#1".to_string(),
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
            "save/load edge is local-only and does not imply distributed checkpoint closure".to_string(),
        ],
    }
}

fn retained_later_refs() -> Vec<String> {
    [
        "layer_insertion_runtime",
        "network_docker_runtime",
        "runtime_package_avatar_admission",
        "distributed_save_load",
        "final_public_abi",
    ]
    .into_iter()
    .map(|value| value.to_string())
    .collect()
}
