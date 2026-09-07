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
    I3LocalnetFalsifier, I3LocalnetFaultAuditFalsifier, I3LocalnetFaultProfile,
    I3LocalnetImageDelivery, I3LocalnetLateIngressAckReaderOutcome, I3LocalnetLateIngressAudit,
    I3LocalnetLateIngressEvidenceRejection, I3LocalnetLateIngressFalsifier,
    I3LocalnetLateIngressLifecycleProvenance,
    I3LocalnetLateIngressNonregisteredAckInputDisposition, I3LocalnetLateIngressOwnerOutcome,
    I3LocalnetLateIngressParentPublication, I3LocalnetLateIngressProfile,
    I3LocalnetLateIngressRequesterOutcome, I3LocalnetLifecycleRejectionCause,
    I3LocalnetObserverSafeDeliveryRecord, I3LocalnetReconnectOwnerOutcome,
    I3LocalnetRejectionAudit, I3LocalnetRemoteAdmissionEvidence, I3LocalnetRemoteEvidenceRejection,
    I3LocalnetRequesterFaultObservation, I3LocalnetRetryAttemptReason, I3LocalnetRetryAudit,
    I3LocalnetRetryAuditFalsifier, I3LocalnetRetryEvidenceRejection, I3LocalnetRetryFalsifier,
    I3LocalnetRetryProfile, I3LocalnetRetryRequesterOutcome, I3LocalnetRunErrorKind,
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

/// A fault result is not a semantic success merely because both child PIDs
/// were naturally reaped.  The terminal reports must remain distinct from
/// I3-2's successful `Completed` reports so a supervisor cannot turn fault
/// cleanup into a successful source-derived round trip.
fn assert_handled_delivery_fault_lifecycle(audit: &I3LocalnetRejectionAudit) {
    let terminal_events = audit.child_terminal_events();
    assert_eq!(
        terminal_events.len(),
        2,
        "a handled delivery fault must retain a terminal event for each exec child"
    );
    assert!(
        terminal_events.iter().all(|event| {
            event.outcome() == I3LocalnetChildTerminalOutcome::HandledDeliveryFault
        })
    );
    assert!(audit.all_children_reaped());
    assert!(audit.no_orphan_child_pids());
    assert!(
        terminal_events.iter().all(|event| {
            event.observed_exit_status_code() == Some(0) && !event.was_force_killed()
        }),
        "a normal handled fault must retain each child's natural zero exit rather than a force-kill cleanup"
    );
    assert!(
        audit.zero_exit_reap_observed_within_deadline(),
        "the normal fault path must retain clean shutdown evidence from natural zero-exit reaping"
    );
    assert!(audit.observer_safe());
}

fn assert_no_rejected_child_transport_observation(audit: &I3LocalnetRejectionAudit) {
    assert_eq!(
        audit.fixed_child_control_descriptors_preserved(),
        None,
        "a handled delivery fault has no rejected-child control observation"
    );
    assert_eq!(
        audit.quic_certificate_initializations(),
        None,
        "a handled delivery fault has no rejected-child certificate count"
    );
    assert_eq!(
        audit.quic_handshake_count(),
        None,
        "a handled delivery fault has no rejected-child handshake count"
    );
}

fn assert_retry_child_sessions(
    audit: &I3LocalnetRetryAudit,
    expected_terminal_outcome: I3LocalnetChildTerminalOutcome,
) {
    let requester = audit.requester_child();
    let owner = audit.owner_child();
    assert_eq!(requester.slot(), I3LocalnetChildSlot::ProcessA);
    assert_eq!(owner.slot(), I3LocalnetChildSlot::ProcessB);
    assert_ne!(requester.slot(), owner.slot());
    assert!(!requester.run_ref().is_empty());
    assert_eq!(requester.run_ref(), owner.run_ref());
    let reconnect_attempt = requester.reconnect_session_runtime_attempt();
    assert_eq!(
        reconnect_attempt.semantic_request_identity_ref(),
        audit.request_identity_ref(),
        "the runtime attempt summary must retain the source-generated semantic identity"
    );
    assert_eq!(
        reconnect_attempt.source_requester_locus(),
        "ParticipantA",
        "the retry summary must retain the runtime-selected requester locus"
    );
    assert!(
        !reconnect_attempt.requester_binding_ref().is_empty(),
        "the retry summary must retain its observer-safe requester binding"
    );
    assert_eq!(
        reconnect_attempt.reason(),
        I3LocalnetRetryAttemptReason::ReconnectRetry,
        "the runtime, not the profile, must classify the begun second-session action as its fixed reconnect retry"
    );
    match audit.profile() {
        I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite => {
            assert!(
                requester.first_session_runtime_attempt().is_none(),
                "a pre-write first session must not consume the runtime delivery-attempt budget"
            );
            assert_eq!(reconnect_attempt.attempt_generation(), 1);
        }
        I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply => {
            let first_attempt = requester.first_session_runtime_attempt().expect(
                "the post-admission profile must retain its real first runtime attempt before reconnect",
            );
            assert_eq!(
                first_attempt.semantic_request_identity_ref(),
                audit.request_identity_ref()
            );
            assert_eq!(first_attempt.source_requester_locus(), "ParticipantA");
            assert!(!first_attempt.requester_binding_ref().is_empty());
            assert_eq!(first_attempt.attempt_generation(), 1);
            assert_eq!(
                first_attempt.reason(),
                I3LocalnetRetryAttemptReason::InitialDelivery
            );
            assert_eq!(reconnect_attempt.attempt_generation(), 2);
        }
    }
    for child in [requester, owner] {
        assert_eq!(child.first_session_generation(), 1);
        assert_eq!(child.reconnect_session_generation(), 2);
        assert!(child.first_session_peer_spki_verified());
        assert!(child.first_session_reciprocal_preface_verified());
        assert!(child.reconnect_session_peer_spki_verified());
        assert!(child.reconnect_session_reciprocal_preface_verified());
        assert_eq!(child.terminal_outcome(), expected_terminal_outcome);
    }
}

fn assert_initial_retry_owner_admission(
    audit: &I3LocalnetRetryAudit,
    request_contract: &Sys5I3AdapterCarrierContract,
    request_identity: &str,
) {
    let initial_sender = audit.initial_request_delivery().expect(
        "post-admission retry evidence must retain the actual first requester send before joining owner admission",
    );
    match audit.initial_owner_admission() {
        Some(I3LocalnetRemoteAdmissionEvidence::Admitted {
            request_receive,
            owner_serve_count,
            owner_mutation_count,
            owner_serve_occurrence_ref,
            owner_write_occurrence_ref,
        }) => {
            assert_delivery_matches_contract(
                request_receive.as_ref(),
                request_contract,
                request_identity,
                None,
            );
            assert_delivery_semantics_match(initial_sender, request_receive.as_ref());
            assert!(
                !initial_sender.candidate_commitment_ref().is_empty(),
                "the first requester send must retain its adapter-computed commitment"
            );
            assert!(
                !request_receive.candidate_commitment_ref().is_empty(),
                "the admitted owner receive must retain its adapter-computed commitment"
            );
            assert_eq!(
                initial_sender.candidate_commitment_ref(),
                request_receive.candidate_commitment_ref(),
                "owner admission must join the exact actual first-session sender commitment"
            );
            assert_eq!(*owner_serve_count, 1);
            assert_eq!(*owner_mutation_count, 1);
            assert!(!owner_serve_occurrence_ref.is_empty());
            assert!(
                owner_write_occurrence_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.is_empty())
            );
        }
        Some(I3LocalnetRemoteAdmissionEvidence::NoAdmission { .. }) => {
            panic!("post-admission retry evidence must not report known no-admission")
        }
        None => panic!("post-admission retry must retain the actual first owner admission"),
    }
}

