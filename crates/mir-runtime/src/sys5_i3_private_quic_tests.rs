//! Component-only regressions for the production retained-ingress reconnect
//! guard.  They exercise neither a Quinn connection nor a pending frame: the
//! real adapter owns those inputs and calls this exact predicate immediately
//! before returning `LocalAttemptRejected`, before decoded semantic admission.

use super::*;

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
