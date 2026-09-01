// This integration target exercises the probe facade; direct candidate dependencies live in the library and child binary.
#![allow(unused_crate_dependencies)]

use std::path::PathBuf;

use mirrorea_i3_probe::{SemanticRequestSeed, build_source_bound_probe};

const ACTIVE_I2_SOURCE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"
);

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
