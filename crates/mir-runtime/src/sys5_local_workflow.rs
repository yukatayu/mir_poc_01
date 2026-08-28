//! One bounded source-first local workflow for the provisional SYS-5 profile.
//!
//! This module deliberately starts after Surface checking, Core elaboration,
//! projection, and admission preparation.  It owns no parser, route builder,
//! capability issuer, expected-result fixture, or filename-based dispatcher.

use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroUsize,
};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::sys5_local_slice::{
    Sys5ArtifactSummary, Sys5CommunicationSummary, Sys5FreshReacquireEvidence,
    Sys5LocalCutPatchError, Sys5LocalProject, Sys5ParticipantLeaveEvidence,
    Sys5ParticipantLeaveFailureEvidence, Sys5PatchDiagnosticKind, Sys5PatchVerdict,
    Sys5PreparedAdmission, Sys5PresentationGapEvidence, Sys5RelationDegradationEvidence,
    Sys5SourceCoreArtifactMapping, Sys5VerticalAction, Sys5VerticalDiagnosticKind,
    Sys5VerticalReceipt, Sys5VerticalSliceError, Sys5VerticalSliceRuntime,
};

const WORKFLOW_DIGEST_DOMAIN: &[u8] = b"mirrorea/sys5/local-workflow/v1\0";

/// A prechecked/preprojected patch plus its sealed source-derived admission.
/// `patch_id` is a caller-facing logical label only; patch verdicts are
/// derived solely by the checked SYS-4 candidate created from its contents.
pub struct Sys5LocalWorkflowPatchProject {
    patch_id: String,
    logical_path: String,
    project: Sys5LocalProject,
    admission: Sys5PreparedAdmission,
}

impl Sys5LocalWorkflowPatchProject {
    pub fn from_project_and_admission(
        patch_id: impl Into<String>,
        project: Sys5LocalProject,
        admission: Sys5PreparedAdmission,
    ) -> Self {
        Self {
            patch_id: patch_id.into(),
            logical_path: "prechecked-patch.mir".to_string(),
            project,
            admission,
        }
    }

    /// Test-local injection surface for a candidate logical label. Production
    /// callers use `with_cli_patch_ordinal`; execution validates both paths
    /// before this label can reach an observer report.
    #[cfg(test)]
    pub(crate) fn with_logical_path(mut self, logical_path: impl Into<String>) -> Self {
        self.logical_path = logical_path.into();
        self
    }

    /// Assigns the only CLI-visible patch provenance form. The caller cannot
    /// pass a host path, traversal, credential-like text, or an arbitrary
    /// label into the public workflow API.
    pub fn with_cli_patch_ordinal(mut self, ordinal: NonZeroUsize) -> Self {
        self.logical_path = format!("cli-patch-{:03}.mir", ordinal.get());
        self
    }
}

/// Prechecked/projection input to the bounded local workflow.
pub struct Sys5LocalWorkflowInput {
    project: Sys5LocalProject,
    admission: Sys5PreparedAdmission,
    patches: Vec<Sys5LocalWorkflowPatchProject>,
}

impl Sys5LocalWorkflowInput {
    pub fn from_project_and_admission(
        project: Sys5LocalProject,
        admission: Sys5PreparedAdmission,
    ) -> Self {
        Self {
            project,
            admission,
            patches: Vec::new(),
        }
    }

    pub fn with_patch_project(mut self, patch: Sys5LocalWorkflowPatchProject) -> Self {
        self.patches.push(patch);
        self
    }
}

/// Actual bounded schedule stages, serialized as stable profile-local names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Sys5LocalWorkflowStep {
    Startup,
    Attack,
    DesignatedPublish,
    ViewerConsume,
    RelationPrimary,
    PresentationGap,
    ParticipantALeave,
    DuplicateParticipantLeave,
    FreshReacquire,
    Save,
    Restore,
    PatchAccepted,
    PatchRejected,
    ConsumerCapabilityRevoke,
    FailedConsume,
    Verification,
}

/// One source-first patch verdict recorded by the actual activation boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowPatchVerdict {
    #[serde(skip)]
    patch_id: String,
    patch_ref: String,
    verdict: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostic: Option<String>,
}

/// Patch candidate provenance joined to its actual activation verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowPatchProvenance {
    patch_id: String,
    logical_path: String,
    checked_program_identity_ref: String,
    patch_ref: String,
    patch_occurrence_ref: String,
    verdict: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostic: Option<String>,
}

impl Sys5WorkflowPatchVerdict {
    pub fn patch_id(&self) -> &str {
        &self.patch_id
    }

    pub fn patch_ref(&self) -> &str {
        &self.patch_ref
    }

    pub fn verdict(&self) -> &str {
        &self.verdict
    }

    pub fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }
}

/// One observer-safe typed failure from an executed vertical action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowTypedFailure {
    diagnostic: String,
    rejected_before_cache_or_state_mutation: bool,
}

impl Sys5WorkflowTypedFailure {
    pub fn diagnostic(&self) -> &str {
        &self.diagnostic
    }

    pub const fn rejected_before_cache_or_state_mutation(&self) -> bool {
        self.rejected_before_cache_or_state_mutation
    }
}

/// A single joined devtools row.  Its `detail_ref` is an opaque reference or
/// a fixed profile-local label; it never contains raw source or M9 material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowJoinedRow {
    kind: String,
    detail_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_branch: Option<Sys5WorkflowExecutionBranch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<Sys5WorkflowCausalDetail>,
}

/// Observer-visible position of a runtime event around the one local
/// save/restore fork. Static source/Core rows intentionally omit this field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Sys5WorkflowExecutionBranch {
    ActivePrefix,
    DiscardedPostCut,
    ActiveRestored,
}

impl Sys5WorkflowJoinedRow {
    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn detail_ref(&self) -> &str {
        &self.detail_ref
    }

    pub fn detail(&self) -> Option<&Sys5WorkflowCausalDetail> {
        self.detail.as_ref()
    }
}

/// A nonzero source range retained from an existing observer-safe typed
/// segment. `start`/`end` are a lossless numeric ordering encoding of the
/// corresponding line/column pairs; the individual coordinates remain
/// available for a reader view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowSourceSpanDetail {
    start: u64,
    end: u64,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

impl Sys5WorkflowSourceSpanDetail {
    pub const fn start(&self) -> u64 {
        self.start
    }

    pub const fn end(&self) -> u64 {
        self.end
    }

    pub const fn start_line(&self) -> u32 {
        self.start_line
    }

    pub const fn start_column(&self) -> u32 {
        self.start_column
    }

    pub const fn end_line(&self) -> u32 {
        self.end_line
    }

    pub const fn end_column(&self) -> u32 {
        self.end_column
    }
}

/// Structured fields parsed from one actual `typed-segment:*` observer row.
/// This is an interpretation of the exact retained row, never a new route or
/// a synthesized runtime occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowCausalDetail {
    segment_kind: String,
    logical_path: String,
    source_span: Sys5WorkflowSourceSpanDetail,
    core_ref: String,
    request_fragment_ref: String,
    serve_fragment_ref: String,
    edge_ref: String,
    /// Generated carrier request identity. This is explicitly distinct from
    /// the following request-enqueue/dispatch/receive/serve occurrences.
    request_identity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_publish_occurrence_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_enqueue_occurrence_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_occurrence_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receive_occurrence_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observe_occurrence_ref: Option<String>,
    serve_occurrence_ref: String,
}

