// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

use mirrorea_i3_probe::{
    FrameDecodeErrorKind,
    FrameDecodeEvent,
    FrameDecoder,
    MAX_PRIVATE_FRAME_BYTES,
    MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES,
    SemanticAdmissionErrorKind,
    SemanticRequestSeed,
    // I3-1 static-adapter codec: this is deliberately separate from the
    // retained I3-0 `FrameDecoder` / `SemanticCarrier` comparison seam.
    StaticAdapterAdmissionErrorKind,
    StaticAdapterFrameDecodeErrorKind,
    StaticAdapterFrameDecodeEvent,
    StaticAdapterFrameDecoder,
    StaticAdapterFrameLimits,
    StaticAdapterWireCompatibility,
    WireCompatibility,
    build_source_bound_probe,
    encode_frame,
    encode_static_adapter_frame,
    private_static_adapter_wire_contract,
    private_wire_contract,
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
            "i3-0-semantic-request-sha256-v2:0000000000000000000000000000000000000000000000000000000000000000",
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
fn private_semantic_request_identity_uses_v2_and_rejects_the_retired_v1_label() {
    let carrier = attack_carrier();
    let identity = carrier.request_identity().as_str();
    let v2_prefix = "i3-0-semantic-request-sha256-v2:";
    let digest = identity
        .strip_prefix(v2_prefix)
        .expect("private emitted request identity uses the v2 hash-domain label");
    assert_eq!(
        digest.len(),
        64,
        "private v2 request identity has SHA-256 width"
    );
    assert!(
        digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "private v2 request identity has hexadecimal digest text"
    );
    assert!(
        !identity.starts_with("i3-0-semantic-request-sha256-v1:"),
        "the retired v1 label is never emitted by the private I3-0 comparison carrier"
    );

    let retired_v1 = format!("i3-0-semantic-request-sha256-v1:{digest}");
    let mut decoder = FrameDecoder::new();
    let candidate = only_candidate(
        decoder
            .push_events(&tampered_frame("request_identity", &retired_v1))
            .expect("a retired-label string remains untrusted decoded private input"),
    );
    let error = attack_edge()
        .admit_untrusted_candidate(candidate)
        .expect_err("the retired private v1 request-identity label is rejected at exact admission");
    assert_eq!(
        error.kind(),
        SemanticAdmissionErrorKind::RequestBindingMismatch
    );
}

fn mutate_legacy_top_level_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Null => *value = serde_json::Value::String("tampered-null".to_string()),
        serde_json::Value::Bool(boolean) => *boolean = !*boolean,
        serde_json::Value::Number(number) => {
            let next = number
                .as_u64()
                .and_then(|value| value.checked_add(1))
                .expect("retained I3-0 numeric fields are non-negative finite integers");
            *value = serde_json::Value::Number(serde_json::Number::from(next));
        }
        serde_json::Value::String(text) => text.push_str("-tampered"),
        serde_json::Value::Array(values) => {
            values.push(serde_json::Value::String(
                "tampered-added-member".to_string(),
            ));
        }
        serde_json::Value::Object(_) => {
            panic!("the retained I3-0 carrier has no nested top-level object field")
        }
    }
}

#[test]
fn legacy_i3_0_codec_revalidates_every_retained_top_level_field_including_seed_and_request_identity()
 {
    let edge = attack_edge();
    let encoded = encode_frame(&attack_carrier()).expect("retained I3-0 carrier encodes");
    let canonical: serde_json::Value =
        serde_json::from_slice(&encoded[4..]).expect("retained I3-0 envelope is JSON");
    let field_names = canonical["carrier"]
        .as_object()
        .expect("retained I3-0 carrier is an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        field_names.contains(&"semantic_request_seed".to_string())
            && field_names.contains(&"request_identity".to_string()),
        "legacy retained inventory keeps seed and semantic request identity as independently revalidated facts"
    );

    for field in field_names {
        let mut mutated = canonical.clone();
        mutate_legacy_top_level_value(
            mutated["carrier"]
                .as_object_mut()
                .expect("retained I3-0 carrier remains object")
                .get_mut(&field)
                .expect("enumerated retained field remains present"),
        );
        let mut decoder = FrameDecoder::new();
        let candidate = only_candidate(
            decoder
                .push_events(&framed(
                    &serde_json::to_vec(&mutated)
                        .expect("type-preserving retained-field mutation is valid JSON"),
                ))
                .expect("a type-preserving carrier mutation remains an untrusted candidate"),
        );
        let error = edge
            .admit_untrusted_candidate(candidate)
            .expect_err("every retained I3-0 field is independently revalidated before admission");
        assert_ne!(
            error.kind(),
            SemanticAdmissionErrorKind::InvalidSemanticRequestSeed,
            "field {field} receives a retained-contract/request-binding mismatch, not an invented runtime path"
        );
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

// I3-1 static-adapter codec RED tests -------------------------------------
//
// These tests intentionally name a separate static-only API.  The output of
// its admission boundary is the already source-derived `SourceBoundAdapterEdge`,
// not a semantic request, runtime occurrence, process message, transport
// session, cache key, retry token, or executable payload.  Retaining this
// distinction prevents I3-1 framing work from widening the accepted I3-0
// runtime comparison carrier.

fn static_adapter_edge() -> mirrorea_i3_probe::SourceBoundAdapterEdge {
    let probe = build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must build a static adapter inventory");
    probe
        .adapter_carrier_edges()
        .iter()
        .find(|edge| edge.operation() == "attack" && edge.edge_kind() == "owner-request")
        .expect("the accepted source must retain the attack owner-request static handoff")
        .clone()
}

fn static_adapter_edges() -> Vec<mirrorea_i3_probe::SourceBoundAdapterEdge> {
    build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must build a static adapter inventory")
        .adapter_carrier_edges()
        .to_vec()
}

fn static_adapter_frame() -> Vec<u8> {
    encode_static_adapter_frame(&static_adapter_edge())
        .expect("a source-derived static adapter handoff must encode privately")
}

fn static_adapter_body(frame: &[u8]) -> Vec<u8> {
    assert!(frame.len() >= 4, "a static adapter frame has a u32 prefix");
    let declared = u32::from_be_bytes(frame[..4].try_into().expect("prefix width"));
    assert_eq!(
        usize::try_from(declared).expect("u32 fits usize"),
        frame.len() - 4
    );
    frame[4..].to_vec()
}

fn static_adapter_framed(body: &[u8]) -> Vec<u8> {
    let length = u32::try_from(body.len()).expect("test body fits the private u32 prefix");
    let mut frame = length.to_be_bytes().to_vec();
    frame.extend_from_slice(body);
    frame
}

fn only_static_adapter_candidate(
    events: Vec<StaticAdapterFrameDecodeEvent>,
) -> mirrorea_i3_probe::UntrustedDecodedStaticAdapterCarrier {
    assert_eq!(
        events.len(),
        1,
        "one complete frame must yield one candidate"
    );
    match events.into_iter().next().expect("one event") {
        StaticAdapterFrameDecodeEvent::Decoded(candidate) => *candidate,
        StaticAdapterFrameDecodeEvent::Rejected(kind) => {
            panic!("expected decoded static candidate, got {kind:?}")
        }
    }
}

fn decode_static_adapter_once(
    frame: &[u8],
) -> mirrorea_i3_probe::UntrustedDecodedStaticAdapterCarrier {
    let mut decoder = StaticAdapterFrameDecoder::new();
    let candidate = only_static_adapter_candidate(
        decoder
            .push_events(frame)
            .expect("one complete static frame must not reject decoder state"),
    );
    assert_eq!(
        decoder
            .finish_event()
            .expect("a complete static frame can finish"),
        None
    );
    candidate
}

fn static_adapter_json(frame: &[u8]) -> serde_json::Value {
    serde_json::from_slice(&static_adapter_body(frame))
        .expect("the canonical private static frame body is JSON")
}

fn static_adapter_json_frame(value: &serde_json::Value) -> Vec<u8> {
    static_adapter_framed(
        &serde_json::to_vec(value).expect("a test-mutated static envelope remains JSON"),
    )
}

fn static_adapter_rejection(frame: &[u8]) -> StaticAdapterFrameDecodeErrorKind {
    let mut decoder = StaticAdapterFrameDecoder::new();
    let events = decoder
        .push_events(frame)
        .expect("frame-content rejection is an event, not a decoder-state error");
    assert_eq!(
        events.len(),
        1,
        "a malformed complete frame emits one rejection"
    );
    match events.into_iter().next().expect("one rejection") {
        StaticAdapterFrameDecodeEvent::Decoded(_) => {
            panic!("expected static frame rejection, got a candidate")
        }
        StaticAdapterFrameDecodeEvent::Rejected(kind) => kind,
    }
}

fn replace_first(bytes: &[u8], needle: &[u8], replacement: &[u8]) -> Vec<u8> {
    let index = bytes
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("canonical static envelope contains expected private member");
    let mut result = Vec::with_capacity(bytes.len() + replacement.len() - needle.len());
    result.extend_from_slice(&bytes[..index]);
    result.extend_from_slice(replacement);
    result.extend_from_slice(&bytes[index + needle.len()..]);
    result
}

#[derive(Clone, Debug)]
enum JsonLeafPathComponent {
    Member(String),
    Element(usize),
}

fn scalar_leaf_paths(
    value: &serde_json::Value,
    prefix: &mut Vec<JsonLeafPathComponent>,
    output: &mut Vec<Vec<JsonLeafPathComponent>>,
) {
    match value {
        serde_json::Value::Object(object) => {
            for (key, child) in object {
                prefix.push(JsonLeafPathComponent::Member(key.clone()));
                scalar_leaf_paths(child, prefix, output);
                prefix.pop();
            }
        }
        serde_json::Value::Array(values) => {
            for (index, child) in values.iter().enumerate() {
                prefix.push(JsonLeafPathComponent::Element(index));
                scalar_leaf_paths(child, prefix, output);
                prefix.pop();
            }
        }
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => output.push(prefix.clone()),
    }
}

fn leaf_at_mut<'a>(
    mut value: &'a mut serde_json::Value,
    path: &[JsonLeafPathComponent],
) -> &'a mut serde_json::Value {
    for component in path {
        value = match component {
            JsonLeafPathComponent::Member(key) => value
                .as_object_mut()
                .expect("path member parent remains an object")
                .get_mut(key)
                .expect("path member remains present"),
            JsonLeafPathComponent::Element(index) => value
                .as_array_mut()
                .expect("path element parent remains an array")
                .get_mut(*index)
                .expect("path element remains present"),
        };
    }
    value
}

