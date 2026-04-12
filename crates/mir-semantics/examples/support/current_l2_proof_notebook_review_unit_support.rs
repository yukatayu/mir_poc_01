use serde::{Deserialize, Serialize};

use super::current_l2_formal_hook_support::{
    ToolNeutralFormalEvidenceRef, ToolNeutralFormalHookArtifact,
};

const FORMAL_HOOK_SCHEMA_VERSION: &str = "draft-current-l2-formal-hook-v0";
const FORMAL_HOOK_ARTIFACT_KIND: &str = "current-l2-tool-neutral-formal-hook";
const REVIEW_UNIT_SCHEMA_VERSION: &str = "draft-current-l2-proof-notebook-review-unit-v0";
const REVIEW_UNIT_ARTIFACT_KIND: &str = "current-l2-proof-notebook-review-unit";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofNotebookReviewUnitArtifact {
    pub schema_version: String,
    pub artifact_kind: String,
    pub subject_kind: String,
    pub subject_ref: String,
    pub review_rows: Vec<ProofNotebookReviewRow>,
    pub checklist: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofNotebookReviewRow {
    pub obligation_kind: String,
    pub evidence_refs: Vec<ProofNotebookReviewUnitEvidenceRef>,
    pub goal_text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofNotebookReviewUnitEvidenceRef {
    pub ref_kind: String,
    pub ref_id: String,
}

pub fn build_proof_notebook_review_unit_artifact(
    artifact: &ToolNeutralFormalHookArtifact,
) -> Result<ProofNotebookReviewUnitArtifact, String> {
    validate_formal_hook_artifact(artifact)?;
    if artifact.subject_ref.is_empty() {
        return Err("formal hook subject_ref must not be empty".to_string());
    }
    if artifact.contract_rows.is_empty() {
        return Err("formal hook contract_rows must not be empty".to_string());
    }

    let review_rows = artifact
        .contract_rows
        .iter()
        .map(|row| build_review_row(&artifact.subject_kind, &artifact.subject_ref, row))
        .collect::<Result<Vec<_>, _>>()?;
    let checklist = build_checklist(artifact, &review_rows);

    Ok(ProofNotebookReviewUnitArtifact {
        schema_version: REVIEW_UNIT_SCHEMA_VERSION.to_string(),
        artifact_kind: REVIEW_UNIT_ARTIFACT_KIND.to_string(),
        subject_kind: artifact.subject_kind.clone(),
        subject_ref: artifact.subject_ref.clone(),
        review_rows,
        checklist,
    })
}

fn build_review_row(
    subject_kind: &str,
    subject_ref: &str,
    row: &super::current_l2_formal_hook_support::ToolNeutralFormalContractRow,
) -> Result<ProofNotebookReviewRow, String> {
    if row.evidence_refs.is_empty() {
        return Err(format!(
            "formal hook row {} must include at least one evidence ref",
            row.obligation_kind
        ));
    }

    let goal_text = derive_goal_text(subject_kind, subject_ref, &row.obligation_kind)?;
    let evidence_refs = row
        .evidence_refs
        .iter()
        .map(clone_evidence_ref)
        .collect::<Vec<_>>();

    Ok(ProofNotebookReviewRow {
        obligation_kind: row.obligation_kind.clone(),
        evidence_refs,
        goal_text,
    })
}

fn derive_goal_text(
    subject_kind: &str,
    subject_ref: &str,
    obligation_kind: &str,
) -> Result<String, String> {
    match (subject_kind, obligation_kind) {
        ("runtime_try_cut_cluster", "rollback_cut_non_interference") => Ok(format!(
            "Review that rollback / atomic-cut evidence does not interfere with surviving runtime behavior for `{subject_ref}`."
        )),
        ("fixture_static_cluster", "canonical_normalization_law") => Ok(format!(
            "Review that canonical normalization preserves the rejected static shape for `{subject_ref}`."
        )),
        ("fixture_static_cluster", "no_re_promotion") => Ok(format!(
            "Review that `{subject_ref}` remains rejected and is not re-promoted by later tooling."
        )),
        _ => Err(format!(
            "unsupported proof notebook review unit pair: subject_kind={subject_kind}, obligation_kind={obligation_kind}"
        )),
    }
}

fn build_checklist(
    artifact: &ToolNeutralFormalHookArtifact,
    review_rows: &[ProofNotebookReviewRow],
) -> Vec<String> {
    let mut checklist = Vec::with_capacity(review_rows.len() + 2);
    checklist.push(format!(
        "Confirm subject `{}` is still reviewed as `{}`.",
        artifact.subject_ref, artifact.subject_kind
    ));
    for row in review_rows {
        checklist.push(format!(
            "Check `{}` against {}.",
            row.obligation_kind,
            format_evidence_refs(&row.evidence_refs)
        ));
    }
    checklist.push(
        "Fail closed if any evidence ref is missing, mismatched, or no longer supports the stated goal."
            .to_string(),
    );
    checklist
}

fn format_evidence_refs(evidence_refs: &[ProofNotebookReviewUnitEvidenceRef]) -> String {
    evidence_refs
        .iter()
        .map(|evidence| format!("`{}:{}`", evidence.ref_kind, evidence.ref_id))
        .collect::<Vec<_>>()
        .join(", ")
}

fn clone_evidence_ref(
    evidence_ref: &ToolNeutralFormalEvidenceRef,
) -> ProofNotebookReviewUnitEvidenceRef {
    ProofNotebookReviewUnitEvidenceRef {
        ref_kind: evidence_ref.ref_kind.clone(),
        ref_id: evidence_ref.ref_id.clone(),
    }
}

fn validate_formal_hook_artifact(artifact: &ToolNeutralFormalHookArtifact) -> Result<(), String> {
    if artifact.schema_version != FORMAL_HOOK_SCHEMA_VERSION {
        return Err(format!(
            "expected formal hook schema_version {FORMAL_HOOK_SCHEMA_VERSION}, got {}",
            artifact.schema_version
        ));
    }
    if artifact.artifact_kind != FORMAL_HOOK_ARTIFACT_KIND {
        return Err(format!(
            "expected formal hook artifact_kind {FORMAL_HOOK_ARTIFACT_KIND}, got {}",
            artifact.artifact_kind
        ));
    }
    match artifact.subject_kind.as_str() {
        "fixture_static_cluster" | "runtime_try_cut_cluster" => Ok(()),
        _ => Err(format!(
            "unsupported formal hook subject_kind {}",
            artifact.subject_kind
        )),
    }
}
