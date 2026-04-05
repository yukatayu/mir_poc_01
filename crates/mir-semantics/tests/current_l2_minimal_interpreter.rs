use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_semantics::{
    CURRENT_L2_HOST_PLAN_SCHEMA_VERSION, CurrentL2Fixture, DirectStyleEvaluator, EffectInput,
    EffectOracle, EffectPlanRule, EffectPlanVerdict, EffectVerdict, FixtureCommitPlan,
    FixtureHostPlan, FixtureHostStub,
    FixtureRuntimeRequirement, NamedProfileRunSummary, NonAdmissibleMetadata,
    NonAdmissibleSubreason, ProfileCatalog, ProfileRunSummary, SelectionMode, SelectionProfile,
    PredicateInput, PredicateOracle, PredicatePlanRule, PredicateVerdict, SelectionRequest,
    SelectionScope, SingleFixtureSelector, StaticGateVerdict, StepControl,
    TraceExpectationOverride, discover_bundles_in_directory,
    host_plan_sidecar_path_for_fixture_path, load_bundle_from_fixture_path, load_fixture_from_path,
    load_host_plan_from_path, load_host_plan_sidecar_for_fixture_path, run_bundle, run_directory,
    run_directory_named_profile, run_directory_profiled, run_directory_selected,
    select_bundles_from_discovery, static_gate,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn load_bundle(name: &str) -> mir_semantics::FixtureBundle {
    load_bundle_from_fixture_path(fixture_path(name)).unwrap()
}

fn expected_outcome(
    outcome: mir_semantics::FixtureRuntimeOutcome,
) -> Option<mir_semantics::TerminalOutcome> {
    match outcome {
        mir_semantics::FixtureRuntimeOutcome::Success => {
            Some(mir_semantics::TerminalOutcome::Success)
        }
        mir_semantics::FixtureRuntimeOutcome::ExplicitFailure => {
            Some(mir_semantics::TerminalOutcome::ExplicitFailure)
        }
        mir_semantics::FixtureRuntimeOutcome::Reject => {
            Some(mir_semantics::TerminalOutcome::Reject)
        }
        mir_semantics::FixtureRuntimeOutcome::NotEvaluated => None,
    }
}

fn unique_temp_json_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "mir-semantics-{label}-{}-{nanos}.json",
        std::process::id()
    ))
}

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../mir-ast/tests/fixtures/current-l2")
}

#[derive(Debug, Clone)]
struct TestPredicateOracle {
    plan: FixtureHostPlan,
}

impl TestPredicateOracle {
    fn new(plan: FixtureHostPlan) -> Self {
        Self { plan }
    }
}

impl PredicateOracle<PredicateInput> for TestPredicateOracle {
    fn eval_predicate(&mut self, input: PredicateInput) -> PredicateVerdict {
        self.plan
            .predicate_rules
            .iter()
            .find(|rule| predicate_rule_matches(rule, &input))
            .map(|rule| rule.verdict)
            .unwrap_or(PredicateVerdict::Unsatisfied)
    }
}

#[derive(Debug, Clone)]
struct TestEffectOracle {
    plan: FixtureHostPlan,
}

impl TestEffectOracle {
    fn new(plan: FixtureHostPlan) -> Self {
        Self { plan }
    }
}

impl EffectOracle<EffectInput, FixtureCommitPlan> for TestEffectOracle {
    fn apply_effect(&mut self, input: EffectInput) -> EffectVerdict<FixtureCommitPlan> {
        if let Some(rule) = self
            .plan
            .effect_rules
            .iter()
            .find(|rule| effect_rule_matches(rule, &input))
        {
            return match &rule.verdict {
                EffectPlanVerdict::Success { commit } => EffectVerdict::Success {
                    commit: commit.clone(),
                },
                EffectPlanVerdict::ExplicitFailure => EffectVerdict::ExplicitFailure,
            };
        }

        EffectVerdict::ExplicitFailure
    }
}

fn predicate_rule_matches(rule: &PredicatePlanRule, input: &PredicateInput) -> bool {
    rule.site == input.site
        && rule.op == input.op
        && rule.mode.is_none_or(|mode| mode == input.mode)
        && rule
            .target
            .as_ref()
            .is_none_or(|target| Some(target) == input.target.as_ref())
        && rule
            .chain_ref
            .as_ref()
            .is_none_or(|chain_ref| Some(chain_ref) == input.chain_ref.as_ref())
        && rule
            .option_ref
            .as_ref()
            .is_none_or(|option_ref| Some(option_ref) == input.option_ref.as_ref())
}

fn effect_rule_matches(rule: &EffectPlanRule, input: &EffectInput) -> bool {
    rule.op == input.op
        && rule.mode.is_none_or(|mode| mode == input.mode)
        && rule
            .selected_target
            .as_ref()
            .is_none_or(|target| target == &input.selected_target)
        && rule
            .chain_ref
            .as_ref()
            .is_none_or(|chain_ref| Some(chain_ref) == input.chain_ref.as_ref())
        && rule
            .selected_option_ref
            .as_ref()
            .is_none_or(|option_ref| Some(option_ref) == input.selected_option_ref.as_ref())
}

fn run_direct_evaluator_with_plan(
    fixture: &CurrentL2Fixture,
    plan: FixtureHostPlan,
) -> DirectStyleEvaluator {
    let mut evaluator = DirectStyleEvaluator::from_fixture(fixture).unwrap();
    let mut predicate_oracle = TestPredicateOracle::new(plan.clone());
    let mut effect_oracle = TestEffectOracle::new(plan);

    loop {
        match evaluator
            .step_once(&mut predicate_oracle, &mut effect_oracle)
            .unwrap()
        {
            StepControl::Continue | StepControl::BubbleFailure(_) => {}
            StepControl::Halt(_) => return evaluator,
        }
    }
}

#[test]
fn bundle_loader_resolves_runtime_and_static_fixtures() {
    let runtime_bundle =
        load_bundle_from_fixture_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    assert_eq!(
        runtime_bundle.runtime_requirement,
        FixtureRuntimeRequirement::RuntimeWithHostPlan
    );
    assert!(runtime_bundle.host_plan_path.is_some());
    assert!(runtime_bundle.host_plan.is_some());

    let static_bundle =
        load_bundle_from_fixture_path(fixture_path("e4-malformed-lineage.json")).unwrap();
    assert_eq!(
        static_bundle.runtime_requirement,
        FixtureRuntimeRequirement::StaticOnly
    );
    assert!(static_bundle.host_plan_path.is_none());
    assert!(static_bundle.host_plan.is_none());
}

