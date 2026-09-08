use super::*;
use crate::sys5_local_slice::{
    Sys5I3AdapterCarrierContract, Sys5LocalProject, Sys5SourceInput, build_project,
};

const SOURCE_PATH: &str = "tests/inline/i3_owner_admission_clock_runtime.mir";
const REQUESTER_SLOT: &str = "process-a";
const OWNER_SLOT: &str = "process-b";
const SECOND_OWNER_SLOT: &str = "process-c";
const TWO_OWNER_SOURCE_PATH: &str = "tests/inline/i3_owner_admission_two_owner_runtime.mir";
const FIRST_OPERATION: &str = "attack_s";
const SECOND_OPERATION: &str = "attack_t";

fn owner_admission_budget_source(ticks: u64) -> String {
    format!(
        "module Mirrorea.Sys5.I3OwnerAdmissionClock

locus WorldAuthority
locus ParticipantA
principal self
type Player

state avatar[id: Player] at WorldAuthority {{
  hp: Int
  visible observer_safe fields (hp)
}}

Role[self] at ParticipantA {{
  when init_avatar_hp() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, DeadlineExpired) within owner_ticks {ticks} {{
    at WorldAuthority {{
      avatar[self].hp = 21
    }}
  }}

  when refresh_avatar_hp() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, DeadlineExpired) {{
    at WorldAuthority {{
      avatar[self].hp = 34
    }}
  }}
}}

with auth MembershipAuth

verify finite_refinement
"
    )
}

fn source_started_runtime_pair(ticks: u64) -> (Sys5I3ProcessRuntime, Sys5I3ProcessRuntime) {
    source_started_runtime_pair_from_source(SOURCE_PATH, owner_admission_budget_source(ticks))
}

fn source_started_runtime_pair_from_source(
    source_path: &str,
    source: String,
) -> (Sys5I3ProcessRuntime, Sys5I3ProcessRuntime) {
    let project = build_project(Sys5SourceInput::inline(source_path, source))
        .expect("the ordinary budgeted source checks before its genuine SYS5 cohort is built");
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(REQUESTER_SLOT, "127.0.0.1:41001", ["ParticipantA"]),
            Sys5I3DeploymentSlot::new(OWNER_SLOT, "127.0.0.1:41002", ["WorldAuthority"]),
        ],
    )
    .expect("the source-derived requester and owner loci map once into two checked slots");
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("the checked project derives one genuine SYS5 cohort");
    let requester = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(REQUESTER_SLOT)
            .expect("the checked requester image is consumed exactly once"),
    )
    .expect("the source-derived requester runtime starts");
    let owner = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(OWNER_SLOT)
            .expect("the checked owner image is consumed exactly once"),
    )
    .expect("the source-derived owner runtime starts");
    (requester, owner)
}

fn terminal_failure_capacity_source() -> String {
    "module Mirrorea.Sys5.I3OwnerAdmissionTerminalCapacity

locus A
locus S
locus T
principal self
type Player

state s_avatar[id: Player] at S {
  hp: Int
}

state t_avatar[id: Player] at T {
  hp: Int
}

Role[self] at A {
  when expire_s() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, DeadlineExpired) within owner_ticks 1 {
    at S {
      s_avatar[self].hp = 21
    }
  }

  when expire_t() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, DeadlineExpired) within owner_ticks 1 {
    at T {
      t_avatar[self].hp = 34
    }
  }
}

with auth MembershipAuth

verify finite_refinement
"
    .to_string()
}

fn source_started_terminal_failure_capacity_runtimes() -> (
    Sys5I3ProcessRuntime,
    Sys5I3ProcessRuntime,
    Sys5I3ProcessRuntime,
) {
    let project = build_project(Sys5SourceInput::inline(
        "tests/inline/i3_owner_admission_terminal_capacity_runtime.mir",
        terminal_failure_capacity_source(),
    ))
    .expect("the two-owner capacity source checks before its three real process runtimes start");
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(REQUESTER_SLOT, "127.0.0.1:41301", ["A"]),
            Sys5I3DeploymentSlot::new(OWNER_SLOT, "127.0.0.1:41302", ["S"]),
            Sys5I3DeploymentSlot::new(SECOND_OWNER_SLOT, "127.0.0.1:41303", ["T"]),
        ],
    )
    .expect("the requester and two owners occupy three exact checked slots");
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("the checked two-owner source derives one genuine three-slot cohort");
    let requester = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(REQUESTER_SLOT)
            .expect("the requester image is consumed exactly once"),
    )
    .expect("the checked requester runtime starts");
    let owner_s = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(OWNER_SLOT)
            .expect("the S owner image is consumed exactly once"),
    )
    .expect("the checked S owner runtime starts");
    let owner_t = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(SECOND_OWNER_SLOT)
            .expect("the T owner image is consumed exactly once"),
    )
    .expect("the checked T owner runtime starts");
    (requester, owner_s, owner_t)
}

fn two_owner_budgeted_source() -> String {
    "module Mirrorea.Sys5.I3OwnerAdmissionTwoOwner

locus A
locus S
locus T
principal self
type Player

state player[id: Player] at S {
  hp: Int
}

state shield[id: Player] at T {
  hp: Int
}

Role[self] at A {
  when attack_s() fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, DeadlineExpired) within owner_ticks 1 {
    at S {
      player[self].hp = 21
    }
  }

  when attack_t() fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable) {
    at T {
      shield[self].hp = 34
    }
  }
}

with auth MembershipAuth

verify finite_refinement
"
    .to_string()
}

fn two_owner_two_clock_source() -> String {
    two_owner_budgeted_source().replacen(
        "when attack_t() fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable) {",
        "when attack_t() fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable, DeadlineExpired) within owner_ticks 1 {",
        1,
    )
}

fn source_started_two_owner_runtime() -> (Sys5I3ProcessRuntime, Sys5I3ProcessRuntime) {
    let project = build_project(Sys5SourceInput::inline(
        "tests/inline/i3_owner_admission_two_owner_two_clock_runtime.mir",
        two_owner_two_clock_source(),
    ))
    .expect("the two independent no-argument checked owner conditions build before runtime start");
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(REQUESTER_SLOT, "127.0.0.1:41201", ["A"]),
            Sys5I3DeploymentSlot::new(OWNER_SLOT, "127.0.0.1:41202", ["S", "T"]),
        ],
    )
    .expect(
        "the requester and both checked owner loci map exactly once into the two runtime slots",
    );
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("the two-clock checked source derives one genuine SYS5 cohort");
    let requester = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(REQUESTER_SLOT)
            .expect("the requester image is consumed once"),
    )
    .expect("the checked requester starts");
    let owner = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(OWNER_SLOT)
            .expect("the shared S/T owner image is consumed once"),
    )
    .expect("the checked shared owner runtime starts");
    (requester, owner)
}

fn source_checked_owner_request_contract(
    project: &Sys5LocalProject,
    operation: &str,
) -> Sys5I3AdapterCarrierContract {
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.operation_id == operation && edge.kind == "owner-request")
        .unwrap_or_else(|| {
            panic!("the checked two-owner source retains {operation}'s request edge")
        });
    project
        .i3_adapter_carrier_contract(&edge.edge_ref)
        .unwrap_or_else(|_| {
            panic!("the checked {operation} request edge has its exact adapter contract")
        })
}

fn started_two_owner_lifecycle_pair(
    revoked_operation: &str,
) -> (
    Sys5I3ProcessRuntime,
    Sys5I3ProcessRuntime,
    Sys5I3AdmittedLifecycleStimulus,
) {
    let project = build_project(Sys5SourceInput::inline(
        TWO_OWNER_SOURCE_PATH,
        two_owner_budgeted_source(),
    ))
    .expect("the two-owner checked source accepts a budget only for no-argument attack_s");
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(REQUESTER_SLOT, "127.0.0.1:41101", ["A"]),
            Sys5I3DeploymentSlot::new(OWNER_SLOT, "127.0.0.1:41102", ["S", "T"]),
        ],
    )
    .expect("the requester and both active owner loci map exactly once");
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("the two-owner checked project derives one genuine SYS5 cohort");
    let revocation_contract = source_checked_owner_request_contract(&project, revoked_operation);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    cohort
        .prestage_owner_capability_revocation(
            "i3-owner-admission-clock-g1-to-g2",
            &revocation_contract,
        )
        .expect("the parent prestages only the exact source-derived selected owner contract");
    let (requester_control, owner_control) = cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            "i3-owner-admission-clock-g1-to-g2",
            REQUESTER_SLOT,
            "requester-spki:i3-owner-admission-clock-g1-to-g2",
            OWNER_SLOT,
            "owner-spki:i3-owner-admission-clock-g1-to-g2",
        )
        .expect("the genuine prestaged lifecycle supplies independently bound requester and owner controls");
    let requester_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(
                    cohort
                        .take_process_image(REQUESTER_SLOT)
                        .expect("the checked requester image is consumed once"),
                )
                .expect("the requester image encodes only as tainted child input"),
        )
        .expect("the requester image remains an untrusted decode candidate before its trusted control validates it");
    let owner_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(
                    cohort
                        .take_process_image(OWNER_SLOT)
                        .expect("the checked owner image is consumed once"),
                )
                .expect("the owner image encodes only as tainted child input"),
        )
        .expect("the owner image remains an untrusted decode candidate before its trusted control validates it");
    let (requester, _requester_control, requester_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(requester_image, requester_control)
        .expect("the matching requester control starts the genuine G1 runtime");
    assert!(
        requester_stimulus.is_none(),
        "only the selected owner child may receive the opaque lifecycle stimulus"
    );
    let (owner, _owner_control, owner_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(owner_image, owner_control)
        .expect("the matching owner control starts the genuine G1 runtime");
    (
        requester,
        owner,
        owner_stimulus.expect("the selected owner receives one opaque G2 lifecycle stimulus"),
    )
}

fn stage_budgeted_request(requester: &mut Sys5I3ProcessRuntime, owner: &mut Sys5I3ProcessRuntime) {
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked budgeted source derives its exact owner request");
    assert!(
        owner
            .accept_inbound(request)
            .expect("the budgeted request stages before ordinary owner execution")
            .is_none(),
        "staging has no owner reply before serialized gate resolution"
    );
}

fn only_owner_clock(owner: &Sys5I3ProcessRuntime) -> Sys5I3OwnerAdmissionClockHandle {
    let mut handles = owner.i3_trusted_owner_admission_clock_handles();
    assert_eq!(
        handles.len(),
        1,
        "one staged source condition exposes one private trusted owner-local clock handle"
    );
    handles.pop().expect(
        "the checked owner clock handle remains available only to the trusted owner controller",
    )
}

fn expect_owner_admission_reserved(
    resolution: Sys5I3OwnerAdmissionResolution,
    expectation: &str,
) -> Sys5I3OwnerAdmissionReserved {
    match resolution {
        Sys5I3OwnerAdmissionResolution::ServeReserved(reserved) => *reserved,
        Sys5I3OwnerAdmissionResolution::DeclaredOwnerFailure(_) => {
            panic!("{expectation}")
        }
    }
}

fn expect_declared_deadline_expired(
    resolution: Sys5I3OwnerAdmissionResolution,
    expectation: &str,
) -> Sys5I3ProcessMessage {
    match resolution {
        Sys5I3OwnerAdmissionResolution::DeclaredOwnerFailure(message) => *message,
        Sys5I3OwnerAdmissionResolution::ServeReserved(_) => {
            panic!("{expectation}")
        }
    }
}

fn encoded_declared_deadline_expired_reply(
    requester: &mut Sys5I3ProcessRuntime,
    owner: &mut Sys5I3ProcessRuntime,
) -> (String, Sys5I3PrivateProcessCodec, Vec<u8>) {
    encoded_declared_deadline_expired_reply_for_operation(requester, owner, "init_avatar_hp", 1)
}

fn encoded_declared_deadline_expired_reply_for_operation(
    requester: &mut Sys5I3ProcessRuntime,
    owner: &mut Sys5I3ProcessRuntime,
    operation: &str,
    resolution_tick: u64,
) -> (String, Sys5I3PrivateProcessCodec, Vec<u8>) {
    let request = requester
        .emit_generated_owner_request(operation)
        .unwrap_or_else(|_| {
            panic!("the checked budgeted source emits one exact {operation} request")
        });
    let request_identity_ref = request.semantic_request_identity_ref().to_string();
    assert!(
        owner
            .accept_inbound(request)
            .expect("the budgeted request stages before a deadline can be resolved")
            .is_none()
    );
    let clock = only_owner_clock(owner);
    owner
        .advance_owner_admission_clock(&clock, resolution_tick)
        .expect("the checked clock reaches the supplied staged-request resolution tick");
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the clock yields its one genuine staged request")
        .expect("the retained request remains Awaiting before resolution");
    let declared_reply = expect_declared_deadline_expired(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("expiry remains a gate decision rather than an owner execution error"),
        "the expired source request yields one normal generated declared-failure reply",
    );
    assert_eq!(declared_reply.kind, Sys5I3ProcessMessageKind::Reply);
    assert!(declared_reply.carrier.is_some());
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let bytes = codec
        .encode_outbound_message(declared_reply)
        .expect("the genuine declared reply encodes through the ordinary private codec");
    (request_identity_ref, codec, bytes)
}

fn mutate_declared_reply_string_field(
    codec: &Sys5I3PrivateProcessCodec,
    bytes: &[u8],
    pointer: &str,
    replacement: &str,
) -> Vec<u8> {
    let body = codec
        .unframe_body(bytes, Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES)
        .expect("the genuine declared reply has one complete bounded frame");
    let mut value = strict_json_value(body)
        .expect("the genuine declared reply body is one strict private JSON value");
    *value
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("the declared-reply snapshot retains {pointer}")) =
        serde_json::Value::String(replacement.to_string());
    let body = serde_json::to_vec(&value)
        .expect("the bounded test-only mutation remains syntactically valid JSON");
    codec
        .frame_body(body, Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES)
        .expect("the bounded test-only mutation reframes through the real private codec limit")
}

