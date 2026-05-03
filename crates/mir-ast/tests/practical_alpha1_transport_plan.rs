use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_ast::practical_alpha1_transport_plan::{
    PracticalAlpha1TransportPlanErrorKind, build_practical_alpha1_transport_plan_path,
};
use serde_json::{Value, json};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

#[test]
fn practical_transport_plan_builds_local_tcp_fixture() {
    let plan = build_practical_alpha1_transport_plan_path(practical_package_dir(
        "tr-a1-01-local-tcp-accepted",
    ))
    .expect("TR-A1-01 should lower into a practical transport plan");

    assert_eq!(
        plan.transport_plan_scope,
        "practical-alpha1-transport-plan-floor"
    );
    assert_eq!(plan.sample_id, "TR-A1-01");
    assert_eq!(plan.package_id, "tr-a1-01-local-tcp-accepted");
    assert_eq!(plan.transport_surface, "local_tcp");
    assert_eq!(plan.transport_medium, "loopback_tcp");
    assert_eq!(plan.bridge_kind, "tcp_json_socket");
}

#[test]
fn practical_transport_plan_builds_docker_fixture() {
    let plan = build_practical_alpha1_transport_plan_path(practical_package_dir(
        "tr-a1-02-docker-two-node-accepted",
    ))
    .expect("TR-A1-02 should lower into a practical transport plan");

    assert_eq!(plan.sample_id, "TR-A1-02");
    assert_eq!(plan.transport_surface, "docker_compose_tcp");
    assert_eq!(plan.transport_medium, "docker_bridge_tcp");
}

#[test]
fn practical_transport_plan_rejects_front_door_only_package_without_transport_section() {
    let error =
        build_practical_alpha1_transport_plan_path(practical_package_dir("src-01-minimal-world"))
            .expect_err("front-door-only package must not auto-promote into transport plan");

    assert_eq!(
        error.kind,
        PracticalAlpha1TransportPlanErrorKind::MissingTransportSection
    );
}

#[test]
fn practical_transport_plan_rejects_empty_transport_medium() {
    let temp_dir = write_temp_package_from_fixture("tr-a1-01-local-tcp-accepted", |value| {
        value["alpha_local_transport_input"]["request_envelope"]["transport_medium"] = Value::Null;
    });
    let error = build_practical_alpha1_transport_plan_path(&temp_dir)
        .expect_err("transport rows require envelope transport_medium");

    assert_eq!(
        error.kind,
        PracticalAlpha1TransportPlanErrorKind::MalformedTransportInput
    );

    fs::remove_dir_all(temp_dir).expect("cleanup temp package dir");
}

#[test]
fn practical_transport_plan_rejects_mismatched_required_capabilities() {
    let temp_dir = write_temp_package_from_fixture("tr-a1-01-local-tcp-accepted", |value| {
        value["alpha_local_transport_input"]["required_capabilities"] = json!(["PublishRoll"]);
    });
    let error = build_practical_alpha1_transport_plan_path(&temp_dir)
        .expect_err("transport plan must keep required capabilities canonical");

    assert_eq!(
        error.kind,
        PracticalAlpha1TransportPlanErrorKind::MalformedTransportInput
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
        "mir-p-a1-05-transport-plan-{}-{unique}",
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
