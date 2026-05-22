use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::full_system_v1_session::run_full_system_v1_session_path;
use mir_semantics::full_system_v1::FullSystemV1ExecutionOutcome;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn write_module(root: &Path, relative_path: &str, source: &str) -> PathBuf {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("module path should have parent"))
        .expect("parent directory should be created");
    fs::write(&path, source).expect("module source should be written");
    path
}

#[test]
fn runtime_session_wraps_success_with_observer_safe_trace() {
    let root = unique_temp_dir("mir-full-system-v1-session-accept");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_module(
        &root,
        "main/src/add-one.mir",
        r#"module Computational.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}
"#,
    );

    let report = run_full_system_v1_session_path(&source, "add_one", 41);

    assert!(report.runtime.accepted, "{report:?}");
    assert_eq!(
        report.runtime.outcome,
        FullSystemV1ExecutionOutcome::Accepted
    );
    assert_eq!(
        report
            .runtime
            .output
            .as_ref()
            .map(|row| row.summary.as_str()),
        Some("Int64(42)")
    );
    assert!(report.runtime.compute_trace.len() >= 1);
    assert!(report.observer_safe_summary.contains("accepted"));
}

#[test]
fn runtime_session_preserves_runtime_rejection_surface() {
    let root = unique_temp_dir("mir-full-system-v1-session-reject");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_module(
        &root,
        "main/src/dynamic-array-bounds.mir",
        r#"module Computational.DynamicArrayBoundsNegative

fn select(x: Int64) -> Int64 {
  let xs: [Int64; 2] = [10, 20]
  return xs[x]
}
"#,
    );

    let report = run_full_system_v1_session_path(&source, "select", 2);

    assert!(!report.runtime.accepted, "{report:?}");
    assert_eq!(
        report.runtime.outcome,
        FullSystemV1ExecutionOutcome::RuntimeRejection
    );
    assert_eq!(
        report
            .runtime
            .runtime_rejection
            .as_ref()
            .map(|row| row.code.as_str()),
        Some("runtime_out_of_bounds")
    );
    assert!(!report.runtime.compute_trace.is_empty());
}

#[test]
fn runtime_session_executes_effectful_transition_with_observer_safe_summary() {
    let root = unique_temp_dir("mir-full-system-v1-session-effectful");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");

    write_module(
        &root,
        "shared/src/add-one.mir",
        r#"module Shared.AddOne

fn add_one(x: Int64) -> Int64 {
  return x + 1
}
"#,
    );
    let source = write_module(
        &root,
        "main/src/effectful-positive.mir",
        r#"module FullSystemV1.EffectfulPositive

import Shared.AddOne

capability HostRead
capability HostWrite
capability Publisher
capability Observer
capability WitnessAuthority
capability HandoffAuthority
capability CutAuthority

effect read_int {
  requires HostRead
  output x: Int64
  failure AdapterUnavailable
}

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

effect publish_roll(value: Int64) {
  requires Publisher
  failure PublishRejected
}

effect observe_roll {
  requires Observer
  output seen: Int64
  failure ObserveRejected
}

effect issue_turn_witness(turn: Int64) {
  requires WitnessAuthority
  output ticket: Text
  failure WitnessRejected
}

effect handoff_turn(ticket: Text) {
  requires HandoffAuthority
  failure HandoffRejected
}

effect seal_places {
  requires CutAuthority
  failure CutRejected
}

effect quiesce_messages {
  requires CutAuthority
  failure CutRejected
}

effect atomic_cut(label: Text) {
  requires CutAuthority
  failure CutRejected
}

transition main at SugorokuPlace requires HostRead, HostWrite, Publisher, Observer, WitnessAuthority, HandoffAuthority, CutAuthority {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
  perform publish_roll(y) via publish_bus
  seen <- perform observe_roll via observe_bus
    ensure seen = y
  ticket <- perform issue_turn_witness(seen) via witness_store
  perform handoff_turn(ticket) via handoff_port
  perform seal_places via session_admin
  perform quiesce_messages via session_admin
  perform atomic_cut("turn-finished") via session_cut
}
"#,
    );

    let report = run_full_system_v1_session_path(&source, "main", 41);

    assert!(report.runtime.accepted, "{report:?}");
    assert_eq!(
        report.runtime.outcome,
        FullSystemV1ExecutionOutcome::Accepted
    );
    assert!(report.observer_safe_summary.contains("accepted"));
    assert!(
        report
            .runtime
            .compute_trace
            .iter()
            .flat_map(|trace| trace.events.iter())
            .any(|event| event.kind == "publish")
    );
    assert!(
        report
            .runtime
            .compute_trace
            .iter()
            .flat_map(|trace| trace.events.iter())
            .any(|event| event.kind == "handoff")
    );
}

#[test]
fn runtime_session_rejects_effectful_missing_witness_transition() {
    let root = unique_temp_dir("mir-full-system-v1-session-effectful-negative");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_module(
        &root,
        "main/src/missing-witness.mir",
        r#"module FullSystemV1.MissingWitness

capability HandoffAuthority

effect handoff_turn(ticket: Text) {
  requires HandoffAuthority
  failure HandoffRejected
}

transition main at SugorokuPlace requires HandoffAuthority {
  perform handoff_turn("ticket#missing") via handoff_port
}
"#,
    );

    let report = run_full_system_v1_session_path(&source, "main", 0);

    assert!(!report.runtime.accepted, "{report:?}");
    assert_eq!(
        report.runtime.outcome,
        FullSystemV1ExecutionOutcome::RuntimeRejection
    );
    assert_eq!(
        report
            .runtime
            .runtime_rejection
            .as_ref()
            .map(|row| row.code.as_str()),
        Some("missing_live_witness")
    );
    assert!(report.observer_safe_summary.contains("runtime rejection"));
}

#[test]
fn runtime_session_rejects_renderer_boundary_without_admission_context() {
    let root = unique_temp_dir("mir-full-system-v1-session-renderer-boundary");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_module(
        &root,
        "main/src/renderer-boundary.mir",
        r#"module FullSystemV1.RendererBoundaryDirect

capability RenderFrame

effect render_pose_frame(snapshot_ref: Text) {
  requires RenderFrame
  output receipt: Text
  failure RendererUnavailable
}

transition render_pose at ClientView requires RenderFrame {
  receipt <- perform render_pose_frame("snapshot#avatar-017") via renderer_frame_packet
}
"#,
    );

    let report = run_full_system_v1_session_path(&source, "render_pose", 0);

    assert!(!report.runtime.accepted, "{report:?}");
    assert_eq!(
        report.runtime.outcome,
        FullSystemV1ExecutionOutcome::RuntimeRejection
    );
    assert_eq!(
        report
            .runtime
            .runtime_rejection
            .as_ref()
            .map(|row| row.code.as_str()),
        Some("unsupported_effect_runtime")
    );
}
