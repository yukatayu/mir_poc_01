use mir_semantics::shared_model::{
    AdverseKind, AnchorEpoch, AtomicCutRef, AuthorityFailure, AuthorityObligation, BadRelationship,
    BindingActivationFrontier, BindingEpoch, BoundedContextCase, BoundedContextEnumerator,
    CapabilityName, ConfigComponent, ConsumerProjectionRequest, ContextClassification, CoreOp,
    CutId, DesignatedEvaluatorRef, DiagnosticCode, EntityRef, FieldRef, GeneratedEdge, Label,
    LeaseEpoch, LocusRef, MaterializationRequest, MembershipEpoch, MutationTarget, OccurrenceId,
    OwnerAuthority, OwnerCommand, PatchSlot, PresentationContext, PresentationContextId,
    PresentationFallback, PresentationSample, PresentationSampleSpec, PrincipalRef,
    PublishedRelation, ReceiptRef, ReceiptRequestRef, RelationDef, RelationKey, RelationOption,
    ResultFrontier, ResultKey, ResultVersion, SaveObject, SemanticInvalidation,
    SharedComponentLayout, SharedConfig, SourceRef, StateKey, Step, SurfaceFragment, TraceKind,
    Transform2, Value, WellFormed, WitnessRef,
};

fn source(line: u32) -> SourceRef {
    SourceRef::new("tests/m5/shared-model.mir", line, 1, line, 80)
}

fn occurrence(name: &str) -> OccurrenceId {
    OccurrenceId::new(name)
}

fn result_frontier(names: &[&str]) -> ResultFrontier {
    ResultFrontier::from_ordered_results(names.iter().map(|name| ResultKey::new(*name)).collect())
        .expect("test result frontier is finite and ordered")
}

fn binding_frontier(names: &[&str]) -> BindingActivationFrontier {
    BindingActivationFrontier::from_ordered_occurrences(
        names.iter().map(|name| occurrence(name)).collect(),
    )
    .expect("test relation frontier is finite and ordered")
}

fn assert_well_formed(config: &SharedConfig) {
    assert_eq!(config.check_well_formed(), WellFormed::Ok);
    assert_single_shared_layout(config);
}

fn assert_single_shared_layout(config: &SharedConfig) {
    assert_eq!(
        config.component_layout(),
        SharedComponentLayout::one_config_with([
            ConfigComponent::OccurrenceHistory,
            ConfigComponent::AuthorityStore,
            ConfigComponent::ObservationLog,
            ConfigComponent::ReceiptStoreR,
            ConfigComponent::DesignatedStoreD,
            ConfigComponent::RelationStoreJ,
            ConfigComponent::InactivePatchSlot,
        ])
    );
    assert!(config.component_layout().has_no_cartesian_m3_m4_wrappers());
}

fn assert_trace_delta(config: &SharedConfig, before: usize, expected: &[TraceKind]) {
    assert_eq!(config.trace_kinds_since(before), expected);
}

fn relation_def(
    relation: &str,
    owner: &LocusRef,
    subject: &EntityRef,
    primary_anchor: &LocusRef,
    fallback_anchor: &LocusRef,
) -> RelationDef {
    RelationDef::follow_with_fallback(
        RelationKey::new(relation),
        owner.clone(),
        subject.clone(),
        RelationOption::anchor(
            primary_anchor.clone(),
            Transform2::translation(3, -2),
            AnchorEpoch::new(7),
        ),
        RelationOption::anchor(
            fallback_anchor.clone(),
            Transform2::identity(),
            AnchorEpoch::new(11),
        ),
        Label::Restricted,
    )
}

struct ActivatedRelationFixture {
    config: SharedConfig,
    owner: LocusRef,
    consumer: PrincipalRef,
    subject: EntityRef,
    primary_anchor: LocusRef,
    fallback_anchor: LocusRef,
    binding_frontier: BindingActivationFrontier,
    relation: RelationKey,
    owner_authority: OwnerAuthority,
}

struct PublishedRelationFixture {
    activated: ActivatedRelationFixture,
    published_relation: PublishedRelation,
}

struct OwnerAuthorityFixture {
    config: SharedConfig,
    owner: LocusRef,
    owner_authority: OwnerAuthority,
    write_state: StateKey,
}

fn owner_authority_fixture() -> OwnerAuthorityFixture {
    let mut config = SharedConfig::empty();
    let owner = LocusRef::new("World");
    let caller = PrincipalRef::new("player-a");
    let owner_admission = config
        .step(Step::admit_owner(
            source(8),
            caller,
            owner.clone(),
            CapabilityName::new("MoveWrite"),
            MembershipEpoch::new(1),
            LeaseEpoch::new(10),
        ))
        .expect("owner admission establishes a matching membership and authority")
        .expect_owner_admission();
    assert_well_formed(&config);

    OwnerAuthorityFixture {
        config,
        owner,
        owner_authority: owner_admission.owner_authority,
        write_state: StateKey::field("player", FieldRef::new("position")),
    }
}

fn set_membership_capability(
    config: &mut SharedConfig,
    owner: &LocusRef,
    capability: CapabilityName,
) {
    config
        .memberships
        .get_mut(owner)
        .expect("owner fixture includes membership")
        .capability = capability;
}

fn set_membership_witness(config: &mut SharedConfig, owner: &LocusRef, witness: WitnessRef) {
    config
        .memberships
        .get_mut(owner)
        .expect("owner fixture includes membership")
        .witness = witness;
}

