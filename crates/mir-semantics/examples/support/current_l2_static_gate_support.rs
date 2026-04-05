use std::path::PathBuf;

use mir_semantics::{CurrentL2Fixture, StaticGateResult, StaticGateVerdict};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct DetachedStaticGateArtifact {
    pub schema_version: &'static str,
    pub artifact_kind: &'static str,
    pub fixture_context: StaticGateFixtureContext,
    pub checker_core: StaticGateCheckerCore,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct StaticGateFixtureContext {
    pub fixture_id: String,
    pub fixture_path: String,
    pub source_example_id: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct StaticGateCheckerCore {
    pub static_verdict: &'static str,
    pub reasons: Vec<String>,
}

pub fn build_detached_static_gate_artifact(
    fixture_path: PathBuf,
    fixture: &CurrentL2Fixture,
    gate: &StaticGateResult,
) -> DetachedStaticGateArtifact {
    DetachedStaticGateArtifact {
        schema_version: "draft-current-l2-static-gate-v0",
        artifact_kind: "current-l2-static-gate-detached-sketch",
        fixture_context: StaticGateFixtureContext {
            fixture_id: fixture.fixture_id.clone(),
            fixture_path: fixture_path.display().to_string(),
            source_example_id: fixture.source_example_id.clone(),
        },
        checker_core: StaticGateCheckerCore {
            static_verdict: static_verdict_name(gate.verdict),
            reasons: gate.reasons.clone(),
        },
    }
}

fn static_verdict_name(value: StaticGateVerdict) -> &'static str {
    match value {
        StaticGateVerdict::Valid => "valid",
        StaticGateVerdict::Malformed => "malformed",
        StaticGateVerdict::Underdeclared => "underdeclared",
    }
}
