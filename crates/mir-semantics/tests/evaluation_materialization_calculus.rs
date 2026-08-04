use mir_semantics::evaluation_materialization::{
    AuthorityOrigin, CapabilityRef, Diagnostic, DiagnosticCode, EvaluationPolicy, EvaluationSite,
    InputFrontier, Locus, Materialization, MaterializationPlan, ObservationPolicy, OccurrenceId,
    Operation, OperationKey, Principal, Provider, RemoteReceipt, SemanticForm, TriggerClock,
    infer_plan,
};

fn locus(name: &str) -> Locus {
    Locus::new(name)
}

fn principal(name: &str) -> Principal {
    Principal::new(name)
}

fn provider(name: &str) -> Provider {
    Provider::new(name)
}

fn capability(name: &str) -> CapabilityRef {
    CapabilityRef::new(name)
}

fn occurrence(name: &str) -> OccurrenceId {
    OccurrenceId::new(name)
}

fn assert_nonempty_operation_key(key: &OperationKey) {
    assert!(
        !key.as_str().is_empty(),
        "M3 operation key must be deterministic and nonempty"
    );
}

fn input_frontier(names: &[&str]) -> InputFrontier {
    InputFrontier::from_ordered_producers(names.iter().map(|name| occurrence(name)).collect())
        .expect("test frontier is finite and ordered")
}

fn materialization_plan<const N: usize>(items: [Materialization; N]) -> MaterializationPlan {
    MaterializationPlan::canonical(items).expect("test materialization plan is valid")
}

fn evaluation_policy(name: &str) -> EvaluationPolicy {
    EvaluationPolicy::declared_deterministic(name)
}

fn observation_policy(name: &str) -> ObservationPolicy {
    ObservationPolicy::declared(name)
}

fn all_materialization_variants() -> [Materialization; 6] {
    [
        Materialization::LocalOnly,
        Materialization::Store,
        Materialization::PublishValue,
        Materialization::PublishRelation,
        Materialization::AdapterStream,
        Materialization::Persist,
    ]
}

fn expected_m3_materialization_subset_is_valid(subset: &[Materialization]) -> bool {
    let contains = |target: Materialization| subset.iter().any(|item| item == &target);
    let empty = subset.is_empty();
    let local_only_conflict = contains(Materialization::LocalOnly) && subset.len() > 1;
    let publish_conflict =
        contains(Materialization::PublishValue) && contains(Materialization::PublishRelation);
    let adapter_persist_conflict =
        contains(Materialization::AdapterStream) && contains(Materialization::Persist);

    !(empty || local_only_conflict || publish_conflict || adapter_persist_conflict)
}

#[test]
fn exposes_finite_coordinate_vocabulary_required_by_m3() {
    let owner = locus("S");
    let caller = principal("browser:self");
    let admitted_provider = provider("physics-adapter");

    let semantic_forms = [
        SemanticForm::Value,
        SemanticForm::State,
        SemanticForm::Relation,
        SemanticForm::Computation,
    ];
    let evaluation_sites = [
        EvaluationSite::Owner(owner.clone()),
        EvaluationSite::Locus(owner.clone()),
        EvaluationSite::DesignatedEvaluator(owner.clone()),
        EvaluationSite::Consumer(caller.clone()),
        EvaluationSite::Provider(admitted_provider.clone()),
    ];
    let trigger_clocks = [
        TriggerClock::OnRequest,
        TriggerClock::OnEvent,
        TriggerClock::OnChange,
        TriggerClock::LogicalTick,
        TriggerClock::FrontierAdvance,
        TriggerClock::PresentationFrame,
        TriggerClock::Explicit,
    ];
    let authority_origins = [
        AuthorityOrigin::Caller(caller),
        AuthorityOrigin::OwnerTransition(owner.clone()),
        AuthorityOrigin::AdmittedEvaluator(owner),
        AuthorityOrigin::AdmittedProvider(admitted_provider),
    ];
    let materializations = [
        Materialization::LocalOnly,
        Materialization::Store,
        Materialization::PublishValue,
        Materialization::PublishRelation,
        Materialization::AdapterStream,
        Materialization::Persist,
    ];

    assert_eq!(semantic_forms.len(), 4);
    assert_eq!(evaluation_sites.len(), 5);
    assert_eq!(trigger_clocks.len(), 7);
    assert_eq!(authority_origins.len(), 4);
    assert_eq!(materializations.len(), 6);
}

