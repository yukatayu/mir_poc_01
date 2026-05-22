use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::posegraph_runtime::{PoseGraphRuntimeOutcome, run_posegraph_runtime_package_path};
use serde_json::json;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn write_package(root: &Path, relative_path: &str, payload: serde_json::Value) -> PathBuf {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("package path should have parent"))
        .expect("parent directory should be created");
    fs::write(
        &path,
        serde_json::to_string_pretty(&payload).expect("payload should serialize"),
    )
    .expect("package payload should be written");
    path
}

#[test]
fn posegraph_runtime_accepts_no_split_frame_and_preserves_anchor_state() {
    let root = unique_temp_dir("posegraph-runtime-accept");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "no-split-frame-positive/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-no-split-frame-positive",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.NoSplitFramePositive",
            "transition_id": "stable_frame",
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
                        },
                        {
                            "anchor_ref": "anchor#world/origin",
                            "reason": "world_origin_last_resort"
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
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(report.accepted, "{report:?}");
    assert_eq!(report.terminal_outcome, PoseGraphRuntimeOutcome::Accepted);
    assert_eq!(
        report.runtime_state.pose_snapshot_frontier,
        "snapshot#avatar-017"
    );
    assert_eq!(report.runtime_state.nodes.len(), 2);
    assert_eq!(report.runtime_state.anchor_bindings.len(), 1);
    assert_eq!(report.runtime_state.anchor_switch_log.len(), 1);
    assert_eq!(
        report.runtime_state.anchor_switch_log[0]
            .required_capability
            .as_str(),
        "ObservePose"
    );
    assert_eq!(
        report.runtime_state.reacquire_required,
        Vec::<String>::new()
    );
}

#[test]
fn posegraph_runtime_accepts_save_load_roundtrip_and_exports_devtools_panels() {
    let root = unique_temp_dir("posegraph-runtime-save-load-roundtrip");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "save-load-roundtrip/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-save-load-roundtrip",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.SaveLoadRoundtrip",
            "transition_id": "pose_roundtrip",
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
                    "current_anchor_witness": "anchor_witness#fresh",
                    "save_load": {
                        "savepoint_ref": "savepoint#pose-06-avatar-017",
                        "saved_pose_snapshot_frontier": "snapshot#avatar-017",
                        "saved_membership_epoch": 3,
                        "saved_owner_epoch": 9,
                        "saved_anchor_switch_sequence": 41,
                        "saved_anchor_witness": "anchor_witness#fresh",
                        "saved_active_anchor": "anchor#avatar-017/head"
                    }
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(report.accepted, "{report:?}");
    assert!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .load_admissible
    );
    assert!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .state_roundtrip_equal
    );
    assert_eq!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .savepoint_ref
            .as_str(),
        "savepoint#pose-06-avatar-017"
    );
    assert!(
        report
            .devtools_export
            .panel_ids
            .contains(&"posegraph_node_list".to_string())
    );
    assert!(
        report
            .devtools_export
            .panel_ids
            .contains(&"pose_snapshot_timeline".to_string())
    );
}

#[test]
fn posegraph_runtime_exports_split_frame_violation() {
    let root = unique_temp_dir("posegraph-runtime-violation");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "split-frame-negative/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-split-frame-negative",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.SplitFrameNegative",
            "transition_id": "split_frame_violation",
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
                    "last_anchor_switch_sequence": 40,
                    "save_load": {
                        "savepoint_ref": "savepoint#pose-05-avatar-017",
                        "saved_pose_snapshot_frontier": "snapshot#avatar-017",
                        "saved_membership_epoch": 3,
                        "saved_owner_epoch": 9,
                        "saved_anchor_switch_sequence": 41,
                        "saved_active_anchor": "anchor#avatar-017/head"
                    }
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::ViolationExport
    );
    assert_eq!(
        report
            .violation
            .as_ref()
            .map(|row| row.violation_kind.as_str()),
        Some("no_split_frame")
    );
    assert!(
        report
            .violation
            .as_ref()
            .expect("violation should exist")
            .detail
            .contains("snapshot mismatch")
    );
    assert!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .load_admissible
    );
    assert!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .state_roundtrip_equal
    );
}

