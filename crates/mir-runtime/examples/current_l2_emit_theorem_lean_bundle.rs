use std::{env, fs, path::PathBuf, process};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};
use serde::Serialize;

#[path = "../tests/support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, build_current_l2_source_sample_theorem_lean_stub_pilot_actualization,
};

#[derive(Debug, Serialize)]
struct LeanStubArtifactPayload {
    schema_version: String,
    artifact_kind: String,
    tool_family: String,
    module_name: String,
    theorem_name: String,
    subject_kind: String,
    subject_ref: String,
    obligation_kind: String,
    evidence_refs: Vec<LeanStubEvidenceRefPayload>,
    source_text: String,
}

#[derive(Debug, Serialize)]
struct LeanStubEvidenceRefPayload {
    ref_kind: String,
    ref_id: String,
}

#[derive(Debug, Serialize)]
struct LeanBundlePayload {
    sample_argument: String,
    sample_id: String,
    sample_path: String,
    pilot_status: String,
    pilot_guard_reason: Option<String>,
    pilot_subject_kind: Option<String>,
    pilot_subject_ref: Option<String>,
    principal_review_unit_refs: Vec<String>,
    binding_preflight_manifest_refs: Vec<String>,
    pilot_binding_refs: Vec<String>,
    code_anchor_refs: Vec<String>,
    repo_local_emitted_artifact_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    lean_stub_artifacts: Vec<LeanStubArtifactPayload>,
}

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <sample-argument> [--host-plan <path>] [--output <bundle-path>]\n\
         example: cargo run -p mir-runtime --example {program} -- \\\n\
         e5-underdeclared-lineage --output /tmp/e5.lean-bundle.json"
    )
}

fn parse_args() -> Result<(String, FixtureHostPlan, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let sample_argument = args
        .next()
        .ok_or_else(|| "missing <sample-argument>".to_string())?;
    let mut host_plan = FixtureHostPlan::default();
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--host-plan" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --host-plan".to_string())?;
                host_plan = load_host_plan_from_path(&PathBuf::from(value))
                    .map_err(|error| format!("failed to load host plan: {error}"))?;
            }
            "-o" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output_path = Some(PathBuf::from(value));
            }
            _ => return Err(format!("unexpected argument: {arg}")),
        }
    }

    Ok((sample_argument, host_plan, output_path))
}

fn status_name(status: CurrentL2EmittedArtifactRouteStatus) -> &'static str {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => "reached",
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => "guarded_not_reached",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_theorem_lean_bundle".to_string());
    let (sample_argument, host_plan, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let actualization =
        build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(&sample_argument, host_plan)?;

    let payload = LeanBundlePayload {
        sample_argument,
        sample_id: actualization.source_report.sample_id,
        sample_path: actualization
            .source_report
            .sample_path
            .display()
            .to_string(),
        pilot_status: status_name(actualization.pilot_status).to_string(),
        pilot_guard_reason: actualization.pilot_guard_reason,
        pilot_subject_kind: actualization.pilot_subject_kind,
        pilot_subject_ref: actualization.pilot_subject_ref,
        principal_review_unit_refs: actualization.principal_review_unit_refs,
        binding_preflight_manifest_refs: actualization.binding_preflight_manifest_refs,
        pilot_binding_refs: actualization.pilot_binding_refs,
        code_anchor_refs: actualization.code_anchor_refs,
        repo_local_emitted_artifact_refs: actualization.repo_local_emitted_artifact_refs,
        compare_floor_refs: actualization.compare_floor_refs,
        guard_refs: actualization.guard_refs,
        kept_later_refs: actualization.kept_later_refs,
        lean_stub_artifacts: actualization
            .lean_stub_artifacts
            .into_iter()
            .map(|artifact| LeanStubArtifactPayload {
                schema_version: artifact.schema_version,
                artifact_kind: artifact.artifact_kind,
                tool_family: artifact.tool_family,
                module_name: artifact.module_name,
                theorem_name: artifact.theorem_name,
                subject_kind: artifact.subject_kind,
                subject_ref: artifact.subject_ref,
                obligation_kind: artifact.obligation_kind,
                evidence_refs: artifact
                    .evidence_refs
                    .into_iter()
                    .map(|evidence| LeanStubEvidenceRefPayload {
                        ref_kind: evidence.ref_kind,
                        ref_id: evidence.ref_id,
                    })
                    .collect(),
                source_text: artifact.source_text,
            })
            .collect(),
    };

    let output = serde_json::to_string_pretty(&payload)?;

    match output_path {
        Some(path) => fs::write(path, output)?,
        None => println!("{output}"),
    }

    Ok(())
}
