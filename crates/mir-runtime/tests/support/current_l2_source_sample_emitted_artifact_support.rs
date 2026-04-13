use std::path::PathBuf;

use mir_runtime::current_l2::{CurrentL2SourceSampleRunReport, run_current_l2_source_sample};
use mir_semantics::{
    FixtureHostPlan, StaticGateVerdict, load_bundle_from_fixture_path, load_fixture_from_path,
    run_bundle, static_gate_detailed,
};

#[path = "../../../mir-semantics/examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../../../mir-semantics/examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../../../mir-semantics/examples/support/current_l2_model_check_carrier_support.rs"]
mod current_l2_model_check_carrier_support;
#[path = "../../../mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs"]
mod current_l2_proof_notebook_review_unit_support;
#[path = "../../../mir-semantics/examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;
use current_l2_formal_hook_support::{
    ToolNeutralFormalHookArtifact, build_formal_hook_from_detached_bundle_artifact,
    build_formal_hook_from_static_gate_artifact,
};
use current_l2_model_check_carrier_support::{
    ModelCheckConcreteCarrierArtifact, build_model_check_concrete_carrier_artifacts,
};
use current_l2_proof_notebook_review_unit_support::{
    ProofNotebookReviewUnitArtifact, build_proof_notebook_review_unit_artifacts,
};
use current_l2_static_gate_support::build_detached_static_gate_artifact;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentL2EmittedArtifactRouteStatus {
    Reached,
    GuardedNotReached,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleEmittedArtifactRoute {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub formal_hook_status: CurrentL2EmittedArtifactRouteStatus,
    pub formal_hook_guard_reason: Option<String>,
    pub formal_hook_artifact: Option<ToolNeutralFormalHookArtifact>,
    pub proof_notebook_review_units: Vec<ProofNotebookReviewUnitArtifact>,
    pub model_check_concrete_carriers: Vec<ModelCheckConcreteCarrierArtifact>,
}

pub fn build_current_l2_source_sample_emitted_artifact_route(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleEmittedArtifactRoute, String> {
    let source_report = run_current_l2_source_sample(sample_argument, host_plan)
        .map_err(|error| error.to_string())?;

    match build_formal_hook_for_source_sample(&source_report) {
        Ok(formal_hook_artifact) => {
            let proof_notebook_review_units =
                build_proof_notebook_review_unit_artifacts(&formal_hook_artifact)?;
            let model_check_concrete_carriers =
                build_model_check_concrete_carrier_artifacts(&formal_hook_artifact)?;

            Ok(CurrentL2SourceSampleEmittedArtifactRoute {
                source_report,
                formal_hook_status: CurrentL2EmittedArtifactRouteStatus::Reached,
                formal_hook_guard_reason: None,
                formal_hook_artifact: Some(formal_hook_artifact),
                proof_notebook_review_units,
                model_check_concrete_carriers,
            })
        }
        Err(error) => Ok(CurrentL2SourceSampleEmittedArtifactRoute {
            source_report,
            formal_hook_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            formal_hook_guard_reason: Some(error),
            formal_hook_artifact: None,
            proof_notebook_review_units: Vec::new(),
            model_check_concrete_carriers: Vec::new(),
        }),
    }
}

fn build_formal_hook_for_source_sample(
    source_report: &CurrentL2SourceSampleRunReport,
) -> Result<ToolNeutralFormalHookArtifact, String> {
    let fixture_path = fixture_path(&source_report.sample_id)?;

    match source_report.runtime_report.checker_floor.static_gate.verdict {
        StaticGateVerdict::Valid => {
            let bundle =
                load_bundle_from_fixture_path(fixture_path).map_err(|error| error.to_string())?;
            let runtime_report = run_bundle(&bundle).map_err(|error| error.to_string())?;
            let detached_bundle = build_detached_bundle_artifact(&bundle, &runtime_report);
            build_formal_hook_from_detached_bundle_artifact(&detached_bundle)
        }
        StaticGateVerdict::Malformed => {
            let fixture = load_fixture_from_path(&fixture_path).map_err(|error| error.to_string())?;
            let gate = static_gate_detailed(&fixture);
            let detached_static = build_detached_static_gate_artifact(fixture_path, &fixture, &gate);
            build_formal_hook_from_static_gate_artifact(&detached_static)
        }
        StaticGateVerdict::Underdeclared => Err(format!(
            "source sample `{}` is outside the current emitted artifact wiring floor because static gate verdict is underdeclared",
            source_report.sample_id
        )),
    }
}

fn fixture_path(sample_id: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(format!("{sample_id}.json"));
    if candidate.is_file() {
        Ok(candidate)
    } else {
        Err(format!(
            "fixture path for source sample `{sample_id}` is missing: {}",
            candidate.display()
        ))
    }
}