/// Replace only the untrusted OwnerReply outcome, retaining the ordinary
/// reply's carrier and rebinding the inserted failure's request-side fields
/// to the receiver's real padded-source pending entry.  The decision fields
/// remain the opaque values from a separate genuine gate-produced expiry;
/// this helper never constructs a failure or an authority proof.
fn snapshot_string(snapshot: &serde_json::Value, pointer: &str, expectation: &str) -> String {
    snapshot
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("{expectation}: snapshot retains a nonempty string at {pointer}"))
        .to_string()
}

fn substitute_declared_expiry_outcome_for_pending(
    codec: &Sys5I3PrivateProcessCodec,
    ordinary_reply_bytes: &[u8],
    genuine_declared_expiry_bytes: &[u8],
    ordinary_request_snapshot: &serde_json::Value,
) -> Vec<u8> {
    let ordinary_body = codec
        .unframe_body(
            ordinary_reply_bytes,
            Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES,
        )
        .expect("the ordinary source-generated reply has one complete bounded frame");
    let mut ordinary = strict_json_value(ordinary_body)
        .expect("the ordinary source-generated reply body is strict private JSON");
    let declared_body = codec
        .unframe_body(
            genuine_declared_expiry_bytes,
            Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES,
        )
        .expect("the genuine gate-produced expiry has one complete bounded frame");
    let declared = strict_json_value(declared_body)
        .expect("the genuine gate-produced expiry body is strict private JSON");
    let declared_outcome = declared
        .pointer("/message/carrier/payload/fields/outcome")
        .cloned()
        .expect("the genuine declared reply retains its typed OwnerReply outcome");
    *ordinary
        .pointer_mut("/message/carrier/payload/fields/outcome")
        .expect("the ordinary reply retains only its typed OwnerReply outcome slot") =
        declared_outcome;

    let request_id = snapshot_string(
        &ordinary,
        "/message/carrier/request_id",
        "the ordinary reply retains its generated request identity",
    );
    let request_carrier_id = snapshot_string(
        &ordinary,
        "/message/carrier/request_carrier_id",
        "the ordinary reply retains its generated request-carrier identity",
    );
    let operation_id = snapshot_string(
        &ordinary,
        "/message/carrier/operation_id",
        "the ordinary reply retains its source-selected operation",
    );
    let request_edge_ref = snapshot_string(
        ordinary_request_snapshot,
        "/edge_ref",
        "the exact retained ordinary request snapshot retains its generated request edge",
    );
    let reply_edge_ref = snapshot_string(
        &ordinary,
        "/message/carrier/edge_ref",
        "the ordinary reply retains its generated reply edge",
    );
    let requester_locus = snapshot_string(
        &ordinary,
        "/message/carrier/target_locus",
        "the ordinary reply retains its checked requester target",
    );
    let owner_locus = snapshot_string(
        &ordinary,
        "/message/carrier/source_locus",
        "the ordinary reply retains its checked owner source",
    );
    let core_ref = snapshot_string(
        &ordinary,
        "/message/carrier/core_ref",
        "the ordinary reply retains its exact checked Core reference",
    );
    let owner_lineage_ref = snapshot_string(
        &ordinary,
        "/message/carrier/m9_owner_lineage_ref",
        "the ordinary reply retains its exact M9 owner lineage",
    );
    for (pointer, replacement) in [
        (
            "/message/carrier/payload/fields/outcome/fields/failure/request_id",
            request_id,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/request_carrier_id",
            request_carrier_id,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/operation_id",
            operation_id,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/request_edge_ref",
            request_edge_ref,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/reply_edge_ref",
            reply_edge_ref,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/requester_locus",
            requester_locus,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/owner_locus",
            owner_locus,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/core_ref",
            core_ref,
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/owner_lineage_ref",
            owner_lineage_ref,
        ),
    ] {
        *ordinary.pointer_mut(pointer).unwrap_or_else(|| {
            panic!("the inserted declared-failure snapshot retains {pointer}")
        }) = serde_json::Value::String(replacement);
    }
    let body = serde_json::to_vec(&ordinary)
        .expect("the test-only untrusted payload substitution remains syntactically valid JSON");
    codec
        .frame_body(body, Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES)
        .expect("the untrusted payload substitution reframes through the real private codec limit")
}

#[test]
fn i3_owner_admission_budget_tick_at_start_serves_once_and_tick_at_deadline_expires_without_owner_effects()
 {
    let (mut on_time_requester, mut on_time_owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut on_time_requester, &mut on_time_owner);
    let on_time_clock = only_owner_clock(&on_time_owner);
    assert_eq!(
        on_time_owner
            .observer_safe_owner_admission_summary()
            .awaiting_count(),
        1,
        "the exact source request is retained Awaiting before its owner-local clock resolves it"
    );

    let on_time_awaiting = on_time_owner
        .take_next_owner_admission_awaiting(&on_time_clock)
        .expect("the owner clock yields its exact staged request deterministically")
        .expect("one checked request remains Awaiting at tick zero");
    let on_time_reserved = expect_owner_admission_reserved(
        on_time_owner
            .resolve_staged_owner_admission(on_time_awaiting)
            .expect("resolve tick zero is strictly before a budget-one deadline"),
        "on-time resolution issues one opaque reservation",
    );
    let on_time_reply = on_time_owner
        .handoff_reserved_owner_admission(on_time_reserved)
        .expect("the one-use reservation reaches the existing actual SYS4/M8 handoff");
    let on_time_request_identity_ref = on_time_reply.semantic_request_identity_ref().to_string();
    assert_eq!(
        on_time_owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "a successful on-time handoff has one actual owner serve record"
    );
    assert_eq!(
        on_time_owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "a successful on-time handoff has one actual owner write record"
    );
    let on_time_record = on_time_owner
        .inbound_owner_request_tombstones
        .get(&on_time_request_identity_ref)
        .expect("the served request retains its exact owner-side replay tombstone");
    assert_eq!(
        on_time_record.tombstone.phase,
        Sys5I3InboundOwnerRequestTombstonePhase::Received,
        "a successful handoff records its actual receipt rather than remaining an outstanding reservation"
    );
    assert!(
        on_time_record.live_admission.is_none(),
        "a received request retains neither an Awaiting issuance nor a live handoff carrier"
    );
    assert_eq!(
        on_time_owner
            .observer_safe_owner_admission_summary()
            .serve_reserved_count(),
        0,
        "a completed handoff cannot remain observable as an outstanding ServeReserved admission"
    );
    let on_time_occurrences = on_time_owner.observer_safe_semantic_occurrences();
    assert_eq!(on_time_occurrences.owner_serve_linearization_count(), 1);
    assert_eq!(on_time_occurrences.actual_owner_write_count(), 1);
    assert!(
        on_time_occurrences
            .owner_serve_linearization_occurrence_ref(&on_time_request_identity_ref)
            .is_some(),
        "the successful reply identity is backed by an actual owner-serve linearization occurrence"
    );
    assert!(
        on_time_occurrences
            .actual_owner_write_occurrence_ref(&on_time_request_identity_ref)
            .is_some(),
        "the successful reply identity is backed by an actual owner-write occurrence"
    );

    let (mut expired_requester, mut expired_owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut expired_requester, &mut expired_owner);
    let expired_clock = only_owner_clock(&expired_owner);
    expired_owner
        .advance_owner_admission_clock(&expired_clock, 1)
        .expect("the owner-local clock may advance to the exact checked deadline");
    let expired_awaiting = expired_owner
        .take_next_owner_admission_awaiting(&expired_clock)
        .expect("the owner clock yields its exact staged request at its deadline")
        .expect("one checked request remains Awaiting until gate resolution");
    let expired_reply = expect_declared_deadline_expired(
        expired_owner
            .resolve_staged_owner_admission(expired_awaiting)
            .expect("deadline expiry is a retained owner decision, not an execution error"),
        "a budget-one request at start-plus-one must produce a declared failure rather than a generated success reply",
    );
    assert_eq!(expired_reply.kind, Sys5I3ProcessMessageKind::Reply);
    assert!(
        expired_reply.carrier.is_some(),
        "the declared expiry remains a normal generated reply carrier for the shared codec path"
    );
    let expired = expired_owner.observer_safe_owner_admission_summary();
    assert_eq!(expired.awaiting_count(), 0);
    assert_eq!(expired.expired_count(), 1);
    assert_eq!(expired.rejected_before_serve_count(), 0);
    assert_eq!(expired.serve_reserved_count(), 0);
    assert_eq!(
        expired_owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "expiry must not be reported as an owner serve"
    );
    assert_eq!(
        expired_owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "expiry must not mutate owner state"
    );
    assert_eq!(
        expired_requester.observer_safe_pending_owner_request_count(),
        1,
        "until a future typed outcome is actually delivered, requester knowledge remains pending"
    );
}

