//! Finite, source-first I2 conformance production/verifier boundary.
//!
//! This module deliberately consumes the checked/projected SYS-3--SYS-5
//! evidence rather than M10 release orchestration, JSON fixtures, or a second
//! source interpreter. The producer owns source loading, checking, projection,
//! admission, and selected execution. The verifier owns the fixed finite row
//! predicates and never supplies a route, Core fact, authority, or verdict to
//! the producer.

use std::{
    collections::BTreeSet,
    fmt, fs,
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::{
    sys2_bounded_model::{
        ExecutionProfile as ModelExecutionProfile, LitmusCase, RequiredEdge, Sys2BoundedModel,
    },
    sys3_projection::{
        BackendEligibility, BackendIneligibilityReason, BackendProfile, CommunicationEdgeKind,
        GlobalProjectionResult, ProjectedOperationFragmentKind,
    },
    sys4_dispatch::{
        ExternalAction, LocalFabric, SourceAction, Sys4DiagnosticKind, Sys4DispatchDiagnostics,
    },
    sys5_local_slice::{
        Sys5ArtifactSummary, Sys5CommunicationSummary, Sys5LocalAdmissionErrorKind,
        Sys5LocalProject, Sys5LocalRuntimeProfile, Sys5SourceInput, Sys5SourceSpan, build_project,
    },
    sys5_local_workflow::{
        Sys5LocalWorkflowInput, Sys5LocalWorkflowPatchProject, Sys5LocalWorkflowReport,
        Sys5LocalWorkflowStep, Sys5SourceBoundExecutionSummary, run_local_workflow_from_project,
    },
};

const REPORT_SCHEMA_VERSION: &str = "mirrorea-i2-conformance-report-v0";
const PROFILE_NAME: &str = "mirrorea-i2-systems-foundation-finite";
const PROFILE_SCOPE: &str = "bounded-finite-i2";
const BOUNDED_IMPLEMENTATION_SOURCE_FINGERPRINT_DOMAIN: &[u8] =
    b"mirrorea/i2/bounded-implementation-source-fingerprint/v1\0";
const MANIFEST_DOMAIN: &[u8] = b"mirrorea/i2/conformance-manifest/v1\0";
const SOURCE_CONTENT_DOMAIN: &[u8] = b"mirrorea/i2/source-content/v1\0";
const SELECTED_BACKEND_DOMAIN: &[u8] = b"mirrorea/i2/selected-backend/v1\0";

const REQUIRED_ROW_IDS: [&str; 22] = [
    "i2.ordinary_source_authority",
    "i2.checked_global_core_identity",
    "i2.core_to_locus_artifacts",
    "i2.generated_communication_complete",
    "i2.actual_dispatch_over_generated_edges",
    "i2.st_ow_selected_correspondence",
    "i2.owner_data_race_freedom_selected_backend",
    "i2.no_hidden_communication",
    "i2.no_direct_remote_store",
    "i2.no_source_free_authority_mint",
    "i2.no_source_free_state_mint",
    "i2.failure_containment",
    "i2.visibility_redaction_preserved",
    "i2.relation_projection_coherence",
    "i2.semantic_presentation_fallback_separation",
    "i2.designated_evaluator_non_reexecution",
    "i2.source_core_artifact_trace_correspondence",
    "i2.save_restore_consistent_local_cut",
    "i2.patch_lifecycle_checked",
    "i2.observer_safe_devtools",
    "i2.projection_determinism",
    "i2.non_claims_and_lifecycle_boundaries",
];

/// One source-first invocation of the finite I2 profile. Paths are read at
/// the host boundary and deliberately never appear in observer output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I2ConformanceInput {
    source_path: PathBuf,
    patch_paths: Vec<PathBuf>,
    selected_ow1_source_path: Option<PathBuf>,
    #[cfg(test)]
    test_falsifier: Option<I2ConformanceFalsifier>,
}

impl I2ConformanceInput {
    pub fn source_path(source_path: PathBuf) -> Self {
        Self {
            source_path,
            patch_paths: Vec::new(),
            selected_ow1_source_path: None,
            #[cfg(test)]
            test_falsifier: None,
        }
    }

    pub fn with_patch_path(mut self, patch_path: PathBuf) -> Self {
        self.patch_paths.push(patch_path);
        self
    }

    pub fn with_selected_ow1_source_path(mut self, source_path: PathBuf) -> Self {
        self.selected_ow1_source_path = Some(source_path);
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_falsifier(mut self, falsifier: I2ConformanceFalsifier) -> Self {
        self.test_falsifier = Some(falsifier);
        self
    }
}

/// Test-only corruption controls. Each case changes a clone of the actual
/// checked projection or raw execution observation before the verifier reads
/// it. They are not a command-line, source, or public compatibility surface.
#[cfg(test)]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum I2ConformanceFalsifier {
    RemoveGeneratedCommunicationEdge {
        operation_id: String,
        edge_kind: String,
    },
    InsertNonDerivedCommunicationEdge {
        edge_ref: String,
        operation_id: String,
        edge_kind: String,
        from_locus: String,
        to_locus: String,
    },
    MoveOwnerOperation {
        operation_id: String,
        from_locus: String,
        to_locus: String,
    },
    BreakSourceMap {
        operation_id: String,
        artifact_ref: String,
    },
    AdmitSourceFreeAuthority {
        principal: String,
        locus: String,
        operation_id: String,
    },
    MutateRemoteStore {
        locus: String,
        state: String,
        index: String,
        field: String,
        value: i64,
    },
    DivergeSelectedBackendTypedResult,
    DivergeSelectedBackendState,
    DivergeSelectedBackendFrontier,
    DivergeSelectedBackendTrace,
    CorruptOfflineCut,
    /// Raise the I2 exit bit only in the raw lifecycle candidate. It must
    /// reject the lifecycle row without activating any runtime capability.
    FlipLifecycleBoundaryClaim,
    /// Replace the retained diagnostic of one actual production control
    /// candidate. The verifier must reject the rows bound to that control
    /// rather than trusting the control's high-level label.
    SubstituteRuntimeControlDiagnostic {
        control_id: String,
        diagnostic_code: String,
    },
    /// Mark an executed evidence candidate unavailable. This verifies that
    /// every row treats a missing/failed positive or falsifier control as a
    /// fail-closed condition.
    FailBoundEvidence {
        evidence_id: String,
    },
    /// Simulate a sealed-admission candidate that admits a manual route. It
    /// must fail the ordinary-source and generated-communication boundaries
    /// without changing active fabric state.
    AdmitManualRouteOrInterface {
        operation_id: String,
        from_locus: String,
        to_locus: String,
    },
    /// Remove an actual required provenance anchor from the raw verifier
    /// input. The verifier must fail precisely the row(s) whose required
    /// domain contract can no longer be joined; no primary fallback is valid.
    RemoveRequiredProvenanceAnchor {
        row_id: String,
    },
}

/// Source/input failure is typed and intentionally omits the host path and
/// source text. The CLI translates it to a redacted JSON diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2ConformanceError {
    MissingPrimarySource,
    MissingSelectedOw1Source,
    InvalidPrimarySource,
    InvalidPatchSource,
    InvalidSelectedOw1Source,
    PatchSetIncomplete,
    WorkflowRejected,
    SelectedBackendRejected,
    ModelEvidenceRejected,
}

impl I2ConformanceError {
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::MissingPrimarySource => "i2_source_path_io_error",
            Self::MissingSelectedOw1Source => "i2_selected_ow1_source_missing",
            Self::InvalidPrimarySource => "i2_source_check_or_projection_error",
            Self::InvalidPatchSource => "i2_patch_check_or_projection_error",
            Self::InvalidSelectedOw1Source => "i2_selected_ow1_check_or_projection_error",
            Self::PatchSetIncomplete => "i2_patch_set_incomplete",
            Self::WorkflowRejected => "i2_primary_workflow_rejected",
            Self::SelectedBackendRejected => "i2_selected_backend_rejected",
            Self::ModelEvidenceRejected => "i2_model_evidence_rejected",
        }
    }
}

impl fmt::Display for I2ConformanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.diagnostic_code())
    }
}

impl std::error::Error for I2ConformanceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2ConformanceStatus {
    Accepted,
    Rejected,
}

impl Serialize for I2ConformanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum I2RowStatus {
    Pass,
    Fail,
}

impl Serialize for I2RowStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
        })
    }
}

/// A normalized checked source range without source text or host path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct I2SourceSpan {
    start: u64,
    end: u64,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