#[test]
fn materialization_plan_canonicalizes_finite_sets_and_rejects_m3_conflicts() {
    assert_eq!(
        MaterializationPlan::canonical([Materialization::Store, Materialization::Store])
            .expect("duplicate Store canonicalizes")
            .as_slice(),
        &[Materialization::Store]
    );

    assert!(
        MaterializationPlan::canonical([Materialization::LocalOnly, Materialization::Store])
            .is_err(),
        "LocalOnly is exclusive"
    );
    assert!(
        MaterializationPlan::canonical([
            Materialization::PublishValue,
            Materialization::PublishRelation
        ])
        .is_err(),
        "PublishValue and PublishRelation are mutually exclusive in one M3 plan"
    );
    assert!(
        MaterializationPlan::canonical([Materialization::AdapterStream, Materialization::Persist])
            .is_err(),
        "AdapterStream and Persist are exclusive in M3"
    );
}

#[test]
fn input_frontier_canonicalizes_finite_producer_sets() {
    let ab = InputFrontier::from_ordered_producers(vec![
        occurrence("producer:a"),
        occurrence("producer:b"),
    ])
    .expect("distinct finite producer set is accepted");
    let ba = InputFrontier::from_ordered_producers(vec![
        occurrence("producer:b"),
        occurrence("producer:a"),
    ])
    .expect("same finite producer set in another order is accepted");

    assert_eq!(ab, ba);
    assert_eq!(
        ab.as_slice(),
        &[occurrence("producer:a"), occurrence("producer:b")]
    );
    assert_eq!(
        ba.as_slice(),
        &[occurrence("producer:a"), occurrence("producer:b")]
    );
    assert!(
        InputFrontier::from_ordered_producers(vec![
            occurrence("producer:a"),
            occurrence("producer:a")
        ])
        .is_err(),
        "a duplicate producer is not a finite set"
    );
}

#[test]
fn exhaustive_materialization_subsets_match_m3_constraints() {
    let variants = all_materialization_variants();
    let mut checked_subsets = 0;

    for mask in 0..(1usize << variants.len()) {
        let subset: Vec<Materialization> = variants
            .iter()
            .enumerate()
            .filter(|(index, _item): &(usize, &Materialization)| (mask & (1usize << *index)) != 0)
            .map(|(_index, item): (usize, &Materialization)| *item)
            .collect();
        let expected_valid = expected_m3_materialization_subset_is_valid(&subset);
        let canonical: Result<MaterializationPlan, Diagnostic> =
            MaterializationPlan::canonical(subset.clone());

        assert_eq!(
            canonical.is_ok(),
            expected_valid,
            "unexpected M3 materialization verdict for mask {mask:06b}: {subset:?}"
        );

        if let Some(duplicate) = subset.first().cloned() {
            let mut duplicate_input = subset.clone();
            duplicate_input.push(duplicate);
            let duplicate_canonical: Result<MaterializationPlan, Diagnostic> =
                MaterializationPlan::canonical(duplicate_input);

            assert_eq!(
                duplicate_canonical.is_ok(),
                expected_valid,
                "a duplicate target must preserve the set verdict for mask {mask:06b}: {subset:?}"
            );

            if let (Ok(canonical), Ok(duplicate_canonical)) = (canonical, duplicate_canonical) {
                assert_eq!(duplicate_canonical, canonical);
            }
        } else {
            assert!(
                canonical.is_err(),
                "empty materialization set is invalid; use LocalOnly explicitly"
            );
        }

        checked_subsets += 1;
    }

    assert_eq!(checked_subsets, 64);
}

