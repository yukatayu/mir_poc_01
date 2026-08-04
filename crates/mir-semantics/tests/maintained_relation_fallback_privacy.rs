use mir_semantics::{
    evaluation_materialization::{InputFrontier, Locus, OccurrenceId, Principal},
    maintained_relation::{
        AnchorEpoch, EntityId, FallbackDomain, MaintainedRelationSpec, PresentationFallback,
        PresentationSample, RelationDiagnosticCode, RelationProjectionContext,
        RelationProjectionHarness, SemanticInvalidation, Transform2, VisibilityLabel, WitnessId,
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

struct RelationBindingFixture {
    activation_frontier: InputFrontier,
    primary_epoch: AnchorEpoch,
    fallback_epoch: AnchorEpoch,
    relation_label: VisibilityLabel,
}

fn relation_binding(
    activation_frontier: InputFrontier,
    primary_epoch: AnchorEpoch,
    fallback_epoch: AnchorEpoch,
    relation_label: VisibilityLabel,
) -> RelationBindingFixture {
    RelationBindingFixture {
        activation_frontier,
        primary_epoch,
        fallback_epoch,
        relation_label,
    }
}

fn relation_spec(
    owner: Locus,
    subject: EntityId,
    primary_anchor: Locus,
    fallback_anchor: Locus,
    binding: RelationBindingFixture,
) -> MaintainedRelationSpec {
    MaintainedRelationSpec::follow_with_fallback(
        owner,
        subject,
        primary_anchor.clone(),
        Transform2::translation(3, -2),
        fallback_anchor.clone(),
        Transform2::identity(),
    )
    .with_activation_frontier(binding.activation_frontier)
    .with_required_anchor_epoch(primary_anchor, binding.primary_epoch)
    .with_required_anchor_epoch(fallback_anchor, binding.fallback_epoch)
    .with_visibility_label(binding.relation_label)
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

#[test]
fn finite_visibility_labels_are_ordered_and_derived_label_joins_every_admitted_anchor() {
    assert!(VisibilityLabel::Public < VisibilityLabel::Restricted);
    assert!(VisibilityLabel::Restricted < VisibilityLabel::Private);

    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation", "presentation-frame"]);
    let primary_epoch = AnchorEpoch::new(2);
    let fallback_epoch = AnchorEpoch::new(3);
    let snapshot = "snapshot-current";
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            relation_binding(
                activation_frontier.clone(),
                primary_epoch,
                fallback_epoch,
                VisibilityLabel::Restricted,
            ),
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Restricted)
        .expect("consumer is admitted only up to Restricted");
    let context = RelationProjectionContext::for_consumer(consumer.clone(), snapshot)
        .with_binding_activation_frontier(activation_frontier.clone())
        .with_sample(admitted_sample(
            primary_anchor,
            &consumer,
            snapshot,
            activation_frontier.clone(),
            Transform2::translation(10, 20),
            VisibilityLabel::Public,
            primary_epoch,
        ))
        .with_sample(admitted_sample(
            fallback_anchor,
            &consumer,
            snapshot,
            activation_frontier,
            Transform2::translation(2, 4),
            VisibilityLabel::Private,
            fallback_epoch,
        ));

    let diagnostic = harness
        .project_for_consumer(&relation, context)
        .expect_err("derived label joins relation plus every admitted anchor label");

    assert_eq!(
        diagnostic.code,
        RelationDiagnosticCode::ProjectionRedactionDenied
    );
    assert_eq!(diagnostic.derived_label, Some(VisibilityLabel::Private));
    assert!(
        diagnostic.raw_transform().is_none(),
        "redaction denial must not carry the private derived transform"
    );
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
    assert!(harness.trace_exposes_no_raw_transforms());
    assert!(harness.owner_mutations_for(&owner).is_empty());
}

#[test]
fn unadmitted_private_anchor_release_denies_before_pose_evaluation_without_raw_trace() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation", "presentation-frame"]);
    let primary_epoch = AnchorEpoch::new(2);
    let fallback_epoch = AnchorEpoch::new(3);
    let snapshot = "snapshot-current";
    let private_pose = Transform2::translation(99, 101);
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            relation_binding(
                activation_frontier.clone(),
                primary_epoch,
                fallback_epoch,
                VisibilityLabel::Public,
            ),
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Private)
        .expect("consumer label authorization is not a sample release");
    let before = harness.relation_state(&relation).clone();
    let context = RelationProjectionContext::for_consumer(consumer.clone(), snapshot)
        .with_binding_activation_frontier(activation_frontier.clone())
        .with_sample(
            PresentationSample::new(
                primary_anchor,
                snapshot,
                activation_frontier.clone(),
                private_pose.clone(),
                VisibilityLabel::Private,
            )
            .with_anchor_epoch(primary_epoch),
        )
        .with_sample(admitted_sample(
            fallback_anchor,
            &consumer,
            snapshot,
            activation_frontier,
            Transform2::translation(2, 4),
            VisibilityLabel::Public,
            fallback_epoch,
        ));

    let diagnostic = harness
        .project_for_consumer(&relation, context)
        .expect_err("presentation samples require explicit release to this consumer");

    assert_eq!(
        diagnostic.code,
        RelationDiagnosticCode::PresentationSampleReleaseDenied
    );
    assert_eq!(
        harness.relation_state(&relation),
        &before,
        "unreleased sample denial must not mutate semantic relation state"
    );
    assert!(
        diagnostic.raw_transform().is_none(),
        "sample-release denial must not carry the private transform"
    );
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
    assert!(harness.trace_exposes_no_raw_transforms());
    assert!(
        !harness
            .trace_redacted_text()
            .contains(private_pose.debug_coordinate_fragment())
    );
    assert!(harness.owner_mutations_for(&owner).is_empty());
}

