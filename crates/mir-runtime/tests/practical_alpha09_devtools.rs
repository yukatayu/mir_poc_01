use std::path::PathBuf;

use mir_runtime::practical_alpha05_host_io::run_practical_alpha05_host_io_path;
use mir_runtime::practical_alpha05_session::{
    load_practical_alpha05_session, save_practical_alpha05_session,
    start_practical_alpha05_session_path,
};
use mir_runtime::practical_alpha08_hotplug_session::run_practical_alpha08_hotplug_path;
use mir_runtime::practical_alpha09_devtools::export_practical_alpha09_devtools;

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_package_dir(name: &str) -> PathBuf {
    practical_root().join("packages").join(name)
}

fn rich_session() -> mir_runtime::practical_alpha05_session::PracticalAlpha05SessionReport {
    let session =
        start_practical_alpha05_session_path(practical_package_dir("run-01-local-sugoroku"))
            .expect("RUN-01 should start a practical alpha-0.5 session");
    let (session, _) = run_practical_alpha05_host_io_path(
        &session,
        practical_package_dir("oa05-07-add-one-host-io"),
    )
    .expect("OA05-07 should add one typed host-I/O lane");
    let (session, _) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-01-debug-layer-attach"),
    )
    .expect("debug attach should mutate the live session");
    let (session, _) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-02-non-admin-debug-rejected"),
    )
    .expect("rejected debug attach should be session-observable");
    let (session, _) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-06-object-package-attach"),
    )
    .expect("object preview should be session-observable");
    let (session, _) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("av-a1-03-unsupported-runtime-fallback"),
    )
    .expect("unsupported runtime fallback should stay rejected but observable");
    let (session, _) = run_practical_alpha08_hotplug_path(
        &session,
        practical_package_dir("hp-a1-07-detach-minimal-contract"),
    )
    .expect("deferred detach boundary should be session-observable");
    let saved = save_practical_alpha05_session(&session, "savepoint#oa09")
        .expect("rich session should save");
    load_practical_alpha05_session(&saved, "savepoint#oa09").expect("rich session should load")
}

#[test]
fn practical_alpha09_exports_required_session_bound_panels() {
    let session = rich_session();
    let export = export_practical_alpha09_devtools(&session)
        .expect("alpha-0.9 devtools export should accept the practical session");

    assert_eq!(
        export.devtools_scope,
        "practical-alpha09-session-devtools-floor"
    );
    assert_eq!(export.session_id, session.session_id);
    assert_eq!(export.export_source_kind, "same_runtime_session_carrier");
    assert!(export.session_bound_export_claimed);
    assert_eq!(export.admin_debug_view_status, "kept_later");
    assert_eq!(
        export.panel_ids,
        vec![
            "event_dag_live_session",
            "local_route_trace",
            "membership_timeline",
            "witness_relation",
            "hotplug_lifecycle",
            "fallback_degradation",
            "save_load_timeline",
            "observer_safe_redacted_view",
            "retention_on_demand_trace",
        ]
    );
    assert!(export.operational_alpha09_export_ready);
}

#[test]
fn practical_alpha09_preserves_rejected_hotplug_as_observation_not_state_mutation() {
    let session = rich_session();
    let export = export_practical_alpha09_devtools(&session)
        .expect("alpha-0.9 devtools export should accept the practical session");

    let lifecycle = &export.export_sections.hotplug_lifecycle;
    assert!(lifecycle.iter().any(|entry| entry.sample_id == "HP-A1-02"
        && entry.terminal_outcome == "rejected"
        && !entry.session_mutated));
    assert!(lifecycle.iter().any(|entry| entry.sample_id == "AV-A1-03"
        && entry.terminal_outcome == "rejected"
        && !entry.session_mutated));
    assert!(
        export
            .export_sections
            .fallback_degradation
            .iter()
            .any(|entry| entry.source_sample_id == "AV-A1-03"
                && entry.native_execution_performed == false)
    );
}

#[test]
fn practical_alpha09_exports_witness_save_load_and_retention_trace() {
    let session = rich_session();
    let export = export_practical_alpha09_devtools(&session)
        .expect("alpha-0.9 devtools export should accept the practical session");

    assert!(export.export_sections.witness_relation.iter().any(|entry| {
        entry
            .witness_refs
            .contains(&"game_started_witness".to_string())
    }));
    assert!(
        export
            .export_sections
            .save_load_timeline
            .iter()
            .any(|entry| entry.savepoint_id == "savepoint#oa09" && entry.state_roundtrip_equal)
    );
    assert_eq!(
        export
            .export_sections
            .retention_query_trace
            .iter()
            .map(|entry| entry.query_outcome.as_str())
            .collect::<Vec<_>>(),
        vec!["hit", "hit", "miss"]
    );
}
