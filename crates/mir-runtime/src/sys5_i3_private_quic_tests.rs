//! Component-only regressions for production private-QUIC adapter guards.
//! They exercise neither a Quinn connection nor a pending frame: the real
//! adapter owns those inputs, including the original-reply classifier covered
//! below after genuine runtime admission has produced its local result.

use super::*;
use crate::sys5_i3_process_runtime::{
    Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3OwnerAdmissionClockHandle,
    Sys5I3OwnerAdmissionResolution, Sys5I3PrivateProcessCodecErrorKind, Sys5I3ProcessCohort,
};
use crate::sys5_local_slice::{Sys5SourceInput, build_project};

const OWNER_ADMISSION_SOURCE_PATH: &str =
    "tests/inline/i3_private_quic_terminal_failure_consumption.mir";
const OWNER_ADMISSION_REQUESTER_SLOT: &str = "private-quic-requester";
const OWNER_ADMISSION_OWNER_SLOT: &str = "private-quic-owner";

fn source_started_budget_one_owner_admission_runtimes()
-> (Sys5I3ProcessRuntime, Sys5I3ProcessRuntime) {
    let project = build_project(Sys5SourceInput::inline(
        OWNER_ADMISSION_SOURCE_PATH,
        "module Mirrorea.Sys5.I3PrivateQuicTerminalFailure

locus WorldAuthority
locus ParticipantA
principal self
type Player

state avatar[id: Player] at WorldAuthority {
  hp: Int
  visible observer_safe fields (hp)
}

Role[self] at ParticipantA {
  when init_avatar_hp() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable, DeadlineExpired) within owner_ticks 1 {
    at WorldAuthority {
      avatar[self].hp = 21
    }
  }
}

with auth MembershipAuth

verify finite_refinement
",
    ))
    .expect("the budget-one source checks before genuine requester and owner images start");
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(
                OWNER_ADMISSION_REQUESTER_SLOT,
                "127.0.0.1:41401",
                ["ParticipantA"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_ADMISSION_OWNER_SLOT,
                "127.0.0.1:41402",
                ["WorldAuthority"],
            ),
        ],
    )
    .expect("the source-derived requester and owner loci map to the exact two checked slots");
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("the checked project derives one genuine requester/owner cohort");
    let requester = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(OWNER_ADMISSION_REQUESTER_SLOT)
            .expect("the checked requester image is consumed exactly once"),
    )
    .expect("the source-derived requester runtime starts");
    let owner = Sys5I3ProcessRuntime::start(
        cohort
            .take_process_image(OWNER_ADMISSION_OWNER_SLOT)
            .expect("the checked owner image is consumed exactly once"),
    )
    .expect("the source-derived owner runtime starts");
    (requester, owner)
}

fn only_owner_admission_clock(owner: &Sys5I3ProcessRuntime) -> Sys5I3OwnerAdmissionClockHandle {
    let mut clocks = owner.i3_trusted_owner_admission_clock_handles();
    assert_eq!(
        clocks.len(),
        1,
        "one checked owner-admission condition exposes one owner-local trusted clock handle"
    );
    clocks
        .pop()
        .expect("the exact source condition retains its sole owner-local clock handle")
}

fn exact_control_preface() -> Sys5I3LocalnetPeerPreface {
    Sys5I3LocalnetPeerPreface::test_only_private_quic_preface_fixture()
}

#[test]
fn retained_ingress_reconnect_binding_requires_the_exact_control_and_one_to_two_generation() {
    let retained_control = exact_control_preface();
    assert!(retained_ingress_matches_reconnect_binding(
        1,
        &retained_control,
        2,
        &retained_control,
    ));

    let wrong_run = retained_control.clone().test_only_with_different_run_ref();
    assert!(
        !retained_ingress_matches_reconnect_binding(1, &retained_control, 2, &wrong_run),
        "a separately verified control for another run cannot consume this retained ingress"
    );

    let wrong_control = retained_control
        .clone()
        .test_only_with_different_image_integrity_ref();
    assert!(
        !retained_ingress_matches_reconnect_binding(1, &retained_control, 2, &wrong_control),
        "a different independently verified control binding cannot consume this retained ingress"
    );

    assert!(
        !retained_ingress_matches_reconnect_binding(2, &retained_control, 3, &retained_control),
        "only a retained generation-one ingress may reach the one consuming generation-two guard"
    );
    assert!(
        !retained_ingress_matches_reconnect_binding(1, &retained_control, 1, &retained_control),
        "the original session may not consume its own retained ingress"
    );
}

