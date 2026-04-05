use std::path::PathBuf;

use mir_semantics::{
    CurrentL2Fixture, StaticGateVerdict, load_fixture_from_path, static_gate_detailed,
};
use serde_json::json;

#[path = "../examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn load_fixture(name: &str) -> CurrentL2Fixture {
    load_fixture_from_path(fixture_path(name)).unwrap()
}

fn synthetic_multi_reason_fixture(chain_names: [&str; 2]) -> CurrentL2Fixture {
    serde_json::from_value(json!({
        "schema_version": "current-l2-ast-fixture-v0",
        "fixture_id": "synthetic_multi_reason",
        "source_example_id": "SYNTHETIC_MULTI_REASON",
        "program": {
            "kind": "Program",
            "body": [{
                "kind": "PlaceBlock",
                "place": "root",
                "body": [{
                    "kind": "OptionDecl",
                    "name": "primary",
                    "target": "profile_doc",
                    "capability": "read",
                    "lease": "live",
                    "admit": null
                }, {
                    "kind": "OptionDecl",
                    "name": "mirror",
                    "target": "profile_doc",
                    "capability": "read",
                    "lease": "live",
                    "admit": null
                }, {
                    "kind": "OptionDecl",
                    "name": "archive",
                    "target": "profile_doc",
                    "capability": "read",
                    "lease": "live",
                    "admit": null
                }, {
                    "kind": "ChainDecl",
                    "name": chain_names[0],
                    "head": "primary",
                    "edges": [{
                        "predecessor": "primary",
                        "successor": "mirror",
                        "lineage_assertion": null
                    }]
                }, {
                    "kind": "ChainDecl",
                    "name": chain_names[1],
                    "head": "primary",
                    "edges": [{
                        "predecessor": "primary",
                        "successor": "archive",
                        "lineage_assertion": {
                            "predecessor": "primary",
                            "successor": "mirror"
                        }
                    }]
                }]
            }]
        },
        "expected_static": {
            "verdict": "malformed",
            "reasons": [],
        },
        "expected_runtime": {
            "enters_evaluation": false,
            "final_outcome": "not_evaluated",
            "notes": []
        },
        "expected_trace_audit": {
            "event_kinds": [],
            "non_admissible_metadata": [],
            "narrative_explanations": [],
            "must_explain": []
        }
    }))
    .unwrap()
}

#[test]
fn static_gate_support_preserves_fixture_context_and_malformed_reasons() {
    let path = fixture_path("e4-malformed-lineage.json");
    let fixture = load_fixture("e4-malformed-lineage.json");
    let gate = static_gate_detailed(&fixture);

    let artifact = build_detached_static_gate_artifact(path.clone(), &fixture, &gate);

    assert_eq!(artifact.fixture_context.fixture_id, "e4_malformed_lineage");
    assert_eq!(artifact.fixture_context.fixture_path, path.display().to_string());
    assert_eq!(artifact.checker_core.static_verdict, "malformed");
    assert!(
        artifact
            .checker_core
            .reasons
            .iter()
            .any(|reason| reason.contains("lineage assertion does not describe"))
    );
}

#[test]
fn static_gate_support_keeps_valid_fixture_reason_list_empty() {
    let path = fixture_path("e3-option-admit-chain.json");
    let fixture = load_fixture("e3-option-admit-chain.json");
    let gate = static_gate_detailed(&fixture);

    assert_eq!(gate.verdict, StaticGateVerdict::Valid);

    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);

    assert_eq!(artifact.checker_core.static_verdict, "valid");
    assert!(artifact.checker_core.reasons.is_empty());
}

#[test]
fn static_gate_reasons_are_deterministic_for_multi_reason_fixture() {
    let left_fixture = synthetic_multi_reason_fixture(["profile_ref_b", "profile_ref_a"]);
    let right_fixture = synthetic_multi_reason_fixture(["profile_ref_a", "profile_ref_b"]);

    let left = static_gate_detailed(&left_fixture);
    let right = static_gate_detailed(&right_fixture);

    let expected = vec![
        "lineage assertion does not describe primary -> archive".to_string(),
        "missing lineage assertion for primary -> mirror".to_string(),
    ];

    assert_eq!(left.reasons, expected);
    assert_eq!(right.reasons, expected);
}

#[test]
fn static_gate_artifact_emits_reason_codes_for_stable_clusters() {
    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture("e5-underdeclared-lineage.json");
    let gate = static_gate_detailed(&fixture);

    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);

    let detached_noncore = artifact
        .detached_noncore
        .expect("stable clusters should emit detached_noncore");

    assert_eq!(
        detached_noncore.reason_codes_scope,
        "stable-clusters-only"
    );
    assert_eq!(
        detached_noncore.reason_codes,
        vec![current_l2_static_gate_support::StaticReasonCodeRow::MissingLineageAssertion {
            predecessor: "primary".to_string(),
            successor: "mirror".to_string(),
        }]
    );
}

#[test]
fn static_gate_artifact_omits_reason_codes_for_unclassified_reason_text() {
    let path = fixture_path("e3-option-admit-chain.json");
    let fixture = load_fixture("e3-option-admit-chain.json");
    let gate = mir_semantics::StaticGateResult {
        verdict: StaticGateVerdict::Valid,
        reasons: vec!["helper-local explanation that has no stable code cluster".to_string()],
    };

    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);

    assert!(artifact.detached_noncore.is_none());
}