#[test]
fn semantic_fallback_and_reacquire_require_owner_bound_relation_capability() {
    let owner = locus("relation-owner");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let activation_frontier = input_frontier(&["binding-activation"]);
    let invalidation_frontier = input_frontier(&["primary-membership-lost"]);
    let reacquire_frontier = input_frontier(&["primary-reacquired"]);
    let mut harness = RelationProjectionHarness::default();

    let activation = harness
        .activate_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            relation_binding(
                activation_frontier,
                AnchorEpoch::new(1),
                AnchorEpoch::new(1),
                VisibilityLabel::Public,
            ),
        ))
        .expect("relation activation returns relation key plus opaque owner authority");
    let relation = activation.relation;
    let owner_authority = activation.current_owner_authority;
    let initial = harness.relation_state(&relation).clone();
    assert_eq!(initial.current_option_index, 0);

    let advanced = harness
        .advance_semantic_fallback(
            &relation,
            owner_authority.clone(),
            primary_anchor.clone(),
            SemanticInvalidation::MembershipLost {
                occurrence: occurrence("primary-membership-lost"),
                frontier: invalidation_frontier,
            },
        )
        .expect("owner-bound capability advances to the fallback option");

    assert_eq!(advanced.domain, FallbackDomain::Semantic);
    assert_eq!(advanced.previous_option_index, 0);
    assert_eq!(advanced.current_option_index, 1);
    assert_eq!(advanced.current_anchor, fallback_anchor);
    assert!(advanced.lineage_step > initial.lineage_step);

    let stale_reacquire = harness
        .reacquire_anchor(
            &relation,
            owner_authority.clone(),
            primary_anchor.clone(),
            WitnessId::new("old-witness"),
            AnchorEpoch::new(1),
            input_frontier(&["old-primary-reacquire"]),
        )
        .expect_err("owner capability still needs fresh witness and epoch");
    assert_eq!(
        stale_reacquire.code,
        RelationDiagnosticCode::StaleRelationWitness
    );

    let fresh_reacquire = harness
        .reacquire_anchor(
            &relation,
            owner_authority,
            primary_anchor.clone(),
            WitnessId::new("fresh-witness"),
            AnchorEpoch::new(2),
            reacquire_frontier,
        )
        .expect("fresh witness/epoch may reacquire the primary anchor with owner authority");

    assert_eq!(fresh_reacquire.domain, FallbackDomain::Semantic);
    assert_eq!(fresh_reacquire.current_anchor, primary_anchor);
    assert_eq!(fresh_reacquire.reacquired_epoch, Some(AnchorEpoch::new(2)));
    assert!(fresh_reacquire.lineage_step > advanced.lineage_step);
    assert!(fresh_reacquire.lineage_epoch > advanced.lineage_epoch);
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
}

