#![allow(dead_code)]

use std::path::PathBuf;

use mir_runtime::current_l2::{CurrentL2SourceSampleRunReport, run_current_l2_source_sample};
use mir_semantics::{
    EventKind, FixtureHostPlan, StaticGateVerdict, load_bundle_from_fixture_path,
    load_fixture_from_path, run_bundle, static_gate_detailed,
};

#[path = "../../../mir-semantics/examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../../../mir-semantics/examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../../../mir-semantics/examples/support/current_l2_lean_theorem_stub_support.rs"]
mod current_l2_lean_theorem_stub_support;
#[path = "../../../mir-semantics/examples/support/current_l2_model_check_carrier_support.rs"]
mod current_l2_model_check_carrier_support;
#[path = "../../../mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs"]
mod current_l2_proof_notebook_review_unit_support;
#[path = "../../../mir-semantics/examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;
use current_l2_formal_hook_support::{
    ToolNeutralFormalContractRow, ToolNeutralFormalEvidenceRef, ToolNeutralFormalHookArtifact,
    build_formal_hook_from_detached_bundle_artifact, build_formal_hook_from_static_gate_artifact,
};
use current_l2_lean_theorem_stub_support::{
    LeanTheoremStubArtifact, build_lean_theorem_stub_artifacts,
};
use current_l2_model_check_carrier_support::{
    ModelCheckConcreteCarrierArtifact, build_model_check_concrete_carrier_artifacts,
};
use current_l2_proof_notebook_review_unit_support::{
    ProofNotebookReviewUnitArtifact, build_proof_notebook_review_unit_artifacts,
};
use current_l2_static_gate_support::build_detached_static_gate_artifact;

const FORMAL_HOOK_SCHEMA_VERSION: &str = "draft-current-l2-formal-hook-v0";
const FORMAL_HOOK_ARTIFACT_KIND: &str = "current-l2-tool-neutral-formal-hook";

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

#[derive(Debug)]
pub struct CurrentL2SourceSamplePreviewArtifactRoute {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub formal_hook_status: CurrentL2EmittedArtifactRouteStatus,
    pub formal_hook_guard_reason: Option<String>,
    pub formal_hook_artifact: Option<ToolNeutralFormalHookArtifact>,
    pub proof_notebook_review_units: Vec<ProofNotebookReviewUnitArtifact>,
    pub model_check_concrete_carriers: Vec<ModelCheckConcreteCarrierArtifact>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckProjectionPrefloor {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub projection_status: CurrentL2EmittedArtifactRouteStatus,
    pub projection_guard_reason: Option<String>,
    pub projection_subject_kind: Option<String>,
    pub projection_subject_ref: Option<String>,
    pub principal_machine_carrier_refs: Vec<String>,
    pub small_cluster_projection_refs: Vec<String>,
    pub property_language_seam_refs: Vec<String>,
    pub tool_binding_seam_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub excluded_family_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremDischargePrefloor {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub discharge_status: CurrentL2EmittedArtifactRouteStatus,
    pub discharge_guard_reason: Option<String>,
    pub discharge_subject_kind: Option<String>,
    pub discharge_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub discharge_entry_reserve_refs: Vec<String>,
    pub transport_seam_refs: Vec<String>,
    pub public_contract_seam_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremFirstPilotActualization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub pilot_status: CurrentL2EmittedArtifactRouteStatus,
    pub pilot_guard_reason: Option<String>,
    pub pilot_subject_kind: Option<String>,
    pub pilot_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub symbolic_evidence_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremDischargeActualFormatProbe {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub probe_status: CurrentL2EmittedArtifactRouteStatus,
    pub probe_guard_reason: Option<String>,
    pub probe_subject_kind: Option<String>,
    pub probe_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub discharge_entry_reserve_refs: Vec<String>,
    pub symbolic_evidence_refs: Vec<String>,
    pub transport_preview_refs: Vec<String>,
    pub public_contract_preview_refs: Vec<String>,
    pub consumer_boundary_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremDischargePublicContractThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub threshold_subject_kind: Option<String>,
    pub threshold_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub discharge_entry_reserve_refs: Vec<String>,
    pub symbolic_evidence_refs: Vec<String>,
    pub transport_preview_refs: Vec<String>,
    pub public_contract_preview_refs: Vec<String>,
    pub consumer_boundary_refs: Vec<String>,
    pub binding_preflight_manifest_refs: Vec<String>,
    pub adapter_boundary_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremContractShapeThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub threshold_subject_kind: Option<String>,
    pub threshold_subject_ref: Option<String>,
    pub transport_shape_refs: Vec<String>,
    pub public_contract_shape_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub contrast_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub transport_candidate_refs: Vec<String>,
    pub public_contract_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub transport_route_refs: Vec<String>,
    pub notebook_contract_route_refs: Vec<String>,
    pub external_binding_reserve_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremResultObjectPreviewActualization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub result_object_route_refs: Vec<String>,
    pub notebook_payload_preview_refs: Vec<String>,
    pub proof_object_schema_reserve_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub proof_object_schema_candidate_refs: Vec<String>,
    pub prover_brand_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub result_object_candidate_refs: Vec<String>,
    pub payload_public_contract_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremResultObjectActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub result_object_route_refs: Vec<String>,
    pub payload_preview_keep_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub result_object_route_refs: Vec<String>,
    pub payload_preview_keep_refs: Vec<String>,
    pub proof_object_schema_candidate_refs: Vec<String>,
    pub prover_brand_candidate_refs: Vec<String>,
    pub final_public_contract_reopen_sequence_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremPublicSeamCompression {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub compression_status: CurrentL2EmittedArtifactRouteStatus,
    pub compression_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub result_object_route_refs: Vec<String>,
    pub payload_preview_keep_refs: Vec<String>,
    pub proof_object_schema_candidate_refs: Vec<String>,
    pub prover_brand_candidate_refs: Vec<String>,
    pub lean_stub_alignment_refs: Vec<String>,
    pub public_seam_residual_refs: Vec<String>,
    pub environment_stop_line_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckPublicSeamCompression {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub compression_status: CurrentL2EmittedArtifactRouteStatus,
    pub compression_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub checker_artifact_route_refs: Vec<String>,
    pub migration_candidate_keep_refs: Vec<String>,
    pub verifier_handoff_candidate_refs: Vec<String>,
    pub tool_brand_candidate_refs: Vec<String>,
    pub property_language_probe_refs: Vec<String>,
    pub tool_seam_probe_refs: Vec<String>,
    pub checker_boundary_probe_refs: Vec<String>,
    pub public_seam_residual_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub property_route_refs: Vec<String>,
    pub checker_contract_route_refs: Vec<String>,
    pub tool_binding_reserve_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub excluded_family_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub checker_artifact_preview_refs: Vec<String>,
    pub verifier_handoff_reserve_refs: Vec<String>,
    pub tool_binding_reserve_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub verifier_handoff_candidate_refs: Vec<String>,
    pub tool_brand_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub public_checker_artifact_candidate_refs: Vec<String>,
    pub checker_migration_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub checker_artifact_route_refs: Vec<String>,
    pub migration_candidate_keep_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub actualization_subject_kind: Option<String>,
    pub actualization_subject_ref: Option<String>,
    pub checker_artifact_route_refs: Vec<String>,
    pub migration_candidate_keep_refs: Vec<String>,
    pub verifier_handoff_candidate_refs: Vec<String>,
    pub tool_brand_candidate_refs: Vec<String>,
    pub final_public_contract_reopen_sequence_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub slice_status: CurrentL2EmittedArtifactRouteStatus,
    pub slice_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub relation_refs: Vec<String>,
    pub authority_handoff_refs: Vec<String>,
    pub runtime_evidence_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub contrast_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub strengthening_status: CurrentL2EmittedArtifactRouteStatus,
    pub strengthening_guard_reason: Option<String>,
    pub fairness_claim_refs: Vec<String>,
    pub witness_core_refs: Vec<String>,
    pub witness_binding_refs: Vec<String>,
    pub runtime_evidence_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub contrast_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleDelegatedRngServicePracticalActualization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub practical_status: CurrentL2EmittedArtifactRouteStatus,
    pub practical_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub provider_boundary_refs: Vec<String>,
    pub optional_attachment_refs: Vec<String>,
    pub runtime_evidence_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub contrast_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub witness_attachment_refs: Vec<String>,
    pub provider_attachment_refs: Vec<String>,
    pub emitted_artifact_reserve_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub contrast_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub witness_route_refs: Vec<String>,
    pub provider_route_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleMinimalCompanionSurface {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub surface_status: CurrentL2EmittedArtifactRouteStatus,
    pub surface_guard_reason: Option<String>,
    pub companion_lines: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleStageBlockSurface {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub surface_status: CurrentL2EmittedArtifactRouteStatus,
    pub surface_guard_reason: Option<String>,
    pub stage_lines: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub surface_status: CurrentL2EmittedArtifactRouteStatus,
    pub surface_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub serial_scope_lines: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub principal_surface_lines: Vec<String>,
    pub secondary_surface_lines: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub contrast_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub principal_surface_lines: Vec<String>,
    pub secondary_surface_lines: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub coupled_status: CurrentL2EmittedArtifactRouteStatus,
    pub coupled_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub witness_contract_candidate_refs: Vec<String>,
    pub provider_contract_candidate_refs: Vec<String>,
    pub emitted_contract_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub alignment_status: CurrentL2EmittedArtifactRouteStatus,
    pub alignment_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub route_pair_refs: Vec<String>,
    pub emitted_contract_pair_refs: Vec<String>,
    pub matched_pair_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub coupled_status: CurrentL2EmittedArtifactRouteStatus,
    pub coupled_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub witness_schema_candidate_refs: Vec<String>,
    pub provider_receipt_candidate_refs: Vec<String>,
    pub combined_public_contract_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderRouteActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub witness_route_actual_refs: Vec<String>,
    pub provider_route_actual_refs: Vec<String>,
    pub schema_candidate_keep_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub witness_schema_route_refs: Vec<String>,
    pub provider_receipt_route_refs: Vec<String>,
    pub combined_public_contract_keep_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub witness_schema_route_refs: Vec<String>,
    pub provider_receipt_route_refs: Vec<String>,
    pub combined_public_contract_keep_refs: Vec<String>,
    pub final_public_contract_reopen_sequence_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub coupled_status: CurrentL2EmittedArtifactRouteStatus,
    pub coupled_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub source_wording_candidate_refs: Vec<String>,
    pub emitted_artifact_schema_candidate_refs: Vec<String>,
    pub coupled_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub actualization_status: CurrentL2EmittedArtifactRouteStatus,
    pub actualization_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub source_wording_route_refs: Vec<String>,
    pub emitted_artifact_candidate_keep_refs: Vec<String>,
    pub actual_adoption_default_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleOrderHandoffWitnessProviderPublicSeamCompression {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub compression_status: CurrentL2EmittedArtifactRouteStatus,
    pub compression_guard_reason: Option<String>,
    pub profile_axis_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub source_wording_route_refs: Vec<String>,
    pub emitted_artifact_candidate_keep_refs: Vec<String>,
    pub serial_scope_lines: Vec<String>,
    pub witness_schema_route_refs: Vec<String>,
    pub provider_receipt_route_refs: Vec<String>,
    pub combined_public_contract_keep_refs: Vec<String>,
    pub trace_alignment_pair_refs: Vec<String>,
    pub public_seam_residual_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremProverBindingPreflight {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub preflight_status: CurrentL2EmittedArtifactRouteStatus,
    pub preflight_guard_reason: Option<String>,
    pub preflight_subject_kind: Option<String>,
    pub preflight_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub symbolic_evidence_refs: Vec<String>,
    pub binding_preflight_manifest_refs: Vec<String>,
    pub adapter_boundary_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremLeanStubPilotActualization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub pilot_status: CurrentL2EmittedArtifactRouteStatus,
    pub pilot_guard_reason: Option<String>,
    pub pilot_subject_kind: Option<String>,
    pub pilot_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub binding_preflight_manifest_refs: Vec<String>,
    pub lean_stub_artifacts: Vec<LeanTheoremStubArtifact>,
    pub pilot_binding_refs: Vec<String>,
    pub code_anchor_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub alignment_status: CurrentL2EmittedArtifactRouteStatus,
    pub alignment_guard_reason: Option<String>,
    pub alignment_subject_kind: Option<String>,
    pub alignment_subject_ref: Option<String>,
    pub principal_review_unit_refs: Vec<String>,
    pub review_unit_pair_refs: Vec<String>,
    pub lean_stub_pair_refs: Vec<String>,
    pub matched_pair_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckSecondLineConcretization {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub concretization_status: CurrentL2EmittedArtifactRouteStatus,
    pub concretization_guard_reason: Option<String>,
    pub concretization_subject_kind: Option<String>,
    pub concretization_subject_ref: Option<String>,
    pub principal_machine_carrier_refs: Vec<String>,
    pub row_local_property_preview_refs: Vec<String>,
    pub secondary_projection_refs: Vec<String>,
    pub request_preflight_refs: Vec<String>,
    pub public_checker_reserve_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub excluded_family_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckPropertyToolSeamProbe {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub probe_status: CurrentL2EmittedArtifactRouteStatus,
    pub probe_guard_reason: Option<String>,
    pub probe_subject_kind: Option<String>,
    pub probe_subject_ref: Option<String>,
    pub principal_machine_carrier_refs: Vec<String>,
    pub row_local_property_preview_refs: Vec<String>,
    pub secondary_projection_refs: Vec<String>,
    pub property_language_probe_refs: Vec<String>,
    pub tool_seam_probe_refs: Vec<String>,
    pub checker_boundary_probe_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub excluded_family_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
}

#[derive(Debug)]
pub struct CurrentL2SourceSampleModelCheckPropertyToolThreshold {
    pub source_report: CurrentL2SourceSampleRunReport,
    pub threshold_status: CurrentL2EmittedArtifactRouteStatus,
    pub threshold_guard_reason: Option<String>,
    pub threshold_subject_kind: Option<String>,
    pub threshold_subject_ref: Option<String>,
    pub principal_machine_carrier_refs: Vec<String>,
    pub row_local_property_preview_refs: Vec<String>,
    pub secondary_projection_refs: Vec<String>,
    pub property_language_probe_refs: Vec<String>,
    pub tool_seam_probe_refs: Vec<String>,
    pub checker_boundary_probe_refs: Vec<String>,
    pub request_preflight_refs: Vec<String>,
    pub public_checker_reserve_refs: Vec<String>,
    pub threshold_default_refs: Vec<String>,
    pub repo_local_emitted_artifact_refs: Vec<String>,
    pub compare_floor_refs: Vec<String>,
    pub excluded_family_refs: Vec<String>,
    pub guard_refs: Vec<String>,
    pub kept_later_refs: Vec<String>,
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

pub fn build_current_l2_source_sample_preview_artifact_route(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSamplePreviewArtifactRoute, String> {
    let source_report = run_current_l2_source_sample(sample_argument, host_plan)
        .map_err(|error| error.to_string())?;

    match build_preview_formal_hook_for_source_sample(&source_report) {
        Ok(formal_hook_artifact) => {
            let proof_notebook_review_units =
                build_proof_notebook_review_unit_artifacts(&formal_hook_artifact)?;
            let model_check_concrete_carriers =
                build_model_check_concrete_carrier_artifacts(&formal_hook_artifact)?;

            Ok(CurrentL2SourceSamplePreviewArtifactRoute {
                source_report,
                formal_hook_status: CurrentL2EmittedArtifactRouteStatus::Reached,
                formal_hook_guard_reason: None,
                formal_hook_artifact: Some(formal_hook_artifact),
                proof_notebook_review_units,
                model_check_concrete_carriers,
            })
        }
        Err(error) => Ok(CurrentL2SourceSamplePreviewArtifactRoute {
            source_report,
            formal_hook_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            formal_hook_guard_reason: Some(error),
            formal_hook_artifact: None,
            proof_notebook_review_units: Vec::new(),
            model_check_concrete_carriers: Vec::new(),
        }),
    }
}

pub fn build_current_l2_source_sample_model_check_projection_prefloor(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckProjectionPrefloor, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan)?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact,
        proof_notebook_review_units: _,
        model_check_concrete_carriers,
    } = preview_route;

    let projection_subject_kind = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_kind.clone());
    let projection_subject_ref = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_ref.clone());

    let principal_machine_carrier_refs = model_check_concrete_carriers
        .iter()
        .map(|carrier| {
            format!(
                "model_check_concrete_carrier:{}:{}",
                carrier.subject_ref, carrier.case.obligation_kind
            )
        })
        .collect::<Vec<_>>();
    let small_cluster_projection_refs = if let Some(artifact) = formal_hook_artifact.as_ref() {
        vec![small_cluster_projection_ref(
            &artifact.subject_kind,
            &artifact.subject_ref,
        )?]
    } else {
        Vec::new()
    };
    let guard_refs = model_check_projection_guard_refs(
        formal_hook_status,
        projection_subject_kind.as_deref(),
        formal_hook_guard_reason.as_deref(),
    );

    Ok(CurrentL2SourceSampleModelCheckProjectionPrefloor {
        source_report,
        projection_status: formal_hook_status,
        projection_guard_reason: formal_hook_guard_reason,
        projection_subject_kind,
        projection_subject_ref,
        principal_machine_carrier_refs,
        small_cluster_projection_refs,
        property_language_seam_refs: vec![
            "property_language_seam:current_l2.model_check.small_cluster.semantic".to_string(),
        ],
        tool_binding_seam_refs: vec![
            "tool_binding_seam:current_l2.model_check.reserve".to_string(),
        ],
        guard_refs,
        excluded_family_refs: vec![
            "excluded_family:typed_reserve_cluster".to_string(),
            "excluded_family:theorem_discharge_transport".to_string(),
            "excluded_family:room_protocol_projection".to_string(),
        ],
        kept_later_refs: vec![
            "kept_later:fairness_replay_operational_profile".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
        ],
    })
}

pub fn build_current_l2_source_sample_theorem_discharge_prefloor(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremDischargePrefloor, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan)?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact,
        proof_notebook_review_units,
        model_check_concrete_carriers: _,
    } = preview_route;

    let discharge_subject_kind = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_kind.clone());
    let discharge_subject_ref = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_ref.clone());

    let principal_review_unit_refs = proof_notebook_review_units
        .iter()
        .map(|unit| {
            format!(
                "proof_notebook_review_unit:{}:{}",
                unit.subject_ref, unit.row.obligation_kind
            )
        })
        .collect::<Vec<_>>();
    let discharge_entry_reserve_refs = proof_notebook_review_units
        .iter()
        .map(|unit| {
            format!(
                "discharge_entry_reserve:{}:{}",
                unit.subject_ref, unit.row.obligation_kind
            )
        })
        .collect::<Vec<_>>();
    let guard_refs =
        theorem_discharge_guard_refs(formal_hook_status, formal_hook_guard_reason.as_deref());

    Ok(CurrentL2SourceSampleTheoremDischargePrefloor {
        source_report,
        discharge_status: formal_hook_status,
        discharge_guard_reason: formal_hook_guard_reason,
        discharge_subject_kind,
        discharge_subject_ref,
        principal_review_unit_refs,
        discharge_entry_reserve_refs,
        transport_seam_refs: vec![
            "transport_seam:current_l2.theorem.discharge.reserve".to_string(),
        ],
        public_contract_seam_refs: vec![
            "public_contract_seam:current_l2.theorem.notebook.reserve".to_string(),
        ],
        guard_refs,
        kept_later_refs: vec![
            "kept_later:concrete_theorem_prover_binding".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
        ],
    })
}

pub fn build_current_l2_source_sample_theorem_first_pilot_actualization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremFirstPilotActualization, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan)?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact,
        proof_notebook_review_units,
        model_check_concrete_carriers: _,
    } = preview_route;

    let pilot_subject_kind = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_kind.clone());
    let pilot_subject_ref = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_ref.clone());

    let principal_review_unit_refs = proof_notebook_review_units
        .iter()
        .map(|unit| {
            format!(
                "proof_notebook_review_unit:{}:{}",
                unit.subject_ref, unit.row.obligation_kind
            )
        })
        .collect::<Vec<_>>();
    let symbolic_evidence_refs = theorem_first_symbolic_evidence_refs(&proof_notebook_review_units);
    let repo_local_emitted_artifact_refs = proof_notebook_review_units
        .iter()
        .map(|unit| {
            format!(
                "repo_local_emitted_artifact:proof_notebook_review_unit:{}:{}",
                unit.subject_ref, unit.row.obligation_kind
            )
        })
        .collect::<Vec<_>>();
    let compare_floor_refs = theorem_first_compare_floor_refs(formal_hook_status);
    let guard_refs = theorem_first_pilot_guard_refs(formal_hook_status);

    Ok(CurrentL2SourceSampleTheoremFirstPilotActualization {
        source_report,
        pilot_status: formal_hook_status,
        pilot_guard_reason: formal_hook_guard_reason,
        pilot_subject_kind,
        pilot_subject_ref,
        principal_review_unit_refs,
        symbolic_evidence_refs,
        repo_local_emitted_artifact_refs,
        compare_floor_refs,
        guard_refs,
        kept_later_refs: vec![
            "kept_later:theorem_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:concrete_theorem_prover_binding".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
        ],
    })
}