#[test]
fn bundle_loader_requires_host_plan_for_runtime_fixture() {
    let source_path = fixture_path("e1-place-atomic-cut.json");
    let temp_dir = std::env::temp_dir().join(format!(
        "mir-semantics-bundle-missing-sidecar-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();

    let copied_fixture_path = temp_dir.join("e1-place-atomic-cut.json");
    fs::copy(&source_path, &copied_fixture_path).unwrap();

    let error = load_bundle_from_fixture_path(&copied_fixture_path).unwrap_err();
    let _ = fs::remove_dir_all(&temp_dir);

    assert!(
        error
            .to_string()
            .contains("runtime fixture requires a .host-plan.json sidecar")
    );
}

#[test]
fn run_bundle_checks_static_runtime_and_trace_expectations() {
    let runtime_bundle =
        load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let runtime_result = run_bundle(&runtime_bundle).unwrap();
    assert_eq!(
        runtime_result.report.terminal_outcome,
        expected_outcome(runtime_bundle.fixture.expected_runtime.final_outcome)
    );
    assert_eq!(
        runtime_result.report.trace_audit_sink.events,
        runtime_bundle.fixture.expected_trace_audit.event_kinds
    );

    let static_bundle =
        load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();
    let static_result = run_bundle(&static_bundle).unwrap();
    assert_eq!(
        static_result.report.static_verdict,
        StaticGateVerdict::Underdeclared
    );
    assert!(!static_result.report.entered_evaluation);
}

#[test]
fn run_bundle_rejects_checked_static_reason_mismatch() {
    let mut bundle =
        load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();
    bundle.fixture.expected_static.checked_reasons = Some(vec![
        "wrong machine-check carrier".to_string(),
        "should fail closed".to_string(),
    ]);

    let err = run_bundle(&bundle).unwrap_err();
    let message = err.to_string();

    assert!(
        message.contains("bundle static checked_reasons mismatch"),
        "{message}"
    );
    assert!(message.contains("e5_underdeclared_lineage"), "{message}");
}

#[test]
fn run_bundle_allows_explanatory_static_reasons_without_checked_carrier() {
    let mut bundle =
        load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    bundle.fixture.expected_static.reasons = vec![
        "different explanatory prose should stay outside machine-check".to_string(),
    ];
    bundle.fixture.expected_static.checked_reasons = None;

    let report = run_bundle(&bundle).unwrap();

    assert_eq!(report.report.static_verdict, StaticGateVerdict::Valid);
}

#[test]
fn run_bundle_accepts_matching_checked_static_reasons() {
    let mut bundle =
        load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();
    bundle.fixture.expected_static.checked_reasons = Some(vec![
        "missing lineage assertion for primary -> mirror".to_string(),
    ]);

    let report = run_bundle(&bundle).unwrap();

    assert_eq!(
        report.report.static_verdict,
        bundle.fixture.expected_static.verdict
    );
}

#[test]
fn run_bundle_accepts_matching_checked_reason_codes_for_lineage_pair_family() {
    let bundle =
        load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();

    let report = run_bundle(&bundle).unwrap();

    assert_eq!(
        report.report.static_verdict,
        bundle.fixture.expected_static.verdict
    );
}

#[test]
fn run_bundle_rejects_checked_reason_code_mismatch() {
    let mut bundle =
        load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();
    bundle.fixture.expected_static.checked_reason_codes = Some(vec![
        mir_semantics::StaticReasonCodeRow::LineageAssertionEdgeMismatch {
            predecessor: "primary".to_string(),
            successor: "mirror".to_string(),
        },
    ]);

    let err = run_bundle(&bundle).unwrap_err();
    let message = err.to_string();

    assert!(
        message.contains("bundle static checked_reason_codes mismatch"),
        "{message}"
    );
    assert!(message.contains("e5_underdeclared_lineage"), "{message}");
}

#[test]
fn run_bundle_accepts_matching_checked_reason_codes_for_target_pair_family() {
    let missing_target =
        load_bundle_from_fixture_path(fixture_path("e12-underdeclared-target-missing.json"))
            .unwrap();
    let missing_target_report = run_bundle(&missing_target).unwrap();

    assert_eq!(
        missing_target_report.report.static_verdict,
        missing_target.fixture.expected_static.verdict
    );

    let target_mismatch =
        load_bundle_from_fixture_path(fixture_path("e19-malformed-target-mismatch.json"))
            .unwrap();
    let target_mismatch_report = run_bundle(&target_mismatch).unwrap();

    assert_eq!(
        target_mismatch_report.report.static_verdict,
        target_mismatch.fixture.expected_static.verdict
    );
}

#[test]
fn run_bundle_accepts_matching_checked_reason_codes_for_remaining_stable_families() {
    let strengthening = load_bundle_from_fixture_path(fixture_path(
        "e13-malformed-capability-strengthening.json",
    ))
    .unwrap();
    let strengthening_report = run_bundle(&strengthening).unwrap();

    assert_eq!(
        strengthening_report.report.static_verdict,
        strengthening.fixture.expected_static.verdict
    );

    let late_strengthening = load_bundle_from_fixture_path(fixture_path(
        "e20-malformed-late-capability-strengthening.json",
    ))
    .unwrap();
    let late_strengthening_report = run_bundle(&late_strengthening).unwrap();

    assert_eq!(
        late_strengthening_report.report.static_verdict,
        late_strengthening.fixture.expected_static.verdict
    );

    let missing_head = load_bundle_from_fixture_path(fixture_path(
        "e16-malformed-missing-chain-head-option.json",
    ))
    .unwrap();
    let missing_head_report = run_bundle(&missing_head).unwrap();

    assert_eq!(
        missing_head_report.report.static_verdict,
        missing_head.fixture.expected_static.verdict
    );

    let missing_predecessor = load_bundle_from_fixture_path(fixture_path(
        "e17-malformed-missing-predecessor-option.json",
    ))
    .unwrap();
    let missing_predecessor_report = run_bundle(&missing_predecessor).unwrap();

    assert_eq!(
        missing_predecessor_report.report.static_verdict,
        missing_predecessor.fixture.expected_static.verdict
    );

    let missing_successor = load_bundle_from_fixture_path(fixture_path(
        "e18-malformed-missing-successor-option.json",
    ))
    .unwrap();
    let missing_successor_report = run_bundle(&missing_successor).unwrap();

    assert_eq!(
        missing_successor_report.report.static_verdict,
        missing_successor.fixture.expected_static.verdict
    );
}

#[test]
fn static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording() {
    let malformed = load_bundle_from_fixture_path(fixture_path("e4-malformed-lineage.json"))
        .expect("fixture should load");
    assert_eq!(
        malformed.fixture.expected_static.checked_reasons,
        Some(vec!["lineage assertion does not describe primary -> mirror".to_string()])
    );
    assert_eq!(
        malformed.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::LineageAssertionEdgeMismatch {
            predecessor: "primary".to_string(),
            successor: "mirror".to_string(),
        }])
    );

    let underdeclared = load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json"))
        .expect("fixture should load");
    assert_eq!(
        underdeclared.fixture.expected_static.checked_reasons,
        Some(vec!["missing lineage assertion for primary -> mirror".to_string()])
    );
    assert_eq!(
        underdeclared.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::MissingLineageAssertion {
            predecessor: "primary".to_string(),
            successor: "mirror".to_string(),
        }])
    );

    let missing_target = load_bundle_from_fixture_path(fixture_path(
        "e12-underdeclared-target-missing.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        missing_target.fixture.expected_static.checked_reasons,
        Some(vec!["declared access target is missing for primary -> mirror".to_string()])
    );
    assert_eq!(
        missing_target.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::DeclaredTargetMissing {
            predecessor: "primary".to_string(),
            successor: "mirror".to_string(),
        }])
    );

    let strengthening = load_bundle_from_fixture_path(fixture_path(
        "e13-malformed-capability-strengthening.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        strengthening.fixture.expected_static.checked_reasons,
        Some(vec!["capability strengthens from read to write".to_string()])
    );
    assert_eq!(
        strengthening.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::CapabilityStrengthens {
            from_capability: "read".to_string(),
            to_capability: "write".to_string(),
        }])
    );

    let late_strengthening = load_bundle_from_fixture_path(fixture_path(
        "e20-malformed-late-capability-strengthening.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        late_strengthening.fixture.expected_static.checked_reasons,
        Some(vec!["capability strengthens from read to write".to_string()])
    );
    assert_eq!(
        late_strengthening.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::CapabilityStrengthens {
            from_capability: "read".to_string(),
            to_capability: "write".to_string(),
        }])
    );

    let duplicate_option = load_bundle_from_fixture_path(fixture_path(
        "e14-malformed-duplicate-option-declaration.json",
    ))
    .expect("fixture should load");
    assert_eq!(duplicate_option.fixture.expected_static.checked_reasons, None);
    assert_eq!(duplicate_option.fixture.expected_static.checked_reason_codes, None);

    let duplicate_chain = load_bundle_from_fixture_path(fixture_path(
        "e15-malformed-duplicate-chain-declaration.json",
    ))
    .expect("fixture should load");
    assert_eq!(duplicate_chain.fixture.expected_static.checked_reasons, None);
    assert_eq!(duplicate_chain.fixture.expected_static.checked_reason_codes, None);

    let missing_head = load_bundle_from_fixture_path(fixture_path(
        "e16-malformed-missing-chain-head-option.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        missing_head.fixture.expected_static.checked_reasons,
        Some(vec![
            "missing option declaration for chain head ghost at root / session / profile_access"
                .to_string(),
        ])
    );
    assert_eq!(
        missing_head.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::MissingChainHeadOption {
            head: "ghost".to_string(),
            scope: "root / session / profile_access".to_string(),
        }])
    );

    let missing_predecessor = load_bundle_from_fixture_path(fixture_path(
        "e17-malformed-missing-predecessor-option.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        missing_predecessor.fixture.expected_static.checked_reasons,
        Some(vec![
            "missing predecessor option ghost at root / session / profile_access".to_string(),
        ])
    );
    assert_eq!(
        missing_predecessor.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::MissingPredecessorOption {
            option: "ghost".to_string(),
            scope: "root / session / profile_access".to_string(),
        }])
    );

    let missing_successor = load_bundle_from_fixture_path(fixture_path(
        "e18-malformed-missing-successor-option.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        missing_successor.fixture.expected_static.checked_reasons,
        Some(vec![
            "missing successor option ghost at root / session / profile_access".to_string(),
        ])
    );
    assert_eq!(
        missing_successor.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::MissingSuccessorOption {
            option: "ghost".to_string(),
            scope: "root / session / profile_access".to_string(),
        }])
    );

    let target_mismatch = load_bundle_from_fixture_path(fixture_path(
        "e19-malformed-target-mismatch.json",
    ))
    .expect("fixture should load");
    assert_eq!(
        target_mismatch.fixture.expected_static.checked_reasons,
        Some(vec![
            "declared access target mismatch between primary and mirror".to_string(),
        ])
    );
    assert_eq!(
        target_mismatch.fixture.expected_static.checked_reason_codes,
        Some(vec![mir_semantics::StaticReasonCodeRow::DeclaredTargetMismatch {
            predecessor: "primary".to_string(),
            successor: "mirror".to_string(),
        }])
    );

    let explanatory_valid =
        load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    assert_eq!(explanatory_valid.fixture.expected_static.checked_reasons, None);
    assert_eq!(explanatory_valid.fixture.expected_static.checked_reason_codes, None);
}

