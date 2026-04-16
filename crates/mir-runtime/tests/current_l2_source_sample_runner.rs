use std::{fs, path::PathBuf};

use mir_runtime::current_l2::{
    CurrentL2TryRollbackStructuralFindingKind, run_current_l2_source_sample,
};
use mir_semantics::{
    EventKind, FixtureHostPlan, NonAdmissibleSubreason, StaticGateVerdict, TerminalOutcome,
    load_bundle_from_fixture_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/current-l2")
        .join(name)
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e1_sample() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    let report =
        run_current_l2_source_sample("e1-place-atomic-cut", bundle.host_plan.unwrap()).unwrap();

    assert_eq!(report.sample_id, "e1-place-atomic-cut");
    assert_eq!(report.sample_path, sample_path("e1-place-atomic-cut.txt"));
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::ExplicitFailure)
    );
    assert_eq!(
        report.runtime_report.run_report.trace_audit_sink.events,
        vec![
            EventKind::PerformSuccess,
            EventKind::AtomicCut,
            EventKind::PerformFailure,
        ]
    );
}

#[test]
fn current_l2_source_sample_runner_resolves_named_e2_sample_and_runs_runtime() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report = run_current_l2_source_sample("e2-try-fallback", bundle.host_plan.unwrap()).unwrap();

    assert_eq!(report.sample_id, "e2-try-fallback");
    assert_eq!(report.sample_path, sample_path("e2-try-fallback.txt"));
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.runtime_report.run_report.trace_audit_sink.events,
        vec![
            EventKind::PerformSuccess,
            EventKind::PerformFailure,
            EventKind::Rollback,
            EventKind::PerformSuccess,
        ]
    );
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e21_sample() {
    let bundle =
        load_bundle_from_fixture_path(fixture_path("e21-try-atomic-cut-frontier.json")).unwrap();
    let report =
        run_current_l2_source_sample("e21-try-atomic-cut-frontier", bundle.host_plan.unwrap())
            .unwrap();

    assert_eq!(report.sample_id, "e21-try-atomic-cut-frontier");
    assert_eq!(report.sample_path, sample_path("e21-try-atomic-cut-frontier.txt"));
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.runtime_report.run_report.trace_audit_sink.events,
        vec![
            EventKind::PerformSuccess,
            EventKind::AtomicCut,
            EventKind::PerformSuccess,
            EventKind::PerformFailure,
            EventKind::Rollback,
            EventKind::PerformSuccess,
        ]
    );
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e22_sample() {
    let bundle = load_bundle_from_fixture_path(fixture_path(
        "e22-try-atomic-cut-place-mismatch.json",
    ))
    .unwrap();
    let report = run_current_l2_source_sample(
        "e22-try-atomic-cut-place-mismatch",
        bundle.host_plan.unwrap(),
    )
    .unwrap();

    assert_eq!(report.sample_id, "e22-try-atomic-cut-place-mismatch");
    assert_eq!(
        report.sample_path,
        sample_path("e22-try-atomic-cut-place-mismatch.txt")
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.runtime_report.run_report.trace_audit_sink.events,
        vec![
            EventKind::PerformSuccess,
            EventKind::AtomicCut,
            EventKind::PerformFailure,
            EventKind::Rollback,
            EventKind::PerformSuccess,
        ]
    );
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e3_sample() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let report =
        run_current_l2_source_sample("e3-option-admit-chain", bundle.host_plan.unwrap()).unwrap();

    assert_eq!(report.sample_id, "e3-option-admit-chain");
    assert_eq!(report.sample_path, sample_path("e3-option-admit-chain.txt"));
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.runtime_report.run_report.trace_audit_sink.events,
        vec![EventKind::PerformSuccess]
    );
    assert_eq!(
        report
            .runtime_report
            .run_report
            .trace_audit_sink
            .non_admissible_metadata
            .len(),
        1
    );
    assert_eq!(
        report
            .runtime_report
            .run_report
            .trace_audit_sink
            .non_admissible_metadata[0]
            .option_ref,
        "owner_writer"
    );
    assert_eq!(
        report
            .runtime_report
            .run_report
            .trace_audit_sink
            .non_admissible_metadata[0]
            .subreason,
        NonAdmissibleSubreason::AdmitMiss
    );
}