fn value_at<'a>(
    mut value: &'a serde_json::Value,
    path: &[JsonLeafPathComponent],
) -> &'a serde_json::Value {
    for component in path {
        value = match component {
            JsonLeafPathComponent::Member(key) => value
                .as_object()
                .expect("path member parent remains an object")
                .get(key)
                .expect("path member remains present"),
            JsonLeafPathComponent::Element(index) => value
                .as_array()
                .expect("path element parent remains an array")
                .get(*index)
                .expect("path element remains present"),
        };
    }
    value
}

fn try_value_at<'a>(
    mut value: &'a serde_json::Value,
    path: &[JsonLeafPathComponent],
) -> Option<&'a serde_json::Value> {
    for component in path {
        value = match component {
            JsonLeafPathComponent::Member(key) => value.as_object()?.get(key)?,
            JsonLeafPathComponent::Element(index) => value.as_array()?.get(*index)?,
        };
    }
    Some(value)
}

fn remove_array_valued_member(root: &mut serde_json::Value, array_path: &[JsonLeafPathComponent]) {
    let (last, parent_path) = array_path
        .split_last()
        .expect("an encoded retained array has a non-empty path");
    let JsonLeafPathComponent::Member(member) = last else {
        panic!("retained array itself is named by an object member")
    };
    leaf_at_mut(root, parent_path)
        .as_object_mut()
        .expect("named retained array parent remains an object")
        .remove(member)
        .expect("named retained array member remains present");
}

fn array_paths(
    value: &serde_json::Value,
    prefix: &mut Vec<JsonLeafPathComponent>,
    output: &mut Vec<Vec<JsonLeafPathComponent>>,
) {
    match value {
        serde_json::Value::Object(object) => {
            for (key, child) in object {
                prefix.push(JsonLeafPathComponent::Member(key.clone()));
                array_paths(child, prefix, output);
                prefix.pop();
            }
        }
        serde_json::Value::Array(values) => {
            output.push(prefix.clone());
            for (index, child) in values.iter().enumerate() {
                prefix.push(JsonLeafPathComponent::Element(index));
                array_paths(child, prefix, output);
                prefix.pop();
            }
        }
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {}
    }
}

fn first_scalar_member(object: &serde_json::Value) -> String {
    object
        .as_object()
        .expect("selected nested value remains an object")
        .iter()
        .find_map(|(key, value)| match value {
            serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => Some(key.clone()),
            serde_json::Value::Null
            | serde_json::Value::Array(_)
            | serde_json::Value::Object(_) => None,
        })
        .expect("selected nested object has one non-null direct scalar member")
}

fn first_named_object_path(
    value: &serde_json::Value,
    fragment: &str,
    prefix: &mut Vec<JsonLeafPathComponent>,
) -> Option<Vec<JsonLeafPathComponent>> {
    let object = value.as_object()?;
    for (key, child) in object {
        prefix.push(JsonLeafPathComponent::Member(key.clone()));
        if key.contains(fragment) && child.is_object() {
            return Some(prefix.clone());
        }
        if let Some(found) = first_named_object_path(child, fragment, prefix) {
            return Some(found);
        }
        prefix.pop();
    }
    None
}

fn first_named_nonempty_object_array_path(
    value: &serde_json::Value,
    fragment: &str,
    prefix: &mut Vec<JsonLeafPathComponent>,
) -> Option<Vec<JsonLeafPathComponent>> {
    let object = value.as_object()?;
    for (key, child) in object {
        prefix.push(JsonLeafPathComponent::Member(key.clone()));
        if key.contains(fragment)
            && child
                .as_array()
                .is_some_and(|values| values.first().is_some_and(serde_json::Value::is_object))
        {
            return Some(prefix.clone());
        }
        if let Some(found) = first_named_nonempty_object_array_path(child, fragment, prefix) {
            return Some(found);
        }
        prefix.pop();
    }
    None
}

fn direct_member_anchor(path: &[JsonLeafPathComponent]) -> String {
    path.iter()
        .rev()
        .find_map(|component| match component {
            JsonLeafPathComponent::Member(member) => Some(member.clone()),
            JsonLeafPathComponent::Element(_) => None,
        })
        .expect("nested object path retains a member anchor")
}

fn escaped_equivalent_member(member: &str) -> String {
    let mut characters = member.chars();
    let first = characters
        .next()
        .expect("private schema member is non-empty ASCII text");
    assert!(first.is_ascii(), "private schema member anchor is ASCII");
    format!("\\u{:04x}{}", u32::from(first), characters.as_str())
}

fn insert_duplicate_direct_object_member(
    body: &[u8],
    object_anchor: &str,
    member: &str,
    original_value: &serde_json::Value,
    escaped_equivalent: bool,
) -> Vec<u8> {
    let needle = format!("\"{object_anchor}\":{{");
    let duplicate_member = if escaped_equivalent {
        escaped_equivalent_member(member)
    } else {
        member.to_string()
    };
    let original_value = serde_json::to_string(original_value)
        .expect("selected canonical member remains serializable for duplicate-key falsification");
    let replacement = format!("\"{object_anchor}\":{{\"{duplicate_member}\":{original_value},");
    replace_first(body, needle.as_bytes(), replacement.as_bytes())
}