#[test]
fn static_gate_rejects_malformed_and_underdeclared_fixtures() {
    let malformed = load_fixture_from_path(fixture_path("e4-malformed-lineage.json")).unwrap();
    let malformed_strengthening = load_fixture_from_path(fixture_path(
        "e13-malformed-capability-strengthening.json",
    ))
    .unwrap();
    let malformed_duplicate_option = load_fixture_from_path(fixture_path(
        "e14-malformed-duplicate-option-declaration.json",
    ))
    .unwrap();
    let malformed_duplicate_chain = load_fixture_from_path(fixture_path(
        "e15-malformed-duplicate-chain-declaration.json",
    ))
    .unwrap();
    let malformed_missing_head = load_fixture_from_path(fixture_path(
        "e16-malformed-missing-chain-head-option.json",
    ))
    .unwrap();
    let malformed_missing_predecessor = load_fixture_from_path(fixture_path(
        "e17-malformed-missing-predecessor-option.json",
    ))
    .unwrap();
    let malformed_missing_successor = load_fixture_from_path(fixture_path(
        "e18-malformed-missing-successor-option.json",
    ))
    .unwrap();
    let malformed_target_mismatch =
        load_fixture_from_path(fixture_path("e19-malformed-target-mismatch.json")).unwrap();
    let malformed_late_strengthening = load_fixture_from_path(fixture_path(
        "e20-malformed-late-capability-strengthening.json",
    ))
    .unwrap();
    let underdeclared =
        load_fixture_from_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();
    let underdeclared_target_missing = load_fixture_from_path(fixture_path(
        "e12-underdeclared-target-missing.json",
    ))
    .unwrap();

    assert_eq!(static_gate(&malformed), StaticGateVerdict::Malformed);
    assert_eq!(
        static_gate(&malformed_strengthening),
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        static_gate(&malformed_duplicate_option),
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        static_gate(&malformed_duplicate_chain),
        StaticGateVerdict::Malformed
    );
    assert_eq!(static_gate(&malformed_missing_head), StaticGateVerdict::Malformed);
    assert_eq!(
        static_gate(&malformed_missing_predecessor),
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        static_gate(&malformed_missing_successor),
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        static_gate(&malformed_target_mismatch),
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        static_gate(&malformed_late_strengthening),
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        static_gate(&underdeclared),
        StaticGateVerdict::Underdeclared
    );
    assert_eq!(
        static_gate(&underdeclared_target_missing),
        StaticGateVerdict::Underdeclared
    );

    for fixture in [
        &malformed,
        &malformed_strengthening,
        &malformed_late_strengthening,
        &malformed_duplicate_option,
        &malformed_duplicate_chain,
        &malformed_missing_head,
        &malformed_missing_predecessor,
        &malformed_missing_successor,
        &malformed_target_mismatch,
        &underdeclared,
        &underdeclared_target_missing,
    ] {
        let harness = FixtureHostStub::default();
        let result = harness.run_fixture(fixture).unwrap();

        assert_eq!(result.static_verdict, fixture.expected_static.verdict);
        assert_eq!(
            result.entered_evaluation,
            fixture.expected_runtime.enters_evaluation
        );
        assert_eq!(result.terminal_outcome, None);
    }
}