#[test]
fn current_l2_source_sample_runner_accepts_explicit_e4_path() {
    let sample = sample_path("e4-malformed-lineage.txt");
    let report = run_current_l2_source_sample(sample.to_str().unwrap(), FixtureHostPlan::default())
        .unwrap();

    assert_eq!(report.sample_id, "e4-malformed-lineage");
    assert_eq!(report.sample_path, fs::canonicalize(sample).unwrap());
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e16_sample() {
    let report = run_current_l2_source_sample(
        "e16-malformed-missing-chain-head-option",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(report.sample_id, "e16-malformed-missing-chain-head-option");
    assert_eq!(
        report.sample_path,
        sample_path("e16-malformed-missing-chain-head-option.txt")
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    let stage1 = report
        .runtime_report
        .checker_floor
        .stage1_reconnect_clusters
        .unwrap();
    assert!(!stage1.same_lineage_floor);
    assert!(stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e13_sample() {
    let report = run_current_l2_source_sample(
        "e13-malformed-capability-strengthening",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(report.sample_id, "e13-malformed-capability-strengthening");
    assert_eq!(
        report.sample_path,
        sample_path("e13-malformed-capability-strengthening.txt")
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    let stage1 = report
        .runtime_report
        .checker_floor
        .stage1_reconnect_clusters
        .unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(stage1.capability_strengthening_floor);
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e19_sample() {
    let report =
        run_current_l2_source_sample("e19-malformed-target-mismatch", FixtureHostPlan::default())
            .unwrap();

    assert_eq!(report.sample_id, "e19-malformed-target-mismatch");
    assert_eq!(
        report.sample_path,
        sample_path("e19-malformed-target-mismatch.txt")
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e20_sample() {
    let report = run_current_l2_source_sample(
        "e20-malformed-late-capability-strengthening",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(report.sample_id, "e20-malformed-late-capability-strengthening");
    assert_eq!(
        report.sample_path,
        sample_path("e20-malformed-late-capability-strengthening.txt")
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    let stage1 = report
        .runtime_report
        .checker_floor
        .stage1_reconnect_clusters
        .unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(stage1.capability_strengthening_floor);
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e18_sample() {
    let report = run_current_l2_source_sample(
        "e18-malformed-missing-successor-option",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(report.sample_id, "e18-malformed-missing-successor-option");
    assert_eq!(
        report.sample_path,
        sample_path("e18-malformed-missing-successor-option.txt")
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    let stage1 = report
        .runtime_report
        .checker_floor
        .stage1_reconnect_clusters
        .unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e23_sample() {
    let report = run_current_l2_source_sample(
        "e23-malformed-try-fallback-missing-fallback-body",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        report.sample_id,
        "e23-malformed-try-fallback-missing-fallback-body"
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    let stage2 = report
        .runtime_report
        .checker_floor
        .stage2_try_rollback_summary
        .unwrap();
    assert_eq!(stage2.findings.len(), 1);
    assert_eq!(
        stage2.findings[0].finding_kind,
        CurrentL2TryRollbackStructuralFindingKind::MissingFallbackBody
    );
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_sample_runner_rejects_unknown_sample_stem() {
    let error =
        run_current_l2_source_sample("missing-sample", FixtureHostPlan::default()).unwrap_err();
    let message = error.to_string();

    assert!(message.contains("source sample not found"));
    assert!(message.contains("missing-sample"));
}

#[test]
fn current_l2_source_sample_runner_rejects_existing_out_of_scope_explicit_file() {
    let temp_path = std::env::temp_dir().join(format!(
        "mir-runtime-out-of-scope-sample-{}.txt",
        std::process::id()
    ));
    fs::write(&temp_path, "place x {\n}\n").unwrap();

    let error = run_current_l2_source_sample(
        temp_path.to_str().unwrap(),
        FixtureHostPlan::default(),
    )
    .unwrap_err();
    let message = error.to_string();
    fs::remove_file(&temp_path).ok();

    assert!(message.contains("outside the current accepted sample set"));
    assert!(message.contains(temp_path.to_str().unwrap()));
}
