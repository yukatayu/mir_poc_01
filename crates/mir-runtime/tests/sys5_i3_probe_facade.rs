use std::collections::{BTreeMap, BTreeSet};

use mir_runtime::sys5_local_slice::{
    Sys5I3AdapterAuthorityRequirementRow, Sys5I3AdapterAuthorityRequirements,
    Sys5I3AdapterCarrierContract, Sys5I3AdapterCarrierVariantFacts,
    Sys5I3AdapterDesignatedInputFacts, Sys5I3ProbeFacadeErrorKind, Sys5I3ProbeRedaction,
    Sys5SourceInput, build_project,
};

const ACTIVE_I2_LOGICAL_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const ACTIVE_I2_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const OWNER_FAILURES: &[&str] = &[
    "StaleMembership",
    "MissingCapability",
    "MissingWitness",
    "VisibilityDenied",
    "RouteUnavailable",
];
const OWNER_EFFECTS_MINIMUM: &[&str] = &["OwnerRequest", "OwnerLocalRead", "OwnerWrite"];
const ATTACK_EFFECTS: &[&str] = &[
    "OwnerRequest",
    "OwnerLocalRead",
    "OwnerWrite",
    "ObserverPublish",
];
const DESIGNATED_INPUT_EFFECTS: &[&str] = &[
    "DesignatedRemoteRequest",
    "DesignatedReceiptUse",
    "DesignatedValuePublish",
];

struct FamilyExpectation {
    kind: &'static str,
    count: usize,
    failures: &'static [&'static str],
    effects: Option<&'static [&'static str]>,
    slots: &'static [&'static str],
    linked_request: bool,
    typed_outcome: bool,
    receipt_consumption: bool,
    authority: &'static [&'static str],
    authority_rows: &'static [AuthorityRowExpectation],
    fingerprint_variant_fields: &'static [&'static str],
}

