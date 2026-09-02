//! RED contract for the private I3-2 actual-process QUIC slice.
//!
//! This target intentionally specifies a `#[doc(hidden)]`, test-facing API
//! rather than a public process, wire, certificate, or deployment ABI.  The
//! supervisor gets only an ordinary-source path and a deadline: it must build
//! and admit the source once, and it must not receive a precomputed semantic
//! result.  Each child is started from its own *tainted* image descriptor plus
//! a separately retained, one-shot trusted-control descriptor.  The public
//! test observations below are observer-safe summaries, never raw image,
//! capability, witness, certificate, private-key, or source-text material.

#![allow(unused_crate_dependencies)]

use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use mir_runtime::sys5_local_slice::{Sys5I3AdapterCarrierContract, Sys5SourceInput, build_project};
use mirrorea_i3_probe::{
    I3LocalnetAdapterRejectionKind, I3LocalnetChildSlot, I3LocalnetChildTerminalOutcome,
    I3LocalnetControlDelivery, I3LocalnetDeliveryPhase, I3LocalnetFailureStage,
    I3LocalnetFalsifier, I3LocalnetImageDelivery, I3LocalnetLifecycleRejectionCause,
    I3LocalnetObserverSafeDeliveryRecord, I3LocalnetRejectionAudit, I3LocalnetRunErrorKind,
    I3ProcessLocalnetRequest, run_i3_process_localnet,
};

const ACTIVE_I2_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"
);
const ACTIVE_I2_LOGICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const ACTIVE_I2_SOURCE_TEXT: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

fn canonical_request() -> I3ProcessLocalnetRequest {
    // This is an ordinary source path, not a supervisor-fed expected outcome
    // or a pre-assembled process image.  The implementation must check/build
    // and admit it exactly once in the supervisor before process launch.
    I3ProcessLocalnetRequest::from_ordinary_source_path(ACTIVE_I2_SOURCE)
        .with_deadline(Duration::from_secs(20))
}

/// Builds a test-side observer expectation only.  It is never passed into the
/// process supervisor, therefore cannot provide a source-free route, carrier,
/// authority, or expected semantic result to the localnet run.
fn active_contract(kind: &str) -> Sys5I3AdapterCarrierContract {
    let project = build_project(Sys5SourceInput::inline(
        ACTIVE_I2_LOGICAL_SOURCE_PATH,
        ACTIVE_I2_SOURCE_TEXT,
    ))
    .expect("the accepted ordinary source remains checkable for observer expectations");
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.operation_id == "init_avatar_hp" && edge.kind == kind)
        .unwrap_or_else(|| panic!("accepted source must generate the {kind} edge"));
    project
        .i3_adapter_carrier_contract(&edge.edge_ref)
        .expect("a generated edge has the matching checked I3 adapter contract")
}

