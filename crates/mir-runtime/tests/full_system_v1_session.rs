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
