use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::full_system_v1_renderer_pose_backend::run_full_system_v1_renderer_pose_backend_path;

fn renderer_sample_path(root: &str, relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/full-system-v1/provider-adapter")
        .join(root)
        .join(relative_path)
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

#[test]
fn renderer_pose_backend_admits_pose_snapshot_delivery() {
    let report = run_full_system_v1_renderer_pose_backend_path(
        renderer_sample_path(
            "renderer-pose-positive",
            "main/src/renderer-pose-positive.mir",
        ),
        renderer_sample_path("renderer-pose-positive", "projection.request.json"),
        renderer_sample_path("renderer-pose-positive", "provider.manifest.json"),
        renderer_sample_path("renderer-pose-positive", "package.mir.json"),
        0,
    );

    assert!(report.accepted, "{report:?}");
    assert!(report.delivery_admitted);
    assert_eq!(report.terminal_outcome, "delivery_admitted");
    assert_eq!(report.provider_id, "renderer-pose-backend");
    assert_eq!(report.provider_kind, "renderer");
    assert_eq!(report.target_id, "renderer-adapter");
    assert_eq!(
        report.delivered_pose_snapshot_ref.as_deref(),
        Some("snapshot#avatar-017")
    );
    assert_eq!(
        report.pose_snapshot_frontier.as_deref(),
        Some("snapshot#avatar-017")
    );
    assert_eq!(
        report.matched_packet_schema_refs,
        vec!["packet.renderer.pose_snapshot".to_string()]
    );
    assert!(report.provider_admission_report.accepted);
    assert!(report.posegraph_runtime_report.accepted);
}

#[test]
fn renderer_pose_backend_blocks_split_frame_violation_exports() {
    let report = run_full_system_v1_renderer_pose_backend_path(
        renderer_sample_path(
            "renderer-pose-split-frame-negative",
            "main/src/renderer-pose-split-frame-negative.mir",
        ),
        renderer_sample_path(
            "renderer-pose-split-frame-negative",
            "projection.request.json",
        ),
        renderer_sample_path(
            "renderer-pose-split-frame-negative",
            "provider.manifest.json",
        ),
        renderer_sample_path("renderer-pose-split-frame-negative", "package.mir.json"),
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert!(!report.delivery_admitted);
    assert_eq!(
        report.terminal_outcome,
        "blocked_posegraph_violation_export"
    );
    assert_eq!(report.blocked_reason.as_deref(), Some("no_split_frame"));
    assert_eq!(
        report.posegraph_runtime_report.observer_safe_summary,
        "posegraph violation export: no_split_frame"
    );
}

#[test]
fn renderer_pose_backend_blocks_reacquire_rejection() {
    let report = run_full_system_v1_renderer_pose_backend_path(
        renderer_sample_path(
            "renderer-pose-reacquire-negative",
            "main/src/renderer-pose-reacquire-negative.mir",
        ),
        renderer_sample_path(
            "renderer-pose-reacquire-negative",
            "projection.request.json",
        ),
        renderer_sample_path("renderer-pose-reacquire-negative", "provider.manifest.json"),
        renderer_sample_path("renderer-pose-reacquire-negative", "package.mir.json"),
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert!(!report.delivery_admitted);
    assert_eq!(
        report.terminal_outcome,
        "blocked_posegraph_runtime_rejection"
    );
    assert_eq!(report.blocked_reason.as_deref(), Some("reacquire_required"));
    assert_eq!(
        report
            .posegraph_runtime_report
            .rejection
            .as_ref()
            .map(|row| row.code.as_str()),
        Some("reacquire_required")
    );
}

#[test]
fn renderer_pose_backend_blocks_posegraph_package_without_binding_context() {
    let root = unique_temp_dir("mir-full-system-v1-renderer-pose-unbound");
    fs::create_dir_all(&root).expect("temp root should be created");
    let posegraph = write_file(
        &root,
        "main/package.mir.json",
        r#"{
  "schema_version": "posegraph-runtime-package-v0",
  "package_id": "package#rogue-renderer-positive",
  "package_kind": "posegraph_runtime",
  "module_id": "AvatarPose.UnboundPackage",
  "transition_id": "stable_frame",
  "runtime_input": {
    "posegraph": {
      "pose_snapshot_frontier": "snapshot#avatar-017",
      "target_pose": {
        "entity_ref": "avatar#999/head",
        "pose_version": 17,
        "pose_snapshot_ref": "snapshot#avatar-017"
      },
      "anchored_pose": {
        "entity_ref": "object#rogue-999",
        "anchor_ref": "anchor#avatar-999/head",
        "pose_version": 17,
        "pose_snapshot_ref": "snapshot#avatar-017",
        "membership_epoch": 3,
        "owner_epoch": 9,
        "state": "stable"
      },
      "anchor_switch_log": [
        {
          "from_anchor": "anchor#avatar-999/shoulder",
          "to_anchor": "anchor#avatar-999/head",
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

    let report = run_full_system_v1_renderer_pose_backend_path(
        renderer_sample_path(
            "renderer-pose-positive",
            "main/src/renderer-pose-positive.mir",
        ),
        renderer_sample_path("renderer-pose-positive", "projection.request.json"),
        renderer_sample_path("renderer-pose-positive", "provider.manifest.json"),
        posegraph,
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert!(!report.delivery_admitted);
    assert_eq!(
        report.terminal_outcome,
        "blocked_posegraph_binding_context_missing"
    );
    assert_eq!(
        report.blocked_reason.as_deref(),
        Some("missing_posegraph_binding_context")
    );
}
