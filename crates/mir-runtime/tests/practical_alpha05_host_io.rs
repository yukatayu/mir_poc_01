use std::path::PathBuf;

use mir_runtime::practical_alpha05_host_io::run_practical_alpha05_host_io_path;
use mir_runtime::practical_alpha05_session::{
    load_practical_alpha05_session, save_practical_alpha05_session,
    start_practical_alpha05_session_path,
};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

#[test]
fn practical_alpha05_host_io_add_one_updates_session_and_observer_view() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("oa05-07-add-one-host-io"))
            .expect("OA05-07 should start the alpha-0.5 session carrier");
    let (next_session, report) = run_practical_alpha05_host_io_path(
        &session,
        practical_package_dir("oa05-07-add-one-host-io"),
    )
    .expect("OA05-07 host-I/O lane should execute");

    assert_eq!(next_session.session_phase, "host_io");
    assert!(next_session.typed_host_io_claimed);
    assert_eq!(report.adapter_kind, "add_one");
    assert_eq!(
        report.session_event_ids_after,
        vec![
            "roll_commit#1",
            "publish_roll#1",
            "witness_draw_pub#1",
            "handoff_turn#1",
            "host_request#1",
            "host_response#1",
        ]
    );
    assert_eq!(
        report.observer_safe_export_after.host_io_events,
        vec!["host_io:AddOne(41)->42"]
    );
}

#[test]
fn practical_alpha05_host_io_history_roundtrips_through_session_savepoint() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("oa05-07-add-one-host-io"))
            .expect("OA05-07 should start the alpha-0.5 session carrier");
    let (after_host_io, _) = run_practical_alpha05_host_io_path(
        &session,
        practical_package_dir("oa05-07-add-one-host-io"),
    )
    .expect("OA05-07 host-I/O lane should execute");
    let saved = save_practical_alpha05_session(&after_host_io, "savepoint#host")
        .expect("host-I/O session should serialize");
    let loaded =
        load_practical_alpha05_session(&saved, "savepoint#host").expect("savepoint should restore");

    assert!(saved.savepoints[0].state_roundtrip_equal);
    assert_eq!(loaded.host_io_history.len(), 1);
    assert_eq!(
        loaded.observer_safe_export.host_io_events,
        vec!["host_io:AddOne(41)->42"]
    );
}
