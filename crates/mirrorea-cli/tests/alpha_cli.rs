use std::{
    fs,
    path::{Path, PathBuf},
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
  "runtime_input": {
    "entry_place": "Place[ProductDemoRoom]",
    "host_io": {
      "adapter_kind": "AddOne",
      "effect_ref": "typed_host_io.add_one",
      "request_payload": {"kind": "int", "value": 41},
      "expected_response": {"kind": "int", "value": 42}
    }
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

fn write_product_package_with_id(package_id: &str) -> PathBuf {
    let dir = unique_temp_dir("mirrorea-alpha-cli-id-test");
    fs::create_dir_all(&dir).expect("temp package dir should be created");
    fs::write(
        dir.join("package.mir.json"),
        PRODUCT_PACKAGE_JSON.replace("product-alpha1-demo", package_id),
    )
    .expect("temp package should be written");
    dir
}

fn run_cli(args: &[&str]) -> Output {
    run_cli_with_session_dir(args, None)
}

fn run_cli_with_session_dir(args: &[&str], session_dir: Option<&Path>) -> Output {
    let mut command = Command::new(cli_bin());
    command.args(args);
    if let Some(session_dir) = session_dir {
        command.env("MIRROREA_ALPHA_SESSION_DIR", session_dir);
    }
    command.output().expect("mirrorea-alpha should run")
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

#[test]
fn save_load_and_quiescent_save_mutate_same_session_file() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mirrorea-cli");
    let session_dir = unique_temp_dir("mirrorea-alpha-cli-save-load");
    fs::create_dir_all(&session_dir).expect("session dir should be created");
    let demo = repo_root.join("samples/product-alpha1/demo");
    let debug_layer = demo.join("packages/debug-layer");

    let run = run_cli_with_session_dir(
        &[
            "run-local",
            demo.to_str().expect("demo path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let run_value = json_stdout(&run);
    assert!(run.status.success());
    let session_id = run_value["session"]["session_id"].as_str().unwrap();

    let attach = run_cli_with_session_dir(
        &[
            "attach",
            session_id,
            debug_layer.to_str().expect("debug path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    assert!(attach.status.success());

    let save = run_cli_with_session_dir(
        &[
            "save",
            session_id,
            "--savepoint",
            "savepoint#r0-cli",
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let save_value = json_stdout(&save);
    assert!(save.status.success());
    assert_eq!(save_value["surface_kind"], "product_alpha1_save_report");
    assert_eq!(save_value["savepoint_class"], "R0_Local");
    assert_eq!(save_value["session"]["phase"], "saved");

    let quiescent = run_cli_with_session_dir(
        &[
            "quiescent-save",
            session_id,
            "--savepoint",
            "savepoint#r2-cli",
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let quiescent_value = json_stdout(&quiescent);
    assert!(quiescent.status.success());
    assert_eq!(
        quiescent_value["surface_kind"],
        "product_alpha1_quiescent_save_report"
    );
    assert_eq!(quiescent_value["savepoint_class"], "R2_Quiescent");
    assert_eq!(quiescent_value["no_inflight"], true);
    assert_eq!(quiescent_value["all_places_sealed"], true);
    assert_eq!(quiescent_value["no_post_cut_send"], true);

    let load = run_cli_with_session_dir(
        &[
            "load",
            "savepoint#r0-cli",
            "--session",
            session_id,
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let load_value = json_stdout(&load);
    assert!(load.status.success());
    assert_eq!(load_value["surface_kind"], "product_alpha1_load_report");
    assert_eq!(load_value["terminal_outcome"], "loaded");
    assert_eq!(load_value["session"]["phase"], "loaded");
    assert_eq!(
        load_value["session"]["active_layers"][0],
        "product-alpha1-debug-layer"
    );
    assert!(
        !String::from_utf8_lossy(&save.stdout).contains("saved_auth_state"),
        "save stdout should not expose persisted admin/auth savepoint internals"
    );
}

#[test]
fn repeated_default_save_and_quiescent_save_keep_session_event_ids_unique() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mirrorea-cli");
    let session_dir = unique_temp_dir("mirrorea-alpha-cli-repeat-save-load");
    fs::create_dir_all(&session_dir).expect("session dir should be created");
    let demo = repo_root.join("samples/product-alpha1/demo");

    let run = run_cli_with_session_dir(
        &[
            "run-local",
            demo.to_str().expect("demo path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let run_value = json_stdout(&run);
    assert!(run.status.success());
    let session_id = run_value["session"]["session_id"].as_str().unwrap();

    for command in ["save", "save", "quiescent-save", "quiescent-save"] {
        let output = run_cli_with_session_dir(
            &[command, session_id, "--format", "json"],
            Some(&session_dir),
        );
        assert!(
            output.status.success(),
            "{command} should succeed without duplicate event ids"
        );
    }

    let session = run_cli_with_session_dir(
        &["session", session_id, "--format", "json"],
        Some(&session_dir),
    );
    let session_value = json_stdout(&session);
    assert!(session.status.success());
    let mut event_ids = std::collections::BTreeSet::new();
    for node in session_value["session"]["event_dag"]["nodes"]
        .as_array()
        .expect("event nodes should be present")
    {
        let event_id = node["event_id"]
            .as_str()
            .expect("event id should be string");
        assert!(
            event_ids.insert(event_id.to_string()),
            "duplicate event id {event_id}"
        );
    }

    let load = run_cli_with_session_dir(
        &[
            "load",
            "savepoint#r0-latest",
            "--session",
            session_id,
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let load_value = json_stdout(&load);
    assert!(load.status.success());
    assert_eq!(load_value["surface_kind"], "product_alpha1_load_report");
    assert_eq!(load_value["terminal_outcome"], "loaded");
    assert_eq!(load_value["session"]["phase"], "loaded");
}

#[test]
fn transport_export_devtools_and_view_use_same_product_session() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mirrorea-cli");
    let session_dir = unique_temp_dir("mirrorea-alpha-cli-transport-viewer");
    fs::create_dir_all(&session_dir).expect("session dir should be created");
    let out_dir = unique_temp_dir("mirrorea-alpha-cli-devtools-out");
    let demo = repo_root.join("samples/product-alpha1/demo");

    let run = run_cli_with_session_dir(
        &[
            "run-local",
            demo.to_str().expect("demo path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let run_value = json_stdout(&run);
    assert!(run.status.success());
    let session_id = run_value["session"]["session_id"].as_str().unwrap();

    let transport = run_cli_with_session_dir(
        &[
            "transport",
            session_id,
            "--mode",
            "local",
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let transport_value = json_stdout(&transport);
    assert!(transport.status.success());
    assert_eq!(
        transport_value["surface_kind"],
        "product_alpha1_transport_report"
    );
    assert_eq!(transport_value["mode"], "local");
    assert_eq!(transport_value["wire_roundtrip_executed"], true);
    assert_eq!(transport_value["session"]["phase"], "transported");

    let export = run_cli_with_session_dir(
        &[
            "export-devtools",
            session_id,
            "--out",
            out_dir.to_str().expect("out dir should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let export_value = json_stdout(&export);
    assert!(export.status.success());
    assert_eq!(
        export_value["surface_kind"],
        "product_alpha1_devtools_export_report"
    );
    assert!(out_dir.join("bundle.json").exists());
    assert!(out_dir.join("index.html").exists());
    let bundle_text = fs::read_to_string(out_dir.join("bundle.json")).unwrap();
    assert!(!bundle_text.contains("witness_refs"));
    assert!(!bundle_text.contains("granted_capabilities"));
    assert!(!bundle_text.contains("ObserveDebugSummary"));
    assert!(!bundle_text.contains("AttachDebugLayer"));
    assert!(
        export_value["panel_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|panel| panel == "message_route_graph")
    );

    let view = run_cli_with_session_dir(
        &[
            "view",
            out_dir.to_str().expect("out dir should be utf-8"),
            "--check",
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let view_value = json_stdout(&view);
    assert!(view.status.success());
    assert_eq!(view_value["surface_kind"], "product_alpha1_view_report");
    assert_eq!(view_value["viewer_openable"], true);
    assert_eq!(view_value["bundle_valid"], true);
    assert_eq!(view_value["html_contains_required_panels"], true);
    assert_eq!(view_value["final_public_viewer_frozen"], false);
}

#[test]
fn view_check_rejects_malformed_or_placeholder_bundle() {
    let out_dir = unique_temp_dir("mirrorea-alpha-cli-bad-viewer-out");
    fs::create_dir_all(&out_dir).expect("out dir should be created");
    fs::write(out_dir.join("bundle.json"), "{}\n").expect("bad bundle should be written");
    fs::write(
        out_dir.join("index.html"),
        "<section id=\"product_overview\"></section>",
    )
    .expect("html should be written");

    let view = run_cli(&[
        "view",
        out_dir.to_str().expect("out dir should be utf-8"),
        "--check",
        "--format",
        "json",
    ]);
    let value = json_stdout(&view);
    assert!(!view.status.success());
    assert_eq!(value["surface_kind"], "product_alpha1_view_report");
    assert_eq!(value["diagnostic_code"], "invalid_viewer_bundle");
    assert_eq!(value["bundle_valid"], false);
}

#[test]
fn view_check_rejects_tampered_observer_unsafe_bundle_keys() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mirrorea-cli");
    let session_dir = unique_temp_dir("mirrorea-alpha-cli-tampered-viewer-session");
    fs::create_dir_all(&session_dir).expect("session dir should be created");
    let out_dir = unique_temp_dir("mirrorea-alpha-cli-tampered-viewer-out");
    let demo = repo_root.join("samples/product-alpha1/demo");

    let run = run_cli_with_session_dir(
        &[
            "run-local",
            demo.to_str().expect("demo path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let run_value = json_stdout(&run);
    assert!(run.status.success());
    let session_id = run_value["session"]["session_id"].as_str().unwrap();
    let export = run_cli_with_session_dir(
        &[
            "export-devtools",
            session_id,
            "--out",
            out_dir.to_str().expect("out dir should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    assert!(export.status.success());

    let bundle_path = out_dir.join("bundle.json");
    let mut bundle: Value =
        serde_json::from_str(&fs::read_to_string(&bundle_path).unwrap()).unwrap();
    bundle["witness_refs"] = serde_json::json!(["private_witness"]);
    bundle["panels"]["auth_capability_decision"]["capability_decisions"][0]["granted_capabilities"] =
        serde_json::json!(["private.capability.admin"]);
    fs::write(&bundle_path, serde_json::to_string_pretty(&bundle).unwrap()).unwrap();

    let view = run_cli(&[
        "view",
        out_dir.to_str().expect("out dir should be utf-8"),
        "--check",
        "--format",
        "json",
    ]);
    let value = json_stdout(&view);
    assert!(!view.status.success());
    assert_eq!(value["diagnostic_code"], "invalid_viewer_bundle");
    assert_eq!(value["bundle_valid"], false);
    assert!(
        value["bundle_error"]
            .as_str()
            .unwrap()
            .contains("forbidden observer-safe")
    );
}

#[test]
fn product_transport_endpoint_helpers_require_fixture_gate() {
    let output = run_cli(&[
        "__product-transport-world-server",
        "/tmp/nonexistent-product-alpha1-session.json",
        "--bind",
        "127.0.0.1:0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);
    assert!(!output.status.success());
    assert_eq!(value["status"], "unsupported");
    assert_eq!(value["diagnostic_code"], "transport_helper_not_authorized");
}

#[test]
fn run_local_writes_product_session_and_attach_mutates_same_session_file() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mirrorea-cli");
    let session_dir = unique_temp_dir("mirrorea-alpha-cli-session-store");
    fs::create_dir_all(&session_dir).expect("session dir should be created");
    let demo = repo_root.join("samples/product-alpha1/demo");
    let debug_layer = demo.join("packages/debug-layer");

    let run = run_cli_with_session_dir(
        &[
            "run-local",
            demo.to_str().expect("demo path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let run_value = json_stdout(&run);

    assert!(
        run.status.success(),
        "run-local should start a product session"
    );
    assert_eq!(run_value["surface_kind"], "product_alpha1_run_local_report");
    assert_eq!(run_value["session"]["phase"], "run_local");
    assert_eq!(run_value["session"]["product_alpha1_ready"], false);
    let session_id = run_value["session"]["session_id"]
        .as_str()
        .expect("session_id should be present");
    let session_path = run_value["session_path"]
        .as_str()
        .expect("session_path should be present");
    assert!(Path::new(session_path).exists());

    let attach = run_cli_with_session_dir(
        &[
            "attach",
            session_id,
            debug_layer
                .to_str()
                .expect("debug layer path should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let attach_value = json_stdout(&attach);

    assert!(
        attach.status.success(),
        "attach should mutate the same session"
    );
    assert_eq!(attach_value["surface_kind"], "product_alpha1_attach_report");
    assert_eq!(attach_value["session_id"], session_id);
    assert_eq!(attach_value["session"]["phase"], "attached");
    assert_eq!(
        attach_value["session"]["active_layers"][0],
        "product-alpha1-debug-layer"
    );
    assert_eq!(
        attach_value["auth_decision"]["overlay_transparency_claimed"],
        false
    );
    assert_eq!(attach_value["session"]["product_alpha1_ready"], false);
}

#[test]
fn session_store_paths_do_not_alias_distinct_session_ids() {
    let session_dir = unique_temp_dir("mirrorea-alpha-cli-session-collision");
    fs::create_dir_all(&session_dir).expect("session dir should be created");
    let slash_id = write_product_package_with_id("product/a");
    let question_id = write_product_package_with_id("product?a");

    let slash = run_cli_with_session_dir(
        &[
            "run-local",
            slash_id.to_str().expect("temp dir should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let slash_value = json_stdout(&slash);
    let question = run_cli_with_session_dir(
        &[
            "run-local",
            question_id.to_str().expect("temp dir should be utf-8"),
            "--format",
            "json",
        ],
        Some(&session_dir),
    );
    let question_value = json_stdout(&question);

    assert!(slash.status.success());
    assert!(question.status.success());
    let slash_path = slash_value["session_path"].as_str().unwrap();
    let question_path = question_value["session_path"].as_str().unwrap();
    assert_ne!(slash_path, question_path);
    assert!(Path::new(slash_path).exists());
    assert!(Path::new(question_path).exists());
}
