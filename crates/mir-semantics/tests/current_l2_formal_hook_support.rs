use std::path::PathBuf;

use mir_semantics::{load_bundle_from_fixture_path, load_fixture_from_path, run_bundle, static_gate_detailed};

#[path = "../examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;
use current_l2_formal_hook_support::{
    build_formal_hook_from_detached_bundle_artifact, build_formal_hook_from_static_gate_artifact,
    ToolNeutralFormalEvidenceRef,
};
use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn formal_hook_support_emits_static_cluster_subject_and_row_refs() {
    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);

    let formal_hook = build_formal_hook_from_static_gate_artifact(&artifact).unwrap();

    assert_eq!(formal_hook.subject_kind, "fixture_static_cluster");
    assert_eq!(formal_hook.subject_ref, "e5_underdeclared_lineage");
    assert_eq!(formal_hook.contract_rows.len(), 2);
    assert_eq!(
        formal_hook.contract_rows[0].obligation_kind,
        "canonical_normalization_law"
    );
    assert_eq!(
        formal_hook.contract_rows[0].evidence_refs,
        vec![
            ToolNeutralFormalEvidenceRef {
                ref_kind: "fixture".to_string(),
                ref_id: "e5_underdeclared_lineage".to_string(),
            },
            ToolNeutralFormalEvidenceRef {
                ref_kind: "static_gate_artifact".to_string(),
                ref_id: "e5_underdeclared_lineage".to_string(),
            },
        ]
    );
    assert_eq!(formal_hook.contract_rows[1].obligation_kind, "no_re_promotion");
    assert_eq!(
        formal_hook.contract_rows[1].evidence_refs,
        vec![ToolNeutralFormalEvidenceRef {
            ref_kind: "fixture".to_string(),
            ref_id: "e5_underdeclared_lineage".to_string(),
        }]
    );
}

#[test]
fn formal_hook_support_rejects_static_artifact_with_wrong_schema_or_kind() {
    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);

    let mut wrong_schema = build_detached_static_gate_artifact(path.clone(), &fixture, &gate);
    wrong_schema.schema_version = "draft-wrong".to_string();
    let schema_error = build_formal_hook_from_static_gate_artifact(&wrong_schema).unwrap_err();
    assert!(
        schema_error.contains("schema_version"),
        "unexpected error: {schema_error}"
    );

    let mut wrong_kind = build_detached_static_gate_artifact(path, &fixture, &gate);
    wrong_kind.artifact_kind = "wrong-kind".to_string();
    let kind_error = build_formal_hook_from_static_gate_artifact(&wrong_kind).unwrap_err();
    assert!(
        kind_error.contains("artifact_kind"),
        "unexpected error: {kind_error}"
    );
}

#[test]
fn formal_hook_support_emits_runtime_try_cut_subject_and_row_refs() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);

    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap();

    assert_eq!(formal_hook.subject_kind, "runtime_try_cut_cluster");
    assert_eq!(formal_hook.subject_ref, "e2_try_fallback");
    assert_eq!(formal_hook.contract_rows.len(), 1);
    assert_eq!(
        formal_hook.contract_rows[0].obligation_kind,
        "rollback_cut_non_interference"
    );
    assert_eq!(
        formal_hook.contract_rows[0].evidence_refs,
        vec![
            ToolNeutralFormalEvidenceRef {
                ref_kind: "fixture".to_string(),
                ref_id: "e2_try_fallback".to_string(),
            },
            ToolNeutralFormalEvidenceRef {
                ref_kind: "runtime_cluster".to_string(),
                ref_id: "e2_try_fallback".to_string(),
            },
        ]
    );
}

#[test]
fn formal_hook_support_rejects_runtime_artifact_outside_try_cut_cluster() {
    let bundle =
        load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);

    let error =
        build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap_err();

    assert!(
        error.contains("runtime_try_cut_cluster"),
        "unexpected error: {error}"
    );
}

#[test]
fn formal_hook_support_rejects_runtime_artifact_with_wrong_schema_or_kind() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();

    let mut wrong_schema = build_detached_bundle_artifact(&bundle, &report);
    wrong_schema.schema_version = "draft-wrong".to_string();
    let schema_error = build_formal_hook_from_detached_bundle_artifact(&wrong_schema).unwrap_err();
    assert!(
        schema_error.contains("schema_version"),
        "unexpected error: {schema_error}"
    );

    let mut wrong_kind = build_detached_bundle_artifact(&bundle, &report);
    wrong_kind.artifact_kind = "wrong-kind".to_string();
    let kind_error = build_formal_hook_from_detached_bundle_artifact(&wrong_kind).unwrap_err();
    assert!(
        kind_error.contains("artifact_kind"),
        "unexpected error: {kind_error}"
    );
}