pub fn build_current_l2_source_sample_theorem_discharge_actual_format_probe(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremDischargeActualFormatProbe, String> {
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_argument,
        host_plan.clone(),
    )?;
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremDischargePrefloor {
        source_report,
        discharge_status,
        discharge_guard_reason,
        discharge_subject_kind,
        discharge_subject_ref,
        principal_review_unit_refs,
        discharge_entry_reserve_refs,
        transport_seam_refs: _,
        public_contract_seam_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = prefloor;

    let CurrentL2SourceSampleTheoremFirstPilotActualization {
        source_report: _,
        pilot_status,
        pilot_guard_reason,
        pilot_subject_kind: _,
        pilot_subject_ref: _,
        principal_review_unit_refs: _,
        symbolic_evidence_refs,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = pilot;

    let probe_status = if discharge_status == CurrentL2EmittedArtifactRouteStatus::Reached
        && pilot_status == CurrentL2EmittedArtifactRouteStatus::Reached
    {
        CurrentL2EmittedArtifactRouteStatus::Reached
    } else {
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    };

    if probe_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = discharge_guard_reason
            .or(pilot_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem discharge prefloor or theorem-first pilot was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem discharge actual-format probe only actualizes reached theorem discharge routes: {guard_detail}"
        );
        return Ok(CurrentL2SourceSampleTheoremDischargeActualFormatProbe {
            source_report,
            probe_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            probe_guard_reason: Some(guard_reason),
            probe_subject_kind: discharge_subject_kind,
            probe_subject_ref: discharge_subject_ref,
            principal_review_unit_refs,
            discharge_entry_reserve_refs,
            symbolic_evidence_refs,
            transport_preview_refs: Vec::new(),
            public_contract_preview_refs: Vec::new(),
            consumer_boundary_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_discharge_actual_format_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            guard_refs: theorem_discharge_actual_format_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: theorem_discharge_actual_format_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleTheoremDischargeActualFormatProbe {
        source_report,
        probe_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        probe_guard_reason: None,
        probe_subject_kind: discharge_subject_kind,
        probe_subject_ref: discharge_subject_ref.clone(),
        principal_review_unit_refs: principal_review_unit_refs.clone(),
        discharge_entry_reserve_refs: discharge_entry_reserve_refs.clone(),
        symbolic_evidence_refs,
        transport_preview_refs: theorem_discharge_actual_format_transport_preview_refs(
            &discharge_entry_reserve_refs,
        )?,
        public_contract_preview_refs: theorem_discharge_actual_format_public_contract_preview_refs(
            discharge_subject_ref.as_deref(),
        ),
        consumer_boundary_refs: theorem_discharge_actual_format_consumer_boundary_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        repo_local_emitted_artifact_refs,
        compare_floor_refs: theorem_discharge_actual_format_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: theorem_discharge_actual_format_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: theorem_discharge_actual_format_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_prover_binding_preflight(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremProverBindingPreflight, String> {
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremFirstPilotActualization {
        source_report,
        pilot_status,
        pilot_guard_reason,
        pilot_subject_kind,
        pilot_subject_ref,
        principal_review_unit_refs,
        symbolic_evidence_refs,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = pilot;

    if pilot_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = pilot_guard_reason.unwrap_or_else(|| {
            format!(
                "theorem-first pilot route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current theorem-prover binding preflight only actualizes reached theorem-first pilot routes: {guard_detail}"
        );
        return Ok(CurrentL2SourceSampleTheoremProverBindingPreflight {
            source_report,
            preflight_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            preflight_guard_reason: Some(guard_reason),
            preflight_subject_kind: pilot_subject_kind,
            preflight_subject_ref: pilot_subject_ref,
            principal_review_unit_refs,
            symbolic_evidence_refs,
            binding_preflight_manifest_refs: Vec::new(),
            adapter_boundary_refs: Vec::new(),
            compare_floor_refs: theorem_binding_preflight_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            guard_refs: theorem_binding_preflight_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: theorem_binding_preflight_kept_later_refs(),
        });
    }

    let binding_preflight_manifest_refs =
        theorem_binding_preflight_manifest_refs(&principal_review_unit_refs)?;

    Ok(CurrentL2SourceSampleTheoremProverBindingPreflight {
        source_report,
        preflight_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        preflight_guard_reason: None,
        preflight_subject_kind: pilot_subject_kind,
        preflight_subject_ref: pilot_subject_ref,
        principal_review_unit_refs,
        symbolic_evidence_refs,
        binding_preflight_manifest_refs,
        adapter_boundary_refs: theorem_binding_adapter_boundary_refs(),
        compare_floor_refs: theorem_binding_preflight_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: theorem_binding_preflight_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: theorem_binding_preflight_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremLeanStubPilotActualization, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan.clone())?;
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report: _,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact: _,
        proof_notebook_review_units,
        model_check_concrete_carriers: _,
    } = preview_route;

    let CurrentL2SourceSampleTheoremProverBindingPreflight {
        source_report,
        preflight_status,
        preflight_guard_reason,
        preflight_subject_kind,
        preflight_subject_ref,
        principal_review_unit_refs,
        symbolic_evidence_refs: _,
        binding_preflight_manifest_refs,
        adapter_boundary_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = preflight;

    if preflight_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || formal_hook_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = preflight_guard_reason
            .or(formal_hook_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem binding preflight or preview route was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem Lean stub pilot only actualizes reached theorem-binding preflight routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleTheoremLeanStubPilotActualization {
            source_report,
            pilot_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            pilot_guard_reason: Some(guard_reason),
            pilot_subject_kind: preflight_subject_kind,
            pilot_subject_ref: preflight_subject_ref,
            principal_review_unit_refs,
            binding_preflight_manifest_refs,
            lean_stub_artifacts: Vec::new(),
            pilot_binding_refs: Vec::new(),
            code_anchor_refs: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            compare_floor_refs: theorem_lean_stub_pilot_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            guard_refs: theorem_lean_stub_pilot_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: theorem_lean_stub_pilot_kept_later_refs(),
        });
    }

    let lean_stub_artifacts = build_lean_theorem_stub_artifacts(&proof_notebook_review_units)?;

    Ok(CurrentL2SourceSampleTheoremLeanStubPilotActualization {
        source_report,
        pilot_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        pilot_guard_reason: None,
        pilot_subject_kind: preflight_subject_kind,
        pilot_subject_ref: preflight_subject_ref.clone(),
        principal_review_unit_refs,
        binding_preflight_manifest_refs,
        lean_stub_artifacts: lean_stub_artifacts.clone(),
        pilot_binding_refs: theorem_lean_stub_pilot_binding_refs(preflight_subject_ref.as_deref()),
        code_anchor_refs: theorem_lean_stub_pilot_code_anchor_refs(),
        repo_local_emitted_artifact_refs: theorem_lean_stub_pilot_repo_local_emitted_artifact_refs(
            &lean_stub_artifacts,
        ),
        compare_floor_refs: theorem_lean_stub_pilot_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: theorem_lean_stub_pilot_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: theorem_lean_stub_pilot_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan.clone())?;
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report: _,
        formal_hook_status: _,
        formal_hook_guard_reason: _,
        formal_hook_artifact: _,
        proof_notebook_review_units,
        model_check_concrete_carriers: _,
    } = preview_route;

    let CurrentL2SourceSampleTheoremLeanStubPilotActualization {
        source_report,
        pilot_status,
        pilot_guard_reason,
        pilot_subject_kind,
        pilot_subject_ref,
        principal_review_unit_refs,
        binding_preflight_manifest_refs: _,
        lean_stub_artifacts,
        pilot_binding_refs: _,
        code_anchor_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = pilot;

    if pilot_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = pilot_guard_reason.unwrap_or_else(|| {
            format!(
                "theorem Lean stub pilot route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current theorem Lean stub trace alignment bridge only actualizes reached theorem Lean stub pilot routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge {
            source_report,
            alignment_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            alignment_guard_reason: Some(guard_reason),
            alignment_subject_kind: pilot_subject_kind,
            alignment_subject_ref: pilot_subject_ref,
            principal_review_unit_refs,
            review_unit_pair_refs: Vec::new(),
            lean_stub_pair_refs: Vec::new(),
            matched_pair_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_lean_stub_trace_alignment_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            guard_refs: theorem_lean_stub_trace_alignment_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: theorem_lean_stub_trace_alignment_kept_later_refs(),
        });
    }

    let review_unit_pair_refs =
        theorem_lean_stub_trace_alignment_review_unit_pair_refs(&proof_notebook_review_units);
    let lean_stub_pair_refs =
        theorem_lean_stub_trace_alignment_lean_stub_pair_refs(&lean_stub_artifacts);
    let matched_pair_refs = theorem_lean_stub_trace_alignment_matched_pair_refs(
        &review_unit_pair_refs,
        &lean_stub_pair_refs,
    )?;

    Ok(CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge {
        source_report,
        alignment_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        alignment_guard_reason: None,
        alignment_subject_kind: pilot_subject_kind,
        alignment_subject_ref: pilot_subject_ref,
        principal_review_unit_refs,
        review_unit_pair_refs,
        lean_stub_pair_refs,
        matched_pair_refs,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: theorem_lean_stub_trace_alignment_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: theorem_lean_stub_trace_alignment_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: theorem_lean_stub_trace_alignment_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremDischargePublicContractThreshold, String> {
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_argument,
        host_plan.clone(),
    )?;
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremDischargeActualFormatProbe {
        source_report,
        probe_status,
        probe_guard_reason,
        probe_subject_kind,
        probe_subject_ref,
        principal_review_unit_refs,
        discharge_entry_reserve_refs,
        symbolic_evidence_refs,
        transport_preview_refs,
        public_contract_preview_refs,
        consumer_boundary_refs,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = probe;

    let CurrentL2SourceSampleTheoremProverBindingPreflight {
        source_report: _,
        preflight_status,
        preflight_guard_reason,
        preflight_subject_kind: _,
        preflight_subject_ref: _,
        principal_review_unit_refs: _,
        symbolic_evidence_refs: _,
        binding_preflight_manifest_refs,
        adapter_boundary_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = preflight;

    let threshold_status = if probe_status == CurrentL2EmittedArtifactRouteStatus::Reached
        && preflight_status == CurrentL2EmittedArtifactRouteStatus::Reached
    {
        CurrentL2EmittedArtifactRouteStatus::Reached
    } else {
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    };

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = probe_guard_reason
            .or(preflight_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem discharge actual-format probe or theorem binding preflight was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem discharge/public-contract threshold only actualizes reached theorem mixed-gate routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremDischargePublicContractThreshold {
                source_report,
                threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                threshold_guard_reason: Some(guard_reason),
                threshold_subject_kind: probe_subject_kind,
                threshold_subject_ref: probe_subject_ref,
                principal_review_unit_refs,
                discharge_entry_reserve_refs,
                symbolic_evidence_refs,
                transport_preview_refs: Vec::new(),
                public_contract_preview_refs: Vec::new(),
                consumer_boundary_refs: Vec::new(),
                binding_preflight_manifest_refs: Vec::new(),
                adapter_boundary_refs: Vec::new(),
                threshold_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs: theorem_contract_threshold_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                guard_refs: theorem_contract_threshold_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: theorem_contract_threshold_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleTheoremDischargePublicContractThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            threshold_guard_reason: None,
            threshold_subject_kind: probe_subject_kind,
            threshold_subject_ref: probe_subject_ref,
            principal_review_unit_refs,
            discharge_entry_reserve_refs,
            symbolic_evidence_refs,
            transport_preview_refs,
            public_contract_preview_refs,
            consumer_boundary_refs,
            binding_preflight_manifest_refs,
            adapter_boundary_refs,
            threshold_default_refs: theorem_contract_threshold_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_contract_threshold_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            guard_refs: theorem_contract_threshold_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: theorem_contract_threshold_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_theorem_contract_shape_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremContractShapeThreshold, String> {
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremDischargePublicContractThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        threshold_subject_kind,
        threshold_subject_ref,
        principal_review_unit_refs: _,
        discharge_entry_reserve_refs: _,
        symbolic_evidence_refs: _,
        transport_preview_refs: _,
        public_contract_preview_refs: _,
        consumer_boundary_refs: _,
        binding_preflight_manifest_refs: _,
        adapter_boundary_refs: _,
        threshold_default_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = threshold_guard_reason.unwrap_or_else(|| {
            format!(
                "theorem threshold route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "theorem contract shape threshold only actualizes reached theorem threshold routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleTheoremContractShapeThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            threshold_guard_reason: Some(guard_reason),
            threshold_subject_kind,
            threshold_subject_ref,
            transport_shape_refs: Vec::new(),
            public_contract_shape_refs: Vec::new(),
            threshold_default_refs: Vec::new(),
            compare_floor_refs: theorem_contract_shape_threshold_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            contrast_refs: theorem_contract_shape_threshold_contrast_refs(),
            guard_refs: theorem_contract_shape_threshold_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: theorem_contract_shape_threshold_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleTheoremContractShapeThreshold {
        source_report,
        threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        threshold_guard_reason: None,
        threshold_subject_kind,
        threshold_subject_ref: threshold_subject_ref.clone(),
        transport_shape_refs: theorem_contract_shape_transport_refs(
            threshold_subject_ref.as_deref(),
        ),
        public_contract_shape_refs: theorem_contract_shape_public_contract_refs(
            threshold_subject_ref.as_deref(),
        ),
        threshold_default_refs: theorem_contract_shape_threshold_default_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        compare_floor_refs: theorem_contract_shape_threshold_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        contrast_refs: theorem_contract_shape_threshold_contrast_refs(),
        guard_refs: theorem_contract_shape_threshold_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: theorem_contract_shape_threshold_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate, String> {
    let threshold = build_current_l2_source_sample_theorem_contract_shape_threshold(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremContractShapeThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        threshold_subject_kind,
        threshold_subject_ref,
        transport_shape_refs: _,
        public_contract_shape_refs: _,
        threshold_default_refs: _,
        compare_floor_refs: _,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = threshold_guard_reason.unwrap_or_else(|| {
            format!(
                "theorem shape threshold route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "theorem transport/public-contract coupled later gate only actualizes reached theorem shape-threshold routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind: threshold_subject_kind,
                actualization_subject_ref: threshold_subject_ref,
                transport_candidate_refs: Vec::new(),
                public_contract_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                compare_floor_refs:
                    theorem_transport_contract_coupled_later_gate_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                guard_refs: theorem_transport_contract_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: theorem_transport_contract_coupled_later_gate_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind: threshold_subject_kind,
            actualization_subject_ref: threshold_subject_ref.clone(),
            transport_candidate_refs: theorem_transport_contract_coupled_later_gate_transport_refs(
                threshold_subject_ref.as_deref(),
            ),
            public_contract_candidate_refs:
                theorem_transport_contract_coupled_later_gate_public_contract_refs(
                    threshold_subject_ref.as_deref(),
                ),
            coupled_default_refs: theorem_transport_contract_coupled_later_gate_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            compare_floor_refs: theorem_transport_contract_coupled_later_gate_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            guard_refs: theorem_transport_contract_coupled_later_gate_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: theorem_transport_contract_coupled_later_gate_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption, String> {
    let coupled_gate =
        build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
            sample_argument,
            host_plan.clone(),
        )?;
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        transport_candidate_refs: _,
        public_contract_candidate_refs: _,
        coupled_default_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = coupled_gate;

    let CurrentL2SourceSampleTheoremDischargePublicContractThreshold {
        source_report: _,
        threshold_status: _,
        threshold_guard_reason: _,
        threshold_subject_kind: _,
        threshold_subject_ref: _,
        principal_review_unit_refs: _,
        discharge_entry_reserve_refs: _,
        symbolic_evidence_refs: _,
        transport_preview_refs: _,
        public_contract_preview_refs: _,
        consumer_boundary_refs: _,
        binding_preflight_manifest_refs: _,
        adapter_boundary_refs: _,
        threshold_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = actualization_guard_reason.unwrap_or_else(|| {
            format!(
                "theorem coupled later gate was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current theorem review-unit transport actual adoption only actualizes reached theorem coupled-later routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                transport_route_refs: Vec::new(),
                notebook_contract_route_refs: Vec::new(),
                external_binding_reserve_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    theorem_review_unit_transport_actual_adoption_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                guard_refs: theorem_review_unit_transport_actual_adoption_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: theorem_review_unit_transport_actual_adoption_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            transport_route_refs: theorem_review_unit_transport_actual_adoption_transport_refs(
                actualization_subject_ref.as_deref(),
            ),
            notebook_contract_route_refs:
                theorem_review_unit_transport_actual_adoption_notebook_contract_refs(
                    actualization_subject_ref.as_deref(),
                ),
            external_binding_reserve_refs:
                theorem_review_unit_transport_actual_adoption_external_binding_reserve_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            actual_adoption_default_refs:
                theorem_review_unit_transport_actual_adoption_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_review_unit_transport_actual_adoption_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            guard_refs: theorem_review_unit_transport_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: theorem_review_unit_transport_actual_adoption_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_theorem_result_object_preview_actualization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremResultObjectPreviewActualization, String> {
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        transport_route_refs: _,
        notebook_contract_route_refs: _,
        external_binding_reserve_refs: _,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = review_unit_actual_adoption;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = actualization_guard_reason.unwrap_or_else(|| {
            format!(
                "theorem review-unit transport actual adoption was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current theorem result-object preview actualization only actualizes reached theorem review-unit adoption routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremResultObjectPreviewActualization {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                result_object_route_refs: Vec::new(),
                notebook_payload_preview_refs: Vec::new(),
                proof_object_schema_reserve_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs: theorem_result_object_preview_actualization_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                guard_refs: theorem_result_object_preview_actualization_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: theorem_result_object_preview_actualization_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleTheoremResultObjectPreviewActualization {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            result_object_route_refs:
                theorem_result_object_preview_actualization_result_object_refs(
                    actualization_subject_ref.as_deref(),
                ),
            notebook_payload_preview_refs:
                theorem_result_object_preview_actualization_payload_preview_refs(
                    actualization_subject_ref.as_deref(),
                ),
            proof_object_schema_reserve_refs:
                theorem_result_object_preview_actualization_proof_object_schema_reserve_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            actual_adoption_default_refs: theorem_result_object_preview_actualization_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_result_object_preview_actualization_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            guard_refs: theorem_result_object_preview_actualization_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: theorem_result_object_preview_actualization_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate, String> {
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            sample_argument,
            host_plan.clone(),
        )?;
    let binding_preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremResultObjectPreviewActualization {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        result_object_route_refs: _,
        notebook_payload_preview_refs: _,
        proof_object_schema_reserve_refs: _,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = result_object_preview;

    let CurrentL2SourceSampleTheoremProverBindingPreflight {
        source_report: _,
        preflight_status,
        preflight_guard_reason,
        preflight_subject_kind: _,
        preflight_subject_ref: _,
        principal_review_unit_refs: _,
        symbolic_evidence_refs: _,
        binding_preflight_manifest_refs: _,
        adapter_boundary_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = binding_preflight;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || preflight_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = actualization_guard_reason
            .or(preflight_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem result-object preview or theorem-binding preflight was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem proof-object schema / prover-brand coupled later gate only actualizes reached theorem result-object preview and theorem-binding preflight routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                proof_object_schema_candidate_refs: Vec::new(),
                prover_brand_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    theorem_proof_object_schema_prover_brand_coupled_later_gate_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                guard_refs: theorem_proof_object_schema_prover_brand_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs:
                    theorem_proof_object_schema_prover_brand_coupled_later_gate_kept_later_refs(),
            },
        );
    }

    Ok(CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate {
        source_report,
        actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        actualization_guard_reason: None,
        actualization_subject_kind,
        actualization_subject_ref: actualization_subject_ref.clone(),
        proof_object_schema_candidate_refs:
            theorem_proof_object_schema_prover_brand_coupled_later_gate_proof_object_schema_candidate_refs(
                actualization_subject_ref.as_deref(),
            ),
        prover_brand_candidate_refs:
            theorem_proof_object_schema_prover_brand_coupled_later_gate_prover_brand_candidate_refs(
                actualization_subject_ref.as_deref(),
            ),
        coupled_default_refs:
            theorem_proof_object_schema_prover_brand_coupled_later_gate_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
        repo_local_emitted_artifact_refs,
        compare_floor_refs:
            theorem_proof_object_schema_prover_brand_coupled_later_gate_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
        guard_refs: theorem_proof_object_schema_prover_brand_coupled_later_gate_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs:
            theorem_proof_object_schema_prover_brand_coupled_later_gate_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate, String> {
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            sample_argument,
            host_plan.clone(),
        )?;
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleTheoremResultObjectPreviewActualization {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        result_object_route_refs: _,
        notebook_payload_preview_refs: _,
        proof_object_schema_reserve_refs: _,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = result_object_preview;

    let CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate {
        source_report: _,
        actualization_status: proof_object_status,
        actualization_guard_reason: proof_object_guard_reason,
        actualization_subject_kind: _,
        actualization_subject_ref: _,
        proof_object_schema_candidate_refs: _,
        prover_brand_candidate_refs: _,
        coupled_default_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: proof_object_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = proof_object_gate;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || proof_object_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = actualization_guard_reason
            .or(proof_object_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem result-object preview or theorem proof-object/prover-brand gate was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem result/payload public-contract coupled later gate only actualizes reached theorem result-object preview and theorem proof-object/prover-brand routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                result_object_candidate_refs: Vec::new(),
                payload_public_contract_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs: vec![
                    "compare_floor:current_l2.theorem_result_payload_public_contract.guard_only"
                        .to_string(),
                ],
                guard_refs: theorem_result_payload_public_contract_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs:
                    theorem_result_payload_public_contract_coupled_later_gate_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            result_object_candidate_refs:
                theorem_result_payload_public_contract_coupled_later_gate_result_object_candidate_refs(
                    actualization_subject_ref.as_deref(),
                ),
            payload_public_contract_candidate_refs:
                theorem_result_payload_public_contract_coupled_later_gate_payload_candidate_refs(
                    actualization_subject_ref.as_deref(),
                ),
            coupled_default_refs:
                theorem_result_payload_public_contract_coupled_later_gate_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs:
                theorem_result_payload_public_contract_coupled_later_gate_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    proof_object_compare_floor_refs,
                ),
            guard_refs:
                theorem_result_payload_public_contract_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            kept_later_refs:
                theorem_result_payload_public_contract_coupled_later_gate_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_theorem_result_object_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremResultObjectActualAdoption, String> {
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let result_payload_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        transport_route_refs: _,
        notebook_contract_route_refs: _,
        external_binding_reserve_refs: _,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = review_unit_actual_adoption;

    let CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate {
        source_report: _,
        actualization_status: result_payload_status,
        actualization_guard_reason: result_payload_guard_reason,
        actualization_subject_kind: _,
        actualization_subject_ref: _,
        result_object_candidate_refs: _,
        payload_public_contract_candidate_refs: _,
        coupled_default_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = result_payload_gate;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || result_payload_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = actualization_guard_reason
            .or(result_payload_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem review-unit actual adoption or theorem result/payload gate was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem result-object actual adoption only actualizes reached theorem review-unit adoption and theorem result/payload routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleTheoremResultObjectActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            actualization_guard_reason: Some(guard_reason),
            actualization_subject_kind,
            actualization_subject_ref,
            result_object_route_refs: Vec::new(),
            payload_preview_keep_refs: Vec::new(),
            actual_adoption_default_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_result_object_actual_adoption_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            guard_refs: theorem_result_object_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: theorem_result_object_actual_adoption_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleTheoremResultObjectActualAdoption {
        source_report,
        actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        actualization_guard_reason: None,
        actualization_subject_kind,
        actualization_subject_ref: actualization_subject_ref.clone(),
        result_object_route_refs: theorem_result_object_actual_adoption_route_refs(
            actualization_subject_ref.as_deref(),
        ),
        payload_preview_keep_refs: theorem_result_object_actual_adoption_payload_preview_keep_refs(
            actualization_subject_ref.as_deref(),
        ),
        actual_adoption_default_refs: theorem_result_object_actual_adoption_default_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        repo_local_emitted_artifact_refs,
        compare_floor_refs: theorem_result_object_actual_adoption_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: theorem_result_object_actual_adoption_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: theorem_result_object_actual_adoption_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold, String> {
    let result_object_actual_adoption =
        build_current_l2_source_sample_theorem_result_object_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleTheoremResultObjectActualAdoption {
        source_report,
        actualization_status: result_object_status,
        actualization_guard_reason: result_object_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        result_object_route_refs,
        payload_preview_keep_refs,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: result_object_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = result_object_actual_adoption;

    let CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate {
        source_report: _,
        actualization_status: proof_object_status,
        actualization_guard_reason: proof_object_guard_reason,
        actualization_subject_kind: _,
        actualization_subject_ref: _,
        proof_object_schema_candidate_refs,
        prover_brand_candidate_refs,
        coupled_default_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = proof_object_gate;

    if result_object_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || proof_object_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = result_object_guard_reason
            .or(proof_object_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem result-object route or theorem proof-object/prover-brand gate was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current theorem final public contract reopen threshold only actualizes reached theorem result-object route and theorem proof-object/prover-brand routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold {
                source_report,
                threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                threshold_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                result_object_route_refs: Vec::new(),
                payload_preview_keep_refs: Vec::new(),
                proof_object_schema_candidate_refs: Vec::new(),
                prover_brand_candidate_refs: Vec::new(),
                final_public_contract_reopen_sequence_refs: Vec::new(),
                threshold_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    theorem_final_public_contract_reopen_threshold_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        result_object_compare_floor_refs,
                    ),
                guard_refs: theorem_final_public_contract_reopen_threshold_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: theorem_final_public_contract_reopen_threshold_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            threshold_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            result_object_route_refs,
            payload_preview_keep_refs,
            proof_object_schema_candidate_refs,
            prover_brand_candidate_refs,
            final_public_contract_reopen_sequence_refs:
                theorem_final_public_contract_reopen_threshold_sequence_refs(
                    actualization_subject_ref.as_deref(),
                ),
            threshold_default_refs: theorem_final_public_contract_reopen_threshold_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: theorem_final_public_contract_reopen_threshold_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
                result_object_compare_floor_refs,
            ),
            guard_refs: theorem_final_public_contract_reopen_threshold_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: theorem_final_public_contract_reopen_threshold_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_theorem_public_seam_compression(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleTheoremPublicSeamCompression, String> {
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_argument,
        host_plan.clone(),
    )?;
    let lean_stub_bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        result_object_route_refs,
        payload_preview_keep_refs,
        proof_object_schema_candidate_refs,
        prover_brand_candidate_refs,
        final_public_contract_reopen_sequence_refs: _,
        threshold_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: threshold_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    let CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge {
        source_report: bridge_source_report,
        alignment_status,
        alignment_guard_reason,
        alignment_subject_kind,
        alignment_subject_ref,
        principal_review_unit_refs: _,
        review_unit_pair_refs: _,
        lean_stub_pair_refs: _,
        matched_pair_refs,
        repo_local_emitted_artifact_refs: bridge_repo_local_emitted_artifact_refs,
        compare_floor_refs: bridge_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = lean_stub_bridge;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || alignment_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = threshold_guard_reason
            .or(alignment_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "theorem final public-contract threshold or Lean-stub bridge was not reached for `{}`",
                    source_report.sample_id
                )
            });

        return Ok(CurrentL2SourceSampleTheoremPublicSeamCompression {
            source_report,
            compression_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            compression_guard_reason: Some(format!(
                "current theorem public seam compression only actualizes reached theorem final-threshold + Lean-stub bridge floors: {guard_detail}"
            )),
            actualization_subject_kind,
            actualization_subject_ref,
            repo_local_emitted_artifact_refs: Vec::new(),
            result_object_route_refs: Vec::new(),
            payload_preview_keep_refs: Vec::new(),
            proof_object_schema_candidate_refs: Vec::new(),
            prover_brand_candidate_refs: Vec::new(),
            lean_stub_alignment_refs: Vec::new(),
            public_seam_residual_refs: Vec::new(),
            environment_stop_line_refs: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.theorem_public_seam_compression.guard_only".to_string(),
            ],
            guard_refs: vec!["guard:theorem_public_seam_compression_not_reached".to_string()],
            kept_later_refs: vec![
                "kept_later:final_public_theorem_result_object".to_string(),
                "kept_later:consumer_shaped_theorem_payload".to_string(),
                "kept_later:concrete_theorem_prover_brand".to_string(),
                "kept_later:proof_object_public_schema".to_string(),
                "kept_later:final_public_verifier_contract".to_string(),
                "kept_later:actual_lean_execution_with_concrete_toolchain".to_string(),
            ],
        });
    }

    if source_report.sample_id != bridge_source_report.sample_id
        || actualization_subject_kind != alignment_subject_kind
        || actualization_subject_ref != alignment_subject_ref
    {
        return Err(format!(
            "theorem public seam compression expected aligned threshold/bridge subjects for `{}` but got threshold {:?}/{:?} and bridge {:?}/{:?}",
            source_report.sample_id,
            actualization_subject_kind,
            actualization_subject_ref,
            alignment_subject_kind,
            alignment_subject_ref
        ));
    }

    let subject_ref = actualization_subject_ref.as_deref().ok_or_else(|| {
        format!(
            "theorem public seam compression expected a subject ref for `{}`",
            source_report.sample_id
        )
    })?;
    let mut merged_repo_local_emitted_artifact_refs = repo_local_emitted_artifact_refs;
    extend_unique_refs(
        &mut merged_repo_local_emitted_artifact_refs,
        bridge_repo_local_emitted_artifact_refs,
    );
    let mut compare_floor_refs = threshold_compare_floor_refs;
    extend_unique_refs(&mut compare_floor_refs, bridge_compare_floor_refs);
    extend_unique_refs(
        &mut compare_floor_refs,
        vec![
            "compare_floor:current_l2.theorem_actual_lean_execution_availability_probe".to_string(),
            "compare_floor:current_l2.theorem_public_seam_compression".to_string(),
        ],
    );

    Ok(CurrentL2SourceSampleTheoremPublicSeamCompression {
        source_report,
        compression_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        compression_guard_reason: None,
        actualization_subject_kind,
        actualization_subject_ref: Some(subject_ref.to_string()),
        repo_local_emitted_artifact_refs: merged_repo_local_emitted_artifact_refs,
        result_object_route_refs,
        payload_preview_keep_refs,
        proof_object_schema_candidate_refs,
        prover_brand_candidate_refs,
        lean_stub_alignment_refs: matched_pair_refs,
        public_seam_residual_refs: vec![
            format!("theorem_public_seam_residual:{subject_ref}:result_object_and_payload_first"),
            format!(
                "theorem_public_seam_residual:{subject_ref}:prover_brand_and_proof_schema_second"
            ),
            format!(
                "theorem_public_seam_residual:{subject_ref}:final_public_verifier_contract_third"
            ),
            format!(
                "theorem_public_seam_residual:{subject_ref}:actual_lean_execution_environment_conditional"
            ),
        ],
        environment_stop_line_refs: vec![
            "environment_probe:current_l2.theorem_actual_lean_execution_availability_probe"
                .to_string(),
            "environment_stop_line:actual_lean_execution_requires_local_toolchain".to_string(),
            "environment_stop_line:lean_stub_bridge_kept_until_local_toolchain_available"
                .to_string(),
        ],
        compare_floor_refs,
        guard_refs: vec![
            "guard:theorem_result_object_payload_pair_first".to_string(),
            "guard:theorem_prover_brand_proof_schema_pair_second".to_string(),
            "guard:final_public_verifier_contract_third".to_string(),
            "guard:actual_lean_execution_environment_conditional".to_string(),
            "guard:lean_stub_bridge_kept_until_local_toolchain_available".to_string(),
        ],
        kept_later_refs: vec![
            "kept_later:final_public_theorem_result_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
            "kept_later:actual_lean_execution_with_concrete_toolchain".to_string(),
        ],
    })
}

pub fn build_current_l2_source_sample_model_check_second_line_concretization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckSecondLineConcretization, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan)?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact,
        proof_notebook_review_units: _,
        model_check_concrete_carriers,
    } = preview_route;

    let concretization_subject_kind = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_kind.clone());
    let concretization_subject_ref = formal_hook_artifact
        .as_ref()
        .map(|artifact| artifact.subject_ref.clone());
    let principal_machine_carrier_refs = model_check_concrete_carriers
        .iter()
        .map(|carrier| {
            format!(
                "model_check_concrete_carrier:{}:{}",
                carrier.subject_ref, carrier.case.obligation_kind
            )
        })
        .collect::<Vec<_>>();

    if formal_hook_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = formal_hook_guard_reason.unwrap_or_else(|| {
            format!(
                "model-check projection pre-floor was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current model-check second line only actualizes reached row-local carrier routes: {guard_detail}"
        );
        return Ok(CurrentL2SourceSampleModelCheckSecondLineConcretization {
            source_report,
            concretization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            concretization_guard_reason: Some(guard_reason),
            concretization_subject_kind,
            concretization_subject_ref,
            principal_machine_carrier_refs,
            row_local_property_preview_refs: Vec::new(),
            secondary_projection_refs: Vec::new(),
            request_preflight_refs: Vec::new(),
            public_checker_reserve_refs: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            compare_floor_refs: model_check_second_line_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            excluded_family_refs: Vec::new(),
            guard_refs: model_check_second_line_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: model_check_second_line_kept_later_refs(),
        });
    }

    let secondary_projection_refs = if let Some(artifact) = formal_hook_artifact.as_ref() {
        vec![small_cluster_projection_ref(
            &artifact.subject_kind,
            &artifact.subject_ref,
        )?]
    } else {
        Vec::new()
    };
    let row_local_property_preview_refs =
        model_check_second_line_property_preview_refs(&principal_machine_carrier_refs)?;
    let request_preflight_refs =
        model_check_second_line_request_preflight_refs(concretization_subject_ref.as_deref());

    Ok(CurrentL2SourceSampleModelCheckSecondLineConcretization {
        source_report,
        concretization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        concretization_guard_reason: None,
        concretization_subject_kind,
        concretization_subject_ref,
        principal_machine_carrier_refs,
        row_local_property_preview_refs,
        secondary_projection_refs,
        request_preflight_refs,
        public_checker_reserve_refs: model_check_second_line_public_checker_reserve_refs(),
        repo_local_emitted_artifact_refs: model_check_second_line_repo_local_emitted_artifact_refs(
            &model_check_concrete_carriers,
        ),
        compare_floor_refs: model_check_second_line_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        excluded_family_refs: model_check_second_line_excluded_family_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: model_check_second_line_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: model_check_second_line_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_model_check_property_tool_seam_probe(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckPropertyToolSeamProbe, String> {
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleModelCheckSecondLineConcretization {
        source_report,
        concretization_status,
        concretization_guard_reason,
        concretization_subject_kind,
        concretization_subject_ref,
        principal_machine_carrier_refs,
        row_local_property_preview_refs,
        secondary_projection_refs,
        request_preflight_refs: _,
        public_checker_reserve_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        excluded_family_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = second_line;

    if concretization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = concretization_guard_reason.unwrap_or_else(|| {
            format!(
                "model-check second line was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current model-check property/tool-seam probe only actualizes reached second-line routes: {guard_detail}"
        );
        return Ok(CurrentL2SourceSampleModelCheckPropertyToolSeamProbe {
            source_report,
            probe_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            probe_guard_reason: Some(guard_reason),
            probe_subject_kind: concretization_subject_kind,
            probe_subject_ref: concretization_subject_ref,
            principal_machine_carrier_refs,
            row_local_property_preview_refs,
            secondary_projection_refs,
            property_language_probe_refs: Vec::new(),
            tool_seam_probe_refs: Vec::new(),
            checker_boundary_probe_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: model_check_property_tool_seam_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            excluded_family_refs: Vec::new(),
            guard_refs: model_check_property_tool_seam_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: model_check_property_tool_seam_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleModelCheckPropertyToolSeamProbe {
        source_report,
        probe_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        probe_guard_reason: None,
        probe_subject_kind: concretization_subject_kind,
        probe_subject_ref: concretization_subject_ref.clone(),
        principal_machine_carrier_refs: principal_machine_carrier_refs.clone(),
        row_local_property_preview_refs: row_local_property_preview_refs,
        secondary_projection_refs,
        property_language_probe_refs: model_check_property_tool_seam_property_language_probe_refs(
            &principal_machine_carrier_refs,
        )?,
        tool_seam_probe_refs: model_check_property_tool_seam_tool_seam_probe_refs(
            concretization_subject_ref.as_deref(),
        ),
        checker_boundary_probe_refs: model_check_property_tool_seam_checker_boundary_probe_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        repo_local_emitted_artifact_refs,
        compare_floor_refs: model_check_property_tool_seam_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        excluded_family_refs: model_check_property_tool_seam_excluded_family_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        guard_refs: model_check_property_tool_seam_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: model_check_property_tool_seam_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_model_check_property_tool_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckPropertyToolThreshold, String> {
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_argument,
        host_plan.clone(),
    )?;
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleModelCheckSecondLineConcretization {
        source_report,
        concretization_status,
        concretization_guard_reason,
        concretization_subject_kind,
        concretization_subject_ref,
        principal_machine_carrier_refs,
        row_local_property_preview_refs,
        secondary_projection_refs,
        request_preflight_refs,
        public_checker_reserve_refs,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        excluded_family_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = second_line;

    let CurrentL2SourceSampleModelCheckPropertyToolSeamProbe {
        source_report: _,
        probe_status,
        probe_guard_reason,
        probe_subject_kind: _,
        probe_subject_ref: _,
        principal_machine_carrier_refs: _,
        row_local_property_preview_refs: _,
        secondary_projection_refs: _,
        property_language_probe_refs,
        tool_seam_probe_refs,
        checker_boundary_probe_refs,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        excluded_family_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = probe;

    let threshold_status = if concretization_status == CurrentL2EmittedArtifactRouteStatus::Reached
        && probe_status == CurrentL2EmittedArtifactRouteStatus::Reached
    {
        CurrentL2EmittedArtifactRouteStatus::Reached
    } else {
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    };

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = concretization_guard_reason
            .or(probe_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "model-check second line or property/tool-seam probe was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current model-check property/tool threshold only actualizes reached model-check mixed-gate routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleModelCheckPropertyToolThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            threshold_guard_reason: Some(guard_reason),
            threshold_subject_kind: concretization_subject_kind,
            threshold_subject_ref: concretization_subject_ref,
            principal_machine_carrier_refs,
            row_local_property_preview_refs,
            secondary_projection_refs,
            property_language_probe_refs: Vec::new(),
            tool_seam_probe_refs: Vec::new(),
            checker_boundary_probe_refs: Vec::new(),
            request_preflight_refs: Vec::new(),
            public_checker_reserve_refs: Vec::new(),
            threshold_default_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: model_check_property_tool_threshold_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            excluded_family_refs: Vec::new(),
            guard_refs: model_check_property_tool_threshold_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: model_check_property_tool_threshold_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleModelCheckPropertyToolThreshold {
        source_report,
        threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        threshold_guard_reason: None,
        threshold_subject_kind: concretization_subject_kind,
        threshold_subject_ref: concretization_subject_ref,
        principal_machine_carrier_refs,
        row_local_property_preview_refs,
        secondary_projection_refs,
        property_language_probe_refs,
        tool_seam_probe_refs,
        checker_boundary_probe_refs,
        request_preflight_refs,
        public_checker_reserve_refs,
        threshold_default_refs: model_check_property_tool_threshold_default_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        repo_local_emitted_artifact_refs,
        compare_floor_refs: model_check_property_tool_threshold_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        excluded_family_refs,
        guard_refs: model_check_property_tool_threshold_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: model_check_property_tool_threshold_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption, String> {
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleModelCheckPropertyToolThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        threshold_subject_kind,
        threshold_subject_ref,
        principal_machine_carrier_refs: _,
        row_local_property_preview_refs: _,
        secondary_projection_refs: _,
        property_language_probe_refs: _,
        tool_seam_probe_refs: _,
        checker_boundary_probe_refs: _,
        request_preflight_refs: _,
        public_checker_reserve_refs: _,
        threshold_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        excluded_family_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = threshold_guard_reason.unwrap_or_else(|| {
            format!(
                "model-check threshold route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current model-check row-local property actual adoption only actualizes reached model-check threshold routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind: threshold_subject_kind,
                actualization_subject_ref: threshold_subject_ref,
                property_route_refs: Vec::new(),
                checker_contract_route_refs: Vec::new(),
                tool_binding_reserve_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    model_check_row_local_property_actual_adoption_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                excluded_family_refs: Vec::new(),
                guard_refs: model_check_row_local_property_actual_adoption_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: model_check_row_local_property_actual_adoption_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind: threshold_subject_kind,
            actualization_subject_ref: threshold_subject_ref.clone(),
            property_route_refs: model_check_row_local_property_actual_adoption_property_route_refs(
                threshold_subject_ref.as_deref(),
            ),
            checker_contract_route_refs:
                model_check_row_local_property_actual_adoption_checker_contract_route_refs(
                    threshold_subject_ref.as_deref(),
                ),
            tool_binding_reserve_refs:
                model_check_row_local_property_actual_adoption_tool_binding_reserve_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            actual_adoption_default_refs:
                model_check_row_local_property_actual_adoption_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs: model_check_row_local_property_actual_adoption_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            excluded_family_refs:
                model_check_row_local_property_actual_adoption_excluded_family_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            guard_refs: model_check_row_local_property_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: model_check_row_local_property_actual_adoption_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization, String> {
    let row_local_actual_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        property_route_refs: _,
        checker_contract_route_refs: _,
        tool_binding_reserve_refs: _,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        excluded_family_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = row_local_actual_adoption;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = actualization_guard_reason.unwrap_or_else(|| {
            format!(
                "model-check row-local actual adoption was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current model-check public-checker artifact preview actualization only actualizes reached model-check row-local adoption routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                checker_artifact_preview_refs: Vec::new(),
                verifier_handoff_reserve_refs: Vec::new(),
                tool_binding_reserve_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    model_check_public_checker_artifact_preview_actualization_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                guard_refs: model_check_public_checker_artifact_preview_actualization_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs:
                    model_check_public_checker_artifact_preview_actualization_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            checker_artifact_preview_refs:
                model_check_public_checker_artifact_preview_actualization_preview_refs(
                    actualization_subject_ref.as_deref(),
                ),
            verifier_handoff_reserve_refs:
                model_check_public_checker_artifact_preview_actualization_handoff_reserve_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            tool_binding_reserve_refs:
                model_check_public_checker_artifact_preview_actualization_tool_binding_reserve_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            actual_adoption_default_refs:
                model_check_public_checker_artifact_preview_actualization_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs:
                model_check_public_checker_artifact_preview_actualization_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            guard_refs: model_check_public_checker_artifact_preview_actualization_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs:
                model_check_public_checker_artifact_preview_actualization_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate, String> {
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_argument,
            host_plan.clone(),
        )?;
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        checker_artifact_preview_refs: _,
        verifier_handoff_reserve_refs: _,
        tool_binding_reserve_refs: _,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = artifact_preview;

    let CurrentL2SourceSampleModelCheckPropertyToolThreshold {
        source_report: _,
        threshold_status,
        threshold_guard_reason,
        threshold_subject_kind: _,
        threshold_subject_ref: _,
        principal_machine_carrier_refs: _,
        row_local_property_preview_refs: _,
        secondary_projection_refs: _,
        property_language_probe_refs: _,
        tool_seam_probe_refs: _,
        checker_boundary_probe_refs: _,
        request_preflight_refs: _,
        public_checker_reserve_refs: _,
        threshold_default_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        excluded_family_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = actualization_guard_reason
            .or(threshold_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "model-check public-checker artifact preview or property/tool threshold was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current model-check tool-brand / verifier-handoff coupled later gate only actualizes reached model-check public-checker preview and property/tool threshold routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                verifier_handoff_candidate_refs: Vec::new(),
                tool_brand_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    model_check_tool_brand_verifier_handoff_coupled_later_gate_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                guard_refs: model_check_tool_brand_verifier_handoff_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs:
                    model_check_tool_brand_verifier_handoff_coupled_later_gate_kept_later_refs(),
            },
        );
    }

    Ok(CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate {
        source_report,
        actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        actualization_guard_reason: None,
        actualization_subject_kind,
        actualization_subject_ref: actualization_subject_ref.clone(),
        verifier_handoff_candidate_refs:
            model_check_tool_brand_verifier_handoff_coupled_later_gate_verifier_handoff_candidate_refs(
                actualization_subject_ref.as_deref(),
            ),
        tool_brand_candidate_refs:
            model_check_tool_brand_verifier_handoff_coupled_later_gate_tool_brand_candidate_refs(
                actualization_subject_ref.as_deref(),
            ),
        coupled_default_refs:
            model_check_tool_brand_verifier_handoff_coupled_later_gate_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
        repo_local_emitted_artifact_refs,
        compare_floor_refs:
            model_check_tool_brand_verifier_handoff_coupled_later_gate_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
        guard_refs: model_check_tool_brand_verifier_handoff_coupled_later_gate_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: model_check_tool_brand_verifier_handoff_coupled_later_gate_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate, String> {
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        verifier_handoff_candidate_refs: _,
        tool_brand_candidate_refs: _,
        coupled_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: tool_handoff_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = tool_handoff;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = actualization_guard_reason.unwrap_or_else(|| {
            format!(
                "model-check tool-brand / verifier-handoff coupled later gate was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current model-check public-checker artifact / migration coupled later gate only actualizes reached model-check tool-brand / verifier-handoff routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                public_checker_artifact_candidate_refs: Vec::new(),
                checker_migration_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    model_check_public_checker_artifact_migration_coupled_later_gate_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        tool_handoff_compare_floor_refs,
                    ),
                guard_refs:
                    model_check_public_checker_artifact_migration_coupled_later_gate_guard_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                kept_later_refs:
                    model_check_public_checker_artifact_migration_coupled_later_gate_kept_later_refs(
                    ),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            public_checker_artifact_candidate_refs:
                model_check_public_checker_artifact_migration_coupled_later_gate_public_checker_artifact_candidate_refs(
                    actualization_subject_ref.as_deref(),
                ),
            checker_migration_candidate_refs:
                model_check_public_checker_artifact_migration_coupled_later_gate_checker_migration_candidate_refs(
                    actualization_subject_ref.as_deref(),
                ),
            coupled_default_refs:
                model_check_public_checker_artifact_migration_coupled_later_gate_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs:
                model_check_public_checker_artifact_migration_coupled_later_gate_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    tool_handoff_compare_floor_refs,
                ),
            guard_refs:
                model_check_public_checker_artifact_migration_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            kept_later_refs:
                model_check_public_checker_artifact_migration_coupled_later_gate_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption, String> {
    let artifact_migration =
        build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate {
        source_report,
        actualization_status,
        actualization_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        public_checker_artifact_candidate_refs: _,
        checker_migration_candidate_refs: _,
        coupled_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: artifact_migration_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = artifact_migration;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = actualization_guard_reason.unwrap_or_else(|| {
            format!(
                "model-check public-checker artifact / migration coupled later gate was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current model-check checker-artifact route actual adoption only actualizes reached model-check public-checker artifact / migration routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                checker_artifact_route_refs: Vec::new(),
                migration_candidate_keep_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    model_check_checker_artifact_route_actual_adoption_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        artifact_migration_compare_floor_refs,
                    ),
                guard_refs: model_check_checker_artifact_route_actual_adoption_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: model_check_checker_artifact_route_actual_adoption_kept_later_refs(
                ),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            checker_artifact_route_refs:
                model_check_checker_artifact_route_actual_adoption_route_refs(
                    actualization_subject_ref.as_deref(),
                ),
            migration_candidate_keep_refs:
                model_check_checker_artifact_route_actual_adoption_migration_keep_refs(
                    actualization_subject_ref.as_deref(),
                ),
            actual_adoption_default_refs:
                model_check_checker_artifact_route_actual_adoption_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs:
                model_check_checker_artifact_route_actual_adoption_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    artifact_migration_compare_floor_refs,
                ),
            guard_refs: model_check_checker_artifact_route_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: model_check_checker_artifact_route_actual_adoption_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold, String> {
    let checker_artifact_route =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let tool_brand_verifier_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption {
        source_report,
        actualization_status: checker_artifact_status,
        actualization_guard_reason: checker_artifact_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        checker_artifact_route_refs,
        migration_candidate_keep_refs,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: checker_artifact_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = checker_artifact_route;

    let CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate {
        source_report: _,
        actualization_status: tool_brand_status,
        actualization_guard_reason: tool_brand_guard_reason,
        actualization_subject_kind: _,
        actualization_subject_ref: _,
        verifier_handoff_candidate_refs,
        tool_brand_candidate_refs,
        coupled_default_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = tool_brand_verifier_handoff;

    if checker_artifact_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || tool_brand_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = checker_artifact_guard_reason
            .or(tool_brand_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "model-check checker-artifact route or tool-brand/verifier-handoff gate was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current model-check final public contract reopen threshold only actualizes reached model-check checker-artifact route and tool-brand/verifier-handoff routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold {
                source_report,
                threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                threshold_guard_reason: Some(guard_reason),
                actualization_subject_kind,
                actualization_subject_ref,
                checker_artifact_route_refs: Vec::new(),
                migration_candidate_keep_refs: Vec::new(),
                verifier_handoff_candidate_refs: Vec::new(),
                tool_brand_candidate_refs: Vec::new(),
                final_public_contract_reopen_sequence_refs: Vec::new(),
                threshold_default_refs: Vec::new(),
                repo_local_emitted_artifact_refs,
                compare_floor_refs:
                    model_check_final_public_contract_reopen_threshold_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        checker_artifact_compare_floor_refs,
                    ),
                guard_refs: model_check_final_public_contract_reopen_threshold_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: model_check_final_public_contract_reopen_threshold_kept_later_refs(
                ),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            threshold_guard_reason: None,
            actualization_subject_kind,
            actualization_subject_ref: actualization_subject_ref.clone(),
            checker_artifact_route_refs,
            migration_candidate_keep_refs,
            verifier_handoff_candidate_refs,
            tool_brand_candidate_refs,
            final_public_contract_reopen_sequence_refs:
                model_check_final_public_contract_reopen_threshold_sequence_refs(
                    actualization_subject_ref.as_deref(),
                ),
            threshold_default_refs: model_check_final_public_contract_reopen_threshold_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            repo_local_emitted_artifact_refs,
            compare_floor_refs:
                model_check_final_public_contract_reopen_threshold_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    checker_artifact_compare_floor_refs,
                ),
            guard_refs: model_check_final_public_contract_reopen_threshold_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: model_check_final_public_contract_reopen_threshold_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_model_check_public_seam_compression(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleModelCheckPublicSeamCompression, String> {
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_argument,
            host_plan.clone(),
        )?;
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        actualization_subject_kind,
        actualization_subject_ref,
        checker_artifact_route_refs,
        migration_candidate_keep_refs,
        verifier_handoff_candidate_refs,
        tool_brand_candidate_refs,
        final_public_contract_reopen_sequence_refs: _,
        threshold_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: threshold_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    let CurrentL2SourceSampleModelCheckPropertyToolSeamProbe {
        source_report: probe_source_report,
        probe_status,
        probe_guard_reason,
        probe_subject_kind,
        probe_subject_ref,
        principal_machine_carrier_refs: _,
        row_local_property_preview_refs: _,
        secondary_projection_refs: _,
        property_language_probe_refs,
        tool_seam_probe_refs,
        checker_boundary_probe_refs,
        repo_local_emitted_artifact_refs: probe_repo_local_emitted_artifact_refs,
        compare_floor_refs: probe_compare_floor_refs,
        excluded_family_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = probe;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || probe_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = threshold_guard_reason
            .or(probe_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "model-check final public-contract threshold or property/tool probe was not reached for `{}`",
                    source_report.sample_id
                )
            });

        return Ok(CurrentL2SourceSampleModelCheckPublicSeamCompression {
            source_report,
            compression_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            compression_guard_reason: Some(format!(
                "current model-check public seam compression only actualizes reached model-check final-threshold + property/tool probe floors: {guard_detail}"
            )),
            actualization_subject_kind,
            actualization_subject_ref,
            repo_local_emitted_artifact_refs: Vec::new(),
            checker_artifact_route_refs: Vec::new(),
            migration_candidate_keep_refs: Vec::new(),
            verifier_handoff_candidate_refs: Vec::new(),
            tool_brand_candidate_refs: Vec::new(),
            property_language_probe_refs: Vec::new(),
            tool_seam_probe_refs: Vec::new(),
            checker_boundary_probe_refs: Vec::new(),
            public_seam_residual_refs: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.model_check.public_seam_compression.guard_only"
                    .to_string(),
            ],
            guard_refs: vec!["guard:model_check_public_seam_compression_not_reached".to_string()],
            kept_later_refs: vec![
                "kept_later:first_settled_property_language".to_string(),
                "kept_later:concrete_model_check_tool_brand".to_string(),
                "kept_later:final_public_checker_artifact".to_string(),
                "kept_later:actual_public_checker_migration".to_string(),
                "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
                "kept_later:production_checker_runtime_policy_contract".to_string(),
                "kept_later:final_public_verifier_contract".to_string(),
            ],
        });
    }

    if source_report.sample_id != probe_source_report.sample_id
        || actualization_subject_kind != probe_subject_kind
        || actualization_subject_ref != probe_subject_ref
    {
        return Err(format!(
            "model-check public seam compression expected aligned threshold/probe subjects for `{}` but got threshold {:?}/{:?} and probe {:?}/{:?}",
            source_report.sample_id,
            actualization_subject_kind,
            actualization_subject_ref,
            probe_subject_kind,
            probe_subject_ref
        ));
    }

    let subject_ref = actualization_subject_ref.as_deref().ok_or_else(|| {
        format!(
            "model-check public seam compression expected a subject ref for `{}`",
            source_report.sample_id
        )
    })?;
    let mut merged_repo_local_emitted_artifact_refs = repo_local_emitted_artifact_refs;
    extend_unique_refs(
        &mut merged_repo_local_emitted_artifact_refs,
        probe_repo_local_emitted_artifact_refs,
    );
    let mut compare_floor_refs = threshold_compare_floor_refs;
    extend_unique_refs(&mut compare_floor_refs, probe_compare_floor_refs);
    extend_unique_refs(
        &mut compare_floor_refs,
        vec!["compare_floor:current_l2.model_check.public_seam_compression".to_string()],
    );

    Ok(CurrentL2SourceSampleModelCheckPublicSeamCompression {
        source_report,
        compression_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        compression_guard_reason: None,
        actualization_subject_kind,
        actualization_subject_ref: Some(subject_ref.to_string()),
        repo_local_emitted_artifact_refs: merged_repo_local_emitted_artifact_refs,
        checker_artifact_route_refs,
        migration_candidate_keep_refs,
        verifier_handoff_candidate_refs,
        tool_brand_candidate_refs,
        property_language_probe_refs,
        tool_seam_probe_refs,
        checker_boundary_probe_refs,
        public_seam_residual_refs: vec![
            format!(
                "model_check_public_seam_residual:{subject_ref}:property_language_and_tool_brand_first"
            ),
            format!(
                "model_check_public_seam_residual:{subject_ref}:public_checker_artifact_and_migration_second"
            ),
            format!(
                "model_check_public_seam_residual:{subject_ref}:verifier_handoff_and_runtime_policy_contract_third"
            ),
            format!(
                "model_check_public_seam_residual:{subject_ref}:final_public_verifier_contract_fourth"
            ),
        ],
        compare_floor_refs,
        guard_refs: vec![
            "guard:model_check_property_language_and_tool_brand_first".to_string(),
            "guard:model_check_public_checker_artifact_and_migration_second".to_string(),
            "guard:model_check_verifier_handoff_and_runtime_policy_contract_third".to_string(),
            "guard:model_check_final_public_verifier_contract_fourth".to_string(),
        ],
        kept_later_refs: vec![
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:final_public_checker_artifact".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ],
    })
}

pub fn build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan)?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact: _,
        proof_notebook_review_units,
        model_check_concrete_carriers,
    } = preview_route;

    let sample_id = source_report.sample_id.clone();
    let reached_authoritative_sample = matches!(
        sample_id.as_str(),
        "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh"
    ) && formal_hook_status
        == CurrentL2EmittedArtifactRouteStatus::Reached;

    if !reached_authoritative_sample {
        let guard_detail = formal_hook_guard_reason.unwrap_or_else(|| {
            format!("current default samples (`p07` / `p08`) were not reached for `{sample_id}`")
        });
        let guard_reason = format!(
            "current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): {guard_detail}"
        );
        return Ok(
            CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
                source_report,
                slice_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                slice_guard_reason: Some(guard_reason),
                profile_axis_refs: Vec::new(),
                relation_refs: Vec::new(),
                authority_handoff_refs: Vec::new(),
                runtime_evidence_refs: Vec::new(),
                repo_local_emitted_artifact_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.authoritative_room.guard_only".to_string(),
                ],
                contrast_refs: vec!["contrast_target:append_friendly_notice_room".to_string()],
                guard_refs: vec!["guard:room_default_slice_not_reached".to_string()],
                kept_later_refs: authoritative_room_kept_later_refs(),
            },
        );
    }

    let runtime_evidence_refs = authoritative_room_runtime_evidence_refs(&source_report);

    Ok(
        CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
            source_report,
            slice_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            slice_guard_reason: None,
            profile_axis_refs: authoritative_room_profile_axis_refs(&sample_id),
            relation_refs: authoritative_room_relation_refs(&sample_id),
            authority_handoff_refs: authoritative_room_handoff_refs(&sample_id),
            runtime_evidence_refs,
            repo_local_emitted_artifact_refs: authoritative_room_repo_local_emitted_artifact_refs(
                &proof_notebook_review_units,
                &model_check_concrete_carriers,
            ),
            compare_floor_refs: vec![
                "compare_floor:current_l2.order_handoff.runner_cli".to_string(),
                "compare_floor:current_l2.verifier_preview_alignment".to_string(),
                "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
            ],
            contrast_refs: vec!["contrast_target:append_friendly_notice_room".to_string()],
            guard_refs: authoritative_room_guard_refs(&sample_id),
            kept_later_refs: authoritative_room_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_auditable_authority_witness_strengthening(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening, String> {
    let vertical_slice =
        build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
        source_report,
        slice_status,
        slice_guard_reason,
        profile_axis_refs: _,
        relation_refs: _,
        authority_handoff_refs: _,
        runtime_evidence_refs,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = vertical_slice;

    let sample_id = source_report.sample_id.clone();
    let reached_witness_sample = matches!(sample_id.as_str(), "p07-dice-late-join-visible-history")
        && slice_status == CurrentL2EmittedArtifactRouteStatus::Reached;

    if !reached_witness_sample {
        let guard_detail = match sample_id.as_str() {
            "p08-dice-stale-reconnect-refresh" => {
                "current default room profile was reached, but this sample does not carry a witness-bearing authoritative draw".to_string()
            }
            _ => slice_guard_reason.unwrap_or_else(|| {
                format!(
                    "sample `{sample_id}` did not reach the witness-bearing authoritative draw strengthening floor"
                )
            }),
        };
        let guard_reason = format!(
            "current witness strengthening only actualizes reached witness-bearing authoritative draw samples (`p07`): {guard_detail}"
        );
        return Ok(
            CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening {
                source_report,
                strengthening_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                strengthening_guard_reason: Some(guard_reason),
                fairness_claim_refs: Vec::new(),
                witness_core_refs: Vec::new(),
                witness_binding_refs: Vec::new(),
                runtime_evidence_refs: Vec::new(),
                repo_local_emitted_artifact_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.auditable_authority_witness.guard_only".to_string(),
                ],
                contrast_refs: auditable_authority_witness_contrast_refs(),
                guard_refs: vec!["guard:witness_strengthening_not_reached".to_string()],
                kept_later_refs: auditable_authority_witness_kept_later_refs(),
            },
        );
    }

    Ok(
        CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening {
            source_report,
            strengthening_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            strengthening_guard_reason: None,
            fairness_claim_refs: vec!["fairness_claim:auditable_authority_witness".to_string()],
            witness_core_refs: vec![
                "witness_field:witness_kind".to_string(),
                "witness_field:action_ref".to_string(),
                "witness_field:draw_slot".to_string(),
                "witness_field:draw_result".to_string(),
            ],
            witness_binding_refs: auditable_authority_witness_binding_refs(&sample_id),
            runtime_evidence_refs,
            repo_local_emitted_artifact_refs,
            compare_floor_refs: vec![
                "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
                "compare_floor:current_l2.auditable_authority_witness.strengthening".to_string(),
            ],
            contrast_refs: auditable_authority_witness_contrast_refs(),
            guard_refs: vec![
                "guard:room_profile_claim_payload_split".to_string(),
                "guard:minimal_witness_core_only".to_string(),
                "guard:witness_bearing_authoritative_sample_reached".to_string(),
            ],
            kept_later_refs: auditable_authority_witness_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_delegated_rng_service_practical_actualization(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleDelegatedRngServicePracticalActualization, String> {
    let preview_route =
        build_current_l2_source_sample_preview_artifact_route(sample_argument, host_plan)?;

    let CurrentL2SourceSamplePreviewArtifactRoute {
        source_report,
        formal_hook_status,
        formal_hook_guard_reason,
        formal_hook_artifact: _,
        proof_notebook_review_units,
        model_check_concrete_carriers,
    } = preview_route;

    let sample_id = source_report.sample_id.clone();
    let reached_delegated_provider_sample = matches!(
        sample_id.as_str(),
        "p09-dice-delegated-rng-provider-placement"
    ) && formal_hook_status
        == CurrentL2EmittedArtifactRouteStatus::Reached;

    if !reached_delegated_provider_sample {
        let guard_detail = match sample_id.as_str() {
            "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh" => {
                "current authoritative-room baseline sample does not execute the delegated provider practical slice".to_string()
            }
            _ => formal_hook_guard_reason.unwrap_or_else(|| {
                format!(
                    "sample `{sample_id}` did not reach the delegated provider practical slice"
                )
            }),
        };
        let guard_reason = format!(
            "current delegated provider package only actualizes reached delegated provider practical samples (`p09`): {guard_detail}"
        );
        return Ok(
            CurrentL2SourceSampleDelegatedRngServicePracticalActualization {
                source_report,
                practical_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                practical_guard_reason: Some(guard_reason),
                profile_axis_refs: Vec::new(),
                provider_boundary_refs: Vec::new(),
                optional_attachment_refs: Vec::new(),
                runtime_evidence_refs: Vec::new(),
                repo_local_emitted_artifact_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.delegated_rng_service.guard_only".to_string(),
                ],
                contrast_refs: delegated_rng_service_contrast_refs(),
                guard_refs: vec!["guard:delegated_provider_slice_not_reached".to_string()],
                kept_later_refs: delegated_rng_service_kept_later_refs(),
            },
        );
    }

    let runtime_evidence_refs = delegated_rng_service_runtime_evidence_refs(&source_report);

    Ok(
        CurrentL2SourceSampleDelegatedRngServicePracticalActualization {
            source_report,
            practical_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            practical_guard_reason: None,
            profile_axis_refs: delegated_rng_service_profile_axis_refs(),
            provider_boundary_refs: delegated_rng_service_provider_boundary_refs(),
            optional_attachment_refs: delegated_rng_service_optional_attachment_refs(),
            runtime_evidence_refs,
            repo_local_emitted_artifact_refs: authoritative_room_repo_local_emitted_artifact_refs(
                &proof_notebook_review_units,
                &model_check_concrete_carriers,
            ),
            compare_floor_refs: vec![
                "compare_floor:current_l2.order_handoff.runner_cli".to_string(),
                "compare_floor:current_l2.delegated_rng_service.practical".to_string(),
            ],
            contrast_refs: delegated_rng_service_contrast_refs(),
            guard_refs: vec![
                "guard:provider_placement_only".to_string(),
                "guard:authority_keeps_commit".to_string(),
                "guard:provider_receipt_optional".to_string(),
                "guard:fairness_claim_not_collapsed_with_attestation".to_string(),
            ],
            kept_later_refs: delegated_rng_service_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold, String> {
    let vertical_slice =
        build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
            sample_argument,
            host_plan.clone(),
        )?;
    let witness = build_current_l2_source_sample_auditable_authority_witness_strengthening(
        sample_argument,
        host_plan.clone(),
    )?;
    let delegated = build_current_l2_source_sample_delegated_rng_service_practical_actualization(
        sample_argument,
        host_plan,
    )?;

    let sample_id = vertical_slice.source_report.sample_id.clone();
    let threshold_status = if vertical_slice.slice_status
        == CurrentL2EmittedArtifactRouteStatus::Reached
        || witness.strengthening_status == CurrentL2EmittedArtifactRouteStatus::Reached
        || delegated.practical_status == CurrentL2EmittedArtifactRouteStatus::Reached
    {
        CurrentL2EmittedArtifactRouteStatus::Reached
    } else {
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    };

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = vertical_slice
            .slice_guard_reason
            .clone()
            .or_else(|| witness.strengthening_guard_reason.clone())
            .or_else(|| delegated.practical_guard_reason.clone())
            .unwrap_or_else(|| {
                format!("witness/provider/artifact reserve route was not reached for `{sample_id}`")
            });
        let guard_reason = format!(
            "current witness/provider/artifact public-shape threshold only actualizes reached reserve routes: {guard_detail}"
        );
        return Ok(
            CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold {
                source_report: vertical_slice.source_report,
                threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                threshold_guard_reason: Some(guard_reason),
                profile_axis_refs: Vec::new(),
                witness_attachment_refs: Vec::new(),
                provider_attachment_refs: Vec::new(),
                emitted_artifact_reserve_refs: Vec::new(),
                threshold_default_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold_guard_only"
                        .to_string(),
                ],
                contrast_refs: witness_provider_artifact_public_shape_contrast_refs(),
                guard_refs: vec!["guard:witness_provider_artifact_threshold_not_reached".to_string()],
                kept_later_refs: witness_provider_artifact_public_shape_kept_later_refs(),
            },
        );
    }

    let profile_axis_refs =
        if delegated.practical_status == CurrentL2EmittedArtifactRouteStatus::Reached {
            delegated.profile_axis_refs.clone()
        } else {
            vertical_slice.profile_axis_refs.clone()
        };
    let witness_attachment_refs = witness_provider_public_shape_witness_attachment_refs(&witness);
    let provider_attachment_refs =
        witness_provider_public_shape_provider_attachment_refs(&delegated);
    let emitted_artifact_reserve_refs =
        witness_provider_public_shape_emitted_artifact_refs(&vertical_slice, &witness, &delegated);
    let compare_floor_refs =
        witness_provider_public_shape_compare_floor_refs(&vertical_slice, &witness, &delegated);
    let source_report = vertical_slice.source_report;

    Ok(
        CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            threshold_guard_reason: None,
            profile_axis_refs,
            witness_attachment_refs,
            provider_attachment_refs,
            emitted_artifact_reserve_refs,
            threshold_default_refs: witness_provider_artifact_public_shape_threshold_default_refs(),
            compare_floor_refs,
            contrast_refs: witness_provider_artifact_public_shape_contrast_refs(),
            guard_refs: witness_provider_artifact_public_shape_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: witness_provider_artifact_public_shape_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption, String> {
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        profile_axis_refs,
        witness_attachment_refs,
        provider_attachment_refs,
        emitted_artifact_reserve_refs,
        threshold_default_refs: _,
        compare_floor_refs,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = threshold_guard_reason.unwrap_or_else(|| {
            format!(
                "witness/provider/artifact threshold route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current witness/provider/artifact public-shape actual adoption only actualizes reached threshold routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                profile_axis_refs,
                witness_route_refs: Vec::new(),
                provider_route_refs: Vec::new(),
                repo_local_emitted_artifact_refs: emitted_artifact_reserve_refs,
                actual_adoption_default_refs: Vec::new(),
                compare_floor_refs:
                    witness_provider_artifact_public_shape_actual_adoption_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        Vec::new(),
                    ),
                guard_refs: witness_provider_artifact_public_shape_actual_adoption_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: witness_provider_artifact_public_shape_kept_later_refs(),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    Ok(
        CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            profile_axis_refs,
            witness_route_refs:
                witness_provider_artifact_public_shape_actual_adoption_witness_route_refs(
                    &sample_id,
                    &witness_attachment_refs,
                ),
            provider_route_refs:
                witness_provider_artifact_public_shape_actual_adoption_provider_route_refs(
                    &sample_id,
                    &provider_attachment_refs,
                ),
            repo_local_emitted_artifact_refs: emitted_artifact_reserve_refs,
            actual_adoption_default_refs:
                witness_provider_artifact_public_shape_actual_adoption_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            compare_floor_refs:
                witness_provider_artifact_public_shape_actual_adoption_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    compare_floor_refs,
                ),
            guard_refs: witness_provider_artifact_public_shape_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: witness_provider_artifact_public_shape_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_minimal_companion_surface(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleMinimalCompanionSurface, String> {
    let vertical_slice =
        build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
        source_report,
        slice_status,
        slice_guard_reason,
        profile_axis_refs: _,
        relation_refs: _,
        authority_handoff_refs: _,
        runtime_evidence_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = vertical_slice;

    if slice_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = slice_guard_reason.unwrap_or_else(|| {
            format!(
                "reached authoritative-room default was not available for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current minimal companion surface only actualizes reached authoritative-room defaults: {guard_detail}"
        );
        return Ok(CurrentL2SourceSampleMinimalCompanionSurface {
            source_report,
            surface_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            surface_guard_reason: Some(guard_reason),
            companion_lines: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.experimental_order_handoff_guard_only".to_string(),
            ],
            guard_refs: vec!["guard:companion_surface_not_reached".to_string()],
            kept_later_refs: minimal_companion_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleMinimalCompanionSurface {
        companion_lines: minimal_companion_lines(&source_report.sample_id),
        source_report,
        surface_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        surface_guard_reason: None,
        compare_floor_refs: vec![
            "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
            "compare_floor:current_l2.experimental_order_handoff_surface".to_string(),
        ],
        guard_refs: vec![
            "guard:semantic_honesty_first".to_string(),
            "guard:helper_local_companion_only".to_string(),
        ],
        kept_later_refs: minimal_companion_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_stage_block_surface(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleStageBlockSurface, String> {
    let vertical_slice =
        build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
        source_report,
        slice_status,
        slice_guard_reason,
        profile_axis_refs: _,
        relation_refs: _,
        authority_handoff_refs: _,
        runtime_evidence_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = vertical_slice;

    if slice_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = slice_guard_reason.unwrap_or_else(|| {
            format!(
                "reached authoritative-room default was not available for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current stage-block secondary surface only actualizes reached authoritative-room defaults: {guard_detail}"
        );
        return Ok(CurrentL2SourceSampleStageBlockSurface {
            source_report,
            surface_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            surface_guard_reason: Some(guard_reason),
            stage_lines: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.experimental_stage_block_guard_only".to_string(),
            ],
            guard_refs: vec!["guard:stage_block_surface_not_reached".to_string()],
            kept_later_refs: stage_block_surface_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleStageBlockSurface {
        stage_lines: stage_block_surface_lines(&source_report.sample_id),
        source_report,
        surface_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        surface_guard_reason: None,
        compare_floor_refs: vec![
            "compare_floor:current_l2.experimental_order_handoff_surface".to_string(),
            "compare_floor:current_l2.experimental_stage_block_surface".to_string(),
        ],
        guard_refs: vec![
            "guard:stage_block_secondary_candidate".to_string(),
            "guard:helper_local_companion_only".to_string(),
        ],
        kept_later_refs: stage_block_surface_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface, String> {
    let source_wording_route =
        build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let delegated_practical =
        build_current_l2_source_sample_delegated_rng_service_practical_actualization(
            sample_argument,
            host_plan.clone(),
        )?;
    let witness_provider_route =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption {
        source_report: route_source_report,
        actualization_status: route_status,
        actualization_guard_reason: route_guard_reason,
        profile_axis_refs: route_profile_axis_refs,
        repo_local_emitted_artifact_refs: route_repo_local_emitted_artifact_refs,
        source_wording_route_refs: _,
        emitted_artifact_candidate_keep_refs: _,
        actual_adoption_default_refs: _,
        compare_floor_refs: route_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = source_wording_route;

    let CurrentL2SourceSampleDelegatedRngServicePracticalActualization {
        source_report: delegated_source_report,
        practical_status: delegated_status,
        practical_guard_reason: delegated_guard_reason,
        profile_axis_refs: delegated_profile_axis_refs,
        provider_boundary_refs: _,
        optional_attachment_refs: _,
        runtime_evidence_refs: _,
        repo_local_emitted_artifact_refs: delegated_repo_local_emitted_artifact_refs,
        compare_floor_refs: delegated_compare_floor_refs,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = delegated_practical;

    let CurrentL2SourceSampleWitnessProviderRouteActualAdoption {
        source_report: _witness_route_source_report,
        actualization_status: witness_route_status,
        actualization_guard_reason: witness_route_guard_reason,
        profile_axis_refs: witness_route_profile_axis_refs,
        witness_route_actual_refs: _,
        provider_route_actual_refs: _,
        schema_candidate_keep_refs: _,
        repo_local_emitted_artifact_refs: witness_route_repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs: witness_route_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = witness_provider_route;

    let sample_id = route_source_report.sample_id.clone();
    let reached_status = match sample_id.as_str() {
        "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh" => {
            route_status == CurrentL2EmittedArtifactRouteStatus::Reached
        }
        "p09-dice-delegated-rng-provider-placement" => {
            delegated_status == CurrentL2EmittedArtifactRouteStatus::Reached
                && witness_route_status == CurrentL2EmittedArtifactRouteStatus::Reached
        }
        _ => false,
    };

    if !reached_status {
        let source_report = match sample_id.as_str() {
            "p09-dice-delegated-rng-provider-placement" => delegated_source_report,
            _ => route_source_report,
        };
        let guard_detail = match sample_id.as_str() {
            "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh" => {
                route_guard_reason.unwrap_or_else(|| {
                    format!(
                        "sample `{sample_id}` did not reach the source-wording route actual adoption floor"
                    )
                })
            }
            "p09-dice-delegated-rng-provider-placement" => delegated_guard_reason
                .or(witness_route_guard_reason)
                .unwrap_or_else(|| {
                    format!(
                        "sample `{sample_id}` did not reach the delegated-provider practical and witness/provider route floors"
                    )
                }),
            _ => route_guard_reason
                .or(delegated_guard_reason)
                .or(witness_route_guard_reason)
                .unwrap_or_else(|| {
                    format!(
                        "sample `{sample_id}` did not reach the authoritative-room serial-scope reserve surface"
                    )
                }),
        };
        let guard_reason = format!(
            "current serial-scope reserve surface only actualizes authoritative-room-specific reached routes (`p07` / `p08` / `p09`): {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface {
            source_report,
            surface_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            surface_guard_reason: Some(guard_reason),
            profile_axis_refs: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            serial_scope_lines: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface.guard_only"
                    .to_string(),
            ],
            guard_refs: vec!["guard:serial_scope_reserve_surface_not_reached".to_string()],
            kept_later_refs: order_handoff_serial_scope_reserve_surface_kept_later_refs(),
        });
    }

    let (source_report, profile_axis_refs, repo_local_emitted_artifact_refs, compare_floor_refs) =
        match sample_id.as_str() {
            "p09-dice-delegated-rng-provider-placement" => {
                let mut refs = delegated_compare_floor_refs;
                for entry in witness_route_compare_floor_refs {
                    if !refs.contains(&entry) {
                        refs.push(entry);
                    }
                }
                refs.push(
                    "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface"
                        .to_string(),
                );
                (
                    delegated_source_report,
                    delegated_profile_axis_refs,
                    delegated_repo_local_emitted_artifact_refs,
                    refs,
                )
            }
            _ => {
                let mut refs = route_compare_floor_refs;
                if !refs.contains(
                    &"compare_floor:current_l2.order_handoff.serial_scope_reserve_surface"
                        .to_string(),
                ) {
                    refs.push(
                        "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface"
                            .to_string(),
                    );
                }
                (
                    route_source_report,
                    route_profile_axis_refs,
                    route_repo_local_emitted_artifact_refs,
                    refs,
                )
            }
        };

    if sample_id == "p09-dice-delegated-rng-provider-placement"
        && repo_local_emitted_artifact_refs != witness_route_repo_local_emitted_artifact_refs
    {
        return Err(format!(
            "serial-scope reserve surface expected delegated provider emitted artifacts to align with witness/provider route for `{sample_id}`"
        ));
    }

    if sample_id == "p09-dice-delegated-rng-provider-placement"
        && profile_axis_refs != witness_route_profile_axis_refs
    {
        return Err(format!(
            "serial-scope reserve surface expected delegated provider profile axes to align with witness/provider route for `{sample_id}`"
        ));
    }

    Ok(CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface {
        source_report,
        surface_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        surface_guard_reason: None,
        profile_axis_refs,
        repo_local_emitted_artifact_refs,
        serial_scope_lines: order_handoff_serial_scope_reserve_surface_lines(&sample_id),
        compare_floor_refs,
        guard_refs: order_handoff_serial_scope_reserve_surface_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: order_handoff_serial_scope_reserve_surface_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_order_handoff_surface_artifact_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold, String> {
    let vertical_slice =
        build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
            sample_argument,
            host_plan.clone(),
        )?;
    let principal_surface = build_current_l2_source_sample_minimal_companion_surface(
        sample_argument,
        host_plan.clone(),
    )?;
    let secondary_surface =
        build_current_l2_source_sample_stage_block_surface(sample_argument, host_plan.clone())?;
    let artifact_threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization {
        source_report,
        slice_status,
        slice_guard_reason,
        profile_axis_refs,
        relation_refs: _,
        authority_handoff_refs: _,
        runtime_evidence_refs: _,
        repo_local_emitted_artifact_refs: _,
        compare_floor_refs: _,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = vertical_slice;

    let CurrentL2SourceSampleMinimalCompanionSurface {
        source_report: _,
        surface_status: principal_status,
        surface_guard_reason: principal_guard_reason,
        companion_lines,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = principal_surface;

    let CurrentL2SourceSampleStageBlockSurface {
        source_report: _,
        surface_status: secondary_status,
        surface_guard_reason: secondary_guard_reason,
        stage_lines,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = secondary_surface;

    let CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold {
        source_report: _,
        threshold_status: artifact_status,
        threshold_guard_reason: artifact_guard_reason,
        profile_axis_refs: _,
        witness_attachment_refs: _,
        provider_attachment_refs: _,
        emitted_artifact_reserve_refs,
        threshold_default_refs: _,
        compare_floor_refs: _,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = artifact_threshold;

    let threshold_status = if slice_status == CurrentL2EmittedArtifactRouteStatus::Reached
        && principal_status == CurrentL2EmittedArtifactRouteStatus::Reached
        && secondary_status == CurrentL2EmittedArtifactRouteStatus::Reached
        && artifact_status == CurrentL2EmittedArtifactRouteStatus::Reached
    {
        CurrentL2EmittedArtifactRouteStatus::Reached
    } else {
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    };

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = slice_guard_reason
            .or(principal_guard_reason)
            .or(secondary_guard_reason)
            .or(artifact_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "authoritative-room vertical slice, principal surface, secondary surface, or witness/provider threshold was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current order-handoff surface/artifact threshold only actualizes reached authoritative-room defaults: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            threshold_guard_reason: Some(guard_reason),
            profile_axis_refs: Vec::new(),
            principal_surface_lines: Vec::new(),
            secondary_surface_lines: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            threshold_default_refs: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.order_handoff.surface_artifact_threshold_guard_only"
                    .to_string(),
            ],
            contrast_refs: order_handoff_surface_artifact_threshold_contrast_refs(),
            guard_refs: vec!["guard:surface_artifact_threshold_not_reached".to_string()],
            kept_later_refs: order_handoff_surface_artifact_threshold_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold {
        source_report,
        threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        threshold_guard_reason: None,
        profile_axis_refs,
        principal_surface_lines: companion_lines,
        secondary_surface_lines: stage_lines,
        repo_local_emitted_artifact_refs: emitted_artifact_reserve_refs,
        threshold_default_refs: order_handoff_surface_artifact_threshold_default_refs(),
        compare_floor_refs: order_handoff_surface_artifact_threshold_compare_floor_refs(),
        contrast_refs: order_handoff_surface_artifact_threshold_contrast_refs(),
        guard_refs: order_handoff_surface_artifact_threshold_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: order_handoff_surface_artifact_threshold_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_order_handoff_surface_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption, String> {
    let threshold = build_current_l2_source_sample_order_handoff_surface_artifact_threshold(
        sample_argument,
        host_plan,
    )?;

    let CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold {
        source_report,
        threshold_status,
        threshold_guard_reason,
        profile_axis_refs,
        principal_surface_lines,
        secondary_surface_lines,
        repo_local_emitted_artifact_refs,
        threshold_default_refs: _,
        compare_floor_refs,
        contrast_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = threshold;

    if threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = threshold_guard_reason.unwrap_or_else(|| {
            format!(
                "order-handoff surface/artifact threshold route was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current order-handoff surface actual adoption only actualizes reached threshold routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            actualization_guard_reason: Some(guard_reason),
            profile_axis_refs,
            principal_surface_lines,
            secondary_surface_lines,
            repo_local_emitted_artifact_refs,
            actual_adoption_default_refs: Vec::new(),
            compare_floor_refs: order_handoff_surface_actual_adoption_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                Vec::new(),
            ),
            guard_refs: order_handoff_surface_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: order_handoff_surface_artifact_threshold_kept_later_refs(),
        });
    }

    Ok(CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption {
        source_report,
        actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        actualization_guard_reason: None,
        profile_axis_refs,
        principal_surface_lines,
        secondary_surface_lines,
        repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: order_handoff_surface_actual_adoption_default_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        compare_floor_refs: order_handoff_surface_actual_adoption_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
            compare_floor_refs,
        ),
        guard_refs: order_handoff_surface_actual_adoption_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: order_handoff_surface_artifact_threshold_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate, String> {
    let public_shape_actual_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let order_handoff_surface_actual_adoption =
        build_current_l2_source_sample_order_handoff_surface_actual_adoption(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption {
        source_report,
        actualization_status,
        actualization_guard_reason,
        profile_axis_refs,
        witness_route_refs,
        provider_route_refs,
        repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = public_shape_actual_adoption;

    let CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption {
        source_report: _,
        actualization_status: order_handoff_status,
        actualization_guard_reason: _,
        profile_axis_refs: _,
        principal_surface_lines: _,
        secondary_surface_lines: _,
        repo_local_emitted_artifact_refs: _,
        actual_adoption_default_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = order_handoff_surface_actual_adoption;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = actualization_guard_reason.unwrap_or_else(|| {
            format!(
                "witness/provider public-shape actual adoption was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current witness/provider emitted-contract coupled later gate only actualizes reached witness/provider public-shape routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
                source_report,
                coupled_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                coupled_guard_reason: Some(guard_reason),
                profile_axis_refs,
                repo_local_emitted_artifact_refs: Vec::new(),
                witness_contract_candidate_refs: Vec::new(),
                provider_contract_candidate_refs: Vec::new(),
                emitted_contract_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate_guard_only"
                        .to_string(),
                ],
                guard_refs:
                    witness_provider_emitted_contract_coupled_later_gate_guard_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                kept_later_refs:
                    witness_provider_emitted_contract_coupled_later_gate_kept_later_refs(),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    let emitted_contract_candidate_refs =
        witness_provider_emitted_contract_coupled_later_gate_emitted_refs(
            &sample_id,
            &repo_local_emitted_artifact_refs,
        );
    Ok(
        CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
            source_report,
            coupled_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            coupled_guard_reason: None,
            profile_axis_refs,
            repo_local_emitted_artifact_refs,
            witness_contract_candidate_refs:
                witness_provider_emitted_contract_coupled_later_gate_witness_refs(
                    &sample_id,
                    &witness_route_refs,
                ),
            provider_contract_candidate_refs:
                witness_provider_emitted_contract_coupled_later_gate_provider_refs(
                    &sample_id,
                    &provider_route_refs,
                ),
            emitted_contract_candidate_refs,
            coupled_default_refs: witness_provider_emitted_contract_coupled_later_gate_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            compare_floor_refs:
                witness_provider_emitted_contract_coupled_later_gate_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    compare_floor_refs,
                    order_handoff_status,
                ),
            guard_refs: witness_provider_emitted_contract_coupled_later_gate_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: witness_provider_emitted_contract_coupled_later_gate_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge, String> {
    let route_actual_adoption =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let public_contract =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderRouteActualAdoption {
        source_report,
        actualization_status: route_status,
        actualization_guard_reason: route_guard_reason,
        profile_axis_refs: route_profile_axis_refs,
        witness_route_actual_refs: _,
        provider_route_actual_refs: _,
        schema_candidate_keep_refs: _,
        repo_local_emitted_artifact_refs: route_repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs: route_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = route_actual_adoption;

    let CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
        source_report: coupled_source_report,
        coupled_status,
        coupled_guard_reason,
        profile_axis_refs: coupled_profile_axis_refs,
        repo_local_emitted_artifact_refs: coupled_repo_local_emitted_artifact_refs,
        witness_contract_candidate_refs: _,
        provider_contract_candidate_refs: _,
        emitted_contract_candidate_refs,
        coupled_default_refs: _,
        compare_floor_refs: coupled_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = public_contract;

    if route_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || coupled_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = route_guard_reason
            .or(coupled_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "witness/provider route actual adoption or emitted-contract coupled-later gate was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current witness/provider emitted-contract trace alignment bridge only actualizes reached route and coupled-later floors: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge {
            source_report,
            alignment_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            alignment_guard_reason: Some(guard_reason),
            profile_axis_refs: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            route_pair_refs: Vec::new(),
            emitted_contract_pair_refs: Vec::new(),
            matched_pair_refs: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge_guard_only"
                    .to_string(),
            ],
            guard_refs: witness_provider_emitted_contract_trace_alignment_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs:
                witness_provider_emitted_contract_trace_alignment_kept_later_refs(),
        });
    }

    if route_profile_axis_refs != coupled_profile_axis_refs {
        return Err(format!(
            "witness/provider emitted-contract trace alignment profile-axis drift for `{}`",
            source_report.sample_id
        ));
    }

    if route_repo_local_emitted_artifact_refs != coupled_repo_local_emitted_artifact_refs {
        return Err(format!(
            "witness/provider emitted-contract trace alignment repo-local emitted artifact drift for `{}`",
            source_report.sample_id
        ));
    }

    if source_report.sample_id != coupled_source_report.sample_id {
        return Err(format!(
            "witness/provider emitted-contract trace alignment sample mismatch: route=`{}`, coupled=`{}`",
            source_report.sample_id, coupled_source_report.sample_id
        ));
    }

    let sample_id = source_report.sample_id.clone();
    let route_pair_refs = witness_provider_emitted_contract_trace_alignment_route_pair_refs(
        &sample_id,
        &route_repo_local_emitted_artifact_refs,
    )?;
    let emitted_contract_pair_refs =
        witness_provider_emitted_contract_trace_alignment_emitted_pair_refs(
            &sample_id,
            &coupled_repo_local_emitted_artifact_refs,
            &emitted_contract_candidate_refs,
        )?;
    let matched_pair_refs = witness_provider_emitted_contract_trace_alignment_matched_pair_refs(
        &route_pair_refs,
        &emitted_contract_pair_refs,
    )?;

    Ok(
        CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge {
            source_report,
            alignment_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            alignment_guard_reason: None,
            profile_axis_refs: route_profile_axis_refs,
            repo_local_emitted_artifact_refs: route_repo_local_emitted_artifact_refs,
            route_pair_refs,
            emitted_contract_pair_refs,
            matched_pair_refs,
            compare_floor_refs:
                witness_provider_emitted_contract_trace_alignment_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    route_compare_floor_refs,
                    coupled_compare_floor_refs,
                ),
            guard_refs: witness_provider_emitted_contract_trace_alignment_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: witness_provider_emitted_contract_trace_alignment_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate, String> {
    let public_contract =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
        source_report,
        coupled_status,
        coupled_guard_reason,
        profile_axis_refs,
        repo_local_emitted_artifact_refs: _,
        witness_contract_candidate_refs,
        provider_contract_candidate_refs,
        emitted_contract_candidate_refs,
        coupled_default_refs: _,
        compare_floor_refs: public_contract_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = public_contract;

    if coupled_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        let guard_detail = coupled_guard_reason.unwrap_or_else(|| {
            format!(
                "witness/provider emitted-contract coupled later gate was not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current witness/provider public-schema coupled later gate only actualizes reached witness/provider emitted-contract routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate {
                source_report,
                coupled_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                coupled_guard_reason: Some(guard_reason),
                profile_axis_refs,
                witness_schema_candidate_refs: Vec::new(),
                provider_receipt_candidate_refs: Vec::new(),
                combined_public_contract_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                compare_floor_refs:
                    witness_provider_public_schema_coupled_later_gate_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        public_contract_compare_floor_refs,
                    ),
                guard_refs: witness_provider_public_schema_coupled_later_gate_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: witness_provider_public_schema_coupled_later_gate_kept_later_refs(
                ),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    Ok(
        CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate {
            source_report,
            coupled_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            coupled_guard_reason: None,
            profile_axis_refs,
            witness_schema_candidate_refs:
                witness_provider_public_schema_coupled_later_gate_witness_schema_refs(
                    &sample_id,
                    &witness_contract_candidate_refs,
                ),
            provider_receipt_candidate_refs:
                witness_provider_public_schema_coupled_later_gate_provider_receipt_refs(
                    &sample_id,
                    &provider_contract_candidate_refs,
                ),
            combined_public_contract_candidate_refs:
                witness_provider_public_schema_coupled_later_gate_combined_contract_refs(
                    &sample_id,
                    &emitted_contract_candidate_refs,
                ),
            coupled_default_refs: witness_provider_public_schema_coupled_later_gate_default_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            compare_floor_refs:
                witness_provider_public_schema_coupled_later_gate_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    public_contract_compare_floor_refs,
                ),
            guard_refs: witness_provider_public_schema_coupled_later_gate_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: witness_provider_public_schema_coupled_later_gate_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_witness_provider_route_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderRouteActualAdoption, String> {
    let public_shape_actual_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let public_schema_coupled_gate =
        build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption {
        source_report,
        actualization_status: public_shape_status,
        actualization_guard_reason: public_shape_guard_reason,
        profile_axis_refs,
        witness_route_refs,
        provider_route_refs,
        actual_adoption_default_refs: _,
        repo_local_emitted_artifact_refs,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = public_shape_actual_adoption;

    let CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate {
        source_report: _,
        coupled_status: public_schema_status,
        coupled_guard_reason: public_schema_guard_reason,
        profile_axis_refs: _,
        witness_schema_candidate_refs,
        provider_receipt_candidate_refs,
        combined_public_contract_candidate_refs,
        coupled_default_refs: _,
        compare_floor_refs: public_schema_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = public_schema_coupled_gate;

    if public_shape_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || public_schema_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = public_schema_guard_reason.or(public_shape_guard_reason).unwrap_or_else(|| {
            format!(
                "witness/provider public-shape + public-schema prior floors were not reached for `{}`",
                source_report.sample_id
            )
        });
        let guard_reason = format!(
            "current witness/provider route actual adoption only actualizes reached witness/provider public-shape + public-schema routes: {guard_detail}"
        );

        return Ok(CurrentL2SourceSampleWitnessProviderRouteActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            actualization_guard_reason: Some(guard_reason),
            profile_axis_refs,
            witness_route_actual_refs: Vec::new(),
            provider_route_actual_refs: Vec::new(),
            schema_candidate_keep_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            actual_adoption_default_refs: Vec::new(),
            compare_floor_refs: witness_provider_route_actual_adoption_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                public_schema_compare_floor_refs,
            ),
            guard_refs: witness_provider_route_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            ),
            kept_later_refs: witness_provider_route_actual_adoption_kept_later_refs(),
        });
    }

    let sample_id = source_report.sample_id.clone();
    Ok(CurrentL2SourceSampleWitnessProviderRouteActualAdoption {
        source_report,
        actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
        actualization_guard_reason: None,
        profile_axis_refs,
        witness_route_actual_refs: witness_provider_route_actual_adoption_witness_route_refs(
            &sample_id,
            &witness_route_refs,
        ),
        provider_route_actual_refs: witness_provider_route_actual_adoption_provider_route_refs(
            &sample_id,
            &provider_route_refs,
        ),
        schema_candidate_keep_refs: witness_provider_route_actual_adoption_schema_keep_refs(
            &sample_id,
            &witness_schema_candidate_refs,
            &provider_receipt_candidate_refs,
            &combined_public_contract_candidate_refs,
        ),
        repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: witness_provider_route_actual_adoption_default_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        compare_floor_refs: witness_provider_route_actual_adoption_compare_floor_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
            public_schema_compare_floor_refs,
        ),
        guard_refs: witness_provider_route_actual_adoption_guard_refs(
            CurrentL2EmittedArtifactRouteStatus::Reached,
        ),
        kept_later_refs: witness_provider_route_actual_adoption_kept_later_refs(),
    })
}

pub fn build_current_l2_source_sample_witness_provider_schema_route_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption, String> {
    let route_actual_adoption =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let public_schema_coupled_gate =
        build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderRouteActualAdoption {
        source_report,
        actualization_status: route_status,
        actualization_guard_reason: route_guard_reason,
        profile_axis_refs,
        witness_route_actual_refs,
        provider_route_actual_refs,
        schema_candidate_keep_refs: _,
        repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs: route_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = route_actual_adoption;

    let CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate {
        source_report: _,
        coupled_status: public_schema_status,
        coupled_guard_reason: public_schema_guard_reason,
        profile_axis_refs: _,
        witness_schema_candidate_refs,
        provider_receipt_candidate_refs,
        combined_public_contract_candidate_refs,
        coupled_default_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = public_schema_coupled_gate;

    if route_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || public_schema_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = public_schema_guard_reason
            .or(route_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "witness/provider route + public-schema prior floors were not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current witness/provider schema route actual adoption only actualizes reached witness/provider route + public-schema routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                profile_axis_refs,
                repo_local_emitted_artifact_refs,
                witness_schema_route_refs: Vec::new(),
                provider_receipt_route_refs: Vec::new(),
                combined_public_contract_keep_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                compare_floor_refs:
                    witness_provider_schema_route_actual_adoption_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        route_compare_floor_refs,
                    ),
                guard_refs: witness_provider_schema_route_actual_adoption_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: witness_provider_schema_route_actual_adoption_kept_later_refs(),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    Ok(
        CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            profile_axis_refs,
            repo_local_emitted_artifact_refs,
            witness_schema_route_refs: witness_provider_schema_route_actual_adoption_witness_refs(
                &sample_id,
                &witness_route_actual_refs,
                &witness_schema_candidate_refs,
            ),
            provider_receipt_route_refs:
                witness_provider_schema_route_actual_adoption_provider_refs(
                    &sample_id,
                    &provider_route_actual_refs,
                    &provider_receipt_candidate_refs,
                ),
            combined_public_contract_keep_refs:
                witness_provider_schema_route_actual_adoption_combined_keep_refs(
                    &sample_id,
                    &combined_public_contract_candidate_refs,
                ),
            actual_adoption_default_refs:
                witness_provider_schema_route_actual_adoption_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            compare_floor_refs: witness_provider_schema_route_actual_adoption_compare_floor_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
                route_compare_floor_refs,
            ),
            guard_refs: witness_provider_schema_route_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: witness_provider_schema_route_actual_adoption_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold, String> {
    let schema_route_actual_adoption =
        build_current_l2_source_sample_witness_provider_schema_route_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let emitted_contract_coupled_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption {
        source_report,
        actualization_status: schema_route_status,
        actualization_guard_reason: schema_route_guard_reason,
        profile_axis_refs,
        repo_local_emitted_artifact_refs,
        witness_schema_route_refs,
        provider_receipt_route_refs,
        combined_public_contract_keep_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs: schema_route_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = schema_route_actual_adoption;

    let CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
        source_report: _,
        coupled_status: emitted_contract_status,
        coupled_guard_reason: emitted_contract_guard_reason,
        profile_axis_refs: _,
        repo_local_emitted_artifact_refs: _,
        witness_contract_candidate_refs: _,
        provider_contract_candidate_refs: _,
        emitted_contract_candidate_refs: _,
        coupled_default_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = emitted_contract_coupled_gate;

    if schema_route_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || emitted_contract_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = emitted_contract_guard_reason
            .or(schema_route_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "witness/provider schema route + emitted-contract prior floors were not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current final public contract reopen threshold only actualizes reached witness/provider schema route + emitted-contract prior floors: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold {
                source_report,
                threshold_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                threshold_guard_reason: Some(guard_reason),
                profile_axis_refs,
                repo_local_emitted_artifact_refs,
                witness_schema_route_refs,
                provider_receipt_route_refs,
                combined_public_contract_keep_refs,
                final_public_contract_reopen_sequence_refs: Vec::new(),
                threshold_default_refs: Vec::new(),
                compare_floor_refs:
                    witness_provider_final_public_contract_reopen_threshold_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        schema_route_compare_floor_refs,
                    ),
                guard_refs: witness_provider_final_public_contract_reopen_threshold_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs:
                    witness_provider_final_public_contract_reopen_threshold_kept_later_refs(),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    Ok(
        CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold {
            source_report,
            threshold_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            threshold_guard_reason: None,
            profile_axis_refs,
            repo_local_emitted_artifact_refs,
            witness_schema_route_refs: witness_schema_route_refs.clone(),
            provider_receipt_route_refs: provider_receipt_route_refs.clone(),
            combined_public_contract_keep_refs: combined_public_contract_keep_refs.clone(),
            final_public_contract_reopen_sequence_refs:
                witness_provider_final_public_contract_reopen_threshold_sequence_refs(
                    &sample_id,
                    &witness_schema_route_refs,
                    &provider_receipt_route_refs,
                    &combined_public_contract_keep_refs,
                ),
            threshold_default_refs:
                witness_provider_final_public_contract_reopen_threshold_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            compare_floor_refs:
                witness_provider_final_public_contract_reopen_threshold_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    schema_route_compare_floor_refs,
                ),
            guard_refs: witness_provider_final_public_contract_reopen_threshold_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs:
                witness_provider_final_public_contract_reopen_threshold_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_order_handoff_source_wording_emitted_artifact_coupled_later_gate(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate, String> {
    let surface_actual_adoption =
        build_current_l2_source_sample_order_handoff_surface_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let witness_provider_coupled_later_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption {
        source_report,
        actualization_status,
        actualization_guard_reason,
        profile_axis_refs,
        principal_surface_lines,
        secondary_surface_lines,
        repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = surface_actual_adoption;

    let CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate {
        source_report: _,
        coupled_status: witness_provider_status,
        coupled_guard_reason: witness_provider_guard_reason,
        profile_axis_refs: _,
        repo_local_emitted_artifact_refs: _,
        witness_contract_candidate_refs: _,
        provider_contract_candidate_refs: _,
        emitted_contract_candidate_refs,
        coupled_default_refs: _,
        compare_floor_refs: witness_provider_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = witness_provider_coupled_later_gate;

    if actualization_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || witness_provider_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = actualization_guard_reason
            .or(witness_provider_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "order-handoff surface actual adoption or witness/provider emitted-contract coupled later gate was not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current order-handoff source wording / emitted-artifact coupled later gate only actualizes reached surface and emitted-contract routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate {
                source_report,
                coupled_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                coupled_guard_reason: Some(guard_reason),
                profile_axis_refs,
                repo_local_emitted_artifact_refs,
                source_wording_candidate_refs: Vec::new(),
                emitted_artifact_schema_candidate_refs: Vec::new(),
                coupled_default_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.order_handoff.source_wording_emitted_artifact.guard_only"
                        .to_string(),
                ],
                guard_refs:
                    order_handoff_source_wording_emitted_artifact_coupled_later_gate_guard_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                    ),
                kept_later_refs:
                    order_handoff_source_wording_emitted_artifact_coupled_later_gate_kept_later_refs(),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    Ok(
        CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate {
            source_report,
            coupled_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            coupled_guard_reason: None,
            profile_axis_refs,
            repo_local_emitted_artifact_refs: repo_local_emitted_artifact_refs.clone(),
            source_wording_candidate_refs:
                order_handoff_source_wording_emitted_artifact_coupled_later_gate_source_wording_refs(
                    &sample_id,
                    &principal_surface_lines,
                    &secondary_surface_lines,
                ),
            emitted_artifact_schema_candidate_refs:
                order_handoff_source_wording_emitted_artifact_coupled_later_gate_emitted_schema_refs(
                    &sample_id,
                    &repo_local_emitted_artifact_refs,
                    &emitted_contract_candidate_refs,
                ),
            coupled_default_refs:
                order_handoff_source_wording_emitted_artifact_coupled_later_gate_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            compare_floor_refs:
                order_handoff_source_wording_emitted_artifact_coupled_later_gate_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    compare_floor_refs,
                    witness_provider_compare_floor_refs,
                ),
            guard_refs: order_handoff_source_wording_emitted_artifact_coupled_later_gate_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs:
                order_handoff_source_wording_emitted_artifact_coupled_later_gate_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption, String> {
    let surface_actual_adoption =
        build_current_l2_source_sample_order_handoff_surface_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let coupled_gate =
        build_current_l2_source_sample_order_handoff_source_wording_emitted_artifact_coupled_later_gate(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption {
        source_report,
        actualization_status: surface_status,
        actualization_guard_reason: surface_guard_reason,
        profile_axis_refs,
        principal_surface_lines,
        secondary_surface_lines,
        repo_local_emitted_artifact_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs: _,
        guard_refs: _,
        kept_later_refs: _,
    } = surface_actual_adoption;

    let CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate {
        source_report: _,
        coupled_status,
        coupled_guard_reason,
        profile_axis_refs: _,
        repo_local_emitted_artifact_refs: _,
        source_wording_candidate_refs,
        emitted_artifact_schema_candidate_refs,
        coupled_default_refs: _,
        compare_floor_refs: coupled_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = coupled_gate;

    if surface_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || coupled_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = coupled_guard_reason
            .or(surface_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "order-handoff surface + source-wording prior floors were not reached for `{}`",
                    source_report.sample_id
                )
            });
        let guard_reason = format!(
            "current order-handoff source wording route actual adoption only actualizes reached order-handoff surface + source-wording routes: {guard_detail}"
        );

        return Ok(
            CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption {
                source_report,
                actualization_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                actualization_guard_reason: Some(guard_reason),
                profile_axis_refs,
                repo_local_emitted_artifact_refs,
                source_wording_route_refs: Vec::new(),
                emitted_artifact_candidate_keep_refs: Vec::new(),
                actual_adoption_default_refs: Vec::new(),
                compare_floor_refs:
                    order_handoff_source_wording_route_actual_adoption_compare_floor_refs(
                        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                        coupled_compare_floor_refs,
                    ),
                guard_refs: order_handoff_source_wording_route_actual_adoption_guard_refs(
                    CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
                ),
                kept_later_refs: order_handoff_source_wording_route_actual_adoption_kept_later_refs(
                ),
            },
        );
    }

    let sample_id = source_report.sample_id.clone();
    Ok(
        CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption {
            source_report,
            actualization_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            actualization_guard_reason: None,
            profile_axis_refs,
            repo_local_emitted_artifact_refs,
            source_wording_route_refs:
                order_handoff_source_wording_route_actual_adoption_source_wording_refs(
                    &sample_id,
                    &principal_surface_lines,
                    &secondary_surface_lines,
                    &source_wording_candidate_refs,
                ),
            emitted_artifact_candidate_keep_refs:
                order_handoff_source_wording_route_actual_adoption_emitted_keep_refs(
                    &sample_id,
                    &emitted_artifact_schema_candidate_refs,
                ),
            actual_adoption_default_refs:
                order_handoff_source_wording_route_actual_adoption_default_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                ),
            compare_floor_refs:
                order_handoff_source_wording_route_actual_adoption_compare_floor_refs(
                    CurrentL2EmittedArtifactRouteStatus::Reached,
                    coupled_compare_floor_refs,
                ),
            guard_refs: order_handoff_source_wording_route_actual_adoption_guard_refs(
                CurrentL2EmittedArtifactRouteStatus::Reached,
            ),
            kept_later_refs: order_handoff_source_wording_route_actual_adoption_kept_later_refs(),
        },
    )
}

pub fn build_current_l2_source_sample_order_handoff_witness_provider_public_seam_compression(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleOrderHandoffWitnessProviderPublicSeamCompression, String> {
    let source_wording_route =
        build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
            sample_argument,
            host_plan.clone(),
        )?;
    let serial_scope_reserve =
        build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
            sample_argument,
            host_plan.clone(),
        )?;
    let witness_trace_alignment =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
            sample_argument,
            host_plan.clone(),
        )?;
    let witness_final_threshold =
        build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold(
            sample_argument,
            host_plan,
        )?;

    let CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption {
        source_report,
        actualization_status: route_status,
        actualization_guard_reason: route_guard_reason,
        profile_axis_refs: route_profile_axis_refs,
        repo_local_emitted_artifact_refs: route_repo_local_emitted_artifact_refs,
        source_wording_route_refs,
        emitted_artifact_candidate_keep_refs,
        actual_adoption_default_refs: _,
        compare_floor_refs: route_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = source_wording_route;

    let CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface {
        source_report: serial_source_report,
        surface_status: serial_status,
        surface_guard_reason: serial_guard_reason,
        profile_axis_refs: serial_profile_axis_refs,
        repo_local_emitted_artifact_refs: serial_repo_local_emitted_artifact_refs,
        serial_scope_lines,
        compare_floor_refs: serial_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = serial_scope_reserve;

    let CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge {
        source_report: trace_source_report,
        alignment_status: trace_status,
        alignment_guard_reason: trace_guard_reason,
        profile_axis_refs: trace_profile_axis_refs,
        repo_local_emitted_artifact_refs: trace_repo_local_emitted_artifact_refs,
        route_pair_refs: _,
        emitted_contract_pair_refs: _,
        matched_pair_refs,
        compare_floor_refs: trace_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = witness_trace_alignment;

    let CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold {
        source_report: threshold_source_report,
        threshold_status,
        threshold_guard_reason,
        profile_axis_refs: threshold_profile_axis_refs,
        repo_local_emitted_artifact_refs: threshold_repo_local_emitted_artifact_refs,
        witness_schema_route_refs,
        provider_receipt_route_refs,
        combined_public_contract_keep_refs,
        final_public_contract_reopen_sequence_refs: _,
        threshold_default_refs: _,
        compare_floor_refs: threshold_compare_floor_refs,
        guard_refs: _,
        kept_later_refs: _,
    } = witness_final_threshold;

    if route_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || serial_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || trace_status != CurrentL2EmittedArtifactRouteStatus::Reached
        || threshold_status != CurrentL2EmittedArtifactRouteStatus::Reached
    {
        let guard_detail = route_guard_reason
            .or(serial_guard_reason)
            .or(trace_guard_reason)
            .or(threshold_guard_reason)
            .unwrap_or_else(|| {
                format!(
                    "order-handoff route/serial or witness/provider bridge/threshold was not reached for `{}`",
                    source_report.sample_id
                )
            });

        return Ok(CurrentL2SourceSampleOrderHandoffWitnessProviderPublicSeamCompression {
            source_report,
            compression_status: CurrentL2EmittedArtifactRouteStatus::GuardedNotReached,
            compression_guard_reason: Some(format!(
                "current order-handoff/witness-provider public seam compression only actualizes reached route/reserve/bridge/threshold floors: {guard_detail}"
            )),
            profile_axis_refs: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            source_wording_route_refs: Vec::new(),
            emitted_artifact_candidate_keep_refs: Vec::new(),
            serial_scope_lines: Vec::new(),
            witness_schema_route_refs: Vec::new(),
            provider_receipt_route_refs: Vec::new(),
            combined_public_contract_keep_refs: Vec::new(),
            trace_alignment_pair_refs: Vec::new(),
            public_seam_residual_refs: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression.guard_only"
                    .to_string(),
            ],
            guard_refs: vec![
                "guard:order_handoff_witness_provider_public_seam_compression_not_reached"
                    .to_string(),
            ],
            kept_later_refs: vec![
                "kept_later:final_parser_grammar".to_string(),
                "kept_later:final_public_parser_checker_runtime_api".to_string(),
                "kept_later:final_source_surface_handoff_wording".to_string(),
                "kept_later:final_emitted_artifact_schema".to_string(),
                "kept_later:final_public_witness_schema".to_string(),
                "kept_later:final_public_provider_receipt_schema".to_string(),
                "kept_later:delegated_provider_attestation".to_string(),
                "kept_later:combined_provider_witness_public_contract".to_string(),
                "kept_later:final_emitted_handoff_contract".to_string(),
                "kept_later:authoritative_room_serial_scope_sugar".to_string(),
                "kept_later:low_level_memory_order_source_surface".to_string(),
                "kept_later:final_modal_foundation_adoption".to_string(),
                "kept_later:exhaustive_shared_space_catalog".to_string(),
            ],
        });
    }

    if source_report.sample_id != serial_source_report.sample_id
        || source_report.sample_id != trace_source_report.sample_id
        || source_report.sample_id != threshold_source_report.sample_id
    {
        return Err(format!(
            "order-handoff/witness-provider public seam compression expected aligned sample ids but got `{}`, `{}`, `{}`, `{}`",
            source_report.sample_id,
            serial_source_report.sample_id,
            trace_source_report.sample_id,
            threshold_source_report.sample_id
        ));
    }

    let sample_id = source_report.sample_id.clone();
    let mut profile_axis_refs = route_profile_axis_refs;
    extend_unique_refs(&mut profile_axis_refs, serial_profile_axis_refs);
    extend_unique_refs(&mut profile_axis_refs, trace_profile_axis_refs);
    extend_unique_refs(&mut profile_axis_refs, threshold_profile_axis_refs);

    let mut repo_local_emitted_artifact_refs = route_repo_local_emitted_artifact_refs;
    extend_unique_refs(
        &mut repo_local_emitted_artifact_refs,
        serial_repo_local_emitted_artifact_refs,
    );
    extend_unique_refs(
        &mut repo_local_emitted_artifact_refs,
        trace_repo_local_emitted_artifact_refs,
    );
    extend_unique_refs(
        &mut repo_local_emitted_artifact_refs,
        threshold_repo_local_emitted_artifact_refs,
    );

    let mut compare_floor_refs = route_compare_floor_refs;
    extend_unique_refs(&mut compare_floor_refs, serial_compare_floor_refs);
    extend_unique_refs(&mut compare_floor_refs, trace_compare_floor_refs);
    extend_unique_refs(&mut compare_floor_refs, threshold_compare_floor_refs);
    extend_unique_refs(
        &mut compare_floor_refs,
        vec![
            "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression"
                .to_string(),
        ],
    );

    Ok(
        CurrentL2SourceSampleOrderHandoffWitnessProviderPublicSeamCompression {
            source_report,
            compression_status: CurrentL2EmittedArtifactRouteStatus::Reached,
            compression_guard_reason: None,
            profile_axis_refs,
            repo_local_emitted_artifact_refs,
            source_wording_route_refs,
            emitted_artifact_candidate_keep_refs,
            serial_scope_lines,
            witness_schema_route_refs,
            provider_receipt_route_refs,
            combined_public_contract_keep_refs,
            trace_alignment_pair_refs: matched_pair_refs,
            public_seam_residual_refs: vec![
                format!(
                    "order_handoff_public_seam_residual:{sample_id}:final_source_surface_handoff_wording_later"
                ),
                format!(
                    "order_handoff_public_seam_residual:{sample_id}:final_emitted_artifact_schema_later"
                ),
                format!("shared_space_public_seam_residual:{sample_id}:public_schema_pair_first"),
                format!(
                    "shared_space_public_seam_residual:{sample_id}:delegated_attestation_and_combined_contract_second"
                ),
                format!(
                    "shared_space_public_seam_residual:{sample_id}:final_emitted_handoff_contract_third"
                ),
            ],
            compare_floor_refs,
            guard_refs: vec![
                "guard:edge_row_vertical_continuation_principal".to_string(),
                "guard:serial_scope_reserve_surface_only".to_string(),
                "guard:witness_provider_trace_alignment_bridge".to_string(),
                "guard:public_schema_pair_first".to_string(),
                "guard:delegated_attestation_and_combined_contract_second".to_string(),
                "guard:final_source_surface_handoff_wording_later".to_string(),
                "guard:final_emitted_artifact_schema_later".to_string(),
                "guard:final_emitted_handoff_contract_third".to_string(),
            ],
            kept_later_refs: vec![
                "kept_later:final_parser_grammar".to_string(),
                "kept_later:final_public_parser_checker_runtime_api".to_string(),
                "kept_later:final_source_surface_handoff_wording".to_string(),
                "kept_later:final_emitted_artifact_schema".to_string(),
                "kept_later:final_public_witness_schema".to_string(),
                "kept_later:final_public_provider_receipt_schema".to_string(),
                "kept_later:delegated_provider_attestation".to_string(),
                "kept_later:combined_provider_witness_public_contract".to_string(),
                "kept_later:final_emitted_handoff_contract".to_string(),
                "kept_later:authoritative_room_serial_scope_sugar".to_string(),
                "kept_later:low_level_memory_order_source_surface".to_string(),
                "kept_later:final_modal_foundation_adoption".to_string(),
                "kept_later:exhaustive_shared_space_catalog".to_string(),
            ],
        },
    )
}

fn extend_unique_refs(target: &mut Vec<String>, refs: Vec<String>) {
    for reference in refs {
        if !target.contains(&reference) {
            target.push(reference);
        }
    }
}

fn build_formal_hook_for_source_sample(
    source_report: &CurrentL2SourceSampleRunReport,
) -> Result<ToolNeutralFormalHookArtifact, String> {
    let fixture_path = fixture_path(&source_report.sample_id)?;

    match source_report
        .runtime_report
        .checker_floor
        .static_gate
        .verdict
    {
        StaticGateVerdict::Valid => {
            let bundle =
                load_bundle_from_fixture_path(fixture_path).map_err(|error| error.to_string())?;
            let runtime_report = run_bundle(&bundle).map_err(|error| error.to_string())?;
            let detached_bundle = build_detached_bundle_artifact(&bundle, &runtime_report);
            build_formal_hook_from_detached_bundle_artifact(&detached_bundle)
        }
        StaticGateVerdict::Malformed | StaticGateVerdict::Underdeclared => {
            let fixture =
                load_fixture_from_path(&fixture_path).map_err(|error| error.to_string())?;
            let gate = static_gate_detailed(&fixture);
            let detached_static =
                build_detached_static_gate_artifact(fixture_path, &fixture, &gate);
            build_formal_hook_from_static_gate_artifact(&detached_static)
        }
    }
}

fn build_preview_formal_hook_for_source_sample(
    source_report: &CurrentL2SourceSampleRunReport,
) -> Result<ToolNeutralFormalHookArtifact, String> {
    let subject_ref = source_report.sample_id.clone();
    match source_report
        .runtime_report
        .checker_floor
        .static_gate
        .verdict
    {
        StaticGateVerdict::Malformed | StaticGateVerdict::Underdeclared => {
            Ok(ToolNeutralFormalHookArtifact {
                schema_version: FORMAL_HOOK_SCHEMA_VERSION.to_string(),
                artifact_kind: FORMAL_HOOK_ARTIFACT_KIND.to_string(),
                subject_kind: "fixture_static_cluster".to_string(),
                subject_ref: subject_ref.clone(),
                contract_rows: vec![
                    ToolNeutralFormalContractRow {
                        obligation_kind: "canonical_normalization_law".to_string(),
                        evidence_refs: vec![
                            sample_local_evidence_ref("fixture", subject_ref.clone()),
                            sample_local_evidence_ref("static_gate_artifact", subject_ref.clone()),
                        ],
                    },
                    ToolNeutralFormalContractRow {
                        obligation_kind: "no_re_promotion".to_string(),
                        evidence_refs: vec![sample_local_evidence_ref("fixture", subject_ref)],
                    },
                ],
            })
        }
        StaticGateVerdict::Valid => {
            let has_try_cut = source_report
                .runtime_report
                .run_report
                .trace_audit_sink
                .events
                .iter()
                .any(|event| matches!(event, EventKind::Rollback | EventKind::AtomicCut));

            if !has_try_cut {
                return Err(format!(
                    "current sample-local preview route only reaches runtime_try_cut_cluster when rollback or atomic-cut evidence exists for `{}`",
                    source_report.sample_id
                ));
            }

            Ok(ToolNeutralFormalHookArtifact {
                schema_version: FORMAL_HOOK_SCHEMA_VERSION.to_string(),
                artifact_kind: FORMAL_HOOK_ARTIFACT_KIND.to_string(),
                subject_kind: "runtime_try_cut_cluster".to_string(),
                subject_ref: subject_ref.clone(),
                contract_rows: vec![ToolNeutralFormalContractRow {
                    obligation_kind: "rollback_cut_non_interference".to_string(),
                    evidence_refs: vec![
                        sample_local_evidence_ref("fixture", subject_ref.clone()),
                        sample_local_evidence_ref("runtime_cluster", subject_ref),
                    ],
                }],
            })
        }
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

fn sample_local_evidence_ref(ref_kind: &str, ref_id: String) -> ToolNeutralFormalEvidenceRef {
    ToolNeutralFormalEvidenceRef {
        ref_kind: ref_kind.to_string(),
        ref_id,
    }
}

fn small_cluster_projection_ref(subject_kind: &str, subject_ref: &str) -> Result<String, String> {
    let cluster_kind = match subject_kind {
        "fixture_static_cluster" => "fixture_static_rejection",
        "runtime_try_cut_cluster" => "runtime_try_cut_local",
        other => {
            return Err(format!(
                "unsupported model-check small-cluster projection subject_kind {}",
                other
            ));
        }
    };
    Ok(format!(
        "small_cluster_projection:{subject_ref}:{cluster_kind}"
    ))
}

fn model_check_projection_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    subject_kind: Option<&str>,
    _guard_reason: Option<&str>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => match subject_kind {
            Some("fixture_static_cluster") => vec!["guard:static_rejection_cluster".to_string()],
            Some("runtime_try_cut_cluster") => {
                vec!["guard:rollback_or_atomic_cut_evidence_present".to_string()]
            }
            Some(other) => vec![format!("guard:unsupported_subject_kind:{other}")],
            None => vec!["guard:projection_subject_missing".to_string()],
        },
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:projection_not_reached".to_string()]
        }
    }
}

fn theorem_discharge_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    _guard_reason: Option<&str>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            vec!["guard:notebook_consumer_threshold_ready".to_string()]
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:discharge_not_reached".to_string()]
        }
    }
}

fn theorem_first_symbolic_evidence_refs(
    review_units: &[ProofNotebookReviewUnitArtifact],
) -> Vec<String> {
    let mut refs = Vec::new();

    for unit in review_units {
        for evidence in &unit.row.evidence_refs {
            let formatted = format!("{}:{}", evidence.ref_kind, evidence.ref_id);
            if !refs.contains(&formatted) {
                refs.push(formatted);
            }
        }
    }

    refs
}

fn theorem_first_compare_floor_refs(status: CurrentL2EmittedArtifactRouteStatus) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:sample_local_preview_aligned_typed_artifact_route".to_string(),
            "compare_floor:current_l2.theorem_discharge_prefloor".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_guarded_preview_only".to_string()]
        }
    }
}

fn theorem_first_pilot_guard_refs(status: CurrentL2EmittedArtifactRouteStatus) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            vec!["guard:notebook_consumer_threshold_ready".to_string()]
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:pilot_not_reached".to_string()]
        }
    }
}

fn theorem_discharge_actual_format_transport_preview_refs(
    discharge_entry_reserve_refs: &[String],
) -> Result<Vec<String>, String> {
    let mut refs = Vec::new();

    for reserve_ref in discharge_entry_reserve_refs {
        let Some(rest) = reserve_ref.strip_prefix("discharge_entry_reserve:") else {
            return Err(format!(
                "unsupported theorem discharge reserve ref {}",
                reserve_ref
            ));
        };
        let Some((subject_ref, obligation_kind)) = rest.split_once(':') else {
            return Err(format!(
                "theorem discharge reserve ref is missing obligation kind {}",
                reserve_ref
            ));
        };
        refs.push(format!(
            "theorem_discharge_transport_preview:{subject_ref}:{obligation_kind}"
        ));
    }

    Ok(refs)
}

fn theorem_discharge_actual_format_public_contract_preview_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_public_contract_preview:{subject_ref}:review_unit_first"),
            format!("theorem_public_contract_preview:{subject_ref}:discharge_entry_adjacent"),
        ],
        None => Vec::new(),
    }
}

fn theorem_discharge_actual_format_consumer_boundary_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "consumer_boundary:notebook_consumer_first".to_string(),
            "consumer_boundary:abstract_discharge_entry_only".to_string(),
            "consumer_boundary:brand_neutral_contract_probe".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_discharge_actual_format_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_discharge_prefloor".to_string(),
            "compare_floor:current_l2.theorem_discharge.actual_format_probe".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_discharge.actual_format_guard_only".to_string()]
        }
    }
}

fn theorem_discharge_actual_format_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:transport_public_contract_coupled_later_gate".to_string(),
            "guard:brand_neutral_contract_probe_only".to_string(),
            "guard:review_unit_not_theorem_result".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_discharge_actual_format_not_reached".to_string()]
        }
    }
}

fn theorem_discharge_actual_format_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:actual_discharge_transport".to_string(),
        "kept_later:public_theorem_contract".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
    ]
}

fn theorem_binding_preflight_manifest_refs(
    review_unit_refs: &[String],
) -> Result<Vec<String>, String> {
    let mut refs = Vec::new();

    for review_unit_ref in review_unit_refs {
        let Some(rest) = review_unit_ref.strip_prefix("proof_notebook_review_unit:") else {
            return Err(format!(
                "unsupported theorem binding preflight review-unit ref {}",
                review_unit_ref
            ));
        };
        let Some((subject_ref, obligation_kind)) = rest.split_once(':') else {
            return Err(format!(
                "theorem binding preflight review-unit ref is missing obligation kind {}",
                review_unit_ref
            ));
        };
        refs.push(format!(
            "theorem_binding_preflight:{subject_ref}:{obligation_kind}"
        ));
    }

    Ok(refs)
}

fn theorem_binding_adapter_boundary_refs() -> Vec<String> {
    vec![
        "adapter_boundary:principal_review_unit_first".to_string(),
        "adapter_boundary:symbolic_evidence_refs_only".to_string(),
        "adapter_boundary:brand_neutral_theorem_request".to_string(),
    ]
}

fn theorem_binding_preflight_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_first_pilot_actualization".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_binding_guarded_preview_only".to_string()]
        }
    }
}

fn theorem_binding_preflight_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:theorem_first_external_target".to_string(),
            "guard:brand_neutral_preflight_only".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:binding_preflight_not_reached".to_string()]
        }
    }
}

fn theorem_binding_preflight_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:actual_discharge_transport".to_string(),
        "kept_later:public_theorem_contract".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
    ]
}

fn theorem_lean_stub_pilot_binding_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_lean_stub_pilot:{subject_ref}:lean_first_principal"),
            format!("theorem_lean_stub_pilot:{subject_ref}:review_unit_input_only"),
            format!("theorem_lean_stub_pilot:{subject_ref}:nonproduction_stub_only"),
            format!("theorem_lean_stub_pilot:{subject_ref}:rocq_iris_fallback_retained"),
        ],
        None => Vec::new(),
    }
}

fn theorem_lean_stub_pilot_code_anchor_refs() -> Vec<String> {
    vec![
        "code_anchor:current_l2_proof_notebook_review_unit_support".to_string(),
        "code_anchor:current_l2_lean_theorem_stub_support".to_string(),
        "code_anchor:current_l2_emit_lean_theorem_stub".to_string(),
    ]
}

fn theorem_lean_stub_pilot_repo_local_emitted_artifact_refs(
    artifacts: &[LeanTheoremStubArtifact],
) -> Vec<String> {
    artifacts
        .iter()
        .map(|artifact| {
            format!(
                "repo_local_emitted_artifact:lean_theorem_stub:{}:{}",
                artifact.subject_ref, artifact.obligation_kind
            )
        })
        .collect()
}

fn theorem_lean_stub_pilot_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_lean_stub_pilot.guard_only".to_string()]
        }
    }
}

fn theorem_lean_stub_pilot_guard_refs(status: CurrentL2EmittedArtifactRouteStatus) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:lean_first_nonproduction_stub_only".to_string(),
            "guard:review_unit_principal_input".to_string(),
            "guard:no_public_theorem_contract_promotion".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_lean_stub_pilot_not_reached".to_string()]
        }
    }
}

fn theorem_lean_stub_pilot_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:actual_lean_tool_execution".to_string(),
        "kept_later:actual_discharge_transport".to_string(),
        "kept_later:public_theorem_contract".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
        "kept_later:rocq_iris_fallback".to_string(),
    ]
}

fn theorem_trace_alignment_pair_ref(subject_ref: &str, obligation_kind: &str) -> String {
    format!("theorem_trace_alignment_pair:{subject_ref}:{obligation_kind}")
}

fn theorem_lean_stub_trace_alignment_review_unit_pair_refs(
    review_units: &[ProofNotebookReviewUnitArtifact],
) -> Vec<String> {
    let mut refs = review_units
        .iter()
        .map(|review_unit| {
            theorem_trace_alignment_pair_ref(
                &review_unit.subject_ref,
                &review_unit.row.obligation_kind,
            )
        })
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn theorem_lean_stub_trace_alignment_lean_stub_pair_refs(
    artifacts: &[LeanTheoremStubArtifact],
) -> Vec<String> {
    let mut refs = artifacts
        .iter()
        .map(|artifact| {
            theorem_trace_alignment_pair_ref(&artifact.subject_ref, &artifact.obligation_kind)
        })
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn theorem_lean_stub_trace_alignment_matched_pair_refs(
    review_unit_pair_refs: &[String],
    lean_stub_pair_refs: &[String],
) -> Result<Vec<String>, String> {
    if review_unit_pair_refs != lean_stub_pair_refs {
        return Err(format!(
            "theorem Lean stub trace alignment pair drift: review_unit_pair_refs={review_unit_pair_refs:?}, lean_stub_pair_refs={lean_stub_pair_refs:?}"
        ));
    }
    Ok(review_unit_pair_refs.to_vec())
}

fn theorem_lean_stub_trace_alignment_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization".to_string(),
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge.guard_only"
                .to_string(),
        ],
    }
}

fn theorem_lean_stub_trace_alignment_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:repo_local_trace_alignment_only".to_string(),
            "guard:no_actual_lean_execution".to_string(),
            "guard:no_public_theorem_contract_promotion".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_lean_stub_trace_alignment_bridge_not_reached".to_string()]
        }
    }
}

fn theorem_lean_stub_trace_alignment_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:actual_lean_tool_execution".to_string(),
        "kept_later:prototype_wide_trace_alignment".to_string(),
        "kept_later:public_theorem_contract".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:cross_tool_public_artifact_conformance_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_contract_threshold_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_contract_default:review_unit_first".to_string(),
            "theorem_contract_default:discharge_entry_adjacent".to_string(),
            "theorem_contract_default:notebook_consumer_first".to_string(),
            "theorem_contract_default:brand_neutral_theorem_request".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_contract_threshold_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_discharge.actual_format_probe".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_contract_threshold".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_contract_threshold_guard_only".to_string()]
        }
    }
}

fn theorem_contract_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:review_unit_first_threshold_only".to_string(),
            "guard:transport_preview_only".to_string(),
            "guard:public_contract_preview_only".to_string(),
            "guard:brand_neutral_request_only".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_contract_threshold_not_reached".to_string()]
        }
    }
}

fn theorem_contract_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:actual_discharge_transport".to_string(),
        "kept_later:public_theorem_contract".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
    ]
}

fn theorem_contract_shape_transport_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_transport_shape:{subject_ref}:review_unit_refs_only"),
            format!("theorem_transport_shape:{subject_ref}:discharge_entry_refs_only"),
            format!("theorem_transport_shape:{subject_ref}:brand_neutral_request_manifest_only"),
            format!("theorem_transport_shape:{subject_ref}:repo_local_emitted_artifact_refs_first"),
        ],
        None => Vec::new(),
    }
}

fn theorem_contract_shape_public_contract_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_public_contract_shape:{subject_ref}:refs_only_reserve_schema"),
            format!("theorem_public_contract_shape:{subject_ref}:symbolic_evidence_refs_only"),
            format!("theorem_public_contract_shape:{subject_ref}:discharge_entry_adjacent"),
            format!("theorem_public_contract_shape:{subject_ref}:proof_object_later"),
        ],
        None => Vec::new(),
    }
}

fn theorem_contract_shape_threshold_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_shape_threshold_default:refs_only_reserve_schema_first".to_string(),
            "theorem_shape_threshold_default:review_unit_transport_anchor".to_string(),
            "theorem_shape_threshold_default:brand_neutral_request_manifest_keep".to_string(),
            "theorem_shape_threshold_default:consumer_shaped_payload_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_contract_shape_threshold_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_discharge.actual_format_probe".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_contract_threshold".to_string(),
            "compare_floor:current_l2.theorem_contract_shape_threshold".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_contract_shape_threshold_guard_only".to_string()]
        }
    }
}

fn theorem_contract_shape_threshold_contrast_refs() -> Vec<String> {
    vec![
        "contrast_target:consumer_shaped_theorem_payload".to_string(),
        "contrast_target:concrete_theorem_prover_payload".to_string(),
        "contrast_target:proof_object_public_schema".to_string(),
        "contrast_target:source_surface_first_theorem_contract".to_string(),
    ]
}

fn theorem_contract_shape_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:theorem_shape_threshold_only".to_string(),
            "guard:refs_only_reserve_schema_first".to_string(),
            "guard:consumer_shaped_payload_later".to_string(),
            "guard:proof_object_public_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_contract_shape_threshold_not_reached".to_string()]
        }
    }
}

fn theorem_contract_shape_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:actual_discharge_transport".to_string(),
        "kept_later:public_theorem_contract".to_string(),
        "kept_later:theorem_result_public_object".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_transport_contract_coupled_later_gate_transport_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_transport_candidate:{subject_ref}:review_unit_anchor"),
            format!("theorem_transport_candidate:{subject_ref}:discharge_entry_adjacent"),
            format!("theorem_transport_candidate:{subject_ref}:symbolic_evidence_refs_only"),
            format!(
                "theorem_transport_candidate:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_transport_contract_coupled_later_gate_public_contract_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_public_contract_candidate:{subject_ref}:notebook_consumer_adjacent"),
            format!("theorem_public_contract_candidate:{subject_ref}:refs_only_reserve_schema"),
            format!(
                "theorem_public_contract_candidate:{subject_ref}:brand_neutral_request_manifest_keep"
            ),
            format!("theorem_public_contract_candidate:{subject_ref}:consumer_payload_later"),
        ],
        None => Vec::new(),
    }
}

fn theorem_transport_contract_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_coupled_later_gate_default:transport_and_public_contract_adjacent_distinct"
                .to_string(),
            "theorem_coupled_later_gate_default:review_unit_anchor".to_string(),
            "theorem_coupled_later_gate_default:refs_only_reserve_schema_first".to_string(),
            "theorem_coupled_later_gate_default:proof_object_public_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_transport_contract_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_discharge.actual_format_probe".to_string(),
            "compare_floor:current_l2.theorem_contract_threshold".to_string(),
            "compare_floor:current_l2.theorem_contract_shape_threshold".to_string(),
            "compare_floor:current_l2.theorem_transport_contract_coupled_later_gate".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.theorem_transport_contract_coupled_later_gate_guard_only"
                .to_string(),
        ],
    }
}

fn theorem_transport_contract_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:transport_public_contract_adjacent_distinct".to_string(),
            "guard:actual_transport_not_yet_adopted".to_string(),
            "guard:public_contract_not_yet_adopted".to_string(),
            "guard:proof_object_public_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_transport_contract_coupled_later_gate_not_reached".to_string()]
        }
    }
}

fn theorem_transport_contract_coupled_later_gate_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:actual_discharge_transport_adoption".to_string(),
        "kept_later:public_theorem_contract_adoption".to_string(),
        "kept_later:theorem_result_public_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_review_unit_transport_actual_adoption_transport_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_transport_route:{subject_ref}:review_unit_anchor"),
            format!("theorem_transport_route:{subject_ref}:discharge_entry_ref_bundle"),
            format!("theorem_transport_route:{subject_ref}:symbolic_evidence_refs_only"),
            format!("theorem_transport_route:{subject_ref}:repo_local_emitted_artifact_refs"),
        ],
        None => Vec::new(),
    }
}

fn theorem_review_unit_transport_actual_adoption_notebook_contract_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_notebook_contract_route:{subject_ref}:notebook_consumer_first"),
            format!("theorem_notebook_contract_route:{subject_ref}:review_unit_reference_bundle"),
            format!("theorem_notebook_contract_route:{subject_ref}:discharge_entry_adjacent"),
            format!(
                "theorem_notebook_contract_route:{subject_ref}:proof_object_public_schema_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_review_unit_transport_actual_adoption_external_binding_reserve_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "binding_reserve:brand_neutral_request_manifest".to_string(),
            "binding_reserve:adapter_boundary_refs_keep".to_string(),
            "binding_reserve:concrete_theorem_prover_brand_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_review_unit_transport_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_actual_adoption_default:review_unit_transport_first".to_string(),
            "theorem_actual_adoption_default:notebook_consumer_contract_first".to_string(),
            "theorem_actual_adoption_default:transport_contract_adjacent_distinct".to_string(),
            "theorem_actual_adoption_default:proof_object_public_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_review_unit_transport_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_transport_contract.coupled_later_gate".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_review_unit_transport.guard_only".to_string()]
        }
    }
}

fn theorem_review_unit_transport_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:review_unit_transport_actual_adoption_only".to_string(),
            "guard:notebook_contract_actual_adoption_only".to_string(),
            "guard:brand_neutral_binding_reserve_keep".to_string(),
            "guard:proof_object_public_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_review_unit_transport_not_reached".to_string()]
        }
    }
}

fn theorem_review_unit_transport_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:theorem_result_public_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_result_object_preview_actualization_result_object_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_result_object_route:{subject_ref}:notebook_consumer_object_first"),
            format!("theorem_result_object_route:{subject_ref}:review_unit_anchor_bundle"),
            format!(
                "theorem_result_object_route:{subject_ref}:consumer_shaped_payload_preview_only"
            ),
            format!("theorem_result_object_route:{subject_ref}:repo_local_emitted_artifact_refs"),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_object_preview_actualization_payload_preview_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_result_payload_preview:{subject_ref}:notebook_consumer_first"),
            format!("theorem_result_payload_preview:{subject_ref}:review_unit_reference_bundle"),
            format!(
                "theorem_result_payload_preview:{subject_ref}:consumer_shaped_payload_preview_only"
            ),
            format!(
                "theorem_result_payload_preview:{subject_ref}:proof_object_public_schema_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_object_preview_actualization_proof_object_schema_reserve_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "proof_object_schema_reserve:brand_neutral_binding_keep".to_string(),
            "proof_object_schema_reserve:proof_object_public_schema_later".to_string(),
            "proof_object_schema_reserve:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_result_object_preview_actualization_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_result_object_preview_default:notebook_consumer_object_first".to_string(),
            "theorem_result_object_preview_default:consumer_shaped_payload_preview_only"
                .to_string(),
            "theorem_result_object_preview_default:proof_object_schema_reserve_keep".to_string(),
            "theorem_result_object_preview_default:final_public_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_result_object_preview_actualization_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_result_object_preview_actualization".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.theorem_result_object_preview.guard_only".to_string()]
        }
    }
}

fn theorem_result_object_preview_actualization_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:result_object_preview_actualization_only".to_string(),
            "guard:consumer_shaped_payload_preview_only".to_string(),
            "guard:proof_object_schema_reserve_keep".to_string(),
            "guard:concrete_theorem_prover_brand_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_result_object_preview_not_reached".to_string()]
        }
    }
}

fn theorem_result_object_preview_actualization_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_proof_object_schema_prover_brand_coupled_later_gate_proof_object_schema_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_proof_object_schema_candidate:{subject_ref}:result_object_preview_adjacent"
            ),
            format!(
                "theorem_proof_object_schema_candidate:{subject_ref}:refs_only_public_schema_candidate"
            ),
            format!(
                "theorem_proof_object_schema_candidate:{subject_ref}:public_contract_not_adopted"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_proof_object_schema_prover_brand_coupled_later_gate_prover_brand_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_prover_brand_candidate:{subject_ref}:brand_neutral_preflight_anchor"),
            format!("theorem_prover_brand_candidate:{subject_ref}:adapter_boundary_refs_keep"),
            format!("theorem_prover_brand_candidate:{subject_ref}:concrete_brand_not_adopted"),
        ],
        None => Vec::new(),
    }
}

fn theorem_proof_object_schema_prover_brand_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_proof_schema_brand_default:result_object_preview_keep".to_string(),
            "theorem_proof_schema_brand_default:proof_object_schema_candidate_only".to_string(),
            "theorem_proof_schema_brand_default:prover_brand_candidate_only".to_string(),
            "theorem_proof_schema_brand_default:final_public_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_proof_object_schema_prover_brand_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_result_object_preview_actualization".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec![
                "compare_floor:current_l2.theorem_proof_object_schema_prover_brand.guard_only"
                    .to_string(),
            ]
        }
    }
}

fn theorem_proof_object_schema_prover_brand_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:proof_object_schema_candidate_only".to_string(),
            "guard:concrete_theorem_prover_brand_candidate_only".to_string(),
            "guard:final_public_theorem_result_object_later".to_string(),
            "guard:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_proof_object_schema_prover_brand_not_reached".to_string()]
        }
    }
}

fn theorem_proof_object_schema_prover_brand_coupled_later_gate_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_result_payload_public_contract_coupled_later_gate_result_object_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_final_result_candidate:{subject_ref}:notebook_consumer_object_first"),
            format!("theorem_final_result_candidate:{subject_ref}:review_unit_transport_anchor"),
            format!(
                "theorem_final_result_candidate:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "theorem_final_result_candidate:{subject_ref}:final_public_result_object_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_payload_public_contract_coupled_later_gate_payload_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_payload_public_contract_candidate:{subject_ref}:consumer_shaped_payload_preview_keep"
            ),
            format!(
                "theorem_payload_public_contract_candidate:{subject_ref}:notebook_consumer_contract_first"
            ),
            format!(
                "theorem_payload_public_contract_candidate:{subject_ref}:consumer_shaped_payload_candidate_only"
            ),
            format!(
                "theorem_payload_public_contract_candidate:{subject_ref}:proof_object_schema_prover_brand_adjacent_not_collapsed"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_payload_public_contract_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_result_payload_default:notebook_consumer_object_first".to_string(),
            "theorem_result_payload_default:consumer_shaped_payload_candidate_only".to_string(),
            "theorem_result_payload_default:proof_object_schema_prover_brand_adjacent_keep"
                .to_string(),
            "theorem_result_payload_default:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_result_payload_public_contract_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut proof_object_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            proof_object_compare_floor_refs.push(
                "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate"
                    .to_string(),
            );
            proof_object_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.theorem_result_payload_public_contract.guard_only"
                .to_string(),
        ],
    }
}

fn theorem_result_payload_public_contract_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:final_theorem_result_candidate_only".to_string(),
            "guard:consumer_shaped_payload_candidate_only".to_string(),
            "guard:proof_object_schema_prover_brand_adjacent_keep".to_string(),
            "guard:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_result_payload_public_contract_not_reached".to_string()]
        }
    }
}

fn theorem_result_payload_public_contract_coupled_later_gate_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_result_object_actual_adoption_route_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_result_object_actual_route:{subject_ref}:review_unit_transport_first"),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:notebook_consumer_object_first"
            ),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:consumer_shaped_payload_preview_keep"
            ),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:proof_object_schema_prover_brand_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_object_actual_adoption_payload_preview_keep_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_result_object_payload_preview_keep:{subject_ref}:notebook_consumer_first"
            ),
            format!(
                "theorem_result_object_payload_preview_keep:{subject_ref}:consumer_shaped_payload_preview_only"
            ),
            format!(
                "theorem_result_object_payload_preview_keep:{subject_ref}:payload_public_contract_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_object_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_result_object_actual_adoption_default:review_unit_transport_first".to_string(),
            "theorem_result_object_actual_adoption_default:notebook_consumer_object_first"
                .to_string(),
            "theorem_result_object_actual_adoption_default:consumer_shaped_payload_preview_keep"
                .to_string(),
            "theorem_result_object_actual_adoption_default:proof_object_schema_prover_brand_later"
                .to_string(),
            "theorem_result_object_actual_adoption_default:final_public_result_object_later"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_result_object_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption".to_string(),
            "compare_floor:current_l2.theorem_result_object_preview_actualization".to_string(),
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate"
                .to_string(),
            "compare_floor:current_l2.theorem_result_object_actual_adoption".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.theorem_result_object_actual_adoption.guard_only".to_string(),
        ],
    }
}

