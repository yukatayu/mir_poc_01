use std::time::Duration;

use mirrorea_i3_probe::{
    CandidateCase, CandidateRun, CandidateRunRequest, CredentialDelivery,
    ReceiverChildCanaryEventKind, SemanticCarrier, SemanticRequestSeed, SourceBoundEdge,
    TransportCandidate, TransportCaptureOrigin, build_source_bound_probe, encode_frame,
    run_candidate_inventory_in_child_processes,
};

const ACTIVE_I2_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"
);
const ACTIVE_ATTACK_REQUEST_SEED: &str = "i3-0-candidate-attack-01";

pub const CASE_INVENTORY: [CandidateCase; 9] = [
    CandidateCase::ConnectWithoutSemanticAdmission,
    CandidateCase::DeterministicFragmentedRoundTrip,
    CandidateCase::TruncatedFrame,
    CandidateCase::OversizedFrame,
    CandidateCase::DisconnectBeforeAdmission,
    CandidateCase::DisconnectAfterAdmissionBeforeResult,
    CandidateCase::DuplicateAcrossReconnect,
    CandidateCase::TamperedSemanticAdmissionReference,
    CandidateCase::ObserverSafeEvidence,
];

pub fn active_attack_edge() -> SourceBoundEdge {
    let probe = build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must produce the source-bound attack edge");

    probe
        .owner_request_edge("attack")
        .expect("the accepted I2 projection must generate the attack edge")
        .clone()
}

pub fn active_attack_carrier() -> SemanticCarrier {
    active_attack_edge()
        .bind_semantic_request(SemanticRequestSeed::new(ACTIVE_ATTACK_REQUEST_SEED))
        .expect("the stable candidate seed must bind the retained attack edge")
}

fn active_attack_binding() -> (SemanticCarrier, SourceBoundEdge) {
    let target_edge = active_attack_edge();
    let carrier = target_edge
        .bind_semantic_request(SemanticRequestSeed::new(ACTIVE_ATTACK_REQUEST_SEED))
        .expect("the stable candidate seed must bind the retained attack edge");
    (carrier, target_edge)
}

pub fn run_candidate(candidate: TransportCandidate) -> CandidateRun {
    let (carrier, target_admission_edge) = active_attack_binding();
    let request = CandidateRunRequest::new(candidate, carrier)
        .with_target_admission_edge(target_admission_edge)
        .with_cases(CASE_INVENTORY)
        .with_deadline(Duration::from_secs(15));
    run_candidate_inventory_in_child_processes(request)
        .expect("candidate inventory must complete before the bounded deadline")
}

