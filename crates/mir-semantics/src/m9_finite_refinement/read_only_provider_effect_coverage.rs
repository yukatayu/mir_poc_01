//! Static checked-source coverage for the finite read-only provider profile.
//!
//! Coverage binds checked facts only. It is neither a finite-verification
//! verdict nor effect-use authority, and it does not discharge the routed
//! runtime requirement.

use crate::{
    shared_model::SourceRef,
    surface_v0_classification::SourceToCoreKind,
    surface_v0_pipeline::{CheckedSurfaceV0, ResidualObligationKind},
};

use super::{M9ReadOnlyProviderEffectContract, M9ReadOnlyProviderEffectContractError};

/// One exact executable source-to-Core association retained in static mixed
/// coverage. Deferred-policy rows are intentionally outside this executable
/// inventory.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9CheckedExecutableSourceAssociation {
    kind: SourceToCoreKind,
    core_ref: String,
    source_ref: SourceRef,
}

impl M9CheckedExecutableSourceAssociation {
    fn from_checked(entry: &crate::surface_v0_pipeline::CheckedSourceMapEntry) -> Self {
        Self {
            kind: entry.kind(),
            core_ref: entry.core_ref().to_string(),
            source_ref: entry.source_ref().clone(),
        }
    }

    pub const fn kind(&self) -> SourceToCoreKind {
        self.kind
    }

    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

/// The one retained runtime requirement for this profile. It remains routed
/// and activation-pending; this static record cannot discharge it.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ReadOnlyProviderEffectRuntimeRequirement {
    kind: ResidualObligationKind,
    operation: String,
    source_ref: SourceRef,
}

impl M9ReadOnlyProviderEffectRuntimeRequirement {
    pub const fn kind(&self) -> ResidualObligationKind {
        self.kind
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub const fn activation_pending(&self) -> bool {
        true
    }

    pub const fn discharges_runtime_requirement(&self) -> bool {
        false
    }
}

/// Typed rejection for a static source coverage record that cannot be derived
/// exactly from the complete checked program.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9ReadOnlyProviderEffectCoverageError {
    MissingCheckedEffect,
    MultipleCheckedEffects,
    InvalidCheckedEffect,
    SourceMapAssociationMismatch,
    RuntimeRequirementMismatch,
}

/// Complete checked-source coverage for one finite read-only provider effect.
///
/// This is deliberately static and non-authorizing. It retains every
/// executable source-map association in its original checked order, rather
/// than treating the provider rows as a count-only side inventory.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9ReadOnlyProviderEffectCoverage {
    program_identity: crate::surface_v0_pipeline::CheckedProgramIdentity,
    contract: M9ReadOnlyProviderEffectContract,
    executable_source_map_associations: Vec<M9CheckedExecutableSourceAssociation>,
    provider_source_map_associations: Vec<M9CheckedExecutableSourceAssociation>,
    runtime_requirement: M9ReadOnlyProviderEffectRuntimeRequirement,
}

