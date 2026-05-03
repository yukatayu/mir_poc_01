use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_ast::practical_alpha1_save_load_plan::{
    PracticalAlpha1SaveLoadPlanErrorKind, build_practical_alpha1_save_load_plan_path,
};
use serde_json::{Value, json};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

#[test]
fn practical_save_load_plan_matches_sl_a1_01_shape() {
    let plan = build_practical_alpha1_save_load_plan_path(practical_package_dir(
        "sl-a1-01-local-save-load-resume",
    ))
    .expect("SL-A1-01 package should lower into the practical save/load plan floor");

    assert_eq!(plan.package_id, "sl-a1-01-local-save-load-resume");
    assert_eq!(
        plan.save_load_plan_scope,
        "practical-alpha1-save-load-plan-floor"
    );
    assert_eq!(plan.sample_id, "SL-A1-01");
    assert_eq!(plan.scenario_kind, "resume_one_dispatch");
    assert_eq!(plan.post_restore_membership_advances.len(), 0);
    assert_eq!(plan.resumed_envelope.envelope_id, "roll_request#2");
}

#[test]
fn practical_save_load_plan_matches_sl_a1_02_shape() {
    let plan = build_practical_alpha1_save_load_plan_path(practical_package_dir(
        "sl-a1-02-local-load-stale-membership-rejected",
    ))
    .expect("SL-A1-02 package should lower into the practical save/load plan floor");

    assert_eq!(
        plan.package_id,
        "sl-a1-02-local-load-stale-membership-rejected"
    );
    assert_eq!(plan.sample_id, "SL-A1-02");
    assert_eq!(plan.scenario_kind, "reject_stale_membership");
    assert_eq!(plan.post_restore_membership_advances.len(), 1);
    assert_eq!(plan.post_restore_membership_advances[0].principal, "Carol");
}

#[test]
fn practical_save_load_plan_rejects_runtime_only_package_without_save_load_section() {
    let error =
        build_practical_alpha1_save_load_plan_path(practical_package_dir("run-01-local-sugoroku"))
            .expect_err(
                "runtime-only package must not auto-lower into the practical save/load floor",
            );

    assert_eq!(
        error.kind,
        PracticalAlpha1SaveLoadPlanErrorKind::MissingSaveLoadSection
    );
}

#[test]
fn practical_save_load_plan_rejects_stale_membership_case_without_post_restore_advance() {
    let temp_dir =
        write_temp_package_from_fixture("sl-a1-02-local-load-stale-membership-rejected", |value| {
            value["alpha_local_save_load_input"]["post_restore_membership_advances"] = json!([]);
        });
    let error = build_practical_alpha1_save_load_plan_path(&temp_dir)
        .expect_err("stale-membership scenario must require a post-restore membership advance");

    assert_eq!(
        error.kind,
        PracticalAlpha1SaveLoadPlanErrorKind::MalformedSaveLoadInput
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
        "mir-p-a1-07-save-load-plan-{}-{unique}",
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
