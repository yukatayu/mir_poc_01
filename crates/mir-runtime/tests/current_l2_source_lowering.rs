use std::{fs, path::PathBuf};

use mir_runtime::current_l2::{
    CurrentL2TryRollbackStructuralVerdict, lower_current_l2_fixed_source_text,
    run_current_l2_runtime_skeleton,
};
use mir_semantics::{
    EventKind, FixtureHostPlan, StaticGateVerdict, TerminalOutcome, load_bundle_from_fixture_path,
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
fn current_l2_source_lowering_matches_e4_fixture_and_stage1_bridge() {
    let source = fs::read_to_string(sample_path("e4-malformed-lineage.txt")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_some());
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(report.checker_floor.static_gate.verdict, StaticGateVerdict::Malformed);
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e2_fixture_and_stage2_bridge() {
    let source = fs::read_to_string(sample_path("e2-try-fallback.txt")).unwrap();
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        bundle.host_plan.unwrap(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    let stage2 = report.checker_floor.stage2_try_rollback_summary.unwrap();
    assert_eq!(stage2.verdict, CurrentL2TryRollbackStructuralVerdict::NoFindings);
    assert!(stage2.findings.is_empty());
    assert_eq!(report.checker_floor.static_gate.verdict, StaticGateVerdict::Valid);
    assert!(report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, Some(TerminalOutcome::Success));
    assert_eq!(
        report.run_report.trace_audit_sink.events,
        vec![
            EventKind::PerformSuccess,
            EventKind::PerformFailure,
            EventKind::Rollback,
            EventKind::PerformSuccess,
        ]
    );
}

#[test]
fn current_l2_source_lowering_matches_e23_fixture_and_empty_fallback_bridge() {
    let source =
        fs::read_to_string(sample_path("e23-malformed-try-fallback-missing-fallback-body.txt"))
            .unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    let stage2 = report.checker_floor.stage2_try_rollback_summary.unwrap();
    assert_eq!(
        stage2.verdict,
        CurrentL2TryRollbackStructuralVerdict::FindingsPresent
    );
    assert_eq!(stage2.findings.len(), 1);
    assert_eq!(report.checker_floor.static_gate.verdict, StaticGateVerdict::Malformed);
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}
