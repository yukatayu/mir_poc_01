use std::path::PathBuf;

use mir_semantics::{
    BatchRunSummary, BundleExecutionFailure, FixtureRuntimeRequirement,
};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct DetachedAggregateArtifact {
    pub schema_version: &'static str,
    pub artifact_kind: &'static str,
    pub aggregate_context: AggregateContextArtifact,
    pub summary_core: AggregateCoreArtifact,
    pub detached_noncore: AggregateNoncoreArtifact,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct AggregateContextArtifact {
    pub directory_path: String,
    pub aggregate_scope: &'static str,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct AggregateCoreArtifact {
    pub total_bundles: usize,
    pub runtime_bundles: usize,
    pub static_only_bundles: usize,
    pub passed: usize,
    pub failed: usize,
    pub bundle_failure_kind_counts_scope: &'static str,
    pub bundle_failure_kind_counts: Vec<BundleFailureKindCountArtifact>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct HostPlanCoverageFailureRef {
    pub fixture_id: String,
    pub fixture_path: String,
    pub runtime_requirement: &'static str,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct BundleFailureKindCountArtifact {
    pub failure_kind: &'static str,
    pub count: usize,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct AggregateNoncoreArtifact {
    pub discovery_failures: usize,
    pub bundle_failures: usize,
    pub host_plan_coverage_failures: Vec<HostPlanCoverageFailureRef>,
}

pub fn build_detached_aggregate_artifact(
    fixture_directory: PathBuf,
    summary: &BatchRunSummary,
) -> DetachedAggregateArtifact {
    DetachedAggregateArtifact {
        schema_version: "draft-current-l2-detached-aggregate-v0",
        artifact_kind: "current-l2-batch-aggregate-detached-sketch",
        aggregate_context: AggregateContextArtifact {
            directory_path: fixture_directory.display().to_string(),
            aggregate_scope: "directory-all",
        },
        summary_core: AggregateCoreArtifact {
            total_bundles: summary.total_bundles,
            runtime_bundles: summary.runtime_bundles,
            static_only_bundles: summary.static_only_bundles,
            passed: summary.passed,
            failed: summary.failed,
            bundle_failure_kind_counts_scope: "migrated-kinds-only",
            bundle_failure_kind_counts: bundle_failure_kind_counts(summary),
        },
        detached_noncore: AggregateNoncoreArtifact {
            discovery_failures: summary.discovery_failures.len(),
            bundle_failures: summary.bundle_failures.len(),
            host_plan_coverage_failures: summary
                .host_plan_coverage_failures
                .iter()
                .map(host_plan_coverage_failure_ref)
                .collect(),
        },
    }
}

fn runtime_requirement_name(value: FixtureRuntimeRequirement) -> &'static str {
    match value {
        FixtureRuntimeRequirement::StaticOnly => "static-only",
        FixtureRuntimeRequirement::RuntimeWithHostPlan => "runtime-with-host-plan",
    }
}

fn host_plan_coverage_failure_ref(
    value: &BundleExecutionFailure,
) -> HostPlanCoverageFailureRef {
    HostPlanCoverageFailureRef {
        fixture_id: value.fixture_id.clone(),
        fixture_path: value.fixture_path.display().to_string(),
        runtime_requirement: runtime_requirement_name(value.runtime_requirement),
    }
}

fn bundle_failure_kind_counts(
    summary: &BatchRunSummary,
) -> Vec<BundleFailureKindCountArtifact> {
    let mut rows = Vec::new();
    if !summary.host_plan_coverage_failures.is_empty() {
        rows.push(BundleFailureKindCountArtifact {
            failure_kind: "host-plan-coverage-failure",
            count: summary.host_plan_coverage_failures.len(),
        });
    }
    rows
}