fn activated_relation_fixture() -> ActivatedRelationFixture {
    let mut config = SharedConfig::empty();
    let owner = LocusRef::new("World");
    let consumer = PrincipalRef::new("client-c");
    let subject = EntityRef::new("player-a");
    let primary_anchor = LocusRef::new("room-anchor");
    let fallback_anchor = LocusRef::new("default-anchor");
    let activation_frontier = binding_frontier(&["binding-activation"]);

    assert_single_shared_layout(&config);
    let before_admission = config.trace_len();
    let owner_admission = config
        .step(Step::admit_owner(
            source(10),
            PrincipalRef::new("owner-agent"),
            owner.clone(),
            CapabilityName::new("RelationOwner"),
            MembershipEpoch::new(1),
            LeaseEpoch::new(30),
        ))
        .expect("owner admission is a direct SharedConfig step")
        .expect_owner_admission();
    assert_trace_delta(
        &config,
        before_admission,
        &[
            TraceKind::MembershipAdmitted,
            TraceKind::CapabilityIssued,
            TraceKind::WitnessIssued,
        ],
    );
    assert_well_formed(&config);

    let before_activation = config.trace_len();
    let activation = config
        .step(Step::activate_relation(
            source(11),
            relation_def(
                "player-room-anchor",
                &owner,
                &subject,
                &primary_anchor,
                &fallback_anchor,
            ),
            owner_admission.owner_authority,
            activation_frontier.clone(),
        ))
        .expect("relation activation is a direct SharedConfig step")
        .expect_relation_activation();
    assert_eq!(
        activation.binding_state.binding_epoch(),
        BindingEpoch::new(1)
    );
    assert_eq!(
        activation.binding_state.activation_frontier(),
        &activation_frontier
    );
    assert!(activation.projected_relation().is_none());
    assert!(config.published_relation(&activation.relation).is_none());
    assert_trace_delta(&config, before_activation, &[TraceKind::RelationActivated]);
    assert_well_formed(&config);

    ActivatedRelationFixture {
        config,
        owner,
        consumer,
        subject,
        primary_anchor,
        fallback_anchor,
        binding_frontier: activation_frontier,
        relation: activation.relation,
        owner_authority: activation.current_owner_authority,
    }
}

fn published_relation_fixture() -> PublishedRelationFixture {
    let mut activated = activated_relation_fixture();
    let before_publish = activated.config.trace_len();
    let publication = activated
        .config
        .step(Step::publish_relation(
            source(12),
            activated.relation.clone(),
            activated.owner_authority.clone(),
        ))
        .expect("relation publication is a distinct owner publish-relation step")
        .expect_relation_publication();
    assert_trace_delta(
        &activated.config,
        before_publish,
        &[TraceKind::RelationPublished],
    );
    assert_well_formed(&activated.config);

    PublishedRelationFixture {
        activated,
        published_relation: publication.published_relation,
    }
}

#[test]
fn surface_fragment_elaborates_to_core_or_typed_diagnostic_with_source_edges() {
    let caller = PrincipalRef::new("player-a");
    let owner = LocusRef::new("World");
    let write_state = StateKey::field("player", FieldRef::new("position"));
    let receipt = ReceiptRef::new("roll-result");
    let source_ref = source(30);
    let fragment = SurfaceFragment::owner_rmw_with_receipt(
        source_ref.clone(),
        caller.clone(),
        owner.clone(),
        OwnerCommand::add(write_state.clone(), Value::int(3)),
        receipt.clone(),
        CapabilityName::new("MoveWrite"),
    );

    let first = fragment.elaborate();
    let second = fragment.elaborate();
    assert_eq!(
        first, second,
        "elaboration from one SourceRef must be deterministic"
    );
    let core = first.expect_core();
    assert_eq!(core.source_ref(), &source_ref);
    assert_eq!(
        core.ops(),
        &[CoreOp::owner_rmw(
            owner.clone(),
            OwnerCommand::add(write_state.clone(), Value::int(3))
        )]
    );
    assert_eq!(
        core.generated_edges(),
        &[
            GeneratedEdge::request(
                source_ref.clone(),
                caller.clone(),
                owner.clone(),
                CapabilityName::new("MoveWrite")
            ),
            GeneratedEdge::receipt_use(source_ref.clone(), receipt.clone()),
            GeneratedEdge::owner_write(source_ref.clone(), owner.clone(), write_state)
        ]
    );
    assert_eq!(
        core.authority_obligations(),
        &[
            AuthorityObligation::capability(
                source_ref.clone(),
                caller.clone(),
                owner.clone(),
                CapabilityName::new("MoveWrite")
            ),
            AuthorityObligation::witness(source_ref.clone(), caller.clone(), owner.clone()),
            AuthorityObligation::receipt_release(source_ref.clone(), receipt)
        ]
    );

    let diagnostic_source = source(45);
    let diagnostic = SurfaceFragment::cross_owner_read_without_receipt(
        diagnostic_source.clone(),
        caller,
        owner.clone(),
        LocusRef::new("Inventory"),
        StateKey::field("inventory", FieldRef::new("held-item")),
    )
    .elaborate()
    .expect_diagnostic();
    assert_eq!(
        diagnostic.code(),
        DiagnosticCode::CrossOwnerOperandRequiresReceipt
    );
    assert_eq!(diagnostic.source_ref(), &diagnostic_source);
    assert_eq!(diagnostic.generated_edges(), &[]);
    assert_eq!(
        diagnostic.authority_obligations(),
        &[AuthorityObligation::explicit_receipt_required(
            diagnostic_source,
            owner,
            LocusRef::new("Inventory")
        )]
    );
}

