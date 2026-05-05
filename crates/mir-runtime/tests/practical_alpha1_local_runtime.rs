use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::practical_alpha1_local_runtime::{
    PracticalAlpha1LocalRuntimeErrorKind, PracticalAlpha1LocalRuntimeReport,
    run_practical_alpha1_local_runtime_path,
};
use serde_json::{Value, json};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

fn read_expected_report(name: &str) -> PracticalAlpha1LocalRuntimeReport {
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
fn practical_local_runtime_matches_run_01_expected_report() {
    let observed =
        run_practical_alpha1_local_runtime_path(practical_package_dir("run-01-local-sugoroku"))
            .expect(
                "RUN-01 practical package should run through the practical local runtime floor",
            );
    let expected = read_expected_report("run-01-local-sugoroku.expected.json");

    assert_eq!(observed, expected);
    assert_eq!(observed.terminal_outcome, "accepted");
    assert_eq!(observed.dispatch_records.len(), 1);
    assert!(!observed.visible_history.is_empty());
}

#[test]
fn practical_local_runtime_matches_run_02_expected_report() {
    let observed = run_practical_alpha1_local_runtime_path(practical_package_dir(
        "run-02-stale-membership-rejected",
    ))
    .expect("RUN-02 practical package should run through the practical local runtime floor");
    let expected = read_expected_report("run-02-stale-membership-rejected.expected.json");

    assert_eq!(observed, expected);
    assert_eq!(observed.terminal_outcome, "rejected");
    assert_eq!(
        observed.reason_family.as_deref(),
        Some("membership_freshness")
    );
    assert!(observed.visible_history.is_empty());
}

#[test]
fn practical_local_runtime_matches_run_03_expected_report() {
    let observed = run_practical_alpha1_local_runtime_path(practical_package_dir(
        "run-03-missing-capability-rejected",
    ))
    .expect("RUN-03 practical package should run through the practical local runtime floor");
    let expected = read_expected_report("run-03-missing-capability-rejected.expected.json");

    assert_eq!(observed, expected);
    assert_eq!(
        observed.dispatch_records[0].dispatch_outcome,
        "rejected_missing_capability"
    );
    assert_eq!(observed.reason_family.as_deref(), Some("authorization"));
}

#[test]
fn practical_local_runtime_matches_run_04_expected_report() {
    let observed = run_practical_alpha1_local_runtime_path(practical_package_dir(
        "run-04-missing-witness-rejected",
    ))
    .expect("RUN-04 practical package should run through the practical local runtime floor");
    let expected = read_expected_report("run-04-missing-witness-rejected.expected.json");

    assert_eq!(observed, expected);
    assert_eq!(
        observed.dispatch_records[0].dispatch_outcome,
        "rejected_missing_witness"
    );
    assert_eq!(observed.reason_family.as_deref(), Some("witness"));
}

#[test]
fn practical_local_runtime_rejects_front_door_only_package_without_runtime_section() {
    let error =
        run_practical_alpha1_local_runtime_path(practical_package_dir("src-01-minimal-world"))
            .expect_err(
                "front-door-only package must not auto-run through the practical local runtime",
            );

    assert_eq!(
        error.kind,
        PracticalAlpha1LocalRuntimeErrorKind::RuntimePlan
    );
}

#[test]
fn practical_local_runtime_rejects_checker_rejected_runtime_package() {
    let temp_dir = write_temp_package_from_fixture("run-01-local-sugoroku", |value| {
        value["alpha_local_checker_input"]["case"] = json!({
            "kind": "precondition_strengthening",
            "sample_id": "CHK-VAR-02-temp-checker-reject",
            "base_precondition": "member",
            "layer_precondition": "admin"
        });
    });
    let error = run_practical_alpha1_local_runtime_path(&temp_dir)
        .expect_err("checker-rejected runtime package must not enter practical local runtime");

    assert_eq!(
        error.kind,
        PracticalAlpha1LocalRuntimeErrorKind::RuntimePlan
    );
    assert!(error.detail.contains("RejectedByChecker"));

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

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before unix epoch")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!(
        "mir-p-a1-03-local-runtime-{}-{unique}",
        std::process::id()
    ));
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
