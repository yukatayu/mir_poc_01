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
    assert_eq!(plan.attach_profile, "debug_trace_layer");
    assert_eq!(
        plan.activation_cut_ref.as_deref(),
        Some("activation_cut#debug_trace_layer_attach")
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
