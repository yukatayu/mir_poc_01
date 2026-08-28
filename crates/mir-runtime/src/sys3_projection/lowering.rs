use std::collections::BTreeSet;

use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedEvaluation, CheckedEvaluationKind, CheckedSurfaceV0, RelationAnchorCore,
        ResidualObligationKind,
    },
};

use super::model::{
    AuthorityRequirements, BackendRequirements, CarrierContract, CheckedCoreIdentity,
    CommunicationEdgeInput, CommunicationEdgeKind, ConsumerRelationProjectionDescriptor,
    DeclaredLogicalTopology, EffectHandlerInput, EffectHandlerKind, GlobalProjectionResult,
    LocusTag, PlacementSpecificCore, ProjectedOperationFragment, ProjectedOperationFragmentKind,
    ProjectedRelation, ProjectedRelationAnchor, ProjectionDiagnosticKind, ProjectionDiagnostics,
    RuntimeAdmissionStatus, RuntimeSeamRequirements, SemanticObligations,
};

pub(crate) fn project_checked_core(
    checked: &CheckedSurfaceV0,
    topology: &DeclaredLogicalTopology,
) -> Result<GlobalProjectionResult, ProjectionDiagnostics> {
    validate_topology(checked, topology)?;
    let runtime_admission_status = if checked.residual_obligations().is_empty() {
        RuntimeAdmissionStatus::AwaitingRuntimeSeam
    } else {
        RuntimeAdmissionStatus::BlockedByResidual
    };
    let mut result = GlobalProjectionResult::new(
        checked.program_identity().clone(),
        topology.loci().clone(),
        runtime_admission_status,
        BackendRequirements::from_combined_owner_source_owner_loci(&combined_loci(checked)),
    );
    for evaluation in checked.evaluations() {
        match evaluation.kind() {
            CheckedEvaluationKind::OwnerRmw => project_owner(&mut result, checked, evaluation),
            CheckedEvaluationKind::PublishRelation => {
                project_relation(&mut result, checked, evaluation)
            }
            CheckedEvaluationKind::DesignatedPublishValue => {
                project_designated(&mut result, checked, evaluation)
            }
            CheckedEvaluationKind::DesignatedResultConsume => {
                project_designated_result_consumer(&mut result, checked, evaluation)
            }
            CheckedEvaluationKind::ConsumerLocalProjection => {}
        }
    }
    result.finalize();
    Ok(result)
}

fn validate_topology(
    checked: &CheckedSurfaceV0,
    topology: &DeclaredLogicalTopology,
) -> Result<(), ProjectionDiagnostics> {
    if checked.program_identity() != topology.checked_program_identity() {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::CheckedProgramIdentityMismatch,
            "logical topology belongs to a different checked program identity",
        ));
    }
    let required = required_loci(checked);
    if let Some(missing) = required.difference(topology.loci()).next() {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::MissingRequiredLocus,
            format!("checked Core requires logical locus {missing}"),
        ));
    }
    if let Some(unknown) = topology.loci().difference(&required).next() {
        return Err(ProjectionDiagnostics::one(
            ProjectionDiagnosticKind::UnknownDeclaredLocus,
            format!("logical topology introduces unknown locus {unknown}"),
        ));
    }
    Ok(())
}

fn required_loci(checked: &CheckedSurfaceV0) -> BTreeSet<String> {
    let mut loci = checked
        .static_environment()
        .loci()
        .iter()
        .map(|locus| locus.name().to_string())
        .collect::<BTreeSet<_>>();
    for evaluation in checked.evaluations() {
        match evaluation.kind() {
            CheckedEvaluationKind::OwnerRmw => {
                let core = evaluation.owner_rmw_core().expect("checked owner Core");
                loci.insert(core.authority_origin_locus().to_string());
                loci.insert(core.owner_locus().to_string());
            }
            CheckedEvaluationKind::PublishRelation => {
                let core = evaluation.relation_core().expect("checked relation Core");
                loci.insert(core.owner_locus().to_string());
                for anchor in [core.primary(), core.fallback()] {
                    if let Some(locus) = anchor.anchor_locus() {
                        loci.insert(locus.to_string());
                    }
                }
                if let Some(consumer) = core.consumer_projection_locus() {
                    loci.insert(consumer.to_string());
                }
            }
            CheckedEvaluationKind::DesignatedPublishValue => {
                let core = evaluation
                    .designated_core()
                    .expect("checked designated Core");
                loci.insert(core.evaluator().to_string());
                for dependency in core.generated_remote_input_dependencies() {
                    loci.insert(dependency.source_owner_locus().to_string());
                }
            }
            CheckedEvaluationKind::DesignatedResultConsume => {
                let core = evaluation
                    .designated_result_consumer_core()
                    .expect("checked designated result consumer Core");
                loci.insert(core.consumer_locus().to_string());
                loci.insert(core.evaluator().to_string());
            }
            CheckedEvaluationKind::ConsumerLocalProjection => {}
        }
    }
    loci
}