fn theorem_result_object_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:review_unit_transport_first".to_string(),
            "guard:notebook_consumer_object_first".to_string(),
            "guard:consumer_shaped_payload_preview_keep".to_string(),
            "guard:proof_object_schema_prover_brand_later".to_string(),
            "guard:final_public_theorem_result_object_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_result_object_actual_adoption_not_reached".to_string()]
        }
    }
}

fn theorem_result_object_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn theorem_final_public_contract_reopen_threshold_sequence_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_final_public_contract_reopen:{subject_ref}:result_object_and_payload_first"
            ),
            format!(
                "theorem_final_public_contract_reopen:{subject_ref}:prover_brand_and_proof_schema_second"
            ),
            format!(
                "theorem_final_public_contract_reopen:{subject_ref}:final_public_verifier_contract_third"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_final_public_contract_reopen_threshold_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "theorem_final_public_contract_reopen_default:result_object_and_payload_first"
                .to_string(),
            "theorem_final_public_contract_reopen_default:prover_brand_and_proof_schema_second"
                .to_string(),
            "theorem_final_public_contract_reopen_default:final_public_verifier_contract_third"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn theorem_final_public_contract_reopen_threshold_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    result_object_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = result_object_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold.guard_only"
                .to_string(),
        ],
    }
}