#[test]
fn runtime_fixtures_reach_expected_outcomes_via_declarative_host_plan() {
    let cases = [
        "e1-place-atomic-cut.json",
        "e2-try-fallback.json",
        "e21-try-atomic-cut-frontier.json",
        "e3-option-admit-chain.json",
        "e6-write-after-expiry.json",
        "e7-write-fallback-after-expiry.json",
        "e8-monotone-degradation-reject.json",
        "e9-monotone-degradation-success.json",
        "e10-perform-on-ensure-failure.json",
        "e11-perform-via-ensure-then-success.json",
    ];

    for fixture_name in cases {
        let bundle = load_bundle(fixture_name);
        let result = run_bundle(&bundle).unwrap();
        assert_eq!(
            result.report.static_verdict, bundle.fixture.expected_static.verdict,
            "{fixture_name}"
        );
        assert_eq!(
            result.report.entered_evaluation, bundle.fixture.expected_runtime.enters_evaluation,
            "{fixture_name}"
        );
        assert!(result.report.terminal_outcome.is_some(), "{fixture_name}");
    }
}

#[test]
fn trace_and_audit_expectations_follow_fixture_or_harness_override() {
    let cases = [
        "e1-place-atomic-cut.json",
        "e2-try-fallback.json",
        "e21-try-atomic-cut-frontier.json",
        "e3-option-admit-chain.json",
        "e6-write-after-expiry.json",
        "e7-write-fallback-after-expiry.json",
        "e8-monotone-degradation-reject.json",
        "e9-monotone-degradation-success.json",
        "e10-perform-on-ensure-failure.json",
        "e11-perform-via-ensure-then-success.json",
    ];

    for fixture_name in cases {
        let bundle = load_bundle(fixture_name);
        let harness = FixtureHostStub::new(bundle.host_plan.clone().unwrap()).unwrap();
        let result = run_bundle(&bundle).unwrap();

        assert_eq!(
            result.report.trace_audit_sink.events, bundle.fixture.expected_trace_audit.event_kinds,
            "{fixture_name}"
        );
        assert_eq!(
            result.report.trace_audit_sink.non_admissible_metadata,
            harness.expected_non_admissible_metadata(&bundle.fixture),
            "{fixture_name}"
        );
        assert_eq!(
            result.report.trace_audit_sink.narrative_explanations,
            harness.expected_narrative_explanations(&bundle.fixture),
            "{fixture_name}"
        );
    }
}

#[test]
fn fallback_and_lease_regression_fixtures_preserve_current_l2_boundaries() {
    let cases = [
        (
            "e7-write-fallback-after-expiry.json",
            Some(mir_semantics::TerminalOutcome::Success),
            vec![mir_semantics::EventKind::PerformSuccess],
            vec![NonAdmissibleMetadata {
                option_ref: "writer".to_string(),
                subreason: NonAdmissibleSubreason::LeaseExpired,
            }],
            Vec::<String>::new(),
        ),
        (
            "e8-monotone-degradation-reject.json",
            Some(mir_semantics::TerminalOutcome::Reject),
            vec![
                mir_semantics::EventKind::PerformFailure,
                mir_semantics::EventKind::Reject,
            ],
            vec![NonAdmissibleMetadata {
                option_ref: "owner_writer".to_string(),
                subreason: NonAdmissibleSubreason::AdmitMiss,
            }],
            vec!["readonly remains a request/capability mismatch narrative explanation".to_string()],
        ),
    ];

    for (
        fixture_name,
        expected_outcome,
        expected_events,
        expected_metadata,
        expected_narrative,
    ) in cases
    {
        let bundle = load_bundle(fixture_name);
        let result = run_bundle(&bundle).unwrap();

        assert_eq!(result.report.terminal_outcome, expected_outcome, "{fixture_name}");
        assert_eq!(result.report.trace_audit_sink.events, expected_events, "{fixture_name}");
        assert_eq!(
            result.report.trace_audit_sink.non_admissible_metadata,
            expected_metadata,
            "{fixture_name}"
        );
        assert_eq!(
            result.report.trace_audit_sink.narrative_explanations,
            expected_narrative,
            "{fixture_name}"
        );
    }
}

