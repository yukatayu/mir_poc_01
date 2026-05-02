use std::{fs, path::PathBuf};

use mir_runtime::alpha_avatar_runtime::{
    build_adapter_undeclared_effect_rejected_report, build_closeout_reports,
    build_custom_mir_avatar_runtime_report, build_placeholder_avatar_runtime_report,
    build_report_by_sample_id, build_revoked_signed_package_rejected_report,
    build_runtime_unavailable_placeholder_report,
    build_signed_overcapability_package_rejected_report,
    build_unsigned_native_package_rejected_report, build_untrusted_native_avatar_rejected_report,
    runtime_package_manifest_lanes,
};
use serde_json::Value;

fn sidecar_path(sample_id: &str) -> PathBuf {
    let relative = match sample_id {
        "AV-01" => {
            "../../samples/alpha/avatar-runtime/av-01-placeholder_avatar_runtime.expected.json"
        }
        "AV-02" => {
            "../../samples/alpha/avatar-runtime/av-02-custom_mir_avatar_runtime.expected.json"
        }
        "AV-06" => {
            "../../samples/alpha/avatar-runtime/av-06-untrusted_native_avatar_rejected.expected.json"
        }
        "AV-08" => {
            "../../samples/alpha/avatar-runtime/av-08-runtime_unavailable_placeholder.expected.json"
        }
        "AV-09" => {
            "../../samples/alpha/avatar-runtime/av-09-adapter_attempts_undeclared_effect.expected.json"
        }
        "HP-11" => {
            "../../samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json"
        }
        "HP-12" => {
            "../../samples/alpha/hotplug-runtime/hp-12-signed_overcapability_package_rejected.expected.json"
        }
        "HP-15" => {
            "../../samples/alpha/hotplug-runtime/hp-15-revoked_signed_package_rejected.expected.json"
        }
        _ => panic!("unknown implemented sample_id {sample_id}"),
    };
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn read_sidecar(sample_id: &str) -> Value {
    let path = sidecar_path(sample_id);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read sidecar {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse sidecar {}: {error}", path.display()))
}

#[test]
fn av_01_accepts_placeholder_avatar_without_native_execution() {
    let report = build_placeholder_avatar_runtime_report().expect("AV-01 report");

    assert_eq!(report.sample_id, "AV-01");
    assert_eq!(report.terminal_outcome, "accepted");
    assert_eq!(report.manifest.package_kind, "placeholder_avatar_runtime");
    assert_eq!(
        report
            .representation_state
            .selected_representation
            .as_deref(),
        Some("static_capsule_placeholder")
    );
    assert!(!report.representation_state.native_execution_performed);
    assert_eq!(
        report.manifest_field_lanes,
        runtime_package_manifest_lanes()
    );
}

#[test]
fn av_02_accepts_custom_mir_avatar_runtime_as_non_core_package() {
    let report = build_custom_mir_avatar_runtime_report().expect("AV-02 report");

    assert_eq!(report.sample_id, "AV-02");
    assert_eq!(report.terminal_outcome, "accepted");
    assert!(
        report
            .manifest
            .provided_roles
            .contains(&"Animatable".to_string())
    );
    assert!(
        report
            .what_it_proves
            .iter()
            .any(|line| line.contains("outside core semantics"))
    );
}

#[test]
fn av_06_rejects_untrusted_native_avatar() {
    let report = build_untrusted_native_avatar_rejected_report().expect("AV-06 report");

    assert_eq!(report.sample_id, "AV-06");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("provenance_policy"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"avatar_native_unsigned".to_string())
    );
}

#[test]
fn av_08_runtime_unavailable_selects_visible_placeholder_fallback() {
    let report = build_runtime_unavailable_placeholder_report().expect("AV-08 report");

    assert_eq!(report.sample_id, "AV-08");
    assert_eq!(report.terminal_outcome, "accepted");
    assert_eq!(report.hotplug_skeleton.verdict.verdict_kind, "rejected");
    assert!(
        report
            .hotplug_skeleton
            .verdict
            .compatibility_reason_refs
            .contains(&"host_capability_missing:HostMirAvatarVM".to_string())
    );
    assert!(report.representation_state.fallback_applied);
    assert_eq!(
        report.representation_state.selected_package_id.as_deref(),
        Some("RuntimePackage[avatar.placeholder.basic@1.0.0]")
    );
    assert_eq!(
        report.representation_state.fallback_reason.as_deref(),
        Some("host_capability_missing:HostMirAvatarVM")
    );
}

#[test]
fn av_09_rejects_undeclared_effect_widening() {
    let report = build_adapter_undeclared_effect_rejected_report().expect("AV-09 report");

    assert_eq!(report.sample_id, "AV-09");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("effect_manifest"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"undeclared_effect:emit_external_avatar_telemetry".to_string())
    );
}

#[test]
fn hp_11_rejects_unsigned_native_package() {
    let report = build_unsigned_native_package_rejected_report().expect("HP-11 report");

    assert_eq!(report.sample_id, "HP-11");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("provenance_policy"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"unsigned_native_package".to_string())
    );
}

#[test]
fn hp_12_rejects_signed_overcapability_package() {
    let report = build_signed_overcapability_package_rejected_report().expect("HP-12 report");

    assert_eq!(report.sample_id, "HP-12");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("capability_policy"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"over_capability:ReadRawCameraBuffer".to_string())
    );
}

#[test]
fn hp_15_rejects_revoked_signed_package() {
    let report = build_revoked_signed_package_rejected_report().expect("HP-15 report");

    assert_eq!(report.sample_id, "HP-15");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("provenance_policy"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"revoked_or_stale_signature".to_string())
    );
}

#[test]
fn closeout_reports_cover_all_implemented_avatar_package_rows() {
    let reports = build_closeout_reports().expect("avatar/package closeout");
    let sample_ids: Vec<_> = reports
        .iter()
        .map(|report| report.sample_id.as_str())
        .collect();

    assert_eq!(
        sample_ids,
        vec![
            "AV-01", "AV-02", "AV-06", "AV-08", "AV-09", "HP-11", "HP-12", "HP-15"
        ]
    );
    for report in &reports {
        assert!(
            report
                .retained_later_refs
                .contains(&"final_public_avatar_api".to_string())
        );
        assert!(
            report
                .retained_later_refs
                .contains(&"native_execution_sandbox_realization".to_string())
        );
    }
}

#[test]
fn implemented_reports_match_committed_sidecars() {
    for sample_id in [
        "AV-01", "AV-02", "AV-06", "AV-08", "AV-09", "HP-11", "HP-12", "HP-15",
    ] {
        let report = build_report_by_sample_id(sample_id).expect("implemented report");
        let report_value = serde_json::to_value(report).expect("serialize report");
        let sidecar = read_sidecar(sample_id);

        assert_eq!(report_value, sidecar, "sidecar drift for {sample_id}");
    }
}
