use std::path::PathBuf;

use mir_semantics::{CurrentL2Fixture, StaticGateResult, StaticGateVerdict};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct DetachedStaticGateArtifact {
    pub schema_version: &'static str,
    pub artifact_kind: &'static str,
    pub fixture_context: StaticGateFixtureContext,
    pub checker_core: StaticGateCheckerCore,
    // Helper-local, wording-derived mirror for stable reason clusters only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_noncore: Option<StaticGateDetachedNoncore>,
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

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct StaticGateDetachedNoncore {
    pub reason_codes_scope: &'static str,
    pub reason_codes: Vec<StaticReasonCodeRow>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StaticReasonCodeRow {
    MissingLineageAssertion {
        predecessor: String,
        successor: String,
    },
    LineageAssertionEdgeMismatch {
        predecessor: String,
        successor: String,
    },
    DeclaredTargetMissing {
        predecessor: String,
        successor: String,
    },
    DeclaredTargetMismatch {
        predecessor: String,
        successor: String,
    },
    CapabilityStrengthens {
        from_capability: String,
        to_capability: String,
    },
    MissingChainHeadOption {
        head: String,
        scope: String,
    },
    MissingPredecessorOption {
        option: String,
        scope: String,
    },
    MissingSuccessorOption {
        option: String,
        scope: String,
    },
}

pub fn build_detached_static_gate_artifact(
    fixture_path: PathBuf,
    fixture: &CurrentL2Fixture,
    gate: &StaticGateResult,
) -> DetachedStaticGateArtifact {
    let reason_codes = gate
        .reasons
        .iter()
        .filter_map(|reason| reason_code_from_reason(reason))
        .collect::<Vec<_>>();
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
        detached_noncore: (!reason_codes.is_empty()).then_some(StaticGateDetachedNoncore {
            reason_codes_scope: "stable-clusters-only",
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

fn reason_code_from_reason(reason: &str) -> Option<StaticReasonCodeRow> {
    if let Some((predecessor, successor)) =
        parse_edge_reason(reason, "missing lineage assertion for ")
    {
        return Some(StaticReasonCodeRow::MissingLineageAssertion {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) =
        parse_edge_reason(reason, "lineage assertion does not describe ")
    {
        return Some(StaticReasonCodeRow::LineageAssertionEdgeMismatch {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) =
        parse_edge_reason(reason, "declared access target is missing for ")
    {
        return Some(StaticReasonCodeRow::DeclaredTargetMissing {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) =
        parse_pair_reason(reason, "declared access target mismatch between ", " and ")
    {
        return Some(StaticReasonCodeRow::DeclaredTargetMismatch {
            predecessor,
            successor,
        });
    }
    if let Some((from_capability, to_capability)) =
        parse_pair_reason(reason, "capability strengthens from ", " to ")
    {
        return Some(StaticReasonCodeRow::CapabilityStrengthens {
            from_capability,
            to_capability,
        });
    }
    if let Some((head, scope)) =
        parse_pair_reason(reason, "missing option declaration for chain head ", " at ")
    {
        return Some(StaticReasonCodeRow::MissingChainHeadOption { head, scope });
    }
    if let Some((option, scope)) =
        parse_pair_reason(reason, "missing predecessor option ", " at ")
    {
        return Some(StaticReasonCodeRow::MissingPredecessorOption { option, scope });
    }
    if let Some((option, scope)) = parse_pair_reason(reason, "missing successor option ", " at ")
    {
        return Some(StaticReasonCodeRow::MissingSuccessorOption { option, scope });
    }
    None
}

fn parse_edge_reason(reason: &str, prefix: &str) -> Option<(String, String)> {
    parse_pair_reason(reason, prefix, " -> ")
}

fn parse_pair_reason(reason: &str, prefix: &str, separator: &str) -> Option<(String, String)> {
    let rest = reason.strip_prefix(prefix)?;
    let (left, right) = rest.split_once(separator)?;
    Some((left.to_string(), right.to_string()))
}
