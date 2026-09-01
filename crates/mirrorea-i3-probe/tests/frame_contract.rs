// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

use mirrorea_i3_probe::{
    FrameDecodeErrorKind, FrameDecodeEvent, FrameDecoder, MAX_PRIVATE_FRAME_BYTES,
    SemanticAdmissionErrorKind, SemanticRequestSeed, WireCompatibility, build_source_bound_probe,
    encode_frame, private_wire_contract,
};

const ACTIVE_I2_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"
);

fn attack_edge() -> mirrorea_i3_probe::SourceBoundEdge {
    let probe = build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must build a source-bound private I3 probe");
    probe
        .owner_request_edge("attack")
        .expect("the checked/projected source must retain attack")
        .clone()
}

fn attack_carrier() -> mirrorea_i3_probe::SemanticCarrier {
    attack_edge()
        .bind_semantic_request(SemanticRequestSeed::new("frame-contract-invocation-01"))
        .expect("a semantic request seed must bind the retained attack contract")
}

fn framed(body: &[u8]) -> Vec<u8> {
    let length = u32::try_from(body.len()).expect("test body must fit the private u32 prefix");
    let mut bytes = length.to_be_bytes().to_vec();
    bytes.extend_from_slice(body);
    bytes
}

fn only_candidate(events: Vec<FrameDecodeEvent>) -> mirrorea_i3_probe::UntrustedDecodedCarrier {
    assert_eq!(events.len(), 1, "one complete frame yields one event");
    match events.into_iter().next().expect("one event") {
        FrameDecodeEvent::Decoded(candidate) => *candidate,
        FrameDecodeEvent::Rejected(kind) => panic!("expected decoded candidate, got {kind:?}"),
    }
}

fn tampered_frame(field: &str, value: &str) -> Vec<u8> {
    let encoded = encode_frame(&attack_carrier()).expect("source-bound carrier must encode");
    let mut envelope: serde_json::Value =
        serde_json::from_slice(&encoded[4..]).expect("private test frame is JSON");
    envelope["carrier"][field] = serde_json::Value::String(value.to_string());
    let body = serde_json::to_vec(&envelope).expect("tampered envelope remains valid JSON");
    framed(&body)
}

#[test]
fn encoded_frame_has_a_big_endian_u32_body_length_without_network_occurrence() {
    let carrier = attack_carrier();
    let encoded = encode_frame(&carrier).expect("source-bound carrier must encode");

    assert!(
        encoded.len() > 4,
        "a frame has a four-byte prefix and a body"
    );
    let declared_length = u32::from_be_bytes(encoded[..4].try_into().expect("prefix width"));
    assert_eq!(declared_length as usize, encoded.len() - 4);
    assert_eq!(
        encode_frame(&carrier).expect("the same semantic carrier re-encodes"),
        encoded,
        "private framing must be deterministic for one semantic carrier"
    );
    let body = std::str::from_utf8(&encoded[4..]).expect("private JSON body is UTF-8");
    assert!(
        !body.contains("network_occurrence"),
        "a network occurrence is not semantic carrier data and must never be serialized"
    );
    for self_asserted_fact in [
        "authority_basis",
        "visibility_fact",
        "transport_metadata",
        "sealed_admission_ref",
    ] {
        assert!(
            !body.contains(self_asserted_fact),
            "untrusted wire input must not assert {self_asserted_fact}"
        );
    }
}

#[test]
fn decoder_yields_only_an_untrusted_candidate_until_exact_retained_validation() {
    let edge = attack_edge();
    let encoded = encode_frame(
        &edge
            .bind_semantic_request(SemanticRequestSeed::new("admission-invocation-01"))
            .expect("seed binds semantic request"),
    )
    .expect("source-bound carrier must encode");
    let mut decoder = FrameDecoder::new();

    let candidate = only_candidate(
        decoder
            .push_events(&encoded)
            .expect("a complete frame can decode to an untrusted candidate"),
    );
    let admitted = edge
        .admit_untrusted_candidate(candidate)
        .expect("only the separately retained exact contract may admit the candidate");
    assert_eq!(admitted.edge_ref(), edge.edge_ref());
    assert_eq!(admitted.target_locus(), "WorldAuthority");
}

#[test]
fn exact_retained_validation_rejects_tampered_semantic_bindings() {
    for (field, value, expected_kind) in [
        (
            "core_ref",
            "tampered-core-ref",
            SemanticAdmissionErrorKind::CoreReferenceMismatch,
        ),
        (
            "target_locus",
            "tampered-target-locus",
            SemanticAdmissionErrorKind::TargetLocusMismatch,
        ),
        (
            "edge_ref",
            "tampered-edge-ref",
            SemanticAdmissionErrorKind::EdgeReferenceMismatch,
        ),
        (
            "request_identity",
            "i3-0-semantic-request-sha256-v1:0000000000000000000000000000000000000000000000000000000000000000",
            SemanticAdmissionErrorKind::RequestBindingMismatch,
        ),
        (
            "retained_contract_fingerprint",
            "sys5-i3-probe-carrier-contract-sha256-v1:0000000000000000000000000000000000000000000000000000000000000000",
            SemanticAdmissionErrorKind::RetainedContractFingerprintMismatch,
        ),
    ] {
        let mut decoder = FrameDecoder::new();
        let candidate = only_candidate(
            decoder
                .push_events(&tampered_frame(field, value))
                .expect("valid JSON with a tampered semantic field still decodes as untrusted"),
        );
        let error = attack_edge()
            .admit_untrusted_candidate(candidate)
            .expect_err(
                "exact retained validation must fail closed on a tampered semantic binding",
            );
        assert_eq!(error.kind(), expected_kind);
    }
}