fn combined_loci(checked: &CheckedSurfaceV0) -> BTreeSet<String> {
    let mut loci = BTreeSet::new();
    for evaluation in checked.evaluations() {
        match evaluation.kind() {
            CheckedEvaluationKind::OwnerRmw => {
                loci.insert(
                    evaluation
                        .owner_rmw_core()
                        .expect("checked owner Core")
                        .owner_locus()
                        .to_string(),
                );
            }
            CheckedEvaluationKind::PublishRelation => {
                loci.insert(
                    evaluation
                        .relation_core()
                        .expect("checked relation Core")
                        .owner_locus()
                        .to_string(),
                );
            }
            CheckedEvaluationKind::DesignatedPublishValue => {
                for dependency in evaluation
                    .designated_core()
                    .expect("checked designated Core")
                    .generated_remote_input_dependencies()
                {
                    loci.insert(dependency.source_owner_locus().to_string());
                }
            }
            CheckedEvaluationKind::DesignatedResultConsume => {}
            CheckedEvaluationKind::ConsumerLocalProjection => {}
        }
    }
    loci
}

fn effect_kinds(
    evaluation: &CheckedEvaluation,
) -> Vec<mir_semantics::surface_v0_pipeline::EffectKind> {
    evaluation
        .effect_row()
        .entries()
        .iter()
        .map(|entry| entry.kind())
        .collect()
}

