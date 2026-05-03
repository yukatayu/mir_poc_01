use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::practical_alpha1_hotplug::{
    PracticalAlpha1HotPlugErrorKind, attach_practical_alpha1_package_path,
};
use serde_json::{Value, json};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

fn read_expected_report(name: &str) -> Value {
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
fn practical_hotplug_matches_debug_attach_row() {
    let report =
        attach_practical_alpha1_package_path(practical_package_dir("hp-a1-01-debug-layer-attach"))
            .expect("HP-A1-01 practical package should attach through the practical hotplug floor");
    let expected = read_expected_report("hp-a1-01-debug-layer-attach.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize report"),
        expected
    );
    assert_eq!(report.hotplug_scope, "practical-alpha1-hotplug-floor");
    assert_eq!(
        report.hotplug_plan_scope,
        "practical-alpha1-hotplug-plan-floor"
    );
    assert_eq!(report.sample_id, "HP-A1-01");
    assert_eq!(report.attach_profile, "debug_trace_layer");
    assert_eq!(report.terminal_outcome, "accepted");
    assert!(report.reason_family.is_none());
    assert!(report.manifest_checks.iter().all(|check| check.passed));
}

#[test]
fn practical_hotplug_matches_non_admin_debug_rejection_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-02-non-admin-debug-rejected",
    ))
    .expect("HP-A1-02 practical package should return explicit rejection report");
    let expected = read_expected_report("hp-a1-02-non-admin-debug-rejected.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize report"),
        expected
    );
    assert_eq!(report.sample_id, "HP-A1-02");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("authorization"));
    assert_eq!(
        report.hotplug_runtime_report.verdict.verdict_kind,
        "rejected"
    );
    assert!(
        report
            .rejection_reason_refs
            .iter()
            .any(|reason| reason.starts_with("attach_capability_missing:"))
    );
}

#[test]
fn practical_hotplug_matches_auth_contract_update_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-03-auth-layer-contract-update",
    ))
    .expect("HP-A1-03 practical package should use explicit contract update path");
    let expected = read_expected_report("hp-a1-03-auth-layer-contract-update.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize report"),
        expected
    );
    assert_eq!(report.sample_id, "HP-A1-03");
    assert_eq!(report.terminal_outcome, "accepted_contract_update");
    assert_eq!(
        report.activation_cut_ref.as_deref(),
        Some("activation_cut#auth_contract_update")
    );
    let preview = report
        .runtime_preview
        .as_ref()
        .expect("auth row should expose runtime preview");
    assert_eq!(preview.terminal_outcome, "accepted_contract_update");
}

#[test]
fn practical_hotplug_matches_ratelimit_preview_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-04-ratelimit-declared-failure",
    ))
    .expect("HP-A1-04 practical package should return declared-failure preview report");
    let expected = read_expected_report("hp-a1-04-ratelimit-declared-failure.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize report"),
        expected
    );
    assert_eq!(report.sample_id, "HP-A1-04");
    assert_eq!(report.terminal_outcome, "accepted");
    let preview = report
        .runtime_preview
        .as_ref()
        .expect("rate-limit row should expose runtime preview");
    assert!(
        preview.reason_refs.contains(&"RateLimited".to_string()),
        "preview should keep the declared failure row explicit"
    );
}

#[test]
fn practical_hotplug_matches_incompatible_patch_rejection_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-05-incompatible-patch-rejected",
    ))
    .expect("HP-A1-05 practical package should return explicit incompatibility rejection");
    let expected = read_expected_report("hp-a1-05-incompatible-patch-rejected.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize report"),
        expected
    );
    assert_eq!(report.sample_id, "HP-A1-05");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("compatibility"));
}

#[test]
fn practical_hotplug_matches_stale_membership_attach_rejection_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-04b1-stale-membership-attach-rejected",
    ))
    .expect("freshness negative row should return an explicit hotplug rejection");

    assert_eq!(report.sample_id, "HP-A1-04B1");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(
        report.reason_family.as_deref(),
        Some("membership_freshness")
    );
    assert!(
        report
            .rejection_reason_refs
            .contains(&"membership_epoch_drift".to_string())
    );
}