/// The trace must publish inventories from child-observed delivery records,
/// rather than supervisor-populated scalar counts.  A sorted distinct list is
/// intentionally sufficient for the private test seam and exposes no raw
/// source, image, carrier bytes, credentials, witnesses, or state payload.
fn expected_ref_inventory(values: &[&str]) -> Vec<String> {
    values
        .iter()
        .map(|value| (*value).to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn delivery_for_phase(
    records: &[I3LocalnetObserverSafeDeliveryRecord],
    phase: I3LocalnetDeliveryPhase,
) -> &I3LocalnetObserverSafeDeliveryRecord {
    let mut matching = records.iter().filter(|record| record.phase() == phase);
    let record = matching.next().unwrap_or_else(|| {
        panic!("the joined trace must contain an actual {phase:?} delivery record")
    });
    assert!(
        matching.next().is_none(),
        "the joined trace must not fabricate or collapse duplicate {phase:?} delivery records"
    );
    record
}

fn assert_delivery_matches_contract(
    record: &I3LocalnetObserverSafeDeliveryRecord,
    contract: &Sys5I3AdapterCarrierContract,
    semantic_request_identity: &str,
    linked_request_identity: Option<&str>,
) {
    assert_eq!(record.source_ref(), contract.source_ref());
    assert_eq!(record.core_ref(), contract.core_ref());
    assert_eq!(
        record.source_artifact_ref(),
        contract.source_artifact_ref(),
        "delivery lineage must identify its source artifact, not just a supervisor image count"
    );
    assert_eq!(
        record.target_artifact_ref(),
        contract.target_artifact_ref(),
        "delivery lineage must identify its target artifact, not just a supervisor image count"
    );
    assert_eq!(record.edge_ref(), contract.edge_ref());
    assert_eq!(
        record.semantic_request_identity_ref(),
        semantic_request_identity
    );
    assert_eq!(
        record.linked_request_identity_ref(),
        linked_request_identity
    );
    assert!(!record.carrier_ref().is_empty());
    assert!(
        !record.network_occurrence_ref().is_empty(),
        "every actual send/receive record must retain its own observer-safe network occurrence"
    );
}

/// Send and receive are distinct transport events, so their network occurrence
/// refs need not match.  Their semantic carrier and complete generated lineage
/// must match exactly; otherwise supervisor-side lineage could masquerade as
/// child delivery evidence.
fn assert_delivery_semantics_match(
    sent: &I3LocalnetObserverSafeDeliveryRecord,
    received: &I3LocalnetObserverSafeDeliveryRecord,
) {
    assert_eq!(sent.source_ref(), received.source_ref());
    assert_eq!(sent.core_ref(), received.core_ref());
    assert_eq!(sent.source_artifact_ref(), received.source_artifact_ref());
    assert_eq!(sent.target_artifact_ref(), received.target_artifact_ref());
    assert_eq!(sent.edge_ref(), received.edge_ref());
    assert_eq!(sent.carrier_ref(), received.carrier_ref());
    assert_eq!(
        sent.semantic_request_identity_ref(),
        received.semantic_request_identity_ref()
    );
    assert_eq!(
        sent.linked_request_identity_ref(),
        received.linked_request_identity_ref()
    );
}

#[test]
fn source_first_localnet_executes_one_remote_owner_round_trip_across_two_reaped_children() {
    let run = run_i3_process_localnet(canonical_request())
        .expect("the finite private I3-2 localnet positive path must complete");

    let process_a = run
        .child(I3LocalnetChildSlot::ProcessA)
        .expect("canonical deployment must launch process A");
    let process_b = run
        .child(I3LocalnetChildSlot::ProcessB)
        .expect("canonical deployment must launch process B");

    assert!(
        process_a.exec_confirmed(),
        "A must be an exec child, not an in-process task"
    );
    assert!(
        process_b.exec_confirmed(),
        "B must be an exec child, not an in-process task"
    );
    assert_ne!(
        process_a.pid(),
        process_b.pid(),
        "the two loci groups need distinct OS PIDs"
    );
    assert_ne!(
        process_a.pid(),
        std::process::id(),
        "the supervisor is not process A"
    );
    assert_ne!(
        process_b.pid(),
        std::process::id(),
        "the supervisor is not process B"
    );
    assert_eq!(
        process_a.assigned_loci(),
        ["ParticipantA", "ViewerC"],
        "the deployment map places only ParticipantA and ViewerC in process A"
    );
    assert_eq!(
        process_b.assigned_loci(),
        ["WorldAuthority", "ParticipantB"],
        "the deployment map places only WorldAuthority and ParticipantB in process B"
    );
    assert_eq!(
        process_a.trusted_control_delivery(),
        I3LocalnetControlDelivery::DedicatedOneShotTrustedFd,
        "each child receives its exact retained startup binding on a dedicated trusted FD"
    );
    assert_eq!(
        process_b.trusted_control_delivery(),
        I3LocalnetControlDelivery::DedicatedOneShotTrustedFd,
        "each child receives its exact retained startup binding on a dedicated trusted FD"
    );
    assert_eq!(
        process_a.tainted_image_delivery(),
        I3LocalnetImageDelivery::DedicatedTaintedImageFd,
        "the untrusted image path must be distinct from the trusted control path"
    );
    assert_eq!(
        process_b.tainted_image_delivery(),
        I3LocalnetImageDelivery::DedicatedTaintedImageFd,
        "the untrusted image path must be distinct from the trusted control path"
    );

    let startup = run.startup_audit();
    assert_eq!(startup.supervisor_ordinary_source_build_count(), 1);
    assert_eq!(startup.supervisor_admission_count(), 1);
    assert_eq!(startup.supervisor_m9_generation_count(), 1);
    assert!(
        startup.child_bootstrap_is_image_only_no_source_or_global_authority(),
        "the startup audit must structurally establish image-only bootstrap with no source or global-authority carriage"
    );
    assert!(
        startup.stores_are_process_local_and_distinct(),
        "the startup audit must structurally establish two distinct child-local stores"
    );
    assert!(
        startup.exact_one_shot_bindings_consumed(),
        "both exact retained startup bindings must be consumed once, after tainted-image decode"
    );

    let execution = run.execution_audit();
    assert_ne!(
        execution.requester_child(),
        execution.owner_child(),
        "the request must cross from requester to remote semantic owner"
    );
    assert_eq!(execution.generated_request_count(), 1);
    assert_eq!(execution.remote_owner_serve_count(), 1);
    assert_eq!(execution.remote_owner_write_count(), 1);
    assert_eq!(execution.generated_reply_count(), 1);
    assert_eq!(execution.requester_local_receipt_count(), 1);
    assert_eq!(
        execution.network_receipt_frame_count(),
        0,
        "receipt remains requester-local; a network receipt frame would collapse the carrier phases"
    );
    assert!(
        execution.source_derived_only(),
        "the run may use only generated artifacts and communication from the admitted source"
    );

    let transport = run.transport_audit();
    assert!(transport.mutually_authenticated_quic_peer_binding());
    assert!(transport.reliable_bidirectional_streams_only());
    assert!(!transport.quic_datagrams_enabled());
    assert_eq!(transport.unauthenticated_semantic_admission_count(), 0);
    assert!(
        transport.ephemeral_endpoint_reuse_verified(),
        "a second bounded run must safely reuse a fresh ephemeral endpoint"
    );

    let trace = run.observer_safe_trace();
    assert!(trace.is_observer_safe());
    assert!(trace.has_exact_source_core_artifact_carrier_network_runtime_chain());
    assert_eq!(trace.source_ref_count(), 1);
    assert_eq!(trace.core_ref_count(), 1);
    assert!(
        trace.artifact_ref_count() >= 2,
        "both deployment images must retain artifact lineage"
    );
    assert_eq!(trace.semantic_request_identity_count(), 1);
    assert!(
        trace.network_occurrence_count() >= 2,
        "request and reply retain distinct network occurrences"
    );
    assert!(
        trace.runtime_occurrence_count() >= 5,
        "request/serve/write/reply/receipt remain distinguishable"
    );

    let request_contract = active_contract("owner-request");
    let reply_contract = active_contract("owner-reply-receipt");

    // Four records are required: the actual request send/receive and actual
    // reply send/receive.  A final supervisor-assembled reference bundle is
    // not evidence that either child carried the generated source/Core/image
    // lineage across the network boundary.
    let deliveries = trace.actual_delivery_records();
    assert_eq!(
        deliveries.len(),
        4,
        "the joined trace must retain every finite request/reply delivery event"
    );
    let request_send = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::RequestSend);
    let request_receive = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::RequestReceive);
    let reply_send = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::ReplySend);
    let reply_receive = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::ReplyReceive);
    let request_identity = request_send.semantic_request_identity_ref();

    assert_delivery_matches_contract(request_send, &request_contract, request_identity, None);
    assert_delivery_matches_contract(request_receive, &request_contract, request_identity, None);
    assert_delivery_matches_contract(
        reply_send,
        &reply_contract,
        request_identity,
        Some(request_identity),
    );
    assert_delivery_matches_contract(
        reply_receive,
        &reply_contract,
        request_identity,
        Some(request_identity),
    );
    assert_delivery_semantics_match(request_send, request_receive);
    assert_delivery_semantics_match(reply_send, reply_receive);
    assert_ne!(
        request_send.carrier_ref(),
        reply_send.carrier_ref(),
        "generated request and reply must retain distinct private carrier references"
    );
    assert_eq!(
        [
            request_send.network_occurrence_ref(),
            request_receive.network_occurrence_ref(),
            reply_send.network_occurrence_ref(),
            reply_receive.network_occurrence_ref(),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len(),
        4,
        "each actual send/receive event must retain a distinct local network occurrence"
    );

    // These inventories must be populated from the four child delivery
    // records.  Scalar counts alone can be produced by testing that buffers
    // are nonempty, and therefore cannot show which checked artifacts crossed
    // this exact process boundary.
    assert_eq!(
        trace.actual_source_ref_inventory(),
        expected_ref_inventory(&[request_contract.source_ref(), reply_contract.source_ref()])
    );
    assert_eq!(
        trace.actual_core_ref_inventory(),
        expected_ref_inventory(&[request_contract.core_ref(), reply_contract.core_ref()])
    );
    assert_eq!(
        trace.actual_artifact_ref_inventory(),
        expected_ref_inventory(&[
            request_contract.source_artifact_ref(),
            request_contract.target_artifact_ref(),
            reply_contract.source_artifact_ref(),
            reply_contract.target_artifact_ref(),
        ]),
        "the actual artifact inventory must contain every process-image endpoint"
    );
    assert_eq!(
        trace.actual_edge_ref_inventory(),
        expected_ref_inventory(&[request_contract.edge_ref(), reply_contract.edge_ref()])
    );
    assert_eq!(
        trace.actual_source_ref_count(),
        trace.actual_source_ref_inventory().len()
    );
    assert_eq!(
        trace.actual_core_ref_count(),
        trace.actual_core_ref_inventory().len()
    );
    assert_eq!(
        trace.actual_artifact_ref_count(),
        trace.actual_artifact_ref_inventory().len()
    );
    assert_eq!(
        trace.actual_edge_ref_count(),
        trace.actual_edge_ref_inventory().len()
    );

    let refs = trace.references();
    assert_eq!(refs.request_source_ref(), request_contract.source_ref());
    assert_eq!(refs.request_core_ref(), request_contract.core_ref());
    assert_eq!(
        refs.request_source_artifact_ref(),
        request_contract.source_artifact_ref()
    );
    assert_eq!(
        refs.request_target_artifact_ref(),
        request_contract.target_artifact_ref()
    );
    assert_eq!(refs.request_edge_ref(), request_contract.edge_ref());
    assert_eq!(refs.reply_source_ref(), reply_contract.source_ref());
    assert_eq!(refs.reply_core_ref(), reply_contract.core_ref());
    assert_eq!(
        refs.reply_source_artifact_ref(),
        reply_contract.source_artifact_ref()
    );
    assert_eq!(
        refs.reply_target_artifact_ref(),
        reply_contract.target_artifact_ref()
    );
    assert_eq!(refs.reply_edge_ref(), reply_contract.edge_ref());
    assert!(!refs.request_carrier_ref().is_empty());
    assert!(!refs.reply_carrier_ref().is_empty());
    assert_ne!(
        refs.request_carrier_ref(),
        refs.reply_carrier_ref(),
        "request and reply carriers must not collapse into one evidence reference"
    );
    assert!(!refs.semantic_request_identity_ref().is_empty());
    assert_eq!(
        refs.network_request_identity_ref(),
        refs.semantic_request_identity_ref(),
        "the observed network request must retain its actual semantic identity"
    );
    assert_eq!(
        refs.network_reply_linked_request_identity_ref(),
        refs.semantic_request_identity_ref(),
        "the observed reply must retain its link to the actual request identity"
    );
    assert_eq!(
        refs.runtime_serve_request_identity_ref(),
        refs.semantic_request_identity_ref(),
        "owner serve must be reported against that same semantic request"
    );
    assert_eq!(
        refs.runtime_write_request_identity_ref(),
        refs.semantic_request_identity_ref(),
        "owner write must be reported against that same semantic request"
    );
    assert_eq!(
        refs.runtime_receipt_linked_request_identity_ref(),
        refs.semantic_request_identity_ref(),
        "requester-local receipt must retain that same request linkage"
    );
    assert!(!refs.network_request_occurrence_ref().is_empty());
    assert!(!refs.network_reply_occurrence_ref().is_empty());
    assert_ne!(
        refs.network_request_occurrence_ref(),
        refs.network_reply_occurrence_ref(),
        "request and reply have distinct network occurrences"
    );
    assert!(!refs.runtime_serve_occurrence_ref().is_empty());
    assert!(!refs.runtime_write_occurrence_ref().is_empty());
    assert!(!refs.runtime_receipt_occurrence_ref().is_empty());
    assert_ne!(
        refs.requester_local_store_ref(),
        refs.owner_local_store_ref(),
        "the joined trace must identify two actual distinct child-local stores"
    );

    let lifecycle = run.lifecycle();
    assert!(process_a.reaped());
    assert!(process_b.reaped());
    let process_a_exit = process_a
        .observed_exit_status()
        .expect("a reaped exec child must retain its observed OS exit status");
    let process_b_exit = process_b
        .observed_exit_status()
        .expect("a reaped exec child must retain its observed OS exit status");
    assert_eq!(
        process_a_exit.code(),
        Some(0),
        "process A must exit successfully after reporting its actual completion"
    );
    assert_eq!(
        process_b_exit.code(),
        Some(0),
        "process B must exit successfully after reporting its actual completion"
    );
    assert!(
        !process_a.was_force_killed(),
        "the positive path must reap process A without force-killing it"
    );
    assert!(
        !process_b.was_force_killed(),
        "the positive path must reap process B without force-killing it"
    );
    assert!(lifecycle.all_children_reaped());
    assert!(
        lifecycle.clean_shutdown(),
        "clean shutdown is valid only after both observed zero-exit children are reaped"
    );
    assert!(
        lifecycle.clean_shutdown_is_backed_by_zero_exit_reaps_without_force_kill(),
        "the lifecycle audit must expose the structural basis of clean shutdown, not a supervisor success flag"
    );
    assert!(
        lifecycle.observed_supervised_process_lifecycle_elapsed()
            <= lifecycle.observed_supervised_process_lifecycle_bound(),
        "the audit must distinguish the bounded child process/reaping phase from unbounded source/build/credential preflight"
    );
    assert!(
        lifecycle.zero_exit_reap_observed_within_deadline(),
        "positive success requires a captured natural zero-exit/reap observation before the process lifecycle bound"
    );
    assert_eq!(
        lifecycle.observed_supervised_process_lifecycle_elapsed(),
        lifecycle.captured_zero_exit_reap_observation_elapsed(),
        "reported lifecycle elapsed must be captured at natural zero-exit reaping, not later during evidence assembly"
    );
}