/// C3 availability regression: expiry must expose a gate-produced, sendable
/// declared outcome. This test intentionally stops before delivery: requester
/// pending state may change only after the exact codec and requester-side
/// validation path consumes that declared outcome.
#[test]
fn i3_owner_admission_budget_expiry_exposes_a_sendable_declared_outcome_before_requester_terminal_consumption()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    let clock = only_owner_clock(&owner);
    owner
        .advance_owner_admission_clock(&clock, 1)
        .expect("the checked owner-local clock reaches the exact source budget deadline");
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the deadline clock yields the retained source-derived request")
        .expect("the genuine request remains Awaiting until its checked resolution");

    let declared_reply = expect_declared_deadline_expired(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("expiry is a committed owner-gate decision, not a local execution error"),
        "C3 API-transition RED: an expired checked request must expose a sendable gate-produced declared outcome rather than disappearing as None",
    );
    assert_eq!(declared_reply.kind, Sys5I3ProcessMessageKind::Reply);
    assert!(declared_reply.carrier.is_some());
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "before a future declared outcome is encoded, validated, and delivered, requester knowledge remains pending"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "the availability of a declared expiry outcome is not an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the availability of a declared expiry outcome is not an owner write"
    );
}

/// Component-only replay coverage: the owner retains its gate-produced expiry
/// after its outcome is dropped. This is not a reconnect or network claim.
#[test]
fn i3_owner_admission_budget_dropped_declared_expiry_rejects_the_exact_request_replay_without_reissue()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let source_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked budgeted source emits one exact requester-pending owner request");
    let request_identity_ref = source_request.semantic_request_identity_ref().to_string();
    let exact_request_bytes = codec.encode_outbound_message(source_request).expect(
        "the exact generated request encodes for its first delivery and later identical replay",
    );
    assert!(
        owner
            .admit_decoded_process_message(
                codec
                    .decode_untrusted_message(&exact_request_bytes)
                    .expect("the first exact request bytes decode only as an untrusted candidate"),
            )
            .expect("the first exact request stages at the checked owner-admission gate")
            .is_none(),
        "staging the genuine budgeted request produces no owner reply before gate resolution"
    );
    let clock = only_owner_clock(&owner);
    owner
        .advance_owner_admission_clock(&clock, 1)
        .expect("the trusted owner-local clock advances to the exact budget-one deadline");
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the trusted clock yields the one retained source request")
        .expect("the genuine request remains Awaiting until its exact expiry resolution");
    let dropped_declared_expiry_reply = expect_declared_deadline_expired(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the exact deadline resolution is a gate-produced declared expiry"),
        "a budget-one request at its deadline must retain expiry rather than reserve a serve",
    );
    drop(dropped_declared_expiry_reply);

    let owner_admission_before_replay = owner.observer_safe_owner_admission_summary();
    let owner_runtime_before_replay = owner.observer_safe_runtime_summary();
    let owner_occurrences_before_replay = owner.observer_safe_semantic_occurrences();
    let owner_outbox_before_replay = owner.observer_safe_outbox_summary();
    let owner_trace_before_replay = owner.fabric.trace().clone();
    let owner_semantic_store_before_replay = owner.fabric.semantic_snapshot();
    let owner_causality_before_replay = owner.fabric.causality().clone();
    assert_eq!(owner_admission_before_replay.awaiting_count(), 0);
    assert_eq!(owner_admission_before_replay.expired_count(), 1);
    assert_eq!(
        owner_admission_before_replay.rejected_before_serve_count(),
        0
    );
    assert_eq!(owner_admission_before_replay.serve_reserved_count(), 0);
    assert_eq!(owner_runtime_before_replay.served_owner_request_count(), 0);
    assert_eq!(owner_runtime_before_replay.actual_owner_write_count(), 0);
    assert_eq!(
        owner_occurrences_before_replay.owner_serve_linearization_count(),
        0
    );
    assert_eq!(
        owner_occurrences_before_replay.actual_owner_write_count(),
        0
    );
    let retained_expiry_before_replay = owner
        .inbound_owner_request_tombstones
        .get(&request_identity_ref)
        .expect("the dropped outcome retains its exact owner-side tombstone");
    assert_eq!(
        retained_expiry_before_replay.tombstone.phase,
        Sys5I3InboundOwnerRequestTombstonePhase::Expired,
        "the retained owner record is terminal Expired rather than Awaiting or ServeReserved"
    );
    assert!(
        retained_expiry_before_replay.live_admission.is_none(),
        "the expired tombstone retains no live issuance that could restart the budget"
    );
    assert!(
        retained_expiry_before_replay
            .tombstone
            .terminal_admission
            .is_some(),
        "the expired tombstone retains the exact gate decision after its outcome is dropped"
    );
    assert_eq!(requester.observer_safe_pending_owner_request_count(), 1);
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        0,
        "dropping an owner outcome cannot manufacture requester terminal knowledge"
    );

    assert_eq!(
        owner
            .admit_decoded_process_message(
                codec
                    .decode_untrusted_message(&exact_request_bytes)
                    .expect("the exact replay bytes remain only an untrusted candidate"),
            )
            .expect_err("the retained Expired tombstone must reject the exact source-generated request replay")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected
    );
    assert_eq!(
        owner.observer_safe_owner_admission_summary(),
        owner_admission_before_replay,
        "the exact request replay cannot restart its budget, issue another expiry, or reserve a serve"
    );
    assert_eq!(
        owner.observer_safe_runtime_summary(),
        owner_runtime_before_replay,
        "the exact request replay cannot manufacture owner serve, write, or successful receipt state"
    );
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        owner_occurrences_before_replay,
        "the exact request replay cannot mint an owner serve or write occurrence"
    );
    assert_eq!(
        owner.observer_safe_outbox_summary(),
        owner_outbox_before_replay,
        "the exact request replay cannot issue another outbound owner outcome"
    );
    assert_eq!(
        owner.fabric.trace(),
        &owner_trace_before_replay,
        "the duplicate rejection must not append owner SYS4 receive or dequeue trace evidence"
    );
    assert!(
        owner
            .fabric
            .semantic_snapshot()
            .same_state(&owner_semantic_store_before_replay),
        "the duplicate rejection must not mutate owner SYS4 state"
    );
    assert_eq!(
        owner.fabric.causality(),
        &owner_causality_before_replay,
        "the duplicate rejection must not add an owner causality edge"
    );
    let retained_expiry_after_replay = owner
        .inbound_owner_request_tombstones
        .get(&request_identity_ref)
        .expect("the duplicate rejection retains the original Expired tombstone");
    assert_eq!(
        retained_expiry_after_replay.tombstone.phase,
        Sys5I3InboundOwnerRequestTombstonePhase::Expired,
        "the duplicate replay cannot relabel retained expiry as another terminal state"
    );
    assert!(retained_expiry_after_replay.live_admission.is_none());
    assert!(
        owner
            .take_next_owner_admission_awaiting(&clock)
            .expect("the trusted owner clock remains inspectable after duplicate rejection")
            .is_none(),
        "the retained expiry cannot reissue a second Awaiting handle"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "without a delivered terminal outcome, requester knowledge remains pending after owner-side replay rejection"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        0,
        "owner-side replay rejection cannot create requester terminal consumption"
    );
}

#[test]
fn i3_owner_admission_budget_declared_expiry_uses_the_shared_reply_codec_and_commits_a_local_terminal_before_removing_pending()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked budgeted source emits one exact pending owner request");
    let request_identity_ref = request.semantic_request_identity_ref().to_string();
    assert!(
        owner
            .accept_inbound(request)
            .expect("the budgeted source request stages before the owner-local deadline decision")
            .is_none()
    );
    let clock = only_owner_clock(&owner);
    owner
        .advance_owner_admission_clock(&clock, 1)
        .expect("the checked owner-local clock reaches the budget-one deadline");
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the owner clock deterministically yields the staged source request")
        .expect("the checked request remains Awaiting until the expiry decision");
    let declared_reply = expect_declared_deadline_expired(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("expiry is a committed gate decision, not an owner execution error"),
        "the expired checked request must yield the gate-produced declared reply carrier",
    );
    assert_eq!(declared_reply.kind, Sys5I3ProcessMessageKind::Reply);
    assert!(
        declared_reply.carrier.is_some(),
        "the declared failure moves through the same generated OwnerReply carrier route as success"
    );
    assert!(
        !declared_reply.is_observer_safe_typed_result_or_receipt(),
        "the remote declared failure is not a requester-local receipt"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "the requester remains pending until it validates and consumes the exact declared reply"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        0,
        "resolution alone cannot claim requester terminal consumption"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "expiry must not manufacture an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "expiry must not manufacture an owner write"
    );

    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let declared_reply_bytes = codec
        .encode_outbound_message(declared_reply)
        .expect("the gate-produced declared reply uses the ordinary private reply codec");
    let local_terminal = requester
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&declared_reply_bytes)
                .expect("the exact declared reply bytes decode only as an untrusted candidate"),
        )
        .expect(
            "the requester validates the exact source/Core/edge/current-generation declared reply",
        )
        .expect(
            "a validated declared failure produces an explicit local terminal consumption result",
        );
    assert!(
        local_terminal.is_observer_safe_terminal_failure_consumed(),
        "a declared expiry must not be confused with the existing no-reply admission shape"
    );
    assert!(
        !local_terminal.is_observer_safe_typed_result_or_receipt(),
        "the local terminal consumption result is not a success receipt"
    );
    assert!(
        local_terminal.has_no_transportable_carrier(),
        "the requester-local terminal consumption result cannot become a third reply carrier"
    );
    assert_eq!(
        local_terminal.linked_request_identity_ref(),
        Some(request_identity_ref.as_str()),
        "the local terminal consumption result remains linked to the exact pending source request it consumed"
    );
    assert_eq!(
        codec
            .encode_outbound_message(local_terminal)
            .expect_err("a requester-local terminal failure must not be serializable")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::TerminalFailureIsLocalOnly
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        0,
        "the requester removes pending only after it retains the validated terminal outcome"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        1,
        "the requester records one actual declared-failure consumption"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        0,
        "a declared failure cannot increment the successful receipt count"
    );
    let requester_occurrences = requester.observer_safe_semantic_occurrences();
    assert_eq!(
        requester_occurrences.requester_terminal_declared_owner_failure_count(),
        1,
        "the consumed terminal is retained as its own observer-safe requester occurrence"
    );
    assert!(
        requester_occurrences
            .requester_terminal_declared_owner_failure_occurrence_ref(&request_identity_ref)
            .is_some(),
        "the retained terminal occurrence remains bound to the exact original request identity"
    );
    assert!(
        requester_occurrences
            .requester_local_receipt_occurrence_ref(&request_identity_ref)
            .is_none(),
        "a declared failure must not install a successful receipt occurrence for the same request"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "requester consumption of a declared expiry cannot retroactively report an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "requester consumption of a declared expiry cannot retroactively report an owner write"
    );
}

