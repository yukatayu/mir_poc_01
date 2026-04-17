use std::{fs, path::PathBuf};

use mir_runtime::current_l2::{
    CurrentL2TryRollbackStructuralVerdict, lower_current_l2_fixed_source_text,
    run_current_l2_runtime_skeleton,
};
use mir_semantics::{
    EventKind, FixtureHostPlan, NonAdmissibleSubreason, ProgramKind, StaticGateVerdict,
    TerminalOutcome, load_bundle_from_fixture_path,
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
fn current_l2_source_lowering_ignores_leading_hash_comment_lines() {
    let source = r#"
# この sample は hash comment を先頭に置いても current L2 lowerer が落ちないことを確認する。
place root {
  place session {
    place authority_cell {
      perform update_authority on profile_authority
        require write
        ensure owner_is(session_user)

      atomic_cut
    }
  }
}
"#;

    let lowered = lower_current_l2_fixed_source_text(source).unwrap();
    assert!(matches!(lowered.program.kind, ProgramKind::Program));
    assert_eq!(lowered.program.body.len(), 1);
}

#[test]
fn current_l2_source_lowering_rejects_non_leading_hash_comment_lines() {
    let source = r#"
# この sample は先頭 comment は許す。
place root {
  # 途中 comment は current convenience cut に入れない。
  place session {
    place authority_cell {
      perform update_authority on profile_authority
        require write
        ensure owner_is(session_user)
    }
  }
}
"#;

    assert!(lower_current_l2_fixed_source_text(source).is_err());
}

#[test]
fn current_l2_source_lowering_matches_e1_fixture_and_atomic_cut_runtime() {
    let source = fs::read_to_string(sample_path("e1-place-atomic-cut.txt")).unwrap();
    let bundle = load_bundle_from_fixture_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        bundle.host_plan.unwrap(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.run_report.entered_evaluation);
    assert_eq!(
        report.run_report.terminal_outcome,
        Some(TerminalOutcome::ExplicitFailure)
    );
    assert_eq!(
        report.run_report.trace_audit_sink.events,
        vec![
            EventKind::PerformSuccess,
            EventKind::AtomicCut,
            EventKind::PerformFailure,
        ]
    );
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
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e16_fixture_and_missing_head_static_stop() {
    let source =
        fs::read_to_string(sample_path("e16-malformed-missing-chain-head-option.txt")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(!stage1.same_lineage_floor);
    assert!(stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e14_fixture_and_duplicate_option_static_stop() {
    let source = fs::read_to_string(sample_path(
        "e14-malformed-duplicate-option-declaration.txt",
    ))
    .unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(!stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e15_fixture_and_duplicate_chain_static_stop() {
    let source =
        fs::read_to_string(sample_path("e15-malformed-duplicate-chain-declaration.txt")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(!stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e13_fixture_and_capability_static_stop() {
    let source =
        fs::read_to_string(sample_path("e13-malformed-capability-strengthening.txt")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e19_fixture_and_stage1_bridge() {
    let source = fs::read_to_string(sample_path("e19-malformed-target-mismatch.txt")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_some());
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e20_fixture_and_late_capability_static_stop() {
    let source = fs::read_to_string(sample_path(
        "e20-malformed-late-capability-strengthening.txt",
    ))
    .unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}

#[test]
fn current_l2_source_lowering_matches_e18_fixture_and_missing_successor_static_stop() {
    let source =
        fs::read_to_string(sample_path("e18-malformed-missing-successor-option.txt")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        FixtureHostPlan::default(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
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
    assert_eq!(
        stage2.verdict,
        CurrentL2TryRollbackStructuralVerdict::NoFindings
    );
    assert!(stage2.findings.is_empty());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.run_report.entered_evaluation);
    assert_eq!(
        report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
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
fn current_l2_source_lowering_matches_e21_fixture_and_try_atomic_cut_frontier() {
    let source = fs::read_to_string(sample_path("e21-try-atomic-cut-frontier.txt")).unwrap();
    let bundle =
        load_bundle_from_fixture_path(fixture_path("e21-try-atomic-cut-frontier.json")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        bundle.host_plan.unwrap(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    let stage2 = report.checker_floor.stage2_try_rollback_summary.unwrap();
    assert_eq!(
        stage2.verdict,
        CurrentL2TryRollbackStructuralVerdict::NoFindings
    );
    assert!(stage2.findings.is_empty());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.run_report.entered_evaluation);
    assert_eq!(
        report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.run_report.trace_audit_sink.events,
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
fn current_l2_source_lowering_matches_e22_fixture_and_nested_place_atomic_cut_mismatch() {
    let source = fs::read_to_string(sample_path("e22-try-atomic-cut-place-mismatch.txt")).unwrap();
    let bundle =
        load_bundle_from_fixture_path(fixture_path("e22-try-atomic-cut-place-mismatch.json"))
            .unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        bundle.host_plan.unwrap(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    let stage2 = report.checker_floor.stage2_try_rollback_summary.unwrap();
    assert_eq!(
        stage2.verdict,
        CurrentL2TryRollbackStructuralVerdict::NoFindings
    );
    assert!(stage2.findings.is_empty());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.run_report.entered_evaluation);
    assert_eq!(
        report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.run_report.trace_audit_sink.events,
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
fn current_l2_source_lowering_matches_e3_fixture_and_admit_chain_runtime() {
    let source = fs::read_to_string(sample_path("e3-option-admit-chain.txt")).unwrap();
    let bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let lowered = lower_current_l2_fixed_source_text(&source).unwrap();
    let report = run_current_l2_runtime_skeleton(
        lowered.program,
        bundle.host_plan.unwrap(),
        Some(lowered.parser_bridge_input),
    )
    .unwrap();

    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.run_report.entered_evaluation);
    assert_eq!(
        report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.run_report.trace_audit_sink.events,
        vec![EventKind::PerformSuccess]
    );
    assert_eq!(
        report
            .run_report
            .trace_audit_sink
            .non_admissible_metadata
            .len(),
        1
    );
    assert_eq!(
        report.run_report.trace_audit_sink.non_admissible_metadata[0].option_ref,
        "owner_writer"
    );
    assert_eq!(
        report.run_report.trace_audit_sink.non_admissible_metadata[0].subreason,
        NonAdmissibleSubreason::AdmitMiss
    );
}

#[test]
fn current_l2_source_lowering_matches_e23_fixture_and_empty_fallback_bridge() {
    let source = fs::read_to_string(sample_path(
        "e23-malformed-try-fallback-missing-fallback-body.txt",
    ))
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
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!report.run_report.entered_evaluation);
    assert_eq!(report.run_report.terminal_outcome, None);
}