fn insert_duplicate_first_array_row_member(
    body: &[u8],
    array_anchor: &str,
    member: &str,
    original_value: &serde_json::Value,
    escaped_equivalent: bool,
) -> Vec<u8> {
    let needle = format!("\"{array_anchor}\":[{{");
    let duplicate_member = if escaped_equivalent {
        escaped_equivalent_member(member)
    } else {
        member.to_string()
    };
    let original_value = serde_json::to_string(original_value).expect(
        "selected canonical row member remains serializable for duplicate-key falsification",
    );
    let replacement = format!("\"{array_anchor}\":[{{\"{duplicate_member}\":{original_value},");
    replace_first(body, needle.as_bytes(), replacement.as_bytes())
}

fn nested_structural_falsifier_cases(
    canonical: &serde_json::Value,
    object_path: &[JsonLeafPathComponent],
    member: &str,
) -> Vec<serde_json::Value> {
    let mut cases = Vec::new();

    let mut missing = canonical.clone();
    leaf_at_mut(
        missing.get_mut("carrier").expect("carrier remains present"),
        object_path,
    )
    .as_object_mut()
    .expect("nested schema target stays an object")
    .remove(member);
    cases.push(missing);

    let mut extra = canonical.clone();
    leaf_at_mut(
        extra.get_mut("carrier").expect("carrier remains present"),
        object_path,
    )
    .as_object_mut()
    .expect("nested schema target stays an object")
    .insert("unexpected".to_string(), serde_json::Value::Bool(true));
    cases.push(extra);

    let mut null = canonical.clone();
    leaf_at_mut(
        null.get_mut("carrier").expect("carrier remains present"),
        object_path,
    )
    .as_object_mut()
    .expect("nested schema target stays an object")
    .insert(member.to_string(), serde_json::Value::Null);
    cases.push(null);

    let mut wrong_type = canonical.clone();
    leaf_at_mut(
        wrong_type
            .get_mut("carrier")
            .expect("carrier remains present"),
        object_path,
    )
    .as_object_mut()
    .expect("nested schema target stays an object")
    .insert(member.to_string(), serde_json::Value::Array(Vec::new()));
    cases.push(wrong_type);

    cases
}

fn mutate_scalar_for_negative_case(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Null => *value = serde_json::Value::String("tampered-null".to_string()),
        serde_json::Value::Bool(boolean) => *boolean = !*boolean,
        serde_json::Value::Number(number) => {
            let next = number
                .as_u64()
                .and_then(|value| value.checked_add(1))
                .or_else(|| {
                    number
                        .as_i64()
                        .and_then(|value| value.checked_add(1))
                        .map(|value| value as u64)
                })
                .expect("canonical static private numeric leaves are finite integers");
            *value = serde_json::Value::Number(serde_json::Number::from(next));
        }
        serde_json::Value::String(text) => text.push_str("-tampered"),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            panic!("scalar leaf traversal never returns a container")
        }
    }
}

fn path_text(path: &[JsonLeafPathComponent]) -> String {
    path.iter()
        .map(|component| match component {
            JsonLeafPathComponent::Member(member) => format!(".{member}"),
            JsonLeafPathComponent::Element(index) => format!("[{index}]"),
        })
        .collect()
}

fn static_adapter_event_trace(chunks: &[Vec<u8>]) -> Vec<StaticAdapterFrameDecodeEvent> {
    let mut decoder = StaticAdapterFrameDecoder::new();
    let mut events = Vec::new();
    for chunk in chunks {
        events.extend(
            decoder
                .push_events(chunk)
                .expect("test corpus must not push after terminal rejection"),
        );
    }
    events
}

fn assert_type_valid_static_mutation_is_rejected_at_source_bound_admission(
    edge: &mirrorea_i3_probe::SourceBoundAdapterEdge,
    frame: &[u8],
    context: &str,
) {
    let candidate = decode_static_adapter_once(frame);
    let error = edge
        .admit_untrusted_static_adapter_candidate(candidate)
        .expect_err("type-valid static mutation must reach and fail source-bound revalidation");
    assert_eq!(
        error.kind(),
        StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch,
        "{context}"
    );
}

fn assert_static_mutation_fails_closed_before_or_at_source_bound_admission(
    edge: &mirrorea_i3_probe::SourceBoundAdapterEdge,
    frame: &[u8],
    context: &str,
) {
    let mut decoder = StaticAdapterFrameDecoder::new();
    let events = decoder
        .push_events(frame)
        .expect("strict rejection is an event rather than a decoder-state error");
    assert_eq!(events.len(), 1, "one complete mutation yields one outcome");
    match events.into_iter().next().expect("one mutation outcome") {
        StaticAdapterFrameDecodeEvent::Decoded(candidate) => {
            let error = edge
                .admit_untrusted_static_adapter_candidate(*candidate)
                .expect_err("decoded mutation must still fail source-bound revalidation");
            assert_eq!(
                error.kind(),
                StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch,
                "{context}"
            );
        }
        StaticAdapterFrameDecodeEvent::Rejected(kind) => assert_eq!(
            kind,
            StaticAdapterFrameDecodeErrorKind::MalformedPayload,
            "{context}"
        ),
    }
}

