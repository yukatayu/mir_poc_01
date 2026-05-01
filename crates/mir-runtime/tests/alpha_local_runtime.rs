use std::{fs, path::PathBuf};

use mir_runtime::alpha_local_runtime::{
    build_local_sugoroku_runtime_report, build_stale_membership_rejection_report,
};
use serde_json::Value;

fn sample_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/alpha/local-runtime")
}

fn read_sidecar(name: &str) -> Value {
    let path = sample_root().join(name);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read sidecar {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse sidecar {}: {error}", path.display()))
}

#[test]
fn local_sugoroku_report_matches_positive_sample_contract() {
    let report = build_local_sugoroku_runtime_report().expect("positive local runtime report");
    let sidecar = read_sidecar("lr-01-local_sugoroku_roll_publish_handoff.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(
        report.terminal_outcome,
        sidecar["expected_runtime"]["terminal_outcome"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        report.current_owner,
        sidecar["expected_runtime"]["current_owner"]
            .as_str()
            .unwrap()
    );
    assert_eq!(report.dispatch_records.len(), 1);
    assert_eq!(report.dispatch_records[0].dispatch_outcome, "accepted");
    assert_eq!(report.runtime_snapshot.membership.membership_epoch, 0);
    assert_eq!(
        report.runtime_snapshot.place_catalog.places["GamePlace[SugorokuGame#1]"],
        "SugorokuGamePlace"
    );
    assert!(
        report
            .event_dag
            .edges
            .iter()
            .any(|edge| edge.relation == "publication_order")
    );
    assert!(
        report
            .event_dag
            .edges
            .iter()
            .any(|edge| edge.relation == "witness_order")
    );
    assert!(
        report
            .visible_history
            .iter()
            .any(|entry| entry.contains("handoff"))
    );
}

#[test]
fn stale_membership_report_rejects_epoch_drift_without_state_change() {
    let report =
        build_stale_membership_rejection_report().expect("stale membership rejection report");
    let sidecar = read_sidecar("lr-02-stale_membership_rejected.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(
        report.terminal_outcome,
        sidecar["expected_runtime"]["terminal_outcome"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        report.reason_family.as_deref(),
        sidecar["expected_runtime"]["reason_family"].as_str()
    );
    assert_eq!(report.dispatch_records.len(), 1);
    assert_eq!(
        report.dispatch_records[0].dispatch_outcome,
        "rejected_stale_membership"
    );
    assert!(
        report.dispatch_records[0]
            .reason_refs
            .contains(&"membership_epoch_drift".to_string())
    );
    assert_eq!(report.current_owner, "Alice");
    assert!(report.visible_history.is_empty());
    assert_eq!(report.runtime_snapshot.membership.membership_epoch, 1);
}

#[test]
fn local_runtime_reports_keep_later_runtime_boundaries_explicit() {
    let report = build_local_sugoroku_runtime_report().expect("positive local runtime report");

    for kept_later in [
        "layer_insertion_runtime",
        "network_docker_runtime",
        "runtime_package_avatar_admission",
        "distributed_save_load",
        "final_public_abi",
    ] {
        assert!(
            report.retained_later_refs.contains(&kept_later.to_string()),
            "missing kept-later ref {kept_later}"
        );
    }
}