#[test]
fn same_owner_rmw_plan_keeps_requester_authority_and_owner_evaluation_separate() {
    let caller = principal("browser:self");
    let owner = locus("S");

    let plan = infer_plan(&Operation::same_owner_rmw(
        caller.clone(),
        locus("BrowserClient[self]"),
        owner.clone(),
        "player",
        principal("target"),
        "hp",
        -10,
        capability("cap:write-player"),
    ))
    .expect("same-owner RMW has one deterministic M3 plan");

    assert_eq!(plan.semantic_form, SemanticForm::State);
    assert_eq!(plan.evaluation_site, EvaluationSite::Owner(owner));
    assert_eq!(plan.trigger, TriggerClock::OnRequest);
    assert_eq!(plan.authority_origin, AuthorityOrigin::Caller(caller));
    assert_eq!(
        plan.materialization,
        materialization_plan([Materialization::Store])
    );
    assert_nonempty_operation_key(&plan.operation_key);
    assert_eq!(plan.input_frontier, None);
    assert!(!plan.requires_explicit_receipt);
}

#[test]
fn explicit_remote_receipt_result_is_value_materialization_not_owner_mutation() {
    let caller = principal("browser:self");
    let owner = locus("Stats");
    let receipt_target = locus("BrowserClient[self]");
    let receipt_frontier = input_frontier(&["serve:stats-score:1", "receive:stats-score:1"]);
    let receipt = RemoteReceipt::typed_i64(
        "receipt:stats-score:1",
        owner.clone(),
        receipt_target.clone(),
        "scoreboard.score",
        receipt_frontier.clone(),
        "Int64",
        42,
    );

    assert_eq!(receipt.producer, owner);
    assert_eq!(receipt.target, receipt_target);
    assert_eq!(receipt.label, "scoreboard.score");

    let plan = infer_plan(&Operation::explicit_remote_receipt_result(
        caller.clone(),
        locus("BrowserClient[self]"),
        receipt.producer.clone(),
        "scoreboard",
        principal("target"),
        "score",
        capability("cap:read-score"),
        receipt.clone(),
        TriggerClock::OnRequest,
    ))
    .expect("explicit remote receipt has one deterministic M3 plan");

    assert_eq!(plan.semantic_form, SemanticForm::Value);
    assert_eq!(
        plan.evaluation_site,
        EvaluationSite::Owner(receipt.producer.clone())
    );
    assert_eq!(plan.trigger, TriggerClock::OnRequest);
    assert_eq!(plan.authority_origin, AuthorityOrigin::Caller(caller));
    assert_eq!(
        plan.materialization,
        materialization_plan([Materialization::PublishValue])
    );
    assert_nonempty_operation_key(&plan.operation_key);
    assert_eq!(plan.input_frontier, Some(receipt_frontier));
    assert_eq!(plan.remote_receipt, Some(receipt));
    assert!(plan.requires_explicit_receipt);
}

#[test]
fn explicit_remote_receipt_result_rejects_producer_or_target_mismatch() {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let receipt_frontier = input_frontier(&["serve:stats-score:2", "receive:stats-score:2"]);
    let receipt = RemoteReceipt::typed_i64(
        "receipt:stats-score:2",
        locus("Stats"),
        requester_locus.clone(),
        "scoreboard.score",
        receipt_frontier,
        "Int64",
        42,
    );

    let producer_mismatch = infer_plan(&Operation::explicit_remote_receipt_result(
        caller.clone(),
        requester_locus.clone(),
        locus("OtherStats"),
        "scoreboard",
        principal("target"),
        "score",
        capability("cap:read-score"),
        receipt.clone(),
        TriggerClock::OnRequest,
    ))
    .expect_err("receipt producer must match the operation's producer coordinate");
    assert_eq!(
        producer_mismatch.code,
        DiagnosticCode::ReceiptProducerMismatch
    );

    let target_mismatch_receipt = RemoteReceipt::typed_i64(
        "receipt:stats-score:3",
        locus("Stats"),
        locus("OtherConsumer"),
        "scoreboard.score",
        input_frontier(&["serve:stats-score:3", "receive:stats-score:3"]),
        "Int64",
        42,
    );
    let target_mismatch = infer_plan(&Operation::explicit_remote_receipt_result(
        caller,
        requester_locus,
        locus("Stats"),
        "scoreboard",
        principal("target"),
        "score",
        capability("cap:read-score"),
        target_mismatch_receipt,
        TriggerClock::OnRequest,
    ))
    .expect_err("receipt target must match the operation's receiving locus");
    assert_eq!(target_mismatch.code, DiagnosticCode::ReceiptTargetMismatch);
}

