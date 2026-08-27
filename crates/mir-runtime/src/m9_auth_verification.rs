//! Bounded M9 authorization, ContractUpdate, and verifier provenance seam.
//!
//! The public types in this module are I1+ internal provisional carriers. They
//! are not a final API, ABI, or wire representation.  Most importantly, this
//! module never turns the M8 direct admission judgment into success: it keeps
//! the checked artifact and its M9 residual rows intact and records an outer
//! M9 result beside them.

#![allow(
    dead_code,
    clippy::enum_variant_names,
    clippy::result_large_err,
    clippy::too_many_arguments
)]

use std::collections::{BTreeMap, BTreeSet};

use mir_semantics::{
    m9_finite_refinement::{M9FiniteContractDelta, M9FiniteRefinementDischarge},
    shared_model::{ResultVersion, SourceRef},
    surface_v0_pipeline::{
        CheckedProgramIdentity, CheckedSourceMapEntry, CheckedSurfaceV0, ResidualObligationKind,
    },
};

#[cfg(test)]
use mir_semantics::m9_finite_refinement::{M9ContractCandidate, M9FiniteRefinementChecker};

#[cfg(test)]
use crate::m8_runtime_admission::{EvidenceRedaction, EvidenceSecurityLabel, M8SecurityClass};
use crate::m8_runtime_admission::{
    M8AdmissionDiagnosticKind, M8AdmissionEvidence, M8DeferredM9Base, M8RuntimeAdmission,
    M8RuntimeInstance, materialize_m9_resolved_base, prepare_deferred_m9_base,
};
use crate::m8_runtime_authority::{
    M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
};
use crate::m8_runtime_designated_value::M8DesignatedAuthorityUse;
use crate::m8_runtime_observer::M8ObserverAuthorityGrant;
use crate::m8_runtime_owner_queue::M8AuthorityUse;
use crate::m8_runtime_patch::M8PatchAuthorityUse;
use crate::m8_runtime_relation_projection::M8RelationAuthorityUse;

const M9_AUTH_CONTRACT_PREFIX: &str = "membership-authority/";
const M9_VERIFY_CONTRACT: &str = "finite-refinement/MembershipAuth";
const M9_POLICY_VERSION: &str = "m9-policy-v1";
const M9_ADMITTED_AUTH_PROVIDER: &str = "provider:membership-root";
const M9_OBSERVER_LABEL: &str = "authority-private";
const M9_OBSERVER_REDACTION: &str = "redact-authority-lineage";
const M9_OBSERVER_RETENTION: &str = "bounded:contract-update-provenance";
pub(crate) const M9_REMOTE_INPUT_VISIBILITY_RESTRICTED_REDACTED: &str = "restricted_redacted";