fn project_owner(
    result: &mut GlobalProjectionResult,
    checked: &CheckedSurfaceV0,
    evaluation: &CheckedEvaluation,
) {
    let core = evaluation.owner_rmw_core().expect("owner checked Core");
    let operation = evaluation.name();
    let owner = core.owner_locus();
    let origin = core.authority_origin_locus();
    let owner_artifact_ref = artifact_ref(owner, operation, "owner-rmw");
    let local_state_schemas = checked
        .static_environment()
        .indexed_state_schema(core.target().namespace())
        .into_iter()
        .cloned()
        .collect();
    result
        .locus_program_mut(owner)
        .add_fragment(ProjectedOperationFragment {
            operation_id: operation.to_string(),
            kind: ProjectedOperationFragmentKind::OwnerRmwExecution,
            source_ref: evaluation.source_ref().clone(),
            core_ref: format!("owner-rmw:{operation}"),
            artifact_ref: owner_artifact_ref.clone(),
            authority_requirements: AuthorityRequirements::standard(
                operation,
                evaluation.source_ref(),
            ),
            declared_failure_row: evaluation.declared_failure_row().clone(),
            generated_failure_row: evaluation.generated_failure_row().clone(),
            placement: PlacementSpecificCore::OwnerRmw {
                core: core.clone(),
                local_state_schemas,
            },
            locus_tag: LocusTag::checked(owner),
            fragment_ref: owner_artifact_ref.clone(),
            checked_core_identity: CheckedCoreIdentity::fragment(
                checked.program_identity().clone(),
                operation,
                ProjectedOperationFragmentKind::OwnerRmwExecution,
                evaluation.source_ref().clone(),
                None,
                None,
            ),
            semantic_obligations: SemanticObligations::from_evaluation(evaluation),
            runtime_seam_requirements: RuntimeSeamRequirements::default(),
            designated_result_consumer_expression_leakage: false,
        });
    result.locus_program_mut(owner).add_failures(
        operation,
        evaluation.declared_failure_row(),
        evaluation.generated_failure_row(),
    );
    result.effect_handler_plan_mut().add(EffectHandlerInput {
        operation: operation.to_string(),
        kind: EffectHandlerKind::OwnerService,
        locus: owner.to_string(),
        source_ref: evaluation.source_ref().clone(),
        core_ref: format!("owner-rmw:{operation}"),
        effect_kinds: effect_kinds(evaluation),
        declared_failure_row: evaluation.declared_failure_row().clone(),
        generated_failure_row: evaluation.generated_failure_row().clone(),
        checked_core_identity: CheckedCoreIdentity::fragment(
            checked.program_identity().clone(),
            operation,
            ProjectedOperationFragmentKind::OwnerRmwExecution,
            evaluation.source_ref().clone(),
            None,
            None,
        ),
    });
    result.observation_plan_mut().add_local_fragment(
        evaluation.source_ref().clone(),
        format!("owner-rmw:{operation}"),
        owner_artifact_ref.clone(),
        ProjectedOperationFragmentKind::OwnerRmwExecution,
    );
    if origin != owner {
        let origin_artifact_ref = artifact_ref(origin, operation, "owner-request");
        let signature = checked
            .static_environment()
            .evaluation_signature_by_identity(
                operation,
                CheckedEvaluationKind::OwnerRmw,
                Some(owner),
            )
            .expect("checked owner operation has signature")
            .clone();
        result
            .locus_program_mut(origin)
            .add_fragment(ProjectedOperationFragment {
                operation_id: operation.to_string(),
                kind: ProjectedOperationFragmentKind::OwnerRequestInvocation,
                source_ref: evaluation.source_ref().clone(),
                core_ref: format!("owner-rmw:{operation}"),
                artifact_ref: origin_artifact_ref.clone(),
                authority_requirements: AuthorityRequirements::standard(
                    operation,
                    evaluation.source_ref(),
                ),
                declared_failure_row: evaluation.declared_failure_row().clone(),
                generated_failure_row: evaluation.generated_failure_row().clone(),
                placement: PlacementSpecificCore::OwnerRequest {
                    signature,
                    origin_locus: origin.to_string(),
                    target_owner_locus: owner.to_string(),
                },
                locus_tag: LocusTag::checked(origin),
                fragment_ref: origin_artifact_ref.clone(),
                checked_core_identity: CheckedCoreIdentity::fragment(
                    checked.program_identity().clone(),
                    operation,
                    ProjectedOperationFragmentKind::OwnerRequestInvocation,
                    evaluation.source_ref().clone(),
                    None,
                    None,
                ),
                semantic_obligations: SemanticObligations::from_evaluation(evaluation),
                runtime_seam_requirements: RuntimeSeamRequirements::default(),
                designated_result_consumer_expression_leakage: false,
            });
        result.observation_plan_mut().add_local_fragment(
            evaluation.source_ref().clone(),
            format!("owner-request:{operation}"),
            origin_artifact_ref.clone(),
            ProjectedOperationFragmentKind::OwnerRequestInvocation,
        );
        result
            .communication_plan_mut()
            .add_derived(CommunicationEdgeInput {
                operation: operation.to_string(),
                kind: CommunicationEdgeKind::OwnerRequest,
                source_locus: origin.to_string(),
                target_locus: owner.to_string(),
                core_ref: format!("owner-rmw:{operation}"),
                source_ref: evaluation.source_ref().clone(),
                carrier_contract: CarrierContract::owner_request(evaluation),
                checked_core_identity: CheckedCoreIdentity::edge(
                    checked.program_identity().clone(),
                    operation,
                    CommunicationEdgeKind::OwnerRequest,
                    evaluation.source_ref().clone(),
                    None,
                    None,
                ),
                source_fragment_ref: origin_artifact_ref.clone(),
                target_fragment_ref: owner_artifact_ref.clone(),
            });
        result
            .communication_plan_mut()
            .add_derived(CommunicationEdgeInput {
                operation: operation.to_string(),
                kind: CommunicationEdgeKind::OwnerReplyReceipt,
                source_locus: owner.to_string(),
                target_locus: origin.to_string(),
                core_ref: format!("owner-rmw:{operation}"),
                source_ref: evaluation.source_ref().clone(),
                carrier_contract: CarrierContract::owner_reply(evaluation),
                checked_core_identity: CheckedCoreIdentity::edge(
                    checked.program_identity().clone(),
                    operation,
                    CommunicationEdgeKind::OwnerReplyReceipt,
                    evaluation.source_ref().clone(),
                    None,
                    None,
                ),
                source_fragment_ref: owner_artifact_ref.clone(),
                target_fragment_ref: origin_artifact_ref.clone(),
            });
    }
}