pub fn assert_common_candidate_contract(run: &CandidateRun, candidate: TransportCandidate) {
    let edge = active_attack_edge();
    let carrier = active_attack_carrier();
    let encoded_frame = encode_frame(&carrier)
        .expect("the stable retained attack carrier must encode before transport comparison");
    let lifecycle = run.process_lifecycle();
    assert!(
        lifecycle.server_and_client_are_distinct_children(),
        "the coordinator is excluded: server and client must be distinct child OS processes"
    );
    assert!(
        lifecycle.deadline_enforced(),
        "child server/client work must have a bounded deadline"
    );
    assert!(
        lifecycle.cleanup_policy_declared(),
        "a normal candidate run may establish only a cleanup policy; forced timeout kill/wait/reap requires dedicated falsifier evidence"
    );
    assert!(
        lifecycle.orphan_cleanup_complete(),
        "no child process may remain after a completed candidate case"
    );
    assert_eq!(
        lifecycle.credential_delivery(),
        CredentialDelivery::InMemoryOrPrivatePipe,
        "certificates/keys must remain in memory or a private child pipe"
    );
    assert!(
        !lifecycle.secret_exposed_via_cli_environment_file_or_log(),
        "keys/certificates must never be placed in CLI, environment, files, or logs"
    );
    assert!(
        !lifecycle.permissive_certificate_verifier_used(),
        "the candidate must not accept every peer certificate"
    );
    assert!(
        !lifecycle.os_trust_store_used(),
        "the bounded probe must not delegate trust to an OS trust store"
    );

    assert_eq!(
        run.rows().iter().map(|row| row.case()).collect::<Vec<_>>(),
        CASE_INVENTORY,
        "both candidates must execute the entire common inventory in the same order"
    );
    for row in run.rows() {
        assert_eq!(row.candidate(), candidate);
        assert!(
            !row.mechanism().is_empty(),
            "candidate-specific mechanism evidence must be explicit"
        );
        assert_eq!(row.program_ref(), edge.program_ref());
        assert_eq!(row.source_ref(), edge.source_ref());
        assert_eq!(row.core_ref(), edge.core_ref());
        assert_eq!(row.source_artifact_ref(), edge.source_artifact_ref());
        assert_eq!(row.target_artifact_ref(), edge.target_artifact_ref());
        assert_eq!(row.edge_ref(), edge.edge_ref());
        assert_eq!(row.request_ref(), carrier.request_identity().as_str());
        assert!(
            !row.occurrence_refs().is_empty(),
            "network occurrences are evidence, distinct from semantic request identity"
        );
        assert!(
            row.occurrence_refs()
                .iter()
                .all(|occurrence| occurrence != carrier.request_identity().as_str()),
            "transport occurrences must not collapse into semantic request identity"
        );
        assert!(row.distinct_os_processes());
        assert!(
            !row.transport_metadata_used_as_authority(),
            "transport metadata must never become authority"
        );
        assert!(row.observer_safe());
        assert!(row.cleanup_complete());
        assert_eq!(
            row.transport_capture_origin(),
            TransportCaptureOrigin::ChildProcessReceive,
            "transport evidence must come from the child that actually received the bytes"
        );
        assert_observer_safe_capture_ref(row.transport_capture_ref());
        assert!(
            !row.retry_initiated(),
            "I3-0 comparison paths must not perform hidden retries"
        );
    }

    assert_row(
        run,
        CandidateCase::ConnectWithoutSemanticAdmission,
        0,
        0,
        false,
        "ConnectedWithoutSemanticAdmission",
    );
    assert_row(
        run,
        CandidateCase::DeterministicFragmentedRoundTrip,
        1,
        1,
        false,
        "Accepted",
    );
    assert_row(
        run,
        CandidateCase::TruncatedFrame,
        0,
        0,
        false,
        "TruncatedFrame",
    );
    assert_row(
        run,
        CandidateCase::OversizedFrame,
        0,
        0,
        false,
        "OversizedFrame",
    );
    assert_row(
        run,
        CandidateCase::DisconnectBeforeAdmission,
        0,
        0,
        false,
        "DisconnectBeforeAdmission",
    );
    assert_row(
        run,
        CandidateCase::DisconnectAfterAdmissionBeforeResult,
        1,
        1,
        false,
        "AmbiguousDelivery",
    );
    assert_row(
        run,
        CandidateCase::TamperedSemanticAdmissionReference,
        0,
        0,
        false,
        "SemanticAdmissionRejected:RetainedContractFingerprintMismatch",
    );
    assert_row(
        run,
        CandidateCase::ObserverSafeEvidence,
        1,
        1,
        false,
        "ObserverSafeEvidence",
    );

    assert_eq!(
        row(run, CandidateCase::DeterministicFragmentedRoundTrip).transport_observed_octets(),
        encoded_frame.len(),
        "the positive child receive must account for the complete encoded frame"
    );
    let positive_replies = row(run, CandidateCase::DeterministicFragmentedRoundTrip)
        .client_child_probe_reply_receipts();
    assert_eq!(
        positive_replies.len(),
        1,
        "the positive path must retain one actual client-child probe reply receipt"
    );
    assert_eq!(
        positive_replies[0].sequence(),
        1,
        "the positive client-child reply receipt must retain its observed sequence"
    );
    assert!(
        positive_replies[0].received_by_client_child(),
        "the positive reply receipt must be captured by the actual client child, not coordinator expectation state"
    );
    assert!(
        !positive_replies[0].receipt_ref().is_empty(),
        "the positive client-child reply receipt must carry an observer-safe reference"
    );
    let truncated_octets = row(run, CandidateCase::TruncatedFrame).transport_observed_octets();
    assert!(
        truncated_octets > 4 && truncated_octets < encoded_frame.len(),
        "truncation evidence must be derived from actual child-received prefix-plus-partial-body bytes"
    );
    assert_eq!(
        row(run, CandidateCase::OversizedFrame).transport_observed_octets(),
        4,
        "an oversized frame is rejected after the child receives only its four-byte length prefix"
    );
    assert_eq!(
        row(run, CandidateCase::DisconnectBeforeAdmission).transport_observed_octets(),
        0,
        "a pre-admission disconnect must not invent a received semantic frame"
    );

    for case in [
        CandidateCase::DeterministicFragmentedRoundTrip,
        CandidateCase::DisconnectAfterAdmissionBeforeResult,
        CandidateCase::DuplicateAcrossReconnect,
        CandidateCase::TamperedSemanticAdmissionReference,
        CandidateCase::ObserverSafeEvidence,
    ] {
        assert_receiver_child_canary_facts(row(run, case));
    }

    let duplicate = row(run, CandidateCase::DuplicateAcrossReconnect);
    assert_eq!(duplicate.semantic_admission_count(), 1);
    assert_eq!(duplicate.handler_count(), 1);
    assert_eq!(
        duplicate.target_contract_authority_revalidation_count(),
        2,
        "both network occurrences must revalidate the target contract/authority profile even though only the first admits and linearizes"
    );
    assert_eq!(duplicate.occurrence_refs().len(), 2);
    assert_ne!(
        duplicate.occurrence_refs()[0],
        duplicate.occurrence_refs()[1]
    );
    assert_eq!(
        duplicate.transport_capture_count(),
        2,
        "the reconnect duplicate must retain two independently child-received frame captures"
    );
    assert!(
        duplicate.stored_decision_returned() || duplicate.typed_outcome() == "DuplicateRequest",
        "a duplicate reconnect must return the stored decision or produce a typed duplicate"
    );
    assert_eq!(
        duplicate.transport_observed_octets(),
        encoded_frame
            .len()
            .checked_mul(2)
            .expect("two private probe frames fit usize"),
        "the duplicate case must retain both receiver-child frame captures"
    );
    let duplicate_replies = duplicate.client_child_probe_reply_receipts();
    assert_eq!(
        duplicate_replies.len(),
        2,
        "a reconnect duplicate must retain the two actual client-child probe reply receipts"
    );
    assert_eq!(duplicate_replies[0].sequence(), 1);
    assert_eq!(duplicate_replies[1].sequence(), 2);
    assert!(
        duplicate_replies
            .iter()
            .all(|reply| reply.received_by_client_child()),
        "both reconnect replies must be actual client-child receipt captures"
    );
    assert!(
        !duplicate_replies[0].receipt_ref().is_empty()
            && !duplicate_replies[1].receipt_ref().is_empty(),
        "client-child reply receipts must expose observer-safe receipt references"
    );
    assert_ne!(
        duplicate_replies[0].receipt_ref(),
        duplicate_replies[1].receipt_ref(),
        "two reply receipts must remain distinct network observations"
    );
    assert!(
        !duplicate_replies[0].stored_decision_ref().is_empty(),
        "the first reply must identify its stored decision without exposing a payload"
    );
    assert_eq!(
        duplicate_replies[0].stored_decision_ref(),
        duplicate_replies[1].stored_decision_ref(),
        "the duplicate reply must return the exact stored decision reached by the first occurrence"
    );
    assert_receiver_child_canary_sequence(
        duplicate,
        &[
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionAccepted,
            ReceiverChildCanaryEventKind::ProbeHandlerLinearized,
            ReceiverChildCanaryEventKind::DecisionStored,
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionRevalidated,
            ReceiverChildCanaryEventKind::StoredDecisionHit,
        ],
    );
    assert_receiver_child_canary_sequence(
        row(run, CandidateCase::DisconnectAfterAdmissionBeforeResult),
        &[
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionAccepted,
            ReceiverChildCanaryEventKind::ProbeHandlerLinearized,
            ReceiverChildCanaryEventKind::DecisionStored,
            ReceiverChildCanaryEventKind::ResultPathLost,
            ReceiverChildCanaryEventKind::AmbiguousDelivery,
        ],
    );
}

