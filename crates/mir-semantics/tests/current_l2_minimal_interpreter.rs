use std::path::PathBuf;

use mir_semantics::{
    EffectPlanRule, FixtureHostPlan, FixtureHostStub, FixtureRuntimeOutcome,
    FixtureStoreMutation, NonAdmissibleMetadata, NonAdmissibleSubreason, PredicatePlanRule,
    PredicateSite, StaticGateVerdict, TerminalOutcome, TraceExpectationOverride,
    load_fixture_from_path, static_gate,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn expected_outcome(outcome: FixtureRuntimeOutcome) -> Option<TerminalOutcome> {
    match outcome {
        FixtureRuntimeOutcome::Success => Some(TerminalOutcome::Success),
        FixtureRuntimeOutcome::ExplicitFailure => Some(TerminalOutcome::ExplicitFailure),
        FixtureRuntimeOutcome::Reject => Some(TerminalOutcome::Reject),
        FixtureRuntimeOutcome::NotEvaluated => None,
    }
}

fn host_plan_for_fixture(name: &str) -> FixtureHostPlan {
    match name {
        "e1-place-atomic-cut.json" => FixtureHostPlan {
            effect_rules: vec![
                EffectPlanRule::success_on(
                    "update_authority",
                    "profile_authority",
                    vec![FixtureStoreMutation::append_record(
                        "profile_authority",
                        "update_authority@profile_authority",
                    )],
                ),
                EffectPlanRule::explicit_failure_on("append_audit", "authority_log"),
            ],
            ..FixtureHostPlan::default()
        },
        "e2-try-fallback.json" => FixtureHostPlan {
            predicate_rules: vec![PredicatePlanRule::unsatisfied(
                PredicateSite::RequestRequire,
                "validate_profile_patch",
            )],
            effect_rules: vec![
                EffectPlanRule::success_on(
                    "stage_profile_patch",
                    "profile_draft",
                    vec![FixtureStoreMutation::append_record(
                        "profile_draft",
                        "stage_profile_patch@profile_draft",
                    )],
                ),
                EffectPlanRule::success_on(
                    "load_last_snapshot",
                    "profile_snapshot",
                    vec![FixtureStoreMutation::append_record(
                        "profile_snapshot",
                        "load_last_snapshot@profile_snapshot",
                    )],
                ),
            ],
            ..FixtureHostPlan::default()
        },
        "e3-option-admit-chain.json" => FixtureHostPlan {
            predicate_rules: vec![PredicatePlanRule::unsatisfied_for_option(
                PredicateSite::OptionAdmit,
                "write_profile",
                "owner_writer",
            )],
            effect_rules: vec![EffectPlanRule::success_via(
                "write_profile",
                "profile_ref",
                "delegated_writer",
                "profile_doc",
                vec![FixtureStoreMutation::append_record(
                    "profile_doc",
                    "write_profile@delegated_writer",
                )],
            )],
            ..FixtureHostPlan::default()
        },
        "e6-write-after-expiry.json" => FixtureHostPlan::default(),
        _ => FixtureHostPlan::default(),
    }
}

#[test]
fn static_gate_rejects_malformed_and_underdeclared_fixtures() {
    let malformed = load_fixture_from_path(fixture_path("e4-malformed-lineage.json")).unwrap();
    let underdeclared =
        load_fixture_from_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();

    assert_eq!(static_gate(&malformed), StaticGateVerdict::Malformed);
    assert_eq!(static_gate(&underdeclared), StaticGateVerdict::Underdeclared);

    for fixture in [&malformed, &underdeclared] {
        let harness = FixtureHostStub::default();
        let result = harness.run_fixture(fixture).unwrap();

        assert_eq!(result.static_verdict, fixture.expected_static.verdict);
        assert_eq!(
            result.entered_evaluation,
            fixture.expected_runtime.enters_evaluation
        );
        assert_eq!(
            result.terminal_outcome,
            expected_outcome(fixture.expected_runtime.final_outcome)
        );
    }
}

#[test]
fn runtime_fixtures_reach_expected_outcomes_via_declarative_host_plan() {
    let cases = [
        "e1-place-atomic-cut.json",
        "e2-try-fallback.json",
        "e3-option-admit-chain.json",
        "e6-write-after-expiry.json",
    ];

    for fixture_name in cases {
        let fixture = load_fixture_from_path(fixture_path(fixture_name)).unwrap();
        let harness = FixtureHostStub::new(host_plan_for_fixture(fixture_name));
        let result = harness.run_fixture(&fixture).unwrap();

        assert_eq!(result.static_verdict, fixture.expected_static.verdict, "{fixture_name}");
        assert_eq!(
            result.entered_evaluation,
            fixture.expected_runtime.enters_evaluation,
            "{fixture_name}"
        );
        assert_eq!(
            result.terminal_outcome,
            expected_outcome(fixture.expected_runtime.final_outcome),
            "{fixture_name}"
        );
    }
}

#[test]
fn trace_and_audit_expectations_follow_fixture_or_harness_override() {
    let cases = [
        "e1-place-atomic-cut.json",
        "e2-try-fallback.json",
        "e3-option-admit-chain.json",
        "e6-write-after-expiry.json",
    ];

    for fixture_name in cases {
        let fixture = load_fixture_from_path(fixture_path(fixture_name)).unwrap();
        let harness = FixtureHostStub::new(host_plan_for_fixture(fixture_name));
        let result = harness.run_fixture(&fixture).unwrap();

        assert_eq!(
            result.trace_audit_sink.events,
            fixture.expected_trace_audit.event_kinds,
            "{fixture_name}"
        );
        assert_eq!(
            result.trace_audit_sink.non_admissible_metadata,
            harness.expected_non_admissible_metadata(&fixture),
            "{fixture_name}"
        );
        assert_eq!(
            result.trace_audit_sink.narrative_explanations,
            harness.expected_narrative_explanations(&fixture),
            "{fixture_name}"
        );
    }
}

#[test]
fn harness_can_override_trace_expectation_without_changing_runtime_plan() {
    let fixture = load_fixture_from_path(fixture_path("e6-write-after-expiry.json")).unwrap();
    let mut plan = host_plan_for_fixture("e6-write-after-expiry.json");
    plan.trace_expectation_override = TraceExpectationOverride {
        non_admissible_metadata: Some(vec![NonAdmissibleMetadata {
            option_ref: "override_writer".into(),
            subreason: NonAdmissibleSubreason::LeaseExpired,
        }]),
        narrative_explanations: Some(vec!["custom narrative".into()]),
    };
    let harness = FixtureHostStub::new(plan);

    assert_eq!(
        harness.expected_non_admissible_metadata(&fixture),
        vec![NonAdmissibleMetadata {
            option_ref: "override_writer".into(),
            subreason: NonAdmissibleSubreason::LeaseExpired,
        }]
    );
    assert_eq!(
        harness.expected_narrative_explanations(&fixture),
        vec!["custom narrative".to_string()]
    );
}