impl Sys5WorkflowCausalDetail {
    pub fn source_span(&self) -> &Sys5WorkflowSourceSpanDetail {
        &self.source_span
    }

    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub fn request_fragment_ref(&self) -> &str {
        &self.request_fragment_ref
    }

    pub fn serve_fragment_ref(&self) -> &str {
        &self.serve_fragment_ref
    }

    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn request_enqueue_occurrence_ref(&self) -> Option<&str> {
        self.request_enqueue_occurrence_ref.as_deref()
    }

    pub fn owner_publish_occurrence_ref(&self) -> Option<&str> {
        self.owner_publish_occurrence_ref.as_deref()
    }

    pub fn dispatch_occurrence_ref(&self) -> Option<&str> {
        self.dispatch_occurrence_ref.as_deref()
    }

    pub fn receive_occurrence_ref(&self) -> Option<&str> {
        self.receive_occurrence_ref.as_deref()
    }

    pub fn observe_occurrence_ref(&self) -> Option<&str> {
        self.observe_occurrence_ref.as_deref()
    }

    pub fn serve_occurrence_ref(&self) -> &str {
        &self.serve_occurrence_ref
    }
}

/// Observer-safe final relation state selected by the actual local fabric.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowRelationSummary {
    relation: String,
    selected_anchor: String,
    selected_floor: String,
    semantic_digest: String,
    lineage_ref: String,
}

/// Observer-safe result/cache evidence from an actual designated publish.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowDesignatedValueSummary {
    value_name: String,
    cache_ref: String,
    version_ref: String,
    result_version: u64,
    delivery_ref: String,
    cache_binding_ref: String,
    latest_value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5WorkflowRelationAlias {
    selected_anchor_ref: String,
    floor_ref: String,
    semantic_ref: String,
    lineage_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5WorkflowDesignatedAlias {
    result_ref: String,
    version: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5WorkflowCacheAlias {
    version_ref: String,
}

/// Opaque lifecycle boundary references parsed from actual lifecycle rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowLifecycleRefs {
    kind: String,
    occurrence_ref: String,
    before_core_ref: String,
    after_core_ref: String,
    before_artifact_ref: String,
    after_artifact_ref: String,
    before_frontier_ref: String,
    after_frontier_ref: String,
}

impl Sys5WorkflowLifecycleRefs {
    pub(crate) fn kind(&self) -> &str {
        &self.kind
    }

    pub(crate) fn occurrence_ref(&self) -> &str {
        &self.occurrence_ref
    }

    pub(crate) fn after_artifact_ref(&self) -> &str {
        &self.after_artifact_ref
    }
}

/// A snapshot of the admitted local runtime after the workflow schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5WorkflowRuntimeSummary {
    state_digest: String,
    observer_safe_state_digest: String,
    relations: Vec<Sys5WorkflowRelationSummary>,
    designated_values: Vec<Sys5WorkflowDesignatedValueSummary>,
    save_lifecycle_refs: Vec<Sys5WorkflowLifecycleRefs>,
    patch_lifecycle_refs: Vec<Sys5WorkflowLifecycleRefs>,
    relation: Sys5WorkflowRelationAlias,
    designated: Sys5WorkflowDesignatedAlias,
    cache: Sys5WorkflowCacheAlias,
}

/// Typed source-bound execution inventory for the finite workflow.  It keeps
/// the producer boundary, sealed admission identity, actual runtime causal
/// segments, and an executed unknown-action admission rejection together so
/// SYS-6 need not trust self-reported "no fixture" booleans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5SourceBoundExecutionInventory {
    input_boundary: String,
    checked_program_identity_ref: String,
    sealed_admission_attestation_ref: String,
    source_core_artifact_mapping_count: usize,
    generated_edge_count: usize,
    actual_causal_segments: Vec<Sys5WorkflowCausalDetail>,
    unknown_source_action_admission: Sys5UnknownSourceActionAdmission,
}

/// Observer-safe compact form exported to the I2 verifier.  Detailed causal
/// segments remain in the workflow inventory and are checked there; this
/// summary carries the producer-boundary and actual admission-control join
/// into the final report without duplicating source-controlled identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Sys5SourceBoundExecutionSummary {
    input_boundary: String,
    sealed_admission_attestation_ref: String,
    actual_causal_segment_count: usize,
    unknown_source_action_admission: Sys5UnknownSourceActionAdmission,
}

/// Actual fail-closed result from the generated source-action admission
/// boundary.  All references are opaque observer-safe identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5UnknownSourceActionAdmission {
    candidate_action_ref: String,
    diagnostic: String,
    rejected_before_dispatch: bool,
    semantic_state_before_ref: String,
    semantic_state_after_ref: String,
}

impl Sys5SourceBoundExecutionInventory {
    pub(crate) fn input_boundary(&self) -> &str {
        &self.input_boundary
    }

    pub(crate) fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity_ref
    }

    pub(crate) fn sealed_admission_attestation_ref(&self) -> &str {
        &self.sealed_admission_attestation_ref
    }

    pub(crate) fn actual_causal_segments(&self) -> &[Sys5WorkflowCausalDetail] {
        &self.actual_causal_segments
    }

    pub(crate) fn unknown_source_action_admission(&self) -> &Sys5UnknownSourceActionAdmission {
        &self.unknown_source_action_admission
    }

    pub(crate) fn i2_observer_safe_summary(&self) -> Sys5SourceBoundExecutionSummary {
        Sys5SourceBoundExecutionSummary {
            input_boundary: self.input_boundary.clone(),
            sealed_admission_attestation_ref: self.sealed_admission_attestation_ref.clone(),
            actual_causal_segment_count: self.actual_causal_segments.len(),
            unknown_source_action_admission: self.unknown_source_action_admission.clone(),
        }
    }
}

impl Sys5UnknownSourceActionAdmission {
    pub(crate) fn candidate_action_ref(&self) -> &str {
        &self.candidate_action_ref
    }

    pub(crate) fn diagnostic(&self) -> &str {
        &self.diagnostic
    }

    pub(crate) const fn rejected_before_dispatch(&self) -> bool {
        self.rejected_before_dispatch
    }

    pub(crate) fn semantic_state_before_ref(&self) -> &str {
        &self.semantic_state_before_ref
    }

    pub(crate) fn semantic_state_after_ref(&self) -> &str {
        &self.semantic_state_after_ref
    }
}

impl Sys5SourceBoundExecutionSummary {
    /// A marker-bearing report candidate used only to exercise SYS-6's
    /// observer renderer. It is not workflow evidence and cannot admit an
    /// action or construct a route.
    pub(crate) fn observer_policy_control(marker: String) -> Self {
        Self {
            input_boundary: "observer-policy-control".to_string(),
            sealed_admission_attestation_ref: marker.clone(),
            actual_causal_segment_count: 0,
            unknown_source_action_admission: Sys5UnknownSourceActionAdmission {
                candidate_action_ref: marker.clone(),
                diagnostic: "RouteUnavailable".to_string(),
                rejected_before_dispatch: true,
                semantic_state_before_ref: marker.clone(),
                semantic_state_after_ref: marker,
            },
        }
    }