/// The held complete frame is a receiver-side transport fact, not decoded
/// semantic provenance.  It may join the independent exact sender contract
/// only through the adapter-derived session-one commitment and its distinct
/// receive occurrence.
fn assert_late_ingress_sender_and_retained_frame<'audit>(
    audit: &'audit I3LocalnetLateIngressAudit,
    request_contract: &Sys5I3AdapterCarrierContract,
) -> &'audit I3LocalnetObserverSafeDeliveryRecord {
    let sender = audit.initial_sender_delivery();
    assert_delivery_matches_contract(sender, request_contract, audit.request_identity_ref(), None);

    let retained = audit.retained_session_one_ingress();
    assert_eq!(retained.session_generation(), 1);
    assert!(
        !sender.candidate_commitment_ref().is_empty(),
        "the actual initial sender delivery must retain its adapter-derived commitment"
    );
    assert!(
        !retained.candidate_commitment_ref().is_empty(),
        "the retained receiver frame must retain its adapter-derived session-one commitment"
    );
    assert_eq!(
        sender.candidate_commitment_ref(),
        retained.candidate_commitment_ref(),
        "the held receiver frame may join only the exact same session-one sender commitment"
    );
    assert!(
        !retained.network_occurrence_ref().is_empty(),
        "the held frame must retain its actual receiver-side network occurrence"
    );
    assert_ne!(
        sender.network_occurrence_ref(),
        retained.network_occurrence_ref(),
        "send and receive are separate actual transport events even for one retained frame"
    );
    sender
}

fn assert_late_ingress_child_sessions(
    audit: &I3LocalnetLateIngressAudit,
    expected_terminal_outcome: I3LocalnetChildTerminalOutcome,
) {
    let requester = audit.requester_child();
    let owner = audit.owner_child();
    assert_eq!(requester.slot(), I3LocalnetChildSlot::ProcessA);
    assert_eq!(owner.slot(), I3LocalnetChildSlot::ProcessB);
    assert_ne!(requester.slot(), owner.slot());
    assert!(!requester.run_ref().is_empty());
    assert_eq!(requester.run_ref(), owner.run_ref());

    for child in [requester, owner] {
        assert_eq!(child.first_session_generation(), 1);
        assert_eq!(child.reconnect_session_generation(), 2);
        assert!(child.first_session_peer_spki_verified());
        assert!(child.first_session_reciprocal_preface_verified());
        assert!(child.reconnect_session_peer_spki_verified());
        assert!(child.reconnect_session_reciprocal_preface_verified());
        assert_eq!(child.terminal_outcome(), expected_terminal_outcome);
    }
}

fn assert_prestaged_late_ingress_rejection(
    audit: &I3LocalnetLateIngressAudit,
    expected_ack_reader: I3LocalnetLateIngressAckReaderOutcome,
    expected_publication: I3LocalnetLateIngressParentPublication,
    expected_publication_commit_count: usize,
    expected_nonregistered_ack_input_disposition: I3LocalnetLateIngressNonregisteredAckInputDisposition,
    expected_nonregistered_ack_input_count: usize,
) {
    assert_eq!(
        audit.profile(),
        I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission
    );
    assert!(!audit.request_identity_ref().is_empty());
    let request_contract = active_contract("owner-request");
    let initial_sender = assert_late_ingress_sender_and_retained_frame(audit, &request_contract);
    assert!(
        audit.late_admission_delivery().is_none(),
        "a rejected held frame must not expose decoded source/Core/artifact/edge/carrier claims as admitted delivery evidence"
    );
    assert_eq!(
        initial_sender.semantic_request_identity_ref(),
        audit.request_identity_ref(),
        "the independently observed original sender, not the unadmitted retained frame, carries source-bound request identity"
    );
    assert_eq!(audit.admission_session_generation(), 2);
    match audit.owner_outcome() {
        Some(I3LocalnetLateIngressOwnerOutcome::CarrierAdmissionRejected {
            owner_serve_count,
            owner_mutation_count,
        }) => {
            assert_eq!(*owner_serve_count, 0);
            assert_eq!(*owner_mutation_count, 0);
        }
        Some(I3LocalnetLateIngressOwnerOutcome::Admitted { .. }) => {
            panic!("a G2-revoked held frame must not receive an owner serve or reply")
        }
        None => {
            panic!("the actual current-authority rejection must retain its typed owner outcome")
        }
    }
    assert_eq!(
        audit.requester_outcome(),
        I3LocalnetLateIngressRequesterOutcome::PendingReplyOrReceiptNotObserved
    );
    assert_eq!(
        audit.outbound_request_frame_write_count(),
        1,
        "authority rejection must not trigger a request resend or a second source frame write"
    );
    assert_eq!(
        audit.lifecycle_provenance(),
        I3LocalnetLateIngressLifecycleProvenance::M9AdmittedLifecycle
    );
    assert_eq!(audit.m9_lifecycle_source_derived(), Some(false));
    assert_eq!(
        audit.registered_owner_ack_reader_outcome(),
        expected_ack_reader
    );
    assert_eq!(audit.parent_publication(), expected_publication);
    assert_eq!(
        audit.parent_publication_commit_count(),
        expected_publication_commit_count,
        "publication count must come from the parent coordinator's actual transition, not the selected profile"
    );
    assert_eq!(
        audit.nonregistered_ack_input_disposition(),
        expected_nonregistered_ack_input_disposition,
        "the audit must report the actual nonregistered ACK input disposition, not merely the selected late-ingress profile"
    );
    assert_eq!(
        audit.nonregistered_ack_input_count(),
        expected_nonregistered_ack_input_count,
        "the audit must retain the actual number of nonregistered ACK inputs observed on child stdout"
    );
    assert_late_ingress_child_sessions(audit, I3LocalnetChildTerminalOutcome::HandledDeliveryFault);
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
fn i3_3_disconnect_before_request_carrier_write_is_unavailable_without_remote_admission() {
    let error = run_i3_process_localnet(
        canonical_request()
            .with_fault_profile(I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite),
    )
    .expect_err("a faulted localnet run must not return the I3-2 positive completion");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::DeliveryUnavailable);
    let fault = error
        .fault_audit()
        .expect("a formed source-derived request fault must retain its observer-safe audit");
    assert_eq!(
        fault.profile(),
        I3LocalnetFaultProfile::DisconnectBeforeRequestCarrierWrite
    );
    assert!(
        !fault.semantic_success(),
        "a handled pre-write fault is not semantic success even if cleanup reaps naturally"
    );
    assert!(
        !fault.request_identity_ref().is_empty(),
        "the requester must retain the original source-derived request identity"
    );
    assert_eq!(
        fault.requester_observation(),
        I3LocalnetRequesterFaultObservation::RequestCarrierWriteNotAttempted,
        "the injected pre-write control must establish no request bytes were written"
    );
    match fault.remote_admission() {
        Some(I3LocalnetRemoteAdmissionEvidence::NoAdmission {
            owner_serve_count,
            owner_mutation_count,
        }) => {
            assert_eq!(*owner_serve_count, 0);
            assert_eq!(*owner_mutation_count, 0);
            // NoAdmission deliberately carries no received-frame/request
            // identity: the requester identity above is not a fabricated
            // remote delivery record.
        }
        Some(I3LocalnetRemoteAdmissionEvidence::Admitted { .. }) => panic!(
            "a request-carrier write that was not attempted cannot be reported as remote admission"
        ),
        None => panic!(
            "the normal controlled pre-write schedule must retain its actual owner NoAdmission report; \
             remote unknown belongs to a separate suppress-audit falsifier"
        ),
    }
    assert_handled_delivery_fault_lifecycle(&error.rejection_audit());
}