#[test]
fn posegraph_runtime_rejects_stale_anchor_membership_epoch() {
    let root = unique_temp_dir("posegraph-runtime-stale-membership");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "stale-anchor-after-membership-advance/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-stale-anchor-membership",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.StaleAnchorAfterMembershipAdvance",
            "transition_id": "reject_stale_anchor",
            "runtime_input": {
                "posegraph": {
                    "anchored_pose": {
                        "entity_ref": "object#hat-017",
                        "anchor_ref": "anchor#avatar-017/head",
                        "pose_version": 17,
                        "pose_snapshot_ref": "snapshot#avatar-017",
                        "membership_epoch": 3,
                        "owner_epoch": 9,
                        "state": "stable"
                    },
                    "current_membership_epoch": 4
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("stale_anchor_membership_epoch")
    );
}

#[test]
fn posegraph_runtime_rejects_anchor_switch_frontier_regression() {
    let root = unique_temp_dir("posegraph-runtime-anchor-switch-regression");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "anchor-switch-frontier-negative/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-anchor-switch-regression",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.AnchorSwitchFrontierNegative",
            "transition_id": "reject_anchor_switch",
            "runtime_input": {
                "posegraph": {
                    "anchor_switch_log": [
                        {
                            "from_anchor": "anchor#avatar-017/shoulder",
                            "to_anchor": "anchor#avatar-017/head",
                            "reason": "fresh_head_visible",
                            "required_capability": "ObservePose",
                            "membership_epoch": 3,
                            "owner_epoch": 9,
                            "sequence": 12,
                            "pose_snapshot_frontier": "snapshot#avatar-017"
                        },
                        {
                            "from_anchor": "anchor#avatar-017/head",
                            "to_anchor": "anchor#avatar-017/shoulder",
                            "reason": "tracking_lost",
                            "required_capability": "ObservePose",
                            "membership_epoch": 3,
                            "owner_epoch": 9,
                            "sequence": 11,
                            "pose_snapshot_frontier": "snapshot#avatar-017"
                        }
                    ],
                    "current_owner_epoch": 9,
                    "last_anchor_switch_sequence": 10
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("anchor_switch_frontier_regression")
    );
}

#[test]
fn posegraph_runtime_rejects_anchor_switch_pose_snapshot_frontier_mismatch() {
    let root = unique_temp_dir("posegraph-runtime-anchor-switch-frontier-mismatch");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "anchor-switch-frontier-mismatch/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-anchor-switch-frontier-mismatch",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.AnchorSwitchFrontierMismatch",
            "transition_id": "reject_anchor_switch_frontier_mismatch",
            "runtime_input": {
                "posegraph": {
                    "pose_snapshot_frontier": "snapshot#avatar-018",
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
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("anchor_switch_frontier_regression")
    );
}

#[test]
fn posegraph_runtime_rejects_stale_anchor_switch_membership_epoch() {
    let root = unique_temp_dir("posegraph-runtime-stale-switch-membership");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "stale-anchor-switch-membership/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-stale-anchor-switch-membership",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.StaleAnchorSwitchMembership",
            "transition_id": "reject_stale_anchor_switch",
            "runtime_input": {
                "posegraph": {
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
                    "current_membership_epoch": 4,
                    "current_owner_epoch": 9,
                    "last_anchor_switch_sequence": 40
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("stale_anchor_membership_epoch")
    );
}

#[test]
fn posegraph_runtime_rejects_save_load_membership_epoch_mismatch() {
    let root = unique_temp_dir("posegraph-runtime-save-load-membership-stale");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "save-load-membership-stale/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-save-load-membership-stale",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.SaveLoadMembershipStale",
            "transition_id": "load_membership_stale",
            "runtime_input": {
                "posegraph": {
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
                    "current_anchor_witness": "anchor_witness#fresh",
                    "save_load": {
                        "savepoint_ref": "savepoint#pose-07-avatar-017",
                        "saved_pose_snapshot_frontier": "snapshot#avatar-017",
                        "saved_membership_epoch": 4,
                        "saved_owner_epoch": 9,
                        "saved_anchor_switch_sequence": 41,
                        "saved_anchor_witness": "anchor_witness#fresh"
                    }
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("save_load_inadmissible")
    );
    assert_eq!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .load_admissible,
        false
    );
    assert_eq!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .savepoint_ref
            .as_str(),
        "savepoint#pose-07-avatar-017"
    );
    assert!(
        report
            .rejection
            .as_ref()
            .expect("rejection should exist")
            .message
            .contains("saved_membership_epoch")
    );
}

#[test]
fn posegraph_runtime_rejects_save_load_stale_anchor_witness() {
    let root = unique_temp_dir("posegraph-runtime-save-load-stale-anchor-witness");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "save-load-stale-anchor-witness/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-save-load-stale-anchor-witness",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.SaveLoadStaleAnchorWitness",
            "transition_id": "load_anchor_witness_stale",
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
                    "current_anchor_witness": "anchor_witness#fresh",
                    "save_load": {
                        "savepoint_ref": "savepoint#pose-10-avatar-017",
                        "saved_pose_snapshot_frontier": "snapshot#avatar-017",
                        "saved_membership_epoch": 3,
                        "saved_owner_epoch": 9,
                        "saved_anchor_switch_sequence": 41,
                        "saved_anchor_witness": "anchor_witness#stale",
                        "saved_active_anchor": "anchor#avatar-017/head"
                    }
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("save_load_inadmissible")
    );
    assert_eq!(
        report
            .save_load_state
            .as_ref()
            .expect("save/load state should exist")
            .load_admissible,
        false
    );
    assert!(
        report
            .rejection
            .as_ref()
            .expect("rejection should exist")
            .message
            .contains("saved_anchor_witness")
    );
}

#[test]
fn posegraph_runtime_requires_explicit_reacquire_after_fallback_only_state() {
    let root = unique_temp_dir("posegraph-runtime-reacquire");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "stale-anchor-reacquire-required/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-reacquire-required",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.StaleAnchorReacquireRequired",
            "transition_id": "explicit_reacquire",
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
                            "reason": "head_lost_tracking"
                        }
                    ],
                    "fresh_anchor_witness": "anchor_witness#fresh",
                    "current_anchor_witness": "anchor_witness#stale"
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("reacquire_required")
    );
    assert_eq!(
        report.runtime_state.reacquire_required,
        vec!["object#hat-017".to_string()]
    );
}

#[test]
fn posegraph_runtime_requires_explicit_reacquire_when_fallback_only_has_no_fresh_witness() {
    let root = unique_temp_dir("posegraph-runtime-reacquire-missing-witness");
    fs::create_dir_all(&root).expect("temp root should be created");
    let package = write_package(
        &root,
        "stale-anchor-reacquire-required/package.mir.json",
        json!({
            "schema_version": "posegraph-runtime-package-v0",
            "package_id": "package#pose-reacquire-required-missing-witness",
            "package_kind": "posegraph_runtime",
            "module_id": "PoseGraph.StaleAnchorReacquireMissingWitness",
            "transition_id": "explicit_reacquire_missing_witness",
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
                            "reason": "head_lost_tracking"
                        }
                    ]
                }
            }
        }),
    );

    let report = run_posegraph_runtime_package_path(&package);

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.terminal_outcome,
        PoseGraphRuntimeOutcome::RuntimeRejection
    );
    assert_eq!(
        report.rejection.as_ref().map(|row| row.code.as_str()),
        Some("reacquire_required")
    );
    assert_eq!(
        report.runtime_state.reacquire_required,
        vec!["object#hat-017".to_string()]
    );
}
