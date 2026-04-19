use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use mir_semantics::{load_fixture_from_path, static_gate_detailed};

#[path = "../examples/support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[path = "../examples/support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[path = "../examples/support/current_l2_lean_theorem_stub_support.rs"]
mod current_l2_lean_theorem_stub_support;
#[path = "../examples/support/current_l2_proof_notebook_review_unit_support.rs"]
mod current_l2_proof_notebook_review_unit_support;
#[path = "../examples/support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_formal_hook_support::build_formal_hook_from_static_gate_artifact;
use current_l2_lean_theorem_stub_support::build_lean_theorem_stub_artifacts;
use current_l2_proof_notebook_review_unit_support::build_proof_notebook_review_unit_artifacts;
use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
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

#[test]
fn static_lean_theorem_stubs_pass_actual_lean_execution_when_toolchain_is_available() {
    if !lean_available() {
        eprintln!("lean unavailable; skipping actual probe");
        return;
    }

    let path = fixture_path("e5-underdeclared-lineage.json");
    let fixture = load_fixture_from_path(&path).unwrap();
    let gate = static_gate_detailed(&fixture);
    let artifact = build_detached_static_gate_artifact(path, &fixture, &gate);
    let formal_hook = build_formal_hook_from_static_gate_artifact(&artifact).unwrap();
    let review_units = build_proof_notebook_review_unit_artifacts(&formal_hook).unwrap();
    let stubs = build_lean_theorem_stub_artifacts(&review_units).unwrap();

    let probe_path = unique_temp_lean_path("current-l2-actual-lean-probe");
    let source_text = stubs
        .iter()
        .map(|stub| stub.source_text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&probe_path, source_text).unwrap();

    let output = Command::new("lean").arg(&probe_path).output().unwrap();
    let _ = fs::remove_file(&probe_path);

    assert!(
        output.status.success(),
        "expected Lean execution to succeed, stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
}
