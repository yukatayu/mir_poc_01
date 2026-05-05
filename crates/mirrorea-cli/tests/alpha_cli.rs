use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

const PRODUCT_PACKAGE_JSON: &str = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "product-alpha1-demo",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": ["typed_host_io.add_one"],
  "failures": ["AdapterUnavailable"],
  "capabilities": ["RollDice", "PublishRoll"],
  "witness_requirements": ["game_started_witness"],
  "membership_requirements": ["active_participant"],
  "auth_policy": {
    "policy_id": "demo-auth-policy",
    "required_bindings": ["participant_membership"]
  },
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {
      "contract_id": "demo-world-contract",
      "variance": "invariant",
      "effect_row": ["typed_host_io.add_one"],
      "failure_row": ["AdapterUnavailable"]
    }
  ],
  "observation_policy": {
    "view_role": "observer_safe",
    "labels": ["observer_safe_summary"]
  },
  "redaction_policy": {
    "level": "observer_safe",
    "redacted_fields": ["raw_witness_payload", "raw_auth_evidence"]
  },
  "retention_policy": {
    "scope": "demo_session",
    "retained_artifacts": ["checker_report", "runtime_plan"]
  },
  "message_recovery_policy": {
    "handled_failures": ["timeout", "reject"],
    "recovery": "retry_then_reject"
  },
  "savepoint_policy": {
    "classes": ["R0", "R2"],
    "quiescent_required": true
  },
  "native_policy": {
    "execution_policy": "disabled",
    "provenance_required": true
  },
  "compatibility": {
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }
}"#;

fn cli_bin() -> &'static str {
    env!("CARGO_BIN_EXE_mirrorea-alpha")
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn write_product_package() -> PathBuf {
    let dir = unique_temp_dir("mirrorea-alpha-cli-test");
    fs::create_dir_all(&dir).expect("temp package dir should be created");
    fs::write(dir.join("package.mir.json"), PRODUCT_PACKAGE_JSON)
        .expect("temp package should be written");
    dir
}

fn run_cli(args: &[&str]) -> Output {
    Command::new(cli_bin())
        .args(args)
        .output()
        .expect("mirrorea-alpha should run")
}

fn json_stdout(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout should be JSON: {error}\nstdout={}\nstderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

#[test]
fn check_accepts_product_alpha_package_and_emits_explicit_evidence() {
    let package_dir = write_product_package();
    let output = run_cli(&[
        "check",
        package_dir.to_str().expect("temp dir should be utf-8"),
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(output.status.success(), "check should accept valid package");
    assert_eq!(
        value["surface_kind"],
        "mirrorea_product_alpha1_check_report"
    );
    assert_eq!(value["package_id"], "product-alpha1-demo");
    assert_eq!(value["verdict"], "accepted");
    assert_eq!(value["product_alpha1_ready"], false);
    assert_eq!(value["final_public_api_frozen"], false);
    assert!(
        value["accepted_obligations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["kind"] == "package_schema_version"
                && row["evidence"] == "schema version accepted")
    );
}

#[test]
fn direct_mir_input_returns_explicit_non_goal_diagnostic() {
    let dir = unique_temp_dir("mirrorea-alpha-direct-mir-test");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let mir_path = dir.join("demo.mir");
    fs::write(&mir_path, "world Demo\n").expect("demo .mir file should be written");

    let output = run_cli(&[
        "check",
        mir_path.to_str().expect("temp path should be utf-8"),
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success(), "direct .mir should be rejected");
    assert_eq!(value["status"], "unsupported");
    assert_eq!(value["command"], "check");
    assert_eq!(value["diagnostic_code"], "direct_mir_non_goal");
    assert_eq!(value["final_public_api_frozen"], false);
}

#[test]
fn invalid_cli_invocation_shape_returns_structured_diagnostic() {
    let package_dir = write_product_package();
    let package = package_dir.to_str().expect("temp dir should be utf-8");

    let extra_arg = run_cli(&["check", package, "unexpected", "--format", "json"]);
    let extra_value = json_stdout(&extra_arg);
    assert!(!extra_arg.status.success());
    assert_eq!(extra_value["status"], "error");
    assert_eq!(extra_value["diagnostic_code"], "unexpected_arguments");

    let bad_flag = run_cli(&["check", package, "--bogus-flag", "--format", "json"]);
    let bad_flag_value = json_stdout(&bad_flag);
    assert!(!bad_flag.status.success());
    assert_eq!(bad_flag_value["status"], "error");
    assert_eq!(bad_flag_value["diagnostic_code"], "unexpected_arguments");

    let bad_format = run_cli(&["--format", "yaml", "check", package]);
    let bad_format_value = json_stdout(&bad_format);
    assert!(!bad_format.status.success());
    assert_eq!(bad_format_value["status"], "error");
    assert_eq!(bad_format_value["diagnostic_code"], "unsupported_format");
}

#[test]
fn unimplemented_commands_reject_direct_mir_with_non_goal_diagnostic() {
    let dir = unique_temp_dir("mirrorea-alpha-run-local-direct-mir-test");
    fs::create_dir_all(&dir).expect("temp dir should be created");
    let mir_path = dir.join("demo.mir");
    fs::write(&mir_path, "world Demo\n").expect("demo .mir file should be written");

    let output = run_cli(&[
        "run-local",
        mir_path.to_str().expect("temp path should be utf-8"),
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success(), "direct .mir should be rejected");
    assert_eq!(value["status"], "unsupported");
    assert_eq!(value["command"], "run-local");
    assert_eq!(value["diagnostic_code"], "direct_mir_non_goal");
    assert_eq!(value["final_public_api_frozen"], false);
}

#[test]
fn unimplemented_alpha_command_family_returns_structured_unsupported_diagnostics() {
    let package_dir = write_product_package();
    let package = package_dir.to_str().expect("temp dir should be utf-8");
    let out_dir = unique_temp_dir("mirrorea-alpha-out");
    let out = out_dir.to_str().expect("temp out should be utf-8");

    let command_args: &[(&str, &[&str])] = &[
        ("run-local", &[package]),
        ("session", &[package]),
        ("attach", &["session-alpha1-demo", package]),
        ("transport", &[package, "--mode", "local"]),
        ("save", &["session-alpha1-demo"]),
        ("load", &["savepoint-alpha1-demo"]),
        ("quiescent-save", &["session-alpha1-demo"]),
        ("export-devtools", &["session-alpha1-demo", "--out", out]),
        ("view", &[out]),
        ("build-native-bundle", &[package, "--out", out]),
        ("demo", &["--out", out]),
    ];

    for (command, args) in command_args {
        let mut argv = vec![*command];
        argv.extend_from_slice(args);
        argv.extend_from_slice(&["--format", "json"]);
        let output = run_cli(&argv);
        let value = json_stdout(&output);

        assert!(
            !output.status.success(),
            "{command} should be unsupported in P-A1-26"
        );
        assert_eq!(value["status"], "unsupported");
        assert_eq!(value["command"], *command);
        assert_eq!(value["implemented"], false);
        assert_eq!(value["product_alpha1_ready"], false);
        assert_eq!(value["final_public_api_frozen"], false);
    }
}