#[test]
fn monotone_degradation_can_reach_later_success_after_middle_failure() {
    let bundle = load_bundle("e9-monotone-degradation-success.json");
    let result = run_bundle(&bundle).unwrap();

    assert_eq!(
        result.report.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::Success)
    );
    assert_eq!(
        result.report.trace_audit_sink.events,
        vec![
            mir_semantics::EventKind::PerformFailure,
            mir_semantics::EventKind::PerformSuccess,
        ]
    );
    assert_eq!(
        result.report.trace_audit_sink.non_admissible_metadata,
        vec![NonAdmissibleMetadata {
            option_ref: "owner_writer".to_string(),
            subreason: NonAdmissibleSubreason::AdmitMiss,
        }]
    );
    assert_eq!(result.report.trace_audit_sink.narrative_explanations, Vec::<String>::new());
}

#[test]
fn harness_can_override_trace_expectation_without_changing_runtime_plan() {
    let fixture_path = fixture_path("e6-write-after-expiry.json");
    let fixture = load_fixture_from_path(&fixture_path).unwrap();
    let mut plan = load_host_plan_sidecar_for_fixture_path(&fixture_path).unwrap();
    plan.trace_expectation_override = TraceExpectationOverride {
        non_admissible_metadata: Some(vec![NonAdmissibleMetadata {
            option_ref: "override_writer".into(),
            subreason: NonAdmissibleSubreason::LeaseExpired,
        }]),
        narrative_explanations: Some(vec!["custom narrative".into()]),
    };
    let harness = FixtureHostStub::new(plan).unwrap();

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

#[test]
fn perform_on_ensure_failure_returns_explicit_failure_without_non_admissible_metadata() {
    let bundle = load_bundle("e10-perform-on-ensure-failure.json");
    let result = run_bundle(&bundle).unwrap();

    assert_eq!(
        result.report.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::ExplicitFailure)
    );
    assert_eq!(
        result.report.trace_audit_sink.events,
        vec![mir_semantics::EventKind::PerformFailure]
    );
    assert!(
        result.report.trace_audit_sink.non_admissible_metadata.is_empty(),
        "direct ensure failure should not fabricate non-admissible metadata"
    );
    assert!(
        result
            .report
            .trace_audit_sink
            .narrative_explanations
            .is_empty(),
        "direct ensure failure should not need capability-mismatch narrative explanation"
    );

    let evaluator =
        run_direct_evaluator_with_plan(&bundle.fixture, bundle.host_plan.clone().unwrap());
    assert_eq!(
        evaluator.state.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::ExplicitFailure)
    );
    assert!(
        evaluator.state.place_store.is_empty(),
        "request-local ensure failure must not commit previewed mutations into place_store"
    );
}

#[test]
fn perform_via_ensure_failure_can_continue_to_later_success() {
    let bundle = load_bundle("e11-perform-via-ensure-then-success.json");
    let result = run_bundle(&bundle).unwrap();

    assert_eq!(
        result.report.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::Success)
    );
    assert_eq!(
        result.report.trace_audit_sink.events,
        vec![
            mir_semantics::EventKind::PerformFailure,
            mir_semantics::EventKind::PerformSuccess,
        ]
    );
    assert!(
        result.report.trace_audit_sink.non_admissible_metadata.is_empty(),
        "request-local ensure failure should not fabricate non-admissible metadata"
    );
    assert!(
        result
            .report
            .trace_audit_sink
            .narrative_explanations
            .is_empty(),
        "request-local ensure continuation should not need capability mismatch narrative explanation"
    );

    let evaluator =
        run_direct_evaluator_with_plan(&bundle.fixture, bundle.host_plan.clone().unwrap());
    assert_eq!(
        evaluator.state.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::Success)
    );
    assert_eq!(
        evaluator.state.place_store,
        BTreeMap::from([(
            "profile_doc".to_string(),
            vec!["write_profile@backup_writer".to_string()]
        )]),
        "earlier ensure failure must discard delegated_writer preview and commit only the later successful writer"
    );
}

#[test]
fn try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback() {
    let bundle = load_bundle("e21-try-atomic-cut-frontier.json");
    let result = run_bundle(&bundle).unwrap();

    assert_eq!(
        result.report.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::Success)
    );
    assert_eq!(
        result.report.trace_audit_sink.events,
        vec![
            mir_semantics::EventKind::PerformSuccess,
            mir_semantics::EventKind::AtomicCut,
            mir_semantics::EventKind::PerformSuccess,
            mir_semantics::EventKind::PerformFailure,
            mir_semantics::EventKind::Rollback,
            mir_semantics::EventKind::PerformSuccess,
        ]
    );
    assert!(
        result.report.trace_audit_sink.non_admissible_metadata.is_empty(),
        "try-body rollback frontier update should not fabricate non-admissible metadata"
    );
    assert!(
        result
            .report
            .trace_audit_sink
            .narrative_explanations
            .is_empty(),
        "try-body rollback frontier update should not require narrative explanation"
    );

    let evaluator =
        run_direct_evaluator_with_plan(&bundle.fixture, bundle.host_plan.clone().unwrap());
    assert_eq!(
        evaluator.state.terminal_outcome,
        Some(mir_semantics::TerminalOutcome::Success)
    );
    assert_eq!(
        evaluator.state.place_store,
        BTreeMap::from([
            (
                "profile_draft".to_string(),
                vec!["stage_profile_patch@profile_draft".to_string()],
            ),
            (
                "profile_snapshot".to_string(),
                vec!["load_last_snapshot@profile_snapshot".to_string()],
            ),
        ]),
        "AtomicCut inside try body must advance the rollback frontier so that pre-cut mutations remain while post-cut mutations roll back before fallback succeeds"
    );
}

#[test]
fn host_plan_loader_reads_sidecar_for_runtime_fixture() {
    let path = fixture_path("e3-option-admit-chain.json");
    let sidecar = host_plan_sidecar_path_for_fixture_path(&path);
    let plan = load_host_plan_from_path(&sidecar).unwrap();

    assert_eq!(
        sidecar,
        fixture_path("e3-option-admit-chain.host-plan.json")
    );
    assert_eq!(plan.predicate_rules.len(), 3);
    assert_eq!(plan.effect_rules.len(), 1);
    assert!(
        plan.trace_expectation_override
            .non_admissible_metadata
            .is_none()
    );
}

#[test]
fn host_plan_loader_rejects_unknown_schema_version() {
    let path = unique_temp_json_path("host-plan-schema");
    fs::write(
        &path,
        format!(
            r#"{{
  "schema_version": "wrong-schema",
  "predicate_rules": [],
  "effect_rules": [],
  "trace_expectation_override": {{}}
}}"#
        ),
    )
    .unwrap();

    let error = load_host_plan_from_path(&path).unwrap_err();
    let _ = fs::remove_file(&path);

    let message = error.to_string();
    assert!(message.contains("unsupported host plan schema version"));
    assert!(message.contains("wrong-schema"));
}

