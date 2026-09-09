//! LOCAL RED contracts for the private I3-3 process-local-cut child custody
//! seam.  This nested module alone may consume the launch's already-issued
//! opaque bootstrap frames.  It is not an FD3/exec/QUIC claim; the separate
//! probe integration test owns actual two-process evidence.

use crate::{
    sys5_i3_process_runtime::{
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3OriginalOwnerRequestAttemptKind,
        Sys5I3ProcessCohort, Sys5I3ProcessRuntimeErrorKind,
    },
    sys5_local_slice::{Sys5LocalProject, Sys5SourceInput, build_project},
};

use super::*;

const CANONICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const CANONICAL_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const BUDGETED_RESERVATION_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-owner-admission/main.mir";
const BUDGETED_RESERVATION_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-owner-admission/main.mir");
const REQUESTER_SLOT: &str = "process-a";
const OWNER_SLOT: &str = "process-b";

fn canonical_project() -> Sys5LocalProject {
    build_project(Sys5SourceInput::inline(
        CANONICAL_SOURCE_PATH,
        CANONICAL_SOURCE,
    ))
    .expect("the accepted ordinary source remains checkable before local-cut custody")
}

fn budgeted_reservation_project() -> Sys5LocalProject {
    build_project(Sys5SourceInput::inline(
        BUDGETED_RESERVATION_SOURCE_PATH,
        BUDGETED_RESERVATION_SOURCE,
    ))
    .expect(
        "the existing checked owner-admission sample remains available for the real Awaiting reservation path",
    )
}

