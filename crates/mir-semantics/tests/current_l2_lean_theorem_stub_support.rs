use std::path::PathBuf;

use mir_semantics::{
    load_bundle_from_fixture_path, load_fixture_from_path, run_bundle, static_gate_detailed,
};
use serde_json::{from_str, to_string};

#[path = "../examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../examples/support/current_l2_lean_theorem_stub_support.rs"]
mod current_l2_lean_theorem_stub_support;
#[path = "../examples/support/current_l2_proof_notebook_review_unit_support.rs"]
mod current_l2_proof_notebook_review_unit_support;
#[path = "../examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;
use current_l2_formal_hook_support::{
    build_formal_hook_from_detached_bundle_artifact, build_formal_hook_from_static_gate_artifact,
};
use current_l2_lean_theorem_stub_support::build_lean_theorem_stub_artifacts;
use current_l2_proof_notebook_review_unit_support::build_proof_notebook_review_unit_artifacts;
use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn lean_theorem_stub_support_emits_runtime_stub() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap();
    let review_units = build_proof_notebook_review_unit_artifacts(&formal_hook).unwrap();

    let stubs = build_lean_theorem_stub_artifacts(&review_units).unwrap();

    assert_eq!(stubs.len(), 1);
    let stub = &stubs[0];
    assert_eq!(stub.tool_family, "lean-first");
    assert_eq!(
        stub.theorem_name,
        "e2_try_fallback__rollback_cut_non_interference"
    );
    assert!(stub.source_text.contains("namespace CurrentL2"));
    assert!(
        stub.source_text
            .contains("theorem e2_try_fallback__rollback_cut_non_interference")
    );
    assert!(stub.source_text.contains("sorry"));
}

#[test]
fn lean_theorem_stub_support_emits_static_stubs() {
    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&artifact).unwrap();
    let review_units = build_proof_notebook_review_unit_artifacts(&formal_hook).unwrap();

    let stubs = build_lean_theorem_stub_artifacts(&review_units).unwrap();

    assert_eq!(stubs.len(), 2);
    assert_eq!(
        stubs[0].theorem_name,
        "e5_underdeclared_lineage__canonical_normalization_law"
    );
    assert_eq!(
        stubs[1].theorem_name,
        "e5_underdeclared_lineage__no_re_promotion"
    );
}

#[test]
fn lean_theorem_stub_support_rejects_wrong_schema_or_artifact_kind() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap();
    let review_units = build_proof_notebook_review_unit_artifacts(&formal_hook).unwrap();

    let mut wrong_schema: Vec<
        current_l2_proof_notebook_review_unit_support::ProofNotebookReviewUnitArtifact,
    > = from_str(&to_string(&review_units).unwrap()).unwrap();
    wrong_schema[0].schema_version = "draft-wrong".to_string();
    let schema_error = build_lean_theorem_stub_artifacts(&wrong_schema).unwrap_err();
    assert!(
        schema_error.contains("schema_version"),
        "unexpected error: {schema_error}"
    );

    let mut wrong_kind = review_units;
    wrong_kind[0].artifact_kind = "wrong-kind".to_string();
    let kind_error = build_lean_theorem_stub_artifacts(&wrong_kind).unwrap_err();
    assert!(
        kind_error.contains("artifact_kind"),
        "unexpected error: {kind_error}"
    );
}

#[test]
fn lean_theorem_stub_support_rejects_empty_goal_or_evidence() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap();
    let review_units = build_proof_notebook_review_unit_artifacts(&formal_hook).unwrap();

    let mut empty_goal: Vec<
        current_l2_proof_notebook_review_unit_support::ProofNotebookReviewUnitArtifact,
    > = from_str(&to_string(&review_units).unwrap()).unwrap();
    empty_goal[0].row.goal_text.clear();
    let goal_error = build_lean_theorem_stub_artifacts(&empty_goal).unwrap_err();
    assert!(
        goal_error.contains("goal_text"),
        "unexpected error: {goal_error}"
    );

    let mut empty_evidence: Vec<
        current_l2_proof_notebook_review_unit_support::ProofNotebookReviewUnitArtifact,
    > = from_str(&to_string(&review_units).unwrap()).unwrap();
    empty_evidence[0].row.evidence_refs.clear();
    let evidence_error = build_lean_theorem_stub_artifacts(&empty_evidence).unwrap_err();
    assert!(
        evidence_error.contains("must include at least one evidence ref"),
        "unexpected error: {evidence_error}"
    );
}