#[test]
fn owner_authority_for_one_relation_cannot_advance_another_relation() {
    let owner = locus("relation-owner");
    let first_primary_anchor = locus("first-primary-anchor");
    let first_fallback_anchor = locus("first-fallback-anchor");
    let second_primary_anchor = locus("second-primary-anchor");
    let second_fallback_anchor = locus("second-fallback-anchor");
    let mut harness = RelationProjectionHarness::default();

    let first_activation = harness
        .activate_relation(relation_spec(
            owner.clone(),
            EntityId::new("first-subject"),
            first_primary_anchor,
            first_fallback_anchor,
            relation_binding(
                input_frontier(&["first-binding-activation"]),
                AnchorEpoch::new(1),
                AnchorEpoch::new(1),
                VisibilityLabel::Public,
            ),
        ))
        .expect("first relation activation returns relation-scoped owner authority");
    let second_activation = harness
        .activate_relation(relation_spec(
            owner.clone(),
            EntityId::new("second-subject"),
            second_primary_anchor.clone(),
            second_fallback_anchor,
            relation_binding(
                input_frontier(&["second-binding-activation"]),
                AnchorEpoch::new(1),
                AnchorEpoch::new(1),
                VisibilityLabel::Public,
            ),
        ))
        .expect("second relation activation returns a distinct relation authority");
    let before_second = harness.relation_state(&second_activation.relation).clone();

    let diagnostic = harness
        .advance_semantic_fallback(
            &second_activation.relation,
            first_activation.current_owner_authority,
            second_primary_anchor,
            SemanticInvalidation::MembershipLost {
                occurrence: occurrence("cross-relation-membership-lost"),
                frontier: input_frontier(&["cross-relation-membership-lost"]),
            },
        )
        .expect_err("owner authority is bound to one relation key, not only to owner locus");

    assert_eq!(
        diagnostic.code,
        RelationDiagnosticCode::RelationAuthorityDenied
    );
    assert_eq!(
        harness.relation_state(&second_activation.relation),
        &before_second
    );
    assert!(harness.owner_mutations_for(&owner).is_empty());
}