#[test]
fn i3_owner_admission_budget_declared_expiry_rejects_malformed_or_mismatched_reply_snapshots_and_replay_without_consuming_pending()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let (request_identity_ref, codec, declared_reply_bytes) =
        encoded_declared_deadline_expired_reply(&mut requester, &mut owner);

    let unknown_outcome = mutate_declared_reply_string_field(
        &codec,
        &declared_reply_bytes,
        "/message/carrier/payload/fields/outcome/kind",
        "unknown_declared_owner_reply_outcome",
    );
    assert_eq!(
        codec
            .decode_untrusted_message(&unknown_outcome)
            .expect_err(
                "an unknown declared owner-reply outcome must fail closed in the private codec"
            )
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "codec rejection cannot consume the exact source requester pending request"
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_count(),
        0,
        "codec rejection cannot fabricate a requester terminal occurrence"
    );

    for (pointer, replacement, label) in [
        (
            "/message/carrier/payload/fields/outcome/fields/failure/request_id",
            "wrong-declared-request-id",
            "wrong declared request identity",
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/reply_edge_ref",
            "wrong-declared-reply-edge",
            "wrong declared reply edge",
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/core_ref",
            "wrong-declared-core",
            "wrong declared Core reference",
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/decision_generation_ref",
            "m9-authority-generation:99999999999999999999",
            "wrong current decision generation",
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/decision_commitment_ref",
            "sys5-i3-owner-admission-deadline-expired-sha256-v1:0000000000000000000000000000000000000000000000000000000000000000",
            "nonempty but inconsistent declared decision commitment",
        ),
        (
            "/message/carrier/payload/fields/outcome/fields/failure/decision_occurrence_ref",
            "sys4-i3-owner-declared-deadline-expired:sys5-i3-owner-admission-deadline-expired-sha256-v1:0000000000000000000000000000000000000000000000000000000000000000",
            "nonempty but inconsistent declared decision occurrence",
        ),
    ] {
        let mutated =
            mutate_declared_reply_string_field(&codec, &declared_reply_bytes, pointer, replacement);
        assert_eq!(
            requester
                .admit_decoded_process_message(
                    codec
                        .decode_untrusted_message(&mutated)
                        .unwrap_or_else(|_| panic!("the {label} frame remains syntactically untrusted JSON")),
                )
                .expect_err(&format!(
                    "a {label} cannot satisfy the requester’s exact source/Core/edge/current-generation pending contract"
                ))
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
        );
        assert_eq!(
            requester.observer_safe_pending_owner_request_count(),
            1,
            "a {label} must leave the genuine requester pending request intact"
        );
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_declared_owner_failure_count(),
            0,
            "a {label} must not count as terminal failure consumption"
        );
        assert_eq!(
            requester
                .observer_safe_semantic_occurrences()
                .requester_terminal_declared_owner_failure_count(),
            0,
            "a {label} must not fabricate a requester terminal occurrence"
        );
    }

    let terminal = requester
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&declared_reply_bytes)
                .expect("the original gate-produced declared reply remains an untrusted candidate"),
        )
        .expect(
            "the exact original declared reply remains admissible after every rejected candidate",
        )
        .expect(
            "the original declared reply produces one explicit requester-local terminal result",
        );
    assert!(terminal.is_observer_safe_terminal_failure_consumed());
    assert_eq!(requester.observer_safe_pending_owner_request_count(), 0);
    assert!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_occurrence_ref(&request_identity_ref)
            .is_some(),
        "the exact validated reply records one terminal occurrence before its pending entry is removed"
    );

    assert_eq!(
        requester
            .admit_decoded_process_message(
                codec
                    .decode_untrusted_message(&declared_reply_bytes)
                    .expect("replayed original bytes remain only an untrusted candidate"),
            )
            .expect_err("a consumed declared terminal cannot be replayed as a second result")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        1,
        "replay cannot increment the accepted terminal-failure count"
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_count(),
        1,
        "replay cannot add a second terminal occurrence for the consumed request"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "rejected terminal candidates and replay cannot manufacture an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "rejected terminal candidates and replay cannot manufacture an owner write"
    );
}

#[test]
fn i3_owner_admission_budget_rejects_a_declared_expiry_substituted_into_a_padded_unbudgeted_reply_before_terminal_or_receipt_mutation()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let padded_request = requester
        .emit_generated_owner_request("refresh_avatar_hp")
        .expect(
            "the source-derived DeadlineExpired-padded request emits through its ordinary route",
        );
    let padded_request_identity_ref = padded_request.semantic_request_identity_ref().to_string();
    let padded_request_record = requester
        .pending_outbound_owner_requests
        .get(&padded_request_identity_ref)
        .expect("the requester retains the exact pending binding before ordinary reply admission");
    let padded_pending = padded_request_record.pending.clone();
    assert!(
        padded_pending.owner_admission_budget().is_none(),
        "a declared DeadlineExpired name without within owner_ticks remains an unbudgeted checked request"
    );
    let padded_request_snapshot = serde_json::to_value(
        padded_request_record
            .exact_carrier
            .i3_private_process_snapshot()
            .expect(
                "the retained exact padded source request has one private snapshot for untrusted-boundary comparison",
            ),
    )
    .expect("the retained exact padded request snapshot serializes for test-only JSON rebinding");
    let ordinary_reply = owner
        .accept_inbound(padded_request)
        .expect("the padded but unannotated source request follows the ordinary owner path")
        .expect("the padded but unannotated source request produces one normal success reply");
    let ordinary_reply_bytes = codec
        .encode_outbound_message(ordinary_reply)
        .expect("the ordinary source-generated success reply uses the shared private codec");
    let (mut expiry_requester, mut expiry_owner) = source_started_runtime_pair(1);
    let (_expiry_request_identity_ref, _expiry_codec, genuine_declared_expiry_bytes) =
        encoded_declared_deadline_expired_reply(&mut expiry_requester, &mut expiry_owner);
    let substituted = substitute_declared_expiry_outcome_for_pending(
        &codec,
        &ordinary_reply_bytes,
        &genuine_declared_expiry_bytes,
        &padded_request_snapshot,
    );

    assert_eq!(
        requester
            .admit_decoded_process_message(
                codec
                    .decode_untrusted_message(&substituted)
                    .expect(
                        "the structurally valid substituted outcome remains only an untrusted codec candidate",
                    ),
            )
            .expect_err(
                "a declared expiry requires the exact checked owner-admission budget; a padded unbudgeted pending request cannot consume it",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "the source-inadmissible declared failure must leave the genuine padded request pending"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        0,
        "a padded unbudgeted request cannot acquire terminal expiry knowledge from an untrusted payload substitution"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        0,
        "rejection occurs before the original ordinary reply is admitted as a success receipt"
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_count(),
        0,
        "the rejected substituted payload creates no requester terminal occurrence"
    );
    assert!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(&padded_request_identity_ref)
            .is_none(),
        "the rejected substituted payload creates no success receipt occurrence either"
    );

    let receipt = requester
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&ordinary_reply_bytes)
                .expect("the retained ordinary reply remains only an untrusted codec candidate"),
        )
        .expect("the original source-generated success reply remains admissible after rejection")
        .expect("the original source-generated success reply yields the existing local receipt");
    assert_eq!(receipt.kind, Sys5I3ProcessMessageKind::Receipt);
    assert!(receipt.is_observer_safe_typed_result_or_receipt());
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        0,
        "only the later original success reply consumes the padded requester pending entry"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        0,
        "the unbudgeted source request never consumes a declared expiry terminal"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        1,
        "the original ordinary reply alone produces one successful receipt"
    );
}

#[test]
fn i3_owner_admission_budget_terminal_failure_capacity_preserves_the_next_genuine_requester_pending_entry()
 {
    let (mut requester, mut owner_s, mut owner_t) =
        source_started_terminal_failure_capacity_runtimes();
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut owner_s_resolution_tick: u64 = 0;

    for expected_terminal_count in 1..=MAX_OUTBOUND_OWNER_TERMINAL_FAILURES {
        owner_s_resolution_tick = owner_s_resolution_tick
            .checked_add(1)
            .expect("the bounded capacity sequence has one representable owner-local tick per staged S request");
        let (request_identity_ref, _generated_codec, bytes) =
            encoded_declared_deadline_expired_reply_for_operation(
                &mut requester,
                &mut owner_s,
                "expire_s",
                owner_s_resolution_tick,
            );
        let terminal = requester
            .admit_decoded_process_message(
                codec
                    .decode_untrusted_message(&bytes)
                    .expect("each exact S expiry reply decodes as only an untrusted candidate"),
            )
            .expect("each exact S expiry reply validates before terminal capacity is full")
            .expect("each exact S expiry produces its explicit local terminal result");
        assert!(terminal.is_observer_safe_terminal_failure_consumed());
        assert!(
            requester
                .observer_safe_semantic_occurrences()
                .requester_terminal_declared_owner_failure_occurrence_ref(&request_identity_ref)
                .is_some(),
            "each validated S expiry retains the exact requester terminal occurrence"
        );
        assert_eq!(
            requester.observer_safe_pending_owner_request_count(),
            0,
            "each consumed S expiry frees only its corresponding requester pending record"
        );
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_declared_owner_failure_count(),
            expected_terminal_count,
            "the requester terminal ledger grows only from actual validated expiry replies"
        );
    }
    assert_eq!(
        owner_s.observer_safe_inbound_owner_request_tombstone_count(),
        MAX_INBOUND_OWNER_REQUEST_TOMBSTONES,
        "the first real owner retains exactly its bounded terminal admission history"
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_count(),
        MAX_OUTBOUND_OWNER_TERMINAL_FAILURES,
        "the requester terminal ledger reaches its exact fixed capacity without an owner ledger overflow"
    );

    let (next_request_identity_ref, _generated_codec, next_bytes) =
        encoded_declared_deadline_expired_reply_for_operation(
            &mut requester,
            &mut owner_t,
            "expire_t",
            1,
        );
    assert_eq!(
        owner_t
            .observer_safe_owner_admission_summary()
            .expired_count(),
        1,
        "the 65th real staged request reaches its independent T-owner deadline before requester terminal-capacity rejection"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "the next genuine T request is still pending before the full requester terminal ledger admits it"
    );
    let requester_trace_before_capacity_refusal = requester.fabric.trace().clone();
    let requester_semantic_store_before_capacity_refusal = requester.fabric.semantic_snapshot();
    let requester_causality_before_capacity_refusal = requester.fabric.causality().clone();
    let requester_participant_before_capacity_refusal = requester
        .fabric
        .locus_runtime("A")
        .expect("the checked requester locus remains present in its local fabric");
    let requester_incoming_endpoint_before_capacity_refusal =
        requester_participant_before_capacity_refusal
            .incoming_endpoint()
            .clone();
    let requester_incoming_mailbox_before_capacity_refusal =
        requester_participant_before_capacity_refusal
            .incoming_mailbox()
            .clone();
    let requester_pending_before_capacity_refusal: std::collections::BTreeMap<_, _> = requester
        .pending_outbound_owner_requests
        .iter()
        .map(|(identity, record)| {
            (
                identity.clone(),
                (record.pending_token_ref.clone(), record.started_attempts),
            )
        })
        .collect();
    let requester_terminal_before_capacity_refusal: std::collections::BTreeMap<_, _> = requester
        .requester_terminal_declared_owner_failures
        .iter()
        .map(|(identity, record)| {
            (
                identity.clone(),
                (
                    record.decision_commitment_ref.clone(),
                    record.decision_occurrence_ref.clone(),
                ),
            )
        })
        .collect();
    let requester_summary_before_capacity_refusal = requester.observer_safe_runtime_summary();
    let requester_occurrences_before_capacity_refusal =
        requester.observer_safe_semantic_occurrences();
    assert_eq!(
        requester
            .admit_decoded_process_message(
                codec
                    .decode_untrusted_message(&next_bytes)
                    .expect("the genuine T expiry reply decodes as only an untrusted candidate"),
            )
            .expect_err(
                "a full terminal ledger must reject rather than evict an earlier terminal outcome"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OutboundTerminalFailureLedgerExhausted
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "terminal-capacity rejection must preserve the exact next source requester pending entry"
    );
    assert!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_occurrence_ref(&next_request_identity_ref)
            .is_none(),
        "a capacity-rejected terminal must not create an untrusted partial terminal occurrence"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        MAX_OUTBOUND_OWNER_TERMINAL_FAILURES,
        "capacity rejection cannot alter the retained accepted-terminal count"
    );
    assert_eq!(
        requester.fabric.trace(),
        &requester_trace_before_capacity_refusal,
        "terminal-capacity refusal must occur before any requester SYS4 receive/dequeue trace mutation"
    );
    assert_eq!(
        requester.fabric.semantic_snapshot(),
        requester_semantic_store_before_capacity_refusal,
        "terminal-capacity refusal cannot mutate requester-local SYS4 state"
    );
    assert_eq!(
        requester.fabric.causality(),
        &requester_causality_before_capacity_refusal,
        "terminal-capacity refusal must not record a phantom requester SYS4 dequeue causality edge"
    );
    let requester_participant_after_capacity_refusal = requester
        .fabric
        .locus_runtime("A")
        .expect("the requester locus remains present after capacity refusal");
    assert_eq!(
        requester_participant_after_capacity_refusal.incoming_endpoint(),
        &requester_incoming_endpoint_before_capacity_refusal,
        "terminal-capacity refusal cannot record an inbound endpoint carrier before rejecting"
    );
    assert_eq!(
        requester_participant_after_capacity_refusal.incoming_mailbox(),
        &requester_incoming_mailbox_before_capacity_refusal,
        "terminal-capacity refusal cannot enqueue or dequeue the genuine T reply"
    );
    let requester_pending_after_capacity_refusal: std::collections::BTreeMap<_, _> = requester
        .pending_outbound_owner_requests
        .iter()
        .map(|(identity, record)| {
            (
                identity.clone(),
                (record.pending_token_ref.clone(), record.started_attempts),
            )
        })
        .collect();
    assert_eq!(
        requester_pending_after_capacity_refusal, requester_pending_before_capacity_refusal,
        "terminal-capacity refusal preserves the exact requester pending-record map"
    );
    let requester_terminal_after_capacity_refusal: std::collections::BTreeMap<_, _> = requester
        .requester_terminal_declared_owner_failures
        .iter()
        .map(|(identity, record)| {
            (
                identity.clone(),
                (
                    record.decision_commitment_ref.clone(),
                    record.decision_occurrence_ref.clone(),
                ),
            )
        })
        .collect();
    assert_eq!(
        requester_terminal_after_capacity_refusal, requester_terminal_before_capacity_refusal,
        "terminal-capacity refusal preserves every already accepted requester terminal record"
    );
    assert_eq!(
        requester.observer_safe_runtime_summary(),
        requester_summary_before_capacity_refusal,
        "terminal-capacity refusal cannot change requester aggregate receive or terminal counts"
    );
    assert_eq!(
        requester.observer_safe_semantic_occurrences(),
        requester_occurrences_before_capacity_refusal,
        "terminal-capacity refusal cannot append a requester terminal or receipt occurrence"
    );
    assert_eq!(
        owner_t
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "the genuine T expiry remains a declared failure, not an owner serve"
    );
    assert_eq!(
        owner_t
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the genuine T expiry remains a declared failure, not an owner write"
    );
}

