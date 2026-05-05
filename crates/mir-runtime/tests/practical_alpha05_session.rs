use std::path::PathBuf;

use mir_runtime::practical_alpha05_session::{
    load_practical_alpha05_session, observe_practical_alpha05_session,
    save_practical_alpha05_session, start_practical_alpha05_session_path,
};

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

#[test]
fn practical_alpha05_session_starts_from_run_01_and_observes_same_frontier() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start a practical alpha-0.5 session");

    assert_eq!(session.session_phase, "started");
    assert_eq!(session.current_owner, "Bob");
    assert_eq!(
        session.observer_safe_export.redaction,
        "observer_safe_session_summary"
    );
    assert_eq!(
        session.observer_safe_export.event_ids,
        vec![
            "roll_commit#1",
            "publish_roll#1",
            "witness_draw_pub#1",
            "handoff_turn#1",
        ]
    );
}

#[test]
fn practical_alpha05_session_save_and_load_restore_same_event_dag() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start a practical alpha-0.5 session");
    let saved = save_practical_alpha05_session(&session, "savepoint#1")
        .expect("savepoint should serialize");
    let loaded =
        load_practical_alpha05_session(&saved, "savepoint#1").expect("savepoint should restore");
    let observed = observe_practical_alpha05_session(&loaded);

    assert_eq!(saved.session_phase, "saved");
    assert!(saved.savepoints[0].state_roundtrip_equal);
    assert_eq!(loaded.session_phase, "loaded");
    assert_eq!(loaded.event_dag, session.event_dag);
    assert_eq!(observed.visible_history, session.visible_history);
}

#[test]
fn practical_alpha05_session_missing_savepoint_is_rejected() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start a practical alpha-0.5 session");
    let error = load_practical_alpha05_session(&session, "savepoint#missing")
        .expect_err("missing savepoint must be rejected");

    assert!(error.detail.contains("does not exist"));
}