#[test]
fn i3_3_disconnect_after_remote_admission_is_request_bound_ambiguity_not_false_success() {
    let error = run_i3_process_localnet(
        canonical_request()
            .with_fault_profile(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission),
    )
    .expect_err("a post-admission delivery loss must not return the I3-2 positive completion");

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::AmbiguousDelivery);
    let fault = error
        .fault_audit()
        .expect("a post-admission fault must retain its original request-bound audit");
    assert_eq!(
        fault.profile(),
        I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission
    );
    assert!(
        !fault.semantic_success(),
        "a missing reply or receipt is never false semantic success"
    );
    let request_identity = fault.request_identity_ref();
    assert!(
        !request_identity.is_empty(),
        "post-admission ambiguity remains bound to the original source-derived request"
    );
    assert_eq!(
        fault.requester_observation(),
        I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved,
        "the requester records the missing reply/receipt without inferring whether owner admission happened"
    );

    match fault.remote_admission() {
        Some(I3LocalnetRemoteAdmissionEvidence::Admitted {
            request_receive,
            owner_serve_count,
            owner_mutation_count,
            owner_serve_occurrence_ref,
            owner_write_occurrence_ref,
        }) => {
            assert_eq!(*owner_serve_count, 1);
            assert_eq!(*owner_mutation_count, 1);
            assert!(!owner_serve_occurrence_ref.is_empty());
            assert!(
                owner_write_occurrence_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.is_empty()),
                "an admitted owner mutation retains its own observer-safe occurrence"
            );
            assert_delivery_matches_contract(
                request_receive,
                &active_contract("owner-request"),
                request_identity,
                None,
            );
        }
        Some(I3LocalnetRemoteAdmissionEvidence::NoAdmission { .. }) => {
            panic!("a post-admission fault must not manufacture a known no-admission verdict")
        }
        None => panic!(
            "the normal controlled post-admission schedule must join the actual owner report; \
             remote unknown belongs to a separate suppress-audit falsifier"
        ),
    }
    let lifecycle = error.rejection_audit();
    assert_eq!(
        lifecycle.stage(),
        I3LocalnetFailureStage::AfterRemoteAdmission,
        "only the validated owner admission record may refine the requester's missing-reply observation"
    );
    assert_handled_delivery_fault_lifecycle(&lifecycle);
}

#[test]
fn i3_3_missing_post_admission_owner_audit_is_ambiguous_unknown_not_lifecycle_rejection() {
    let error = run_i3_process_localnet(canonical_request().with_fault_profile(
        I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit,
    ))
    .expect_err("a post-admission fault with no owner report must not return I3-2 completion");

    assert_eq!(
        error.kind(),
        I3LocalnetRunErrorKind::AmbiguousDelivery,
        "the requester cannot infer remote admission from a missing reply or suppressed owner audit"
    );
    let fault = error
        .fault_audit()
        .expect("the requester formed a source-derived request before the selected fault");
    assert_eq!(
        fault.profile(),
        I3LocalnetFaultProfile::DisconnectAfterRemoteAdmissionSuppressOwnerAudit
    );
    assert!(
        !fault.semantic_success(),
        "a handled ambiguous delivery is never semantic success"
    );
    assert!(
        !fault.request_identity_ref().is_empty(),
        "the requester retains the original source-derived request identity"
    );
    assert_eq!(
        fault.requester_observation(),
        I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved
    );
    assert!(
        fault.remote_admission().is_none(),
        "a deliberately suppressed owner report is remote unknown, never a fabricated zero-mutation verdict"
    );

    let lifecycle = error.rejection_audit();
    assert_eq!(
        lifecycle.stage(),
        I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
        "a deliberately suppressed owner report leaves only the requester-side missing-reply observation"
    );
    assert_eq!(lifecycle.spawned_child_count(), 2);
    assert!(lifecycle.all_children_reaped());
    assert!(lifecycle.no_orphan_child_pids());
    assert!(
        lifecycle.child_terminal_event_count() < lifecycle.spawned_child_count(),
        "the suppress-audit profile must not be rejected merely because process B supplied no terminal owner audit"
    );
    assert!(lifecycle.observer_safe());
}

#[test]
fn i3_3_normal_post_admission_profile_rejects_an_unexpected_owner_terminal() {
    let error = run_i3_process_localnet(
        canonical_request()
            .with_fault_profile(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission)
            .with_fault_audit_falsifier(
                I3LocalnetFaultAuditFalsifier::ReplaceOwnerFaultWithRejectedTerminal,
            ),
    )
    .expect_err(
        "a post-admission schedule with an unexpected owner terminal must not become ambiguity",
    );

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
    assert!(
        error.fault_audit().is_none(),
        "an unexpected owner terminal is an explicit lifecycle rejection, not remote unknown evidence"
    );
    let lifecycle = error.rejection_audit();
    assert_eq!(
        lifecycle.stage(),
        I3LocalnetFailureStage::AfterRemoteAdmission,
        "the unexpected owner terminal arrives only after the actual remote admission"
    );
    assert_eq!(lifecycle.child_owner_starts(), 1);
    assert_eq!(lifecycle.semantic_admission_count(), 1);
    assert_eq!(lifecycle.owner_mutation_count(), 1);
    assert_eq!(lifecycle.child_terminal_event_count(), 2);
    assert!(
        lifecycle
            .child_terminal_events()
            .iter()
            .any(|event| event.outcome() == I3LocalnetChildTerminalOutcome::Rejected),
        "the rejection audit must retain the actual unexpected owner terminal"
    );
    assert!(lifecycle.all_children_reaped());
    assert!(lifecycle.no_orphan_child_pids());
    assert!(lifecycle.observer_safe());
}

#[test]
fn i3_3_malformed_owner_audit_contract_is_rejected_without_validated_remote_evidence() {
    for (falsifier, label) in [
        (
            I3LocalnetFaultAuditFalsifier::MutateOwnerRequestReceiveCoreRef,
            "core-ref",
        ),
        (
            I3LocalnetFaultAuditFalsifier::SetOwnerRequestReceiveLinkedRequestIdentity,
            "linked-request-identity",
        ),
        (
            I3LocalnetFaultAuditFalsifier::ClearOwnerRequestReceiveCarrierRef,
            "carrier-ref",
        ),
        (
            I3LocalnetFaultAuditFalsifier::ClearOwnerRequestReceiveNetworkOccurrenceRef,
            "network-occurrence-ref",
        ),
    ] {
        let error = run_i3_process_localnet(
            canonical_request()
                .with_fault_profile(I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission)
                .with_fault_audit_falsifier(falsifier),
        )
        .expect_err(
            "a malformed owner report after actual admission must not return I3-2 completion",
        );

        assert_eq!(
            error.kind(),
            I3LocalnetRunErrorKind::AmbiguousDelivery,
            "{label} corruption leaves requester delivery ambiguous rather than successful"
        );
        let fault = error
            .fault_audit()
            .expect("the requester formed a source-derived request before the selected fault");
        assert_eq!(
            fault.profile(),
            I3LocalnetFaultProfile::DisconnectAfterRemoteAdmission
        );
        assert!(
            !fault.semantic_success(),
            "a missing reply remains non-success even when the owner report itself is rejected"
        );
        assert!(
            !fault.request_identity_ref().is_empty(),
            "the requester retains its source-derived identity while the remote report is rejected"
        );
        assert_eq!(
            fault.requester_observation(),
            I3LocalnetRequesterFaultObservation::ReplyOrReceiptNotObserved
        );
        assert!(
            fault.remote_admission().is_none(),
            "{label} corruption must not escape as validated remote evidence"
        );
        assert_eq!(
            fault.remote_evidence_rejection(),
            Some(I3LocalnetRemoteEvidenceRejection::ProvenanceMismatch),
            "the supervisor must identify the rejected observer report rather than silently treating it as no admission"
        );

        let lifecycle = error.rejection_audit();
        assert_eq!(
            lifecycle.stage(),
            I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
            "{label} corruption cannot refine requester-side missing-reply evidence into remote admission"
        );
        assert_handled_delivery_fault_lifecycle(&lifecycle);
        assert_eq!(lifecycle.aggregate_semantic_admission_count(), 1);
        assert_eq!(lifecycle.aggregate_owner_mutation_count(), 1);
    }
}