#[test]
fn i3_owner_admission_budget_awaiting_request_does_not_block_an_independent_unbudgeted_deadline_padded_owner_action()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    assert_eq!(
        owner
            .observer_safe_owner_admission_summary()
            .awaiting_count(),
        1,
        "the budgeted source request remains held at the owner gate"
    );

    let independent_request = requester
        .emit_generated_owner_request("refresh_avatar_hp")
        .expect("a distinct unannotated source action emits through the same checked requester");
    let independent_reply = owner
        .accept_inbound(independent_request)
        .expect("the independent unbudgeted owner action reaches ordinary checked execution")
        .expect(
            "a DeadlineExpired failure name without a checked owner budget remains an ordinary successful owner action",
        );
    assert_eq!(independent_reply.kind, Sys5I3ProcessMessageKind::Reply);
    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 1);
    assert_eq!(summary.expired_count(), 0);
    assert_eq!(summary.rejected_before_serve_count(), 0);
    assert_eq!(summary.serve_reserved_count(), 0);
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        0,
        "a source declaration padded with DeadlineExpired but no owner_ticks budget cannot fabricate terminal failure consumption"
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_count(),
        0,
        "the padded ordinary action has no requester declared-failure occurrence"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "only the independently eligible source action produces an actual owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "only the independently eligible source action produces an actual owner write"
    );
}

#[test]
fn i3_owner_admission_budget_full_owner_ledger_retains_completed_and_awaiting_requests_without_eviction()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    for expected_count in 1..=MAX_INBOUND_OWNER_REQUEST_TOMBSTONES {
        let request = requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect("each bounded request is independently generated by the checked source");
        assert!(
            owner
                .accept_inbound(request)
                .expect("each distinct budgeted request stages before owner execution")
                .is_none()
        );
        assert_eq!(
            owner.observer_safe_inbound_owner_request_tombstone_count(),
            expected_count,
            "the owner retains each exact source-derived request instead of replacing an earlier admission"
        );
    }
    assert_eq!(
        owner
            .observer_safe_owner_admission_summary()
            .awaiting_count(),
        MAX_INBOUND_OWNER_REQUEST_TOMBSTONES,
        "the full ledger is comprised of actual staged requests, not synthetic capacity entries"
    );

    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the checked owner clock deterministically selects one genuine held request")
        .expect("the full ledger has at least one Awaiting request");
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the first retained request resolves at tick zero"),
        "the first retained request receives one exact reservation",
    );
    let reply = owner
        .handoff_reserved_owner_admission(reserved)
        .expect("the selected real request serves before the ledger capacity is tested");
    assert!(
        requester
            .accept_inbound(reply)
            .expect("the requester admits the exact owner reply")
            .is_some(),
        "one actual reply receipt frees only its corresponding requester-pending record"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        MAX_INBOUND_OWNER_REQUEST_TOMBSTONES - 1,
        "the completed request frees one requester slot while the owner keeps its duplicate tombstone"
    );
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        MAX_INBOUND_OWNER_REQUEST_TOMBSTONES,
        "owner completion retains the bounded terminal record rather than freeing a capacity slot"
    );
    assert_eq!(
        owner
            .observer_safe_owner_admission_summary()
            .awaiting_count(),
        MAX_INBOUND_OWNER_REQUEST_TOMBSTONES - 1,
        "the other real source requests remain Awaiting after exactly one handoff"
    );

    let next_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the requester can generate one next source request after its completed receipt freed a pending slot");
    let capacity_error = match owner.accept_inbound(next_request) {
        Ok(_) => {
            panic!(
                "the owner must reject a new identity once every retained ledger slot is occupied"
            )
        }
        Err(error) => error,
    };
    assert_eq!(
        capacity_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::InboundRequestLedgerExhausted
    );
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        MAX_INBOUND_OWNER_REQUEST_TOMBSTONES,
        "capacity rejection cannot evict the completed record or any still-Awaiting source request"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "capacity rejection cannot manufacture a second owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "capacity rejection cannot manufacture a second owner write"
    );
}

#[test]
fn i3_owner_admission_budget_equal_backward_and_foreign_clock_controls_leave_the_awaiting_record_unchanged()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    let owner_clock = only_owner_clock(&owner);
    owner
        .advance_owner_admission_clock(&owner_clock, 0)
        .expect("an equal owner tick is an accepted no-op");
    let after_equal = owner.observer_safe_owner_admission_summary();
    assert_eq!(after_equal.awaiting_count(), 1);
    assert_eq!(after_equal.expired_count(), 0);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0
    );

    owner
        .advance_owner_admission_clock(&owner_clock, 1)
        .expect("the owner clock advances monotonically without resolving a request");
    assert_eq!(
        owner
            .advance_owner_admission_clock(&owner_clock, 0)
            .expect_err("a backward owner tick must fail closed")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected
    );
    let after_backward = owner.observer_safe_owner_admission_summary();
    assert_eq!(after_backward.awaiting_count(), 1);
    assert_eq!(after_backward.expired_count(), 0);

    let (mut foreign_requester, mut foreign_owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut foreign_requester, &mut foreign_owner);
    let foreign_clock = only_owner_clock(&foreign_owner);
    assert_eq!(
        owner
            .advance_owner_admission_clock(&foreign_clock, 2)
            .expect_err("a clock handle from a distinct owner runtime cannot control this runtime")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected
    );
    assert_eq!(
        owner.observer_safe_owner_admission_summary(),
        after_backward,
        "foreign clock rejection leaves the genuine awaiting record unchanged"
    );
    assert_eq!(
        foreign_owner
            .observer_safe_owner_admission_summary()
            .awaiting_count(),
        1,
        "foreign-handle rejection also leaves the other genuine runtime untouched"
    );

    let awaiting_from_owner = owner
        .take_next_owner_admission_awaiting(&owner_clock)
        .expect("the genuine owner still offers its retained Awaiting record")
        .expect("one retained Awaiting record remains available for the foreign-handle falsifier");
    assert_eq!(
        foreign_owner
            .resolve_staged_owner_admission(awaiting_from_owner)
            .err()
            .expect("a distinct runtime cannot resolve another runtime's opaque Awaiting handle")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionResolutionRejected
    );
    assert_eq!(
        owner.observer_safe_owner_admission_summary(),
        after_backward,
        "foreign Awaiting rejection must retain the genuine owner ledger entry"
    );
    let reacquired = owner
        .take_next_owner_admission_awaiting(&owner_clock)
        .expect("a failed foreign resolution cannot consume the genuine owner issuance")
        .expect("the genuine owner reacquires its exact retained Awaiting record");
    let _declared_expiry = expect_declared_deadline_expired(
        owner
            .resolve_staged_owner_admission(reacquired)
            .expect("the genuine owner can resolve its reacquired Awaiting record"),
        "the genuine owner resolves at its current deadline tick to retained expiry",
    );
    assert_eq!(
        owner
            .observer_safe_owner_admission_summary()
            .expired_count(),
        1,
        "the foreign-handle path must not prevent genuine owner resolution"
    );
}

