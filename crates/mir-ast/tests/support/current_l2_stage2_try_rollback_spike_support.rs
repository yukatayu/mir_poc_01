use std::{fs, path::PathBuf};

use mir_ast::current_l2::Stage2ParsedTryFallback;
use serde_json::Value;

const TRY_FALLBACK_KIND: &str = "TryFallback";
const ATOMIC_CUT_KIND: &str = "AtomicCut";
const MISSING_FALLBACK_BODY: &str = "missing_fallback_body";
const DISALLOWED_FALLBACK_PLACEMENT: &str = "disallowed_fallback_placement";
const NO_FINDINGS: &str = "no_findings";
const FINDINGS_PRESENT: &str = "findings_present";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage2TryRollbackFindingRow {
    pub subject_kind: String,
    pub finding_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage2TryRollbackStructuralSummary {
    pub verdict: String,
    pub findings: Vec<Stage2TryRollbackFindingRow>,
}

pub fn summarize_stage2_try_rollback_findings(
    parsed: &Stage2ParsedTryFallback,
) -> Stage2TryRollbackStructuralSummary {
    let mut findings = Vec::new();

    if parsed.fallback_body().is_empty() {
        findings.push(Stage2TryRollbackFindingRow {
            subject_kind: TRY_FALLBACK_KIND.to_string(),
            finding_kind: MISSING_FALLBACK_BODY.to_string(),
        });
    }

    for statement in parsed.fallback_body() {
        if statement.is_atomic_cut() {
            findings.push(Stage2TryRollbackFindingRow {
                subject_kind: ATOMIC_CUT_KIND.to_string(),
                finding_kind: DISALLOWED_FALLBACK_PLACEMENT.to_string(),
            });
        }
    }

    let verdict = if findings.is_empty() {
        NO_FINDINGS.to_string()
    } else {
        FINDINGS_PRESENT.to_string()
    };

    Stage2TryRollbackStructuralSummary { verdict, findings }
}

pub fn load_expected_try_rollback_expectation(
    fixture_name: &str,
) -> Result<Stage2TryRollbackStructuralSummary, String> {
    let path = fixture_path(fixture_name);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read fixture {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse fixture {}: {error}", path.display()))?;
    let expected_static = value
        .get("expected_static")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("fixture {} is missing `expected_static`", path.display()))?;

    let verdict = expected_static
        .get("checked_try_rollback_structural_verdict")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "fixture {} is missing `checked_try_rollback_structural_verdict`",
                path.display()
            )
        })?
        .to_string();

    let finding_rows = expected_static
        .get("checked_try_rollback_structural_findings")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "fixture {} is missing `checked_try_rollback_structural_findings`",
                path.display()
            )
        })?;

    let mut findings = Vec::new();
    for row in finding_rows {
        let row_object = row.as_object().ok_or_else(|| {
            format!(
                "fixture {} has non-object try/rollback finding row",
                path.display()
            )
        })?;
        let subject_kind = row_object
            .get("subject_kind")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!(
                    "fixture {} has try/rollback row without `subject_kind`",
                    path.display()
                )
            })?;
        let finding_kind = row_object
            .get("finding_kind")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!(
                    "fixture {} has try/rollback row without `finding_kind`",
                    path.display()
                )
            })?;
        findings.push(Stage2TryRollbackFindingRow {
            subject_kind: subject_kind.to_string(),
            finding_kind: finding_kind.to_string(),
        });
    }

    Ok(Stage2TryRollbackStructuralSummary { verdict, findings })
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/current-l2")
        .join(name)
}
