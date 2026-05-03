#[path = "support/practical_alpha1_checker_support.rs"]
mod support;

use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_ast::practical_alpha1_runtime_plan::{
    PracticalAlpha1RuntimePlanErrorKind, build_practical_alpha1_runtime_plan_path,
};
use serde_json::{Value, json};
use support::practical_package_dir;

#[test]
fn practical_alpha1_runtime_plan_builds_run_01_checked_local_plan() {
    let plan =
        build_practical_alpha1_runtime_plan_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 practical package should lower into a checked runtime plan");

    assert_eq!(plan.package_id, "run-01-local-sugoroku");
    assert_eq!(plan.world_id, "practical_sugoroku_world");
    assert_eq!(plan.entry_place, "GamePlace[SugorokuGame#1]");
    assert_eq!(plan.queue_kind, "in_process_fifo");
    assert_eq!(plan.initial_envelopes.len(), 1);
    assert_eq!(plan.bootstrap_participants.len(), 2);
    assert_eq!(
        plan.initial_envelopes[0].dispatch_outcome,
        "queued_local_dispatch"
    );
    assert_eq!(
        plan.runtime_plan_scope,
        "practical-alpha1-runtime-plan-floor"
    );
    assert!(
        plan.retained_later_refs
            .contains(&"docker_transport_execution".to_string())
    );
    assert!(!plan.run_local_claimed);
    assert!(!plan.run_docker_claimed);
}

#[test]
fn practical_alpha1_runtime_plan_builds_run_02_with_membership_frontier_advance() {
    let plan = build_practical_alpha1_runtime_plan_path(practical_package_dir(
        "run-02-stale-membership-rejected",
    ))
    .expect("RUN-02 practical package should lower into a checked runtime plan");

    assert_eq!(plan.package_id, "run-02-stale-membership-rejected");
    assert_eq!(plan.initial_envelopes.len(), 1);
    assert_eq!(
        plan.initial_envelopes[0].dispatch_outcome,
        "queued_local_dispatch"
    );
    assert_eq!(plan.pre_dispatch_membership_advances.len(), 1);
    assert_eq!(plan.pre_dispatch_membership_advances[0].principal, "Carol");
}

#[test]
fn practical_alpha1_runtime_plan_rejects_packages_that_fail_checker_floor() {
    let error = build_practical_alpha1_runtime_plan_path(practical_package_dir(
        "chk-var-02-precondition-strengthening-rejected",
    ))
    .expect_err("checker-rejected package must not lower into a runtime plan");

    assert_eq!(
        error.kind,
        PracticalAlpha1RuntimePlanErrorKind::RejectedByChecker
    );
}

#[test]
fn practical_alpha1_runtime_plan_rejects_front_door_only_package_without_runtime_section() {
    let error =
        build_practical_alpha1_runtime_plan_path(practical_package_dir("src-01-minimal-world"))
            .expect_err("front-door-only package must not auto-promote into runtime-plan floor");

    assert_eq!(
        error.kind,
        PracticalAlpha1RuntimePlanErrorKind::MissingRuntimeSection
    );
}

#[test]
fn practical_alpha1_runtime_plan_rejects_malformed_runtime_queue_kind() {
    let temp_dir = write_temp_package_from_fixture("run-01-local-sugoroku", |value| {
        value["alpha_local_runtime_input"]["queue_kind"] = json!("cross_process_fifo");
    });
    let error = build_practical_alpha1_runtime_plan_path(&temp_dir)
        .expect_err("unsupported queue kind must be rejected before runtime-plan build");

    assert_eq!(
        error.kind,
        PracticalAlpha1RuntimePlanErrorKind::MalformedRuntimeInput
    );
    assert!(error.detail.contains("queue_kind = in_process_fifo"));

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

fn write_temp_package_from_fixture(
    fixture_name: &str,
    mutate: impl FnOnce(&mut Value),
) -> std::path::PathBuf {
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
        "mir-p-a1-03-runtime-plan-{}-{unique}",
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