#[test]
fn i3_3_reconnect_before_initial_carrier_write_reuses_the_original_request_once() {
    let run = run_i3_process_localnet(
        canonical_request().with_retry_profile(I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite),
    )
    .expect("a pre-write reconnect must deliver the retained original request once and consume one receipt");

    let retry = run
        .retry_audit()
        .expect("the selected retry profile must retain its separate observer-safe retry audit");
    assert_eq!(
        retry.profile(),
        I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite
    );
    assert!(!retry.request_identity_ref().is_empty());
    assert!(retry.original_request_succeeded());
    assert_eq!(
        retry.requester_outcome(),
        I3LocalnetRetryRequesterOutcome::ReceiptConsumed
    );
    assert!(!retry.first_session_carrier_write_attempted());
    assert!(
        retry.initial_request_delivery().is_none(),
        "no request delivery record may be fabricated for the pre-write session"
    );
    assert!(
        retry.initial_owner_admission().is_none(),
        "no owner admission may be inferred when the first request write never began"
    );
    assert!(retry.evidence_rejection().is_none());

    let request_contract = active_contract("owner-request");
    let reply_contract = active_contract("owner-reply-receipt");
    let reconnect_request = retry.reconnect_request_delivery();
    assert_delivery_matches_contract(
        reconnect_request,
        &request_contract,
        retry.request_identity_ref(),
        None,
    );
    assert!(!reconnect_request.candidate_commitment_ref().is_empty());
    match retry.reconnect_owner_outcome() {
        Some(I3LocalnetReconnectOwnerOutcome::Replied {
            owner_serve_count,
            owner_mutation_count,
            owner_serve_occurrence_ref,
            owner_write_occurrence_ref,
        }) => {
            assert_eq!(*owner_serve_count, 1);
            assert_eq!(*owner_mutation_count, 1);
            assert!(!owner_serve_occurrence_ref.is_empty());
            assert!(
                owner_write_occurrence_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.is_empty())
            );
        }
        Some(I3LocalnetReconnectOwnerOutcome::DuplicateRequestRejected { .. }) => {
            panic!(
                "the first actual request delivery must receive one owner reply, not a duplicate rejection"
            )
        }
        None => {
            panic!("the completed pre-write retry must retain its actual reconnect owner outcome")
        }
    }

    assert_retry_child_sessions(retry, I3LocalnetChildTerminalOutcome::Completed);
    let requester = run
        .child(I3LocalnetChildSlot::ProcessA)
        .expect("the retry run retains its actual requester child");
    let owner = run
        .child(I3LocalnetChildSlot::ProcessB)
        .expect("the retry run retains its actual owner child");
    assert_ne!(requester.pid(), owner.pid());
    assert!(requester.reaped() && owner.reaped());
    assert!(!requester.was_force_killed() && !owner.was_force_killed());

    let execution = run.execution_audit();
    assert_eq!(execution.generated_request_count(), 1);
    assert_eq!(execution.remote_owner_serve_count(), 1);
    assert_eq!(execution.remote_owner_write_count(), 1);
    assert_eq!(execution.generated_reply_count(), 1);
    assert_eq!(execution.requester_local_receipt_count(), 1);
    assert_eq!(execution.network_receipt_frame_count(), 0);

    let deliveries = run.observer_safe_trace().actual_delivery_records();
    let request_send = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::RequestSend);
    let request_receive = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::RequestReceive);
    let reply_send = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::ReplySend);
    let reply_receive = delivery_for_phase(deliveries, I3LocalnetDeliveryPhase::ReplyReceive);
    assert_delivery_matches_contract(
        request_send,
        &request_contract,
        retry.request_identity_ref(),
        None,
    );
    assert_delivery_matches_contract(
        request_receive,
        &request_contract,
        retry.request_identity_ref(),
        None,
    );
    assert_delivery_matches_contract(
        reply_send,
        &reply_contract,
        retry.request_identity_ref(),
        Some(retry.request_identity_ref()),
    );
    assert_delivery_matches_contract(
        reply_receive,
        &reply_contract,
        retry.request_identity_ref(),
        Some(retry.request_identity_ref()),
    );
    assert_delivery_semantics_match(request_send, reconnect_request);
    assert_delivery_semantics_match(request_send, request_receive);
    assert_delivery_semantics_match(reply_send, reply_receive);
    assert_eq!(
        request_send.candidate_commitment_ref(),
        reconnect_request.candidate_commitment_ref(),
        "the trace and retry audit must retain the adapter-computed sender commitment for one exact reconnect request"
    );
}

#[test]
fn i3_3_reconnect_after_owner_admission_retains_requester_unknown_and_rejects_the_exact_duplicate()
{
    let error = run_i3_process_localnet(
        canonical_request()
            .with_retry_profile(I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply),
    )
    .expect_err(
        "a retry after actual owner admission but before its reply must retain requester ambiguity",
    );

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::AmbiguousDelivery);
    assert!(error.fault_audit().is_none());
    let retry = error.retry_audit().expect(
        "the selected retry profile must retain a retry audit separate from the fault audit",
    );
    assert_eq!(
        retry.profile(),
        I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply
    );
    assert!(!retry.request_identity_ref().is_empty());
    assert!(!retry.original_request_succeeded());
    assert_eq!(
        retry.requester_outcome(),
        I3LocalnetRetryRequesterOutcome::PendingReplyOrReceiptNotObserved
    );
    assert!(retry.first_session_carrier_write_attempted());
    assert!(retry.evidence_rejection().is_none());

    let request_contract = active_contract("owner-request");
    let initial_request = retry
        .initial_request_delivery()
        .expect("the first post-admission session must retain its actual request delivery");
    let reconnect_request = retry.reconnect_request_delivery();
    assert_delivery_matches_contract(
        initial_request,
        &request_contract,
        retry.request_identity_ref(),
        None,
    );
    assert_delivery_matches_contract(
        reconnect_request,
        &request_contract,
        retry.request_identity_ref(),
        None,
    );
    assert_delivery_semantics_match(initial_request, reconnect_request);
    assert_ne!(
        initial_request.network_occurrence_ref(),
        reconnect_request.network_occurrence_ref(),
        "the two actual session deliveries retain distinct network occurrences"
    );
    assert!(!initial_request.candidate_commitment_ref().is_empty());
    assert!(!reconnect_request.candidate_commitment_ref().is_empty());
    assert_ne!(
        initial_request.candidate_commitment_ref(),
        reconnect_request.candidate_commitment_ref(),
        "the same exact request bytes must retain distinct session-scoped commitments across generations"
    );
    assert_initial_retry_owner_admission(retry, &request_contract, retry.request_identity_ref());

    match retry.reconnect_owner_outcome() {
        Some(I3LocalnetReconnectOwnerOutcome::DuplicateRequestRejected {
            candidate_commitment_ref,
            network_occurrence_ref,
            owner_serve_count,
            owner_mutation_count,
        }) => {
            assert_eq!(
                candidate_commitment_ref,
                reconnect_request.candidate_commitment_ref(),
                "duplicate evidence must join the actual reconnect sender commitment, not only an unrelated nonempty reference"
            );
            assert!(!candidate_commitment_ref.is_empty());
            assert!(!network_occurrence_ref.is_empty());
            assert_eq!(*owner_serve_count, 1);
            assert_eq!(*owner_mutation_count, 1);
        }
        Some(I3LocalnetReconnectOwnerOutcome::Replied { .. }) => {
            panic!("the exact retry after owner admission must not produce a second reply")
        }
        None => panic!("the exact reconnect duplicate must retain its typed owner outcome"),
    }

    assert_retry_child_sessions(retry, I3LocalnetChildTerminalOutcome::HandledDeliveryFault);
    let lifecycle = error.rejection_audit();
    assert_handled_delivery_fault_lifecycle(&lifecycle);
    assert_no_rejected_child_transport_observation(&lifecycle);
    assert!(lifecycle.requester_pending_request_is_retained());
    assert_eq!(
        lifecycle.child_owner_starts(),
        1,
        "the post-admission retry failure must retain the actual owner start"
    );
    assert_eq!(lifecycle.aggregate_semantic_admission_count(), 1);
    assert_eq!(lifecycle.aggregate_owner_mutation_count(), 1);
}

