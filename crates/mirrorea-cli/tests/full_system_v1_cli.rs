use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

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

fn write_file(root: &Path, relative_path: &str, content: &str) -> PathBuf {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("path should have parent"))
        .expect("parent should be created");
    fs::write(&path, content).expect("file should be written");
    path
}

fn server_client_sample_path(relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/full-system-v1/server-client/role-split-positive")
        .join(relative_path)
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
fn project_full_v1_emits_projection_report() {
    let root = unique_temp_dir("mirrorea-full-system-v1-cli-projection");
    fs::create_dir_all(&root).expect("temp root should be created");
    let source = write_file(
        &root,
        "main/src/host-boundary-positive.mir",
        r#"module FullSystemV1.HostBoundaryPositive

capability HostRead
capability HostWrite

effect read_int {
  requires HostRead
  output x: Int64
  failure AdapterUnavailable
}

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition main at HostPlace requires HostRead, HostWrite {
  x <- perform read_int via host_input
  perform write_int(x) via host_output
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "host-boundary-positive",
  "targets": [
    {
      "target_id": "world-server",
      "role": "server",
      "place_refs": ["HostPlace"],
      "entry_transitions": ["main"],
      "observation_policy": "authoritative_world_state",
      "redaction_policy": "world_private",
      "retention_policy": "session_authority_log",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": false
    },
    {
      "target_id": "host-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "native_disabled",
      "save_load_authority": false,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "host_input",
      "boundary_kind": "ffi",
      "effect_names": ["read_int"],
      "from_target": "host-adapter",
      "to_target": "world-server",
      "authority": "adapter_read_only",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.host_input.read_int",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    },
    {
      "boundary_ref": "host_output",
      "boundary_kind": "ffi",
      "effect_names": ["write_int"],
      "from_target": "world-server",
      "to_target": "host-adapter",
      "authority": "adapter_write_side_effect",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.host_output.write_int",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    }
  ]
}"#,
    );

    let output = run_cli(&[
        "project-full-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(output.status.success(), "{value:?}");
    assert_eq!(value["surface_kind"], "full_system_v1_projection_report");
    assert_eq!(value["accepted"], true);
    assert_eq!(value["projection_id"], "host-boundary-positive");
    assert_eq!(value["final_public_api_frozen"], false);
    assert_eq!(value["packet_schemas"], serde_json::json!([]));
    assert_eq!(value["ffi_schemas"].as_array().map(Vec::len), Some(2));
    assert_eq!(
        value["ffi_schemas"][0]["request_fields"],
        serde_json::json!([])
    );
    assert_eq!(
        value["ffi_schemas"][0]["response_fields"],
        serde_json::json!([{ "name": "x", "ty": "Int64" }])
    );
    assert!(
        value["residual_obligations"]
            .as_array()
            .expect("residual obligations should be an array")
            .iter()
            .any(|row| row["code"] == "packet_ffi_transport_semantics_deferred")
    );
}

#[test]
fn project_full_v1_reports_authority_rejection() {
    let root = unique_temp_dir("mirrorea-full-system-v1-cli-projection-reject");
    fs::create_dir_all(&root).expect("temp root should be created");
    let source = write_file(
        &root,
        "main/src/client-write-authority-negative.mir",
        r#"module FullSystemV1.ClientWriteAuthorityNegative

capability HostWrite

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition update at ClientView requires HostWrite {
  perform write_int(1) via client_output
}
"#,
    );
    let request = write_file(
        &root,
        "main/projection.request.json",
        r#"{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "client-write-authority-negative",
  "targets": [
    {
      "target_id": "world-client",
      "role": "client",
      "place_refs": ["ClientView"],
      "entry_transitions": ["update"],
      "observation_policy": "observer_safe_projection",
      "redaction_policy": "observer_safe",
      "retention_policy": "client_ephemeral",
      "provider_policy": "no_provider_calls",
      "save_load_authority": false,
      "prediction_allowed": true
    },
    {
      "target_id": "host-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "native_disabled",
      "save_load_authority": false,
      "prediction_allowed": false
    }
  ],
  "boundaries": [
    {
      "boundary_ref": "client_output",
      "boundary_kind": "ffi",
      "effect_names": ["write_int"],
      "from_target": "world-client",
      "to_target": "host-adapter",
      "authority": "client_world_write",
      "required_witnesses": [],
      "packet_schema_ref": null,
      "ffi_schema_ref": "ffi.client.write_int",
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "no_state_transfer"
    }
  ]
}"#,
    );

    let output = run_cli(&[
        "project-full-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success());
    assert_eq!(value["accepted"], false);
    assert_eq!(
        value["diagnostics"][0]["code"],
        "client_write_authority_escalation"
    );
}

#[test]
fn run_full_v1_split_executes_local_role_reports() {
    let source = server_client_sample_path("main/src/role-split-positive.mir");
    let request = server_client_sample_path("projection.request.json");

    let output = run_cli(&[
        "run-full-v1-split",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--input",
        "40",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(output.status.success(), "{value:?}");
    assert_eq!(value["surface_kind"], "full_system_v1_local_split_report");
    assert_eq!(value["accepted"], true);
    assert_eq!(value["projection_id"], "role-split-positive");
    assert_eq!(value["launch_mode"], "same_binary_local_role_wrapper");
    assert!(
        value["residual_obligations"]
            .as_array()
            .expect("residual obligations should be an array")
            .iter()
            .any(|row| row["code"] == "docker_process_carrier_deferred")
    );
    assert!(
        !value["residual_obligations"]
            .as_array()
            .expect("residual obligations should be an array")
            .iter()
            .any(|row| row["code"] == "server_client_runtime_split_deferred")
    );
    assert_eq!(value["target_reports"].as_array().map(Vec::len), Some(3));
    let target_reports = value["target_reports"]
        .as_array()
        .expect("target reports should be an array");
    let server = target_reports
        .iter()
        .find(|row| row["target_id"] == "world-server")
        .expect("server report should exist");
    let client = target_reports
        .iter()
        .find(|row| row["target_id"] == "world-client")
        .expect("client report should exist");
    let adapter = target_reports
        .iter()
        .find(|row| row["target_id"] == "host-adapter")
        .expect("adapter report should exist");
    assert_eq!(
        server["execution_kind"],
        serde_json::json!("authoritative_runtime")
    );
    assert_eq!(
        client["execution_kind"],
        serde_json::json!("authoritative_runtime")
    );
    assert_eq!(
        adapter["execution_kind"],
        serde_json::json!("passive_endpoint")
    );
}

#[test]
fn run_full_v1_split_rejects_non_admitted_entry_override() {
    let source = server_client_sample_path("main/src/role-split-positive.mir");
    let request = server_client_sample_path("projection.request.json");

    let output = run_cli(&[
        "run-full-v1-split",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--input",
        "40",
        "--target",
        "world-client",
        "--entry",
        "main",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success());
    assert_eq!(value["accepted"], false);
    assert_eq!(value["selected_target_id"], "world-client");
    assert_eq!(value["entry_override"], "main");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "entry_transition_not_admitted"
    );
    assert_eq!(
        value["rejected_rows"],
        serde_json::json!(["world-client:entry_transition_not_admitted"])
    );
}