#[test]
fn shared_config_steps_owner_receipt_designated_activation_and_publication_directly() {
    let mut config = SharedConfig::empty();
    let owner = LocusRef::new("World");
    let caller = PrincipalRef::new("player-a");
    let write_state = StateKey::field("score", FieldRef::new("points"));
    assert_well_formed(&config);

    let before_admission = config.trace_len();
    let owner_admission = config
        .step(Step::admit_owner(
            source(60),
            caller.clone(),
            owner.clone(),
            CapabilityName::new("ScoreWrite"),
            MembershipEpoch::new(1),
            LeaseEpoch::new(10),
        ))
        .expect("owner admission is a direct SharedConfig step")
        .expect_owner_admission();
    assert_trace_delta(
        &config,
        before_admission,
        &[
            TraceKind::MembershipAdmitted,
            TraceKind::CapabilityIssued,
            TraceKind::WitnessIssued,
        ],
    );
    assert_well_formed(&config);

    let before_rmw = config.trace_len();
    let rmw = config
        .step(Step::owner_rmw(
            source(61),
            owner.clone(),
            owner_admission.owner_authority.clone(),
            OwnerCommand::add(write_state.clone(), Value::int(5)),
        ))
        .expect("owner RMW is evaluated inside SharedConfig")
        .expect_owner_rmw();
    assert_eq!(rmw.store_value(&write_state), Some(&Value::int(5)));
    assert_trace_delta(&config, before_rmw, &[TraceKind::OwnerReadModifyWrite]);
    assert_well_formed(&config);

    let receipt_request = ReceiptRequestRef::new("roll-request");
    let receipt_ref = ReceiptRef::new("roll-result");
    let receipt_frontier = result_frontier(&["rng-roll-v1"]);
    let before_request = config.trace_len();
    config
        .step(Step::request_receipt(
            source(62),
            receipt_request.clone(),
            caller.clone(),
            owner.clone(),
            ResultKey::new("rng.roll"),
            receipt_frontier.clone(),
            Label::Restricted,
        ))
        .expect("typed receipt request enters R through SharedConfig")
        .expect_receipt_request();
    assert_trace_delta(&config, before_request, &[TraceKind::ReceiptRequested]);
    assert_well_formed(&config);

    let before_serve = config.trace_len();
    config
        .step(Step::serve_receipt(
            source(63),
            receipt_request.clone(),
            owner.clone(),
            owner_admission.owner_authority.clone(),
        ))
        .expect("receipt serve validates owner authority")
        .expect_receipt_serve();
    assert_trace_delta(&config, before_serve, &[TraceKind::ReceiptServed]);
    assert_well_formed(&config);

    let before_reply = config.trace_len();
    config
        .step(Step::reply_receipt(
            source(64),
            receipt_request.clone(),
            receipt_ref.clone(),
            Value::int(4),
            Label::Restricted,
        ))
        .expect("receipt reply records typed released value")
        .expect_receipt_reply();
    assert_trace_delta(&config, before_reply, &[TraceKind::ReceiptReplied]);
    assert_well_formed(&config);

    let before_receive = config.trace_len();
    let receipt = config
        .step(Step::receive_receipt(
            source(65),
            receipt_request.clone(),
            receipt_ref.clone(),
            caller.clone(),
            owner.clone(),
            Label::Restricted,
        ))
        .expect("receive completes the exact request-serve-reply-receive release chain")
        .expect_receipt();
    assert_eq!(receipt.request, receipt_request);
    assert_eq!(receipt.frontier, receipt_frontier);
    assert_eq!(
        receipt.release_chain(),
        &[
            TraceKind::ReceiptRequested,
            TraceKind::ReceiptServed,
            TraceKind::ReceiptReplied,
            TraceKind::ReceiptReceived,
        ]
    );
    assert_trace_delta(&config, before_receive, &[TraceKind::ReceiptReceived]);
    assert_well_formed(&config);

    let before_receipt_use = config.trace_len();
    config
        .step(Step::owner_rmw_with_receipt(
            source(66),
            owner.clone(),
            owner_admission.owner_authority.clone(),
            receipt_ref,
            OwnerCommand::add(write_state.clone(), Value::int(4)),
        ))
        .expect("owner RMW may use only a complete stored typed receipt")
        .expect_owner_rmw();
    assert_trace_delta(
        &config,
        before_receipt_use,
        &[
            TraceKind::ReceiptConsumedByOwner,
            TraceKind::OwnerReadModifyWrite,
        ],
    );
    assert_well_formed(&config);

    let evaluator = DesignatedEvaluatorRef::new("Physics");
    let designated_key = ResultKey::new("collision");
    let designated_frontier = result_frontier(&["physics-input-v1"]);
    let before_designated = config.trace_len();
    let designated = config
        .step(Step::designated_decide(
            source(67),
            evaluator.clone(),
            designated_key.clone(),
            designated_frontier.clone(),
            Value::bool(false),
            Label::Public,
        ))
        .expect("designated result is decided in SharedConfig")
        .expect_designated_result();
    assert_eq!(designated.evaluator, evaluator);
    assert_eq!(designated.result, designated_key);
    assert_eq!(designated.frontier, designated_frontier);
    assert_eq!(designated.version, ResultVersion::new(1));
    assert_trace_delta(
        &config,
        before_designated,
        &[TraceKind::DesignatedResultDecided],
    );
    assert_well_formed(&config);

    let before_duplicate = config.trace_len();
    let duplicate = config
        .step(Step::designated_decide(
            source(68),
            evaluator.clone(),
            designated_key.clone(),
            designated_frontier.clone(),
            Value::bool(false),
            Label::Public,
        ))
        .expect("duplicate designated result publication is stable")
        .expect_designated_result();
    assert_eq!(duplicate.version, designated.version);
    assert_eq!(duplicate.value, designated.value);
    assert_trace_delta(
        &config,
        before_duplicate,
        &[TraceKind::DesignatedResultDuplicate],
    );
    assert_well_formed(&config);

    let before_consume = config.trace_len();
    config
        .step(Step::consume_designated_result(
            source(69),
            PrincipalRef::new("observer-c"),
            evaluator.clone(),
            designated_key.clone(),
            designated_frontier.clone(),
            designated.version,
        ))
        .expect("designated result has one explicit bounded consume")
        .expect_designated_consumption();
    assert_eq!(
        config.designated_consumption_count(&designated_key, &designated_frontier),
        1
    );
    assert_trace_delta(
        &config,
        before_consume,
        &[TraceKind::DesignatedResultConsumed],
    );
    assert_well_formed(&config);

    let before_second_consume = config.trace_len();
    let before_second_consume_snapshot = config.snapshot();
    let second_consumer = config
        .step(Step::consume_designated_result(
            source(70),
            PrincipalRef::new("observer-d"),
            evaluator.clone(),
            designated_key.clone(),
            designated_frontier.clone(),
            designated.version,
        ))
        .expect_err("designated result identity has one bounded consume, not one per consumer");
    assert_eq!(
        second_consumer.code(),
        DiagnosticCode::DesignatedResultAlreadyConsumed
    );
    assert_eq!(config.snapshot(), before_second_consume_snapshot);
    assert_eq!(
        config.designated_consumption_count(&designated_key, &designated_frontier),
        1
    );
    assert_trace_delta(
        &config,
        before_second_consume,
        &[TraceKind::DesignatedResultConsumed],
    );
    assert_well_formed(&config);

    let binding_activation = binding_frontier(&["relation-binding-activation"]);
    let before_relation = config.trace_len();
    let activation = config
        .step(Step::activate_relation(
            source(72),
            relation_def(
                "player-room-anchor",
                &owner,
                &EntityRef::new("player-a"),
                &LocusRef::new("room-anchor"),
                &LocusRef::new("default-anchor"),
            ),
            owner_admission.owner_authority,
            binding_activation.clone(),
        ))
        .expect("activation mutates only J")
        .expect_relation_activation();
    assert_eq!(
        activation.binding_state.activation_frontier(),
        &binding_activation
    );
    assert_eq!(
        activation.binding_state.binding_epoch(),
        BindingEpoch::new(1)
    );
    assert!(activation.projected_relation().is_none());
    assert!(config.published_relation(&activation.relation).is_none());
    assert_trace_delta(&config, before_relation, &[TraceKind::RelationActivated]);
    assert_well_formed(&config);

    let before_publish = config.trace_len();
    let published = config
        .step(Step::publish_relation(
            source(73),
            activation.relation.clone(),
            activation.current_owner_authority,
        ))
        .expect("publication is a distinct owner publish-relation step")
        .expect_relation_publication();
    assert_eq!(published.relation, activation.relation);
    assert_eq!(published.published_relation.selected_option_index(), 0);
    assert_eq!(
        published.published_relation.binding_epoch(),
        BindingEpoch::new(1)
    );
    assert_eq!(
        published.published_relation.activation_frontier(),
        &binding_activation
    );
    assert_eq!(
        published
            .published_relation
            .required_anchor_epoch(&LocusRef::new("room-anchor")),
        Some(AnchorEpoch::new(7))
    );
    assert_eq!(
        published
            .published_relation
            .required_anchor_epoch(&LocusRef::new("default-anchor")),
        Some(AnchorEpoch::new(11))
    );
    assert_eq!(published.published_relation.label(), Label::Restricted);
    assert!(!published.published_relation.uses_result_frontier());
    assert_trace_delta(&config, before_publish, &[TraceKind::RelationPublished]);
    assert_well_formed(&config);
}

