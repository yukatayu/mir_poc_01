// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
    path::PathBuf,
};

use mir_runtime::sys5_local_slice::{
    Sys5I3AdapterCarrierContract, Sys5I3ProbeCarrierContract, Sys5SourceInput, build_project,
};
use mirrorea_i3_probe::{
    SemanticRequestSeed, SourceBoundAdapterEdge, SourceBoundEdge, SourceBoundProbe,
    build_source_bound_probe,
};

const ACTIVE_I2_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"
);

fn adapter_carrier_edges(probe: &SourceBoundProbe) -> &[SourceBoundAdapterEdge] {
    probe.adapter_carrier_edges()
}

fn assert_source_bound_adapter_edge_matches_adapter_contract(
    edge: &SourceBoundAdapterEdge,
    contract: &Sys5I3AdapterCarrierContract,
) {
    assert_eq!(edge.program_ref(), contract.checked_program_ref());
    assert_eq!(edge.operation(), contract.operation_id());
    assert_eq!(edge.edge_kind(), contract.edge_kind());
    assert_eq!(edge.lifecycle_kind(), contract.lifecycle_kind());
    assert_eq!(edge.source_locus(), contract.source_locus());
    assert_eq!(edge.target_locus(), contract.target_locus());
    assert_eq!(edge.logical_source_path(), contract.logical_source_path());
    assert_eq!(edge.source_span(), contract.source_span());
    assert_eq!(edge.source_ref(), contract.source_ref());
    assert_eq!(edge.core_ref(), contract.core_ref());
    assert_eq!(edge.source_artifact_ref(), contract.source_artifact_ref());
    assert_eq!(edge.target_artifact_ref(), contract.target_artifact_ref());
    assert_eq!(edge.edge_ref(), contract.edge_ref());
    assert_eq!(
        edge.declared_failure_names(),
        contract.declared_failure_names()
    );
    assert_eq!(edge.effect_kind_names(), contract.effect_kind_names());
    assert_eq!(
        edge.required_occurrence_slot_names(),
        contract.required_occurrence_slot_names()
    );
    assert_eq!(
        edge.requires_linked_request_identity(),
        contract.requires_linked_request_identity()
    );
    assert_eq!(
        edge.requires_typed_outcome(),
        contract.requires_typed_outcome()
    );
    assert_eq!(
        edge.requires_receipt_consumption_state(),
        contract.requires_receipt_consumption_state()
    );
    assert_eq!(
        edge.authority_category_names(),
        contract.authority_requirements().category_names()
    );
    assert_eq!(
        edge.authority_requirement_rows(),
        contract.authority_requirements().rows()
    );
    assert_eq!(
        edge.requires_membership_epoch_and_incarnation(),
        contract
            .authority_requirements()
            .requires_membership_epoch_and_incarnation()
    );
    assert_eq!(
        edge.requires_capability_and_witness_refs(),
        contract
            .authority_requirements()
            .requires_capability_and_witness_refs()
    );
    assert_eq!(edge.redaction(), contract.redaction());
    assert_eq!(edge.checked_core_bound(), contract.checked_core_bound());
    assert_eq!(edge.transfers_authority(), contract.transfers_authority());
    assert_eq!(
        edge.mints_authority_without_source(),
        contract.mints_authority_without_source()
    );
    assert_eq!(
        edge.public_api_or_wire_contract(),
        contract.public_api_or_wire_contract()
    );
    assert_eq!(
        edge.retained_contract_fingerprint(),
        contract.full_retained_contract_fingerprint()
    );
    assert_eq!(edge.variant_facts(), contract.variant_facts());
}

