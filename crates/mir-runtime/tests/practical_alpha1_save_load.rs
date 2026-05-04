use std::{fs, path::PathBuf};

use mir_runtime::practical_alpha1_save_load::{
    PracticalAlpha1SaveLoadErrorKind, PracticalAlpha1SaveLoadReport,
    run_practical_alpha1_save_load_path,
};
use serde_json::Value;

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

fn read_expected_report(name: &str) -> PracticalAlpha1SaveLoadReport {
    let path = practical_root().join("expected").join(name);
    let text = fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("failed to read expected report {}: {error}", path.display())
    });
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!(
            "failed to parse expected report {}: {error}",
            path.display()
        )
    })
}

#[test]
fn practical_save_load_matches_sl_a1_01_expected_report() {
    let observed = run_practical_alpha1_save_load_path(practical_package_dir(
        "sl-a1-01-local-save-load-resume",
    ))
    .expect("SL-A1-01 package should run through the practical save/load floor");
    let expected = read_expected_report("sl-a1-01-local-save-load-resume.expected.json");

    assert_eq!(observed, expected);
    assert_eq!(observed.terminal_outcome, "accepted");
    assert!(observed.state_roundtrip_equal);
    assert_eq!(observed.resumed_dispatch_records.len(), 1);
}

#[test]
fn practical_save_load_matches_sl_a1_02_expected_report() {
    let observed = run_practical_alpha1_save_load_path(practical_package_dir(
        "sl-a1-02-local-load-stale-membership-rejected",
    ))
    .expect("SL-A1-02 package should run through the practical save/load floor");
    let expected =
        read_expected_report("sl-a1-02-local-load-stale-membership-rejected.expected.json");

    assert_eq!(observed, expected);
    assert_eq!(observed.terminal_outcome, "rejected");
    assert_eq!(
        observed.resumed_dispatch_records[0].dispatch_outcome,
        "rejected_stale_membership"
    );
    assert_eq!(observed.retention_scope, "report_local_inventory");
    assert_eq!(
        observed
            .retained_artifacts
            .iter()
            .map(|artifact| artifact.artifact_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "saved_membership_frontier#SL-A1-02",
            "restored_membership_frontier#SL-A1-02",
            "stale_membership_reject#SL-A1-02",
            "resumed_event_dag#SL-A1-02",
        ]
    );
}

#[test]
fn practical_save_load_rejects_package_without_save_load_section() {
    let error = run_practical_alpha1_save_load_path(practical_package_dir("run-01-local-sugoroku"))
        .expect_err("runtime-only package must not run through the practical save/load floor");

    assert_eq!(error.kind, PracticalAlpha1SaveLoadErrorKind::SaveLoadPlan);
}

#[test]
fn practical_save_load_rejects_checker_rejected_package() {
    let temp_dir = write_temp_package_from_fixture("sl-a1-01-local-save-load-resume", |value| {
        value["alpha_local_checker_input"]["case"] = serde_json::json!({
            "kind": "orphan_receive",
            "sample_id": "CHK-CUT-01-temp-reject",
            "receive_event": "receive_roll#temp",
            "missing_predecessor": "send_roll#temp"
        });
    });
    let error = run_practical_alpha1_save_load_path(&temp_dir)
        .expect_err("checker-rejected package must not enter practical save/load execution");

    assert_eq!(error.kind, PracticalAlpha1SaveLoadErrorKind::SaveLoadPlan);

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

fn write_temp_package_from_fixture(fixture_name: &str, mutate: impl FnOnce(&mut Value)) -> PathBuf {
    let package_path = practical_package_dir(fixture_name).join("package.mir.json");
    let text = fs::read_to_string(&package_path).unwrap_or_else(|error| {
        panic!("failed to read fixture {}: {error}", package_path.display())
    });
    let mut value: Value = serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!(
            "failed to parse fixture {}: {error}",
            package_path.display()
        )
    });
    mutate(&mut value);

    let temp_dir = std::env::temp_dir().join(format!(
        "mir-p-a1-07-save-load-runtime-{}",
        std::process::id()
    ));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).expect("cleanup previous temp dir");
    }
    fs::create_dir_all(&temp_dir).unwrap_or_else(|error| {
        panic!("failed to create temp dir {}: {error}", temp_dir.display())
    });
    fs::write(
        temp_dir.join("package.mir.json"),
        serde_json::to_string_pretty(&value).expect("serialize mutated practical package"),
    )
    .unwrap_or_else(|error| {
        panic!(
            "failed to write temp package {}: {error}",
            temp_dir.display()
        )
    });
    temp_dir
}