#[test]
fn host_plan_loader_rejects_overlapping_predicate_rules() {
    let path = unique_temp_json_path("host-plan-overlap");
    fs::write(
        &path,
        format!(
            r#"{{
  "schema_version": "{schema}",
  "predicate_rules": [
    {{
      "site": "request-require",
      "op": "validate_profile_patch",
      "verdict": "unsatisfied"
    }},
    {{
      "site": "request-require",
      "op": "validate_profile_patch",
      "mode": "on",
      "verdict": "satisfied"
    }}
  ],
  "effect_rules": [],
  "trace_expectation_override": {{}}
}}"#,
            schema = CURRENT_L2_HOST_PLAN_SCHEMA_VERSION
        ),
    )
    .unwrap();

    let error = load_host_plan_from_path(&path).unwrap_err();
    let _ = fs::remove_file(&path);

    assert!(
        error
            .to_string()
            .contains("overlapping predicate plan rules are forbidden")
    );
}

#[test]
fn host_plan_loader_rejects_overlapping_effect_rules() {
    let path = unique_temp_json_path("host-plan-effect-overlap");
    fs::write(
        &path,
        format!(
            r#"{{
  "schema_version": "{schema}",
  "predicate_rules": [],
  "effect_rules": [
    {{
      "op": "write_profile",
      "mode": "via",
      "chain_ref": "profile_ref",
      "verdict": {{
        "kind": "explicit-failure"
      }}
    }},
    {{
      "op": "write_profile",
      "mode": "via",
      "chain_ref": "profile_ref",
      "selected_option_ref": "delegated_writer",
      "verdict": {{
        "kind": "success",
        "commit": {{
          "mutations": []
        }}
      }}
    }}
  ],
  "trace_expectation_override": {{}}
}}"#,
            schema = CURRENT_L2_HOST_PLAN_SCHEMA_VERSION
        ),
    )
    .unwrap();

    let error = load_host_plan_from_path(&path).unwrap_err();
    let _ = fs::remove_file(&path);

    assert!(
        error
            .to_string()
            .contains("overlapping effect plan rules are forbidden")
    );
}

#[test]
fn harness_rejects_uncovered_oracle_calls_without_synthetic_success_commit() {
    let fixture = load_fixture_from_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    let harness = FixtureHostStub::new(FixtureHostPlan::default()).unwrap();

    let error = harness.run_fixture(&fixture).unwrap_err();
    let message = error.to_string();

    assert!(message.contains("host plan did not cover all oracle calls"));
    assert!(!message.contains("Success { commit"));
}

#[test]
fn discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only() {
    let discovery = discover_bundles_in_directory(fixture_dir()).unwrap();

    assert_eq!(discovery.total_candidates, 21);
    assert_eq!(discovery.failures.len(), 0);
    assert_eq!(discovery.bundles.len(), 21);
    assert_eq!(
        discovery
            .bundles
            .iter()
            .filter(|bundle| bundle.runtime_requirement
                == FixtureRuntimeRequirement::RuntimeWithHostPlan)
            .count(),
        10
    );
    assert_eq!(
        discovery
            .bundles
            .iter()
            .filter(|bundle| bundle.runtime_requirement == FixtureRuntimeRequirement::StaticOnly)
            .count(),
        11
    );
}

#[test]
fn run_directory_returns_summary_for_current_l2_fixture_dir() {
    let summary = run_directory(fixture_dir()).unwrap();

    assert_eq!(summary.total_bundles, 21);
    assert_eq!(summary.runtime_bundles, 10);
    assert_eq!(summary.static_only_bundles, 11);
    assert_eq!(summary.passed, 21);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.discovery_failures.len(), 0);
    assert_eq!(summary.host_plan_coverage_failures.len(), 0);
}