fn assert_local_attempt_rejected(result: Result<(), Sys5I3PrivateQuicError>, context: &str) {
    match result {
        Err(Sys5I3PrivateQuicError::LocalAttemptRejected) => {}
        Ok(()) => panic!("{context} must reject locally before any I/O or runtime handoff"),
        Err(_) => panic!("{context} must remain a local one-shot rejection"),
    }
}

#[test]
fn retained_ingress_acquisition_permit_is_one_shot_and_consumes_before_runtime_handoff() {
    // Component-only state-machine contract.  These fixed occurrence strings
    // are neither frames nor authority/candidate fixtures: production derives
    // the corresponding reference only after it has reserved one real
    // session-one receive occurrence.
    let occurrence = "i3-private-quic-test:receive:one";
    let wrong_occurrence = "i3-private-quic-test:receive:other";

    let mut interrupted = Sys5I3PrivateQuicPendingIngressPermit::Unacquired;
    interrupted
        .reserve_acquisition()
        .expect("the first verified generation-one acquisition reserves before awaiting I/O");
    assert_local_attempt_rejected(
        interrupted.reserve_acquisition(),
        "a cancelled, failed-read, malformed-frame, or codec-failed acquisition",
    );
    interrupted.complete_acquisition(occurrence).expect(
        "the reserved state remains fail-closed rather than silently reopening acquisition",
    );

    let mut completed = Sys5I3PrivateQuicPendingIngressPermit::Unacquired;
    completed
        .reserve_acquisition()
        .expect("one acquisition begins from the sole unacquired state");
    completed
        .complete_acquisition(occurrence)
        .expect("only a nonempty occurrence completes one reserved acquisition");
    assert_local_attempt_rejected(
        completed.reserve_acquisition(),
        "a second acquisition after one complete retained ingress",
    );
    assert_local_attempt_rejected(
        completed.consume_completed(wrong_occurrence),
        "a reconnect with a different retained occurrence",
    );
    completed
        .consume_completed(occurrence)
        .expect("the exact retained occurrence is consumed immediately before runtime admission");
    assert_local_attempt_rejected(
        completed.reserve_acquisition(),
        "a second acquisition after the reconnect has consumed the retained ingress",
    );
    assert_local_attempt_rejected(
        completed.consume_completed(occurrence),
        "a second admission attempt after the runtime handoff has begun",
    );

    let mut empty_occurrence = Sys5I3PrivateQuicPendingIngressPermit::Unacquired;
    empty_occurrence
        .reserve_acquisition()
        .expect("the empty-reference negative starts from a real reserve state");
    assert_local_attempt_rejected(
        empty_occurrence.complete_acquisition(""),
        "an acquisition without its reserved receive occurrence reference",
    );
    assert_local_attempt_rejected(
        empty_occurrence.reserve_acquisition(),
        "a failed completion must retain the original acquisition reservation",
    );
}