#[test]
fn swapped_complete_image_and_binding_pairs_are_rejected_before_child_start_or_network_activity() {
    // The falsifier swaps both complete image and retained binding pairs, while
    // preserving the private child keys and their fixed control descriptors.
    // Therefore a digest carried in the tainted image alone cannot authorize
    // startup: the child-bound trusted-control association has to reject it.
    let error = run_i3_process_localnet(
        canonical_request().with_falsifier(I3LocalnetFalsifier::SwapImageAndBindingPairs),
    )
    .expect_err("swapping two otherwise complete process-image/binding pairs must fail closed");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::StartBindingRejected);
    let audit = error.rejection_audit();
    assert_eq!(audit.stage(), I3LocalnetFailureStage::BeforeOwnerStart);
    assert!(audit.fixed_child_control_descriptors_preserved());
    assert_eq!(audit.child_owner_starts(), 0);
    assert_eq!(audit.quic_certificate_initializations(), 0);
    assert_eq!(audit.quic_handshake_count(), 0);
    assert_eq!(audit.semantic_admission_count(), 0);
    assert_eq!(audit.owner_mutation_count(), 0);
    assert!(audit.all_children_reaped());
    assert!(audit.observer_safe());
}

#[test]
fn invalid_reciprocal_preface_is_rejected_before_semantic_admission() {
    // This retained falsifier changes the private reciprocal preface.  It is
    // deliberately *not* evidence of a real wrong mTLS peer; that stronger
    // case is specified separately below.
    let error = run_i3_process_localnet(
        canonical_request().with_falsifier(I3LocalnetFalsifier::InjectUnauthenticatedReply),
    )
    .expect_err("a reply from an unauthenticated peer must not reach semantic admission");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::PeerBindingRejected);
    let audit = error.rejection_audit();
    assert_eq!(
        audit.stage(),
        I3LocalnetFailureStage::BeforeSemanticAdmission
    );
    assert_eq!(audit.semantic_admission_count(), 0);
    assert_eq!(audit.owner_mutation_count(), 0);
    assert!(audit.all_children_reaped());
    assert!(audit.observer_safe());
}