fn theorem_final_public_contract_reopen_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:result_object_and_payload_first".to_string(),
            "guard:prover_brand_and_proof_schema_second".to_string(),
            "guard:final_public_verifier_contract_third".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:theorem_final_public_contract_reopen_threshold_not_reached".to_string()]
        }
    }
}

fn theorem_final_public_contract_reopen_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_second_line_property_preview_refs(
    principal_machine_carrier_refs: &[String],
) -> Result<Vec<String>, String> {
    let mut refs = Vec::new();

    for carrier_ref in principal_machine_carrier_refs {
        let Some(rest) = carrier_ref.strip_prefix("model_check_concrete_carrier:") else {
            return Err(format!(
                "unsupported model-check second-line carrier ref {}",
                carrier_ref
            ));
        };
        let Some((subject_ref, obligation_kind)) = rest.split_once(':') else {
            return Err(format!(
                "model-check second-line carrier ref is missing obligation kind {}",
                carrier_ref
            ));
        };
        refs.push(format!(
            "property_preview:row_local:{subject_ref}:{obligation_kind}"
        ));
    }

    Ok(refs)
}

fn model_check_second_line_request_preflight_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("model_check_request_preflight:{subject_ref}:row_local_property_preview"),
            format!(
                "model_check_request_preflight:{subject_ref}:small_cluster_semantic_projection"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_second_line_public_checker_reserve_refs() -> Vec<String> {
    vec![
        "public_checker_second_reserve:payload_schema".to_string(),
        "public_checker_second_reserve:api_read_relation".to_string(),
        "public_checker_second_reserve:command_surface".to_string(),
        "public_checker_second_reserve:shared_output_contract".to_string(),
        "public_checker_second_reserve:boundary".to_string(),
        "public_checker_second_reserve:verifier_handoff_surface".to_string(),
    ]
}

fn model_check_second_line_repo_local_emitted_artifact_refs(
    model_check_concrete_carriers: &[ModelCheckConcreteCarrierArtifact],
) -> Vec<String> {
    model_check_concrete_carriers
        .iter()
        .map(|carrier| {
            format!(
                "repo_local_emitted_artifact:model_check_concrete_carrier:{}:{}",
                carrier.subject_ref, carrier.case.obligation_kind
            )
        })
        .collect()
}

fn model_check_second_line_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.model_check_projection_prefloor".to_string(),
            "compare_floor:current_l2.model_check.second_line_concretization".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec![
                "compare_floor:current_l2.model_check.second_line_guarded_preview_only".to_string(),
            ]
        }
    }
}