    pub(crate) fn redact_observer_sensitive_identifiers(&mut self) {
        redact_source_bound_observer_string(&mut self.input_boundary);
        redact_source_bound_observer_string(&mut self.sealed_admission_attestation_ref);
        redact_source_bound_observer_string(
            &mut self.unknown_source_action_admission.candidate_action_ref,
        );
        redact_source_bound_observer_string(&mut self.unknown_source_action_admission.diagnostic);
        redact_source_bound_observer_string(
            &mut self
                .unknown_source_action_admission
                .semantic_state_before_ref,
        );
        redact_source_bound_observer_string(
            &mut self
                .unknown_source_action_admission
                .semantic_state_after_ref,
        );
    }
}

fn redact_source_bound_observer_string(value: &mut String) {
    let lower = value.to_ascii_lowercase();
    if [
        "credential",
        "capability_secret",
        "witness_secret",
        "token=",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        *value = "[redacted-observer-sensitive-identifier]".to_string();
    }
}

/// Deterministic observer-safe result of one actual `LocalFabric` workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5LocalWorkflowReport {
    runtime_profile: String,
    local_fabric_instance_count: usize,
    source_authority: String,
    public_api_or_wire_contract: bool,
    final_public_api_frozen: bool,
    public_wire_frozen: bool,
    runtime_reparsed_source: bool,
    #[serde(rename = "name_dispatch_used")]
    used_fixture_name_or_expected_json: bool,
    #[serde(rename = "untrusted_interface_admitted")]
    accepted_manual_route_or_interface: bool,
    accepted_runtime_core_or_authority_injection: bool,
    checked_program_identity_ref: String,
    source_core_artifact_mappings: Vec<Sys5SourceCoreArtifactMapping>,
    locus_programs: Vec<Sys5ArtifactSummary>,
    generated_communication: Vec<Sys5CommunicationSummary>,
    runtime_summary: Sys5WorkflowRuntimeSummary,
    source_bound_execution: Sys5SourceBoundExecutionInventory,
    loci: Vec<String>,
    actual_steps: Vec<Sys5LocalWorkflowStep>,
    patch_verdicts: Vec<Sys5WorkflowPatchVerdict>,
    patch_provenance: Vec<Sys5WorkflowPatchProvenance>,
    presentation_gap_results: Vec<Sys5PresentationGapEvidence>,
    participant_leave_results: Vec<Sys5ParticipantLeaveEvidence>,
    relation_degradation_results: Vec<Sys5RelationDegradationEvidence>,
    participant_leave_failures: Vec<Sys5ParticipantLeaveFailureEvidence>,
    fresh_reacquire_results: Vec<Sys5FreshReacquireEvidence>,
    typed_failures: Vec<Sys5WorkflowTypedFailure>,
    joined_rows: Vec<Sys5WorkflowJoinedRow>,
}

impl Sys5LocalWorkflowReport {
    pub fn runtime_profile(&self) -> &str {
        &self.runtime_profile
    }

    pub const fn local_fabric_instance_count(&self) -> usize {
        self.local_fabric_instance_count
    }

    pub fn source_authority(&self) -> &str {
        &self.source_authority
    }

    pub const fn public_api_or_wire_contract(&self) -> bool {
        self.public_api_or_wire_contract
    }

    pub const fn final_public_api_frozen(&self) -> bool {
        self.final_public_api_frozen
    }

    pub const fn public_wire_frozen(&self) -> bool {
        self.public_wire_frozen
    }

    pub const fn runtime_reparsed_source(&self) -> bool {
        self.runtime_reparsed_source
    }

    pub const fn used_fixture_name_or_expected_json(&self) -> bool {
        self.used_fixture_name_or_expected_json
    }

    pub const fn accepted_manual_route_or_interface(&self) -> bool {
        self.accepted_manual_route_or_interface
    }

    pub const fn accepted_runtime_core_or_authority_injection(&self) -> bool {
        self.accepted_runtime_core_or_authority_injection
    }

    pub fn has_step(&self, step: Sys5LocalWorkflowStep) -> bool {
        self.actual_steps.contains(&step)
    }

    pub fn patch_verdict(&self, patch_id: &str) -> Option<Sys5PatchVerdict> {
        self.patch_verdicts
            .iter()
            .find(|entry| entry.patch_id == patch_id)
            .and_then(|entry| match entry.verdict.as_str() {
                "accepted" => Some(Sys5PatchVerdict::Accepted),
                "rejected" => Some(Sys5PatchVerdict::Rejected),
                _ => None,
            })
    }

    pub fn patch_diagnostic(&self, patch_id: &str) -> Option<Sys5PatchDiagnosticKind> {
        self.patch_verdicts
            .iter()
            .find(|entry| entry.patch_id == patch_id)
            .and_then(|entry| entry.diagnostic.as_deref())
            .and_then(patch_diagnostic_from_name)
    }

    pub fn has_typed_failure(&self, diagnostic: Sys5VerticalDiagnosticKind) -> bool {
        self.typed_failures
            .iter()
            .any(|entry| entry.diagnostic == vertical_diagnostic_name(diagnostic))
    }

    pub fn failure_rejected_before_state_mutation(
        &self,
        diagnostic: Sys5VerticalDiagnosticKind,
    ) -> bool {
        self.typed_failures.iter().any(|entry| {
            entry.diagnostic == vertical_diagnostic_name(diagnostic)
                && entry.rejected_before_cache_or_state_mutation
        })
    }

    pub fn has_joined_row_kind(&self, kind: &str) -> bool {
        self.joined_rows.iter().any(|row| row.kind == kind)
    }

    /// A deterministic compact observer view. JSON serialization preserves
    /// the profile's ordered schedule and joined evidence rows.
    pub fn render_compact(&self) -> String {
        serde_json::to_string(self).expect("SYS-5 workflow report is serializable")
    }

