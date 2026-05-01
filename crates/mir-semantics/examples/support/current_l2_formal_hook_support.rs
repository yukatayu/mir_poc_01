use serde::{Deserialize, Serialize};

use super::current_l2_detached_bundle_support::DetachedBundleArtifact;
use super::current_l2_static_gate_support::DetachedStaticGateArtifact;

const FORMAL_HOOK_SCHEMA_VERSION: &str = "draft-current-l2-formal-hook-v0";
const FORMAL_HOOK_ARTIFACT_KIND: &str = "current-l2-tool-neutral-formal-hook";
const STATIC_GATE_SCHEMA_VERSION: &str = "draft-current-l2-static-gate-v0";
const STATIC_GATE_ARTIFACT_KIND: &str = "current-l2-static-gate-detached-sketch";
#[allow(dead_code)]
const DETACHED_BUNDLE_SCHEMA_VERSION: &str = "draft-current-l2-detached-bundle-v0";
#[allow(dead_code)]
const DETACHED_BUNDLE_ARTIFACT_KIND: &str = "current-l2-bundle-detached-sketch";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolNeutralFormalHookArtifact {
    pub schema_version: String,
    pub artifact_kind: String,
    pub subject_kind: String,
    pub subject_ref: String,
    pub contract_rows: Vec<ToolNeutralFormalContractRow>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolNeutralFormalContractRow {
    pub obligation_kind: String,
    pub evidence_refs: Vec<ToolNeutralFormalEvidenceRef>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolNeutralFormalEvidenceRef {
    pub ref_kind: String,
    pub ref_id: String,
}

pub fn build_formal_hook_from_static_gate_artifact(
    artifact: &DetachedStaticGateArtifact,
) -> Result<ToolNeutralFormalHookArtifact, String> {
    validate_static_gate_artifact(artifact)?;
    if artifact.checker_core.static_verdict == "valid" {
        return Err("fixture_static_cluster requires a non-valid static gate artifact".to_string());
    }

    let subject_ref = artifact.fixture_context.fixture_id.clone();

    Ok(ToolNeutralFormalHookArtifact {
        schema_version: FORMAL_HOOK_SCHEMA_VERSION.to_string(),
        artifact_kind: FORMAL_HOOK_ARTIFACT_KIND.to_string(),
        subject_kind: "fixture_static_cluster".to_string(),
        subject_ref: subject_ref.clone(),
        contract_rows: vec![
            ToolNeutralFormalContractRow {
                obligation_kind: "canonical_normalization_law".to_string(),
                evidence_refs: vec![
                    symbolic_evidence_ref("fixture", subject_ref.clone()),
                    symbolic_evidence_ref("static_gate_artifact", subject_ref.clone()),
                ],
            },
            ToolNeutralFormalContractRow {
                obligation_kind: "no_re_promotion".to_string(),
                evidence_refs: vec![symbolic_evidence_ref("fixture", subject_ref)],
            },
        ],
    })
}

#[allow(dead_code)]
pub fn build_formal_hook_from_detached_bundle_artifact(
    artifact: &DetachedBundleArtifact,
) -> Result<ToolNeutralFormalHookArtifact, String> {
    validate_detached_bundle_artifact(artifact)?;

    let has_try_cut = artifact
        .payload_core
        .event_kinds
        .iter()
        .any(|kind| kind == "rollback" || kind == "atomic-cut");
    if !has_try_cut {
        return Err(
            "runtime_try_cut_cluster requires rollback or atomic-cut evidence in the detached bundle artifact"
                .to_string(),
        );
    }

    let subject_ref = artifact.bundle_context.fixture_id.clone();

    Ok(ToolNeutralFormalHookArtifact {
        schema_version: FORMAL_HOOK_SCHEMA_VERSION.to_string(),
        artifact_kind: FORMAL_HOOK_ARTIFACT_KIND.to_string(),
        subject_kind: "runtime_try_cut_cluster".to_string(),
        subject_ref: subject_ref.clone(),
        contract_rows: vec![ToolNeutralFormalContractRow {
            obligation_kind: "rollback_cut_non_interference".to_string(),
            evidence_refs: vec![
                symbolic_evidence_ref("fixture", subject_ref.clone()),
                symbolic_evidence_ref("runtime_cluster", subject_ref),
            ],
        }],
    })
}

fn symbolic_evidence_ref(ref_kind: &str, ref_id: String) -> ToolNeutralFormalEvidenceRef {
    ToolNeutralFormalEvidenceRef {
        ref_kind: ref_kind.to_string(),
        ref_id,
    }
}

fn validate_static_gate_artifact(artifact: &DetachedStaticGateArtifact) -> Result<(), String> {
    if artifact.schema_version != STATIC_GATE_SCHEMA_VERSION {
        return Err(format!(
            "expected static gate schema_version {STATIC_GATE_SCHEMA_VERSION}, got {}",
            artifact.schema_version
        ));
    }
    if artifact.artifact_kind != STATIC_GATE_ARTIFACT_KIND {
        return Err(format!(
            "expected static gate artifact_kind {STATIC_GATE_ARTIFACT_KIND}, got {}",
            artifact.artifact_kind
        ));
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_detached_bundle_artifact(artifact: &DetachedBundleArtifact) -> Result<(), String> {
    if artifact.schema_version != DETACHED_BUNDLE_SCHEMA_VERSION {
        return Err(format!(
            "expected detached bundle schema_version {DETACHED_BUNDLE_SCHEMA_VERSION}, got {}",
            artifact.schema_version
        ));
    }
    if artifact.artifact_kind != DETACHED_BUNDLE_ARTIFACT_KIND {
        return Err(format!(
            "expected detached bundle artifact_kind {DETACHED_BUNDLE_ARTIFACT_KIND}, got {}",
            artifact.artifact_kind
        ));
    }
    Ok(())
}
