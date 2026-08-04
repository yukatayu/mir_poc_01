use mir_semantics::evaluation_materialization::{
    AuthorityOrigin, CapabilityRef, DiagnosticCode, EvaluationMaterializationHarness,
    EvaluationPolicy, EvaluationSite, InputFrontier, Locus, Materialization, MaterializationPlan,
    ObservationPolicy, OccurrenceId, Operation, OperationKey, Principal, RemoteReceipt,
    SemanticForm, TraceDetail, TraceEntryKind, TriggerClock,
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

fn assert_nonempty_operation_key(key: &OperationKey) {
    assert!(
        !key.as_str().is_empty(),
        "M3 operation key must be deterministic and nonempty"
    );
}

fn assert_all_trace_rows_have_operation_keys(harness: &EvaluationMaterializationHarness) {
    for entry in harness.trace() {
        assert_nonempty_operation_key(&entry.operation_key);
    }
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

fn attack_operation(
    caller: &Principal,
    requester_locus: &Locus,
    owner: &Locus,
    target: &Principal,
    capability: &CapabilityRef,
) -> Operation {
    Operation::same_owner_rmw(
        caller.clone(),
        requester_locus.clone(),
        owner.clone(),
        "player",
        target.clone(),
        "hp",
        -10,
        capability.clone(),
    )
}

#[allow(clippy::too_many_arguments)]
fn run_remote_i64_receipt_flow(
    harness: &mut EvaluationMaterializationHarness,
    caller: Principal,
    producer: Locus,
    target: Locus,
    id: &str,
    label: &str,
    input_frontier: InputFrontier,
    value_i64: i64,
) -> RemoteReceipt {
    harness.grant_receipt_release(caller.clone(), producer.clone(), target.clone(), label);
    let request = harness
        .request_remote_i64_receipt(caller, producer, target, id, label, input_frontier, "Int64")
        .expect("remote receipt request is admitted");
    let served = harness
        .serve_remote_i64_receipt_request(request, value_i64)
        .expect("remote producer serves the request");
    let reply = harness
        .reply_remote_i64_receipt(served)
        .expect("remote producer creates a typed reply");
    harness
        .receive_remote_receipt(reply)
        .expect("target receives the typed causal receipt")
}

fn assert_receipt_flow_trace(harness: &EvaluationMaterializationHarness, receipt: &RemoteReceipt) {
    let flow = [
        TraceEntryKind::RemoteReceiptRequested,
        TraceEntryKind::RemoteReceiptServed,
        TraceEntryKind::RemoteReceiptReplied,
        TraceEntryKind::RemoteReceiptReceived,
    ];

    for kind in flow {
        let row = harness
            .trace()
            .iter()
            .find(|entry| entry.kind == kind)
            .unwrap_or_else(|| panic!("missing receipt flow trace row: {kind:?}"));
        assert_nonempty_operation_key(&row.operation_key);
        assert_eq!(row.receipt_producer, Some(receipt.producer.clone()));
        assert_eq!(row.receipt_target, Some(receipt.target.clone()));
        assert_eq!(row.receipt_label, Some(receipt.label.clone()));
    }
}

fn assert_no_receipt_flow_trace_for_label(harness: &EvaluationMaterializationHarness, label: &str) {
    assert!(
        !harness.trace().iter().any(|entry| {
            matches!(
                entry.kind,
                TraceEntryKind::RemoteReceiptRequested
                    | TraceEntryKind::RemoteReceiptServed
                    | TraceEntryKind::RemoteReceiptReplied
                    | TraceEntryKind::RemoteReceiptReceived
            ) && entry.receipt_label.as_deref() == Some(label)
        }),
        "denied receipt release must not create request/serve/reply/receive trace rows for {label}"
    );
}

#[allow(clippy::too_many_arguments)]
fn assert_receipt_request_denied_without_matching_release(
    harness: &mut EvaluationMaterializationHarness,
    caller: Principal,
    producer: Locus,
    target: Locus,
    id: &str,
    label: &str,
    input_frontier: InputFrontier,
) {
    let diagnostic = harness
        .request_remote_i64_receipt(caller, producer, target, id, label, input_frontier, "Int64")
        .expect_err("receipt request requires an admitted matching release");
    assert_eq!(diagnostic.code, DiagnosticCode::ReceiptReleaseDenied);
    assert_no_receipt_flow_trace_for_label(harness, label);
}

fn run_two_accepted_rmw_trace() -> (
    EvaluationMaterializationHarness,
    Locus,
    Principal,
    Principal,
) {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let owner = locus("S");
    let target = principal("target");
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let capability = harness.grant_capability(caller.clone(), owner.clone(), "player", "hp");

    harness
        .enqueue(attack_operation(
            &caller,
            &requester_locus,
            &owner,
            &target,
            &capability,
        ))
        .expect("first attack request is accepted into the owner queue");
    harness
        .enqueue(attack_operation(
            &caller,
            &requester_locus,
            &owner,
            &target,
            &capability,
        ))
        .expect("second attack request is accepted into the owner queue");

    harness.service_owner_queue(&owner);

    (harness, owner, target, caller)
}

#[test]
fn owner_queue_services_two_accepted_rmw_requests_serially() {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let owner = locus("S");
    let target = principal("target");
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let capability = harness.grant_capability(caller.clone(), owner.clone(), "player", "hp");

    harness
        .enqueue(attack_operation(
            &caller,
            &requester_locus,
            &owner,
            &target,
            &capability,
        ))
        .expect("first attack request is accepted into the owner queue");
    harness
        .enqueue(attack_operation(
            &caller,
            &requester_locus,
            &owner,
            &target,
            &capability,
        ))
        .expect("second attack request is accepted into the owner queue");

    harness.service_owner_queue(&owner);

    assert_eq!(harness.i64_state(&owner, "player", &target, "hp"), Some(80));

    let accepted: Vec<_> = harness
        .trace()
        .iter()
        .filter(|entry| entry.kind == TraceEntryKind::OwnerServiceAccepted)
        .collect();
    assert_eq!(accepted.len(), 2);

    for entry in accepted {
        let plan = entry
            .eval_plan
            .as_ref()
            .expect("owner service trace carries the M3 eval plan");
        assert_eq!(plan.semantic_form, SemanticForm::State);
        assert_nonempty_operation_key(&plan.operation_key);
        assert_eq!(plan.evaluation_site, EvaluationSite::Owner(owner.clone()));
        assert_eq!(
            plan.authority_origin,
            AuthorityOrigin::Caller(caller.clone())
        );
        assert_eq!(plan.trigger, TriggerClock::OnRequest);
        assert_eq!(
            plan.materialization,
            materialization_plan([Materialization::Store])
        );
        assert_eq!(
            entry.requester_private_value_result, None,
            "same-owner RMW must not return S-private operand values to the requester"
        );
    }
    assert_all_trace_rows_have_operation_keys(&harness);
}

#[test]
fn deterministic_replay_of_same_owner_two_request_trace_matches_public_evidence() {
    let (first, first_owner, first_target, first_caller) = run_two_accepted_rmw_trace();
    let (second, second_owner, second_target, second_caller) = run_two_accepted_rmw_trace();

    assert_eq!(first_owner, second_owner);
    assert_eq!(first_target, second_target);
    assert_eq!(first_caller, second_caller);
    assert_eq!(
        first.i64_state(&first_owner, "player", &first_target, "hp"),
        Some(80)
    );
    assert_eq!(
        first.i64_state(&first_owner, "player", &first_target, "hp"),
        second.i64_state(&second_owner, "player", &second_target, "hp")
    );
    assert_eq!(first.trace(), second.trace());
    assert_eq!(first.trace().len(), second.trace().len());

    for (left, right) in first.trace().iter().zip(second.trace()) {
        assert_nonempty_operation_key(&left.operation_key);
        assert_eq!(&left.kind, &right.kind);
        assert_eq!(left.operation_key, right.operation_key);
        assert_eq!(
            left.requester_private_value_result,
            right.requester_private_value_result
        );

        match (&left.eval_plan, &right.eval_plan) {
            (Some(left_plan), Some(right_plan)) => {
                assert_nonempty_operation_key(&left_plan.operation_key);
                assert_eq!(left_plan.operation_key, right_plan.operation_key);
                assert_eq!(left_plan.semantic_form, right_plan.semantic_form);
                assert_eq!(left_plan.evaluation_site, right_plan.evaluation_site);
                assert_eq!(left_plan.trigger, right_plan.trigger);
                assert_eq!(left_plan.authority_origin, right_plan.authority_origin);
                assert_eq!(left_plan.materialization, right_plan.materialization);
                assert_eq!(left_plan.input_frontier, right_plan.input_frontier);
                assert_eq!(left_plan.remote_receipt, right_plan.remote_receipt);
                assert_eq!(
                    left_plan.requires_explicit_receipt,
                    right_plan.requires_explicit_receipt
                );
            }
            (None, None) => {}
            (left_plan, right_plan) => {
                panic!("replayed trace plan presence differs: {left_plan:?} vs {right_plan:?}");
            }
        }
    }

    let accepted_count = first
        .trace()
        .iter()
        .filter(|entry| entry.kind == TraceEntryKind::OwnerServiceAccepted)
        .count();
    assert_eq!(accepted_count, 2);
    assert_all_trace_rows_have_operation_keys(&first);
    assert_all_trace_rows_have_operation_keys(&second);
}

#[test]
fn invalid_capability_records_missing_capability_and_leaves_owner_state_untouched() {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let owner = locus("S");
    let target = principal("target");
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let invalid_capability = CapabilityRef::new("cap:missing");

    harness
        .enqueue(attack_operation(
            &caller,
            &requester_locus,
            &owner,
            &target,
            &invalid_capability,
        ))
        .expect("capability validity is checked by owner service");

    harness.service_owner_queue(&owner);

    assert_eq!(
        harness.i64_state(&owner, "player", &target, "hp"),
        Some(100)
    );
    let failure = harness
        .trace()
        .iter()
        .find(|entry| {
            matches!(
                &entry.kind,
                TraceEntryKind::OwnerServiceRejected(DiagnosticCode::MissingCapability)
            )
        })
        .expect("missing capability is an explicit owner-service occurrence");
    let plan = failure
        .eval_plan
        .as_ref()
        .expect("failure trace still carries the M3 eval plan");
    assert_nonempty_operation_key(&plan.operation_key);
    assert_eq!(plan.evaluation_site, EvaluationSite::Owner(owner));
    assert_eq!(plan.authority_origin, AuthorityOrigin::Caller(caller));
    assert_eq!(
        plan.materialization,
        materialization_plan([Materialization::Store])
    );
    assert_eq!(failure.requester_private_value_result, None);
    assert_all_trace_rows_have_operation_keys(&harness);
}

#[test]
fn unannotated_cross_owner_operand_is_a_diagnostic_before_queue_mutation() {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let owner = locus("S");
    let target = principal("target");
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let capability = harness.grant_capability(caller.clone(), owner.clone(), "player", "hp");

    let diagnostic = harness
        .enqueue(Operation::unannotated_cross_owner_operand(
            caller,
            requester_locus,
            owner.clone(),
            locus("RemoteStats"),
            "player",
            target.clone(),
            "hp",
            -10,
            capability,
        ))
        .expect_err("unannotated cross-owner operand must be rejected");

    assert_eq!(diagnostic.code, DiagnosticCode::CrossOwnerOperand);
    assert_eq!(
        harness.i64_state(&owner, "player", &target, "hp"),
        Some(100)
    );
    assert!(harness.trace().iter().any(|entry| {
        matches!(
            &entry.kind,
            TraceEntryKind::Diagnostic(DiagnosticCode::CrossOwnerOperand)
        )
    }));
    assert_all_trace_rows_have_operation_keys(&harness);
}

#[test]
fn remote_receipt_request_requires_admitted_release_matching_caller_target_and_label() {
    let caller = principal("browser:self");
    let other_caller = principal("browser:other");
    let requester_locus = locus("BrowserClient[self]");
    let producer = locus("Stats");
    let owner = locus("S");
    let other_target_owner = locus("OtherOwner");
    let target = principal("target");
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let write_capability = harness.grant_capability(caller.clone(), owner.clone(), "player", "hp");

    assert_receipt_request_denied_without_matching_release(
        &mut harness,
        caller.clone(),
        producer.clone(),
        owner.clone(),
        "receipt:release-denied:missing",
        "stats.bonus",
        input_frontier(&["serve:release-denied:missing"]),
    );

    harness.grant_receipt_release(
        caller.clone(),
        producer.clone(),
        owner.clone(),
        "stats.allowed",
    );
    assert_receipt_request_denied_without_matching_release(
        &mut harness,
        caller.clone(),
        producer.clone(),
        owner.clone(),
        "receipt:release-denied:label",
        "stats.denied",
        input_frontier(&["serve:release-denied:label"]),
    );

    harness.grant_receipt_release(
        caller.clone(),
        producer.clone(),
        owner.clone(),
        "stats.bonus",
    );
    assert_receipt_request_denied_without_matching_release(
        &mut harness,
        caller.clone(),
        producer.clone(),
        other_target_owner,
        "receipt:release-denied:target",
        "stats.bonus",
        input_frontier(&["serve:release-denied:target"]),
    );
    assert_receipt_request_denied_without_matching_release(
        &mut harness,
        other_caller,
        producer.clone(),
        owner.clone(),
        "receipt:release-denied:caller",
        "stats.bonus",
        input_frontier(&["serve:release-denied:caller"]),
    );

    let manual_receipt = RemoteReceipt::typed_i64(
        "receipt:release-denied:manual",
        producer,
        owner.clone(),
        "stats.bonus",
        input_frontier(&["serve:release-denied:manual"]),
        "Int64",
        -5,
    );
    let diagnostic = harness
        .enqueue(Operation::owner_rmw_using_remote_receipt(
            caller,
            requester_locus,
            owner.clone(),
            "player",
            target.clone(),
            "hp",
            manual_receipt,
            write_capability,
        ))
        .expect_err("manual or denied receipt is not admitted into the received receipt store");
    assert_eq!(diagnostic.code, DiagnosticCode::MissingReceipt);

    harness.service_owner_queue(&owner);
    assert_eq!(
        harness.i64_state(&owner, "player", &target, "hp"),
        Some(100)
    );
    assert_no_receipt_flow_trace_for_label(&harness, "stats.bonus");
    assert_all_trace_rows_have_operation_keys(&harness);
}

#[test]
fn owner_transition_using_remote_receipt_requires_preexisting_typed_causal_receipt() {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let owner = locus("S");
    let receipt_owner = locus("Stats");
    let target = principal("target");
    let receipt_frontier = input_frontier(&["serve:stats-bonus:1", "receive:stats-bonus:1"]);
    let receipt = RemoteReceipt::typed_i64(
        "receipt:stats-bonus:1",
        receipt_owner.clone(),
        owner.clone(),
        "stats.bonus",
        receipt_frontier.clone(),
        "Int64",
        -5,
    );
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let capability = harness.grant_capability(caller.clone(), owner.clone(), "player", "hp");
    assert_eq!(receipt.producer, receipt_owner);
    assert_eq!(receipt.target, owner);
    assert_eq!(receipt.label, "stats.bonus");

    let operation = Operation::owner_rmw_using_remote_receipt(
        caller.clone(),
        requester_locus.clone(),
        owner.clone(),
        "player",
        target.clone(),
        "hp",
        receipt.clone(),
        capability.clone(),
    );

    let diagnostic = harness
        .enqueue(operation.clone())
        .expect_err("receipt-dependent owner transition cannot run before receipt exists");
    assert_eq!(diagnostic.code, DiagnosticCode::MissingReceipt);
    assert_eq!(
        harness.i64_state(&owner, "player", &target, "hp"),
        Some(100)
    );

    let receipt = run_remote_i64_receipt_flow(
        &mut harness,
        caller.clone(),
        receipt_owner.clone(),
        owner.clone(),
        "receipt:stats-bonus:1",
        "stats.bonus",
        receipt_frontier.clone(),
        -5,
    );
    assert_receipt_flow_trace(&harness, &receipt);
    harness
        .enqueue(Operation::owner_rmw_using_remote_receipt(
            caller,
            requester_locus,
            owner.clone(),
            "player",
            target.clone(),
            "hp",
            receipt.clone(),
            capability,
        ))
        .expect("preexisting typed causal receipt can feed the owner transition");
    harness.service_owner_queue(&owner);

    assert_eq!(harness.i64_state(&owner, "player", &target, "hp"), Some(95));
    let accepted = harness
        .trace()
        .iter()
        .find(|entry| entry.kind == TraceEntryKind::OwnerServiceAccepted)
        .expect("receipt-fed owner transition is serviced");
    let plan = accepted
        .eval_plan
        .as_ref()
        .expect("receipt-fed transition trace carries the M3 eval plan");
    assert_eq!(plan.semantic_form, SemanticForm::State);
    assert_nonempty_operation_key(&plan.operation_key);
    assert_eq!(plan.input_frontier, Some(receipt_frontier));
    assert_eq!(plan.remote_receipt, Some(receipt));
    assert_eq!(
        plan.materialization,
        materialization_plan([Materialization::Store])
    );
    assert_eq!(accepted.requester_private_value_result, None);
    assert_all_trace_rows_have_operation_keys(&harness);
}

#[test]
fn remote_receipt_bound_to_different_target_owner_is_rejected_before_queue_mutation() {
    let caller = principal("browser:self");
    let requester_locus = locus("BrowserClient[self]");
    let owner = locus("S");
    let receipt_owner = locus("Stats");
    let wrong_target_owner = locus("OtherOwner");
    let target = principal("target");
    let receipt_frontier = input_frontier(&["serve:stats-bonus:2", "receive:stats-bonus:2"]);
    let receipt = RemoteReceipt::typed_i64(
        "receipt:stats-bonus:2",
        receipt_owner.clone(),
        wrong_target_owner.clone(),
        "stats.bonus",
        receipt_frontier,
        "Int64",
        -5,
    );
    let mut harness = EvaluationMaterializationHarness::default();

    harness.set_i64_state(owner.clone(), "player", target.clone(), "hp", 100);
    let capability = harness.grant_capability(caller.clone(), owner.clone(), "player", "hp");
    assert_eq!(receipt.producer, receipt_owner);
    assert_eq!(receipt.target, wrong_target_owner);
    assert_eq!(receipt.label, "stats.bonus");

    let registered_receipt = run_remote_i64_receipt_flow(
        &mut harness,
        caller.clone(),
        receipt.producer.clone(),
        receipt.target.clone(),
        "receipt:stats-bonus:2",
        "stats.bonus",
        receipt.input_frontier.clone(),
        -5,
    );
    assert_eq!(registered_receipt, receipt);
    assert_receipt_flow_trace(&harness, &receipt);

    let diagnostic = harness
        .enqueue(Operation::owner_rmw_using_remote_receipt(
            caller,
            requester_locus,
            owner.clone(),
            "player",
            target.clone(),
            "hp",
            receipt.clone(),
            capability,
        ))
        .expect_err("receipt target mismatch is rejected before owner queue mutation");

    assert_eq!(diagnostic.code, DiagnosticCode::ReceiptTargetMismatch);
    assert_eq!(
        harness.i64_state(&owner, "player", &target, "hp"),
        Some(100)
    );
    harness.service_owner_queue(&owner);
    assert_eq!(
        harness.i64_state(&owner, "player", &target, "hp"),
        Some(100)
    );
    assert!(!harness.trace().iter().any(|entry| {
        entry.kind == TraceEntryKind::OwnerRequestQueued
            && entry
                .eval_plan
                .as_ref()
                .and_then(|plan| plan.remote_receipt.as_ref())
                == Some(&receipt)
    }));
    assert_all_trace_rows_have_operation_keys(&harness);
}

#[test]
fn duplicate_designated_evaluation_at_same_key_and_frontier_publishes_once() {
    let evaluator = locus("RulesEngine");
    let frontier = input_frontier(&["serve:hp-read:10", "serve:atk-read:11"]);
    let reversed_frontier = input_frontier(&["serve:atk-read:11", "serve:hp-read:10"]);
    assert_eq!(frontier, reversed_frontier);
    let eval_policy = evaluation_policy("combat.damage.v1");
    let observe_policy = observation_policy("observer_safe:combat.damage");
    let policy_stamp = eval_policy.stamp_with(&observe_policy);
    let mut harness = EvaluationMaterializationHarness::default();
    let operation = Operation::designated_evaluation(
        AuthorityOrigin::AdmittedEvaluator(evaluator.clone()),
        evaluator.clone(),
        "combat.damage.approved",
        frontier.clone(),
        7,
        eval_policy.clone(),
        observe_policy.clone(),
        TriggerClock::FrontierAdvance,
    );
    let duplicate_operation = Operation::designated_evaluation(
        AuthorityOrigin::AdmittedEvaluator(evaluator.clone()),
        evaluator.clone(),
        "combat.damage.approved",
        reversed_frontier.clone(),
        7,
        eval_policy.clone(),
        observe_policy.clone(),
        TriggerClock::FrontierAdvance,
    );

    let first = harness
        .evaluate_designated(operation.clone())
        .expect("first designated evaluation publishes a value");
    let duplicate = harness
        .evaluate_designated(duplicate_operation)
        .expect("duplicate same key/frontier reuses the stable publication");

    assert_eq!(first, duplicate);
    assert_eq!(first.key, "combat.damage.approved");
    assert_eq!(first.input_frontier, frontier);
    assert_eq!(first.version, 1);
    assert_eq!(first.value_i64, 7);
    assert_eq!(first.evaluation_policy, eval_policy);
    assert_eq!(first.observation_policy, observe_policy);
    assert_eq!(first.policy_stamp, policy_stamp);
    assert_eq!(
        harness.published_values_for("combat.damage.approved", &first.input_frontier),
        vec![first.clone()]
    );
    assert_eq!(
        harness.semantic_consumption_count("combat.damage.approved", &first.input_frontier),
        0
    );

    let publish_trace = harness
        .trace()
        .iter()
        .find(|entry| entry.kind == TraceEntryKind::ValuePublished)
        .expect("the stable publication appears in the trace");
    let plan = publish_trace
        .eval_plan
        .as_ref()
        .expect("publish trace carries the M3 eval plan");
    assert_nonempty_operation_key(&publish_trace.operation_key);
    assert_nonempty_operation_key(&plan.operation_key);
    let published_value = match &publish_trace.detail {
        TraceDetail::PublishedValue(value) => value,
        other => panic!("publish trace must retain published policy detail, got {other:?}"),
    };
    assert_eq!(
        plan.evaluation_site,
        EvaluationSite::DesignatedEvaluator(evaluator.clone())
    );
    assert_eq!(
        plan.authority_origin,
        AuthorityOrigin::AdmittedEvaluator(evaluator.clone())
    );
    assert_eq!(plan.trigger, TriggerClock::FrontierAdvance);
    assert_eq!(
        plan.materialization,
        materialization_plan([Materialization::PublishValue])
    );
    assert_eq!(plan.input_frontier, Some(first.input_frontier.clone()));
    assert_eq!(
        plan.evaluation_policy,
        Some(first.evaluation_policy.clone())
    );
    assert_eq!(
        plan.observation_policy,
        Some(first.observation_policy.clone())
    );
    assert_eq!(plan.policy_stamp, Some(first.policy_stamp.clone()));
    assert_eq!(published_value.evaluation_policy, first.evaluation_policy);
    assert_eq!(published_value.observation_policy, first.observation_policy);
    assert_eq!(published_value.policy_stamp, first.policy_stamp);

    let publish_count = harness
        .trace()
        .iter()
        .filter(|entry| entry.kind == TraceEntryKind::ValuePublished)
        .count();
    let consumption_count = harness
        .trace()
        .iter()
        .filter(|entry| entry.kind == TraceEntryKind::SemanticConsumption)
        .count();
    assert_eq!(publish_count, 1);
    assert_eq!(
        consumption_count, 0,
        "designated evaluation publishes but does not consume for a consumer"
    );

    let consumer = principal("consumer:C");
    let first_consumption = harness
        .consume_designated(
            evaluator.clone(),
            "combat.damage.approved",
            reversed_frontier,
            consumer.clone(),
        )
        .expect("consumer consumption is explicit");
    assert!(
        !first_consumption.identity.as_str().is_empty(),
        "semantic consumption must carry a stable identity"
    );
    assert_eq!(first_consumption.consumer, consumer);
    assert_eq!(first_consumption.key, "combat.damage.approved");
    assert_eq!(first_consumption.input_frontier, first.input_frontier);
    assert_eq!(first_consumption.policy_stamp, first.policy_stamp);
    assert_eq!(
        harness.semantic_consumption_count("combat.damage.approved", &first.input_frontier),
        1
    );

    let duplicate_consumption = harness
        .consume_designated(
            evaluator,
            "combat.damage.approved",
            first.input_frontier.clone(),
            first_consumption.consumer.clone(),
        )
        .expect("duplicate consumer consumption returns the existing consumption identity");
    assert_eq!(duplicate_consumption, first_consumption);
    assert_eq!(
        harness.semantic_consumption_count("combat.damage.approved", &first.input_frontier),
        1
    );

    let consumption_trace = harness
        .trace()
        .iter()
        .find(|entry| entry.kind == TraceEntryKind::SemanticConsumption)
        .expect("explicit consumer consumption is traced exactly once");
    assert_nonempty_operation_key(&consumption_trace.operation_key);
    match &consumption_trace.detail {
        TraceDetail::SemanticConsumption(consumption) => {
            assert_eq!(consumption, &first_consumption);
        }
        other => panic!("semantic consumption trace must retain identity detail, got {other:?}"),
    }
    assert_all_trace_rows_have_operation_keys(&harness);
}
