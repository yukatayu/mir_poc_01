use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    build_current_l2_source_sample_theorem_lean_stub_pilot_actualization,
};

fn order_handoff_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

fn typed_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-typed-proof-model-check")
        .join(name)
}

fn prototype_host_plan_path(sample_path: &Path) -> PathBuf {
    sample_path.with_extension("host-plan.json")
}

fn prototype_host_plan(sample_path: &Path) -> FixtureHostPlan {
    load_host_plan_from_path(&prototype_host_plan_path(sample_path)).unwrap()
}

fn lean_available() -> bool {
    Command::new("lean")
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn unique_temp_lean_path(stem: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("{stem}-{timestamp}.lean"))
}

fn assert_runtime_prototype_passes_actual_lean_execution(sample_path: &Path) {
    let actualization = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(sample_path),
    )
    .unwrap();

    assert_eq!(
        actualization.pilot_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(actualization.pilot_guard_reason.is_none());
    assert!(!actualization.lean_stub_artifacts.is_empty());

    let probe_path = unique_temp_lean_path("current-l2-prototype-actual-lean-probe");
    let source_text = actualization
        .lean_stub_artifacts
        .iter()
        .map(|artifact| artifact.source_text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&probe_path, source_text).unwrap();

    let output = Command::new("lean").arg(&probe_path).output().unwrap();
    let _ = fs::remove_file(&probe_path);

    assert!(
        output.status.success(),
        "expected actual Lean execution to succeed for {}, stderr={}",
        sample_path.display(),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn theorem_actual_lean_execution_reaches_typed_runtime_prototype() {
    if !lean_available() {
        eprintln!("lean unavailable; skipping actual prototype widening probe");
        return;
    }

    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    assert_runtime_prototype_passes_actual_lean_execution(&sample_path);
}

#[test]
fn theorem_actual_lean_execution_reaches_authorized_ifc_runtime_prototype() {
    if !lean_available() {
        eprintln!("lean unavailable; skipping actual prototype widening probe");
        return;
    }

    let sample_path =
        typed_prototype_sample_path("p10-typed-authorized-fingerprint-declassification.txt");
    assert_runtime_prototype_passes_actual_lean_execution(&sample_path);
}

#[test]
fn theorem_actual_lean_execution_reaches_unauthorized_ifc_runtime_prototype() {
    if !lean_available() {
        eprintln!("lean unavailable; skipping actual prototype widening probe");
        return;
    }

    let sample_path =
        typed_prototype_sample_path("p11-typed-unauthorized-fingerprint-release.txt");
    assert_runtime_prototype_passes_actual_lean_execution(&sample_path);
}

#[test]
fn theorem_actual_lean_execution_reaches_late_join_runtime_prototype() {
    if !lean_available() {
        eprintln!("lean unavailable; skipping actual prototype widening probe");
        return;
    }

    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    assert_runtime_prototype_passes_actual_lean_execution(&sample_path);
}

#[test]
fn theorem_actual_lean_execution_reaches_stale_reconnect_runtime_prototype() {
    if !lean_available() {
        eprintln!("lean unavailable; skipping actual prototype widening probe");
        return;
    }

    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    assert_runtime_prototype_passes_actual_lean_execution(&sample_path);
}
