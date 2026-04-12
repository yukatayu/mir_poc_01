use mir_semantics::{
    BundleRunReport, EventKind, FixtureBundle, FixtureRuntimeRequirement, NonAdmissibleMetadata,
    NonAdmissibleSubreason, StaticGateVerdict, TerminalOutcome,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetachedBundleArtifact {
    pub schema_version: String,
    pub artifact_kind: String,
    pub bundle_context: BundleContextArtifact,
    pub payload_core: PayloadCoreArtifact,
    pub detached_noncore: DetachedNoncoreArtifact,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleContextArtifact {
    pub fixture_id: String,
    pub fixture_path: String,
    pub host_plan_path: Option<String>,
    pub runtime_requirement: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PayloadCoreArtifact {
    pub static_verdict: String,
    pub entered_evaluation: bool,
    pub terminal_outcome: Option<String>,
    pub event_kinds: Vec<String>,
    pub non_admissible_metadata: Vec<NonAdmissibleMetadataArtifact>,
    pub narrative_explanations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NonAdmissibleMetadataArtifact {
    pub option_ref: String,
    pub subreason: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetachedNoncoreArtifact {
    pub steps_executed: usize,
}

pub fn build_detached_bundle_artifact(
    bundle: &FixtureBundle,
    bundle_report: &BundleRunReport,
) -> DetachedBundleArtifact {
    let report = &bundle_report.report;
    DetachedBundleArtifact {
        schema_version: "draft-current-l2-detached-bundle-v0".to_string(),
        artifact_kind: "current-l2-bundle-detached-sketch".to_string(),
        bundle_context: BundleContextArtifact {
            fixture_id: bundle.fixture.fixture_id.clone(),
            fixture_path: bundle.fixture_path.display().to_string(),
            host_plan_path: bundle
                .host_plan_path
                .as_ref()
                .map(|path| path.display().to_string()),
            runtime_requirement: runtime_requirement_name(bundle.runtime_requirement).to_string(),
        },
        payload_core: PayloadCoreArtifact {
            static_verdict: static_verdict_name(report.static_verdict).to_string(),
            entered_evaluation: report.entered_evaluation,
            terminal_outcome: report
                .terminal_outcome
                .map(|value| terminal_outcome_name(value).to_string()),
            event_kinds: report
                .trace_audit_sink
                .events
                .iter()
                .copied()
                .map(|value| event_kind_name(value).to_string())
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
        subreason: non_admissible_subreason_name(value.subreason).to_string(),
    }
}
