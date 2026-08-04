use mir_semantics::{
    evaluation_materialization::{InputFrontier, Locus, OccurrenceId, Principal},
    maintained_relation::{
        AnchorEpoch, EntityId, MaintainedRelationSpec, PresentationSample, RelationDiagnosticCode,
        RelationProjectionContext, RelationProjectionHarness, Transform2, VisibilityLabel,
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

fn relation_spec(
    owner: Locus,
    subject: EntityId,
    primary_anchor: Locus,
    fallback_anchor: Locus,
    activation_frontier: InputFrontier,
    primary_epoch: AnchorEpoch,
    fallback_epoch: AnchorEpoch,
) -> MaintainedRelationSpec {
    MaintainedRelationSpec::follow_with_fallback(
        owner,
        subject,
        primary_anchor.clone(),
        Transform2::translation(3, -2),
        fallback_anchor.clone(),
        Transform2::identity(),
    )
    .with_activation_frontier(activation_frontier)
    .with_required_anchor_epoch(primary_anchor, primary_epoch)
    .with_required_anchor_epoch(fallback_anchor, fallback_epoch)
    .with_visibility_label(VisibilityLabel::Public)
}

fn admitted_sample(
    anchor: Locus,
    consumer: &Principal,
    snapshot: &str,
    activation_frontier: InputFrontier,
    transform: Transform2,
    epoch: AnchorEpoch,
) -> PresentationSample {
    PresentationSample::new(
        anchor,
        snapshot,
        activation_frontier,
        transform,
        VisibilityLabel::Public,
    )
    .with_anchor_epoch(epoch)
    .with_release_to(consumer.clone())
}

#[test]
fn split_frame_samples_reject_before_projection_and_preserve_relation_state() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation"]);
    let primary_epoch = AnchorEpoch::new(2);
    let fallback_epoch = AnchorEpoch::new(3);
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner,
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            activation_frontier.clone(),
            primary_epoch,
            fallback_epoch,
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let before = harness.relation_state(&relation).clone();
    let split_context =
        RelationProjectionContext::for_consumer(consumer.clone(), "snapshot-primary")
            .with_binding_activation_frontier(activation_frontier.clone())
            .with_sample(admitted_sample(
                primary_anchor,
                &consumer,
                "snapshot-primary",
                activation_frontier.clone(),
                Transform2::translation(10, 20),
                primary_epoch,
            ))
            .with_sample(admitted_sample(
                fallback_anchor,
                &consumer,
                "snapshot-fallback",
                activation_frontier,
                Transform2::translation(2, 4),
                fallback_epoch,
            ));

    let diagnostic = harness
        .project_for_consumer(&relation, split_context)
        .expect_err("split-frame samples must not project a maintained relation");

    assert_eq!(
        diagnostic.code,
        RelationDiagnosticCode::SplitFrameProjection
    );
    assert_eq!(
        harness.relation_state(&relation),
        &before,
        "split-frame rejection must not mutate semantic relation state"
    );
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
    assert!(harness.semantic_fallback_events_for(&relation).is_empty());
}

#[test]
fn mismatched_binding_frontier_rejects_without_semantic_state_change() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation"]);
    let stale_frontier = input_frontier(&["stale-presentation-frame"]);
    let primary_epoch = AnchorEpoch::new(2);
    let fallback_epoch = AnchorEpoch::new(3);
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner,
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            activation_frontier,
            primary_epoch,
            fallback_epoch,
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let before = harness.relation_state(&relation).clone();
    let context = RelationProjectionContext::for_consumer(consumer.clone(), "snapshot-current")
        .with_binding_activation_frontier(stale_frontier.clone())
        .with_sample(admitted_sample(
            primary_anchor,
            &consumer,
            "snapshot-current",
            stale_frontier.clone(),
            Transform2::translation(10, 20),
            primary_epoch,
        ))
        .with_sample(admitted_sample(
            fallback_anchor,
            &consumer,
            "snapshot-current",
            stale_frontier,
            Transform2::translation(2, 4),
            fallback_epoch,
        ));

    let diagnostic = harness
        .project_for_consumer(&relation, context)
        .expect_err("presentation context frontier must match binding activation frontier");

    assert_eq!(
        diagnostic.code,
        RelationDiagnosticCode::BindingActivationFrontierMismatch
    );
    assert_eq!(harness.relation_state(&relation), &before);
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
    assert!(harness.semantic_fallback_events_for(&relation).is_empty());
}

#[test]
fn stale_anchor_sample_is_rejected_without_triggering_semantic_fallback() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation", "fresh-anchor-epoch"]);
    let primary_epoch = AnchorEpoch::new(2);
    let fallback_epoch = AnchorEpoch::new(3);
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner,
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            activation_frontier.clone(),
            primary_epoch,
            fallback_epoch,
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let before = harness.relation_state(&relation).clone();
    let context = RelationProjectionContext::for_consumer(consumer.clone(), "snapshot-current")
        .with_binding_activation_frontier(activation_frontier.clone())
        .with_sample(admitted_sample(
            primary_anchor,
            &consumer,
            "snapshot-current",
            activation_frontier.clone(),
            Transform2::translation(10, 20),
            AnchorEpoch::new(1),
        ))
        .with_sample(admitted_sample(
            fallback_anchor,
            &consumer,
            "snapshot-current",
            activation_frontier,
            Transform2::translation(2, 4),
            fallback_epoch,
        ));

    let diagnostic = harness
        .project_for_consumer(&relation, context)
        .expect_err("stale anchor samples are not admissible for semantic use");

    assert_eq!(
        diagnostic.code,
        RelationDiagnosticCode::BindingAnchorEpochMismatch
    );
    assert_eq!(
        harness.relation_state(&relation),
        &before,
        "stale presentation data must not advance semantic fallback"
    );
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
    assert!(harness.semantic_fallback_events_for(&relation).is_empty());
}