#[test]
fn i3_3_reconnect_duplicate_evidence_requires_the_actual_sender_commitment() {
    for (falsifier, expected_rejection, label) in [
        (
            I3LocalnetRetryAuditFalsifier::MutateReconnectSenderCandidateCommitment,
            I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMismatch,
            "mismatched",
        ),
        (
            I3LocalnetRetryAuditFalsifier::ClearReconnectSenderCandidateCommitment,
            I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMissing,
            "missing",
        ),
        (
            I3LocalnetRetryAuditFalsifier::UseInitialSessionCandidateCommitmentForReconnect,
            I3LocalnetRetryEvidenceRejection::ReconnectAttemptCommitmentMismatch,
            "initial-session",
        ),
    ] {
        let error = run_i3_process_localnet(
            canonical_request()
                .with_retry_profile(I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply)
                .with_retry_audit_falsifier(falsifier),
        )
        .expect_err(
            "observer-only commitment corruption after the real retry must preserve requester uncertainty",
        );

        assert_eq!(error.kind(), I3LocalnetRunErrorKind::AmbiguousDelivery);
        let retry = error.retry_audit().expect(
            "a retry profile with observer corruption must retain typed retry evidence rather than fault evidence",
        );
        assert!(!retry.original_request_succeeded());
        assert_eq!(
            retry.requester_outcome(),
            I3LocalnetRetryRequesterOutcome::PendingReplyOrReceiptNotObserved
        );
        assert_eq!(retry.evidence_rejection(), Some(expected_rejection));
        assert!(
            retry.reconnect_owner_outcome().is_none(),
            "{label} sender commitment evidence must not publish a duplicate-owner conclusion"
        );

        let request_contract = active_contract("owner-request");
        let initial_request = retry.initial_request_delivery().expect(
            "the actual first post-admission request remains observed under observer corruption",
        );
        let reconnect_request = retry.reconnect_request_delivery();
        assert_delivery_matches_contract(
            initial_request,
            &request_contract,
            retry.request_identity_ref(),
            None,
        );
        assert_delivery_matches_contract(
            reconnect_request,
            &request_contract,
            retry.request_identity_ref(),
            None,
        );
        assert_delivery_semantics_match(initial_request, reconnect_request);
        assert_ne!(
            initial_request.network_occurrence_ref(),
            reconnect_request.network_occurrence_ref()
        );
        assert_initial_retry_owner_admission(
            retry,
            &request_contract,
            retry.request_identity_ref(),
        );
        assert_retry_child_sessions(retry, I3LocalnetChildTerminalOutcome::HandledDeliveryFault);

        let lifecycle = error.rejection_audit();
        assert_handled_delivery_fault_lifecycle(&lifecycle);
        assert_no_rejected_child_transport_observation(&lifecycle);
        assert!(lifecycle.requester_pending_request_is_retained());
        assert_eq!(
            lifecycle.child_owner_starts(),
            1,
            "{label} observer corruption must not erase the actual owner start"
        );
        assert_eq!(lifecycle.aggregate_semantic_admission_count(), 1);
        assert_eq!(lifecycle.aggregate_owner_mutation_count(), 1);
    }
}

#[test]
fn i3_3_reconnect_requester_binding_corruption_rejects_retry_evidence_without_an_owner_conclusion()
{
    let error = run_i3_process_localnet(
        canonical_request()
            .with_retry_profile(I3LocalnetRetryProfile::ReconnectAfterOwnerAdmissionBeforeReply)
            .with_retry_audit_falsifier(
                I3LocalnetRetryAuditFalsifier::MutateReconnectRequesterBindingRef,
            ),
    )
    .expect_err(
        "a nonempty reconnect requester-binding mismatch must reject observer evidence after the real retry",
    );

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::LifecycleRejected);
    assert!(
        error.retry_audit().is_none(),
        "a malformed requester binding must not publish an accepted retry audit or owner conclusion"
    );
    let lifecycle = error.rejection_audit();
    assert_eq!(
        lifecycle.stage(),
        I3LocalnetFailureStage::LifecycleEvidenceRejected,
        "a bad observer binding must be typed lifecycle evidence rejection, not remote admission"
    );
    assert_handled_delivery_fault_lifecycle(&lifecycle);
    assert_no_rejected_child_transport_observation(&lifecycle);
    assert_eq!(lifecycle.child_owner_starts(), 1);
    assert!(lifecycle.requester_pending_request_is_retained());
    assert_eq!(lifecycle.aggregate_semantic_admission_count(), 1);
    assert_eq!(lifecycle.aggregate_owner_mutation_count(), 1);
}

#[test]
fn i3_3_retry_on_a_verified_initial_session_is_a_local_attempt_rejection_not_peer_failure() {
    let error = run_i3_process_localnet(
        canonical_request()
            .with_retry_profile(I3LocalnetRetryProfile::ReconnectBeforeInitialCarrierWrite)
            .with_retry_falsifier(I3LocalnetRetryFalsifier::RetryOnInitialVerifiedSession),
    )
    .expect_err("a reconnect retry cannot begin on the fully verified initial session");

    let lifecycle = error.rejection_audit();
    assert_eq!(
        lifecycle.adapter_rejection_kind(),
        Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected),
        "a wrong session generation is local attempt misuse after peer verification, not a peer-binding failure"
    );
    assert_ne!(
        lifecycle.adapter_rejection_kind(),
        Some(I3LocalnetAdapterRejectionKind::PeerBindingRejected)
    );
    assert!(
        error.retry_audit().is_none(),
        "a generation-one misuse occurs before reconnect transport, so it must not fabricate a mandatory two-session retry audit"
    );
    assert_eq!(
        lifecycle.fixed_child_control_descriptors_preserved(),
        Some(true)
    );
    assert_eq!(
        lifecycle.quic_handshake_count(),
        Some(1),
        "the local generation-one rejection must retain its one actual initial-session handshake"
    );
    assert_eq!(lifecycle.quic_certificate_initializations(), Some(1));
    assert!(
        lifecycle.requester_first_session_peer_spki_verified(),
        "the local attempt guard must run only after actual first-session peer-SPKI verification"
    );
    assert!(
        lifecycle.requester_first_session_reciprocal_preface_verified(),
        "the local attempt guard must run only after actual first-session reciprocal-preface verification"
    );
    assert!(
        lifecycle.requester_pending_request_is_retained(),
        "a rejected local attempt must leave the original requester pending"
    );
    assert_eq!(
        lifecycle.semantic_admission_count(),
        0,
        "the wrong-session guard must run before owner semantic admission"
    );
    assert_eq!(
        lifecycle.owner_mutation_count(),
        0,
        "the wrong-session guard must run before owner mutation"
    );
}

