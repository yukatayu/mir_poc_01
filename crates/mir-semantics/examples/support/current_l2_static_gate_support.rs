use std::path::PathBuf;

use mir_semantics::{
    CurrentL2Fixture, StaticGateResult, StaticGateVerdict, StaticReasonCodeRow,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetachedStaticGateArtifact {
    pub schema_version: String,
    pub artifact_kind: String,
    pub fixture_context: StaticGateFixtureContext,
    pub checker_core: StaticGateCheckerCore,
    // Helper-local, wording-derived mirror for stable reason clusters only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_noncore: Option<StaticGateDetachedNoncore>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StaticGateFixtureContext {
    pub fixture_id: String,
    pub fixture_path: String,
    pub source_example_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StaticGateCheckerCore {
    pub static_verdict: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StaticGateDetachedNoncore {
    pub reason_codes_scope: String,
    pub reason_codes: Vec<StaticReasonCodeRow>,
}

pub fn build_detached_static_gate_artifact(
    fixture_path: PathBuf,
    fixture: &CurrentL2Fixture,
    gate: &StaticGateResult,
) -> DetachedStaticGateArtifact {
    let reason_codes = detached_reason_code_rows(&gate.reasons);
    DetachedStaticGateArtifact {
        schema_version: "draft-current-l2-static-gate-v0".to_string(),
        artifact_kind: "current-l2-static-gate-detached-sketch".to_string(),
        fixture_context: StaticGateFixtureContext {
            fixture_id: fixture.fixture_id.clone(),
            fixture_path: fixture_path.display().to_string(),
            source_example_id: fixture.source_example_id.clone(),
        },
        checker_core: StaticGateCheckerCore {
            static_verdict: static_verdict_name(gate.verdict).to_string(),
            reasons: gate.reasons.clone(),
        },
        detached_noncore: (!reason_codes.is_empty()).then_some(StaticGateDetachedNoncore {
            reason_codes_scope: "stable-clusters-only".to_string(),
            reason_codes,
        }),
    }
}

fn static_verdict_name(value: StaticGateVerdict) -> &'static str {
    match value {
        StaticGateVerdict::Valid => "valid",
        StaticGateVerdict::Malformed => "malformed",
        StaticGateVerdict::Underdeclared => "underdeclared",
    }
}

fn detached_reason_code_rows(reasons: &[String]) -> Vec<StaticReasonCodeRow> {
    reasons
        .iter()
        .filter_map(|reason| detached_reason_code_from_reason(reason))
        .collect()
}

fn detached_reason_code_from_reason(reason: &str) -> Option<StaticReasonCodeRow> {
    if let Some((predecessor, successor)) =
        parse_reason_pair(reason, "missing lineage assertion for ", " -> ")
    {
        return Some(StaticReasonCodeRow::MissingLineageAssertion {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) = parse_reason_pair(
        reason,
        "lineage assertion does not describe ",
        " -> ",
    ) {
        return Some(StaticReasonCodeRow::LineageAssertionEdgeMismatch {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) = parse_reason_pair(
        reason,
        "declared access target is missing for ",
        " -> ",
    ) {
        return Some(StaticReasonCodeRow::DeclaredTargetMissing {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) = parse_reason_pair(
        reason,
        "declared access target mismatch between ",
        " and ",
    ) {
        return Some(StaticReasonCodeRow::DeclaredTargetMismatch {
            predecessor,
            successor,
        });
    }
    if let Some((from_capability, to_capability)) = parse_reason_pair(
        reason,
        "capability strengthens from ",
        " to ",
    ) {
        return Some(StaticReasonCodeRow::CapabilityStrengthens {
            from_capability,
            to_capability,
        });
    }
    if let Some((head, scope)) = parse_reason_pair(
        reason,
        "missing option declaration for chain head ",
        " at ",
    ) {
        return Some(StaticReasonCodeRow::MissingChainHeadOption { head, scope });
    }
    if let Some((option, scope)) = parse_reason_pair(
        reason,
        "missing predecessor option ",
        " at ",
    ) {
        return Some(StaticReasonCodeRow::MissingPredecessorOption {
            option,
            scope,
        });
    }
    if let Some((option, scope)) = parse_reason_pair(
        reason,
        "missing successor option ",
        " at ",
    ) {
        return Some(StaticReasonCodeRow::MissingSuccessorOption {
            option,
            scope,
        });
    }
    None
}

fn parse_reason_pair(reason: &str, prefix: &str, separator: &str) -> Option<(String, String)> {
    let tail = reason.strip_prefix(prefix)?;
    let (left, right) = tail.split_once(separator)?;
    Some((left.trim().to_owned(), right.trim().to_owned()))
}