#[test]
fn designated_evaluation_binds_authoritative_site_trigger_and_publication() {
    let evaluator = locus("RulesEngine");
    let frontier = input_frontier(&["serve:hp-read:10", "serve:atk-read:11"]);
    let eval_policy = evaluation_policy("combat.damage.v1");
    let observe_policy = observation_policy("observer_safe:combat.damage");
    let policy_stamp = eval_policy.stamp_with(&observe_policy);

    let plan = infer_plan(&Operation::designated_evaluation(
        AuthorityOrigin::AdmittedEvaluator(evaluator.clone()),
        evaluator.clone(),
        "combat.damage.approved",
        frontier.clone(),
        7,
        eval_policy.clone(),
        observe_policy.clone(),
        TriggerClock::FrontierAdvance,
    ))
    .expect("designated evaluator has one deterministic M3 plan");

    assert_eq!(plan.semantic_form, SemanticForm::Value);
    assert_eq!(
        plan.evaluation_site,
        EvaluationSite::DesignatedEvaluator(evaluator.clone())
    );
    assert_eq!(plan.trigger, TriggerClock::FrontierAdvance);
    assert_eq!(
        plan.authority_origin,
        AuthorityOrigin::AdmittedEvaluator(evaluator)
    );
    assert_eq!(
        plan.materialization,
        materialization_plan([Materialization::PublishValue])
    );
    assert_nonempty_operation_key(&plan.operation_key);
    assert_eq!(plan.input_frontier, Some(frontier));
    assert_eq!(plan.evaluation_policy, Some(eval_policy));
    assert_eq!(plan.observation_policy, Some(observe_policy));
    assert_eq!(plan.policy_stamp, Some(policy_stamp));
    assert!(!plan.requires_explicit_receipt);
}

#[test]
fn unannotated_cross_owner_operand_is_rejected_instead_of_inferred_transaction() {
    let caller = principal("browser:self");
    let write_owner = locus("S");

    let diagnostic = infer_plan(&Operation::unannotated_cross_owner_operand(
        caller.clone(),
        locus("BrowserClient[self]"),
        write_owner.clone(),
        locus("RemoteStats"),
        "player",
        principal("target"),
        "hp",
        -10,
        capability("cap:write-player"),
    ))
    .expect_err("unannotated cross-owner operand must not infer multi-owner atomicity");

    assert_eq!(diagnostic.code, DiagnosticCode::CrossOwnerOperand);
    assert_eq!(
        diagnostic.evaluation_site,
        Some(EvaluationSite::Owner(write_owner))
    );
    assert_eq!(
        diagnostic.authority_origin,
        Some(AuthorityOrigin::Caller(caller))
    );
}

#[test]
fn unbound_designated_decision_is_ambiguous_until_the_site_is_named() {
    let caller = principal("browser:self");
    let eval_policy = evaluation_policy("combat.damage.v1");
    let observe_policy = observation_policy("observer_safe:combat.damage");

    let diagnostic = infer_plan(&Operation::designated_evaluation_without_site(
        caller.clone(),
        "combat.damage.approved",
        input_frontier(&["serve:hp-read:10", "serve:atk-read:11"]),
        eval_policy,
        observe_policy,
        TriggerClock::FrontierAdvance,
    ))
    .expect_err("authoritative designated decision needs an explicit evaluator");

    assert_eq!(diagnostic.code, DiagnosticCode::AmbiguousEvaluation);
    assert_eq!(diagnostic.evaluation_site, None);
    assert_eq!(
        diagnostic.authority_origin,
        Some(AuthorityOrigin::Caller(caller))
    );
}