fn model_check_second_line_excluded_family_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "excluded_family:theorem_discharge_transport".to_string(),
            "excluded_family:room_protocol_projection".to_string(),
            "excluded_family:provider_receipt_fairness_family".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_second_line_guard_refs(status: CurrentL2EmittedArtifactRouteStatus) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:row_local_property_preview_only".to_string(),
            "guard:brand_neutral_model_check_request_only".to_string(),
            "guard:keep_public_checker_chain_docs_only".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_second_line_not_reached".to_string()]
        }
    }
}

fn model_check_second_line_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
    ]
}

fn model_check_property_tool_seam_property_language_probe_refs(
    principal_machine_carrier_refs: &[String],
) -> Result<Vec<String>, String> {
    let mut refs = Vec::new();

    for carrier_ref in principal_machine_carrier_refs {
        let Some(rest) = carrier_ref.strip_prefix("model_check_concrete_carrier:") else {
            return Err(format!(
                "unsupported model-check property/tool-seam carrier ref {}",
                carrier_ref
            ));
        };
        let Some((subject_ref, obligation_kind)) = rest.split_once(':') else {
            return Err(format!(
                "model-check property/tool-seam carrier ref is missing obligation kind {}",
                carrier_ref
            ));
        };
        refs.push(format!(
            "property_language_probe:row_local:{subject_ref}:{obligation_kind}"
        ));
    }

    Ok(refs)
}