fn canonical_deployment(project: &Sys5LocalProject) -> Sys5I3Deployment {
    Sys5I3Deployment::from_checked_project(
        project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41311",
                ["ParticipantA", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41312",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect("the accepted four loci map exactly once to the two ordinary slots")
}

fn genuine_local_cut_launch_from_project(
    project: &Sys5LocalProject,
    run_ref: &str,
) -> (Sys5I3ProcessLocalCutLaunch, Sys5I3PrivateProcessCodec) {
    let deployment = canonical_deployment(project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(project, &deployment)
        .expect("the genuine source-derived cohort performs one ordinary admitted construction");
    let launch = cohort
        .take_i3_process_local_cut_launch(
            &codec,
            run_ref,
            &format!("requester-spki:{run_ref}"),
            &format!("owner-spki:{run_ref}"),
        )
        .expect("only the fresh complete cohort issues one private local-cut bootstrap pair");
    (launch, codec)
}

fn genuine_fresh_local_cut_launch() -> (Sys5I3ProcessLocalCutLaunch, Sys5I3PrivateProcessCodec) {
    let project = canonical_project();
    genuine_local_cut_launch_from_project(&project, "i3-3-local-cut-issued-bootstrap-pair")
}

fn genuine_budgeted_reservation_local_cut_launch()
-> (Sys5I3ProcessLocalCutLaunch, Sys5I3PrivateProcessCodec) {
    let project = budgeted_reservation_project();
    genuine_local_cut_launch_from_project(&project, "i3-3-local-cut-budgeted-reservation")
}

/// Drive the existing source-generated request through the ordinary private
/// codec and B's admitted host driver until B retains a real `Awaiting`
/// reservation. This remains LOCAL custody evidence: it neither creates a
/// QUIC session nor claims FD3/child execution.
fn genuine_local_pair_with_awaiting_owner_reservation()
-> (Sys5I3ProcessLocalCutTestPair, Sys5I3PrivateProcessCodec) {
    let (mut launch, codec) = genuine_budgeted_reservation_local_cut_launch();
    let mut pair = launch
        .test_only_start_local_capsule_pair_from_issued_bootstraps(&codec)
        .expect("the genuine issued bootstrap pair starts only the private local capsule pair");

    let first_request = pair
        .requester
        .runtime
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked requester emits its actual initial source action");
    let pending = pair
        .requester
        .runtime
        .into_original_owner_request_pending(first_request)
        .expect("only the genuine initial source action yields the retained requester pending");
    let first_delivery = pair
        .requester
        .runtime
        .authorize_original_owner_request_attempt(
            &pending,
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
        )
        .expect("the genuine requester pending authorizes its one initial local delivery");
    let request_bytes = first_delivery.encoded_message_bytes().to_vec();
    pair.requester
        .runtime
        .commit_authorized_original_owner_request_attempt(&first_delivery)
        .expect("the genuine initial delivery records one started source attempt");
    pair.requester.initial_pending = Some(pending);

    let staged = pair
        .owner
        .runtime
        .admit_decoded_process_message(
            codec
                .decode_untrusted_message(&request_bytes)
                .expect("the retained initial request bytes decode only as an untrusted candidate"),
        )
        .expect("the genuine owner admits the exact source-derived initial request");
    assert!(
        staged.is_none(),
        "the existing budgeted initial request remains in the genuine owner Awaiting state"
    );
    assert!(
        pair.owner.runtime.has_i3_owner_admission_reservation(),
        "the admitted Awaiting state retains an actual owner admission reservation"
    );

    (pair, codec)
}

/// Drive the ordinary source's genuine initial request/reply path through the
/// existing codec and local owner admission. This deliberately makes no
/// transport, child-install, or QUIC claim.
fn genuine_local_pair_after_first_receipt() -> Sys5I3ProcessLocalCutTestPair {
    let (mut launch, codec) = genuine_fresh_local_cut_launch();
    let mut pair = launch
        .test_only_start_local_capsule_pair_from_issued_bootstraps(&codec)
        .expect("the genuine issued bootstrap pair starts only the private local capsule pair");
    let first_request = pair
        .requester
        .runtime
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the ordinary checked requester emits its actual initial source action");
    let pending = pair
        .requester
        .runtime
        .into_original_owner_request_pending(first_request)
        .expect("only the genuine ordinary initial action yields the retained requester pending");
    let first_delivery = pair
        .requester
        .runtime
        .authorize_original_owner_request_attempt(
            &pending,
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
        )
        .expect("the genuine requester pending authorizes its one initial local delivery");
    let request_bytes = first_delivery.encoded_message_bytes().to_vec();
    pair.requester
        .runtime
        .commit_authorized_original_owner_request_attempt(&first_delivery)
        .expect("the ordinary initial delivery records one started source attempt");
    pair.requester.initial_pending = Some(pending);
    let reply =
        pair.owner
            .runtime
            .admit_decoded_process_message(codec.decode_untrusted_message(&request_bytes).expect(
                "the retained ordinary request bytes decode only as an untrusted candidate",
            ))
            .expect("the ordinary checked owner admits the exact initial source request")
            .expect("the ordinary unbudgeted initial request produces its actual reply directly");
    let reply_bytes = codec
        .encode_outbound_message(reply)
        .expect("the genuine owner reply uses the existing private codec");
    let pending = pair
        .requester
        .initial_pending
        .take()
        .expect("the exact genuine first pending remains retained until A consumes its reply");
    let receipt = pair
        .requester
        .runtime
        .admit_decoded_original_owner_reply(
            codec
                .decode_untrusted_message(&reply_bytes)
                .expect("the genuine owner reply remains untrusted until requester admission"),
            &pending,
        )
        .expect("the requester consumes only its exact retained original reply")
        .expect("the genuine first reply yields one requester-local receipt");
    assert!(
        receipt.is_observer_safe_typed_result_or_receipt(),
        "the local sequence obtains the existing typed requester receipt before cut admission"
    );
    pair
}

#[test]
fn i3_3_requester_runtime_rejects_cut_admission_before_the_genuine_initial_reply() {
    // This verifies the runtime's actual receipt prerequisite separately from
    // the capsule's session-custody guard. A sessionless capsule must not be
    // treated as a local-cut positive merely because this runtime predicate
    // exists.
    let (mut launch, codec) = genuine_fresh_local_cut_launch();
    let mut pair = launch
        .test_only_start_local_capsule_pair_from_issued_bootstraps(&codec)
        .expect("the genuine issued bootstrap pair starts only the private local capsule pair");
    let rejection = match pair
        .requester
        .runtime
        .commit_i3_process_local_cut_after_initial_receipt()
    {
        Ok(_) => {
            panic!("bootstrap alone must not satisfy the runtime's first-receipt cut prerequisite")
        }
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
        "the runtime cannot mint a local-cut admission before actual first reply consumption"
    );
}

#[test]
fn i3_3_issued_owner_capsule_cannot_admit_requester_local_cut() {
    // The pair is genuine, but B has no authority to commit A's requester
    // local cut. This does not assert a global pair-cut protocol.
    let (mut launch, codec) = genuine_fresh_local_cut_launch();
    let mut pair = launch
        .test_only_start_local_capsule_pair_from_issued_bootstraps(&codec)
        .expect("the genuine issued bootstrap pair starts only the private local capsule pair");
    let rejection = match pair.owner_mut().admit_process_local_cut() {
        Ok(()) => panic!("B must not commit A's requester-local cut"),
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessLocalCutErrorKind::WrongRole,
        "the exact issued owner role rejects requester-local cut admission"
    );
}

#[test]
fn i3_3_local_owner_awaiting_reservation_rejects_cut_before_wrong_role() {
    let (mut pair, _codec) = genuine_local_pair_with_awaiting_owner_reservation();
    let before = pair.owner.runtime.observer_safe_runtime_summary();
    let rejection = match pair.owner.admit_process_local_cut() {
        Ok(()) => panic!("a real Awaiting owner reservation must deny a local cut"),
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessLocalCutErrorKind::OwnerAdmissionReservation,
        "the real retained owner reservation takes precedence over B's wrong role"
    );
    assert_eq!(
        pair.owner.runtime.observer_safe_runtime_summary(),
        before,
        "the local cut rejection cannot serve or write through the retained reservation"
    );
    assert_eq!(
        before.actual_owner_write_count(),
        0,
        "the still-Awaiting reservation has no owner write before the host driver runs"
    );
}

#[test]
fn i3_3_local_cut_admission_token_rejects_a_second_live_requester_without_mutation() {
    let mut pair = genuine_local_pair_after_first_receipt();
    let (mut second_launch, second_codec) = genuine_fresh_local_cut_launch();
    let mut second_pair = second_launch
        .test_only_start_local_capsule_pair_from_issued_bootstraps(&second_codec)
        .expect("a separate genuine launch starts a distinct live requester runtime");
    let before = second_pair
        .requester
        .runtime
        .observer_safe_runtime_summary();
    let pending_before = second_pair
        .requester
        .runtime
        .observer_safe_pending_owner_request_count();
    let admission = pair
        .requester
        .runtime
        .commit_i3_process_local_cut_after_initial_receipt()
        .expect("the genuine first local receipt creates one runtime-bound local-cut admission");

    let rejection = match second_pair
        .requester
        .runtime
        .emit_generated_owner_request_after_process_local_cut("init_avatar_hp", &admission)
    {
        Ok(_) => panic!(
            "a cut admission from another live requester runtime must not emit a source action"
        ),
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
        "the genuine token is bound to its original live requester runtime"
    );
    assert_eq!(
        second_pair
            .requester
            .runtime
            .observer_safe_runtime_summary(),
        before,
        "a foreign cut admission cannot mutate the second requester runtime"
    );
    assert_eq!(
        second_pair
            .requester
            .runtime
            .observer_safe_pending_owner_request_count(),
        pending_before,
        "a foreign cut admission cannot create an outbound pending request"
    );
}

#[test]
fn i3_3_genuine_post_cut_followup_retains_the_receipt_admission_enqueue_causality() {
    // This is LOCAL causal evidence only: the ordinary checked source drives
    // both requests through the live runtime, without claiming a QUIC or
    // inherited-child execution path.
    let mut pair = genuine_local_pair_after_first_receipt();
    let admission = pair
        .requester
        .runtime
        .commit_i3_process_local_cut_after_initial_receipt()
        .expect("the genuine first requester receipt creates one live local-cut admission");
    let followup = pair
        .requester
        .runtime
        .emit_generated_owner_request_after_process_local_cut("init_avatar_hp", &admission)
        .expect("the live admission permits the genuine later checked source action");

    assert!(
        pair.requester
            .runtime
            .test_only_i3_process_local_cut_followup_causally_reaches(&admission, &followup),
        "the retained graph must bind the genuine first receipt to cut admission and that admission to the later source enqueue"
    );
}

#[test]
fn i3_3_local_completed_round_trip_without_a_live_session_rejects_cut() {
    let mut pair = genuine_local_pair_after_first_receipt();
    let before = pair.requester.runtime.observer_safe_runtime_summary();
    let rejection = match pair.requester.admit_process_local_cut() {
        Ok(()) => {
            panic!("a receipt-complete capsule without a live session must not commit a local cut")
        }
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessLocalCutErrorKind::SessionUnavailable,
        "a completed local receipt does not substitute for the capsule's live session custody"
    );
    assert_eq!(
        pair.requester.runtime.observer_safe_runtime_summary(),
        before,
        "a sessionless cut rejection cannot create a local admission effect"
    );
}