    pub fn observer_safe_digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(WORKFLOW_DIGEST_DOMAIN);
        hasher.update(self.render_compact().as_bytes());
        format!("sys5-local-workflow-sha256-v1:{:x}", hasher.finalize())
    }

    pub fn loci(&self) -> &[String] {
        &self.loci
    }

    pub fn actual_steps(&self) -> &[Sys5LocalWorkflowStep] {
        &self.actual_steps
    }

    pub fn patch_verdicts(&self) -> &[Sys5WorkflowPatchVerdict] {
        &self.patch_verdicts
    }

    /// Typed projection evidence retained from the checked Core and SYS-3
    /// projector. Consumers use these fields directly rather than reparsing
    /// this report's JSON representation.
    pub fn source_core_artifact_mappings(&self) -> &[Sys5SourceCoreArtifactMapping] {
        &self.source_core_artifact_mappings
    }

    pub fn locus_programs(&self) -> &[Sys5ArtifactSummary] {
        &self.locus_programs
    }

    pub fn generated_communication(&self) -> &[Sys5CommunicationSummary] {
        &self.generated_communication
    }

    /// The actual source/Core/artifact-bound runtime inventory retained by
    /// this workflow. It is the SYS-6 source-first evidence boundary.
    pub fn source_bound_execution(&self) -> &Sys5SourceBoundExecutionInventory {
        &self.source_bound_execution
    }

    pub fn typed_failures(&self) -> &[Sys5WorkflowTypedFailure] {
        &self.typed_failures
    }

    pub fn joined_rows(&self) -> &[Sys5WorkflowJoinedRow] {
        &self.joined_rows
    }

    /// Narrow typed invariant summaries consumed by SYS-6. They inspect the
    /// actual SYS-5 receipts retained above; no JSON report is reparsed and
    /// no source/schedule input is introduced at this boundary.
    pub(crate) fn has_relation_fallback_invariants_for_i2(&self) -> bool {
        self.participant_leave_results
            .iter()
            .any(Sys5ParticipantLeaveEvidence::satisfies_i2_relation_leave)
            && self
                .fresh_reacquire_results
                .iter()
                .any(Sys5FreshReacquireEvidence::satisfies_i2_fresh_relation_reacquire)
    }

    pub(crate) fn has_presentation_gap_invariants_for_i2(&self) -> bool {
        self.presentation_gap_results
            .iter()
            .any(Sys5PresentationGapEvidence::satisfies_i2_semantic_presentation_separation)
    }

    pub(crate) fn has_designated_result_invariants_for_i2(&self) -> bool {
        self.runtime_summary.designated_values.iter().all(|value| {
            value.result_version > 0
                && !value.cache_ref.is_empty()
                && !value.version_ref.is_empty()
                && !value.delivery_ref.is_empty()
                && !value.cache_binding_ref.is_empty()
        }) && !self.runtime_summary.designated_values.is_empty()
            && self.runtime_summary.designated.version > 0
            && !self.runtime_summary.designated.result_ref.is_empty()
            && !self.runtime_summary.cache.version_ref.is_empty()
    }

    pub(crate) fn has_local_cut_invariants_for_i2(&self) -> bool {
        self.runtime_summary
            .save_lifecycle_refs
            .iter()
            .all(|entry| {
                !entry.kind.is_empty()
                    && !entry.occurrence_ref.is_empty()
                    && !entry.before_frontier_ref.is_empty()
                    && !entry.after_frontier_ref.is_empty()
            })
            && !self.runtime_summary.save_lifecycle_refs.is_empty()
    }

    pub(crate) fn save_lifecycle_refs_for_i2(&self) -> &[Sys5WorkflowLifecycleRefs] {
        &self.runtime_summary.save_lifecycle_refs
    }

    pub(crate) fn patch_lifecycle_refs_for_i2(&self) -> &[Sys5WorkflowLifecycleRefs] {
        &self.runtime_summary.patch_lifecycle_refs
    }

    pub(crate) fn has_patch_lifecycle_invariants_for_i2(&self) -> bool {
        self.patch_verdicts
            .iter()
            .any(|verdict| verdict.verdict == "accepted")
            && self
                .patch_verdicts
                .iter()
                .any(|verdict| verdict.verdict == "rejected")
            && self.patch_provenance.iter().all(|provenance| {
                !provenance.checked_program_identity_ref.is_empty()
                    && !provenance.patch_ref.is_empty()
                    && !provenance.patch_occurrence_ref.is_empty()
            })
            && !self.runtime_summary.patch_lifecycle_refs.is_empty()
    }

    pub(crate) fn has_duplicate_leave_fail_closed_for_i2(&self) -> bool {
        self.participant_leave_failures
            .iter()
            .any(Sys5ParticipantLeaveFailureEvidence::satisfies_i2_duplicate_leave_fail_closed)
    }

    /// Verify the typed execution inventory without reparsing serialized JSON
    /// or trusting a "fixture lookup was not used" declaration. Every actual
    /// causal segment must join a checked Core ref, projected fragment pair,
    /// generated communication edge, and nonempty runtime occurrences. The
    /// producer input is exclusively a checked project plus sealed admission;
    /// an unknown ordinary action was actually rejected before dispatch.
    pub(crate) fn has_source_bound_execution_for_i2(&self) -> bool {
        let inventory = &self.source_bound_execution;
        if inventory.input_boundary != "checked_project_and_sealed_admission"
            || inventory.checked_program_identity_ref != self.checked_program_identity_ref
            || inventory.sealed_admission_attestation_ref.is_empty()
            || inventory.source_core_artifact_mapping_count
                != self.source_core_artifact_mappings.len()
            || inventory.generated_edge_count != self.generated_communication.len()
            || inventory.actual_causal_segments.is_empty()
            || !inventory
                .unknown_source_action_admission
                .rejected_before_dispatch
            || inventory.unknown_source_action_admission.diagnostic != "UnknownSourceAction"
            || inventory
                .unknown_source_action_admission
                .candidate_action_ref
                .is_empty()
            || inventory
                .unknown_source_action_admission
                .semantic_state_before_ref
                .is_empty()
            || inventory
                .unknown_source_action_admission
                .semantic_state_before_ref
                != inventory
                    .unknown_source_action_admission
                    .semantic_state_after_ref
        {
            return false;
        }

        let core_refs = self
            .source_core_artifact_mappings
            .iter()
            .map(|mapping| mapping.core_ref.as_str())
            .chain(
                self.generated_communication
                    .iter()
                    .filter_map(|edge| edge.core_ref.as_deref()),
            )
            .collect::<BTreeSet<_>>();
        let fragment_refs = self
            .locus_programs
            .iter()
            .map(|artifact| artifact.fragment_ref.as_str())
            .collect::<BTreeSet<_>>();
        let edge_refs = self
            .generated_communication
            .iter()
            .map(|edge| edge.edge_ref.as_str())
            .collect::<BTreeSet<_>>();
        inventory.actual_causal_segments.iter().all(|segment| {
            !segment.core_ref().is_empty()
                && core_refs.contains(segment.core_ref())
                && !segment.request_fragment_ref().is_empty()
                && fragment_refs.contains(segment.request_fragment_ref())
                && !segment.serve_fragment_ref().is_empty()
                && fragment_refs.contains(segment.serve_fragment_ref())
                && !segment.edge_ref().is_empty()
                && edge_refs.contains(segment.edge_ref())
                && !segment.request_identity().is_empty()
                && segment
                    .request_enqueue_occurrence_ref()
                    .is_some_and(|value| !value.is_empty())
                && segment
                    .dispatch_occurrence_ref()
                    .is_some_and(|value| !value.is_empty())
                && segment
                    .receive_occurrence_ref()
                    .is_some_and(|value| !value.is_empty())
                && !segment.serve_occurrence_ref().is_empty()
        })
    }
}