#[test]
fn ca_signed_wrong_spki_reply_peer_preserves_requester_pending_store_counter_and_occurrence_state()
{
    // This is stronger than the old bad-preface case: a distinct leaf signed
    // by the run CA must complete the real QUIC peer attempt, but its SPKI is
    // not the child-bound expected peer.  The requester has already emitted
    // its source-derived request, so the rejection must leave its pending,
    // store, counter, and semantic-occurrence snapshot exactly unchanged.
    let error = run_i3_process_localnet(
        canonical_request()
            .with_falsifier(I3LocalnetFalsifier::DeliverReplyFromCaSignedWrongSpkiPeer),
    )
    .expect_err("a CA-signed but child-unexpected peer must not deliver a reply");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::PeerBindingRejected);
    let audit = error.rejection_audit();
    assert_eq!(
        audit.stage(),
        I3LocalnetFailureStage::BeforeSemanticAdmission
    );
    assert!(audit.real_wrong_peer_delivery_attempted());
    assert_eq!(
        audit.adapter_rejection_kind(),
        Some(I3LocalnetAdapterRejectionKind::PeerBindingRejected),
        "the adapter must report the exact peer-binding rejection, not flatten it into a connect or frame error"
    );
    assert!(
        audit
            .wrong_peer_ca_validated_leaf_ref()
            .is_some_and(|reference| !reference.is_empty()),
        "the rejected wrong peer must have presented a CA-valid leaf, retained only as an observer-safe reference"
    );
    let expected_spki = audit
        .expected_peer_spki_ref()
        .expect("the child-bound expected SPKI reference must be retained for the comparison");
    let actual_spki = audit.actual_peer_spki_ref().expect(
        "the CA-valid wrong peer's actual SPKI reference must be retained for the comparison",
    );
    assert!(!expected_spki.is_empty());
    assert!(!actual_spki.is_empty());
    assert_ne!(
        expected_spki, actual_spki,
        "a CA-valid leaf is still rejected when its actual SPKI differs from the child-bound expected SPKI"
    );
    assert_eq!(
        audit.requester_observer_state_before(),
        audit.requester_observer_state_after(),
        "wrong-peer reply delivery must preserve requester pending/store/counter/occurrence state"
    );
    assert!(
        audit.requester_pending_request_is_retained(),
        "the original generated request remains pending for a future explicit I3-3 disposition"
    );
    assert_eq!(audit.semantic_admission_count(), 0);
    assert!(audit.all_children_reaped());
    assert!(audit.observer_safe());
}