/// One fixed finite I2 row. Every row has concrete evidence anchors; the
/// profile has no `N/A`, waiver, or unknown status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct I2ConformanceRow {
    id: String,
    status: I2RowStatus,
    scope: String,
    evidence_class: String,
    provenance_anchor_ref: String,
    checked_program_identity_ref: String,
    source_span: I2SourceSpan,
    core_ref: String,
    artifact_ref: String,
    edge_ref: String,
    request_identity: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    dispatch_occurrence_ref: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    receive_occurrence_ref: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    serve_occurrence_ref: String,
    authority_source: String,
    locus_program_ref: String,
    positive_evidence_refs: Vec<String>,
    falsifier_evidence_refs: Vec<String>,
    controls: Vec<I2Control>,
    subclaims: Vec<I2OwnerPreservationSubclaim>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_ow1_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_ow1_parse: Option<I2SelectedParseTelemetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_ow1_projection: Option<I2SelectedProjectionTelemetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    st_backend_telemetry: Option<I2BackendTelemetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ow1_backend_telemetry: Option<I2BackendTelemetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    st_semantic_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ow1_semantic_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    four_locus_ow1_workflow_claimed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_toy_ow1_residual: Option<I2BackendResidual>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2Control {
    id: String,
    evidence_ref: String,
    outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2OwnerPreservationSubclaim {
    id: String,
    status: I2RowStatus,
    operation_id: String,
    expected_owner_locus: String,
    observed_owner_locus: String,
    evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2SelectedParseTelemetry {
    locus_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2SelectedProjectionTelemetry {
    count_source: String,
    artifact_count: usize,
    generated_edge_count: usize,
    actual_artifact_refs: Vec<String>,
    actual_generated_edge_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2BackendTelemetry {
    runtime_profile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_worker_locus: Option<String>,
    worker_owned_m8: bool,
    mailbox_fifo: bool,
    lifecycle_refs: Vec<String>,
    typed_receipt_ref: String,
    typed_result_ref: String,
    state_digest: String,
    frontier_ref: String,
    trace_digest: String,
    action_outcomes: Vec<I2SelectedActionTelemetry>,
    all_actions_succeeded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    same_mailbox_fifo_control: Option<I2SameMailboxFifoControl>,
}

/// Per-action result from an actual selected-backend dispatch.  A failed
/// dispatch remains an attempted action but is never reported as completed
/// correspondence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2SelectedActionTelemetry {
    action_ref: String,
    attempted: bool,
    completed: bool,
    status: String,
    result_kind: String,
    typed_result_ref: String,
    receipt_occurrence_ref: String,
    attempted_provenance_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostic_code: Option<String>,
}

/// Exact two-message FIFO witness copied from SYS-4's retained generated
/// endpoint trace. Both requests are actually enqueued into one owner mailbox
/// before the first serve, then complete through generated reply paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2SameMailboxFifoControl {
    source: String,
    all_actions_succeeded: bool,
    same_mailbox_owner_locus: String,
    request_ids: Vec<String>,
    enqueue_order: Vec<String>,
    serve_order: Vec<String>,
    second_enqueued_before_first_serve: bool,
    typed_receipt_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2BackendResidual {
    diagnostic_code: String,
    reason: String,
    profile: String,
    admission_phase: String,
    typed_admission_reason: I2TypedAdmissionReason,
    mutated_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2TypedAdmissionReason {
    code: String,
    owner_loci: Vec<String>,
    source_owner_loci: Vec<String>,
    owner_loci_semantics: String,
    source_owner_loci_semantics: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2EvidenceInventories {
    checked_program_identity_refs: Vec<String>,
    core_refs: Vec<String>,
    artifact_refs: Vec<String>,
    communication_edge_refs: Vec<String>,
    request_identity_refs: Vec<String>,
    runtime_occurrence_refs: Vec<String>,
    provenance_anchors: Vec<I2ProvenanceAnchor>,
    executed_evidence: Vec<I2ExecutedEvidence>,
    source_first_causal_provenance: Vec<I2SourceFirstCausalProvenance>,
}

/// Typed, observer-safe source-first provenance. These records are produced
/// from the checked project, sealed admission, and retained SYS-4 occurrence
/// inventory; they replace legacy "not used" booleans in the I2 JSON.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2SourceFirstCausalProvenance {
    id: String,
    kind: String,
    source: String,
    produced_by: String,
    row_ids: Vec<String>,
    property_ids: Vec<String>,
    producer_invocation_ref: String,
    typed_producer_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_content_identity_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logical_source_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_span_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    checked_program_identity_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sealed_admission_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_route_or_interface_admitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_core_or_authority_injection_admitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_state_injection_admitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    communication_plan_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_occurrence_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_unchanged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation_applied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostic_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_before_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_after_ref: Option<String>,
}

/// A property-specific reference copied from actual SYS-5 workflow rows or
/// SYS-2 model output. It never invents a Core/artifact/edge identifier just
/// to make a cross-join look complete.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ProvenanceAnchor {
    id: String,
    domain: String,
    produced_by: String,
    kind: String,
    source: String,
    causal_segment_ref: String,
    lifecycle_ref: String,
    model_ref: String,
    row_ids: Vec<String>,
    property_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_worker_locus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_locus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    designated_evaluator_locus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_locus: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    source_loci: Vec<String>,
    #[serde(skip)]
    runtime_anchor: RuntimeAnchor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ExecutedEvidence {
    id: String,
    kind: String,
    outcome: String,
    evidence_class: String,
    produced_by: String,
    row_ids: Vec<String>,
    property_ids: Vec<String>,
    control_ref: String,
    producer_invocation_ref: String,
    executed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_identity_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_identity_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostic_code: Option<String>,
    /// The exact SYS-4 typed diagnostic observed at the production boundary.
    /// This remains separate from the I2 control label (for example,
    /// `OfflineCutCorruption`) so a generic runtime error cannot be relabelled
    /// as a successful falsifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    observed_runtime_diagnostic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_endpoint_attempt_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline_cut_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    corruption_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_before_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_after_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_unchanged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    candidate_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redacted_output_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker_present_in_candidate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker_absent_after_redaction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker_bearing_report_candidate_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redacted_serialized_output_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_boundary_candidate: Option<I2LifecycleBoundaryCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overclaim_candidate: Option<I2LifecycleBoundaryCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2LifecycleState {
    broad_i1_exit_accepted: bool,
    i2_entry_accepted: bool,
    i2_exit_accepted: bool,
    sys7_goal_active: bool,
    i3_program_active: bool,
    public_transport_claim: bool,
    real_transport_selected: bool,
    production_deployment_claim: bool,
}

/// Observer-safe materialization of a typed lifecycle candidate. It records
/// a candidate's declared lifecycle bits and the actual no-runtime-mutation
/// result of inspecting it; it never grants a lifecycle transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2LifecycleBoundaryCandidate {
    source: String,
    candidate_ref: String,
    producer_invocation_ref: String,
    broad_i1_exit_accepted: bool,
    i2_entry_accepted: bool,
    i2_exit_accepted: bool,
    sys7_goal_active: bool,
    i3_program_active: bool,
    public_transport_claim: bool,
    real_transport_selected: bool,
    production_deployment_claim: bool,
    accepted: bool,
    mutation_applied: bool,
}

/// Raw, typed lifecycle candidate owned by the I2 producer.  The verifier
/// reads this value and decides whether it remains inside the deliberately
/// preacceptance I2 boundary; it does not trust a precomputed row verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct LifecycleBoundaryState {
    broad_i1_exit_accepted: bool,
    i2_entry_accepted: bool,
    i2_exit_accepted: bool,
    sys7_goal_active: bool,
    i3_program_active: bool,
    public_transport_claim: bool,
    real_transport_selected: bool,
    production_deployment_claim: bool,
    non_claims: Vec<String>,
}

impl LifecycleBoundaryState {
    fn preacceptance() -> Self {
        Self {
            broad_i1_exit_accepted: false,
            i2_entry_accepted: false,
            i2_exit_accepted: false,
            sys7_goal_active: false,
            i3_program_active: false,
            public_transport_claim: false,
            real_transport_selected: false,
            production_deployment_claim: false,
            non_claims: non_claims(),
        }
    }

    fn is_preacceptance_boundary(&self) -> bool {
        !self.broad_i1_exit_accepted
            && !self.i2_entry_accepted
            && !self.i2_exit_accepted
            && !self.sys7_goal_active
            && !self.i3_program_active
            && !self.public_transport_claim
            && !self.real_transport_selected
            && !self.production_deployment_claim
            && self.non_claims == non_claims()
    }

    fn i2_exit_overclaim_candidate(&self) -> Self {
        let mut candidate = self.clone();
        candidate.i2_exit_accepted = true;
        candidate.public_transport_claim = true;
        candidate
    }

    fn observer_state(&self) -> I2LifecycleState {
        I2LifecycleState {
            broad_i1_exit_accepted: self.broad_i1_exit_accepted,
            i2_entry_accepted: self.i2_entry_accepted,
            i2_exit_accepted: self.i2_exit_accepted,
            sys7_goal_active: self.sys7_goal_active,
            i3_program_active: self.i3_program_active,
            public_transport_claim: self.public_transport_claim,
            real_transport_selected: self.real_transport_selected,
            production_deployment_claim: self.production_deployment_claim,
        }
    }
}

/// Provisional source fingerprint for the bounded I2 implementation evidence.
/// It is intentionally not a runtime identity, accepted release cut, or Git
/// revision claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2BoundedImplementationSourceFingerprint {
    id: String,
    scope: String,
    runtime_identity_claim: bool,
    public_release_cut: bool,
    source_components: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2Rejection {
    mutation_stage: String,
    diagnostic_code: String,
    validator_invocation: I2ValidatorInvocation,
    candidate_identity_before: String,
    candidate_identity_after: String,
    snapshots: I2StableSnapshots,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_endpoint_attempt: Option<I2RuntimeEndpointAttempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offline_cut_candidate: Option<I2OfflineCutCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_backend_divergence: Option<I2SelectedBackendDivergence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_boundary_candidate: Option<I2LifecycleBoundaryCandidate>,
    #[cfg(test)]
    #[serde(skip_serializing_if = "Option::is_none")]
    control_diagnostic_candidate: Option<I2ControlDiagnosticCandidate>,
    #[cfg(test)]
    #[serde(skip_serializing_if = "Option::is_none")]
    executed_evidence_candidate: Option<I2ExecutedEvidenceCandidate>,
    #[cfg(test)]
    #[serde(skip_serializing_if = "Option::is_none")]
    manual_route_or_interface_candidate: Option<I2ManualRouteOrInterfaceCandidate>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ControlDiagnosticCandidate {
    source: String,
    control_id: String,
    expected_diagnostic_code: String,
    observed_diagnostic_code: String,
    accepted: bool,
    mutation_applied: bool,
    candidate_ref: String,
    producer_invocation_ref: String,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ExecutedEvidenceCandidate {
    source: String,
    evidence_id: String,
    outcome: String,
    executed: bool,
    accepted: bool,
    affected_row_ids: Vec<String>,
    candidate_ref: String,
    producer_invocation_ref: String,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ManualRouteOrInterfaceCandidate {
    source: String,
    manual_route_or_interface_admitted: bool,
    accepted: bool,
    mutation_applied: bool,
    candidate_ref: String,
    producer_invocation_ref: String,
    semantic_state_before: String,
    semantic_state_after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ValidatorInvocation {
    invoked: bool,
    result: String,
    validator_invocation_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2StableSnapshots {
    semantic_before: String,
    semantic_after: String,
    runtime_before: String,
    runtime_after: String,
    authority_before: String,
    authority_after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2RuntimeEndpointAttempt {
    diagnostic_code: String,
    attempt_ref: String,
    producer_invocation_ref: String,
    underlying_state_before: String,
    underlying_state_after: String,
    authority_state_before: String,
    authority_state_after: String,
    semantic_state_before: String,
    semantic_state_after: String,
    mutation_applied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2OfflineCutCandidate {
    diagnostic_code: String,
    source: String,
    cut_ref: String,
    restore_attempt_ref: String,
    restore_result: String,
    mutation_applied: bool,
    state_digest_before: String,
    state_digest_after: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2SelectedBackendDivergence {
    diagnostic_code: String,
    source: String,
    control_ref: String,
    producer_invocation_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    st_typed_result_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ow1_typed_result_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    st_state_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ow1_state_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    st_frontier_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ow1_frontier_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    st_trace_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ow1_trace_digest: Option<String>,
}

/// Deterministic, observer-safe result of the finite I2 verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct I2ConformanceReport {
    schema_version: String,
    command: String,
    profile_name: String,
    profile_scope: String,
    status: I2ConformanceStatus,
    source_authority: String,
    public_api_or_wire_contract: bool,
    final_public_api_frozen: bool,
    public_wire_frozen: bool,
    checked_program_identity_ref: String,
    artifact_inventory_digest: String,
    source_bound_execution: Sys5SourceBoundExecutionSummary,
    bounded_implementation_source_fingerprint: I2BoundedImplementationSourceFingerprint,
    i2_manifest_identity_ref: String,
    loci: Vec<String>,
    rows: Vec<I2ConformanceRow>,
    inventories: I2EvidenceInventories,
    lifecycle_state: I2LifecycleState,
    evidence_classes: Vec<String>,
    non_claims: Vec<String>,
    #[cfg(test)]
    #[serde(skip_serializing_if = "Option::is_none")]
    test_only_falsifier: Option<bool>,
    #[cfg(test)]
    #[serde(skip_serializing_if = "Option::is_none")]
    rejection: Option<I2Rejection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typed_rejection: Option<I2TypedRejection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2TypedRejection {
    diagnostic_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    observer_policy: Option<I2ObserverPolicy>,
}

/// A fail-closed observer policy result. It keeps only a fixed redaction
/// label; raw identifiers never cross the report boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2ObserverPolicy {
    action: String,
    redacted_identifiers: Vec<I2RedactedIdentifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct I2RedactedIdentifier {
    redacted_as: String,
}

impl I2ConformanceReport {
    pub const fn status(&self) -> I2ConformanceStatus {
        self.status
    }

    /// The only JSON view for an external observer. It clones and redacts the
    /// report immediately before serialization so no caller, including the
    /// CLI, can accidentally serialize a raw intermediate value.
    pub fn observer_safe_value(&self) -> serde_json::Value {
        let mut observer_view = self.clone();
        observer_view.redact_observer_sensitive_identifiers();
        serde_json::to_value(observer_view).expect("I2 conformance report is serializable")
    }

    /// A stable compact observer serialization used only for deterministic
    /// finite reproduction. It carries redacted observer data, never raw
    /// sources, credentials, capability/witness material, or host paths.
    pub fn render_compact(&self) -> String {
        serde_json::to_string(&self.observer_safe_value())
            .expect("I2 observer-safe conformance report is serializable")
    }

    fn redact_observer_sensitive_identifiers(&mut self) {
        redact_observer_string(&mut self.checked_program_identity_ref);
        redact_observer_string(&mut self.artifact_inventory_digest);
        self.source_bound_execution
            .redact_observer_sensitive_identifiers();
        redact_observer_string(&mut self.bounded_implementation_source_fingerprint.id);
        redact_observer_string(&mut self.bounded_implementation_source_fingerprint.scope);
        redact_string_vec(
            &mut self
                .bounded_implementation_source_fingerprint
                .source_components,
        );
        redact_observer_string(&mut self.i2_manifest_identity_ref);
        for locus in &mut self.loci {
            redact_observer_string(locus);
        }
        redact_string_vec(&mut self.inventories.checked_program_identity_refs);
        for entry in &mut self.inventories.core_refs {
            redact_observer_string(entry);
        }
        for entry in &mut self.inventories.artifact_refs {
            redact_observer_string(entry);
        }
        for entry in &mut self.inventories.communication_edge_refs {
            redact_observer_string(entry);
        }
        redact_string_vec(&mut self.inventories.request_identity_refs);
        redact_string_vec(&mut self.inventories.runtime_occurrence_refs);
        for anchor in &mut self.inventories.provenance_anchors {
            redact_observer_string(&mut anchor.id);
            redact_observer_string(&mut anchor.domain);
            redact_observer_string(&mut anchor.produced_by);
            redact_observer_string(&mut anchor.kind);
            redact_observer_string(&mut anchor.source);
            redact_observer_string(&mut anchor.causal_segment_ref);
            redact_observer_string(&mut anchor.lifecycle_ref);
            redact_observer_string(&mut anchor.model_ref);
            redact_string_vec(&mut anchor.row_ids);
            redact_string_vec(&mut anchor.property_ids);
            redact_option_string(&mut anchor.owner_worker_locus);
            redact_option_string(&mut anchor.requester_locus);
            redact_option_string(&mut anchor.designated_evaluator_locus);
            redact_option_string(&mut anchor.consumer_locus);
            redact_string_vec(&mut anchor.source_loci);
        }
        for evidence in &mut self.inventories.executed_evidence {
            redact_observer_string(&mut evidence.id);
            redact_observer_string(&mut evidence.kind);
            redact_observer_string(&mut evidence.outcome);
            redact_observer_string(&mut evidence.evidence_class);
            redact_observer_string(&mut evidence.produced_by);
            redact_string_vec(&mut evidence.row_ids);
            redact_string_vec(&mut evidence.property_ids);
            redact_observer_string(&mut evidence.control_ref);
            redact_observer_string(&mut evidence.producer_invocation_ref);
            redact_option_string(&mut evidence.candidate_identity_before);
            redact_option_string(&mut evidence.candidate_identity_after);
            redact_option_string(&mut evidence.diagnostic_code);
            redact_option_string(&mut evidence.observed_runtime_diagnostic);
            redact_option_string(&mut evidence.runtime_endpoint_attempt_ref);
            redact_option_string(&mut evidence.offline_cut_ref);
            redact_option_string(&mut evidence.corruption_kind);
            redact_option_string(&mut evidence.state_before_digest);
            redact_option_string(&mut evidence.state_after_digest);
            redact_option_string(&mut evidence.candidate_source);
            redact_option_string(&mut evidence.redacted_output_source);
            redact_option_string(&mut evidence.marker_bearing_report_candidate_ref);
            redact_option_string(&mut evidence.redacted_serialized_output_ref);
        }
        for provenance in &mut self.inventories.source_first_causal_provenance {
            redact_observer_string(&mut provenance.id);
            redact_observer_string(&mut provenance.kind);
            redact_observer_string(&mut provenance.source);
            redact_observer_string(&mut provenance.produced_by);
            redact_string_vec(&mut provenance.row_ids);
            redact_string_vec(&mut provenance.property_ids);
            redact_observer_string(&mut provenance.producer_invocation_ref);
            redact_observer_string(&mut provenance.typed_producer_ref);
            redact_option_string(&mut provenance.source_content_identity_ref);
            redact_option_string(&mut provenance.logical_source_ref);
            redact_option_string(&mut provenance.source_span_ref);
            redact_option_string(&mut provenance.checked_program_identity_ref);
            redact_option_string(&mut provenance.sealed_admission_ref);
            redact_option_string(&mut provenance.routing_source);
            redact_option_string(&mut provenance.communication_plan_ref);
            redact_option_string(&mut provenance.edge_ref);
            redact_option_string(&mut provenance.dispatch_occurrence_ref);
            redact_option_string(&mut provenance.candidate_kind);
            redact_option_string(&mut provenance.diagnostic_code);
            redact_option_string(&mut provenance.candidate_ref);
            redact_option_string(&mut provenance.state_before_ref);
            redact_option_string(&mut provenance.state_after_ref);
        }
        for row in &mut self.rows {
            redact_observer_string(&mut row.id);
            redact_observer_string(&mut row.scope);
            redact_observer_string(&mut row.evidence_class);
            redact_observer_string(&mut row.provenance_anchor_ref);
            redact_observer_string(&mut row.checked_program_identity_ref);
            redact_observer_string(&mut row.core_ref);
            redact_observer_string(&mut row.artifact_ref);
            redact_observer_string(&mut row.edge_ref);
            redact_observer_string(&mut row.request_identity);
            redact_observer_string(&mut row.dispatch_occurrence_ref);
            redact_observer_string(&mut row.receive_occurrence_ref);
            redact_observer_string(&mut row.serve_occurrence_ref);
            redact_observer_string(&mut row.authority_source);
            redact_observer_string(&mut row.locus_program_ref);
            redact_string_vec(&mut row.positive_evidence_refs);
            redact_string_vec(&mut row.falsifier_evidence_refs);
            for control in &mut row.controls {
                redact_observer_string(&mut control.id);
                redact_observer_string(&mut control.evidence_ref);
                redact_observer_string(&mut control.outcome);
            }
            for subclaim in &mut row.subclaims {
                redact_observer_string(&mut subclaim.id);
                redact_observer_string(&mut subclaim.operation_id);
                redact_observer_string(&mut subclaim.expected_owner_locus);
                redact_observer_string(&mut subclaim.observed_owner_locus);
                redact_observer_string(&mut subclaim.evidence_ref);
            }
            redact_option_string(&mut row.selected_ow1_source);
            if let Some(projection) = &mut row.selected_ow1_projection {
                redact_observer_string(&mut projection.count_source);
                redact_string_vec(&mut projection.actual_artifact_refs);
                redact_string_vec(&mut projection.actual_generated_edge_refs);
            }
            if let Some(telemetry) = &mut row.st_backend_telemetry {
                redact_backend_telemetry(telemetry);
            }
            if let Some(telemetry) = &mut row.ow1_backend_telemetry {
                redact_backend_telemetry(telemetry);
            }
            redact_option_string(&mut row.st_semantic_digest);
            redact_option_string(&mut row.ow1_semantic_digest);
            if let Some(residual) = &mut row.full_toy_ow1_residual {
                redact_observer_string(&mut residual.diagnostic_code);
                redact_observer_string(&mut residual.reason);
                redact_observer_string(&mut residual.profile);
                redact_observer_string(&mut residual.admission_phase);
                redact_observer_string(&mut residual.typed_admission_reason.code);
                redact_string_vec(&mut residual.typed_admission_reason.owner_loci);
                redact_string_vec(&mut residual.typed_admission_reason.source_owner_loci);
                redact_observer_string(&mut residual.typed_admission_reason.owner_loci_semantics);
                redact_observer_string(
                    &mut residual.typed_admission_reason.source_owner_loci_semantics,
                );
            }
        }
    }
}

/// Exercise the exact report redaction and serialization path with a
/// source-controlled sensitive identifier. The control is positive only when
/// the raw marker is absent from the rendered observer view and the typed
/// policy records redaction.
fn observer_policy_renderer_redacts_candidate() -> ObserverPolicyControl {
    let marker = "credential_secret_control_candidate".to_string();
    let report = I2ConformanceReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        command: "conform-i2".to_string(),
        profile_name: PROFILE_NAME.to_string(),
        profile_scope: PROFILE_SCOPE.to_string(),
        status: I2ConformanceStatus::Rejected,
        source_authority: "ordinary_mir_source".to_string(),
        public_api_or_wire_contract: false,
        final_public_api_frozen: false,
        public_wire_frozen: false,
        checked_program_identity_ref: marker.clone(),
        artifact_inventory_digest: marker.clone(),
        source_bound_execution: Sys5SourceBoundExecutionSummary::observer_policy_control(
            marker.clone(),
        ),
        bounded_implementation_source_fingerprint: I2BoundedImplementationSourceFingerprint {
            id: "i2-observer-policy-control".to_string(),
            scope: "i2-provisional-implementation-source".to_string(),
            runtime_identity_claim: false,
            public_release_cut: false,
            source_components: vec![marker.clone()],
        },
        i2_manifest_identity_ref: "i2-observer-policy-control".to_string(),
        loci: vec![marker.clone()],
        rows: Vec::new(),
        inventories: I2EvidenceInventories {
            checked_program_identity_refs: vec![marker.clone()],
            core_refs: vec![marker.clone()],
            artifact_refs: vec![marker.clone()],
            communication_edge_refs: vec![marker.clone()],
            request_identity_refs: vec![marker.clone()],
            runtime_occurrence_refs: vec![marker.clone()],
            provenance_anchors: Vec::new(),
            executed_evidence: Vec::new(),
            source_first_causal_provenance: Vec::new(),
        },
        lifecycle_state: I2LifecycleState {
            broad_i1_exit_accepted: false,
            i2_entry_accepted: false,
            i2_exit_accepted: false,
            sys7_goal_active: false,
            i3_program_active: false,
            public_transport_claim: false,
            real_transport_selected: false,
            production_deployment_claim: false,
        },
        evidence_classes: Vec::new(),
        non_claims: Vec::new(),
        #[cfg(test)]
        test_only_falsifier: None,
        #[cfg(test)]
        rejection: None,
        typed_rejection: Some(I2TypedRejection {
            diagnostic_code: "ObserverSensitiveIdentifier".to_string(),
            observer_policy: Some(I2ObserverPolicy {
                action: "redact".to_string(),
                redacted_identifiers: vec![I2RedactedIdentifier {
                    redacted_as: "[redacted-observer-sensitive-identifier]".to_string(),
                }],
            }),
        }),
    };
    let candidate_before = control_identity(
        "observer-policy-marker-bearing-candidate",
        &serde_json::to_string(&report).expect("observer policy candidate is serializable"),
    );
    let rendered = report.render_compact();
    let candidate_after = control_identity("observer-policy-redacted-render", &rendered);
    let marker_present_in_candidate = serde_json::to_string(&report)
        .expect("observer policy candidate is serializable")
        .contains(&marker);
    let marker_absent_after_redaction = !rendered.contains(&marker);
    let detected = marker_present_in_candidate
        && marker_absent_after_redaction
        && rendered.contains("[redacted-observer-sensitive-identifier]")
        && report
            .typed_rejection
            .as_ref()
            .and_then(|rejection| rejection.observer_policy.as_ref())
            .is_some_and(|policy| policy.action == "redact");
    ObserverPolicyControl {
        detected,
        candidate_before,
        candidate_after,
        marker_present_in_candidate,
        marker_absent_after_redaction,
        producer_invocation_ref: control_identity(
            "observer-policy-renderer-invocation",
            &(&report.schema_version, &report.command, &rendered),
        ),
    }
}

/// Run the producer then verify its raw typed evidence. The producer has no
/// predicate/verdict input; the verifier has no source parser, runtime plan,
/// authority issuer, or expected-result lookup.
pub fn run_i2_conformance(
    input: I2ConformanceInput,
) -> Result<I2ConformanceReport, I2ConformanceError> {
    #[cfg(not(test))]
    let evidence = I2EvidenceProducer::produce(&input)?;
    #[cfg(test)]
    let mut evidence = I2EvidenceProducer::produce(&input)?;
    #[cfg(test)]
    if let Some(falsifier) = input.test_falsifier {
        evidence.apply_test_falsifier(falsifier);
    }
    Ok(I2ConformanceVerifier::verify(evidence))
}

struct I2EvidenceProducer;

impl I2EvidenceProducer {
    fn produce(input: &I2ConformanceInput) -> Result<RawI2Evidence, I2ConformanceError> {
        let primary_source =
            read_source(&input.source_path, I2ConformanceError::MissingPrimarySource)?;
        let primary_content_identity_ref = source_content_identity_ref(&primary_source);
        let primary_project = build_project(Sys5SourceInput::inline(
            "i2-primary-source.mir",
            primary_source.clone(),
        ))
        .map_err(|_| I2ConformanceError::InvalidPrimarySource)?;
        let replay_project = build_project(Sys5SourceInput::inline(
            "i2-primary-source.mir",
            primary_source.clone(),
        ))
        .map_err(|_| I2ConformanceError::InvalidPrimarySource)?;
        let projection_deterministic = primary_project.semantic_summary()
            == replay_project.semantic_summary()
            && primary_project.checked_program_identity_ref()
                == replay_project.checked_program_identity_ref();

        if input.patch_paths.len() < 2 {
            return Err(I2ConformanceError::PatchSetIncomplete);
        }
        let mut workflow_input = Sys5LocalWorkflowInput::from_project_and_admission(
            primary_project.clone(),
            primary_project
                .prepare_canonical_local_st_admission()
                .map_err(|_| I2ConformanceError::WorkflowRejected)?,
        );
        let mut patch_identity_refs = Vec::new();
        for (index, patch_path) in input.patch_paths.iter().enumerate() {
            let patch_source = read_source(patch_path, I2ConformanceError::InvalidPatchSource)?;
            let patch_project = build_project(Sys5SourceInput::inline(
                "i2-primary-source.mir",
                patch_source,
            ))
            .map_err(|_| I2ConformanceError::InvalidPatchSource)?;
            let patch_identity = patch_project.checked_program_identity_ref().to_string();
            let patch_admission = patch_project
                .prepare_canonical_local_st_admission()
                .map_err(|_| I2ConformanceError::WorkflowRejected)?;
            workflow_input = workflow_input.with_patch_project(
                Sys5LocalWorkflowPatchProject::from_project_and_admission(
                    format!("i2-patch-{:03}", index + 1),
                    patch_project,
                    patch_admission,
                )
                .with_cli_patch_ordinal(
                    NonZeroUsize::new(index + 1).expect("enumerated I2 patch ordinal is nonzero"),
                ),
            );
            patch_identity_refs.push(patch_identity);
        }
        let workflow = run_local_workflow_from_project(workflow_input)
            .map_err(|_| I2ConformanceError::WorkflowRejected)?;

        let selected_ow1_path = input
            .selected_ow1_source_path
            .as_deref()
            .ok_or(I2ConformanceError::MissingSelectedOw1Source)?;
        let selected_ow1_source = read_source(
            selected_ow1_path,
            I2ConformanceError::MissingSelectedOw1Source,
        )?;
        let selected_ow1_logical_source = observer_logical_source_path(selected_ow1_path);
        let selected_ow1_content_identity_ref = source_content_identity_ref(&selected_ow1_source);
        let selected_st = run_selected_backend(
            &selected_ow1_source,
            Sys5LocalRuntimeProfile::St,
            "i2-selected-ow1-source.mir",
        )?;
        let selected_ow1 = run_selected_backend(
            &selected_ow1_source,
            Sys5LocalRuntimeProfile::Ow1,
            "i2-selected-ow1-source.mir",
        )?;

        let model_st = selected_model(ModelExecutionProfile::SingleThread);
        let model_ow1 = selected_model(ModelExecutionProfile::OneOwnerWorker);
        if !model_st.passes_all_litmus()
            || !model_ow1.passes_all_litmus()
            || !model_st.selected_semantic_results_match(&model_ow1)
        {
            return Err(I2ConformanceError::ModelEvidenceRejected);
        }

        let expected_projection = ProjectionFacts::from_summary(primary_project.semantic_summary());
        let observed_projection =
            ProjectionFacts::from_projection(primary_project.projected_result_for_i2_evidence());
        let full_toy_ow1_residual =
            match primary_project.prepare_canonical_local_admission(Sys5LocalRuntimeProfile::Ow1) {
                Ok(_) => I2BackendResidual {
                    diagnostic_code: "UnexpectedOw1Admission".to_string(),
                    reason: "FullToyOw1WasUnexpectedlyAdmitted".to_string(),
                    profile: "OW1".to_string(),
                    admission_phase: "canonical_local_admission".to_string(),
                    typed_admission_reason: I2TypedAdmissionReason {
                        code: "UnexpectedOw1Admission".to_string(),
                        owner_loci: Vec::new(),
                        source_owner_loci: Vec::new(),
                        owner_loci_semantics: "runtime_combined_owner_loci".to_string(),
                        source_owner_loci_semantics: "source_declared_owner_loci".to_string(),
                    },
                    mutated_state: false,
                },
                Err(error) => full_toy_ow1_residual(
                    error.kind(),
                    primary_project
                        .projected_result_for_i2_evidence()
                        .backend_requirements()
                        .eligibility(BackendProfile::Ow1),
                    primary_project.projected_result_for_i2_evidence(),
                ),
            };
        let anchor = RuntimeAnchor::from_workflow(&workflow, &expected_projection)
            .ok_or(I2ConformanceError::WorkflowRejected)?;
        let model_st_fingerprint = control_identity(
            "sys2-model-st-fingerprint",
            &model_st.deterministic_fingerprint(),
        );
        let model_ow1_fingerprint = control_identity(
            "sys2-model-ow1-fingerprint",
            &model_ow1.deterministic_fingerprint(),
        );
        let provenance_anchors = actual_provenance_anchors(
            &workflow,
            &expected_projection,
            &anchor,
            &selected_st,
            &model_ow1_fingerprint,
        );
        let lifecycle_boundary = LifecycleBoundaryState::preacceptance();
        let production_controls = run_production_controls(
            &primary_project,
            &workflow,
            &model_st,
            &model_ow1,
            &lifecycle_boundary,
        );
        Ok(RawI2Evidence {
            primary_content_identity_ref,
            selected_ow1_content_identity_ref,
            selected_ow1_logical_source,
            primary_checked_program_identity_ref: primary_project
                .checked_program_identity_ref()
                .to_string(),
            patch_identity_refs,
            expected_projection,
            observed_projection,
            workflow,
            selected_st,
            selected_ow1,
            model_st_fingerprint,
            model_ow1_fingerprint,
            model_no_source_free_authority_mints: model_st.no_source_free_authority_mints()
                && model_ow1.no_source_free_authority_mints(),
            model_no_stale_authority_use: model_st.no_stale_authority_use()
                && model_ow1.no_stale_authority_use(),
            projection_deterministic,
            full_toy_ow1_residual,
            anchor,
            provenance_anchors,
            projection_validator_positive: production_controls.projection_validator_positive,
            projection_missing_edge_control_detected: production_controls
                .projection_missing_edge_control_detected,
            projection_extra_edge_control_detected: production_controls
                .projection_extra_edge_control_detected,
            runtime_authority_control_detected: production_controls
                .runtime_authority_control_detected,
            runtime_store_control_detected: production_controls.runtime_store_control_detected,
            runtime_source_free_state_mint_control_detected: production_controls
                .runtime_source_free_state_mint_control_detected,
            manual_route_or_interface_rejected: production_controls
                .manual_route_or_interface_rejected,
            observer_control_detected: production_controls.observer_control_detected,
            lifecycle_boundary,
            lifecycle_control: production_controls.lifecycle_control,
            executed_evidence: production_controls.executed_evidence,
            observer_sensitive_source: contains_observer_sensitive_identifier(&primary_source)
                || contains_observer_sensitive_identifier(&selected_ow1_source),
            #[cfg(test)]
            test_projection: primary_project.clone_projection_for_i2_test(),
            #[cfg(test)]
            test_primary_source: primary_source,
            #[cfg(test)]
            test_rejection: None,
        })
    }
}

/// The fixed finite verifier. It only reads `RawI2Evidence` and records a
/// pass/fail result; it never runs parsing, admission, a scheduler, or a
/// conformance-selected semantic shortcut.
struct I2ConformanceVerifier;

impl I2ConformanceVerifier {
    fn verify(evidence: RawI2Evidence) -> I2ConformanceReport {
        let workflow = &evidence.workflow;
        let expected = &evidence.expected_projection;
        let observed = &evidence.observed_projection;
        #[cfg(test)]
        let test_rejection = evidence.test_rejection.clone();
        #[cfg(not(test))]
        let test_rejection: Option<I2Rejection> = None;
        let rejected_control = |code: &str| {
            test_rejection
                .as_ref()
                .is_some_and(|rejection| rejection.diagnostic_code == code)
        };
        let complete_edges = expected.edges == observed.edges
            && observed
                .edges
                .iter()
                .all(|edge| edge.derived_from_checked_core)
            && evidence.projection_validator_positive
            && evidence.projection_missing_edge_control_detected;
        let no_hidden_communication = observed.edges.iter().all(|edge| {
            edge.derived_from_checked_core && !edge.transfers_authority && edge.core_ref.is_some()
        }) && evidence.projection_extra_edge_control_detected
            && evidence.manual_route_or_interface_rejected;
        let artifact_identity = expected.artifacts == observed.artifacts
            && observed
                .artifacts
                .iter()
                .all(|artifact| artifact.derived_from_checked_core);
        let workflow_has_actual_dispatch = workflow.has_step(Sys5LocalWorkflowStep::Attack)
            && workflow.has_step(Sys5LocalWorkflowStep::DesignatedPublish)
            && workflow.has_step(Sys5LocalWorkflowStep::ViewerConsume)
            && evidence.anchor.has_full_dispatch_lifecycle();
        let selected_backends_match = evidence.selected_st.completed_generated_dispatches
            && evidence.selected_ow1.completed_generated_dispatches
            && evidence.selected_st.all_actions_succeeded
            && evidence.selected_ow1.all_actions_succeeded
            && evidence.selected_st.semantic_digest == evidence.selected_ow1.semantic_digest
            && evidence.selected_st.typed_receipt_ref == evidence.selected_ow1.typed_receipt_ref
            && evidence.selected_st.typed_result_ref == evidence.selected_ow1.typed_result_ref
            && evidence.selected_st.state_digest == evidence.selected_ow1.state_digest
            && evidence.selected_st.frontier_ref == evidence.selected_ow1.frontier_ref
            && evidence.selected_st.trace_digest == evidence.selected_ow1.trace_digest;
        let source_bound_execution = workflow.source_authority() == "ordinary_mir_source"
            && workflow.has_source_bound_execution_for_i2();
        let ordinary_source_authority =
            source_bound_execution && evidence.manual_route_or_interface_rejected;
        let failure_contained = workflow.has_step(Sys5LocalWorkflowStep::FailedConsume)
            && workflow.failure_rejected_before_state_mutation(
                crate::sys5_local_slice::Sys5VerticalDiagnosticKind::MissingConsumerCapability,
            )
            && workflow.has_step(Sys5LocalWorkflowStep::PatchRejected);
        let relation_coherent = workflow.has_step(Sys5LocalWorkflowStep::RelationPrimary)
            && workflow.has_step(Sys5LocalWorkflowStep::ParticipantALeave)
            && workflow.has_step(Sys5LocalWorkflowStep::FreshReacquire)
            && workflow.has_joined_row_kind("relation_selected_fallback")
            && workflow.has_relation_fallback_invariants_for_i2();
        let fallback_separated = workflow.has_step(Sys5LocalWorkflowStep::PresentationGap)
            && workflow.has_joined_row_kind("presentation_gap")
            && workflow.has_joined_row_kind("participant_leave")
            && workflow.has_presentation_gap_invariants_for_i2();
        let designated_once = workflow.has_step(Sys5LocalWorkflowStep::DesignatedPublish)
            && workflow.has_step(Sys5LocalWorkflowStep::ViewerConsume)
            && workflow.has_joined_row_kind("designated_result_version")
            && workflow.has_designated_result_invariants_for_i2();
        let trace_correspondence = evidence.anchor.has_projection_trace_join()
            && expected.contains_artifact_ref(&evidence.anchor.artifact_ref)
            && expected.contains_edge_ref(&evidence.anchor.edge_ref)
            && observed.contains_artifact_ref(&evidence.anchor.artifact_ref)
            && observed.contains_edge_ref(&evidence.anchor.edge_ref);
        let cut_consistent = workflow.has_step(Sys5LocalWorkflowStep::Save)
            && workflow.has_step(Sys5LocalWorkflowStep::Restore)
            && workflow.has_joined_row_kind("save_cut")
            && workflow.has_joined_row_kind("restore_cut")
            && workflow.has_local_cut_invariants_for_i2()
            // The test-only hook changes the owned raw conformance candidate,
            // never the live cut. Keep its rejection observable in the exact
            // property it exercises rather than accepting a detached
            // rejection record.
            && !rejected_control("OfflineCutCorruption");
        let patch_checked = workflow.has_step(Sys5LocalWorkflowStep::PatchAccepted)
            && workflow.has_step(Sys5LocalWorkflowStep::PatchRejected)
            && workflow.patch_verdicts().len() >= 2
            && workflow.has_patch_lifecycle_invariants_for_i2();
        let observer_safe = source_bound_execution
            && evidence.anchor.is_observer_safe()
            && !evidence.primary_content_identity_ref.is_empty()
            && !evidence.selected_ow1_content_identity_ref.is_empty()
            && !evidence.observer_sensitive_source
            && evidence.observer_control_detected;
        let visibility_preserved = evidence.model_no_stale_authority_use
            && !evidence.model_st_fingerprint.is_empty()
            && !evidence.model_ow1_fingerprint.is_empty();
        let source_free_authority = source_bound_execution
            && evidence.model_no_source_free_authority_mints
            && evidence.runtime_authority_control_detected;

        let no_direct_remote_store = source_bound_execution
            && !rejected_control("DirectRemoteStoreMutation")
            && !rejected_control("SourceFreeStateMint")
            && evidence.runtime_store_control_detected;
        let actual_dispatch = workflow_has_actual_dispatch;
        let source_free_authority =
            source_free_authority && !rejected_control("SourceFreeAuthorityMint");
        let no_source_free_state_mint = source_bound_execution
            && !rejected_control("SourceFreeStateMint")
            && evidence.runtime_source_free_state_mint_control_detected;
        let owner_data_race_free = selected_backends_match
            && evidence.selected_ow1.worker_owned_m8
            && evidence.selected_ow1.mailbox_fifo;
        let lifecycle_boundary_preserved = evidence.lifecycle_boundary.is_preacceptance_boundary()
            && evidence.lifecycle_control.runtime_unchanged();

        let checked_program_identity_ref = checked_program_identity_binding(
            &evidence.primary_checked_program_identity_ref,
            &evidence.primary_content_identity_ref,
        );
        let artifact_inventory_digest =
            artifact_inventory_digest(&observed.artifacts, &evidence.primary_content_identity_ref);

        let mut rows = vec![
            row(
                "i2.ordinary_source_authority",
                ordinary_source_authority,
                "primary-source-first",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.checked_global_core_identity",
                !evidence.primary_checked_program_identity_ref.is_empty() && artifact_identity,
                "checked-finite-core",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.core_to_locus_artifacts",
                artifact_identity,
                "four-locus-projection",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.generated_communication_complete",
                complete_edges,
                "generated-edge-inventory",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.actual_dispatch_over_generated_edges",
                actual_dispatch,
                "st-local-dispatch",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.st_ow_selected_correspondence",
                selected_backends_match,
                "selected-one-owner-worker-fragment",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.owner_data_race_freedom_selected_backend",
                owner_data_race_free,
                "selected-one-owner-worker-fragment",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.no_hidden_communication",
                no_hidden_communication,
                "generated-edge-only",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.no_direct_remote_store",
                no_direct_remote_store,
                "locus-endpoint-boundary",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.no_source_free_authority_mint",
                source_free_authority,
                "m9-admitted-source-authority",
                "model-checked-bounded",
                &evidence.anchor,
            ),
            row(
                "i2.no_source_free_state_mint",
                no_source_free_state_mint,
                "source-admitted-state-only",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.failure_containment",
                failure_contained,
                "typed-failure-before-mutation",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.relation_projection_coherence",
                relation_coherent,
                "finite-relation-fallback",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.semantic_presentation_fallback_separation",
                fallback_separated,
                "presentation-gap-nonmutation",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.designated_evaluator_non_reexecution",
                designated_once,
                "designated-result-delivery",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.source_core_artifact_trace_correspondence",
                trace_correspondence,
                "source-core-artifact-occurrence",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.save_restore_consistent_local_cut",
                cut_consistent,
                "st-local-cut-restore",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.patch_lifecycle_checked",
                patch_checked,
                "checked-patch-lifecycle",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.visibility_redaction_preserved",
                visibility_preserved && observer_safe,
                "observer-safe-redaction",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.observer_safe_devtools",
                observer_safe,
                "reference-only-observer-view",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.projection_determinism",
                evidence.projection_deterministic,
                "same-source-repeat",
                "runtime-monitored",
                &evidence.anchor,
            ),
            row(
                "i2.non_claims_and_lifecycle_boundaries",
                lifecycle_boundary_preserved,
                "provisional-internal-boundary",
                "runtime-monitored",
                &evidence.anchor,
            ),
        ];
        attach_actual_provenance_anchors(&mut rows, &evidence.provenance_anchors);
        enforce_bound_evidence_outcomes(&mut rows, &evidence.executed_evidence);
        if let Some(selected_row) = rows
            .iter_mut()
            .find(|row| row.id == "i2.st_ow_selected_correspondence")
        {
            selected_row.selected_ow1_source = Some(evidence.selected_ow1_logical_source.clone());
            selected_row.selected_ow1_parse = Some(I2SelectedParseTelemetry {
                locus_count: evidence.selected_st.locus_count,
            });
            selected_row.selected_ow1_projection = Some(I2SelectedProjectionTelemetry {
                count_source: "actual-projection-inventory".to_string(),
                artifact_count: evidence.selected_st.artifact_count,
                generated_edge_count: evidence.selected_st.generated_edge_count,
                actual_artifact_refs: evidence.selected_st.artifact_refs.clone(),
                actual_generated_edge_refs: evidence.selected_st.generated_edge_refs.clone(),
            });
            selected_row.st_backend_telemetry = Some(evidence.selected_st.telemetry());
            selected_row.ow1_backend_telemetry = Some(evidence.selected_ow1.telemetry());
            selected_row.st_semantic_digest = Some(evidence.selected_st.semantic_digest.clone());
            selected_row.ow1_semantic_digest = Some(evidence.selected_ow1.semantic_digest.clone());
            selected_row.four_locus_ow1_workflow_claimed = Some(false);
            selected_row.full_toy_ow1_residual = Some(evidence.full_toy_ow1_residual.clone());
        }
        if let Some(artifact_row) = rows
            .iter_mut()
            .find(|row| row.id == "i2.core_to_locus_artifacts")
        {
            let observed_owner_locus = observed
                .owner_locus("attack")
                .unwrap_or("missing-owner")
                .to_string();
            artifact_row.subclaims.push(I2OwnerPreservationSubclaim {
                id: "owner-preservation-worldauthority-attack".to_string(),
                status: if observed_owner_locus == "WorldAuthority" {
                    I2RowStatus::Pass
                } else {
                    I2RowStatus::Fail
                },
                operation_id: "attack".to_string(),
                expected_owner_locus: "WorldAuthority".to_string(),
                observed_owner_locus,
                evidence_ref: "i2-evidence:projection-validator-positive".to_string(),
            });
        }
        rows.sort_by(|left, right| left.id.cmp(&right.id));
        let inventory_is_exact = rows.len() == REQUIRED_ROW_IDS.len()
            && rows
                .iter()
                .map(|row| row.id.as_str())
                .collect::<BTreeSet<_>>()
                == REQUIRED_ROW_IDS.into_iter().collect();
        let status = if inventory_is_exact
            && rows.iter().all(|row| row.status == I2RowStatus::Pass)
            && !evidence.observer_sensitive_source
        {
            I2ConformanceStatus::Accepted
        } else {
            I2ConformanceStatus::Rejected
        };
        let bounded_implementation_source_fingerprint = bounded_implementation_source_fingerprint();
        let manifest = I2ManifestMaterial {
            bounded_implementation_source_fingerprint_id: bounded_implementation_source_fingerprint
                .id
                .clone(),
            primary_content_identity_ref: evidence.primary_content_identity_ref.clone(),
            selected_ow1_content_identity_ref: evidence.selected_ow1_content_identity_ref.clone(),
            primary_checked_program_identity_ref: evidence
                .primary_checked_program_identity_ref
                .clone(),
            patch_identity_refs: evidence.patch_identity_refs.clone(),
            selected_st_semantic_digest: evidence.selected_st.semantic_digest.clone(),
            selected_ow1_semantic_digest: evidence.selected_ow1.semantic_digest.clone(),
            model_st_fingerprint: evidence.model_st_fingerprint.clone(),
            model_ow1_fingerprint: evidence.model_ow1_fingerprint.clone(),
        };
        let i2_manifest_identity_ref = manifest_identity_ref(&manifest);
        let mut report = I2ConformanceReport {
            schema_version: REPORT_SCHEMA_VERSION.to_string(),
            command: "conform-i2".to_string(),
            profile_name: PROFILE_NAME.to_string(),
            profile_scope: PROFILE_SCOPE.to_string(),
            status,
            source_authority: "ordinary_mir_source".to_string(),
            public_api_or_wire_contract: false,
            final_public_api_frozen: false,
            public_wire_frozen: false,
            checked_program_identity_ref,
            artifact_inventory_digest,
            source_bound_execution: workflow.source_bound_execution().i2_observer_safe_summary(),
            bounded_implementation_source_fingerprint,
            i2_manifest_identity_ref,
            loci: expected.loci.clone(),
            inventories: inventories(&evidence, observed, &rows),
            lifecycle_state: evidence.lifecycle_boundary.observer_state(),
            rows,
            evidence_classes: vec![
                "model-checked-bounded".to_string(),
                "runtime-monitored".to_string(),
            ],
            non_claims: evidence.lifecycle_boundary.non_claims.clone(),
            #[cfg(test)]
            test_only_falsifier: test_rejection.as_ref().map(|_| true),
            #[cfg(test)]
            rejection: test_rejection,
            typed_rejection: evidence
                .observer_sensitive_source
                .then(|| I2TypedRejection {
                    diagnostic_code: "ObserverSensitiveIdentifier".to_string(),
                    observer_policy: Some(I2ObserverPolicy {
                        action: "redact".to_string(),
                        redacted_identifiers: vec![I2RedactedIdentifier {
                            redacted_as: "[redacted-observer-sensitive-identifier]".to_string(),
                        }],
                    }),
                }),
        };
        if evidence.observer_sensitive_source {
            report.redact_observer_sensitive_identifiers();
        }
        report
    }
}

fn row(
    id: &str,
    passed: bool,
    scope: &str,
    evidence_class: &str,
    anchor: &RuntimeAnchor,
) -> I2ConformanceRow {
    let provenance = actual_provenance_for_row(id, anchor);
    let anchor = &provenance.runtime_anchor;
    let (positive_evidence_ref, falsifier_evidence_ref) = evidence_refs_for(id);
    I2ConformanceRow {
        id: id.to_string(),
        status: if passed {
            I2RowStatus::Pass
        } else {
            I2RowStatus::Fail
        },
        scope: scope.to_string(),
        evidence_class: evidence_class.to_string(),
        provenance_anchor_ref: provenance.id.clone(),
        checked_program_identity_ref: anchor.checked_program_identity_ref.clone(),
        source_span: anchor.source_span,
        core_ref: anchor.core_ref.clone(),
        artifact_ref: anchor.artifact_ref.clone(),
        edge_ref: anchor.edge_ref.clone(),
        request_identity: anchor.request_identity.clone(),
        dispatch_occurrence_ref: anchor.dispatch_occurrence_ref.clone(),
        receive_occurrence_ref: anchor.receive_occurrence_ref.clone(),
        serve_occurrence_ref: anchor.serve_occurrence_ref.clone(),
        authority_source: "source_admission_m9".to_string(),
        locus_program_ref: anchor.artifact_ref.clone(),
        positive_evidence_refs: vec![positive_evidence_ref.to_string()],
        falsifier_evidence_refs: vec![falsifier_evidence_ref.to_string()],
        controls: controls_for(id)
            .into_iter()
            .map(|(control_id, evidence_ref, outcome)| I2Control {
                id: control_id.to_string(),
                evidence_ref: evidence_ref.to_string(),
                outcome: outcome.to_string(),
            })
            .collect(),
        subclaims: Vec::new(),
        selected_ow1_source: None,
        selected_ow1_parse: None,
        selected_ow1_projection: None,
        st_backend_telemetry: None,
        ow1_backend_telemetry: None,
        st_semantic_digest: None,
        ow1_semantic_digest: None,
        four_locus_ow1_workflow_claimed: None,
        full_toy_ow1_residual: None,
    }
}

fn actual_provenance_for_row(id: &str, fallback: &RuntimeAnchor) -> I2ProvenanceAnchor {
    // This placeholder is replaced by the verifier's post-construction
    // attachment below. It is intentionally an actual attack occurrence,
    // not a fabricated property reference, so a missing property anchor
    // fails closed when the attachment cannot be found.
    I2ProvenanceAnchor {
        id: fallback.serve_occurrence_ref.clone(),
        domain: "workflow".to_string(),
        produced_by: "sys5-workflow".to_string(),
        kind: "generated-owner-dispatch".to_string(),
        source: "actual".to_string(),
        causal_segment_ref: fallback.serve_occurrence_ref.clone(),
        lifecycle_ref: String::new(),
        model_ref: String::new(),
        row_ids: vec![id.to_string()],
        property_ids: Vec::new(),
        owner_worker_locus: None,
        requester_locus: None,
        designated_evaluator_locus: None,
        consumer_locus: None,
        source_loci: Vec::new(),
        runtime_anchor: fallback.clone(),
    }
}

/// The fixed profile assigns every row one and only one actual provenance
/// domain.  Do not silently retain the construction-time owner-dispatch
/// anchor when a specialised row loses its relation, designated, lifecycle,
/// selected-backend, or model evidence.  That would turn a cross-join failure
/// into a misleading primary-workflow claim.
#[derive(Debug, Clone, Copy)]
struct RequiredAnchorContract {
    domain: &'static str,
    kind: &'static str,
    source: &'static str,
}

fn required_anchor_contract(row_id: &str) -> RequiredAnchorContract {
    match row_id {
        "i2.relation_projection_coherence" | "i2.semantic_presentation_fallback_separation" => {
            RequiredAnchorContract {
                domain: "workflow",
                kind: "relation-fallback-lineage",
                source: "actual",
            }
        }
        "i2.designated_evaluator_non_reexecution" => RequiredAnchorContract {
            domain: "workflow",
            kind: "designated-result-delivery",
            source: "actual",
        },
        "i2.save_restore_consistent_local_cut" => RequiredAnchorContract {
            domain: "workflow",
            kind: "save-cut-lifecycle",
            source: "actual",
        },
        "i2.patch_lifecycle_checked" => RequiredAnchorContract {
            domain: "workflow",
            kind: "patch-lifecycle",
            source: "actual",
        },
        "i2.st_ow_selected_correspondence" => RequiredAnchorContract {
            domain: "selected-backend",
            kind: "selected-st-ow-correspondence",
            source: "actual-selected-ow1-source",
        },
        "i2.owner_data_race_freedom_selected_backend" => RequiredAnchorContract {
            domain: "model",
            kind: "st-ow-refinement-model",
            source: "actual",
        },
        _ => RequiredAnchorContract {
            domain: "workflow",
            kind: "generated-owner-dispatch",
            source: "actual",
        },
    }
}

fn clear_unbound_provenance(row: &mut I2ConformanceRow) {
    row.status = I2RowStatus::Fail;
    row.provenance_anchor_ref = "missing-or-ambiguous-required-provenance".to_string();
    row.checked_program_identity_ref.clear();
    row.source_span = I2SourceSpan {
        start: 0,
        end: 0,
        start_line: 0,
        start_column: 0,
        end_line: 0,
        end_column: 0,
    };
    row.core_ref.clear();
    row.artifact_ref.clear();
    row.edge_ref.clear();
    row.request_identity.clear();
    row.dispatch_occurrence_ref.clear();
    row.receive_occurrence_ref.clear();
    row.serve_occurrence_ref.clear();
    row.locus_program_ref.clear();
}

fn attach_actual_provenance_anchors(rows: &mut [I2ConformanceRow], anchors: &[I2ProvenanceAnchor]) {
    for row in rows {
        let contract = required_anchor_contract(&row.id);
        let matches = anchors
            .iter()
            .filter(|anchor| {
                anchor.domain == contract.domain
                    && anchor.kind == contract.kind
                    && anchor.source == contract.source
                    && anchor.row_ids.iter().any(|row_id| row_id == &row.id)
                    && anchor
                        .property_ids
                        .iter()
                        .any(|property_id| property_id == &row.id)
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            clear_unbound_provenance(row);
            continue;
        }
        let provenance = matches[0];
        row.provenance_anchor_ref = provenance.id.clone();
        row.checked_program_identity_ref = provenance
            .runtime_anchor
            .checked_program_identity_ref
            .clone();
        row.source_span = provenance.runtime_anchor.source_span;
        row.core_ref = provenance.runtime_anchor.core_ref.clone();
        row.artifact_ref = provenance.runtime_anchor.artifact_ref.clone();
        row.edge_ref = provenance.runtime_anchor.edge_ref.clone();
        row.request_identity = provenance.runtime_anchor.request_identity.clone();
        row.dispatch_occurrence_ref = provenance.runtime_anchor.dispatch_occurrence_ref.clone();
        row.receive_occurrence_ref = provenance.runtime_anchor.receive_occurrence_ref.clone();
        row.serve_occurrence_ref = provenance.runtime_anchor.serve_occurrence_ref.clone();
        row.locus_program_ref = provenance.runtime_anchor.artifact_ref.clone();
    }
}

/// Every finite I2 row is sound only while each evidence reference it names
/// resolves to an actually executed record with the role-specific outcome.
/// This is deliberately verifier-side: producer controls may fail closed, but
/// relabelling a failed control must never leave an otherwise independent row
/// passing.
fn enforce_bound_evidence_outcomes(rows: &mut [I2ConformanceRow], evidence: &[I2ExecutedEvidence]) {
    for row in rows {
        let matching_evidence = |reference: &str, expected_outcome: &str| {
            evidence.iter().any(|entry| {
                entry.id == reference
                    && entry.executed
                    && entry.outcome == expected_outcome
                    && expected_runtime_diagnostic_for(reference).is_none_or(|expected| {
                        entry.observed_runtime_diagnostic.as_deref() == Some(expected)
                    })
                    && entry.row_ids.iter().any(|row_id| row_id == &row.id)
                    && entry
                        .property_ids
                        .iter()
                        .any(|property_id| property_id == &row.id)
            })
        };
        let positives_valid = !row.positive_evidence_refs.is_empty()
            && row
                .positive_evidence_refs
                .iter()
                .all(|reference| matching_evidence(reference, "observed"));
        let falsifiers_valid = !row.falsifier_evidence_refs.is_empty()
            && row
                .falsifier_evidence_refs
                .iter()
                .all(|reference| matching_evidence(reference, "detected"));
        let controls_valid = row
            .controls
            .iter()
            .all(|control| matching_evidence(&control.evidence_ref, &control.outcome));
        // Inventory bindings are semantically meaningful too: a control that
        // explicitly names this property cannot fail while the row continues
        // to pass merely because another representative falsifier was also
        // linked by `falsifier_evidence_refs`.
        let all_bound_evidence_valid = evidence
            .iter()
            .filter(|entry| {
                entry.row_ids.iter().any(|row_id| row_id == &row.id)
                    || entry
                        .property_ids
                        .iter()
                        .any(|property_id| property_id == &row.id)
            })
            .all(|entry| {
                entry.executed
                    && matches!(entry.outcome.as_str(), "observed" | "detected")
                    && expected_runtime_diagnostic_for(&entry.id).is_none_or(|expected| {
                        entry.observed_runtime_diagnostic.as_deref() == Some(expected)
                    })
            });
        if !(positives_valid && falsifiers_valid && controls_valid && all_bound_evidence_valid) {
            row.status = I2RowStatus::Fail;
        }
    }
}

fn expected_runtime_diagnostic_for(evidence_id: &str) -> Option<&'static str> {
    match evidence_id {
        "i2-evidence:runtime-authority-override-detected" => Some("SourceFreeAuthorityMint"),
        "i2-evidence:runtime-cross-locus-store-detected" => Some("DirectRemoteStoreMutation"),
        "i2-evidence:runtime-source-free-state-mint-detected" => Some("SourceFreeStateMint"),
        "i2-evidence:offline-cut-corruption-detected" => Some("ProgramProjectionMismatch"),
        _ => None,
    }
}

#[cfg(test)]
fn expected_control_diagnostic_for(evidence_id: &str) -> Option<&'static str> {
    match evidence_id {
        // The observer-facing I2 control label is deliberately distinct from
        // the exact SYS-4 restore diagnostic verified above.
        "i2-evidence:offline-cut-corruption-detected" => Some("OfflineCutCorruption"),
        _ => expected_runtime_diagnostic_for(evidence_id),
    }
}

fn evidence_refs_for(id: &str) -> (&'static str, &'static str) {
    match id {
        "i2.checked_global_core_identity"
        | "i2.core_to_locus_artifacts"
        | "i2.generated_communication_complete"
        | "i2.actual_dispatch_over_generated_edges"
        | "i2.projection_determinism" => (
            "i2-evidence:projection-validator-positive",
            "i2-evidence:projection-missing-edge-detected",
        ),
        "i2.no_hidden_communication" => (
            "i2-evidence:projection-validator-positive",
            "i2-evidence:projection-non-derived-edge-detected",
        ),
        "i2.st_ow_selected_correspondence" | "i2.owner_data_race_freedom_selected_backend" => (
            "i2-evidence:selected-backend-positive",
            "i2-evidence:model-required-edge-detected",
        ),
        "i2.no_direct_remote_store" => (
            "i2-evidence:workflow-positive",
            "i2-evidence:runtime-cross-locus-store-detected",
        ),
        "i2.no_source_free_state_mint" => (
            "i2-evidence:workflow-positive",
            "i2-evidence:runtime-source-free-state-mint-detected",
        ),
        "i2.no_source_free_authority_mint" | "i2.ordinary_source_authority" => (
            "i2-evidence:workflow-positive",
            "i2-evidence:runtime-authority-override-detected",
        ),
        "i2.visibility_redaction_preserved" | "i2.observer_safe_devtools" => (
            "i2-evidence:observer-policy-positive",
            "i2-evidence:observer-sensitive-scan-detected",
        ),
        "i2.failure_containment" | "i2.patch_lifecycle_checked" => (
            "i2-evidence:workflow-positive",
            "i2-evidence:workflow-rejection-detected",
        ),
        "i2.save_restore_consistent_local_cut" => (
            "i2-evidence:workflow-positive",
            "i2-evidence:offline-cut-corruption-detected",
        ),
        "i2.non_claims_and_lifecycle_boundaries" => (
            "i2-evidence:workflow-positive",
            "i2-evidence:lifecycle-boundary-detected",
        ),
        _ => (
            "i2-evidence:workflow-positive",
            "i2-evidence:workflow-invariant-detected",
        ),
    }
}

fn controls_for(id: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match id {
        "i2.failure_containment" => vec![(
            "missing-consumer-capability-fail-closed",
            "i2-evidence:workflow-rejection-detected",
            "detected",
        )],
        "i2.visibility_redaction_preserved" | "i2.observer_safe_devtools" => {
            vec![(
                "observer-safe-redaction-no-secret-material",
                "i2-evidence:observer-sensitive-scan-detected",
                "detected",
            )]
        }
        "i2.relation_projection_coherence" => {
            vec![(
                "relation-primary-fallback-fresh-reacquire",
                "i2-evidence:workflow-invariant-detected",
                "detected",
            )]
        }
        "i2.semantic_presentation_fallback_separation" => {
            vec![(
                "presentation-gap-does-not-mutate-semantic-lineage",
                "i2-evidence:workflow-invariant-detected",
                "detected",
            )]
        }
        "i2.designated_evaluator_non_reexecution" => {
            vec![(
                "viewer-consumes-versioned-designated-result-without-re-evaluation",
                "i2-evidence:workflow-invariant-detected",
                "detected",
            )]
        }
        "i2.save_restore_consistent_local_cut" => {
            vec![(
                "local-cut-restore-retains-artifact-mailbox-and-trace-frontier",
                "i2-evidence:offline-cut-corruption-detected",
                "detected",
            )]
        }
        "i2.patch_lifecycle_checked" => vec![
            (
                "designated-plus-two-patch-accepted",
                "i2-evidence:workflow-positive",
                "observed",
            ),
            (
                "owner-rmw-change-patch-rejected",
                "i2-evidence:workflow-rejection-detected",
                "detected",
            ),
        ],
        "i2.non_claims_and_lifecycle_boundaries" => vec![(
            "preacceptance-lifecycle-boundary",
            "i2-evidence:lifecycle-boundary-detected",
            "detected",
        )],
        "i2.core_to_locus_artifacts" => vec![(
            "owner-preservation-worldauthority-attack",
            "i2-evidence:projection-validator-positive",
            "observed",
        )],
        _ => vec![(
            "finite-source-first-control",
            evidence_refs_for(id).0,
            "observed",
        )],
    }
}

fn actual_provenance_anchors(
    workflow: &Sys5LocalWorkflowReport,
    facts: &ProjectionFacts,
    fallback: &RuntimeAnchor,
    selected_backend: &SelectedBackendRun,
    model_ow1_fingerprint: &str,
) -> Vec<I2ProvenanceAnchor> {
    let relation = workflow
        .joined_rows()
        .iter()
        .filter_map(|row| row.detail())
        .find(|detail| detail.core_ref() == "relation:bird_follow")
        .and_then(|detail| RuntimeAnchor::from_actual_causal_detail(detail, facts));
    let designated = workflow
        .joined_rows()
        .iter()
        .filter_map(|row| row.detail())
        .find(|detail| detail.core_ref().starts_with("designated-consume:"))
        .and_then(|detail| RuntimeAnchor::from_actual_causal_detail(detail, facts));
    let save = workflow
        .save_lifecycle_refs_for_i2()
        .iter()
        .find(|entry| entry.kind() == "SaveCut");
    let patch = workflow
        .patch_lifecycle_refs_for_i2()
        .iter()
        .find(|entry| entry.kind() == "PatchAccepted");

    let generic_row_ids = REQUIRED_ROW_IDS
        .iter()
        .filter(|row| {
            !matches!(
                **row,
                "i2.relation_projection_coherence"
                    | "i2.semantic_presentation_fallback_separation"
                    | "i2.designated_evaluator_non_reexecution"
                    | "i2.save_restore_consistent_local_cut"
                    | "i2.patch_lifecycle_checked"
                    | "i2.st_ow_selected_correspondence"
                    | "i2.owner_data_race_freedom_selected_backend"
            )
        })
        .map(|row| (*row).to_string())
        .collect::<Vec<_>>();
    let mut anchors = vec![I2ProvenanceAnchor {
        id: fallback.serve_occurrence_ref.clone(),
        domain: "workflow".to_string(),
        produced_by: "sys5-workflow".to_string(),
        kind: "generated-owner-dispatch".to_string(),
        source: "actual".to_string(),
        causal_segment_ref: fallback.serve_occurrence_ref.clone(),
        lifecycle_ref: String::new(),
        model_ref: String::new(),
        row_ids: generic_row_ids.clone(),
        property_ids: generic_row_ids,
        owner_worker_locus: None,
        requester_locus: None,
        designated_evaluator_locus: None,
        consumer_locus: None,
        source_loci: Vec::new(),
        runtime_anchor: fallback.clone(),
    }];
    if let Some(anchor) = relation {
        anchors.push(I2ProvenanceAnchor {
            id: anchor.serve_occurrence_ref.clone(),
            domain: "workflow".to_string(),
            produced_by: "sys5-workflow".to_string(),
            kind: "relation-fallback-lineage".to_string(),
            source: "actual".to_string(),
            causal_segment_ref: anchor.serve_occurrence_ref.clone(),
            lifecycle_ref: String::new(),
            model_ref: String::new(),
            row_ids: vec![
                "i2.relation_projection_coherence".to_string(),
                "i2.semantic_presentation_fallback_separation".to_string(),
            ],
            property_ids: vec![
                "i2.relation_projection_coherence".to_string(),
                "i2.semantic_presentation_fallback_separation".to_string(),
            ],
            owner_worker_locus: None,
            requester_locus: None,
            designated_evaluator_locus: None,
            consumer_locus: None,
            source_loci: Vec::new(),
            runtime_anchor: anchor,
        });
    }
    if let Some(anchor) = designated {
        anchors.push(I2ProvenanceAnchor {
            id: anchor.serve_occurrence_ref.clone(),
            domain: "workflow".to_string(),
            produced_by: "sys5-workflow".to_string(),
            kind: "designated-result-delivery".to_string(),
            source: "actual".to_string(),
            causal_segment_ref: anchor.serve_occurrence_ref.clone(),
            lifecycle_ref: String::new(),
            model_ref: String::new(),
            row_ids: vec!["i2.designated_evaluator_non_reexecution".to_string()],
            property_ids: vec!["i2.designated_evaluator_non_reexecution".to_string()],
            owner_worker_locus: None,
            requester_locus: None,
            designated_evaluator_locus: None,
            consumer_locus: None,
            source_loci: Vec::new(),
            runtime_anchor: anchor,
        });
    }
    if let Some(lifecycle) = save {
        anchors.push(I2ProvenanceAnchor {
            id: lifecycle.occurrence_ref().to_string(),
            domain: "workflow".to_string(),
            produced_by: "sys5-workflow".to_string(),
            kind: "save-cut-lifecycle".to_string(),
            source: "actual".to_string(),
            causal_segment_ref: String::new(),
            lifecycle_ref: lifecycle.occurrence_ref().to_string(),
            model_ref: String::new(),
            row_ids: vec!["i2.save_restore_consistent_local_cut".to_string()],
            property_ids: vec!["i2.save_restore_consistent_local_cut".to_string()],
            owner_worker_locus: None,
            requester_locus: None,
            designated_evaluator_locus: None,
            consumer_locus: None,
            source_loci: Vec::new(),
            runtime_anchor: RuntimeAnchor::from_actual_lifecycle(fallback, lifecycle),
        });
    }
    if let Some(lifecycle) = patch {
        anchors.push(I2ProvenanceAnchor {
            id: lifecycle.occurrence_ref().to_string(),
            domain: "workflow".to_string(),
            produced_by: "sys5-workflow".to_string(),
            kind: "patch-lifecycle".to_string(),
            source: "actual".to_string(),
            causal_segment_ref: String::new(),
            lifecycle_ref: lifecycle.occurrence_ref().to_string(),
            model_ref: String::new(),
            row_ids: vec!["i2.patch_lifecycle_checked".to_string()],
            property_ids: vec!["i2.patch_lifecycle_checked".to_string()],
            owner_worker_locus: None,
            requester_locus: None,
            designated_evaluator_locus: None,
            consumer_locus: None,
            source_loci: Vec::new(),
            runtime_anchor: RuntimeAnchor::from_actual_lifecycle(fallback, lifecycle),
        });
    }
    anchors.push(I2ProvenanceAnchor {
        id: selected_backend.runtime_anchor.serve_occurrence_ref.clone(),
        domain: "selected-backend".to_string(),
        produced_by: "sys4-dispatch".to_string(),
        kind: "selected-st-ow-correspondence".to_string(),
        source: "actual-selected-ow1-source".to_string(),
        causal_segment_ref: selected_backend.runtime_anchor.serve_occurrence_ref.clone(),
        lifecycle_ref: String::new(),
        model_ref: String::new(),
        row_ids: vec!["i2.st_ow_selected_correspondence".to_string()],
        property_ids: vec!["i2.st_ow_selected_correspondence".to_string()],
        owner_worker_locus: Some(selected_backend.owner_worker_locus.clone()),
        requester_locus: Some(selected_backend.requester_locus.clone()),
        designated_evaluator_locus: Some(selected_backend.designated_evaluator_locus.clone()),
        consumer_locus: Some(selected_backend.consumer_locus.clone()),
        source_loci: selected_backend.source_loci.clone(),
        runtime_anchor: selected_backend.runtime_anchor.clone(),
    });
    let model_ref = model_ow1_fingerprint.to_string();
    anchors.push(I2ProvenanceAnchor {
        id: model_ref.clone(),
        domain: "model".to_string(),
        produced_by: "sys2-model".to_string(),
        kind: "st-ow-refinement-model".to_string(),
        source: "actual".to_string(),
        causal_segment_ref: String::new(),
        lifecycle_ref: String::new(),
        model_ref,
        row_ids: vec!["i2.owner_data_race_freedom_selected_backend".to_string()],
        property_ids: vec!["i2.owner_data_race_freedom_selected_backend".to_string()],
        owner_worker_locus: None,
        requester_locus: None,
        designated_evaluator_locus: None,
        consumer_locus: None,
        source_loci: Vec::new(),
        runtime_anchor: RuntimeAnchor::from_actual_model(fallback),
    });
    anchors
}

fn checked_program_identity_binding(checked_identity: &str, source_identity: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i2/checked-program-source-binding/v1\0");
    hasher.update(checked_identity.as_bytes());
    hasher.update([0]);
    hasher.update(source_identity.as_bytes());
    format!("i2-checked-program-sha256-v1:{:x}", hasher.finalize())
}

fn artifact_inventory_digest(artifacts: &[ArtifactFact], source_identity: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i2/artifact-inventory-source-binding/v1\0");
    hasher.update(source_identity.as_bytes());
    for artifact in artifacts {
        hasher.update(format!("{artifact:?}").as_bytes());
        hasher.update([0]);
    }
    format!("i2-artifact-inventory-sha256-v1:{:x}", hasher.finalize())
}

fn inventories(
    evidence: &RawI2Evidence,
    observed: &ProjectionFacts,
    rows: &[I2ConformanceRow],
) -> I2EvidenceInventories {
    let mut checked_program_identity_refs = BTreeSet::new();
    let mut core_refs = BTreeSet::new();
    let mut artifact_refs = BTreeSet::new();
    let mut communication_edge_refs = BTreeSet::new();
    for artifact in &observed.artifacts {
        checked_program_identity_refs.insert(artifact.checked_program_identity.clone());
        core_refs.insert(artifact.core_ref.clone());
        artifact_refs.insert(artifact.fragment_ref.clone());
    }
    for edge in &observed.edges {
        checked_program_identity_refs.insert(edge.checked_program_identity.clone());
        if let Some(core_ref) = &edge.core_ref {
            core_refs.insert(core_ref.clone());
        }
        communication_edge_refs.insert(edge.edge_ref.clone());
    }
    checked_program_identity_refs.insert(evidence.anchor.checked_program_identity_ref.clone());
    core_refs.insert(evidence.anchor.core_ref.clone());
    artifact_refs.insert(evidence.anchor.artifact_ref.clone());
    communication_edge_refs.insert(evidence.anchor.edge_ref.clone());
    let mut request_identity_refs = BTreeSet::from([evidence.anchor.request_identity.clone()]);
    let mut runtime_occurrence_refs = BTreeSet::from([
        evidence.anchor.dispatch_occurrence_ref.clone(),
        evidence.anchor.receive_occurrence_ref.clone(),
        evidence.anchor.serve_occurrence_ref.clone(),
    ]);
    for row in rows {
        if !row.checked_program_identity_ref.is_empty() {
            checked_program_identity_refs.insert(row.checked_program_identity_ref.clone());
        }
        if !row.core_ref.is_empty() {
            core_refs.insert(row.core_ref.clone());
        }
        if !row.artifact_ref.is_empty() {
            artifact_refs.insert(row.artifact_ref.clone());
        }
        if !row.locus_program_ref.is_empty() {
            artifact_refs.insert(row.locus_program_ref.clone());
        }
        if !row.edge_ref.is_empty() {
            communication_edge_refs.insert(row.edge_ref.clone());
        }
        if !row.request_identity.is_empty() {
            request_identity_refs.insert(row.request_identity.clone());
        }
        for occurrence in [
            &row.dispatch_occurrence_ref,
            &row.receive_occurrence_ref,
            &row.serve_occurrence_ref,
        ] {
            if !occurrence.is_empty() {
                runtime_occurrence_refs.insert(occurrence.clone());
            }
        }
    }
    I2EvidenceInventories {
        checked_program_identity_refs: checked_program_identity_refs.into_iter().collect(),
        core_refs: core_refs.into_iter().collect(),
        artifact_refs: artifact_refs.into_iter().collect(),
        communication_edge_refs: communication_edge_refs.into_iter().collect(),
        request_identity_refs: request_identity_refs.into_iter().collect(),
        runtime_occurrence_refs: runtime_occurrence_refs.into_iter().collect(),
        provenance_anchors: evidence.provenance_anchors.clone(),
        executed_evidence: evidence.executed_evidence.clone(),
        source_first_causal_provenance: source_first_causal_provenance(evidence, observed),
    }
}

fn source_first_entry(
    id: &str,
    kind: &str,
    source: &str,
    produced_by: &str,
    row_ids: Vec<String>,
    typed_producer_ref: String,
) -> I2SourceFirstCausalProvenance {
    I2SourceFirstCausalProvenance {
        id: id.to_string(),
        kind: kind.to_string(),
        source: source.to_string(),
        produced_by: produced_by.to_string(),
        property_ids: row_ids.clone(),
        row_ids,
        producer_invocation_ref: control_identity(
            "source-first-causal-producer-invocation",
            &(id, kind, source, produced_by, &typed_producer_ref),
        ),
        typed_producer_ref,
        source_content_identity_ref: None,
        logical_source_ref: None,
        source_span_ref: None,
        checked_program_identity_ref: None,
        sealed_admission_ref: None,
        manual_route_or_interface_admitted: None,
        runtime_core_or_authority_injection_admitted: None,
        runtime_state_injection_admitted: None,
        routing_source: None,
        communication_plan_ref: None,
        edge_ref: None,
        dispatch_occurrence_ref: None,
        candidate_kind: None,
        accepted: None,
        state_unchanged: None,
        mutation_applied: None,
        diagnostic_code: None,
        candidate_ref: None,
        state_before_ref: None,
        state_after_ref: None,
    }
}

fn source_first_causal_provenance(
    evidence: &RawI2Evidence,
    observed: &ProjectionFacts,
) -> Vec<I2SourceFirstCausalProvenance> {
    let inventory = evidence.workflow.source_bound_execution();
    let unknown = inventory.unknown_source_action_admission();
    let mut ordinary = source_first_entry(
        "i2-source-bound-workflow-inventory",
        "ordinary-source-bound",
        "actual-source-bound-causal-inventory",
        "sys5-workflow",
        vec!["i2.ordinary_source_authority".to_string()],
        control_identity(
            "ordinary-source-bound-inventory",
            &(
                inventory.input_boundary(),
                inventory.checked_program_identity_ref(),
                inventory.actual_causal_segments().len(),
            ),
        ),
    );
    ordinary.source_content_identity_ref = Some(evidence.primary_content_identity_ref.clone());
    ordinary.logical_source_ref = Some("i2-primary-source.mir".to_string());
    ordinary.source_span_ref = Some(control_identity(
        "ordinary-source-bound-span",
        &evidence.anchor.source_span,
    ));

    let mut checked = source_first_entry(
        "i2-checked-project",
        "checked-project",
        "actual-checked-project",
        "sys3-projection",
        vec![
            "i2.ordinary_source_authority".to_string(),
            "i2.checked_global_core_identity".to_string(),
        ],
        evidence.primary_checked_program_identity_ref.clone(),
    );
    checked.checked_program_identity_ref =
        Some(evidence.primary_checked_program_identity_ref.clone());

    let mut sealed = source_first_entry(
        "i2-sealed-admission",
        "sealed-admission",
        "actual-sealed-admission",
        "sys5-workflow",
        vec!["i2.ordinary_source_authority".to_string()],
        inventory.sealed_admission_attestation_ref().to_string(),
    );
    sealed.sealed_admission_ref = Some(inventory.sealed_admission_attestation_ref().to_string());
    // These booleans are individual outcomes of the actual SYS-4 controls
    // invoked by the producer; they are not top-level self-attestations.
    sealed.manual_route_or_interface_admitted = Some(!evidence.manual_route_or_interface_rejected);
    sealed.runtime_core_or_authority_injection_admitted =
        Some(!evidence.runtime_authority_control_detected);
    sealed.runtime_state_injection_admitted =
        Some(!evidence.runtime_source_free_state_mint_control_detected);

    let mut dispatch = source_first_entry(
        "i2-generated-dispatch",
        "generated-dispatch",
        "actual-generated-dispatch",
        "sys4-dispatch",
        vec![
            "i2.ordinary_source_authority".to_string(),
            "i2.actual_dispatch_over_generated_edges".to_string(),
        ],
        evidence.anchor.dispatch_occurrence_ref.clone(),
    );
    dispatch.routing_source = Some("generated_communication_plan".to_string());
    dispatch.communication_plan_ref = Some(control_identity(
        "generated-communication-plan",
        &observed.edges,
    ));
    dispatch.edge_ref = Some(evidence.anchor.edge_ref.clone());
    dispatch.dispatch_occurrence_ref = Some(evidence.anchor.dispatch_occurrence_ref.clone());

    let mut unknown_action = source_first_entry(
        "i2-unknown-source-action-admission",
        "unknown-action-admission-rejection",
        "actual-unknown-action-admission-candidate",
        "sys4-dispatch",
        vec!["i2.ordinary_source_authority".to_string()],
        unknown.candidate_action_ref().to_string(),
    );
    unknown_action.candidate_kind = Some("unknown-source-action".to_string());
    unknown_action.accepted = Some(!unknown.rejected_before_dispatch());
    unknown_action.state_unchanged =
        Some(unknown.semantic_state_before_ref() == unknown.semantic_state_after_ref());
    unknown_action.mutation_applied =
        Some(unknown.semantic_state_before_ref() != unknown.semantic_state_after_ref());
    unknown_action.diagnostic_code = Some(unknown.diagnostic().to_string());
    unknown_action.candidate_ref = Some(unknown.candidate_action_ref().to_string());
    unknown_action.state_before_ref = Some(unknown.semantic_state_before_ref().to_string());
    unknown_action.state_after_ref = Some(unknown.semantic_state_after_ref().to_string());

    vec![ordinary, checked, sealed, dispatch, unknown_action]
}

#[derive(Debug, Clone)]
struct RawI2Evidence {
    primary_content_identity_ref: String,
    selected_ow1_content_identity_ref: String,
    selected_ow1_logical_source: String,
    primary_checked_program_identity_ref: String,
    patch_identity_refs: Vec<String>,
    expected_projection: ProjectionFacts,
    observed_projection: ProjectionFacts,
    workflow: Sys5LocalWorkflowReport,
    selected_st: SelectedBackendRun,
    selected_ow1: SelectedBackendRun,
    model_st_fingerprint: String,
    model_ow1_fingerprint: String,
    model_no_source_free_authority_mints: bool,
    model_no_stale_authority_use: bool,
    projection_deterministic: bool,
    full_toy_ow1_residual: I2BackendResidual,
    anchor: RuntimeAnchor,
    provenance_anchors: Vec<I2ProvenanceAnchor>,
    projection_validator_positive: bool,
    projection_missing_edge_control_detected: bool,
    projection_extra_edge_control_detected: bool,
    runtime_authority_control_detected: bool,
    runtime_store_control_detected: bool,
    runtime_source_free_state_mint_control_detected: bool,
    manual_route_or_interface_rejected: bool,
    observer_control_detected: bool,
    lifecycle_boundary: LifecycleBoundaryState,
    lifecycle_control: LifecycleBoundaryControl,
    executed_evidence: Vec<I2ExecutedEvidence>,
    observer_sensitive_source: bool,
    #[cfg(test)]
    test_projection: GlobalProjectionResult,
    #[cfg(test)]
    test_primary_source: String,
    #[cfg(test)]
    test_rejection: Option<I2Rejection>,
}

impl RawI2Evidence {
    #[cfg(test)]
    fn apply_test_falsifier(&mut self, falsifier: I2ConformanceFalsifier) {
        match falsifier {
            I2ConformanceFalsifier::RemoveGeneratedCommunicationEdge {
                operation_id,
                edge_kind,
            } => {
                if let Some(kind) = edge_kind_from_test_name(&edge_kind) {
                    let before = candidate_identity("projection", &self.observed_projection);
                    self.test_projection
                        .for_test_remove_derived_edge(&operation_id, kind);
                    self.observed_projection =
                        ProjectionFacts::from_projection(&self.test_projection);
                    let after = candidate_identity("projection", &self.observed_projection);
                    self.record_test_rejection(
                        "projection_candidate",
                        "MissingGeneratedCommunicationEdge",
                        before,
                        after,
                        self.expected_projection.edges != self.observed_projection.edges,
                    );
                }
            }
            I2ConformanceFalsifier::InsertNonDerivedCommunicationEdge {
                edge_ref,
                operation_id,
                edge_kind,
                from_locus,
                to_locus,
            } => {
                if let Some(kind) = edge_kind_from_test_name(&edge_kind) {
                    let before = candidate_identity("communication", &self.observed_projection);
                    self.test_projection.for_test_insert_non_derived_edge(
                        &edge_ref,
                        kind,
                        &from_locus,
                        &to_locus,
                        &operation_id,
                    );
                    self.observed_projection =
                        ProjectionFacts::from_projection(&self.test_projection);
                    let after = candidate_identity("communication", &self.observed_projection);
                    let rejected = self
                        .observed_projection
                        .edges
                        .iter()
                        .any(|edge| !edge.derived_from_checked_core);
                    self.record_test_rejection(
                        "communication_candidate",
                        "NonDerivedCommunicationEdge",
                        before,
                        after,
                        rejected,
                    );
                }
            }
            I2ConformanceFalsifier::MoveOwnerOperation {
                operation_id,
                from_locus,
                to_locus,
            } => {
                let before = candidate_identity("artifact", &self.observed_projection);
                self.test_projection.for_test_move_owner_operation(
                    &operation_id,
                    &from_locus,
                    &to_locus,
                );
                self.observed_projection = ProjectionFacts::from_projection(&self.test_projection);
                let after = candidate_identity("artifact", &self.observed_projection);
                self.record_test_rejection(
                    "artifact_candidate",
                    "OwnerOperationMoved",
                    before,
                    after,
                    self.expected_projection.owner_locus("attack")
                        != self.observed_projection.owner_locus("attack"),
                );
            }
            I2ConformanceFalsifier::BreakSourceMap {
                operation_id,
                artifact_ref,
            } => {
                let before = candidate_identity("provenance", &self.observed_projection);
                let _ = self.test_projection.for_test_rewrite_fragment_ref(
                    &operation_id,
                    ProjectedOperationFragmentKind::OwnerRmwExecution,
                    &artifact_ref,
                );
                self.observed_projection = ProjectionFacts::from_projection(&self.test_projection);
                let after = candidate_identity("provenance", &self.observed_projection);
                self.record_test_rejection(
                    "provenance_candidate",
                    "SourceMapMismatch",
                    before,
                    after,
                    !self
                        .observed_projection
                        .contains_artifact_ref(&self.anchor.artifact_ref),
                );
            }
            I2ConformanceFalsifier::AdmitSourceFreeAuthority {
                principal,
                locus,
                operation_id,
            } => {
                let candidate = test_authority_override_candidate(
                    &self.test_primary_source,
                    &principal,
                    &locus,
                    &operation_id,
                );
                self.record_test_rejection(
                    "runtime_candidate",
                    "SourceFreeAuthorityMint",
                    candidate.before,
                    candidate.after,
                    candidate.rejected,
                );
                self.test_rejection
                    .as_mut()
                    .expect("runtime candidate rejection is recorded")
                    .runtime_endpoint_attempt = Some(candidate.attempt);
            }
            I2ConformanceFalsifier::MutateRemoteStore {
                locus,
                state,
                index,
                field,
                value,
            } => {
                let candidate = test_remote_store_candidate(
                    &self.test_primary_source,
                    &locus,
                    &state,
                    &index,
                    &field,
                    value,
                );
                self.record_test_rejection(
                    "runtime_candidate",
                    &candidate.diagnostic,
                    candidate.before,
                    candidate.after,
                    candidate.rejected,
                );
                self.test_rejection
                    .as_mut()
                    .expect("runtime candidate rejection is recorded")
                    .runtime_endpoint_attempt = Some(candidate.attempt);
            }
            I2ConformanceFalsifier::DivergeSelectedBackendTypedResult => {
                let before =
                    candidate_identity("st_typed_result_ref", &self.selected_st.typed_result_ref);
                self.selected_ow1
                    .typed_result_ref
                    .push_str(":test-diverged");
                self.record_test_rejection(
                    "selected_backend_candidate",
                    "SelectedBackendTypedResultDivergence",
                    before,
                    candidate_identity("ow1_typed_result_ref", &self.selected_ow1.typed_result_ref),
                    self.selected_st.typed_result_ref != self.selected_ow1.typed_result_ref,
                );
                self.set_selected_backend_divergence("SelectedBackendTypedResultDivergence");
            }
            I2ConformanceFalsifier::DivergeSelectedBackendState => {
                let before = candidate_identity("st_state_digest", &self.selected_st.state_digest);
                self.selected_ow1.state_digest.push_str(":test-diverged");
                self.record_test_rejection(
                    "selected_backend_candidate",
                    "SelectedBackendStateDivergence",
                    before,
                    candidate_identity("ow1_state_digest", &self.selected_ow1.state_digest),
                    self.selected_st.state_digest != self.selected_ow1.state_digest,
                );
                self.set_selected_backend_divergence("SelectedBackendStateDivergence");
            }
            I2ConformanceFalsifier::DivergeSelectedBackendFrontier => {
                let before = candidate_identity("st_frontier_ref", &self.selected_st.frontier_ref);
                self.selected_ow1.frontier_ref.push_str(":test-diverged");
                self.record_test_rejection(
                    "selected_backend_candidate",
                    "SelectedBackendFrontierDivergence",
                    before,
                    candidate_identity("ow1_frontier_ref", &self.selected_ow1.frontier_ref),
                    self.selected_st.frontier_ref != self.selected_ow1.frontier_ref,
                );
                self.set_selected_backend_divergence("SelectedBackendFrontierDivergence");
            }
            I2ConformanceFalsifier::DivergeSelectedBackendTrace => {
                let before = candidate_identity("st_trace_digest", &self.selected_st.trace_digest);
                self.selected_ow1.trace_digest.push_str(":test-diverged");
                self.record_test_rejection(
                    "selected_backend_candidate",
                    "SelectedBackendTraceDivergence",
                    before,
                    candidate_identity("ow1_trace_digest", &self.selected_ow1.trace_digest),
                    self.selected_st.trace_digest != self.selected_ow1.trace_digest,
                );
                self.set_selected_backend_divergence("SelectedBackendTraceDivergence");
            }
            I2ConformanceFalsifier::CorruptOfflineCut => {
                let candidate = test_offline_cut_corruption_candidate(&self.test_primary_source);
                self.record_test_rejection(
                    "offline_cut_candidate",
                    "OfflineCutCorruption",
                    candidate.before,
                    candidate.after,
                    candidate.rejected,
                );
                self.test_rejection
                    .as_mut()
                    .expect("offline cut rejection is recorded")
                    .offline_cut_candidate = Some(candidate.detail);
            }
            I2ConformanceFalsifier::FlipLifecycleBoundaryClaim => {
                let before =
                    candidate_identity("lifecycle-boundary-candidate", &self.lifecycle_boundary);
                self.lifecycle_boundary.i2_exit_accepted = true;
                self.lifecycle_boundary.public_transport_claim = true;
                let after =
                    candidate_identity("lifecycle-boundary-candidate", &self.lifecycle_boundary);
                let overclaim_candidate = lifecycle_boundary_candidate(
                    &self.lifecycle_boundary,
                    after.clone(),
                    &self.lifecycle_control.runtime_state_before,
                    &self.lifecycle_control.runtime_state_after,
                );
                self.record_test_rejection(
                    "lifecycle_boundary_candidate",
                    "LifecycleBoundaryOverclaim",
                    before,
                    after,
                    !self.lifecycle_boundary.is_preacceptance_boundary()
                        && self.lifecycle_control.runtime_unchanged(),
                );
                self.test_rejection
                    .as_mut()
                    .expect("lifecycle boundary rejection is recorded")
                    .lifecycle_boundary_candidate = Some(overclaim_candidate);
            }
            I2ConformanceFalsifier::SubstituteRuntimeControlDiagnostic {
                control_id,
                diagnostic_code,
            } => {
                let Some(index) = self
                    .executed_evidence
                    .iter()
                    .position(|evidence| evidence.id == control_id)
                else {
                    return;
                };
                let expected_diagnostic_code = expected_control_diagnostic_for(&control_id)
                    .unwrap_or("MissingExpectedRuntimeDiagnostic")
                    .to_string();
                let before = candidate_identity(
                    "control-diagnostic-candidate",
                    &self.executed_evidence[index],
                );
                self.executed_evidence[index].observed_runtime_diagnostic =
                    Some(diagnostic_code.clone());
                let after = candidate_identity(
                    "control-diagnostic-candidate",
                    &self.executed_evidence[index],
                );
                let accepted = diagnostic_code == expected_diagnostic_code;
                let candidate = I2ControlDiagnosticCandidate {
                    source: "actual-control-diagnostic-candidate".to_string(),
                    control_id: control_id.clone(),
                    expected_diagnostic_code,
                    observed_diagnostic_code: diagnostic_code,
                    accepted,
                    mutation_applied: false,
                    candidate_ref: after.clone(),
                    producer_invocation_ref: candidate_identity(
                        "control-diagnostic-producer-invocation",
                        &(&control_id, &after),
                    ),
                };
                self.record_test_rejection(
                    "control_diagnostic_candidate",
                    "ControlDiagnosticMismatch",
                    before,
                    after,
                    !accepted,
                );
                self.test_rejection
                    .as_mut()
                    .expect("control diagnostic rejection is recorded")
                    .control_diagnostic_candidate = Some(candidate);
            }
            I2ConformanceFalsifier::FailBoundEvidence { evidence_id } => {
                let Some(index) = self
                    .executed_evidence
                    .iter()
                    .position(|evidence| evidence.id == evidence_id)
                else {
                    return;
                };
                let before = candidate_identity(
                    "executed-evidence-candidate",
                    &self.executed_evidence[index],
                );
                self.executed_evidence[index].executed = false;
                self.executed_evidence[index].outcome = "control-failed".to_string();
                let after = candidate_identity(
                    "executed-evidence-candidate",
                    &self.executed_evidence[index],
                );
                let candidate = I2ExecutedEvidenceCandidate {
                    source: "actual-executed-evidence-candidate".to_string(),
                    evidence_id: evidence_id.clone(),
                    outcome: self.executed_evidence[index].outcome.clone(),
                    executed: self.executed_evidence[index].executed,
                    accepted: false,
                    affected_row_ids: self.executed_evidence[index].row_ids.clone(),
                    candidate_ref: after.clone(),
                    producer_invocation_ref: candidate_identity(
                        "executed-evidence-producer-invocation",
                        &(&evidence_id, &after),
                    ),
                };
                self.record_test_rejection(
                    "executed_evidence_candidate",
                    "BoundEvidenceNotExecuted",
                    before,
                    after,
                    true,
                );
                self.test_rejection
                    .as_mut()
                    .expect("executed evidence rejection is recorded")
                    .executed_evidence_candidate = Some(candidate);
            }
            I2ConformanceFalsifier::AdmitManualRouteOrInterface {
                operation_id,
                from_locus,
                to_locus,
            } => {
                let before = candidate_identity(
                    "manual-route-or-interface-candidate",
                    &(
                        self.manual_route_or_interface_rejected,
                        &operation_id,
                        &from_locus,
                        &to_locus,
                    ),
                );
                self.manual_route_or_interface_rejected = false;
                let after = candidate_identity(
                    "manual-route-or-interface-candidate",
                    &(
                        self.manual_route_or_interface_rejected,
                        &operation_id,
                        &from_locus,
                        &to_locus,
                    ),
                );
                let semantic_state = candidate_identity(
                    "manual-route-or-interface-semantic-state",
                    &self.workflow.observer_safe_digest(),
                );
                let candidate = I2ManualRouteOrInterfaceCandidate {
                    source: "actual-manual-route-or-interface-candidate".to_string(),
                    manual_route_or_interface_admitted: true,
                    accepted: false,
                    mutation_applied: false,
                    candidate_ref: after.clone(),
                    producer_invocation_ref: candidate_identity(
                        "manual-route-or-interface-producer-invocation",
                        &(&operation_id, &from_locus, &to_locus, &after),
                    ),
                    semantic_state_before: semantic_state.clone(),
                    semantic_state_after: semantic_state,
                };
                self.record_test_rejection(
                    "source_first_admission_candidate",
                    "ManualRouteOrInterfaceAdmitted",
                    before,
                    after,
                    true,
                );
                self.test_rejection
                    .as_mut()
                    .expect("manual route/interface rejection is recorded")
                    .manual_route_or_interface_candidate = Some(candidate);
            }
            I2ConformanceFalsifier::RemoveRequiredProvenanceAnchor { row_id } => {
                let before =
                    candidate_identity("provenance-anchor-inventory", &self.provenance_anchors);
                let mut removed = false;
                for anchor in &mut self.provenance_anchors {
                    let before_row_count = anchor.row_ids.len();
                    anchor.row_ids.retain(|bound_row| bound_row != &row_id);
                    anchor
                        .property_ids
                        .retain(|property_row| property_row != &row_id);
                    removed |= anchor.row_ids.len() != before_row_count;
                }
                let after =
                    candidate_identity("provenance-anchor-inventory", &self.provenance_anchors);
                self.record_test_rejection(
                    "provenance_anchor_inventory",
                    "MissingRequiredProvenanceAnchor",
                    before,
                    after,
                    removed,
                );
            }
        }
    }

    #[cfg(test)]
    fn record_test_rejection(
        &mut self,
        mutation_stage: &str,
        diagnostic_code: &str,
        candidate_identity_before: String,
        candidate_identity_after: String,
        validator_rejected: bool,
    ) {
        let snapshots = stable_snapshots(&self.workflow, &self.anchor);
        self.test_rejection = Some(I2Rejection {
            mutation_stage: mutation_stage.to_string(),
            diagnostic_code: diagnostic_code.to_string(),
            validator_invocation: I2ValidatorInvocation {
                invoked: true,
                result: if validator_rejected {
                    "rejected"
                } else {
                    "accepted"
                }
                .to_string(),
                validator_invocation_ref: candidate_identity(
                    "validator-invocation",
                    &(mutation_stage, diagnostic_code, validator_rejected),
                ),
            },
            candidate_identity_before,
            candidate_identity_after,
            snapshots,
            runtime_endpoint_attempt: None,
            offline_cut_candidate: None,
            selected_backend_divergence: None,
            lifecycle_boundary_candidate: None,
            control_diagnostic_candidate: None,
            executed_evidence_candidate: None,
            manual_route_or_interface_candidate: None,
        });
    }

    #[cfg(test)]
    fn set_selected_backend_divergence(&mut self, diagnostic_code: &str) {
        let detail = I2SelectedBackendDivergence {
            diagnostic_code: diagnostic_code.to_string(),
            source: "actual-selected-backend-candidate".to_string(),
            control_ref: candidate_identity(
                "selected-backend-divergence-control",
                &(diagnostic_code, &self.selected_st.semantic_digest),
            ),
            producer_invocation_ref: candidate_identity(
                "selected-backend-divergence-producer",
                &(
                    &self.selected_st.typed_receipt_ref,
                    &self.selected_ow1.typed_receipt_ref,
                ),
            ),
            st_typed_result_ref: None,
            ow1_typed_result_ref: None,
            st_state_digest: None,
            ow1_state_digest: None,
            st_frontier_ref: None,
            ow1_frontier_ref: None,
            st_trace_digest: None,
            ow1_trace_digest: None,
        };
        let mut detail = detail;
        match diagnostic_code {
            "SelectedBackendTypedResultDivergence" => {
                detail.st_typed_result_ref = Some(self.selected_st.typed_result_ref.clone());
                detail.ow1_typed_result_ref = Some(self.selected_ow1.typed_result_ref.clone());
            }
            "SelectedBackendStateDivergence" => {
                detail.st_state_digest = Some(self.selected_st.state_digest.clone());
                detail.ow1_state_digest = Some(self.selected_ow1.state_digest.clone());
            }
            "SelectedBackendFrontierDivergence" => {
                detail.st_frontier_ref = Some(self.selected_st.frontier_ref.clone());
                detail.ow1_frontier_ref = Some(self.selected_ow1.frontier_ref.clone());
            }
            "SelectedBackendTraceDivergence" => {
                detail.st_trace_digest = Some(self.selected_st.trace_digest.clone());
                detail.ow1_trace_digest = Some(self.selected_ow1.trace_digest.clone());
            }
            _ => {}
        }
        self.test_rejection
            .as_mut()
            .expect("selected backend rejection is recorded")
            .selected_backend_divergence = Some(detail);
    }
}

struct ProductionControls {
    projection_validator_positive: bool,
    projection_missing_edge_control_detected: bool,
    projection_extra_edge_control_detected: bool,
    runtime_authority_control_detected: bool,
    runtime_store_control_detected: bool,
    runtime_source_free_state_mint_control_detected: bool,
    manual_route_or_interface_rejected: bool,
    observer_control_detected: bool,
    lifecycle_control: LifecycleBoundaryControl,
    executed_evidence: Vec<I2ExecutedEvidence>,
}

/// A production-safe lifecycle falsifier: it changes only a typed candidate,
/// never the live project, admission, or fabric state.
#[derive(Debug, Clone)]
struct LifecycleBoundaryControl {
    baseline_candidate_ref: String,
    overclaim_candidate_ref: String,
    runtime_state_before: String,
    runtime_state_after: String,
    observed_candidate: I2LifecycleBoundaryCandidate,
    overclaim_candidate: I2LifecycleBoundaryCandidate,
}

impl LifecycleBoundaryControl {
    fn runtime_unchanged(&self) -> bool {
        self.runtime_state_before == self.runtime_state_after
    }
}

/// Evidence from the real report-rendering policy path.  The candidate
/// identity is hashed before redaction, while the after identity hashes the
/// actual serialized redacted observer view; neither exposes the marker.
struct ObserverPolicyControl {
    detected: bool,
    candidate_before: String,
    candidate_after: String,
    producer_invocation_ref: String,
    marker_present_in_candidate: bool,
    marker_absent_after_redaction: bool,
}

fn lifecycle_boundary_control(
    workflow: &Sys5LocalWorkflowReport,
    boundary: &LifecycleBoundaryState,
) -> LifecycleBoundaryControl {
    // Constructing and checking a lifecycle candidate is deliberately
    // external to the running fabric. Capture the same typed workflow state
    // on either side so the control records, rather than assumes, no runtime
    // mutation.
    let runtime_state_before = control_identity(
        "lifecycle-boundary-runtime-state",
        &workflow.observer_safe_digest(),
    );
    let overclaim = boundary.i2_exit_overclaim_candidate();
    let runtime_state_after = control_identity(
        "lifecycle-boundary-runtime-state",
        &workflow.observer_safe_digest(),
    );
    let baseline_candidate_ref = control_identity("lifecycle-boundary-candidate", boundary);
    let overclaim_candidate_ref = control_identity("lifecycle-boundary-candidate", &overclaim);
    let observed_candidate = lifecycle_boundary_candidate(
        boundary,
        baseline_candidate_ref.clone(),
        &runtime_state_before,
        &runtime_state_after,
    );
    let overclaim_candidate = lifecycle_boundary_candidate(
        &overclaim,
        overclaim_candidate_ref.clone(),
        &runtime_state_before,
        &runtime_state_after,
    );
    LifecycleBoundaryControl {
        baseline_candidate_ref,
        overclaim_candidate_ref,
        runtime_state_before,
        runtime_state_after,
        observed_candidate,
        overclaim_candidate,
    }
}

fn lifecycle_boundary_candidate(
    boundary: &LifecycleBoundaryState,
    candidate_ref: String,
    runtime_state_before: &str,
    runtime_state_after: &str,
) -> I2LifecycleBoundaryCandidate {
    I2LifecycleBoundaryCandidate {
        source: "actual-lifecycle-boundary-candidate".to_string(),
        producer_invocation_ref: control_identity(
            "lifecycle-boundary-producer-invocation",
            &(boundary, &candidate_ref),
        ),
        candidate_ref,
        broad_i1_exit_accepted: boundary.broad_i1_exit_accepted,
        i2_entry_accepted: boundary.i2_entry_accepted,
        i2_exit_accepted: boundary.i2_exit_accepted,
        sys7_goal_active: boundary.sys7_goal_active,
        i3_program_active: boundary.i3_program_active,
        public_transport_claim: boundary.public_transport_claim,
        real_transport_selected: boundary.real_transport_selected,
        production_deployment_claim: boundary.production_deployment_claim,
        accepted: boundary.is_preacceptance_boundary(),
        mutation_applied: runtime_state_before != runtime_state_after,
    }
}

/// Execute a small set of bounded controls against owned candidates or live
/// SYS-4 admission boundaries. These are production-safe: source, Core,
/// authority, and active stores are never replaced by the candidate under
/// test. One control may cover several finite rows, but every reference in
/// the report is bound to this actual invocation record.
fn run_production_controls(
    project: &Sys5LocalProject,
    workflow: &Sys5LocalWorkflowReport,
    _model_st: &crate::sys2_bounded_model::ModelCheckReport,
    _model_ow1: &crate::sys2_bounded_model::ModelCheckReport,
    lifecycle_boundary: &LifecycleBoundaryState,
) -> ProductionControls {
    let all_rows = REQUIRED_ROW_IDS
        .iter()
        .map(|row| (*row).to_string())
        .collect::<Vec<_>>();
    let projection = project.projected_result_for_i2_evidence();
    // The structural SYS-3 verifier is retained as one line of defence, but
    // the acceptance predicate also requires a separate inventory derived
    // directly from retained checked Core evaluations.  Do not turn this into
    // a projector-versus-itself comparison: the second line is what makes the
    // missing/extra edge controls meaningful to the I2 profile.
    let projection_validator_positive = project.validates_i2_projection_candidate(projection)
        && project.i2_candidate_covers_checked_core_requirements(projection);
    let canonical_after = control_identity("projection-canonical-after", projection);

    let owner_edge = projection
        .communication_plan()
        .edges()
        .iter()
        .find(|edge| edge.kind() == CommunicationEdgeKind::OwnerRequest)
        .cloned();
    let (
        projection_missing_edge_control_detected,
        missing_before,
        missing_after,
        projection_extra_edge_control_detected,
        extra_before,
        extra_after,
    ) = if let Some(ref edge) = owner_edge {
        let candidate =
            projection.conformance_candidate_without_edge(edge.operation_id(), edge.kind());
        let before = control_identity("projection-missing-edge-before", projection);
        let after = control_identity("projection-missing-edge-after", &candidate);
        let extra = projection.conformance_candidate_with_extra_edge(
            "i2-control:non-derived-edge",
            edge.kind(),
            edge.source_locus(),
            edge.target_locus(),
            edge.operation_id(),
        );
        let extra_before = control_identity("projection-extra-edge-before", projection);
        let extra_after = control_identity("projection-extra-edge-after", &extra);
        (
            !project.validates_i2_projection_candidate(&candidate)
                && !project.i2_candidate_covers_checked_core_requirements(&candidate),
            before,
            after,
            !project.validates_i2_projection_candidate(&extra)
                && !project.i2_candidate_covers_checked_core_requirements(&extra),
            extra_before,
            extra_after,
        )
    } else {
        (
            false,
            control_identity("projection-missing-edge-before", projection),
            control_identity("projection-missing-edge-after", projection),
            false,
            control_identity("projection-extra-edge-before", projection),
            control_identity("projection-extra-edge-after", projection),
        )
    };

    let runtime_controls = execute_runtime_boundary_controls(project, owner_edge.as_ref());
    let observer_control = observer_policy_renderer_redacts_candidate();
    let observer_control_detected = observer_control.detected;
    let lifecycle_control = lifecycle_boundary_control(workflow, lifecycle_boundary);
    let model_without_owner_edge = Sys2BoundedModel::new()
        .with_profile(ModelExecutionProfile::OneOwnerWorker)
        .with_bound(6)
        .with_required_edges([])
        .with_litmus_cases([LitmusCase::owner_request_serve_message_passing()])
        .check();
    let model_edge_detected = !model_without_owner_edge.passes_all_litmus();
    let workflow_rejection_detected = workflow.failure_rejected_before_state_mutation(
        crate::sys5_local_slice::Sys5VerticalDiagnosticKind::MissingConsumerCapability,
    ) && workflow
        .patch_verdicts()
        .iter()
        .any(|verdict| verdict.verdict() == "rejected");
    let workflow_invariant_detected = workflow.has_relation_fallback_invariants_for_i2()
        && workflow.has_presentation_gap_invariants_for_i2()
        && workflow.has_designated_result_invariants_for_i2()
        && workflow.has_duplicate_leave_fail_closed_for_i2();

    let mut executed_evidence = vec![
        positive_evidence(
            "i2-evidence:workflow-positive",
            "workflow-executed-observation",
            "sys5-workflow",
            all_rows.clone(),
            control_identity("workflow-invocation", &workflow.observer_safe_digest()),
        ),
        positive_evidence(
            "i2-evidence:projection-validator-positive",
            "checked-core-projection-validator",
            "sys3-projection",
            vec![
                "i2.checked_global_core_identity".to_string(),
                "i2.core_to_locus_artifacts".to_string(),
                "i2.generated_communication_complete".to_string(),
                "i2.actual_dispatch_over_generated_edges".to_string(),
                "i2.no_hidden_communication".to_string(),
                "i2.projection_determinism".to_string(),
            ],
            canonical_after,
        ),
        positive_evidence(
            "i2-evidence:selected-backend-positive",
            "selected-st-ow-executed-observation",
            "sys4-dispatch",
            vec![
                "i2.st_ow_selected_correspondence".to_string(),
                "i2.owner_data_race_freedom_selected_backend".to_string(),
            ],
            control_identity(
                "selected-backend-invocation",
                &project.checked_program_identity_ref(),
            ),
        ),
        positive_evidence(
            "i2-evidence:observer-policy-positive",
            "observer-policy-scan",
            "sys6-validator",
            vec![
                "i2.visibility_redaction_preserved".to_string(),
                "i2.observer_safe_devtools".to_string(),
            ],
            observer_control.producer_invocation_ref.clone(),
        ),
        detected_evidence(
            "i2-evidence:projection-missing-edge-detected",
            "projection-candidate-falsifier",
            "sys3-projection",
            vec![
                "i2.checked_global_core_identity".to_string(),
                "i2.core_to_locus_artifacts".to_string(),
                "i2.generated_communication_complete".to_string(),
                "i2.actual_dispatch_over_generated_edges".to_string(),
                "i2.projection_determinism".to_string(),
            ],
            missing_before,
            missing_after,
            "MissingGeneratedCommunicationEdge",
        ),
        detected_evidence(
            "i2-evidence:projection-non-derived-edge-detected",
            "non-derived-communication-edge-falsifier",
            "sys3-projection",
            vec!["i2.no_hidden_communication".to_string()],
            extra_before,
            extra_after,
            "NonDerivedCommunicationEdge",
        ),
        detected_evidence(
            "i2-evidence:runtime-authority-override-detected",
            "runtime-endpoint-falsifier",
            "sys4-dispatch",
            vec![
                "i2.ordinary_source_authority".to_string(),
                "i2.no_source_free_authority_mint".to_string(),
            ],
            runtime_controls.runtime_before.clone(),
            runtime_controls.runtime_after.clone(),
            "SourceFreeAuthorityMint",
        ),
        detected_evidence(
            "i2-evidence:runtime-cross-locus-store-detected",
            "runtime-endpoint-falsifier",
            "sys4-dispatch",
            vec![
                "i2.no_direct_remote_store".to_string(),
                "i2.no_source_free_state_mint".to_string(),
            ],
            runtime_controls.runtime_before.clone(),
            runtime_controls.runtime_after.clone(),
            "DirectRemoteStoreMutation",
        ),
        detected_evidence(
            "i2-evidence:runtime-source-free-state-mint-detected",
            "runtime-endpoint-falsifier",
            "sys4-dispatch",
            vec!["i2.no_source_free_state_mint".to_string()],
            runtime_controls.runtime_before.clone(),
            runtime_controls.runtime_after.clone(),
            "SourceFreeStateMint",
        ),
        detected_evidence(
            "i2-evidence:offline-cut-corruption-detected",
            "offline-cut-falsifier",
            "sys4-dispatch",
            vec!["i2.save_restore_consistent_local_cut".to_string()],
            runtime_controls.offline_before.clone(),
            runtime_controls.offline_after.clone(),
            "OfflineCutCorruption",
        ),
        detected_evidence(
            "i2-evidence:observer-sensitive-scan-detected",
            "observer-policy-falsifier",
            "sys6-validator",
            vec![
                "i2.visibility_redaction_preserved".to_string(),
                "i2.observer_safe_devtools".to_string(),
            ],
            observer_control.candidate_before.clone(),
            observer_control.candidate_after.clone(),
            "ObserverSensitiveIdentifier",
        ),
        detected_evidence(
            "i2-evidence:model-required-edge-detected",
            "bounded-model-falsifier",
            "sys2-model",
            vec![
                "i2.st_ow_selected_correspondence".to_string(),
                "i2.owner_data_race_freedom_selected_backend".to_string(),
            ],
            control_identity("model-before", &"required-owner-request-serve"),
            control_identity(
                "model-after",
                &model_without_owner_edge.deterministic_fingerprint(),
            ),
            "MissingOwnerRequestServeEdge",
        ),
        detected_evidence(
            "i2-evidence:workflow-rejection-detected",
            "workflow-typed-rejection-observation",
            "sys5-workflow",
            vec![
                "i2.failure_containment".to_string(),
                "i2.patch_lifecycle_checked".to_string(),
            ],
            control_identity(
                "workflow-rejection-before",
                &workflow.observer_safe_digest(),
            ),
            control_identity("workflow-rejection-after", &workflow.render_compact()),
            "MissingConsumerCapability",
        ),
        detected_evidence(
            "i2-evidence:workflow-invariant-detected",
            "duplicate-participant-leave-falsifier",
            "sys5-workflow",
            vec![
                "i2.relation_projection_coherence".to_string(),
                "i2.semantic_presentation_fallback_separation".to_string(),
                "i2.designated_evaluator_non_reexecution".to_string(),
                "i2.source_core_artifact_trace_correspondence".to_string(),
            ],
            control_identity(
                "workflow-invariants-before",
                &workflow.observer_safe_digest(),
            ),
            control_identity("workflow-invariants-after", &workflow.render_compact()),
            "DuplicateParticipantLeave",
        ),
        detected_evidence(
            "i2-evidence:lifecycle-boundary-detected",
            "lifecycle-boundary-control",
            "sys6-validator",
            vec!["i2.non_claims_and_lifecycle_boundaries".to_string()],
            lifecycle_control.baseline_candidate_ref.clone(),
            lifecycle_control.overclaim_candidate_ref.clone(),
            "LifecycleBoundaryOverclaim",
        ),
    ];
    // Do not report a control as detected if its real invocation failed.
    for evidence in &mut executed_evidence {
        let observed_runtime_diagnostic = match evidence.id.as_str() {
            "i2-evidence:runtime-authority-override-detected" => {
                observer_sys4_diagnostic(runtime_controls.authority_diagnostic)
            }
            "i2-evidence:runtime-cross-locus-store-detected" => {
                observer_sys4_diagnostic(runtime_controls.store_diagnostic)
            }
            "i2-evidence:runtime-source-free-state-mint-detected" => {
                observer_sys4_diagnostic(runtime_controls.state_mint_diagnostic)
            }
            "i2-evidence:offline-cut-corruption-detected" => {
                observer_sys4_diagnostic(runtime_controls.offline_cut_diagnostic)
            }
            _ => None,
        };
        evidence.observed_runtime_diagnostic = observed_runtime_diagnostic;
        let detected = match evidence.id.as_str() {
            "i2-evidence:projection-missing-edge-detected" => {
                projection_missing_edge_control_detected
            }
            "i2-evidence:projection-non-derived-edge-detected" => {
                projection_extra_edge_control_detected
            }
            "i2-evidence:runtime-authority-override-detected" => {
                runtime_controls.authority_override_rejected
            }
            "i2-evidence:runtime-cross-locus-store-detected" => {
                runtime_controls.cross_locus_store_rejected
            }
            "i2-evidence:runtime-source-free-state-mint-detected" => {
                runtime_controls.source_free_state_mint_rejected
            }
            "i2-evidence:offline-cut-corruption-detected" => runtime_controls.offline_cut_rejected,
            "i2-evidence:observer-sensitive-scan-detected" => observer_control_detected,
            "i2-evidence:model-required-edge-detected" => model_edge_detected,
            "i2-evidence:workflow-rejection-detected" => workflow_rejection_detected,
            "i2-evidence:workflow-invariant-detected" => workflow_invariant_detected,
            _ => true,
        };
        if evidence.outcome == "detected" && !detected {
            evidence.outcome = "rejected".to_string();
            evidence.diagnostic_code = Some("ControlExecutionFailedClosed".to_string());
        }
        if matches!(
            evidence.id.as_str(),
            "i2-evidence:runtime-authority-override-detected"
                | "i2-evidence:runtime-cross-locus-store-detected"
                | "i2-evidence:runtime-source-free-state-mint-detected"
        ) {
            evidence.state_before_digest = Some(runtime_controls.state_before.clone());
            evidence.state_after_digest = Some(runtime_controls.state_after.clone());
            evidence.state_unchanged = Some(runtime_controls.underlying_state_unchanged);
        }
        if evidence.id == "i2-evidence:observer-sensitive-scan-detected" {
            evidence.candidate_source = Some("actual-marker-bearing-report-candidate".to_string());
            evidence.redacted_output_source = Some("actual-redacted-serialized-output".to_string());
            evidence.marker_present_in_candidate =
                Some(observer_control.marker_present_in_candidate);
            evidence.marker_absent_after_redaction =
                Some(observer_control.marker_absent_after_redaction);
            evidence.marker_bearing_report_candidate_ref =
                Some(observer_control.candidate_before.clone());
            evidence.redacted_serialized_output_ref =
                Some(observer_control.candidate_after.clone());
            evidence.producer_invocation_ref = observer_control.producer_invocation_ref.clone();
        }
        if evidence.id == "i2-evidence:lifecycle-boundary-detected" {
            evidence.candidate_source = Some("actual-lifecycle-boundary-candidate".to_string());
            evidence.state_before_digest = Some(lifecycle_control.runtime_state_before.clone());
            evidence.state_after_digest = Some(lifecycle_control.runtime_state_after.clone());
            evidence.state_unchanged = Some(lifecycle_control.runtime_unchanged());
            evidence.lifecycle_boundary_candidate =
                Some(lifecycle_control.observed_candidate.clone());
            evidence.overclaim_candidate = Some(lifecycle_control.overclaim_candidate.clone());
        }
    }
    ProductionControls {
        projection_validator_positive,
        projection_missing_edge_control_detected,
        projection_extra_edge_control_detected,
        runtime_authority_control_detected: runtime_controls.authority_override_rejected,
        runtime_store_control_detected: runtime_controls.cross_locus_store_rejected,
        runtime_source_free_state_mint_control_detected: runtime_controls
            .source_free_state_mint_rejected,
        manual_route_or_interface_rejected: runtime_controls.manual_route_or_interface_rejected,
        observer_control_detected,
        lifecycle_control,
        executed_evidence,
    }
}

fn execute_runtime_boundary_controls(
    project: &Sys5LocalProject,
    owner_edge: Option<&crate::sys3_projection::CommunicationEdge>,
) -> RuntimeBoundaryControls {
    let fallback = || RuntimeBoundaryControls {
        manual_route_or_interface_rejected: false,
        authority_override_rejected: false,
        cross_locus_store_rejected: false,
        source_free_state_mint_rejected: false,
        offline_cut_rejected: false,
        underlying_state_unchanged: false,
        authority_diagnostic: None,
        store_diagnostic: None,
        state_mint_diagnostic: None,
        offline_cut_diagnostic: None,
        runtime_before: control_identity("runtime-before", &"unavailable"),
        runtime_after: control_identity("runtime-after", &"unavailable"),
        state_before: control_identity("runtime-state-before", &"unavailable"),
        state_after: control_identity("runtime-state-after", &"unavailable"),
        offline_before: control_identity("offline-cut-before", &"unavailable"),
        offline_after: control_identity("offline-cut-after", &"unavailable"),
    };
    let Ok(admission) = project.prepare_canonical_local_st_admission() else {
        return fallback();
    };
    let (program, sealed_admission) = admission.into_parts_for_sys4();
    let Ok(mut fabric) = LocalFabric::bootstrap(
        program.clone(),
        sealed_admission.clone(),
        BackendProfile::St,
    ) else {
        return fallback();
    };
    let active_before = fabric.active_runtime_identity_snapshot();
    let state_before_snapshot = fabric.semantic_snapshot();
    let before = control_identity("runtime-before", &active_before);
    let state_before = control_identity("runtime-state-before", &state_before_snapshot);
    let authority_diagnostic = observed_sys4_diagnostic(fabric.validate_external_action(
        &ExternalAction::conformance_attempt_authority_override(
            SourceAction::owner_operation("attack").with_argument("target", "self"),
        ),
    ));
    let manual_route_or_interface_diagnostic = owner_edge.and_then(|edge| {
        observed_sys4_diagnostic(fabric.validate_external_action(
            &ExternalAction::conformance_attempt_target_override(
                SourceAction::owner_operation(edge.operation_id()),
                edge.target_locus(),
            ),
        ))
    });
    let store_diagnostic =
        owner_edge.and_then(|edge| {
            observed_sys4_diagnostic(fabric.reject_external_cross_locus_store_attempt(
                edge.source_locus(),
                edge.target_locus(),
            ))
        });
    let state_mint_diagnostic = owner_edge.and_then(|edge| {
        observed_sys4_diagnostic(fabric.validate_external_action(
            &ExternalAction::conformance_attempt_source_free_state_mint(
                edge.target_locus(),
                "source-free-state",
                "self",
                "value",
                1,
            ),
        ))
    });
    let active_after = fabric.active_runtime_identity_snapshot();
    let state_after_snapshot = fabric.semantic_snapshot();
    let after = control_identity("runtime-after", &active_after);
    let state_after = control_identity("runtime-state-after", &state_after_snapshot);
    let underlying_state_unchanged = state_before_snapshot == state_after_snapshot;
    let (offline_cut_diagnostic, offline_before, offline_after) =
        match fabric.save_local_cut("i2-conformance-offline-cut") {
            Ok(cut) => {
                let before =
                    control_identity("offline_cut_ref", &cut.observer_safe_integrity_material());
                let candidate = cut.conformance_corrupt_offline_cut_candidate();
                let after = control_identity(
                    "corruption_kind",
                    &candidate.observer_safe_integrity_material(),
                );
                let diagnostic = observed_sys4_diagnostic(LocalFabric::restore_local_cut(
                    program,
                    sealed_admission,
                    BackendProfile::St,
                    &candidate,
                ));
                (diagnostic, before, after)
            }
            Err(_) => (
                None,
                control_identity("offline_cut_ref", &"save-failed"),
                control_identity("corruption_kind", &"save-failed"),
            ),
        };
    RuntimeBoundaryControls {
        manual_route_or_interface_rejected: manual_route_or_interface_diagnostic
            == Some(Sys4DiagnosticKind::ExternalTargetOverrideRejected)
            && underlying_state_unchanged,
        authority_override_rejected: authority_diagnostic
            == Some(Sys4DiagnosticKind::SourceFreeAuthorityMint)
            && underlying_state_unchanged,
        cross_locus_store_rejected: store_diagnostic
            == Some(Sys4DiagnosticKind::DirectRemoteStoreMutation)
            && underlying_state_unchanged,
        source_free_state_mint_rejected: state_mint_diagnostic
            == Some(Sys4DiagnosticKind::SourceFreeStateMint)
            && underlying_state_unchanged,
        offline_cut_rejected: offline_cut_diagnostic
            == Some(Sys4DiagnosticKind::ProgramProjectionMismatch),
        underlying_state_unchanged,
        authority_diagnostic,
        store_diagnostic,
        state_mint_diagnostic,
        offline_cut_diagnostic,
        runtime_before: before,
        runtime_after: after,
        state_before,
        state_after,
        offline_before,
        offline_after,
    }
}

struct RuntimeBoundaryControls {
    manual_route_or_interface_rejected: bool,
    authority_override_rejected: bool,
    cross_locus_store_rejected: bool,
    source_free_state_mint_rejected: bool,
    offline_cut_rejected: bool,
    underlying_state_unchanged: bool,
    authority_diagnostic: Option<Sys4DiagnosticKind>,
    store_diagnostic: Option<Sys4DiagnosticKind>,
    state_mint_diagnostic: Option<Sys4DiagnosticKind>,
    offline_cut_diagnostic: Option<Sys4DiagnosticKind>,
    runtime_before: String,
    runtime_after: String,
    state_before: String,
    state_after: String,
    offline_before: String,
    offline_after: String,
}

fn observed_sys4_diagnostic<T>(
    result: Result<T, Sys4DispatchDiagnostics>,
) -> Option<Sys4DiagnosticKind> {
    result.err().map(|diagnostics| diagnostics.primary().kind())
}

fn observer_sys4_diagnostic(kind: Option<Sys4DiagnosticKind>) -> Option<String> {
    kind.map(|kind| format!("{kind:?}"))
}

fn positive_evidence(
    id: &str,
    kind: &str,
    produced_by: &str,
    row_ids: Vec<String>,
    producer_invocation_ref: String,
) -> I2ExecutedEvidence {
    I2ExecutedEvidence {
        id: id.to_string(),
        kind: kind.to_string(),
        outcome: "observed".to_string(),
        evidence_class: "runtime-monitored".to_string(),
        produced_by: produced_by.to_string(),
        property_ids: row_ids.clone(),
        row_ids,
        control_ref: format!("control:{id}"),
        producer_invocation_ref,
        executed: true,
        candidate_identity_before: None,
        candidate_identity_after: None,
        diagnostic_code: None,
        observed_runtime_diagnostic: None,
        runtime_endpoint_attempt_ref: None,
        offline_cut_ref: None,
        corruption_kind: None,
        state_before_digest: None,
        state_after_digest: None,
        state_unchanged: None,
        candidate_source: None,
        redacted_output_source: None,
        marker_present_in_candidate: None,
        marker_absent_after_redaction: None,
        marker_bearing_report_candidate_ref: None,
        redacted_serialized_output_ref: None,
        lifecycle_boundary_candidate: None,
        overclaim_candidate: None,
    }
}

fn detected_evidence(
    id: &str,
    kind: &str,
    produced_by: &str,
    row_ids: Vec<String>,
    before: String,
    after: String,
    diagnostic_code: &str,
) -> I2ExecutedEvidence {
    let runtime_endpoint_attempt_ref = matches!(
        id,
        "i2-evidence:runtime-authority-override-detected"
            | "i2-evidence:runtime-cross-locus-store-detected"
            | "i2-evidence:runtime-source-free-state-mint-detected"
    )
    .then(|| control_identity("runtime_endpoint_attempt_ref", &id));
    let offline_cut_ref = (diagnostic_code == "OfflineCutCorruption")
        .then(|| control_identity("offline_cut_ref", &id));
    I2ExecutedEvidence {
        id: id.to_string(),
        kind: kind.to_string(),
        outcome: "detected".to_string(),
        evidence_class: if produced_by == "sys2-model" {
            "model-checked-bounded".to_string()
        } else {
            "runtime-monitored".to_string()
        },
        produced_by: produced_by.to_string(),
        property_ids: row_ids.clone(),
        row_ids,
        control_ref: format!("control:{id}"),
        producer_invocation_ref: control_identity("producer-invocation", &id),
        executed: true,
        candidate_identity_before: Some(before),
        candidate_identity_after: Some(after),
        diagnostic_code: Some(diagnostic_code.to_string()),
        observed_runtime_diagnostic: None,
        runtime_endpoint_attempt_ref,
        offline_cut_ref,
        corruption_kind: (diagnostic_code == "OfflineCutCorruption")
            .then(|| "OfflineCutCorruption".to_string()),
        state_before_digest: None,
        state_after_digest: None,
        state_unchanged: None,
        candidate_source: None,
        redacted_output_source: None,
        marker_present_in_candidate: None,
        marker_absent_after_redaction: None,
        marker_bearing_report_candidate_ref: None,
        redacted_serialized_output_ref: None,
        lifecycle_boundary_candidate: None,
        overclaim_candidate: None,
    }
}

fn control_identity<T: fmt::Debug>(kind: &str, value: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i2/production-control-identity/v1\0");
    hasher.update(kind.as_bytes());
    hasher.update([0]);
    hasher.update(format!("{value:?}").as_bytes());
    format!("i2-control-sha256-v1:{:x}", hasher.finalize())
}

fn contains_observer_sensitive_identifier(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    [
        "credential",
        "capability_secret",
        "witness_secret",
        "token=",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn redact_observer_string(value: &mut String) {
    if contains_observer_sensitive_identifier(value) {
        *value = "[redacted-observer-sensitive-identifier]".to_string();
    }
}

fn redact_option_string(value: &mut Option<String>) {
    if let Some(value) = value {
        redact_observer_string(value);
    }
}

fn redact_string_vec(values: &mut [String]) {
    for value in values {
        redact_observer_string(value);
    }
}

fn redact_backend_telemetry(telemetry: &mut I2BackendTelemetry) {
    redact_observer_string(&mut telemetry.runtime_profile);
    redact_option_string(&mut telemetry.sole_worker_locus);
    redact_string_vec(&mut telemetry.lifecycle_refs);
    redact_observer_string(&mut telemetry.typed_receipt_ref);
    redact_observer_string(&mut telemetry.typed_result_ref);
    redact_observer_string(&mut telemetry.state_digest);
    redact_observer_string(&mut telemetry.frontier_ref);
    redact_observer_string(&mut telemetry.trace_digest);
    for action in &mut telemetry.action_outcomes {
        redact_observer_string(&mut action.action_ref);
        redact_observer_string(&mut action.result_kind);
        redact_observer_string(&mut action.typed_result_ref);
        redact_observer_string(&mut action.receipt_occurrence_ref);
        redact_observer_string(&mut action.attempted_provenance_ref);
        redact_option_string(&mut action.diagnostic_code);
    }
    if let Some(fifo) = &mut telemetry.same_mailbox_fifo_control {
        redact_observer_string(&mut fifo.source);
        redact_observer_string(&mut fifo.same_mailbox_owner_locus);
        redact_string_vec(&mut fifo.request_ids);
        redact_string_vec(&mut fifo.enqueue_order);
        redact_string_vec(&mut fifo.serve_order);
        redact_string_vec(&mut fifo.typed_receipt_refs);
    }
}

#[cfg(test)]
fn candidate_identity<T: fmt::Debug>(kind: &str, candidate: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i2/test-candidate-identity/v1\0");
    hasher.update(kind.as_bytes());
    hasher.update([0]);
    hasher.update(format!("{candidate:?}").as_bytes());
    format!("i2-test-candidate-sha256-v1:{:x}", hasher.finalize())
}

#[cfg(test)]
fn stable_snapshots(
    workflow: &Sys5LocalWorkflowReport,
    anchor: &RuntimeAnchor,
) -> I2StableSnapshots {
    let semantic = candidate_identity("semantic-snapshot", &workflow.observer_safe_digest());
    let runtime = candidate_identity("runtime-snapshot", &workflow.render_compact());
    let authority = candidate_identity("authority-snapshot", &anchor.checked_program_identity_ref);
    I2StableSnapshots {
        semantic_before: semantic.clone(),
        semantic_after: semantic,
        runtime_before: runtime.clone(),
        runtime_after: runtime,
        authority_before: authority.clone(),
        authority_after: authority,
    }
}

/// Exercise the real SYS-4 external-action admission path with an attempted
/// authority override.  The candidate is never admitted into the fabric; the
/// normal source-derived program is bootstrapped separately only to ensure
/// the failed candidate cannot alter its active semantic state.
#[cfg(test)]
struct TestRuntimeBoundaryCandidate {
    before: String,
    after: String,
    rejected: bool,
    diagnostic: String,
    attempt: I2RuntimeEndpointAttempt,
}

#[cfg(test)]
fn test_authority_override_candidate(
    source: &str,
    principal: &str,
    locus: &str,
    operation_id: &str,
) -> TestRuntimeBoundaryCandidate {
    let Ok(project) = build_project(Sys5SourceInput::inline(
        "i2-test-runtime-candidate.mir",
        source.to_string(),
    )) else {
        return unavailable_runtime_candidate(
            "SourceFreeAuthorityMint",
            &(principal, locus, operation_id),
        );
    };
    let Ok(admission) = project.prepare_canonical_local_st_admission() else {
        return unavailable_runtime_candidate(
            "SourceFreeAuthorityMint",
            &(principal, locus, operation_id),
        );
    };
    let (program, sealed_admission) = admission.into_parts_for_sys4();
    let Ok(active) = LocalFabric::bootstrap(program, sealed_admission, BackendProfile::St) else {
        return unavailable_runtime_candidate(
            "SourceFreeAuthorityMint",
            &(principal, locus, operation_id),
        );
    };
    let candidate = ExternalAction::conformance_attempt_authority_override(
        SourceAction::owner_operation(operation_id).with_argument("target", "self"),
    );
    let result = active.validate_external_action(&candidate);
    runtime_boundary_candidate(&active, candidate, "SourceFreeAuthorityMint", result)
}

/// The direct-store negative control reaches the live SYS-4 locus boundary.
/// It does not corrupt an offline cut: the requested cross-locus store path
/// is rejected before a state location or mutation is admitted.
#[cfg(test)]
fn test_remote_store_candidate(
    source: &str,
    locus: &str,
    state: &str,
    index: &str,
    field: &str,
    value: i64,
) -> TestRuntimeBoundaryCandidate {
    let diagnostic = if locus == "WorldAuthority" {
        "SourceFreeStateMint"
    } else {
        "DirectRemoteStoreMutation"
    };
    let Ok(project) = build_project(Sys5SourceInput::inline(
        "i2-test-runtime-candidate.mir",
        source.to_string(),
    )) else {
        return unavailable_runtime_candidate(diagnostic, &(locus, state, index, field, value));
    };
    let Ok(admission) = project.prepare_canonical_local_st_admission() else {
        return unavailable_runtime_candidate(diagnostic, &(locus, state, index, field, value));
    };
    let (program, sealed_admission) = admission.into_parts_for_sys4();
    let Ok(active) = LocalFabric::bootstrap(program, sealed_admission, BackendProfile::St) else {
        return unavailable_runtime_candidate(diagnostic, &(locus, state, index, field, value));
    };
    if diagnostic == "SourceFreeStateMint" {
        let candidate = ExternalAction::conformance_attempt_source_free_state_mint(
            locus, state, index, field, value,
        );
        let result = active.validate_external_action(&candidate);
        runtime_boundary_candidate(&active, candidate, diagnostic, result)
    } else {
        let candidate = ExternalAction::conformance_attempt_source_free_state_mint(
            locus, state, index, field, value,
        );
        // This is an actual cross-locus runtime attempt. Pick the opposite
        // live locus when the candidate names ParticipantA so the control
        // cannot accidentally degrade into a same-locus validation error.
        let origin = if locus == "ParticipantA" {
            "WorldAuthority"
        } else {
            "ParticipantA"
        };
        let result = active.reject_external_cross_locus_store_attempt(origin, locus);
        runtime_boundary_candidate(&active, candidate, diagnostic, result)
    }
}

#[cfg(test)]
fn runtime_boundary_candidate(
    active: &LocalFabric,
    candidate: ExternalAction,
    expected_diagnostic: &str,
    result: Result<(), crate::sys4_dispatch::Sys4DispatchDiagnostics>,
) -> TestRuntimeBoundaryCandidate {
    let underlying_before =
        candidate_identity("runtime-underlying-before", &active.semantic_snapshot());
    let authority_before = candidate_identity(
        "runtime-authority-before",
        &active.active_runtime_identity_snapshot(),
    );
    let semantic_before =
        candidate_identity("runtime-semantic-before", &active.semantic_snapshot());
    let before = candidate_identity(
        "runtime-candidate-before",
        &active.active_runtime_identity_snapshot(),
    );
    let diagnostic = result
        .as_ref()
        .err()
        .map(|diagnostic| format!("{:?}", diagnostic.primary().kind()))
        .unwrap_or_else(|| "UnexpectedRuntimeAdmission".to_string());
    let after = candidate_identity("runtime-candidate-after", &(&candidate, &diagnostic));
    let rejected = result.is_err() && diagnostic == expected_diagnostic;
    let unchanged = underlying_before
        == candidate_identity("runtime-underlying-before", &active.semantic_snapshot())
        && authority_before
            == candidate_identity(
                "runtime-authority-before",
                &active.active_runtime_identity_snapshot(),
            )
        && semantic_before
            == candidate_identity("runtime-semantic-before", &active.semantic_snapshot());
    TestRuntimeBoundaryCandidate {
        before,
        after,
        rejected: rejected && unchanged,
        diagnostic,
        attempt: I2RuntimeEndpointAttempt {
            diagnostic_code: expected_diagnostic.to_string(),
            attempt_ref: candidate_identity("runtime-endpoint-attempt", &candidate),
            producer_invocation_ref: candidate_identity(
                "runtime-endpoint-producer",
                &expected_diagnostic,
            ),
            underlying_state_before: underlying_before,
            underlying_state_after: candidate_identity(
                "runtime-underlying-before",
                &active.semantic_snapshot(),
            ),
            authority_state_before: authority_before,
            authority_state_after: candidate_identity(
                "runtime-authority-before",
                &active.active_runtime_identity_snapshot(),
            ),
            semantic_state_before: semantic_before,
            semantic_state_after: candidate_identity(
                "runtime-semantic-before",
                &active.semantic_snapshot(),
            ),
            mutation_applied: !unchanged,
        },
    }
}

#[cfg(test)]
fn unavailable_runtime_candidate<T: fmt::Debug>(
    diagnostic: &str,
    candidate: &T,
) -> TestRuntimeBoundaryCandidate {
    let unavailable = candidate_identity("runtime-boundary-unavailable", candidate);
    TestRuntimeBoundaryCandidate {
        before: unavailable.clone(),
        after: candidate_identity("runtime-boundary-unavailable-after", candidate),
        rejected: false,
        diagnostic: diagnostic.to_string(),
        attempt: I2RuntimeEndpointAttempt {
            diagnostic_code: diagnostic.to_string(),
            attempt_ref: unavailable.clone(),
            producer_invocation_ref: unavailable.clone(),
            underlying_state_before: unavailable.clone(),
            underlying_state_after: unavailable.clone(),
            authority_state_before: unavailable.clone(),
            authority_state_after: unavailable.clone(),
            semantic_state_before: unavailable.clone(),
            semantic_state_after: unavailable.clone(),
            mutation_applied: unavailable
                != candidate_identity("runtime-boundary-unavailable", candidate),
        },
    }
}

#[cfg(test)]
struct TestOfflineCutCandidate {
    before: String,
    after: String,
    rejected: bool,
    detail: I2OfflineCutCandidate,
}

#[cfg(test)]
fn test_offline_cut_corruption_candidate(source: &str) -> TestOfflineCutCandidate {
    let unavailable = || {
        let state_digest_before = candidate_identity("offline-cut-state-before", &"unavailable");
        let state_digest_after = candidate_identity("offline-cut-state-after", &"unavailable");
        TestOfflineCutCandidate {
            before: candidate_identity("offline-cut-before", &"unavailable"),
            after: candidate_identity("offline-cut-after", &"unavailable"),
            rejected: false,
            detail: I2OfflineCutCandidate {
                diagnostic_code: "OfflineCutCorruption".to_string(),
                source: "actual-cut-restore-control".to_string(),
                cut_ref: String::new(),
                restore_attempt_ref: String::new(),
                restore_result: "unavailable".to_string(),
                mutation_applied: state_digest_before != state_digest_after,
                state_digest_before,
                state_digest_after,
            },
        }
    };
    let Ok(project) = build_project(Sys5SourceInput::inline("i2-test-offline-cut.mir", source))
    else {
        return unavailable();
    };
    let Ok(admission) = project.prepare_canonical_local_st_admission() else {
        return unavailable();
    };
    let (program, sealed_admission) = admission.into_parts_for_sys4();
    let Ok(mut fabric) = LocalFabric::bootstrap(
        program.clone(),
        sealed_admission.clone(),
        BackendProfile::St,
    ) else {
        return unavailable();
    };
    let state_before = candidate_identity("offline-cut-state-before", &fabric.semantic_snapshot());
    let Ok(cut) = fabric.save_local_cut("i2-test-offline-cut") else {
        return unavailable();
    };
    let cut_ref = candidate_identity("offline-cut-ref", &cut.observer_safe_integrity_material());
    let candidate = cut.conformance_corrupt_offline_cut_candidate();
    let restore_attempt_ref = candidate_identity(
        "offline-cut-restore-attempt",
        &candidate.observer_safe_integrity_material(),
    );
    let result =
        LocalFabric::restore_local_cut(program, sealed_admission, BackendProfile::St, &candidate);
    let rejected = result.is_err();
    let state_after = candidate_identity("offline-cut-state-before", &fabric.semantic_snapshot());
    TestOfflineCutCandidate {
        before: cut_ref.clone(),
        after: restore_attempt_ref.clone(),
        rejected,
        detail: I2OfflineCutCandidate {
            diagnostic_code: "OfflineCutCorruption".to_string(),
            source: "actual-cut-restore-control".to_string(),
            cut_ref,
            restore_attempt_ref,
            restore_result: if rejected {
                "typed_rejected"
            } else {
                "typed_accepted"
            }
            .to_string(),
            mutation_applied: state_before != state_after,
            state_digest_before: state_before.clone(),
            state_digest_after: state_after,
        },
    }
}

#[cfg(test)]
fn edge_kind_from_test_name(name: &str) -> Option<CommunicationEdgeKind> {
    match name {
        "owner-request" => Some(CommunicationEdgeKind::OwnerRequest),
        "owner-reply-receipt" => Some(CommunicationEdgeKind::OwnerReplyReceipt),
        "relation-projection-publication" => {
            Some(CommunicationEdgeKind::RelationProjectionPublication)
        }
        "designated-input-request" => Some(CommunicationEdgeKind::DesignatedInputRequest),
        "designated-input-receipt" => Some(CommunicationEdgeKind::DesignatedInputReceipt),
        "designated-result-delivery" => Some(CommunicationEdgeKind::DesignatedResultDelivery),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectionFacts {
    loci: Vec<String>,
    artifacts: Vec<ArtifactFact>,
    edges: Vec<EdgeFact>,
}

impl ProjectionFacts {
    fn from_summary(summary: &crate::sys5_local_slice::Sys5SemanticSummary) -> Self {
        let mut artifacts = summary
            .artifacts
            .iter()
            .map(ArtifactFact::from_summary)
            .collect::<Vec<_>>();
        artifacts.sort();
        let mut edges = summary
            .generated_communication
            .iter()
            .map(EdgeFact::from_summary)
            .collect::<Vec<_>>();
        edges.sort();
        Self {
            loci: summary.loci.clone(),
            artifacts,
            edges,
        }
    }

    fn from_projection(projection: &GlobalProjectionResult) -> Self {
        let mut artifacts = projection
            .locus_order()
            .into_iter()
            .flat_map(|locus| {
                projection
                    .locus_program(locus)
                    .expect("projection retains each locus")
                    .operation_fragments()
                    .iter()
                    .map(move |fragment| ArtifactFact {
                        locus: locus.to_string(),
                        kind: fragment_kind_name(fragment.fragment_kind()).to_string(),
                        operation_id: fragment.operation_id().to_string(),
                        derived_from_checked_core: true,
                        source_span: span_from_source_ref(fragment.source_ref()),
                        core_ref: fragment.core_ref().unwrap_or_default().to_string(),
                        fragment_ref: fragment.fragment_ref().to_string(),
                        checked_program_identity: sys5_checked_identity_ref(
                            fragment
                                .checked_core_identity()
                                .checked_program_identity()
                                .stable_key(),
                        ),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        artifacts.sort();
        let mut edges = projection
            .communication_plan()
            .edges()
            .iter()
            .map(|edge| EdgeFact {
                kind: edge_kind_name(edge.kind()).to_string(),
                from_locus: edge.source_locus().to_string(),
                to_locus: edge.target_locus().to_string(),
                operation_id: edge.operation_id().to_string(),
                derived_from_checked_core: edge.is_derived_from_checked_core(),
                transfers_authority: edge.transfers_authority(),
                source_span: span_from_source_ref(&edge.source_ref()),
                core_ref: edge.core_ref().map(str::to_string),
                edge_ref: edge.edge_ref().to_string(),
                source_fragment_ref: edge.source_fragment_ref().clone(),
                target_fragment_ref: edge.target_fragment_ref().clone(),
                checked_program_identity: sys5_checked_identity_ref(
                    edge.checked_core_identity()
                        .checked_program_identity()
                        .stable_key(),
                ),
            })
            .collect::<Vec<_>>();
        edges.sort();
        Self {
            loci: projection
                .locus_order()
                .into_iter()
                .map(str::to_string)
                .collect(),
            artifacts,
            edges,
        }
    }

    fn owner_locus(&self, operation: &str) -> Option<&str> {
        self.artifacts
            .iter()
            .find(|artifact| {
                artifact.operation_id == operation && artifact.kind == "owner-rmw-evaluation"
            })
            .map(|artifact| artifact.locus.as_str())
    }

    fn contains_artifact_ref(&self, artifact_ref: &str) -> bool {
        self.artifacts
            .iter()
            .any(|artifact| artifact.fragment_ref == artifact_ref)
    }

    fn contains_edge_ref(&self, edge_ref: &str) -> bool {
        self.edges.iter().any(|edge| edge.edge_ref == edge_ref)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ArtifactFact {
    locus: String,
    kind: String,
    operation_id: String,
    derived_from_checked_core: bool,
    source_span: I2SourceSpan,
    core_ref: String,
    fragment_ref: String,
    checked_program_identity: String,
}

impl ArtifactFact {
    fn from_summary(summary: &Sys5ArtifactSummary) -> Self {
        Self {
            locus: summary.locus.clone(),
            kind: summary.kind.clone(),
            operation_id: summary.operation_id.clone(),
            derived_from_checked_core: summary.derived_from_checked_core,
            source_span: span_from_summary(summary.source_span),
            core_ref: summary.core_ref.clone(),
            fragment_ref: summary.fragment_ref.clone(),
            checked_program_identity: summary.checked_program_identity.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EdgeFact {
    kind: String,
    from_locus: String,
    to_locus: String,
    operation_id: String,
    derived_from_checked_core: bool,
    transfers_authority: bool,
    source_span: I2SourceSpan,
    core_ref: Option<String>,
    edge_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    checked_program_identity: String,
}

impl EdgeFact {
    fn from_summary(summary: &Sys5CommunicationSummary) -> Self {
        Self {
            kind: summary.kind.clone(),
            from_locus: summary.from_locus.clone(),
            to_locus: summary.to_locus.clone(),
            operation_id: summary.operation_id.clone(),
            derived_from_checked_core: summary.derived_from_checked_core,
            transfers_authority: summary.transfers_authority,
            source_span: span_from_summary(summary.source_span),
            core_ref: summary.core_ref.clone(),
            edge_ref: summary.edge_ref.clone(),
            source_fragment_ref: summary.source_fragment_ref.clone(),
            target_fragment_ref: summary.target_fragment_ref.clone(),
            checked_program_identity: summary.checked_program_identity.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeAnchor {
    checked_program_identity_ref: String,
    source_span: I2SourceSpan,
    core_ref: String,
    artifact_ref: String,
    edge_ref: String,
    request_identity: String,
    dispatch_occurrence_ref: String,
    receive_occurrence_ref: String,
    serve_occurrence_ref: String,
}

impl RuntimeAnchor {
    fn from_workflow(workflow: &Sys5LocalWorkflowReport, facts: &ProjectionFacts) -> Option<Self> {
        let fallback = facts.artifacts.iter().find(|artifact| {
            artifact.operation_id == "attack" && artifact.kind == "owner-rmw-evaluation"
        })?;
        let detail = workflow
            .joined_rows()
            .iter()
            .filter_map(|row| row.detail())
            .find(|detail| {
                detail.core_ref() == fallback.core_ref
                    && detail.request_enqueue_occurrence_ref().is_some()
                    && detail.dispatch_occurrence_ref().is_some()
                    && detail.receive_occurrence_ref().is_some()
                    && !detail.serve_occurrence_ref().is_empty()
            })?;
        Some(Self {
            checked_program_identity_ref: fallback.checked_program_identity.clone(),
            source_span: I2SourceSpan {
                start: detail.source_span().start(),
                end: detail.source_span().end(),
                start_line: detail.source_span().start_line(),
                start_column: detail.source_span().start_column(),
                end_line: detail.source_span().end_line(),
                end_column: detail.source_span().end_column(),
            },
            core_ref: detail.core_ref().to_string(),
            artifact_ref: detail.serve_fragment_ref().to_string(),
            edge_ref: detail.edge_ref().to_string(),
            request_identity: detail.request_identity().to_string(),
            dispatch_occurrence_ref: detail.dispatch_occurrence_ref()?.to_string(),
            receive_occurrence_ref: detail.receive_occurrence_ref()?.to_string(),
            serve_occurrence_ref: detail.serve_occurrence_ref().to_string(),
        })
    }

    fn from_actual_causal_detail(
        detail: &crate::sys5_local_workflow::Sys5WorkflowCausalDetail,
        facts: &ProjectionFacts,
    ) -> Option<Self> {
        let artifact = facts
            .artifacts
            .iter()
            .find(|artifact| {
                artifact.core_ref == detail.core_ref()
                    && (artifact.fragment_ref == detail.request_fragment_ref()
                        || artifact.fragment_ref == detail.serve_fragment_ref())
            })
            .or_else(|| {
                facts.artifacts.iter().find(|artifact| {
                    artifact.fragment_ref == detail.request_fragment_ref()
                        || artifact.fragment_ref == detail.serve_fragment_ref()
                })
            })?;
        facts
            .edges
            .iter()
            .find(|edge| edge.edge_ref == detail.edge_ref())?;
        // Relation publication uses publish/observe labels while owner and
        // designated paths use dispatch/receive. Preserve whichever actual
        // causal occurrences SYS-5 retained; do not manufacture a missing
        // transport stage merely to give every property an attack-shaped row.
        let dispatch_occurrence_ref = detail
            .dispatch_occurrence_ref()
            .or_else(|| detail.request_enqueue_occurrence_ref())
            .or_else(|| detail.owner_publish_occurrence_ref())
            .unwrap_or_default()
            .to_string();
        let receive_occurrence_ref = detail
            .receive_occurrence_ref()
            .or_else(|| detail.observe_occurrence_ref())
            .unwrap_or_default()
            .to_string();
        (!detail.serve_occurrence_ref().is_empty()).then(|| Self {
            checked_program_identity_ref: artifact.checked_program_identity.clone(),
            source_span: I2SourceSpan {
                start: detail.source_span().start(),
                end: detail.source_span().end(),
                start_line: detail.source_span().start_line(),
                start_column: detail.source_span().start_column(),
                end_line: detail.source_span().end_line(),
                end_column: detail.source_span().end_column(),
            },
            core_ref: detail.core_ref().to_string(),
            artifact_ref: detail.serve_fragment_ref().to_string(),
            edge_ref: detail.edge_ref().to_string(),
            request_identity: detail.request_identity().to_string(),
            dispatch_occurrence_ref,
            receive_occurrence_ref,
            serve_occurrence_ref: detail.serve_occurrence_ref().to_string(),
        })
    }

    fn from_actual_lifecycle(
        fallback: &Self,
        lifecycle: &crate::sys5_local_workflow::Sys5WorkflowLifecycleRefs,
    ) -> Self {
        Self {
            checked_program_identity_ref: fallback.checked_program_identity_ref.clone(),
            source_span: fallback.source_span,
            // Lifecycle rows retain checked-program and cut-artifact evidence,
            // not an operation Core or generated communication edge.
            core_ref: String::new(),
            artifact_ref: lifecycle.after_artifact_ref().to_string(),
            edge_ref: String::new(),
            request_identity: String::new(),
            dispatch_occurrence_ref: lifecycle.occurrence_ref().to_string(),
            receive_occurrence_ref: String::new(),
            serve_occurrence_ref: String::new(),
        }
    }

    fn from_actual_model(fallback: &Self) -> Self {
        Self {
            checked_program_identity_ref: fallback.checked_program_identity_ref.clone(),
            source_span: fallback.source_span,
            core_ref: String::new(),
            artifact_ref: String::new(),
            edge_ref: String::new(),
            request_identity: String::new(),
            dispatch_occurrence_ref: String::new(),
            receive_occurrence_ref: String::new(),
            serve_occurrence_ref: String::new(),
        }
    }

    fn has_full_dispatch_lifecycle(&self) -> bool {
        !self.request_identity.is_empty()
            && !self.dispatch_occurrence_ref.is_empty()
            && !self.receive_occurrence_ref.is_empty()
            && !self.serve_occurrence_ref.is_empty()
    }

    fn is_observer_safe(&self) -> bool {
        self.source_span.end > self.source_span.start
            && [
                &self.checked_program_identity_ref,
                &self.core_ref,
                &self.artifact_ref,
                &self.edge_ref,
                &self.request_identity,
                &self.dispatch_occurrence_ref,
                &self.receive_occurrence_ref,
                &self.serve_occurrence_ref,
            ]
            .into_iter()
            .all(|reference| {
                !reference.is_empty() && !contains_observer_sensitive_identifier(reference)
            })
    }

    fn has_projection_trace_join(&self) -> bool {
        self.source_span.end > self.source_span.start
            && !self.checked_program_identity_ref.is_empty()
            && !self.core_ref.is_empty()
            && !self.artifact_ref.is_empty()
            && !self.edge_ref.is_empty()
            && self.has_full_dispatch_lifecycle()
    }
}

#[derive(Debug, Clone)]
struct SelectedBackendRun {
    semantic_digest: String,
    completed_generated_dispatches: bool,
    runtime_profile: String,
    sole_worker_locus: Option<String>,
    worker_owned_m8: bool,
    mailbox_fifo: bool,
    lifecycle_refs: Vec<String>,
    typed_receipt_ref: String,
    typed_result_ref: String,
    state_digest: String,
    frontier_ref: String,
    trace_digest: String,
    action_outcomes: Vec<SelectedActionOutcome>,
    all_actions_succeeded: bool,
    same_mailbox_fifo_control: Option<I2SameMailboxFifoControl>,
    locus_count: usize,
    artifact_count: usize,
    generated_edge_count: usize,
    artifact_refs: Vec<String>,
    generated_edge_refs: Vec<String>,
    owner_worker_locus: String,
    requester_locus: String,
    designated_evaluator_locus: String,
    consumer_locus: String,
    source_loci: Vec<String>,
    runtime_anchor: RuntimeAnchor,
}

impl SelectedBackendRun {
    fn telemetry(&self) -> I2BackendTelemetry {
        I2BackendTelemetry {
            runtime_profile: self.runtime_profile.clone(),
            sole_worker_locus: self.sole_worker_locus.clone(),
            worker_owned_m8: self.worker_owned_m8,
            mailbox_fifo: self.mailbox_fifo,
            lifecycle_refs: self.lifecycle_refs.clone(),
            typed_receipt_ref: self.typed_receipt_ref.clone(),
            typed_result_ref: self.typed_result_ref.clone(),
            state_digest: self.state_digest.clone(),
            frontier_ref: self.frontier_ref.clone(),
            trace_digest: self.trace_digest.clone(),
            action_outcomes: self
                .action_outcomes
                .iter()
                .map(SelectedActionOutcome::telemetry)
                .collect(),
            all_actions_succeeded: self.all_actions_succeeded,
            same_mailbox_fifo_control: self.same_mailbox_fifo_control.clone(),
        }
    }
}

fn run_selected_backend(
    source: &str,
    profile: Sys5LocalRuntimeProfile,
    logical_path: &str,
) -> Result<SelectedBackendRun, I2ConformanceError> {
    let project = build_project(Sys5SourceInput::inline(logical_path, source))
        .map_err(|_| I2ConformanceError::InvalidSelectedOw1Source)?;
    let designated_value = project
        .semantic_summary()
        .artifacts
        .iter()
        .find(|artifact| artifact.kind == "designated-evaluation")
        .map(|artifact| artifact.operation_id.clone())
        .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let selected_summary = project.semantic_summary();
    let locus_count = selected_summary.loci.len();
    let artifact_refs = selected_summary
        .artifacts
        .iter()
        .map(|artifact| artifact.fragment_ref.clone())
        .collect::<Vec<_>>();
    let generated_edge_refs = selected_summary
        .generated_communication
        .iter()
        .map(|edge| edge.edge_ref.clone())
        .collect::<Vec<_>>();
    let artifact_count = artifact_refs.len();
    let generated_edge_count = generated_edge_refs.len();
    let selected_owner_edge = selected_summary
        .generated_communication
        .iter()
        .find(|edge| edge.kind == "owner-request" && edge.operation_id == "attack")
        .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let owner_worker_locus = selected_owner_edge.to_locus.clone();
    let requester_locus = selected_owner_edge.from_locus.clone();
    let designated_evaluator_locus = selected_summary
        .artifacts
        .iter()
        .find(|artifact| artifact.kind == "designated-evaluation")
        .map(|artifact| artifact.locus.clone())
        .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let consumer_locus = selected_summary
        .artifacts
        .iter()
        .find(|artifact| artifact.kind == "designated-result-consumer")
        .map(|artifact| artifact.locus.clone())
        .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let source_loci = selected_summary.loci.clone();
    let admission = project
        .prepare_canonical_local_admission(profile)
        .map_err(|_| I2ConformanceError::SelectedBackendRejected)?;
    let startup_operations = admission
        .source_derived_startup_operations_for_i2()
        .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let fifo_operation = startup_operations
        .first()
        .cloned()
        .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let owner_operations = project
        .semantic_summary()
        .artifacts
        .iter()
        .filter(|artifact| {
            artifact.kind == "owner-rmw-evaluation"
                && !startup_operations.contains(&artifact.operation_id)
        })
        .map(|artifact| artifact.operation_id.clone())
        .collect::<Vec<_>>();
    let [owner_operation] = owner_operations.as_slice() else {
        return Err(I2ConformanceError::SelectedBackendRejected);
    };
    let (program, sealed_admission) = admission.into_parts_for_sys4();
    let backend = match profile {
        Sys5LocalRuntimeProfile::St => BackendProfile::St,
        Sys5LocalRuntimeProfile::Ow1 => BackendProfile::Ow1,
    };
    let mut fabric = LocalFabric::bootstrap(program, sealed_admission, backend)
        .map_err(|_| I2ConformanceError::SelectedBackendRejected)?;
    let mut outcomes = startup_operations
        .into_iter()
        .map(|operation| run_selected_action(&mut fabric, SourceAction::owner_operation(operation)))
        .collect::<Vec<_>>();
    outcomes.extend([
        run_selected_action(
            &mut fabric,
            SourceAction::owner_operation(owner_operation).with_argument("target", "self"),
        ),
        run_selected_action(
            &mut fabric,
            SourceAction::designated_tick(designated_value.clone())
                .with_tick("F", "i2-selected-tick"),
        ),
        run_selected_action(
            &mut fabric,
            SourceAction::consume_designated_result(designated_value),
        ),
    ]);
    let fifo_first_action = SourceAction::owner_operation(&fifo_operation);
    let fifo_second_action = SourceAction::owner_operation(&fifo_operation);
    let fifo_execution = fabric
        .execute_source_owner_fifo_pair(fifo_first_action.clone(), fifo_second_action.clone())
        .map_err(|_| I2ConformanceError::SelectedBackendRejected)?;
    let fifo_witness = fifo_execution.witness().clone();
    let fifo_outcomes = [
        selected_action_success(&fifo_first_action, fifo_execution.first_receipt()),
        selected_action_success(&fifo_second_action, fifo_execution.second_receipt()),
    ];
    let fifo_typed_receipt_refs = fifo_outcomes
        .iter()
        .map(|outcome| outcome.typed_receipt_ref.clone())
        .collect::<Vec<_>>();
    outcomes.extend(fifo_outcomes);
    // A selected ST/OW run is evidence only when every source-derived setup
    // and representative action actually returns a typed success.  A shared
    // error outcome is recorded as an error, never promoted to correspondence.
    let completed_generated_dispatches = outcomes.iter().all(SelectedActionOutcome::succeeded);
    let actual_trace = fabric
        .trace()
        .canonical_correspondence_excluding_debug_worker_tokens();
    let runtime_anchor =
        selected_runtime_anchor(&fabric, selected_summary, &fifo_operation, &outcomes)
            .ok_or(I2ConformanceError::SelectedBackendRejected)?;
    let backend_facts = fabric.observer_backend_execution_facts();
    let same_mailbox_fifo_control = Some(I2SameMailboxFifoControl {
        source: "actual-source-derived-queued-owner-pair".to_string(),
        all_actions_succeeded: outcomes.iter().all(SelectedActionOutcome::succeeded),
        same_mailbox_owner_locus: fifo_witness.owner_locus().to_string(),
        request_ids: fifo_witness.request_ids().to_vec(),
        enqueue_order: fifo_witness.enqueue_order().to_vec(),
        serve_order: fifo_witness.serve_order().to_vec(),
        second_enqueued_before_first_serve: fifo_witness.second_enqueued_before_first_serve(),
        typed_receipt_refs: fifo_typed_receipt_refs,
    });
    let mailbox_fifo = same_mailbox_fifo_control.as_ref().is_some_and(|control| {
        control.all_actions_succeeded
            && control.request_ids.len() >= 2
            && control.request_ids == control.enqueue_order
            && control.enqueue_order == control.serve_order
            && control.second_enqueued_before_first_serve
            && control.typed_receipt_refs.len() == 2
            && control
                .typed_receipt_refs
                .iter()
                .all(|receipt| !receipt.is_empty())
    });
    let lifecycle_refs = actual_trace
        .iter()
        .enumerate()
        .map(|(index, (operation, kind, _))| {
            let mut hasher = Sha256::new();
            hasher.update(b"mirrorea/i2/selected-backend-trace-ref/v1\0");
            hasher.update(operation.as_bytes());
            hasher.update(format!("{kind:?}").as_bytes());
            hasher.update(index.to_le_bytes());
            format!("i2-backend-trace:{:x}", hasher.finalize())
        })
        .collect::<Vec<_>>();
    let typed_result_ref = control_identity(
        "selected-backend-typed-results",
        &outcomes
            .iter()
            .map(SelectedActionOutcome::typed_result_ref)
            .collect::<Vec<_>>(),
    );
    let typed_receipt_ref = control_identity(
        "selected-backend-typed-receipts",
        &outcomes
            .iter()
            .map(SelectedActionOutcome::typed_receipt_ref)
            .collect::<Vec<_>>(),
    );
    let frontier_ref = control_identity(
        "selected-backend-frontiers",
        &outcomes
            .iter()
            .map(SelectedActionOutcome::frontier_ref)
            .collect::<Vec<_>>(),
    );
    let state_digest = control_identity(
        "selected-backend-active-state",
        &fabric.active_runtime_identity_snapshot(),
    );
    let trace_digest = control_identity("selected-backend-canonical-trace", &actual_trace);
    let mut hasher = Sha256::new();
    hasher.update(SELECTED_BACKEND_DOMAIN);
    for outcome in &outcomes {
        hasher.update(outcome.classification().as_bytes());
        hasher.update([0]);
    }
    hasher.update(typed_receipt_ref.as_bytes());
    hasher.update(typed_result_ref.as_bytes());
    hasher.update(state_digest.as_bytes());
    hasher.update(frontier_ref.as_bytes());
    hasher.update(trace_digest.as_bytes());
    let all_actions_succeeded = outcomes.iter().all(SelectedActionOutcome::succeeded);
    Ok(SelectedBackendRun {
        semantic_digest: format!("i2-selected-semantic-sha256-v1:{:x}", hasher.finalize()),
        completed_generated_dispatches,
        runtime_profile: match profile {
            Sys5LocalRuntimeProfile::St => "ST",
            Sys5LocalRuntimeProfile::Ow1 => "OW1",
        }
        .to_string(),
        worker_owned_m8: backend_facts.worker_owned_m8(),
        mailbox_fifo,
        sole_worker_locus: backend_facts.worker_locus().map(ToOwned::to_owned),
        lifecycle_refs,
        typed_receipt_ref,
        typed_result_ref,
        state_digest,
        frontier_ref,
        trace_digest,
        action_outcomes: outcomes,
        all_actions_succeeded,
        same_mailbox_fifo_control,
        locus_count,
        artifact_count,
        generated_edge_count,
        artifact_refs,
        generated_edge_refs,
        owner_worker_locus,
        requester_locus,
        designated_evaluator_locus,
        consumer_locus,
        source_loci,
        runtime_anchor,
    })
}

/// Build one selected-source anchor from an actual source-derived owner
/// request.  This must never reuse the primary workflow's WorldAuthority
/// attack anchor: it joins the selected A/S/E/C projection with its own
/// generated endpoint and M8 serve occurrences.
fn selected_runtime_anchor(
    fabric: &LocalFabric,
    summary: &crate::sys5_local_slice::Sys5SemanticSummary,
    operation: &str,
    outcomes: &[SelectedActionOutcome],
) -> Option<RuntimeAnchor> {
    let edge = summary.generated_communication.iter().find(|edge| {
        edge.kind == "owner-request"
            && edge.operation_id == operation
            && edge.derived_from_checked_core
    })?;
    let request_id = outcomes
        .iter()
        .find(|outcome| outcome.action_ref == operation)
        .and_then(|outcome| outcome.request_id.as_deref())?;
    let endpoint = fabric.observer_exact_endpoint_occurrences(
        request_id,
        crate::sys4_dispatch::Sys4TraceKind::Dispatched,
        crate::sys4_dispatch::Sys4TraceKind::Received,
        CommunicationEdgeKind::OwnerRequest,
        &edge.from_locus,
        &edge.to_locus,
    )?;
    let serve_occurrence_ref = fabric.observer_exact_m8_occurrence(
        request_id,
        crate::m8_runtime_local_cut::M8LocalTraceKind::OwnerWrite,
    )?;
    let artifact = summary
        .artifacts
        .iter()
        .find(|artifact| artifact.fragment_ref == endpoint.target_fragment_ref())?;
    (artifact.core_ref == endpoint.core_ref()
        && edge.edge_ref == endpoint.edge_ref()
        && !endpoint.request_enqueue_occurrence_id().is_empty()
        && !endpoint.dispatch_occurrence_id().is_empty()
        && !endpoint.receive_occurrence_id().is_empty()
        && !serve_occurrence_ref.is_empty())
    .then(|| RuntimeAnchor {
        checked_program_identity_ref: artifact.checked_program_identity.clone(),
        source_span: span_from_source_ref(endpoint.source_ref()),
        core_ref: endpoint.core_ref().to_string(),
        artifact_ref: endpoint.target_fragment_ref().to_string(),
        edge_ref: endpoint.edge_ref().to_string(),
        request_identity: request_id.to_string(),
        dispatch_occurrence_ref: endpoint.dispatch_occurrence_id().to_string(),
        receive_occurrence_ref: endpoint.receive_occurrence_id().to_string(),
        serve_occurrence_ref: serve_occurrence_ref.to_string(),
    })
}

#[derive(Debug, Clone)]
struct SelectedActionOutcome {
    action_ref: String,
    attempted_provenance_ref: String,
    classification: &'static str,
    request_id: Option<String>,
    receipt_occurrence_ref: String,
    typed_receipt_ref: String,
    typed_result_ref: String,
    frontier_ref: String,
    succeeded: bool,
    diagnostic_code: Option<String>,
}

fn run_selected_action(fabric: &mut LocalFabric, action: SourceAction) -> SelectedActionOutcome {
    let action_ref = action.operation_id().to_string();
    let attempted_provenance_ref = selected_action_provenance_ref(&action);
    match fabric.dispatch_source_action(action) {
        Ok(receipt) => {
            selected_action_success_with_provenance(action_ref, attempted_provenance_ref, &receipt)
        }
        Err(diagnostic) => SelectedActionOutcome {
            action_ref,
            attempted_provenance_ref,
            classification: match diagnostic.primary().kind() {
                crate::sys4_dispatch::Sys4DiagnosticKind::M8ExecutionRejected => {
                    "typed-m8-rejection"
                }
                crate::sys4_dispatch::Sys4DiagnosticKind::MissingTypedDesignatedValue => {
                    "typed-missing-designated-value"
                }
                crate::sys4_dispatch::Sys4DiagnosticKind::MissingPublishedResult => {
                    "typed-missing-published-result"
                }
                _ => "typed-dispatch-rejection",
            },
            request_id: None,
            receipt_occurrence_ref: String::new(),
            typed_receipt_ref: control_identity(
                "selected-action-typed-error-receipt",
                &format!("{:?}", diagnostic.primary().kind()),
            ),
            typed_result_ref: control_identity(
                "selected-action-typed-error",
                &format!("{:?}", diagnostic.primary().kind()),
            ),
            frontier_ref: control_identity("selected-action-error-frontier", &"missing"),
            succeeded: false,
            diagnostic_code: Some(format!("{:?}", diagnostic.primary().kind())),
        },
    }
}

fn selected_action_provenance_ref(action: &SourceAction) -> String {
    control_identity(
        "selected-action-runtime-invocation",
        &(action.operation_id(), action),
    )
}

fn selected_action_success(
    action: &SourceAction,
    receipt: &crate::sys4_dispatch::FabricReceipt,
) -> SelectedActionOutcome {
    selected_action_success_with_provenance(
        action.operation_id().to_string(),
        selected_action_provenance_ref(action),
        receipt,
    )
}

fn selected_action_success_with_provenance(
    action_ref: String,
    attempted_provenance_ref: String,
    receipt: &crate::sys4_dispatch::FabricReceipt,
) -> SelectedActionOutcome {
    SelectedActionOutcome {
        action_ref,
        attempted_provenance_ref,
        classification: match receipt.typed_value() {
            crate::sys4_dispatch::RuntimeValue::Int(_) => "typed-int-success",
            crate::sys4_dispatch::RuntimeValue::Unit => "typed-unit-success",
        },
        request_id: Some(receipt.request_id().to_string()),
        receipt_occurrence_ref: control_identity(
            "selected-action-receipt-occurrence",
            &(
                receipt.request_id(),
                receipt.delivery_id(),
                receipt.operation_id(),
            ),
        ),
        typed_receipt_ref: control_identity(
            "selected-action-typed-receipt",
            &(
                receipt.request_id(),
                receipt.delivery_id(),
                receipt.operation_id(),
                receipt.origin_locus(),
                receipt.target_locus(),
                receipt.result_version(),
            ),
        ),
        typed_result_ref: control_identity(
            "selected-action-typed-result",
            &(
                receipt.operation_id(),
                receipt.typed_value(),
                receipt.result_version(),
            ),
        ),
        frontier_ref: control_identity(
            "selected-action-frontier",
            &(receipt.logical_tick_id(), receipt.logical_tick_frontier()),
        ),
        succeeded: true,
        diagnostic_code: None,
    }
}

impl SelectedActionOutcome {
    fn classification(&self) -> &str {
        self.classification
    }

    fn typed_result_ref(&self) -> &str {
        &self.typed_result_ref
    }

    fn typed_receipt_ref(&self) -> &str {
        &self.typed_receipt_ref
    }

    fn frontier_ref(&self) -> &str {
        &self.frontier_ref
    }

    const fn succeeded(&self) -> bool {
        self.succeeded
    }

    fn telemetry(&self) -> I2SelectedActionTelemetry {
        I2SelectedActionTelemetry {
            action_ref: self.action_ref.clone(),
            attempted: true,
            completed: self.succeeded,
            status: if self.succeeded {
                "typed_success".to_string()
            } else {
                "typed_rejection".to_string()
            },
            result_kind: self.classification.to_string(),
            typed_result_ref: self.typed_result_ref.clone(),
            receipt_occurrence_ref: self.receipt_occurrence_ref.clone(),
            attempted_provenance_ref: self.attempted_provenance_ref.clone(),
            diagnostic_code: self.diagnostic_code.clone(),
        }
    }
}

fn selected_model(profile: ModelExecutionProfile) -> crate::sys2_bounded_model::ModelCheckReport {
    Sys2BoundedModel::new()
        .with_profile(profile)
        .with_bound(6)
        .with_required_edges([
            RequiredEdge::OwnerRequestServe,
            RequiredEdge::PublishObserve,
            RequiredEdge::WitnessCreateUse,
            RequiredEdge::CapabilityGrantUse,
            RequiredEdge::RevocationVisibility,
            RequiredEdge::PatchActivationVisibility,
            RequiredEdge::CutSaveQuiescence,
            RequiredEdge::RelationEpochSample,
            RequiredEdge::SameOwnerReadsFromCoherence,
            RequiredEdge::PresentationGapNonmutation,
        ])
        .with_litmus_cases([
            LitmusCase::owner_request_serve_message_passing(),
            LitmusCase::store_buffering_calibration(),
            LitmusCase::publication_observation(),
            LitmusCase::witness_creation_use(),
            LitmusCase::capability_revoke_use_race(),
            LitmusCase::patch_activate_request_race(),
            LitmusCase::save_cut_mutation_race(),
            LitmusCase::relation_epoch_sample_race(),
            LitmusCase::same_owner_two_request_rmw(),
            LitmusCase::presentation_gap_nonmutation(),
        ])
        .check()
}

#[derive(Debug, Serialize)]
struct I2ManifestMaterial {
    bounded_implementation_source_fingerprint_id: String,
    primary_content_identity_ref: String,
    selected_ow1_content_identity_ref: String,
    primary_checked_program_identity_ref: String,
    patch_identity_refs: Vec<String>,
    selected_st_semantic_digest: String,
    selected_ow1_semantic_digest: String,
    model_st_fingerprint: String,
    model_ow1_fingerprint: String,
}

fn bounded_implementation_source_fingerprint() -> I2BoundedImplementationSourceFingerprint {
    let mut hasher = Sha256::new();
    hasher.update(BOUNDED_IMPLEMENTATION_SOURCE_FINGERPRINT_DOMAIN);
    for source in [
        include_str!("sys6_i2_conformance.rs"),
        include_str!("sys5_local_slice.rs"),
        include_str!("sys5_local_workflow.rs"),
        include_str!("sys4_dispatch.rs"),
        include_str!("sys2_bounded_model.rs"),
        include_str!("sys2_execution_backend.rs"),
        include_str!("sys3_projection/lowering.rs"),
        include_str!("sys3_projection/model.rs"),
        include_str!("sys3_projection/validate.rs"),
        include_str!("m9_auth_verification.rs"),
        include_str!("semantic_runtime_kernel.rs"),
        include_str!("../../mir-semantics/src/surface_v0_pipeline.rs"),
    ] {
        hasher.update(source.as_bytes());
        hasher.update([0]);
    }
    I2BoundedImplementationSourceFingerprint {
        id: format!(
            "i2-bounded-implementation-source-sha256-v1:{:x}",
            hasher.finalize()
        ),
        scope: "i2-provisional-implementation-source".to_string(),
        runtime_identity_claim: false,
        public_release_cut: false,
        source_components: vec![
            "crates/mir-runtime/src/sys6_i2_conformance.rs".to_string(),
            "crates/mir-runtime/src/sys5_local_slice.rs".to_string(),
            "crates/mir-runtime/src/sys5_local_workflow.rs".to_string(),
            "crates/mir-runtime/src/sys4_dispatch.rs".to_string(),
            "crates/mir-runtime/src/sys2_bounded_model.rs".to_string(),
            "crates/mir-runtime/src/sys2_execution_backend.rs".to_string(),
            "crates/mir-runtime/src/sys3_projection/lowering.rs".to_string(),
            "crates/mir-runtime/src/sys3_projection/model.rs".to_string(),
            "crates/mir-runtime/src/sys3_projection/validate.rs".to_string(),
            "crates/mir-runtime/src/m9_auth_verification.rs".to_string(),
            "crates/mir-runtime/src/semantic_runtime_kernel.rs".to_string(),
            "crates/mir-semantics/src/surface_v0_pipeline.rs".to_string(),
        ],
    }
}

fn manifest_identity_ref(material: &I2ManifestMaterial) -> String {
    let encoded = serde_json::to_vec(material).expect("I2 manifest material is serializable");
    let mut hasher = Sha256::new();
    hasher.update(MANIFEST_DOMAIN);
    hasher.update(encoded);
    format!("i2-conformance-sha256-v1:{:x}", hasher.finalize())
}

fn source_content_identity_ref(source: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(SOURCE_CONTENT_DOMAIN);
    hasher.update(source.as_bytes());
    format!("i2-source-content-sha256-v1:{:x}", hasher.finalize())
}

fn read_source(path: &Path, error: I2ConformanceError) -> Result<String, I2ConformanceError> {
    fs::read_to_string(path).map_err(|_| error)
}

/// A source path may be shown only when it is already a safe repository-style
/// logical path.  Absolute host paths and credential-like path components are
/// replaced by a stable logical label before they enter observer evidence.
fn observer_logical_source_path(path: &Path) -> String {
    let candidate = path.to_string_lossy();
    let logical_candidate = candidate
        .find("samples/")
        .map(|index| candidate[index..].to_string())
        .unwrap_or_else(|| candidate.into_owned());
    let lower = logical_candidate.to_ascii_lowercase();
    let safe = !logical_candidate.starts_with('/')
        && !logical_candidate.contains("..")
        && logical_candidate.ends_with(".mir")
        && logical_candidate
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'-' | b'_'))
        && !["secret", "token", "credential", "witness", "capability"]
            .iter()
            .any(|fragment| lower.contains(fragment));
    if safe {
        logical_candidate
    } else {
        "i2-selected-ow1-source.mir".to_string()
    }
}

fn span_from_summary(span: Sys5SourceSpan) -> I2SourceSpan {
    I2SourceSpan {
        start: span.start,
        end: span.end,
        start_line: span.start_line,
        start_column: span.start_column,
        end_line: span.end_line,
        end_column: span.end_column,
    }
}

fn span_from_source_ref(source: &mir_semantics::shared_model::SourceRef) -> I2SourceSpan {
    I2SourceSpan {
        start: u64::from(source.start_line) * 1_000_000 + u64::from(source.start_column),
        end: u64::from(source.end_line) * 1_000_000 + u64::from(source.end_column),
        start_line: source.start_line,
        start_column: source.start_column,
        end_line: source.end_line,
        end_column: source.end_column,
    }
}

fn sys5_checked_identity_ref(identity: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/checked-program-ref/v1\0");
    hasher.update(
        u64::try_from(identity.len())
            .expect("checked program identity length fits u64")
            .to_le_bytes(),
    );
    hasher.update(identity.as_bytes());
    format!("sys5-checked-program-sha256-v1:{:x}", hasher.finalize())
}

fn fragment_kind_name(kind: ProjectedOperationFragmentKind) -> &'static str {
    match kind {
        ProjectedOperationFragmentKind::OwnerRequestInvocation => "owner-request-invocation",
        ProjectedOperationFragmentKind::OwnerRmwExecution => "owner-rmw-evaluation",
        ProjectedOperationFragmentKind::RelationPublication => "relation-publication",
        ProjectedOperationFragmentKind::ConsumerLocalRelationProjection => {
            "consumer-local-relation-projection"
        }
        ProjectedOperationFragmentKind::DesignatedRemoteInputService => {
            "designated-remote-input-service"
        }
        ProjectedOperationFragmentKind::DesignatedEvaluation => "designated-evaluation",
        ProjectedOperationFragmentKind::DesignatedResultConsumer => "designated-result-consumer",
    }
}

fn edge_kind_name(kind: CommunicationEdgeKind) -> &'static str {
    match kind {
        CommunicationEdgeKind::OwnerRequest => "owner-request",
        CommunicationEdgeKind::OwnerReplyReceipt => "owner-reply-receipt",
        CommunicationEdgeKind::RelationProjectionPublication => "relation-projection-publication",
        CommunicationEdgeKind::DesignatedInputRequest => "designated-input-request",
        CommunicationEdgeKind::DesignatedInputReceipt => "designated-input-receipt",
        CommunicationEdgeKind::DesignatedResultDelivery => "designated-result-delivery",
        CommunicationEdgeKind::AbsoluteValueStream => "absolute-value-stream",
    }
}

fn full_toy_ow1_residual(
    admission_error: Sys5LocalAdmissionErrorKind,
    eligibility: BackendEligibility,
    projection: &GlobalProjectionResult,
) -> I2BackendResidual {
    let diagnostic_code = match admission_error {
        Sys5LocalAdmissionErrorKind::BackendIneligible => "BackendIneligible",
        Sys5LocalAdmissionErrorKind::ProjectionFabricMismatch => "ProjectionFabricMismatch",
        Sys5LocalAdmissionErrorKind::M9Rejected => "M9Rejected",
        _ => "AdmissionRejected",
    }
    .to_string();
    let reason = match eligibility {
        BackendEligibility::Ineligible {
            reason: BackendIneligibilityReason::MultipleCombinedOwnerSourceOwnerLoci { .. },
        } => "MultipleCombinedOwnerSourceOwnerLoci".to_string(),
        BackendEligibility::Ineligible {
            reason: BackendIneligibilityReason::NoCombinedOwnerSourceOwnerLocus,
        } => "NoCombinedOwnerSourceOwnerLocus".to_string(),
        BackendEligibility::Ineligible {
            reason: BackendIneligibilityReason::Ow1WorkerCutDeferred,
        } => "Ow1WorkerCutDeferred".to_string(),
        BackendEligibility::Eligible => "NoBackendIneligibilityReason".to_string(),
    };
    // Preserve the two distinct sources of the OW1 admission residual.  The
    // backend's combined candidate set is useful for deciding eligibility,
    // but serializing it twice would incorrectly claim that source-declared
    // owner roles and runtime owner roles are the same observation.
    let mut owner_loci = BTreeSet::new();
    let mut source_owner_loci = BTreeSet::new();
    for fragment in projection.sys4_artifact_fragments().entries() {
        if let Some(core) = fragment.owner_rmw_checked_core() {
            owner_loci.insert(core.owner_locus().to_string());
        }
        if let Some(core) = fragment.relation_checked_core() {
            owner_loci.insert(core.owner_locus().to_string());
        }
        if let Some(dependency) = fragment.designated_remote_input_dependency() {
            source_owner_loci.insert(dependency.source_owner_locus().to_string());
        }
    }
    I2BackendResidual {
        diagnostic_code,
        reason: reason.clone(),
        profile: "OW1".to_string(),
        admission_phase: "canonical_local_admission".to_string(),
        typed_admission_reason: I2TypedAdmissionReason {
            code: reason,
            owner_loci: owner_loci.into_iter().collect(),
            source_owner_loci: source_owner_loci.into_iter().collect(),
            owner_loci_semantics: "runtime_combined_owner_loci".to_string(),
            source_owner_loci_semantics: "source_declared_owner_loci".to_string(),
        },
        mutated_state: false,
    }
}

fn non_claims() -> Vec<String> {
    vec![
        "real transport".to_string(),
        "public ABI or wire freeze".to_string(),
        "durable distributed save/load".to_string(),
        "general metatheory".to_string(),
        "arbitrary scheduler fairness".to_string(),
        "arbitrary relation DAG theorem".to_string(),
        "four-locus OW1 whole-workflow execution".to_string(),
        "broad I1 exit".to_string(),
        "I2 lifecycle exit".to_string(),
        "I3 activation".to_string(),
    ]
}
