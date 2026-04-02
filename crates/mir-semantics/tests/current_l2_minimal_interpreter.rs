use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_semantics::{
    CURRENT_L2_HOST_PLAN_SCHEMA_VERSION, FixtureHostPlan, FixtureHostStub,
    FixtureRuntimeRequirement, NamedProfileRunSummary, NonAdmissibleMetadata,
    NonAdmissibleSubreason, ProfileCatalog, ProfileRunSummary, SelectionMode, SelectionProfile,
    SelectionRequest, SelectionScope, SingleFixtureSelector, StaticGateVerdict,
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
fn static_gate_rejects_malformed_and_underdeclared_fixtures() {
    let malformed = load_fixture_from_path(fixture_path("e4-malformed-lineage.json")).unwrap();
    let underdeclared =
        load_fixture_from_path(fixture_path("e5-underdeclared-lineage.json")).unwrap();

    assert_eq!(static_gate(&malformed), StaticGateVerdict::Malformed);
    assert_eq!(
        static_gate(&underdeclared),
        StaticGateVerdict::Underdeclared
    );

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
        "e3-option-admit-chain.json",
        "e6-write-after-expiry.json",
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

    assert_eq!(discovery.total_candidates, 6);
    assert_eq!(discovery.failures.len(), 0);
    assert_eq!(discovery.bundles.len(), 6);
    assert_eq!(
        discovery
            .bundles
            .iter()
            .filter(|bundle| bundle.runtime_requirement
                == FixtureRuntimeRequirement::RuntimeWithHostPlan)
            .count(),
        4
    );
    assert_eq!(
        discovery
            .bundles
            .iter()
            .filter(|bundle| bundle.runtime_requirement == FixtureRuntimeRequirement::StaticOnly)
            .count(),
        2
    );
}

#[test]
fn run_directory_returns_summary_for_current_l2_fixture_dir() {
    let summary = run_directory(fixture_dir()).unwrap();

    assert_eq!(summary.total_bundles, 6);
    assert_eq!(summary.runtime_bundles, 4);
    assert_eq!(summary.static_only_bundles, 2);
    assert_eq!(summary.passed, 6);
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

    assert_eq!(selected.total_candidates, 4);
    assert_eq!(selected.runtime_bundles, 4);
    assert_eq!(selected.static_only_bundles, 0);
    assert_eq!(selected.failures.len(), 0);
    assert_eq!(
        stems,
        vec![
            "e1-place-atomic-cut",
            "e2-try-fallback",
            "e3-option-admit-chain",
            "e6-write-after-expiry",
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

    assert_eq!(selected.total_candidates, 2);
    assert_eq!(selected.runtime_bundles, 0);
    assert_eq!(selected.static_only_bundles, 2);
    assert_eq!(selected.failures.len(), 0);
    assert_eq!(stems, vec!["e4-malformed-lineage", "e5-underdeclared-lineage"]);
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

    assert_eq!(summary.total_bundles, 4);
    assert_eq!(summary.runtime_bundles, 4);
    assert_eq!(summary.static_only_bundles, 0);
    assert_eq!(summary.passed, 4);
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
fn run_directory_profiled_includes_profile_name_in_summary() {
    let profile = SelectionProfile::new(
        "runtime-all",
        SelectionRequest::new().with_scope(SelectionScope::RuntimeOnly),
    );

    let summary = run_directory_profiled(fixture_dir(), &profile).unwrap();

    assert_eq!(summary.profile_name, "runtime-all");
    assert_profile_selected_counts(&summary, 4, 4, 0);
    assert_eq!(summary.passed, 4);
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

fn assert_named_profile_selected_counts(
    summary: &NamedProfileRunSummary,
    total_selected_bundles: usize,
    runtime_selected_bundles: usize,
    static_selected_bundles: usize,
) {
    assert_eq!(summary.total_selected_bundles, total_selected_bundles);
    assert_eq!(summary.runtime_selected_bundles, runtime_selected_bundles);
    assert_eq!(summary.static_selected_bundles, static_selected_bundles);
}

#[derive(Clone, Copy)]
struct NamedProfileBehaviorCase {
    alias: &'static str,
    // Keep request expectations independent from ProfileCatalog::resolve().
    // These are literal public-behavior checks, not a second path into the
    // catalog implementation under test.
    expected_request: fn() -> SelectionRequest,
    total_selected_bundles: usize,
    runtime_selected_bundles: usize,
    static_selected_bundles: usize,
    passed: usize,
    failed: usize,
    selected_fixture_suffix: Option<&'static str>,
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
        total_selected_bundles: 4,
        runtime_selected_bundles: 4,
        static_selected_bundles: 0,
        passed: 4,
        failed: 0,
        selected_fixture_suffix: None,
    },
    NamedProfileBehaviorCase {
        alias: "smoke-static",
        expected_request: expected_smoke_static_request,
        total_selected_bundles: 2,
        runtime_selected_bundles: 0,
        static_selected_bundles: 2,
        passed: 2,
        failed: 0,
        selected_fixture_suffix: None,
    },
    NamedProfileBehaviorCase {
        alias: "runtime-e3",
        expected_request: expected_runtime_e3_request,
        total_selected_bundles: 1,
        runtime_selected_bundles: 1,
        static_selected_bundles: 0,
        passed: 1,
        failed: 0,
        selected_fixture_suffix: Some("e3-option-admit-chain.json"),
    },
    NamedProfileBehaviorCase {
        alias: "static-e4",
        expected_request: expected_static_e4_request,
        total_selected_bundles: 1,
        runtime_selected_bundles: 0,
        static_selected_bundles: 1,
        passed: 1,
        failed: 0,
        selected_fixture_suffix: Some("e4-malformed-lineage.json"),
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
        assert_named_profile_selected_counts(
            &summary,
            case.total_selected_bundles,
            case.runtime_selected_bundles,
            case.static_selected_bundles,
        );
        assert_eq!(summary.passed, case.passed);
        assert_eq!(summary.failed, case.failed);
        assert_eq!(summary.bundle_reports.len(), case.total_selected_bundles);

        if let Some(expected_suffix) = case.selected_fixture_suffix {
            assert_eq!(summary.bundle_reports.len(), 1);
            assert!(summary.bundle_reports[0]
                .fixture_path
                .ends_with(expected_suffix));
        }
    }
}

#[test]
fn run_directory_named_profile_rejects_unknown_alias() {
    let error = run_directory_named_profile(fixture_dir(), "no-such-profile").unwrap_err();

    assert!(error.to_string().contains("unknown named profile alias"));
}
