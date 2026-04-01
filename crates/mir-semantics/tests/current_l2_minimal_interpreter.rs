use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use mir_semantics::{
    CURRENT_L2_HOST_PLAN_SCHEMA_VERSION, FixtureHostStub, FixtureRuntimeOutcome,
    NonAdmissibleMetadata, NonAdmissibleSubreason, StaticGateVerdict, TerminalOutcome,
    TraceExpectationOverride, host_plan_sidecar_path_for_fixture_path, load_fixture_from_path,
    load_host_plan_from_path, load_host_plan_sidecar_for_fixture_path, static_gate,
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

fn load_runtime_fixture_and_harness(name: &str) -> (mir_semantics::CurrentL2Fixture, FixtureHostStub) {
    let path = fixture_path(name);
    let fixture = load_fixture_from_path(&path).unwrap();
    let plan = load_host_plan_sidecar_for_fixture_path(&path).unwrap();
    let harness = FixtureHostStub::new(plan).unwrap();
    (fixture, harness)
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
        let (fixture, harness) = load_runtime_fixture_and_harness(fixture_name);
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
        let (fixture, harness) = load_runtime_fixture_and_harness(fixture_name);
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
fn host_plan_loader_reads_sidecar_for_runtime_fixture() {
    let path = fixture_path("e3-option-admit-chain.json");
    let sidecar = host_plan_sidecar_path_for_fixture_path(&path);
    let plan = load_host_plan_from_path(&sidecar).unwrap();

    assert_eq!(sidecar, fixture_path("e3-option-admit-chain.host-plan.json"));
    assert_eq!(plan.predicate_rules.len(), 3);
    assert_eq!(plan.effect_rules.len(), 1);
    assert!(plan.trace_expectation_override.non_admissible_metadata.is_none());
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

    assert!(error
        .to_string()
        .contains("overlapping predicate plan rules are forbidden"));
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

    assert!(error
        .to_string()
        .contains("overlapping effect plan rules are forbidden"));
}
