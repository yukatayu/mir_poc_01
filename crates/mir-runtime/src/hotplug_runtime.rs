//! Non-production hot-plug runtime/report narrow cut.
//!
//! This module consumes admitted `mirrorea-core` hot-plug carriers plus the thin
//! logical runtime substrate. The current line includes a narrow runtime-side
//! engine-state projection over that admitted carrier/substrate floor.
//!
//! It does not actualize rollback protocol, durable migration / reattach
//! semantics, distributed activation ordering, or a final public hot-plug ABI.

use mirrorea_core::{
    HotPlugRequest, HotPlugVerdict, LogicalPlaceRuntimeShell, LogicalPlaceRuntimeSnapshot,
    MirroreaCoreError, hotplug_request_lanes, hotplug_verdict_lanes,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HotPlugRuntimeSkeletonReport {
    pub skeleton_scope: String,
    pub request_lanes: Vec<String>,
    pub verdict_lanes: Vec<String>,
    pub request: HotPlugRequest,
    pub verdict: HotPlugVerdict,
    pub runtime_snapshot: LogicalPlaceRuntimeSnapshot,
    pub consumed_substrates: Vec<String>,
    pub retained_later_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HotPlugRuntimeEngineState {
    pub state_kind: String,
    pub request_ref: String,
    pub operation_kind: String,
    pub verdict_kind: String,
    pub requesting_principal: String,
    pub requesting_participant_place: String,
    pub active_membership_epoch: u64,
    pub reason_refs: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HotPlugRuntimeEngineReport {
    pub engine_scope: String,
    pub skeleton: HotPlugRuntimeSkeletonReport,
    pub engine_state: HotPlugRuntimeEngineState,
    pub retained_later_refs: Vec<String>,
    pub notes: Vec<String>,
}

pub fn assemble_hotplug_runtime_skeleton_report(
    shell: &LogicalPlaceRuntimeShell,
    request: HotPlugRequest,
    verdict: HotPlugVerdict,
) -> Result<HotPlugRuntimeSkeletonReport, MirroreaCoreError> {
    request.validate()?;
    verdict.validate()?;
    if verdict.request_ref != request.request_id {
        return Err(MirroreaCoreError::new(
            "hot-plug runtime skeleton verdict request_ref must match request.request_id",
        ));
    }

    let runtime_snapshot = shell.snapshot();
    let participant = runtime_snapshot
        .membership
        .members
        .get(&request.requesting_principal)
        .ok_or_else(|| {
            MirroreaCoreError::new(format!(
                "hot-plug runtime skeleton requires active membership for requesting principal {}",
                request.requesting_principal
            ))
        })?;
    if !participant.active {
        return Err(MirroreaCoreError::new(format!(
            "hot-plug runtime skeleton requires requesting principal {} to be active",
            request.requesting_principal
        )));
    }
    if participant.place != request.requesting_participant_place {
        return Err(MirroreaCoreError::new(
            "hot-plug runtime skeleton requesting participant place drifted from runtime snapshot",
        ));
    }
    if runtime_snapshot
        .place_catalog
        .places
        .get(&request.attachpoint_ref)
        .map(String::as_str)
        != Some("AttachPoint")
    {
        return Err(MirroreaCoreError::new(
            "hot-plug runtime skeleton requires a registered AttachPoint place",
        ));
    }

    Ok(HotPlugRuntimeSkeletonReport {
        skeleton_scope: "report_local_orchestration_skeleton".to_string(),
        request_lanes: hotplug_request_lanes(),
        verdict_lanes: hotplug_verdict_lanes(),
        request,
        verdict,
        runtime_snapshot,
        consumed_substrates: vec![
            "LogicalPlaceRuntimeShell".to_string(),
            "MembershipRegistry".to_string(),
            "PlaceCatalog".to_string(),
            "HotPlugRequest".to_string(),
            "HotPlugVerdict".to_string(),
        ],
        retained_later_refs: vec![
            "helper_local_lifecycle_ids".to_string(),
            "completed_engine".to_string(),
            "rollback_protocol".to_string(),
            "durable_migration".to_string(),
            "distributed_activation_ordering".to_string(),
            "final_public_hotplug_abi".to_string(),
        ],
        notes: vec![
            "consumer-side runtime/report assembly over admitted carriers and existing substrate"
                .to_string(),
            "no helper-local lifecycle ids imported as canonical runtime state".to_string(),
        ],
    })
}

pub fn assemble_hotplug_runtime_engine_report(
    shell: &LogicalPlaceRuntimeShell,
    request: HotPlugRequest,
    verdict: HotPlugVerdict,
) -> Result<HotPlugRuntimeEngineReport, MirroreaCoreError> {
    let skeleton = assemble_hotplug_runtime_skeleton_report(shell, request, verdict)?;
    let engine_state = HotPlugRuntimeEngineState {
        state_kind: engine_state_kind(
            &skeleton.request.operation_kind,
            &skeleton.verdict.verdict_kind,
        )?,
        request_ref: skeleton.request.request_id.clone(),
        operation_kind: skeleton.request.operation_kind.clone(),
        verdict_kind: skeleton.verdict.verdict_kind.clone(),
        requesting_principal: skeleton.request.requesting_principal.clone(),
        requesting_participant_place: skeleton.request.requesting_participant_place.clone(),
        active_membership_epoch: skeleton.runtime_snapshot.membership.membership_epoch,
        reason_refs: [
            skeleton.verdict.compatibility_reason_refs.clone(),
            skeleton.verdict.authorization_reason_refs.clone(),
            skeleton.verdict.membership_freshness_reason_refs.clone(),
        ]
        .concat(),
        notes: vec![
            "reason-to-state mapping over admitted request/verdict carrier only".to_string(),
            "rollback / durable migration / distributed activation ordering remain later"
                .to_string(),
        ],
    };

    Ok(HotPlugRuntimeEngineReport {
        engine_scope: "runtime_side_engine_state_progression_narrow".to_string(),
        retained_later_refs: skeleton.retained_later_refs.clone(),
        skeleton,
        engine_state,
        notes: vec![
            "runtime-side engine state progression remains narrow and non-public".to_string(),
            "helper-local lifecycle ids remain outside canonical runtime state".to_string(),
        ],
    })
}

pub fn build_hotplug_runtime_skeleton_report()
-> Result<HotPlugRuntimeSkeletonReport, MirroreaCoreError> {
    let (shell, request, verdict) = build_example_admitted_inputs()?;
    assemble_hotplug_runtime_skeleton_report(&shell, request, verdict)
}

pub fn build_hotplug_runtime_engine_report() -> Result<HotPlugRuntimeEngineReport, MirroreaCoreError>
{
    let (shell, request, verdict) = build_example_admitted_inputs()?;
    assemble_hotplug_runtime_engine_report(&shell, request, verdict)
}

fn build_example_admitted_inputs()
-> Result<(LogicalPlaceRuntimeShell, HotPlugRequest, HotPlugVerdict), MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place("AttachPoint[ExampleRoom#1]", "AttachPoint")?;
    shell.add_initial_participant("ExampleAdmin")?;

    let request = HotPlugRequest {
        request_id: "hotplug_request#runtime_skeleton".to_string(),
        attachpoint_ref: "AttachPoint[ExampleRoom#1]".to_string(),
        patch_ref: "Patch[ExamplePackage@runtime]".to_string(),
        operation_kind: "attach".to_string(),
        requesting_principal: "ExampleAdmin".to_string(),
        requesting_participant_place: "ParticipantPlace[ExampleAdmin]".to_string(),
        message_envelope_ref: "hotplug_request_envelope#runtime_skeleton".to_string(),
        auth_evidence_ref: None,
        capability_refs: vec![
            "ManageAttachPoint(ExampleRoom#1)".to_string(),
            "InstallPatch(ExamplePackage@runtime)".to_string(),
        ],
        witness_refs: vec!["membership_frontier_snapshot#runtime".to_string()],
        notes: vec![
            "example runtime/report assembly input".to_string(),
            "no helper-local lifecycle ids imported as canonical runtime state".to_string(),
        ],
    };
    let verdict = HotPlugVerdict {
        request_ref: request.request_id.clone(),
        verdict_kind: "accepted".to_string(),
        compatibility_reason_refs: vec!["attachpoint_registered".to_string()],
        authorization_reason_refs: vec!["attach_capability_present".to_string()],
        membership_freshness_reason_refs: vec!["membership_frontier_verified".to_string()],
        notes: vec![
            "example runtime/report assembly verdict".to_string(),
            "completed engine semantics remain deferred".to_string(),
        ],
    };

    Ok((shell, request, verdict))
}

fn engine_state_kind(
    operation_kind: &str,
    verdict_kind: &str,
) -> Result<String, MirroreaCoreError> {
    match (operation_kind, verdict_kind) {
        ("attach", "accepted") => Ok("attach_ready_for_activation_cut".to_string()),
        ("attach", "rejected") => Ok("attach_rejected_before_activation".to_string()),
        ("attach", "deferred") => Ok("attach_deferred_before_activation".to_string()),
        ("detach", "accepted") => Ok("detach_ready_for_boundary_cut".to_string()),
        ("detach", "rejected") => Ok("detach_rejected_before_boundary".to_string()),
        ("detach", "deferred") => Ok("detach_deferred_before_boundary".to_string()),
        _ => Err(MirroreaCoreError::new(format!(
            "hot-plug runtime engine report does not admit operation_kind `{operation_kind}` with verdict_kind `{verdict_kind}` in the current narrow cut"
        ))),
    }
}
