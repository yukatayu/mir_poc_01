use mir_semantics::{
    evaluation_materialization::{
        AuthorityOrigin, EvalPlan, EvaluationSite, InputFrontier, Locus, Materialization,
        MaterializationPlan, OccurrenceId, Principal, SemanticForm, TriggerClock,
    },
    maintained_relation::{
        AnchorEpoch, EntityId, MaintainedRelationSpec, PresentationSample, RelationDependency,
        RelationDiagnosticCode, RelationId, RelationProjectionContext, RelationProjectionHarness,
        Transform2, VisibilityLabel,
    },
};

fn locus(name: &str) -> Locus {
    Locus::new(name)
}

fn principal(name: &str) -> Principal {
    Principal::new(name)
}

fn occurrence(name: &str) -> OccurrenceId {
    OccurrenceId::new(name)
}

fn input_frontier(names: &[&str]) -> InputFrontier {
    InputFrontier::from_ordered_producers(names.iter().map(|name| occurrence(name)).collect())
        .expect("test frontier is finite and ordered")
}

fn relation_subject() -> EntityId {
    EntityId::new("dependent-subject")
}

struct AnchorBindingFixture {
    activation_frontier: InputFrontier,
    primary_anchor: Locus,
    fallback_anchor: Locus,
    primary_epoch: AnchorEpoch,
    fallback_epoch: AnchorEpoch,
}

fn anchor_binding(
    activation_frontier: InputFrontier,
    primary_anchor: Locus,
    fallback_anchor: Locus,
    primary_epoch: AnchorEpoch,
    fallback_epoch: AnchorEpoch,
) -> AnchorBindingFixture {
    AnchorBindingFixture {
        activation_frontier,
        primary_anchor,
        fallback_anchor,
        primary_epoch,
        fallback_epoch,
    }
}

fn relation_spec(
    owner: Locus,
    subject: EntityId,
    binding: &AnchorBindingFixture,
    primary_offset: Transform2,
    fallback_offset: Transform2,
    relation_label: VisibilityLabel,
) -> MaintainedRelationSpec {
    MaintainedRelationSpec::follow_with_fallback(
        owner,
        subject,
        binding.primary_anchor.clone(),
        primary_offset,
        binding.fallback_anchor.clone(),
        fallback_offset,
    )
    .with_activation_frontier(binding.activation_frontier.clone())
    .with_required_anchor_epoch(binding.primary_anchor.clone(), binding.primary_epoch)
    .with_required_anchor_epoch(binding.fallback_anchor.clone(), binding.fallback_epoch)
    .with_visibility_label(relation_label)
}

fn admitted_sample(
    anchor: Locus,
    consumer: &Principal,
    snapshot: &str,
    activation_frontier: InputFrontier,
    transform: Transform2,
    label: VisibilityLabel,
    epoch: AnchorEpoch,
) -> PresentationSample {
    PresentationSample::new(anchor, snapshot, activation_frontier, transform, label)
        .with_anchor_epoch(epoch)
        .with_release_to(consumer.clone())
}

fn coherent_context(
    consumer: Principal,
    snapshot: &str,
    binding: &AnchorBindingFixture,
    primary_transform: Transform2,
    fallback_transform: Transform2,
) -> RelationProjectionContext {
    RelationProjectionContext::for_consumer(consumer.clone(), snapshot)
        .with_binding_activation_frontier(binding.activation_frontier.clone())
        .with_sample(admitted_sample(
            binding.primary_anchor.clone(),
            &consumer,
            snapshot,
            binding.activation_frontier.clone(),
            primary_transform,
            VisibilityLabel::Public,
            binding.primary_epoch,
        ))
        .with_sample(admitted_sample(
            binding.fallback_anchor.clone(),
            &consumer,
            snapshot,
            binding.activation_frontier.clone(),
            fallback_transform,
            VisibilityLabel::Public,
            binding.fallback_epoch,
        ))
}

fn assert_owner_semantic_relation_plan(plan: &EvalPlan, owner: &Locus) {
    assert_eq!(plan.semantic_form, SemanticForm::Relation);
    assert_eq!(plan.evaluation_site, EvaluationSite::Owner(owner.clone()));
    assert_eq!(plan.trigger, TriggerClock::FrontierAdvance);
    assert_eq!(
        plan.authority_origin,
        AuthorityOrigin::OwnerTransition(owner.clone())
    );
    assert_eq!(
        plan.materialization,
        MaterializationPlan::canonical([Materialization::Store, Materialization::PublishRelation])
            .expect("owner relation plan stores the DAG and publishes relation metadata")
    );
    assert_eq!(
        plan.input_frontier, None,
        "binding activation frontier is M4 binding state, not an M3 designated frontier"
    );
    assert!(
        !plan.operation_key.as_str().is_empty(),
        "M4 owner relation plan must retain deterministic operation identity"
    );
}