fn model_check_property_tool_seam_tool_seam_probe_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("tool_seam_probe:{subject_ref}:brand_neutral_model_check_request"),
            format!("tool_seam_probe:{subject_ref}:small_cluster_semantic_projection"),
        ],
        None => Vec::new(),
    }
}

fn model_check_property_tool_seam_checker_boundary_probe_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "checker_boundary_probe:row_local_property_preview_first".to_string(),
            "checker_boundary_probe:brand_neutral_tool_probe_only".to_string(),
            "checker_boundary_probe:public_checker_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_property_tool_seam_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.model_check.second_line_concretization".to_string(),
            "compare_floor:current_l2.model_check.property_tool_seam_probe".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["compare_floor:current_l2.model_check.property_tool_seam_guard_only".to_string()]
        }
    }
}

fn model_check_property_tool_seam_excluded_family_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "excluded_family:theorem_discharge_actual_format".to_string(),
            "excluded_family:room_protocol_projection".to_string(),
            "excluded_family:provider_receipt_fairness_family".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_property_tool_seam_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:settled_property_language_not_adopted".to_string(),
            "guard:brand_neutral_tool_probe_only".to_string(),
            "guard:public_checker_contract_later_gate".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_property_tool_seam_not_reached".to_string()]
        }
    }
}

