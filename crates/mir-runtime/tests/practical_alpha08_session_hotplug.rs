use std::path::PathBuf;

use mir_runtime::practical_alpha05_session::{
    load_practical_alpha05_session, save_practical_alpha05_session,
    start_practical_alpha05_session_path,
};
use mir_runtime::practical_alpha08_hotplug_session::run_practical_alpha08_hotplug_path;

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

#[test]
fn practical_alpha08_debug_attach_mutates_live_session_and_observer_export() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start the alpha-0.5 session carrier");

    let (next_session, report) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-01-debug-layer-attach"),
    )
    .expect("HP-A1-01 should attach against the live session carrier");

    assert_eq!(next_session.session_phase, "hotplug");
    assert!(next_session.same_session_hotplug_claimed);
    assert_eq!(next_session.active_layers, vec!["debug_trace_layer"]);
    assert_eq!(next_session.hotplug_lifecycle.len(), 1);
    assert_eq!(
        next_session.observer_safe_export.active_layers,
        vec!["debug_trace_layer"]
    );
    assert_eq!(report.terminal_outcome, "accepted");
    assert!(report.session_mutated);
    assert_eq!(
        report.observer_safe_export_after.hotplug_events,
        vec!["hotplug:HP-A1-01:accepted"]
    );
}

#[test]
fn practical_alpha08_rejected_attach_leaves_session_frontier_unchanged() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start the alpha-0.5 session carrier");

    let (next_session, report) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-02-non-admin-debug-rejected"),
    )
    .expect("HP-A1-02 should return an explicit rejection report");

    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("authorization"));
    assert!(!report.session_mutated);
    assert_eq!(next_session, session);
    assert!(report.observer_safe_export_after.hotplug_events.is_empty());
}

#[test]
fn practical_alpha08_object_preview_and_deferred_detach_persist_through_save_load() {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start the alpha-0.5 session carrier");
    let (after_object_attach, object_report) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-06-object-package-attach"),
    )
    .expect("HP-A1-06 should project one object preview into the live session");
    let (after_detach_boundary, detach_report) = run_practical_alpha08_hotplug_path(
        &after_object_attach,
        practical_package_dir("hp-a1-07-detach-minimal-contract"),
    )
    .expect("HP-A1-07 should project one deferred detach boundary into the live session");
    let saved = save_practical_alpha05_session(&after_detach_boundary, "savepoint#hotplug")
        .expect("hot-plug session should serialize");
    let loaded = load_practical_alpha05_session(&saved, "savepoint#hotplug")
        .expect("savepoint should restore hot-plug state");

    assert_eq!(
        object_report.terminal_outcome,
        "accepted_object_attach_preview"
    );
    assert_eq!(
        after_object_attach.object_preview_inventory[0].selected_representation,
        "static_capsule_placeholder"
    );
    assert_eq!(
        detach_report.terminal_outcome,
        "deferred_detach_minimal_contract"
    );
    assert_eq!(after_detach_boundary.hotplug_lifecycle.len(), 2);
    assert_eq!(
        after_detach_boundary.observer_safe_export.hotplug_events,
        vec![
            "hotplug:HP-A1-06:accepted_object_attach_preview",
            "hotplug:HP-A1-07:deferred_detach_minimal_contract",
        ]
    );
    assert!(saved.savepoints[0].state_roundtrip_equal);
    assert_eq!(loaded.object_preview_inventory.len(), 1);
    assert_eq!(loaded.hotplug_lifecycle.len(), 2);
    assert_eq!(
        loaded.observer_safe_export.object_preview_inventory,
        vec!["hp-a1-06-object-package-attach:static_capsule_placeholder"]
    );
}