fn project_relation(
    result: &mut GlobalProjectionResult,
    checked: &CheckedSurfaceV0,
    evaluation: &CheckedEvaluation,
) {
    let core = evaluation.relation_core().expect("relation checked Core");
    let name = evaluation.name();
    let owner = core.owner_locus();
    let owner_artifact_ref = artifact_ref(owner, name, "relation-publication");
    result
        .locus_program_mut(owner)
        .add_fragment(ProjectedOperationFragment {
            operation_id: name.to_string(),
            kind: ProjectedOperationFragmentKind::RelationPublication,
            source_ref: evaluation.source_ref().clone(),
            core_ref: format!("relation:{name}"),
            artifact_ref: owner_artifact_ref.clone(),
            authority_requirements: AuthorityRequirements::empty(),
            declared_failure_row: evaluation.declared_failure_row().clone(),
            generated_failure_row: evaluation.generated_failure_row().clone(),
            placement: PlacementSpecificCore::RelationOwner { core: core.clone() },
            locus_tag: LocusTag::checked(owner),
            fragment_ref: owner_artifact_ref.clone(),
            checked_core_identity: CheckedCoreIdentity::fragment(
                checked.program_identity().clone(),
                name,
                ProjectedOperationFragmentKind::RelationPublication,
                evaluation.source_ref().clone(),
                None,
                None,
            ),
            semantic_obligations: SemanticObligations::from_evaluation(evaluation),
            runtime_seam_requirements: RuntimeSeamRequirements::default(),
            designated_result_consumer_expression_leakage: false,
        });
    result.observation_plan_mut().add_local_fragment(
        evaluation.source_ref().clone(),
        format!("relation:{name}"),
        owner_artifact_ref.clone(),
        ProjectedOperationFragmentKind::RelationPublication,
    );
    if let Some(consumer) = core.consumer_projection_locus() {
        let consumer_artifact_ref = artifact_ref(consumer, name, "consumer-projection");
        result
            .locus_program_mut(consumer)
            .add_fragment(ProjectedOperationFragment {
                operation_id: name.to_string(),
                kind: ProjectedOperationFragmentKind::ConsumerLocalRelationProjection,
                source_ref: evaluation.source_ref().clone(),
                core_ref: format!("relation-consumer:{name}"),
                artifact_ref: consumer_artifact_ref.clone(),
                authority_requirements: AuthorityRequirements::empty(),
                declared_failure_row: evaluation.declared_failure_row().clone(),
                generated_failure_row: evaluation.generated_failure_row().clone(),
                placement: PlacementSpecificCore::RelationConsumer {
                    descriptor: ConsumerRelationProjectionDescriptor {
                        source_relation: name.to_string(),
                        owner_locus: owner.to_string(),
                        consumer_locus: consumer.to_string(),
                        source_ref: evaluation.source_ref().clone(),
                    },
                },
                locus_tag: LocusTag::checked(consumer),
                fragment_ref: consumer_artifact_ref.clone(),
                checked_core_identity: CheckedCoreIdentity::fragment(
                    checked.program_identity().clone(),
                    name,
                    ProjectedOperationFragmentKind::ConsumerLocalRelationProjection,
                    evaluation.source_ref().clone(),
                    None,
                    None,
                ),
                semantic_obligations: SemanticObligations::from_evaluation(evaluation),
                runtime_seam_requirements: RuntimeSeamRequirements::default(),
                designated_result_consumer_expression_leakage: false,
            });
        result.observation_plan_mut().add_local_fragment(
            evaluation.source_ref().clone(),
            format!("relation-consumer:{name}"),
            consumer_artifact_ref.clone(),
            ProjectedOperationFragmentKind::ConsumerLocalRelationProjection,
        );
        if owner != consumer {
            result
                .communication_plan_mut()
                .add_derived(CommunicationEdgeInput {
                    operation: name.to_string(),
                    kind: CommunicationEdgeKind::RelationProjectionPublication,
                    source_locus: owner.to_string(),
                    target_locus: consumer.to_string(),
                    core_ref: format!("relation:{name}"),
                    source_ref: evaluation.source_ref().clone(),
                    carrier_contract: CarrierContract::relation_publication(evaluation),
                    checked_core_identity: CheckedCoreIdentity::edge(
                        checked.program_identity().clone(),
                        name,
                        CommunicationEdgeKind::RelationProjectionPublication,
                        evaluation.source_ref().clone(),
                        None,
                        None,
                    ),
                    source_fragment_ref: owner_artifact_ref.clone(),
                    target_fragment_ref: consumer_artifact_ref.clone(),
                });
        }
    }
    result.relation_graph_mut().add_relation(ProjectedRelation {
        name: name.to_string(),
        owner_locus: owner.to_string(),
        subject: core.subject().to_string(),
        subject_type: core.subject_type().to_string(),
        primary_anchor: projected_anchor(core.primary(), evaluation.source_ref()),
        fallback_anchor: projected_anchor(core.fallback(), evaluation.source_ref()),
        binding_frontier: core.binding_frontier().clone(),
        consumer_locus: core.consumer_projection_locus().map(str::to_string),
        residual_source_refs: relation_residual_source_refs(checked, name),
    });
    result.persistence_plan_mut().add_relation(name);
}