#[test]
fn i3_3_late_session_one_ingress_under_g1_is_admitted_once_after_verified_session_two() {
    let run = run_i3_process_localnet(
        canonical_request().with_late_ingress_profile(
            I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect,
        ),
    )
    .expect(
        "a real complete session-one frame held before semantic admission must admit once under unchanged G1 on session two",
    );

    let late = run
        .late_ingress_audit()
        .expect("the selected late-ingress schedule must retain a dedicated observer-safe audit");
    assert_eq!(
        late.profile(),
        I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect
    );
    assert!(!late.request_identity_ref().is_empty());
    let request_contract = active_contract("owner-request");
    let initial_sender = assert_late_ingress_sender_and_retained_frame(late, &request_contract);
    let retained = late.retained_session_one_ingress();
    let admitted = late.late_admission_delivery().expect(
        "unchanged G1 must publish a full delivery record only after the retained frame is semantically admitted",
    );
    assert_delivery_matches_contract(
        admitted,
        &request_contract,
        late.request_identity_ref(),
        None,
    );
    assert_delivery_semantics_match(initial_sender, admitted);
    assert_eq!(
        admitted.candidate_commitment_ref(),
        retained.candidate_commitment_ref(),
        "the successful session-two admission must retain the original session-one frame commitment"
    );
    assert_eq!(
        admitted.network_occurrence_ref(),
        retained.network_occurrence_ref(),
        "admission of one held frame must not mint or relabel a second receiver occurrence"
    );
    assert_eq!(late.admission_session_generation(), 2);
    match late.owner_outcome() {
        Some(I3LocalnetLateIngressOwnerOutcome::Admitted {
            owner_serve_count,
            owner_mutation_count,
            ..
        }) => {
            assert_eq!(*owner_serve_count, 1);
            assert_eq!(*owner_mutation_count, 1);
        }
        Some(I3LocalnetLateIngressOwnerOutcome::CarrierAdmissionRejected { .. }) => {
            panic!("unchanged G1 must admit the held original frame rather than reject it")
        }
        None => panic!("a successful late admission must retain its actual owner outcome"),
    }
    assert_eq!(
        late.requester_outcome(),
        I3LocalnetLateIngressRequesterOutcome::ReceiptConsumed
    );
    assert_eq!(late.outbound_request_frame_write_count(), 1);
    assert_eq!(
        late.lifecycle_provenance(),
        I3LocalnetLateIngressLifecycleProvenance::NotSelected
    );
    assert_eq!(late.m9_lifecycle_source_derived(), None);
    assert_eq!(
        late.registered_owner_ack_reader_outcome(),
        I3LocalnetLateIngressAckReaderOutcome::NotSelected
    );
    assert_eq!(
        late.parent_publication(),
        I3LocalnetLateIngressParentPublication::NoPrestageSelected
    );
    assert_eq!(
        late.nonregistered_ack_input_disposition(),
        I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved
    );
    assert_eq!(late.nonregistered_ack_input_count(), 0);
    assert_late_ingress_child_sessions(late, I3LocalnetChildTerminalOutcome::Completed);

    let execution = run.execution_audit();
    assert_eq!(execution.generated_request_count(), 1);
    assert_eq!(execution.remote_owner_serve_count(), 1);
    assert_eq!(execution.remote_owner_write_count(), 1);
    assert_eq!(execution.generated_reply_count(), 1);
    assert_eq!(execution.requester_local_receipt_count(), 1);
    assert_eq!(execution.network_receipt_frame_count(), 0);
    let requester = run
        .child(I3LocalnetChildSlot::ProcessA)
        .expect("the late-ingress run retains its actual requester child");
    let owner = run
        .child(I3LocalnetChildSlot::ProcessB)
        .expect("the late-ingress run retains its actual owner child");
    assert_ne!(requester.pid(), owner.pid());
    assert!(requester.reaped() && owner.reaped());
    assert!(!requester.was_force_killed() && !owner.was_force_killed());
}

#[test]
fn i3_3_late_ingress_repeat_acquisition_rejects_before_io_without_discarding_the_original_pending_frame()
 {
    let run = run_i3_process_localnet(
        canonical_request()
            .with_late_ingress_profile(
                I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect,
            )
            .with_late_ingress_falsifier(
                I3LocalnetLateIngressFalsifier::RepeatRetainedIngressAcquisition,
            ),
    )
    .expect(
        "a local repeat-acquisition rejection after one real held frame must preserve the original pending ingress for the normal G1 receipt path",
    );

    let late = run
        .late_ingress_audit()
        .expect("the actual one-frame G1 schedule retains its observer-safe late-ingress audit");
    assert_eq!(
        late.profile(),
        I3LocalnetLateIngressProfile::HoldSessionOneIngressAcrossVerifiedReconnect
    );
    assert_eq!(
        late.retained_ingress_repeat_acquisition_rejection(),
        Some(I3LocalnetAdapterRejectionKind::LocalAttemptRejected),
        "the second production acquisition must fail locally before it can read, reserve, decode, or admit another frame"
    );
    let request_contract = active_contract("owner-request");
    let initial_sender = assert_late_ingress_sender_and_retained_frame(late, &request_contract);
    let retained = late.retained_session_one_ingress();
    let admitted = late.late_admission_delivery().expect(
        "the first opaque pending ingress remains the only frame admitted after the repeat rejection",
    );
    assert_delivery_matches_contract(
        admitted,
        &request_contract,
        late.request_identity_ref(),
        None,
    );
    assert_delivery_semantics_match(initial_sender, admitted);
    assert_eq!(
        admitted.candidate_commitment_ref(),
        retained.candidate_commitment_ref(),
        "the eventual admission must consume the original session-one commitment, not a second acquisition"
    );
    assert_eq!(
        admitted.network_occurrence_ref(),
        retained.network_occurrence_ref(),
        "the eventual admission must consume the original receiver occurrence without a second I/O reservation"
    );
    assert_eq!(late.admission_session_generation(), 2);
    match late.owner_outcome() {
        Some(I3LocalnetLateIngressOwnerOutcome::Admitted {
            owner_serve_count,
            owner_mutation_count,
            ..
        }) => {
            assert_eq!(*owner_serve_count, 1);
            assert_eq!(*owner_mutation_count, 1);
        }
        Some(I3LocalnetLateIngressOwnerOutcome::CarrierAdmissionRejected { .. }) => {
            panic!("the preserved original G1 pending frame must admit once")
        }
        None => {
            panic!("the successful original pending frame must retain the actual owner outcome")
        }
    }
    assert_eq!(
        late.requester_outcome(),
        I3LocalnetLateIngressRequesterOutcome::ReceiptConsumed
    );
    assert_eq!(late.outbound_request_frame_write_count(), 1);
    assert_eq!(
        late.lifecycle_provenance(),
        I3LocalnetLateIngressLifecycleProvenance::NotSelected
    );
    assert_eq!(late.m9_lifecycle_source_derived(), None);
    assert_eq!(
        late.registered_owner_ack_reader_outcome(),
        I3LocalnetLateIngressAckReaderOutcome::NotSelected
    );
    assert_eq!(
        late.parent_publication(),
        I3LocalnetLateIngressParentPublication::NoPrestageSelected
    );
    assert_eq!(
        late.nonregistered_ack_input_disposition(),
        I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved
    );
    assert_eq!(late.nonregistered_ack_input_count(), 0);
    assert_late_ingress_child_sessions(late, I3LocalnetChildTerminalOutcome::Completed);

    let execution = run.execution_audit();
    assert_eq!(execution.generated_request_count(), 1);
    assert_eq!(execution.remote_owner_serve_count(), 1);
    assert_eq!(execution.remote_owner_write_count(), 1);
    assert_eq!(execution.generated_reply_count(), 1);
    assert_eq!(execution.requester_local_receipt_count(), 1);
    assert_eq!(execution.network_receipt_frame_count(), 0);
    let requester = run
        .child(I3LocalnetChildSlot::ProcessA)
        .expect("the repeated-acquisition G1 path retains its actual requester child");
    let owner = run
        .child(I3LocalnetChildSlot::ProcessB)
        .expect("the repeated-acquisition G1 path retains its actual owner child");
    assert_ne!(requester.pid(), owner.pid());
    assert!(requester.reaped() && owner.reaped());
    assert!(!requester.was_force_killed() && !owner.was_force_killed());
}