#[test]
fn well_formed_rejects_membership_capability_or_witness_mismatch() {
    let fixture = owner_authority_fixture();

    let mut capability_mismatch = fixture.config.clone();
    set_membership_capability(
        &mut capability_mismatch,
        &fixture.owner,
        CapabilityName::new("WrongCapability"),
    );
    assert_eq!(
        capability_mismatch.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied)
    );

    let mut witness_mismatch = fixture.config;
    set_membership_witness(
        &mut witness_mismatch,
        &fixture.owner,
        WitnessRef::new("wrong-membership-witness"),
    );
    assert_eq!(
        witness_mismatch.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::StaleWitness)
    );
}

#[test]
fn owner_mutation_rejects_membership_capability_or_witness_mismatch_without_mutation() {
    let fixture = owner_authority_fixture();

    let mut capability_mismatch = fixture.config.clone();
    set_membership_capability(
        &mut capability_mismatch,
        &fixture.owner,
        CapabilityName::new("WrongCapability"),
    );

    let mut witness_mismatch = fixture.config;
    set_membership_witness(
        &mut witness_mismatch,
        &fixture.owner,
        WitnessRef::new("wrong-membership-witness"),
    );

    for mut config in [capability_mismatch, witness_mismatch] {
        let before = config.trace_len();
        let before_snapshot = config.snapshot();
        let diagnostic = config
            .step(Step::owner_rmw(
                source(79),
                fixture.owner.clone(),
                fixture.owner_authority.clone(),
                OwnerCommand::add(fixture.write_state.clone(), Value::int(1)),
            ))
            .expect_err(
                "owner mutation must reject if membership capability or witness diverges from authority",
            );
        assert_eq!(diagnostic.code(), DiagnosticCode::OwnerAuthorityDenied);
        assert_eq!(config.snapshot(), before_snapshot);
        assert_trace_delta(&config, before, &[TraceKind::RelationAuthorityRejected]);
        assert!(config.owner_stores.is_empty());
    }
}

#[test]
fn legacy_record_receipt_cannot_fabricate_complete_receipt_or_authorize_owner_use() {
    let mut config = SharedConfig::empty();
    let owner = LocusRef::new("World");
    let caller = PrincipalRef::new("player-a");
    let write_state = StateKey::field("score", FieldRef::new("points"));
    let owner_admission = config
        .step(Step::admit_owner(
            source(80),
            caller.clone(),
            owner.clone(),
            CapabilityName::new("ScoreWrite"),
            MembershipEpoch::new(1),
            LeaseEpoch::new(10),
        ))
        .expect("owner admission establishes authority for later owner-use check")
        .expect_owner_admission();
    assert_well_formed(&config);

    let receipt = ReceiptRef::new("fabricated-roll");
    let before_direct_record = config.trace_len();
    let before_direct_snapshot = config.snapshot();
    let direct_record = config
        .step(Step::record_receipt(
            source(81),
            receipt.clone(),
            caller,
            owner.clone(),
            ResultKey::new("rng.roll"),
            result_frontier(&["fabricated-frontier"]),
            Label::Restricted,
        ))
        .expect_err("legacy direct receipt insertion cannot fabricate a complete release chain");
    assert_eq!(
        direct_record.code(),
        DiagnosticCode::ReceiptReleaseChainInvalid
    );
    assert_eq!(config.snapshot(), before_direct_snapshot);
    assert!(!config.receipts_r.contains_key(&receipt));
    assert_trace_delta(&config, before_direct_record, &[TraceKind::ReceiptRecorded]);
    assert_well_formed(&config);

    let before_owner_use = config.trace_len();
    let before_owner_use_snapshot = config.snapshot();
    let owner_use = config
        .step(Step::owner_rmw_with_receipt(
            source(82),
            owner.clone(),
            owner_admission.owner_authority,
            receipt.clone(),
            OwnerCommand::add(write_state, Value::int(4)),
        ))
        .expect_err("owner use requires a receipt produced by the typed release chain");
    assert_eq!(owner_use.code(), DiagnosticCode::ReceiptRequestMissing);
    assert_eq!(config.snapshot(), before_owner_use_snapshot);
    assert!(!config.receipts_r.contains_key(&receipt));
    assert_trace_delta(
        &config,
        before_owner_use,
        &[TraceKind::ReceiptConsumedByOwner],
    );
    assert_well_formed(&config);
}

