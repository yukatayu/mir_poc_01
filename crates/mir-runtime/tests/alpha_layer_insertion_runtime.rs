use std::{fs, path::PathBuf};

use mir_runtime::alpha_layer_insertion_runtime::{
    build_auth_layer_contract_update_report, build_debug_layer_attach_report,
    build_debug_layer_non_admin_rejection_report, build_declared_ratelimit_report,
    build_incompatible_patch_rejection_report,
};
use serde_json::Value;

fn sample_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/alpha/layer-insertion")
}

fn read_sidecar(name: &str) -> Value {
    let path = sample_root().join(name);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read sidecar {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse sidecar {}: {error}", path.display()))
}

#[test]
fn debug_layer_attach_requires_authority_and_emits_trace_only_after_attach() {
    let report = build_debug_layer_attach_report().expect("LI-01 report");
    let sidecar = read_sidecar("li-01-debug_layer_attach_authorized.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(report.terminal_outcome, "accepted".to_string());
    assert!(report.compatibility.failed_reason_refs.is_empty());
    assert!(report.active_layers_before.is_empty());
    assert_eq!(
        report.active_layers_after,
        vec!["debug_trace_layer".to_string()]
    );
    assert!(report.pre_attach_trace_rows.is_empty());
    assert_eq!(report.post_attach_trace_rows.len(), 2);
    assert!(
        report
            .post_attach_trace_rows
            .iter()
            .all(|row| serde_json::to_value(row.redaction_level).unwrap()
                == sidecar["expected_runtime"]["redaction_level"])
    );
    assert_eq!(report.source_runtime_sample_ref.as_deref(), Some("LR-01"));
}

#[test]
fn debug_layer_non_admin_rejection_leaves_no_layer_and_no_trace() {
    let report = build_debug_layer_non_admin_rejection_report().expect("LI-02 report");
    let sidecar = read_sidecar("li-02-debug_layer_non_admin_rejected.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(report.terminal_outcome, "rejected".to_string());
    assert!(
        report
            .hotplug_runtime_report
            .verdict
            .authorization_reason_refs
            .contains(&"attach_capability_missing".to_string())
    );
    assert!(report.active_layers_after.is_empty());
    assert!(report.post_attach_trace_rows.is_empty());
}

#[test]
fn auth_layer_attach_is_explicit_contract_update_path() {
    let report = build_auth_layer_contract_update_report().expect("LI-03 report");
    let sidecar = read_sidecar("li-03-auth_layer_contract_update_path.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(
        report.terminal_outcome,
        "accepted_contract_update".to_string()
    );
    assert_eq!(
        report.compatibility.accepted_path,
        "explicit_contract_update".to_string()
    );
    assert_eq!(
        report.attach_request.contract_update_ref.as_deref(),
        sidecar["expected_runtime"]["required_contract_update_ref"].as_str()
    );
}

#[test]
fn rate_limit_layer_attach_requires_declared_failure_budget() {
    let report = build_declared_ratelimit_report().expect("LI-04 report");
    let sidecar = read_sidecar("li-04-ratelimit_declared_failure.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(report.terminal_outcome, "accepted".to_string());
    assert!(
        report
            .compatibility
            .passed_reason_refs
            .contains(&"failure_row_within_declared_budget".to_string())
    );
    let preview = report.runtime_preview.expect("rate-limit preview");
    assert_eq!(
        preview.terminal_outcome,
        sidecar["expected_runtime"]["preview_terminal_outcome"]
            .as_str()
            .unwrap()
    );
}

#[test]
fn incompatible_patch_is_rejected_before_activation() {
    let report = build_incompatible_patch_rejection_report().expect("LI-05 report");
    let sidecar = read_sidecar("li-05-incompatible_patch_rejected.expected.json");

    assert_eq!(report.sample_id, sidecar["sample_id"].as_str().unwrap());
    assert_eq!(report.terminal_outcome, "rejected".to_string());
    assert!(
        report
            .compatibility
            .failed_reason_refs
            .contains(&"provided_interface_shrunk".to_string())
    );
    assert!(
        report
            .compatibility
            .failed_reason_refs
            .contains(&"observation_labels_widened".to_string())
    );
    assert!(report.active_layers_after.is_empty());
}

#[test]
fn layer_insertion_reports_keep_later_runtime_boundaries_explicit() {
    let report = build_debug_layer_attach_report().expect("LI-01 report");

    for kept_later in [
        "completed_hotplug_lifecycle",
        "detach_runtime",
        "rollback_protocol",
        "durable_migration",
        "distributed_activation_ordering",
        "network_docker_runtime",
        "distributed_save_load",
        "runtime_package_avatar_admission",
        "final_public_layer_attachment_abi",
    ] {
        assert!(
            report.retained_later_refs.contains(&kept_later.to_string()),
            "missing kept-later ref {kept_later}"
        );
    }
}