#[test]
fn decoder_preserves_completed_valid_event_before_terminal_invalid_event_regardless_of_chunking() {
    let valid = encode_frame(&attack_carrier()).expect("source-bound carrier must encode");
    let oversized = u32::try_from(MAX_PRIVATE_FRAME_BYTES + 1)
        .expect("the private maximum leaves one representable oversized prefix")
        .to_be_bytes();
    let mut coalesced_bytes = valid.clone();
    coalesced_bytes.extend_from_slice(&oversized);

    let mut coalesced = FrameDecoder::new();
    let coalesced_events = coalesced
        .push_events(&coalesced_bytes)
        .expect("batch/event decoding preserves completed events before a terminal rejection");

    let mut split = FrameDecoder::new();
    let mut split_events = split
        .push_events(&valid)
        .expect("the valid frame yields its untrusted candidate");
    split_events.extend(
        split
            .push_events(&oversized)
            .expect("the oversized prefix yields a terminal typed event"),
    );

    assert_eq!(coalesced_events, split_events);
    assert_eq!(coalesced_events.len(), 2);
    assert!(matches!(coalesced_events[0], FrameDecodeEvent::Decoded(_)));
    assert!(matches!(
        coalesced_events[1],
        FrameDecodeEvent::Rejected(FrameDecodeErrorKind::OversizedFrame)
    ));
}

#[test]
fn oversized_prefix_is_rejected_before_any_body_bytes_arrive() {
    let oversized = u32::try_from(MAX_PRIVATE_FRAME_BYTES + 1)
        .expect("the private maximum leaves one representable oversized prefix")
        .to_be_bytes();
    let mut decoder = FrameDecoder::new();

    assert_eq!(
        decoder
            .push_events(&oversized)
            .expect("an oversized prefix becomes a typed terminal event"),
        [FrameDecodeEvent::Rejected(
            FrameDecodeErrorKind::OversizedFrame
        )]
    );
}

#[test]
fn incomplete_malformed_and_unknown_version_frames_are_typed_without_admission() {
    let mut prefix_decoder = FrameDecoder::new();
    assert!(
        prefix_decoder
            .push_events(&[0, 0, 0])
            .expect("a partial prefix remains pending")
            .is_empty()
    );
    assert_eq!(
        prefix_decoder
            .finish_event()
            .expect("finish returns a typed event"),
        Some(FrameDecodeEvent::Rejected(
            FrameDecodeErrorKind::TruncatedPrefix
        ))
    );

    let encoded = encode_frame(&attack_carrier()).expect("source-bound carrier must encode");
    let mut body_decoder = FrameDecoder::new();
    assert!(
        body_decoder
            .push_events(&encoded[..encoded.len() - 1])
            .expect("a partial body remains pending")
            .is_empty()
    );
    assert_eq!(
        body_decoder
            .finish_event()
            .expect("finish returns a typed event"),
        Some(FrameDecodeEvent::Rejected(
            FrameDecodeErrorKind::TruncatedBody
        ))
    );

    for (body, expected_kind) in [
        (
            br#"{"version":999,"carrier":null}"#.as_slice(),
            FrameDecodeErrorKind::UnknownVersion,
        ),
        (
            br#"{not-json"#.as_slice(),
            FrameDecodeErrorKind::MalformedPayload,
        ),
    ] {
        let mut decoder = FrameDecoder::new();
        assert_eq!(
            decoder
                .push_events(&framed(body))
                .expect("a complete invalid frame yields a typed event"),
            [FrameDecodeEvent::Rejected(expected_kind)]
        );
    }
}

#[test]
fn clean_finish_terminalizes_the_decoder_and_rejects_later_input() {
    let mut decoder = FrameDecoder::new();
    assert_eq!(
        decoder
            .finish_event()
            .expect("a clean end-of-input has no terminal rejection event"),
        None
    );

    let error = match decoder
        .push_events(&encode_frame(&attack_carrier()).expect("attack carrier encodes"))
    {
        Ok(events) => panic!(
            "no frame may be accepted after the decoder has observed clean end-of-input; event_count={}",
            events.len()
        ),
        Err(error) => error,
    };
    assert_eq!(error.kind(), FrameDecodeErrorKind::DecoderRejected);
    assert_eq!(
        decoder.finish_event().map_err(|error| error.kind()),
        Err(FrameDecodeErrorKind::DecoderRejected),
        "a second finish cannot reopen a terminal decoder"
    );
}

#[test]
fn frame_limit_is_explicitly_private_and_provisional() {
    let contract = private_wire_contract();
    assert_ne!(
        contract.max_frame_bytes(),
        0,
        "the private frame limit must be nonzero"
    );
    assert_eq!(contract.max_frame_bytes(), MAX_PRIVATE_FRAME_BYTES);
    assert_eq!(
        contract.compatibility(),
        WireCompatibility::PrivateProvisional,
        "the I3-0 frame limit is not a public wire compatibility promise"
    );
}
