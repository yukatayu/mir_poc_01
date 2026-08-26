//! Finite M9 verifier lane for the retained `verify finite_refinement` row.
//!
//! This module deliberately produces only a discharged-evidence record or a
//! diagnostic.  It does not create authority, execute a Core operation, or
//! mutate an M8 runtime.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedProgramIdentity, CheckedSurfaceV0, EffectKind, ResidualObligationKind,
    },
};

pub const M9_FINITE_REFINEMENT_WITNESS_SCHEMA: &str = "m9-proof-witness-required";
pub const M9_MEMBERSHIP_AUTH_PRECONDITION: &str = "MembershipAuth";
pub const M9_MEMBERSHIP_AUTH_CAPABILITY: &str = "MembershipAuth";
pub const M9_AUTH_REJECTED_FAILURE: &str = "AuthRejected";
pub const M9_AUTHORITY_OBSERVATION_LABEL: &str = "authority-private";
pub const M9_AUTHORITY_OBSERVATION_REDACTION: &str = "redact-authority-lineage";

/// Finite effect kinds retained from the M7 checked artifact.  This is a
/// closed M9 verifier carrier rather than a new runtime effect vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum M9FiniteEffectKind {
    OwnerRequest,
    OwnerLocalRead,
    OwnerWrite,
    ObserverPublish,
    RelationPublish,
    DesignatedRemoteRequest,
    DesignatedReceiptUse,
    DesignatedValuePublish,
    DesignatedResultDelivery,
    DesignatedResultConsume,
    /// Candidate-only sentinel for a requested effect with no M7 source row.
    /// It exists so the finite checker can represent and reject an actual
    /// effect-set expansion without inventing a runtime effect primitive.
    ExternalUndeclared,
}

impl From<EffectKind> for M9FiniteEffectKind {
    fn from(value: EffectKind) -> Self {
        match value {
            EffectKind::OwnerRequest => Self::OwnerRequest,
            EffectKind::OwnerLocalRead => Self::OwnerLocalRead,
            EffectKind::OwnerWrite => Self::OwnerWrite,
            EffectKind::ObserverPublish => Self::ObserverPublish,
            EffectKind::ActorReadReply => Self::ExternalUndeclared,
            EffectKind::RelationPublish => Self::RelationPublish,
            EffectKind::DesignatedRemoteRequest => Self::DesignatedRemoteRequest,
            EffectKind::DesignatedReceiptUse => Self::DesignatedReceiptUse,
            EffectKind::DesignatedValuePublish => Self::DesignatedValuePublish,
            EffectKind::DesignatedResultDelivery => Self::DesignatedResultDelivery,
            EffectKind::DesignatedResultConsume => Self::DesignatedResultConsume,
        }
    }
}

/// Normalized finite contract content.  Every collection is ordered so a
/// checked source and candidate have one stable equality/fingerprint form.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M9FiniteContract {
    preconditions: BTreeSet<String>,
    capability_requirements: BTreeSet<String>,
    failures: BTreeSet<String>,
    effects: BTreeSet<M9FiniteEffectKind>,
    observations: BTreeMap<String, String>,
}

impl M9FiniteContract {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn preconditions(&self) -> Vec<&str> {
        self.preconditions.iter().map(String::as_str).collect()
    }

    pub fn capability_requirements(&self) -> Vec<&str> {
        self.capability_requirements
            .iter()
            .map(String::as_str)
            .collect()
    }

    pub fn failures(&self) -> Vec<&str> {
        self.failures.iter().map(String::as_str).collect()
    }

    pub fn effects(&self) -> Vec<M9FiniteEffectKind> {
        self.effects.iter().copied().collect()
    }

    pub fn observations(&self) -> Vec<(&str, &str)> {
        self.observations
            .iter()
            .map(|(label, redaction)| (label.as_str(), redaction.as_str()))
            .collect()
    }

