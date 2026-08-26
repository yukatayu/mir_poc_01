use mir_semantics::surface_v0_pipeline::CheckedSurfaceV0;

use super::{
    lowering::project_checked_core,
    model::{
        DeclaredLogicalTopology, GlobalProjectionResult, ProjectionDiagnosticKind,
        ProjectionDiagnostics,
    },
};

/// Recompute the pure projection and reject a candidate that diverges from it.
/// The verifier is intentionally structural: it neither enriches the candidate
/// nor turns a static plan into runtime admission.
pub(crate) fn verify_projection(
    checked: &CheckedSurfaceV0,
    topology: &DeclaredLogicalTopology,
    candidate: &GlobalProjectionResult,
) -> Result<(), ProjectionDiagnostics> {
    if candidate.checked_program_identity() != checked.program_identity() {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::CheckedProgramIdentityMismatch,
            "candidate projection is not bound to the supplied checked program",
        ));
    }
    let expected = project_checked_core(checked, topology)?;

    for expected_fragment in
        expected
            .sys4_artifact_fragments()
            .entries()
            .iter()
            .filter(|fragment| {
                fragment.fragment_kind()
                    == super::model::ProjectedOperationFragmentKind::DesignatedResultConsumer
            })
    {
        let expected_locus = expected_fragment.locus_tag().as_str();
        let matching =
            candidate
                .locus_order()
                .into_iter()
                .filter_map(|locus| {
                    candidate.locus_program(locus).and_then(|program| {
                        program.operations().single(
                        expected_fragment.operation_id(),
                        super::model::ProjectedOperationFragmentKind::DesignatedResultConsumer,
                    ).map(|fragment| (locus, fragment))
                    })
                })
                .collect::<Vec<_>>();
        let Some((actual_locus, actual)) = matching.first() else {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::MissingDerivedFragment,
                format!(
                    "candidate omits designated result consumer fragment {}",
                    expected_fragment.operation_id()
                ),
            ));
        };
        if *actual_locus != expected_locus || actual.locus_tag().as_str() != expected_locus {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::DesignatedResultConsumerMoved,
                format!(
                    "designated result consumer {} is not placed at {expected_locus}",
                    expected_fragment.operation_id()
                ),
            ));
        }
        if actual.exposes_typed_expression() || actual.exposes_raw_input() {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::DesignatedResultConsumerExpressionLeakage,
                format!(
                    "designated result consumer {} exposes producer content",
                    expected_fragment.operation_id()
                ),
            ));
        }
    }

    if candidate.backend_requirements() != expected.backend_requirements() {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::BackendEligibilityMismatch,
            "candidate backend eligibility differs from the deterministic projection",
        ));
    }
    if candidate.persistence_plan() != expected.persistence_plan() {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::PersistencePlanMismatch,
            "candidate per-locus persistence assignments differ from the projection",
        ));
    }
    if candidate
        .effect_handler_plan()
        .entries()
        .iter()
        .any(|handler| !handler.is_source_bound())
    {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::EffectHandlerProvenanceMismatch,
            "candidate effect handler lacks checked source/Core provenance",
        ));
    }

    if !candidate
        .projected_source_map()
        .matches(expected.projected_source_map())
    {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::SourceMapMismatch,
            "candidate source/Core/artifact correspondence differs from canonical projection",
        ));
    }

    for operation in owner_operations(checked) {
        if candidate.owner_locus_for_operation(operation)
            != expected.owner_locus_for_operation(operation)
        {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::OwnerOperationMoved,
                format!("owner operation {operation} is not placed at its checked owner"),
            ));
        }
    }

    let expected_edges = expected.communication_plan().edges();
    let actual_edges = candidate.communication_plan().edges();
    if actual_edges
        .iter()
        .any(|edge| !edge.is_derived_from_checked_core() || !expected_edges.contains(edge))
    {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::ExtraNonDerivedEdge,
            "candidate adds a communication edge not derived by checked Core projection",
        ));
    }
    if expected_edges
        .iter()
        .any(|edge| !actual_edges.contains(edge))
    {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::MissingDerivedEdge,
            "candidate omits a checked-Core-derived communication edge",
        ));
    }
    if candidate != &expected {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::StructuralMismatch,
            "candidate differs from deterministic canonical projection",
        ));
    }
    Ok(())
}

fn owner_operations(checked: &CheckedSurfaceV0) -> Vec<&str> {
    checked
        .evaluations()
        .iter()
        .filter_map(|evaluation| evaluation.owner_rmw_core().map(|_| evaluation.name()))
        .collect()
}