#[test]
fn run_directory_reports_discovery_failure_for_runtime_fixture_without_sidecar() {
    let temp_dir = std::env::temp_dir().join(format!(
        "mir-semantics-batch-missing-sidecar-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();
    fs::copy(
        fixture_path("e1-place-atomic-cut.json"),
        temp_dir.join("e1-place-atomic-cut.json"),
    )
    .unwrap();

    let summary = run_directory(&temp_dir).unwrap();
    let _ = fs::remove_dir_all(&temp_dir);

    assert_eq!(summary.total_bundles, 1);
    assert_eq!(summary.runtime_bundles, 1);
    assert_eq!(summary.static_only_bundles, 0);
    assert_eq!(summary.discovery_failures.len(), 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.host_plan_coverage_failures.len(), 0);
    assert!(summary.discovery_failures.iter().any(|failure| {
        failure
            .error
            .contains("runtime fixture requires a .host-plan.json sidecar")
    }));
}

#[test]
fn run_directory_reports_host_plan_coverage_failures_in_summary() {
    let temp_dir = std::env::temp_dir().join(format!(
        "mir-semantics-batch-host-plan-coverage-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).unwrap();
    fs::copy(
        fixture_path("e1-place-atomic-cut.json"),
        temp_dir.join("e1-place-atomic-cut.json"),
    )
    .unwrap();
    fs::write(
        temp_dir.join("e1-place-atomic-cut.host-plan.json"),
        format!(
            r#"{{
  "schema_version": "{schema}",
  "predicate_rules": [],
  "effect_rules": [],
  "trace_expectation_override": {{}}
}}"#,
            schema = CURRENT_L2_HOST_PLAN_SCHEMA_VERSION
        ),
    )
    .unwrap();

    let summary = run_directory(&temp_dir).unwrap();
    let _ = fs::remove_dir_all(&temp_dir);

    assert_eq!(summary.total_bundles, 1);
    assert_eq!(summary.runtime_bundles, 1);
    assert_eq!(summary.static_only_bundles, 0);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.discovery_failures.len(), 0);
    assert_eq!(summary.host_plan_coverage_failures.len(), 1);
    assert!(summary.bundle_reports.iter().any(|report| {
        matches!(
            &report.outcome,
            mir_semantics::BatchBundleOutcome::Failed {
                error,
                host_plan_coverage_failure: true
            } if error.contains("host plan did not cover all oracle calls")
        )
    }));
}

#[test]
fn selection_runtime_only_keeps_only_runtime_bundles() {
    let discovery = discover_bundles_in_directory(fixture_dir()).unwrap();
    let selected = select_bundles_from_discovery(discovery, &SelectionMode::RuntimeOnly).unwrap();

    let stems: Vec<_> = selected
        .bundles
        .iter()
        .map(|bundle| {
            bundle
                .fixture_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    assert_eq!(selected.total_candidates, 10);
    assert_eq!(selected.runtime_bundles, 10);
    assert_eq!(selected.static_only_bundles, 0);
    assert_eq!(selected.failures.len(), 0);
    assert_eq!(
        stems,
        vec![
            "e1-place-atomic-cut",
            "e10-perform-on-ensure-failure",
            "e11-perform-via-ensure-then-success",
            "e2-try-fallback",
            "e21-try-atomic-cut-frontier",
            "e3-option-admit-chain",
            "e6-write-after-expiry",
            "e7-write-fallback-after-expiry",
            "e8-monotone-degradation-reject",
            "e9-monotone-degradation-success",
        ]
    );
}

#[test]
fn selection_static_only_keeps_only_static_bundles() {
    let discovery = discover_bundles_in_directory(fixture_dir()).unwrap();
    let selected = select_bundles_from_discovery(discovery, &SelectionMode::StaticOnly).unwrap();

    let stems: Vec<_> = selected
        .bundles
        .iter()
        .map(|bundle| {
            bundle
                .fixture_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    assert_eq!(selected.total_candidates, 11);
    assert_eq!(selected.runtime_bundles, 0);
    assert_eq!(selected.static_only_bundles, 11);
    assert_eq!(selected.failures.len(), 0);
    assert_eq!(
        stems,
        vec![
            "e12-underdeclared-target-missing",
            "e13-malformed-capability-strengthening",
            "e14-malformed-duplicate-option-declaration",
            "e15-malformed-duplicate-chain-declaration",
            "e16-malformed-missing-chain-head-option",
            "e17-malformed-missing-predecessor-option",
            "e18-malformed-missing-successor-option",
            "e19-malformed-target-mismatch",
            "e20-malformed-late-capability-strengthening",
            "e4-malformed-lineage",
            "e5-underdeclared-lineage",
        ]
    );
}

#[test]
fn run_directory_selected_single_fixture_runs_only_requested_fixture() {
    let summary = run_directory_selected(
        fixture_dir(),
        &SelectionMode::SingleFixture(SingleFixtureSelector::Stem(
            "e2-try-fallback".to_string(),
        )),
    )
    .unwrap();

    assert_eq!(summary.total_bundles, 1);
    assert_eq!(summary.runtime_bundles, 1);
    assert_eq!(summary.static_only_bundles, 0);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.bundle_reports.len(), 1);
    assert!(summary.bundle_reports[0]
        .fixture_path
        .ends_with("e2-try-fallback.json"));
}

#[test]
fn run_directory_selected_runtime_only_executes_only_runtime_bundles() {
    let summary = run_directory_selected(fixture_dir(), &SelectionMode::RuntimeOnly).unwrap();

    assert_eq!(summary.total_bundles, 10);
    assert_eq!(summary.runtime_bundles, 10);
    assert_eq!(summary.static_only_bundles, 0);
    assert_eq!(summary.passed, 10);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.discovery_failures.len(), 0);
    assert_eq!(summary.host_plan_coverage_failures.len(), 0);
}

#[test]
fn run_directory_selected_single_fixture_accepts_path_selector() {
    let summary = run_directory_selected(
        fixture_dir(),
        &SelectionMode::SingleFixture(SingleFixtureSelector::Path(fixture_path(
            "e6-write-after-expiry.json",
        ))),
    )
    .unwrap();

    assert_eq!(summary.total_bundles, 1);
    assert_eq!(summary.runtime_bundles, 1);
    assert_eq!(summary.static_only_bundles, 0);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.bundle_reports.len(), 1);
    assert!(summary.bundle_reports[0]
        .fixture_path
        .ends_with("e6-write-after-expiry.json"));
}

#[test]
fn run_directory_selected_rejects_unknown_single_fixture() {
    let error = run_directory_selected(
        fixture_dir(),
        &SelectionMode::SingleFixture(SingleFixtureSelector::Stem(
            "does-not-exist".to_string(),
        )),
    )
    .unwrap_err();

    assert!(error.to_string().contains("selected fixture was not found"));
}

fn assert_profile_selected_counts(
    summary: &ProfileRunSummary,
    total_selected_bundles: usize,
    runtime_selected_bundles: usize,
    static_selected_bundles: usize,
) {
    assert_eq!(summary.total_selected_bundles, total_selected_bundles);
    assert_eq!(summary.runtime_selected_bundles, runtime_selected_bundles);
    assert_eq!(summary.static_selected_bundles, static_selected_bundles);
}

// Profile-layer integration tests own selected-count and concrete fixture-shape
// coverage. Named-profile tests below should only verify alias resolution,
// literal resolved_request behavior, unknown alias failure, and thin
// delegation into this profiled execution path.
#[test]
fn run_directory_profiled_runtime_single_fixture_runs_one_runtime_bundle() {
    let profile = SelectionProfile::new(
        "runtime-e2",
        SelectionRequest::new()
            .with_scope(SelectionScope::RuntimeOnly)
            .with_single_fixture(SingleFixtureSelector::Stem("e2-try-fallback".to_string())),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "runtime-e2");
    assert_profile_selected_counts(&summary, 1, 1, 0);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.bundle_reports.len(), 1);
    assert!(summary.bundle_reports[0]
        .fixture_path
        .ends_with("e2-try-fallback.json"));
}

#[test]
fn run_directory_profiled_runtime_e3_runs_one_runtime_bundle() {
    let profile = SelectionProfile::new(
        "runtime-e3",
        SelectionRequest::new()
            .with_scope(SelectionScope::RuntimeOnly)
            .with_single_fixture(SingleFixtureSelector::Stem(
                "e3-option-admit-chain".to_string(),
            )),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "runtime-e3");
    assert_profile_selected_counts(&summary, 1, 1, 0);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.bundle_reports.len(), 1);
    assert!(summary.bundle_reports[0]
        .fixture_path
        .ends_with("e3-option-admit-chain.json"));
}

#[test]
fn run_directory_profiled_static_single_fixture_runs_one_static_bundle() {
    let profile = SelectionProfile::new(
        "static-e4",
        SelectionRequest::new()
            .with_scope(SelectionScope::StaticOnly)
            .with_single_fixture(SingleFixtureSelector::Stem(
                "e4-malformed-lineage".to_string(),
            )),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "static-e4");
    assert_profile_selected_counts(&summary, 1, 0, 1);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.bundle_reports.len(), 1);
    assert!(summary.bundle_reports[0]
        .fixture_path
        .ends_with("e4-malformed-lineage.json"));
}

