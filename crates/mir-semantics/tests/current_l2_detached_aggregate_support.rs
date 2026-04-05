use std::path::PathBuf;

use mir_semantics::{
    BatchRunSummary, BundleExecutionFailure, FixtureRuntimeRequirement,
};

#[path = "../examples/support/current_l2_detached_aggregate_support.rs"]
mod current_l2_detached_aggregate_support;

use current_l2_detached_aggregate_support::{
    build_detached_aggregate_artifact, BundleFailureKindCountArtifact,
    HostPlanCoverageFailureRef,
};

fn coverage_failure(path: &str, fixture_id: &str) -> BundleExecutionFailure {
    BundleExecutionFailure {
        fixture_path: PathBuf::from(path),
        fixture_id: fixture_id.to_string(),
        runtime_requirement: FixtureRuntimeRequirement::RuntimeWithHostPlan,
        error: "host plan did not cover all oracle calls".to_string(),
    }
}

#[test]
fn detached_aggregate_support_keeps_anchor_refs_and_typed_count() {
    let summary = BatchRunSummary {
        total_bundles: 3,
        runtime_bundles: 2,
        static_only_bundles: 1,
        passed: 2,
        failed: 1,
        discovery_failures: Vec::new(),
        bundle_failures: vec![coverage_failure(
            "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json",
            "e3_option_admit_chain",
        )],
        host_plan_coverage_failures: vec![coverage_failure(
            "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json",
            "e3_option_admit_chain",
        )],
        bundle_reports: Vec::new(),
    };

    let artifact = build_detached_aggregate_artifact(
        PathBuf::from("crates/mir-ast/tests/fixtures/current-l2"),
        &summary,
    );

    assert_eq!(artifact.summary_core.total_bundles, 3);
    assert_eq!(
        artifact.summary_core.bundle_failure_kind_counts_scope,
        "migrated-kinds-only"
    );
    assert_eq!(
        artifact.detached_noncore.host_plan_coverage_failures.len(),
        1
    );
    assert_eq!(
        artifact.detached_noncore.host_plan_coverage_failures[0],
        HostPlanCoverageFailureRef {
            fixture_id: "e3_option_admit_chain".to_string(),
            fixture_path:
                "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json"
                    .to_string(),
            runtime_requirement: "runtime-with-host-plan",
        }
    );
    assert_eq!(
        artifact.summary_core.bundle_failure_kind_counts,
        vec![BundleFailureKindCountArtifact {
            failure_kind: "host-plan-coverage-failure",
            count: 1,
        }]
    );
}

#[test]
fn detached_aggregate_support_omits_typed_rows_when_no_coverage_failure_exists() {
    let summary = BatchRunSummary {
        total_bundles: 1,
        runtime_bundles: 1,
        static_only_bundles: 0,
        passed: 1,
        failed: 0,
        discovery_failures: Vec::new(),
        bundle_failures: Vec::new(),
        host_plan_coverage_failures: Vec::new(),
        bundle_reports: Vec::new(),
    };

    let artifact = build_detached_aggregate_artifact(
        PathBuf::from("crates/mir-ast/tests/fixtures/current-l2"),
        &summary,
    );

    assert_eq!(
        artifact.summary_core.bundle_failure_kind_counts_scope,
        "migrated-kinds-only"
    );
    assert!(artifact.detached_noncore.host_plan_coverage_failures.is_empty());
    assert!(artifact.summary_core.bundle_failure_kind_counts.is_empty());
}
