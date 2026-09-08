//! Dedicated static projection for the checked read-only provider profile.
//!
//! This module is deliberately separate from ordinary SYS-3 projection. It
//! derives only source-bound fragments and communication edges. It neither
//! admits an effect policy, issues an effect-use capability, installs a host
//! adapter, nor produces an executable artifact.

use mir_semantics::{
    m9_finite_refinement::M9ReadOnlyProviderEffectCoverage,
    surface_v0_pipeline::{CheckedEvaluation, CheckedEvaluationKind, CheckedSurfaceV0},
};

use super::{
    lowering::{artifact_ref, combined_loci, project_legacy_evaluation, validate_topology},
    model::{
        AuthorityRequirements, BackendRequirements, CarrierContract, CheckedCoreIdentity,
        CommunicationEdgeInput, CommunicationEdgeKind, DeclaredLogicalTopology,
        GlobalProjectionResult, LocusTag, PlacementSpecificCore, ProjectedOperationFragment,
        ProjectedOperationFragmentKind, ProjectionDiagnosticKind, ProjectionDiagnostics,
        RuntimeAdmissionStatus, RuntimeSeamRequirements, SemanticObligations,
    },
};

/// A source-derived provider static projection. The retained global projection
/// contains the actual three role fragments and two generated edges; this
/// wrapper adds exact checked coverage and an explicit inactive boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReadOnlyProviderEffectStaticProjection {
    projection: GlobalProjectionResult,
    provider_coverage: M9ReadOnlyProviderEffectCoverage,
}

impl ReadOnlyProviderEffectStaticProjection {
    pub(crate) fn checked_program_identity(
        &self,
    ) -> &mir_semantics::surface_v0_pipeline::CheckedProgramIdentity {
        self.projection.checked_program_identity()
    }

    pub(crate) fn projection(&self) -> &GlobalProjectionResult {
        &self.projection
    }

    pub(crate) fn provider_coverage(&self) -> &M9ReadOnlyProviderEffectCoverage {
        &self.provider_coverage
    }

    /// Static source coverage never constitutes activation, effect-use
    /// permission, or an executable runtime image.
    pub(crate) const fn activation_pending(&self) -> bool {
        true
    }
}

/// Project the full checked program through the dedicated static path. Legacy
/// forms use their existing lowerings, while the one checked provider form is
/// retained as distinct static roles and edges.
pub(crate) fn project_read_only_provider_effect_static(
    checked: &CheckedSurfaceV0,
    topology: &DeclaredLogicalTopology,
    provider_coverage: M9ReadOnlyProviderEffectCoverage,
) -> Result<ReadOnlyProviderEffectStaticProjection, ProjectionDiagnostics> {
    if provider_coverage.program_identity() != checked.program_identity()
        || !provider_coverage.matches_checked(checked)
        || !provider_coverage.runtime_requirement().activation_pending()
    {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::StructuralMismatch,
            "provider static coverage must match the full checked source",
        ));
    }
    validate_topology(checked, topology)?;
    let provider_evaluations = checked
        .evaluations()
        .iter()
        .filter(|evaluation| evaluation.kind() == CheckedEvaluationKind::ReadOnlyProviderEffect)
        .collect::<Vec<_>>();
    let [provider_evaluation] = provider_evaluations.as_slice() else {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::StructuralMismatch,
            "provider static projection requires exactly one checked provider effect",
        ));
    };
    let mut projection = GlobalProjectionResult::new(
        checked.program_identity().clone(),
        topology.loci().clone(),
        RuntimeAdmissionStatus::BlockedByResidual,
        BackendRequirements::from_combined_owner_source_owner_loci(&combined_loci(checked)?),
    );
    for evaluation in checked.evaluations() {
        if evaluation.kind() != CheckedEvaluationKind::ReadOnlyProviderEffect {
            project_legacy_evaluation(&mut projection, checked, evaluation)?;
        }
    }
    project_provider_effect(&mut projection, checked, provider_evaluation)?;
    projection.finalize();
    Ok(ReadOnlyProviderEffectStaticProjection {
        projection,
        provider_coverage,
    })
}

/// Re-derive and compare the complete static projection. Equality is
/// intentional: counts alone cannot validate role, edge, Core, or provenance
/// association changes.
pub(crate) fn verify_read_only_provider_effect_static_projection(
    checked: &CheckedSurfaceV0,
    topology: &DeclaredLogicalTopology,
    candidate: &ReadOnlyProviderEffectStaticProjection,
) -> Result<(), ProjectionDiagnostics> {
    let expected = project_read_only_provider_effect_static(
        checked,
        topology,
        candidate.provider_coverage.clone(),
    )?;
    if expected != *candidate {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::StructuralMismatch,
            "provider static projection must retain exact checked associations",
        ));
    }
    Ok(())
}

