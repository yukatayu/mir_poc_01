// This integration target exercises the selected I3-1 QUIC static-adapter
// seam. It is private/provisional evidence, not a public wire or runtime API.
#![allow(unused_crate_dependencies)]

use std::{
    collections::{BTreeMap, BTreeSet},
    net::IpAddr,
};

use mirrorea_i3_probe::{
    ObserverEvidenceErrorKind, SourceBoundAdapterEdge, StaticAdapterAdmissionErrorKind,
    StaticAdapterQuicAdmissionOutcome, StaticAdapterQuicFalsifier,
    StaticAdapterQuicObserverErrorKind, StaticAdapterQuicPlatformClaim, StaticAdapterQuicRun,
    StaticAdapterQuicRunErrorKind, StaticAdapterQuicTransportEventKind, build_source_bound_probe,
    encode_private_static_adapter_quic_ingress_for_test, encode_static_adapter_frame,
    private_static_adapter_frame_reference, private_static_adapter_snapshot_reference,
    run_static_adapter_quic_loopback, run_static_adapter_quic_loopback_from_private_ingress,
    run_static_adapter_quic_loopback_with_falsifier, validate_observer_safe_evidence,
    validate_static_adapter_quic_observer_evidence,
};

const ACTIVE_I2_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"
);
const PRIVATE_QUIC_REFERENCE_PREFIX: &str = "mirrorea-i3-static-adapter-private-ref-sha256-v1:";

fn source_inventory() -> Vec<SourceBoundAdapterEdge> {
    build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must build the closed source-derived adapter inventory")
        .adapter_carrier_edges()
        .to_vec()
}

fn reversed_receiver_inventory() -> Vec<SourceBoundAdapterEdge> {
    let mut inventory = source_inventory();
    inventory.reverse();
    inventory
}

fn invalid_sender_inventory(count: usize) -> Vec<SourceBoundAdapterEdge> {
    let full = source_inventory();
    assert_eq!(
        full.len(),
        12,
        "the closed I3-1 source fixture has twelve edges"
    );
    match count {
        2 | 11 => full[..count].to_vec(),
        13 => {
            let mut oversized = full;
            oversized.push(
                oversized
                    .first()
                    .expect("the closed finite fixture has an edge to duplicate")
                    .clone(),
            );
            oversized
        }
        _ => panic!("this finite-profile falsifier only constructs 2, 11, or 13 sender edges"),
    }
}

fn expected_family_counts() -> BTreeMap<String, usize> {
    BTreeMap::from([
        ("designated-input-receipt".to_string(), 1),
        ("designated-input-request".to_string(), 1),
        ("designated-result-delivery".to_string(), 1),
        ("owner-reply-receipt".to_string(), 4),
        ("owner-request".to_string(), 4),
        ("relation-projection-publication".to_string(), 1),
    ])
}

fn expected_frame_references(inventory: &[SourceBoundAdapterEdge]) -> Vec<String> {
    inventory
        .iter()
        .map(|edge| {
            let frame = encode_static_adapter_frame(edge)
                .expect("every retained source edge must encode before selected QUIC transport");
            private_static_adapter_frame_reference(&frame)
        })
        .collect()
}

fn assert_private_quic_reference(reference: &str, label: &str) {
    let digest = reference
        .strip_prefix(PRIVATE_QUIC_REFERENCE_PREFIX)
        .unwrap_or_else(|| {
            panic!("{label} must be a fixed-domain private QUIC reference, not raw material")
        });
    assert_eq!(digest.len(), 64, "{label} must have SHA-256 width");
    assert!(
        digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "{label} must contain only a hexadecimal digest"
    );
}

fn assert_transport_records(run: &StaticAdapterQuicRun, expected_server_accepts: usize) {
    let events = run.transport_events();
    assert!(
        events
            .iter()
            .any(|event| event.kind() == StaticAdapterQuicTransportEventKind::UdpSocketBound),
        "the selected seam must record an actual loopback UDP bind"
    );
    assert_eq!(
        events
            .iter()
            .filter(
                |event| event.kind() == StaticAdapterQuicTransportEventKind::QuicHandshakeCompleted
            )
            .count(),
        1,
        "the server-side records must contain exactly one actual QUIC handshake"
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind() == StaticAdapterQuicTransportEventKind::CertificateEvidence)
            .count(),
        1,
        "certificate evidence is a per-run transport artifact, not authority"
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind() == StaticAdapterQuicTransportEventKind::SessionEvidence)
            .count(),
        1,
        "session evidence is a per-run transport artifact, not authority"
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| {
                event.kind()
                    == StaticAdapterQuicTransportEventKind::ServerAcceptedBidirectionalStream
            })
            .count(),
        expected_server_accepts,
        "each source-bound handoff must reach an actual server accept_bi ingress"
    );
    for event in events {
        assert_private_quic_reference(event.evidence_ref(), "transport event evidence");
    }

    let features = run.transport_features();
    assert!(
        features.reliable_bidirectional_streams_only(),
        "the selected I3-1 seam is QUIC reliable bidirectional streams only"
    );
    assert!(
        !features.datagram_enabled(),
        "QUIC datagrams are excluded from this selected adapter"
    );
    assert!(
        !features.zero_rtt_enabled(),
        "this private seam must not smuggle 0-RTT/replay semantics into I3-1"
    );
}

