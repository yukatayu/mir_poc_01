use std::{env, fs, path::PathBuf, process};

use mir_semantics::{
    BatchRunSummary, BundleExecutionFailure, FixtureRuntimeRequirement, run_directory,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DetachedAggregateArtifact {
    schema_version: &'static str,
    artifact_kind: &'static str,
    aggregate_context: AggregateContextArtifact,
    summary_core: AggregateCoreArtifact,
    detached_noncore: AggregateNoncoreArtifact,
}

#[derive(Debug, Serialize)]
struct AggregateContextArtifact {
    directory_path: String,
    aggregate_scope: &'static str,
}

#[derive(Debug, Serialize)]
struct AggregateCoreArtifact {
    total_bundles: usize,
    runtime_bundles: usize,
    static_only_bundles: usize,
    passed: usize,
    failed: usize,
    bundle_failure_kind_counts_scope: &'static str,
    bundle_failure_kind_counts: Vec<BundleFailureKindCountArtifact>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct HostPlanCoverageFailureRef {
    fixture_id: String,
    fixture_path: String,
    runtime_requirement: &'static str,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct BundleFailureKindCountArtifact {
    failure_kind: &'static str,
    count: usize,
}

#[derive(Debug, Serialize)]
struct AggregateNoncoreArtifact {
    discovery_failures: usize,
    bundle_failures: usize,
    host_plan_coverage_failures: Vec<HostPlanCoverageFailureRef>,
}

impl DetachedAggregateArtifact {
    fn from_summary(fixture_directory: PathBuf, summary: &BatchRunSummary) -> Self {
        Self {
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

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <fixture-directory> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         crates/mir-ast/tests/fixtures/current-l2 --output /tmp/current-l2-batch.json"
    )
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let mut fixture_directory = None;
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output_path = Some(PathBuf::from(value));
            }
            _ if fixture_directory.is_none() => {
                fixture_directory = Some(PathBuf::from(arg));
            }
            _ => {
                return Err(format!("unexpected argument: {arg}"));
            }
        }
    }

    fixture_directory
        .map(|path| (path, output_path))
        .ok_or_else(|| "missing <fixture-directory>".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_detached_aggregate".to_string());
    let (fixture_directory, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let summary = run_directory(&fixture_directory)?;
    let artifact = DetachedAggregateArtifact::from_summary(fixture_directory, &summary);
    let payload = serde_json::to_string_pretty(&artifact)?;

    match output_path {
        Some(path) => fs::write(path, payload)?,
        None => println!("{payload}"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn coverage_failure(path: &str, fixture_id: &str) -> BundleExecutionFailure {
        BundleExecutionFailure {
            fixture_path: PathBuf::from(path),
            fixture_id: fixture_id.to_string(),
            runtime_requirement: FixtureRuntimeRequirement::RuntimeWithHostPlan,
            error: "host plan did not cover all oracle calls".to_string(),
        }
    }

    #[test]
    fn detached_aggregate_keeps_anchor_refs_and_typed_count() {
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

        let artifact = DetachedAggregateArtifact::from_summary(
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
        assert_eq!(artifact.detached_noncore.bundle_failures, 1);
        assert_eq!(artifact.detached_noncore.discovery_failures, 0);
    }

    #[test]
    fn detached_aggregate_omits_typed_rows_when_no_coverage_failure_exists() {
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

        let artifact = DetachedAggregateArtifact::from_summary(
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
}
