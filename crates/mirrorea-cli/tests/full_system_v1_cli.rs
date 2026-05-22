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

fn provider_sample_path(root: &str, relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/full-system-v1/provider-adapter")
        .join(root)
        .join(relative_path)
}

fn write_renderer_pose_source(root: &Path, module_name: &str) -> PathBuf {
    write_file(
        root,
        &format!("main/src/{module_name}.mir"),
        &format!(
            r#"module FullSystemV1.{module_name}

capability RenderFrame

effect render_pose_frame(snapshot_ref: Text) {{
  requires RenderFrame
  output receipt: Text
  failure RendererUnavailable
  failure PresentationDropped
}}

transition render_pose at ClientView requires RenderFrame {{
  let pose_snapshot_ref: Text = "snapshot#avatar-017"
  receipt <- perform render_pose_frame(pose_snapshot_ref) via renderer_frame_packet
}}
"#
        ),
    )
}

fn write_renderer_pose_request(root: &Path, projection_id: &str) -> PathBuf {
    write_file(
        root,
        "main/projection.request.json",
        &format!(
            r#"{{
  "schema_version": "full-system-v1-projection-request-v0",
  "projection_id": "{projection_id}",
  "targets": [
    {{
      "target_id": "world-server",
      "role": "server",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "authoritative_world_state",
      "redaction_policy": "world_private",
      "retention_policy": "session_authority_log",
      "provider_policy": "no_provider_calls",
      "save_load_authority": true,
      "prediction_allowed": false
    }},
    {{
      "target_id": "world-client",
      "role": "client",
      "place_refs": ["ClientView"],
      "entry_transitions": ["render_pose"],
      "observation_policy": "observer_safe_projection",
      "redaction_policy": "observer_safe",
      "retention_policy": "client_ephemeral",
      "provider_policy": "no_provider_calls",
      "save_load_authority": false,
      "prediction_allowed": true
    }},
    {{
      "target_id": "renderer-adapter",
      "role": "adapter",
      "place_refs": [],
      "entry_transitions": [],
      "observation_policy": "adapter_request_reply",
      "redaction_policy": "adapter_local",
      "retention_policy": "adapter_local_debug",
      "provider_policy": "provider_inventory_only",
      "save_load_authority": false,
      "prediction_allowed": false
    }}
  ],
  "boundaries": [
    {{
      "boundary_ref": "renderer_frame_packet",
      "boundary_kind": "packet",
      "effect_names": ["render_pose_frame"],
      "from_target": "world-client",
      "to_target": "renderer-adapter",
      "authority": "observer_safe_pose_snapshot",
      "required_witnesses": [],
      "packet_schema_ref": "packet.renderer.pose_snapshot",
      "ffi_schema_ref": null,
      "rollback_cut_compatible": true,
      "replay_compatible": true,
      "save_load_obligation": "observer_projection_only"
    }}
  ]
}}"#
        ),
    )
}

fn write_renderer_pose_provider(root: &Path, provider_id: &str) -> PathBuf {
    write_file(
        root,
        "main/provider.manifest.json",
        &format!(
            r#"{{
  "schema_version": "full-system-v1-provider-manifest-v0",
  "provider_id": "{provider_id}",
  "provider_kind": "renderer",
  "target_id": "renderer-adapter",
  "input_schema": "render_frame_request",
  "output_schema": "render_frame_observation",
  "effect_row": ["render_pose_frame"],
  "failure_row": ["PresentationDropped"],
  "required_capabilities": ["RenderFrame"],
  "authority_policy": {{
    "semantic_authority_owner": "mir_mirrorea",
    "provider_may_grant_authority": false,
    "provider_may_mutate_world_state": false
  }},
  "resource_limits": {{
    "max_memory_mb": 64,
    "max_cpu_ms": 16,
    "semantic_state_owner": "mir_mirrorea",
    "provider_handles_are_nonsemantic": true
  }},
  "sandbox_policy": {{
    "sandbox_required": false,
    "current_status": "inventory_only"
  }},
  "observation_policy": {{
    "provider_receives_redacted_observation_only": true,
    "retention_owner": "mir_mirrorea"
  }},
  "redaction_policy": {{
    "provider_may_emit_unredacted_debug": false,
    "redaction_label_required": true
  }},
  "retention_policy": "adapter_local_debug",
  "packet_boundary": "packet.renderer.pose_snapshot",
  "ffi_boundary": null,
  "native_execution_policy": "Disabled",
  "wasm_execution_policy": "InventoryOnly",
  "rollback_replay_cut_policy": "Replayable"
}}"#
        ),
    )
}