/// C3 adapter RED: the real owner gate can deliver a requester-local terminal
/// consumption result, which must not fall through the legacy receipt-only
/// filter and be returned as `Pending(FrameRejected)` after its pending entry
/// has already been removed.
#[test]
fn original_owner_reply_classifier_keeps_a_genuine_declared_expiry_as_terminal_consumption() {
    let (mut requester, mut owner) = source_started_budget_one_owner_admission_runtimes();
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();

    let source_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked budget-one source emits its exact owner request");
    let request_identity_ref = source_request.semantic_request_identity_ref().to_string();
    let pending = requester
        .into_original_owner_request_pending(source_request)
        .expect("only the source-generated request becomes the opaque original pending handle");
    let initial_attempt = requester
        .authorize_original_owner_request_attempt(
            &pending,
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
        )
        .expect("the exact original pending handle authorizes one initial generated delivery");
    let request_bytes = initial_attempt.encoded_message_bytes().to_vec();
    requester
        .commit_authorized_original_owner_request_attempt(&initial_attempt)
        .expect(
            "the initial delivery attempt commits before the owner receives its shared-codec frame",
        );
    assert!(
        owner
            .admit_decoded_process_message(
                codec.decode_untrusted_message(&request_bytes).expect(
                    "the source-generated request bytes decode only as an untrusted candidate"
                ),
            )
            .expect("the genuine request stages at its checked owner-admission gate")
            .is_none(),
        "staging the genuine request does not produce a success reply"
    );

    let clock = only_owner_admission_clock(&owner);
    owner
        .advance_owner_admission_clock(&clock, 1)
        .expect("only the owner-local trusted control advances to the exact budget-one deadline");
    let awaiting = owner
        .take_next_owner_admission_awaiting(&clock)
        .expect("the trusted owner clock yields the retained checked request")
        .expect("the one staged request remains Awaiting until serialized resolution");
    let declared_expiry_reply = match owner
        .resolve_staged_owner_admission(awaiting)
        .expect("expiry is a gate-produced declared outcome, not a raw runtime error")
    {
        Sys5I3OwnerAdmissionResolution::DeclaredOwnerFailure(reply) => *reply,
        Sys5I3OwnerAdmissionResolution::ServeReserved(_) => {
            panic!("the exact start-plus-one deadline must not reserve a serve")
        }
    };
    assert!(
        !declared_expiry_reply.has_no_transportable_carrier(),
        "the gate-produced declared failure remains an outbound candidate rather than a requester-local no-carrier terminal"
    );

    let declared_expiry_bytes = codec
        .encode_outbound_message(declared_expiry_reply)
        .expect("the generated declared expiry reply uses the shared private codec");
    let local_terminal = requester
        .admit_decoded_original_owner_reply(
            codec
                .decode_untrusted_message(&declared_expiry_bytes)
                .expect(
                    "the shared-codec expiry bytes are still untrusted before requester validation",
                ),
            &pending,
        )
        .expect("the exact gate-produced expiry validates for its original pending handle")
        .expect("the valid expiry produces one explicit requester-local terminal result");

    let classified = classify_admitted_original_owner_reply(Some(local_terminal)).expect(
        "the adapter must classify an actual terminal consumption before filtering replies",
    );
    let terminal = match classified {
        Sys5I3PrivateQuicAdmittedOriginalOwnerReply::TerminalFailureConsumed(terminal) => terminal,
        Sys5I3PrivateQuicAdmittedOriginalOwnerReply::Receipt(_) => {
            panic!("a declared expiry terminal must not be relabeled as a successful receipt")
        }
    };
    assert!(terminal.is_observer_safe_terminal_failure_consumed());
    assert!(!terminal.is_observer_safe_typed_result_or_receipt());
    assert!(terminal.has_no_transportable_carrier());
    assert_eq!(
        terminal.semantic_request_identity_ref(),
        request_identity_ref
    );
    assert_eq!(
        codec
            .encode_outbound_message(terminal)
            .expect_err("the requester-local terminal must not re-enter the transport codec")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::TerminalFailureIsLocalOnly
    );

    assert_eq!(requester.observer_safe_pending_owner_request_count(), 0);
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_declared_owner_failure_count(),
        1,
        "the terminal count comes from the actual validated requester transition"
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_terminal_declared_owner_failure_count(),
        1,
        "one accepted expiry retains one terminal decision occurrence before pending removal"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "the expiry branch does not perform an owner serve"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the expiry branch does not mutate owner state"
    );
}
