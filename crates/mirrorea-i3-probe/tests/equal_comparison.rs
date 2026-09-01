// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

mod support;

use mirrorea_i3_probe::{CandidateCase, SemanticFalsifierOrigin, TransportCandidate};

use support::{CASE_INVENTORY, assert_common_candidate_contract, run_candidate};

#[test]
fn tls_and_quic_normalize_to_the_same_semantic_evidence_inventory() {
    let tls = run_candidate(TransportCandidate::TlsOverTcpFramedReliableStream);
    let quic = run_candidate(TransportCandidate::QuicReliableBidirectionalStream);

    assert_common_candidate_contract(&tls, TransportCandidate::TlsOverTcpFramedReliableStream);
    assert_common_candidate_contract(&quic, TransportCandidate::QuicReliableBidirectionalStream);
    assert_eq!(
        tls.normalized_rows(),
        quic.normalized_rows(),
        "candidate, mechanism, and timing may differ; semantic references and outcomes may not"
    );
    assert_eq!(
        tls.normalized_rows()
            .iter()
            .map(|row| row.case())
            .collect::<Vec<_>>(),
        CASE_INVENTORY
    );

    let tls_falsifier = tls
        .row(CandidateCase::TamperedSemanticAdmissionReference)
        .expect("TLS inventory includes the common semantic falsifier");
    let quic_falsifier = quic
        .row(CandidateCase::TamperedSemanticAdmissionReference)
        .expect("QUIC inventory includes the common semantic falsifier");
    assert_eq!(
        tls_falsifier.semantic_falsifier_frame_ref(),
        quic_falsifier.semantic_falsifier_frame_ref(),
        "TLS and QUIC must receive the exact same common falsifier frame"
    );
    assert_eq!(
        tls_falsifier.retained_contract_fingerprint(),
        quic_falsifier.retained_contract_fingerprint(),
        "candidate choice must not change the full retained contract that binds the falsifier"
    );
    assert_eq!(
        tls_falsifier.semantic_falsifier_origin(),
        SemanticFalsifierOrigin::CommonHarness,
        "a candidate module may not provide semantic tamper bytes through the test-facing API"
    );
    assert_eq!(
        quic_falsifier.semantic_falsifier_origin(),
        SemanticFalsifierOrigin::CommonHarness,
        "the same source-first common harness must provide QUIC's falsifier frame"
    );
}