#[test]
fn admitted_projection_uses_published_relation_not_direct_j_lookup() {
    let mut fixture = published_relation_fixture();
    let consumer = fixture.activated.consumer.clone();
    let relation = fixture.activated.relation.clone();
    let before_authorization = fixture.activated.config.trace_len();
    fixture
        .activated
        .config
        .step(Step::authorize_projection(
            source(90),
            consumer.clone(),
            Label::Private,
        ))
        .expect("consumer projection admission is explicit")
        .expect_projection_admission();
    assert_trace_delta(
        &fixture.activated.config,
        before_authorization,
        &[TraceKind::ConsumerProjectionAdmitted],
    );
    assert_well_formed(&fixture.activated.config);

    let context = PresentationContext::for_consumer(
        PresentationContextId::new("frame-current"),
        consumer.clone(),
        fixture.activated.binding_frontier.clone(),
    )
    .with_sample(PresentationSample::released(
        fixture.activated.primary_anchor.clone(),
        consumer.clone(),
        fixture.activated.binding_frontier.clone(),
        AnchorEpoch::new(7),
        Transform2::translation(10, 20),
        Label::Public,
    ))
    .with_sample(PresentationSample::released(
        fixture.activated.fallback_anchor.clone(),
        consumer.clone(),
        fixture.activated.binding_frontier.clone(),
        AnchorEpoch::new(11),
        Transform2::translation(2, 4),
        Label::Private,
    ));
    let before_direct_j_trace = fixture.activated.config.trace_len();
    let before_direct_j_binding = fixture.activated.config.relation_binding(&relation).clone();
    let direct_j = fixture
        .activated
        .config
        .step(Step::project_relation_from_j(
            source(91),
            relation.clone(),
            context.clone(),
        ))
        .expect_err("consumer projection must use a published carrier, not mutable J lookup");
    assert_eq!(direct_j.code(), DiagnosticCode::RelationPublicationRequired);
    assert_eq!(
        fixture.activated.config.relation_binding(&relation),
        &before_direct_j_binding
    );
    assert_trace_delta(
        &fixture.activated.config,
        before_direct_j_trace,
        &[TraceKind::ConsumerProjectionRejected],
    );
    assert_well_formed(&fixture.activated.config);

    let before_projection_trace = fixture.activated.config.trace_len();
    let before_binding = fixture.activated.config.relation_binding(&relation).clone();
    let projection = fixture
        .activated
        .config
        .step(Step::project_published_relation(
            source(92),
            fixture.published_relation.clone(),
            context,
        ))
        .expect("admitted coherent projection is local to the consumer")
        .expect_projection();

    assert_eq!(
        projection.subject_transform,
        Transform2::translation(13, 18)
    );
    assert_eq!(projection.derived_label, Label::Private);
    assert!(projection.absolute_stream_entries().is_empty());
    assert!(
        fixture
            .activated
            .config
            .absolute_stream_for(&fixture.activated.subject)
            .is_empty()
    );
    assert_eq!(
        fixture.activated.config.relation_binding(&relation),
        &before_binding
    );
    assert_trace_delta(
        &fixture.activated.config,
        before_projection_trace,
        &[TraceKind::ConsumerProjection],
    );
    assert_well_formed(&fixture.activated.config);

    let gap_context = PresentationContext::for_consumer(
        PresentationContextId::new("frame-gap"),
        consumer,
        fixture.activated.binding_frontier.clone(),
    )
    .with_presentation_gap(PresentationFallback::hold_last_local(
        fixture.activated.subject.clone(),
        Transform2::translation(7, 7),
    ));
    let before_gap_trace = fixture.activated.config.trace_len();
    let before_gap_binding = fixture.activated.config.relation_binding(&relation).clone();
    let gap_projection = fixture
        .activated
        .config
        .step(Step::project_published_relation(
            source(93),
            fixture.published_relation,
            gap_context,
        ))
        .expect("presentation gap is consumer-local")
        .expect_projection();

    assert!(gap_projection.presentation_only);
    assert_eq!(
        fixture.activated.config.relation_binding(&relation),
        &before_gap_binding
    );
    assert_trace_delta(
        &fixture.activated.config,
        before_gap_trace,
        &[TraceKind::PresentationGap],
    );
    assert_well_formed(&fixture.activated.config);
}

#[test]
fn consumer_projection_requesting_store_publish_value_or_j_mutation_is_rejected() {
    let mut fixture = published_relation_fixture();
    let consumer = fixture.activated.consumer.clone();
    let relation = fixture.activated.relation.clone();
    let before_authorization = fixture.activated.config.trace_len();
    fixture
        .activated
        .config
        .step(Step::authorize_projection(
            source(110),
            consumer.clone(),
            Label::Private,
        ))
        .expect("consumer projection admission is explicit")
        .expect_projection_admission();
    assert_trace_delta(
        &fixture.activated.config,
        before_authorization,
        &[TraceKind::ConsumerProjectionAdmitted],
    );
    assert_well_formed(&fixture.activated.config);

    let context = PresentationContext::for_consumer(
        PresentationContextId::new("mixed-materialization-falsifier"),
        consumer.clone(),
        fixture.activated.binding_frontier.clone(),
    )
    .with_sample(PresentationSample::released(
        fixture.activated.primary_anchor.clone(),
        consumer.clone(),
        fixture.activated.binding_frontier.clone(),
        AnchorEpoch::new(7),
        Transform2::translation(10, 20),
        Label::Public,
    ))
    .with_sample(PresentationSample::released(
        fixture.activated.fallback_anchor.clone(),
        consumer,
        fixture.activated.binding_frontier.clone(),
        AnchorEpoch::new(11),
        Transform2::translation(2, 4),
        Label::Restricted,
    ));
    let before_snapshot = fixture.activated.config.snapshot();
    let before_trace = fixture.activated.config.trace_len();
    let diagnostic = fixture
        .activated
        .config
        .step(Step::project_published_relation_with_request(
            source(111),
            fixture.published_relation,
            context,
            ConsumerProjectionRequest::new()
                .with_materialization(MaterializationRequest::store_publish_value())
                .with_mutation_target(MutationTarget::RelationStoreJ),
        ))
        .expect_err("consumer projection cannot store, publish a value, or mutate J");

    assert_eq!(
        diagnostic.code(),
        DiagnosticCode::ConsumerProjectionMaterializationDenied
    );
    assert_eq!(
        diagnostic.rejected_materialization(),
        Some(MaterializationRequest::store_publish_value())
    );
    assert_eq!(
        diagnostic.rejected_mutation_target(),
        Some(MutationTarget::RelationStoreJ)
    );
    assert_eq!(fixture.activated.config.snapshot(), before_snapshot);
    assert_eq!(
        fixture
            .activated
            .config
            .relation_binding(&relation)
            .relation(),
        &relation
    );
    assert_trace_delta(
        &fixture.activated.config,
        before_trace,
        &[TraceKind::ConsumerProjectionRejected],
    );
    assert_well_formed(&fixture.activated.config);
}

