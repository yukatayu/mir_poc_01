use serde::{Deserialize, Serialize};

use super::current_l2_proof_notebook_review_unit_support::{
    ProofNotebookReviewUnitArtifact, ProofNotebookReviewUnitEvidenceRef,
};

const REVIEW_UNIT_SCHEMA_VERSION: &str = "draft-current-l2-proof-notebook-review-unit-v0";
const REVIEW_UNIT_ARTIFACT_KIND: &str = "current-l2-proof-notebook-review-unit";
const LEAN_STUB_SCHEMA_VERSION: &str = "draft-current-l2-lean-theorem-stub-v0";
const LEAN_STUB_ARTIFACT_KIND: &str = "current-l2-lean-theorem-stub";
const LEAN_STUB_TOOL_FAMILY: &str = "lean-first";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LeanTheoremStubArtifact {
    pub schema_version: String,
    pub artifact_kind: String,
    pub tool_family: String,
    pub module_name: String,
    pub theorem_name: String,
    pub subject_kind: String,
    pub subject_ref: String,
    pub obligation_kind: String,
    pub evidence_refs: Vec<LeanTheoremStubEvidenceRef>,
    pub source_text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LeanTheoremStubEvidenceRef {
    pub ref_kind: String,
    pub ref_id: String,
}

pub fn build_lean_theorem_stub_artifacts(
    review_units: &[ProofNotebookReviewUnitArtifact],
) -> Result<Vec<LeanTheoremStubArtifact>, String> {
    if review_units.is_empty() {
        return Err("review_units must not be empty".to_string());
    }

    review_units
        .iter()
        .map(build_lean_theorem_stub_artifact)
        .collect()
}

fn build_lean_theorem_stub_artifact(
    review_unit: &ProofNotebookReviewUnitArtifact,
) -> Result<LeanTheoremStubArtifact, String> {
    validate_review_unit(review_unit)?;
    if review_unit.subject_ref.trim().is_empty() {
        return Err("review_unit subject_ref must not be empty".to_string());
    }
    if review_unit.row.goal_text.trim().is_empty() {
        return Err("review_unit row goal_text must not be empty".to_string());
    }
    if review_unit.row.evidence_refs.is_empty() {
        return Err(format!(
            "review_unit row {} must include at least one evidence ref",
            review_unit.row.obligation_kind
        ));
    }

    let theorem_name = format!(
        "{}__{}",
        sanitize_identifier(&review_unit.subject_ref),
        sanitize_identifier(&review_unit.row.obligation_kind)
    );
    let module_name = format!(
        "CurrentL2.{}",
        pascal_case_identifier(&review_unit.subject_ref)
    );
    let evidence_refs = review_unit
        .row
        .evidence_refs
        .iter()
        .map(clone_evidence_ref)
        .collect::<Vec<_>>();
    let source_text = build_source_text(&module_name, &theorem_name, review_unit, &evidence_refs);

    Ok(LeanTheoremStubArtifact {
        schema_version: LEAN_STUB_SCHEMA_VERSION.to_string(),
        artifact_kind: LEAN_STUB_ARTIFACT_KIND.to_string(),
        tool_family: LEAN_STUB_TOOL_FAMILY.to_string(),
        module_name,
        theorem_name,
        subject_kind: review_unit.subject_kind.clone(),
        subject_ref: review_unit.subject_ref.clone(),
        obligation_kind: review_unit.row.obligation_kind.clone(),
        evidence_refs,
        source_text,
    })
}

fn build_source_text(
    module_name: &str,
    theorem_name: &str,
    review_unit: &ProofNotebookReviewUnitArtifact,
    evidence_refs: &[LeanTheoremStubEvidenceRef],
) -> String {
    let evidence_block = evidence_refs
        .iter()
        .map(|evidence| format!("-- evidence {}:{}", evidence.ref_kind, evidence.ref_id))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "/- current-l2 Lean-first theorem stub (non-production)\n\
subject_kind = {subject_kind}\n\
subject_ref = {subject_ref}\n\
obligation_kind = {obligation_kind}\n\
module_name = {module_name}\n\
-/\n\
namespace CurrentL2\n\n\
-- goal: {goal_text}\n\
{evidence_block}\n\
theorem {theorem_name} : Prop := by\n\
  sorry\n\n\
end CurrentL2\n",
        subject_kind = review_unit.subject_kind,
        subject_ref = review_unit.subject_ref,
        obligation_kind = review_unit.row.obligation_kind,
        module_name = module_name,
        goal_text = review_unit.row.goal_text,
        evidence_block = evidence_block,
        theorem_name = theorem_name,
    )
}

fn validate_review_unit(review_unit: &ProofNotebookReviewUnitArtifact) -> Result<(), String> {
    if review_unit.schema_version != REVIEW_UNIT_SCHEMA_VERSION {
        return Err(format!(
            "expected review_unit schema_version {REVIEW_UNIT_SCHEMA_VERSION}, got {}",
            review_unit.schema_version
        ));
    }
    if review_unit.artifact_kind != REVIEW_UNIT_ARTIFACT_KIND {
        return Err(format!(
            "expected review_unit artifact_kind {REVIEW_UNIT_ARTIFACT_KIND}, got {}",
            review_unit.artifact_kind
        ));
    }
    Ok(())
}

fn clone_evidence_ref(evidence_ref: &ProofNotebookReviewUnitEvidenceRef) -> LeanTheoremStubEvidenceRef {
    LeanTheoremStubEvidenceRef {
        ref_kind: evidence_ref.ref_kind.clone(),
        ref_id: evidence_ref.ref_id.clone(),
    }
}

fn sanitize_identifier(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut last_was_underscore = false;

    for ch in input.chars() {
        let normalized = if ch.is_ascii_alphanumeric() { ch } else { '_' };
        if normalized == '_' {
            if !last_was_underscore {
                output.push('_');
            }
            last_was_underscore = true;
        } else {
            output.push(normalized);
            last_was_underscore = false;
        }
    }

    while output.ends_with('_') {
        output.pop();
    }

    if output.is_empty() {
        return "current_l2_goal".to_string();
    }

    if output.chars().next().unwrap().is_ascii_digit() {
        output.insert(0, '_');
    }

    output
}

fn pascal_case_identifier(input: &str) -> String {
    let sanitized = sanitize_identifier(input);
    let mut output = String::new();

    for segment in sanitized.split('_').filter(|segment| !segment.is_empty()) {
        let mut chars = segment.chars();
        if let Some(first) = chars.next() {
            output.push(first.to_ascii_uppercase());
            output.extend(chars.map(|ch| ch.to_ascii_lowercase()));
        }
    }

    if output.is_empty() {
        "CurrentL2Goal".to_string()
    } else {
        output
    }
}