#[test]
fn i3_owner_admission_budget_checked_owner_clocks_expire_and_serve_independently() {
    let (mut requester, mut owner) = source_started_two_owner_runtime();
    for operation in [FIRST_OPERATION, SECOND_OPERATION] {
        let request = requester
            .emit_generated_owner_request(operation)
            .unwrap_or_else(|_| {
                panic!("the checked {operation} source request emits without caller timing input")
            });
        assert!(
            owner
                .accept_inbound(request)
                .unwrap_or_else(|_| panic!(
                    "the exact {operation} source request stages at its own checked owner clock"
                ))
                .is_none()
        );
    }
    let mut clocks = owner.i3_trusted_owner_admission_clock_handles();
    assert_eq!(
        clocks.len(),
        2,
        "one runtime retains separate trusted owner-local clocks for the two checked owner conditions"
    );
    let s_position = clocks
        .iter()
        .position(|handle| handle.clock_key.owner_locus == "S")
        .expect("the S condition owns one private checked clock");
    let s_clock = clocks.swap_remove(s_position);
    let t_clock = clocks
        .pop()
        .expect("the remaining private clock belongs to the distinct T condition");
    assert_eq!(t_clock.clock_key.owner_locus, "T");

    owner
        .advance_owner_admission_clock(&s_clock, 1)
        .expect("advancing S to its deadline cannot advance T");
    let s_awaiting = owner
        .take_next_owner_admission_awaiting(&s_clock)
        .expect("the S clock selects only its own Awaiting request")
        .expect("the S request is Awaiting until its own clock resolves it");
    let _s_declared_expiry = expect_declared_deadline_expired(
        owner
            .resolve_staged_owner_admission(s_awaiting)
            .expect("S expiry is a retained local owner decision"),
        "S at tick one expires its budget-one request without selecting T",
    );
    let after_s_expiry = owner.observer_safe_owner_admission_summary();
    assert_eq!(after_s_expiry.awaiting_count(), 1);
    assert_eq!(after_s_expiry.expired_count(), 1);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "S expiry does not serve either owner operation"
    );

    let t_awaiting = owner
        .take_next_owner_admission_awaiting(&t_clock)
        .expect("T's independent tick-zero clock selects only the T request")
        .expect("T remains Awaiting despite S expiry");
    let t_reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(t_awaiting)
            .expect("T resolves at its untouched tick zero"),
        "T receives its exact one-use reservation independently of S",
    );
    owner
        .handoff_reserved_owner_admission(t_reserved)
        .expect("the independent T reservation completes through the genuine owner handoff");
    let after_t_handoff = owner.observer_safe_owner_admission_summary();
    assert_eq!(after_t_handoff.awaiting_count(), 0);
    assert_eq!(after_t_handoff.expired_count(), 1);
    assert_eq!(after_t_handoff.serve_reserved_count(), 0);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "only T's still-on-time operation is served"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "only T's still-on-time operation writes"
    );
}

#[test]
fn i3_owner_admission_budget_checked_clock_exists_before_staging_and_overflow_rejects_before_inbound_reservation()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let clock = only_owner_clock(&owner);
    owner
        .advance_owner_admission_clock(&clock, u64::MAX)
        .expect("the checked owner-local clock is available before any request stages");
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked source still derives its exact owner request at the maximum tick");
    let overflow_error = match owner.accept_inbound(request) {
        Ok(_) => panic!("start tick plus budget overflow must reject before staging"),
        Err(error) => error,
    };
    assert_eq!(
        overflow_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionDeadlineOverflow
    );
    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 0);
    assert_eq!(summary.expired_count(), 0);
    assert_eq!(summary.rejected_before_serve_count(), 0);
    assert_eq!(summary.serve_reserved_count(), 0);
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        0,
        "overflow must happen before an inbound duplicate/tombstone reservation is committed"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "overflow is preflight, not a serve decision"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "overflow is preflight, not an owner mutation"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "owner preflight overflow must not consume the source requester pending request"
    );
}

#[test]
fn i3_owner_admission_budget_failed_handoff_retains_the_consumed_reservation_without_owner_effects_or_reissue()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the staged source request yields its opaque Awaiting handle")
        .expect("the fresh staged request remains Awaiting at tick zero");
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the on-time Awaiting request resolves through the exact owner gate"),
        "the on-time request obtains exactly one opaque reservation",
    );
    assert_eq!(
        owner
            .observer_safe_owner_admission_summary()
            .serve_reserved_count(),
        1,
        "admission reservation precedes actual downstream owner evidence"
    );
    owner
        .test_only_reject_next_staged_owner_admission_handoff(&reserved)
        .expect("the borrowed genuine reservation arms exactly one real post-dequeue M8 rejection");
    assert!(
        owner
            .handoff_reserved_owner_admission(reserved)
            .err()
            .is_some(),
        "the armed real post-dequeue M8 rejection makes the one-use handoff fail"
    );
    let retained = owner.observer_safe_owner_admission_summary();
    assert_eq!(retained.awaiting_count(), 0);
    assert_eq!(retained.expired_count(), 0);
    assert_eq!(retained.rejected_before_serve_count(), 0);
    assert_eq!(retained.serve_reserved_count(), 1);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "failed handoff must not misreport its reservation as an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "failed handoff must not mutate owner state"
    );
    assert!(
        owner
            .take_next_owner_admission_awaiting(&clock)
            .expect("retained reservation lookup remains a valid trusted clock operation")
            .is_none(),
        "a consumed reserved permit cannot be reissued as a second Awaiting handle"
    );
}

#[test]
fn i3_owner_admission_budget_dropped_reserved_handoff_cannot_reissue_or_count_a_serve() {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the source-derived request yields its exact Awaiting handle")
        .expect("the fresh request remains Awaiting at tick zero");
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the checked G1 request resolves through the real owner gate"),
        "the on-time request issues one non-Clone reservation",
    );
    drop(reserved);

    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 0);
    assert_eq!(summary.expired_count(), 0);
    assert_eq!(summary.rejected_before_serve_count(), 0);
    assert_eq!(summary.serve_reserved_count(), 1);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "dropping the moved reservation cannot claim a downstream owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "dropping the moved reservation cannot mutate owner state"
    );
    assert!(
        owner
            .take_next_owner_admission_awaiting(&clock)
            .expect("trusted clock inspection remains available after the moved reservation drops")
            .is_none(),
        "dropping a Reserved handoff consumes its one issuance rather than allowing a second permit"
    );
}

#[test]
fn i3_owner_admission_budget_live_carrier_binding_mismatch_rejects_before_consuming_the_awaiting_issuance()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the trusted owner clock yields the exact staged request")
        .expect("the budgeted source request remains Awaiting at tick zero");
    let request_identity_ref = awaiting.request_identity_ref.clone();
    let different_source_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the same checked source can generate a distinct second owner carrier without minting authority");
    let different_source_carrier = different_source_request.carrier.expect(
        "the second genuine source request retains its exact carrier until owner admission",
    );
    let original_source_carrier = {
        let record = owner
            .inbound_owner_request_tombstones
            .get_mut(&request_identity_ref)
            .expect("the Awaiting request retains its exact live source-derived carrier");
        let live = record
            .live_admission
            .as_mut()
            .expect("Awaiting keeps its single unconsumed admission issuance and carrier");
        std::mem::replace(&mut live.carrier, different_source_carrier)
    };

    assert_eq!(
        owner
            .resolve_staged_owner_admission(awaiting)
            .err()
            .expect(
                "a different genuine carrier cannot satisfy the original pending request's exact live binding before issuance",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    {
        let record = owner
            .inbound_owner_request_tombstones
            .get(&request_identity_ref)
            .expect("binding rejection retains the original source request ledger record");
        assert_eq!(
            record.tombstone.phase,
            Sys5I3InboundOwnerRequestTombstonePhase::Awaiting,
            "a CarrierBindingMismatch cannot terminalize the original record before permit issuance"
        );
        assert!(
            record.live_admission.is_some(),
            "the failed immutable-binding check leaves the original sole issuance in its Awaiting record"
        );
        assert!(
            record.tombstone.terminal_admission.is_none(),
            "a pre-issuance binding rejection records no terminal serve disposition for the original request"
        );
    }
    assert_eq!(
        owner
            .observer_safe_owner_admission_summary()
            .awaiting_count(),
        1,
        "the observer-safe count agrees that the genuine mismatched carrier did not consume the original request"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the pure binding mismatch occurs before M8 owner mutation"
    );

    owner
        .inbound_owner_request_tombstones
        .get_mut(&request_identity_ref)
        .expect(
            "the retained Awaiting record remains available for exact source carrier restoration",
        )
        .live_admission
        .as_mut()
        .expect("the failed binding mismatch did not remove the original live admission")
        .carrier = original_source_carrier;
    let restored_awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("a failed pure binding check must not consume the opaque Awaiting handle")
        .expect(
            "the original request can be reacquired after its exact source carrier is restored",
        );
    let restored_reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(restored_awaiting)
            .expect(
                "restoring the original exact carrier allows its untouched issuance to resolve",
            ),
        "the exact source request receives the one reservation it retained",
    );
    owner
        .handoff_reserved_owner_admission(restored_reserved)
        .expect(
        "the original source-bound request can complete exactly once after the negative is removed",
    );
}

#[test]
fn i3_owner_admission_budget_two_owner_no_argument_fixture_serves_under_g1_before_g2_lifecycle_cases()
 {
    let (mut requester, mut owner, _owner_stimulus) =
        started_two_owner_lifecycle_pair(SECOND_OPERATION);
    let request = requester
        .emit_generated_owner_request(FIRST_OPERATION)
        .expect(
        "the source-derived no-argument attack_s carrier emits without an argument injection seam",
    );
    assert!(
        owner
            .accept_inbound(request)
            .expect("the budgeted no-argument attack_s request stages at G1")
            .is_none()
    );
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the S budget clock yields the source-derived attack_s request")
        .expect("attack_s remains Awaiting before its G1 resolution");
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the G1 source request resolves at tick zero"),
        "the no-argument G1 request receives one exact reservation",
    );
    owner
        .handoff_reserved_owner_admission(reserved)
        .expect("the no-argument fixture genuinely reaches the existing SYS4/M8 handoff under G1");
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "the fixture proves missing request arguments no longer mask the G2 eligibility assertion"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "the G1 no-argument control performs one actual owner write"
    );
}

#[test]
fn i3_owner_admission_budget_g2_revoking_a_distinct_owner_keeps_the_held_checked_request_eligible_once()
 {
    let (mut requester, mut owner, owner_stimulus) =
        started_two_owner_lifecycle_pair(SECOND_OPERATION);
    let request = requester
        .emit_generated_owner_request(FIRST_OPERATION)
        .expect("the checked source emits the budgeted S owner request while the G2 candidate still revokes only T");
    assert!(
        owner
            .accept_inbound(request)
            .expect("the exact attack_s request stages before any G2 lifecycle install")
            .is_none()
    );
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the S owner clock yields the retained source-derived attack_s request")
        .expect("the exact attack_s request remains Awaiting at tick zero");

    owner
        .install_admitted_owner_capability_successor(owner_stimulus)
        .expect("the genuine G1-to-G2 successor revokes only attack_t at T");
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("G2 must freshly revalidate and issue attack_s because its distinct S authority remains current"),
        "the preserved attack_s request receives one fresh-G2 reservation",
    );
    let _reply = owner
        .handoff_reserved_owner_admission(reserved)
        .expect("the fresh-G2 reservation reaches the existing actual SYS4/M8 handoff once");
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "only the retained attack_s request is actually served after G2"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "the preserved checked request performs one actual owner write after G2"
    );
    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 0);
    assert_eq!(summary.expired_count(), 0);
    assert_eq!(summary.rejected_before_serve_count(), 0);
    assert_eq!(summary.serve_reserved_count(), 0);
}