/// Run the fixed SYS-5 local scenario from checked/projected input only.
///
/// No ordinary source is parsed here.  The post-cut admission is rebuilt from
/// the retained checked project only, so the restore does not receive a raw
/// authority or source input either.
pub fn run_local_workflow_from_project(
    input: Sys5LocalWorkflowInput,
) -> Result<Sys5LocalWorkflowReport, Sys5LocalWorkflowError> {
    let Sys5LocalWorkflowInput {
        project,
        admission,
        patches,
    } = input;
    let semantic = project.semantic_summary().clone();
    let checked_program_identity_ref = project.checked_program_identity_ref().to_string();
    let loci = semantic.loci.clone();
    let restore_admission = admission.clone_for_local_restore();
    let mut runtime = admission
        .start_vertical_slice_runtime()
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    let unknown_source_action_admission = runtime.reject_unknown_source_action_for_i2();
    let mut actual_steps = vec![Sys5LocalWorkflowStep::Startup];

    runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    actual_steps.push(Sys5LocalWorkflowStep::Attack);

    runtime
        .dispatch(Sys5VerticalAction::world_tick("workflow-tick-0"))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    actual_steps.push(Sys5LocalWorkflowStep::DesignatedPublish);
    runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    actual_steps.push(Sys5LocalWorkflowStep::ViewerConsume);

    let relation = canonical_relation_id(&semantic)?;
    runtime
        .dispatch(Sys5VerticalAction::publish_relation(&relation))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    actual_steps.push(Sys5LocalWorkflowStep::RelationPrimary);

    let cut = runtime
        .save_local_cut("workflow-cut-0")
        .map_err(Sys5LocalWorkflowError::from_cut_patch)?;
    actual_steps.push(Sys5LocalWorkflowStep::Save);
    // This is deliberately a real post-cut operation. Restoring the saved
    // cut below proves the prefix boundary rather than treating cut as a
    // report-only snapshot.
    let post_cut_row_start = runtime.observer_safe_joined_report().ordered_rows().len();
    runtime
        .dispatch(Sys5VerticalAction::world_tick("workflow-post-cut"))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    let post_cut_rows =
        runtime.observer_safe_joined_report().ordered_rows()[post_cut_row_start..].to_vec();
    runtime = restore_admission
        .restore_vertical_slice_runtime(&cut)
        .map_err(Sys5LocalWorkflowError::from_cut_patch)?;
    actual_steps.push(Sys5LocalWorkflowStep::Restore);

    let mut patch_verdicts = Vec::new();
    let mut patch_provenance = Vec::new();
    for patch in patches {
        if !is_safe_patch_logical_label(&patch.logical_path) {
            return Err(Sys5LocalWorkflowError::UnsafePatchLogicalProvenance);
        }
        let patch_checked_program_identity_ref =
            patch.project.checked_program_identity_ref().to_string();
        let logical_path = patch.logical_path.clone();
        let patch_ref = workflow_ref(&format!(
            "{}:{}",
            logical_path, patch_checked_program_identity_ref
        ));
        let candidate =
            crate::sys5_local_slice::Sys5LocalPatchCandidate::from_source_project_and_admission(
                patch.patch_id.clone(),
                &runtime,
                patch.project,
                patch.admission,
            )
            .map_err(Sys5LocalWorkflowError::from_cut_patch)?;
        let outcome = runtime
            .activate_source_first_patch(candidate)
            .map_err(Sys5LocalWorkflowError::from_cut_patch)?;
        let patch_occurrence_ref = outcome.patch_occurrence_ref().to_string();
        let diagnostic = outcome.primary_diagnostic_kind().map(patch_diagnostic_name);
        let verdict = match outcome.verdict() {
            Sys5PatchVerdict::Accepted => {
                actual_steps.push(Sys5LocalWorkflowStep::PatchAccepted);
                "accepted"
            }
            Sys5PatchVerdict::Rejected => {
                actual_steps.push(Sys5LocalWorkflowStep::PatchRejected);
                "rejected"
            }
        };
        patch_verdicts.push(Sys5WorkflowPatchVerdict {
            patch_id: patch.patch_id.clone(),
            patch_ref: patch_ref.clone(),
            verdict: verdict.to_string(),
            diagnostic: diagnostic.clone(),
        });
        patch_provenance.push(Sys5WorkflowPatchProvenance {
            patch_id: patch.patch_id,
            logical_path,
            checked_program_identity_ref: patch_checked_program_identity_ref,
            patch_ref,
            patch_occurrence_ref,
            verdict: verdict.to_string(),
            diagnostic,
        });
    }

    // Patch activation is intentionally closed over the restored initial M9
    // lineage. The later leave/fresh-reacquire sequence advances membership
    // lineage and is not claimed to commute with patch activation in this
    // finite SYS-5 trace.
    let leave_receipt = runtime
        .dispatch(Sys5VerticalAction::participant_a_leave_relation_primary(
            &relation,
        ))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    let participant_leave_results = vec![
        leave_receipt
            .participant_leave_evidence()
            .cloned()
            .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?,
    ];
    let relation_degradation_results =
        vec![participant_leave_results[0].relation_degradation().clone()];
    actual_steps.push(Sys5LocalWorkflowStep::ParticipantALeave);
    let duplicate_leave = runtime
        .dispatch(Sys5VerticalAction::participant_a_leave_relation_primary(
            &relation,
        ))
        .expect_err("the same source-bound participant leave must fail closed");
    if duplicate_leave.kind() != Sys5VerticalDiagnosticKind::DuplicateParticipantLeave {
        return Err(Sys5LocalWorkflowError::from_vertical(duplicate_leave));
    }
    let participant_leave_failures = vec![
        runtime
            .last_participant_leave_failure()
            .cloned()
            .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?,
    ];
    actual_steps.push(Sys5LocalWorkflowStep::DuplicateParticipantLeave);
    let presentation_gap_receipt = runtime
        .dispatch(Sys5VerticalAction::viewer_c_presentation_gap(&relation))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    let presentation_gap_results = vec![
        presentation_gap_receipt
            .presentation_gap_evidence()
            .cloned()
            .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?,
    ];
    actual_steps.push(Sys5LocalWorkflowStep::PresentationGap);
    let fresh_reacquire_receipt = runtime
        .dispatch(Sys5VerticalAction::fresh_reacquire_relation_primary(
            &relation,
        ))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    let fresh_reacquire_results = vec![
        fresh_reacquire_receipt
            .fresh_reacquire_evidence()
            .cloned()
            .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?,
    ];
    actual_steps.push(Sys5LocalWorkflowStep::FreshReacquire);

    let value_name = canonical_designated_value(&semantic)?;
    // A checked designated-only patch may replace the result frontier. Publish
    // through the new active artifact before revoking the consumer, so the
    // final failure demonstrates authority containment rather than a missing
    // pre-patch cache entry.
    let final_publish = runtime
        .dispatch(Sys5VerticalAction::world_tick("workflow-post-patch"))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    runtime
        .dispatch(Sys5VerticalAction::revoke_viewer_c_consumer_capability(
            &value_name,
        ))
        .map_err(Sys5LocalWorkflowError::from_vertical)?;
    actual_steps.push(Sys5LocalWorkflowStep::ConsumerCapabilityRevoke);
    let failure = runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect_err("revoked consumer capability must fail closed");
    let typed_failures = vec![Sys5WorkflowTypedFailure {
        diagnostic: vertical_diagnostic_name(failure.kind()).to_string(),
        rejected_before_cache_or_state_mutation: failure
            .rejected_before_m8_cache_or_state_mutation(),
    }];
    actual_steps.push(Sys5LocalWorkflowStep::FailedConsume);
    if !runtime
        .observer_safe_runtime_snapshot()
        .verification_summary()
        .is_discharged("finite_refinement")
    {
        return Err(Sys5LocalWorkflowError::VerificationNotDischarged);
    }
    actual_steps.push(Sys5LocalWorkflowStep::Verification);

    let runtime_summary =
        runtime_summary_from_actual(&runtime, &relation, &value_name, &final_publish)?;
    let joined_rows = joined_rows_from_actual(&semantic, &runtime, &typed_failures, &post_cut_rows);
    let source_bound_execution = source_bound_execution_from_actual(
        &semantic,
        &checked_program_identity_ref,
        runtime.sealed_admission_attestation_ref(),
        &joined_rows,
        unknown_source_action_admission,
    );
    Ok(Sys5LocalWorkflowReport {
        runtime_profile: "ST".to_string(),
        local_fabric_instance_count: runtime.local_fabric_instance_count(),
        source_authority: "ordinary_mir_source".to_string(),
        public_api_or_wire_contract: false,
        final_public_api_frozen: false,
        public_wire_frozen: false,
        runtime_reparsed_source: false,
        used_fixture_name_or_expected_json: false,
        accepted_manual_route_or_interface: false,
        accepted_runtime_core_or_authority_injection: false,
        checked_program_identity_ref,
        source_core_artifact_mappings: semantic.source_core_artifact_mappings.clone(),
        locus_programs: semantic.artifacts.clone(),
        generated_communication: semantic.generated_communication.clone(),
        runtime_summary,
        source_bound_execution,
        loci,
        actual_steps,
        patch_verdicts,
        patch_provenance,
        presentation_gap_results,
        participant_leave_results,
        relation_degradation_results,
        participant_leave_failures,
        fresh_reacquire_results,
        typed_failures,
        joined_rows,
    })
}

