use std::path::PathBuf;

use mir_ast::current_l2::{parse_stage1_program_text, parse_stage2_try_rollback_text};
use mir_runtime::current_l2::{
    CurrentL2ParserBridgeInput, CurrentL2TryRollbackStructuralVerdict,
    run_current_l2_runtime_skeleton,
};
use mir_semantics::{
    ChainEdge, EventKind, FixtureHostPlan, LineageAssertion, Program, ProgramKind, Statement,
    StaticGateVerdict, TerminalOutcome, load_bundle_from_fixture_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn stage1_capability_program(from: &str, to: &str) -> Program {
    Program {
        kind: ProgramKind::Program,
        body: vec![Statement::PlaceBlock {
            place: "root".to_string(),
            body: vec![
                Statement::OptionDecl {
                    name: "source".to_string(),
                    target: "profile_doc".to_string(),
                    capability: from.to_string(),
                    lease: "live".to_string(),
                    admit: None,
                },
                Statement::OptionDecl {
                    name: "dest".to_string(),
                    target: "profile_doc".to_string(),
                    capability: to.to_string(),
                    lease: "live".to_string(),
                    admit: None,
                },
                Statement::ChainDecl {
                    name: "profile_ref".to_string(),
                    head: "source".to_string(),
                    edges: vec![ChainEdge {
                        predecessor: "source".to_string(),
                        successor: "dest".to_string(),
                        lineage_assertion: Some(LineageAssertion {
                            predecessor: "source".to_string(),
                            successor: "dest".to_string(),
                        }),
                    }],
                },
            ],
        }],
    }
}

#[test]
fn current_l2_runtime_skeleton_runs_chain_program_with_stage1_bridge_summary() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let parsed = parse_stage1_program_text(
        r#"
        option owner_writer on profile_doc capability write lease live
        option delegated_writer on profile_doc capability write lease live
        chain profile_ref = owner_writer
        fallback delegated_writer @ lineage(owner_writer -> delegated_writer)
        "#,
    )
    .unwrap();

    let report = run_current_l2_runtime_skeleton(
        bundle.fixture.program,
        bundle.host_plan.unwrap(),
        Some(CurrentL2ParserBridgeInput {
            stage1_program: Some(parsed),
            stage2_try_fallback: None,
        }),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(stage1.same_lineage_floor);
    assert!(!stage1.missing_option_structure_floor);
    assert!(!stage1.capability_strengthening_floor);
    assert!(report.checker_floor.stage2_try_rollback_summary.is_none());
    assert_eq!(report.checker_floor.static_gate.verdict, StaticGateVerdict::Valid);
    assert!(report.run_report.entered_evaluation);
    assert_eq!(
        report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
    assert_eq!(
        report.run_report.trace_audit_sink.events,
        vec![EventKind::PerformSuccess]
    );
}

#[test]
fn current_l2_runtime_skeleton_runs_try_fallback_with_stage2_bridge_summary() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let parsed = parse_stage2_try_rollback_text(
        r#"
        try {
        perform stage_profile_patch on profile_draft
        perform validate_profile_patch on profile_draft
        } fallback {
        perform load_last_snapshot on profile_snapshot
        }
        "#,
    )
    .unwrap();

    let report = run_current_l2_runtime_skeleton(
        bundle.fixture.program,
        bundle.host_plan.unwrap(),
        Some(CurrentL2ParserBridgeInput {
            stage1_program: None,
            stage2_try_fallback: Some(parsed),
        }),
    )
    .unwrap();

    let stage2 = report.checker_floor.stage2_try_rollback_summary.unwrap();
    assert_eq!(stage2.verdict, CurrentL2TryRollbackStructuralVerdict::NoFindings);
    assert!(stage2.findings.is_empty());
    assert!(report.checker_floor.stage1_reconnect_clusters.is_none());
    assert_eq!(report.checker_floor.static_gate.verdict, StaticGateVerdict::Valid);
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
fn current_l2_runtime_skeleton_rejects_mismatched_stage1_bridge_input() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let mismatched = parse_stage1_program_text(
        r#"
        option owner_writer on profile_doc capability read lease live
        option delegated_writer on profile_doc capability write lease live
        chain profile_ref = owner_writer
        fallback delegated_writer @ lineage(owner_writer -> delegated_writer)
        "#,
    )
    .unwrap();

    let error = run_current_l2_runtime_skeleton(
        bundle.fixture.program,
        bundle.host_plan.unwrap(),
        Some(CurrentL2ParserBridgeInput {
            stage1_program: Some(mismatched),
            stage2_try_fallback: None,
        }),
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("stage 1 parser bridge does not match semantic program subset")
    );
}

#[test]
fn current_l2_runtime_skeleton_keeps_stage1_capability_strengthening_frozen_to_read_write_only() {
    let program = stage1_capability_program("read", "append");
    let parsed = parse_stage1_program_text(
        r#"
        option source on profile_doc capability read lease live
        option dest on profile_doc capability append lease live
        chain profile_ref = source
        fallback dest @ lineage(source -> dest)
        "#,
    )
    .unwrap();

    let report = run_current_l2_runtime_skeleton(
        program,
        FixtureHostPlan::default(),
        Some(CurrentL2ParserBridgeInput {
            stage1_program: Some(parsed),
            stage2_try_fallback: None,
        }),
    )
    .unwrap();

    let stage1 = report.checker_floor.stage1_reconnect_clusters.unwrap();
    assert!(!stage1.capability_strengthening_floor);
    assert_eq!(
        report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert_eq!(
        report.run_report.terminal_outcome,
        None
    );
}