    pub fn with_precondition(mut self, precondition: impl Into<String>) -> Self {
        self.preconditions.insert(precondition.into());
        self
    }

    pub fn with_capability_requirement(mut self, capability: impl Into<String>) -> Self {
        self.capability_requirements.insert(capability.into());
        self
    }

    pub fn with_failure(mut self, failure: impl Into<String>) -> Self {
        self.failures.insert(failure.into());
        self
    }

    pub fn with_effect(mut self, effect: M9FiniteEffectKind) -> Self {
        self.effects.insert(effect);
        self
    }

    pub fn with_observation(
        mut self,
        label: impl Into<String>,
        redaction: impl Into<String>,
    ) -> Self {
        self.observations.insert(label.into(), redaction.into());
        self
    }

    fn normalized_fingerprint(&self) -> String {
        let preconditions = self
            .preconditions
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",");
        let capabilities = self
            .capability_requirements
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",");
        let failures = self.failures.iter().cloned().collect::<Vec<_>>().join(",");
        let effects = self
            .effects
            .iter()
            .map(|effect| format!("{effect:?}"))
            .collect::<Vec<_>>()
            .join(",");
        let observations = self
            .observations
            .iter()
            .map(|(label, redaction)| format!("{label}->{redaction}"))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "preconditions=[{preconditions}];capabilities=[{capabilities}];failures=[{failures}];effects=[{effects}];observations=[{observations}]"
        )
    }
}

/// Normalized finite delta expected by a verifier discharge and compared to
/// the non-transparent runtime ContractUpdate before its activation cut.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M9FiniteContractDelta {
    preconditions: BTreeSet<String>,
    capability_requirements: BTreeSet<String>,
    failures: BTreeSet<String>,
    observations: BTreeMap<String, String>,
}

impl M9FiniteContractDelta {
    pub fn preconditions(&self) -> Vec<&str> {
        self.preconditions.iter().map(String::as_str).collect()
    }

    pub fn capability_requirements(&self) -> Vec<&str> {
        self.capability_requirements
            .iter()
            .map(String::as_str)
            .collect()
    }

    pub fn failures(&self) -> Vec<&str> {
        self.failures.iter().map(String::as_str).collect()
    }

    pub fn observations(&self) -> Vec<(&str, &str)> {
        self.observations
            .iter()
            .map(|(label, redaction)| (label.as_str(), redaction.as_str()))
            .collect()
    }

    pub fn with_precondition(mut self, precondition: impl Into<String>) -> Self {
        self.preconditions.insert(precondition.into());
        self
    }

    pub fn with_capability_requirement(mut self, capability: impl Into<String>) -> Self {
        self.capability_requirements.insert(capability.into());
        self
    }

    pub fn with_failure(mut self, failure: impl Into<String>) -> Self {
        self.failures.insert(failure.into());
        self
    }

    pub fn with_observation(
        mut self,
        label: impl Into<String>,
        redaction: impl Into<String>,
    ) -> Self {
        self.observations.insert(label.into(), redaction.into());
        self
    }
}

/// A source-bound finite refinement request. `source_contract` must exactly
/// equal the contract deterministically derived from the checked artifact;
/// `candidate_contract` is the proposed non-transparent strengthening.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ContractCandidate {
    source_contract: M9FiniteContract,
    candidate_contract: M9FiniteContract,
}

impl M9ContractCandidate {
    pub fn new(source_contract: M9FiniteContract, candidate_contract: M9FiniteContract) -> Self {
        Self {
            source_contract,
            candidate_contract,
        }
    }

    pub fn from_checked_surface(checked: &CheckedSurfaceV0) -> Self {
        let source_contract = source_contract_from_checked(checked);
        Self {
            candidate_contract: source_contract.clone(),
            source_contract,
        }
    }

    pub fn source_contract(&self) -> &M9FiniteContract {
        &self.source_contract
    }

    pub fn candidate_contract(&self) -> &M9FiniteContract {
        &self.candidate_contract
    }