#[test]
fn relation_authority_rejects_wrong_relation_and_stale_binding_witness() {
    let mut fixture = activated_relation_fixture();
    let first_relation = fixture.relation.clone();
    let first_authority = fixture.owner_authority.clone();
    let other_subject = EntityRef::new("other-player");
    let other_primary = LocusRef::new("other-room-anchor");
    let other_fallback = LocusRef::new("other-default-anchor");

    let before_other_activation = fixture.config.trace_len();
    let other_activation = fixture
        .config
        .step(Step::activate_relation(
            source(130),
            relation_def(
                "other-player-room-anchor",
                &fixture.owner,
                &other_subject,
                &other_primary,
                &other_fallback,
            ),
            first_authority.clone(),
            binding_frontier(&["other-binding-activation"]),
        ))
        .expect("same owner may activate another relation with current owner authority")
        .expect_relation_activation();
    assert_trace_delta(
        &fixture.config,
        before_other_activation,
        &[TraceKind::RelationActivated],
    );
    assert_well_formed(&fixture.config);

    let before_wrong_relation_trace = fixture.config.trace_len();
    let before_wrong_relation_binding = fixture
        .config
        .relation_binding(&other_activation.relation)
        .clone();
    let wrong_relation = fixture
        .config
        .step(Step::advance_relation_binding(
            source(131),
            other_activation.relation.clone(),
            first_authority.clone(),
            SemanticInvalidation::membership_lost(
                occurrence("wrong-relation-membership-lost"),
                binding_frontier(&["wrong-relation-membership-lost"]),
            ),
        ))
        .expect_err("owner authority is bound to the exact relation key");
    assert_eq!(
        wrong_relation.code(),
        DiagnosticCode::RelationAuthorityDenied
    );
    assert_eq!(
        wrong_relation.authority_failure(),
        Some(AuthorityFailure::RelationMismatch)
    );
    assert_eq!(
        fixture.config.relation_binding(&other_activation.relation),
        &before_wrong_relation_binding
    );
    assert_trace_delta(
        &fixture.config,
        before_wrong_relation_trace,
        &[TraceKind::RelationAuthorityRejected],
    );
    assert_well_formed(&fixture.config);

    let before_advance = fixture.config.trace_len();
    let advanced = fixture
        .config
        .step(Step::advance_relation_binding(
            source(132),
            first_relation.clone(),
            first_authority.clone(),
            SemanticInvalidation::membership_lost(
                occurrence("primary-membership-lost"),
                binding_frontier(&["primary-membership-lost"]),
            ),
        ))
        .expect("current authority may advance the selected relation binding")
        .expect_relation_advance();
    assert_eq!(advanced.current_option_index, 1);
    assert_trace_delta(
        &fixture.config,
        before_advance,
        &[TraceKind::RelationAdvanced],
    );
    assert_well_formed(&fixture.config);

    let before_reacquire = fixture.config.trace_len();
    let reacquired = fixture
        .config
        .step(Step::reacquire_relation_binding(
            source(133),
            first_relation.clone(),
            first_authority.clone(),
            fixture.primary_anchor.clone(),
            WitnessRef::new("fresh-primary-witness"),
            BindingEpoch::new(2),
            binding_frontier(&["primary-reacquired"]),
        ))
        .expect("reacquire validates old authority plus fresh lineage witness")
        .expect_relation_reacquire();
    assert_eq!(reacquired.current_option_index, 0);
    assert_trace_delta(
        &fixture.config,
        before_reacquire,
        &[TraceKind::RelationReacquired],
    );
    assert_well_formed(&fixture.config);

    let before_stale_trace = fixture.config.trace_len();
    let before_stale_binding = fixture.config.relation_binding(&first_relation).clone();
    let stale = fixture
        .config
        .step(Step::advance_relation_binding(
            source(134),
            first_relation.clone(),
            first_authority,
            SemanticInvalidation::membership_lost(
                occurrence("stale-post-reacquire-authority"),
                binding_frontier(&["stale-post-reacquire-authority"]),
            ),
        ))
        .expect_err("pre-reacquire authority is stale for the new binding epoch and witness");
    assert_eq!(stale.code(), DiagnosticCode::RelationAuthorityDenied);
    assert_eq!(
        stale.authority_failure(),
        Some(AuthorityFailure::StaleBindingEpochOrWitness)
    );
    assert_eq!(
        fixture.config.relation_binding(&first_relation),
        &before_stale_binding
    );
    assert_trace_delta(
        &fixture.config,
        before_stale_trace,
        &[TraceKind::RelationAuthorityRejected],
    );
    assert_well_formed(&fixture.config);
}

#[test]
fn relation_authority_rejects_membership_capability_mismatch_after_activation() {
    let mut fixture = activated_relation_fixture();
    let membership_witness = fixture
        .config
        .memberships
        .get(&fixture.owner)
        .expect("activated relation fixture includes owner membership")
        .witness
        .clone();
    assert_ne!(membership_witness, fixture.owner_authority.witness);
    assert_well_formed(&fixture.config);

    set_membership_capability(
        &mut fixture.config,
        &fixture.owner,
        CapabilityName::new("WrongCapability"),
    );
    assert_eq!(
        fixture.config.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied)
    );

    let before_snapshot = fixture.config.snapshot();
    let before_trace = fixture.config.trace_len();
    let diagnostic = fixture
        .config
        .step(Step::publish_relation(
            source(140),
            fixture.relation.clone(),
            fixture.owner_authority,
        ))
        .expect_err("relation authority must reject mismatched owner membership capability");
    assert_eq!(diagnostic.code(), DiagnosticCode::RelationAuthorityDenied);
    assert_eq!(
        diagnostic.authority_failure(),
        Some(AuthorityFailure::UnknownAuthority)
    );
    assert_eq!(fixture.config.snapshot(), before_snapshot);
    assert!(
        fixture
            .config
            .published_relation(&fixture.relation)
            .is_none()
    );
    assert_trace_delta(
        &fixture.config,
        before_trace,
        &[TraceKind::RelationAuthorityRejected],
    );
    assert_eq!(
        fixture.config.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied)
    );
}