#[test]
fn i3_owner_admission_budget_g2_revoke_before_resolution_rejects_the_held_request_before_serve() {
    let (mut requester, mut owner, owner_stimulus) =
        started_two_owner_lifecycle_pair(FIRST_OPERATION);
    let request = requester
        .emit_generated_owner_request(FIRST_OPERATION)
        .expect(
        "the checked source emits the exact S request before the genuine selected G2 revocation",
    );
    assert!(
        owner
            .accept_inbound(request)
            .expect("the exact request stages while its G1 authority remains current")
            .is_none()
    );
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the S owner clock yields the retained G1 request")
        .expect("the request remains Awaiting before the selected G2 installs");
    owner
        .install_admitted_owner_capability_successor(owner_stimulus)
        .expect("the genuine G1-to-G2 successor revokes the exact held attack_s authority");

    assert_eq!(
        owner
            .resolve_staged_owner_admission(awaiting)
            .err()
            .expect(
                "the live G2 missing capability rejects the retained request before a permit issues"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingCapability
    );
    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 0);
    assert_eq!(summary.expired_count(), 0);
    assert_eq!(summary.rejected_before_serve_count(), 1);
    assert_eq!(summary.serve_reserved_count(), 0);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "the revoked held request must not become an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the revoked held request must not mutate owner state"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "the pre-serve authority rejection leaves requester knowledge pending until a later typed outcome exists"
    );
    assert!(
        owner
            .take_next_owner_admission_awaiting(&clock)
            .expect("the trusted clock can inspect the terminal ledger without reissuing")
            .is_none(),
        "RejectedBeforeServe cannot be reissued as a second Awaiting handle"
    );
}

#[test]
fn i3_owner_admission_budget_g1_reservation_cannot_handoff_after_a_genuine_g2_generation_change() {
    let (mut requester, mut owner, owner_stimulus) =
        started_two_owner_lifecycle_pair(SECOND_OPERATION);
    let request = requester
        .emit_generated_owner_request(FIRST_OPERATION)
        .expect("the checked source emits the budgeted S request before its G1 reservation");
    assert!(
        owner
            .accept_inbound(request)
            .expect("the exact request stages under genuine G1 authority")
            .is_none()
    );
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the trusted S clock yields the exact retained G1 request")
        .expect("the request is Awaiting before any lifecycle transition");
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the current G1 issues one exact reservation before later G2 installation"),
        "the on-time G1 resolution yields one opaque reservation",
    );
    owner
        .install_admitted_owner_capability_successor(owner_stimulus)
        .expect("the genuine G2 changes generation only by revoking distinct attack_t authority");

    let g2_handoff_error = match owner.handoff_reserved_owner_admission(reserved) {
        Ok(_) => panic!("a permit bound to G1 must not survive final live-G2 verification"),
        Err(error) => error,
    };
    assert_eq!(
        g2_handoff_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 0);
    assert_eq!(summary.expired_count(), 0);
    assert_eq!(summary.rejected_before_serve_count(), 0);
    assert_eq!(summary.serve_reserved_count(), 1);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "a post-permit G2 change must not report a serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "a post-permit G2 change must not mutate owner state"
    );
    assert!(
        owner
            .take_next_owner_admission_awaiting(&clock)
            .expect("the terminal reserved ledger remains observable through its trusted clock")
            .is_none(),
        "a G1 permit consumed at failed G2 handoff cannot reissue"
    );
}

#[test]
fn i3_owner_admission_budget_late_selected_revocation_cannot_rewrite_reserved_resolution_provenance()
 {
    let (mut requester, mut owner, owner_stimulus) =
        started_two_owner_lifecycle_pair(FIRST_OPERATION);
    let request = requester
        .emit_generated_owner_request(FIRST_OPERATION)
        .expect("the checked source emits the budgeted S request before its G1 reservation");
    assert!(
        owner
            .accept_inbound(request)
            .expect("the exact request stages under genuine G1 authority")
            .is_none()
    );
    let clock = only_owner_clock(&owner);
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the trusted S clock yields the exact retained G1 request")
        .expect("the request is Awaiting before its G1 reservation");
    let request_identity_ref = awaiting.request_identity_ref.clone();
    let reserved = expect_owner_admission_reserved(
        owner
            .resolve_staged_owner_admission(awaiting)
            .expect("the exact G1 request issues one reservation before later authority loss"),
        "the source request is on time at the initial owner tick",
    );
    let (reserved_resolution_tick, reserved_generation_ref) = {
        let terminal = owner
            .inbound_owner_request_tombstones
            .get(&request_identity_ref)
            .and_then(|record| record.tombstone.terminal_admission.as_ref())
            .expect(
                "the genuine G1 reservation retains its immutable initial resolution provenance",
            );
        (
            terminal.resolution_tick,
            terminal.resolution_generation_ref.clone(),
        )
    };
    owner.advance_owner_admission_clock(&clock, 1).expect(
        "a later owner-local clock tick is valid but cannot revise an already issued reservation",
    );
    owner
        .install_admitted_owner_capability_successor(owner_stimulus)
        .expect("the genuine G1-to-G2 successor revokes the selected held S authority");

    let selected_revocation_handoff_error = match owner.handoff_reserved_owner_admission(reserved) {
        Ok(_) => panic!(
            "the final live authority check rejects the old reservation before owner execution"
        ),
        Err(error) => error,
    };
    assert_eq!(
        selected_revocation_handoff_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingCapability
    );
    let terminal_after_revoke = owner
        .inbound_owner_request_tombstones
        .get(&request_identity_ref)
        .and_then(|record| record.tombstone.terminal_admission.as_ref())
        .expect("the consumed reservation retains its original terminal provenance after live authority rejection");
    assert_eq!(
        terminal_after_revoke.resolution_tick, reserved_resolution_tick,
        "a later clock tick and selected revocation cannot overwrite the original reservation resolution tick"
    );
    assert_eq!(
        terminal_after_revoke.resolution_generation_ref, reserved_generation_ref,
        "a later G2 authority failure cannot relabel the immutable G1 reservation provenance"
    );
    let summary = owner.observer_safe_owner_admission_summary();
    assert_eq!(summary.awaiting_count(), 0);
    assert_eq!(summary.serve_reserved_count(), 1);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "a selected post-reservation revocation cannot report a serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "a selected post-reservation revocation cannot mutate owner state"
    );
    assert!(
        owner
            .take_next_owner_admission_awaiting(&clock)
            .expect("the owner clock can inspect the retained terminal record")
            .is_none(),
        "a reservation consumed by final current-authority rejection cannot reissue"
    );
}

#[test]
fn i3_owner_admission_host_driver_at_current_tick_serves_one_actual_budgeted_request() {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    stage_budgeted_request(&mut requester, &mut owner);

    let mut drivers = owner.i3_admitted_owner_admission_host_drivers();
    assert_eq!(
        drivers.len(),
        1,
        "the one checked budgeted owner condition derives one opaque host driver"
    );
    let driver = drivers
        .pop()
        .expect("the source-derived owner host driver remains available to its admitted runtime");
    let reply = owner
        .drive_next_owner_admission(&driver, None)
        .expect(
            "the admitted host driver validates itself before resolving its next retained request",
        )
        .expect("the current owner tick resolves the exact budget-one request before its deadline");
    let receipt = requester
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&codec.encode_outbound_message(reply).expect(
                    "the real driver-produced success reply encodes through the private codec",
                ))
                .expect("the real driver-produced success reply decodes only as untrusted input"),
        )
        .expect("the original requester accepts its exact source-derived success reply")
        .expect("the original requester obtains the existing local receipt");
    assert!(receipt.is_observer_safe_typed_result_or_receipt());
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "a current-tick host drive reaches the existing one actual owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "a current-tick host drive reaches the existing actual owner write"
    );
    assert_eq!(
        owner
            .observer_safe_semantic_occurrences()
            .owner_serve_linearization_count(),
        1,
        "the host driver neither bypasses nor duplicates the existing M8/SYS4 serve path"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        0,
        "the delivered real success reply consumes only its original requester-pending request"
    );
}

#[test]
fn i3_owner_admission_host_driver_at_deadline_delivers_one_terminal_without_m8_serve() {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    stage_budgeted_request(&mut requester, &mut owner);

    let mut drivers = owner.i3_admitted_owner_admission_host_drivers();
    let driver = drivers
        .pop()
        .expect("the checked budgeted condition has one admitted host driver");
    assert!(
        drivers.is_empty(),
        "one checked owner/domain has no second host admission controller"
    );
    let declared_expiry = owner
        .drive_next_owner_admission(&driver, Some(1))
        .expect("the admitted driver may advance exactly to the source budget-one deadline")
        .expect("the staged request resolves to one real declared expiry reply at its deadline");
    let terminal = requester
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&codec.encode_outbound_message(declared_expiry).expect(
                    "the driver-produced declared expiry uses the existing private reply codec",
                ))
                .expect(
                    "the driver-produced declared expiry decodes only as untrusted requester input",
                ),
        )
        .expect("the exact declared expiry validates against its requester-pending request")
        .expect("the validated declared expiry produces the existing local terminal outcome");
    assert!(terminal.is_observer_safe_terminal_failure_consumed());
    let admission = owner.observer_safe_owner_admission_summary();
    assert_eq!(admission.awaiting_count(), 0);
    assert_eq!(admission.expired_count(), 1);
    assert_eq!(admission.serve_reserved_count(), 0);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "deadline expiry reaches no M8/SYS4 owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "deadline expiry reaches no authoritative owner write"
    );
    assert_eq!(
        owner
            .observer_safe_semantic_occurrences()
            .owner_serve_linearization_count(),
        0,
        "deadline expiry creates no M8-backed owner-serve linearization"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        1,
        "the actual requester accepts exactly one declared owner terminal"
    );
}