fn assert_terminal_after_content_rejection(
    frame: &[u8],
    expected: StaticAdapterFrameDecodeErrorKind,
) {
    let mut decoder = StaticAdapterFrameDecoder::new();
    assert_eq!(
        decoder
            .push_events(frame)
            .expect("content rejection is emitted as an event"),
        [StaticAdapterFrameDecodeEvent::Rejected(expected)]
    );
    assert_eq!(
        decoder
            .push_events(&static_adapter_frame())
            .expect_err("content rejection terminalizes the decoder")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
    assert_eq!(
        decoder
            .finish_event()
            .expect_err("content rejection also terminalizes finish")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
}

fn every_one_cut(frame: &[u8]) -> Vec<Vec<Vec<u8>>> {
    (1..frame.len())
        .map(|cut| vec![frame[..cut].to_vec(), frame[cut..].to_vec()])
        .collect()
}

fn deterministic_multisplit(frame: &[u8]) -> Vec<Vec<u8>> {
    let mut seed = 0x7f4a_7c15_u32;
    let mut offset = 0;
    let mut chunks = Vec::new();
    while offset < frame.len() {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let width = 1 + usize::try_from(seed % 11).expect("small pseudo-random width fits usize");
        let end = (offset + width).min(frame.len());
        chunks.push(frame[offset..end].to_vec());
        offset = end;
    }
    chunks
}

#[test]
fn static_adapter_codec_is_private_provisional_and_separate_from_i3_0_framing() {
    let contract = private_static_adapter_wire_contract();
    assert_ne!(contract.max_frame_bytes(), 0);
    assert_eq!(
        contract.max_frame_bytes(),
        MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES
    );
    assert_ne!(contract.max_push_bytes(), 0);
    assert_ne!(contract.max_decoded_events_per_push(), 0);
    assert_eq!(
        contract.compatibility(),
        StaticAdapterWireCompatibility::PrivateProvisional
    );

    for edge in static_adapter_edges() {
        let frame = encode_static_adapter_frame(&edge)
            .expect("each source-derived static adapter handoff encodes privately");
        let body_bytes = static_adapter_body(&frame);
        let body =
            std::str::from_utf8(&body_bytes).expect("private adapter envelope is UTF-8 JSON");
        for forbidden_runtime_claim in [
            "network_occurrence",
            "session",
            "certificate",
            "request_identity",
            "semantic_request_seed",
            "payload",
            "retry_occurrence",
            "retry_attempt",
            "retry_command",
            "retry_count",
            "retry_token",
            "hidden_retry_token",
            "cache",
            "MailboxEnvelope",
        ] {
            assert!(
                !body.contains(forbidden_runtime_claim),
                "static source-bound handoff {} must not serialize runtime fact {forbidden_runtime_claim}",
                edge.edge_ref()
            );
        }
    }
}

#[test]
fn static_adapter_codec_losslessly_retains_the_designated_result_delivery_contract() {
    let edge = static_adapter_edges()
        .into_iter()
        .find(|edge| edge.edge_kind() == "designated-result-delivery")
        .expect("closed static inventory retains a designated-result delivery edge");
    let frame = encode_static_adapter_frame(&edge)
        .expect("designated-result static handoff encodes privately");
    let canonical = static_adapter_json(&frame);
    let facts = canonical["carrier"]["variant"]["facts"]
        .as_object()
        .expect("designated-result static wire has closed variant facts");
    assert_eq!(
        facts.get("static_delivery_contract"),
        Some(&serde_json::Value::String(
            "ReturnExistingNoNewConsumption".to_string()
        )),
        "the static delivery contract is an explicit closed semantic fact, not a hidden retry behavior"
    );
    assert!(
        !facts.contains_key("delivery_contract_ref"),
        "the canonical static wire retains the typed delivery contract itself, not only an opaque surrogate reference"
    );

    let mut unknown_delivery_contract = canonical.clone();
    unknown_delivery_contract["carrier"]["variant"]["facts"]["static_delivery_contract"] =
        serde_json::Value::String("UnknownFutureDeliveryContract".to_string());
    assert_eq!(
        static_adapter_rejection(&static_adapter_json_frame(&unknown_delivery_contract)),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload,
        "an unknown static delivery contract string fails strict decoding before an untrusted candidate exists"
    );

    let candidate = decode_static_adapter_once(&frame);
    assert_eq!(
        candidate.retained_static_contract(),
        edge.retained_static_contract(),
        "the decoded untrusted static snapshot retains the designated-result delivery contract losslessly"
    );
}

#[test]
fn static_adapter_codec_preserves_exact_opaque_retained_facts_for_the_closed_source_bound_inventory()
 {
    let edges = static_adapter_edges();
    assert_eq!(
        edges.len(),
        12,
        "accepted I2 projection has twelve static edges"
    );
    let family_count = edges
        .iter()
        .map(|edge| edge.edge_kind())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    assert_eq!(
        family_count, 6,
        "the static codec corpus covers every closed accepted carrier family"
    );

    for edge in &edges {
        let frame = encode_static_adapter_frame(edge)
            .expect("each exact static source-bound edge must encode");
        let candidate = decode_static_adapter_once(&frame);
        assert_eq!(
            candidate.retained_static_contract(),
            edge.retained_static_contract(),
            "decoded untrusted data retains an opaque exact static snapshot before admission"
        );
        assert_eq!(
            candidate.retained_static_contract_fingerprint_field_names(),
            edge.retained_static_contract_fingerprint_field_names(),
            "decoded static snapshot retains the production-owned exhaustive fingerprint inventory"
        );
        assert!(
            !edge
                .retained_static_contract_fingerprint_field_names()
                .is_empty(),
            "the exhaustive SYS-5 visitor inventory cannot become vacuous"
        );
        let admitted = edge
            .admit_untrusted_static_adapter_candidate(candidate)
            .expect("the matching source-bound static handoff admits its exact private snapshot");
        assert_eq!(admitted, *edge);
        assert_eq!(admitted.edge_ref(), edge.edge_ref());
    }

    let mut non_self_cross_admission_mismatches = 0_usize;
    for (source_index, source) in edges.iter().enumerate() {
        let source_frame = encode_static_adapter_frame(source)
            .expect("each exact static source-bound edge must encode");
        for (target_index, target) in edges.iter().enumerate() {
            if source_index == target_index {
                continue;
            }
            assert_ne!(
                source.retained_static_contract(),
                target.retained_static_contract(),
                "every distinct source-bound edge must retain a distinct opaque static contract"
            );
            let error = target
                .admit_untrusted_static_adapter_candidate(decode_static_adapter_once(&source_frame))
                .expect_err(
                    "a type-valid static candidate cannot cross-admit to another retained edge",
                );
            assert_eq!(
                error.kind(),
                StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch,
                "cross-admission rejects a different opaque retained static contract"
            );
            non_self_cross_admission_mismatches += 1;
        }
    }
    assert_eq!(
        non_self_cross_admission_mismatches,
        12 * 11,
        "all ordered non-self static-contract cross admissions must be rejected"
    );
}

#[test]
fn static_adapter_codec_rejects_duplicate_known_keys_before_static_admission() {
    let body = static_adapter_body(&static_adapter_frame());
    let duplicate_marker = replace_first(
        &body,
        br#""marker":"#,
        br#""marker":"duplicate-marker","marker":"#,
    );
    let duplicate_version = replace_first(&body, br#""version":"#, br#""version":1,"version":"#);
    let duplicate_carrier = replace_first(&body, br#""carrier":"#, br#""carrier":null,"carrier":"#);
    let duplicate_nested_edge_kind = replace_first(
        &body,
        br#""carrier":{"#,
        br#""carrier":{"edge_kind":"owner-request","edge_kind":"owner-request","#,
    );
    let escaped_duplicate_marker = replace_first(
        &body,
        br#""marker":"#,
        br#""marker":"duplicate-marker","\u006darker":"#,
    );

    for duplicate in [
        duplicate_marker,
        duplicate_version,
        duplicate_carrier,
        duplicate_nested_edge_kind,
        escaped_duplicate_marker,
    ] {
        assert_eq!(
            static_adapter_rejection(&static_adapter_framed(&duplicate)),
            StaticAdapterFrameDecodeErrorKind::MalformedPayload,
            "duplicate JSON object members must fail before an untrusted static candidate exists"
        );
    }
}

#[test]
fn static_adapter_codec_rejects_missing_extra_null_and_wrong_type_members_before_admission() {
    let canonical = static_adapter_json(&static_adapter_frame());
    let mut cases = Vec::new();

    let mut missing_envelope = canonical.clone();
    missing_envelope
        .as_object_mut()
        .expect("envelope object")
        .remove("marker");
    cases.push(missing_envelope);

    let mut extra_envelope = canonical.clone();
    extra_envelope
        .as_object_mut()
        .expect("envelope object")
        .insert("unexpected".to_string(), serde_json::Value::Bool(true));
    cases.push(extra_envelope);

    let mut null_envelope = canonical.clone();
    null_envelope["marker"] = serde_json::Value::Null;
    cases.push(null_envelope);

    let mut wrong_envelope = canonical.clone();
    wrong_envelope["version"] = serde_json::Value::String("one".to_string());
    cases.push(wrong_envelope);

    let mut missing_carrier = canonical.clone();
    missing_carrier["carrier"]
        .as_object_mut()
        .expect("canonical carrier object")
        .remove("edge_kind");
    cases.push(missing_carrier);

    let mut extra_carrier = canonical.clone();
    extra_carrier["carrier"]
        .as_object_mut()
        .expect("canonical carrier object")
        .insert("unexpected".to_string(), serde_json::Value::Bool(true));
    cases.push(extra_carrier);

    let mut null_carrier = canonical.clone();
    null_carrier["carrier"]["edge_kind"] = serde_json::Value::Null;
    cases.push(null_carrier);

    let mut wrong_carrier = canonical;
    wrong_carrier["carrier"]["edge_kind"] = serde_json::Value::Bool(true);
    cases.push(wrong_carrier);

    for value in cases {
        assert_eq!(
            static_adapter_rejection(&static_adapter_json_frame(&value)),
            StaticAdapterFrameDecodeErrorKind::MalformedPayload,
            "closed private schema rejects structural variation before static admission"
        );
    }
}

#[test]
fn static_adapter_codec_is_strict_inside_variant_facts_and_authority_row_objects() {
    let mut one_edge_per_family = std::collections::BTreeMap::new();
    for edge in static_adapter_edges() {
        one_edge_per_family
            .entry(edge.edge_kind().to_string())
            .or_insert(edge);
    }
    assert_eq!(
        one_edge_per_family.len(),
        6,
        "strict nested-schema corpus has one canonical edge for every closed family"
    );

    let mut authority_row_checked = false;
    for (family, edge) in &one_edge_per_family {
        let frame = encode_static_adapter_frame(edge)
            .expect("each canonical static family edge serializes");
        let canonical = static_adapter_json(&frame);
        let carrier = canonical
            .get("carrier")
            .expect("canonical static envelope retains carrier");
        let variant_path = first_named_object_path(carrier, "variant", &mut Vec::new())
            .expect("every static family retains a family-specific variant object");
        let variant_member = first_scalar_member(value_at(carrier, &variant_path));
        let variant_value = value_at(carrier, &variant_path)
            .get(&variant_member)
            .expect("selected variant member remains present")
            .clone();
        let variant_anchor = direct_member_anchor(&variant_path);
        let body = static_adapter_body(&frame);

        for duplicate in [
            insert_duplicate_direct_object_member(
                &body,
                &variant_anchor,
                &variant_member,
                &variant_value,
                false,
            ),
            insert_duplicate_direct_object_member(
                &body,
                &variant_anchor,
                &variant_member,
                &variant_value,
                true,
            ),
        ] {
            assert_eq!(
                static_adapter_rejection(&static_adapter_framed(&duplicate)),
                StaticAdapterFrameDecodeErrorKind::MalformedPayload,
                "family {family}: nested duplicate, including escaped-equivalent key, fails before admission"
            );
        }
        let variant_cases =
            nested_structural_falsifier_cases(&canonical, &variant_path, &variant_member);
        assert_eq!(
            variant_cases.len(),
            4,
            "family {family}: variant has missing/extra/null/wrong-type falsifiers"
        );
        for malformed in variant_cases {
            assert_eq!(
                static_adapter_rejection(&static_adapter_json_frame(&malformed)),
                StaticAdapterFrameDecodeErrorKind::MalformedPayload,
                "family {family}: nested closed variant schema rejects structural variation"
            );
        }

        if !authority_row_checked {
            let authority_rows_path =
                first_named_nonempty_object_array_path(carrier, "authority", &mut Vec::new())
                    .expect("at least one canonical family retains a non-empty authority row list");
            let mut authority_row_path = authority_rows_path.clone();
            authority_row_path.push(JsonLeafPathComponent::Element(0));
            let authority_member = first_scalar_member(value_at(carrier, &authority_row_path));
            let authority_value = value_at(carrier, &authority_row_path)
                .get(&authority_member)
                .expect("selected authority row member remains present")
                .clone();
            let authority_anchor = direct_member_anchor(&authority_rows_path);
            for duplicate in [
                insert_duplicate_first_array_row_member(
                    &body,
                    &authority_anchor,
                    &authority_member,
                    &authority_value,
                    false,
                ),
                insert_duplicate_first_array_row_member(
                    &body,
                    &authority_anchor,
                    &authority_member,
                    &authority_value,
                    true,
                ),
            ] {
                assert_eq!(
                    static_adapter_rejection(&static_adapter_framed(&duplicate)),
                    StaticAdapterFrameDecodeErrorKind::MalformedPayload,
                    "authority row duplicate, including escaped-equivalent key, fails before admission"
                );
            }
            let authority_cases = nested_structural_falsifier_cases(
                &canonical,
                &authority_row_path,
                &authority_member,
            );
            assert_eq!(
                authority_cases.len(),
                4,
                "one non-empty authority row has missing/extra/null/wrong-type falsifiers"
            );
            for malformed in authority_cases {
                assert_eq!(
                    static_adapter_rejection(&static_adapter_json_frame(&malformed)),
                    StaticAdapterFrameDecodeErrorKind::MalformedPayload,
                    "authority row closed schema rejects structural variation"
                );
            }
            authority_row_checked = true;
        }
    }
    assert!(
        authority_row_checked,
        "strict nested-schema corpus locates a non-empty authority row"
    );
}

#[test]
fn static_adapter_codec_distinguishes_wrong_marker_from_unknown_version_with_marker_precedence() {
    let canonical = static_adapter_json(&static_adapter_frame());

    let mut wrong_marker = canonical.clone();
    wrong_marker["marker"] = serde_json::Value::String("wrong-marker".to_string());
    assert_eq!(
        static_adapter_rejection(&static_adapter_json_frame(&wrong_marker)),
        StaticAdapterFrameDecodeErrorKind::MarkerMismatch
    );

    let mut unknown_version = canonical.clone();
    unknown_version["version"] = serde_json::Value::Number(999_u64.into());
    assert_eq!(
        static_adapter_rejection(&static_adapter_json_frame(&unknown_version)),
        StaticAdapterFrameDecodeErrorKind::UnknownVersion
    );

    let mut both_wrong = wrong_marker;
    both_wrong["version"] = serde_json::Value::Number(999_u64.into());
    assert_eq!(
        static_adapter_rejection(&static_adapter_json_frame(&both_wrong)),
        StaticAdapterFrameDecodeErrorKind::MarkerMismatch,
        "marker validation has deterministic precedence when both envelope claims are invalid"
    );
}

#[test]
fn static_adapter_codec_classifies_headers_before_a_future_shaped_carrier() {
    let canonical = static_adapter_json(&static_adapter_frame());
    let current_marker = serde_json::to_string(
        canonical
            .get("marker")
            .expect("canonical static envelope retains marker"),
    )
    .expect("canonical marker remains JSON-serializable");
    let current_version = canonical
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .expect("canonical static envelope retains a numeric version");
    let future_carrier = r#"{"future_shape":true}"#;

    let unknown_version =
        format!(r#"{{"marker":{current_marker},"version":999,"carrier":{future_carrier}}}"#);
    assert_eq!(
        static_adapter_rejection(&static_adapter_framed(unknown_version.as_bytes())),
        StaticAdapterFrameDecodeErrorKind::UnknownVersion,
        "an unknown header version wins before a syntactically valid future carrier is decoded"
    );

    let wrong_marker_and_unknown_version =
        format!(r#"{{"marker":"wrong-marker","version":999,"carrier":{future_carrier}}}"#);
    assert_eq!(
        static_adapter_rejection(&static_adapter_framed(
            wrong_marker_and_unknown_version.as_bytes(),
        )),
        StaticAdapterFrameDecodeErrorKind::MarkerMismatch,
        "marker mismatch has deterministic precedence over version and future carrier shape"
    );

    let current_headers_and_future_carrier = format!(
        r#"{{"marker":{current_marker},"version":{current_version},"carrier":{future_carrier}}}"#
    );
    assert_eq!(
        static_adapter_rejection(&static_adapter_framed(
            current_headers_and_future_carrier.as_bytes(),
        )),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload,
        "a future carrier is not accepted under the current closed header"
    );
}

#[test]
fn static_adapter_codec_rejects_a_zero_length_body_as_malformed_and_terminal() {
    let mut decoder = StaticAdapterFrameDecoder::new();
    assert_eq!(
        decoder
            .push_events(&[0, 0, 0, 0])
            .expect("a zero-length declared body has a typed terminal event"),
        [StaticAdapterFrameDecodeEvent::Rejected(
            StaticAdapterFrameDecodeErrorKind::MalformedPayload
        )],
        "a complete zero-length body is malformed, never an end-of-input truncation"
    );
    assert_eq!(
        decoder
            .push_events(&static_adapter_frame())
            .expect_err("zero-length malformed body terminalizes later input")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
    assert_eq!(
        decoder
            .finish_event()
            .expect_err("zero-length malformed body terminalizes finish")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
}

#[test]
fn static_adapter_codec_terminalizes_after_every_representative_content_or_truncation_rejection() {
    let valid = static_adapter_frame();
    let mut malformed_body = static_adapter_body(&valid);
    malformed_body[0] = b'!';
    let malformed = static_adapter_framed(&malformed_body);
    let duplicate = static_adapter_framed(&replace_first(
        &static_adapter_body(&valid),
        br#""marker":"#,
        br#""marker":"duplicate-marker","marker":"#,
    ));
    let canonical = static_adapter_json(&valid);
    let mut wrong_marker = canonical.clone();
    wrong_marker["marker"] = serde_json::Value::String("wrong-marker".to_string());
    let mut unknown_version = canonical;
    unknown_version["version"] = serde_json::Value::Number(999_u64.into());
    let oversized: Vec<u8> = u32::try_from(MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES + 1)
        .expect("private maximum permits representable oversized prefix")
        .to_be_bytes()
        .to_vec();

    assert_terminal_after_content_rejection(
        &malformed,
        StaticAdapterFrameDecodeErrorKind::MalformedPayload,
    );
    assert_terminal_after_content_rejection(
        &duplicate,
        StaticAdapterFrameDecodeErrorKind::MalformedPayload,
    );
    assert_terminal_after_content_rejection(
        &static_adapter_json_frame(&wrong_marker),
        StaticAdapterFrameDecodeErrorKind::MarkerMismatch,
    );
    assert_terminal_after_content_rejection(
        &static_adapter_json_frame(&unknown_version),
        StaticAdapterFrameDecodeErrorKind::UnknownVersion,
    );
    assert_terminal_after_content_rejection(
        &oversized,
        StaticAdapterFrameDecodeErrorKind::OversizedFrame,
    );

    let mut truncated = StaticAdapterFrameDecoder::new();
    assert!(
        truncated
            .push_events(&valid[..valid.len() - 1])
            .expect("incomplete valid frame stays pending")
            .is_empty()
    );
    assert_eq!(
        truncated
            .finish_event()
            .expect("truncated completion is typed"),
        Some(StaticAdapterFrameDecodeEvent::Rejected(
            StaticAdapterFrameDecodeErrorKind::TruncatedBody
        ))
    );
    assert_eq!(
        truncated
            .push_events(&valid)
            .expect_err("truncated finish terminalizes later pushes")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
    assert_eq!(
        truncated
            .finish_event()
            .expect_err("truncated finish terminalizes later finishes")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
}

#[test]
fn static_adapter_codec_rejects_second_value_or_non_whitespace_suffix_but_accepts_trailing_whitespace()
 {
    let body = static_adapter_body(&static_adapter_frame());
    let mut second_value = body.clone();
    second_value.extend_from_slice(br#"{}"#);
    assert_eq!(
        static_adapter_rejection(&static_adapter_framed(&second_value)),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload
    );

    let mut non_whitespace = body.clone();
    non_whitespace.extend_from_slice(b"!");
    assert_eq!(
        static_adapter_rejection(&static_adapter_framed(&non_whitespace)),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload
    );

    let mut whitespace = body;
    whitespace.extend_from_slice(b" \n\t\r");
    let candidate = decode_static_adapter_once(&static_adapter_framed(&whitespace));
    static_adapter_edge()
        .admit_untrusted_static_adapter_candidate(candidate)
        .expect("trailing JSON whitespace remains one complete private envelope");
}

#[test]
fn static_adapter_codec_is_chunking_invariant_for_valid_malformed_duplicate_and_oversized_inputs() {
    let valid = static_adapter_frame();
    let mut malformed_body = static_adapter_body(&valid);
    malformed_body[0] = b'!';
    let malformed = static_adapter_framed(&malformed_body);
    let duplicate = static_adapter_framed(&replace_first(
        &static_adapter_body(&valid),
        br#""marker":"#,
        br#""marker":"duplicate-marker","marker":"#,
    ));
    let oversized: Vec<u8> = u32::try_from(MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES + 1)
        .expect("private max permits a representable oversized prefix")
        .to_be_bytes()
        .to_vec();

    let chunking_frames: [&[u8]; 3] = [&valid, &malformed, &duplicate];
    for frame in chunking_frames {
        let baseline_chunks: Vec<Vec<u8>> = vec![frame.to_vec()];
        let baseline = static_adapter_event_trace(&baseline_chunks);
        for chunks in every_one_cut(frame) {
            assert_eq!(
                static_adapter_event_trace(&chunks),
                baseline,
                "one-cut fragmentation cannot change private static decode events"
            );
        }
        assert_eq!(
            static_adapter_event_trace(&deterministic_multisplit(frame)),
            baseline,
            "fixed pseudo-random fragmentation cannot change private static decode events"
        );
    }

    let oversized_baseline = static_adapter_event_trace(std::slice::from_ref(&oversized));
    for chunks in every_one_cut(&oversized) {
        assert_eq!(
            static_adapter_event_trace(&chunks),
            oversized_baseline,
            "one-cut fragmentation cannot change private static oversized-frame rejection"
        );
    }
    assert_eq!(
        static_adapter_event_trace(&deterministic_multisplit(&oversized)),
        oversized_baseline,
        "fixed pseudo-random fragmentation cannot change private static oversized-frame rejection"
    );
}

#[test]
fn static_adapter_codec_requires_a_complete_declared_body_and_rejects_over_limit_prefixes() {
    let valid = static_adapter_frame();
    let mut decoder = StaticAdapterFrameDecoder::new();
    for end in 1..valid.len() {
        assert!(
            decoder
                .push_events(&valid[end - 1..end])
                .expect("incomplete prefixes and bodies are pending, not state errors")
                .is_empty(),
            "no static candidate exists before byte {end} completes its declared body"
        );
    }
    assert!(matches!(
        decoder
            .push_events(&valid[valid.len() - 1..])
            .expect("completion decodes")
            .as_slice(),
        [StaticAdapterFrameDecodeEvent::Decoded(_)]
    ));

    let max_body = vec![b'x'; MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES];
    let max_frame = static_adapter_framed(&max_body);
    assert_eq!(
        static_adapter_rejection(&max_frame),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload,
        "a body at the byte maximum reaches codec validation rather than allocation failure"
    );

    let oversized = u32::try_from(MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES + 1)
        .expect("private max permits a representable oversized prefix")
        .to_be_bytes();
    assert_eq!(
        static_adapter_rejection(&oversized),
        StaticAdapterFrameDecodeErrorKind::OversizedFrame
    );
}

#[test]
fn static_adapter_codec_preserves_prior_complete_events_across_coalescing_partial_tail_and_terminal_finish()
 {
    let valid = static_adapter_frame();
    let mut two_valid = valid.clone();
    two_valid.extend_from_slice(&valid);
    let events = static_adapter_event_trace(&[two_valid]);
    assert!(matches!(
        events.as_slice(),
        [
            StaticAdapterFrameDecodeEvent::Decoded(_),
            StaticAdapterFrameDecodeEvent::Decoded(_)
        ]
    ));

    let mut valid_then_oversized = valid.clone();
    valid_then_oversized.extend_from_slice(
        &u32::try_from(MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES + 1)
            .expect("private maximum permits a representable oversized prefix")
            .to_be_bytes(),
    );
    assert!(matches!(
        static_adapter_event_trace(&[valid_then_oversized]).as_slice(),
        [
            StaticAdapterFrameDecodeEvent::Decoded(_),
            StaticAdapterFrameDecodeEvent::Rejected(
                StaticAdapterFrameDecodeErrorKind::OversizedFrame
            )
        ]
    ));

    let mut malformed_body = static_adapter_body(&valid);
    malformed_body[0] = b'!';
    let mut valid_then_malformed = valid.clone();
    valid_then_malformed.extend_from_slice(&static_adapter_framed(&malformed_body));
    assert!(matches!(
        static_adapter_event_trace(&[valid_then_malformed]).as_slice(),
        [
            StaticAdapterFrameDecodeEvent::Decoded(_),
            StaticAdapterFrameDecodeEvent::Rejected(
                StaticAdapterFrameDecodeErrorKind::MalformedPayload
            )
        ]
    ));

    let mut prefix_tail = valid.clone();
    prefix_tail.extend_from_slice(&[0, 0, 0]);
    let mut prefix_decoder = StaticAdapterFrameDecoder::new();
    let prefix_events = prefix_decoder
        .push_events(&prefix_tail)
        .expect("complete event survives trailing partial prefix");
    assert!(matches!(
        prefix_events.as_slice(),
        [StaticAdapterFrameDecodeEvent::Decoded(_)]
    ));
    assert_eq!(
        prefix_decoder
            .finish_event()
            .expect("typed end-of-input event"),
        Some(StaticAdapterFrameDecodeEvent::Rejected(
            StaticAdapterFrameDecodeErrorKind::TruncatedPrefix
        ))
    );

    let mut body_tail = valid.clone();
    body_tail.extend_from_slice(&8_u32.to_be_bytes());
    body_tail.extend_from_slice(b"partial");
    let mut body_decoder = StaticAdapterFrameDecoder::new();
    let body_events = body_decoder
        .push_events(&body_tail)
        .expect("complete event survives trailing partial body");
    assert!(matches!(
        body_events.as_slice(),
        [StaticAdapterFrameDecodeEvent::Decoded(_)]
    ));
    assert_eq!(
        body_decoder
            .finish_event()
            .expect("typed end-of-input event"),
        Some(StaticAdapterFrameDecodeEvent::Rejected(
            StaticAdapterFrameDecodeErrorKind::TruncatedBody
        ))
    );

    let mut clean_decoder = StaticAdapterFrameDecoder::new();
    assert_eq!(clean_decoder.finish_event().expect("clean finish"), None);
    assert_eq!(
        clean_decoder
            .push_events(&valid)
            .expect_err("clean finish terminalizes the static decoder")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
}

#[test]
fn static_adapter_codec_enforces_private_per_push_byte_and_decoded_event_limits_without_erasing_prior_events()
 {
    let valid = static_adapter_frame();
    let contract = private_static_adapter_wire_contract();

    let mut byte_reset = StaticAdapterFrameDecoder::with_private_limits(
        StaticAdapterFrameLimits::new(
            contract.max_frame_bytes(),
            valid.len(),
            contract.max_decoded_events_per_push(),
        )
        .expect("test limits are internally consistent"),
    );
    for push_index in 0..2 {
        assert!(
            matches!(
                byte_reset
                    .push_events(&valid)
                    .expect("each independently within-limit byte push succeeds")
                    .as_slice(),
                [StaticAdapterFrameDecodeEvent::Decoded(_)]
            ),
            "byte budget resets for independent push {push_index}"
        );
    }
    let mut one_byte_too_many = valid.clone();
    one_byte_too_many.push(0);
    let mut byte_limited = StaticAdapterFrameDecoder::with_private_limits(
        StaticAdapterFrameLimits::new(
            contract.max_frame_bytes(),
            valid.len(),
            contract.max_decoded_events_per_push(),
        )
        .expect("test limits are internally consistent"),
    );
    assert_eq!(
        byte_limited
            .push_events(&one_byte_too_many)
            .expect("byte-limit failure is preserved as a terminal typed event"),
        [StaticAdapterFrameDecodeEvent::Rejected(
            StaticAdapterFrameDecodeErrorKind::PushByteLimitExceeded
        )]
    );
    assert_eq!(
        byte_limited
            .push_events(&valid)
            .expect_err("byte-limit rejection terminalizes later pushes")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
    assert_eq!(
        byte_limited
            .finish_event()
            .expect_err("byte-limit rejection terminalizes finish")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );

    let mut event_reset = StaticAdapterFrameDecoder::with_private_limits(
        StaticAdapterFrameLimits::new(contract.max_frame_bytes(), valid.len() * 2, 1)
            .expect("test limits are internally consistent"),
    );
    for push_index in 0..2 {
        assert!(
            matches!(
                event_reset
                    .push_events(&valid)
                    .expect("one decoded candidate remains within each independent push budget")
                    .as_slice(),
                [StaticAdapterFrameDecodeEvent::Decoded(_)]
            ),
            "decoded-event budget resets for independent push {push_index}"
        );
    }
    let mut event_limited = StaticAdapterFrameDecoder::with_private_limits(
        StaticAdapterFrameLimits::new(contract.max_frame_bytes(), valid.len() * 2, 1)
            .expect("test limits are internally consistent"),
    );
    let mut two_valid = valid.clone();
    two_valid.extend_from_slice(&valid);
    let events = event_limited
        .push_events(&two_valid)
        .expect("event-limit failure is an event after already-complete candidates");
    assert!(matches!(
        events.as_slice(),
        [
            StaticAdapterFrameDecodeEvent::Decoded(_),
            StaticAdapterFrameDecodeEvent::Rejected(
                StaticAdapterFrameDecodeErrorKind::PushDecodedEventLimitExceeded
            )
        ]
    ));
    assert_eq!(
        event_limited
            .push_events(&valid)
            .expect_err("decoded-event-limit rejection terminalizes later pushes")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
    assert_eq!(
        event_limited
            .finish_event()
            .expect_err("decoded-event-limit rejection terminalizes finish")
            .kind(),
        StaticAdapterFrameDecodeErrorKind::DecoderRejected
    );
}

#[test]
fn static_adapter_codec_revalidates_every_retained_static_scalar_without_a_parallel_test_schema() {
    let edges = static_adapter_edges();
    let mut total_scalar_paths = 0_usize;
    let mut total_type_valid_mismatches = 0_usize;

    for edge in &edges {
        let canonical = static_adapter_json(
            &encode_static_adapter_frame(edge)
                .expect("each source-bound static edge serializes its exact contract"),
        );
        let carrier = canonical
            .get("carrier")
            .expect("canonical static envelope retains a carrier member");
        let mut paths = Vec::new();
        scalar_leaf_paths(carrier, &mut Vec::new(), &mut paths);
        assert!(
            !paths.is_empty(),
            "each closed static family retains scalar facts to mutate"
        );
        total_scalar_paths += paths.len();
        let mut type_valid_mismatches = 0_usize;

        for path in paths {
            let mut mutated = canonical.clone();
            mutate_scalar_for_negative_case(leaf_at_mut(
                mutated
                    .get_mut("carrier")
                    .expect("canonical carrier remains present"),
                &path,
            ));
            let frame = static_adapter_json_frame(&mutated);
            let mut decoder = StaticAdapterFrameDecoder::new();
            let events = decoder
                .push_events(&frame)
                .expect("type or lexical rejection remains an event");
            assert_eq!(events.len(), 1, "one complete mutation yields one outcome");
            match events.into_iter().next().expect("one mutation outcome") {
                StaticAdapterFrameDecodeEvent::Decoded(candidate) => {
                    let error = edge
                        .admit_untrusted_static_adapter_candidate(*candidate)
                        .expect_err(
                            "type-valid retained static mutation must fail at source-bound admission",
                        );
                    assert_eq!(
                        error.kind(),
                        StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch,
                        "type-valid mutation {} must preserve the static mismatch group",
                        path_text(&path)
                    );
                    type_valid_mismatches += 1;
                    total_type_valid_mismatches += 1;
                }
                StaticAdapterFrameDecodeEvent::Rejected(kind) => assert_eq!(
                    kind,
                    StaticAdapterFrameDecodeErrorKind::MalformedPayload,
                    "closed enum/lexical mutation {} fails strict schema before admission",
                    path_text(&path)
                ),
            }
        }
        assert!(
            type_valid_mismatches > 0,
            "every source-bound edge has at least one type-valid static mismatch; cross-admission independently supplies a second such negative"
        );
    }
    assert!(
        total_scalar_paths > 0 && total_type_valid_mismatches >= edges.len(),
        "all twelve canonical static frames contribute dynamic scalar falsifiers"
    );
}

#[test]
fn static_adapter_codec_preserves_array_order_and_multiplicity_without_a_parallel_test_schema() {
    let edges = static_adapter_edges();
    let canonical_frames = edges
        .iter()
        .map(|edge| {
            (
                edge,
                static_adapter_json(
                    &encode_static_adapter_frame(edge)
                        .expect("each source-bound static edge serializes its exact contract"),
                ),
            )
        })
        .collect::<Vec<_>>();
    let mut nonempty_array_count = 0_usize;
    let mut empty_array_count = 0_usize;
    let mut additions_to_emptied_lists = 0_usize;
    let mut reorderable_array_count = 0_usize;
    let mut authority_rows_seen = false;
    let mut failure_list_seen = false;
    let mut effect_list_seen = false;
    let mut occurrence_list_seen = false;
    let mut frontier_list_seen = false;

    let mut compatible_empty_list_transplants = 0_usize;

    for (edge, canonical) in &canonical_frames {
        let carrier = canonical
            .get("carrier")
            .expect("canonical static envelope retains a carrier member");
        let mut paths = Vec::new();
        array_paths(carrier, &mut Vec::new(), &mut paths);
        assert!(
            !paths.is_empty(),
            "each static carrier retains at least one list-valued fact"
        );

        for path in paths {
            let label = path_text(&path);
            authority_rows_seen |= label.contains("authority");
            failure_list_seen |= label.contains("failure");
            effect_list_seen |= label.contains("effect");
            occurrence_list_seen |= label.contains("occurrence");
            frontier_list_seen |= label.contains("frontier");
            let values = value_at(carrier, &path)
                .as_array()
                .expect("discovered array path still resolves to an array");

            if values.is_empty() {
                empty_array_count += 1;
                let mut omitted = canonical.clone();
                remove_array_valued_member(
                    omitted.get_mut("carrier").expect("carrier remains present"),
                    &path,
                );
                assert_ne!(
                    omitted, *canonical,
                    "removing empty array-valued member {label} changes canonical JSON"
                );
                assert_eq!(
                    static_adapter_rejection(&static_adapter_json_frame(&omitted)),
                    StaticAdapterFrameDecodeErrorKind::MalformedPayload,
                    "empty retained list {label} is required, not a defaultable omission"
                );

                let mut added = canonical.clone();
                leaf_at_mut(
                    added.get_mut("carrier").expect("carrier remains present"),
                    &path,
                )
                .as_array_mut()
                .expect("array remains mutable")
                .push(serde_json::Value::String(
                    "tampered-added-member".to_string(),
                ));
                assert_ne!(
                    added, *canonical,
                    "adding an untyped value to empty list {label} changes canonical JSON"
                );
                assert_static_mutation_fails_closed_before_or_at_source_bound_admission(
                    edge,
                    &static_adapter_json_frame(&added),
                    &format!(
                        "an attacker cannot grow an empty retained list with an untyped member: {label}"
                    ),
                );

                if let Some(donor_values) = canonical_frames.iter().find_map(|(donor, source)| {
                    (donor.edge_ref() != edge.edge_ref())
                        .then(|| {
                            try_value_at(
                                source
                                    .get("carrier")
                                    .expect("donor canonical envelope retains carrier"),
                                &path,
                            )
                        })
                        .flatten()
                        .and_then(serde_json::Value::as_array)
                        .filter(|values| !values.is_empty())
                }) {
                    let mut transplanted = canonical.clone();
                    leaf_at_mut(
                        transplanted
                            .get_mut("carrier")
                            .expect("carrier remains present"),
                        &path,
                    )
                    .as_array_mut()
                    .expect("array remains mutable")
                    .extend(donor_values.iter().cloned());
                    assert_ne!(
                        transplanted, *canonical,
                        "transplanting a path-compatible canonical list changes {label}"
                    );
                    assert_type_valid_static_mutation_is_rejected_at_source_bound_admission(
                        edge,
                        &static_adapter_json_frame(&transplanted),
                        &format!("transplanting another edge's type-valid list into {label}"),
                    );
                    compatible_empty_list_transplants += 1;
                }
                continue;
            }

            nonempty_array_count += 1;
            let mut removed = canonical.clone();
            leaf_at_mut(
                removed.get_mut("carrier").expect("carrier remains present"),
                &path,
            )
            .as_array_mut()
            .expect("array remains mutable")
            .remove(0);
            assert_ne!(
                removed, *canonical,
                "removing one retained member from {label} changes canonical JSON"
            );
            assert_type_valid_static_mutation_is_rejected_at_source_bound_admission(
                edge,
                &static_adapter_json_frame(&removed),
                &format!("removing one retained list member from {label}"),
            );

            let mut duplicated = canonical.clone();
            let duplicated_values = leaf_at_mut(
                duplicated
                    .get_mut("carrier")
                    .expect("carrier remains present"),
                &path,
            )
            .as_array_mut()
            .expect("array remains mutable");
            duplicated_values.push(
                duplicated_values
                    .first()
                    .expect("non-empty list keeps its first member")
                    .clone(),
            );
            assert_ne!(
                duplicated, *canonical,
                "duplicating one retained member in {label} changes canonical JSON"
            );
            assert_type_valid_static_mutation_is_rejected_at_source_bound_admission(
                edge,
                &static_adapter_json_frame(&duplicated),
                &format!("duplicating one retained list member in {label}"),
            );

            let mut emptied_then_added = canonical.clone();
            let emptied_then_added_values = leaf_at_mut(
                emptied_then_added
                    .get_mut("carrier")
                    .expect("carrier remains present"),
                &path,
            )
            .as_array_mut()
            .expect("array remains mutable");
            let first = emptied_then_added_values
                .first()
                .expect("non-empty list keeps its first member")
                .clone();
            emptied_then_added_values.clear();
            emptied_then_added_values.push(first.clone());
            emptied_then_added_values.push(first);
            additions_to_emptied_lists += 1;
            assert_ne!(
                emptied_then_added, *canonical,
                "adding typed values after emptying {label} changes canonical JSON"
            );
            assert_type_valid_static_mutation_is_rejected_at_source_bound_admission(
                edge,
                &static_adapter_json_frame(&emptied_then_added),
                &format!("adding typed members after emptying retained list {label}"),
            );

            if values.len() >= 2 && values[0] != values[1] {
                reorderable_array_count += 1;
                let mut reordered = canonical.clone();
                leaf_at_mut(
                    reordered
                        .get_mut("carrier")
                        .expect("carrier remains present"),
                    &path,
                )
                .as_array_mut()
                .expect("array remains mutable")
                .swap(0, 1);
                assert_ne!(
                    reordered, *canonical,
                    "reordering distinct retained values in {label} changes canonical JSON"
                );
                assert_type_valid_static_mutation_is_rejected_at_source_bound_admission(
                    edge,
                    &static_adapter_json_frame(&reordered),
                    &format!("reordering retained list members in {label}"),
                );
            }
        }
    }

    assert!(
        nonempty_array_count > 0,
        "canonical corpus has non-empty retained lists"
    );
    assert!(
        additions_to_emptied_lists > 0,
        "canonical corpus exercises type-valid additions to dynamically emptied retained lists"
    );
    assert!(
        empty_array_count > 0,
        "canonical corpus has at least one exact empty retained list"
    );
    assert!(
        compatible_empty_list_transplants > 0,
        "canonical corpus has at least one path-compatible empty retained list that receives a type-valid cross-edge transplant"
    );
    assert!(
        reorderable_array_count > 0,
        "canonical corpus has order-sensitive lists"
    );
    assert!(
        authority_rows_seen,
        "canonical corpus serializes ordered authority rows"
    );
    assert!(
        failure_list_seen,
        "canonical corpus serializes declared failure lists"
    );
    assert!(
        effect_list_seen,
        "canonical corpus serializes effect-kind lists"
    );
    assert!(
        occurrence_list_seen,
        "canonical corpus serializes occurrence-slot lists"
    );
    assert!(
        frontier_list_seen,
        "canonical corpus serializes variant frontier-style lists"
    );
}

#[test]
fn static_adapter_codec_fails_closed_on_deep_attacker_json_without_claiming_a_specific_recursion_limit()
 {
    let canonical = static_adapter_json(&static_adapter_frame());
    let mut shallow = canonical.clone();
    shallow["carrier"] = serde_json::json!([[[["not-a-static-carrier"]]]]);
    assert_eq!(
        static_adapter_rejection(&static_adapter_json_frame(&shallow)),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload
    );

    let marker = serde_json::to_string(
        canonical
            .get("marker")
            .expect("canonical private envelope retains its marker"),
    )
    .expect("marker stays serializable");
    let version = serde_json::to_string(
        canonical
            .get("version")
            .expect("canonical private envelope retains its version"),
    )
    .expect("version stays serializable");
    let mut over_depth_body = format!("{{\"marker\":{marker},\"version\":{version},\"carrier\":");
    over_depth_body.push_str(&"[".repeat(256));
    over_depth_body.push_str("\"not-a-static-carrier\"");
    over_depth_body.push_str(&"]".repeat(256));
    over_depth_body.push('}');
    assert_eq!(
        static_adapter_rejection(&static_adapter_framed(over_depth_body.as_bytes())),
        StaticAdapterFrameDecodeErrorKind::MalformedPayload,
        "deep attacker JSON fails closed before any static candidate or admission exists"
    );
}