#[test]
fn save_from_cut_rejects_invalid_shared_config_before_creating_save_object() {
    let relation = RelationKey::new("player-room-anchor");
    let mut bad =
        SharedConfig::unchecked_with_bad_relationship(BadRelationship::AuthorityStoreToJ {
            relation: relation.clone(),
            owner: LocusRef::new("World"),
            authority_binding_epoch: BindingEpoch::new(1),
            j_binding_epoch: BindingEpoch::new(2),
        });
    assert_eq!(
        bad.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::BadRelationship)
    );
    assert_eq!(
        bad.bad_relationship(),
        Some(BadRelationship::AuthorityStoreToJ {
            relation,
            owner: LocusRef::new("World"),
            authority_binding_epoch: BindingEpoch::new(1),
            j_binding_epoch: BindingEpoch::new(2),
        })
    );

    let before_save = bad.trace_len();
    let before_snapshot = bad.snapshot();
    let diagnostic = bad
        .step(Step::save_from_cut(
            source(145),
            AtomicCutRef::new(CutId::new("invalid-config-cut")),
        ))
        .expect_err("save_from_cut must reject malformed SharedConfig before saving");
    assert_eq!(diagnostic.code(), DiagnosticCode::BadRelationship);
    assert_eq!(bad.snapshot(), before_snapshot);
    assert_trace_delta(&bad, before_save, &[TraceKind::SaveRejected]);
    assert_eq!(
        bad.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::BadRelationship)
    );
}

#[test]
fn atomic_cut_save_and_restore_validate_saved_provenance_without_mutation() {
    let mut fixture = published_relation_fixture();
    let relation = fixture.activated.relation.clone();
    let frame_id = PresentationContextId::new("ephemeral-frame");
    let context = PresentationContext::for_consumer(
        frame_id.clone(),
        fixture.activated.consumer.clone(),
        fixture.activated.binding_frontier.clone(),
    )
    .with_presentation_gap(PresentationFallback::hold_last_local(
        fixture.activated.subject.clone(),
        Transform2::translation(5, 5),
    ));
    let before_authorization = fixture.activated.config.trace_len();
    fixture
        .activated
        .config
        .step(Step::authorize_projection(
            source(150),
            fixture.activated.consumer.clone(),
            Label::Restricted,
        ))
        .expect("projection authorization is explicit")
        .expect_projection_admission();
    assert_trace_delta(
        &fixture.activated.config,
        before_authorization,
        &[TraceKind::ConsumerProjectionAdmitted],
    );
    assert_well_formed(&fixture.activated.config);

    let before_projection = fixture.activated.config.trace_len();
    fixture
        .activated
        .config
        .step(Step::project_published_relation(
            source(151),
            fixture.published_relation,
            context,
        ))
        .expect("presentation context is usable but remains ephemeral")
        .expect_projection();
    assert_trace_delta(
        &fixture.activated.config,
        before_projection,
        &[TraceKind::PresentationGap],
    );
    assert_well_formed(&fixture.activated.config);
    assert_eq!(fixture.activated.config.patch_slot(), PatchSlot::Inactive);

    let before_invalid_save = fixture.activated.config.trace_len();
    let invalid_save = fixture
        .activated
        .config
        .step(Step::save_from_cut(
            source(152),
            AtomicCutRef::new(CutId::new("missing-cut")),
        ))
        .expect_err("save requires an existing valid atomic cut occurrence");
    assert_eq!(invalid_save.code(), DiagnosticCode::MissingAtomicCut);
    assert_trace_delta(
        &fixture.activated.config,
        before_invalid_save,
        &[TraceKind::SaveRejected],
    );
    assert_well_formed(&fixture.activated.config);

    let before_cut = fixture.activated.config.trace_len();
    let cut = fixture
        .activated
        .config
        .step(Step::atomic_cut(
            source(153),
            fixture.activated.owner.clone(),
            CutId::new("cut-after-relation"),
        ))
        .expect("atomic cut appends a cut occurrence to H")
        .expect_atomic_cut();
    assert_eq!(cut.owner, fixture.activated.owner);
    assert_trace_delta(
        &fixture.activated.config,
        before_cut,
        &[TraceKind::AtomicCut],
    );
    assert_well_formed(&fixture.activated.config);

    let before_save = fixture.activated.config.trace_len();
    let save = fixture
        .activated
        .config
        .step(Step::save_from_cut(source(154), cut.cut_ref.clone()))
        .expect("save is admitted only from a valid cut-ending configuration")
        .expect_save_object();
    assert!(save.cut().ends_with(&cut.occurrence));
    assert!(save.is_consistent_with_cut());
    assert_eq!(save.patch_slot(), PatchSlot::Inactive);
    assert!(save.semantic_provenance().contains_relation(&relation));
    assert!(
        save.semantic_provenance()
            .contains_owner_authority(&fixture.activated.owner_authority)
    );
    assert!(
        save.semantic_provenance()
            .contains_binding_frontier(&fixture.activated.binding_frontier)
    );
    assert!(!save.contains_presentation_context(&frame_id));
    assert_trace_delta(
        &fixture.activated.config,
        before_save,
        &[TraceKind::SaveObjectCreated],
    );
    assert_well_formed(&fixture.activated.config);

    struct RestoreCase {
        save: SaveObject,
        expected_restore_code: DiagnosticCode,
        expected_reconstructed_wf: WellFormed,
        expected_cut_consistent: bool,
    }

    let restore_cases = vec![
        RestoreCase {
            save: save.clone().with_saved_authority_relation(
                fixture.activated.owner_authority.clone(),
                RelationKey::new("not-the-saved-relation"),
            ),
            expected_restore_code: DiagnosticCode::RelationAuthorityDenied,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::BadRelationship),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_membership_principal(
                fixture.activated.owner.clone(),
                PrincipalRef::new("wrong-principal"),
            ),
            expected_restore_code: DiagnosticCode::OwnerAuthorityDenied,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_membership_capability(
                fixture.activated.owner.clone(),
                CapabilityName::new("WrongCapability"),
            ),
            expected_restore_code: DiagnosticCode::OwnerAuthorityDenied,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_membership_witness(
                fixture.activated.owner.clone(),
                WitnessRef::new("wrong-membership-witness"),
            ),
            expected_restore_code: DiagnosticCode::StaleWitness,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::StaleWitness),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_membership_epoch(
                fixture.activated.owner.clone(),
                MembershipEpoch::new(99),
            ),
            expected_restore_code: DiagnosticCode::StaleMembership,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::StaleMembership),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_membership_lease_epoch(
                fixture.activated.owner.clone(),
                LeaseEpoch::new(99),
            ),
            expected_restore_code: DiagnosticCode::OwnerAuthorityDenied,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_membership_lease_epoch(
                fixture.activated.owner.clone(),
                LeaseEpoch::new(0),
            ),
            expected_restore_code: DiagnosticCode::ExpiredLease,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::ExpiredLease),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_authority_witness(
                fixture.activated.owner_authority.clone(),
                WitnessRef::new("not-the-saved-witness"),
            ),
            expected_restore_code: DiagnosticCode::StaleWitness,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::StaleWitness),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.clone().with_saved_authority_lease_epoch(
                fixture.activated.owner_authority.clone(),
                LeaseEpoch::new(0),
            ),
            expected_restore_code: DiagnosticCode::ExpiredLease,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::ExpiredLease),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save
                .clone()
                .with_saved_relation_binding_epoch(relation.clone(), BindingEpoch::new(99)),
            expected_restore_code: DiagnosticCode::StaleRelationLineage,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::StaleRelationLineage),
            expected_cut_consistent: true,
        },
        RestoreCase {
            save: save.with_saved_atomic_cut_occurrence(
                cut.cut_ref.clone(),
                occurrence("not-the-saved-cut-occurrence"),
            ),
            expected_restore_code: DiagnosticCode::MissingAtomicCut,
            expected_reconstructed_wf: WellFormed::Violation(DiagnosticCode::MissingAtomicCut),
            expected_cut_consistent: false,
        },
    ];

    for RestoreCase {
        save: invalid_save,
        expected_restore_code,
        expected_reconstructed_wf,
        expected_cut_consistent,
    } in restore_cases
    {
        assert_eq!(
            invalid_save.is_consistent_with_cut(),
            expected_cut_consistent
        );
        assert_eq!(
            invalid_save.reconstructed_config().check_well_formed(),
            expected_reconstructed_wf
        );
        let mut restore_target = SharedConfig::empty();
        let before_restore_snapshot = restore_target.snapshot();
        let before_restore_trace = restore_target.trace_len();
        let diagnostic = restore_target
            .restore(invalid_save)
            .expect_err("restore derives rejection from saved provenance mismatch");
        assert_eq!(diagnostic.code(), expected_restore_code);
        assert_eq!(restore_target.snapshot(), before_restore_snapshot);
        assert_eq!(restore_target.patch_slot(), PatchSlot::Inactive);
        assert_trace_delta(
            &restore_target,
            before_restore_trace,
            &[TraceKind::RestoreRejected],
        );
        assert_well_formed(&restore_target);
    }
}