fn assert_stalled_localnet_case_is_bounded_and_reaped(
    falsifier: I3LocalnetFalsifier,
    expected_stage: I3LocalnetFailureStage,
) {
    let start = Instant::now();
    let error = run_i3_process_localnet(
        canonical_request()
            .with_deadline(Duration::from_millis(250))
            .with_falsifier(falsifier),
    )
    .expect_err("the explicitly stalled private test path must time out and clean up");
    let elapsed = start.elapsed();

    assert_eq!(
        error.kind(),
        I3LocalnetRunErrorKind::LifecycleDeadlineExceeded
    );
    let audit = error.rejection_audit();
    assert_eq!(audit.stage(), expected_stage);
    assert!(audit.deadline_enforced());
    assert!(audit.reaper_deadline_enforced());
    assert!(audit.all_children_reaped());
    assert!(audit.observer_safe());
    assert!(
        elapsed < Duration::from_secs(5),
        "a bounded 250ms private deadline must not leave a stalled child or cleanup wait hanging"
    );
}

#[test]
fn stalled_image_or_control_bootstrap_hits_its_deadline_and_reaps_children() {
    assert_stalled_localnet_case_is_bounded_and_reaped(
        I3LocalnetFalsifier::StallImageOrControlBootstrap,
        I3LocalnetFailureStage::BootstrapDeadline,
    );
}

