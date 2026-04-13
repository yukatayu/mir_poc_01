use serde::{Deserialize, Serialize};

use super::current_l2_formal_hook_support::{
    ToolNeutralFormalContractRow, ToolNeutralFormalEvidenceRef, ToolNeutralFormalHookArtifact,
};

const FORMAL_HOOK_SCHEMA_VERSION: &str = "draft-current-l2-formal-hook-v0";
const FORMAL_HOOK_ARTIFACT_KIND: &str = "current-l2-tool-neutral-formal-hook";
const MODEL_CHECK_CARRIER_SCHEMA_VERSION: &str = "draft-current-l2-model-check-concrete-carrier-v0";
const MODEL_CHECK_CARRIER_ARTIFACT_KIND: &str = "current-l2-model-check-concrete-carrier";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelCheckConcreteCarrierArtifact {
    pub schema_version: String,
    pub artifact_kind: String,
    pub subject_kind: String,
    pub subject_ref: String,
    pub case: ModelCheckConcreteCarrierCase,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelCheckConcreteCarrierCase {
    pub obligation_kind: String,
    pub evidence_refs: Vec<ModelCheckConcreteCarrierEvidenceRef>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelCheckConcreteCarrierEvidenceRef {
    pub ref_kind: String,
    pub ref_id: String,
}

pub fn build_model_check_concrete_carrier_artifacts(
    artifact: &ToolNeutralFormalHookArtifact,
) -> Result<Vec<ModelCheckConcreteCarrierArtifact>, String> {
    validate_formal_hook_artifact(artifact)?;
    if artifact.subject_ref.is_empty() {
        return Err("formal hook subject_ref must not be empty".to_string());
    }
    if artifact.contract_rows.is_empty() {
        return Err("formal hook contract_rows must not be empty".to_string());
    }

    artifact
        .contract_rows
        .iter()
        .map(|row| build_carrier_case(&artifact.subject_kind, row))
        .map(|case| {
            case.map(|case| ModelCheckConcreteCarrierArtifact {
                schema_version: MODEL_CHECK_CARRIER_SCHEMA_VERSION.to_string(),
                artifact_kind: MODEL_CHECK_CARRIER_ARTIFACT_KIND.to_string(),
                subject_kind: artifact.subject_kind.clone(),
                subject_ref: artifact.subject_ref.clone(),
                case,
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn build_carrier_case(
    subject_kind: &str,
    row: &ToolNeutralFormalContractRow,
) -> Result<ModelCheckConcreteCarrierCase, String> {
    if row.evidence_refs.is_empty() {
        return Err(format!(
            "formal hook row {} must include at least one evidence ref",
            row.obligation_kind
        ));
    }
    validate_supported_pair(subject_kind, &row.obligation_kind)?;

    Ok(ModelCheckConcreteCarrierCase {
        obligation_kind: row.obligation_kind.clone(),
        evidence_refs: row.evidence_refs.iter().map(clone_evidence_ref).collect(),
    })
}

fn validate_supported_pair(subject_kind: &str, obligation_kind: &str) -> Result<(), String> {
    match (subject_kind, obligation_kind) {
        ("runtime_try_cut_cluster", "rollback_cut_non_interference")
        | ("fixture_static_cluster", "canonical_normalization_law")
        | ("fixture_static_cluster", "no_re_promotion") => Ok(()),
        _ => Err(format!(
            "unsupported model-check carrier pair: subject_kind={subject_kind}, obligation_kind={obligation_kind}"
        )),
    }
}

fn clone_evidence_ref(
    evidence_ref: &ToolNeutralFormalEvidenceRef,
) -> ModelCheckConcreteCarrierEvidenceRef {
    ModelCheckConcreteCarrierEvidenceRef {
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