fn project_provider_effect(
    result: &mut GlobalProjectionResult,
    checked: &CheckedSurfaceV0,
    evaluation: &CheckedEvaluation,
) -> Result<(), ProjectionDiagnostics> {
    let core = evaluation.read_only_provider_effect_core().ok_or_else(|| {
        ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::StructuralMismatch,
            "provider projection requires its checked provider Core",
        )
    })?;
    let operation = core.operation();
    let core_ref = format!("provider-effect:{operation}");
    let requester_ref = artifact_ref(core.requester_locus(), operation, "provider-requester");
    let service_ref = artifact_ref(core.executor_locus(), operation, "provider-service");
    let consumer_ref = artifact_ref(
        core.result_consumer_locus(),
        operation,
        "provider-result-consumer",
    );
    for (locus, kind, artifact_ref) in [
        (
            core.requester_locus(),
            ProjectedOperationFragmentKind::ReadOnlyProviderEffectRequester,
            requester_ref.clone(),
        ),
        (
            core.executor_locus(),
            ProjectedOperationFragmentKind::ReadOnlyProviderEffectService,
            service_ref.clone(),
        ),
        (
            core.result_consumer_locus(),
            ProjectedOperationFragmentKind::ReadOnlyProviderEffectResultConsumer,
            consumer_ref.clone(),
        ),
    ] {
        result
            .locus_program_mut(locus)
            .add_fragment(ProjectedOperationFragment {
                operation_id: operation.to_string(),
                kind,
                source_ref: core.source_ref().clone(),
                core_ref: core_ref.clone(),
                artifact_ref: artifact_ref.clone(),
                authority_requirements: AuthorityRequirements::read_only_provider_effect(
                    operation,
                    core.source_ref(),
                ),
                declared_failure_row: evaluation.declared_failure_row().clone(),
                generated_failure_row: evaluation.generated_failure_row().clone(),
                placement: PlacementSpecificCore::ReadOnlyProviderEffect { core: core.clone() },
                locus_tag: LocusTag::checked(locus),
                fragment_ref: artifact_ref,
                checked_core_identity: CheckedCoreIdentity::fragment(
                    checked.program_identity().clone(),
                    operation,
                    kind,
                    core.source_ref().clone(),
                    None,
                    None,
                ),
                semantic_obligations: SemanticObligations::from_evaluation(evaluation),
                runtime_seam_requirements: RuntimeSeamRequirements::read_only_provider_effect(),
                designated_result_consumer_expression_leakage: false,
            });
    }
    for locus in [
        core.requester_locus(),
        core.executor_locus(),
        core.result_consumer_locus(),
    ] {
        result.locus_program_mut(locus).add_failures(
            operation,
            evaluation.declared_failure_row(),
            evaluation.generated_failure_row(),
        );
    }
    result
        .communication_plan_mut()
        .add_derived(CommunicationEdgeInput {
            operation: operation.to_string(),
            kind: CommunicationEdgeKind::ReadOnlyProviderEffectRequest,
            source_locus: core.requester_locus().to_string(),
            target_locus: core.executor_locus().to_string(),
            core_ref: core_ref.clone(),
            source_ref: core.source_ref().clone(),
            carrier_contract: CarrierContract::read_only_provider_effect_request(evaluation),
            checked_core_identity: CheckedCoreIdentity::edge(
                checked.program_identity().clone(),
                operation,
                CommunicationEdgeKind::ReadOnlyProviderEffectRequest,
                core.source_ref().clone(),
                None,
                None,
            ),
            source_fragment_ref: requester_ref,
            target_fragment_ref: service_ref.clone(),
            designated_remote_input_requirement: None,
        });
    result
        .communication_plan_mut()
        .add_derived(CommunicationEdgeInput {
            operation: operation.to_string(),
            kind: CommunicationEdgeKind::ReadOnlyProviderEffectResult,
            source_locus: core.executor_locus().to_string(),
            target_locus: core.result_consumer_locus().to_string(),
            core_ref,
            source_ref: core.source_ref().clone(),
            carrier_contract: CarrierContract::read_only_provider_effect_result(evaluation),
            checked_core_identity: CheckedCoreIdentity::edge(
                checked.program_identity().clone(),
                operation,
                CommunicationEdgeKind::ReadOnlyProviderEffectResult,
                core.source_ref().clone(),
                None,
                None,
            ),
            source_fragment_ref: service_ref,
            target_fragment_ref: consumer_ref,
            designated_remote_input_requirement: None,
        });
    Ok(())
}