impl M9ReadOnlyProviderEffectCoverage {
    /// Derive exact static coverage from the full checked source. This does
    /// not verify a candidate, grant effect use, or make the profile active.
    pub fn from_checked(
        checked: &CheckedSurfaceV0,
    ) -> Result<Self, M9ReadOnlyProviderEffectCoverageError> {
        let provider_evaluations = checked
            .evaluations()
            .iter()
            .filter(|evaluation| evaluation.read_only_provider_effect_core().is_some())
            .collect::<Vec<_>>();
        let [provider_evaluation] = provider_evaluations.as_slice() else {
            return Err(if provider_evaluations.is_empty() {
                M9ReadOnlyProviderEffectCoverageError::MissingCheckedEffect
            } else {
                M9ReadOnlyProviderEffectCoverageError::MultipleCheckedEffects
            });
        };
        let provider_core = provider_evaluation
            .read_only_provider_effect_core()
            .expect("filtered provider evaluation retains checked Core");
        let contract =
            M9ReadOnlyProviderEffectContract::try_from_checked(checked, provider_core.operation())
                .map_err(|error| match error {
                    M9ReadOnlyProviderEffectContractError::MissingCheckedEffect => {
                        M9ReadOnlyProviderEffectCoverageError::MissingCheckedEffect
                    }
                    M9ReadOnlyProviderEffectContractError::MultipleCheckedEffects => {
                        M9ReadOnlyProviderEffectCoverageError::MultipleCheckedEffects
                    }
                    M9ReadOnlyProviderEffectContractError::InvalidCheckedEffect => {
                        M9ReadOnlyProviderEffectCoverageError::InvalidCheckedEffect
                    }
                })?;

        let executable_source_map_associations = checked
            .source_map()
            .entries()
            .iter()
            .filter(|entry| entry.kind() != SourceToCoreKind::DeferredPolicy)
            .map(M9CheckedExecutableSourceAssociation::from_checked)
            .collect::<Vec<_>>();
        let provider_source_map_associations = executable_source_map_associations
            .iter()
            .filter(|association| is_provider_source_to_core_kind(association.kind()))
            .cloned()
            .collect::<Vec<_>>();
        let expected_provider_associations = expected_provider_source_map_associations(&contract);
        if provider_source_map_associations != expected_provider_associations {
            return Err(M9ReadOnlyProviderEffectCoverageError::SourceMapAssociationMismatch);
        }

        let matching_requirements = checked
            .residual_obligations()
            .entries()
            .iter()
            .filter(|entry| {
                entry.kind() == ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported
                    && entry.name() == contract.operation()
            })
            .collect::<Vec<_>>();
        let [requirement] = matching_requirements.as_slice() else {
            return Err(M9ReadOnlyProviderEffectCoverageError::RuntimeRequirementMismatch);
        };
        if requirement.source_ref() != contract.source_ref() {
            return Err(M9ReadOnlyProviderEffectCoverageError::RuntimeRequirementMismatch);
        }

        Ok(Self {
            program_identity: checked.program_identity().clone(),
            contract,
            executable_source_map_associations,
            provider_source_map_associations,
            runtime_requirement: M9ReadOnlyProviderEffectRuntimeRequirement {
                kind: requirement.kind(),
                operation: requirement.name().to_string(),
                source_ref: requirement.source_ref().clone(),
            },
        })
    }

    pub fn program_identity(&self) -> &crate::surface_v0_pipeline::CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn contract(&self) -> &M9ReadOnlyProviderEffectContract {
        &self.contract
    }

    pub fn executable_source_map_associations(&self) -> &[M9CheckedExecutableSourceAssociation] {
        &self.executable_source_map_associations
    }

    pub fn provider_source_map_associations(&self) -> &[M9CheckedExecutableSourceAssociation] {
        &self.provider_source_map_associations
    }

    pub fn runtime_requirement(&self) -> &M9ReadOnlyProviderEffectRuntimeRequirement {
        &self.runtime_requirement
    }

    pub fn matches_checked(&self, checked: &CheckedSurfaceV0) -> bool {
        matches!(Self::from_checked(checked), Ok(coverage) if coverage == *self)
    }

    pub const fn grants_authority(&self) -> bool {
        false
    }

    pub const fn permits_effect_use(&self) -> bool {
        false
    }

    pub const fn discharges_runtime_requirement(&self) -> bool {
        false
    }
}

fn is_provider_source_to_core_kind(kind: SourceToCoreKind) -> bool {
    matches!(
        kind,
        SourceToCoreKind::ReadOnlyProviderEffectRequest
            | SourceToCoreKind::ReadOnlyProviderEffectInvocation
            | SourceToCoreKind::ReadOnlyProviderEffectResult
            | SourceToCoreKind::ReadOnlyProviderEffectResultConsume
    )
}

fn expected_provider_source_map_associations(
    contract: &M9ReadOnlyProviderEffectContract,
) -> Vec<M9CheckedExecutableSourceAssociation> {
    let operation = contract.operation();
    let requester = contract.requester_locus();
    let executor = contract.executor_locus();
    let consumer = contract.result_consumer_locus();
    [
        (SourceToCoreKind::ReadOnlyProviderEffectRequest, "request"),
        (
            SourceToCoreKind::ReadOnlyProviderEffectInvocation,
            "invocation",
        ),
        (SourceToCoreKind::ReadOnlyProviderEffectResult, "result"),
        (
            SourceToCoreKind::ReadOnlyProviderEffectResultConsume,
            "result-consume",
        ),
    ]
    .into_iter()
    .map(|(kind, suffix)| M9CheckedExecutableSourceAssociation {
        kind,
        core_ref: format!(
            "{operation}:provider-effect-{suffix}:requester={requester}:executor={executor}:consumer={consumer}"
        ),
        source_ref: contract.source_ref().clone(),
    })
    .collect()
}