#[test]
fn source_bound_probe_uses_the_retained_attack_contract_without_network_input() {
    let probe = build_source_bound_probe(PathBuf::from(ACTIVE_I2_SOURCE))
        .expect("the accepted I2 source must build a source-bound private I3 probe");
    let edge = probe
        .owner_request_edge("attack")
        .expect("the retained I2 projection must expose the attack owner request");

    assert_eq!(edge.operation(), "attack");
    assert_eq!(edge.source_locus(), "ParticipantA");
    assert_eq!(edge.target_locus(), "WorldAuthority");
    assert_eq!(edge.edge_kind(), "owner-request");
    assert_eq!(edge.lifecycle_kind(), "owner-request");
    for (label, reference) in [
        ("source", edge.source_ref()),
        ("Core", edge.core_ref()),
        ("source artifact", edge.source_artifact_ref()),
        ("target artifact", edge.target_artifact_ref()),
        ("generated edge", edge.edge_ref()),
        ("checked program", edge.program_ref()),
    ] {
        assert!(
            !reference.is_empty(),
            "the exact retained {label} reference must remain non-empty"
        );
    }
    assert_eq!(edge.program_ref(), probe.program_ref());
    assert_eq!(
        edge.declared_failure_names().to_vec(),
        vec![
            "StaleMembership".to_string(),
            "MissingCapability".to_string(),
            "MissingWitness".to_string(),
            "VisibilityDenied".to_string(),
            "RouteUnavailable".to_string(),
        ]
    );
    assert_eq!(
        edge.effect_kind_names().to_vec(),
        vec![
            "OwnerRequest".to_string(),
            "OwnerLocalRead".to_string(),
            "OwnerWrite".to_string(),
            "ObserverPublish".to_string(),
        ],
        "the probe must not substitute projected artifact kinds for checked EffectKind names"
    );
    assert_eq!(
        edge.required_occurrence_slot_names().to_vec(),
        vec!["Request".to_string()]
    );
    assert!(edge.requires_membership_epoch_and_incarnation());
    assert!(edge.requires_capability_and_witness_refs());
    assert!(edge.checked_core_bound());
    assert!(edge.reference_only_redaction());
    assert!(!edge.transfers_authority());

    let source_text = std::fs::read_to_string(ACTIVE_I2_SOURCE)
        .expect("the committed active source remains readable for legacy comparison");
    let project = build_project(Sys5SourceInput::inline(
        "samples/clean-near-end/mirrorea-i2-local-toy/main.mir",
        source_text,
    ))
    .expect("the same source must retain the I3-0 owner-request facade");
    let legacy_contract: Sys5I3ProbeCarrierContract = project
        .i3_probe_carrier_contract(edge.edge_ref())
        .expect("the legacy owner-request edge must resolve through the I3-0 facade");
    assert_eq!(
        edge.retained_contract_fingerprint(),
        legacy_contract.full_retained_contract_fingerprint(),
        "owner_request_edge must retain the original I3-0 contract and fingerprint"
    );
}

#[test]
fn source_bound_probe_errors_are_milestone_neutral() {
    let missing_source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("does-not-exist-source-bound-probe-input.mir");
    let error = build_source_bound_probe(&missing_source)
        .expect_err("a missing source must still yield a typed private probe error");
    assert_eq!(error.to_string(), "I3 source input is unreadable");
}

#[test]
fn legacy_owner_request_identity_and_serialization_survive_adapter_inventory() {
    let probe = build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must build a source-bound private I3 probe");
    let legacy_edge: &SourceBoundEdge = probe
        .owner_request_edge("attack")
        .expect("the I3-0 owner request remains separately selectable");
    let seed = SemanticRequestSeed::new("legacy-owner-request-invocation-01");
    let before_inventory = legacy_edge
        .bind_semantic_request(seed.clone())
        .expect("the legacy owner request binds before adapter inventory observation");

    let inventory: &[SourceBoundAdapterEdge] = adapter_carrier_edges(&probe);
    assert_eq!(inventory.len(), 12);
    let after_inventory = legacy_edge
        .bind_semantic_request(seed.clone())
        .expect("adapter inventory observation cannot alter legacy request binding");
    assert_eq!(
        before_inventory.request_identity(),
        after_inventory.request_identity(),
        "a fixed seed keeps the original I3-0 RequestIdentity"
    );

    let encoded = serde_json::to_string(legacy_edge)
        .expect("the original I3-0 SourceBoundEdge remains serializable");
    let decoded: SourceBoundEdge = serde_json::from_str(&encoded)
        .expect("the original I3-0 SourceBoundEdge remains deserializable");
    assert_eq!(decoded, *legacy_edge);
    assert_eq!(
        decoded
            .bind_semantic_request(seed)
            .expect("a decoded legacy owner edge retains its I3-0 binding")
            .request_identity(),
        before_inventory.request_identity()
    );
}