    pub fn with_candidate_contract(mut self, candidate_contract: M9FiniteContract) -> Self {
        self.candidate_contract = candidate_contract;
        self
    }

    /// The one finite M9 contract strengthening used by the non-transparent
    /// MembershipAuth layer.  Callers still present the full candidate to the
    /// checker; this helper only avoids duplicated assembly of its typed rows.
    pub fn membership_auth_strengthening(mut self) -> Self {
        self.candidate_contract = self
            .candidate_contract
            .with_precondition(M9_MEMBERSHIP_AUTH_PRECONDITION)
            .with_capability_requirement(M9_MEMBERSHIP_AUTH_CAPABILITY)
            .with_failure(M9_AUTH_REJECTED_FAILURE)
            .with_observation(
                M9_AUTHORITY_OBSERVATION_LABEL,
                M9_AUTHORITY_OBSERVATION_REDACTION,
            );
        self
    }
}

fn source_contract_from_checked(checked: &CheckedSurfaceV0) -> M9FiniteContract {
    let mut contract = M9FiniteContract::default();
    for evaluation in checked.evaluations() {
        for failure in evaluation.declared_failure_row().names() {
            contract.failures.insert(failure);
        }
        for failure in evaluation.generated_failure_row().names() {
            contract.failures.insert(failure);
        }
        for effect in evaluation.effect_row().entries() {
            contract.effects.insert(effect.kind().into());
        }
        let obligations = evaluation.generated_obligations();
        if obligations.contains_capability() {
            contract
                .capability_requirements
                .insert("Capability".to_string());
        }
        if obligations.contains_witness() {
            contract
                .capability_requirements
                .insert("Witness".to_string());
        }
        if obligations.contains_authority() {
            contract
                .capability_requirements
                .insert("Authority".to_string());
        }
        if let Some(designated) = evaluation.designated_core() {
            contract.observations.insert(
                format!("evaluation:{}", evaluation.name()),
                designated.observation_policy().name.clone(),
            );
        }
    }
    contract
}

fn normalized_delta(
    source: &M9FiniteContract,
    candidate: &M9FiniteContract,
) -> M9FiniteContractDelta {
    M9FiniteContractDelta {
        preconditions: candidate
            .preconditions
            .difference(&source.preconditions)
            .cloned()
            .collect(),
        capability_requirements: candidate
            .capability_requirements
            .difference(&source.capability_requirements)
            .cloned()
            .collect(),
        failures: candidate
            .failures
            .difference(&source.failures)
            .cloned()
            .collect(),
        observations: candidate
            .observations
            .iter()
            .filter(|(label, redaction)| source.observations.get(*label) != Some(*redaction))
            .map(|(label, redaction)| (label.clone(), redaction.clone()))
            .collect(),
    }
}

