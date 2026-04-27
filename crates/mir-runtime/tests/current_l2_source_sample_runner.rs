use std::path::PathBuf;

use mir_runtime::current_l2::run_current_l2_source_sample;
use mir_semantics::{
    FixtureHostPlan, StaticGateVerdict, TerminalOutcome, load_bundle_from_fixture_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn current_l2_source_sample_runner_accepts_named_e2_sample() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let report =
        run_current_l2_source_sample("e2-try-fallback", bundle.host_plan.unwrap()).unwrap();

    assert_eq!(report.sample_id, "e2-try-fallback");
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Valid
    );
    assert!(report.runtime_report.run_report.entered_evaluation);
    assert_eq!(
        report.runtime_report.run_report.terminal_outcome,
        Some(TerminalOutcome::Success)
    );
}

#[test]
fn current_l2_source_sample_runner_rejects_path_outside_fixed_subset() {
    let temp = std::env::temp_dir().join("outside-current-l2-sample.txt");
    std::fs::write(&temp, "place root {}").unwrap();

    let error = run_current_l2_source_sample(temp.to_str().unwrap(), FixtureHostPlan::default())
        .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("source sample path is outside the current accepted sample set")
    );

    std::fs::remove_file(temp).unwrap();
}
