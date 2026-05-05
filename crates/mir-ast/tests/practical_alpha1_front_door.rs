#[path = "support/practical_alpha1_front_door_support.rs"]
mod practical_alpha1_front_door_support;

use mir_ast::practical_alpha1::{
    PracticalAlpha1FrontDoorErrorKind, load_practical_alpha1_package_path,
    parse_practical_alpha1_package_text, practical_alpha1_front_door_manifest,
};

use practical_alpha1_front_door_support::{
    load_expected_front_door_error, load_expected_front_door_summary,
    lower_package_to_fixture_summary, practical_expected_dir, practical_expected_file,
    practical_package_dir, practical_package_file,
};

#[test]
fn practical_alpha1_front_door_manifest_keeps_nonfinal_package_json_cut() {
    let manifest = practical_alpha1_front_door_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "practical_alpha1_nonfinal_package_json_front_door"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &[
            "package.mir.json",
            "world",
            "places",
            "fallback_chains",
            "layers",
            "manifest",
            "native",
            "alpha_local_checker_input",
            "alpha_local_runtime_input",
            "alpha_local_hotplug_input",
            "alpha_local_transport_input",
            "alpha_local_save_load_input",
            "alpha_local_host_io_input",
        ]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &[
            "mir_ast_practical_alpha1_module",
            "practical_alpha1_front_door_tests"
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "final_textual_alpha_source_grammar",
            "typed_ir_checker_integration",
            "runtime_plan_execution"
        ]
    );
}

#[test]
fn practical_alpha1_front_door_loads_src_01_minimal_world_fixture() {
    let package = load_practical_alpha1_package_path(practical_package_dir("src-01-minimal-world"))
        .expect("src-01 package directory should load");
    let actual = lower_package_to_fixture_summary(&package);
    let expected = load_expected_front_door_summary("src-01-minimal-world.expected.json")
        .expect("src-01 expected summary should load");

    assert_eq!(actual, expected);
}

#[test]
fn practical_alpha1_front_door_loads_src_02_fallback_chain_fixture() {
    let package =
        load_practical_alpha1_package_path(practical_package_file("src-02-fallback-chain"))
            .expect("src-02 package file should load");
    let actual = lower_package_to_fixture_summary(&package);
    let expected = load_expected_front_door_summary("src-02-fallback-chain.expected.json")
        .expect("src-02 expected summary should load");

    assert_eq!(actual, expected);
}

#[test]
fn practical_alpha1_front_door_loads_src_03_layer_attach_fixture() {
    let package = load_practical_alpha1_package_path(practical_package_dir("src-03-layer-debug"))
        .expect("src-03 package directory should load");
    let actual = lower_package_to_fixture_summary(&package);
    let expected = load_expected_front_door_summary("src-03-layer-debug.expected.json")
        .expect("src-03 expected summary should load");

    assert_eq!(actual, expected);
}

#[test]
fn practical_alpha1_front_door_loads_src_04_manifest_fixture() {
    let package =
        load_practical_alpha1_package_path(practical_package_file("src-04-layer-manifest"))
            .expect("src-04 package file should load");
    let actual = lower_package_to_fixture_summary(&package);
    let expected = load_expected_front_door_summary("src-04-layer-manifest.expected.json")
        .expect("src-04 expected summary should load");

    assert_eq!(actual, expected);
}

#[test]
fn practical_alpha1_front_door_rejects_src_05_invalid_syntax() {
    let error = load_practical_alpha1_package_path(practical_package_dir("src-05-invalid-syntax"))
        .expect_err("src-05 invalid JSON should be rejected");
    let expected = load_expected_front_door_error("src-05-invalid-syntax.expected.json")
        .expect("src-05 expected error summary should load");

    assert!(!expected.accepted);
    assert_eq!(format!("{:?}", error.kind), expected.error_kind);
}

#[test]
fn practical_alpha1_front_door_rejects_existing_non_package_json_file() {
    let error = load_practical_alpha1_package_path(practical_expected_file(
        "src-01-minimal-world.expected.json",
    ))
    .expect_err("non-package JSON file should be rejected before parse");

    assert_eq!(
        error.kind,
        PracticalAlpha1FrontDoorErrorKind::MissingPackageFile
    );
}

#[test]
fn practical_alpha1_front_door_rejects_directory_missing_package_file() {
    let error = load_practical_alpha1_package_path(practical_expected_dir())
        .expect_err("directory without package.mir.json should be rejected");

    assert_eq!(
        error.kind,
        PracticalAlpha1FrontDoorErrorKind::MissingPackageFile
    );
}

#[test]
fn practical_alpha1_front_door_rejects_unsupported_format_version() {
    let error = parse_practical_alpha1_package_text(
        r#"{
  "format_version": "mirrorea-practical-alpha1-v999",
  "package_id": "schema-bad-format",
  "package_kind": "world",
  "world": {
    "id": "demo_world",
    "entry_place": "lobby"
  }
}"#,
    )
    .expect_err("unsupported format version should be rejected");

    assert_eq!(error.kind, PracticalAlpha1FrontDoorErrorKind::SchemaDecode);
}

#[test]
fn practical_alpha1_front_door_rejects_unsupported_package_kind() {
    let error = parse_practical_alpha1_package_text(
        r#"{
  "format_version": "mirrorea-practical-alpha1-v0",
  "package_id": "schema-bad-kind",
  "package_kind": "widget"
}"#,
    )
    .expect_err("unsupported package kind should be rejected");

    assert_eq!(error.kind, PracticalAlpha1FrontDoorErrorKind::SchemaDecode);
}

#[test]
fn practical_alpha1_front_door_rejects_world_package_missing_world_block() {
    let error = parse_practical_alpha1_package_text(
        r#"{
  "format_version": "mirrorea-practical-alpha1-v0",
  "package_id": "schema-missing-world",
  "package_kind": "world"
}"#,
    )
    .expect_err("world package without world block should be rejected");

    assert_eq!(error.kind, PracticalAlpha1FrontDoorErrorKind::SchemaDecode);
}
