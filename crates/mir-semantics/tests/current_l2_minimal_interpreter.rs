use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use mir_semantics::{
    CURRENT_L2_HOST_PLAN_SCHEMA_VERSION, FixtureHostPlan, FixtureHostStub, FixtureRuntimeRequirement, NonAdmissibleMetadata,
    NonAdmissibleSubreason, StaticGateVerdict, TraceExpectationOverride,
    host_plan_sidecar_path_for_fixture_path, load_bundle_from_fixture_path,
    load_fixture_from_path, load_host_plan_from_path, load_host_plan_sidecar_for_fixture_path,
    run_bundle, static_gate,
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
        mir_semantics::FixtureRuntimeOutcome::Success => Some(mir_semantics::TerminalOutcome::Success),
        mir_semantics::FixtureRuntimeOutcome::ExplicitFailure => {
            Some(mir_semantics::TerminalOutcome::ExplicitFailure)
        }
        mir_semantics::FixtureRuntimeOutcome::Reject => Some(mir_semantics::TerminalOutcome::Reject),
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

#[test]
fn bundle_loader_resolves_runtime_and_static_fixtures() {
    let runtime_bundle = load_bundle_from_fixture_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    assert_eq!(
        runtime_bundle.runtime_requirement,
        FixtureRuntimeRequirement::RuntimeWithHostPlan
    );
    assert!(runtime_bundle.host_plan_path.is_some());
    assert!(runtime_bundle.host_plan.is_some());

    let static_bundle = load_bundle_from_fixture_path(fixture_path("e4-malformed-lineage.json")).unwrap();
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

    assert!(error
        .to_string()
        .contains("runtime fixture requires a .host-plan.json sidecar"));
}

#[test]
fn run_bundle_checks_static_runtime_and_trace_expectations() {
    let runtime_bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let runtime_result = run_bundle(&runtime_bundle).unwrap();
    assert_eq!(
        runtime_result.report.terminal_outcome,
        expected_outcome(runtime_bundle.fixture.expected_runtime.final_outcome)
    );
    assert_eq!(
        runtime_result.report.trace_audit_sink.events,
        runtime_bundle.fixture.expected_trace_audit.event_kinds
    );

    let static_bundle = load_bundle_from_fixture_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();
    let static_result = run_bundle(&static_bundle).unwrap();
    assert_eq!(static_result.report.static_verdict, StaticGateVerdict::Underdeclared);
    assert!(!static_result.report.entered_evaluation);
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
        assert_eq!(result.terminal_outcome, None);
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
        let bundle = load_bundle(fixture_name);
        let result = run_bundle(&bundle).unwrap();
        assert_eq!(result.report.static_verdict, bundle.fixture.expected_static.verdict, "{fixture_name}");
        assert_eq!(result.report.entered_evaluation, bundle.fixture.expected_runtime.enters_evaluation, "{fixture_name}");
        assert!(result.report.terminal_outcome.is_some(), "{fixture_name}");
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
        let bundle = load_bundle(fixture_name);
        let harness = FixtureHostStub::new(bundle.host_plan.clone().unwrap()).unwrap();
        let result = run_bundle(&bundle).unwrap();

        assert_eq!(
            result.report.trace_audit_sink.events,
            bundle.fixture.expected_trace_audit.event_kinds,
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

#[test]
fn harness_rejects_uncovered_oracle_calls_without_synthetic_success_commit() {
    let fixture = load_fixture_from_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    let harness = FixtureHostStub::new(FixtureHostPlan::default()).unwrap();

    let error = harness.run_fixture(&fixture).unwrap_err();
    let message = error.to_string();

    assert!(message.contains("host plan did not cover all oracle calls"));
    assert!(!message.contains("Success { commit"));
}