fn model_check_property_tool_seam_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
    ]
}

fn model_check_property_tool_threshold_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_threshold_default:row_local_property_preview_first".to_string(),
            "model_check_threshold_default:small_cluster_semantic_projection_second".to_string(),
            "model_check_threshold_default:brand_neutral_model_check_request".to_string(),
            "model_check_threshold_default:public_checker_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_property_tool_threshold_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.model_check.property_tool_seam_probe".to_string(),
            "compare_floor:current_l2.model_check.property_tool_threshold".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.property_tool_threshold_guard_only".to_string(),
        ],
    }
}

fn model_check_property_tool_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:row_local_property_preview_threshold_only".to_string(),
            "guard:property_language_probe_only".to_string(),
            "guard:brand_neutral_model_check_request_only".to_string(),
            "guard:public_checker_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_property_tool_threshold_not_reached".to_string()]
        }
    }
}

fn model_check_property_tool_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
    ]
}

fn model_check_row_local_property_actual_adoption_property_route_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("model_check_property_route:{subject_ref}:row_local_preview_bundle"),
            format!("model_check_property_route:{subject_ref}:property_language_probe_bundle"),
            format!("model_check_property_route:{subject_ref}:small_cluster_projection_second"),
            format!("model_check_property_route:{subject_ref}:repo_local_emitted_artifact_refs"),
        ],
        None => Vec::new(),
    }
}

fn model_check_row_local_property_actual_adoption_checker_contract_route_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_checker_contract_route:{subject_ref}:checker_boundary_probe_first"
            ),
            format!(
                "model_check_checker_contract_route:{subject_ref}:public_checker_reserve_bundle"
            ),
            format!(
                "model_check_checker_contract_route:{subject_ref}:public_checker_contract_later"
            ),
            format!(
                "model_check_checker_contract_route:{subject_ref}:verifier_handoff_artifact_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_row_local_property_actual_adoption_tool_binding_reserve_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_tool_binding_reserve:brand_neutral_request_manifest".to_string(),
            "model_check_tool_binding_reserve:concrete_tool_brand_later".to_string(),
            "model_check_tool_binding_reserve:runtime_policy_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_row_local_property_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_actual_adoption_default:row_local_property_route_first".to_string(),
            "model_check_actual_adoption_default:checker_boundary_contract_first".to_string(),
            "model_check_actual_adoption_default:brand_neutral_tool_binding_reserve_keep"
                .to_string(),
            "model_check_actual_adoption_default:public_checker_handoff_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_row_local_property_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.model_check.property_tool_threshold".to_string(),
            "compare_floor:current_l2.model_check.second_line_concretization".to_string(),
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption_guard_only"
                .to_string(),
        ],
    }
}

fn model_check_row_local_property_actual_adoption_excluded_family_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "excluded_family:theorem_discharge_actual_format".to_string(),
            "excluded_family:room_protocol_projection".to_string(),
            "excluded_family:provider_receipt_fairness_family".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_row_local_property_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:row_local_property_actual_adoption_only".to_string(),
            "guard:checker_boundary_contract_actual_adoption_only".to_string(),
            "guard:brand_neutral_tool_binding_reserve_keep".to_string(),
            "guard:public_checker_handoff_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_row_local_property_actual_adoption_not_reached".to_string()]
        }
    }
}

fn model_check_row_local_property_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:consumer_shaped_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_public_checker_artifact_preview_actualization_preview_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_public_checker_preview:{subject_ref}:consumer_shaped_artifact_preview_only"
            ),
            format!("model_check_public_checker_preview:{subject_ref}:checker_boundary_bundle"),
            format!(
                "model_check_public_checker_preview:{subject_ref}:row_local_property_route_bundle"
            ),
            format!(
                "model_check_public_checker_preview:{subject_ref}:repo_local_emitted_artifact_refs"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_public_checker_artifact_preview_actualization_handoff_reserve_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_verifier_handoff_reserve:public_checker_migration_later".to_string(),
            "model_check_verifier_handoff_reserve:emitted_handoff_artifact_later".to_string(),
            "model_check_verifier_handoff_reserve:runtime_policy_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_public_checker_artifact_preview_actualization_tool_binding_reserve_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_tool_binding_reserve:brand_neutral_request_manifest".to_string(),
            "model_check_tool_binding_reserve:concrete_tool_brand_later".to_string(),
            "model_check_tool_binding_reserve:runtime_policy_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_public_checker_artifact_preview_actualization_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_public_checker_preview_default:consumer_shaped_artifact_preview_only"
                .to_string(),
            "model_check_public_checker_preview_default:verifier_handoff_reserve_keep".to_string(),
            "model_check_public_checker_preview_default:brand_neutral_tool_binding_reserve_keep"
                .to_string(),
            "model_check_public_checker_preview_default:runtime_policy_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_public_checker_artifact_preview_actualization_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption".to_string(),
            "compare_floor:current_l2.model_check.second_line_concretization".to_string(),
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.public_checker_artifact_preview.guard_only"
                .to_string(),
        ],
    }
}

fn model_check_public_checker_artifact_preview_actualization_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:public_checker_artifact_preview_actualization_only".to_string(),
            "guard:verifier_handoff_reserve_keep".to_string(),
            "guard:brand_neutral_tool_binding_reserve_keep".to_string(),
            "guard:runtime_policy_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_public_checker_artifact_preview_not_reached".to_string()]
        }
    }
}

fn model_check_public_checker_artifact_preview_actualization_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_tool_brand_verifier_handoff_coupled_later_gate_verifier_handoff_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_verifier_handoff_candidate:{subject_ref}:public_checker_preview_adjacent"
            ),
            format!(
                "model_check_verifier_handoff_candidate:{subject_ref}:emitted_handoff_artifact_candidate"
            ),
            format!(
                "model_check_verifier_handoff_candidate:{subject_ref}:runtime_policy_contract_candidate"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_tool_brand_verifier_handoff_coupled_later_gate_tool_brand_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_tool_brand_candidate:{subject_ref}:brand_neutral_request_manifest_keep"
            ),
            format!("model_check_tool_brand_candidate:{subject_ref}:concrete_tool_brand_candidate"),
            format!(
                "model_check_tool_brand_candidate:{subject_ref}:public_checker_artifact_not_adopted"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_tool_brand_verifier_handoff_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_tool_handoff_default:public_checker_preview_keep".to_string(),
            "model_check_tool_handoff_default:verifier_handoff_candidate_only".to_string(),
            "model_check_tool_handoff_default:tool_brand_candidate_only".to_string(),
            "model_check_tool_handoff_default:final_public_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_tool_brand_verifier_handoff_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization"
                .to_string(),
            "compare_floor:current_l2.model_check.property_tool_threshold".to_string(),
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff.guard_only"
                .to_string(),
        ],
    }
}

fn model_check_tool_brand_verifier_handoff_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:verifier_handoff_candidate_only".to_string(),
            "guard:concrete_tool_brand_candidate_only".to_string(),
            "guard:final_public_checker_artifact_later".to_string(),
            "guard:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_tool_brand_verifier_handoff_not_reached".to_string()]
        }
    }
}

fn model_check_tool_brand_verifier_handoff_coupled_later_gate_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_public_checker_artifact_migration_coupled_later_gate_public_checker_artifact_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_final_public_checker_candidate:{subject_ref}:consumer_shaped_artifact_preview_keep"
            ),
            format!(
                "model_check_final_public_checker_candidate:{subject_ref}:checker_boundary_contract_anchor"
            ),
            format!(
                "model_check_final_public_checker_candidate:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "model_check_final_public_checker_candidate:{subject_ref}:final_public_checker_artifact_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_public_checker_artifact_migration_coupled_later_gate_checker_migration_candidate_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_checker_migration_candidate:{subject_ref}:verifier_handoff_candidate_keep"
            ),
            format!(
                "model_check_checker_migration_candidate:{subject_ref}:tool_brand_candidate_adjacent_keep"
            ),
            format!(
                "model_check_checker_migration_candidate:{subject_ref}:actual_public_checker_migration_candidate_only"
            ),
            format!(
                "model_check_checker_migration_candidate:{subject_ref}:runtime_policy_contract_adjacent_not_collapsed"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_public_checker_artifact_migration_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_public_checker_default:consumer_shaped_artifact_candidate_only"
                .to_string(),
            "model_check_public_checker_default:migration_candidate_only".to_string(),
            "model_check_public_checker_default:tool_brand_verifier_handoff_adjacent_keep"
                .to_string(),
            "model_check_public_checker_default:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_public_checker_artifact_migration_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    tool_handoff_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = tool_handoff_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.public_checker_artifact_migration.guard_only"
                .to_string(),
        ],
    }
}

fn model_check_public_checker_artifact_migration_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:consumer_shaped_public_checker_candidate_only".to_string(),
            "guard:actual_public_checker_migration_candidate_only".to_string(),
            "guard:tool_brand_verifier_handoff_adjacent_keep".to_string(),
            "guard:final_public_verifier_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_public_checker_artifact_migration_not_reached".to_string()]
        }
    }
}

fn model_check_public_checker_artifact_migration_coupled_later_gate_kept_later_refs() -> Vec<String>
{
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_checker_artifact_route_actual_adoption_route_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:row_local_property_route_first"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:checker_boundary_contract_anchor"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:consumer_shaped_checker_artifact_candidate_only"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:final_public_checker_artifact_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_checker_artifact_route_actual_adoption_migration_keep_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:verifier_handoff_candidate_adjacent_keep"
            ),
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:tool_brand_candidate_adjacent_keep"
            ),
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:actual_public_checker_migration_candidate_only"
            ),
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:runtime_policy_contract_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_checker_artifact_route_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_checker_artifact_actual_adoption_default:row_local_property_route_first"
                .to_string(),
            "model_check_checker_artifact_actual_adoption_default:checker_boundary_contract_anchor"
                .to_string(),
            "model_check_checker_artifact_actual_adoption_default:consumer_shaped_checker_artifact_candidate_only"
                .to_string(),
            "model_check_checker_artifact_actual_adoption_default:migration_candidate_adjacent_keep"
                .to_string(),
            "model_check_checker_artifact_actual_adoption_default:final_public_checker_artifact_later"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_checker_artifact_route_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    artifact_migration_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = artifact_migration_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption.guard_only"
                .to_string(),
        ],
    }
}

fn model_check_checker_artifact_route_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:row_local_property_route_first".to_string(),
            "guard:checker_boundary_contract_anchor".to_string(),
            "guard:consumer_shaped_checker_artifact_candidate_only".to_string(),
            "guard:migration_candidate_adjacent_keep".to_string(),
            "guard:final_public_checker_artifact_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_checker_artifact_route_actual_adoption_not_reached".to_string()]
        }
    }
}

fn model_check_checker_artifact_route_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_final_public_contract_reopen_threshold_sequence_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:property_language_and_tool_brand_first"
            ),
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:public_checker_artifact_and_migration_second"
            ),
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:verifier_handoff_and_runtime_policy_contract_third"
            ),
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:final_public_verifier_contract_fourth"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_final_public_contract_reopen_threshold_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "model_check_final_public_contract_reopen_default:property_language_and_tool_brand_first"
                .to_string(),
            "model_check_final_public_contract_reopen_default:public_checker_artifact_and_migration_second"
                .to_string(),
            "model_check_final_public_contract_reopen_default:verifier_handoff_and_runtime_policy_contract_third"
                .to_string(),
            "model_check_final_public_contract_reopen_default:final_public_verifier_contract_fourth"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn model_check_final_public_contract_reopen_threshold_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    checker_artifact_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = checker_artifact_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold.guard_only"
                .to_string(),
        ],
    }
}

fn model_check_final_public_contract_reopen_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:property_language_and_tool_brand_first".to_string(),
            "guard:public_checker_artifact_and_migration_second".to_string(),
            "guard:verifier_handoff_and_runtime_policy_contract_third".to_string(),
            "guard:final_public_verifier_contract_fourth".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:model_check_final_public_contract_reopen_threshold_not_reached".to_string()]
        }
    }
}

fn model_check_final_public_contract_reopen_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn authoritative_room_profile_axis_refs(sample_id: &str) -> Vec<String> {
    let mut refs = vec![
        "profile_axis:activation:authority-ack".to_string(),
        "profile_axis:authority_placement:single_room_authority".to_string(),
        "profile_axis:consistency_mode:authoritative_serial_transition".to_string(),
        "profile_axis:rng_source:authority_rng".to_string(),
    ];

    match sample_id {
        "p07-dice-late-join-visible-history" => {
            refs.push("profile_axis:late_join:published_history_visible_as_past".to_string())
        }
        "p08-dice-stale-reconnect-refresh" => {
            refs.push("profile_axis:stale_reconnect:fail_then_refresh".to_string());
            refs.push("profile_axis:replay:stale_incompatible_replay_invalidated".to_string());
        }
        _ => {}
    }

    refs.push("profile_axis:fairness_claim:no_distributed_fairness_theorem_required".to_string());
    refs
}

fn authoritative_room_relation_refs(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "relation_family:program_order".to_string(),
            "relation_family:publication_order".to_string(),
            "relation_family:observation_order".to_string(),
            "relation_family:witness_order".to_string(),
            "relation_family:finalization_order".to_string(),
            "relation_family:scoped_happens_before".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "relation_family:program_order".to_string(),
            "relation_family:observation_order".to_string(),
            "relation_family:witness_order".to_string(),
            "relation_family:finalization_order".to_string(),
            "relation_family:scoped_happens_before".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn authoritative_room_handoff_refs(sample_id: &str) -> Vec<String> {
    let mut refs = vec![
        "authority_handoff:owner_slot:single_room_authority".to_string(),
        "authority_handoff:stage_family:authoritative_serial_transition".to_string(),
        "authority_handoff:payload_ref:dice_state".to_string(),
    ];

    match sample_id {
        "p07-dice-late-join-visible-history" => {
            refs.insert(
                2,
                "authority_handoff:stage_sequence:publish_then_handoff".to_string(),
            );
        }
        "p08-dice-stale-reconnect-refresh" => {
            refs.insert(
                2,
                "authority_handoff:stage_sequence:fail_then_refresh".to_string(),
            );
        }
        _ => {}
    }

    refs
}

fn authoritative_room_runtime_evidence_refs(
    source_report: &CurrentL2SourceSampleRunReport,
) -> Vec<String> {
    let mut refs = Vec::new();

    for event in &source_report
        .runtime_report
        .run_report
        .trace_audit_sink
        .events
    {
        let formatted = format!("runtime_event:{}", event_kind_ref(event));
        if !refs.contains(&formatted) {
            refs.push(formatted);
        }
    }

    match source_report.sample_id.as_str() {
        "p07-dice-late-join-visible-history" => {
            extend_place_records(
                &mut refs,
                &source_report.runtime_report.run_report.final_place_store,
                "dice_state",
            );
            extend_place_records(
                &mut refs,
                &source_report.runtime_report.run_report.final_place_store,
                "observer_debug_text_output",
            );
        }
        "p08-dice-stale-reconnect-refresh" => {
            extend_place_records(
                &mut refs,
                &source_report.runtime_report.run_report.final_place_store,
                "dice_state",
            );
            extend_place_records(
                &mut refs,
                &source_report.runtime_report.run_report.final_place_store,
                "reconnect_debug_text_output",
            );
        }
        _ => {}
    }

    refs
}

fn authoritative_room_repo_local_emitted_artifact_refs(
    proof_notebook_review_units: &[ProofNotebookReviewUnitArtifact],
    model_check_concrete_carriers: &[ModelCheckConcreteCarrierArtifact],
) -> Vec<String> {
    let mut refs = proof_notebook_review_units
        .iter()
        .map(|unit| {
            format!(
                "repo_local_emitted_artifact:proof_notebook_review_unit:{}:{}",
                unit.subject_ref, unit.row.obligation_kind
            )
        })
        .collect::<Vec<_>>();

    refs.extend(model_check_concrete_carriers.iter().map(|carrier| {
        format!(
            "repo_local_emitted_artifact:model_check_concrete_carrier:{}:{}",
            carrier.subject_ref, carrier.case.obligation_kind
        )
    }));

    refs
}

fn authoritative_room_guard_refs(sample_id: &str) -> Vec<String> {
    let mut refs = vec!["guard:authoritative_room_default_profile".to_string()];
    match sample_id {
        "p07-dice-late-join-visible-history" => {
            refs.push("guard:late_join_history_visible_as_past".to_string());
        }
        "p08-dice-stale-reconnect-refresh" => {
            refs.push("guard:stale_reconnect_fail_then_refresh".to_string());
        }
        _ => {}
    }
    refs
}

fn authoritative_room_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:auditable_authority_witness".to_string(),
        "kept_later:delegated_rng_service".to_string(),
        "kept_later:distributed_randomness_provider".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn auditable_authority_witness_binding_refs(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "witness_binding:witness_kind:authority_draw_witness".to_string(),
            "witness_binding:action_ref:handoff_dice_authority@dice_state".to_string(),
            "witness_binding:draw_slot:primary".to_string(),
            "witness_binding:draw_result_binding:publish_roll_result@dice_state".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn auditable_authority_witness_contrast_refs() -> Vec<String> {
    vec![
        "contrast_target:append_friendly_notice_room".to_string(),
        "contrast_target:note_only_witness".to_string(),
        "contrast_target:expanded_attested_receipt".to_string(),
    ]
}

fn auditable_authority_witness_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:delegated_rng_service".to_string(),
        "kept_later:provider_receipt_optional_attachment".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:distributed_randomness_provider".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn delegated_rng_service_profile_axis_refs() -> Vec<String> {
    vec![
        "profile_axis:activation:authority-ack".to_string(),
        "profile_axis:authority_placement:single_room_authority".to_string(),
        "profile_axis:consistency_mode:authoritative_serial_transition".to_string(),
        "profile_axis:rng_source:delegated_rng_service".to_string(),
        "profile_axis:fairness_claim:opaque_authority_trust".to_string(),
    ]
}

fn delegated_rng_service_provider_boundary_refs() -> Vec<String> {
    vec![
        "provider_boundary:placement:delegated_rng_service".to_string(),
        "provider_boundary:authority_keeps_commit".to_string(),
        "provider_boundary:provider_returns_draw_not_state_transition".to_string(),
        "provider_boundary:room_state_mutation_by_authority".to_string(),
        "provider_boundary:witness_core_unchanged".to_string(),
    ]
}

fn delegated_rng_service_optional_attachment_refs() -> Vec<String> {
    vec![
        "optional_attachment:provider_draw_ref".to_string(),
        "optional_attachment:provider_receipt_ref".to_string(),
    ]
}

fn delegated_rng_service_runtime_evidence_refs(
    source_report: &CurrentL2SourceSampleRunReport,
) -> Vec<String> {
    let mut refs = Vec::new();

    for event in &source_report
        .runtime_report
        .run_report
        .trace_audit_sink
        .events
    {
        let formatted = format!("runtime_event:{}", event_kind_ref(event));
        if !refs.contains(&formatted) {
            refs.push(formatted);
        }
    }

    extend_place_records(
        &mut refs,
        &source_report.runtime_report.run_report.final_place_store,
        "dice_state",
    );
    extend_place_records(
        &mut refs,
        &source_report.runtime_report.run_report.final_place_store,
        "provider_debug_text_output",
    );

    refs
}

fn delegated_rng_service_contrast_refs() -> Vec<String> {
    vec![
        "contrast_target:authority_rng_baseline".to_string(),
        "contrast_target:delegated_provider_attestation".to_string(),
        "contrast_target:distributed_randomness_provider".to_string(),
    ]
}

fn delegated_rng_service_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:auditable_authority_witness_combination".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:distributed_randomness_provider".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:control_plane_separated_carrier".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_public_shape_witness_attachment_refs(
    witness: &CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening,
) -> Vec<String> {
    if witness.strengthening_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        return Vec::new();
    }

    vec![
        "witness_public_shape_reserve:witness_kind".to_string(),
        "witness_public_shape_reserve:action_ref".to_string(),
        "witness_public_shape_reserve:draw_slot".to_string(),
        "witness_public_shape_reserve:draw_result_binding".to_string(),
    ]
}

fn witness_provider_public_shape_provider_attachment_refs(
    delegated: &CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
) -> Vec<String> {
    if delegated.practical_status != CurrentL2EmittedArtifactRouteStatus::Reached {
        return Vec::new();
    }

    delegated.optional_attachment_refs.clone()
}

fn witness_provider_public_shape_emitted_artifact_refs(
    vertical_slice: &CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization,
    witness: &CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening,
    delegated: &CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
) -> Vec<String> {
    if delegated.practical_status == CurrentL2EmittedArtifactRouteStatus::Reached {
        return delegated.repo_local_emitted_artifact_refs.clone();
    }
    if witness.strengthening_status == CurrentL2EmittedArtifactRouteStatus::Reached {
        return witness.repo_local_emitted_artifact_refs.clone();
    }
    if vertical_slice.slice_status == CurrentL2EmittedArtifactRouteStatus::Reached {
        return vertical_slice.repo_local_emitted_artifact_refs.clone();
    }
    Vec::new()
}

fn witness_provider_public_shape_compare_floor_refs(
    vertical_slice: &CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization,
    witness: &CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening,
    delegated: &CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
) -> Vec<String> {
    let mut refs = Vec::new();
    if vertical_slice.slice_status == CurrentL2EmittedArtifactRouteStatus::Reached {
        refs.push("compare_floor:current_l2.authoritative_room.vertical_slice".to_string());
    }
    if witness.strengthening_status == CurrentL2EmittedArtifactRouteStatus::Reached {
        refs.push("compare_floor:current_l2.auditable_authority_witness.strengthening".to_string());
    }
    if delegated.practical_status == CurrentL2EmittedArtifactRouteStatus::Reached {
        refs.push("compare_floor:current_l2.delegated_rng_service.practical".to_string());
    }
    refs.push(
        "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold".to_string(),
    );
    refs
}

fn witness_provider_artifact_public_shape_threshold_default_refs() -> Vec<String> {
    vec![
        "public_shape_threshold_default:claim_payload_split_first".to_string(),
        "public_shape_threshold_default:repo_local_emitted_artifact_refs_first".to_string(),
        "public_shape_threshold_default:optional_attachment_refs_only".to_string(),
        "public_shape_threshold_default:combined_public_contract_later".to_string(),
    ]
}

fn witness_provider_artifact_public_shape_contrast_refs() -> Vec<String> {
    vec![
        "contrast_target:append_friendly_notice_room".to_string(),
        "contrast_target:note_only_witness".to_string(),
        "contrast_target:delegated_provider_attestation".to_string(),
        "contrast_target:combined_public_contract".to_string(),
    ]
}

fn witness_provider_artifact_public_shape_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:public_shape_threshold_only".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:optional_attachment_refs_only".to_string(),
            "guard:combined_public_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:witness_provider_artifact_threshold_not_reached".to_string()]
        }
    }
}

fn witness_provider_artifact_public_shape_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_artifact_public_shape_actual_adoption_witness_route_refs(
    sample_id: &str,
    witness_attachment_refs: &[String],
) -> Vec<String> {
    if witness_attachment_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("witness_public_shape_route:{sample_id}:claim_payload_split_first"),
            format!("witness_public_shape_route:{sample_id}:witness_attachment_refs_only"),
            format!("witness_public_shape_route:{sample_id}:symbolic_binding_ref_keep"),
            format!("witness_public_shape_route:{sample_id}:combined_public_contract_later"),
        ]
    }
}

fn witness_provider_artifact_public_shape_actual_adoption_provider_route_refs(
    sample_id: &str,
    provider_attachment_refs: &[String],
) -> Vec<String> {
    if provider_attachment_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("provider_public_shape_route:{sample_id}:provider_attachment_refs_only"),
            format!("provider_public_shape_route:{sample_id}:optional_provider_attachment_keep"),
            format!("provider_public_shape_route:{sample_id}:delegated_provider_attestation_later"),
            format!("provider_public_shape_route:{sample_id}:combined_public_contract_later"),
        ]
    }
}

fn witness_provider_artifact_public_shape_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "public_shape_actual_adoption_default:claim_payload_split_first".to_string(),
            "public_shape_actual_adoption_default:repo_local_emitted_artifact_refs_first"
                .to_string(),
            "public_shape_actual_adoption_default:optional_attachment_refs_only".to_string(),
            "public_shape_actual_adoption_default:combined_public_contract_later".to_string(),
            "public_shape_actual_adoption_default:witness_provider_routes_noncollapsed".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn witness_provider_artifact_public_shape_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut threshold_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            threshold_compare_floor_refs.push(
                "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption"
                    .to_string(),
            );
            threshold_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption_guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_artifact_public_shape_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:public_shape_actual_adoption_only".to_string(),
            "guard:claim_payload_split_first".to_string(),
            "guard:optional_attachment_refs_only".to_string(),
            "guard:combined_public_contract_later".to_string(),
            "guard:witness_provider_routes_noncollapsed".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:witness_provider_artifact_actual_adoption_not_reached".to_string()]
        }
    }
}

fn minimal_companion_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "profile authoritative_room_default".to_string(),
            "activation authority-ack".to_string(),
            "authority single_room_authority".to_string(),
            "consistency authoritative_serial_transition".to_string(),
            "rng authority_rng".to_string(),
            "publication publish_roll_result@dice_state".to_string(),
            "handoff handoff_dice_authority@dice_state".to_string(),
            "late_join published_history_visible_as_past".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "profile authoritative_room_default".to_string(),
            "activation authority-ack".to_string(),
            "authority single_room_authority".to_string(),
            "consistency authoritative_serial_transition".to_string(),
            "rng authority_rng".to_string(),
            "rollback stale_reconnect".to_string(),
            "refresh refresh_owner_snapshot@dice_state".to_string(),
            "replay stale_incompatible_replay_invalidated".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn minimal_companion_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn stage_block_surface_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "transition handoff_turn(dice_owner = player_a)".to_string(),
            "stage publish:".to_string(),
            "  publish publish_roll_result@dice_state".to_string(),
            "stage handoff:".to_string(),
            "  handoff handoff_dice_authority@dice_state".to_string(),
            "    after publish(publish_roll_result@dice_state)".to_string(),
            "    requires witness(publish_roll_result@dice_state)".to_string(),
            "stage observe:".to_string(),
            "  observe late_join_view@dice_state".to_string(),
            "    after handoff(handoff_dice_authority@dice_state)".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "transition reconnect_refresh(dice_owner = player_a)".to_string(),
            "stage rollback:".to_string(),
            "  rollback stale_reconnect".to_string(),
            "stage refresh:".to_string(),
            "  refresh refresh_owner_snapshot@dice_state".to_string(),
            "    after rollback(stale_reconnect)".to_string(),
            "stage replay:".to_string(),
            "  invalidate stale_incompatible_replay@dice_state".to_string(),
            "    after refresh(refresh_owner_snapshot@dice_state)".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn stage_block_surface_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn order_handoff_serial_scope_reserve_surface_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "serial on dice_authority {".to_string(),
            "  publish publish_roll_result@dice_state".to_string(),
            "  handoff handoff_dice_authority@dice_state".to_string(),
            "    requires witness(publish_roll_result@dice_state)".to_string(),
            "  observe late_join_view@dice_state".to_string(),
            "}".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "serial on dice_authority {".to_string(),
            "  rollback stale_reconnect".to_string(),
            "  refresh refresh_owner_snapshot@dice_state".to_string(),
            "  invalidate stale_incompatible_replay@dice_state".to_string(),
            "}".to_string(),
        ],
        "p09-dice-delegated-rng-provider-placement" => vec![
            "serial on dice_authority {".to_string(),
            "  fetch fetch_provider_roll@delegated_rng".to_string(),
            "  publish publish_roll_result@dice_state".to_string(),
            "  handoff handoff_dice_authority@dice_state".to_string(),
            "}".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn order_handoff_serial_scope_reserve_surface_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:serial_scope_room_specific_reserve".to_string(),
            "guard:principal_edge_row_surface_unchanged".to_string(),
            "guard:helper_local_surface_only".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:serial_scope_reserve_surface_not_reached".to_string()]
        }
    }
}