fn assert_exact_observer_schema(run: &StaticAdapterQuicRun) {
    let observer = run.observer_safe_evidence();
    validate_observer_safe_evidence(observer)
        .expect("selected QUIC evidence must pass the repository observer/redaction validator");
    validate_static_adapter_quic_observer_evidence(observer).expect(
        "selected QUIC evidence must pass the exact static-adapter observer schema validator",
    );

    let value: serde_json::Value =
        serde_json::from_str(observer).expect("selected QUIC observer evidence is JSON");
    let root = value
        .as_object()
        .expect("selected QUIC observer evidence is a non-vacuous object");
    assert_eq!(
        root.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "ingress_events",
            "platform_claim",
            "schema",
            "summary",
            "transport_events",
        ]),
        "the exact private observer schema contains only reference/reason/count fields"
    );
    assert_eq!(
        root.get("schema").and_then(serde_json::Value::as_str),
        Some("mirrorea-i3-static-adapter-quic-observer-v1")
    );
    let transport_events = root
        .get("transport_events")
        .and_then(serde_json::Value::as_array)
        .filter(|events| !events.is_empty())
        .expect("observer evidence contains non-vacuous transport events");
    for event in transport_events {
        let event = event
            .as_object()
            .expect("every transport event is a structured object");
        assert_eq!(
            event.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from(["evidence_ref", "kind"]),
            "transport events expose only a kind and private reference"
        );
    }
    let ingress_events = root
        .get("ingress_events")
        .and_then(serde_json::Value::as_array)
        .filter(|events| !events.is_empty())
        .expect(
            "observer evidence contains actual ingress facts rather than an empty success shell",
        );
    for event in ingress_events {
        let event = event
            .as_object()
            .expect("every ingress event is a structured object");
        assert_eq!(
            event.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            BTreeSet::from([
                "admission_outcome",
                "decoded_full_snapshot_ref",
                "selected_receiver_full_snapshot_ref",
                "selected_receiver_retained_edge_ref",
                "sender_edge_ref",
                "canonical_source_frame_ref",
                "server_received_frame_ref",
                "transmitted_frame_ref",
                "untrusted_reference_hint",
            ]),
            "ingress events expose only static frame/snapshot/selection/admission evidence"
        );
    }
    let summary = root
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .expect("observer evidence contains a structured count-only summary");
    assert_eq!(
        summary.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "admitted_family_counts",
            "admitted_handoff_count",
            "ingress_event_count",
            "receiver_retained_inventory_count",
            "source_sender_inventory_count",
        ]),
        "summary remains count-only and cannot become a runtime/cache surface"
    );

    for forbidden in [
        "candidate",
        "canary",
        "cache",
        "handler",
        "retry",
        "occurrence",
        "127.0.0.1",
        "::1",
        "BEGIN PRIVATE KEY",
        "BEGIN CERTIFICATE",
        "private_key_der",
        "certificate_der",
        "samples/clean-near-end/mirrorea-i2-local-toy/main.mir",
    ] {
        assert!(
            !observer.contains(forbidden),
            "observer schema must not expose an I3-0/runtime field, endpoint, or raw secret/source material: {forbidden}"
        );
    }
    assert_no_raw_ip_address_string(&value);
}

fn assert_no_raw_ip_address_string(value: &serde_json::Value) {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                assert_no_raw_ip_address_string(value);
            }
        }
        serde_json::Value::Object(values) => {
            for value in values.values() {
                assert_no_raw_ip_address_string(value);
            }
        }
        serde_json::Value::String(value) => assert!(
            value.parse::<IpAddr>().is_err(),
            "observer evidence must not expose a raw IPv4 or IPv6 endpoint"
        ),
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {}
    }
}

fn replace_first_private_reference(observer: &str, replacement: &str) -> String {
    let start = observer
        .find(PRIVATE_QUIC_REFERENCE_PREFIX)
        .expect("non-vacuous selected QUIC observer evidence contains a private reference");
    let end = start + PRIVATE_QUIC_REFERENCE_PREFIX.len() + 64;
    assert!(
        observer.get(start..end).is_some(),
        "the private reference uses a fixed SHA-256 width"
    );
    let mut mutated = observer.to_string();
    mutated.replace_range(start..end, replacement);
    mutated
}

enum NestedObserverObject {
    TransportEvent,
    IngressEvent,
    Summary,
}

