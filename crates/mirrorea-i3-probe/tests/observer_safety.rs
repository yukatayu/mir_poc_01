// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

// This target uses only the runner from the shared candidate contract; the other helpers are exercised by candidate targets.
#[allow(dead_code)]
mod support;

use mirrorea_i3_probe::{
    ObserverEvidenceErrorKind, TransportCandidate, render_observer_safe_evidence,
    validate_observer_safe_evidence,
};

use support::run_candidate;

#[test]
fn candidate_evidence_is_structured_and_observer_safe() {
    for candidate in [
        TransportCandidate::TlsOverTcpFramedReliableStream,
        TransportCandidate::QuicReliableBidirectionalStream,
    ] {
        let run = run_candidate(candidate);
        let evidence = render_observer_safe_evidence(run.rows())
            .expect("the candidate inventory must render structured observer evidence");
        assert!(evidence.is_structured());
        validate_observer_safe_evidence(evidence.as_str())
            .expect("emitted observer evidence must remain safe for an observer");
    }
}

#[test]
fn observer_scan_rejects_raw_sensitive_material_and_transport_authority_claims() {
    let raw_key_material = format!(
        r#"{{"{}":"test-only-not-a-real-key"}}"#,
        ["private", "key"].concat()
    );
    let raw_certificate_material = format!(
        r#"{{"certificate":"{}"}}"#,
        ["-----BEGIN ", "CERTIFICATE----- test-only"].concat()
    );
    for (unsafe_evidence, expected_kind) in [
        (
            raw_key_material.as_str(),
            ObserverEvidenceErrorKind::RawKeyMaterial,
        ),
        (
            raw_certificate_material.as_str(),
            ObserverEvidenceErrorKind::RawCertificateMaterial,
        ),
        (
            r#"{"source_text":"module PrivateExample"}"#,
            ObserverEvidenceErrorKind::RawSourceText,
        ),
        (
            r#"{"source_path":"/private/test/main.mir"}"#,
            ObserverEvidenceErrorKind::HostSourcePath,
        ),
        (
            r#"{"payload":"private-test-payload"}"#,
            ObserverEvidenceErrorKind::RawPayload,
        ),
        (
            r#"{"capability":"private-test-capability"}"#,
            ObserverEvidenceErrorKind::CapabilityMaterial,
        ),
        (
            r#"{"witness":"private-test-witness"}"#,
            ObserverEvidenceErrorKind::WitnessMaterial,
        ),
        (
            r#"{"private_state":"private-test-state"}"#,
            ObserverEvidenceErrorKind::PrivateState,
        ),
        (
            r#"{"transport_metadata_used_as_authority":true}"#,
            ObserverEvidenceErrorKind::TransportAuthorityClaim,
        ),
    ] {
        let rejection = validate_observer_safe_evidence(unsafe_evidence)
            .expect_err("observer scan must reject forbidden raw or authority material");
        assert_eq!(rejection.kind(), expected_kind);
    }
}
