use std::{fs, path::PathBuf};

use mir_runtime::alpha_local_runtime::build_local_save_load_resume_report;
use serde_json::{Value, to_value};

fn sample_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/alpha/cut-save-load")
}

fn read_sidecar(name: &str) -> Value {
    let path = sample_root().join(name);
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read sidecar {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse sidecar {}: {error}", path.display()))
}

#[test]
fn local_save_load_report_restores_state_and_resumes_dispatch() {
    let report = build_local_save_load_resume_report().expect("local save/load report");
    let sidecar = read_sidecar("cut-04-local_save_load_valid.expected.json");

    assert_eq!(to_value(&report).expect("serialize report"), sidecar);
    assert!(report.state_roundtrip_equal);
    assert_eq!(
        report.saved_runtime_snapshot,
        report.restored_runtime_snapshot
    );
    assert_eq!(report.saved_runtime_snapshot.membership.membership_epoch, 0);
    assert_eq!(report.restored_visible_history.len(), 2);
    assert_eq!(report.resumed_dispatch_records.len(), 1);
    assert_eq!(
        report.resumed_dispatch_records[0].dispatch_outcome,
        "accepted"
    );
    assert!(
        report.resumed_dispatch_records[0]
            .reason_refs
            .contains(&"saved_local_state_restored".to_string())
    );
    assert!(
        report
            .retained_later_refs
            .contains(&"distributed_save_load".to_string())
    );
}