fn redaction_rank(redaction: &str) -> u8 {
    match redaction {
        "public" | "none" => 0,
        "conservative" => 1,
        "authority-private" => 2,
        M9_AUTHORITY_OBSERVATION_REDACTION => 3,
        _ => 0,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct M9ObligationId(String);

impl M9ObligationId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ProofWitnessSchema(String);

impl M9ProofWitnessSchema {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ProofArtifactHash(String);

impl M9ProofArtifactHash {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9FiniteRefinementEvidence {
    obligation_id: M9ObligationId,
    program_identity: Option<CheckedProgramIdentity>,
    verify_residual: Option<String>,
    theorem: Option<String>,
    witness_schema: Option<M9ProofWitnessSchema>,
    source_ref: Option<SourceRef>,
    artifact_hash: Option<M9ProofArtifactHash>,
    module_contract: Option<(String, String)>,
    replay_source: Option<String>,
}

impl M9FiniteRefinementEvidence {
    pub fn new(obligation_id: M9ObligationId) -> Self {
        Self {
            obligation_id,
            program_identity: None,
            verify_residual: None,
            theorem: None,
            witness_schema: None,
            source_ref: None,
            artifact_hash: None,
            module_contract: None,
            replay_source: None,
        }
    }

    pub fn for_program_identity(mut self, identity: CheckedProgramIdentity) -> Self {
        self.program_identity = Some(identity);
        self
    }

    pub fn for_module(mut self, module: impl Into<String>) -> Self {
        let module = module.into();
        self.program_identity = self.program_identity.map(|identity| {
            CheckedProgramIdentity::new(
                module,
                identity.source_file().to_string(),
                identity.root_source_ref().clone(),
            )
        });
        self
    }

    pub fn for_verify_residual(mut self, name: impl Into<String>) -> Self {
        self.verify_residual = Some(name.into());
        self
    }

    pub fn with_theorem(mut self, theorem: impl Into<String>) -> Self {
        self.theorem = Some(theorem.into());
        self
    }

    pub fn with_witness_schema(mut self, schema: M9ProofWitnessSchema) -> Self {
        self.witness_schema = Some(schema);
        self
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn with_artifact_hash(mut self, artifact_hash: M9ProofArtifactHash) -> Self {
        self.artifact_hash = Some(artifact_hash);
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

    pub fn with_replay_source(mut self, source: impl Into<String>) -> Self {
        self.replay_source = Some(source.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9FiniteRefinementErrorKind {
    MissingVerifyResidual,
    SourceContractMismatch,
    MissingMembershipAuthPrecondition,
    MissingMembershipAuthCapability,
    MissingAuthRejectedFailure,
    MissingAuthorityObservationRedaction,
    RemovedBaselinePrecondition,
    RemovedBaselineCapability,
    RemovedBaselineFailure,
    EffectExpansion,
    ObservationPolicyWeakening,
    ProgramIdentityMismatch,
    ResidualNameMismatch,
    TheoremMismatch,
    WitnessSchemaMismatch,
    SourceRefMismatch,
    MissingArtifactHash,
    ModuleContractMismatch,
    ReplayedEvidence,
    UnverifiedArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9FiniteRefinementDiagnostic {
    kind: M9FiniteRefinementErrorKind,
}

impl M9FiniteRefinementDiagnostic {
    pub const fn kind(&self) -> M9FiniteRefinementErrorKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9FiniteRefinementDiagnostics {
    primary: M9FiniteRefinementDiagnostic,
}

impl M9FiniteRefinementDiagnostics {
    fn one(kind: M9FiniteRefinementErrorKind) -> Self {
        Self {
            primary: M9FiniteRefinementDiagnostic { kind },
        }
    }

    pub fn primary(&self) -> &M9FiniteRefinementDiagnostic {
        &self.primary
    }

    pub const fn discharges_obligation(&self) -> bool {
        false
    }

    pub const fn emits_verdict(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9FiniteRefinementDischarge {
    obligation_id: M9ObligationId,
    program_identity: CheckedProgramIdentity,
    residual_name: String,
    source_ref: SourceRef,
    module_contract: (String, String),
    normalized_candidate_fingerprint: String,
    expected_delta: M9FiniteContractDelta,
}

impl M9FiniteRefinementDischarge {
    pub fn obligation_id(&self) -> &M9ObligationId {
        &self.obligation_id
    }

    pub const fn residual_kind(&self) -> ResidualObligationKind {
        ResidualObligationKind::VerifyDeferred
    }

    pub fn residual_name(&self) -> &str {
        &self.residual_name
    }

    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn module_contract(&self) -> (&str, &str) {
        (&self.module_contract.0, &self.module_contract.1)
    }

    pub fn normalized_candidate_fingerprint(&self) -> &str {
        &self.normalized_candidate_fingerprint
    }

    pub fn expected_delta(&self) -> &M9FiniteContractDelta {
        &self.expected_delta
    }

    pub fn binds_exact_delta(&self, delta: &M9FiniteContractDelta) -> bool {
        &self.expected_delta == delta
    }

    pub const fn grants_authority(&self) -> bool {
        false
    }

    pub const fn mutates_runtime_state(&self) -> bool {
        false
    }

    pub fn finite_profile(&self) -> &'static str {
        "OBL-026/verifier-evidence-non-authority"
    }

    pub fn lean_theorem_name(&self) -> &'static str {
        "verifier_evidence_cannot_mint_authority_or_activate_contract_update"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct M9FiniteRefinementChecker {
    _private: (),
}

impl M9FiniteRefinementChecker {
    /// Check an actual finite ContractUpdate candidate against the complete
    /// normalized M7 baseline.  A residual name or nonempty source map alone
    /// is never enough to resolve `verify finite_refinement`.
    pub fn discharge_candidate(
        &self,
        checked: &CheckedSurfaceV0,
        candidate: M9ContractCandidate,
    ) -> Result<M9FiniteRefinementDischarge, M9FiniteRefinementDiagnostics> {
        let Some(residual) = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|entry| entry.kind() == ResidualObligationKind::VerifyDeferred)
        else {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::MissingVerifyResidual,
            ));
        };
        if residual.name() != "finite_refinement" || checked.program_identity().module().is_empty()
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::UnverifiedArtifact,
            ));
        }
        let source_contract = source_contract_from_checked(checked);
        if candidate.source_contract != source_contract {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::SourceContractMismatch,
            ));
        }
        let after = &candidate.candidate_contract;
        if !source_contract
            .preconditions
            .is_subset(&after.preconditions)
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::RemovedBaselinePrecondition,
            ));
        }
        if !source_contract
            .capability_requirements
            .is_subset(&after.capability_requirements)
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::RemovedBaselineCapability,
            ));
        }
        if !source_contract.failures.is_subset(&after.failures) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::RemovedBaselineFailure,
            ));
        }
        if !after.effects.is_subset(&source_contract.effects) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::EffectExpansion,
            ));
        }
        if source_contract
            .observations
            .iter()
            .any(|(label, redaction)| {
                after
                    .observations
                    .get(label)
                    .is_none_or(|candidate_redaction| {
                        redaction_rank(candidate_redaction) < redaction_rank(redaction)
                    })
            })
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::ObservationPolicyWeakening,
            ));
        }
        if !after
            .preconditions
            .contains(M9_MEMBERSHIP_AUTH_PRECONDITION)
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::MissingMembershipAuthPrecondition,
            ));
        }
        if !after
            .capability_requirements
            .contains(M9_MEMBERSHIP_AUTH_CAPABILITY)
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::MissingMembershipAuthCapability,
            ));
        }
        if !after.failures.contains(M9_AUTH_REJECTED_FAILURE) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::MissingAuthRejectedFailure,
            ));
        }
        match after
            .observations
            .get(M9_AUTHORITY_OBSERVATION_LABEL)
            .map(String::as_str)
        {
            Some(redaction)
                if redaction_rank(redaction)
                    < redaction_rank(M9_AUTHORITY_OBSERVATION_REDACTION) =>
            {
                return Err(M9FiniteRefinementDiagnostics::one(
                    M9FiniteRefinementErrorKind::ObservationPolicyWeakening,
                ));
            }
            Some(M9_AUTHORITY_OBSERVATION_REDACTION) => {}
            _ => {
                return Err(M9FiniteRefinementDiagnostics::one(
                    M9FiniteRefinementErrorKind::MissingAuthorityObservationRedaction,
                ));
            }
        }
        Ok(M9FiniteRefinementDischarge {
            obligation_id: M9ObligationId::new(format!(
                "OBL-M9-finite-refinement-{}",
                checked.program_identity().module()
            )),
            program_identity: checked.program_identity().clone(),
            residual_name: residual.name().to_string(),
            source_ref: residual.source_ref().clone(),
            module_contract: (
                checked.program_identity().module().to_string(),
                "finite-refinement/MembershipAuth".to_string(),
            ),
            normalized_candidate_fingerprint: after.normalized_fingerprint(),
            expected_delta: normalized_delta(&source_contract, after),
        })
    }

    /// Compatibility carrier for diagnostics-only callers.  Its public
    /// string fields are deliberately insufficient to manufacture a verifier
    /// discharge; successful evidence must come from
    /// `discharge_checked_artifact` above.
    pub fn discharge(
        &self,
        checked: &CheckedSurfaceV0,
        evidence: M9FiniteRefinementEvidence,
    ) -> Result<M9FiniteRefinementDischarge, M9FiniteRefinementDiagnostics> {
        let Some(residual) = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|entry| entry.kind() == ResidualObligationKind::VerifyDeferred)
        else {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::MissingVerifyResidual,
            ));
        };
        if evidence.program_identity.as_ref() != Some(checked.program_identity()) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::ProgramIdentityMismatch,
            ));
        }
        if evidence.verify_residual.as_deref() != Some(residual.name()) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::ResidualNameMismatch,
            ));
        }
        if evidence.theorem.as_deref() != Some(residual.name()) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::TheoremMismatch,
            ));
        }
        if evidence
            .witness_schema
            .as_ref()
            .map(M9ProofWitnessSchema::as_str)
            != Some(M9_FINITE_REFINEMENT_WITNESS_SCHEMA)
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::WitnessSchemaMismatch,
            ));
        }
        if evidence.source_ref.as_ref() != Some(residual.source_ref()) {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::SourceRefMismatch,
            ));
        }
        if evidence
            .artifact_hash
            .as_ref()
            .is_none_or(|hash| hash.as_str().trim().is_empty())
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::MissingArtifactHash,
            ));
        }
        if evidence
            .replay_source
            .as_deref()
            .is_some_and(|source| source != checked.source_file())
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::ReplayedEvidence,
            ));
        }
        let Some(module_contract) = evidence.module_contract else {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::ModuleContractMismatch,
            ));
        };
        if module_contract.0 != checked.program_identity().module()
            || module_contract.1 != "finite-refinement/MembershipAuth"
        {
            return Err(M9FiniteRefinementDiagnostics::one(
                M9FiniteRefinementErrorKind::ModuleContractMismatch,
            ));
        }

        let _ = (evidence.obligation_id, module_contract);
        Err(M9FiniteRefinementDiagnostics::one(
            M9FiniteRefinementErrorKind::UnverifiedArtifact,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9LeanTheorem {
    name: String,
    contains_sorry: bool,
    contains_admit: bool,
    uses_untrusted_axiom: bool,
}

impl M9LeanTheorem {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn contains_sorry(&self) -> bool {
        self.contains_sorry
    }

    pub const fn contains_admit(&self) -> bool {
        self.contains_admit
    }

    pub const fn uses_untrusted_axiom(&self) -> bool {
        self.uses_untrusted_axiom
    }

    pub fn finite_profile(&self) -> &'static str {
        "OBL-026/verifier-evidence-non-authority"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9LeanObligationIndex {
    source: String,
}

impl M9LeanObligationIndex {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        Ok(Self {
            source: fs::read_to_string(path)?,
        })
    }

    /// Lean and runtime evidence have distinct identities.  This deliberately
    /// checks theorem existence and hole markers only; source binding belongs
    /// to `M9FiniteRefinementDischarge` above.
    pub fn theorem_named(&self, name: &str) -> Option<M9LeanTheorem> {
        let declaration = format!("theorem {name}");
        self.source.contains(&declaration).then(|| M9LeanTheorem {
            name: name.to_string(),
            contains_sorry: contains_lean_token(&self.source, "sorry"),
            contains_admit: contains_lean_token(&self.source, "admit"),
            uses_untrusted_axiom: contains_lean_token(&self.source, "axiom"),
        })
    }
}

fn contains_lean_token(source: &str, token: &str) -> bool {
    source
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .any(|candidate| candidate == token)
}