fn assert_consumer_projection_plan(plan: &EvalPlan, consumer: &Principal) {
    assert_eq!(plan.semantic_form, SemanticForm::Relation);
    assert_eq!(
        plan.evaluation_site,
        EvaluationSite::Consumer(consumer.clone())
    );
    assert_eq!(plan.trigger, TriggerClock::PresentationFrame);
    assert_eq!(
        plan.authority_origin,
        AuthorityOrigin::Caller(consumer.clone())
    );
    assert_eq!(
        plan.materialization,
        MaterializationPlan::canonical([Materialization::LocalOnly])
            .expect("consumer projection is local-only")
    );
    assert!(
        !plan
            .materialization
            .as_slice()
            .contains(&Materialization::PublishValue)
    );
    assert!(
        !plan
            .materialization
            .as_slice()
            .contains(&Materialization::AdapterStream)
    );
    assert_eq!(
        plan.input_frontier, None,
        "presentation context frontier is not an M3 designated-result frontier"
    );
    assert!(
        !plan.operation_key.as_str().is_empty(),
        "M4 consumer projection plan must retain deterministic operation identity"
    );
}

#[test]
fn project_then_evaluate_matches_evaluate_then_project_for_exact_binding_frontier_and_epochs() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation", "anchor-epochs"]);
    let primary_epoch = AnchorEpoch::new(2);
    let fallback_epoch = AnchorEpoch::new(3);
    let snapshot = "snapshot-current";
    let primary = Transform2::translation(10, 20);
    let fallback = Transform2::translation(2, 4);
    let offset = Transform2::translation(3, -2);
    let binding = anchor_binding(
        activation_frontier.clone(),
        primary_anchor.clone(),
        fallback_anchor.clone(),
        primary_epoch,
        fallback_epoch,
    );
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            &binding,
            offset.clone(),
            Transform2::identity(),
            VisibilityLabel::Public,
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let context = coherent_context(
        consumer.clone(),
        snapshot,
        &binding,
        primary.clone(),
        fallback,
    );

    let projected_then_evaluated = harness
        .project_then_evaluate(&relation, context.clone())
        .expect("project-then-evaluate succeeds for one exact binding frame");
    let evaluated_then_projected = harness
        .evaluate_then_project(&relation, context)
        .expect("evaluate-then-project succeeds for the same exact binding frame");

    assert_eq!(
        projected_then_evaluated.subject_transform,
        primary.compose(&offset),
        "M4 relation projection uses checked Transform2 composition primary o offset"
    );
    assert_eq!(
        projected_then_evaluated.subject_transform,
        evaluated_then_projected.subject_transform
    );
    assert_eq!(
        projected_then_evaluated.input_frontier,
        evaluated_then_projected.input_frontier
    );
    assert_eq!(
        projected_then_evaluated.binding_activation_frontier(),
        &activation_frontier
    );
    assert_eq!(
        projected_then_evaluated.required_anchor_epoch(&primary_anchor),
        Some(primary_epoch)
    );
    assert_eq!(
        projected_then_evaluated.required_anchor_epoch(&fallback_anchor),
        Some(fallback_epoch)
    );
    assert_eq!(
        projected_then_evaluated.derived_label,
        VisibilityLabel::Public
    );
    assert_owner_semantic_relation_plan(&projected_then_evaluated.owner_plan, &owner);
    assert_consumer_projection_plan(&projected_then_evaluated.consumer_plan, &consumer);
    assert_owner_semantic_relation_plan(&evaluated_then_projected.owner_plan, &owner);
    assert_consumer_projection_plan(&evaluated_then_projected.consumer_plan, &consumer);
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
}

