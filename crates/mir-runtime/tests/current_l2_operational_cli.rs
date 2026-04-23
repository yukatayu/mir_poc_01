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

#[test]
fn operational_cli_pretty_runs_runtime_sample() {
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
    assert!(output.contains("static_verdict: valid"));
    assert!(output.contains("entered_evaluation: true"));
}

#[test]
fn operational_cli_json_reports_clean_authorized_sample() {
    let output = run_current_l2_operational_cli([
        "check-source-sample",
        "samples/clean-near-end/typing/01_authorized_declassification.mir",
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "01_authorized_declassification");
    assert_eq!(value["static_verdict"], "valid");
    assert_eq!(value["terminal_outcome"], "success");
    assert_eq!(
        value["constraints_solved"][0],
        "authority(Alice) >= FingerprintAuthority.Releaser"
    );
}

#[test]
fn operational_cli_json_reports_clean_rejection() {
    let output = run_current_l2_operational_cli([
        "check-source-sample",
        "samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir",
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "02_unauthorized_declassification_rejected");
    assert_eq!(value["static_verdict"], "malformed");
    assert_eq!(value["reason_family"], "authority_preorder_constraint_failed");
    assert_eq!(value["entered_evaluation"], false);
}

#[test]
fn operational_cli_rejects_missing_host_plan_for_current_l2_sample() {
    let error = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--format",
        "json",
    ])
    .unwrap_err();

    assert_eq!(error.exit_code(), 2);
    assert!(
        error
            .to_string()
            .contains("missing --host-plan <path> and no adjacent host plan was found")
    );
}