pub fn row(run: &CandidateRun, case: CandidateCase) -> &mirrorea_i3_probe::EvidenceRow {
    run.row(case)
        .unwrap_or_else(|| panic!("candidate inventory omitted {case:?}"))
}

fn assert_row(
    run: &CandidateRun,
    case: CandidateCase,
    semantic_admission_count: usize,
    handler_count: usize,
    stored_decision_returned: bool,
    typed_outcome: &str,
) {
    let row = row(run, case);
    assert_eq!(row.semantic_admission_count(), semantic_admission_count);
    assert_eq!(row.handler_count(), handler_count);
    assert_eq!(row.stored_decision_returned(), stored_decision_returned);
    assert_eq!(row.typed_outcome(), typed_outcome);
}

fn assert_observer_safe_capture_ref(capture_ref: &str) {
    let digest = capture_ref
        .strip_prefix("i3-0-transport-capture-sha256-v1:")
        .expect(
            "observer evidence must expose a namespaced transport-capture digest, never raw bytes",
        );
    assert_eq!(
        digest.len(),
        64,
        "the transport-capture reference must contain one SHA-256 digest"
    );
    assert!(
        digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "the transport-capture reference must be a digest rather than raw transport data"
    );
}

fn assert_receiver_child_canary_facts(row: &mirrorea_i3_probe::EvidenceRow) {
    let events = row.receiver_child_canary_events();
    assert!(
        !events.is_empty(),
        "every semantic-bearing row must retain actual receiver-child canary facts"
    );
    for (index, event) in events.iter().enumerate() {
        assert_eq!(
            event.sequence(),
            index + 1,
            "receiver-child canary facts must retain an execution order rather than expectation-only provenance"
        );
    }
}

fn assert_receiver_child_canary_sequence(
    row: &mirrorea_i3_probe::EvidenceRow,
    expected: &[ReceiverChildCanaryEventKind],
) {
    assert_eq!(
        row.receiver_child_canary_events()
            .iter()
            .map(|event| event.kind())
            .collect::<Vec<_>>(),
        expected,
        "the row must expose actual receiver-child semantic events in causal order"
    );
}