#[test]
fn bounded_context_enumeration_classifies_coherent_stale_release_and_adverse_cases() {
    let owner = LocusRef::new("World");
    let consumer = PrincipalRef::new("client-c");
    let subject = EntityRef::new("player-a");
    let primary_anchor = LocusRef::new("room-anchor");
    let fallback_anchor = LocusRef::new("default-anchor");
    let frontier = binding_frontier(&["binding-activation"]);
    let relation = relation_def(
        "player-room-anchor",
        &owner,
        &subject,
        &primary_anchor,
        &fallback_anchor,
    );
    let enumerator = BoundedContextEnumerator::new(
        relation,
        consumer.clone(),
        frontier.clone(),
        vec![
            PresentationSampleSpec::required(
                primary_anchor.clone(),
                AnchorEpoch::new(7),
                Label::Public,
            ),
            PresentationSampleSpec::required(
                fallback_anchor.clone(),
                AnchorEpoch::new(11),
                Label::Restricted,
            ),
        ],
    );

    let cases = enumerator.enumerate();
    assert!(cases.iter().any(|case: &BoundedContextCase| {
        case.classification()
            == ContextClassification::Coherent {
                derived_label: Label::Restricted,
            }
    }));
    assert!(cases.iter().any(|case: &BoundedContextCase| {
        case.classification()
            == ContextClassification::Rejected {
                code: DiagnosticCode::BindingActivationFrontierMismatch,
            }
    }));
    assert!(cases.iter().any(|case: &BoundedContextCase| {
        case.classification()
            == ContextClassification::Rejected {
                code: DiagnosticCode::PresentationSampleReleaseDenied,
            }
    }));

    let adverse = enumerator.explicit_adverse_counterexample(
        AdverseKind::SplitFrontierWithReleasedSamples,
        PresentationContext::for_consumer(
            PresentationContextId::new("split-frontier-adverse"),
            consumer,
            frontier.clone(),
        )
        .with_sample(PresentationSample::released(
            primary_anchor,
            PrincipalRef::new("client-c"),
            frontier,
            AnchorEpoch::new(7),
            Transform2::translation(10, 20),
            Label::Public,
        ))
        .with_sample(PresentationSample::released(
            fallback_anchor,
            PrincipalRef::new("client-c"),
            binding_frontier(&["different-frame"]),
            AnchorEpoch::new(11),
            Transform2::translation(2, 4),
            Label::Restricted,
        )),
    );

    assert_eq!(
        adverse.classification(),
        ContextClassification::AdverseCounterexample {
            kind: AdverseKind::SplitFrontierWithReleasedSamples,
            code: DiagnosticCode::SplitFrameProjection,
        }
    );
    assert!(adverse.trace_delta().is_empty());
    assert!(adverse.semantic_mutation_delta().is_empty());
}

#[test]
fn direct_shared_config_well_formed_detects_bad_relationships() {
    let relation = RelationKey::new("player-room-anchor");
    let bad = SharedConfig::unchecked_with_bad_relationship(BadRelationship::AuthorityStoreToJ {
        relation: relation.clone(),
        owner: LocusRef::new("World"),
        authority_binding_epoch: BindingEpoch::new(1),
        j_binding_epoch: BindingEpoch::new(2),
    });

    assert_eq!(
        bad.check_well_formed(),
        WellFormed::Violation(DiagnosticCode::BadRelationship)
    );
    assert_eq!(
        bad.bad_relationship(),
        Some(BadRelationship::AuthorityStoreToJ {
            relation,
            owner: LocusRef::new("World"),
            authority_binding_epoch: BindingEpoch::new(1),
            j_binding_epoch: BindingEpoch::new(2),
        })
    );
}