#[test]
fn run_directory_profiled_runtime_path_selector_runs_requested_bundle() {
    let profile = SelectionProfile::new(
        "runtime-path-e6",
        SelectionRequest::new()
            .with_scope(SelectionScope::RuntimeOnly)
            .with_single_fixture(SingleFixtureSelector::Path(fixture_path(
                "e6-write-after-expiry.json",
            ))),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "runtime-path-e6");
    assert_profile_selected_counts(&summary, 1, 1, 0);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.bundle_reports.len(), 1);
    assert!(summary.bundle_reports[0]
        .fixture_path
        .ends_with("e6-write-after-expiry.json"));
}

#[test]
fn run_directory_profiled_static_only_includes_profile_name_in_summary() {
    let profile = SelectionProfile::new(
        "static-all",
        SelectionRequest::new().with_scope(SelectionScope::StaticOnly),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "static-all");
    assert_profile_selected_counts(&summary, 11, 0, 11);
    assert_eq!(summary.passed, 11);
    assert_eq!(summary.failed, 0);
}

#[test]
fn run_directory_profiled_includes_profile_name_in_summary() {
    let profile = SelectionProfile::new(
        "runtime-all",
        SelectionRequest::new().with_scope(SelectionScope::RuntimeOnly),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "runtime-all");
    assert_profile_selected_counts(&summary, 10, 10, 0);
    assert_eq!(summary.passed, 10);
    assert_eq!(summary.failed, 0);
}

#[test]
fn run_directory_profiled_rejects_unknown_single_fixture() {
    let profile = SelectionProfile::new(
        "missing-runtime",
        SelectionRequest::new()
            .with_scope(SelectionScope::RuntimeOnly)
            .with_single_fixture(SingleFixtureSelector::Stem(
                "does-not-exist".to_string(),
            )),
    );

    let error = run_directory_profiled(fixture_dir(), &profile).unwrap_err();

    assert!(error.to_string().contains("selected fixture was not found"));
}

// Keep public named-profile request expectations literal in this file, but let
// selection-shape behavior reuse the existing profile-layer execution path.
// This keeps the catalog integration test focused on alias -> request
// resolution, unknown alias handling, and thin delegation, instead of
// restating count/suffix facts already owned by the profile-layer tests above.
fn assert_named_profile_execution_matches_profiled_summary(
    alias: &str,
    summary: &NamedProfileRunSummary,
    expected_request: SelectionRequest,
) {
    let expected_profile = SelectionProfile::new(alias, expected_request);
    let profiled = run_directory_profiled(fixture_dir(), &expected_profile).unwrap();

    assert_eq!(summary.total_selected_bundles, profiled.total_selected_bundles);
    assert_eq!(
        summary.runtime_selected_bundles,
        profiled.runtime_selected_bundles
    );
    assert_eq!(
        summary.static_selected_bundles,
        profiled.static_selected_bundles
    );
    assert_eq!(summary.passed, profiled.passed);
    assert_eq!(summary.failed, profiled.failed);
    assert_eq!(summary.discovery_failures, profiled.discovery_failures);
    assert_eq!(summary.bundle_failures, profiled.bundle_failures);
    assert_eq!(
        summary.host_plan_coverage_failures,
        profiled.host_plan_coverage_failures
    );
    assert_eq!(summary.bundle_reports, profiled.bundle_reports);
}

#[derive(Clone, Copy)]
struct NamedProfileBehaviorCase {
    alias: &'static str,
    // Keep request expectations independent from ProfileCatalog::resolve().
    // These are literal public-behavior checks, not a second path into the
    // catalog implementation under test.
    expected_request: fn() -> SelectionRequest,
}

fn expected_smoke_runtime_request() -> SelectionRequest {
    SelectionRequest::new().with_scope(SelectionScope::RuntimeOnly)
}

fn expected_smoke_static_request() -> SelectionRequest {
    SelectionRequest::new().with_scope(SelectionScope::StaticOnly)
}

fn expected_runtime_e3_request() -> SelectionRequest {
    SelectionRequest::new()
        .with_scope(SelectionScope::RuntimeOnly)
        .with_single_fixture(SingleFixtureSelector::Stem(
            "e3-option-admit-chain".to_string(),
        ))
}

fn expected_static_e4_request() -> SelectionRequest {
    SelectionRequest::new()
        .with_scope(SelectionScope::StaticOnly)
        .with_single_fixture(SingleFixtureSelector::Stem(
            "e4-malformed-lineage".to_string(),
        ))
}

const NAMED_PROFILE_BEHAVIOR_CASES: &[NamedProfileBehaviorCase] = &[
    NamedProfileBehaviorCase {
        alias: "smoke-runtime",
        expected_request: expected_smoke_runtime_request,
    },
    NamedProfileBehaviorCase {
        alias: "smoke-static",
        expected_request: expected_smoke_static_request,
    },
    NamedProfileBehaviorCase {
        alias: "runtime-e3",
        expected_request: expected_runtime_e3_request,
    },
    NamedProfileBehaviorCase {
        alias: "static-e4",
        expected_request: expected_static_e4_request,
    },
];

fn expected_named_profile_aliases() -> Vec<&'static str> {
    NAMED_PROFILE_BEHAVIOR_CASES
        .iter()
        .map(|case| case.alias)
        .collect()
}

#[test]
fn named_profile_catalog_lists_expected_aliases() {
    let aliases = ProfileCatalog::aliases();

    assert_eq!(aliases, expected_named_profile_aliases());
}

#[test]
fn run_directory_named_profiles_match_catalog_resolution_and_expected_selection() {
    for case in NAMED_PROFILE_BEHAVIOR_CASES {
        let summary = run_directory_named_profile(fixture_dir(), case.alias).unwrap();

        assert_eq!(summary.profile_name, case.alias);
        assert_eq!(summary.resolved_request, (case.expected_request)());
        assert_named_profile_execution_matches_profiled_summary(
            case.alias,
            &summary,
            (case.expected_request)(),
        );
    }
}

#[test]
fn run_directory_named_profile_rejects_unknown_alias() {
    let error = run_directory_named_profile(fixture_dir(), "no-such-profile").unwrap_err();

    assert!(error.to_string().contains("unknown named profile alias"));
}
