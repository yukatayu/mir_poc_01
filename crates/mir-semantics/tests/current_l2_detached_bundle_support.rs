use std::path::PathBuf;

use mir_semantics::{load_bundle_from_fixture_path, run_bundle};

#[path = "../examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;

use current_l2_detached_bundle_support::{
    build_detached_bundle_artifact, NonAdmissibleMetadataArtifact,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn detached_bundle_support_preserves_runtime_fixture_core_and_context() {
    let path = fixture_path("e3-option-admit-chain.json");
    let bundle = load_bundle_from_fixture_path(path.clone()).unwrap();
    let report = run_bundle(&bundle).unwrap();

    let artifact = build_detached_bundle_artifact(&bundle, &report);

    assert_eq!(artifact.bundle_context.fixture_id, "e3_option_admit_chain");
    assert_eq!(artifact.bundle_context.fixture_path, path.display().to_string());
    assert_eq!(
        artifact.bundle_context.host_plan_path,
        Some(
            fixture_path("e3-option-admit-chain.host-plan.json")
                .display()
                .to_string()
        )
    );
    assert_eq!(
        artifact.bundle_context.runtime_requirement,
        "runtime-with-host-plan"
    );
    assert_eq!(artifact.payload_core.static_verdict, "valid");
    assert!(artifact.payload_core.entered_evaluation);
    assert_eq!(
        artifact.payload_core.terminal_outcome,
        Some("success".to_string())
    );
    assert_eq!(artifact.payload_core.event_kinds, vec!["perform-success"]);
    assert_eq!(
        artifact.payload_core.non_admissible_metadata,
        vec![NonAdmissibleMetadataArtifact {
            option_ref: "owner_writer".to_string(),
            subreason: "admit-miss".to_string(),
        }]
    );
    assert_eq!(artifact.payload_core.narrative_explanations, Vec::<String>::new());
    assert_eq!(
        artifact.detached_noncore.steps_executed,
        report.report.steps_executed
    );
}

#[test]
fn detached_bundle_support_preserves_static_only_fixture_split() {
    let path = fixture_path("e4-malformed-lineage.json");
    let bundle = load_bundle_from_fixture_path(path.clone()).unwrap();
    let report = run_bundle(&bundle).unwrap();

    let artifact = build_detached_bundle_artifact(&bundle, &report);

    assert_eq!(artifact.bundle_context.fixture_id, "e4_malformed_lineage");
    assert_eq!(artifact.bundle_context.fixture_path, path.display().to_string());
    assert_eq!(artifact.bundle_context.host_plan_path, None);
    assert_eq!(artifact.bundle_context.runtime_requirement, "static-only");
    assert_eq!(artifact.payload_core.static_verdict, "malformed");
    assert!(!artifact.payload_core.entered_evaluation);
    assert_eq!(artifact.payload_core.terminal_outcome, None);
    assert!(artifact.payload_core.event_kinds.is_empty());
    assert_eq!(
        artifact.detached_noncore.steps_executed,
        report.report.steps_executed
    );
}