fn insert_unknown_nested_canary(observer: &str, target: NestedObserverObject) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    let root = value
        .as_object_mut()
        .expect("the canonical observer evidence is an object");
    let target = match target {
        NestedObserverObject::TransportEvent => root
            .get_mut("transport_events")
            .and_then(serde_json::Value::as_array_mut)
            .and_then(|events| events.first_mut())
            .and_then(serde_json::Value::as_object_mut),
        NestedObserverObject::IngressEvent => root
            .get_mut("ingress_events")
            .and_then(serde_json::Value::as_array_mut)
            .and_then(|events| events.first_mut())
            .and_then(serde_json::Value::as_object_mut),
        NestedObserverObject::Summary => root
            .get_mut("summary")
            .and_then(serde_json::Value::as_object_mut),
    }
    .expect("the canonical observer schema has the requested nested object");
    target.insert(
        "unexpected_base64_canary".to_string(),
        serde_json::Value::String("TUlSLU5FU1RFRC1TQ0hFTUEtQ0FOQVJZ".to_string()),
    );
    serde_json::to_string(&value).expect("the nested-canary observer mutation remains JSON")
}

fn alter_summary_count(observer: &str, field: &str) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    let summary = value
        .get_mut("summary")
        .and_then(serde_json::Value::as_object_mut)
        .expect("the canonical observer evidence retains a structured summary");
    let original = summary
        .get(field)
        .and_then(serde_json::Value::as_u64)
        .expect("the requested summary field is a bounded integer count");
    summary.insert(
        field.to_string(),
        serde_json::Value::Number(serde_json::Number::from(original + 1)),
    );
    serde_json::to_string(&value).expect("the count-altered observer mutation remains JSON")
}

fn alter_summary_family_count(observer: &str, family: &str) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    let families = value
        .get_mut("summary")
        .and_then(serde_json::Value::as_object_mut)
        .and_then(|summary| summary.get_mut("admitted_family_counts"))
        .and_then(serde_json::Value::as_object_mut)
        .expect("the canonical observer summary retains admitted family counts");
    let original = families
        .get(family)
        .and_then(serde_json::Value::as_u64)
        .expect("the selected family is a bounded integer count");
    families.insert(
        family.to_string(),
        serde_json::Value::Number(serde_json::Number::from(original + 1)),
    );
    serde_json::to_string(&value).expect("the family-count-altered observer mutation remains JSON")
}

fn remove_required_transport_inventory(observer: &str) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    value
        .as_object_mut()
        .expect("the canonical observer evidence is an object")
        .remove("transport_events")
        .expect("the canonical observer evidence retains transport inventory");
    serde_json::to_string(&value).expect("the missing-transport observer mutation remains JSON")
}

fn empty_required_transport_inventory(observer: &str) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    *value
        .get_mut("transport_events")
        .expect("the canonical observer evidence retains transport inventory") =
        serde_json::Value::Array(Vec::new());
    serde_json::to_string(&value).expect("the empty-transport observer mutation remains JSON")
}

fn remove_required_transport_kind(observer: &str, required_kind: &str) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    let events = value
        .get_mut("transport_events")
        .and_then(serde_json::Value::as_array_mut)
        .expect("the canonical observer evidence retains transport inventory");
    let original_len = events.len();
    events.retain(|event| {
        event.get("kind").and_then(serde_json::Value::as_str) != Some(required_kind)
    });
    assert!(
        events.len() < original_len,
        "the canonical transport inventory contains the required {required_kind} event"
    );
    serde_json::to_string(&value).expect("the incomplete-transport observer mutation remains JSON")
}

fn fabricate_ingress_reference(observer: &str, field: &str) -> String {
    let mut value: serde_json::Value =
        serde_json::from_str(observer).expect("the canonical observer evidence is JSON");
    let ingress = value
        .get_mut("ingress_events")
        .and_then(serde_json::Value::as_array_mut)
        .and_then(|events| events.first_mut())
        .and_then(serde_json::Value::as_object_mut)
        .expect("the canonical observer evidence retains a nonempty ingress inventory");
    ingress.insert(
        field.to_string(),
        serde_json::Value::String("fabricated-retained-edge-reference".to_string()),
    );
    serde_json::to_string(&value).expect("the fabricated-reference observer mutation remains JSON")
}

fn assert_admitted_event_has_exact_sender_binding(
    event: &mirrorea_i3_probe::StaticAdapterQuicIngressEvent,
) {
    if event.admission_outcome() != StaticAdapterQuicAdmissionOutcome::Admitted {
        return;
    }
    assert_eq!(
        event.sender_edge_ref(),
        event.selected_receiver_retained_edge_ref(),
        "an admitted selected-QUIC ingress is exact source-edge equality, never same-family substitution"
    );
    assert_eq!(
        event.canonical_source_frame_ref(),
        event.transmitted_frame_ref(),
        "an admitted selected-QUIC ingress has no pre-send source-frame substitution"
    );
    assert_eq!(
        event.transmitted_frame_ref(),
        event.server_received_frame_ref(),
        "an admitted selected-QUIC ingress records the actual received transport frame"
    );
    assert_eq!(
        event.decoded_full_snapshot_ref(),
        event.selected_receiver_full_snapshot_ref(),
        "an admitted selected-QUIC ingress exact-admits the selected retained snapshot"
    );
}