fn write_posegraph_package(root: &Path, package_id: &str, payload: &str) -> PathBuf {
    write_file(root, &format!("{package_id}/package.mir.json"), payload)
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

#[test]
fn admit_provider_v1_reports_inventory_admission() {
    let source = provider_sample_path(
        "viewer-diagnostic-positive",
        "main/src/viewer-diagnostic-positive.mir",
    );
    let request = provider_sample_path("viewer-diagnostic-positive", "projection.request.json");
    let provider = provider_sample_path("viewer-diagnostic-positive", "provider.manifest.json");

    let output = run_cli(&[
        "admit-provider-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--provider",
        provider.to_str().expect("provider should be utf-8"),
        "--input",
        "0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(output.status.success(), "{value:?}");
    assert_eq!(
        value["surface_kind"],
        "full_system_v1_provider_admission_report"
    );
    assert_eq!(value["accepted"], true);
    assert_eq!(value["provider_id"], "viewer-diagnostic-exporter");
    assert_eq!(value["terminal_outcome"], "inventory_admitted");
    assert_eq!(value["execution_admitted"], false);
    assert_eq!(value["target_id"], "diagnostic-adapter");
    assert_eq!(value["target_provider_policy"], "provider_inventory_only");
}

#[test]
fn admit_provider_v1_reports_native_disabled_policy() {
    let source = provider_sample_path(
        "native-disabled-negative",
        "main/src/native-disabled-negative.mir",
    );
    let request = provider_sample_path("native-disabled-negative", "projection.request.json");
    let provider = provider_sample_path("native-disabled-negative", "provider.manifest.json");

    let output = run_cli(&[
        "admit-provider-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--provider",
        provider.to_str().expect("provider should be utf-8"),
        "--input",
        "0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success());
    assert_eq!(value["accepted"], false);
    assert_eq!(value["terminal_outcome"], "native_execution_disabled");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "native_execution_disabled_by_default"
    );
}

#[test]
fn admit_provider_v1_rejects_over_capability_manifest() {
    let source = provider_sample_path(
        "over-capability-negative",
        "main/src/over-capability-negative.mir",
    );
    let request = provider_sample_path("over-capability-negative", "projection.request.json");
    let provider = provider_sample_path("over-capability-negative", "provider.manifest.json");

    let output = run_cli(&[
        "admit-provider-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--provider",
        provider.to_str().expect("provider should be utf-8"),
        "--input",
        "0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success());
    assert_eq!(value["accepted"], false);
    assert_eq!(value["terminal_outcome"], "rejected");
    assert_eq!(value["diagnostics"][0]["code"], "provider_over_capability");
}

#[test]
fn render_pose_backend_v1_reports_delivery_admission() {
    let root = unique_temp_dir("mirrorea-full-system-v1-cli-renderer-pose");
    fs::create_dir_all(&root).expect("temp root should be created");
    let source = write_renderer_pose_source(&root, "RendererPosePositive");
    let request = write_renderer_pose_request(&root, "renderer-pose-positive");
    let provider = write_renderer_pose_provider(&root, "renderer-pose-backend");
    let posegraph = write_posegraph_package(
        &root,
        "posegraph-positive",
        r#"{
  "schema_version": "posegraph-runtime-package-v0",
  "package_id": "package#pose-renderer-positive",
  "package_kind": "posegraph_runtime",
  "module_id": "AvatarPose.RendererPositive",
  "transition_id": "stable_frame",
  "binding_context": {
    "projection_id": "renderer-pose-positive",
    "source_module_refs": ["FullSystemV1.RendererPosePositive"],
    "from_target_id": "world-client",
    "to_target_id": "renderer-adapter",
    "boundary_ref": "renderer_frame_packet",
    "entry_transition": "render_pose",
    "provider_id": "renderer-pose-backend"
  },
  "runtime_input": {
    "posegraph": {
      "pose_snapshot_frontier": "snapshot#avatar-017",
      "target_pose": {
        "entity_ref": "avatar#017/head",
        "pose_version": 17,
        "pose_snapshot_ref": "snapshot#avatar-017"
      },
      "anchored_pose": {
        "entity_ref": "object#hat-017",
        "anchor_ref": "anchor#avatar-017/head",
        "pose_version": 17,
        "pose_snapshot_ref": "snapshot#avatar-017",
        "membership_epoch": 3,
        "owner_epoch": 9,
        "state": "stable"
      },
      "fallback_chain": [
        {
          "anchor_ref": "anchor#avatar-017/shoulder",
          "reason": "occlusion_fallback"
        }
      ],
      "anchor_switch_log": [
        {
          "from_anchor": "anchor#avatar-017/shoulder",
          "to_anchor": "anchor#avatar-017/head",
          "reason": "fresh_head_visible",
          "required_capability": "ObservePose",
          "membership_epoch": 3,
          "owner_epoch": 9,
          "sequence": 41,
          "pose_snapshot_frontier": "snapshot#avatar-017"
        }
      ],
      "current_membership_epoch": 3,
      "current_owner_epoch": 9,
      "last_anchor_switch_sequence": 40,
      "fresh_anchor_witness": "anchor_witness#fresh",
      "current_anchor_witness": "anchor_witness#fresh"
    }
  }
}"#,
    );

    let output = run_cli(&[
        "render-pose-backend-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--provider",
        provider.to_str().expect("provider should be utf-8"),
        "--posegraph-package",
        posegraph
            .to_str()
            .expect("posegraph package should be utf-8"),
        "--input",
        "0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(output.status.success(), "{value:?}");
    assert_eq!(
        value["surface_kind"],
        "full_system_v1_renderer_pose_backend_report"
    );
    assert_eq!(value["accepted"], true);
    assert_eq!(value["delivery_admitted"], true);
    assert_eq!(value["terminal_outcome"], "delivery_admitted");
    assert_eq!(value["provider_id"], "renderer-pose-backend");
    assert_eq!(value["provider_kind"], "renderer");
    assert_eq!(value["target_id"], "renderer-adapter");
    assert_eq!(value["semantic_owner"], "mir_mirrorea");
    assert_eq!(value["pose_snapshot_frontier"], "snapshot#avatar-017");
    assert_eq!(value["delivered_pose_snapshot_ref"], "snapshot#avatar-017");
    assert_eq!(
        value["matched_packet_schema_refs"],
        serde_json::json!(["packet.renderer.pose_snapshot"])
    );
}

#[test]
fn render_pose_backend_v1_blocks_split_frame_violation() {
    let root = unique_temp_dir("mirrorea-full-system-v1-cli-renderer-pose-violation");
    fs::create_dir_all(&root).expect("temp root should be created");
    let source = write_renderer_pose_source(&root, "RendererPoseSplitFrameNegative");
    let request = write_renderer_pose_request(&root, "renderer-pose-split-frame-negative");
    let provider = write_renderer_pose_provider(&root, "renderer-pose-backend");
    let posegraph = write_posegraph_package(
        &root,
        "posegraph-split-frame-negative",
        r#"{
  "schema_version": "posegraph-runtime-package-v0",
  "package_id": "package#pose-renderer-split-frame-negative",
  "package_kind": "posegraph_runtime",
  "module_id": "AvatarPose.RendererSplitFrameNegative",
  "transition_id": "split_frame_violation",
  "binding_context": {
    "projection_id": "renderer-pose-split-frame-negative",
    "source_module_refs": ["FullSystemV1.RendererPoseSplitFrameNegative"],
    "from_target_id": "world-client",
    "to_target_id": "renderer-adapter",
    "boundary_ref": "renderer_frame_packet",
    "entry_transition": "render_pose",
    "provider_id": "renderer-pose-backend"
  },
  "runtime_input": {
    "posegraph": {
      "target_pose": {
        "entity_ref": "avatar#017/head",
        "pose_version": 17,
        "pose_snapshot_ref": "snapshot#avatar-017"
      },
      "anchored_pose": {
        "entity_ref": "object#hat-017",
        "anchor_ref": "anchor#avatar-017/head",
        "pose_version": 18,
        "pose_snapshot_ref": "snapshot#avatar-018",
        "membership_epoch": 3,
        "owner_epoch": 9,
        "state": "stable"
      },
      "anchor_switch_log": [
        {
          "from_anchor": "anchor#avatar-017/shoulder",
          "to_anchor": "anchor#avatar-017/head",
          "reason": "fresh_head_visible",
          "required_capability": "ObservePose",
          "membership_epoch": 3,
          "owner_epoch": 9,
          "sequence": 41,
          "pose_snapshot_frontier": "snapshot#avatar-017"
        }
      ],
      "current_membership_epoch": 3,
      "current_owner_epoch": 9,
      "last_anchor_switch_sequence": 40
    }
  }
}"#,
    );

    let output = run_cli(&[
        "render-pose-backend-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--provider",
        provider.to_str().expect("provider should be utf-8"),
        "--posegraph-package",
        posegraph
            .to_str()
            .expect("posegraph package should be utf-8"),
        "--input",
        "0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success());
    assert_eq!(value["accepted"], false);
    assert_eq!(value["delivery_admitted"], false);
    assert_eq!(
        value["terminal_outcome"],
        "blocked_posegraph_violation_export"
    );
    assert_eq!(value["blocked_reason"], "no_split_frame");
    assert_eq!(
        value["posegraph_runtime_report"]["terminal_outcome"],
        "ViolationExport"
    );
}

#[test]
fn render_pose_backend_v1_blocks_reacquire_rejection() {
    let root = unique_temp_dir("mirrorea-full-system-v1-cli-renderer-pose-reacquire");
    fs::create_dir_all(&root).expect("temp root should be created");
    let source = write_renderer_pose_source(&root, "RendererPoseReacquireNegative");
    let request = write_renderer_pose_request(&root, "renderer-pose-reacquire-negative");
    let provider = write_renderer_pose_provider(&root, "renderer-pose-backend");
    let posegraph = write_posegraph_package(
        &root,
        "posegraph-reacquire-negative",
        r#"{
  "schema_version": "posegraph-runtime-package-v0",
  "package_id": "package#pose-renderer-reacquire-negative",
  "package_kind": "posegraph_runtime",
  "module_id": "AvatarPose.RendererPoseReacquireNegative",
  "transition_id": "explicit_reacquire",
  "binding_context": {
    "projection_id": "renderer-pose-reacquire-negative",
    "source_module_refs": ["FullSystemV1.RendererPoseReacquireNegative"],
    "from_target_id": "world-client",
    "to_target_id": "renderer-adapter",
    "boundary_ref": "renderer_frame_packet",
    "entry_transition": "render_pose",
    "provider_id": "renderer-pose-backend"
  },
  "runtime_input": {
    "posegraph": {
      "anchored_pose": {
        "entity_ref": "object#hat-017",
        "anchor_ref": "anchor#avatar-017/shoulder",
        "pose_version": 17,
        "pose_snapshot_ref": "snapshot#avatar-017",
        "membership_epoch": 3,
        "owner_epoch": 9,
        "state": "fallback_only"
      },
      "fallback_chain": [
        {
          "anchor_ref": "anchor#avatar-017/shoulder",
          "reason": "occlusion_fallback"
        }
      ],
      "save_load": {
        "savepoint_ref": "savepoint#pose-09-avatar-017",
        "saved_pose_snapshot_frontier": "snapshot#avatar-017",
        "saved_membership_epoch": 3,
        "saved_owner_epoch": 9,
        "saved_anchor_switch_sequence": 41,
        "saved_anchor_witness": "anchor_witness#fresh",
        "saved_active_anchor": "anchor#avatar-017/head"
      }
    }
  }
}"#,
    );

    let output = run_cli(&[
        "render-pose-backend-v1",
        source.to_str().expect("source should be utf-8"),
        "--request",
        request.to_str().expect("request should be utf-8"),
        "--provider",
        provider.to_str().expect("provider should be utf-8"),
        "--posegraph-package",
        posegraph
            .to_str()
            .expect("posegraph package should be utf-8"),
        "--input",
        "0",
        "--format",
        "json",
    ]);
    let value = json_stdout(&output);

    assert!(!output.status.success());
    assert_eq!(value["accepted"], false);
    assert_eq!(value["delivery_admitted"], false);
    assert_eq!(
        value["terminal_outcome"],
        "blocked_posegraph_runtime_rejection"
    );
    assert_eq!(value["blocked_reason"], "reacquire_required");
    assert_eq!(
        value["posegraph_runtime_report"]["terminal_outcome"],
        "RuntimeRejection"
    );
}