#[test]
fn stalled_child_cleanup_hits_its_reaper_deadline_and_reaps_children() {
    assert_stalled_localnet_case_is_bounded_and_reaped(
        I3LocalnetFalsifier::StallCleanup,
        I3LocalnetFailureStage::CleanupDeadline,
    );
}

const COMPLETED_CHILD_MAIN_DEADLINE: Duration = Duration::from_secs(1);
const COMPLETED_CHILD_REAPER_ALLOWANCE: Duration = Duration::from_secs(2);

/// A child `Completed` event is only a structurally valid child report.  It is
/// not supervisor success: the parent must still observe child exit and reap
/// it.  The reaper budget is separate and finite so this test cannot mask an
/// orphan with an unbounded cleanup wait.  Ordinary-source build, admission,
/// and credential preflight occur before this measured process lifecycle and
/// are deliberately outside the finite I3-2 child execution budget.
fn completed_child_lifecycle_failure_is_bounded(
    falsifier: I3LocalnetFalsifier,
    expected_kind: I3LocalnetRunErrorKind,
) -> I3LocalnetRejectionAudit {
    let start = Instant::now();
    let error = run_i3_process_localnet(
        canonical_request()
            .with_deadline(COMPLETED_CHILD_MAIN_DEADLINE)
            .with_reaper_allowance(COMPLETED_CHILD_REAPER_ALLOWANCE)
            .with_falsifier(falsifier),
    )
    .expect_err(
        "a child report followed by invalid process lifecycle must not be accepted as localnet success",
    );
    let elapsed = start.elapsed();

    assert_eq!(error.kind(), expected_kind);
    let audit = error.rejection_audit();
    assert!(
        audit.structurally_valid_completed_child_report_observed(),
        "the falsifier must emit a normal-shaped Completed report before its process-lifecycle fault"
    );
    assert!(audit.all_children_reaped());
    assert!(
        audit.no_orphan_child_pids(),
        "failure cleanup must reap every spawned child and leave no orphan PID"
    );
    assert!(audit.observer_safe());
    assert!(
        elapsed < Duration::from_secs(5),
        "the complete CI call needs a coarse no-hang bound even though preflight is outside the process lifecycle budget"
    );
    assert_eq!(
        audit.observed_supervised_process_lifecycle_bound(),
        COMPLETED_CHILD_MAIN_DEADLINE + COMPLETED_CHILD_REAPER_ALLOWANCE,
        "the recorded process lifecycle bound must be the selected main deadline plus explicit reaper allowance"
    );
    assert!(
        audit.observed_supervised_process_lifecycle_elapsed()
            <= audit.observed_supervised_process_lifecycle_bound(),
        "only the observed child execution/reaping phase is required to fit the I3-2 lifecycle budget"
    );
    audit
}

