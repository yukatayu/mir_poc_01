use std::path::PathBuf;

use mir_runtime::full_system_v1_local_split::FullSystemV1LocalRoleExecutionKind;
use mir_runtime::full_system_v1_provider_admission::run_full_system_v1_provider_admission_path;

fn provider_sample_path(root: &str, relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/full-system-v1/provider-adapter")
        .join(root)
        .join(relative_path)
}

#[test]
fn provider_admission_accepts_viewer_diagnostic_inventory_row() {
    let report = run_full_system_v1_provider_admission_path(
        provider_sample_path(
            "viewer-diagnostic-positive",
            "main/src/viewer-diagnostic-positive.mir",
        ),
        provider_sample_path("viewer-diagnostic-positive", "projection.request.json"),
        provider_sample_path("viewer-diagnostic-positive", "provider.manifest.json"),
        0,
    );

    assert!(report.accepted, "{report:?}");
    assert_eq!(report.provider_id, "viewer-diagnostic-exporter");
    assert_eq!(report.provider_kind, "viewer-diagnostic-exporter");
    assert_eq!(report.target_id, "diagnostic-adapter");
    assert_eq!(report.target_provider_policy, "provider_inventory_only");
    assert_eq!(report.terminal_outcome, "inventory_admitted");
    assert!(!report.execution_admitted);
    assert!(
        report
            .matched_ffi_schema_refs
            .contains(&"ffi.diagnostic.export_preview".to_string())
    );
    assert!(
        !report
            .residual_obligations
            .iter()
            .any(|row| row.code == "provider_admission_deferred")
    );
    assert!(
        report
            .residual_obligations
            .iter()
            .any(|row| row.code == "provider_execution_runtime_deferred")
    );
    let split = report
        .local_split_report
        .expect("accepted provider rows should preserve local split evidence");
    assert!(split.accepted);
    assert!(
        split
            .target_reports
            .iter()
            .any(|row| row.target_id == "world-client"
                && matches!(
                    row.execution_kind,
                    FullSystemV1LocalRoleExecutionKind::AuthoritativeRuntime
                ))
    );
    assert!(
        split
            .target_reports
            .iter()
            .any(|row| row.target_id == "diagnostic-adapter"
                && matches!(
                    row.execution_kind,
                    FullSystemV1LocalRoleExecutionKind::PassiveEndpoint
                ))
    );
}

#[test]
fn provider_admission_rejects_over_capability_manifest() {
    let report = run_full_system_v1_provider_admission_path(
        provider_sample_path(
            "over-capability-negative",
            "main/src/over-capability-negative.mir",
        ),
        provider_sample_path("over-capability-negative", "projection.request.json"),
        provider_sample_path("over-capability-negative", "provider.manifest.json"),
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert_eq!(report.provider_id, "viewer-diagnostic-exporter-overreach");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.diagnostics[0].code, "provider_over_capability");
    assert!(
        report
            .rejected_rows
            .contains(&"viewer-diagnostic-exporter-overreach:provider_over_capability".to_string())
    );
}

#[test]
fn provider_admission_rejects_missing_rollback_policy() {
    let report = run_full_system_v1_provider_admission_path(
        provider_sample_path(
            "missing-rollback-negative",
            "main/src/missing-rollback-negative.mir",
        ),
        provider_sample_path("missing-rollback-negative", "projection.request.json"),
        provider_sample_path("missing-rollback-negative", "provider.manifest.json"),
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert_eq!(
        report.provider_id,
        "viewer-diagnostic-exporter-missing-rollback"
    );
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(
        report.diagnostics[0].code,
        "missing_rollback_replay_cut_policy"
    );
}

#[test]
fn provider_admission_rejects_native_execution_when_disabled_by_default() {
    let report = run_full_system_v1_provider_admission_path(
        provider_sample_path(
            "native-disabled-negative",
            "main/src/native-disabled-negative.mir",
        ),
        provider_sample_path("native-disabled-negative", "projection.request.json"),
        provider_sample_path("native-disabled-negative", "provider.manifest.json"),
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert_eq!(report.provider_id, "native-library-bridge");
    assert_eq!(report.terminal_outcome, "native_execution_disabled");
    assert!(
        report
            .rejected_rows
            .contains(&"native-library-bridge:native_execution_disabled_by_default".to_string())
    );
}

#[test]
fn provider_admission_reports_native_execution_disabled_policy() {
    let report = run_full_system_v1_provider_admission_path(
        provider_sample_path(
            "native-disabled-negative",
            "main/src/native-disabled-negative.mir",
        ),
        provider_sample_path("native-disabled-negative", "projection.request.json"),
        provider_sample_path("native-disabled-negative", "provider.manifest.json"),
        0,
    );

    assert!(!report.accepted, "{report:?}");
    assert_eq!(report.terminal_outcome, "native_execution_disabled");
    assert_eq!(
        report.diagnostics[0].code,
        "native_execution_disabled_by_default"
    );
}

#[test]
fn provider_admission_keeps_wasm_inventory_only_without_execution_claim() {
    let report = run_full_system_v1_provider_admission_path(
        provider_sample_path(
            "wasm-inventory-positive",
            "main/src/wasm-inventory-positive.mir",
        ),
        provider_sample_path("wasm-inventory-positive", "projection.request.json"),
        provider_sample_path("wasm-inventory-positive", "provider.manifest.json"),
        0,
    );

    assert!(report.accepted, "{report:?}");
    assert_eq!(report.provider_id, "wasm-sandbox");
    assert_eq!(report.target_provider_policy, "wasm_inventory_only");
    assert_eq!(report.terminal_outcome, "wasm_inventory_only");
    assert!(!report.execution_admitted);
    assert_eq!(report.diagnostics[0].code, "wasm_inventory_only");
    assert!(
        report
            .residual_obligations
            .iter()
            .any(|row| row.code == "sandboxed_wasm_execution_deferred")
    );
}