#[test]
fn adapter_inventory_is_nominally_distinct_and_non_serde() {
    assert_ne!(
        TypeId::of::<SourceBoundAdapterEdge>(),
        TypeId::of::<SourceBoundEdge>()
    );

    struct Check<T>(PhantomData<T>);
    trait AmbiguousIfSerialize<A> {
        fn probe() {}
    }
    impl<T> AmbiguousIfSerialize<()> for Check<T> {}
    impl<T: serde::Serialize> AmbiguousIfSerialize<u8> for Check<T> {}

    trait AmbiguousIfDeserializeOwned<A> {
        fn probe() {}
    }
    impl<T> AmbiguousIfDeserializeOwned<()> for Check<T> {}
    impl<T: serde::de::DeserializeOwned> AmbiguousIfDeserializeOwned<u8> for Check<T> {}

    <Check<SourceBoundAdapterEdge> as AmbiguousIfSerialize<_>>::probe();
    <Check<SourceBoundAdapterEdge> as AmbiguousIfDeserializeOwned<_>>::probe();
}

#[test]
fn semantic_request_identity_is_seeded_separately_from_edge_and_network_occurrence() {
    let probe = build_source_bound_probe(ACTIVE_I2_SOURCE)
        .expect("the accepted I2 source must build a source-bound private I3 probe");
    let edge = probe
        .owner_request_edge("attack")
        .expect("the retained I2 projection must expose the attack owner request");

    let retry_seed = SemanticRequestSeed::new("attack-invocation-01");
    let first = edge
        .bind_semantic_request(retry_seed.clone())
        .expect("a well-formed invocation seed binds a semantic request");
    let retry = edge
        .bind_semantic_request(retry_seed)
        .expect("the same invocation seed represents an explicit retry");
    let next = edge
        .bind_semantic_request(SemanticRequestSeed::new("attack-invocation-02"))
        .expect("a new invocation seed represents a different semantic request");

    assert_eq!(first.request_identity(), retry.request_identity());
    assert_ne!(first.request_identity(), next.request_identity());
    assert_ne!(
        first.request_identity().as_str(),
        edge.edge_ref(),
        "semantic request identity must not collapse into generated-edge identity"
    );
    assert_eq!(
        first.retained_contract_fingerprint(),
        edge.retained_contract_fingerprint(),
        "the bound carrier must retain the full checked contract fingerprint"
    );
    assert_eq!(
        first.request_identity().retained_contract_fingerprint(),
        edge.retained_contract_fingerprint(),
        "retry identity evidence must name the full retained contract it binds"
    );
    assert_eq!(
        retry.request_identity().retained_contract_fingerprint(),
        first.request_identity().retained_contract_fingerprint(),
        "an explicit retry keeps its original semantic request/fingerprint binding"
    );
}

