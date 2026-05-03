use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::practical_alpha1_transport::{
    PracticalAlpha1TransportErrorKind, run_practical_alpha1_transport_path,
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
fn practical_transport_matches_local_tcp_accept_row() {
    let report =
        run_practical_alpha1_transport_path(practical_package_dir("tr-a1-01-local-tcp-accepted"))
            .expect("TR-A1-01 should return a practical transport report");
    let expected = read_expected_report("tr-a1-01-local-tcp-accepted.expected.json");

    assert_eq!(
        serde_json::to_value(&report).expect("serialize report"),
        expected
    );
    assert_eq!(report.sample_id, "TR-A1-01");
    assert_eq!(report.transport_surface, "local_tcp");
    assert_eq!(report.terminal_outcome, "accepted");
}

#[test]
fn practical_transport_matches_stale_membership_rejection_row() {
    let report = run_practical_alpha1_transport_path(practical_package_dir(
        "tr-a1-03-stale-membership-rejected",
    ))
    .expect("TR-A1-03 should return an explicit practical transport rejection");

    assert_eq!(report.sample_id, "TR-A1-03");
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
fn practical_transport_matches_missing_capability_rejection_row() {
    let report = run_practical_alpha1_transport_path(practical_package_dir(
        "tr-a1-04-missing-capability-rejected",
    ))
    .expect("TR-A1-04 should return an explicit capability rejection");

    assert_eq!(report.sample_id, "TR-A1-04");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("capability"));
    assert!(
        report
            .rejection_reason_refs
            .iter()
            .any(|reason| reason.starts_with("missing_capability:"))
    );
}

#[test]
fn practical_transport_matches_missing_witness_rejection_row() {
    let report = run_practical_alpha1_transport_path(practical_package_dir(
        "tr-a1-05-missing-witness-rejected",
    ))
    .expect("TR-A1-05 should return an explicit witness rejection");

    assert_eq!(report.sample_id, "TR-A1-05");
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
fn practical_transport_matches_observer_safe_route_trace_row() {
    let report = run_practical_alpha1_transport_path(practical_package_dir(
        "tr-a1-06-observer-safe-route-trace",
    ))
    .expect("TR-A1-06 should return an observer-safe route trace report");

    assert_eq!(report.sample_id, "TR-A1-06");
    assert_eq!(report.terminal_outcome, "accepted");
    assert_eq!(report.observer_route_trace.len(), 2);
    for row in &report.observer_route_trace {
        assert_eq!(row.redaction, "observer_safe_route_trace");
    }
}

#[test]
fn practical_transport_matches_auth_lane_row() {
    let report =
        run_practical_alpha1_transport_path(practical_package_dir("tr-a1-07-auth-lane-preserved"))
            .expect("TR-A1-07 should keep auth evidence separate from transport metadata");

    assert_eq!(report.sample_id, "TR-A1-07");
    assert_eq!(report.terminal_outcome, "accepted");
    let auth_lane = report.auth_lane.expect("auth lane");
    assert!(auth_lane.auth_present);
    assert!(auth_lane.preserved_separately);
    assert!(
        auth_lane
            .bindings
            .contains(&"transport=network_transport_lane".to_string())
    );
}

#[test]
fn practical_transport_rejects_front_door_only_package_without_transport_section() {
    let error = run_practical_alpha1_transport_path(practical_package_dir("src-01-minimal-world"))
        .expect_err("front-door-only package must not auto-run through practical transport");

    assert_eq!(error.kind, PracticalAlpha1TransportErrorKind::TransportPlan);
}

#[test]
fn practical_transport_rejects_unknown_destination_before_acceptance() {
    let temp_dir = write_temp_package_from_fixture("tr-a1-01-local-tcp-accepted", |value| {
        value["alpha_local_transport_input"]["request_envelope"]["to_place"] =
            json!("UnknownPlace[Nowhere]");
    });
    let report = run_practical_alpha1_transport_path(&temp_dir)
        .expect("mutated transport fixture should still return a report");

    assert_eq!(report.terminal_outcome, "rejected");
    assert!(
        report
            .rejection_reason_refs
            .contains(&"destination_place_unregistered".to_string())
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
        "mir-p-a1-05-transport-runtime-{}-{unique}",
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