#[test]
fn pre_reacquire_owner_authority_is_stale_after_new_binding_authority_is_issued() {
    let owner = locus("relation-owner");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let mut harness = RelationProjectionHarness::default();

    let activation = harness
        .activate_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            primary_anchor.clone(),
            fallback_anchor.clone(),
            relation_binding(
                input_frontier(&["binding-activation"]),
                AnchorEpoch::new(1),
                AnchorEpoch::new(1),
                VisibilityLabel::Public,
            ),
        ))
        .expect("relation activation returns current binding authority");
    let relation = activation.relation;
    let pre_reacquire_authority = activation.current_owner_authority;
    let advanced = harness
        .advance_semantic_fallback(
            &relation,
            pre_reacquire_authority.clone(),
            primary_anchor.clone(),
            SemanticInvalidation::MembershipLost {
                occurrence: occurrence("primary-membership-lost"),
                frontier: input_frontier(&["primary-membership-lost"]),
            },
        )
        .expect("current activation authority may advance the first semantic fallback");
    let reacquired = harness
        .reacquire_anchor(
            &relation,
            pre_reacquire_authority.clone(),
            primary_anchor.clone(),
            WitnessId::new("fresh-witness"),
            AnchorEpoch::new(2),
            input_frontier(&["primary-reacquired"]),
        )
        .expect("reacquire validates current old authority and returns successor authority");
    let post_reacquire_authority = reacquired.current_owner_authority.clone();
    let before_stale_attempt = harness.relation_state(&relation).clone();

    let stale_diagnostic = harness
        .advance_semantic_fallback(
            &relation,
            pre_reacquire_authority,
            primary_anchor.clone(),
            SemanticInvalidation::MembershipLost {
                occurrence: occurrence("post-reacquire-stale-authority"),
                frontier: input_frontier(&["post-reacquire-stale-authority"]),
            },
        )
        .expect_err("authority bound to the pre-reacquire binding is stale after reacquire");

    assert_eq!(
        stale_diagnostic.code,
        RelationDiagnosticCode::RelationAuthorityDenied
    );
    assert_eq!(harness.relation_state(&relation), &before_stale_attempt);

    let current_advance = harness
        .advance_semantic_fallback(
            &relation,
            post_reacquire_authority,
            primary_anchor,
            SemanticInvalidation::MembershipLost {
                occurrence: occurrence("post-reacquire-current-authority"),
                frontier: input_frontier(&["post-reacquire-current-authority"]),
            },
        )
        .expect("reacquire returns the new current authority for later owner mutation");

    assert_eq!(current_advance.previous_option_index, 0);
    assert_eq!(current_advance.current_option_index, 1);
    assert!(current_advance.lineage_step > reacquired.lineage_step);
    assert!(reacquired.lineage_step > advanced.lineage_step);
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
}

#[test]
fn presentation_gap_uses_presentation_fallback_without_semantic_relation_mutation() {
    let owner = locus("relation-owner");
    let consumer = principal("consumer-c");
    let subject = relation_subject();
    let primary_anchor = locus("primary-anchor");
    let fallback_anchor = locus("fallback-anchor");
    let frontier = input_frontier(&["binding-activation", "presentation-frame"]);
    let snapshot = "snapshot-current";
    let mut harness = RelationProjectionHarness::default();

    let relation = harness
        .define_relation(relation_spec(
            owner.clone(),
            subject.clone(),
            primary_anchor,
            fallback_anchor,
            relation_binding(
                frontier.clone(),
                AnchorEpoch::new(1),
                AnchorEpoch::new(1),
                VisibilityLabel::Public,
            ),
        ))
        .expect("generic typed Core relation is admitted");
    harness
        .authorize_consumer(consumer.clone(), VisibilityLabel::Public)
        .expect("consumer is admitted for public projection");
    let before = harness.relation_state(&relation).clone();
    let context = RelationProjectionContext::for_consumer(consumer, snapshot)
        .with_binding_activation_frontier(frontier)
        .with_presentation_gap(PresentationFallback::hold_last_local(
            subject.clone(),
            Transform2::translation(7, 7),
        ));

    let outcome = harness
        .project_for_consumer(&relation, context)
        .expect("presentation fallback may cover a consumer-local sample gap");

    assert_eq!(outcome.fallback_domain, Some(FallbackDomain::Presentation));
    assert!(
        outcome.presentation_only,
        "presentation fallback output is not semantic relation state"
    );
    assert_eq!(
        harness.relation_state(&relation),
        &before,
        "presentation gaps must not mutate semantic relation lineage or fallback option"
    );
    assert!(harness.semantic_fallback_events_for(&relation).is_empty());
    assert!(harness.owner_mutations_for(&owner).is_empty());
    assert!(harness.absolute_value_stream_for(&subject).is_empty());
}
