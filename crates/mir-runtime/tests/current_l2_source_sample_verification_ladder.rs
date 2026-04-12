use std::path::PathBuf;

use mir_runtime::current_l2::run_current_l2_source_sample;
use mir_semantics::{
    FixtureHostPlan, StaticGateVerdict, TerminalOutcome, load_bundle_from_fixture_path,
    load_fixture_from_path, run_bundle, static_gate_detailed,
};

#[path = "../../mir-semantics/examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../../mir-semantics/examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../../mir-semantics/examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;
use current_l2_formal_hook_support::{
    build_formal_hook_from_detached_bundle_artifact, build_formal_hook_from_static_gate_artifact,
};
use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn verification_ladder_marks_e1_as_runtime_and_formal_hook_reached() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e1-place-atomic-cut.json")).unwrap();
    let source_report =
        run_current_l2_source_sample("e1-place-atomic-cut", bundle.host_plan.clone().unwrap())
            .unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        source_report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::ExplicitFailure)
    );

    let detached_bundle = build_detached_bundle_artifact(&bundle, &run_bundle(&bundle).unwrap());
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&detached_bundle).unwrap();

    assert_eq!(formal_hook.subject_kind, "runtime_try_cut_cluster");
    assert_eq!(formal_hook.subject_ref, "e1_place_atomic_cut");
}

#[test]
fn verification_ladder_marks_e2_as_runtime_and_formal_hook_reached() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let source_report =
        run_current_l2_source_sample("e2-try-fallback", bundle.host_plan.clone().unwrap()).unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        source_report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );

    let detached_bundle = build_detached_bundle_artifact(&bundle, &run_bundle(&bundle).unwrap());
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&detached_bundle).unwrap();

    assert_eq!(formal_hook.subject_kind, "runtime_try_cut_cluster");
    assert_eq!(formal_hook.subject_ref, "e2_try_fallback");
}

#[test]
fn verification_ladder_marks_e21_as_runtime_and_formal_hook_reached() {
    let bundle =
        load_bundle_from_fixture_path(fixture_path("e21-try-atomic-cut-frontier.json")).unwrap();
    let source_report = run_current_l2_source_sample(
        "e21-try-atomic-cut-frontier",
        bundle.host_plan.clone().unwrap(),
    )
    .unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        source_report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );

    let detached_bundle = build_detached_bundle_artifact(&bundle, &run_bundle(&bundle).unwrap());
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&detached_bundle).unwrap();

    assert_eq!(formal_hook.subject_kind, "runtime_try_cut_cluster");
    assert_eq!(formal_hook.subject_ref, "e21_try_atomic_cut_frontier");
}

#[test]
fn verification_ladder_marks_e22_as_runtime_and_formal_hook_reached() {
    let bundle = load_bundle_from_fixture_path(fixture_path(
        "e22-try-atomic-cut-place-mismatch.json",
    ))
    .unwrap();
    let source_report = run_current_l2_source_sample(
        "e22-try-atomic-cut-place-mismatch",
        bundle.host_plan.clone().unwrap(),
    )
    .unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        source_report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );

    let detached_bundle = build_detached_bundle_artifact(&bundle, &run_bundle(&bundle).unwrap());
    let formal_hook = build_formal_hook_from_detached_bundle_artifact(&detached_bundle).unwrap();

    assert_eq!(formal_hook.subject_kind, "runtime_try_cut_cluster");
    assert_eq!(formal_hook.subject_ref, "e22_try_atomic_cut_place_mismatch");
}

#[test]
fn verification_ladder_marks_e3_as_runtime_reached_but_formal_hook_guarded() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let source_report =
        run_current_l2_source_sample("e3-option-admit-chain", bundle.host_plan.clone().unwrap())
            .unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        source_report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );

    let detached_bundle = build_detached_bundle_artifact(&bundle, &run_bundle(&bundle).unwrap());
    let error =
        build_formal_hook_from_detached_bundle_artifact(&detached_bundle).unwrap_err();

    assert!(error.contains("runtime_try_cut_cluster"));
}

#[test]
fn verification_ladder_marks_e4_as_static_stop_and_static_formal_hook_reached() {
    let source_report =
        run_current_l2_source_sample("e4-malformed-lineage", FixtureHostPlan::default()).unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(source_report.runtime_report.run_report.terminal_outcome, None);

    let path = fixture_path("e4-malformed-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let detached_static = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&detached_static).unwrap();

    assert_eq!(formal_hook.subject_kind, "fixture_static_cluster");
    assert_eq!(formal_hook.subject_ref, "e4_malformed_lineage");
}

#[test]
fn verification_ladder_marks_e19_as_static_stop_and_static_formal_hook_reached() {
    let source_report = run_current_l2_source_sample(
        "e19-malformed-target-mismatch",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(source_report.runtime_report.run_report.terminal_outcome, None);

    let path = fixture_path("e19-malformed-target-mismatch.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let detached_static = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&detached_static).unwrap();

    assert_eq!(formal_hook.subject_kind, "fixture_static_cluster");
    assert_eq!(formal_hook.subject_ref, "e19_malformed_target_mismatch");
}

#[test]
fn verification_ladder_marks_e23_as_static_stop_and_static_formal_hook_reached() {
    let source_report = run_current_l2_source_sample(
        "e23-malformed-try-fallback-missing-fallback-body",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        source_report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(!source_report.runtime_report.run_report.entered_evaluation);
    assert_eq!(source_report.runtime_report.run_report.terminal_outcome, None);

    let path = fixture_path("e23-malformed-try-fallback-missing-fallback-body.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let detached_static = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&detached_static).unwrap();

    assert_eq!(formal_hook.subject_kind, "fixture_static_cluster");
    assert_eq!(
        formal_hook.subject_ref,
        "e23_malformed_try_fallback_missing_fallback_body"
    );
}