/// Typed workflow rejection without source text, host path, raw authority, or
/// credential material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sys5LocalWorkflowError {
    Vertical(String),
    CutPatch(String),
    Admission(String),
    MissingCanonicalRelation,
    MissingCanonicalDesignatedValue,
    MissingObserverSafeRuntimeEvidence,
    UnsafePatchLogicalProvenance,
    VerificationNotDischarged,
}

impl Sys5LocalWorkflowError {
    fn from_vertical(error: Sys5VerticalSliceError) -> Self {
        Self::Vertical(vertical_diagnostic_name(error.kind()).to_string())
    }

    fn from_cut_patch(error: Sys5LocalCutPatchError) -> Self {
        Self::CutPatch(format!("{:?}", error.kind()))
    }
}

impl std::fmt::Display for Sys5LocalWorkflowError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "SYS-5 local workflow rejected: {self:?}")
    }
}

impl std::error::Error for Sys5LocalWorkflowError {}

fn is_safe_patch_logical_label(label: &str) -> bool {
    const CREDENTIAL_LIKE_FRAGMENTS: &[&str] = &[
        "secret",
        "token",
        "credential",
        "capability",
        "witness",
        "password",
    ];
    let lower = label.to_ascii_lowercase();
    !label.is_empty()
        && label.len() <= 96
        && label.ends_with(".mir")
        && !label.contains("..")
        && label
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
        && !CREDENTIAL_LIKE_FRAGMENTS
            .iter()
            .any(|fragment| lower.contains(fragment))
}

fn canonical_relation_id(
    semantic: &crate::sys5_local_slice::Sys5SemanticSummary,
) -> Result<String, Sys5LocalWorkflowError> {
    let relations = semantic
        .generated_communication
        .iter()
        .filter(|edge| edge.kind == "relation-projection-publication")
        .map(|edge| edge.operation_id.clone())
        .collect::<BTreeSet<_>>();
    (relations.len() == 1)
        .then(|| relations.into_iter().next().expect("one relation"))
        .ok_or(Sys5LocalWorkflowError::MissingCanonicalRelation)
}

fn canonical_designated_value(
    semantic: &crate::sys5_local_slice::Sys5SemanticSummary,
) -> Result<String, Sys5LocalWorkflowError> {
    let values = semantic
        .generated_communication
        .iter()
        .filter(|edge| edge.kind == "designated-result-delivery")
        .map(|edge| edge.operation_id.clone())
        .collect::<BTreeSet<_>>();
    (values.len() == 1)
        .then(|| values.into_iter().next().expect("one designated value"))
        .ok_or(Sys5LocalWorkflowError::MissingCanonicalDesignatedValue)
}

fn source_bound_execution_from_actual(
    semantic: &crate::sys5_local_slice::Sys5SemanticSummary,
    checked_program_identity_ref: &str,
    sealed_admission_attestation_ref: &str,
    joined_rows: &[Sys5WorkflowJoinedRow],
    unknown_source_action_admission: crate::sys5_local_slice::Sys5SourceActionAdmissionControl,
) -> Sys5SourceBoundExecutionInventory {
    let actual_causal_segments = joined_rows
        .iter()
        .filter_map(Sys5WorkflowJoinedRow::detail)
        .cloned()
        .collect::<Vec<_>>();
    Sys5SourceBoundExecutionInventory {
        input_boundary: "checked_project_and_sealed_admission".to_string(),
        checked_program_identity_ref: checked_program_identity_ref.to_string(),
        sealed_admission_attestation_ref: sealed_admission_attestation_ref.to_string(),
        source_core_artifact_mapping_count: semantic.source_core_artifact_mappings.len(),
        generated_edge_count: semantic.generated_communication.len(),
        actual_causal_segments,
        unknown_source_action_admission: Sys5UnknownSourceActionAdmission {
            candidate_action_ref: unknown_source_action_admission
                .candidate_action_ref()
                .to_string(),
            diagnostic: unknown_source_action_admission.diagnostic().to_string(),
            rejected_before_dispatch: unknown_source_action_admission.rejected_before_dispatch(),
            semantic_state_before_ref: unknown_source_action_admission
                .semantic_state_before_ref()
                .to_string(),
            semantic_state_after_ref: unknown_source_action_admission
                .semantic_state_after_ref()
                .to_string(),
        },
    }
}

fn joined_rows_from_actual(
    semantic: &crate::sys5_local_slice::Sys5SemanticSummary,
    runtime: &Sys5VerticalSliceRuntime,
    typed_failures: &[Sys5WorkflowTypedFailure],
    post_cut_rows: &[String],
) -> Vec<Sys5WorkflowJoinedRow> {
    let mut rows = Vec::new();
    for mapping in &semantic.source_core_artifact_mappings {
        rows.push(joined("source_span", &mapping.source_path));
        rows.push(joined("core_operation", &mapping.core_ref));
        rows.push(joined("per_locus_artifact", &mapping.fragment_ref));
    }
    for edge in &semantic.generated_communication {
        rows.push(joined("generated_communication_edge", &edge.edge_ref));
    }
    let mut post_cut_inserted = false;
    let mut active_restored = false;
    for row in runtime.observer_safe_joined_report().ordered_rows() {
        if !post_cut_inserted && row.starts_with("lifecycle:RestoreCut") {
            append_actual_runtime_rows(
                &mut rows,
                post_cut_rows,
                Sys5WorkflowExecutionBranch::DiscardedPostCut,
            );
            post_cut_inserted = true;
            active_restored = true;
        }
        append_actual_runtime_row(
            &mut rows,
            row,
            if active_restored {
                Sys5WorkflowExecutionBranch::ActiveRestored
            } else {
                Sys5WorkflowExecutionBranch::ActivePrefix
            },
        );
    }
    if !post_cut_inserted {
        append_actual_runtime_rows(
            &mut rows,
            post_cut_rows,
            Sys5WorkflowExecutionBranch::DiscardedPostCut,
        );
    }
    for failure in typed_failures {
        rows.push(joined_with_branch(
            "authority_failure",
            &failure.diagnostic,
            Sys5WorkflowExecutionBranch::ActiveRestored,
        ));
    }
    rows
}