#[test]
fn practical_hotplug_matches_missing_witness_attach_rejection_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-04b2-missing-witness-attach-rejected",
    ))
    .expect("missing-witness row should return an explicit hotplug rejection");

    assert_eq!(report.sample_id, "HP-A1-04B2");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("witness"));
    assert!(
        report
            .rejection_reason_refs
            .iter()
            .any(|reason| reason.starts_with("missing_witness:"))
    );
}

#[test]
fn practical_hotplug_matches_object_package_attach_preview_row() {
    let report = attach_practical_alpha1_package_path(practical_package_dir(
        "hp-a1-06-object-package-attach",
    ))
    .expect("HP-A1-06 should return a non-final object attach preview report");

    assert_eq!(report.sample_id, "HP-A1-06");
    assert_eq!(report.package_kind, "object");
    assert_eq!(report.terminal_outcome, "accepted_object_attach_preview");
    assert!(report.object_attach_preview.is_some());
    assert!(!report.object_attach_claimed);
}

#[test]
fn practical_hotplug_rejects_front_door_only_package_without_hotplug_section() {
    let error = attach_practical_alpha1_package_path(practical_package_dir("src-01-minimal-world"))
        .expect_err("front-door-only package must not auto-run through practical hotplug");

    assert_eq!(error.kind, PracticalAlpha1HotPlugErrorKind::HotPlugPlan);
}

#[test]
fn practical_hotplug_rejection_is_driven_by_freshness_fields_not_sample_id() {
    let temp_dir =
        write_temp_package_from_fixture("hp-a1-04b1-stale-membership-attach-rejected", |value| {
            value["alpha_local_hotplug_input"]
                .as_object_mut()
                .expect("hotplug input object")
                .remove("pre_attach_membership_advances");
        });
    let report = attach_practical_alpha1_package_path(&temp_dir)
        .expect("mutated stale-membership fixture should still produce a report");

    assert_ne!(
        report.reason_family.as_deref(),
        Some("membership_freshness")
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

#[test]
fn practical_hotplug_rejection_is_driven_by_witness_contents_not_sample_id() {
    let temp_dir =
        write_temp_package_from_fixture("hp-a1-04b2-missing-witness-attach-rejected", |value| {
            value["alpha_local_hotplug_input"]["witness_refs"] =
                json!(["membership_frontier_snapshot#practical_hotplug"]);
        });
    let report = attach_practical_alpha1_package_path(&temp_dir)
        .expect("mutated witness fixture should still produce a report");

    assert_ne!(report.reason_family.as_deref(), Some("witness"));

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

#[test]
fn practical_hotplug_rejects_auth_profile_without_contract_update_ref() {
    let temp_dir =
        write_temp_package_from_fixture("hp-a1-03-auth-layer-contract-update", |value| {
            value["alpha_local_hotplug_input"]
                .as_object_mut()
                .expect("hotplug input object")
                .remove("contract_update_ref");
        });
    let error = attach_practical_alpha1_package_path(&temp_dir)
        .expect_err("auth profile without contract update ref must not enter attach floor");

    assert_eq!(error.kind, PracticalAlpha1HotPlugErrorKind::HotPlugPlan);

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
        "mir-p-a1-04a-hotplug-runtime-{}-{unique}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).unwrap_or_else(|error| {
        panic!("failed to create temp dir {}: {error}", temp_dir.display())
    });
    fs::write(
        temp_dir.join("package.mir.json"),
        serde_json::to_string_pretty(&json!(value)).expect("serialize mutated practical package"),
    )
    .unwrap_or_else(|error| {
        panic!(
            "failed to write temp package {}: {error}",
            temp_dir.display()
        )
    });
    temp_dir
}
