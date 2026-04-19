use std::path::PathBuf;

use mir_semantics::{
    load_bundle_from_fixture_path, load_fixture_from_path, run_bundle, static_gate_detailed,
};
use serde_json::{from_str, to_string};

#[path = "../examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../examples/support/current_l2_model_check_carrier_support.rs"]
mod current_l2_model_check_carrier_support;
#[path = "../examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;
use current_l2_formal_hook_support::{
    build_formal_hook_from_detached_bundle_artifact, build_formal_hook_from_static_gate_artifact,
};
use current_l2_model_check_carrier_support::build_model_check_concrete_carrier_artifacts;
use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn model_check_carrier_support_emits_runtime_row_local_case() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap();

    let carriers = build_model_check_concrete_carrier_artifacts(&formal_hook).unwrap();

    assert_eq!(carriers.len(), 1);
    let carrier = &carriers[0];
    assert_eq!(carrier.subject_kind, "runtime_try_cut_cluster");
    assert_eq!(carrier.subject_ref, "e2_try_fallback");
    assert_eq!(
        carrier.case.obligation_kind,
        "rollback_cut_non_interference"
    );
    assert_eq!(
        carrier.case.evidence_refs,
        vec![
            current_l2_model_check_carrier_support::ModelCheckConcreteCarrierEvidenceRef {
                ref_kind: "fixture".to_string(),
                ref_id: "e2_try_fallback".to_string(),
            },
            current_l2_model_check_carrier_support::ModelCheckConcreteCarrierEvidenceRef {
                ref_kind: "runtime_cluster".to_string(),
                ref_id: "e2_try_fallback".to_string(),
            },
        ]
    );
}

#[test]
fn model_check_carrier_support_emits_static_row_local_cases() {
    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&artifact).unwrap();

    let carriers = build_model_check_concrete_carrier_artifacts(&formal_hook).unwrap();

    assert_eq!(carriers.len(), 2);
    assert_eq!(carriers[0].subject_kind, "fixture_static_cluster");
    assert_eq!(carriers[0].subject_ref, "e5_underdeclared_lineage");
    assert_eq!(
        carriers[0].case.obligation_kind,
        "canonical_normalization_law"
    );
    assert_eq!(carriers[1].case.obligation_kind, "no_re_promotion");
}

#[test]
fn model_check_carrier_support_rejects_wrong_schema_or_artifact_kind() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_bundle(&bundle).unwrap();
    let artifact = build_detached_bundle_artifact(&bundle, &report);
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&artifact).unwrap();

    let mut wrong_schema: current_l2_formal_hook_support::ToolNeutralFormalHookArtifact =
        from_str(&to_string(&formal_hook).unwrap()).unwrap();
    wrong_schema.schema_version = "draft-wrong".to_string();
    let schema_error = build_model_check_concrete_carrier_artifacts(&wrong_schema).unwrap_err();
    assert!(
        schema_error.contains("schema_version"),
        "unexpected error: {schema_error}"
    );

    let mut wrong_kind = formal_hook;
    wrong_kind.artifact_kind = "wrong-kind".to_string();
    let kind_error = build_model_check_concrete_carrier_artifacts(&wrong_kind).unwrap_err();
    assert!(
        kind_error.contains("artifact_kind"),
        "unexpected error: {kind_error}"
    );
}

#[test]
fn model_check_carrier_support_rejects_unsupported_pairs_and_empty_evidence() {
    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&artifact).unwrap();

    let mut unsupported_pair: current_l2_formal_hook_support::ToolNeutralFormalHookArtifact =
        from_str(&to_string(&formal_hook).unwrap()).unwrap();
    unsupported_pair.contract_rows[0].obligation_kind = "rollback_cut_non_interference".to_string();
    let pair_error = build_model_check_concrete_carrier_artifacts(&unsupported_pair).unwrap_err();
    assert!(
        pair_error.contains("unsupported model-check carrier pair"),
        "unexpected error: {pair_error}"
    );

    let mut empty_evidence: current_l2_formal_hook_support::ToolNeutralFormalHookArtifact =
        from_str(&to_string(&formal_hook).unwrap()).unwrap();
    empty_evidence.contract_rows[0].evidence_refs.clear();
    let evidence_error = build_model_check_concrete_carrier_artifacts(&empty_evidence).unwrap_err();
    assert!(
        evidence_error.contains("must include at least one evidence ref"),
        "unexpected error: {evidence_error}"
    );
}