#[test]
fn consumer_local_projection_does_not_emit_absolute_subject_stream() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation", "presentation-frame"]);
    let primary_epoch = AnchorEpoch::new(5);
    let fallback_epoch = AnchorEpoch::new(8);
    let snapshot = "snapshot-current";
    let binding = anchor_binding(
        activation_frontier,
        primary_anchor,
        fallback_anchor,
        primary_epoch,
        fallback_epoch,
    );
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            &binding,
            Transform2::translation(1, 1),
            Transform2::identity(),
            VisibilityLabel::Public,
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let context = coherent_context(
        consumer.clone(),
        snapshot,
        &binding,
        Transform2::translation(8, 9),
        Transform2::translation(0, 0),
    );

    let projected = harness
        .project_for_consumer(&relation, context)
        .expect("consumer may locally project a maintained relation");

    assert_owner_semantic_relation_plan(&projected.owner_plan, &owner);
    assert_consumer_projection_plan(&projected.consumer_plan, &consumer);
    assert_eq!(projected.semantic_owner, owner);
    assert_eq!(projected.consumer, consumer);
    assert_eq!(projected.subject, subject);
    assert!(
        harness
            .absolute_value_stream_for(&projected.subject)
            .is_empty()
    );
    assert!(
        harness
            .adapter_stream_for_subject(&projected.subject)
            .is_empty()
    );
}

#[test]
fn checked_transform_composition_overflow_rejects_without_saturated_projection() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation", "overflow-frame"]);
    let primary_epoch = AnchorEpoch::new(1);
    let fallback_epoch = AnchorEpoch::new(1);
    let snapshot = "snapshot-current";
    let binding = anchor_binding(
        activation_frontier,
        primary_anchor,
        fallback_anchor,
        primary_epoch,
        fallback_epoch,
    );
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            &binding,
            Transform2::translation(1, 0),
            Transform2::identity(),
            VisibilityLabel::Public,
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let before = harness.relation_state(&relation).clone();
    let context = coherent_context(
        consumer,
        snapshot,
        &binding,
        Transform2::translation(i64::MAX, 0),
        Transform2::translation(0, 0),
    );

    let diagnostic = harness
        .project_for_consumer(&relation, context)
        .expect_err("checked Transform2 composition rejects overflow instead of saturating");

    assert_eq!(diagnostic.code, RelationDiagnosticCode::TransformOverflow);
    assert_eq!(harness.relation_state(&relation), &before);
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
    assert!(harness.owner_mutations_for(&owner).is_empty());
}

#[test]
fn relation_dependency_cycles_reject_while_simple_anchor_relation_is_accepted() {
    let owner = locus("relation-owner");
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation"]);
    let binding = anchor_binding(
        activation_frontier,
        primary_anchor,
        fallback_anchor,
        AnchorEpoch::new(1),
        AnchorEpoch::new(1),
    );
    let mut harness = RelationProjectionHarness::default();

    let simple_anchor_relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            EntityId::new("simple-subject"),
            &binding,
            Transform2::translation(1, 0),
            Transform2::identity(),
            VisibilityLabel::Public,
        ))
        .expect("a simple generic anchor relation is accepted");
    assert!(
        !simple_anchor_relation.as_str().is_empty(),
        "accepted relation id is stable and nonempty"
    );

    let self_cycle = RelationId::new("self-cycle");
    let self_cycle_diagnostic = harness
        .define_relation(
            relation_spec(
                owner.clone(),
                EntityId::new("self-cycle-subject"),
                &binding,
                Transform2::translation(1, 0),
                Transform2::identity(),
                VisibilityLabel::Public,
            )
            .with_relation_id(self_cycle.clone())
            .with_dependency(RelationDependency::Relation(self_cycle)),
        )
        .expect_err("a direct self-dependency is rejected at relation registration");
    assert_eq!(
        self_cycle_diagnostic.code,
        RelationDiagnosticCode::RelationCycle
    );

    let left = RelationId::new("cycle-left");
    let right = RelationId::new("cycle-right");
    let cyclic_batch_diagnostic = harness
        .define_relation_batch([
            relation_spec(
                owner.clone(),
                EntityId::new("cycle-left-subject"),
                &binding,
                Transform2::translation(1, 0),
                Transform2::identity(),
                VisibilityLabel::Public,
            )
            .with_relation_id(left.clone())
            .with_dependency(RelationDependency::Relation(right.clone())),
            relation_spec(
                owner,
                EntityId::new("cycle-right-subject"),
                &binding,
                Transform2::translation(0, 1),
                Transform2::identity(),
                VisibilityLabel::Public,
            )
            .with_relation_id(right)
            .with_dependency(RelationDependency::Relation(left)),
        ])
        .expect_err("a finite two-relation dependency cycle is rejected at registration");
    assert_eq!(
        cyclic_batch_diagnostic.code,
        RelationDiagnosticCode::RelationCycle
    );
}