fn order_handoff_serial_scope_reserve_surface_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:serial_scope_public_promotion".to_string(),
        "kept_later:serial_scope_beyond_authoritative_room".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_artifact_schema".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn order_handoff_surface_artifact_threshold_default_refs() -> Vec<String> {
    vec![
        "surface_threshold_default:edge_row_vertical_continuation_principal".to_string(),
        "surface_threshold_default:readable_high_level_relation_vocabulary".to_string(),
        "surface_threshold_default:stage_block_secondary_candidate".to_string(),
        "surface_threshold_default:repo_local_emitted_artifact_refs_first".to_string(),
        "surface_threshold_default:witness_clause_public_row_later".to_string(),
    ]
}

fn order_handoff_surface_artifact_threshold_compare_floor_refs() -> Vec<String> {
    vec![
        "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
        "compare_floor:current_l2.experimental_order_handoff_surface".to_string(),
        "compare_floor:current_l2.experimental_stage_block_surface".to_string(),
        "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold".to_string(),
        "compare_floor:current_l2.order_handoff.surface_artifact_threshold".to_string(),
    ]
}

fn order_handoff_surface_artifact_threshold_contrast_refs() -> Vec<String> {
    vec![
        "contrast_target:authoritative_room_serial_scope_sugar".to_string(),
        "contrast_target:low_level_memory_order_source_surface".to_string(),
        "contrast_target:packed_metadata_line".to_string(),
        "contrast_target:combined_public_contract".to_string(),
    ]
}

fn order_handoff_surface_artifact_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:surface_threshold_only".to_string(),
            "guard:edge_row_principal".to_string(),
            "guard:stage_block_secondary_only".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:final_wording_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:surface_artifact_threshold_not_reached".to_string()]
        }
    }
}

fn order_handoff_surface_artifact_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn order_handoff_surface_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "surface_actual_adoption_default:edge_row_vertical_continuation_principal".to_string(),
            "surface_actual_adoption_default:readable_high_level_relation_vocabulary".to_string(),
            "surface_actual_adoption_default:stage_block_secondary_keep".to_string(),
            "surface_actual_adoption_default:repo_local_emitted_artifact_refs_first".to_string(),
            "surface_actual_adoption_default:final_wording_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn order_handoff_surface_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut threshold_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            threshold_compare_floor_refs
                .push("compare_floor:current_l2.order_handoff.surface_actual_adoption".to_string());
            threshold_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.order_handoff.surface_actual_adoption_guard_only".to_string(),
        ],
    }
}

fn order_handoff_surface_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:surface_actual_adoption_only".to_string(),
            "guard:edge_row_principal".to_string(),
            "guard:stage_block_secondary_keep".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:final_wording_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:order_handoff_surface_actual_adoption_not_reached".to_string()]
        }
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_witness_refs(
    sample_id: &str,
    witness_route_refs: &[String],
) -> Vec<String> {
    if witness_route_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("witness_public_contract_candidate:{sample_id}:claim_payload_split_first"),
            format!("witness_public_contract_candidate:{sample_id}:witness_route_noncollapsed"),
            format!(
                "witness_public_contract_candidate:{sample_id}:final_public_witness_schema_later"
            ),
            format!("witness_public_contract_candidate:{sample_id}:combined_public_contract_later"),
        ]
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_provider_refs(
    sample_id: &str,
    provider_route_refs: &[String],
) -> Vec<String> {
    if provider_route_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "provider_public_contract_candidate:{sample_id}:optional_provider_attachment_keep"
            ),
            format!("provider_public_contract_candidate:{sample_id}:provider_route_noncollapsed"),
            format!(
                "provider_public_contract_candidate:{sample_id}:delegated_provider_attestation_later"
            ),
            format!(
                "provider_public_contract_candidate:{sample_id}:combined_public_contract_later"
            ),
        ]
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_emitted_refs(
    sample_id: &str,
    repo_local_emitted_artifact_refs: &[String],
) -> Vec<String> {
    if repo_local_emitted_artifact_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "emitted_handoff_contract_candidate:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "emitted_handoff_contract_candidate:{sample_id}:public_contract_adjacent_not_collapsed"
            ),
            format!(
                "emitted_handoff_contract_candidate:{sample_id}:final_emitted_handoff_contract_later"
            ),
        ]
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "public_contract_coupled_later_default:claim_payload_split_first".to_string(),
            "public_contract_coupled_later_default:repo_local_emitted_artifact_refs_first"
                .to_string(),
            "public_contract_coupled_later_default:witness_provider_routes_noncollapsed"
                .to_string(),
            "public_contract_coupled_later_default:combined_public_contract_later".to_string(),
            "public_contract_coupled_later_default:final_emitted_handoff_contract_later"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut public_shape_compare_floor_refs: Vec<String>,
    order_handoff_status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            if order_handoff_status == CurrentL2EmittedArtifactRouteStatus::Reached {
                public_shape_compare_floor_refs.push(
                    "compare_floor:current_l2.order_handoff.surface_actual_adoption".to_string(),
                );
            }
            public_shape_compare_floor_refs.push(
                "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate"
                    .to_string(),
            );
            public_shape_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate_guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:public_contract_coupled_later_gate_only".to_string(),
            "guard:claim_payload_split_first".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:witness_provider_routes_noncollapsed".to_string(),
            "guard:combined_public_contract_later".to_string(),
            "guard:final_emitted_handoff_contract_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "guard:witness_provider_emitted_contract_coupled_later_gate_not_reached".to_string(),
        ],
    }
}

fn witness_provider_emitted_contract_coupled_later_gate_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_emitted_contract_trace_alignment_pair_ref(
    sample_id: &str,
    obligation_kind: &str,
) -> String {
    format!("witness_provider_emitted_contract_trace_alignment_pair:{sample_id}:{obligation_kind}")
}

fn witness_provider_emitted_contract_trace_alignment_route_pair_refs(
    sample_id: &str,
    repo_local_emitted_artifact_refs: &[String],
) -> Result<Vec<String>, String> {
    let mut refs = Vec::new();
    for entry in repo_local_emitted_artifact_refs {
        let Some((_, obligation_kind)) = entry.rsplit_once(':') else {
            return Err(format!(
                "witness/provider emitted-contract trace alignment could not parse obligation kind from repo-local emitted artifact ref `{entry}`"
            ));
        };
        let pair =
            witness_provider_emitted_contract_trace_alignment_pair_ref(sample_id, obligation_kind);
        if !refs.contains(&pair) {
            refs.push(pair);
        }
    }
    refs.sort();
    Ok(refs)
}

fn witness_provider_emitted_contract_trace_alignment_emitted_pair_refs(
    sample_id: &str,
    repo_local_emitted_artifact_refs: &[String],
    emitted_contract_candidate_refs: &[String],
) -> Result<Vec<String>, String> {
    if emitted_contract_candidate_refs.is_empty() {
        return Ok(Vec::new());
    }
    witness_provider_emitted_contract_trace_alignment_route_pair_refs(
        sample_id,
        repo_local_emitted_artifact_refs,
    )
}

fn witness_provider_emitted_contract_trace_alignment_matched_pair_refs(
    route_pair_refs: &[String],
    emitted_contract_pair_refs: &[String],
) -> Result<Vec<String>, String> {
    if route_pair_refs != emitted_contract_pair_refs {
        return Err(format!(
            "witness/provider emitted-contract trace alignment pair drift: route_pair_refs={route_pair_refs:?}, emitted_contract_pair_refs={emitted_contract_pair_refs:?}"
        ));
    }
    Ok(route_pair_refs.to_vec())
}

fn witness_provider_emitted_contract_trace_alignment_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut route_compare_floor_refs: Vec<String>,
    coupled_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            for entry in coupled_compare_floor_refs {
                if !route_compare_floor_refs.contains(&entry) {
                    route_compare_floor_refs.push(entry);
                }
            }
            route_compare_floor_refs.push(
                "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge"
                    .to_string(),
            );
            route_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge_guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_emitted_contract_trace_alignment_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:repo_local_trace_alignment_only".to_string(),
            "guard:no_final_public_contract_promotion".to_string(),
            "guard:no_final_public_schema_promotion".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:witness_provider_emitted_contract_trace_alignment_not_reached".to_string()]
        }
    }
}

fn witness_provider_emitted_contract_trace_alignment_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:trace_alignment_beyond_representative_corpus".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_public_schema_coupled_later_gate_witness_schema_refs(
    sample_id: &str,
    witness_contract_candidate_refs: &[String],
) -> Vec<String> {
    if witness_contract_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("witness_schema_candidate:{sample_id}:claim_payload_split_first"),
            format!("witness_schema_candidate:{sample_id}:witness_route_noncollapsed"),
            format!("witness_schema_candidate:{sample_id}:symbolic_binding_ref_keep"),
            format!("witness_schema_candidate:{sample_id}:final_public_witness_schema_later"),
        ]
    }
}

fn witness_provider_public_schema_coupled_later_gate_provider_receipt_refs(
    sample_id: &str,
    provider_contract_candidate_refs: &[String],
) -> Vec<String> {
    if provider_contract_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("provider_receipt_candidate:{sample_id}:provider_route_noncollapsed"),
            format!("provider_receipt_candidate:{sample_id}:optional_provider_attachment_keep"),
            format!(
                "provider_receipt_candidate:{sample_id}:delegated_provider_attestation_adjacent_keep"
            ),
            format!(
                "provider_receipt_candidate:{sample_id}:final_public_provider_receipt_schema_later"
            ),
        ]
    }
}

fn witness_provider_public_schema_coupled_later_gate_combined_contract_refs(
    sample_id: &str,
    emitted_contract_candidate_refs: &[String],
) -> Vec<String> {
    if emitted_contract_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "combined_public_contract_candidate:{sample_id}:witness_provider_routes_noncollapsed"
            ),
            format!(
                "combined_public_contract_candidate:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "combined_public_contract_candidate:{sample_id}:combined_public_contract_candidate_only"
            ),
            format!(
                "combined_public_contract_candidate:{sample_id}:final_emitted_handoff_contract_adjacent_keep"
            ),
        ]
    }
}

fn witness_provider_public_schema_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "public_schema_coupled_default:claim_payload_split_first".to_string(),
            "public_schema_coupled_default:witness_provider_routes_noncollapsed".to_string(),
            "public_schema_coupled_default:combined_public_contract_candidate_only".to_string(),
            "public_schema_coupled_default:final_emitted_handoff_contract_adjacent_keep"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn witness_provider_public_schema_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut public_contract_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            public_contract_compare_floor_refs.push(
                "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate"
                    .to_string(),
            );
            public_contract_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate_guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_public_schema_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:final_public_witness_provider_schema_candidate_only".to_string(),
            "guard:combined_public_contract_candidate_only".to_string(),
            "guard:final_emitted_handoff_contract_adjacent_keep".to_string(),
            "guard:exhaustive_shared_space_catalog_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:witness_provider_public_schema_coupled_later_gate_not_reached".to_string()]
        }
    }
}

fn witness_provider_public_schema_coupled_later_gate_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_route_actual_adoption_witness_route_refs(
    sample_id: &str,
    witness_route_refs: &[String],
) -> Vec<String> {
    if witness_route_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("witness_provider_route_actual:{sample_id}:claim_payload_split_first"),
            format!("witness_provider_route_actual:{sample_id}:witness_route_first"),
            format!(
                "witness_provider_route_actual:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!("witness_provider_route_actual:{sample_id}:public_schema_candidate_keep"),
            format!("witness_provider_route_actual:{sample_id}:combined_public_contract_later"),
        ]
    }
}

fn witness_provider_route_actual_adoption_provider_route_refs(
    sample_id: &str,
    provider_route_refs: &[String],
) -> Vec<String> {
    if provider_route_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("witness_provider_route_actual:{sample_id}:provider_route_first"),
            format!(
                "witness_provider_route_actual:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!("witness_provider_route_actual:{sample_id}:public_schema_candidate_keep"),
            format!(
                "witness_provider_route_actual:{sample_id}:delegated_provider_attestation_later"
            ),
            format!("witness_provider_route_actual:{sample_id}:combined_public_contract_later"),
        ]
    }
}

fn witness_provider_route_actual_adoption_schema_keep_refs(
    sample_id: &str,
    witness_schema_candidate_refs: &[String],
    provider_receipt_candidate_refs: &[String],
    combined_public_contract_candidate_refs: &[String],
) -> Vec<String> {
    let mut refs = Vec::new();

    if !witness_schema_candidate_refs.is_empty() {
        refs.push(format!(
            "witness_provider_schema_keep:{sample_id}:final_public_witness_schema_candidate_only"
        ));
    }
    if !provider_receipt_candidate_refs.is_empty() {
        refs.push(format!(
            "witness_provider_schema_keep:{sample_id}:final_public_provider_receipt_schema_candidate_only"
        ));
    }
    if !combined_public_contract_candidate_refs.is_empty() {
        refs.push(format!(
            "witness_provider_schema_keep:{sample_id}:combined_public_contract_candidate_only"
        ));
        refs.push(format!(
            "witness_provider_schema_keep:{sample_id}:final_emitted_handoff_contract_adjacent_keep"
        ));
    }

    refs
}

fn witness_provider_route_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "witness_provider_route_actual_adoption_default:claim_payload_split_first"
                .to_string(),
            "witness_provider_route_actual_adoption_default:route_noncollapse_first"
                .to_string(),
            "witness_provider_route_actual_adoption_default:repo_local_emitted_artifact_refs_first"
                .to_string(),
            "witness_provider_route_actual_adoption_default:public_schema_candidate_keep"
                .to_string(),
            "witness_provider_route_actual_adoption_default:combined_public_contract_later"
                .to_string(),
            "witness_provider_route_actual_adoption_default:final_emitted_handoff_contract_adjacent_keep"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn witness_provider_route_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    public_schema_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = public_schema_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.witness_provider_route_actual_adoption".to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_route_actual_adoption.guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_route_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:claim_payload_split_first".to_string(),
            "guard:witness_provider_route_first".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:public_schema_candidate_keep".to_string(),
            "guard:combined_public_contract_later".to_string(),
            "guard:final_emitted_handoff_contract_adjacent_keep".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:witness_provider_route_actual_adoption_not_reached".to_string()]
        }
    }
}

fn witness_provider_route_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_schema_route_actual_adoption_witness_refs(
    sample_id: &str,
    witness_route_actual_refs: &[String],
    witness_schema_candidate_refs: &[String],
) -> Vec<String> {
    if witness_route_actual_refs.is_empty() || witness_schema_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!("witness_provider_schema_route_actual:{sample_id}:claim_payload_split_first"),
            format!(
                "witness_provider_schema_route_actual:{sample_id}:witness_schema_candidate_keep"
            ),
            format!("witness_provider_schema_route_actual:{sample_id}:witness_route_first"),
            format!(
                "witness_provider_schema_route_actual:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "witness_provider_schema_route_actual:{sample_id}:combined_public_contract_later"
            ),
        ]
    }
}

fn witness_provider_schema_route_actual_adoption_provider_refs(
    sample_id: &str,
    provider_route_actual_refs: &[String],
    provider_receipt_candidate_refs: &[String],
) -> Vec<String> {
    if provider_route_actual_refs.is_empty() || provider_receipt_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "witness_provider_schema_route_actual:{sample_id}:provider_receipt_candidate_keep"
            ),
            format!("witness_provider_schema_route_actual:{sample_id}:provider_route_first"),
            format!(
                "witness_provider_schema_route_actual:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "witness_provider_schema_route_actual:{sample_id}:delegated_provider_attestation_later"
            ),
            format!(
                "witness_provider_schema_route_actual:{sample_id}:combined_public_contract_later"
            ),
        ]
    }
}

fn witness_provider_schema_route_actual_adoption_combined_keep_refs(
    sample_id: &str,
    combined_public_contract_candidate_refs: &[String],
) -> Vec<String> {
    if combined_public_contract_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "witness_provider_combined_contract_keep:{sample_id}:combined_public_contract_candidate_only"
            ),
            format!(
                "witness_provider_combined_contract_keep:{sample_id}:final_emitted_handoff_contract_adjacent_keep"
            ),
        ]
    }
}

fn witness_provider_schema_route_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "witness_provider_schema_route_actual_adoption_default:claim_payload_split_first"
                .to_string(),
            "witness_provider_schema_route_actual_adoption_default:schema_route_first"
                .to_string(),
            "witness_provider_schema_route_actual_adoption_default:repo_local_emitted_artifact_refs_first"
                .to_string(),
            "witness_provider_schema_route_actual_adoption_default:combined_public_contract_candidate_keep"
                .to_string(),
            "witness_provider_schema_route_actual_adoption_default:final_emitted_handoff_contract_adjacent_keep"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn witness_provider_schema_route_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    route_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = route_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.witness_provider_schema_route_actual_adoption"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_schema_route_actual_adoption.guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_schema_route_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:claim_payload_split_first".to_string(),
            "guard:witness_provider_schema_route_first".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:combined_public_contract_candidate_keep".to_string(),
            "guard:final_emitted_handoff_contract_adjacent_keep".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:witness_provider_schema_route_actual_adoption_not_reached".to_string()]
        }
    }
}

fn witness_provider_schema_route_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn witness_provider_final_public_contract_reopen_threshold_sequence_refs(
    sample_id: &str,
    witness_schema_route_refs: &[String],
    provider_receipt_route_refs: &[String],
    combined_public_contract_keep_refs: &[String],
) -> Vec<String> {
    if witness_schema_route_refs.is_empty()
        && provider_receipt_route_refs.is_empty()
        && combined_public_contract_keep_refs.is_empty()
    {
        Vec::new()
    } else {
        vec![
            format!("witness_provider_final_contract_reopen:{sample_id}:public_schema_pair_first"),
            format!(
                "witness_provider_final_contract_reopen:{sample_id}:delegated_attestation_and_combined_contract_second"
            ),
            format!(
                "witness_provider_final_contract_reopen:{sample_id}:final_emitted_handoff_contract_third"
            ),
            format!(
                "witness_provider_final_contract_reopen:{sample_id}:exhaustive_shared_space_catalog_later"
            ),
        ]
    }
}

fn witness_provider_final_public_contract_reopen_threshold_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "witness_provider_final_contract_reopen_default:public_schema_pair_first"
                .to_string(),
            "witness_provider_final_contract_reopen_default:delegated_attestation_and_combined_contract_second"
                .to_string(),
            "witness_provider_final_contract_reopen_default:final_emitted_handoff_contract_third"
                .to_string(),
            "witness_provider_final_contract_reopen_default:exhaustive_shared_space_catalog_later"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn witness_provider_final_public_contract_reopen_threshold_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    schema_route_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = schema_route_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold.guard_only"
                .to_string(),
        ],
    }
}

fn witness_provider_final_public_contract_reopen_threshold_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:public_schema_pair_first".to_string(),
            "guard:delegated_attestation_and_combined_contract_second".to_string(),
            "guard:final_emitted_handoff_contract_third".to_string(),
            "guard:exhaustive_shared_space_catalog_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "guard:witness_provider_final_public_contract_reopen_threshold_not_reached".to_string(),
        ],
    }
}

fn witness_provider_final_public_contract_reopen_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn order_handoff_source_wording_emitted_artifact_coupled_later_gate_source_wording_refs(
    sample_id: &str,
    principal_surface_lines: &[String],
    secondary_surface_lines: &[String],
) -> Vec<String> {
    if principal_surface_lines.is_empty() {
        return Vec::new();
    }

    let mut refs = vec![
        format!(
            "order_handoff_source_wording_candidate:{sample_id}:edge_row_vertical_continuation_principal"
        ),
        format!(
            "order_handoff_source_wording_candidate:{sample_id}:readable_high_level_relation_vocabulary"
        ),
    ];

    if !secondary_surface_lines.is_empty() {
        refs.push(format!(
            "order_handoff_source_wording_candidate:{sample_id}:stage_block_secondary_keep"
        ));
    }

    refs.push(format!(
        "order_handoff_source_wording_candidate:{sample_id}:thread_node_same_causal_language"
    ));
    refs.push(format!(
        "order_handoff_source_wording_candidate:{sample_id}:final_source_surface_handoff_wording_later"
    ));
    refs
}

fn order_handoff_source_wording_emitted_artifact_coupled_later_gate_emitted_schema_refs(
    sample_id: &str,
    repo_local_emitted_artifact_refs: &[String],
    emitted_contract_candidate_refs: &[String],
) -> Vec<String> {
    if repo_local_emitted_artifact_refs.is_empty() || emitted_contract_candidate_refs.is_empty() {
        return Vec::new();
    }

    vec![
        format!(
            "order_handoff_emitted_artifact_schema_candidate:{sample_id}:repo_local_emitted_artifact_refs_first"
        ),
        format!(
            "order_handoff_emitted_artifact_schema_candidate:{sample_id}:source_surface_actual_adoption_adjacent"
        ),
        format!(
            "order_handoff_emitted_artifact_schema_candidate:{sample_id}:witness_provider_contract_adjacent_not_collapsed"
        ),
        format!(
            "order_handoff_emitted_artifact_schema_candidate:{sample_id}:final_emitted_artifact_schema_later"
        ),
    ]
}

fn order_handoff_source_wording_emitted_artifact_coupled_later_gate_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "source_wording_emitted_artifact_default:edge_row_vertical_continuation_principal"
                .to_string(),
            "source_wording_emitted_artifact_default:readable_high_level_relation_vocabulary"
                .to_string(),
            "source_wording_emitted_artifact_default:stage_block_secondary_keep".to_string(),
            "source_wording_emitted_artifact_default:thread_node_same_causal_language".to_string(),
            "source_wording_emitted_artifact_default:repo_local_emitted_artifact_refs_first"
                .to_string(),
            "source_wording_emitted_artifact_default:final_public_wording_and_schema_later"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn order_handoff_source_wording_emitted_artifact_coupled_later_gate_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    mut surface_compare_floor_refs: Vec<String>,
    witness_provider_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            for entry in witness_provider_compare_floor_refs {
                if !surface_compare_floor_refs.contains(&entry) {
                    surface_compare_floor_refs.push(entry);
                }
            }
            let final_ref =
                "compare_floor:current_l2.order_handoff.source_wording_emitted_artifact.coupled_later_gate"
                    .to_string();
            if !surface_compare_floor_refs.contains(&final_ref) {
                surface_compare_floor_refs.push(final_ref);
            }
            surface_compare_floor_refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.order_handoff.source_wording_emitted_artifact.guard_only"
                .to_string(),
        ],
    }
}

fn order_handoff_source_wording_emitted_artifact_coupled_later_gate_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:source_wording_candidate_only".to_string(),
            "guard:emitted_artifact_schema_candidate_only".to_string(),
            "guard:thread_node_same_causal_language_keep".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
            "guard:final_emitted_artifact_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:order_handoff_source_wording_emitted_artifact_not_reached".to_string()]
        }
    }
}

fn order_handoff_source_wording_emitted_artifact_coupled_later_gate_kept_later_refs() -> Vec<String>
{
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_artifact_schema".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn order_handoff_source_wording_route_actual_adoption_source_wording_refs(
    sample_id: &str,
    principal_surface_lines: &[String],
    _secondary_surface_lines: &[String],
    source_wording_candidate_refs: &[String],
) -> Vec<String> {
    if principal_surface_lines.is_empty() || source_wording_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "order_handoff_source_wording_actual_route:{sample_id}:edge_row_vertical_continuation_principal"
            ),
            format!(
                "order_handoff_source_wording_actual_route:{sample_id}:readable_high_level_relation_vocabulary"
            ),
            format!(
                "order_handoff_source_wording_actual_route:{sample_id}:stage_block_secondary_keep"
            ),
            format!(
                "order_handoff_source_wording_actual_route:{sample_id}:thread_node_same_causal_language"
            ),
            format!(
                "order_handoff_source_wording_actual_route:{sample_id}:final_source_surface_handoff_wording_later"
            ),
        ]
    }
}

fn order_handoff_source_wording_route_actual_adoption_emitted_keep_refs(
    sample_id: &str,
    emitted_artifact_schema_candidate_refs: &[String],
) -> Vec<String> {
    if emitted_artifact_schema_candidate_refs.is_empty() {
        Vec::new()
    } else {
        vec![
            format!(
                "order_handoff_emitted_artifact_keep:{sample_id}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "order_handoff_emitted_artifact_keep:{sample_id}:source_surface_actual_adoption_adjacent"
            ),
            format!(
                "order_handoff_emitted_artifact_keep:{sample_id}:witness_provider_contract_adjacent_not_collapsed"
            ),
            format!(
                "order_handoff_emitted_artifact_keep:{sample_id}:final_emitted_artifact_schema_later"
            ),
        ]
    }
}

fn order_handoff_source_wording_route_actual_adoption_default_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "order_handoff_source_wording_actual_adoption_default:edge_row_vertical_continuation_principal"
                .to_string(),
            "order_handoff_source_wording_actual_adoption_default:readable_high_level_relation_vocabulary"
                .to_string(),
            "order_handoff_source_wording_actual_adoption_default:stage_block_secondary_keep"
                .to_string(),
            "order_handoff_source_wording_actual_adoption_default:thread_node_same_causal_language"
                .to_string(),
            "order_handoff_source_wording_actual_adoption_default:repo_local_emitted_artifact_refs_first"
                .to_string(),
            "order_handoff_source_wording_actual_adoption_default:emitted_artifact_schema_candidate_keep"
                .to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
    }
}

fn order_handoff_source_wording_route_actual_adoption_compare_floor_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
    coupled_compare_floor_refs: Vec<String>,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let mut refs = coupled_compare_floor_refs;
            refs.push(
                "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption"
                    .to_string(),
            );
            refs
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => vec![
            "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption.guard_only"
                .to_string(),
        ],
    }
}

fn order_handoff_source_wording_route_actual_adoption_guard_refs(
    status: CurrentL2EmittedArtifactRouteStatus,
) -> Vec<String> {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => vec![
            "guard:edge_row_vertical_continuation_principal".to_string(),
            "guard:readable_high_level_relation_vocabulary".to_string(),
            "guard:stage_block_secondary_keep".to_string(),
            "guard:thread_node_same_causal_language".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:emitted_artifact_schema_candidate_keep".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
            "guard:final_emitted_artifact_schema_later".to_string(),
        ],
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            vec!["guard:order_handoff_source_wording_route_actual_adoption_not_reached".to_string()]
        }
    }
}

fn order_handoff_source_wording_route_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_artifact_schema".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn event_kind_ref(event: &EventKind) -> &'static str {
    match event {
        EventKind::PerformSuccess => "perform-success",
        EventKind::PerformFailure => "perform-failure",
        EventKind::Rollback => "rollback",
        EventKind::AtomicCut => "atomic-cut",
        EventKind::Reject => "reject",
    }
}

fn extend_place_records(
    refs: &mut Vec<String>,
    place_store: &std::collections::BTreeMap<String, Vec<String>>,
    key: &str,
) {
    if let Some(records) = place_store.get(key) {
        for record in records {
            let prefix = if key.contains("debug") {
                format!("debug_output:{key}:")
            } else {
                format!("place_record:{key}:")
            };
            let formatted = format!("{prefix}{record}");
            if !refs.contains(&formatted) {
                refs.push(formatted);
            }
        }
    }
}