fn assert_admitted_ingress(
    run: &StaticAdapterQuicRun,
    sender_inventory: &[SourceBoundAdapterEdge],
    receiver_inventory: &[SourceBoundAdapterEdge],
) {
    let expected_frames = expected_frame_references(sender_inventory);
    let receiver_by_edge_ref = receiver_inventory
        .iter()
        .map(|edge| (edge.edge_ref(), edge))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(receiver_by_edge_ref.len(), 12);
    assert_eq!(run.source_sender_inventory_count(), 12);
    assert_eq!(run.receiver_retained_inventory_count(), 12);
    assert_eq!(run.ingress_events().len(), 12);
    assert_eq!(
        run.admitted_handoffs(),
        sender_inventory,
        "receiver-selected contracts retain sender order without depending on receiver inventory position"
    );
    assert_eq!(run.admitted_family_counts(), expected_family_counts());

    let mut mapped_sender_refs = BTreeSet::new();
    for ((event, expected_frame), sender_edge) in run
        .ingress_events()
        .iter()
        .zip(expected_frames)
        .zip(sender_inventory)
    {
        mapped_sender_refs.insert(event.sender_edge_ref());
        assert_admitted_event_has_exact_sender_binding(event);
        assert_eq!(event.sender_edge_ref(), sender_edge.edge_ref());
        assert_eq!(event.untrusted_reference_hint(), sender_edge.edge_ref());
        assert_eq!(event.canonical_source_frame_ref(), expected_frame);
        assert_eq!(
            event.transmitted_frame_ref(),
            event.canonical_source_frame_ref(),
            "the normal selected seam transmits the canonical source-derived frame without in-flight mutation"
        );
        assert_eq!(
            event.server_received_frame_ref(),
            event.transmitted_frame_ref(),
            "the server records the actual QUIC bidirectional-stream frame, not a sender-side assertion"
        );
        assert_private_quic_reference(
            event.canonical_source_frame_ref(),
            "canonical source frame reference",
        );
        assert_private_quic_reference(event.transmitted_frame_ref(), "transmitted frame reference");
        assert_private_quic_reference(
            event.server_received_frame_ref(),
            "server receive reference",
        );
        assert_eq!(
            event.decoded_full_snapshot_ref(),
            private_static_adapter_snapshot_reference(sender_edge),
            "the server record names the decoded untrusted snapshot before receiver-owned exact admission"
        );
        let receiver_edge = *receiver_by_edge_ref
            .get(event.selected_receiver_retained_edge_ref())
            .expect("every sender ingress must resolve through the separately retained receiver inventory");
        assert_eq!(
            event.selected_receiver_retained_edge_ref(),
            sender_edge.edge_ref(),
            "the receiver may select only its separately retained source inventory by untrusted edge-ref hint"
        );
        assert_eq!(
            event.selected_receiver_full_snapshot_ref(),
            private_static_adapter_snapshot_reference(receiver_edge)
        );
        assert_eq!(
            event.admission_outcome(),
            StaticAdapterQuicAdmissionOutcome::Admitted
        );
    }
    assert_eq!(
        mapped_sender_refs,
        sender_inventory
            .iter()
            .map(|edge| edge.edge_ref())
            .collect::<BTreeSet<_>>(),
        "all twelve sender edges must map through receiver lookup even when its inventory is permuted"
    );

    for edge in run.admitted_handoffs() {
        assert!(edge.checked_core_bound());
        assert!(!edge.transfers_authority());
        assert!(!edge.mints_authority_without_source());
        assert!(!edge.public_api_or_wire_contract());
    }
}