fn append_actual_runtime_rows(
    rows: &mut Vec<Sys5WorkflowJoinedRow>,
    source_rows: &[String],
    execution_branch: Sys5WorkflowExecutionBranch,
) {
    for row in source_rows {
        append_actual_runtime_row(rows, row, execution_branch);
    }
}

fn append_actual_runtime_row(
    rows: &mut Vec<Sys5WorkflowJoinedRow>,
    row: &str,
    execution_branch: Sys5WorkflowExecutionBranch,
) {
    if let Some(detail) = parse_typed_causal_segment(row) {
        rows.push(Sys5WorkflowJoinedRow {
            kind: "typed_causal_segment".to_string(),
            detail_ref: row.to_string(),
            execution_branch: Some(execution_branch),
            detail: Some(detail),
        });
        return;
    }
    if row.starts_with("typed-presentation-gap:") {
        rows.push(joined_with_branch(
            "presentation_gap",
            row,
            execution_branch,
        ));
        return;
    }
    if row.starts_with("typed-participant-leave:") {
        rows.push(joined_with_branch(
            "participant_leave",
            row,
            execution_branch,
        ));
        return;
    }
    if row.starts_with("typed-participant-leave-failure:") {
        rows.push(joined_with_branch(
            "participant_leave_failure",
            row,
            execution_branch,
        ));
        return;
    }
    if row.starts_with("typed-participant-fresh-reacquire:") {
        rows.push(joined_with_branch(
            "participant_fresh_reacquire",
            row,
            execution_branch,
        ));
        return;
    }
    let kind = if row.starts_with("request:")
        || row.starts_with("request-enqueue:")
        || row.starts_with("dispatch:")
        || row.starts_with("receive:")
        || row.starts_with("serve:")
    {
        "runtime_occurrence"
    } else if row.starts_with("owner-mutation:") {
        "owner_state_mutation"
    } else if row.starts_with("failure:") || row.starts_with("auth:") {
        "authority_failure"
    } else if row.starts_with("designated:") {
        "designated_result_version"
    } else if row.starts_with("lifecycle:SaveCut") {
        "save_cut"
    } else if row.starts_with("lifecycle:RestoreCut") {
        "restore_cut"
    } else if row.starts_with("lifecycle:Patch") {
        "patch_lifecycle"
    } else if row.starts_with("verification:") {
        "verification_residual"
    } else if row.starts_with("presentation-gap:") {
        "presentation_gap"
    } else if row.starts_with("relation-selected:") {
        "relation_selected_fallback"
    } else {
        return;
    };
    rows.push(joined_with_branch(kind, row, execution_branch));
}

fn runtime_summary_from_actual(
    runtime: &Sys5VerticalSliceRuntime,
    relation: &str,
    value_name: &str,
    final_publish: &Sys5VerticalReceipt,
) -> Result<Sys5WorkflowRuntimeSummary, Sys5LocalWorkflowError> {
    let relation_shadow = runtime
        .observer_relation_shadow("ViewerC", relation)
        .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?;
    let latest_value = final_publish
        .typed_int()
        .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?;
    let result_version = final_publish
        .designated_result_version()
        .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?;
    let delivery_ref = final_publish
        .designated_delivery_ref()
        .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?
        .to_string();
    let cache_binding_ref = final_publish
        .designated_cache_binding_ref()
        .ok_or(Sys5LocalWorkflowError::MissingObserverSafeRuntimeEvidence)?
        .to_string();
    let state_digest = runtime.observer_safe_state_digest();
    let relation_summary = Sys5WorkflowRelationSummary {
        relation: relation_shadow.relation().to_string(),
        selected_anchor: relation_shadow.selected_anchor().to_string(),
        selected_floor: relation_shadow.selected_floor().to_string(),
        semantic_digest: relation_shadow.semantic_digest().to_string(),
        lineage_ref: relation_shadow.lineage_ref().to_string(),
    };
    let version_ref = workflow_ref(&format!(
        "designated-result-version:{}:{}",
        value_name, result_version
    ));
    let designated_summary = Sys5WorkflowDesignatedValueSummary {
        value_name: value_name.to_string(),
        cache_ref: runtime.designated_cache_digest(value_name, "ViewerC"),
        version_ref: version_ref.clone(),
        result_version,
        delivery_ref: delivery_ref.clone(),
        cache_binding_ref: cache_binding_ref.clone(),
        latest_value,
    };
    let (save_lifecycle_refs, patch_lifecycle_refs) =
        lifecycle_refs_from_actual(runtime.observer_safe_joined_report().ordered_rows());
    Ok(Sys5WorkflowRuntimeSummary {
        state_digest: state_digest.clone(),
        observer_safe_state_digest: state_digest,
        relations: vec![relation_summary.clone()],
        designated_values: vec![designated_summary.clone()],
        save_lifecycle_refs,
        patch_lifecycle_refs,
        relation: Sys5WorkflowRelationAlias {
            selected_anchor_ref: workflow_ref(&format!(
                "relation-selected-anchor:{}:{}",
                relation_summary.relation, relation_summary.selected_anchor
            )),
            floor_ref: workflow_ref(&format!(
                "relation-selected-floor:{}:{}",
                relation_summary.relation, relation_summary.selected_floor
            )),
            semantic_ref: relation_summary.semantic_digest,
            lineage_ref: relation_summary.lineage_ref,
        },
        designated: Sys5WorkflowDesignatedAlias {
            result_ref: delivery_ref,
            version: designated_summary.result_version,
        },
        cache: Sys5WorkflowCacheAlias {
            version_ref: cache_binding_ref,
        },
    })
}

fn lifecycle_refs_from_actual(
    rows: &[String],
) -> (
    Vec<Sys5WorkflowLifecycleRefs>,
    Vec<Sys5WorkflowLifecycleRefs>,
) {
    let mut save_refs = Vec::new();
    let mut patch_refs = Vec::new();
    for row in rows {
        let Some(parsed) = parse_lifecycle_refs(row) else {
            continue;
        };
        match parsed.kind.as_str() {
            "SaveCut" | "RestoreCut" => save_refs.push(parsed),
            "PatchAccepted" | "PatchRejected" => patch_refs.push(parsed),
            _ => {}
        }
    }
    (save_refs, patch_refs)
}