#[test]
fn completed_child_that_exits_nonzero_is_lifecycle_rejected_and_reaped() {
    let audit = completed_child_lifecycle_failure_is_bounded(
        I3LocalnetFalsifier::CompletedThenNonzero,
        I3LocalnetRunErrorKind::LifecycleRejected,
    );
    assert!(audit.completed_child_exited_nonzero());
    assert!(
        !audit.deadline_enforced(),
        "a reported nonzero exit is an immediate lifecycle rejection, not a timeout"
    );
    assert!(
        !audit.reaper_deadline_enforced(),
        "a normally reaped nonzero child must not be reported as a reaper timeout"
    );
}

#[test]
fn completed_child_that_ignores_graceful_completion_hits_reaper_deadline_without_orphaning() {
    let audit = completed_child_lifecycle_failure_is_bounded(
        I3LocalnetFalsifier::CompletedThenHang,
        I3LocalnetRunErrorKind::LifecycleDeadlineExceeded,
    );

    assert!(audit.completed_child_ignored_graceful_completion());
    assert!(audit.completed_child_was_force_killed_after_reaper_allowance());
    assert!(audit.deadline_enforced());
    assert!(audit.reaper_deadline_enforced());
}

#[test]
fn delayed_supervisor_zero_exit_observation_past_lifecycle_deadline_is_rejected_not_accepted() {
    // Both children have already emitted structurally valid Completed reports
    // and naturally exited zero.  The fault is solely a delayed supervisor
    // terminal-reap observation, proving that a later evidence-assembly pass
    // cannot retroactively turn that missed lifecycle deadline into success.
    // Its private hook must wait past the *same absolute* main-plus-reaper
    // deadline that started before spawn; it must not create a fresh
    // post-Completed observation clock.
    // The process lifecycle begins before child spawn.  A real server Ready
    // report consumes roughly 144ms on the supported CI host, so 250ms cannot
    // deterministically reach both natural Completed reports; one second is
    // the smallest stable finite main budget for this exact falsifier.
    let main_deadline = Duration::from_secs(1);
    let reaper_allowance = Duration::from_millis(200);
    let error = run_i3_process_localnet(
        canonical_request()
            .with_deadline(main_deadline)
            .with_reaper_allowance(reaper_allowance)
            .with_falsifier(I3LocalnetFalsifier::DelaySupervisorExitObservationPastDeadline),
    )
    .expect_err(
        "late terminal-reap observation must be a typed lifecycle deadline failure, never an accepted run",
    );

    assert_eq!(
        error.kind(),
        I3LocalnetRunErrorKind::LifecycleDeadlineExceeded
    );
    let audit = error.rejection_audit();
    assert!(audit.deadline_enforced());
    assert!(audit.all_children_reaped());
    assert!(audit.no_orphan_child_pids());
    assert!(
        !audit.zero_exit_reap_observed_within_deadline(),
        "a delayed supervisor observation must remain visibly after the lifecycle bound"
    );
    assert_eq!(
        audit.observed_supervised_process_lifecycle_bound(),
        main_deadline + reaper_allowance
    );
    assert!(
        audit.captured_zero_exit_reap_observation_elapsed()
            > audit.observed_supervised_process_lifecycle_bound(),
        "the captured terminal-reap observation itself, rather than later evidence work, must establish the deadline miss"
    );

    let terminal_events = audit.child_terminal_events();
    assert_eq!(
        terminal_events.len(),
        2,
        "both naturally completed child terminal reports must survive the aggregate deadline failure"
    );
    for event in terminal_events {
        assert_eq!(event.outcome(), I3LocalnetChildTerminalOutcome::Completed);
        assert_eq!(
            event.observed_exit_status_code(),
            Some(0),
            "the deadline failure must retain each child's natural zero exit evidence"
        );
        assert!(
            !event.was_force_killed(),
            "late observation of an already-zero child must not be recorded as a forced kill"
        );
    }
    assert!(audit.observer_safe());
}

#[test]
fn setup_failure_selected_with_stall_mode_keeps_its_actual_lifecycle_cause_not_a_timeout() {
    // The private compound falsifier makes setup/control fail before the
    // stall can become a deadline event.  It protects error classification:
    // no implementation may reinterpret an immediate setup failure as a
    // generic `LifecycleDeadlineExceeded` merely because a stall mode was
    // also selected.
    let error = run_i3_process_localnet(
        canonical_request()
            .with_deadline(COMPLETED_CHILD_MAIN_DEADLINE)
            .with_reaper_allowance(COMPLETED_CHILD_REAPER_ALLOWANCE)
            .with_falsifier(I3LocalnetFalsifier::SetupFailureDuringStallMode),
    )
    .expect_err("setup/control failure must be retained ahead of a selected stall mode");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
    let audit = error.rejection_audit();
    assert_eq!(
        audit.lifecycle_rejection_cause(),
        Some(I3LocalnetLifecycleRejectionCause::SetupOrControlFailure),
        "the typed rejection must retain the actual setup/control cause"
    );
    assert!(
        !audit.deadline_enforced(),
        "an immediate setup/control failure must not fabricate deadline enforcement"
    );
    assert!(
        !audit.reaper_deadline_enforced(),
        "an immediate setup/control failure must not fabricate reaper deadline enforcement"
    );
    assert!(audit.all_children_reaped());
    assert!(audit.no_orphan_child_pids());
    assert!(audit.observer_safe());
}