#[test]
fn source_bound_probe_exposes_exactly_the_six_accepted_generated_adapter_families() {
    let probe = build_source_bound_probe(PathBuf::from(ACTIVE_I2_SOURCE))
        .expect("the accepted I2 source must build a source-bound private I3 probe");
    let source_text = std::fs::read_to_string(ACTIVE_I2_SOURCE).expect(
        "the committed active I2 source must remain readable for source/projection comparison",
    );
    let project = build_project(Sys5SourceInput::inline(
        "samples/clean-near-end/mirrorea-i2-local-toy/main.mir",
        source_text,
    ))
    .expect("the same ordinary source must project before adapter inventory comparison");
    let accepted_family_kinds = project.i3_adapter_accepted_family_kind_names();
    let generated_edges = &project.semantic_summary().generated_communication;
    assert_eq!(generated_edges.len(), 12);
    assert!(
        generated_edges
            .iter()
            .all(|edge| edge.derived_from_checked_core),
        "the ordinary active source must not retain a non-derived generated edge"
    );
    assert!(
        generated_edges
            .iter()
            .all(|edge| accepted_family_kinds.contains(&edge.kind.as_str())),
        "a generated seventh family must not silently enter the closed adapter inventory"
    );
    let expected_edges = generated_edges
        .iter()
        .filter(|edge| {
            edge.derived_from_checked_core && accepted_family_kinds.contains(&edge.kind.as_str())
        })
        .collect::<Vec<_>>();

    let adapter_edges: &[SourceBoundAdapterEdge] = adapter_carrier_edges(&probe);
    assert_eq!(adapter_edges.len(), 12);
    assert_eq!(adapter_edges.len(), expected_edges.len());

    let expected_refs = expected_edges
        .iter()
        .map(|edge| edge.edge_ref.as_str())
        .collect::<BTreeSet<_>>();
    let actual_refs: BTreeSet<&str> = adapter_edges
        .iter()
        .map(|edge: &SourceBoundAdapterEdge| edge.edge_ref())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_refs, expected_refs,
        "the source builder must take every accepted edge reference from checked projection, never a handwritten route"
    );

    let expected_by_ref = expected_edges
        .iter()
        .map(|&edge| (edge.edge_ref.as_str(), edge))
        .collect::<BTreeMap<_, _>>();
    for adapter_edge in adapter_edges {
        let adapter_edge: &SourceBoundAdapterEdge = adapter_edge;
        let expected = expected_by_ref
            .get(adapter_edge.edge_ref())
            .expect("every exposed adapter edge must be an exact checked-projection edge");
        assert_eq!(adapter_edge.program_ref(), probe.program_ref());
        assert_eq!(
            adapter_edge.program_ref(),
            expected.checked_program_identity
        );
        assert_eq!(adapter_edge.operation(), expected.operation_id);
        assert_eq!(adapter_edge.edge_kind(), expected.kind);
        assert_eq!(adapter_edge.lifecycle_kind(), expected.kind);
        assert_eq!(adapter_edge.source_locus(), expected.from_locus);
        assert_eq!(adapter_edge.target_locus(), expected.to_locus);
        assert_eq!(
            adapter_edge.core_ref(),
            expected.core_ref.as_deref().unwrap()
        );
        assert_eq!(
            adapter_edge.source_artifact_ref(),
            expected.source_fragment_ref
        );
        assert_eq!(
            adapter_edge.target_artifact_ref(),
            expected.target_fragment_ref
        );
        let contract: Sys5I3AdapterCarrierContract = project
            .i3_adapter_carrier_contract(adapter_edge.edge_ref())
            .expect("every exposed source-probe edge must retain its exact adapter contract");
        assert_source_bound_adapter_edge_matches_adapter_contract(adapter_edge, &contract);
    }

    let family_counts: BTreeMap<&str, usize> = adapter_edges.iter().fold(
        BTreeMap::new(),
        |mut counts: BTreeMap<&str, usize>, edge: &SourceBoundAdapterEdge| {
            *counts.entry(edge.edge_kind()).or_insert(0_usize) += 1;
            counts
        },
    );
    assert_eq!(
        family_counts,
        BTreeMap::from([
            ("designated-input-receipt", 1),
            ("designated-input-request", 1),
            ("designated-result-delivery", 1),
            ("owner-reply-receipt", 4),
            ("owner-request", 4),
            ("relation-projection-publication", 1),
        ])
    );
    assert_eq!(
        family_counts.keys().copied().collect::<BTreeSet<_>>(),
        accepted_family_kinds
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "the source probe must expose all and only the closed adapter family inventory"
    );
}