fn parse_lifecycle_refs(row: &str) -> Option<Sys5WorkflowLifecycleRefs> {
    let tail = row.strip_prefix("lifecycle:")?;
    let (kind, fields) = tail.split_once(':')?;
    let fields = observer_row_fields(fields);
    let occurrence_ref = fields
        .get("cut_occurrence_ref")
        .or_else(|| fields.get("restore_occurrence_ref"))
        .or_else(|| fields.get("patch_occurrence_ref"))?
        .to_string();
    Some(Sys5WorkflowLifecycleRefs {
        kind: kind.to_string(),
        occurrence_ref,
        before_core_ref: fields.get("before_core_ref")?.to_string(),
        after_core_ref: fields.get("after_core_ref")?.to_string(),
        before_artifact_ref: fields.get("before_artifact_ref")?.to_string(),
        after_artifact_ref: fields.get("after_artifact_ref")?.to_string(),
        before_frontier_ref: fields.get("before_activation_frontier")?.to_string(),
        after_frontier_ref: fields.get("after_activation_frontier")?.to_string(),
    })
}

fn parse_typed_causal_segment(row: &str) -> Option<Sys5WorkflowCausalDetail> {
    let tail = row.strip_prefix("typed-segment:")?;
    let (segment_kind, fields) = tail.split_once(':')?;
    let fields = observer_row_fields(fields);
    let source_span = parse_source_span(fields.get("source_span")?)?;
    Some(Sys5WorkflowCausalDetail {
        segment_kind: segment_kind.to_string(),
        logical_path: fields.get("logical_path")?.to_string(),
        source_span,
        core_ref: fields.get("core_ref")?.to_string(),
        request_fragment_ref: fields.get("source_fragment_ref")?.to_string(),
        serve_fragment_ref: fields.get("target_fragment_ref")?.to_string(),
        edge_ref: fields.get("edge_ref")?.to_string(),
        request_identity: fields.get("request_identity")?.to_string(),
        owner_publish_occurrence_ref: fields
            .get("owner_publish_occurrence_id")
            .map(|value| (*value).to_string()),
        request_enqueue_occurrence_ref: fields
            .get("request_enqueue_occurrence_id")
            .map(|value| (*value).to_string()),
        dispatch_occurrence_ref: fields
            .get("dispatch_occurrence_id")
            .map(|value| (*value).to_string()),
        receive_occurrence_ref: fields
            .get("receive_occurrence_id")
            .map(|value| (*value).to_string()),
        observe_occurrence_ref: fields
            .get("consumer_observe_occurrence_id")
            .map(|value| (*value).to_string()),
        serve_occurrence_ref: fields.get("serve_occurrence_id")?.to_string(),
    })
}

fn observer_row_fields(row: &str) -> BTreeMap<&str, &str> {
    row.split(';')
        .filter_map(|field| field.split_once('='))
        .collect()
}

fn parse_source_span(value: &str) -> Option<Sys5WorkflowSourceSpanDetail> {
    let (start, end) = value.split_once('-')?;
    let (start_line, start_column) = parse_source_position(start)?;
    let (end_line, end_column) = parse_source_position(end)?;
    let start = encode_source_position(start_line, start_column)?;
    let end = encode_source_position(end_line, end_column)?;
    (end > start).then_some(Sys5WorkflowSourceSpanDetail {
        start,
        end,
        start_line,
        start_column,
        end_line,
        end_column,
    })
}

fn parse_source_position(value: &str) -> Option<(u32, u32)> {
    let (line, column) = value.split_once(':')?;
    Some((line.parse().ok()?, column.parse().ok()?))
}

fn encode_source_position(line: u32, column: u32) -> Option<u64> {
    let encoded = u64::from(line)
        .checked_mul(1_000_000)?
        .checked_add(u64::from(column))?;
    (encoded > 0).then_some(encoded)
}

fn joined(kind: &str, detail: &str) -> Sys5WorkflowJoinedRow {
    Sys5WorkflowJoinedRow {
        kind: kind.to_string(),
        // Every input reaches this boundary through the checked projection or
        // the observer-safe vertical report. Keep its exact opaque reference
        // so one devtools view can follow the source/Core/artifact/occurrence
        // line without a second hand-joined lookup file.
        detail_ref: detail.to_string(),
        execution_branch: None,
        detail: None,
    }
}

fn joined_with_branch(
    kind: &str,
    detail: &str,
    execution_branch: Sys5WorkflowExecutionBranch,
) -> Sys5WorkflowJoinedRow {
    Sys5WorkflowJoinedRow {
        kind: kind.to_string(),
        detail_ref: detail.to_string(),
        execution_branch: Some(execution_branch),
        detail: None,
    }
}

fn workflow_ref(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(WORKFLOW_DIGEST_DOMAIN);
    hasher.update(value.as_bytes());
    format!("sys5-workflow-ref-v1:{:x}", hasher.finalize())
}

fn patch_diagnostic_name(kind: Sys5PatchDiagnosticKind) -> String {
    format!("{kind:?}")
}

fn patch_diagnostic_from_name(name: &str) -> Option<Sys5PatchDiagnosticKind> {
    match name {
        "StaleFrontier" => Some(Sys5PatchDiagnosticKind::StaleFrontier),
        "NonQuiescentPendingCarrier" => Some(Sys5PatchDiagnosticKind::NonQuiescentPendingCarrier),
        "TopologyOwnerRouteMismatch" => Some(Sys5PatchDiagnosticKind::TopologyOwnerRouteMismatch),
        "OwnerRmwExpressionChanged" => Some(Sys5PatchDiagnosticKind::OwnerRmwExpressionChanged),
        "NonDesignatedCoreMaterialChanged" => {
            Some(Sys5PatchDiagnosticKind::NonDesignatedCoreMaterialChanged)
        }
        "M9AuthorityLineageMismatch" => Some(Sys5PatchDiagnosticKind::M9AuthorityLineageMismatch),
        "IncompleteCandidateAdmission" => {
            Some(Sys5PatchDiagnosticKind::IncompleteCandidateAdmission)
        }
        "BackendIneligible" => Some(Sys5PatchDiagnosticKind::BackendIneligible),
        _ => None,
    }
}

fn vertical_diagnostic_name(kind: Sys5VerticalDiagnosticKind) -> &'static str {
    match kind {
        Sys5VerticalDiagnosticKind::UnknownSourceOperation => "UnknownSourceOperation",
        Sys5VerticalDiagnosticKind::UnknownSourceValue => "UnknownSourceValue",
        Sys5VerticalDiagnosticKind::MissingPublishedDesignatedValue => {
            "MissingPublishedDesignatedValue"
        }
        Sys5VerticalDiagnosticKind::MissingConsumerCapability => "MissingConsumerCapability",
        Sys5VerticalDiagnosticKind::RelationTransitionRejected => "RelationTransitionRejected",
        Sys5VerticalDiagnosticKind::BackendIneligible => "BackendIneligible",
        Sys5VerticalDiagnosticKind::FabricBootRejected => "FabricBootRejected",
        Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete => "VerticalInventoryIncomplete",
        Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed => {
            "RelationFreshBindingAlreadyConsumed"
        }
        Sys5VerticalDiagnosticKind::DuplicateParticipantLeave => "DuplicateParticipantLeave",
        Sys5VerticalDiagnosticKind::DispatchRejected => "DispatchRejected",
    }
}