#[test]
fn undersized_reaper_allowance_is_rejected_before_any_child_is_spawned() {
    // The private I3-2 launcher has a fixed nonzero reserve for force-reaping
    // a child that ignores graceful completion.  An allowance below it,
    // including zero, is malformed launch input: it is not a runtime timeout.
    let reserve = I3ProcessLocalnetRequest::minimum_force_reap_reserve();
    assert!(
        reserve > Duration::ZERO,
        "a finite localnet launcher must publish a nonzero force-reap reserve"
    );
    let just_below_reserve = reserve
        .checked_sub(Duration::from_nanos(1))
        .expect("a nonzero force-reap reserve has an undersized finite predecessor");

    for allowance in [Duration::ZERO, just_below_reserve] {
        let error = run_i3_process_localnet(
            canonical_request()
                .with_reaper_allowance(allowance)
                .with_falsifier(I3LocalnetFalsifier::CompletedThenHang),
        )
        .expect_err("an allowance below the fixed force-reap reserve must fail before child spawn");

        assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
        let audit = error.rejection_audit();
        assert_eq!(
            audit.lifecycle_rejection_cause(),
            Some(I3LocalnetLifecycleRejectionCause::InvalidReaperAllowance),
            "invalid reaper capacity must retain its configuration cause rather than masquerading as a deadline"
        );
        assert_eq!(audit.spawned_child_count(), 0);
        assert!(audit.no_orphan_child_pids());
        assert!(audit.all_children_reaped());
        assert!(
            !audit.deadline_enforced() && !audit.reaper_deadline_enforced(),
            "no deadline is enforced when input validation rejects before process spawn"
        );
        assert!(audit.observer_safe());
    }
}

#[test]
fn asymmetric_completed_and_rejected_child_reports_preserve_both_terminal_and_mutation_evidence() {
    // One child reports Rejected while its counterpart already reports a
    // structurally valid Completed path that admitted and mutated.  The
    // terminal run is rejected, but its evidence must aggregate *both* child
    // reports rather than overwrite the completed side with zero counters.
    let error = run_i3_process_localnet(
        canonical_request()
            .with_deadline(COMPLETED_CHILD_MAIN_DEADLINE)
            .with_reaper_allowance(COMPLETED_CHILD_REAPER_ALLOWANCE)
            .with_falsifier(I3LocalnetFalsifier::AsymmetricCompletedAndRejected),
    )
    .expect_err("asymmetric terminal child reports must reject the aggregate run");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
    let audit = error.rejection_audit();
    let terminal_events = audit.child_terminal_events();
    assert_eq!(
        terminal_events.len(),
        2,
        "both child terminal reports must remain present in the rejected aggregate"
    );
    assert_eq!(
        terminal_events
            .iter()
            .filter(|event| event.outcome() == I3LocalnetChildTerminalOutcome::Rejected)
            .count(),
        1
    );
    let completed = terminal_events
        .iter()
        .find(|event| event.outcome() == I3LocalnetChildTerminalOutcome::Completed)
        .expect("the counterpart Completed report must not be erased by a child Rejected report");
    assert!(completed.semantic_admission_count() > 0);
    assert!(completed.owner_mutation_count() > 0);
    assert_eq!(
        audit.aggregate_semantic_admission_count(),
        terminal_events
            .iter()
            .map(|event| event.semantic_admission_count())
            .sum::<usize>(),
        "aggregate admissions must account for every reported child terminal event"
    );
    assert_eq!(
        audit.aggregate_owner_mutation_count(),
        terminal_events
            .iter()
            .map(|event| event.owner_mutation_count())
            .sum::<usize>(),
        "aggregate mutations must account for every reported child terminal event"
    );
    assert!(audit.aggregate_semantic_admission_count() > 0);
    assert!(audit.aggregate_owner_mutation_count() > 0);
    assert!(audit.all_children_reaped());
    assert!(audit.no_orphan_child_pids());
    assert!(audit.observer_safe());
}