fn assert_rejected_static_ingress(
    run: &StaticAdapterQuicRun,
    source_sender_inventory_count: usize,
) {
    assert_eq!(
        run.platform_claim(),
        StaticAdapterQuicPlatformClaim::LinuxX86_64LocalhostOnly
    );
    assert_transport_records(run, 1);
    assert_eq!(
        run.source_sender_inventory_count(),
        source_sender_inventory_count,
        "the evidence census counts only the source sender edges actually supplied to this run"
    );
    assert_eq!(run.receiver_retained_inventory_count(), 12);
    assert_eq!(run.ingress_events().len(), 1);
    assert!(run.admitted_handoffs().is_empty());
    assert_eq!(run.admitted_family_counts(), BTreeMap::new());

    let event = &run.ingress_events()[0];
    for (label, reference) in [
        ("sender edge", event.sender_edge_ref()),
        ("untrusted reference hint", event.untrusted_reference_hint()),
        ("decoded full snapshot", event.decoded_full_snapshot_ref()),
        (
            "selected receiver full snapshot",
            event.selected_receiver_full_snapshot_ref(),
        ),
        (
            "selected receiver edge",
            event.selected_receiver_retained_edge_ref(),
        ),
        ("canonical source frame", event.canonical_source_frame_ref()),
        ("transmitted frame", event.transmitted_frame_ref()),
        ("server received frame", event.server_received_frame_ref()),
    ] {
        assert!(!reference.is_empty(), "{label} evidence must be non-empty");
    }
    assert_private_quic_reference(
        event.canonical_source_frame_ref(),
        "falsifier canonical source frame reference",
    );
    assert_private_quic_reference(
        event.transmitted_frame_ref(),
        "falsifier transmitted frame reference",
    );
    assert_private_quic_reference(
        event.server_received_frame_ref(),
        "falsifier server receive reference",
    );
    assert_eq!(
        event.admission_outcome(),
        StaticAdapterQuicAdmissionOutcome::Rejected(
            StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch
        )
    );
    assert_exact_observer_schema(run);
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_requires_the_closed_twelve_sender_finite_profile_before_transport_or_falsifier_slicing()
 {
    let receiver = reversed_receiver_inventory();
    assert_eq!(
        receiver.len(),
        12,
        "receiver inventory is the accepted closed profile"
    );

    for count in [2, 11, 13] {
        let sender = invalid_sender_inventory(count);
        assert_eq!(sender.len(), count);

        let normal = match run_static_adapter_quic_loopback(&sender, &receiver) {
            Ok(_) => {
                panic!(
                    "non-profile sender inventory must fail before any selected QUIC evidence run"
                )
            }
            Err(error) => error,
        };
        assert_eq!(
            normal.kind(),
            StaticAdapterQuicRunErrorKind::InvalidFiniteProfileInventory,
            "normal selected QUIC entry rejects {count}-edge sender inventory before socket/handshake/stream evidence exists"
        );

        for falsifier in [
            StaticAdapterQuicFalsifier::WrongRetainedReferenceHint,
            StaticAdapterQuicFalsifier::TamperedTargetLocus,
        ] {
            let error = match run_static_adapter_quic_loopback_with_falsifier(
                &sender, &receiver, falsifier,
            ) {
                Ok(_) => panic!(
                    "falsifier entry must validate the complete finite sender profile before selecting its representative one-edge ingress"
                ),
                Err(error) => error,
            };
            assert_eq!(
                error.kind(),
                StaticAdapterQuicRunErrorKind::InvalidFiniteProfileInventory,
                "{falsifier:?} cannot turn a {count}-edge sender inventory into an accepted one-edge profile"
            );
        }
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_reliable_bidi_seam_records_actual_server_ingress_against_a_separate_retained_receiver_inventory()
 {
    let sender_first = source_inventory();
    let receiver_first = reversed_receiver_inventory();
    let sender_second = source_inventory();
    let receiver_second = reversed_receiver_inventory();
    assert_ne!(sender_first, receiver_first);
    assert_ne!(sender_second, receiver_second);

    let first = run_static_adapter_quic_loopback(&sender_first, &receiver_first)
        .expect("the selected private QUIC seam must admit the complete static source inventory");
    let second = run_static_adapter_quic_loopback(&sender_second, &receiver_second)
        .expect("a fresh private QUIC session must preserve the same static admissions");

    for (run, sender, receiver) in [
        (&first, sender_first.as_slice(), receiver_first.as_slice()),
        (
            &second,
            sender_second.as_slice(),
            receiver_second.as_slice(),
        ),
    ] {
        assert_eq!(
            run.platform_claim(),
            StaticAdapterQuicPlatformClaim::LinuxX86_64LocalhostOnly,
            "this test is explicitly Linux x86_64 localhost evidence, not macOS/Windows/browser/WAN/production support"
        );
        assert_transport_records(run, 12);
        assert_admitted_ingress(run, sender, receiver);
        assert_exact_observer_schema(run);
    }

    assert_eq!(
        first.ingress_events(),
        second.ingress_events(),
        "session/certificate churn cannot alter source-bound ingress/decode/admission records"
    );
    let first_transport_refs = first
        .transport_events()
        .iter()
        .map(|event| event.evidence_ref())
        .collect::<BTreeSet<_>>();
    let second_transport_refs = second
        .transport_events()
        .iter()
        .map(|event| event.evidence_ref())
        .collect::<BTreeSet<_>>();
    assert!(
        first_transport_refs.is_disjoint(&second_transport_refs),
        "fresh certificates/sessions must be derived into fresh transport evidence, never used as semantic identity"
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_observer_validator_rejects_raw_endpoint_and_secret_or_certificate_canary_encodings()
 {
    let sender = source_inventory();
    let receiver = reversed_receiver_inventory();
    let run = run_static_adapter_quic_loopback(&sender, &receiver)
        .expect("the selected private QUIC seam must provide non-vacuous observer evidence");
    let observer = run.observer_safe_evidence();

    for replacement in [
        "127.0.0.1",
        "::1",
        "4D49522D544553542D5345435245542D43414E415259",
        "TUlSLVRFU1QtU0VDUkVULUNBTkFSWQ==",
    ] {
        let malformed = replace_first_private_reference(observer, replacement);
        let error = validate_static_adapter_quic_observer_evidence(&malformed).expect_err(
            "endpoint/raw-hex/raw-base64 material cannot occupy a private reference field",
        );
        assert_eq!(
            error.kind(),
            StaticAdapterQuicObserverErrorKind::InvalidPrivateReference
        );
    }

    let raw_certificate = replace_first_private_reference(
        observer,
        "-----BEGIN CERTIFICATE----- MIR-TEST-CERTIFICATE-CANARY",
    );
    let error = validate_observer_safe_evidence(&raw_certificate)
        .expect_err("the repository-wide observer validator rejects raw certificate text");
    assert_eq!(
        error.kind(),
        ObserverEvidenceErrorKind::RawCertificateMaterial
    );

    for target in [
        NestedObserverObject::TransportEvent,
        NestedObserverObject::IngressEvent,
        NestedObserverObject::Summary,
    ] {
        let malformed = insert_unknown_nested_canary(observer, target);
        let error = validate_static_adapter_quic_observer_evidence(&malformed)
            .expect_err("nested unknown observer fields must fail the exact static-adapter schema");
        assert_eq!(
            error.kind(),
            StaticAdapterQuicObserverErrorKind::UnexpectedField
        );
    }

    for field in [
        "sender_edge_ref",
        "untrusted_reference_hint",
        "selected_receiver_retained_edge_ref",
    ] {
        let fabricated = fabricate_ingress_reference(observer, field);
        let error = validate_static_adapter_quic_observer_evidence(&fabricated)
            .expect_err("fabricated retained-edge references must fail exact observer validation");
        assert_eq!(
            error.kind(),
            StaticAdapterQuicObserverErrorKind::MalformedEvidence,
            "observer field {field} must be a retained source-edge reference, not arbitrary text"
        );
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_observer_validator_rejects_altered_summary_counts_and_missing_or_incomplete_transport_inventory()
 {
    let sender = source_inventory();
    let receiver = reversed_receiver_inventory();
    let run = run_static_adapter_quic_loopback(&sender, &receiver)
        .expect("the selected private QUIC seam must provide canonical observer evidence");
    let observer = run.observer_safe_evidence();

    for field in [
        "source_sender_inventory_count",
        "receiver_retained_inventory_count",
        "ingress_event_count",
        "admitted_handoff_count",
    ] {
        let altered = alter_summary_count(observer, field);
        let error = validate_static_adapter_quic_observer_evidence(&altered)
            .expect_err("an altered observer summary count must not pass exact validation");
        assert_eq!(
            error.kind(),
            StaticAdapterQuicObserverErrorKind::MalformedEvidence,
            "summary count {field} must correspond to actual observer inventory"
        );
    }
    let jointly_altered = alter_summary_count(
        &alter_summary_count(observer, "source_sender_inventory_count"),
        "ingress_event_count",
    );
    let error = validate_static_adapter_quic_observer_evidence(&jointly_altered).expect_err(
        "self-consistent-looking source and ingress count changes must still fail against the actual observer events",
    );
    assert_eq!(
        error.kind(),
        StaticAdapterQuicObserverErrorKind::MalformedEvidence
    );
    let altered_family = alter_summary_family_count(observer, "owner-request");
    let error = validate_static_adapter_quic_observer_evidence(&altered_family)
        .expect_err("an altered observer family count must not pass exact validation");
    assert_eq!(
        error.kind(),
        StaticAdapterQuicObserverErrorKind::MalformedEvidence
    );

    let missing = remove_required_transport_inventory(observer);
    let error = validate_static_adapter_quic_observer_evidence(&missing)
        .expect_err("missing required transport inventory must fail the exact schema");
    assert_eq!(
        error.kind(),
        StaticAdapterQuicObserverErrorKind::UnexpectedField
    );

    for incomplete in [
        empty_required_transport_inventory(observer),
        remove_required_transport_kind(observer, "QuicHandshakeCompleted"),
    ] {
        let error = validate_static_adapter_quic_observer_evidence(&incomplete)
            .expect_err("an empty or incomplete required transport inventory must fail closed");
        assert_eq!(
            error.kind(),
            StaticAdapterQuicObserverErrorKind::MalformedEvidence
        );
    }
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
#[test]
fn selected_quic_loopback_is_typed_unsupported_outside_the_explicit_linux_x86_64_scope() {
    let sender = source_inventory();
    let receiver = reversed_receiver_inventory();
    let error = run_static_adapter_quic_loopback(&sender, &receiver)
        .expect_err("this private selected-adapter run has no non-Linux/x86_64 support claim");
    assert_eq!(
        error.kind(),
        StaticAdapterQuicRunErrorKind::UnsupportedPlatform
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_receiver_lookup_is_a_function_of_the_same_received_private_ingress_bytes_only() {
    let sender = source_inventory();
    let receiver = reversed_receiver_inventory();
    let sender_edge = &sender[0];
    let wrong_receiver_edge = receiver
        .iter()
        .find(|edge| edge.edge_ref() != sender_edge.edge_ref())
        .expect("the retained receiver inventory contains a different edge for the wrong-hint falsifier");
    let canonical_frame = encode_static_adapter_frame(sender_edge)
        .expect("the source-derived sender edge has one canonical private static frame");
    let received_ingress = encode_private_static_adapter_quic_ingress_for_test(
        wrong_receiver_edge.edge_ref(),
        &canonical_frame,
    )
    .expect("the test must construct one complete private selected-QUIC ingress envelope");

    // Both runs receive bit-identical lookup-hint/frame bytes.  This
    // doc-hidden test seam deliberately accepts no falsifier enum: receiver
    // selection must therefore be a function of `received_hint` from those
    // bytes, never a pre-send/out-of-band test branch.
    let first = run_static_adapter_quic_loopback_from_private_ingress(
        sender_edge,
        &receiver,
        &received_ingress,
    )
    .expect("the received-ingress seam reaches typed actual QUIC admission");
    let second = run_static_adapter_quic_loopback_from_private_ingress(
        sender_edge,
        &receiver,
        &received_ingress,
    )
    .expect(
        "the same private ingress bytes remain semantically deterministic across fresh sessions",
    );

    assert_eq!(
        first.ingress_events(),
        second.ingress_events(),
        "identical received private ingress bytes cannot acquire a different receiver interpretation from out-of-band falsifier state"
    );
    assert_rejected_static_ingress(&first, 1);
    let event = &first.ingress_events()[0];
    assert_eq!(event.sender_edge_ref(), sender_edge.edge_ref());
    assert_eq!(
        event.untrusted_reference_hint(),
        wrong_receiver_edge.edge_ref(),
        "the observed receiver lookup hint is decoded from the transmitted ingress bytes"
    );
    assert_eq!(
        event.selected_receiver_retained_edge_ref(),
        wrong_receiver_edge.edge_ref(),
        "receiver selection must use the transmitted received_hint before exact full-snapshot admission"
    );
    assert_eq!(
        event.decoded_full_snapshot_ref(),
        private_static_adapter_snapshot_reference(sender_edge),
        "the canonical transmitted frame still decodes to the original sender contract"
    );
    assert_eq!(
        event.selected_receiver_full_snapshot_ref(),
        private_static_adapter_snapshot_reference(wrong_receiver_edge)
    );
    assert_ne!(
        event.decoded_full_snapshot_ref(),
        event.selected_receiver_full_snapshot_ref(),
        "a hinted different retained edge is rejected by exact static snapshot admission"
    );
    assert_eq!(
        event.admission_outcome(),
        StaticAdapterQuicAdmissionOutcome::Rejected(
            StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch
        )
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_same_family_private_ingress_substitution_never_admits_a_different_sender_edge() {
    let sender_inventory = source_inventory();
    let receiver_inventory = reversed_receiver_inventory();
    let sender_edge = sender_inventory
        .iter()
        .find(|edge| edge.edge_kind() == "owner-request")
        .expect("the closed source inventory retains owner-request edges");
    let same_family_receiver_edge = receiver_inventory
        .iter()
        .find(|edge| {
            edge.edge_kind() == sender_edge.edge_kind() && edge.edge_ref() != sender_edge.edge_ref()
        })
        .expect("the finite source inventory has a distinct owner-request edge in the same family");
    let substituted_frame = encode_static_adapter_frame(same_family_receiver_edge)
        .expect("the other same-family retained edge has a canonical private static frame");
    let private_ingress = encode_private_static_adapter_quic_ingress_for_test(
        same_family_receiver_edge.edge_ref(),
        &substituted_frame,
    )
    .expect("the test must construct a complete same-family substituted private ingress");

    let run = run_static_adapter_quic_loopback_from_private_ingress(
        sender_edge,
        &receiver_inventory,
        &private_ingress,
    )
    .expect("the complete substituted ingress reaches typed fail-closed selected-QUIC admission");
    let event = &run.ingress_events()[0];
    assert_admitted_event_has_exact_sender_binding(event);
    assert_rejected_static_ingress(&run, 1);

    assert_eq!(event.sender_edge_ref(), sender_edge.edge_ref());
    assert_eq!(
        event.untrusted_reference_hint(),
        same_family_receiver_edge.edge_ref()
    );
    assert_eq!(
        event.selected_receiver_retained_edge_ref(),
        same_family_receiver_edge.edge_ref()
    );
    assert_ne!(
        event.sender_edge_ref(),
        event.selected_receiver_retained_edge_ref(),
        "same carrier family is not an exact source-edge admission identity"
    );
    assert_ne!(
        event.canonical_source_frame_ref(),
        event.transmitted_frame_ref(),
        "the substituted B frame differs from the canonical source A frame"
    );
    assert_eq!(
        event.transmitted_frame_ref(),
        event.server_received_frame_ref(),
        "the server evidence retains the actual same-family substituted transport bytes"
    );
    assert_eq!(
        event.decoded_full_snapshot_ref(),
        event.selected_receiver_full_snapshot_ref(),
        "the B snapshot may exactly match B, but it cannot be admitted on behalf of source A"
    );
    assert_eq!(
        event.admission_outcome(),
        StaticAdapterQuicAdmissionOutcome::Rejected(
            StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch
        )
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_wrong_retained_reference_hint_reaches_actual_ingress_but_selects_a_different_receiver_contract()
 {
    let sender = source_inventory();
    let receiver = reversed_receiver_inventory();
    let run = run_static_adapter_quic_loopback_with_falsifier(
        &sender,
        &receiver,
        StaticAdapterQuicFalsifier::WrongRetainedReferenceHint,
    )
    .expect("the private QUIC wrong-hint falsifier must reach a typed server outcome");

    assert_rejected_static_ingress(&run, 12);
    let event = &run.ingress_events()[0];
    let sender_edge = &sender[0];
    let receiver_by_edge_ref = receiver
        .iter()
        .map(|edge| (edge.edge_ref(), edge))
        .collect::<BTreeMap<_, _>>();
    let selected_receiver_edge = *receiver_by_edge_ref
        .get(event.selected_receiver_retained_edge_ref())
        .expect("the wrong hint still selects only an independently retained receiver edge");

    assert_eq!(event.sender_edge_ref(), sender_edge.edge_ref());
    assert_eq!(
        event.canonical_source_frame_ref(),
        expected_frame_references(std::slice::from_ref(sender_edge))[0],
        "the wrong-hint run retains the pre-falsifier source frame identity"
    );
    assert_eq!(
        event.canonical_source_frame_ref(),
        event.transmitted_frame_ref(),
        "wrong retained hints do not mutate the source-derived frame before transport"
    );
    assert_eq!(
        event.transmitted_frame_ref(),
        event.server_received_frame_ref(),
        "the server must record the actual unmodified QUIC stream frame"
    );
    assert_eq!(
        event.decoded_full_snapshot_ref(),
        private_static_adapter_snapshot_reference(sender_edge),
        "the received frame still decodes to the exact sender contract snapshot"
    );
    assert_ne!(event.untrusted_reference_hint(), sender_edge.edge_ref());
    assert_eq!(
        event.selected_receiver_retained_edge_ref(),
        event.untrusted_reference_hint(),
        "the selected receiver edge comes from only the untrusted reference hint before full comparison"
    );
    assert_ne!(
        event.selected_receiver_retained_edge_ref(),
        sender_edge.edge_ref()
    );
    assert_eq!(
        event.selected_receiver_full_snapshot_ref(),
        private_static_adapter_snapshot_reference(selected_receiver_edge)
    );
    assert_ne!(
        event.decoded_full_snapshot_ref(),
        event.selected_receiver_full_snapshot_ref(),
        "the wrong receiver selection must fail full static snapshot admission"
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn selected_quic_tampered_target_reaches_actual_ingress_but_never_retargets_receiver_lookup() {
    let sender = source_inventory();
    let receiver = reversed_receiver_inventory();
    let run = run_static_adapter_quic_loopback_with_falsifier(
        &sender,
        &receiver,
        StaticAdapterQuicFalsifier::TamperedTargetLocus,
    )
    .expect("the private QUIC tampered-target falsifier must reach a typed server outcome");

    assert_rejected_static_ingress(&run, 12);
    let event = &run.ingress_events()[0];
    let sender_edge = &sender[0];
    let receiver_by_edge_ref = receiver
        .iter()
        .map(|edge| (edge.edge_ref(), edge))
        .collect::<BTreeMap<_, _>>();
    let selected_receiver_edge = *receiver_by_edge_ref
        .get(event.selected_receiver_retained_edge_ref())
        .expect("the original untrusted hint must still select a receiver-owned retained edge");

    assert_eq!(event.sender_edge_ref(), sender_edge.edge_ref());
    assert_eq!(
        event.canonical_source_frame_ref(),
        expected_frame_references(std::slice::from_ref(sender_edge))[0],
        "the target-tamper run retains a reference to the source-derived frame before falsifier mutation"
    );
    assert_eq!(event.untrusted_reference_hint(), sender_edge.edge_ref());
    assert_eq!(
        event.selected_receiver_retained_edge_ref(),
        sender_edge.edge_ref(),
        "tampering a target cannot retarget receiver inventory selection"
    );
    assert_eq!(
        event.selected_receiver_full_snapshot_ref(),
        private_static_adapter_snapshot_reference(selected_receiver_edge)
    );
    assert_ne!(
        event.canonical_source_frame_ref(),
        event.transmitted_frame_ref(),
        "target tampering must be represented as a pre-send transformation of the canonical source frame"
    );
    assert_eq!(
        event.transmitted_frame_ref(),
        event.server_received_frame_ref(),
        "the server receive record identifies the actual tampered bytes that traversed the reliable bidirectional stream"
    );
    assert_ne!(
        event.decoded_full_snapshot_ref(),
        event.selected_receiver_full_snapshot_ref(),
        "the decoded tampered target differs from the exact retained receiver snapshot"
    );
}
