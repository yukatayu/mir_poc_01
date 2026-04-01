use std::path::PathBuf;

use mir_semantics::{
    EffectInput, EffectOracle, EffectVerdict, FixtureRuntimeOutcome, PredicateInput,
    PredicateOracle, PredicateSite, PredicateVerdict, StaticGateVerdict, SuccessCarrier,
    TerminalOutcome, load_fixture_from_path, run_to_completion, static_gate,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct StubCommit {
    target: String,
    record: String,
}

impl SuccessCarrier for StubCommit {
    fn preview_place_store(&self, place_store: &mir_semantics::PlaceStore) -> mir_semantics::PlaceStore {
        let mut preview = place_store.clone();
        preview
            .entry(self.target.clone())
            .or_default()
            .push(self.record.clone());
        preview
    }

    fn apply_place_store(self, place_store: &mut mir_semantics::PlaceStore) {
        place_store
            .entry(self.target)
            .or_default()
            .push(self.record);
    }
}

#[derive(Default)]
struct StubPredicateOracle;

impl PredicateOracle<PredicateInput> for StubPredicateOracle {
    fn eval_predicate(&mut self, input: PredicateInput) -> PredicateVerdict {
        match (input.site, input.op.as_str(), input.option_ref.as_deref()) {
            (PredicateSite::OptionAdmit, _, Some("owner_writer")) => PredicateVerdict::Unsatisfied,
            (PredicateSite::RequestRequire, "validate_profile_patch", _) => {
                PredicateVerdict::Unsatisfied
            }
            _ => PredicateVerdict::Satisfied,
        }
    }
}

#[derive(Default)]
struct StubEffectOracle;

impl EffectOracle<EffectInput, StubCommit> for StubEffectOracle {
    fn apply_effect(&mut self, input: EffectInput) -> EffectVerdict<StubCommit> {
        if input.op == "append_audit" {
            return EffectVerdict::ExplicitFailure;
        }

        let target = input.selected_target.clone();
        let label = input
            .selected_option_ref
            .clone()
            .unwrap_or_else(|| target.clone());

        EffectVerdict::Success {
            commit: StubCommit {
                target,
                record: format!("{}@{}", input.op, label),
            },
        }
    }
}

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

#[test]
fn static_gate_rejects_malformed_and_underdeclared_fixtures() {
    let malformed = load_fixture_from_path(fixture_path("e4-malformed-lineage.json")).unwrap();
    let underdeclared =
        load_fixture_from_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();

    assert_eq!(static_gate(&malformed), StaticGateVerdict::Malformed);
    assert_eq!(static_gate(&underdeclared), StaticGateVerdict::Underdeclared);

    for fixture in [&malformed, &underdeclared] {
        let mut predicate = StubPredicateOracle;
        let mut effect = StubEffectOracle;
        let result = run_to_completion(fixture, &mut predicate, &mut effect).unwrap();

        assert_eq!(result.static_verdict, fixture.expected_static.verdict);
        assert_eq!(
            result.entered_evaluation,
            fixture.expected_runtime.enters_evaluation
        );
        assert_eq!(
            result.terminal_outcome,
            expected_outcome(fixture.expected_runtime.final_outcome)
        );
        assert_eq!(
            result.trace_audit_sink.events,
            fixture.expected_trace_audit.event_kinds
        );
        assert_eq!(
            result.trace_audit_sink.narrative_explanations,
            fixture.expected_trace_audit.narrative_explanations
        );
        assert!(
            result.trace_audit_sink.non_admissible_metadata.is_empty(),
            "{}",
            fixture.fixture_id
        );
    }
}

#[test]
fn runtime_fixtures_reach_expected_outcomes() {
    let cases = [
        "e1-place-atomic-cut.json",
        "e2-try-fallback.json",
        "e3-option-admit-chain.json",
        "e6-write-after-expiry.json",
    ];

    for fixture_name in cases {
        let fixture = load_fixture_from_path(fixture_path(fixture_name)).unwrap();
        let mut predicate = StubPredicateOracle;
        let mut effect = StubEffectOracle;

        let result = run_to_completion(&fixture, &mut predicate, &mut effect).unwrap();

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
        assert_eq!(
            result.trace_audit_sink.events,
            fixture.expected_trace_audit.event_kinds,
            "{fixture_name}"
        );
        let expected_metadata = fixture
            .expected_trace_audit
            .non_admissible_metadata
            .iter()
            .map(|entry| mir_semantics::NonAdmissibleMetadata {
                option_ref: entry.option_ref.clone(),
                subreason: entry.subreason,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            result.trace_audit_sink.non_admissible_metadata,
            expected_metadata,
            "{fixture_name}"
        );
        assert_eq!(
            result.trace_audit_sink.narrative_explanations,
            fixture.expected_trace_audit.narrative_explanations,
            "{fixture_name}"
        );
    }
}
