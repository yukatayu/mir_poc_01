use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleMinimalCompanionSurface,
    build_current_l2_source_sample_minimal_companion_surface,
};

fn order_handoff_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

fn prototype_host_plan_path(sample_path: &Path) -> PathBuf {
    sample_path.with_extension("host-plan.json")
}

fn prototype_host_plan(sample_path: &Path) -> FixtureHostPlan {
    load_host_plan_from_path(&prototype_host_plan_path(sample_path)).unwrap()
}

fn assert_surface(
    surface: &CurrentL2SourceSampleMinimalCompanionSurface,
    expected_lines: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(surface.surface_status, CurrentL2EmittedArtifactRouteStatus::Reached);
    assert!(surface.surface_guard_reason.is_none());
    assert_eq!(
        surface.companion_lines,
        expected_lines
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        surface.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        surface.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        surface.kept_later_refs,
        vec![
            "kept_later:final_parser_grammar".to_string(),
            "kept_later:final_public_parser_checker_runtime_api".to_string(),
            "kept_later:low_level_memory_order_source_surface".to_string(),
            "kept_later:final_modal_foundation_adoption".to_string(),
        ]
    );
}

#[test]
fn minimal_companion_surface_reaches_late_join_room_profile() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let surface = build_current_l2_source_sample_minimal_companion_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        surface.source_report.sample_id,
        "p07-dice-late-join-visible-history"
    );
    assert_surface(
        &surface,
        &[
            "profile authoritative_room_default",
            "activation authority-ack",
            "authority single_room_authority",
            "consistency authoritative_serial_transition",
            "rng authority_rng",
            "publication publish_roll_result@dice_state",
            "handoff handoff_dice_authority@dice_state",
            "late_join published_history_visible_as_past",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.experimental_order_handoff_surface",
        ],
        &[
            "guard:semantic_honesty_first",
            "guard:helper_local_companion_only",
        ],
    );
}

#[test]
fn minimal_companion_surface_reaches_stale_reconnect_room_profile() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let surface = build_current_l2_source_sample_minimal_companion_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(surface.source_report.sample_id, "p08-dice-stale-reconnect-refresh");
    assert_surface(
        &surface,
        &[
            "profile authoritative_room_default",
            "activation authority-ack",
            "authority single_room_authority",
            "consistency authoritative_serial_transition",
            "rng authority_rng",
            "rollback stale_reconnect",
            "refresh refresh_owner_snapshot@dice_state",
            "replay stale_incompatible_replay_invalidated",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.experimental_order_handoff_surface",
        ],
        &[
            "guard:semantic_honesty_first",
            "guard:helper_local_companion_only",
        ],
    );
}

#[test]
fn minimal_companion_surface_keeps_guarded_chain_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let surface = build_current_l2_source_sample_minimal_companion_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(surface.source_report.sample_id, "p05-dice-owner-guarded-chain");
    assert_eq!(
        surface.surface_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(surface
        .surface_guard_reason
        .as_ref()
        .unwrap()
        .contains("minimal companion"));
    assert!(surface.companion_lines.is_empty());
    assert_eq!(
        surface.compare_floor_refs,
        vec!["compare_floor:current_l2.experimental_order_handoff_guard_only".to_string()]
    );
    assert_eq!(
        surface.guard_refs,
        vec!["guard:companion_surface_not_reached".to_string()]
    );
}