struct AuthorityRowExpectation {
    requirement_kind: &'static str,
    generated_obligation_kind: Option<&'static str>,
    generated_obligation_detail: Option<&'static str>,
    provenance: &'static str,
    authority_category: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DesignatedInputDependencyObservation {
    dependency_ordinal: usize,
    typed_state_read_ref: String,
    designated_evaluator_locus: String,
    source_owner_locus: String,
    requester_site_ref: String,
    authority_origin_ref: String,
    request_ref: String,
    receipt_use_ref: String,
}

const SEALED_RUNTIME_SEAM_PROVENANCE: &str = "RequiredFromSealedRuntimeSeam";

const fn authority_row(
    requirement_kind: &'static str,
    generated_obligation_kind: Option<&'static str>,
    generated_obligation_detail: Option<&'static str>,
    authority_category: Option<&'static str>,
) -> AuthorityRowExpectation {
    AuthorityRowExpectation {
        requirement_kind,
        generated_obligation_kind,
        generated_obligation_detail,
        provenance: SEALED_RUNTIME_SEAM_PROVENANCE,
        authority_category,
    }
}

const OWNER_AUTHORITY_ROWS: &[AuthorityRowExpectation] = &[
    authority_row(
        "MembershipEpochIncarnation",
        None,
        None,
        Some("MembershipEpochIncarnation"),
    ),
    authority_row(
        "LiveCapabilityRef",
        Some("Capability"),
        None,
        Some("OwnerCapabilityRef"),
    ),
    authority_row(
        "LiveWitnessRef",
        Some("Witness"),
        None,
        Some("OwnerWitnessRef"),
    ),
];

const DESIGNATED_INPUT_AUTHORITY_ROWS: &[AuthorityRowExpectation] = &[
    authority_row(
        "MembershipEpochIncarnation",
        None,
        None,
        Some("MembershipEpochIncarnation"),
    ),
    authority_row(
        "ProducerReleaseCapabilitySlot",
        None,
        None,
        Some("ProducerReleaseCapability"),
    ),
    authority_row(
        "ProducerReleaseWitnessSlot",
        None,
        None,
        Some("ProducerReleaseWitness"),
    ),
    authority_row(
        "EvaluatorDecisionAuthoritySlot",
        Some("AdmittedEvaluatorAuthority"),
        None,
        Some("EvaluatorDecisionAuthority"),
    ),
];

const DESIGNATED_RESULT_AUTHORITY_ROWS: &[AuthorityRowExpectation] = &[
    authority_row(
        "ConsumerMembershipEpochIncarnation",
        Some("DesignatedResultConsumerAuthority"),
        None,
        Some("DesignatedResultConsumerMembership"),
    ),
    authority_row(
        "ConsumerCapabilityRef",
        Some("DesignatedResultConsumerAuthority"),
        None,
        Some("DesignatedResultConsumerCapability"),
    ),
    authority_row(
        "ConsumerWitnessRef",
        Some("DesignatedResultConsumerAuthority"),
        None,
        Some("DesignatedResultConsumerWitness"),
    ),
];

const FAMILY_EXPECTATIONS: &[FamilyExpectation] = &[
    FamilyExpectation {
        kind: "owner-request",
        count: 4,
        failures: OWNER_FAILURES,
        effects: None,
        slots: &["Request"],
        linked_request: false,
        typed_outcome: false,
        receipt_consumption: false,
        authority: &[
            "MembershipEpochIncarnation",
            "OwnerCapabilityRef",
            "OwnerWitnessRef",
        ],
        authority_rows: OWNER_AUTHORITY_ROWS,
        fingerprint_variant_fields: &[
            "origin-principal-ref",
            "origin-locus-template",
            "target-owner-locus-template",
        ],
    },
    FamilyExpectation {
        kind: "owner-reply-receipt",
        count: 4,
        failures: OWNER_FAILURES,
        effects: None,
        slots: &["Request", "Serve", "Reply", "Receive"],
        linked_request: true,
        typed_outcome: true,
        receipt_consumption: false,
        authority: &[
            "MembershipEpochIncarnation",
            "OwnerCapabilityRef",
            "OwnerWitnessRef",
        ],
        authority_rows: OWNER_AUTHORITY_ROWS,
        fingerprint_variant_fields: &[
            "origin-principal-ref",
            "origin-locus-template",
            "target-owner-locus-template",
        ],
    },
    FamilyExpectation {
        kind: "designated-input-request",
        count: 1,
        failures: &[],
        effects: Some(DESIGNATED_INPUT_EFFECTS),
        slots: &["Request"],
        linked_request: false,
        typed_outcome: false,
        receipt_consumption: false,
        authority: &[
            "MembershipEpochIncarnation",
            "ProducerReleaseCapability",
            "ProducerReleaseWitness",
            "EvaluatorDecisionAuthority",
        ],
        authority_rows: DESIGNATED_INPUT_AUTHORITY_ROWS,
        fingerprint_variant_fields: &[
            "dependency-ordinal",
            "typed-state-read-ref",
            "requester-site-ref",
            "authority-origin-ref",
            "request-ref",
            "receipt-use-ref",
            "designated-evaluator-locus",
            "source-owner-locus",
            "frontier-requirement-names",
        ],
    },
    FamilyExpectation {
        kind: "designated-input-receipt",
        count: 1,
        failures: &[],
        effects: Some(DESIGNATED_INPUT_EFFECTS),
        slots: &["Request", "Serve", "Reply", "Receive"],
        linked_request: true,
        typed_outcome: true,
        receipt_consumption: true,
        authority: &[
            "MembershipEpochIncarnation",
            "ProducerReleaseCapability",
            "ProducerReleaseWitness",
            "EvaluatorDecisionAuthority",
        ],
        authority_rows: DESIGNATED_INPUT_AUTHORITY_ROWS,
        fingerprint_variant_fields: &[
            "dependency-ordinal",
            "typed-state-read-ref",
            "requester-site-ref",
            "authority-origin-ref",
            "request-ref",
            "receipt-use-ref",
            "designated-evaluator-locus",
            "source-owner-locus",
            "frontier-requirement-names",
        ],
    },
    FamilyExpectation {
        kind: "relation-projection-publication",
        count: 1,
        failures: &[],
        effects: Some(&["RelationPublish"]),
        slots: &["Publish", "Observe"],
        linked_request: false,
        typed_outcome: false,
        receipt_consumption: false,
        authority: &[],
        authority_rows: &[],
        fingerprint_variant_fields: &["relation-name", "publication-locus", "consumer-locus"],
    },
    FamilyExpectation {
        kind: "designated-result-delivery",
        count: 1,
        failures: &[],
        effects: Some(&["DesignatedResultDelivery", "DesignatedResultConsume"]),
        slots: &["Publish", "Receive", "Consume"],
        linked_request: true,
        typed_outcome: true,
        receipt_consumption: true,
        authority: &[
            "DesignatedResultConsumerMembership",
            "DesignatedResultConsumerCapability",
            "DesignatedResultConsumerWitness",
        ],
        authority_rows: DESIGNATED_RESULT_AUTHORITY_ROWS,
        fingerprint_variant_fields: &[
            "evaluator-locus",
            "consumer-locus",
            "result-version-ref",
            "input-frontier-ref",
            "result-frontier-ref",
            "observation-policy-ref",
            "policy-stamp-ref",
            "static-retry-contract",
        ],
    },
];

const COMMON_FINGERPRINT_FIELDS: &[&str] = &[
    "variant-discriminant",
    "checked-program-ref",
    "operation-id",
    "edge-kind",
    "lifecycle-kind",
    "source-locus",
    "target-locus",
    "logical-source-path",
    "source-span",
    "source-ref",
    "core-ref",
    "source-artifact-ref",
    "target-artifact-ref",
    "edge-ref",
    "declared-failure-names",
    "effect-kind-names",
    "required-occurrence-slot-names",
    "requires-linked-request-identity",
    "requires-typed-outcome",
    "requires-receipt-consumption-state",
    "authority-category-names",
    "authority-requirement-rows",
    "requires-membership-epoch-and-incarnation",
    "requires-capability-and-witness-refs",
    "redaction",
    "checked-core-bound",
    "transfers-authority",
    "mints-authority-without-source",
    "public-api-or-wire-contract",
];

fn active_project() -> mir_runtime::sys5_local_slice::Sys5LocalProject {
    build_project(Sys5SourceInput::inline(
        ACTIVE_I2_LOGICAL_PATH,
        ACTIVE_I2_SOURCE,
    ))
    .expect("the accepted I2 source must build before a retained-projection lookup")
}

fn assert_sha256_reference(reference: &str, prefix: &str) {
    let digest = reference
        .strip_prefix(prefix)
        .expect("a reference must use its domain-separated prefix");
    assert_eq!(digest.len(), 64);
    assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn assert_adapter_contract_is_observer_safe(contract: &Sys5I3AdapterCarrierContract) {
    let observer_rendered = format!("{contract:?};{:?}", contract.variant_facts());
    for forbidden in [
        ACTIVE_I2_SOURCE,
        env!("CARGO_MANIFEST_DIR"),
        "avatar[target].hp",
        "participant_input[self].focus + 1",
        "MembershipAuth",
        "cap:",
        "witness:",
        "principal self",
        "private_payload",
    ] {
        assert!(
            !observer_rendered.contains(forbidden),
            "the adapter contract must expose observer-safe references only, not {forbidden:?}"
        );
    }
}

fn assert_exact_authority_requirement_rows(
    authority: &Sys5I3AdapterAuthorityRequirements,
    expected: &[AuthorityRowExpectation],
) {
    let rows: &[Sys5I3AdapterAuthorityRequirementRow] = authority.rows();
    assert_eq!(
        rows.len(),
        expected.len(),
        "the retained authority row count must preserve multiplicity"
    );
    for (actual, expected) in rows.iter().zip(expected) {
        assert_eq!(actual.requirement_kind_name(), expected.requirement_kind);
        assert_eq!(
            actual.generated_obligation_present(),
            expected.generated_obligation_kind.is_some()
        );
        assert_eq!(
            actual.generated_obligation_kind_name(),
            expected.generated_obligation_kind
        );
        assert_eq!(
            actual.generated_obligation_detail_name(),
            expected.generated_obligation_detail
        );
        assert_eq!(actual.provenance_name(), expected.provenance);
        assert_eq!(
            actual.authority_category_name(),
            expected.authority_category
        );
    }
}

fn designated_input_dependency_observation(
    facts: &Sys5I3AdapterDesignatedInputFacts,
) -> DesignatedInputDependencyObservation {
    DesignatedInputDependencyObservation {
        dependency_ordinal: facts.dependency_ordinal(),
        typed_state_read_ref: facts.typed_state_read_ref().to_string(),
        designated_evaluator_locus: facts.designated_evaluator_locus().to_string(),
        source_owner_locus: facts.source_owner_locus().to_string(),
        requester_site_ref: facts.requester_site_ref().to_string(),
        authority_origin_ref: facts.authority_origin_ref().to_string(),
        request_ref: facts.request_ref().to_string(),
        receipt_use_ref: facts.receipt_use_ref().to_string(),
    }
}

fn assert_designated_input_dependency_refs_are_observer_safe(
    facts: &Sys5I3AdapterDesignatedInputFacts,
) {
    let references = [
        (
            facts.requester_site_ref(),
            "sys5-i3-adapter-designated-requester-site-sha256-v1:",
        ),
        (
            facts.authority_origin_ref(),
            "sys5-i3-adapter-designated-authority-origin-sha256-v1:",
        ),
        (
            facts.request_ref(),
            "sys5-i3-adapter-designated-request-sha256-v1:",
        ),
        (
            facts.receipt_use_ref(),
            "sys5-i3-adapter-designated-receipt-use-sha256-v1:",
        ),
    ];
    for (reference, prefix) in references {
        assert_sha256_reference(reference, prefix);
    }
    assert_eq!(
        references
            .iter()
            .map(|(_, prefix)| *prefix)
            .collect::<BTreeSet<_>>()
            .len(),
        references.len(),
        "each designated dependency reference must use its own domain"
    );
}

fn assert_active_owner_effects(operation: &str, effects: &[String]) {
    let expected = match operation {
        "attack" => ATTACK_EFFECTS,
        "init_avatar_atk" => OWNER_EFFECTS_MINIMUM,
        "init_avatar_hp" => ATTACK_EFFECTS,
        "init_focus" => ATTACK_EFFECTS,
        unexpected => panic!("unexpected active owner carrier operation: {unexpected}"),
    };
    assert_eq!(
        effects, expected,
        "{operation} retains its exact effect row"
    );
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

#[test]
fn i3_adapter_facade_retains_exact_common_contracts_and_closed_variant_facts() {
    let project = active_project();
    let accepted_kinds = project.i3_adapter_accepted_family_kind_names();
    assert_eq!(
        accepted_kinds,
        [
            "owner-request",
            "owner-reply-receipt",
            "designated-input-request",
            "designated-input-receipt",
            "relation-projection-publication",
            "designated-result-delivery",
        ]
    );
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
            .all(|edge| accepted_kinds.contains(&edge.kind.as_str())),
        "a generated seventh family must not silently enter the closed adapter inventory"
    );
    let expected_edges = generated_edges
        .iter()
        .filter(|edge| {
            edge.derived_from_checked_core && accepted_kinds.contains(&edge.kind.as_str())
        })
        .collect::<Vec<_>>();
    let counts = expected_edges
        .iter()
        .fold(BTreeMap::new(), |mut counts, edge| {
            *counts.entry(edge.kind.as_str()).or_insert(0_usize) += 1;
            counts
        });
    assert_eq!(expected_edges.len(), 12);
    assert_eq!(
        counts,
        FAMILY_EXPECTATIONS
            .iter()
            .map(|expected| (expected.kind, expected.count))
            .collect()
    );

    for edge in expected_edges
        .iter()
        .filter(|edge| matches!(edge.kind.as_str(), "owner-request" | "owner-reply-receipt"))
    {
        let contract = project
            .i3_adapter_carrier_contract(&edge.edge_ref)
            .expect("every active owner edge must retain its checked carrier effect row");
        assert_active_owner_effects(contract.operation_id(), contract.effect_kind_names());
    }

    let mut fingerprints = BTreeSet::new();
    let mut designated_input_pairs = BTreeMap::<
        String,
        (
            Option<DesignatedInputDependencyObservation>,
            Option<DesignatedInputDependencyObservation>,
        ),
    >::new();
    for edge in expected_edges {
        let expected = FAMILY_EXPECTATIONS
            .iter()
            .find(|expected| expected.kind == edge.kind)
            .expect("the closed adapter inventory must have every generated family");
        let contract: Sys5I3AdapterCarrierContract = project
            .i3_adapter_carrier_contract(&edge.edge_ref)
            .expect("every accepted checked edge must expose one retained adapter contract");
        let repeated = project
            .i3_adapter_carrier_contract(&edge.edge_ref)
            .expect("a repeated lookup must retain the same exact adapter contract");

        assert_eq!(
            contract.checked_program_ref(),
            edge.checked_program_identity
        );
        assert_eq!(contract.operation_id(), edge.operation_id);
        assert_eq!(contract.edge_kind(), edge.kind);
        assert_eq!(contract.lifecycle_kind(), edge.kind);
        assert_eq!(contract.source_locus(), edge.from_locus);
        assert_eq!(contract.target_locus(), edge.to_locus);
        assert_eq!(contract.logical_source_path(), edge.source_path);
        assert_eq!(contract.source_span(), edge.source_span);
        let expected_source_ref = format!(
            "{}:{}:{}-{}:{}",
            edge.source_path,
            edge.source_span.start_line,
            edge.source_span.start_column,
            edge.source_span.end_line,
            edge.source_span.end_column,
        );
        assert_eq!(contract.source_ref(), expected_source_ref);
        assert!(!contract.source_ref().contains(env!("CARGO_MANIFEST_DIR")));
        assert!(!contract.source_ref().contains(ACTIVE_I2_SOURCE));
        assert_eq!(contract.core_ref(), edge.core_ref.as_deref().unwrap());
        assert_eq!(contract.source_artifact_ref(), edge.source_fragment_ref);
        assert_eq!(contract.target_artifact_ref(), edge.target_fragment_ref);
        assert_eq!(contract.edge_ref(), edge.edge_ref);
        assert_eq!(contract.declared_failure_names(), expected.failures);
        if let Some(effects) = expected.effects {
            assert_eq!(contract.effect_kind_names(), effects);
        }
        assert_eq!(contract.required_occurrence_slot_names(), expected.slots);
        assert_eq!(
            contract.requires_linked_request_identity(),
            expected.linked_request
        );
        assert_eq!(contract.requires_typed_outcome(), expected.typed_outcome);
        assert_eq!(
            contract.requires_receipt_consumption_state(),
            expected.receipt_consumption
        );
        let authority = contract.authority_requirements();
        assert_exact_authority_requirement_rows(authority, expected.authority_rows);
        assert_eq!(authority.category_names(), expected.authority);
        assert_eq!(
            authority.requires_membership_epoch_and_incarnation(),
            !expected.authority.is_empty()
        );
        assert_eq!(
            authority.requires_capability_and_witness_refs(),
            !expected.authority.is_empty()
        );
        assert_eq!(contract.redaction(), Sys5I3ProbeRedaction::ReferenceOnly);
        assert!(contract.checked_core_bound());
        assert!(!contract.transfers_authority());
        assert!(!contract.mints_authority_without_source());
        assert!(!contract.public_api_or_wire_contract());

        match (edge.kind.as_str(), contract.variant_facts()) {
            ("owner-request", Sys5I3AdapterCarrierVariantFacts::OwnerRequest(facts)) => {
                assert_sha256_reference(
                    facts.origin_principal_ref(),
                    "sys5-i3-adapter-owner-principal-sha256-v1:",
                );
                assert_eq!(facts.origin_locus_template(), edge.from_locus);
                assert_eq!(facts.target_owner_locus_template(), edge.to_locus);
            }
            ("owner-reply-receipt", Sys5I3AdapterCarrierVariantFacts::OwnerReplyReceipt(facts)) => {
                assert_sha256_reference(
                    facts.origin_principal_ref(),
                    "sys5-i3-adapter-owner-principal-sha256-v1:",
                );
                assert_eq!(facts.origin_locus_template(), edge.to_locus);
                assert_eq!(facts.target_owner_locus_template(), edge.from_locus);
                let request_edge = generated_edges
                    .iter()
                    .find(|candidate| {
                        candidate.kind == "owner-request"
                            && candidate.operation_id == edge.operation_id
                    })
                    .expect("every owner reply receipt must pair with its operation's request");
                let request_contract = project
                    .i3_adapter_carrier_contract(&request_edge.edge_ref)
                    .expect("the paired owner request must retain its adapter contract");
                let request_facts = match request_contract.variant_facts() {
                    Sys5I3AdapterCarrierVariantFacts::OwnerRequest(request_facts) => request_facts,
                    unexpected => panic!(
                        "the paired owner request must retain OwnerRequest facts, got {unexpected:?}"
                    ),
                };
                assert_eq!(
                    facts.origin_principal_ref(),
                    request_facts.origin_principal_ref()
                );
                assert_eq!(
                    facts.origin_locus_template(),
                    request_facts.origin_locus_template()
                );
                assert_eq!(
                    facts.target_owner_locus_template(),
                    request_facts.target_owner_locus_template()
                );
            }
            (
                "designated-input-request",
                Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(facts),
            )
            | (
                "designated-input-receipt",
                Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(facts),
            ) => {
                assert_eq!(facts.dependency_ordinal(), 0);
                assert_sha256_reference(
                    facts.typed_state_read_ref(),
                    "sys5-i3-adapter-designated-read-sha256-v1:",
                );
                assert_eq!(facts.designated_evaluator_locus(), "WorldAuthority");
                assert_eq!(facts.source_owner_locus(), "ParticipantA");
                assert_designated_input_dependency_refs_are_observer_safe(facts);
                let pair = designated_input_pairs
                    .entry(contract.operation_id().to_string())
                    .or_insert((None, None));
                let observed = designated_input_dependency_observation(facts);
                let slot = if edge.kind == "designated-input-request" {
                    &mut pair.0
                } else {
                    &mut pair.1
                };
                assert!(
                    slot.replace(observed).is_none(),
                    "each operation retains exactly one designated {kind} dependency snapshot",
                    kind = edge.kind
                );
                assert_eq!(
                    facts.frontier_requirement_names(),
                    if edge.kind == "designated-input-request" {
                        &["Input"]
                    } else {
                        &["Result"]
                    }
                );
            }
            (
                "relation-projection-publication",
                Sys5I3AdapterCarrierVariantFacts::RelationProjectionPublication(facts),
            ) => {
                assert_eq!(facts.relation_name(), "bird_follow");
                assert_eq!(facts.publication_locus(), edge.from_locus);
                assert_eq!(facts.consumer_locus(), edge.to_locus);
            }
            (
                "designated-result-delivery",
                Sys5I3AdapterCarrierVariantFacts::DesignatedResultDelivery(facts),
            ) => {
                assert_eq!(facts.evaluator_locus(), edge.from_locus);
                assert_eq!(facts.consumer_locus(), edge.to_locus);
                for (reference, prefix) in [
                    (
                        facts.result_version_ref(),
                        "sys5-i3-adapter-result-version-sha256-v1:",
                    ),
                    (
                        facts.input_frontier_ref(),
                        "sys5-i3-adapter-input-frontier-sha256-v1:",
                    ),
                    (
                        facts.result_frontier_ref(),
                        "sys5-i3-adapter-result-frontier-sha256-v1:",
                    ),
                    (
                        facts.observation_policy_ref(),
                        "sys5-i3-adapter-observation-policy-sha256-v1:",
                    ),
                    (
                        facts.policy_stamp_ref(),
                        "sys5-i3-adapter-policy-stamp-sha256-v1:",
                    ),
                ] {
                    assert_sha256_reference(reference, prefix);
                }
                assert_eq!(
                    facts.static_retry_contract_name(),
                    "ReturnExistingNoNewConsumption"
                );
            }
            (kind, facts) => panic!("the closed {kind:?} family received {facts:?}"),
        }
        assert_adapter_contract_is_observer_safe(&contract);

        let expected_fingerprint_fields = COMMON_FINGERPRINT_FIELDS
            .iter()
            .chain(expected.fingerprint_variant_fields)
            .map(|field| (*field).to_string())
            .collect::<Vec<_>>();
        let actual_fingerprint_fields = contract
            .full_retained_contract_fingerprint_field_names()
            .to_vec();
        assert_eq!(
            actual_fingerprint_fields.len(),
            expected_fingerprint_fields.len(),
            "the typed fingerprint visitor must not omit or add a semantic field"
        );
        assert_eq!(
            actual_fingerprint_fields, expected_fingerprint_fields,
            "the fingerprint field inventory must preserve its canonical exhaustive order"
        );
        assert_eq!(
            contract
                .full_retained_contract_fingerprint_field_names()
                .iter()
                .collect::<BTreeSet<_>>()
                .len(),
            contract
                .full_retained_contract_fingerprint_field_names()
                .len(),
            "the typed fingerprint visitor must not visit a semantic field twice"
        );
        let fingerprint = contract.full_retained_contract_fingerprint();
        assert_eq!(fingerprint, repeated.full_retained_contract_fingerprint());
        assert_sha256_reference(fingerprint, "sys5-i3-adapter-carrier-contract-sha256-v1:");
        assert!(fingerprints.insert(fingerprint.to_string()));
    }
    assert_eq!(
        designated_input_pairs.len(),
        1,
        "the active source retains one designated dependency pair"
    );
    for (operation, (request, receipt)) in designated_input_pairs {
        assert_eq!(
            request.expect("a designated request must retain dependency facts"),
            receipt.expect("a designated receipt must retain dependency facts"),
            "{operation} request and receipt must retain one identical designated dependency"
        );
    }
    assert_eq!(fingerprints.len(), 12);
}

#[test]
fn i3_adapter_facade_rejects_an_unknown_edge_ref() {
    let error = active_project()
        .i3_adapter_carrier_contract("missing-generated-edge-ref")
        .expect_err("the generic adapter facade must not reconstruct an unknown edge");
    assert_eq!(error.kind(), Sys5I3ProbeFacadeErrorKind::UnknownEdgeRef);
}

#[test]
fn i3_adapter_facade_keeps_absolute_value_stream_outside_closed_six_family_inventory() {
    let project = active_project();
    let accepted_kinds = project.i3_adapter_accepted_family_kind_names();
    assert_eq!(accepted_kinds.len(), 6);
    assert!(!accepted_kinds.contains(&"absolute-value-stream"));
    assert!(
        project
            .semantic_summary()
            .generated_communication
            .iter()
            .all(|edge| accepted_kinds.contains(&edge.kind.as_str())),
        "any generated seventh family must be rejected rather than silently entering the closed adapter algebra"
    );
}