fn projected_anchor(
    anchor: &RelationAnchorCore,
    source_ref: &SourceRef,
) -> ProjectedRelationAnchor {
    ProjectedRelationAnchor {
        anchor: anchor.anchor().to_string(),
        anchor_locus: anchor.anchor_locus().map(str::to_string),
        anchor_locus_source_ref: anchor.anchor_locus_source_ref().cloned(),
        epoch: anchor.epoch().to_string(),
        transform: anchor.transform().clone(),
        source_ref: source_ref.clone(),
    }
}

fn relation_residual_source_refs(
    checked: &CheckedSurfaceV0,
    name: &str,
) -> Vec<(ResidualObligationKind, SourceRef)> {
    checked
        .residual_obligations()
        .entries()
        .iter()
        .filter(|entry| entry.name() == name)
        .map(|entry| (entry.kind(), entry.source_ref().clone()))
        .collect()
}

fn project_designated(
    result: &mut GlobalProjectionResult,
    checked: &CheckedSurfaceV0,
    evaluation: &CheckedEvaluation,
) {
    let core = evaluation
        .designated_core()
        .expect("designated checked Core");
    let operation = format!("{}.{}", core.evaluator(), core.result());
    let evaluator = core.evaluator();
    let evaluator_artifact_ref = artifact_ref(evaluator, &operation, "designated-evaluation");
    result
        .locus_program_mut(evaluator)
        .add_fragment(ProjectedOperationFragment {
            operation_id: operation.clone(),
            kind: ProjectedOperationFragmentKind::DesignatedEvaluation,
            source_ref: evaluation.source_ref().clone(),
            core_ref: format!("designated:{operation}"),
            artifact_ref: evaluator_artifact_ref.clone(),
            authority_requirements: AuthorityRequirements::designated(
                &operation,
                evaluation.source_ref(),
            ),
            declared_failure_row: evaluation.declared_failure_row().clone(),
            generated_failure_row: evaluation.generated_failure_row().clone(),
            placement: PlacementSpecificCore::DesignatedEvaluator { core: core.clone() },
            locus_tag: LocusTag::checked(evaluator),
            fragment_ref: evaluator_artifact_ref.clone(),
            checked_core_identity: CheckedCoreIdentity::fragment(
                checked.program_identity().clone(),
                &operation,
                ProjectedOperationFragmentKind::DesignatedEvaluation,
                evaluation.source_ref().clone(),
                None,
                None,
            ),
            semantic_obligations: SemanticObligations::from_evaluation(evaluation),
            runtime_seam_requirements: RuntimeSeamRequirements::default(),
            designated_result_consumer_expression_leakage: false,
        });
    result.effect_handler_plan_mut().add(EffectHandlerInput {
        operation: operation.clone(),
        kind: EffectHandlerKind::DesignatedEvaluator,
        locus: evaluator.to_string(),
        source_ref: evaluation.source_ref().clone(),
        core_ref: format!("designated:{operation}"),
        effect_kinds: effect_kinds(evaluation),
        declared_failure_row: evaluation.declared_failure_row().clone(),
        generated_failure_row: evaluation.generated_failure_row().clone(),
        checked_core_identity: CheckedCoreIdentity::fragment(
            checked.program_identity().clone(),
            &operation,
            ProjectedOperationFragmentKind::DesignatedEvaluation,
            evaluation.source_ref().clone(),
            None,
            None,
        ),
    });
    result.observation_plan_mut().add_local_fragment(
        evaluation.source_ref().clone(),
        format!("designated:{operation}"),
        evaluator_artifact_ref.clone(),
        ProjectedOperationFragmentKind::DesignatedEvaluation,
    );
    for (dependency_ordinal, dependency) in core
        .generated_remote_input_dependencies()
        .iter()
        .enumerate()
    {
        let source_owner = dependency.source_owner_locus();
        let source_ref = dependency.typed_state_read().source_ref();
        let source_artifact_ref = format!(
            "{}:{dependency_ordinal}",
            artifact_ref(source_owner, &operation, "designated-source-read")
        );
        let local_state_schemas = checked
            .static_environment()
            .indexed_state_schema(dependency.typed_state_read().namespace())
            .into_iter()
            .cloned()
            .collect();
        result
            .locus_program_mut(source_owner)
            .add_fragment(ProjectedOperationFragment {
                operation_id: operation.clone(),
                kind: ProjectedOperationFragmentKind::DesignatedRemoteInputService,
                source_ref: source_ref.clone(),
                core_ref: format!(
                    "designated-source:{operation}:{}",
                    dependency.typed_state_read().namespace()
                ),
                artifact_ref: source_artifact_ref.clone(),
                authority_requirements: AuthorityRequirements::designated(&operation, &source_ref),
                declared_failure_row: evaluation.declared_failure_row().clone(),
                generated_failure_row: evaluation.generated_failure_row().clone(),
                placement: PlacementSpecificCore::DesignatedSource {
                    dependency: dependency.clone(),
                    local_state_schemas,
                },
                locus_tag: LocusTag::checked(source_owner),
                fragment_ref: source_artifact_ref.clone(),
                checked_core_identity: CheckedCoreIdentity::fragment(
                    checked.program_identity().clone(),
                    &operation,
                    ProjectedOperationFragmentKind::DesignatedRemoteInputService,
                    source_ref.clone(),
                    Some(dependency_ordinal),
                    Some(dependency.clone()),
                ),
                semantic_obligations: SemanticObligations::from_evaluation(evaluation),
                runtime_seam_requirements: RuntimeSeamRequirements::default(),
                designated_result_consumer_expression_leakage: false,
            });
        result.effect_handler_plan_mut().add(EffectHandlerInput {
            operation: operation.clone(),
            kind: EffectHandlerKind::DesignatedSourceService,
            locus: source_owner.to_string(),
            source_ref: source_ref.clone(),
            core_ref: format!(
                "designated-source:{operation}:{}",
                dependency.typed_state_read().namespace()
            ),
            effect_kinds: effect_kinds(evaluation),
            declared_failure_row: evaluation.declared_failure_row().clone(),
            generated_failure_row: evaluation.generated_failure_row().clone(),
            checked_core_identity: CheckedCoreIdentity::fragment(
                checked.program_identity().clone(),
                &operation,
                ProjectedOperationFragmentKind::DesignatedRemoteInputService,
                source_ref.clone(),
                Some(dependency_ordinal),
                Some(dependency.clone()),
            ),
        });
        result.observation_plan_mut().add_local_fragment(
            source_ref.clone(),
            format!("designated-source:{operation}"),
            source_artifact_ref.clone(),
            ProjectedOperationFragmentKind::DesignatedRemoteInputService,
        );
        if evaluator != source_owner {
            result
                .communication_plan_mut()
                .add_derived(CommunicationEdgeInput {
                    operation: operation.clone(),
                    kind: CommunicationEdgeKind::DesignatedInputRequest,
                    source_locus: evaluator.to_string(),
                    target_locus: source_owner.to_string(),
                    core_ref: format!(
                        "designated-input:{operation}:{}",
                        dependency.typed_state_read().namespace()
                    ),
                    source_ref: source_ref.clone(),
                    carrier_contract: CarrierContract::designated_request(
                        &operation, evaluation, dependency,
                    ),
                    checked_core_identity: CheckedCoreIdentity::edge(
                        checked.program_identity().clone(),
                        &operation,
                        CommunicationEdgeKind::DesignatedInputRequest,
                        source_ref.clone(),
                        Some(dependency_ordinal),
                        Some(dependency.clone()),
                    ),
                    source_fragment_ref: evaluator_artifact_ref.clone(),
                    target_fragment_ref: source_artifact_ref.clone(),
                });
            result
                .communication_plan_mut()
                .add_derived(CommunicationEdgeInput {
                    operation: operation.clone(),
                    kind: CommunicationEdgeKind::DesignatedInputReceipt,
                    source_locus: source_owner.to_string(),
                    target_locus: evaluator.to_string(),
                    core_ref: format!(
                        "designated-input:{operation}:{}",
                        dependency.typed_state_read().namespace()
                    ),
                    source_ref,
                    carrier_contract: CarrierContract::designated_receipt(
                        &operation, evaluation, dependency,
                    ),
                    checked_core_identity: CheckedCoreIdentity::edge(
                        checked.program_identity().clone(),
                        &operation,
                        CommunicationEdgeKind::DesignatedInputReceipt,
                        dependency.typed_state_read().source_ref(),
                        Some(dependency_ordinal),
                        Some(dependency.clone()),
                    ),
                    source_fragment_ref: source_artifact_ref.clone(),
                    target_fragment_ref: evaluator_artifact_ref.clone(),
                });
        }
    }
    result.persistence_plan_mut().add_designated(operation);
}