#[test]
fn i3_3_late_session_one_ingress_after_b_installs_g2_is_carrier_admission_rejected() {
    let error = run_i3_process_localnet(
        canonical_request().with_late_ingress_profile(
            I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
        ),
    )
    .expect_err(
        "the exact session-one frame must be revalidated against B's installed G2 before semantic admission",
    );

    let diagnostic_lifecycle = error.rejection_audit();
    let terminal_metadata = diagnostic_lifecycle
        .child_terminal_events()
        .iter()
        .map(|event| {
            (
                event.outcome(),
                event.semantic_admission_count(),
                event.owner_mutation_count(),
                event.observed_exit_status_code(),
                event.was_force_killed(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        error.kind(),
        I3LocalnetRunErrorKind::AmbiguousDelivery,
        "G2 late ingress must retain the actual remote current-authority rejection rather than collapse into generic lifecycle rejection; observer-safe diagnostic: stage={:?}, lifecycle_cause={:?}, owner_starts={}, semantic_admissions={}, owner_mutations={}, aggregate_semantic_admissions={}, aggregate_owner_mutations={}, requester_pending={}, terminals={terminal_metadata:?}, children_reaped={}, no_orphan_pids={}, late_ingress_audit_present={}, late_ingress_evidence_rejection={:?}",
        diagnostic_lifecycle.stage(),
        diagnostic_lifecycle.lifecycle_rejection_cause(),
        diagnostic_lifecycle.child_owner_starts(),
        diagnostic_lifecycle.semantic_admission_count(),
        diagnostic_lifecycle.owner_mutation_count(),
        diagnostic_lifecycle.aggregate_semantic_admission_count(),
        diagnostic_lifecycle.aggregate_owner_mutation_count(),
        diagnostic_lifecycle.requester_pending_request_is_retained(),
        diagnostic_lifecycle.all_children_reaped(),
        diagnostic_lifecycle.no_orphan_child_pids(),
        error.late_ingress_audit().is_some(),
        error.late_ingress_evidence_rejection(),
    );
    let late = error.late_ingress_audit().expect(
        "the late rejection must retain actual schedule and owner outcome evidence without treating rejection as a reply",
    );
    assert_prestaged_late_ingress_rejection(
        late,
        I3LocalnetLateIngressAckReaderOutcome::AcceptedRegisteredOwnerChildFd,
        I3LocalnetLateIngressParentPublication::G2Published,
        1,
        I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved,
        0,
    );

    let lifecycle = error.rejection_audit();
    assert_eq!(
        lifecycle.stage(),
        I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
        "a typed remote rejection keeps the requester original pending rather than becoming a reply/receipt success"
    );
    assert_handled_delivery_fault_lifecycle(&lifecycle);
    assert!(lifecycle.requester_pending_request_is_retained());
    assert_eq!(lifecycle.child_owner_starts(), 1);
    assert_eq!(lifecycle.aggregate_semantic_admission_count(), 0);
    assert_eq!(lifecycle.aggregate_owner_mutation_count(), 0);
}

#[test]
fn i3_3_late_ingress_terminal_validation_rejects_untrusted_control_or_unauthenticated_admission_observations()
 {
    for (falsifier, corrupted_slot, expected_unauthenticated_admissions, requester_pending, label) in [
        (
            I3LocalnetLateIngressFalsifier::SetOwnerLateIngressTerminalUnauthenticatedAdmissionAndClearTrustedControl,
            I3LocalnetChildSlot::ProcessB,
            1,
            true,
            "owner unauthenticated admission with cleared trusted control",
        ),
        (
            I3LocalnetLateIngressFalsifier::ClearRequesterLateIngressTerminalTrustedControl,
            I3LocalnetChildSlot::ProcessA,
            0,
            false,
            "requester cleared trusted control",
        ),
    ] {
        let error = run_i3_process_localnet(
            canonical_request()
                .with_late_ingress_profile(
                    I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
                )
                .with_late_ingress_falsifier(falsifier),
        )
        .expect_err(
            "a mutated observer-only terminal record must not validate a late-ingress semantic conclusion",
        );

        assert_eq!(
            error.kind(),
            I3LocalnetRunErrorKind::LifecycleRejected,
            "{label} is a terminal-evidence rejection, never a remote admission or reply"
        );
        assert!(
            error.late_ingress_audit().is_none(),
            "{label} must not publish a validated late-ingress audit or owner conclusion"
        );
        assert_eq!(
            error.late_ingress_evidence_rejection(),
            None,
            "{label} is validated from terminal structure, not a retained-ingress commitment join"
        );

        let lifecycle = error.rejection_audit();
        assert_eq!(
            lifecycle.stage(),
            I3LocalnetFailureStage::LifecycleEvidenceRejected,
            "{label} must retain the explicit terminal-shape validation failure"
        );
        assert_handled_delivery_fault_lifecycle(&lifecycle);
        assert_eq!(lifecycle.child_owner_starts(), 1);
        assert_eq!(lifecycle.aggregate_semantic_admission_count(), 0);
        assert_eq!(lifecycle.aggregate_owner_mutation_count(), 0);
        assert_eq!(
            lifecycle.requester_pending_request_is_retained(),
            requester_pending,
            "{label} may retain pending only when A's independently validated terminal record remains trustworthy"
        );

        let matching = lifecycle
            .child_terminal_events()
            .iter()
            .filter(|event| event.slot() == Some(corrupted_slot))
            .collect::<Vec<_>>();
        assert_eq!(
            matching.len(),
            1,
            "{label} must retain exactly one observer-safe terminal record for the corrupted child slot"
        );
        let corrupted = matching[0];
        assert_eq!(corrupted.trusted_control_consumed(), Some(false));
        assert_eq!(
            corrupted.unauthenticated_semantic_admission_count(),
            Some(expected_unauthenticated_admissions),
            "{label} must preserve the actual corrupted observer fields rather than silently normalizing them"
        );
        assert_eq!(corrupted.semantic_admission_count(), 0);
        assert_eq!(corrupted.owner_mutation_count(), 0);
    }
}

#[test]
fn i3_3_late_ingress_candidate_binding_tamper_rejects_before_any_sender_or_owner_evidence() {
    let error = run_i3_process_localnet(
        canonical_request()
            .with_late_ingress_profile(
                I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
            )
            .with_late_ingress_falsifier(
                I3LocalnetLateIngressFalsifier::GenuineCandidateBindingTamper,
            ),
    )
    .expect_err(
        "a genuine prestaged candidate paired with a mismatched independently trusted binding must reject before B starts",
    );

    assert_eq!(error.kind(), I3LocalnetRunErrorKind::StartBindingRejected);
    assert!(
        error.late_ingress_audit().is_none(),
        "a bootstrap rejection before B is ready cannot fabricate A sender, retained-frame, owner, requester-pending, or ACK evidence"
    );
    let lifecycle = error.rejection_audit();
    assert_eq!(lifecycle.stage(), I3LocalnetFailureStage::BeforeOwnerStart);
    assert_eq!(
        lifecycle.fixed_child_control_descriptors_preserved(),
        Some(true)
    );
    assert_eq!(lifecycle.child_owner_starts(), 0);
    assert_eq!(lifecycle.quic_certificate_initializations(), Some(0));
    assert_eq!(lifecycle.quic_handshake_count(), Some(0));
    assert_eq!(lifecycle.semantic_admission_count(), 0);
    assert_eq!(lifecycle.owner_mutation_count(), 0);
    assert!(
        !lifecycle.requester_pending_request_is_retained(),
        "when A never sends, the harness must not infer requester pending state from the selected candidate tamper"
    );
    assert!(lifecycle.all_children_reaped());
    assert!(lifecycle.no_orphan_child_pids());
    assert!(lifecycle.observer_safe());
}

#[test]
fn i3_3_late_ingress_ack_completion_accepts_only_registered_b_fd_and_observes_actual_a_stdout_candidate()
 {
    for (
        falsifier,
        expected_reader,
        expected_publication,
        expected_publication_commit_count,
        expected_nonregistered_ack_input_disposition,
        expected_nonregistered_ack_input_count,
        label,
    ) in [
        (
            I3LocalnetLateIngressFalsifier::RouteTaintedAckCandidateFromActualAStdout,
            I3LocalnetLateIngressAckReaderOutcome::UntrustedRouteIgnored,
            I3LocalnetLateIngressParentPublication::PublicationIncomplete,
            0,
            I3LocalnetLateIngressNonregisteredAckInputDisposition::RequesterStdoutTaintedCandidateIgnored,
            1,
            "actual-requester-stdout-tainted-candidate",
        ),
        (
            I3LocalnetLateIngressFalsifier::DropBRegisteredAck,
            I3LocalnetLateIngressAckReaderOutcome::LostBeforeParentAcceptance,
            I3LocalnetLateIngressParentPublication::PublicationIncomplete,
            0,
            I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved,
            0,
            "dropped-registered-b",
        ),
        (
            I3LocalnetLateIngressFalsifier::ReplayBRegisteredAck,
            I3LocalnetLateIngressAckReaderOutcome::ReplayRejected,
            I3LocalnetLateIngressParentPublication::G2Published,
            1,
            I3LocalnetLateIngressNonregisteredAckInputDisposition::NotObserved,
            0,
            "replayed-registered-b",
        ),
    ] {
        let error = run_i3_process_localnet(
            canonical_request()
                .with_late_ingress_profile(
                    I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
                )
                .with_late_ingress_falsifier(falsifier),
        )
        .expect_err(
            "each failed or replayed acknowledgement route leaves the original requester without a reply or receipt",
        );

        assert_eq!(
            error.kind(),
            I3LocalnetRunErrorKind::AmbiguousDelivery,
            "{label} must not convert current-authority rejection into a source success"
        );
        let late = error.late_ingress_audit().expect(
            "post-install acknowledgement routing retains actual session-one ingress and session-two current-authority evidence",
        );
        assert_prestaged_late_ingress_rejection(
            late,
            expected_reader,
            expected_publication,
            expected_publication_commit_count,
            expected_nonregistered_ack_input_disposition,
            expected_nonregistered_ack_input_count,
        );

        let lifecycle = error.rejection_audit();
        assert_eq!(
            lifecycle.stage(),
            I3LocalnetFailureStage::RequesterReplyOrReceiptNotObserved,
            "{label} cannot fabricate a reply/receipt from a lifecycle ACK route"
        );
        assert_handled_delivery_fault_lifecycle(&lifecycle);
        assert!(lifecycle.requester_pending_request_is_retained());
        assert_eq!(lifecycle.child_owner_starts(), 1);
        assert_eq!(lifecycle.aggregate_semantic_admission_count(), 0);
        assert_eq!(lifecycle.aggregate_owner_mutation_count(), 0);
    }
}

#[test]
fn i3_3_late_ingress_retained_commitment_evidence_requires_the_exact_initial_sender_commitment() {
    for (falsifier, expected_rejection, label) in [
        (
            I3LocalnetLateIngressFalsifier::ClearRetainedIngressCandidateCommitment,
            I3LocalnetLateIngressEvidenceRejection::RetainedIngressCommitmentMissing,
            "missing",
        ),
        (
            I3LocalnetLateIngressFalsifier::MutateRetainedIngressCandidateCommitment,
            I3LocalnetLateIngressEvidenceRejection::RetainedIngressCommitmentMismatch,
            "mismatched",
        ),
    ] {
        let error = run_i3_process_localnet(
            canonical_request()
                .with_late_ingress_profile(
                    I3LocalnetLateIngressProfile::PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
                )
                .with_late_ingress_falsifier(falsifier),
        )
        .expect_err(
            "missing or mismatched retained-frame commitment evidence must not publish a late-admission owner conclusion",
        );

        assert_eq!(
            error.kind(),
            I3LocalnetRunErrorKind::LifecycleRejected,
            "{label} retained-frame evidence corruption is a typed evidence rejection, not an owner reply or requester success"
        );
        assert_eq!(
            error.late_ingress_evidence_rejection(),
            Some(expected_rejection),
            "{label} must retain the exact failed commitment join rather than silently treating it as no admission"
        );
        assert!(
            error.late_ingress_audit().is_none(),
            "{label} evidence rejection must not fabricate a validated late-ingress audit or owner semantic conclusion"
        );

        let lifecycle = error.rejection_audit();
        assert_eq!(
            lifecycle.stage(),
            I3LocalnetFailureStage::LifecycleEvidenceRejected,
            "{label} must retain the typed observer-join failure rather than reclassifying it as remote admission"
        );
        assert_handled_delivery_fault_lifecycle(&lifecycle);
        assert!(
            lifecycle.requester_pending_request_is_retained(),
            "{label} must preserve the independently source-bound requester pending fact"
        );
        assert_eq!(lifecycle.child_owner_starts(), 1);
        assert_eq!(lifecycle.aggregate_semantic_admission_count(), 0);
        assert_eq!(lifecycle.aggregate_owner_mutation_count(), 0);
    }
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
    assert_eq!(
        audit.fixed_child_control_descriptors_preserved(),
        Some(true)
    );
    assert_eq!(audit.child_owner_starts(), 0);
    assert_eq!(audit.quic_certificate_initializations(), Some(0));
    assert_eq!(audit.quic_handshake_count(), Some(0));
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
