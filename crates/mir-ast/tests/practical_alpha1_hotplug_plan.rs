use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_ast::practical_alpha1_hotplug_plan::{
    PracticalAlpha1HotPlugPlanErrorKind, build_practical_alpha1_hotplug_plan_path,
};
use serde_json::{Value, json};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

#[test]
fn practical_hotplug_plan_builds_debug_layer_attach_fixture() {
    let plan = build_practical_alpha1_hotplug_plan_path(practical_package_dir(
        "hp-a1-01-debug-layer-attach",
    ))
    .expect("HP-A1-01 should lower into a practical hotplug plan");

    assert_eq!(
        plan.hotplug_plan_scope,
        "practical-alpha1-hotplug-plan-floor"
    );
    assert_eq!(plan.sample_id, "HP-A1-01");
    assert_eq!(plan.package_id, "hp-a1-01-debug-layer-attach");
    assert_eq!(plan.package_kind, "layer");
    assert_eq!(plan.operation_kind, "attach");
    assert_eq!(plan.attach_profile, "debug_trace_layer");
    assert_eq!(
        plan.activation_cut_ref.as_deref(),
        Some("activation_cut#debug_trace_layer_attach")
    );
}

#[test]
fn practical_hotplug_plan_carries_freshness_claims_for_attach_negative_rows() {
    let plan = build_practical_alpha1_hotplug_plan_path(practical_package_dir(
        "hp-a1-04b1-stale-membership-attach-rejected",
    ))
    .expect("freshness negative row should still lower into a practical hotplug plan");

    assert_eq!(plan.sample_id, "HP-A1-04B1");
    assert_eq!(plan.package_kind, "layer");
    assert_eq!(plan.membership_epoch, 0);
    assert_eq!(plan.member_incarnation, 0);
    assert_eq!(plan.pre_attach_membership_advances.len(), 1);
}

#[test]
fn practical_hotplug_plan_allows_object_package_attach_preview_fixture() {
    let plan = build_practical_alpha1_hotplug_plan_path(practical_package_dir(
        "hp-a1-06-object-package-attach",
    ))
    .expect("HP-A1-06 should lower into a practical hotplug plan");

    assert_eq!(plan.sample_id, "HP-A1-06");
    assert_eq!(plan.package_kind, "object");
    assert_eq!(plan.attach_profile, "placeholder_avatar_object_package");
    assert_eq!(
        plan.attachpoint_ref,
        "AttachPoint[AlphaRoom#1::AvatarRuntime]"
    );
}

#[test]
fn practical_hotplug_plan_lowers_detach_minimal_contract_fixture() {
    let plan = build_practical_alpha1_hotplug_plan_path(practical_package_dir(
        "hp-a1-07-detach-minimal-contract",
    ))
    .expect("HP-A1-07 should lower into a practical hotplug plan");

    assert_eq!(plan.sample_id, "HP-A1-07");
    assert_eq!(plan.package_kind, "layer");
    assert_eq!(plan.operation_kind, "detach");
    assert!(plan.activation_cut_ref.is_none());
    assert_eq!(
        plan.detach_boundary_ref.as_deref(),
        Some("detach_boundary#alpha_local_hotplug_minimal_contract")
    );
}

#[test]
fn practical_hotplug_plan_rejects_package_without_hotplug_section() {
    let error =
        build_practical_alpha1_hotplug_plan_path(practical_package_dir("src-04-layer-manifest"))
            .expect_err("front-door-only package must not auto-promote into hotplug plan");

    assert_eq!(
        error.kind,
        PracticalAlpha1HotPlugPlanErrorKind::MissingHotPlugSection
    );
}

#[test]
fn practical_hotplug_plan_rejects_auth_profile_without_contract_update_ref() {
    let temp_dir =
        write_temp_package_from_fixture("hp-a1-03-auth-layer-contract-update", |value| {
            value["alpha_local_hotplug_input"]
                .as_object_mut()
                .expect("hotplug input object")
                .remove("contract_update_ref");
        });
    let error = build_practical_alpha1_hotplug_plan_path(&temp_dir)
        .expect_err("auth profile without contract update ref must be rejected");

    assert_eq!(
        error.kind,
        PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

#[test]
fn practical_hotplug_plan_rejects_object_detach_preview_requests() {
    let temp_dir = write_temp_package_from_fixture("hp-a1-06-object-package-attach", |value| {
        value["alpha_local_hotplug_input"]["operation_kind"] = json!("detach");
    });
    let error = build_practical_alpha1_hotplug_plan_path(&temp_dir)
        .expect_err("object detach requests must remain outside the current practical floor");

    assert_eq!(
        error.kind,
        PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

#[test]
fn practical_hotplug_plan_rejects_detach_without_boundary_ref() {
    let temp_dir = write_temp_package_from_fixture("hp-a1-07-detach-minimal-contract", |value| {
        value["alpha_local_hotplug_input"]
            .as_object_mut()
            .expect("hotplug input object")
            .remove("detach_boundary_ref");
    });
    let error = build_practical_alpha1_hotplug_plan_path(&temp_dir)
        .expect_err("detach requests without detach_boundary_ref must be rejected");

    assert_eq!(
        error.kind,
        PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput
    );

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
        "mir-p-a1-04a-hotplug-plan-{}-{unique}",
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