fn project_designated_result_consumer(
    result: &mut GlobalProjectionResult,
    checked: &CheckedSurfaceV0,
    evaluation: &CheckedEvaluation,
) {
    let core = evaluation
        .designated_result_consumer_core()
        .expect("designated result consumer checked Core");
    let operation = format!("{}.{}", core.evaluator(), core.result());
    let evaluator = core.evaluator();
    let consumer = core.consumer_locus();
    let evaluator_fragment_ref = artifact_ref(evaluator, &operation, "designated-evaluation");
    let consumer_fragment_ref = artifact_ref(consumer, &operation, "designated-result-consumer");
    let core_ref = format!(
        "designated-consume:{}.{}:{}",
        core.evaluator(),
        core.result(),
        core.consumer_locus(),
    );
    result
        .locus_program_mut(consumer)
        .add_fragment(ProjectedOperationFragment {
            operation_id: operation.clone(),
            kind: ProjectedOperationFragmentKind::DesignatedResultConsumer,
            source_ref: evaluation.source_ref().clone(),
            core_ref: core_ref.clone(),
            artifact_ref: consumer_fragment_ref.clone(),
            authority_requirements: AuthorityRequirements::designated_result_consumer(
                &operation,
                evaluation.source_ref(),
            ),
            declared_failure_row: evaluation.declared_failure_row().clone(),
            generated_failure_row: evaluation.generated_failure_row().clone(),
            placement: PlacementSpecificCore::DesignatedResultConsumer { core: core.clone() },
            locus_tag: LocusTag::checked(consumer),
            fragment_ref: consumer_fragment_ref.clone(),
            checked_core_identity: CheckedCoreIdentity::fragment(
                checked.program_identity().clone(),
                &operation,
                ProjectedOperationFragmentKind::DesignatedResultConsumer,
                evaluation.source_ref().clone(),
                None,
                None,
            ),
            semantic_obligations: SemanticObligations::from_evaluation(evaluation),
            runtime_seam_requirements: RuntimeSeamRequirements::designated_result_consumer(),
            designated_result_consumer_expression_leakage: false,
        });
    result.observation_plan_mut().add_local_fragment(
        evaluation.source_ref().clone(),
        core_ref.clone(),
        consumer_fragment_ref.clone(),
        ProjectedOperationFragmentKind::DesignatedResultConsumer,
    );
    result
        .communication_plan_mut()
        .add_derived(CommunicationEdgeInput {
            operation: operation.clone(),
            kind: CommunicationEdgeKind::DesignatedResultDelivery,
            source_locus: evaluator.to_string(),
            target_locus: consumer.to_string(),
            core_ref,
            source_ref: evaluation.source_ref().clone(),
            carrier_contract: CarrierContract::designated_result_delivery(evaluation),
            checked_core_identity: CheckedCoreIdentity::edge(
                checked.program_identity().clone(),
                &operation,
                CommunicationEdgeKind::DesignatedResultDelivery,
                evaluation.source_ref().clone(),
                None,
                None,
            ),
            source_fragment_ref: evaluator_fragment_ref,
            target_fragment_ref: consumer_fragment_ref,
        });
    result
        .static_conflict_policy_mut()
        .add_designated_result_consumer(operation, consumer);
}

fn artifact_ref(locus: &str, operation: &str, role: &str) -> String {
    format!("artifact:{locus}:{operation}:{role}")
}
