use std::{env, fs, path::PathBuf, process};

use mir_semantics::{
    BundleRunReport, EventKind, FixtureBundle, FixtureRuntimeRequirement, NonAdmissibleMetadata,
    NonAdmissibleSubreason, StaticGateVerdict, TerminalOutcome, load_bundle_from_fixture_path,
    run_bundle,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DetachedBundleArtifact {
    schema_version: &'static str,
    artifact_kind: &'static str,
    bundle_context: BundleContextArtifact,
    payload_core: PayloadCoreArtifact,
    detached_noncore: DetachedNoncoreArtifact,
}

#[derive(Debug, Serialize)]
struct BundleContextArtifact {
    fixture_id: String,
    fixture_path: String,
    host_plan_path: Option<String>,
    runtime_requirement: &'static str,
}

#[derive(Debug, Serialize)]
struct PayloadCoreArtifact {
    static_verdict: &'static str,
    entered_evaluation: bool,
    terminal_outcome: Option<&'static str>,
    event_kinds: Vec<&'static str>,
    non_admissible_metadata: Vec<NonAdmissibleMetadataArtifact>,
    narrative_explanations: Vec<String>,
}

#[derive(Debug, Serialize)]
struct NonAdmissibleMetadataArtifact {
    option_ref: String,
    subreason: &'static str,
}

#[derive(Debug, Serialize)]
struct DetachedNoncoreArtifact {
    steps_executed: usize,
}

impl DetachedBundleArtifact {
    fn from_bundle(bundle: &FixtureBundle, bundle_report: &BundleRunReport) -> Self {
        let report = &bundle_report.report;
        Self {
            schema_version: "draft-current-l2-detached-bundle-v0",
            artifact_kind: "current-l2-bundle-detached-sketch",
            bundle_context: BundleContextArtifact {
                fixture_id: bundle.fixture.fixture_id.clone(),
                fixture_path: bundle.fixture_path.display().to_string(),
                host_plan_path: bundle
                    .host_plan_path
                    .as_ref()
                    .map(|path| path.display().to_string()),
                runtime_requirement: runtime_requirement_name(bundle.runtime_requirement),
            },
            payload_core: PayloadCoreArtifact {
                static_verdict: static_verdict_name(report.static_verdict),
                entered_evaluation: report.entered_evaluation,
                terminal_outcome: report.terminal_outcome.map(terminal_outcome_name),
                event_kinds: report
                    .trace_audit_sink
                    .events
                    .iter()
                    .copied()
                    .map(event_kind_name)
                    .collect(),
                non_admissible_metadata: report
                    .trace_audit_sink
                    .non_admissible_metadata
                    .iter()
                    .map(non_admissible_metadata_artifact)
                    .collect(),
                narrative_explanations: report.trace_audit_sink.narrative_explanations.clone(),
            },
            detached_noncore: DetachedNoncoreArtifact {
                steps_executed: report.steps_executed,
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

fn static_verdict_name(value: StaticGateVerdict) -> &'static str {
    match value {
        StaticGateVerdict::Valid => "valid",
        StaticGateVerdict::Malformed => "malformed",
        StaticGateVerdict::Underdeclared => "underdeclared",
    }
}

fn terminal_outcome_name(value: TerminalOutcome) -> &'static str {
    match value {
        TerminalOutcome::Success => "success",
        TerminalOutcome::ExplicitFailure => "explicit_failure",
        TerminalOutcome::Reject => "Reject",
    }
}

fn event_kind_name(value: EventKind) -> &'static str {
    match value {
        EventKind::PerformSuccess => "perform-success",
        EventKind::PerformFailure => "perform-failure",
        EventKind::Rollback => "rollback",
        EventKind::AtomicCut => "atomic-cut",
        EventKind::Reject => "Reject",
    }
}

fn non_admissible_subreason_name(value: NonAdmissibleSubreason) -> &'static str {
    match value {
        NonAdmissibleSubreason::AdmitMiss => "admit-miss",
        NonAdmissibleSubreason::LeaseExpired => "lease-expired",
    }
}

fn non_admissible_metadata_artifact(
    value: &NonAdmissibleMetadata,
) -> NonAdmissibleMetadataArtifact {
    NonAdmissibleMetadataArtifact {
        option_ref: value.option_ref.clone(),
        subreason: non_admissible_subreason_name(value.subreason),
    }
}

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <fixture-path> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --output /tmp/e3.json"
    )
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let mut fixture_path = None;
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output_path = Some(PathBuf::from(value));
            }
            _ if fixture_path.is_none() => {
                fixture_path = Some(PathBuf::from(arg));
            }
            _ => {
                return Err(format!("unexpected argument: {arg}"));
            }
        }
    }

    fixture_path
        .map(|path| (path, output_path))
        .ok_or_else(|| "missing <fixture-path>".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_detached_bundle".to_string());
    let (fixture_path, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let bundle = load_bundle_from_fixture_path(&fixture_path)?;
    let report = run_bundle(&bundle)?;
    let artifact = DetachedBundleArtifact::from_bundle(&bundle, &report);
    let payload = serde_json::to_string_pretty(&artifact)?;

    match output_path {
        Some(path) => fs::write(path, payload)?,
        None => println!("{payload}"),
    }

    Ok(())
}