#[test]
fn i3_owner_admission_host_driver_rejects_foreign_and_backward_controls_without_mutation() {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    stage_budgeted_request(&mut requester, &mut owner);
    let mut drivers = owner.i3_admitted_owner_admission_host_drivers();
    let driver = drivers
        .pop()
        .expect("the local checked condition derives its one host driver");
    let (_foreign_requester, foreign_owner) = source_started_runtime_pair(1);
    let mut foreign_drivers = foreign_owner.i3_admitted_owner_admission_host_drivers();
    let foreign_driver = foreign_drivers
        .pop()
        .expect("a different source-started runtime has its own opaque host driver");

    let owner_snapshot = |runtime: &Sys5I3ProcessRuntime| {
        (
            runtime
                .owner_admission_clocks
                .iter()
                .map(|(key, state)| {
                    (
                        (key.owner_locus.clone(), key.clock_domain.clone()),
                        state.tick,
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>(),
            runtime
                .inbound_owner_request_tombstones
                .iter()
                .map(|(request_identity_ref, record)| {
                    (
                        request_identity_ref.clone(),
                        (
                            record.tombstone.carrier_snapshot_binding_bytes.clone(),
                            record.tombstone.phase,
                            record.tombstone.terminal_admission.is_some(),
                            record.live_admission.is_some(),
                        ),
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>(),
            runtime.observer_safe_owner_admission_summary(),
            runtime.observer_safe_runtime_summary(),
            runtime.observer_safe_semantic_occurrences(),
            runtime.fabric.trace().clone(),
            runtime.fabric.semantic_snapshot(),
            runtime.fabric.causality().clone(),
        )
    };

    let before_foreign = owner_snapshot(&owner);
    assert_eq!(
        owner
            .drive_next_owner_admission(&foreign_driver, None)
            .expect_err(
                "a host driver issued to a foreign runtime must fail before even an empty drive"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected
    );
    assert_eq!(
        owner_snapshot(&owner),
        before_foreign,
        "foreign host-driver rejection leaves exact clock, retained ledger, SYS4 state, and causality untouched"
    );

    let dropped_declared_expiry = owner
        .drive_next_owner_admission(&driver, Some(1))
        .expect("the genuine local driver advances to its exact checked deadline")
        .expect("the genuine local driver produces the retained declared expiry once");
    drop(dropped_declared_expiry);
    let before_backward = owner_snapshot(&owner);
    assert_eq!(
        owner
            .drive_next_owner_admission(&driver, Some(0))
            .expect_err(
                "a host driver must reject a backward owner-local tick before resolving again"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OwnerAdmissionClockRejected
    );
    assert_eq!(
        owner_snapshot(&owner),
        before_backward,
        "backward host-driver rejection leaves exact clock, expired ledger, SYS4 state, and causality untouched"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "dropped expiry and rejected host controls cannot consume the original requester pending request"
    );
}

#[test]
fn i3_owner_admission_host_driver_empty_and_settled_drives_issue_no_new_admission() {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let mut drivers = owner.i3_admitted_owner_admission_host_drivers();
    let driver = drivers.pop().expect(
        "the checked budgeted source derives its one host driver before any request stages",
    );
    let empty_trace = owner.fabric.trace().clone();
    let empty_causality = owner.fabric.causality().clone();
    assert!(
        owner
            .drive_next_owner_admission(&driver, None)
            .expect(
                "an admitted host driver validates before reporting an empty deterministic queue"
            )
            .is_none(),
        "an empty owner admission queue cannot manufacture a reply or admission issuance"
    );
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        0
    );
    assert_eq!(owner.fabric.trace(), &empty_trace);
    assert_eq!(owner.fabric.causality(), &empty_causality);

    stage_budgeted_request(&mut requester, &mut owner);
    let dropped_declared_expiry = owner
        .drive_next_owner_admission(&driver, Some(1))
        .expect("the genuine driver may resolve its one staged request at the checked deadline")
        .expect(
            "the deadline produces one declared expiry that this component test deliberately drops",
        );
    drop(dropped_declared_expiry);
    let admission_before_repeat = owner.observer_safe_owner_admission_summary();
    let clock_before_repeat: Vec<u64> = owner
        .owner_admission_clocks
        .values()
        .map(|state| state.tick)
        .collect();
    let ledger_before_repeat: std::collections::BTreeMap<_, _> = owner
        .inbound_owner_request_tombstones
        .iter()
        .map(|(request_identity_ref, record)| {
            (
                request_identity_ref.clone(),
                (record.tombstone.phase, record.live_admission.is_some()),
            )
        })
        .collect();
    assert!(
        owner
            .drive_next_owner_admission(&driver, None)
            .expect("a repeated genuine host drive remains valid after its prior request settled")
            .is_none(),
        "a settled request cannot receive a duplicate admission, terminal, reservation, or reply"
    );
    assert_eq!(
        owner.observer_safe_owner_admission_summary(),
        admission_before_repeat,
        "the repeated host drive cannot reissue the retained terminal admission"
    );
    assert_eq!(
        owner
            .owner_admission_clocks
            .values()
            .map(|state| state.tick)
            .collect::<Vec<_>>(),
        clock_before_repeat,
        "a None host tick preserves the owner-local clock after settlement"
    );
    assert_eq!(
        owner
            .inbound_owner_request_tombstones
            .iter()
            .map(|(request_identity_ref, record)| {
                (
                    request_identity_ref.clone(),
                    (record.tombstone.phase, record.live_admission.is_some()),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>(),
        ledger_before_repeat,
        "the repeated host drive retains the exact settled ledger record without a fresh issuance"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "dropping the one generated expiry and repeating an empty drive leaves requester state pending"
    );
}

#[test]
fn i3_unbudgeted_source_exposes_no_owner_admission_clock_or_host_driver() {
    let (_requester, owner) = source_started_runtime_pair_from_source(
        "tests/inline/i3_unbudgeted_owner_admission_host_driver.mir",
        owner_admission_budget_source(1).replacen(" within owner_ticks 1", "", 1),
    );
    assert!(
        owner.i3_trusted_owner_admission_clock_handles().is_empty(),
        "the legacy unbudgeted source derives no private owner-admission clock"
    );
    assert!(
        owner.i3_admitted_owner_admission_host_drivers().is_empty(),
        "the legacy unbudgeted source derives no host admission driver"
    );
}

#[test]
fn i3_owner_admission_expiry_decision_observer_matches_the_genuine_serialized_outcome_and_requester_terminal()
 {
    let (mut requester, mut owner) = source_started_runtime_pair(1);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked budget-one source emits one genuine requester pending request");
    let request_identity_ref = request.semantic_request_identity_ref().to_string();
    assert!(
        owner
            .accept_inbound(request)
            .expect("the genuine source request stages before the admitted driver resolves it")
            .is_none()
    );
    let mut drivers = owner.i3_admitted_owner_admission_host_drivers();
    let driver = drivers
        .pop()
        .expect("the checked budgeted condition derives its one admitted host driver");
    let declared_expiry = owner
        .drive_next_owner_admission(&driver, Some(1))
        .expect("the admitted driver reaches the exact checked budget-one deadline")
        .expect("the real SYS4 gate produces one declared expiry reply at the deadline");
    let serialized_reply = codec
        .encode_outbound_message(declared_expiry)
        .expect("the real gate-produced declared expiry uses the existing private reply codec");
    let serialized_outcome = strict_json_value(
        codec
            .unframe_body(
                &serialized_reply,
                Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES,
            )
            .expect("the real generated expiry has one bounded codec frame"),
    )
    .expect("the real generated expiry body is strict private JSON");
    let serialized_commitment_ref = snapshot_string(
        &serialized_outcome,
        "/message/carrier/payload/fields/outcome/fields/failure/decision_commitment_ref",
        "the real generated expiry retains its gate decision commitment",
    );
    let serialized_occurrence_ref = snapshot_string(
        &serialized_outcome,
        "/message/carrier/payload/fields/outcome/fields/failure/decision_occurrence_ref",
        "the real generated expiry retains its gate decision occurrence",
    );
    let decision = owner
        .observer_safe_owner_admission_expiry_decision(&request_identity_ref)
        .expect(
            "only the genuine retained Expired ledger entry exposes its observer-safe decision",
        );
    assert_eq!(
        decision.decision_commitment_ref(),
        serialized_commitment_ref
    );
    assert_eq!(
        decision.decision_occurrence_ref(),
        serialized_occurrence_ref
    );
    let canonical_digest = decision
        .decision_commitment_ref()
        .strip_prefix("sys5-i3-owner-admission-deadline-expired-sha256-v1:")
        .expect("the retained decision commitment keeps the canonical expiry commitment domain");
    assert_eq!(canonical_digest.len(), 64);
    assert!(
        canonical_digest
            .chars()
            .all(|character| character.is_ascii_hexdigit()),
        "the observer-safe commitment remains the canonical SHA-256 correlation rather than caller text"
    );
    assert_eq!(
        decision.decision_occurrence_ref(),
        format!(
            "sys4-i3-owner-declared-deadline-expired:{}",
            decision.decision_commitment_ref()
        ),
        "the observer-safe occurrence is canonically correlated to the same gate decision"
    );
    let admission = owner.observer_safe_owner_admission_summary();
    assert_eq!(admission.expired_count(), 1);
    assert_eq!(admission.serve_reserved_count(), 0);
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "expiry retains a decision but reaches no M8/SYS4 owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_semantic_occurrences()
            .owner_serve_linearization_count(),
        0,
        "expiry retains no M8-backed owner-serve linearization"
    );

    let terminal = requester
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&serialized_reply)
                .expect("the real serialized expiry decodes only as requester untrusted input"),
        )
        .expect("the exact generated expiry validates against the real requester pending request")
        .expect("the real requester retains one declared-failure terminal outcome");
    assert!(terminal.is_observer_safe_terminal_failure_consumed());
    assert_eq!(
        terminal.linked_request_identity_ref(),
        Some(request_identity_ref.as_str()),
        "the requester terminal remains linked to the exact source-generated request"
    );
    let requester_terminal = requester
        .requester_terminal_declared_owner_failures
        .get(&request_identity_ref)
        .expect("the requester retains its exact terminal decision after validated delivery");
    assert_eq!(
        requester_terminal.decision_commitment_ref,
        decision.decision_commitment_ref(),
        "requester terminal consumption retains the exact owner-side gate commitment"
    );
    assert_eq!(
        terminal.terminal_failure_decision_occurrence_ref.as_deref(),
        Some(requester_terminal.decision_occurrence_ref.as_str()),
        "the terminal result exposes the exact locally retained occurrence for the same validated expiry"
    );
}

#[test]
fn i3_owner_admission_expiry_decision_stays_absent_when_genuine_g2_revocation_rejects_before_deadline()
 {
    let (mut requester, mut owner, owner_stimulus) =
        started_two_owner_lifecycle_pair(FIRST_OPERATION);
    let request = requester
        .emit_generated_owner_request(FIRST_OPERATION)
        .expect(
            "the checked G1 source emits the genuine budgeted S request before its G2 revocation",
        );
    let request_identity_ref = request.semantic_request_identity_ref().to_string();
    assert!(
        owner
            .accept_inbound(request)
            .expect("the genuine G1 request stages before the admitted M9 successor arrives")
            .is_none()
    );
    let mut drivers = owner.i3_admitted_owner_admission_host_drivers();
    let driver = drivers
        .pop()
        .expect("the checked G1 budget condition derives one admitted host driver");
    owner
        .install_admitted_owner_capability_successor(owner_stimulus)
        .expect("the genuine admitted M9 G2 successor revokes the exact held S authority");
    assert!(
        owner
            .observer_safe_owner_admission_expiry_decision(&request_identity_ref)
            .is_none(),
        "an Awaiting request has no expiry decision before the host attempts resolution"
    );
    let semantic_before_rejection = owner.fabric.semantic_snapshot();
    let trace_before_rejection = owner.fabric.trace().clone();
    let causality_before_rejection = owner.fabric.causality().clone();
    let runtime_before_rejection = owner.observer_safe_runtime_summary();
    let occurrences_before_rejection = owner.observer_safe_semantic_occurrences();

    assert_eq!(
        owner
            .drive_next_owner_admission(&driver, Some(1))
            .expect_err(
                "the current G2 authority rejection wins before a deadline expiry decision can be produced"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingCapability
    );
    let admission = owner.observer_safe_owner_admission_summary();
    assert_eq!(admission.awaiting_count(), 0);
    assert_eq!(admission.expired_count(), 0);
    assert_eq!(admission.rejected_before_serve_count(), 1);
    assert_eq!(admission.serve_reserved_count(), 0);
    assert!(
        owner
            .observer_safe_owner_admission_expiry_decision(&request_identity_ref)
            .is_none(),
        "a typed current-authority rejection must not manufacture an expiry observer record"
    );
    assert_eq!(
        owner.observer_safe_runtime_summary(),
        runtime_before_rejection,
        "the authority rejection reaches no owner serve or authoritative write"
    );
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        occurrences_before_rejection,
        "the authority rejection creates no M8/SYS4 owner occurrence"
    );
    assert!(
        owner
            .fabric
            .semantic_snapshot()
            .same_state(&semantic_before_rejection),
        "the authority rejection cannot mutate owner SYS4 semantic state"
    );
    assert_eq!(
        owner.fabric.trace(),
        &trace_before_rejection,
        "the authority rejection cannot append a SYS4 receive or dequeue trace"
    );
    assert_eq!(
        owner.fabric.causality(),
        &causality_before_rejection,
        "the authority rejection cannot append a causality graph occurrence"
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        1,
        "without an expiry decision or reply, the genuine requester remains pending"
    );
}
