use std::path::PathBuf;

use mir_runtime::current_l2_cli::run_current_l2_operational_cli;
use serde_json::Value;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn host_plan_path(name: &str) -> PathBuf {
    fixture_path(name).with_extension("host-plan.json")
}

fn prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

#[test]
fn operational_cli_pretty_runs_runtime_sample_with_explicit_host_plan_path() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("shell: mir-current-l2"));
    assert!(output.contains("command: run-source-sample"));
    assert!(output.contains("sample: e2-try-fallback"));
    assert!(output.contains("host_plan_path:"));
    assert!(output.contains("static_gate: valid"));
    assert!(output.contains("terminal_outcome: success"));
    assert!(output.contains("- perform-success"));
    assert!(output.contains("- perform-failure"));
    assert!(output.contains("- rollback"));
}

#[test]
fn operational_cli_json_reports_static_stop_without_entering_evaluation() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        "e13-malformed-capability-strengthening",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["shell"], "mir-current-l2");
    assert_eq!(value["command"], "run-source-sample");
    assert_eq!(value["sample"], "e13-malformed-capability-strengthening");
    assert_eq!(
        value["checker_floor"]["static_gate"]["verdict"],
        "malformed"
    );
    assert_eq!(
        value["checker_floor"]["stage1_reconnect_clusters"]["capability_strengthening_floor"],
        true
    );
    assert_eq!(value["runtime"]["entered_evaluation"], false);
    assert_eq!(value["runtime"]["terminal_outcome"], Value::Null);
}

#[test]
fn operational_cli_rejects_missing_host_plan_flag() {
    let error = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--format",
        "pretty",
    ])
    .unwrap_err();

    assert_eq!(error.exit_code(), 2);
    assert!(
        error
            .to_string()
            .contains("missing --host-plan <path> and no adjacent host plan was found")
    );
    assert!(error.to_string().contains(
        "mir-current-l2 run-source-sample <sample-or-path> [--host-plan <path>] --format pretty|json"
    ));
}

#[test]
fn operational_cli_rejects_unknown_format() {
    let error = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "yaml",
    ])
    .unwrap_err();

    assert_eq!(error.exit_code(), 2);
    assert!(error.to_string().contains("unsupported format `yaml`"));
}

#[test]
fn operational_cli_uses_adjacent_host_plan_for_prototype_sample_when_omitted() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        prototype_sample_path("p01-dice-publication-handoff.txt")
            .to_str()
            .unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("sample: p01-dice-publication-handoff"));
    assert!(output.contains("host_plan_path:"));
    assert!(output.contains("terminal_outcome: success"));
    assert!(output.contains("debug_outputs:"));
    assert!(output.contains("dice_debug_text_output:"));
    assert!(output.contains("roll_dice: player_a -> visible"));
}
