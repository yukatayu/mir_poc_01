use mir_runtime::sys5_local_slice::{
    Sys5I3ProbeFacadeErrorKind, Sys5I3ProbeRedaction, Sys5SourceInput, build_project,
};

const ACTIVE_I2_LOGICAL_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const ACTIVE_I2_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

fn active_project() -> mir_runtime::sys5_local_slice::Sys5LocalProject {
    build_project(Sys5SourceInput::inline(
        ACTIVE_I2_LOGICAL_PATH,
        ACTIVE_I2_SOURCE,
    ))
    .expect("the accepted I2 source must build before a retained-projection lookup")
}

#[test]
fn i3_probe_facade_selects_the_exact_retained_attack_contract_by_edge_ref() {
    let project = active_project();
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| {
            edge.kind == "owner-request"
                && edge.operation_id == "attack"
                && edge.from_locus == "ParticipantA"
                && edge.to_locus == "WorldAuthority"
        })
        .expect("the accepted projection must retain the attack owner-request edge");

    let contract = project
        .i3_probe_carrier_contract(&edge.edge_ref)
        .expect("the exact retained edge must yield one observer-safe contract snapshot");

    assert_eq!(
        contract.checked_program_ref(),
        edge.checked_program_identity
    );
    assert_eq!(contract.operation_id(), "attack");
    assert_eq!(contract.edge_kind(), "owner-request");
    assert_eq!(contract.lifecycle_kind(), "owner-request");
    assert_eq!(contract.source_locus(), "ParticipantA");
    assert_eq!(contract.target_locus(), "WorldAuthority");
    assert_eq!(contract.core_ref(), edge.core_ref.as_deref().unwrap());
    assert_eq!(contract.source_artifact_ref(), edge.source_fragment_ref);
    assert_eq!(contract.target_artifact_ref(), edge.target_fragment_ref);
    assert_eq!(contract.edge_ref(), edge.edge_ref);
    assert!(
        !contract.source_ref().is_empty(),
        "the retained source reference must survive without reconstructing source text"
    );
    assert_eq!(
        contract.declared_failure_names().to_vec(),
        vec![
            "StaleMembership".to_string(),
            "MissingCapability".to_string(),
            "MissingWitness".to_string(),
            "VisibilityDenied".to_string(),
            "RouteUnavailable".to_string(),
        ]
    );
    assert_eq!(
        contract.effect_kind_names().to_vec(),
        vec![
            "OwnerRequest".to_string(),
            "OwnerLocalRead".to_string(),
            "OwnerWrite".to_string(),
            "ObserverPublish".to_string(),
        ],
        "effect facts must retain EffectKind names rather than projected artifact-kind labels"
    );
    assert_eq!(
        contract.required_occurrence_slot_names().to_vec(),
        vec!["Request".to_string()]
    );

    let authority = contract.authority_requirements();
    assert!(authority.requires_membership_epoch_and_incarnation());
    assert!(authority.requires_capability_and_witness_refs());
    assert_eq!(
        authority.category_names().to_vec(),
        vec![
            "MembershipEpochIncarnation".to_string(),
            "OwnerCapabilityRef".to_string(),
            "OwnerWitnessRef".to_string(),
        ],
        "the façade reports categories only, never authority/capability/witness values"
    );
    assert_eq!(contract.redaction(), Sys5I3ProbeRedaction::ReferenceOnly);
    assert!(contract.checked_core_bound());
    assert!(!contract.transfers_authority());

    let observer_rendered = format!("{contract:?}");
    for forbidden in [
        ACTIVE_I2_SOURCE,
        env!("CARGO_MANIFEST_DIR"),
        "MembershipAuth",
        "avatar[target].hp",
        "principal self",
    ] {
        assert!(
            !observer_rendered.contains(forbidden),
            "the façade snapshot must retain references only, not {forbidden:?}"
        );
    }
}

#[test]
fn i3_probe_facade_rejects_an_absent_edge_ref() {
    let project = active_project();

    let error = project
        .i3_probe_carrier_contract("missing-generated-edge-ref")
        .expect_err("a probe must not reconstruct a contract when no exact retained edge exists");
    assert_eq!(error.kind(), Sys5I3ProbeFacadeErrorKind::UnknownEdgeRef);
}

#[test]
fn i3_probe_facade_exposes_the_full_owner_request_contract_as_reference_only_facts() {
    let project = active_project();
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.kind == "owner-request" && edge.operation_id == "attack")
        .expect("the accepted projection must retain attack as an owner request");
    let contract = project
        .i3_probe_carrier_contract(&edge.edge_ref)
        .expect("the retained attack edge must expose its full observer-safe contract");
    let same_contract = project
        .i3_probe_carrier_contract(&edge.edge_ref)
        .expect("a second retained lookup must preserve the same contract");

    let fingerprint = contract.full_retained_contract_fingerprint();
    assert_eq!(
        fingerprint,
        same_contract.full_retained_contract_fingerprint(),
        "one retained carrier contract must have a stable full-contract fingerprint"
    );
    assert_sha256_reference(fingerprint, "sys5-i3-probe-carrier-contract-sha256-v1:");

    let owner = contract
        .owner_request_facts()
        .expect("an owner-request carrier must retain its owner-request-only facts");
    assert!(owner.request_template_present());
    assert_eq!(owner.request_template_slot_names(), ["Request"]);
    assert_eq!(owner.origin_locus_template(), "ParticipantA");
    assert_eq!(owner.target_owner_locus_template(), "WorldAuthority");
    assert!(!owner.origin_principal_ref().is_empty());
    assert_ne!(
        owner.origin_principal_ref(),
        "self",
        "the façade must expose an opaque principal reference, never the principal value"
    );
    assert!(!owner.requires_any_frontier());
    assert!(owner.frontier_requirement_names().is_empty());
    assert!(!owner.requires_receipt_consumption_state());
    assert!(!owner.designated_result_details_present());

    let observer_rendered = format!("{contract:?};{owner:?}");
    for forbidden in [ACTIVE_I2_SOURCE, "principal self", "MembershipAuth"] {
        assert!(
            !observer_rendered.contains(forbidden),
            "the expanded façade must remain reference-only and omit {forbidden:?}"
        );
    }
}

fn assert_sha256_reference(reference: &str, prefix: &str) {
    let digest = reference
        .strip_prefix(prefix)
        .expect("the retained contract fingerprint must use its domain-separated prefix");
    assert_eq!(digest.len(), 64);
    assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
}