/// Canonical release identity for one checked source-owner input read.  The
/// internal release capability carries this value; downstream code may only
/// verify it against checked Core, never invent it from a carrier header.
pub(crate) fn canonical_designated_remote_input_release_label(
    namespace: &str,
    index: Option<&str>,
    field: Option<&str>,
    source_owner: &str,
    frontier: &str,
) -> String {
    format!(
        "input:{}[{}].{}:{}:{}",
        namespace,
        index.unwrap_or(""),
        field.unwrap_or(""),
        source_owner,
        frontier
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9SourceArtifact {
    program_identity: CheckedProgramIdentity,
    root_source_ref: SourceRef,
    owner_evaluation_scopes: BTreeSet<(String, String)>,
    relation_scopes: BTreeSet<(String, String, String)>,
    designated_scopes: BTreeSet<(String, String, String)>,
    designated_remote_input_release_scopes:
        BTreeSet<(String, String, String, usize, String, String, String)>,
}

impl M9SourceArtifact {
    pub fn from_checked_surface(checked: &CheckedSurfaceV0) -> Self {
        Self {
            program_identity: checked.program_identity().clone(),
            root_source_ref: checked.program_identity().root_source_ref().clone(),
            owner_evaluation_scopes: checked
                .evaluations()
                .iter()
                .filter_map(|evaluation| {
                    evaluation.owner_rmw_core().map(|owner| {
                        (
                            evaluation.name().to_string(),
                            owner.owner_locus().to_string(),
                        )
                    })
                })
                .collect(),
            relation_scopes: checked
                .evaluations()
                .iter()
                .filter_map(|evaluation| {
                    evaluation.relation_core().map(|relation| {
                        (
                            evaluation.name().to_string(),
                            relation.owner_locus().to_string(),
                            relation
                                .binding_frontier()
                                .as_slice()
                                .first()
                                .map_or_else(String::new, |occurrence| {
                                    occurrence.as_str().to_string()
                                }),
                        )
                    })
                })
                .collect(),
            designated_scopes: checked
                .evaluations()
                .iter()
                .filter_map(|evaluation| {
                    evaluation.designated_core().map(|designated| {
                        (
                            designated.evaluator().to_string(),
                            designated.result().to_string(),
                            designated
                                .trigger()
                                .frontier()
                                .map_or_else(String::new, ToString::to_string),
                        )
                    })
                })
                .collect(),
            designated_remote_input_release_scopes: checked
                .evaluations()
                .iter()
                .flat_map(|evaluation| {
                    evaluation
                        .designated_core()
                        .into_iter()
                        .flat_map(|designated| {
                            let evaluator = designated.evaluator().to_string();
                            let result = designated.result().to_string();
                            let frontier = designated
                                .trigger()
                                .frontier()
                                .map_or_else(String::new, ToString::to_string);
                            designated
                                .generated_remote_input_dependencies()
                                .iter()
                                .enumerate()
                                .map(move |(dependency_index, dependency)| {
                                    let read = dependency.typed_state_read();
                                    let producer_locus =
                                        dependency.source_owner_locus().to_string();
                                    let release_label =
                                        canonical_designated_remote_input_release_label(
                                            read.namespace(),
                                            read.index(),
                                            read.field(),
                                            &producer_locus,
                                            &frontier,
                                        );
                                    (
                                        producer_locus,
                                        evaluator.clone(),
                                        result.clone(),
                                        dependency_index,
                                        frontier.clone(),
                                        release_label,
                                        M9_REMOTE_INPUT_VISIBILITY_RESTRICTED_REDACTED.to_string(),
                                    )
                                })
                        })
                })
                .collect(),
        }
    }

    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn root_source_ref(&self) -> &SourceRef {
        &self.root_source_ref
    }

    /// Internal M10 negative-validation seam.  The ordinary construction path
    /// remains `from_checked_surface`; this only permits the reference system
    /// to present a deliberately non-matching retained source artifact to the
    /// existing M9 outer-admission validator.
    pub(crate) fn with_validation_program_identity(
        mut self,
        program_identity: CheckedProgramIdentity,
    ) -> Self {
        self.program_identity = program_identity;
        self
    }

    fn contains_owner_evaluation_scope(&self, evaluation: &str, owner_locus: &str) -> bool {
        self.owner_evaluation_scopes
            .contains(&(evaluation.to_string(), owner_locus.to_string()))
    }

    fn contains_relation_scope(
        &self,
        relation: &str,
        owner_locus: &str,
        binding_frontier: &str,
    ) -> bool {
        self.relation_scopes.contains(&(
            relation.to_string(),
            owner_locus.to_string(),
            binding_frontier.to_string(),
        ))
    }

    fn contains_designated_scope(
        &self,
        evaluator: &str,
        result: &str,
        input_frontier: &str,
    ) -> bool {
        self.designated_scopes.contains(&(
            evaluator.to_string(),
            result.to_string(),
            input_frontier.to_string(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn contains_designated_remote_input_release_scope(
        &self,
        producer_locus: &str,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        input_frontier: &str,
        release_label: &str,
        visibility: &str,
    ) -> bool {
        self.designated_remote_input_release_scopes.contains(&(
            producer_locus.to_string(),
            evaluator.to_string(),
            result.to_string(),
            dependency_index,
            input_frontier.to_string(),
            release_label.to_string(),
            visibility.to_string(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ResidualBinding {
    kind: ResidualObligationKind,
    name: String,
    source_ref: Option<SourceRef>,
    module: Option<String>,
    contract: Option<String>,
}

impl M9ResidualBinding {
    pub fn auth_deferred(name: impl Into<String>) -> Self {
        Self::new(ResidualObligationKind::AuthDeferred, name)
    }

    pub fn verify_deferred(name: impl Into<String>) -> Self {
        Self::new(ResidualObligationKind::VerifyDeferred, name)
    }

    fn new(kind: ResidualObligationKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            source_ref: None,
            module: None,
            contract: None,
        }
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn with_module_contract(
        mut self,
        module: impl Into<String>,
        contract: impl Into<String>,
    ) -> Self {
        self.module = Some(module.into());
        self.contract = Some(contract.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AdmissionEnvelope {
    program_identity: CheckedProgramIdentity,
    original_source_artifact: Option<M9SourceArtifact>,
    residual_bindings: Vec<M9ResidualBinding>,
}

impl M9AdmissionEnvelope {
    pub fn for_checked_identity(program_identity: CheckedProgramIdentity) -> Self {
        Self {
            program_identity,
            original_source_artifact: None,
            residual_bindings: Vec::new(),
        }
    }

    pub fn with_original_source_artifact(mut self, artifact: M9SourceArtifact) -> Self {
        self.original_source_artifact = Some(artifact);
        self
    }

    pub fn with_residual_binding(mut self, binding: M9ResidualBinding) -> Self {
        self.residual_bindings.push(binding);
        self
    }

    pub fn apply_delta(mut self, delta: M9AdmissionBindingDelta) -> Self {
        match delta {
            M9AdmissionBindingDelta::Remove(name) => {
                if let Some(index) = self
                    .residual_bindings
                    .iter()
                    .position(|binding| binding.name == name)
                {
                    self.residual_bindings.remove(index);
                }
            }
            M9AdmissionBindingDelta::AddExtra(binding) => self.residual_bindings.push(binding),
            M9AdmissionBindingDelta::Duplicate(name) => {
                if let Some(binding) = self
                    .residual_bindings
                    .iter()
                    .find(|binding| binding.name == name)
                    .cloned()
                {
                    self.residual_bindings.push(binding);
                }
            }
            M9AdmissionBindingDelta::Replace(name, replacement) => {
                if let Some(binding) = self
                    .residual_bindings
                    .iter_mut()
                    .find(|binding| binding.name == name)
                {
                    *binding = replacement;
                }
            }
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M9AdmissionBindingDelta {
    Remove(String),
    AddExtra(M9ResidualBinding),
    Duplicate(String),
    Replace(String, M9ResidualBinding),
}

impl M9AdmissionBindingDelta {
    pub fn remove(name: impl Into<String>) -> Self {
        Self::Remove(name.into())
    }

    pub fn add_extra(binding: M9ResidualBinding) -> Self {
        Self::AddExtra(binding)
    }

    pub fn duplicate(name: impl Into<String>) -> Self {
        Self::Duplicate(name.into())
    }

    pub fn replace(name: impl Into<String>, replacement: M9ResidualBinding) -> Self {
        Self::Replace(name.into(), replacement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9AdmissionErrorKind {
    ProgramIdentityMismatch,
    SourceArtifactMismatch,
    MissingResidualBinding,
    UnexpectedResidualBinding,
    DuplicateResidualBinding,
    ConflictingResidualBinding,
    ResidualKindMismatch,
    SourceRefMismatch,
    M8BaseEvidenceMissing,
    M8BaseEvidenceMismatch,
    ProviderOrTransportIsNotAuthority,
    InvalidMembershipLineage,
    UnadmittedAuthProvider,
    InvalidCapabilityLineage,
    CapabilityPolicyRejected,
    DuplicateMembershipReference,
    ConflictingMembershipReference,
    DuplicateCapabilityReference,
    ConflictingCapabilityReference,
    DuplicateWitnessReference,
    ConflictingWitnessReference,
    InvalidAuthorityCut,
    ReplayedAuthorityCut,
    CompactionBeforeAuditCut,
    MissingVerifyDischarge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AdmissionDiagnostic {
    kind: M9AdmissionErrorKind,
}

impl M9AdmissionDiagnostic {
    pub const fn kind(&self) -> M9AdmissionErrorKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AdmissionDiagnostics {
    primary: M9AdmissionDiagnostic,
}

impl M9AdmissionDiagnostics {
    fn one(kind: M9AdmissionErrorKind) -> Self {
        Self {
            primary: M9AdmissionDiagnostic { kind },
        }
    }

    pub fn primary(&self) -> &M9AdmissionDiagnostic {
        &self.primary
    }

    pub const fn has_runtime_success(&self) -> bool {
        false
    }

    pub const fn grants_authority(&self) -> bool {
        false
    }

    pub const fn emits_verdict(&self) -> bool {
        false
    }

    pub const fn admitted_base(&self) -> Option<&M9AdmittedBase> {
        None
    }

    pub const fn runtime_admission(&self) -> Option<&M9RuntimeAdmitted> {
        None
    }

    pub const fn m8_semantic_state(&self) -> Option<()> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ResidualBindings(Vec<M9ResidualBinding>);

impl M9ResidualBindings {
    pub fn tuples(&self) -> Vec<(ResidualObligationKind, &str, SourceRef, &str, &str)> {
        self.0
            .iter()
            .filter_map(|binding| {
                Some((
                    binding.kind,
                    binding.name.as_str(),
                    binding.source_ref.clone()?,
                    binding.module.as_deref()?,
                    binding.contract.as_deref()?,
                ))
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9OuterAdmission {
    program_identity: CheckedProgramIdentity,
    source_artifact: M9SourceArtifact,
    residual_bindings: M9ResidualBindings,
}

impl M9OuterAdmission {
    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn source_artifact(&self) -> &M9SourceArtifact {
        &self.source_artifact
    }

    pub fn residual_bindings(&self) -> &M9ResidualBindings {
        &self.residual_bindings
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AdmittedBaseBindings(Vec<M9ResidualBinding>);

impl M9AdmittedBaseBindings {
    pub fn tuples(&self) -> Vec<(ResidualObligationKind, &str, &str, &str)> {
        self.0
            .iter()
            .filter_map(|binding| {
                Some((
                    binding.kind,
                    binding.name.as_str(),
                    binding.module.as_deref()?,
                    binding.contract.as_deref()?,
                ))
            })
            .collect()
    }
}

/// Opaque source-bound M9 base.  Its embedded M8 plan view is intentionally
/// not exposed as an `M8RuntimeInstance` or a public M8 runtime constructor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AdmittedBase {
    program_identity: CheckedProgramIdentity,
    m8_base_evidence: Vec<M8AdmissionEvidence>,
    m9_residual_bindings: M9AdmittedBaseBindings,
    outer_admission: M9OuterAdmission,
    ordered_source_to_core_map: Vec<CheckedSourceMapEntry>,
    plan_count: usize,
    _embedded_m8_base: M8DeferredM9Base,
}

impl M9AdmittedBase {
    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn m8_base_evidence(&self) -> &[M8AdmissionEvidence] {
        &self.m8_base_evidence
    }

    pub fn m9_residual_bindings(&self) -> &M9AdmittedBaseBindings {
        &self.m9_residual_bindings
    }

    pub const fn exposes_raw_m8_instance(&self) -> bool {
        false
    }

    pub const fn is_prepared_base(&self) -> bool {
        true
    }

    pub const fn is_runtime_admitted(&self) -> bool {
        false
    }

    pub const fn has_runtime_success(&self) -> bool {
        false
    }

    pub const fn m8_semantic_state(&self) -> Option<()> {
        None
    }

    fn m8_payload_snapshot(&self) -> M9M8PayloadSnapshot {
        M9M8PayloadSnapshot {
            program_identity: self.program_identity.clone(),
            evidence: self.m8_base_evidence.clone(),
        }
    }

    /// Reserved M10 composition hook.  The authority runtime is initialized
    /// from the exact outer admission retained by this base; callers cannot
    /// substitute a provider or a different checked artifact.
    pub(crate) fn authority_runtime(&self) -> M9AuthorityRuntime {
        M9AuthorityRuntime::from_outer_admission(self.outer_admission.clone())
    }

    #[allow(dead_code)] // Called only by the reserved crate-private M10 seam.
    fn into_embedded_m8_base(self) -> M8DeferredM9Base {
        self._embedded_m8_base
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct M9AdmissionRuntime {
    _private: (),
}

impl M9AdmissionRuntime {
    pub fn admit_outer(
        &self,
        checked: CheckedSurfaceV0,
        envelope: M9AdmissionEnvelope,
    ) -> Result<M9OuterAdmission, M9AdmissionDiagnostics> {
        validate_outer(&checked, &envelope)?;
        Ok(M9OuterAdmission {
            program_identity: checked.program_identity().clone(),
            source_artifact: envelope
                .original_source_artifact
                .expect("validated outer envelope retains source artifact"),
            residual_bindings: M9ResidualBindings(envelope.residual_bindings),
        })
    }

    pub fn admit_source_bound_base(
        &self,
        checked: CheckedSurfaceV0,
        m8_admission: M8RuntimeAdmission,
        envelope: M9AdmissionEnvelope,
    ) -> Result<M9AdmittedBase, M9AdmissionDiagnostics> {
        let outer = self.admit_outer(checked.clone(), envelope)?;
        let embedded =
            prepare_deferred_m9_base(&checked, &m8_admission).map_err(|diagnostics| {
                let kind = match diagnostics.primary().kind() {
                    M8AdmissionDiagnosticKind::MissingResidualEvidence => {
                        M9AdmissionErrorKind::M8BaseEvidenceMissing
                    }
                    _ => M9AdmissionErrorKind::M8BaseEvidenceMismatch,
                };
                M9AdmissionDiagnostics::one(kind)
            })?;
        let embedded_residuals_match_outer = embedded.deferred_residuals().iter().all(|residual| {
            outer.residual_bindings.0.iter().any(|binding| {
                binding.kind == residual.kind()
                    && binding.name == residual.name()
                    && binding.source_ref.as_ref() == Some(residual.source_ref())
            })
        });
        if embedded.program_identity() != checked.program_identity()
            || embedded.deferred_residuals().len() != outer.residual_bindings.0.len()
            || !embedded_residuals_match_outer
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::M8BaseEvidenceMismatch,
            ));
        }
        Ok(M9AdmittedBase {
            program_identity: checked.program_identity().clone(),
            m8_base_evidence: m8_admission.evidence().to_vec(),
            m9_residual_bindings: M9AdmittedBaseBindings(outer.residual_bindings.0.clone()),
            outer_admission: outer,
            ordered_source_to_core_map: checked.source_map().entries().to_vec(),
            plan_count: checked.evaluations().len(),
            _embedded_m8_base: embedded,
        })
    }

    pub fn prepare_outer(&self, source_artifact: M9SourceArtifact) -> M9PreparedOuter {
        M9PreparedOuter {
            source_artifact,
            auth_residuals: BTreeSet::new(),
            unresolved_verify_residuals: BTreeSet::new(),
        }
    }

    /// Resolve the prepared M9 base only after exact typed authority lineage
    /// and the retained finite-refinement discharge are present.  The result,
    /// rather than `M9AdmittedBase`, is the sole public carrier that may enter
    /// ContractUpdate or the future M10 execution seam.
    pub fn admit_runtime(
        &self,
        base: M9AdmittedBase,
        authority_runtime: M9AuthorityRuntime,
        evidence: M9FinalAdmissionEvidence,
    ) -> Result<M9RuntimeAdmitted, M9AdmissionDiagnostics> {
        if evidence.finite_refinement.is_none() {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::MissingVerifyDischarge,
            ));
        }
        if !final_evidence_matches_base(&base, &authority_runtime, &evidence) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }
        Ok(M9RuntimeAdmitted {
            base,
            authority_runtime,
            evidence,
        })
    }
}

/// Exact authority and verifier evidence consumed by the final M9 judgment.
/// Its records can only be produced through the typed authority runtime and
/// finite verifier lanes; public code cannot construct live records from a
/// string or transport identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9FinalAdmissionEvidence {
    membership_ref: String,
    capability_ref: String,
    witness_ref: String,
    finite_refinement: Option<M9FiniteRefinementDischarge>,
}

impl M9FinalAdmissionEvidence {
    pub fn new(
        membership_ref: impl Into<String>,
        capability_ref: impl Into<String>,
        witness_ref: impl Into<String>,
        finite_refinement: M9FiniteRefinementDischarge,
    ) -> Self {
        Self {
            membership_ref: membership_ref.into(),
            capability_ref: capability_ref.into(),
            witness_ref: witness_ref.into(),
            finite_refinement: Some(finite_refinement),
        }
    }

    pub fn from_lineage(
        membership: &M9MembershipAuth,
        capability: &M9CapabilityAuth,
        witness: &M9WitnessAuth,
        finite_refinement: M9FiniteRefinementDischarge,
    ) -> Self {
        Self::new(
            membership.ref_id(),
            capability.ref_id(),
            witness.ref_id(),
            finite_refinement,
        )
    }

    pub fn membership_ref(&self) -> &str {
        &self.membership_ref
    }

    pub fn capability_ref(&self) -> &str {
        &self.capability_ref
    }

    pub fn witness_ref(&self) -> &str {
        &self.witness_ref
    }

    pub fn finite_refinement(&self) -> Option<&M9FiniteRefinementDischarge> {
        self.finite_refinement.as_ref()
    }

    pub fn without_finite_refinement_discharge(mut self) -> Self {
        self.finite_refinement = None;
        self
    }
}

/// Fully resolved outer M9 runtime carrier.  It remains opaque with respect to
/// M8: it can cross a crate-private M10 execution seam but never exposes a
/// public `M8RuntimeInstance` or grants direct M8 admission success.
#[derive(PartialEq, Eq)]
pub struct M9RuntimeAdmitted {
    base: M9AdmittedBase,
    authority_runtime: M9AuthorityRuntime,
    evidence: M9FinalAdmissionEvidence,
}

/// Crate-private M9-to-M8 execution material.  Only the final M9 judgment
/// creates this carrier; it exposes neither provider data nor an authority
/// constructor to callers.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct M9RuntimeExecutionSeam {
    instance: M8RuntimeInstance,
    authority_state: M8AuthorityState,
    authority_snapshot_projection: String,
    authority_membership_projection: String,
    authority_grant_projection: String,
    owner_uses: BTreeMap<(String, String, String), M8AuthorityUse>,
    patch_uses: BTreeMap<(String, String, String), M8PatchAuthorityUse>,
    relation_uses: BTreeMap<(String, String), M8RelationAuthorityUse>,
    designated_evaluation_uses: BTreeMap<(String, String), M8DesignatedAuthorityUse>,
    designated_consumption_uses: BTreeMap<(String, String), M8DesignatedAuthorityUse>,
    observer_authorities: BTreeMap<String, M8ObserverAuthorityGrant>,
    translation_refs: BTreeMap<String, (String, String, String)>,
    kernel_owner_lineages: BTreeMap<(String, String, String), M9KernelOwnerLineage>,
    kernel_designated_remote_input_lineages:
        BTreeMap<(String, String, String, usize, String), M9KernelDesignatedRemoteInputLineage>,
    authority_generation: M9AuthorityGeneration,
    /// All normal production paths construct this seam only after the finite
    /// residual discharge has been accepted.  The test-only incomplete helper
    /// below starts from that same typed path and then marks the resulting
    /// carrier unavailable to a later SYS-4 admission check; it never
    /// fabricates authority facts.
    final_residual_discharge_complete: bool,
    #[cfg_attr(not(test), allow(dead_code))]
    authority_successor: Option<M9AuthoritySuccessorPublisher>,
}

/// Immutable, crate-private authority successor view.  It is produced by the
/// M9 boundary and carries the exact translated inventory and audit lineages
/// needed by the kernel; it is neither a credential constructor nor a wire
/// carrier.
#[derive(Clone)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct M9AuthorityGeneration {
    program_identity: String,
    generation: u64,
    generation_ref: String,
    authority_state: M8AuthorityState,
    owner_uses: BTreeMap<(String, String, String), M8AuthorityUse>,
    designated_evaluation_uses: BTreeMap<(String, String), M8DesignatedAuthorityUse>,
    designated_consumption_uses: BTreeMap<(String, String), M8DesignatedAuthorityUse>,
    kernel_owner_lineages: BTreeMap<(String, String, String), M9KernelOwnerLineage>,
    revoked_owner_capabilities: BTreeSet<(String, String, String)>,
    revoked_designated_consumption_capabilities: BTreeSet<(String, String)>,
    kernel_designated_remote_input_lineages:
        BTreeMap<(String, String, String, usize, String), M9KernelDesignatedRemoteInputLineage>,
    designated_consumer_failures: BTreeMap<(String, String), M9AdmissionErrorKind>,
    designated_consumer_witness_retirements: BTreeSet<(String, String)>,
    designated_source_release_failures:
        BTreeMap<(String, String, String, usize, String), M9AdmissionErrorKind>,
    designated_consumer_validation_occurrences: BTreeMap<(String, String, String), usize>,
    owner_operation_validation_occurrences: BTreeMap<(String, String, String), usize>,
    source_release_validation_occurrences: BTreeMap<(String, String, String), usize>,
}

/// Observer-safe identity for one M9-issued designated-result consumer
/// lineage.  It deliberately exposes no credential or witness payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9DesignatedConsumerLineage {
    consumer_locus: String,
    opaque_lineage_ref: String,
}

impl M9DesignatedConsumerLineage {
    pub(crate) fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub(crate) fn opaque_lineage_ref(&self) -> &str {
        &self.opaque_lineage_ref
    }
}

/// Observer-safe identity for one checked producer release lineage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9DesignatedSourceReleaseLineage {
    opaque_lineage_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9SealedGeneration {
    generation: u64,
    generation_ref: String,
    m9_produced: bool,
}

impl M9SealedGeneration {
    pub(crate) const fn is_m9_produced(&self) -> bool {
        self.m9_produced
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum M9AuthorityTransitionKind {
    DesignatedConsumerCapabilityRevoked,
    DesignatedConsumerMembershipRetired,
    DesignatedConsumerWitnessRetired,
    DesignatedSourceReleaseRevoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9SealedTransitionInspection {
    transition_kind: M9AuthorityTransitionKind,
    prior_generation: M9SealedGeneration,
    successor_generation: M9SealedGeneration,
    consumer_lineage: Option<M9DesignatedConsumerLineage>,
    source_release_lineage: Option<M9DesignatedSourceReleaseLineage>,
}

impl M9SealedTransitionInspection {
    pub(crate) const fn transition_kind(&self) -> M9AuthorityTransitionKind {
        self.transition_kind
    }
    pub(crate) fn prior_generation(&self) -> M9SealedGeneration {
        self.prior_generation.clone()
    }
    pub(crate) fn successor_generation(&self) -> M9SealedGeneration {
        self.successor_generation.clone()
    }
    pub(crate) fn consumer_lineage(&self) -> &M9DesignatedConsumerLineage {
        self.consumer_lineage
            .as_ref()
            .expect("consumer transition retains its sealed lineage")
    }
    pub(crate) fn source_release_lineage(&self) -> &M9DesignatedSourceReleaseLineage {
        self.source_release_lineage
            .as_ref()
            .expect("source-release transition retains its sealed lineage")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9AuthorityInspection {
    generation: M9SealedGeneration,
    consumers: BTreeMap<(String, String), M9DesignatedConsumerLineage>,
    source_releases:
        BTreeMap<(String, String, String, usize, String), M9DesignatedSourceReleaseLineage>,
    designated_consumer_validation_occurrences: BTreeMap<(String, String, String), usize>,
    owner_operation_validation_occurrences: BTreeMap<(String, String, String), usize>,
    source_release_validation_occurrences: BTreeMap<(String, String, String), usize>,
}

impl M9AuthorityInspection {
    pub(crate) fn generation(&self) -> M9SealedGeneration {
        self.generation.clone()
    }
    pub(crate) fn designated_consumer_lineage(
        &self,
        value_name: &str,
        consumer_locus: &str,
    ) -> Option<&M9DesignatedConsumerLineage> {
        self.consumers
            .get(&(value_name.to_string(), consumer_locus.to_string()))
    }
    pub(crate) fn designated_source_release_lineage<F: std::fmt::Debug>(
        &self,
        evaluator: &str,
        result: &str,
        source_locus: &str,
        dependency_index: usize,
        _input_frontier: F,
    ) -> Option<&M9DesignatedSourceReleaseLineage> {
        self.source_releases.iter().find_map(
            |(
                (candidate_evaluator, candidate_result, candidate_source, candidate_index, _),
                lineage,
            )| {
                (candidate_evaluator == evaluator
                    && candidate_result == result
                    && candidate_source == source_locus
                    && *candidate_index == dependency_index)
                    .then_some(lineage)
            },
        )
    }
    pub(crate) fn validation_occurrence_count(
        &self,
        operation: &str,
        consumer: &str,
        semantic_identity: &str,
    ) -> usize {
        self.designated_consumer_validation_occurrences
            .get(&(
                operation.to_string(),
                consumer.to_string(),
                semantic_identity.to_string(),
            ))
            .copied()
            .unwrap_or(0)
    }

    pub(crate) fn owner_operation_validation_count(
        &self,
        operation: &str,
        owner_locus: &str,
        request_id: &str,
    ) -> usize {
        self.owner_operation_validation_occurrences
            .get(&(
                operation.to_string(),
                owner_locus.to_string(),
                request_id.to_string(),
            ))
            .copied()
            .unwrap_or(0)
    }

    pub(crate) fn source_release_validation_count(
        &self,
        result: &str,
        source_locus: &str,
        request_id: &str,
    ) -> usize {
        self.source_release_validation_occurrences
            .get(&(
                result.to_string(),
                source_locus.to_string(),
                request_id.to_string(),
            ))
            .copied()
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9CacheValidationInspection {
    generation: M9SealedGeneration,
    consumer_lineage: M9DesignatedConsumerLineage,
    semantic_identity: String,
    consumer_locus: String,
    occurrence_id: String,
}

impl M9CacheValidationInspection {
    pub(crate) fn generation(&self) -> M9SealedGeneration {
        self.generation.clone()
    }
    pub(crate) fn consumer_lineage(&self) -> &M9DesignatedConsumerLineage {
        &self.consumer_lineage
    }
    pub(crate) fn semantic_identity(&self) -> &str {
        &self.semantic_identity
    }
    pub(crate) fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }
    pub(crate) fn occurrence_id(&self) -> &str {
        &self.occurrence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9SourceReleaseValidationInspection {
    generation: M9SealedGeneration,
    lineage: M9DesignatedSourceReleaseLineage,
    occurrence_id: String,
}

impl M9SourceReleaseValidationInspection {
    pub(crate) fn generation(&self) -> M9SealedGeneration {
        self.generation.clone()
    }
    pub(crate) fn lineage(&self) -> &M9DesignatedSourceReleaseLineage {
        &self.lineage
    }
    pub(crate) fn occurrence_id(&self) -> &str {
        &self.occurrence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9SealedFailureInspection {
    admission_error_kind: M9AdmissionErrorKind,
    installed_generation: M9SealedGeneration,
    consumer_lineage: M9DesignatedConsumerLineage,
    request_id: String,
    semantic_identity: String,
    consumer_locus: String,
}

impl M9SealedFailureInspection {
    pub(crate) const fn admission_error_kind(&self) -> M9AdmissionErrorKind {
        self.admission_error_kind
    }
    pub(crate) fn installed_generation(&self) -> M9SealedGeneration {
        self.installed_generation.clone()
    }
    pub(crate) fn consumer_lineage(&self) -> &M9DesignatedConsumerLineage {
        &self.consumer_lineage
    }
    pub(crate) fn request_id(&self) -> &str {
        &self.request_id
    }
    pub(crate) fn semantic_identity(&self) -> &str {
        &self.semantic_identity
    }
    pub(crate) fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }
    pub(crate) const fn rejected_before_m8_non_consuming_validation(&self) -> bool {
        true
    }
}

/// The sole mutable publisher for successor authority generations.  It stays
/// in the M9 module and calls M9 revocation before materializing the next
/// immutable view; the kernel receives only the resulting generation.
#[derive(Clone)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct M9AuthoritySuccessorPublisher {
    base: M9AdmittedBase,
    evidence: M9FinalAdmissionEvidence,
    authority_runtime: M9AuthorityRuntime,
    current: M9AuthorityGeneration,
}

impl std::fmt::Debug for M9AuthorityGeneration {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9AuthorityGeneration")
            .field("generation", &self.generation)
            .field("program_identity", &self.program_identity)
            .finish_non_exhaustive()
    }
}

/// Read-only authority facts shared by an initial M9 execution seam and an
/// immutable M9 authority successor.  The semantic kernel only consumes this
/// view; it cannot publish or transform authority through it.
pub(crate) trait M9KernelAuthorityView {
    fn kernel_owner_lineage(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M9KernelOwnerLineage>;
    fn owner_authority_use(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M8AuthorityUse>;
    fn kernel_designated_remote_input_lineage(
        &self,
        producer_locus: &str,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        input_frontier: &str,
    ) -> Option<M9KernelDesignatedRemoteInputLineage>;
}

/// Historical compatibility spelling for M10's consumer-side bridge.  New
/// runtime kernel code must depend on [`M9RuntimeExecutionSeam`] directly.
pub(crate) type M9M10ExecutionSeam = M9RuntimeExecutionSeam;

/// M9-sealed owner lineage exposed only to the internal SYS-1 kernel.  This
/// is an authenticated execution fact, not a credential constructor or wire
/// carrier.  The M10 facade may still consume its legacy M8 bridge separately.
#[derive(Clone)]
pub(crate) struct M9KernelOwnerLineage {
    principal: String,
    owner_locus: String,
    membership_ref: String,
    membership_epoch: String,
    membership_incarnation: String,
    capability_ref: String,
    witness_ref: String,
}

impl std::fmt::Debug for M9KernelOwnerLineage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M9KernelOwnerLineage(<sealed>)")
    }
}

impl M9KernelOwnerLineage {
    pub(crate) fn principal(&self) -> &str {
        &self.principal
    }

    pub(crate) fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub(crate) fn membership_ref(&self) -> &str {
        &self.membership_ref
    }

    pub(crate) fn membership_epoch(&self) -> &str {
        &self.membership_epoch
    }

    pub(crate) fn membership_incarnation(&self) -> &str {
        &self.membership_incarnation
    }

    pub(crate) fn capability_ref(&self) -> &str {
        &self.capability_ref
    }

    pub(crate) fn witness_ref(&self) -> &str {
        &self.witness_ref
    }
}

/// M9-sealed producer authority for exactly one checked designated remote
/// input dependency.  The sealed scope, not evaluator authority or carrier
/// metadata, fixes the release tuple and its producer-side lineage.
#[derive(Clone)]
pub(crate) struct M9KernelDesignatedRemoteInputLineage {
    principal: String,
    producer_locus: String,
    evaluator: String,
    result: String,
    dependency_index: usize,
    input_frontier: String,
    release_label: String,
    visibility: String,
    membership_ref: String,
    membership_epoch: String,
    membership_incarnation: String,
    capability_ref: String,
    witness_ref: String,
}

impl std::fmt::Debug for M9KernelDesignatedRemoteInputLineage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M9KernelDesignatedRemoteInputLineage(<sealed>)")
    }
}

impl M9KernelDesignatedRemoteInputLineage {
    pub(crate) fn principal(&self) -> &str {
        &self.principal
    }

    pub(crate) fn membership_ref(&self) -> &str {
        &self.membership_ref
    }

    pub(crate) fn producer_locus(&self) -> &str {
        &self.producer_locus
    }

    pub(crate) fn evaluator(&self) -> &str {
        &self.evaluator
    }

    pub(crate) fn result(&self) -> &str {
        &self.result
    }

    pub(crate) const fn dependency_index(&self) -> usize {
        self.dependency_index
    }

    pub(crate) fn input_frontier(&self) -> &str {
        &self.input_frontier
    }

    pub(crate) fn release_label(&self) -> &str {
        &self.release_label
    }

    pub(crate) fn visibility(&self) -> &str {
        &self.visibility
    }

    pub(crate) fn membership_epoch(&self) -> &str {
        &self.membership_epoch
    }

    pub(crate) fn membership_incarnation(&self) -> &str {
        &self.membership_incarnation
    }

    pub(crate) fn capability_ref(&self) -> &str {
        &self.capability_ref
    }

    pub(crate) fn witness_ref(&self) -> &str {
        &self.witness_ref
    }
}

/// A sealed M9 authority snapshot translated for one already-admitted M8
/// execution session.  This is intentionally crate-private: only M9 may
/// materialize M8 authority records from authenticated M9 lineage.
#[derive(Clone)]
pub(crate) struct M9M10AuthorityBridge {
    authority_state: M8AuthorityState,
    authority_snapshot_projection: String,
    owner_use: Option<(String, String, M8AuthorityUse)>,
    patch_use: Option<M8PatchAuthorityUse>,
    relation_uses: BTreeMap<(String, String), M8RelationAuthorityUse>,
}

/// The only crate-private bridge by which M8 may synchronize entity presence
/// from an M9 membership lineage. Its fields are opaque outside this module;
/// callers cannot construct presence from a membership string or infer raw
/// membership provenance from debug output.
#[derive(Clone)]
pub(crate) struct M9M8EntityPresenceBridge {
    namespace: String,
    identity: String,
    source_ref: SourceRef,
    status: M9M8EntityPresenceStatus,
    sealed_membership_ref: String,
    sealed_capability_ref: String,
    sealed_witness_ref: String,
    sealed_epoch: String,
    sealed_incarnation: String,
    m9_snapshot_ref: String,
    m8_authority_use_ref: String,
}

impl std::fmt::Debug for M9M8EntityPresenceBridge {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M9M8EntityPresenceBridge(<sealed>)")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum M9M8EntityPresenceStatus {
    Live,
    Retired,
}

impl M9M8EntityPresenceStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Retired => "retired",
        }
    }
}

impl M9M8EntityPresenceBridge {
    pub(crate) fn namespace(&self) -> &str {
        &self.namespace
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) const fn status(&self) -> M9M8EntityPresenceStatus {
        self.status
    }

    pub(crate) fn sealed_membership_ref(&self) -> &str {
        &self.sealed_membership_ref
    }

    pub(crate) fn sealed_capability_ref(&self) -> &str {
        &self.sealed_capability_ref
    }

    pub(crate) fn sealed_witness_ref(&self) -> &str {
        &self.sealed_witness_ref
    }

    pub(crate) fn sealed_epoch(&self) -> &str {
        &self.sealed_epoch
    }

    pub(crate) fn sealed_incarnation(&self) -> &str {
        &self.sealed_incarnation
    }

    pub(crate) fn m9_snapshot_ref(&self) -> &str {
        &self.m9_snapshot_ref
    }

    pub(crate) fn m8_authority_use_ref(&self) -> &str {
        &self.m8_authority_use_ref
    }
}

/// Crate-private inventory for M10's before/after no-mint evidence. Raw
/// references never leave the runtime crate; M10 renders only opaque hashes.
#[derive(Clone)]
pub(crate) struct M9AuthorityFactInventory {
    membership_refs: BTreeSet<String>,
    grant_refs: BTreeSet<String>,
    witness_refs: BTreeSet<String>,
    retirement_tombstones: BTreeSet<String>,
}

impl std::fmt::Debug for M9AuthorityFactInventory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9AuthorityFactInventory")
            .field("membership_count", &self.membership_refs.len())
            .field("grant_count", &self.grant_refs.len())
            .field("witness_count", &self.witness_refs.len())
            .field(
                "retirement_tombstone_count",
                &self.retirement_tombstones.len(),
            )
            .finish()
    }
}

impl M9AuthorityFactInventory {
    pub(crate) fn membership_refs(&self) -> &BTreeSet<String> {
        &self.membership_refs
    }

    pub(crate) fn grant_refs(&self) -> &BTreeSet<String> {
        &self.grant_refs
    }

    pub(crate) fn witness_refs(&self) -> &BTreeSet<String> {
        &self.witness_refs
    }

    pub(crate) fn retirement_tombstones(&self) -> &BTreeSet<String> {
        &self.retirement_tombstones
    }
}

fn m9_opaque_ref(input: &str) -> String {
    let hash = input
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325_u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        });
    format!("m9-sealed:{hash:016x}")
}

impl M9M10AuthorityBridge {
    pub(crate) fn authority_state(&self) -> M8AuthorityState {
        self.authority_state.clone()
    }

    pub(crate) fn authority_snapshot_projection(&self) -> &str {
        &self.authority_snapshot_projection
    }

    pub(crate) fn owner_use(&self) -> Option<(String, String, M8AuthorityUse)> {
        self.owner_use.clone()
    }

    pub(crate) fn patch_use(&self) -> Option<M8PatchAuthorityUse> {
        self.patch_use.clone()
    }

    /// Return the exact M8 relation transition use derived from an active M9
    /// membership/capability/witness lineage.  This remains an internal
    /// bridge accessor; it does not mint M8 authority.
    pub(crate) fn relation_authority_use(
        &self,
        relation: &str,
        transition: &str,
    ) -> Option<M8RelationAuthorityUse> {
        self.relation_uses
            .get(&(relation.to_string(), transition.to_string()))
            .cloned()
    }
}

impl M9RuntimeExecutionSeam {
    /// The initial immutable authority generation consumed by SYS-2.  The
    /// mutable authority runtime remains entirely inside M9; downstream code
    /// gets only this opaque view and its translated M8 inventory.
    pub(crate) fn initial_authority_generation(&self) -> M9AuthorityGeneration {
        self.authority_generation.clone()
    }

    pub(crate) const fn has_complete_final_residual_discharge(&self) -> bool {
        self.final_residual_discharge_complete
    }

    #[cfg(test)]
    pub(crate) fn into_authority_successor_publisher(
        mut self,
    ) -> Option<M9AuthoritySuccessorPublisher> {
        self.authority_successor.take()
    }

    /// SYS-4's test boundary deliberately reuses the normal typed M9
    /// admission route.  It selects the only accepted finite owner or
    /// designated fragment from checked source; it does not construct a seam
    /// from fixture data or fabricated grants.
    #[cfg(test)]
    pub(crate) fn test_real_admitted_sys4_fabric_seam(
        checked: &CheckedSurfaceV0,
    ) -> Result<Self, String> {
        if let Some(designated) = checked
            .evaluations()
            .iter()
            .find(|evaluation| evaluation.designated_core().is_some())
        {
            let result = designated.result_name().ok_or_else(|| {
                "SYS-4 designated evaluation lacks checked result name".to_string()
            })?;
            return Self::test_real_admitted_designated_seam_for_kernel(
                checked,
                designated.name(),
                result,
                0,
                true,
            );
        }

        let owner = checked
            .evaluations()
            .iter()
            .find(|evaluation| evaluation.owner_rmw_core().is_some())
            .ok_or_else(|| {
                "SYS-4 source has no admitted owner or designated fragment".to_string()
            })?;
        let owner_locus = owner
            .owner_rmw_core()
            .expect("selected owner evaluation retains Core")
            .owner_locus();
        Self::test_real_admitted_owner_seam_for_kernel(
            checked,
            owner.name(),
            owner.actor_authority_origin(),
            owner_locus,
        )
    }

    /// Negative SYS-4 admission keeps the same source-bound M9/authority
    /// pipeline and only removes the final discharge availability bit.  This
    /// represents an incomplete candidate, not a forged authority seam.
    #[cfg(test)]
    pub(crate) fn test_incomplete_sys4_fabric_seam_missing_residual_discharge(
        checked: &CheckedSurfaceV0,
    ) -> Result<Self, String> {
        let mut seam = Self::test_real_admitted_sys4_fabric_seam(checked)?;
        seam.final_residual_discharge_complete = false;
        Ok(seam)
    }

    /// Build a complete owner-operation admission through the normal typed M9
    /// authority pipeline.  It remains test-only so this does not create a
    /// production source-free admission path.
    #[cfg(test)]
    pub(crate) fn test_real_admitted_owner_seam_for_kernel(
        checked: &CheckedSurfaceV0,
        operation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Result<Self, String> {
        let evaluation = checked
            .evaluation(operation)
            .ok_or_else(|| "kernel test requested an unchecked owner operation".to_string())?;
        let core = evaluation
            .owner_rmw_core()
            .ok_or_else(|| "kernel test requested a non-owner operation".to_string())?;
        if core.owner_locus() != owner_locus {
            return Err("kernel test requested a mismatched owner locus".to_string());
        }
        Self::test_real_admitted_owner_kernel_seam(
            checked,
            [(operation, principal, owner_locus)],
            None,
        )
    }

    /// Test-only admission utility which still uses one normal M9 authority
    /// runtime for every listed owner capability (and, optionally, the one
    /// source-owner designated release).  It deliberately has no map
    /// insertion or synthetic lineage path: the resulting seam is exactly
    /// what M9 translates from its active typed inventory.
    #[cfg(test)]
    fn test_real_admitted_owner_kernel_seam<const N: usize>(
        checked: &CheckedSurfaceV0,
        owners: [(&str, &str, &str); N],
        remote_release: Option<(&str, &str, usize)>,
    ) -> Result<Self, String> {
        if owners.is_empty() {
            return Err("kernel test requested empty owner set".to_string());
        }
        let remote_dependency = remote_release
            .map(|(evaluator, result, dependency_index)| {
                let dependency = checked
                    .designated_result(evaluator, result)
                    .and_then(|evaluation| evaluation.designated_core())
                    .and_then(|core| {
                        core.generated_remote_input_dependencies()
                            .get(dependency_index)
                    })
                    .ok_or_else(|| {
                        "kernel test requested an unchecked designated dependency".to_string()
                    })?;
                let input_frontier = checked
                    .designated_result(evaluator, result)
                    .and_then(|evaluation| evaluation.designated_core())
                    .and_then(|core| core.trigger().frontier())
                    .ok_or_else(|| {
                        "kernel test designated dependency lacks a checked frontier".to_string()
                    })?;
                Ok::<_, String>((
                    evaluator,
                    result,
                    dependency_index,
                    dependency,
                    input_frontier,
                ))
            })
            .transpose()?;
        for &(operation, principal, owner_locus) in &owners {
            let evaluation = checked
                .evaluation(operation)
                .ok_or_else(|| "kernel test requested an unchecked owner operation".to_string())?;
            let owner_core = evaluation
                .owner_rmw_core()
                .ok_or_else(|| "kernel test requested a non-owner operation".to_string())?;
            if evaluation.actor_authority_origin() != principal
                || owner_core.owner_locus() != owner_locus
            {
                return Err("kernel test requested a mismatched owner lineage".to_string());
            }
        }

        let m9 = M9AdmissionRuntime::default();
        let base = m9
            .admit_source_bound_base(
                checked.clone(),
                test_kernel_m8_admission_for(checked)?,
                test_kernel_m9_envelope_for(checked),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 base admission: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let auth_residual = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
            .ok_or_else(|| "kernel test source lacks AuthDeferred".to_string())?;
        let mut authority = base.authority_runtime();
        let mut final_lineage = None;
        let mut remote_release_issued = false;
        for (owner_index, (operation, principal, owner_locus)) in owners.into_iter().enumerate() {
            let epoch = format!("epoch{}", owner_index + 1);
            let incarnation = format!("incarnation:{principal}:{owner_locus}:{epoch}");
            let attestation = authority
                .issue_membership_attestation(
                    principal,
                    owner_locus,
                    &epoch,
                    incarnation.clone(),
                    auth_residual.name(),
                    auth_residual.source_ref().clone(),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 membership attestation: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let membership = authority
                .authenticate_membership(
                    M9MembershipRequest::new(principal, owner_locus, &epoch)
                        .with_incarnation(incarnation)
                        .with_auth_residual(
                            auth_residual.name(),
                            auth_residual.source_ref().clone(),
                        )
                        .with_issued_provider_attestation(attestation),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 membership: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            if final_lineage.is_none() {
                let contract_capability = authority
                    .authorize_capability(
                        M9CapabilityGrantRequest::new("kernel-test-contract-capability")
                            .with_membership_ref(membership.ref_id())
                            .with_scope(M9CapabilityScope::contract_update(
                                checked.program_identity().module(),
                                format!("membership-authority/{}", auth_residual.name()),
                            ))
                            .with_lineage_epoch(membership.epoch())
                            .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!(
                            "kernel test M9 contract capability: {:?}",
                            diagnostics.primary().kind()
                        )
                    })?;
                let contract_witness = authority
                    .materialize_witness(
                        M9WitnessRequest::new("kernel-test-contract-witness")
                            .with_membership_ref(membership.ref_id())
                            .with_capability_ref(contract_capability.ref_id())
                            .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!(
                            "kernel test M9 contract witness: {:?}",
                            diagnostics.primary().kind()
                        )
                    })?;
                final_lineage = Some((membership.clone(), contract_capability, contract_witness));
            }
            let owner_capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "kernel-test-owner-evaluation:{operation}:{owner_locus}:{principal}:{epoch}"
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::owner_evaluation(operation, owner_locus))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 owner capability: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let _owner_witness = authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "kernel-test-owner-witness:{operation}:{owner_locus}:{principal}:{epoch}"
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(owner_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 owner witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;

            if let Some((evaluator, result, dependency_index, dependency, input_frontier)) =
                remote_dependency
                && !remote_release_issued
                && dependency.source_owner_locus() == owner_locus
            {
                let evaluation_capability = authority
                    .authorize_capability(
                        M9CapabilityGrantRequest::new(format!(
                            "kernel-test-designated-evaluation:{evaluator}:{result}:{input_frontier}"
                        ))
                        .with_membership_ref(membership.ref_id())
                        .with_scope(M9CapabilityScope::designated_evaluation(
                            evaluator,
                            result,
                            input_frontier,
                        ))
                        .with_lineage_epoch(membership.epoch())
                        .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!(
                            "kernel test M9 designated capability: {:?}",
                            diagnostics.primary().kind()
                        )
                    })?;
                let _evaluation_witness = authority
                    .materialize_witness(
                        M9WitnessRequest::new(format!(
                            "kernel-test-designated-evaluation-witness:{evaluator}:{result}:{input_frontier}"
                        ))
                        .with_membership_ref(membership.ref_id())
                        .with_capability_ref(evaluation_capability.ref_id())
                        .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!(
                            "kernel test M9 designated witness: {:?}",
                            diagnostics.primary().kind()
                        )
                    })?;
                let read = dependency.typed_state_read();
                let release_label = canonical_designated_remote_input_release_label(
                    read.namespace(),
                    read.index(),
                    read.field(),
                    owner_locus,
                    input_frontier,
                );
                let release_capability = authority
                    .authorize_capability(
                        M9CapabilityGrantRequest::new(format!(
                            "cap:attack:{owner_locus}:{principal}:{epoch}"
                        ))
                        .with_membership_ref(membership.ref_id())
                        .with_scope(M9CapabilityScope::designated_remote_input_release(
                            owner_locus,
                            evaluator,
                            result,
                            dependency_index,
                            input_frontier,
                            release_label,
                            M9_REMOTE_INPUT_VISIBILITY_RESTRICTED_REDACTED,
                        ))
                        .with_lineage_epoch(membership.epoch())
                        .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!(
                            "kernel test M9 designated release capability: {:?}",
                            diagnostics.primary().kind()
                        )
                    })?;
                let _release_witness = authority
                    .materialize_witness(
                        M9WitnessRequest::new(format!(
                            "witness:attack:{owner_locus}:{principal}:{epoch}"
                        ))
                        .with_membership_ref(membership.ref_id())
                        .with_capability_ref(release_capability.ref_id())
                        .with_source_ref(auth_residual.source_ref().clone()),
                    )
                    .map_err(|diagnostics| {
                        format!(
                            "kernel test M9 designated release witness: {:?}",
                            diagnostics.primary().kind()
                        )
                    })?;
                remote_release_issued = true;
            }
        }
        if remote_dependency.is_some() && !remote_release_issued {
            return Err(
                "kernel test remote release source owner lacks admitted owner lineage".to_string(),
            );
        }
        let (membership, contract_capability, contract_witness) = final_lineage
            .ok_or_else(|| "kernel test owner admission lacks final lineage".to_string())?;
        let discharge = M9FiniteRefinementChecker::default()
            .discharge_candidate(
                checked,
                M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 finite refinement: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        m9.admit_runtime(
            base,
            authority,
            M9FinalAdmissionEvidence::from_lineage(
                &membership,
                &contract_capability,
                &contract_witness,
                discharge,
            ),
        )
        .map(M9RuntimeAdmitted::into_m10_execution_seam)
        .map_err(|diagnostics| {
            format!(
                "kernel test final M9 admission: {:?}",
                diagnostics.primary().kind()
            )
        })
    }

    /// Build a complete M9 admission through the same typed source, authority,
    /// witness, and finite-refinement operations used by the production seam.
    /// This is deliberately test-only: it supplies a real admitted seam to
    /// kernel boundary tests without adding a production source-free seal.
    #[cfg(test)]
    pub(crate) fn test_real_admitted_designated_remote_input_seam_for_kernel(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Result<Self, String> {
        Self::test_real_admitted_designated_seam_for_kernel(
            checked,
            evaluator,
            result,
            dependency_index,
            true,
        )
    }

    #[cfg(test)]
    pub(crate) fn test_real_admitted_owner_and_designated_remote_input_seam_for_kernel(
        checked: &CheckedSurfaceV0,
        operation: &str,
        principal: &str,
        owner_locus: &str,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Result<Self, String> {
        Self::test_real_admitted_owner_kernel_seam(
            checked,
            [(operation, principal, owner_locus)],
            Some((evaluator, result, dependency_index)),
        )
    }

    #[cfg(test)]
    pub(crate) fn test_real_successor_generation_revoking_owner_cap_for_kernel(
        checked: &CheckedSurfaceV0,
        operation: &str,
        principal: &str,
        owner_locus: &str,
        prior: &M9AuthorityGeneration,
    ) -> Result<M9AuthorityGeneration, String> {
        if prior.program_identity() != checked.program_identity().stable_key() {
            return Err("kernel test successor program identity mismatch".to_string());
        }
        let seam = Self::test_real_admitted_owner_seam_for_kernel(
            checked,
            operation,
            principal,
            owner_locus,
        )?;
        let mut publisher = seam
            .into_authority_successor_publisher()
            .ok_or_else(|| "kernel test M9 successor publisher unavailable".to_string())?;
        publisher
            .revoke_owner_capability(operation, principal, owner_locus)
            .map_err(|diagnostics| {
                format!(
                    "kernel test real M9 owner revocation: {:?}",
                    diagnostics.primary().kind()
                )
            })
    }

    #[cfg(test)]
    pub(crate) fn test_real_admitted_multi_owner_seam_for_kernel<const N: usize>(
        checked: &CheckedSurfaceV0,
        owners: [(&str, &str, &str); N],
    ) -> Result<Self, String> {
        Self::test_real_admitted_owner_kernel_seam(checked, owners, None)
    }

    /// The multiple-owner variant is intentionally still one M9 admission:
    /// it gives SYS-2 an authentic pre-profile inventory for checking that a
    /// later revocation retranslates every unrelated owner and release lane.
    #[cfg(test)]
    pub(crate) fn test_real_admitted_multi_owner_and_designated_remote_input_seam_for_kernel<
        const N: usize,
    >(
        checked: &CheckedSurfaceV0,
        owners: [(&str, &str, &str); N],
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Result<Self, String> {
        Self::test_real_admitted_owner_kernel_seam(
            checked,
            owners,
            Some((evaluator, result, dependency_index)),
        )
    }

    /// Build a real admitted evaluator capability without the distinct
    /// producer-side release scope.  SYS-1 uses this only to prove that a
    /// designated evaluator cannot manufacture a remote-input release.
    #[cfg(test)]
    pub(crate) fn test_real_admitted_designated_evaluator_only_seam_for_kernel(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        input_frontier: &str,
    ) -> Result<Self, String> {
        let designated = checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .ok_or_else(|| "kernel test requested an unchecked designated result".to_string())?;
        if designated.trigger().frontier() != Some(input_frontier) {
            return Err("kernel test requested a mismatched designated frontier".to_string());
        }
        if designated.generated_remote_input_dependencies().is_empty() {
            return Err(
                "kernel test designated result lacks a remote input dependency".to_string(),
            );
        }
        Self::test_real_admitted_designated_seam_for_kernel(checked, evaluator, result, 0, false)
    }

    #[cfg(test)]
    fn test_real_admitted_designated_seam_for_kernel(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        issue_remote_input_release: bool,
    ) -> Result<Self, String> {
        let dependency = checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| {
                core.generated_remote_input_dependencies()
                    .get(dependency_index)
            })
            .ok_or_else(|| {
                "kernel test requested an unchecked designated dependency".to_string()
            })?;
        let input_frontier = checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| core.trigger().frontier())
            .ok_or_else(|| {
                "kernel test designated dependency lacks a checked frontier".to_string()
            })?;
        let principal = checked
            .static_environment()
            .principals()
            .first()
            .map(|principal| principal.name())
            .ok_or_else(|| "kernel test source lacks a principal".to_string())?;
        let source_owner = dependency.source_owner_locus();
        let m9 = M9AdmissionRuntime::default();
        let base = m9
            .admit_source_bound_base(
                checked.clone(),
                test_kernel_m8_admission_for(checked)?,
                test_kernel_m9_envelope_for(checked),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 base admission: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let auth_residual = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
            .ok_or_else(|| "kernel test source lacks AuthDeferred".to_string())?;
        let epoch = "epoch1";
        let incarnation = format!("incarnation:{principal}:{source_owner}:{epoch}");
        let mut authority = base.authority_runtime();
        let attestation = authority
            .issue_membership_attestation(
                principal,
                source_owner,
                epoch,
                incarnation.clone(),
                auth_residual.name(),
                auth_residual.source_ref().clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 membership attestation: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let membership = authority
            .authenticate_membership(
                M9MembershipRequest::new(principal, source_owner, epoch)
                    .with_incarnation(incarnation)
                    .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                    .with_issued_provider_attestation(attestation),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 membership: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let contract_capability = authority
            .authorize_capability(
                M9CapabilityGrantRequest::new("kernel-test-contract-capability")
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::contract_update(
                        checked.program_identity().module(),
                        format!("membership-authority/{}", auth_residual.name()),
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 contract capability: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let contract_witness = authority
            .materialize_witness(
                M9WitnessRequest::new("kernel-test-contract-witness")
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(contract_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 contract witness: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        // The evaluator is a semantic locus in its own right.  Its
        // membership cannot be borrowed from the source-owner release path.
        let evaluator_epoch = "epoch-evaluator-1";
        let evaluator_incarnation =
            format!("incarnation:{principal}:{evaluator}:{evaluator_epoch}");
        let evaluator_attestation = authority
            .issue_membership_attestation(
                principal,
                evaluator,
                evaluator_epoch,
                evaluator_incarnation.clone(),
                auth_residual.name(),
                auth_residual.source_ref().clone(),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 evaluator membership attestation: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let evaluator_membership = authority
            .authenticate_membership(
                M9MembershipRequest::new(principal, evaluator, evaluator_epoch)
                    .with_incarnation(evaluator_incarnation)
                    .with_auth_residual(auth_residual.name(), auth_residual.source_ref().clone())
                    .with_issued_provider_attestation(evaluator_attestation),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 evaluator membership: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let evaluation_capability = authority
            .authorize_capability(
                M9CapabilityGrantRequest::new(format!(
                    "kernel-test-designated-evaluation:{evaluator}:{result}:{input_frontier}"
                ))
                .with_membership_ref(evaluator_membership.ref_id())
                .with_scope(M9CapabilityScope::designated_evaluation(
                    evaluator,
                    result,
                    input_frontier,
                ))
                .with_lineage_epoch(evaluator_membership.epoch())
                .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 designated capability: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        let _evaluation_witness = authority
            .materialize_witness(
                M9WitnessRequest::new(format!(
                    "kernel-test-designated-evaluation-witness:{evaluator}:{result}:{input_frontier}"
                ))
                .with_membership_ref(evaluator_membership.ref_id())
                .with_capability_ref(evaluation_capability.ref_id())
                .with_source_ref(auth_residual.source_ref().clone()),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 designated witness: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        if issue_remote_input_release {
            let read = dependency.typed_state_read();
            let release_label = canonical_designated_remote_input_release_label(
                read.namespace(),
                read.index(),
                read.field(),
                source_owner,
                input_frontier,
            );
            let release_capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "cap:attack:{source_owner}:{principal}:{epoch}"
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::designated_remote_input_release(
                        source_owner,
                        evaluator,
                        result,
                        dependency_index,
                        input_frontier,
                        release_label,
                        M9_REMOTE_INPUT_VISIBILITY_RESTRICTED_REDACTED,
                    ))
                    .with_lineage_epoch(membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 designated release capability: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let _release_witness = authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "witness:attack:{source_owner}:{principal}:{epoch}"
                    ))
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(release_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 designated release witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
        }
        // A designated-result consumer is an independent admitted locus.  Its
        // membership, capability, and witness are issued by the same M9
        // authority runtime as evaluator/release authority; the delivery
        // endpoint cannot manufacture any of the three facts.
        if let Some(consumer_evaluation) = checked.evaluations().iter().find(|evaluation| {
            evaluation
                .designated_result_consumer_core()
                .is_some_and(|core| core.evaluator() == evaluator && core.result() == result)
        }) {
            let consumer_core = consumer_evaluation
                .designated_result_consumer_core()
                .expect("selected designated consumer retains Core");
            let consumer_locus = consumer_core.consumer_locus();
            let consumer_epoch = "epoch-consumer-1";
            let consumer_incarnation =
                format!("incarnation:{principal}:{consumer_locus}:{consumer_epoch}");
            let consumer_attestation = authority
                .issue_membership_attestation(
                    principal,
                    consumer_locus,
                    consumer_epoch,
                    consumer_incarnation.clone(),
                    auth_residual.name(),
                    auth_residual.source_ref().clone(),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 consumer membership attestation: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let consumer_membership = authority
                .authenticate_membership(
                    M9MembershipRequest::new(principal, consumer_locus, consumer_epoch)
                        .with_incarnation(consumer_incarnation)
                        .with_auth_residual(
                            auth_residual.name(),
                            auth_residual.source_ref().clone(),
                        )
                        .with_issued_provider_attestation(consumer_attestation),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 consumer membership: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let consumer_capability = authority
                .authorize_capability(
                    M9CapabilityGrantRequest::new(format!(
                        "kernel-test-designated-consumption:{consumer_locus}:{evaluator}:{result}"
                    ))
                    .with_membership_ref(consumer_membership.ref_id())
                    .with_scope(M9CapabilityScope::designated_consumption(
                        consumer_locus,
                        format!("{evaluator}.{result}"),
                        consumer_core.result_version().value(),
                    ))
                    .with_lineage_epoch(consumer_membership.epoch())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 designated consumer capability: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
            let _consumer_witness = authority
                .materialize_witness(
                    M9WitnessRequest::new(format!(
                        "kernel-test-designated-consumption-witness:{consumer_locus}:{evaluator}:{result}"
                    ))
                    .with_membership_ref(consumer_membership.ref_id())
                    .with_capability_ref(consumer_capability.ref_id())
                    .with_source_ref(auth_residual.source_ref().clone()),
                )
                .map_err(|diagnostics| {
                    format!(
                        "kernel test M9 designated consumer witness: {:?}",
                        diagnostics.primary().kind()
                    )
                })?;
        }
        let discharge = M9FiniteRefinementChecker::default()
            .discharge_candidate(
                checked,
                M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
            )
            .map_err(|diagnostics| {
                format!(
                    "kernel test M9 finite refinement: {:?}",
                    diagnostics.primary().kind()
                )
            })?;
        m9.admit_runtime(
            base,
            authority,
            M9FinalAdmissionEvidence::from_lineage(
                &membership,
                &contract_capability,
                &contract_witness,
                discharge,
            ),
        )
        .map(M9RuntimeAdmitted::into_m10_execution_seam)
        .map_err(|diagnostics| {
            format!(
                "kernel test final M9 admission: {:?}",
                diagnostics.primary().kind()
            )
        })
    }

    /// Split the final M9 seam for the live semantic kernel.  The immutable
    /// initial generation and its sole M9-owned successor publisher travel
    /// together; callers cannot synthesize a successor from M8 state.
    pub(crate) fn into_kernel_parts(
        self,
    ) -> Option<(
        M8RuntimeInstance,
        M8AuthorityState,
        M9AuthorityGeneration,
        M9AuthoritySuccessorPublisher,
    )> {
        let Self {
            instance,
            authority_state,
            authority_generation,
            authority_successor,
            ..
        } = self;
        authority_successor
            .map(|publisher| (instance, authority_state, authority_generation, publisher))
    }

    pub(crate) fn into_parts(self) -> (M8RuntimeInstance, M8AuthorityState) {
        (self.instance, self.authority_state)
    }

    pub(crate) fn canonical_m9_snapshot_projection(&self) -> &str {
        &self.authority_snapshot_projection
    }

    pub(crate) fn canonical_m9_membership_projection(&self) -> &str {
        &self.authority_membership_projection
    }

    pub(crate) fn canonical_m9_grant_projection(&self) -> &str {
        &self.authority_grant_projection
    }

    pub(crate) fn owner_authority_use(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M8AuthorityUse> {
        self.owner_uses
            .get(&(
                evaluation.to_string(),
                principal.to_string(),
                owner_locus.to_string(),
            ))
            .cloned()
    }

    /// M10's patch route can only consume the already admitted M9
    /// ContractUpdate lineage.  This is an explicit execution use, not an
    /// automatic contract attachment or a provider/minting API.
    pub(crate) fn patch_authority_use(
        &self,
        patch_program: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M8PatchAuthorityUse> {
        self.patch_uses
            .get(&(
                patch_program.to_string(),
                principal.to_string(),
                owner_locus.to_string(),
            ))
            .cloned()
    }

    pub(crate) fn relation_authority_use(
        &self,
        relation: &str,
        transition: &str,
    ) -> Option<M8RelationAuthorityUse> {
        self.relation_uses
            .get(&(relation.to_string(), transition.to_string()))
            .cloned()
    }

    pub(crate) fn designated_evaluation_authority_use(
        &self,
        evaluator: &str,
        result: &str,
    ) -> Option<M8DesignatedAuthorityUse> {
        self.designated_evaluation_uses
            .get(&(evaluator.to_string(), result.to_string()))
            .cloned()
    }

    pub(crate) fn designated_consumption_authority_use(
        &self,
        consumer: &str,
        value_name: &str,
    ) -> Option<M8DesignatedAuthorityUse> {
        self.designated_consumption_uses
            .get(&(consumer.to_string(), value_name.to_string()))
            .cloned()
    }

    pub(crate) fn observer_authority(
        &self,
        observer_principal: &str,
    ) -> Option<M8ObserverAuthorityGrant> {
        self.observer_authorities.get(observer_principal).cloned()
    }

    pub(crate) fn translation_refs(
        &self,
        capability_ref: &str,
    ) -> Option<&(String, String, String)> {
        self.translation_refs.get(capability_ref)
    }

    /// Read a M9-issued owner lineage for the internal semantic runtime
    /// kernel.  The lookup cannot issue, refresh, or transform authority.
    pub(crate) fn kernel_owner_lineage(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M9KernelOwnerLineage> {
        self.kernel_owner_lineages
            .get(&(
                evaluation.to_string(),
                principal.to_string(),
                owner_locus.to_string(),
            ))
            .cloned()
    }

    /// Read the active producer-side release lineage for one exact checked
    /// designated dependency.  The evaluator's decision capability cannot
    /// satisfy this lookup.
    pub(crate) fn kernel_designated_remote_input_lineage(
        &self,
        producer_locus: &str,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        input_frontier: &str,
    ) -> Option<M9KernelDesignatedRemoteInputLineage> {
        self.kernel_designated_remote_input_lineages
            .get(&(
                producer_locus.to_string(),
                evaluator.to_string(),
                result.to_string(),
                dependency_index,
                input_frontier.to_string(),
            ))
            .cloned()
    }
}

impl M9AuthorityGeneration {
    /// A kernel-reference profile has no M8 authority inventory.  It exists
    /// only for deterministic crate tests that do not enter an admitted M9
    /// execution seam.
    pub(crate) fn reference(program_identity: impl Into<String>) -> Self {
        Self {
            program_identity: program_identity.into(),
            generation: 0,
            generation_ref: "m9-authority-generation:reference:00000000000000000000".to_string(),
            authority_state: M8AuthorityState::new(),
            owner_uses: BTreeMap::new(),
            designated_evaluation_uses: BTreeMap::new(),
            designated_consumption_uses: BTreeMap::new(),
            kernel_owner_lineages: BTreeMap::new(),
            revoked_owner_capabilities: BTreeSet::new(),
            revoked_designated_consumption_capabilities: BTreeSet::new(),
            kernel_designated_remote_input_lineages: BTreeMap::new(),
            designated_consumer_failures: BTreeMap::new(),
            designated_consumer_witness_retirements: BTreeSet::new(),
            designated_source_release_failures: BTreeMap::new(),
            designated_consumer_validation_occurrences: BTreeMap::new(),
            owner_operation_validation_occurrences: BTreeMap::new(),
            source_release_validation_occurrences: BTreeMap::new(),
        }
    }

    pub(crate) fn program_identity(&self) -> &str {
        &self.program_identity
    }

    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn generation_ref(&self) -> &str {
        &self.generation_ref
    }

    pub(crate) fn authority_state(&self) -> M8AuthorityState {
        self.authority_state.clone()
    }

    /// Return observer-safe handles for the authority facts which M9 already
    /// admitted.  SYS-4 can validate one of these handles, but it cannot use
    /// this inspection to manufacture a credential or a successor.
    pub(crate) fn sealed_inspection(&self) -> M9AuthorityInspection {
        let consumers = self
            .designated_consumption_uses
            .iter()
            .map(|((consumer, value), use_)| {
                (
                    (value.clone(), consumer.clone()),
                    M9DesignatedConsumerLineage {
                        consumer_locus: consumer.clone(),
                        opaque_lineage_ref: m9_opaque_ref(&format!(
                            "consumer:{consumer}:{value}:{:?}:{:?}:{:?}",
                            use_.membership_ref(),
                            use_.capability_ref(),
                            use_.witness_ref(),
                        )),
                    },
                )
            })
            .collect();
        let source_releases = self
            .kernel_designated_remote_input_lineages
            .iter()
            .map(|((source, evaluator, result, dependency, frontier), lineage)| {
                (
                    (
                        evaluator.clone(),
                        result.clone(),
                        source.clone(),
                        *dependency,
                        frontier.clone(),
                    ),
                    M9DesignatedSourceReleaseLineage {
                        opaque_lineage_ref: m9_opaque_ref(&format!(
                            "release:{source}:{evaluator}:{result}:{dependency}:{frontier}:{}:{}:{}:{}",
                            self.generation_ref,
                            lineage.membership_ref(),
                            lineage.capability_ref(),
                            lineage.witness_ref(),
                        )),
                    },
                )
            })
            .collect();
        M9AuthorityInspection {
            generation: M9SealedGeneration {
                generation: self.generation,
                generation_ref: self.generation_ref.clone(),
                m9_produced: true,
            },
            consumers,
            source_releases,
            designated_consumer_validation_occurrences: self
                .designated_consumer_validation_occurrences
                .clone(),
            owner_operation_validation_occurrences: self
                .owner_operation_validation_occurrences
                .clone(),
            source_release_validation_occurrences: self
                .source_release_validation_occurrences
                .clone(),
        }
    }

    pub(crate) fn validate_designated_consumer(
        &mut self,
        value_name: &str,
        consumer: &str,
        request_id: &str,
        semantic_identity: &str,
    ) -> Result<M9CacheValidationInspection, M9SealedFailureInspection> {
        *self
            .designated_consumer_validation_occurrences
            .entry((
                value_name.to_string(),
                consumer.to_string(),
                semantic_identity.to_string(),
            ))
            .or_default() += 1;
        let inspection = self.sealed_inspection();
        let lineage = inspection
            .designated_consumer_lineage(value_name, consumer)
            .cloned()
            .unwrap_or_else(|| M9DesignatedConsumerLineage {
                consumer_locus: consumer.to_string(),
                opaque_lineage_ref: m9_opaque_ref(&format!(
                    "absent-consumer:{value_name}:{consumer}:{}",
                    self.generation_ref
                )),
            });
        let failure = self
            .designated_consumer_failures
            .get(&(consumer.to_string(), value_name.to_string()))
            .copied()
            .or_else(|| {
                self.designated_consumption_capability_is_revoked(consumer, value_name)
                    .then_some(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })
            .or_else(|| {
                self.designated_consumption_authority_use(consumer, value_name)
                    .is_none()
                    .then_some(M9AdmissionErrorKind::InvalidCapabilityLineage)
            });
        if let Some(admission_error_kind) = failure {
            return Err(M9SealedFailureInspection {
                admission_error_kind,
                installed_generation: inspection.generation,
                consumer_lineage: lineage,
                request_id: request_id.to_string(),
                semantic_identity: semantic_identity.to_string(),
                consumer_locus: consumer.to_string(),
            });
        }
        Ok(M9CacheValidationInspection {
            generation: inspection.generation,
            consumer_lineage: lineage,
            semantic_identity: semantic_identity.to_string(),
            consumer_locus: consumer.to_string(),
            occurrence_id: format!(
                "m9-cache-validation:{}:{}:{}",
                self.generation, request_id, consumer
            ),
        })
    }

    pub(crate) fn validate_designated_source_release(
        &mut self,
        operation_id: &str,
        evaluator: &str,
        result: &str,
        source_locus: &str,
        dependency_index: usize,
        input_frontier: &str,
        expected: &M9DesignatedSourceReleaseLineage,
        request_id: &str,
    ) -> Option<M9SourceReleaseValidationInspection> {
        *self
            .source_release_validation_occurrences
            .entry((
                operation_id.to_string(),
                source_locus.to_string(),
                request_id.to_string(),
            ))
            .or_default() += 1;
        let inspection = self.sealed_inspection();
        let key = (
            evaluator.to_string(),
            result.to_string(),
            source_locus.to_string(),
            dependency_index,
            input_frontier.to_string(),
        );
        let lineage = inspection.source_releases.get(&key)?.clone();
        if &lineage != expected || self.designated_source_release_failures.contains_key(&key) {
            return None;
        }
        Some(M9SourceReleaseValidationInspection {
            generation: inspection.generation,
            lineage,
            occurrence_id: format!(
                "m9-source-release-validation:{}:{}:{}",
                self.generation, request_id, source_locus
            ),
        })
    }

    pub(crate) fn designated_consumer_witness_is_retired(
        &self,
        consumer: &str,
        value_name: &str,
    ) -> bool {
        self.designated_consumer_witness_retirements
            .contains(&(consumer.to_string(), value_name.to_string()))
    }

    pub(crate) fn owner_lineage_ref(&self, operation: &str, owner_locus: &str) -> Option<String> {
        self.owner_authority_for_operation(operation, owner_locus)
            .and_then(|(principal, _)| {
                self.kernel_owner_lineages
                    .get(&(operation.to_string(), principal, owner_locus.to_string()))
                    .map(|lineage| {
                        m9_opaque_ref(&format!(
                            "owner:{}:{}:{}:{}:{}:{}",
                            operation,
                            owner_locus,
                            lineage.membership_ref(),
                            lineage.capability_ref(),
                            lineage.witness_ref(),
                            self.generation_ref,
                        ))
                    })
            })
    }

    pub(crate) fn transition_inspection(
        &self,
        prior: &M9AuthorityInspection,
        transition_kind: M9AuthorityTransitionKind,
        consumer_lineage: Option<M9DesignatedConsumerLineage>,
        source_release_lineage: Option<M9DesignatedSourceReleaseLineage>,
    ) -> M9SealedTransitionInspection {
        M9SealedTransitionInspection {
            transition_kind,
            prior_generation: prior.generation.clone(),
            successor_generation: self.sealed_inspection().generation,
            consumer_lineage,
            source_release_lineage,
        }
    }

    /// A successor may add tombstones but may never remove one already
    /// published by an earlier immutable generation.
    pub(crate) fn preserves_tombstones_from(&self, prior: &Self) -> bool {
        prior
            .revoked_owner_capabilities
            .is_subset(&self.revoked_owner_capabilities)
            && prior
                .revoked_designated_consumption_capabilities
                .is_subset(&self.revoked_designated_consumption_capabilities)
            && prior
                .designated_consumer_failures
                .iter()
                .all(|(key, value)| self.designated_consumer_failures.get(key) == Some(value))
            && prior
                .designated_source_release_failures
                .iter()
                .all(|(key, value)| self.designated_source_release_failures.get(key) == Some(value))
            && prior
                .designated_consumer_witness_retirements
                .is_subset(&self.designated_consumer_witness_retirements)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn with_successor_generation_and_revoked_owner_lineage(
        mut self,
        previous: &Self,
        operation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<Self> {
        let key = (
            operation.to_string(),
            principal.to_string(),
            owner_locus.to_string(),
        );
        let prior_lineage = previous.kernel_owner_lineages.get(&key)?.clone();
        let prior_use = previous.owner_uses.get(&key)?.clone();
        if previous.revoked_owner_capabilities.contains(&key) {
            return None;
        }
        self.generation = previous.generation.checked_add(1)?;
        self.generation_ref = format!("m9-authority-generation:{:020}", self.generation);
        self.kernel_owner_lineages
            .insert(key.clone(), prior_lineage);
        self.owner_uses.insert(key.clone(), prior_use);
        self.revoked_owner_capabilities = previous.revoked_owner_capabilities.clone();
        self.revoked_owner_capabilities.insert(key);
        self.designated_evaluation_uses = previous.designated_evaluation_uses.clone();
        self.designated_consumption_uses = previous.designated_consumption_uses.clone();
        self.revoked_designated_consumption_capabilities =
            previous.revoked_designated_consumption_capabilities.clone();
        self.kernel_designated_remote_input_lineages =
            previous.kernel_designated_remote_input_lineages.clone();
        self.designated_consumer_failures = previous.designated_consumer_failures.clone();
        self.designated_consumer_witness_retirements =
            previous.designated_consumer_witness_retirements.clone();
        self.designated_source_release_failures =
            previous.designated_source_release_failures.clone();
        self.designated_consumer_validation_occurrences =
            previous.designated_consumer_validation_occurrences.clone();
        self.owner_operation_validation_occurrences =
            previous.owner_operation_validation_occurrences.clone();
        self.source_release_validation_occurrences =
            previous.source_release_validation_occurrences.clone();
        Some(self)
    }

    pub(crate) fn owner_capability_is_revoked(
        &self,
        operation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> bool {
        self.revoked_owner_capabilities.contains(&(
            operation.to_string(),
            principal.to_string(),
            owner_locus.to_string(),
        ))
    }

    /// Finds the M9-issued authority use for a finite owner operation without
    /// reinterpreting the origin locus as a principal.
    pub(crate) fn owner_authority_for_operation(
        &self,
        operation: &str,
        owner_locus: &str,
    ) -> Option<(String, M8AuthorityUse)> {
        self.owner_uses.iter().find_map(
            |((candidate_operation, principal, candidate_owner), authority_use)| {
                (candidate_operation == operation && candidate_owner == owner_locus)
                    .then(|| (principal.clone(), authority_use.clone()))
            },
        )
    }

    /// Validate one owner operation from an exact SYS-4 request.  The
    /// increment is an M9-owned observation; downstream code receives only
    /// the already admitted authority use and cannot manufacture a lineage.
    pub(crate) fn validate_owner_operation(
        &mut self,
        operation: &str,
        owner_locus: &str,
        request_id: &str,
    ) -> Option<(String, M8AuthorityUse)> {
        *self
            .owner_operation_validation_occurrences
            .entry((
                operation.to_string(),
                owner_locus.to_string(),
                request_id.to_string(),
            ))
            .or_default() += 1;
        self.owner_authority_for_operation(operation, owner_locus)
    }

    pub(crate) fn designated_evaluation_authority_use(
        &self,
        evaluator: &str,
        result: &str,
    ) -> Option<M8DesignatedAuthorityUse> {
        self.designated_evaluation_uses
            .get(&(evaluator.to_string(), result.to_string()))
            .cloned()
    }

    pub(crate) fn designated_consumption_authority_use(
        &self,
        consumer: &str,
        value_name: &str,
    ) -> Option<M8DesignatedAuthorityUse> {
        self.designated_consumption_uses
            .get(&(consumer.to_string(), value_name.to_string()))
            .cloned()
    }

    pub(crate) fn designated_consumption_capability_is_revoked(
        &self,
        consumer: &str,
        value_name: &str,
    ) -> bool {
        self.revoked_designated_consumption_capabilities
            .contains(&(consumer.to_string(), value_name.to_string()))
    }

    fn with_successor_generation_and_revoked_designated_consumption(
        mut self,
        previous: &Self,
        consumer: &str,
        value_name: &str,
    ) -> Option<Self> {
        let key = (consumer.to_string(), value_name.to_string());
        let prior_use = previous.designated_consumption_uses.get(&key)?.clone();
        if previous
            .revoked_designated_consumption_capabilities
            .contains(&key)
        {
            return None;
        }
        self.generation = previous.generation.checked_add(1)?;
        self.generation_ref = format!("m9-authority-generation:{:020}", self.generation);
        self.owner_uses = previous.owner_uses.clone();
        self.kernel_owner_lineages = previous.kernel_owner_lineages.clone();
        self.designated_evaluation_uses = previous.designated_evaluation_uses.clone();
        self.designated_consumption_uses = previous.designated_consumption_uses.clone();
        self.designated_consumption_uses
            .insert(key.clone(), prior_use);
        self.revoked_owner_capabilities = previous.revoked_owner_capabilities.clone();
        self.revoked_designated_consumption_capabilities =
            previous.revoked_designated_consumption_capabilities.clone();
        self.revoked_designated_consumption_capabilities.insert(key);
        self.kernel_designated_remote_input_lineages =
            previous.kernel_designated_remote_input_lineages.clone();
        self.designated_consumer_failures = previous.designated_consumer_failures.clone();
        self.designated_consumer_witness_retirements =
            previous.designated_consumer_witness_retirements.clone();
        self.designated_source_release_failures =
            previous.designated_source_release_failures.clone();
        self.designated_consumer_validation_occurrences =
            previous.designated_consumer_validation_occurrences.clone();
        self.owner_operation_validation_occurrences =
            previous.owner_operation_validation_occurrences.clone();
        self.source_release_validation_occurrences =
            previous.source_release_validation_occurrences.clone();
        Some(self)
    }

    fn with_successor_generation_and_source_release_failure(
        mut self,
        previous: &Self,
        evaluator: &str,
        result: &str,
        source_locus: &str,
        dependency_index: usize,
        input_frontier: &str,
    ) -> Option<Self> {
        let key = (
            evaluator.to_string(),
            result.to_string(),
            source_locus.to_string(),
            dependency_index,
            input_frontier.to_string(),
        );
        if previous
            .designated_source_release_failures
            .contains_key(&key)
        {
            return None;
        }
        self.generation = previous.generation.checked_add(1)?;
        self.generation_ref = format!("m9-authority-generation:{:020}", self.generation);
        self.owner_uses = previous.owner_uses.clone();
        self.kernel_owner_lineages = previous.kernel_owner_lineages.clone();
        self.designated_evaluation_uses = previous.designated_evaluation_uses.clone();
        self.designated_consumption_uses = previous.designated_consumption_uses.clone();
        self.revoked_owner_capabilities = previous.revoked_owner_capabilities.clone();
        self.revoked_designated_consumption_capabilities =
            previous.revoked_designated_consumption_capabilities.clone();
        self.kernel_designated_remote_input_lineages =
            previous.kernel_designated_remote_input_lineages.clone();
        self.designated_consumer_failures = previous.designated_consumer_failures.clone();
        self.designated_consumer_witness_retirements =
            previous.designated_consumer_witness_retirements.clone();
        self.designated_source_release_failures =
            previous.designated_source_release_failures.clone();
        self.designated_source_release_failures
            .insert(key, M9AdmissionErrorKind::InvalidCapabilityLineage);
        self.designated_consumer_validation_occurrences =
            previous.designated_consumer_validation_occurrences.clone();
        self.owner_operation_validation_occurrences =
            previous.owner_operation_validation_occurrences.clone();
        self.source_release_validation_occurrences =
            previous.source_release_validation_occurrences.clone();
        Some(self)
    }
}

impl M9AuthoritySuccessorPublisher {
    pub(crate) fn current_inspection(&self) -> M9AuthorityInspection {
        self.current.sealed_inspection()
    }

    pub(crate) fn revoke_owner_capability(
        &mut self,
        operation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Result<M9AuthorityGeneration, M9AdmissionDiagnostics> {
        let lineage = self
            .current
            .kernel_owner_lineages
            .get(&(
                operation.to_string(),
                principal.to_string(),
                owner_locus.to_string(),
            ))
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        self.authority_runtime.revoke(
            M9Revocation::capability(lineage.capability_ref())
                .with_witness_ref(lineage.witness_ref()),
        )?;
        let translated = M9RuntimeAdmitted {
            base: self.base.clone(),
            authority_runtime: self.authority_runtime.clone(),
            evidence: self.evidence.clone(),
        }
        .into_m10_execution_seam()
        .initial_authority_generation();
        let successor = translated
            .with_successor_generation_and_revoked_owner_lineage(
                &self.current,
                operation,
                principal,
                owner_locus,
            )
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        self.current = successor.clone();
        Ok(successor)
    }

    /// Revoke the exact checked designated-result consumer scope from this
    /// publisher's currently admitted M9 inventory.  The caller selects only
    /// the checked `(consumer, value)` identity; the capability and witness
    /// reference remain sealed inside M9.
    pub(crate) fn revoke_designated_consumption_capability(
        &mut self,
        consumer: &str,
        value_name: &str,
    ) -> Result<M9AuthorityGeneration, M9AdmissionDiagnostics> {
        let use_ = self
            .current
            .designated_consumption_authority_use(consumer, value_name)
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let (Some(capability_ref), Some(witness_ref)) = (use_.capability_ref(), use_.witness_ref())
        else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        self.authority_runtime
            .revoke(M9Revocation::capability(capability_ref).with_witness_ref(witness_ref))?;
        let translated = M9RuntimeAdmitted {
            base: self.base.clone(),
            authority_runtime: self.authority_runtime.clone(),
            evidence: self.evidence.clone(),
        }
        .into_m10_execution_seam()
        .initial_authority_generation();
        let successor = translated
            .with_successor_generation_and_revoked_designated_consumption(
                &self.current,
                consumer,
                value_name,
            )
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let mut successor = successor;
        successor.designated_consumer_failures.insert(
            (consumer.to_string(), value_name.to_string()),
            M9AdmissionErrorKind::InvalidCapabilityLineage,
        );
        self.current = successor.clone();
        Ok(successor)
    }

    /// Produce a successor through M9 membership retirement. SYS-4 consumes
    /// only the resulting immutable authority generation.
    pub(crate) fn retire_designated_consumption_membership(
        &mut self,
        consumer: &str,
        value_name: &str,
    ) -> Result<M9AuthorityGeneration, M9AdmissionDiagnostics> {
        let use_ = self
            .current
            .designated_consumption_authority_use(consumer, value_name)
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let membership_ref = use_.membership_ref().ok_or_else(|| {
            M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidMembershipLineage)
        })?;
        self.authority_runtime.retire_membership(
            membership_ref,
            format!("sys4-consumer-retire:{consumer}:{value_name}"),
        )?;
        let translated = M9RuntimeAdmitted {
            base: self.base.clone(),
            authority_runtime: self.authority_runtime.clone(),
            evidence: self.evidence.clone(),
        }
        .into_m10_execution_seam()
        .initial_authority_generation();
        let successor = translated
            .with_successor_generation_and_revoked_designated_consumption(
                &self.current,
                consumer,
                value_name,
            )
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let mut successor = successor;
        successor.designated_consumer_failures.insert(
            (consumer.to_string(), value_name.to_string()),
            M9AdmissionErrorKind::InvalidMembershipLineage,
        );
        self.current = successor.clone();
        Ok(successor)
    }

    /// Retire the checked consumption witness without giving SYS-4 authority
    /// to mutate any M9 lineage.
    pub(crate) fn retire_designated_consumption_witness(
        &mut self,
        consumer: &str,
        value_name: &str,
    ) -> Result<M9AuthorityGeneration, M9AdmissionDiagnostics> {
        let use_ = self
            .current
            .designated_consumption_authority_use(consumer, value_name)
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let witness_ref = use_.witness_ref().ok_or_else(|| {
            M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
        })?;
        self.authority_runtime.retire_witness(witness_ref)?;
        let translated = M9RuntimeAdmitted {
            base: self.base.clone(),
            authority_runtime: self.authority_runtime.clone(),
            evidence: self.evidence.clone(),
        }
        .into_m10_execution_seam()
        .initial_authority_generation();
        let successor = translated
            .with_successor_generation_and_revoked_designated_consumption(
                &self.current,
                consumer,
                value_name,
            )
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let mut successor = successor;
        successor.designated_consumer_failures.insert(
            (consumer.to_string(), value_name.to_string()),
            M9AdmissionErrorKind::InvalidCapabilityLineage,
        );
        successor
            .designated_consumer_witness_retirements
            .insert((consumer.to_string(), value_name.to_string()));
        self.current = successor.clone();
        Ok(successor)
    }

    /// Revoke exactly one sealed source-release capability.  The caller can
    /// name only the opaque lineage returned by a prior inspection; M9 still
    /// resolves and revokes the concrete capability/witness internally.
    pub(crate) fn revoke_designated_source_release(
        &mut self,
        wanted: &M9DesignatedSourceReleaseLineage,
    ) -> Result<M9AuthorityGeneration, M9AdmissionDiagnostics> {
        let inspection = self.current.sealed_inspection();
        let ((evaluator, result, source, dependency, frontier), _) = inspection
            .source_releases
            .iter()
            .find(|(_, lineage)| *lineage == wanted)
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        let source_lineage = self
            .current
            .kernel_designated_remote_input_lineages
            .get(&(
                source.clone(),
                evaluator.clone(),
                result.clone(),
                *dependency,
                frontier.clone(),
            ))
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        self.authority_runtime.revoke(
            M9Revocation::capability(source_lineage.capability_ref())
                .with_witness_ref(source_lineage.witness_ref()),
        )?;
        let translated = M9RuntimeAdmitted {
            base: self.base.clone(),
            authority_runtime: self.authority_runtime.clone(),
            evidence: self.evidence.clone(),
        }
        .into_m10_execution_seam()
        .initial_authority_generation();
        let successor = translated
            .with_successor_generation_and_source_release_failure(
                &self.current,
                evaluator,
                result,
                source,
                *dependency,
                frontier,
            )
            .ok_or_else(|| {
                M9AdmissionDiagnostics::one(M9AdmissionErrorKind::InvalidCapabilityLineage)
            })?;
        self.current = successor.clone();
        Ok(successor)
    }
}

impl M9KernelAuthorityView for M9RuntimeExecutionSeam {
    fn kernel_owner_lineage(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M9KernelOwnerLineage> {
        M9RuntimeExecutionSeam::kernel_owner_lineage(self, evaluation, principal, owner_locus)
    }

    fn owner_authority_use(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M8AuthorityUse> {
        M9RuntimeExecutionSeam::owner_authority_use(self, evaluation, principal, owner_locus)
    }

    fn kernel_designated_remote_input_lineage(
        &self,
        producer_locus: &str,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        input_frontier: &str,
    ) -> Option<M9KernelDesignatedRemoteInputLineage> {
        M9RuntimeExecutionSeam::kernel_designated_remote_input_lineage(
            self,
            producer_locus,
            evaluator,
            result,
            dependency_index,
            input_frontier,
        )
    }
}

impl M9KernelAuthorityView for M9AuthorityGeneration {
    fn kernel_owner_lineage(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M9KernelOwnerLineage> {
        self.kernel_owner_lineages
            .get(&(
                evaluation.to_string(),
                principal.to_string(),
                owner_locus.to_string(),
            ))
            .cloned()
    }

    fn owner_authority_use(
        &self,
        evaluation: &str,
        principal: &str,
        owner_locus: &str,
    ) -> Option<M8AuthorityUse> {
        self.owner_uses
            .get(&(
                evaluation.to_string(),
                principal.to_string(),
                owner_locus.to_string(),
            ))
            .cloned()
    }

    fn kernel_designated_remote_input_lineage(
        &self,
        producer_locus: &str,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        input_frontier: &str,
    ) -> Option<M9KernelDesignatedRemoteInputLineage> {
        self.kernel_designated_remote_input_lineages
            .get(&(
                producer_locus.to_string(),
                evaluator.to_string(),
                result.to_string(),
                dependency_index,
                input_frontier.to_string(),
            ))
            .cloned()
    }
}

#[cfg(test)]
fn test_kernel_m8_admission_for(checked: &CheckedSurfaceV0) -> Result<M8RuntimeAdmission, String> {
    let mut admission = M8RuntimeAdmission::new(checked.program_identity().clone());
    for residual in checked.residual_obligations().entries() {
        match residual.kind() {
            ResidualObligationKind::Visibility => {
                admission = admission.with_evidence(M8AdmissionEvidence::RelationVisibility {
                    relation: residual.name().to_string(),
                    label: EvidenceSecurityLabel::new("relation:restricted")
                        .with_class(M8SecurityClass::Restricted),
                    redaction: EvidenceRedaction::new("relation-redacted"),
                    source_ref: residual.source_ref().clone(),
                });
            }
            ResidualObligationKind::RelationLifetime => {
                let relation = checked
                    .relation(residual.name())
                    .and_then(|evaluation| evaluation.relation_core())
                    .ok_or_else(|| {
                        "kernel test relation lifetime lacks checked Core".to_string()
                    })?;
                let frontier = relation
                    .binding_frontier()
                    .as_slice()
                    .first()
                    .ok_or_else(|| "kernel test relation lifetime lacks frontier".to_string())?
                    .as_str()
                    .to_string();
                admission = admission.with_evidence(M8AdmissionEvidence::RelationLifetime {
                    relation: residual.name().to_string(),
                    live_lease: format!("kernel-test-lease:{}", residual.name()),
                    binding_frontier: frontier,
                    source_ref: residual.source_ref().clone(),
                });
            }
            ResidualObligationKind::FallbackValidity => {
                let relation = checked
                    .relation(residual.name())
                    .and_then(|evaluation| evaluation.relation_core())
                    .ok_or_else(|| "kernel test fallback lacks checked Core".to_string())?;
                admission =
                    admission.with_evidence(M8AdmissionEvidence::RelationFallbackValidity {
                        relation: residual.name().to_string(),
                        primary_epoch: relation.primary().epoch().to_string(),
                        fallback_epoch: relation.fallback().epoch().to_string(),
                        source_ref: residual.source_ref().clone(),
                    });
            }
            ResidualObligationKind::ValueVisibilityRedaction => {
                admission =
                    admission.with_evidence(M8AdmissionEvidence::ValueVisibilityRedaction {
                        value: residual.name().to_string(),
                        label: EvidenceSecurityLabel::new("value:restricted")
                            .with_class(M8SecurityClass::Restricted),
                        redaction: EvidenceRedaction::new("value-redacted"),
                        source_ref: residual.source_ref().clone(),
                    });
            }
            ResidualObligationKind::AuthDeferred | ResidualObligationKind::VerifyDeferred => {}
        }
    }
    Ok(admission)
}

#[cfg(test)]
fn test_kernel_m9_envelope_for(checked: &CheckedSurfaceV0) -> M9AdmissionEnvelope {
    let mut envelope =
        M9AdmissionEnvelope::for_checked_identity(checked.program_identity().clone())
            .with_original_source_artifact(M9SourceArtifact::from_checked_surface(checked));
    for residual in checked.residual_obligations().entries() {
        let (binding, contract) = match residual.kind() {
            ResidualObligationKind::AuthDeferred => (
                M9ResidualBinding::auth_deferred(residual.name()),
                format!("membership-authority/{}", residual.name()),
            ),
            ResidualObligationKind::VerifyDeferred => (
                M9ResidualBinding::verify_deferred(residual.name()),
                "finite-refinement/MembershipAuth".to_string(),
            ),
            _ => continue,
        };
        envelope = envelope.with_residual_binding(
            binding
                .with_source_ref(residual.source_ref().clone())
                .with_module_contract(checked.program_identity().module(), contract),
        );
    }
    envelope
}

impl std::fmt::Debug for M9RuntimeAdmitted {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9RuntimeAdmitted")
            .field("program_identity", &self.base.program_identity)
            .field("runtime_admitted", &true)
            .finish()
    }
}

impl M9RuntimeAdmitted {
    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.base.program_identity
    }

    pub fn final_evidence(&self) -> &M9FinalAdmissionEvidence {
        &self.evidence
    }

    pub const fn is_runtime_admitted(&self) -> bool {
        true
    }

    pub const fn has_runtime_success(&self) -> bool {
        true
    }

    pub const fn exposes_raw_m8_instance(&self) -> bool {
        false
    }

    fn m8_payload_snapshot(&self) -> M9M8PayloadSnapshot {
        self.base.m8_payload_snapshot()
    }

    pub fn ordered_source_to_core_map(&self) -> &[CheckedSourceMapEntry] {
        &self.base.ordered_source_to_core_map
    }

    pub const fn plan_count(&self) -> usize {
        self.base.plan_count
    }

    /// Reserved for M10 composition.  The result is crate-private so the
    /// resolved M9 wrapper, not a public constructor, remains the route into
    /// owner/relation/designated execution.
    /// Materialize the M8 execution plan and only the active owner-evaluation
    /// lineage already sealed in this M9 admission.  ContractUpdate records
    /// remain evidence for the final M9 judgment; this bridge neither applies
    /// an update nor exposes a provider or minting path.
    pub(crate) fn into_m10_execution_seam(self) -> M9M10ExecutionSeam {
        let M9RuntimeAdmitted {
            base,
            authority_runtime,
            evidence,
        } = self;
        let snapshot = authority_runtime.authority_snapshot();
        let authority_snapshot_projection = authority_runtime.canonical_snapshot_projection();
        let authority_membership_projection = authority_runtime.canonical_membership_projection();
        let authority_grant_projection = authority_runtime.canonical_grant_projection();
        let mut authority_state = M8AuthorityState::new();
        for membership in snapshot.memberships.values().filter(|membership| {
            membership.active
                && snapshot
                    .current_memberships
                    .get(&(membership.principal.clone(), membership.locus.clone()))
                    == Some(&membership.reference)
        }) {
            authority_state = authority_state.with_membership_record(
                M8MembershipRecord::already_admitted(membership.reference.clone())
                    .with_principal(membership.principal.clone())
                    .with_locus(membership.locus.clone())
                    .with_epoch(membership.epoch.clone()),
            );
        }

        let mut owner_uses = BTreeMap::new();
        let mut patch_uses = BTreeMap::new();
        let mut relation_uses = BTreeMap::new();
        let mut designated_evaluation_uses = BTreeMap::new();
        let mut designated_consumption_uses = BTreeMap::new();
        let mut observer_authorities = BTreeMap::new();
        let mut translation_refs = BTreeMap::new();
        let mut kernel_owner_lineages = BTreeMap::new();
        let mut kernel_designated_remote_input_lineages = BTreeMap::new();
        for capability in snapshot
            .capabilities
            .values()
            .filter(|capability| capability.active)
        {
            let M9CapabilityScope::OwnerEvaluation {
                evaluation,
                owner_locus,
            } = &capability.scope
            else {
                continue;
            };
            let Some(membership) = snapshot.memberships.get(&capability.membership_ref) else {
                continue;
            };
            if !membership.active
                || membership.epoch != capability.lineage_epoch
                || snapshot
                    .current_memberships
                    .get(&(membership.principal.clone(), membership.locus.clone()))
                    != Some(&membership.reference)
            {
                continue;
            }
            authority_state = authority_state.with_capability_grant(
                M8CapabilityGrant::already_admitted(capability.reference.clone())
                    .for_owner_evaluation(evaluation.clone())
                    .with_owner_locus(owner_locus.clone())
                    .with_principal(membership.principal.clone())
                    .with_membership_ref(membership.reference.clone())
                    .with_epoch(membership.epoch.clone()),
            );
            if let Some(witness) = snapshot.witnesses.values().find(|witness| {
                witness.live
                    && witness.membership_ref == membership.reference
                    && witness.capability_ref == capability.reference
            }) {
                authority_state = authority_state.with_witness_record(
                    M8WitnessRecord::live(witness.reference.clone())
                        .for_capability(capability.reference.clone())
                        .with_membership_ref(membership.reference.clone())
                        .with_epoch(membership.epoch.clone()),
                );
                owner_uses.insert(
                    (
                        evaluation.clone(),
                        membership.principal.clone(),
                        owner_locus.clone(),
                    ),
                    M8AuthorityUse::for_principal(membership.principal.clone())
                        .with_membership_ref(membership.reference.clone())
                        .with_capability_ref(capability.reference.clone())
                        .with_witness_ref(witness.reference.clone()),
                );
                kernel_owner_lineages.insert(
                    (
                        evaluation.clone(),
                        membership.principal.clone(),
                        owner_locus.clone(),
                    ),
                    M9KernelOwnerLineage {
                        principal: membership.principal.clone(),
                        owner_locus: owner_locus.clone(),
                        membership_ref: membership.reference.clone(),
                        membership_epoch: membership.epoch.clone(),
                        membership_incarnation: membership.incarnation.clone(),
                        capability_ref: capability.reference.clone(),
                        witness_ref: witness.reference.clone(),
                    },
                );
                translation_refs.insert(
                    capability.reference.clone(),
                    (
                        membership.reference.clone(),
                        capability.reference.clone(),
                        witness.reference.clone(),
                    ),
                );
            }
        }
        for capability in snapshot
            .capabilities
            .values()
            .filter(|capability| capability.active)
        {
            let M9CapabilityScope::ContractUpdate { module, .. } = &capability.scope else {
                continue;
            };
            let Some(membership) = snapshot.memberships.get(&capability.membership_ref) else {
                continue;
            };
            if !membership.active
                || membership.epoch != capability.lineage_epoch
                || snapshot
                    .current_memberships
                    .get(&(membership.principal.clone(), membership.locus.clone()))
                    != Some(&membership.reference)
            {
                continue;
            }
            let Some(witness) = snapshot.witnesses.values().find(|witness| {
                witness.live
                    && witness.membership_ref == membership.reference
                    && witness.capability_ref == capability.reference
            }) else {
                continue;
            };
            authority_state = authority_state.with_capability_grant(
                M8CapabilityGrant::already_admitted(capability.reference.clone())
                    .for_patch_activation(module.clone())
                    .with_owner_locus(membership.locus.clone())
                    .with_principal(membership.principal.clone())
                    .with_membership_ref(membership.reference.clone())
                    .with_epoch(membership.epoch.clone()),
            );
            authority_state = authority_state.with_witness_record(
                M8WitnessRecord::live(witness.reference.clone())
                    .for_capability(capability.reference.clone())
                    .with_membership_ref(membership.reference.clone())
                    .with_epoch(membership.epoch.clone()),
            );
            patch_uses.insert(
                (
                    module.clone(),
                    membership.principal.clone(),
                    membership.locus.clone(),
                ),
                M8PatchAuthorityUse::for_patch_program(module.clone())
                    .with_owner_locus(membership.locus.clone())
                    .with_principal(membership.principal.clone())
                    .with_membership_ref(membership.reference.clone())
                    .with_capability_ref(capability.reference.clone())
                    .with_witness_ref(witness.reference.clone()),
            );
            translation_refs.insert(
                capability.reference.clone(),
                (
                    membership.reference.clone(),
                    capability.reference.clone(),
                    witness.reference.clone(),
                ),
            );
        }
        for capability in snapshot
            .capabilities
            .values()
            .filter(|capability| capability.active)
        {
            let Some(membership) = snapshot.memberships.get(&capability.membership_ref) else {
                continue;
            };
            let Some(witness) = snapshot.witnesses.values().find(|witness| {
                witness.live
                    && witness.membership_ref == membership.reference
                    && witness.capability_ref == capability.reference
            }) else {
                continue;
            };
            if !membership.active
                || membership.epoch != capability.lineage_epoch
                || snapshot
                    .current_memberships
                    .get(&(membership.principal.clone(), membership.locus.clone()))
                    != Some(&membership.reference)
            {
                continue;
            }
            match &capability.scope {
                M9CapabilityScope::RelationTransition {
                    relation,
                    transition,
                    owner_locus,
                    binding_frontier: _,
                } => {
                    // M9's membership epoch authenticates the authority
                    // lineage.  M8's relation binding epoch is a separate
                    // semantic clock, so translate the admitted transition
                    // into the exact M8 epoch that it is allowed to serve.
                    let execution_binding_epoch = match transition.as_str() {
                        "invalidate_primary" => "binding_epoch:1".to_string(),
                        "reacquire_primary" => "binding_epoch:2".to_string(),
                        other => format!("m9-transition:{other}:{}", capability.reference),
                    };
                    authority_state = authority_state.with_capability_grant(
                        M8CapabilityGrant::already_admitted(capability.reference.clone())
                            .for_relation_transition(relation.clone(), transition.clone())
                            .with_owner_locus(owner_locus.clone())
                            .with_principal(membership.principal.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_epoch(membership.epoch.clone())
                            .with_binding_epoch(execution_binding_epoch.clone()),
                    );
                    authority_state = authority_state.with_witness_record(
                        M8WitnessRecord::live(witness.reference.clone())
                            .for_capability(capability.reference.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_epoch(membership.epoch.clone()),
                    );
                    relation_uses.insert(
                        (relation.clone(), transition.clone()),
                        M8RelationAuthorityUse::for_relation(relation.clone())
                            .with_owner_locus(owner_locus.clone())
                            .with_transition(transition.clone())
                            .with_principal(membership.principal.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_capability_ref(capability.reference.clone())
                            .with_membership_epoch(membership.epoch.clone())
                            .with_binding_epoch(execution_binding_epoch)
                            .with_witness_ref(witness.reference.clone())
                            .with_witness_epoch(membership.epoch.clone()),
                    );
                    translation_refs.insert(
                        capability.reference.clone(),
                        (
                            membership.reference.clone(),
                            capability.reference.clone(),
                            witness.reference.clone(),
                        ),
                    );
                }
                M9CapabilityScope::DesignatedEvaluation {
                    evaluator,
                    result,
                    input_frontier,
                } => {
                    authority_state = authority_state.with_capability_grant(
                        M8CapabilityGrant::already_admitted(capability.reference.clone())
                            .for_designated_evaluation(evaluator.clone(), result.clone())
                            .with_evaluator_locus(evaluator.clone())
                            .with_principal(membership.principal.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_input_frontier(input_frontier.clone())
                            .with_epoch(membership.epoch.clone()),
                    );
                    authority_state = authority_state.with_witness_record(
                        M8WitnessRecord::live(witness.reference.clone())
                            .for_capability(capability.reference.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_epoch(membership.epoch.clone()),
                    );
                    designated_evaluation_uses.insert(
                        (evaluator.clone(), result.clone()),
                        M8DesignatedAuthorityUse::for_evaluator(evaluator.clone())
                            .with_principal(membership.principal.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_capability_ref(capability.reference.clone())
                            .with_witness_ref(witness.reference.clone()),
                    );
                    translation_refs.insert(
                        capability.reference.clone(),
                        (
                            membership.reference.clone(),
                            capability.reference.clone(),
                            witness.reference.clone(),
                        ),
                    );
                }
                M9CapabilityScope::DesignatedRemoteInputRelease {
                    producer_locus,
                    evaluator,
                    result,
                    dependency_index,
                    input_frontier,
                    release_label,
                    visibility,
                } => {
                    // This scope is producer-side release authority only. It
                    // does not materialize an M8 evaluator capability.
                    if membership.locus != *producer_locus {
                        continue;
                    }
                    kernel_designated_remote_input_lineages.insert(
                        (
                            producer_locus.clone(),
                            evaluator.clone(),
                            result.clone(),
                            *dependency_index,
                            input_frontier.clone(),
                        ),
                        M9KernelDesignatedRemoteInputLineage {
                            principal: membership.principal.clone(),
                            producer_locus: producer_locus.clone(),
                            evaluator: evaluator.clone(),
                            result: result.clone(),
                            dependency_index: *dependency_index,
                            input_frontier: input_frontier.clone(),
                            release_label: release_label.clone(),
                            visibility: visibility.clone(),
                            membership_ref: membership.reference.clone(),
                            membership_epoch: membership.epoch.clone(),
                            membership_incarnation: membership.incarnation.clone(),
                            capability_ref: capability.reference.clone(),
                            witness_ref: witness.reference.clone(),
                        },
                    );
                    translation_refs.insert(
                        capability.reference.clone(),
                        (
                            membership.reference.clone(),
                            capability.reference.clone(),
                            witness.reference.clone(),
                        ),
                    );
                }
                M9CapabilityScope::DesignatedConsumption {
                    consumer,
                    value_name,
                    result_version,
                } => {
                    authority_state = authority_state.with_capability_grant(
                        M8CapabilityGrant::already_admitted(capability.reference.clone())
                            .for_designated_consumption(consumer.clone(), value_name.clone())
                            .with_consumer_locus(consumer.clone())
                            .with_principal(membership.principal.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_result_version(ResultVersion::new(*result_version))
                            .with_epoch(membership.epoch.clone()),
                    );
                    authority_state = authority_state.with_witness_record(
                        M8WitnessRecord::live(witness.reference.clone())
                            .for_capability(capability.reference.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_epoch(membership.epoch.clone()),
                    );
                    designated_consumption_uses.insert(
                        (consumer.clone(), value_name.clone()),
                        M8DesignatedAuthorityUse::for_consumer(consumer.clone())
                            .with_principal(membership.principal.clone())
                            .with_membership_ref(membership.reference.clone())
                            .with_capability_ref(capability.reference.clone())
                            .with_witness_ref(witness.reference.clone()),
                    );
                    translation_refs.insert(
                        capability.reference.clone(),
                        (
                            membership.reference.clone(),
                            capability.reference.clone(),
                            witness.reference.clone(),
                        ),
                    );
                }
                M9CapabilityScope::Observation {
                    observer_principal, ..
                } => {
                    observer_authorities.insert(
                        observer_principal.clone(),
                        M8ObserverAuthorityGrant::already_admitted(capability.reference.clone())
                            .for_principal(observer_principal.clone())
                            .with_epoch(membership.epoch.clone()),
                    );
                    translation_refs.insert(
                        capability.reference.clone(),
                        (
                            membership.reference.clone(),
                            capability.reference.clone(),
                            witness.reference.clone(),
                        ),
                    );
                }
                M9CapabilityScope::OwnerEvaluation { .. }
                | M9CapabilityScope::ContractUpdate { .. } => {}
            }
        }
        let authority_generation = M9AuthorityGeneration {
            program_identity: base.program_identity.stable_key(),
            generation: 0,
            generation_ref: "m9-authority-generation:00000000000000000000".to_string(),
            authority_state: authority_state.clone(),
            owner_uses: owner_uses.clone(),
            designated_evaluation_uses: designated_evaluation_uses.clone(),
            designated_consumption_uses: designated_consumption_uses.clone(),
            kernel_owner_lineages: kernel_owner_lineages.clone(),
            revoked_owner_capabilities: BTreeSet::new(),
            revoked_designated_consumption_capabilities: BTreeSet::new(),
            kernel_designated_remote_input_lineages: kernel_designated_remote_input_lineages
                .clone(),
            designated_consumer_failures: BTreeMap::new(),
            designated_consumer_witness_retirements: BTreeSet::new(),
            designated_source_release_failures: BTreeMap::new(),
            designated_consumer_validation_occurrences: BTreeMap::new(),
            owner_operation_validation_occurrences: BTreeMap::new(),
            source_release_validation_occurrences: BTreeMap::new(),
        };
        let authority_successor = M9AuthoritySuccessorPublisher {
            base: base.clone(),
            evidence,
            authority_runtime: authority_runtime.clone(),
            current: authority_generation.clone(),
        };
        let instance = materialize_m9_resolved_base(base.into_embedded_m8_base());
        M9M10ExecutionSeam {
            instance,
            authority_state,
            authority_snapshot_projection,
            authority_membership_projection,
            authority_grant_projection,
            owner_uses,
            patch_uses,
            relation_uses,
            designated_evaluation_uses,
            designated_consumption_uses,
            observer_authorities,
            translation_refs,
            kernel_owner_lineages,
            kernel_designated_remote_input_lineages,
            authority_generation,
            final_residual_discharge_complete: true,
            authority_successor: Some(authority_successor),
        }
    }

    #[allow(dead_code)] // Compatibility for internal callers before M10.
    pub(crate) fn into_m8_execution_seam(self) -> M8RuntimeInstance {
        self.into_m10_execution_seam().into_parts().0
    }
}

fn final_evidence_matches_base(
    base: &M9AdmittedBase,
    authority_runtime: &M9AuthorityRuntime,
    evidence: &M9FinalAdmissionEvidence,
) -> bool {
    let auth_binding = base.m9_residual_bindings.0.iter().find(|binding| {
        binding.kind == ResidualObligationKind::AuthDeferred
            && binding.module.as_deref() == Some(base.program_identity.module())
            && binding.contract.as_deref()
                == Some(format!("{M9_AUTH_CONTRACT_PREFIX}{}", binding.name).as_str())
    });
    let verify_binding = base.m9_residual_bindings.0.iter().find(|binding| {
        binding.kind == ResidualObligationKind::VerifyDeferred
            && binding.module.as_deref() == Some(base.program_identity.module())
            && binding.contract.as_deref() == Some(M9_VERIFY_CONTRACT)
    });
    let (Some(auth_binding), Some(verify_binding)) = (auth_binding, verify_binding) else {
        return false;
    };
    let (
        Some(auth_ref),
        Some(auth_module),
        Some(auth_contract),
        Some(verify_ref),
        Some(verify_module),
        Some(verify_contract),
    ) = (
        auth_binding.source_ref.as_ref(),
        auth_binding.module.as_deref(),
        auth_binding.contract.as_deref(),
        verify_binding.source_ref.as_ref(),
        verify_binding.module.as_deref(),
        verify_binding.contract.as_deref(),
    )
    else {
        return false;
    };
    let Some(outer) = authority_runtime.outer_admission.as_ref() else {
        return false;
    };
    let (Some(membership), Some(capability), Some(witness)) = (
        authority_runtime
            .snapshot
            .memberships
            .get(&evidence.membership_ref),
        authority_runtime
            .snapshot
            .capabilities
            .get(&evidence.capability_ref),
        authority_runtime
            .snapshot
            .witnesses
            .get(&evidence.witness_ref),
    ) else {
        return false;
    };
    outer == &base.outer_admission
        && membership.active
        && authority_runtime
            .snapshot
            .current_memberships
            .get(&(membership.principal.clone(), membership.locus.clone()))
            == Some(&membership.reference)
        && capability.active
        && witness.live
        && !authority_runtime
            .snapshot
            .revoked_capabilities
            .contains(&evidence.capability_ref)
        && membership.auth_residual_source_ref.as_ref() == Some(auth_ref)
        && capability.membership_ref == membership.reference
        && capability.lineage_epoch == membership.epoch
        && capability.policy_version == "m9-policy-v1"
        && capability.source_ref.as_ref() == Some(auth_ref)
        && matches!(
            &capability.scope,
            M9CapabilityScope::ContractUpdate { module, contract }
                if module == auth_module && contract == auth_contract
        )
        && witness.membership_ref == membership.reference
        && witness.capability_ref == capability.reference
        && witness.source_ref.as_ref() == Some(auth_ref)
        && evidence
            .finite_refinement
            .as_ref()
            .is_some_and(|finite_refinement| {
                finite_refinement.residual_kind() == ResidualObligationKind::VerifyDeferred
                    && finite_refinement.program_identity() == &base.program_identity
                    && finite_refinement.residual_name() == verify_binding.name
                    && finite_refinement.source_ref() == verify_ref
                    && finite_refinement.module_contract() == (verify_module, verify_contract)
            })
}

fn validate_outer(
    checked: &CheckedSurfaceV0,
    envelope: &M9AdmissionEnvelope,
) -> Result<(), M9AdmissionDiagnostics> {
    if &envelope.program_identity != checked.program_identity() {
        return Err(M9AdmissionDiagnostics::one(
            M9AdmissionErrorKind::ProgramIdentityMismatch,
        ));
    }
    if envelope
        .original_source_artifact
        .as_ref()
        .is_none_or(|artifact| artifact.program_identity() != checked.program_identity())
    {
        return Err(M9AdmissionDiagnostics::one(
            M9AdmissionErrorKind::SourceArtifactMismatch,
        ));
    }
    let expected = checked
        .residual_obligations()
        .entries()
        .iter()
        .filter(|residual| {
            matches!(
                residual.kind(),
                ResidualObligationKind::AuthDeferred | ResidualObligationKind::VerifyDeferred
            )
        })
        .collect::<Vec<_>>();

    for residual in &expected {
        let bindings = envelope
            .residual_bindings
            .iter()
            .filter(|binding| binding.name == residual.name())
            .collect::<Vec<_>>();
        let Some(binding) = bindings.first() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::MissingResidualBinding,
            ));
        };
        if bindings.len() > 1 {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::DuplicateResidualBinding,
            ));
        }
        if binding.kind != residual.kind() {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::ResidualKindMismatch,
            ));
        }
        if binding.source_ref.as_ref() != Some(residual.source_ref()) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::SourceRefMismatch,
            ));
        }
        if binding.module.as_deref() != Some(checked.program_identity().module()) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::ConflictingResidualBinding,
            ));
        }
        let expected_contract = match residual.kind() {
            ResidualObligationKind::AuthDeferred => {
                format!("{M9_AUTH_CONTRACT_PREFIX}{}", residual.name())
            }
            ResidualObligationKind::VerifyDeferred => M9_VERIFY_CONTRACT.to_string(),
            _ => unreachable!("filtered to M9 residuals"),
        };
        if binding.contract.as_deref() != Some(expected_contract.as_str()) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::ConflictingResidualBinding,
            ));
        }
    }
    if envelope.residual_bindings.len() != expected.len() {
        return Err(M9AdmissionDiagnostics::one(
            M9AdmissionErrorKind::UnexpectedResidualBinding,
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9PreparedOuter {
    source_artifact: M9SourceArtifact,
    auth_residuals: BTreeSet<String>,
    unresolved_verify_residuals: BTreeSet<String>,
}

impl M9PreparedOuter {
    pub fn with_auth_residual(mut self, name: impl Into<String>) -> Self {
        self.auth_residuals.insert(name.into());
        self
    }

    pub fn with_unresolved_verify_residual(mut self, name: impl Into<String>) -> Self {
        self.unresolved_verify_residuals.insert(name.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M9BaseAdmissionEvidenceDelta {
    Remove(ResidualObligationKind, String),
    ReplaceSourceRef(ResidualObligationKind, String, SourceRef),
}

impl M9BaseAdmissionEvidenceDelta {
    pub fn remove(kind: ResidualObligationKind, name: impl Into<String>) -> Self {
        Self::Remove(kind, name.into())
    }

    pub fn replace_source_ref(
        kind: ResidualObligationKind,
        name: impl Into<String>,
        source_ref: SourceRef,
    ) -> Self {
        Self::ReplaceSourceRef(kind, name.into(), source_ref)
    }

    pub fn apply_to(self, admission: M8RuntimeAdmission) -> M8RuntimeAdmission {
        let mut rewritten = M8RuntimeAdmission::new(admission.program_identity().clone());
        for evidence in admission.evidence().iter().cloned() {
            let keep = !matches!(
                &self,
                Self::Remove(kind, name)
                    if evidence_kind(&evidence) == *kind && evidence_name(&evidence) == name
            );
            if !keep {
                continue;
            }
            let evidence = match &self {
                Self::ReplaceSourceRef(kind, name, source_ref)
                    if evidence_kind(&evidence) == *kind && evidence_name(&evidence) == name =>
                {
                    evidence_with_source_ref(evidence, source_ref.clone())
                }
                _ => evidence,
            };
            rewritten = rewritten.with_evidence(evidence);
        }
        rewritten
    }
}

fn evidence_kind(evidence: &M8AdmissionEvidence) -> ResidualObligationKind {
    match evidence {
        M8AdmissionEvidence::RelationVisibility { .. } => ResidualObligationKind::Visibility,
        M8AdmissionEvidence::RelationLifetime { .. } => ResidualObligationKind::RelationLifetime,
        M8AdmissionEvidence::RelationFallbackValidity { .. } => {
            ResidualObligationKind::FallbackValidity
        }
        M8AdmissionEvidence::ValueVisibilityRedaction { .. } => {
            ResidualObligationKind::ValueVisibilityRedaction
        }
        M8AdmissionEvidence::AuthDeferred { .. } => ResidualObligationKind::AuthDeferred,
        M8AdmissionEvidence::VerifyDeferred { .. } => ResidualObligationKind::VerifyDeferred,
    }
}

fn evidence_name(evidence: &M8AdmissionEvidence) -> &str {
    match evidence {
        M8AdmissionEvidence::RelationVisibility { relation, .. }
        | M8AdmissionEvidence::RelationLifetime { relation, .. }
        | M8AdmissionEvidence::RelationFallbackValidity { relation, .. } => relation,
        M8AdmissionEvidence::ValueVisibilityRedaction { value, .. } => value,
        M8AdmissionEvidence::AuthDeferred { name, .. }
        | M8AdmissionEvidence::VerifyDeferred { name, .. } => name,
    }
}

fn evidence_with_source_ref(
    evidence: M8AdmissionEvidence,
    source_ref: SourceRef,
) -> M8AdmissionEvidence {
    match evidence {
        M8AdmissionEvidence::RelationVisibility {
            relation,
            label,
            redaction,
            ..
        } => M8AdmissionEvidence::RelationVisibility {
            relation,
            label,
            redaction,
            source_ref,
        },
        M8AdmissionEvidence::RelationLifetime {
            relation,
            live_lease,
            binding_frontier,
            ..
        } => M8AdmissionEvidence::RelationLifetime {
            relation,
            live_lease,
            binding_frontier,
            source_ref,
        },
        M8AdmissionEvidence::RelationFallbackValidity {
            relation,
            primary_epoch,
            fallback_epoch,
            ..
        } => M8AdmissionEvidence::RelationFallbackValidity {
            relation,
            primary_epoch,
            fallback_epoch,
            source_ref,
        },
        M8AdmissionEvidence::ValueVisibilityRedaction {
            value,
            label,
            redaction,
            ..
        } => M8AdmissionEvidence::ValueVisibilityRedaction {
            value,
            label,
            redaction,
            source_ref,
        },
        M8AdmissionEvidence::AuthDeferred {
            name,
            authority_label,
            ..
        } => M8AdmissionEvidence::AuthDeferred {
            name,
            authority_label,
            source_ref,
        },
        M8AdmissionEvidence::VerifyDeferred {
            name,
            theorem,
            witness_schema,
            ..
        } => M8AdmissionEvidence::VerifyDeferred {
            name,
            theorem,
            witness_schema,
            source_ref,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9TransportClaim(String);

impl M9TransportClaim {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub const fn grant_authority(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M9TransportClaims(Vec<M9TransportClaim>);

impl M9TransportClaims {
    pub const fn grant_authority(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M9AuthorityClaim {
    TransportSession(String),
    LocusName(String),
    ProviderName(String),
    PrincipalName(String),
}

impl M9AuthorityClaim {
    pub fn from_transport_session(value: impl Into<String>) -> Self {
        Self::TransportSession(value.into())
    }

    pub fn from_locus_name(value: impl Into<String>) -> Self {
        Self::LocusName(value.into())
    }

    pub fn from_provider_name(value: impl Into<String>) -> Self {
        Self::ProviderName(value.into())
    }

    pub fn from_principal_name(value: impl Into<String>) -> Self {
        Self::PrincipalName(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct M9ProofRef(String);

impl M9ProofRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M9MembershipProofClaim {
    principal: String,
    locus: String,
    epoch: String,
    incarnation: String,
    policy_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ProviderProof {
    provider_ref: String,
    auth_kind: Option<String>,
    source_ref: Option<SourceRef>,
    proof_ref: Option<M9ProofRef>,
    membership_claim: Option<M9MembershipProofClaim>,
}

/// Issued only inside the runtime crate after the finite provider lane has
/// checked the admitted outer artifact.  Public `M9ProviderProof` values are
/// transportable claims, not this authority-bearing attestation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9IssuedProviderAttestation {
    residual_name: String,
    source_ref: SourceRef,
    proof_ref: M9ProofRef,
    membership_claim: M9MembershipProofClaim,
}

impl M9ProviderProof {
    pub fn new(provider_ref: impl Into<String>) -> Self {
        Self {
            provider_ref: provider_ref.into(),
            auth_kind: None,
            source_ref: None,
            proof_ref: None,
            membership_claim: None,
        }
    }

    pub fn for_auth_kind(mut self, name: impl Into<String>) -> Self {
        self.auth_kind = Some(name.into());
        self
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn with_proof_ref(mut self, proof_ref: M9ProofRef) -> Self {
        self.proof_ref = Some(proof_ref);
        self
    }

    pub fn with_membership_claim(
        mut self,
        principal: impl Into<String>,
        locus: impl Into<String>,
        epoch: impl Into<String>,
        incarnation: impl Into<String>,
        policy_version: impl Into<String>,
    ) -> Self {
        self.membership_claim = Some(M9MembershipProofClaim {
            principal: principal.into(),
            locus: locus.into(),
            epoch: epoch.into(),
            incarnation: incarnation.into(),
            policy_version: policy_version.into(),
        });
        self
    }

    pub fn for_principal(mut self, principal: impl Into<String>) -> Self {
        self.membership_claim_mut().principal = principal.into();
        self
    }

    pub fn for_locus(mut self, locus: impl Into<String>) -> Self {
        self.membership_claim_mut().locus = locus.into();
        self
    }

    pub fn for_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.membership_claim_mut().epoch = epoch.into();
        self
    }

    pub fn for_incarnation(mut self, incarnation: impl Into<String>) -> Self {
        self.membership_claim_mut().incarnation = incarnation.into();
        self
    }

    pub fn for_policy_version(mut self, policy_version: impl Into<String>) -> Self {
        self.membership_claim_mut().policy_version = policy_version.into();
        self
    }

    fn membership_claim_mut(&mut self) -> &mut M9MembershipProofClaim {
        self.membership_claim
            .get_or_insert_with(|| M9MembershipProofClaim {
                principal: String::new(),
                locus: String::new(),
                epoch: String::new(),
                incarnation: String::new(),
                policy_version: String::new(),
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9MembershipRequest {
    principal: String,
    locus: String,
    epoch: String,
    incarnation: String,
    policy_version: String,
    auth_residual: Option<(String, SourceRef)>,
    provider_proof: Option<M9ProviderProof>,
    issued_provider_attestation: Option<M9IssuedProviderAttestation>,
    authority_claim: Option<M9AuthorityClaim>,
    transport_claims: M9TransportClaims,
}

impl M9MembershipRequest {
    pub fn new(
        principal: impl Into<String>,
        locus: impl Into<String>,
        epoch: impl Into<String>,
    ) -> Self {
        let principal = principal.into();
        let locus = locus.into();
        let epoch = epoch.into();
        Self {
            incarnation: format!("incarnation:{principal}:{locus}:{epoch}"),
            principal,
            locus,
            epoch,
            policy_version: M9_POLICY_VERSION.to_string(),
            auth_residual: None,
            provider_proof: None,
            issued_provider_attestation: None,
            authority_claim: None,
            transport_claims: M9TransportClaims::default(),
        }
    }

    pub fn with_auth_residual(mut self, name: impl Into<String>, source_ref: SourceRef) -> Self {
        self.auth_residual = Some((name.into(), source_ref));
        self
    }

    pub fn with_incarnation(mut self, incarnation: impl Into<String>) -> Self {
        self.incarnation = incarnation.into();
        self
    }

    pub fn with_policy_version(mut self, policy_version: impl Into<String>) -> Self {
        self.policy_version = policy_version.into();
        self
    }

    pub fn with_provider_proof(mut self, proof: M9ProviderProof) -> Self {
        self.provider_proof = Some(proof);
        self
    }

    #[allow(dead_code)] // crate-internal finite provider adapter/test seam.
    pub(crate) fn with_issued_provider_attestation(
        mut self,
        attestation: M9IssuedProviderAttestation,
    ) -> Self {
        self.issued_provider_attestation = Some(attestation);
        self
    }

    pub fn with_authority_claim(mut self, claim: M9AuthorityClaim) -> Self {
        self.authority_claim = Some(claim);
        self
    }

    pub fn with_transport_claim(mut self, claim: M9TransportClaim) -> Self {
        self.transport_claims.0.push(claim);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9MembershipAuth {
    reference: String,
    principal: String,
    locus: String,
    epoch: String,
    incarnation: String,
    auth_residual_source_ref: Option<SourceRef>,
    provider_ref: String,
    proof_ref: Option<M9ProofRef>,
    policy_version: String,
    transport_claims: M9TransportClaims,
    active: bool,
}

impl M9MembershipAuth {
    pub fn ref_id(&self) -> &str {
        &self.reference
    }

    pub fn principal(&self) -> &str {
        &self.principal
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn epoch(&self) -> &str {
        &self.epoch
    }

    pub fn incarnation(&self) -> &str {
        &self.incarnation
    }

    pub fn auth_residual_source_ref(&self) -> &SourceRef {
        self.auth_residual_source_ref
            .as_ref()
            .expect("authenticated membership always retains auth SourceRef")
    }

    pub fn provider_ref(&self) -> &str {
        &self.provider_ref
    }

    pub fn transport_claims(&self) -> &M9TransportClaims {
        &self.transport_claims
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M9CapabilityScope {
    OwnerEvaluation {
        evaluation: String,
        owner_locus: String,
    },
    ContractUpdate {
        module: String,
        contract: String,
    },
    Observation {
        observer_principal: String,
        label: String,
        redaction: String,
        retention: String,
    },
    RelationTransition {
        relation: String,
        transition: String,
        owner_locus: String,
        binding_frontier: String,
    },
    DesignatedEvaluation {
        evaluator: String,
        result: String,
        input_frontier: String,
    },
    /// Producer-side authority to release exactly one source-derived input
    /// into a designated evaluator.  It is intentionally distinct from the
    /// evaluator's decision authority.
    DesignatedRemoteInputRelease {
        producer_locus: String,
        evaluator: String,
        result: String,
        dependency_index: usize,
        input_frontier: String,
        release_label: String,
        visibility: String,
    },
    DesignatedConsumption {
        consumer: String,
        value_name: String,
        result_version: u64,
    },
}

impl M9CapabilityScope {
    pub fn owner_evaluation(evaluation: impl Into<String>, owner_locus: impl Into<String>) -> Self {
        Self::OwnerEvaluation {
            evaluation: evaluation.into(),
            owner_locus: owner_locus.into(),
        }
    }

    pub fn contract_update(module: impl Into<String>, contract: impl Into<String>) -> Self {
        Self::ContractUpdate {
            module: module.into(),
            contract: contract.into(),
        }
    }

    pub fn observation(
        observer_principal: impl Into<String>,
        label: impl Into<String>,
        redaction: impl Into<String>,
        retention: impl Into<String>,
    ) -> Self {
        Self::Observation {
            observer_principal: observer_principal.into(),
            label: label.into(),
            redaction: redaction.into(),
            retention: retention.into(),
        }
    }

    /// Fixed finite policy for observer-safe M9 historic provenance.
    pub fn bounded_observation(observer_principal: impl Into<String>) -> Self {
        let observer_principal = observer_principal.into();
        Self::observation(
            format!("observer:{observer_principal}"),
            M9_OBSERVER_LABEL,
            M9_OBSERVER_REDACTION,
            M9_OBSERVER_RETENTION,
        )
    }

    /// These scope constructors are crate-internal M10 bridge vocabulary.
    /// They retain the checked relation/value site rather than allowing M10
    /// to fabricate an already-admitted M8 authority record.
    pub(crate) fn relation_transition(
        relation: impl Into<String>,
        transition: impl Into<String>,
        owner_locus: impl Into<String>,
        binding_frontier: impl Into<String>,
    ) -> Self {
        Self::RelationTransition {
            relation: relation.into(),
            transition: transition.into(),
            owner_locus: owner_locus.into(),
            binding_frontier: binding_frontier.into(),
        }
    }

    pub(crate) fn designated_evaluation(
        evaluator: impl Into<String>,
        result: impl Into<String>,
        input_frontier: impl Into<String>,
    ) -> Self {
        Self::DesignatedEvaluation {
            evaluator: evaluator.into(),
            result: result.into(),
            input_frontier: input_frontier.into(),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn designated_remote_input_release(
        producer_locus: impl Into<String>,
        evaluator: impl Into<String>,
        result: impl Into<String>,
        dependency_index: usize,
        input_frontier: impl Into<String>,
        release_label: impl Into<String>,
        visibility: impl Into<String>,
    ) -> Self {
        Self::DesignatedRemoteInputRelease {
            producer_locus: producer_locus.into(),
            evaluator: evaluator.into(),
            result: result.into(),
            dependency_index,
            input_frontier: input_frontier.into(),
            release_label: release_label.into(),
            visibility: visibility.into(),
        }
    }

    pub(crate) fn designated_consumption(
        consumer: impl Into<String>,
        value_name: impl Into<String>,
        result_version: u64,
    ) -> Self {
        Self::DesignatedConsumption {
            consumer: consumer.into(),
            value_name: value_name.into(),
            result_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9CapabilityGrantRequest {
    reference: String,
    membership_ref: Option<String>,
    scope: Option<M9CapabilityScope>,
    lineage_epoch: Option<String>,
    source_ref: Option<SourceRef>,
    authority_claim: Option<M9AuthorityClaim>,
}

impl M9CapabilityGrantRequest {
    pub fn new(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            membership_ref: None,
            scope: None,
            lineage_epoch: None,
            source_ref: None,
            authority_claim: None,
        }
    }

    pub fn with_membership_ref(mut self, membership_ref: impl Into<String>) -> Self {
        self.membership_ref = Some(membership_ref.into());
        self
    }

    pub fn with_scope(mut self, scope: M9CapabilityScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_lineage_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.lineage_epoch = Some(epoch.into());
        self
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn with_authority_claim(mut self, claim: M9AuthorityClaim) -> Self {
        self.authority_claim = Some(claim);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9CapabilityAuth {
    reference: String,
    membership_ref: String,
    scope: M9CapabilityScope,
    lineage_epoch: String,
    policy_version: String,
    source_ref: Option<SourceRef>,
    active: bool,
}

impl M9CapabilityAuth {
    pub fn ref_id(&self) -> &str {
        &self.reference
    }

    pub fn membership_ref(&self) -> &str {
        &self.membership_ref
    }

    pub fn scope(&self) -> &M9CapabilityScope {
        &self.scope
    }

    pub fn lineage_epoch(&self) -> &str {
        &self.lineage_epoch
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9WitnessRequest {
    reference: String,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    source_ref: Option<SourceRef>,
}

impl M9WitnessRequest {
    pub fn new(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            membership_ref: None,
            capability_ref: None,
            source_ref: None,
        }
    }

    pub fn with_membership_ref(mut self, membership_ref: impl Into<String>) -> Self {
        self.membership_ref = Some(membership_ref.into());
        self
    }

    pub fn with_capability_ref(mut self, capability_ref: impl Into<String>) -> Self {
        self.capability_ref = Some(capability_ref.into());
        self
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9WitnessAuth {
    reference: String,
    membership_ref: String,
    capability_ref: String,
    source_ref: Option<SourceRef>,
    live: bool,
}

impl M9WitnessAuth {
    pub fn ref_id(&self) -> &str {
        &self.reference
    }

    pub fn membership_ref(&self) -> &str {
        &self.membership_ref
    }

    pub fn capability_ref(&self) -> &str {
        &self.capability_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9FactUse {
    capability_ref: String,
    membership_ref: Option<String>,
    witness_ref: Option<String>,
    epoch: Option<String>,
    scope: Option<M9CapabilityScope>,
    copied_from: Option<String>,
    revocation_ref: Option<String>,
}

impl M9FactUse {
    pub fn capability(reference: impl Into<String>) -> Self {
        Self {
            capability_ref: reference.into(),
            membership_ref: None,
            witness_ref: None,
            epoch: None,
            scope: None,
            copied_from: None,
            revocation_ref: None,
        }
    }

    pub fn with_membership_ref(mut self, reference: impl Into<String>) -> Self {
        self.membership_ref = Some(reference.into());
        self
    }

    pub fn with_witness_ref(mut self, reference: impl Into<String>) -> Self {
        self.witness_ref = Some(reference.into());
        self
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }

    pub fn with_scope(mut self, scope: M9CapabilityScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_copied_from(mut self, reference: impl Into<String>) -> Self {
        self.copied_from = Some(reference.into());
        self
    }

    pub fn with_revocation_ref(mut self, reference: impl Into<String>) -> Self {
        self.revocation_ref = Some(reference.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9EvidenceInvalidationKind {
    RevokedCapability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9EvidenceInvalidation {
    kind: M9EvidenceInvalidationKind,
    dependent_refs: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct M9EvidenceGraph {
    invalidations: BTreeMap<String, M9EvidenceInvalidation>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    artifacts: BTreeSet<String>,
}

impl M9EvidenceGraph {
    pub fn invalidated_artifacts_for(&self, capability_ref: &str) -> Vec<String> {
        self.invalidations
            .get(capability_ref)
            .map(|invalidation| invalidation.dependent_refs.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn revoke(&mut self, capability_ref: &str, dependent_refs: impl IntoIterator<Item = String>) {
        let mut all_dependents = self
            .dependencies
            .get(capability_ref)
            .cloned()
            .unwrap_or_default();
        all_dependents.extend(dependent_refs);
        self.artifacts.extend(all_dependents.iter().cloned());
        let invalidation = self
            .invalidations
            .entry(capability_ref.to_string())
            .or_insert_with(|| M9EvidenceInvalidation {
                kind: M9EvidenceInvalidationKind::RevokedCapability,
                dependent_refs: BTreeSet::new(),
            });
        invalidation.dependent_refs.extend(all_dependents);
    }

    fn add_dependent(&mut self, capability_ref: &str, dependent_ref: impl Into<String>) {
        let dependent_ref = dependent_ref.into();
        self.artifacts.insert(dependent_ref.clone());
        self.dependencies
            .entry(capability_ref.to_string())
            .or_default()
            .insert(dependent_ref);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9Revocation {
    capability_ref: String,
    source_ref: Option<SourceRef>,
    witness_ref: Option<String>,
    dependent_artifacts: BTreeSet<String>,
}

impl M9Revocation {
    pub fn capability(capability_ref: impl Into<String>) -> Self {
        Self {
            capability_ref: capability_ref.into(),
            source_ref: None,
            witness_ref: None,
            dependent_artifacts: BTreeSet::new(),
        }
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn with_witness_ref(mut self, witness_ref: impl Into<String>) -> Self {
        self.witness_ref = Some(witness_ref.into());
        self
    }

    pub fn with_dependent_artifact(mut self, artifact_ref: impl Into<String>) -> Self {
        self.dependent_artifacts.insert(artifact_ref.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct M9AuthoritySnapshot {
    memberships: BTreeMap<String, M9MembershipAuth>,
    capabilities: BTreeMap<String, M9CapabilityAuth>,
    witnesses: BTreeMap<String, M9WitnessAuth>,
    revoked_capabilities: BTreeSet<String>,
    consumed_proof_refs: BTreeSet<M9ProofRef>,
    current_memberships: BTreeMap<(String, String), String>,
    /// A retired membership remains inspectable until its sealed audit cut.
    /// The value is the frontier at which its tombstone became durable.
    retired_memberships: BTreeMap<String, String>,
}

/// Crate-private immutable authority cut.  It deliberately owns a coherent
/// M9 snapshot and evidence graph; callers cannot reconstruct one from
/// public membership/capability/witness values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9AuthorityCut {
    reference: String,
    program_identity: String,
    snapshot: M9AuthoritySnapshot,
    evidence_graph: M9EvidenceGraph,
}

#[derive(Clone, PartialEq, Eq)]
pub struct M9AuthorityRuntime {
    outer_admission: Option<M9OuterAdmission>,
    snapshot: M9AuthoritySnapshot,
    evidence_graph: M9EvidenceGraph,
    next_authority_cut: u64,
    restored_authority_cuts: BTreeSet<String>,
}

impl std::fmt::Debug for M9AuthorityRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9AuthorityRuntime")
            .field("has_outer_admission", &self.outer_admission.is_some())
            .field(
                "active_membership_count",
                &self.snapshot.current_memberships.len(),
            )
            .finish()
    }
}

impl M9AuthorityRuntime {
    pub fn empty() -> Self {
        Self {
            outer_admission: None,
            snapshot: M9AuthoritySnapshot::default(),
            evidence_graph: M9EvidenceGraph::default(),
            next_authority_cut: 0,
            restored_authority_cuts: BTreeSet::new(),
        }
    }

    pub fn from_outer_admission(outer_admission: M9OuterAdmission) -> Self {
        Self {
            outer_admission: Some(outer_admission),
            snapshot: M9AuthoritySnapshot::default(),
            evidence_graph: M9EvidenceGraph::default(),
            next_authority_cut: 0,
            restored_authority_cuts: BTreeSet::new(),
        }
    }

    pub(crate) fn authority_snapshot(&self) -> M9AuthoritySnapshot {
        self.snapshot.clone()
    }

    pub(crate) fn authority_fact_inventory(&self) -> M9AuthorityFactInventory {
        M9AuthorityFactInventory {
            membership_refs: self.snapshot.memberships.keys().cloned().collect(),
            grant_refs: self.snapshot.capabilities.keys().cloned().collect(),
            witness_refs: self.snapshot.witnesses.keys().cloned().collect(),
            retirement_tombstones: self.snapshot.retired_memberships.keys().cloned().collect(),
        }
    }

    pub(crate) fn evidence_graph(&self) -> &M9EvidenceGraph {
        &self.evidence_graph
    }

    /// Exact membership lineage domain for crate-internal cross-layer
    /// receipts.  Grants, witnesses, revocations, and consumed proof records
    /// are intentionally excluded so membership-only transitions do not
    /// inherit an unrelated authority hash change.
    pub(crate) fn canonical_membership_projection(&self) -> String {
        let memberships = self.snapshot.memberships.values().map(|membership| {
            let source_ref = membership.auth_residual_source_ref.as_ref().map_or_else(
                || "none".to_string(),
                |source_ref| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        source_ref.path,
                        source_ref.start_line,
                        source_ref.start_column,
                        source_ref.end_line,
                        source_ref.end_column,
                    )
                },
            );
            format!(
                "membership|{}|{}|{}|{}|{}|{}|{}|{}|auth_source_ref|{}|proof_ref|{}",
                membership.reference,
                membership.principal,
                membership.locus,
                membership.epoch,
                membership.incarnation,
                membership.provider_ref,
                membership.policy_version,
                membership.active,
                source_ref,
                membership
                    .proof_ref
                    .as_ref()
                    .map_or("", |proof| proof.0.as_str()),
            )
        });
        let current_memberships =
            self.snapshot
                .current_memberships
                .iter()
                .map(|((principal, locus), membership_ref)| {
                    format!("current_membership|{principal}|{locus}|{membership_ref}")
                });
        let retired =
            self.snapshot
                .retired_memberships
                .iter()
                .map(|(membership_ref, audit_frontier)| {
                    format!("retired_membership|{membership_ref}|{audit_frontier}")
                });
        memberships
            .chain(current_memberships)
            .chain(retired)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Exact grant/evidence domain for crate-internal cross-layer receipts.
    /// Capability and witness SourceRefs are rendered in full so an otherwise
    /// identical grant at a different source span cannot share a receipt hash.
    pub(crate) fn canonical_grant_projection(&self) -> String {
        let capabilities = self.snapshot.capabilities.values().map(|capability| {
            let source_ref = capability.source_ref.as_ref().map_or_else(
                || "none".to_string(),
                |source_ref| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        source_ref.path,
                        source_ref.start_line,
                        source_ref.start_column,
                        source_ref.end_line,
                        source_ref.end_column,
                    )
                },
            );
            format!(
                "capability|{}|{}|{}|{}|{}|{}|source_ref|{}",
                capability.reference,
                capability.membership_ref,
                canonical_m9_capability_scope(&capability.scope),
                capability.lineage_epoch,
                capability.policy_version,
                capability.active,
                source_ref,
            )
        });
        let witnesses = self.snapshot.witnesses.values().map(|witness| {
            let source_ref = witness.source_ref.as_ref().map_or_else(
                || "none".to_string(),
                |source_ref| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        source_ref.path,
                        source_ref.start_line,
                        source_ref.start_column,
                        source_ref.end_line,
                        source_ref.end_column,
                    )
                },
            );
            format!(
                "witness|{}|{}|{}|{}|{}|source_ref|{}",
                witness.reference,
                witness.membership_ref,
                witness.capability_ref,
                witness.source_ref.is_some(),
                witness.live,
                source_ref,
            )
        });
        let revoked = self
            .snapshot
            .revoked_capabilities
            .iter()
            .map(|reference| format!("revoked|{reference}"));
        let consumed = self
            .snapshot
            .consumed_proof_refs
            .iter()
            .map(|reference| format!("consumed_proof|{}", reference.0));
        capabilities
            .chain(witnesses)
            .chain(revoked)
            .chain(consumed)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Canonical internal aggregate for diagnostics that truly require the
    /// full M9 authority state.  Domain-native receipt hashes consume the two
    /// projections above independently.
    pub(crate) fn canonical_snapshot_projection(&self) -> String {
        [
            self.canonical_membership_projection(),
            self.canonical_grant_projection(),
        ]
        .join("\n")
    }

    /// Revalidate one already-issued M9 membership/capability/witness lineage
    /// against the current sealed snapshot and produce the sole M9→M8 entity
    /// presence bridge. A live record requires a current live lineage; a
    /// retired record requires the exact durable membership tombstone. Neither
    /// M8 nor M10 can construct this bridge from raw facts.
    pub(crate) fn m10_entity_presence_bridge(
        &self,
        membership: &M9MembershipAuth,
        capability: &M9CapabilityAuth,
        witness: &M9WitnessAuth,
        namespace: &str,
        identity: &str,
        source_ref: SourceRef,
    ) -> Result<M9M8EntityPresenceBridge, M9AdmissionDiagnostics> {
        let Some(snapshot_membership) = self.snapshot.memberships.get(membership.ref_id()) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        };
        let Some(snapshot_capability) = self.snapshot.capabilities.get(capability.ref_id()) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let Some(snapshot_witness) = self.snapshot.witnesses.get(witness.ref_id()) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let membership_matches = snapshot_membership.reference == membership.reference
            && snapshot_membership.principal == membership.principal
            && snapshot_membership.locus == membership.locus
            && snapshot_membership.epoch == membership.epoch
            && snapshot_membership.incarnation == membership.incarnation;
        let capability_matches = snapshot_capability.reference == capability.reference
            && snapshot_capability.membership_ref == membership.reference
            && snapshot_capability.lineage_epoch == membership.epoch;
        let witness_matches = snapshot_witness.reference == witness.reference
            && snapshot_witness.membership_ref == membership.reference
            && snapshot_witness.capability_ref == capability.reference;
        if !membership_matches
            || !capability_matches
            || !witness_matches
            || snapshot_membership.principal != identity
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }

        let membership_key = (
            snapshot_membership.principal.clone(),
            snapshot_membership.locus.clone(),
        );
        let status = if snapshot_membership.active
            && snapshot_capability.active
            && snapshot_witness.live
            && self.snapshot.current_memberships.get(&membership_key)
                == Some(&snapshot_membership.reference)
        {
            M9M8EntityPresenceStatus::Live
        } else if !snapshot_membership.active
            && !snapshot_capability.active
            && !snapshot_witness.live
            && self
                .snapshot
                .retired_memberships
                .contains_key(&snapshot_membership.reference)
        {
            M9M8EntityPresenceStatus::Retired
        } else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        };
        let snapshot_projection = self.canonical_snapshot_projection();
        Ok(M9M8EntityPresenceBridge {
            namespace: namespace.to_string(),
            identity: identity.to_string(),
            source_ref,
            status,
            sealed_membership_ref: m9_opaque_ref(&format!(
                "membership|{}",
                snapshot_membership.reference
            )),
            sealed_capability_ref: m9_opaque_ref(&format!(
                "capability|{}",
                snapshot_capability.reference
            )),
            sealed_witness_ref: m9_opaque_ref(&format!("witness|{}", snapshot_witness.reference)),
            sealed_epoch: m9_opaque_ref(&format!(
                "epoch|{}|{}",
                snapshot_membership.reference, snapshot_membership.epoch
            )),
            sealed_incarnation: m9_opaque_ref(&format!(
                "incarnation|{}|{}",
                snapshot_membership.reference, snapshot_membership.incarnation
            )),
            m9_snapshot_ref: m9_opaque_ref(&format!("snapshot|{snapshot_projection}")),
            m8_authority_use_ref: m9_opaque_ref(&format!(
                "m8-presence-use|{}|{}|{}",
                snapshot_membership.reference, namespace, identity
            )),
        })
    }

    /// Translate already authenticated M9 lineage into the sealed M8
    /// inventory for a persistent M10 execution session.  The caller only
    /// passes typed facts it received from this authority runtime; this method
    /// revalidates them before it constructs any M8 authority record.
    pub(crate) fn m10_authority_bridge(
        &mut self,
        membership: &M9MembershipAuth,
        contract_capability: &M9CapabilityAuth,
        contract_witness: &M9WitnessAuth,
        owner: Option<(&str, &str, &M9CapabilityAuth, &M9WitnessAuth)>,
    ) -> M9M10AuthorityBridge {
        let authority_snapshot_projection = self.canonical_snapshot_projection();
        let contract_is_live = self
            .use_authority(
                M9FactUse::capability(contract_capability.ref_id())
                    .with_membership_ref(membership.ref_id())
                    .with_witness_ref(contract_witness.ref_id())
                    .with_epoch(membership.epoch())
                    .with_scope(contract_capability.scope().clone()),
            )
            .is_ok();
        if !contract_is_live {
            return M9M10AuthorityBridge {
                authority_state: M8AuthorityState::new(),
                authority_snapshot_projection,
                owner_use: None,
                patch_use: None,
                relation_uses: BTreeMap::new(),
            };
        }

        let M9CapabilityScope::ContractUpdate { module, .. } = contract_capability.scope() else {
            return M9M10AuthorityBridge {
                authority_state: M8AuthorityState::new(),
                authority_snapshot_projection,
                owner_use: None,
                patch_use: None,
                relation_uses: BTreeMap::new(),
            };
        };
        let mut authority_state = M8AuthorityState::new()
            .with_membership_record(
                M8MembershipRecord::already_admitted(membership.ref_id())
                    .with_principal(membership.principal())
                    .with_locus(membership.locus())
                    .with_epoch(membership.epoch()),
            )
            .with_capability_grant(
                M8CapabilityGrant::already_admitted(contract_capability.ref_id())
                    .for_patch_activation(module)
                    .with_owner_locus(membership.locus())
                    .with_principal(membership.principal())
                    .with_membership_ref(membership.ref_id())
                    .with_epoch(membership.epoch()),
            )
            .with_witness_record(
                M8WitnessRecord::live(contract_witness.ref_id())
                    .for_capability(contract_capability.ref_id())
                    .with_membership_ref(membership.ref_id())
                    .with_epoch(membership.epoch()),
            );
        let patch_use = Some(
            M8PatchAuthorityUse::for_patch_program(module)
                .with_owner_locus(membership.locus())
                .with_principal(membership.principal())
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(contract_capability.ref_id())
                .with_witness_ref(contract_witness.ref_id()),
        );
        let owner_use = owner.and_then(|(evaluation, owner_locus, capability, witness)| {
            let owner_is_live = self
                .use_authority(
                    M9FactUse::capability(capability.ref_id())
                        .with_membership_ref(membership.ref_id())
                        .with_witness_ref(witness.ref_id())
                        .with_epoch(membership.epoch())
                        .with_scope(capability.scope().clone()),
                )
                .is_ok();
            if !owner_is_live {
                return None;
            }
            authority_state = authority_state
                .clone()
                .with_capability_grant(
                    M8CapabilityGrant::already_admitted(capability.ref_id())
                        .for_owner_evaluation(evaluation)
                        .with_owner_locus(owner_locus)
                        .with_principal(membership.principal())
                        .with_membership_ref(membership.ref_id())
                        .with_epoch(membership.epoch()),
                )
                .with_witness_record(
                    M8WitnessRecord::live(witness.ref_id())
                        .for_capability(capability.ref_id())
                        .with_membership_ref(membership.ref_id())
                        .with_epoch(membership.epoch()),
                );
            Some((
                evaluation.to_string(),
                owner_locus.to_string(),
                M8AuthorityUse::for_principal(membership.principal())
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(capability.ref_id())
                    .with_witness_ref(witness.ref_id()),
            ))
        });
        let mut relation_uses = BTreeMap::new();
        let relation_capabilities = self
            .snapshot
            .capabilities
            .values()
            .filter(|capability| {
                capability.active
                    && capability.membership_ref == membership.ref_id()
                    && matches!(
                        capability.scope,
                        M9CapabilityScope::RelationTransition { .. }
                    )
            })
            .cloned()
            .collect::<Vec<_>>();
        for capability in relation_capabilities {
            let M9CapabilityScope::RelationTransition {
                relation,
                transition,
                owner_locus,
                binding_frontier: _,
            } = capability.scope()
            else {
                continue;
            };
            let Some(witness) = self
                .snapshot
                .witnesses
                .values()
                .find(|witness| {
                    witness.live
                        && witness.membership_ref == membership.ref_id()
                        && witness.capability_ref == capability.ref_id()
                })
                .cloned()
            else {
                continue;
            };
            if self
                .use_authority(
                    M9FactUse::capability(capability.ref_id())
                        .with_membership_ref(membership.ref_id())
                        .with_witness_ref(witness.ref_id())
                        .with_epoch(membership.epoch())
                        .with_scope(capability.scope().clone()),
                )
                .is_err()
            {
                continue;
            }
            let binding_epoch = match transition.as_str() {
                "invalidate_primary" => "binding_epoch:1".to_string(),
                "reacquire_primary" => "binding_epoch:2".to_string(),
                other => format!("m9-transition:{other}:{}", capability.ref_id()),
            };
            authority_state = authority_state
                .with_capability_grant(
                    M8CapabilityGrant::already_admitted(capability.ref_id())
                        .for_relation_transition(relation.clone(), transition.clone())
                        .with_owner_locus(owner_locus.clone())
                        .with_principal(membership.principal())
                        .with_membership_ref(membership.ref_id())
                        .with_epoch(membership.epoch())
                        .with_binding_epoch(binding_epoch.clone()),
                )
                .with_witness_record(
                    M8WitnessRecord::live(witness.ref_id())
                        .for_capability(capability.ref_id())
                        .with_membership_ref(membership.ref_id())
                        .with_epoch(membership.epoch()),
                );
            relation_uses.insert(
                (relation.clone(), transition.clone()),
                M8RelationAuthorityUse::for_relation(relation.clone())
                    .with_owner_locus(owner_locus.clone())
                    .with_transition(transition.clone())
                    .with_principal(membership.principal())
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref(capability.ref_id())
                    .with_membership_epoch(membership.epoch())
                    .with_binding_epoch(binding_epoch)
                    .with_witness_ref(witness.ref_id())
                    .with_witness_epoch(membership.epoch()),
            );
        }
        M9M10AuthorityBridge {
            authority_state,
            authority_snapshot_projection,
            owner_use,
            patch_use,
            relation_uses,
        }
    }

    /// Save the whole M9 authority state as one coherent internal cut.  The
    /// cut is intentionally not a transport or public persistence format.
    pub(crate) fn save_authority_cut(&mut self) -> Result<M9AuthorityCut, M9AdmissionDiagnostics> {
        let Some(outer) = self.outer_admission.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidAuthorityCut,
            ));
        };
        if !Self::snapshot_is_coherent(&self.snapshot) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidAuthorityCut,
            ));
        }
        let reference = format!(
            "m9-authority-cut:{}:{}",
            outer.program_identity.stable_key(),
            self.next_authority_cut
        );
        self.next_authority_cut += 1;
        Ok(M9AuthorityCut {
            reference,
            program_identity: outer.program_identity.stable_key(),
            snapshot: self.snapshot.clone(),
            evidence_graph: self.evidence_graph.clone(),
        })
    }

    /// Restore exactly one previously saved cut into a fresh compatible M9
    /// authority runtime.  A cut with split lineage, inconsistent live
    /// authority, a different checked program, or a second delivery fails
    /// closed before it changes the runtime.
    pub(crate) fn restore_authority_cut(
        &mut self,
        cut: M9AuthorityCut,
    ) -> Result<(), M9AdmissionDiagnostics> {
        let Some(outer) = self.outer_admission.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidAuthorityCut,
            ));
        };
        if cut.program_identity != outer.program_identity.stable_key()
            || self.restored_authority_cuts.contains(&cut.reference)
            || !Self::snapshot_is_coherent(&cut.snapshot)
        {
            return Err(M9AdmissionDiagnostics::one(
                if self.restored_authority_cuts.contains(&cut.reference) {
                    M9AdmissionErrorKind::ReplayedAuthorityCut
                } else {
                    M9AdmissionErrorKind::InvalidAuthorityCut
                },
            ));
        }
        self.snapshot = cut.snapshot;
        self.evidence_graph = cut.evidence_graph;
        self.restored_authority_cuts.insert(cut.reference);
        Ok(())
    }

    /// Retire a membership as one authority operation.  It removes the
    /// current principal/locus mapping, tombstones the membership, and
    /// invalidates every active capability/witness in that lineage while
    /// retaining the evidence graph until the named audit frontier permits
    /// compaction.
    pub(crate) fn retire_membership(
        &mut self,
        membership_ref: &str,
        audit_frontier: impl Into<String>,
    ) -> Result<(), M9AdmissionDiagnostics> {
        let Some(membership) = self.snapshot.memberships.get(membership_ref).cloned() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        };
        let membership_key = (membership.principal.clone(), membership.locus.clone());
        if !membership.active
            || self.snapshot.current_memberships.get(&membership_key) != Some(&membership.reference)
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        }
        let audit_frontier = audit_frontier.into();
        self.snapshot.current_memberships.remove(&membership_key);
        self.snapshot
            .retired_memberships
            .insert(membership.reference.clone(), audit_frontier);
        self.snapshot
            .memberships
            .get_mut(membership_ref)
            .expect("checked membership remains present")
            .active = false;

        let capability_refs = self
            .snapshot
            .capabilities
            .values()
            .filter(|capability| capability.membership_ref == membership_ref && capability.active)
            .map(|capability| capability.reference.clone())
            .collect::<Vec<_>>();
        for capability_ref in capability_refs {
            self.retire_capability(
                &capability_ref,
                [format!("membership-retired:{membership_ref}")],
            );
        }
        Ok(())
    }

    /// Retire a live witness while retaining its M9 lineage and invalidation
    /// evidence.  This is deliberately narrower than capability revocation.
    pub(crate) fn retire_witness(
        &mut self,
        witness_ref: &str,
    ) -> Result<(), M9AdmissionDiagnostics> {
        let Some(witness) = self.snapshot.witnesses.get(witness_ref).cloned() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        if !witness.live {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }
        self.snapshot
            .witnesses
            .get_mut(witness_ref)
            .expect("checked witness remains present")
            .live = false;
        self.evidence_graph
            .revoke(&witness.capability_ref, [witness.reference]);
        Ok(())
    }

    /// Tombstones may be compacted only at the exact recorded audit cut.  A
    /// caller cannot erase a current membership or pre-audit evidence.
    pub(crate) fn compact_retired_membership(
        &mut self,
        membership_ref: &str,
        audit_frontier: &str,
    ) -> Result<(), M9AdmissionDiagnostics> {
        if self
            .snapshot
            .retired_memberships
            .get(membership_ref)
            .is_none_or(|recorded| recorded != audit_frontier)
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::CompactionBeforeAuditCut,
            ));
        }
        self.snapshot.retired_memberships.remove(membership_ref);
        Ok(())
    }

    fn retire_capability(
        &mut self,
        capability_ref: &str,
        dependent_refs: impl IntoIterator<Item = String>,
    ) {
        if let Some(capability) = self.snapshot.capabilities.get_mut(capability_ref) {
            capability.active = false;
        }
        let mut all_dependents = dependent_refs.into_iter().collect::<BTreeSet<_>>();
        for witness in self.snapshot.witnesses.values_mut() {
            if witness.capability_ref == capability_ref {
                witness.live = false;
                all_dependents.insert(witness.reference.clone());
            }
        }
        self.snapshot
            .revoked_capabilities
            .insert(capability_ref.to_string());
        self.evidence_graph.revoke(capability_ref, all_dependents);
    }

    fn snapshot_is_coherent(snapshot: &M9AuthoritySnapshot) -> bool {
        let current_memberships_are_live =
            snapshot
                .current_memberships
                .iter()
                .all(|((principal, locus), membership_ref)| {
                    snapshot
                        .memberships
                        .get(membership_ref)
                        .is_some_and(|membership| {
                            membership.active
                                && membership.principal == *principal
                                && membership.locus == *locus
                                && !snapshot.retired_memberships.contains_key(membership_ref)
                        })
                });
        let active_capabilities_are_current = snapshot.capabilities.values().all(|capability| {
            !capability.active
                || snapshot
                    .memberships
                    .get(&capability.membership_ref)
                    .is_some_and(|membership| {
                        membership.active
                            && snapshot
                                .current_memberships
                                .get(&(membership.principal.clone(), membership.locus.clone()))
                                == Some(&membership.reference)
                    })
        });
        let live_witnesses_are_bound = snapshot.witnesses.values().all(|witness| {
            !witness.live
                || snapshot
                    .capabilities
                    .get(&witness.capability_ref)
                    .is_some_and(|capability| {
                        capability.active
                            && capability.membership_ref == witness.membership_ref
                            && snapshot
                                .memberships
                                .get(&witness.membership_ref)
                                .is_some_and(|membership| membership.active)
                    })
        });
        let revoked_are_inactive = snapshot.revoked_capabilities.iter().all(|capability_ref| {
            snapshot
                .capabilities
                .get(capability_ref)
                .is_some_and(|capability| !capability.active)
        });
        let tombstones_are_not_current =
            snapshot.retired_memberships.keys().all(|membership_ref| {
                snapshot
                    .memberships
                    .get(membership_ref)
                    .is_some_and(|membership| {
                        !membership.active
                            && snapshot
                                .current_memberships
                                .get(&(membership.principal.clone(), membership.locus.clone()))
                                != Some(&membership.reference)
                    })
            });
        current_memberships_are_live
            && active_capabilities_are_current
            && live_witnesses_are_bound
            && revoked_are_inactive
            && tombstones_are_not_current
    }

    #[allow(dead_code)] // finite provider adapter/test seam; not public authority minting.
    pub(crate) fn issue_membership_attestation(
        &self,
        principal: impl Into<String>,
        locus: impl Into<String>,
        epoch: impl Into<String>,
        incarnation: impl Into<String>,
        residual_name: impl Into<String>,
        source_ref: SourceRef,
    ) -> Result<M9IssuedProviderAttestation, M9AdmissionDiagnostics> {
        let residual_name = residual_name.into();
        let Some(outer) = self.outer_admission.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        };
        let outer_binds_residual = outer.residual_bindings.0.iter().any(|binding| {
            binding.kind == ResidualObligationKind::AuthDeferred
                && binding.name == residual_name
                && binding.source_ref.as_ref() == Some(&source_ref)
                && binding.contract.as_deref()
                    == Some(&format!("{M9_AUTH_CONTRACT_PREFIX}{residual_name}"))
        });
        if !outer_binds_residual {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        }
        let principal = principal.into();
        let locus = locus.into();
        let epoch = epoch.into();
        let incarnation = incarnation.into();
        Ok(M9IssuedProviderAttestation {
            proof_ref: M9ProofRef(format!(
                "issued:{}:{principal}:{locus}:{epoch}:{incarnation}",
                outer.program_identity.stable_key()
            )),
            residual_name,
            source_ref,
            membership_claim: M9MembershipProofClaim {
                principal,
                locus,
                epoch,
                incarnation,
                policy_version: M9_POLICY_VERSION.to_string(),
            },
        })
    }

    pub fn authenticate_membership(
        &mut self,
        request: M9MembershipRequest,
    ) -> Result<M9MembershipAuth, M9AdmissionDiagnostics> {
        if request.authority_claim.is_some() {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::ProviderOrTransportIsNotAuthority,
            ));
        }
        let Some((residual_name, residual_ref)) = request.auth_residual.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        };
        let Some(attestation) = request.issued_provider_attestation.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::UnadmittedAuthProvider,
            ));
        };
        let outer_binds_residual = self.outer_admission.as_ref().is_some_and(|outer| {
            outer.residual_bindings.0.iter().any(|binding| {
                binding.kind == ResidualObligationKind::AuthDeferred
                    && binding.name == *residual_name
                    && binding.source_ref.as_ref() == Some(residual_ref)
                    && binding.contract.as_deref()
                        == Some(&format!("{M9_AUTH_CONTRACT_PREFIX}{residual_name}"))
            })
        });
        if !outer_binds_residual
            || attestation.residual_name != *residual_name
            || attestation.source_ref != *residual_ref
            || attestation.membership_claim.principal != request.principal
            || attestation.membership_claim.locus != request.locus
            || attestation.membership_claim.epoch != request.epoch
            || attestation.membership_claim.incarnation != request.incarnation
            || attestation.membership_claim.policy_version != request.policy_version
            || request.policy_version != M9_POLICY_VERSION
            || self
                .snapshot
                .consumed_proof_refs
                .contains(&attestation.proof_ref)
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidMembershipLineage,
            ));
        }
        let membership = M9MembershipAuth {
            reference: format!(
                "membership:{}:{}:{}",
                request.principal, request.locus, request.epoch
            ),
            principal: request.principal,
            locus: request.locus,
            incarnation: request.incarnation,
            epoch: request.epoch,
            auth_residual_source_ref: Some(residual_ref.clone()),
            provider_ref: M9_ADMITTED_AUTH_PROVIDER.to_string(),
            proof_ref: Some(attestation.proof_ref.clone()),
            policy_version: request.policy_version,
            transport_claims: request.transport_claims,
            active: true,
        };
        if self
            .snapshot
            .memberships
            .contains_key(&membership.reference)
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::DuplicateMembershipReference,
            ));
        }
        self.invalidate_prior_lineage_for_fresh_membership(&membership);
        self.snapshot
            .consumed_proof_refs
            .insert(attestation.proof_ref.clone());
        self.snapshot.current_memberships.insert(
            (membership.principal.clone(), membership.locus.clone()),
            membership.reference.clone(),
        );
        self.snapshot
            .memberships
            .insert(membership.reference.clone(), membership.clone());
        Ok(membership)
    }

    fn invalidate_prior_lineage_for_fresh_membership(&mut self, membership: &M9MembershipAuth) {
        let key = (membership.principal.clone(), membership.locus.clone());
        let Some(prior_ref) = self.snapshot.current_memberships.get(&key).cloned() else {
            return;
        };
        let Some(prior_membership) = self.snapshot.memberships.get(&prior_ref) else {
            return;
        };
        if prior_membership.epoch == membership.epoch
            && prior_membership.incarnation == membership.incarnation
            && prior_membership.provider_ref == membership.provider_ref
        {
            return;
        }
        let stale_capabilities = self
            .snapshot
            .capabilities
            .values()
            .filter(|capability| capability.membership_ref == prior_ref && capability.active)
            .map(|capability| capability.reference.clone())
            .collect::<Vec<_>>();
        if let Some(prior_membership) = self.snapshot.memberships.get_mut(&prior_ref) {
            prior_membership.active = false;
        }
        for capability_ref in stale_capabilities {
            if let Some(capability) = self.snapshot.capabilities.get_mut(&capability_ref) {
                capability.active = false;
            }
            let mut dependent_refs = BTreeSet::new();
            for witness in self.snapshot.witnesses.values_mut() {
                if witness.capability_ref == capability_ref {
                    witness.live = false;
                    dependent_refs.insert(witness.reference.clone());
                }
            }
            self.snapshot
                .revoked_capabilities
                .insert(capability_ref.clone());
            self.evidence_graph.revoke(&capability_ref, dependent_refs);
        }
    }

    pub fn authorize_capability(
        &mut self,
        request: M9CapabilityGrantRequest,
    ) -> Result<M9CapabilityAuth, M9AdmissionDiagnostics> {
        if request.authority_claim.is_some() {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::ProviderOrTransportIsNotAuthority,
            ));
        }
        let (Some(membership_ref), Some(scope), Some(lineage_epoch), Some(source_ref)) = (
            request.membership_ref.as_ref(),
            request.scope.as_ref(),
            request.lineage_epoch.as_ref(),
            request.source_ref.as_ref(),
        ) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let Some(membership) = self.snapshot.memberships.get(membership_ref) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        if !membership.active
            || membership.epoch != *lineage_epoch
            || membership.auth_residual_source_ref.as_ref() != Some(source_ref)
            || membership.policy_version != M9_POLICY_VERSION
            || self
                .snapshot
                .current_memberships
                .get(&(membership.principal.clone(), membership.locus.clone()))
                != Some(membership_ref)
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }
        if let Some(existing) = self.snapshot.capabilities.get(&request.reference) {
            let exact_duplicate = existing.membership_ref == *membership_ref
                && existing.scope == *scope
                && existing.lineage_epoch == *lineage_epoch
                && existing.source_ref.as_ref() == Some(source_ref);
            return Err(M9AdmissionDiagnostics::one(if exact_duplicate {
                M9AdmissionErrorKind::DuplicateCapabilityReference
            } else {
                M9AdmissionErrorKind::ConflictingCapabilityReference
            }));
        }
        if !self.permits_capability_scope(membership, scope) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::CapabilityPolicyRejected,
            ));
        }
        let capability = M9CapabilityAuth {
            reference: request.reference,
            membership_ref: membership_ref.clone(),
            scope: scope.clone(),
            lineage_epoch: lineage_epoch.clone(),
            policy_version: M9_POLICY_VERSION.to_string(),
            source_ref: Some(source_ref.clone()),
            active: true,
        };
        self.snapshot
            .capabilities
            .insert(capability.reference.clone(), capability.clone());
        Ok(capability)
    }

    pub fn materialize_witness(
        &mut self,
        request: M9WitnessRequest,
    ) -> Result<M9WitnessAuth, M9AdmissionDiagnostics> {
        let (Some(membership_ref), Some(capability_ref), Some(source_ref)) = (
            request.membership_ref.as_ref(),
            request.capability_ref.as_ref(),
            request.source_ref.as_ref(),
        ) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        if let Some(existing) = self.snapshot.witnesses.get(&request.reference) {
            let exact_duplicate = existing.membership_ref == *membership_ref
                && existing.capability_ref == *capability_ref
                && existing.source_ref.as_ref() == Some(source_ref);
            return Err(M9AdmissionDiagnostics::one(if exact_duplicate {
                M9AdmissionErrorKind::DuplicateWitnessReference
            } else {
                M9AdmissionErrorKind::ConflictingWitnessReference
            }));
        }
        let Some(membership) = self.snapshot.memberships.get(membership_ref) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let Some(capability) = self.snapshot.capabilities.get(capability_ref) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        if !membership.active
            || !capability.active
            || capability.membership_ref != *membership_ref
            || membership.auth_residual_source_ref.as_ref() != Some(source_ref)
            || capability.source_ref.as_ref() != Some(source_ref)
            || self
                .snapshot
                .current_memberships
                .get(&(membership.principal.clone(), membership.locus.clone()))
                != Some(membership_ref)
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }
        let witness = M9WitnessAuth {
            reference: request.reference,
            membership_ref: membership_ref.clone(),
            capability_ref: capability_ref.clone(),
            source_ref: Some(source_ref.clone()),
            live: true,
        };
        self.snapshot
            .witnesses
            .insert(witness.reference.clone(), witness.clone());
        Ok(witness)
    }

    fn permits_capability_scope(
        &self,
        membership: &M9MembershipAuth,
        scope: &M9CapabilityScope,
    ) -> bool {
        let Some(outer) = self.outer_admission.as_ref() else {
            return false;
        };
        match scope {
            M9CapabilityScope::OwnerEvaluation {
                evaluation,
                owner_locus,
            } => outer
                .source_artifact
                .contains_owner_evaluation_scope(evaluation, owner_locus),
            M9CapabilityScope::ContractUpdate { module, contract } => {
                outer.residual_bindings.0.iter().any(|binding| {
                    binding.kind == ResidualObligationKind::AuthDeferred
                        && binding.module.as_deref() == Some(module)
                        && binding.contract.as_deref() == Some(contract)
                })
            }
            M9CapabilityScope::Observation {
                observer_principal,
                label,
                redaction,
                retention,
            } => {
                observer_principal == &format!("observer:{}", membership.principal)
                    && label == M9_OBSERVER_LABEL
                    && redaction == M9_OBSERVER_REDACTION
                    && retention == M9_OBSERVER_RETENTION
            }
            M9CapabilityScope::RelationTransition {
                relation,
                owner_locus,
                binding_frontier,
                ..
            } => outer.source_artifact.contains_relation_scope(
                relation,
                owner_locus,
                binding_frontier,
            ),
            M9CapabilityScope::DesignatedEvaluation {
                evaluator,
                result,
                input_frontier,
            } => outer
                .source_artifact
                .contains_designated_scope(evaluator, result, input_frontier),
            M9CapabilityScope::DesignatedRemoteInputRelease {
                producer_locus,
                evaluator,
                result,
                dependency_index,
                input_frontier,
                release_label,
                visibility,
            } => {
                membership.locus == *producer_locus
                    && outer
                        .source_artifact
                        .contains_designated_remote_input_release_scope(
                            producer_locus,
                            evaluator,
                            result,
                            *dependency_index,
                            input_frontier,
                            release_label,
                            visibility,
                        )
            }
            M9CapabilityScope::DesignatedConsumption {
                consumer,
                value_name,
                ..
            } => value_name
                .split_once('.')
                .is_some_and(|(evaluator, result)| {
                    !consumer.is_empty()
                        && outer.source_artifact.contains_designated_scope(
                            evaluator,
                            result,
                            // The consumer scope is tied to the checked value;
                            // the input frontier is checked at evaluation.
                            outer
                                .source_artifact
                                .designated_scopes
                                .iter()
                                .find_map(|(candidate_evaluator, candidate_result, frontier)| {
                                    (candidate_evaluator == evaluator && candidate_result == result)
                                        .then_some(frontier.as_str())
                                })
                                .unwrap_or(""),
                        )
                }),
        }
    }

    pub fn use_authority(&mut self, fact: M9FactUse) -> Result<(), M9AdmissionDiagnostics> {
        let Some(capability) = self.snapshot.capabilities.get(&fact.capability_ref) else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let Some(membership_ref) = fact.membership_ref.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let Some(witness_ref) = fact.witness_ref.as_ref() else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        let membership = self.snapshot.memberships.get(membership_ref);
        let witness = self.snapshot.witnesses.get(witness_ref);
        if !capability.active
            || self
                .snapshot
                .revoked_capabilities
                .contains(&fact.capability_ref)
            || membership.is_none_or(|membership| !membership.active)
            || witness.is_none_or(|witness| !witness.live)
            || capability.membership_ref != *membership_ref
            || witness.is_none_or(|witness| {
                witness.membership_ref != *membership_ref
                    || witness.capability_ref != fact.capability_ref
            })
            || fact.epoch.as_deref() != Some(capability.lineage_epoch.as_str())
            || fact
                .scope
                .as_ref()
                .is_some_and(|scope| scope != &capability.scope)
            || fact.copied_from.is_some()
            || fact.revocation_ref.is_some()
            || membership.is_none_or(|membership| {
                self.snapshot
                    .current_memberships
                    .get(&(membership.principal.clone(), membership.locus.clone()))
                    != Some(membership_ref)
            })
        {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }
        Ok(())
    }

    fn validates_revocation(&self, revocation: &M9Revocation) -> bool {
        self.snapshot
            .capabilities
            .get(&revocation.capability_ref)
            .is_some_and(|capability| {
                capability.active
                    && (revocation.source_ref.is_none()
                        || revocation.source_ref.as_ref() == capability.source_ref.as_ref())
                    && revocation.witness_ref.as_ref().is_none_or(|witness_ref| {
                        self.snapshot
                            .witnesses
                            .get(witness_ref)
                            .is_some_and(|witness| {
                                witness.live && witness.capability_ref == revocation.capability_ref
                            })
                    })
            })
    }

    pub fn revoke(&mut self, revocation: M9Revocation) -> Result<(), M9AdmissionDiagnostics> {
        if !self.validates_revocation(&revocation) {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        }
        let capability = self
            .snapshot
            .capabilities
            .get_mut(&revocation.capability_ref)
            .expect("validated revocation retains capability");
        capability.active = false;
        let mut dependent_refs = revocation.dependent_artifacts;
        for witness in self.snapshot.witnesses.values_mut() {
            if witness.capability_ref == revocation.capability_ref {
                witness.live = false;
                dependent_refs.insert(witness.reference.clone());
            }
        }
        self.snapshot
            .revoked_capabilities
            .insert(revocation.capability_ref.clone());
        self.evidence_graph
            .revoke(&revocation.capability_ref, dependent_refs);
        Ok(())
    }

    fn observer_authorization(
        &self,
        request: &M9ObserverRequest,
    ) -> Option<M9ObserverAuthorization> {
        if request.revocation_ref.is_some() {
            return None;
        }
        let membership = self.snapshot.memberships.get(&request.membership_ref)?;
        let capability = self.snapshot.capabilities.get(&request.capability_ref)?;
        let witness = self.snapshot.witnesses.get(&request.witness_ref)?;
        let M9CapabilityScope::Observation {
            observer_principal,
            label,
            redaction,
            retention,
        } = &capability.scope
        else {
            return None;
        };
        let proof_ref = membership.proof_ref.as_ref()?;
        if !membership.active
            || !capability.active
            || !witness.live
            || self
                .snapshot
                .revoked_capabilities
                .contains(&request.capability_ref)
            || membership.reference != request.membership_ref
            || self
                .snapshot
                .current_memberships
                .get(&(membership.principal.clone(), membership.locus.clone()))
                != Some(&membership.reference)
            || request.observer_principal != format!("observer:{}", membership.principal)
            || observer_principal != &request.observer_principal
            || capability.membership_ref != membership.reference
            || capability.lineage_epoch != request.epoch
            || capability.source_ref.as_ref() != membership.auth_residual_source_ref.as_ref()
            || witness.membership_ref != membership.reference
            || witness.capability_ref != capability.reference
            || witness.source_ref.as_ref() != capability.source_ref.as_ref()
            || label != M9_OBSERVER_LABEL
            || redaction != M9_OBSERVER_REDACTION
            || retention != M9_OBSERVER_RETENTION
        {
            return None;
        }
        Some(M9ObserverAuthorization {
            observer_principal: observer_principal.clone(),
            label: label.clone(),
            redaction: redaction.clone(),
            retention: retention.clone(),
            source_ref: capability.source_ref.clone()?,
            proof_ref: proof_ref.clone(),
        })
    }

    fn accepts_contract_authority(
        &self,
        authority: &M9ContractAuthorityUse,
        module: &str,
        contract: &str,
    ) -> bool {
        if authority.copied_from.is_some()
            || authority.revocation_ref.is_some()
            || authority.provider_name.is_some()
            || authority.locus_name.is_some()
            || authority.session_ref.is_some()
            || authority.target_module.as_deref() != Some(module)
        {
            return false;
        }
        let (Some(capability_ref), Some(witness_ref), Some(epoch)) = (
            authority.capability_ref.as_ref(),
            authority.witness_ref.as_ref(),
            authority.epoch.as_ref(),
        ) else {
            return false;
        };
        let Some(capability) = self.snapshot.capabilities.get(capability_ref) else {
            return false;
        };
        let Some(membership) = self.snapshot.memberships.get(&capability.membership_ref) else {
            return false;
        };
        let Some(witness) = self.snapshot.witnesses.get(witness_ref) else {
            return false;
        };
        capability.active
            && membership.active
            && self
                .snapshot
                .current_memberships
                .get(&(membership.principal.clone(), membership.locus.clone()))
                == Some(&membership.reference)
            && witness.live
            && !self.snapshot.revoked_capabilities.contains(capability_ref)
            && capability.lineage_epoch == *epoch
            && authority.membership_ref.as_deref() == Some(capability.membership_ref.as_str())
            && witness.membership_ref == capability.membership_ref
            && witness.capability_ref == capability.reference
            && matches!(
                &capability.scope,
                M9CapabilityScope::ContractUpdate {
                    module: scope_module,
                    contract: scope_contract,
                } if scope_module == module && scope_contract == contract
            )
    }
}

fn canonical_m9_capability_scope(scope: &M9CapabilityScope) -> String {
    match scope {
        M9CapabilityScope::OwnerEvaluation {
            evaluation,
            owner_locus,
        } => format!("owner_evaluation:{evaluation}:{owner_locus}"),
        M9CapabilityScope::ContractUpdate { module, contract } => {
            format!("contract_update:{module}:{contract}")
        }
        M9CapabilityScope::Observation {
            observer_principal,
            label,
            redaction,
            retention,
        } => format!("observation:{observer_principal}:{label}:{redaction}:{retention}"),
        M9CapabilityScope::RelationTransition {
            relation,
            transition,
            owner_locus,
            binding_frontier,
        } => {
            format!("relation_transition:{relation}:{transition}:{owner_locus}:{binding_frontier}")
        }
        M9CapabilityScope::DesignatedEvaluation {
            evaluator,
            result,
            input_frontier,
        } => format!("designated_evaluation:{evaluator}:{result}:{input_frontier}"),
        M9CapabilityScope::DesignatedRemoteInputRelease {
            producer_locus,
            evaluator,
            result,
            dependency_index,
            input_frontier,
            release_label,
            visibility,
        } => format!(
            "designated_remote_input_release:{producer_locus}:{evaluator}:{result}:{dependency_index}:{input_frontier}:{release_label}:{visibility}"
        ),
        M9CapabilityScope::DesignatedConsumption {
            consumer,
            value_name,
            result_version,
        } => format!("designated_consumption:{consumer}:{value_name}:{result_version}"),
    }
}

/// Immutable M8-owned payload retained across M9 contract updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9M8PayloadSnapshot {
    program_identity: CheckedProgramIdentity,
    evidence: Vec<M8AdmissionEvidence>,
}

impl M9M8PayloadSnapshot {
    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn evidence(&self) -> &[M8AdmissionEvidence] {
        &self.evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9PreconditionDelta(String);

impl M9PreconditionDelta {
    pub fn strengthens(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9CapabilityRequirementDelta(String);

impl M9CapabilityRequirementDelta {
    pub fn requires(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M9FailureDelta(Vec<String>);

impl M9FailureDelta {
    pub fn adds_declared(value: impl Into<String>) -> Self {
        Self(vec![value.into()])
    }

    pub fn added_declared(&self) -> Vec<&str> {
        self.0.iter().map(String::as_str).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M9ObservationDeltaKind {
    AddRedactedLabel,
    WidenLabel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ObservationDelta {
    kind: M9ObservationDeltaKind,
    labels: Vec<(String, String)>,
}

impl M9ObservationDelta {
    pub fn adds_redacted_label(label: impl Into<String>, redaction: impl Into<String>) -> Self {
        Self {
            kind: M9ObservationDeltaKind::AddRedactedLabel,
            labels: vec![(label.into(), redaction.into())],
        }
    }

    pub fn widens_label(label: impl Into<String>, new_label: impl Into<String>) -> Self {
        Self {
            kind: M9ObservationDeltaKind::WidenLabel,
            labels: vec![(label.into(), new_label.into())],
        }
    }

    pub fn added_redacted_labels(&self) -> Vec<(String, String)> {
        self.labels.clone()
    }

    fn weakens_policy(&self) -> bool {
        self.kind == M9ObservationDeltaKind::WidenLabel
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M9ContractDelta {
    preconditions: Vec<M9PreconditionDelta>,
    capability_requirements: Vec<M9CapabilityRequirementDelta>,
    failure: M9FailureDelta,
    observation: M9ObservationDelta,
}

impl Default for M9ObservationDelta {
    fn default() -> Self {
        Self {
            kind: M9ObservationDeltaKind::AddRedactedLabel,
            labels: Vec::new(),
        }
    }
}

impl M9ContractDelta {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_precondition(mut self, delta: M9PreconditionDelta) -> Self {
        self.preconditions.push(delta);
        self
    }

    pub fn with_capability_requirement(mut self, delta: M9CapabilityRequirementDelta) -> Self {
        self.capability_requirements.push(delta);
        self
    }

    pub fn with_failure(mut self, delta: M9FailureDelta) -> Self {
        self.failure = delta;
        self
    }

    pub fn with_observation(mut self, delta: M9ObservationDelta) -> Self {
        self.observation = delta;
        self
    }

    pub fn failure(&self) -> &M9FailureDelta {
        &self.failure
    }

    pub fn observation(&self) -> &M9ObservationDelta {
        &self.observation
    }

    fn normalized_finite_delta(&self) -> M9FiniteContractDelta {
        let mut delta = M9FiniteContractDelta::default();
        for precondition in &self.preconditions {
            delta = delta.with_precondition(precondition.as_str());
        }
        for capability in &self.capability_requirements {
            delta = delta.with_capability_requirement(capability.as_str());
        }
        for failure in &self.failure.0 {
            delta = delta.with_failure(failure);
        }
        for (label, redaction) in &self.observation.labels {
            delta = delta.with_observation(label, redaction);
        }
        delta
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9LayerDescriptor {
    layer_ref: String,
    transparent: bool,
    contract_ref: Option<String>,
    module_contract: Option<(String, String)>,
}

impl M9LayerDescriptor {
    pub fn new(layer_ref: impl Into<String>) -> Self {
        Self {
            layer_ref: layer_ref.into(),
            transparent: true,
            contract_ref: None,
            module_contract: None,
        }
    }

    pub fn non_transparent(mut self) -> Self {
        self.transparent = false;
        self
    }

    pub fn with_contract_ref(mut self, contract_ref: impl Into<String>) -> Self {
        self.contract_ref = Some(contract_ref.into());
        self
    }

    pub fn with_module_contract(
        mut self,
        module: impl Into<String>,
        contract: impl Into<String>,
    ) -> Self {
        self.module_contract = Some((module.into(), contract.into()));
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9ContractUpdateKind {
    Attach,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ContractAuthorityUse {
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    witness_ref: Option<String>,
    epoch: Option<String>,
    target_module: Option<String>,
    copied_from: Option<String>,
    revocation_ref: Option<String>,
    provider_name: Option<String>,
    locus_name: Option<String>,
    session_ref: Option<String>,
}

impl M9ContractAuthorityUse {
    pub fn from_grant_and_witness(capability: &M9CapabilityAuth, witness: &M9WitnessAuth) -> Self {
        let target_module = match &capability.scope {
            M9CapabilityScope::ContractUpdate { module, .. } => Some(module.clone()),
            M9CapabilityScope::OwnerEvaluation { .. }
            | M9CapabilityScope::Observation { .. }
            | M9CapabilityScope::RelationTransition { .. }
            | M9CapabilityScope::DesignatedEvaluation { .. }
            | M9CapabilityScope::DesignatedRemoteInputRelease { .. }
            | M9CapabilityScope::DesignatedConsumption { .. } => None,
        };
        Self {
            membership_ref: Some(capability.membership_ref.clone()),
            capability_ref: Some(capability.reference.clone()),
            witness_ref: Some(witness.reference.clone()),
            epoch: Some(capability.lineage_epoch.clone()),
            target_module,
            copied_from: None,
            revocation_ref: None,
            provider_name: None,
            locus_name: None,
            session_ref: None,
        }
    }

    pub fn from_provider_name(provider_name: impl Into<String>) -> Self {
        Self {
            membership_ref: None,
            capability_ref: None,
            witness_ref: None,
            epoch: None,
            target_module: None,
            copied_from: None,
            revocation_ref: None,
            provider_name: Some(provider_name.into()),
            locus_name: None,
            session_ref: None,
        }
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }

    pub fn with_target_contract(mut self, module: impl Into<String>) -> Self {
        self.target_module = Some(module.into());
        self
    }

    pub fn with_copied_from(mut self, capability_ref: impl Into<String>) -> Self {
        self.copied_from = Some(capability_ref.into());
        self
    }

    pub fn with_revocation_ref(mut self, revocation_ref: impl Into<String>) -> Self {
        self.revocation_ref = Some(revocation_ref.into());
        self
    }

    pub fn with_locus_name(mut self, locus_name: impl Into<String>) -> Self {
        self.locus_name = Some(locus_name.into());
        self
    }

    pub fn with_session_ref(mut self, session_ref: impl Into<String>) -> Self {
        self.session_ref = Some(session_ref.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M9ActiveContract {
    layers: Vec<M9LayerDescriptor>,
}

impl M9ActiveContract {
    pub fn layer_refs(&self) -> Vec<&str> {
        self.layers
            .iter()
            .map(|layer| layer.layer_ref.as_str())
            .collect()
    }

    pub fn contains_layer(&self, layer_ref: &str) -> bool {
        self.layers.iter().any(|layer| layer.layer_ref == layer_ref)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct M9ActivationCut {
    cut_id: String,
    before_m8_payload: M9M8PayloadSnapshot,
    after_m8_payload: M9M8PayloadSnapshot,
    removed_layers: BTreeSet<String>,
    tombstoned_capabilities: BTreeSet<String>,
    invalidated_dependents: BTreeSet<String>,
}

impl M9ActivationCut {
    pub fn preserves_m8_payload_invariant(
        &self,
        before: &M9M8PayloadSnapshot,
        after: &M9M8PayloadSnapshot,
    ) -> bool {
        &self.before_m8_payload == before && &self.after_m8_payload == after && before == after
    }

    pub fn removes_layer(&self, layer_ref: &str) -> bool {
        self.removed_layers.contains(layer_ref)
    }

    pub fn tombstones_capability(&self, capability_ref: &str) -> bool {
        self.tombstoned_capabilities.contains(capability_ref)
    }

    pub fn invalidates_dependent(&self, artifact_ref: &str) -> bool {
        self.invalidated_dependents.contains(artifact_ref)
    }
}

impl std::fmt::Debug for M9ActivationCut {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9ActivationCut")
            .field("cut_id", &self.cut_id)
            .field("removed_layer_count", &self.removed_layers.len())
            .field(
                "tombstoned_capability_count",
                &self.tombstoned_capabilities.len(),
            )
            .field(
                "invalidated_dependent_count",
                &self.invalidated_dependents.len(),
            )
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct M9ContractUpdateProvenance {
    update_ref: String,
    activation_cut_id: String,
    authority_refs: BTreeSet<String>,
}

impl M9ContractUpdateProvenance {
    pub fn update_ref(&self) -> &str {
        &self.update_ref
    }

    pub fn activation_cut_id(&self) -> &str {
        &self.activation_cut_id
    }

    pub fn contains_authority_ref(&self, authority_ref: &str) -> bool {
        self.authority_refs.contains(authority_ref)
    }
}

impl std::fmt::Debug for M9ContractUpdateProvenance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9ContractUpdateProvenance")
            .field("update_ref", &self.update_ref)
            .field("activation_cut_id", &self.activation_cut_id)
            .field("authority_ref_count", &self.authority_refs.len())
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct M9ContractUpdateOutcome {
    kind: M9ContractUpdateKind,
    layer_ref: String,
    activation_cut: Option<M9ActivationCut>,
    contract_delta: M9ContractDelta,
    provenance: M9ContractUpdateProvenance,
}

impl std::fmt::Debug for M9ContractUpdateOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9ContractUpdateOutcome")
            .field("kind", &self.kind)
            .field("layer_ref", &self.layer_ref)
            .field("activation_cut", &self.activation_cut)
            .field("contract_delta", &self.contract_delta)
            .field("provenance", &self.provenance)
            .finish()
    }
}

impl M9ContractUpdateOutcome {
    pub const fn kind(&self) -> M9ContractUpdateKind {
        self.kind
    }

    pub fn layer_ref(&self) -> &str {
        &self.layer_ref
    }

    pub fn activation_cut(&self) -> Option<&M9ActivationCut> {
        self.activation_cut.as_ref()
    }

    pub fn contract_delta(&self) -> &M9ContractDelta {
        &self.contract_delta
    }

    pub fn provenance(&self) -> &M9ContractUpdateProvenance {
        &self.provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9ContractUpdateDiagnosticsKind {
    MissingContractUpdate,
    InvalidContractAuthority,
    UndischargedVerificationObligation,
    FiniteRefinementMismatch,
    ObservationPolicyWeakening,
    MissingRemovalRevocation,
    DuplicateRemovalRevocation,
    DuplicateActiveLayer,
    MissingActiveLayer,
    UnexpectedOuterAdmissionOnContractUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ContractUpdateDiagnostic {
    kind: M9ContractUpdateDiagnosticsKind,
}

impl M9ContractUpdateDiagnostic {
    pub const fn kind(&self) -> M9ContractUpdateDiagnosticsKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ContractUpdateDiagnostics {
    primary: M9ContractUpdateDiagnostic,
}

impl M9ContractUpdateDiagnostics {
    fn one(kind: M9ContractUpdateDiagnosticsKind) -> Self {
        Self {
            primary: M9ContractUpdateDiagnostic { kind },
        }
    }

    pub fn primary(&self) -> &M9ContractUpdateDiagnostic {
        &self.primary
    }

    pub const fn activation_cut(&self) -> Option<&M9ActivationCut> {
        None
    }

    pub const fn has_runtime_success(&self) -> bool {
        false
    }

    pub const fn emits_verdict(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ContractUpdate {
    update_ref: String,
    kind: M9ContractUpdateKind,
    layer: Option<M9LayerDescriptor>,
    authority: Option<M9ContractAuthorityUse>,
    delta: M9ContractDelta,
    finite_refinement: Option<M9FiniteRefinementDischarge>,
    removal_revocations: Vec<M9Revocation>,
    outer_admission: Option<M9PreparedOuter>,
}

impl M9ContractUpdate {
    pub fn new(update_ref: impl Into<String>, kind: M9ContractUpdateKind) -> Self {
        Self {
            update_ref: update_ref.into(),
            kind,
            layer: None,
            authority: None,
            delta: M9ContractDelta::default(),
            finite_refinement: None,
            removal_revocations: Vec::new(),
            outer_admission: None,
        }
    }

    pub fn with_layer(mut self, layer: M9LayerDescriptor) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn with_authority(mut self, authority: M9ContractAuthorityUse) -> Self {
        self.authority = Some(authority);
        self
    }

    pub fn with_delta(mut self, delta: M9ContractDelta) -> Self {
        self.delta = delta;
        self
    }

    /// Binds this exact non-transparent delta to the finite verifier evidence
    /// already consumed by final M9 admission. It is rechecked before any
    /// activation cut or M8 payload observation is created.
    pub fn with_finite_refinement(mut self, discharge: M9FiniteRefinementDischarge) -> Self {
        self.finite_refinement = Some(discharge);
        self
    }

    /// A non-transparent removal is one cut: removed layer and all listed
    /// authority dependents are tombstoned together.  A separate later
    /// revocation would leave stale capability state in between.
    pub fn with_removal_revocation(mut self, revocation: M9Revocation) -> Self {
        self.removal_revocations.push(revocation);
        self
    }

    pub fn with_outer_admission(mut self, outer_admission: M9PreparedOuter) -> Self {
        self.outer_admission = Some(outer_admission);
        self
    }
}

/// A typed request for the only public M9 historic-provenance projection.
/// The current authority runtime rechecks every reference, so copying this
/// request cannot preserve observation authority after revocation or epoch
/// drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ObserverRequest {
    observer_principal: String,
    membership_ref: String,
    capability_ref: String,
    witness_ref: String,
    epoch: String,
    target_contract: Option<String>,
    revocation_ref: Option<String>,
}

impl M9ObserverRequest {
    pub fn from_grant_and_witness(capability: &M9CapabilityAuth, witness: &M9WitnessAuth) -> Self {
        let observer_principal = match &capability.scope {
            M9CapabilityScope::Observation {
                observer_principal, ..
            } => observer_principal.clone(),
            M9CapabilityScope::OwnerEvaluation { .. }
            | M9CapabilityScope::ContractUpdate { .. }
            | M9CapabilityScope::RelationTransition { .. }
            | M9CapabilityScope::DesignatedEvaluation { .. }
            | M9CapabilityScope::DesignatedRemoteInputRelease { .. }
            | M9CapabilityScope::DesignatedConsumption { .. } => String::new(),
        };
        Self {
            observer_principal,
            membership_ref: capability.membership_ref.clone(),
            capability_ref: capability.reference.clone(),
            witness_ref: witness.reference.clone(),
            epoch: capability.lineage_epoch.clone(),
            target_contract: None,
            revocation_ref: None,
        }
    }

    pub fn with_target_contract(mut self, target_contract: impl Into<String>) -> Self {
        self.target_contract = Some(target_contract.into());
        self
    }

    pub fn with_revocation_ref(mut self, revocation_ref: impl Into<String>) -> Self {
        self.revocation_ref = Some(revocation_ref.into());
        self
    }
}

pub type M9ContractObservationUse = M9ObserverRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ContractObservationRequest {
    observation_ref: String,
    observer: Option<M9ContractObservationUse>,
    update_ref: Option<String>,
    provenance_ref: Option<String>,
    unprivileged_observer: Option<String>,
}

impl M9ContractObservationRequest {
    pub fn new(observation_ref: impl Into<String>) -> Self {
        Self {
            observation_ref: observation_ref.into(),
            observer: None,
            update_ref: None,
            provenance_ref: None,
            unprivileged_observer: None,
        }
    }

    pub fn with_observer(mut self, observer: M9ContractObservationUse) -> Self {
        self.observer = Some(observer);
        self
    }

    pub fn with_update_ref(mut self, update_ref: impl Into<String>) -> Self {
        self.update_ref = Some(update_ref.into());
        self
    }

    pub fn with_provenance_ref(mut self, provenance_ref: impl Into<String>) -> Self {
        self.provenance_ref = Some(provenance_ref.into());
        self
    }

    pub fn with_unprivileged_observer(mut self, observer: impl Into<String>) -> Self {
        self.unprivileged_observer = Some(observer.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9ObserverStructuralRowKind {
    Provenance,
    Invalidation,
}

pub type M9ContractObservationRowKind = M9ObserverStructuralRowKind;

/// Deliberately opaque: a caller can correlate one observer projection with a
/// later M10 explanation without receiving an update, capability, witness, or
/// provider identifier.
#[derive(Clone, PartialEq, Eq)]
pub struct M9ObserverOpaqueToken(String);

impl std::fmt::Debug for M9ObserverOpaqueToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M9ObserverOpaqueToken(<opaque>)")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct M9ObserverSafeProvenanceRow {
    row_kind: M9ObserverStructuralRowKind,
    observer_principal: String,
    label: String,
    redaction: String,
    retention: String,
    source_ref: Option<SourceRef>,
    proof_ref: Option<M9ProofRef>,
    provenance_ref: String,
    dependent_count: usize,
    activation_cut_or_reason_token: M9ObserverOpaqueToken,
}

impl std::fmt::Debug for M9ObserverSafeProvenanceRow {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9ObserverSafeProvenanceRow")
            .field("row_kind", &self.row_kind)
            .field("observer_principal", &self.observer_principal)
            .field("label", &self.label)
            .field("redaction", &self.redaction)
            .field("retention", &self.retention)
            .field("source_ref", &self.source_ref)
            .field("proof_ref", &self.proof_ref)
            .field("provenance_ref", &self.provenance_ref)
            .field("dependent_count", &self.dependent_count)
            .field(
                "activation_cut_or_reason_token",
                &self.activation_cut_or_reason_token,
            )
            .finish()
    }
}

impl M9ObserverSafeProvenanceRow {
    pub const fn row_kind(&self) -> M9ObserverStructuralRowKind {
        self.row_kind
    }

    pub const fn kind(&self) -> M9ObserverStructuralRowKind {
        self.row_kind
    }

    pub fn observer_principal(&self) -> &str {
        &self.observer_principal
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn redaction(&self) -> &str {
        &self.redaction
    }

    pub fn retention(&self) -> &str {
        &self.retention
    }

    pub fn source_ref(&self) -> Option<&SourceRef> {
        self.source_ref.as_ref()
    }

    pub fn proof_ref(&self) -> Option<&M9ProofRef> {
        self.proof_ref.as_ref()
    }

    pub fn provenance_ref(&self) -> &str {
        &self.provenance_ref
    }

    pub const fn dependent_count(&self) -> usize {
        self.dependent_count
    }

    pub fn activation_cut_or_reason_token(&self) -> &M9ObserverOpaqueToken {
        &self.activation_cut_or_reason_token
    }

    pub fn redacted_payload(&self) -> String {
        format!(
            "row={:?};observer={};label={};redaction={};retention={};dependents={}",
            self.row_kind,
            self.observer_principal,
            self.label,
            self.redaction,
            self.retention,
            self.dependent_count,
        )
    }
}

pub type M9ContractObservationRow = M9ObserverSafeProvenanceRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9ObserverDiagnosticsKind {
    InvalidObserverAuthority,
}

pub type M9ContractObservationDiagnosticsKind = M9ObserverDiagnosticsKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ObserverDiagnostic {
    kind: M9ObserverDiagnosticsKind,
}

impl M9ObserverDiagnostic {
    pub const fn kind(&self) -> M9ObserverDiagnosticsKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ObserverDiagnostics {
    primary: M9ObserverDiagnostic,
}

pub type M9ContractObservationDiagnostics = M9ObserverDiagnostics;

impl M9ObserverDiagnostics {
    fn invalid_authority() -> Self {
        Self {
            primary: M9ObserverDiagnostic {
                kind: M9ObserverDiagnosticsKind::InvalidObserverAuthority,
            },
        }
    }

    pub fn primary(&self) -> &M9ObserverDiagnostic {
        &self.primary
    }

    pub const fn has_runtime_success(&self) -> bool {
        false
    }
}

#[derive(Clone)]
struct M9ObserverAuthorization {
    observer_principal: String,
    label: String,
    redaction: String,
    retention: String,
    source_ref: SourceRef,
    proof_ref: M9ProofRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M9ObserverStructuralRecord {
    row_kind: M9ObserverStructuralRowKind,
    dependent_count: usize,
    provenance_ref: String,
    activation_cut_or_reason_ref: String,
    activation_cut_or_reason_token: M9ObserverOpaqueToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M9AttachedLayerLineage {
    membership_ref: String,
    capability_ref: String,
    witness_ref: String,
    epoch: String,
    update_ref: String,
    activation_cut_id: String,
}

#[derive(PartialEq, Eq)]
pub struct M9ContractRuntime {
    runtime_admission: M9RuntimeAdmitted,
    m8_payload: M9M8PayloadSnapshot,
    active_contract: M9ActiveContract,
    next_activation_cut: u64,
    observer_history: Vec<M9ObserverStructuralRecord>,
    attached_layer_lineages: BTreeMap<String, M9AttachedLayerLineage>,
}

impl std::fmt::Debug for M9ContractRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("M9ContractRuntime")
            .field(
                "program_identity",
                self.runtime_admission.program_identity(),
            )
            .field("active_layer_count", &self.active_contract.layers.len())
            .field("next_activation_cut", &self.next_activation_cut)
            .field("observer_history_count", &self.observer_history.len())
            .finish()
    }
}

impl M9ContractRuntime {
    pub fn from_runtime_admitted(runtime_admission: M9RuntimeAdmitted) -> Self {
        let m8_payload = runtime_admission.m8_payload_snapshot();
        Self {
            runtime_admission,
            m8_payload,
            active_contract: M9ActiveContract::default(),
            next_activation_cut: 0,
            observer_history: Vec::new(),
            attached_layer_lineages: BTreeMap::new(),
        }
    }

    pub fn m8_payload_snapshot(&self) -> M9M8PayloadSnapshot {
        self.m8_payload.clone()
    }

    pub fn active_contract(&self) -> &M9ActiveContract {
        &self.active_contract
    }

    #[allow(dead_code)] // Crate-internal validation/M10 seam only; never a public projection.
    pub(crate) fn evidence_graph(&self) -> &M9EvidenceGraph {
        self.runtime_admission.authority_runtime.evidence_graph()
    }

    #[allow(dead_code)] // Crate-internal validation/M10 seam only; never a public projection.
    pub(crate) fn authority_snapshot(&self) -> M9AuthoritySnapshot {
        self.runtime_admission
            .authority_runtime
            .authority_snapshot()
    }

    /// Project historic M9 provenance without exposing capability, witness,
    /// membership, provider, transport, layer, or update payloads.
    pub fn observe_provenance(
        &self,
        request: M9ObserverRequest,
    ) -> Result<Vec<M9ObserverSafeProvenanceRow>, M9ObserverDiagnostics> {
        let Some(authorization) = self
            .runtime_admission
            .authority_runtime
            .observer_authorization(&request)
        else {
            return Err(M9ObserverDiagnostics::invalid_authority());
        };
        Ok(self
            .observer_history
            .iter()
            .map(|record| M9ObserverSafeProvenanceRow {
                row_kind: record.row_kind,
                observer_principal: authorization.observer_principal.clone(),
                label: authorization.label.clone(),
                redaction: authorization.redaction.clone(),
                retention: authorization.retention.clone(),
                source_ref: Some(authorization.source_ref.clone()),
                proof_ref: Some(authorization.proof_ref.clone()),
                provenance_ref: record.provenance_ref.clone(),
                dependent_count: record.dependent_count,
                activation_cut_or_reason_token: record.activation_cut_or_reason_token.clone(),
            })
            .collect())
    }

    pub fn observe_contract_evidence(
        &self,
        request: M9ContractObservationRequest,
    ) -> Result<Vec<M9ContractObservationRow>, M9ContractObservationDiagnostics> {
        let (Some(observer), Some(update_ref), Some(provenance_ref)) = (
            request.observer.as_ref(),
            request.update_ref.as_deref(),
            request.provenance_ref.as_deref(),
        ) else {
            return Err(M9ObserverDiagnostics::invalid_authority());
        };
        if request.unprivileged_observer.is_some()
            || observer
                .target_contract
                .as_deref()
                .is_some_and(|target| target != self.runtime_admission.program_identity().module())
            || !self.observer_history.iter().any(|record| {
                record.provenance_ref == update_ref
                    && record.activation_cut_or_reason_ref == provenance_ref
            })
        {
            return Err(M9ObserverDiagnostics::invalid_authority());
        }
        self.observe_provenance(observer.clone())
    }

    pub fn remove_layer(
        &mut self,
        _layer_ref: &str,
    ) -> Result<M9ContractUpdateOutcome, M9ContractUpdateDiagnostics> {
        Err(M9ContractUpdateDiagnostics::one(
            M9ContractUpdateDiagnosticsKind::MissingContractUpdate,
        ))
    }

    pub fn apply_implicit_contract_delta(
        &mut self,
        _delta: M9ContractDelta,
    ) -> Result<M9ContractUpdateOutcome, M9ContractUpdateDiagnostics> {
        Err(M9ContractUpdateDiagnostics::one(
            M9ContractUpdateDiagnosticsKind::MissingContractUpdate,
        ))
    }

    pub fn apply_contract_update(
        &mut self,
        update: M9ContractUpdate,
    ) -> Result<M9ContractUpdateOutcome, M9ContractUpdateDiagnostics> {
        if update.outer_admission.is_some() {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::UnexpectedOuterAdmissionOnContractUpdate,
            ));
        }
        let (Some(layer), Some(authority)) = (update.layer.as_ref(), update.authority.as_ref())
        else {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::InvalidContractAuthority,
            ));
        };
        match update.kind {
            M9ContractUpdateKind::Attach
                if self.active_contract.contains_layer(&layer.layer_ref) =>
            {
                return Err(M9ContractUpdateDiagnostics::one(
                    M9ContractUpdateDiagnosticsKind::DuplicateActiveLayer,
                ));
            }
            M9ContractUpdateKind::Remove
                if !self.active_contract.contains_layer(&layer.layer_ref) =>
            {
                return Err(M9ContractUpdateDiagnostics::one(
                    M9ContractUpdateDiagnosticsKind::MissingActiveLayer,
                ));
            }
            _ => {}
        }
        if update.delta.observation.weakens_policy() {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::ObservationPolicyWeakening,
            ));
        }
        if update.kind == M9ContractUpdateKind::Attach {
            let Some(discharge) = update.finite_refinement.as_ref() else {
                return Err(M9ContractUpdateDiagnostics::one(
                    M9ContractUpdateDiagnosticsKind::UndischargedVerificationObligation,
                ));
            };
            let admitted_discharge = self
                .runtime_admission
                .evidence
                .finite_refinement()
                .filter(|admitted| *admitted == discharge);
            if admitted_discharge.is_none()
                || !discharge.binds_exact_delta(&update.delta.normalized_finite_delta())
            {
                return Err(M9ContractUpdateDiagnostics::one(
                    M9ContractUpdateDiagnosticsKind::FiniteRefinementMismatch,
                ));
            }
        }
        let Some((module, contract)) = layer.module_contract.as_ref() else {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::InvalidContractAuthority,
            ));
        };
        if layer.transparent
            || layer
                .contract_ref
                .as_ref()
                .is_none_or(|contract_ref| contract_ref.is_empty())
            || module != self.runtime_admission.program_identity().module()
            || !self
                .runtime_admission
                .authority_runtime
                .accepts_contract_authority(authority, module, contract)
        {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::InvalidContractAuthority,
            ));
        }
        if update.kind == M9ContractUpdateKind::Remove && update.removal_revocations.is_empty() {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::MissingRemovalRevocation,
            ));
        }
        if update.kind == M9ContractUpdateKind::Remove
            && update
                .removal_revocations
                .iter()
                .map(|revocation| &revocation.capability_ref)
                .collect::<BTreeSet<_>>()
                .len()
                != update.removal_revocations.len()
        {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::DuplicateRemovalRevocation,
            ));
        }
        let attached_lineage = if update.kind == M9ContractUpdateKind::Remove {
            let Some(attached_lineage) = self.attached_layer_lineages.get(&layer.layer_ref) else {
                return Err(M9ContractUpdateDiagnostics::one(
                    M9ContractUpdateDiagnosticsKind::InvalidContractAuthority,
                ));
            };
            let matches_attached_authority = authority.membership_ref.as_deref()
                == Some(attached_lineage.membership_ref.as_str())
                && authority.capability_ref.as_deref()
                    == Some(attached_lineage.capability_ref.as_str())
                && authority.witness_ref.as_deref() == Some(attached_lineage.witness_ref.as_str())
                && authority.epoch.as_deref() == Some(attached_lineage.epoch.as_str());
            let matches_attached_revocation = matches!(
                update.removal_revocations.as_slice(),
                [revocation]
                    if revocation.capability_ref == attached_lineage.capability_ref
                        && revocation.witness_ref.as_deref()
                            == Some(attached_lineage.witness_ref.as_str())
            );
            if !matches_attached_authority || !matches_attached_revocation {
                return Err(M9ContractUpdateDiagnostics::one(
                    M9ContractUpdateDiagnosticsKind::InvalidContractAuthority,
                ));
            }
            Some(attached_lineage.clone())
        } else {
            None
        };
        if update.kind == M9ContractUpdateKind::Remove
            && update.removal_revocations.iter().any(|revocation| {
                !self
                    .runtime_admission
                    .authority_runtime
                    .validates_revocation(revocation)
            })
        {
            return Err(M9ContractUpdateDiagnostics::one(
                M9ContractUpdateDiagnosticsKind::InvalidContractAuthority,
            ));
        }
        let before_m8_payload = self.m8_payload.clone();
        let update_ref = update.update_ref.clone();
        let cut_id = format!("m9-activation-cut:{}", self.next_activation_cut);
        self.next_activation_cut = self.next_activation_cut.saturating_add(1);
        let tombstoned_capabilities = update
            .removal_revocations
            .iter()
            .map(|revocation| revocation.capability_ref.clone())
            .collect::<BTreeSet<_>>();
        match update.kind {
            M9ContractUpdateKind::Attach => self.active_contract.layers.push(layer.clone()),
            M9ContractUpdateKind::Remove => self
                .active_contract
                .layers
                .retain(|active| active.layer_ref != layer.layer_ref),
        }
        let capability_ref = authority
            .capability_ref
            .as_ref()
            .expect("validated contract authority retains capability ref");
        self.runtime_admission
            .authority_runtime
            .evidence_graph
            .add_dependent(capability_ref, update_ref.clone());
        for revocation in update.removal_revocations {
            self.runtime_admission
                .authority_runtime
                .revoke(revocation)
                .expect("prevalidated atomic removal revocation must remain valid");
        }
        let invalidated_dependents = tombstoned_capabilities
            .iter()
            .flat_map(|capability_ref| {
                self.runtime_admission
                    .authority_runtime
                    .evidence_graph
                    .invalidated_artifacts_for(capability_ref)
            })
            .collect::<BTreeSet<_>>();
        let authority_refs = [
            authority.membership_ref.as_ref(),
            authority.capability_ref.as_ref(),
            authority.witness_ref.as_ref(),
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
        self.observer_history.push(M9ObserverStructuralRecord {
            row_kind: M9ObserverStructuralRowKind::Provenance,
            dependent_count: invalidated_dependents.len(),
            provenance_ref: update_ref.clone(),
            activation_cut_or_reason_ref: cut_id.clone(),
            activation_cut_or_reason_token: M9ObserverOpaqueToken(format!(
                "m9-observer-safe/{cut_id}"
            )),
        });
        if update.kind == M9ContractUpdateKind::Remove {
            self.observer_history
                .extend(invalidated_dependents.iter().enumerate().map(|(index, _)| {
                    M9ObserverStructuralRecord {
                        row_kind: M9ObserverStructuralRowKind::Invalidation,
                        dependent_count: 1,
                        provenance_ref: update_ref.clone(),
                        activation_cut_or_reason_ref: cut_id.clone(),
                        activation_cut_or_reason_token: M9ObserverOpaqueToken(format!(
                            "m9-observer-safe/{cut_id}/invalidation/{index}"
                        )),
                    }
                }));
        }
        match update.kind {
            M9ContractUpdateKind::Attach => {
                let (Some(membership_ref), Some(capability_ref), Some(witness_ref), Some(epoch)) = (
                    authority.membership_ref.as_ref(),
                    authority.capability_ref.as_ref(),
                    authority.witness_ref.as_ref(),
                    authority.epoch.as_ref(),
                ) else {
                    unreachable!("accepted ContractUpdate authority retains complete lineage")
                };
                self.attached_layer_lineages.insert(
                    layer.layer_ref.clone(),
                    M9AttachedLayerLineage {
                        membership_ref: membership_ref.clone(),
                        capability_ref: capability_ref.clone(),
                        witness_ref: witness_ref.clone(),
                        epoch: epoch.clone(),
                        update_ref: update_ref.clone(),
                        activation_cut_id: cut_id.clone(),
                    },
                );
            }
            M9ContractUpdateKind::Remove => {
                let _ = attached_lineage.expect("validated active remove retains attached lineage");
                self.attached_layer_lineages.remove(&layer.layer_ref);
            }
        }
        Ok(M9ContractUpdateOutcome {
            kind: update.kind,
            layer_ref: layer.layer_ref.clone(),
            activation_cut: Some(M9ActivationCut {
                cut_id: cut_id.clone(),
                before_m8_payload,
                after_m8_payload: self.m8_payload.clone(),
                removed_layers: if update.kind == M9ContractUpdateKind::Remove {
                    BTreeSet::from([layer.layer_ref.clone()])
                } else {
                    BTreeSet::new()
                },
                tombstoned_capabilities,
                invalidated_dependents,
            }),
            contract_delta: update.delta,
            provenance: M9ContractUpdateProvenance {
                update_ref,
                activation_cut_id: cut_id,
                authority_refs,
            },
        })
    }

    pub fn apply_revocation(
        &mut self,
        revocation: M9Revocation,
    ) -> Result<(), M9AdmissionDiagnostics> {
        let Some(attached_lineage) = self
            .attached_layer_lineages
            .values()
            .find(|lineage| {
                lineage.capability_ref == revocation.capability_ref
                    && revocation.witness_ref.as_deref() == Some(lineage.witness_ref.as_str())
            })
            .cloned()
        else {
            return Err(M9AdmissionDiagnostics::one(
                M9AdmissionErrorKind::InvalidCapabilityLineage,
            ));
        };
        self.runtime_admission
            .authority_runtime
            .revoke(revocation)?;
        let invalidated_dependents = self
            .runtime_admission
            .authority_runtime
            .evidence_graph
            .invalidated_artifacts_for(&attached_lineage.capability_ref);
        self.observer_history.push(M9ObserverStructuralRecord {
            row_kind: M9ObserverStructuralRowKind::Invalidation,
            dependent_count: invalidated_dependents.len(),
            provenance_ref: attached_lineage.update_ref,
            activation_cut_or_reason_ref: attached_lineage.activation_cut_id.clone(),
            activation_cut_or_reason_token: M9ObserverOpaqueToken(format!(
                "m9-observer-safe/{}/direct-revocation",
                attached_lineage.activation_cut_id
            )),
        });
        Ok(())
    }
}

#[cfg(test)]
#[path = "m9_auth_verification_unit_tests.rs"]
mod m9_auth_verification_unit_tests;
