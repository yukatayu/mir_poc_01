use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
    CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface,
    CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption,
    CurrentL2SourceSampleWitnessProviderRouteActualAdoption,
    build_current_l2_source_sample_delegated_rng_service_practical_actualization,
    build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface,
    build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption,
    build_current_l2_source_sample_witness_provider_route_actual_adoption,
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

fn assert_room_default_surface_matches_source_wording_route(
    surface: &CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface,
    source_wording_route: &CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption,
    expected_lines: &[&str],
    expected_compare_floor_refs: &[&str],
) {
    assert_eq!(
        surface.surface_status,
        source_wording_route.actualization_status
    );
    assert_eq!(
        surface.surface_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(surface.surface_guard_reason.is_none());
    assert_eq!(
        surface.profile_axis_refs,
        source_wording_route.profile_axis_refs
    );
    assert_eq!(
        surface.repo_local_emitted_artifact_refs,
        source_wording_route.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        surface.serial_scope_lines,
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
        vec![
            "guard:serial_scope_room_specific_reserve".to_string(),
            "guard:principal_edge_row_surface_unchanged".to_string(),
            "guard:helper_local_surface_only".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
        ]
    );
    assert_eq!(
        surface.kept_later_refs,
        vec![
            "kept_later:final_parser_grammar".to_string(),
            "kept_later:final_public_parser_checker_runtime_api".to_string(),
            "kept_later:serial_scope_public_promotion".to_string(),
            "kept_later:serial_scope_beyond_authoritative_room".to_string(),
            "kept_later:final_source_surface_handoff_wording".to_string(),
            "kept_later:final_emitted_artifact_schema".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:low_level_memory_order_source_surface".to_string(),
            "kept_later:final_modal_foundation_adoption".to_string(),
        ]
    );
}

fn assert_delegated_provider_surface_matches_prior_routes(
    surface: &CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface,
    delegated: &CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
    witness_provider_route: &CurrentL2SourceSampleWitnessProviderRouteActualAdoption,
    expected_lines: &[&str],
    expected_compare_floor_refs: &[&str],
) {
    assert_eq!(surface.surface_status, delegated.practical_status);
    assert_eq!(
        surface.surface_status,
        witness_provider_route.actualization_status
    );
    assert_eq!(
        surface.surface_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(surface.surface_guard_reason.is_none());
    assert_eq!(surface.profile_axis_refs, delegated.profile_axis_refs);
    assert_eq!(
        surface.profile_axis_refs,
        witness_provider_route.profile_axis_refs
    );
    assert_eq!(
        surface.repo_local_emitted_artifact_refs,
        delegated.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        surface.repo_local_emitted_artifact_refs,
        witness_provider_route.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        surface.serial_scope_lines,
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
        vec![
            "guard:serial_scope_room_specific_reserve".to_string(),
            "guard:principal_edge_row_surface_unchanged".to_string(),
            "guard:helper_local_surface_only".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
        ]
    );
}

#[test]
fn order_handoff_serial_scope_reserve_surface_reaches_late_join_room_default() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let source_wording_route =
        build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let surface = build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        surface.source_report.sample_id,
        "p07-dice-late-join-visible-history"
    );
    assert_room_default_surface_matches_source_wording_route(
        &surface,
        &source_wording_route,
        &[
            "serial on dice_authority {",
            "  publish publish_roll_result@dice_state",
            "  handoff handoff_dice_authority@dice_state",
            "    requires witness(publish_roll_result@dice_state)",
            "  observe late_join_view@dice_state",
            "}",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.experimental_order_handoff_surface",
            "compare_floor:current_l2.experimental_stage_block_surface",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.order_handoff.surface_artifact_threshold",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.order_handoff.source_wording_emitted_artifact.coupled_later_gate",
            "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption",
            "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface",
        ],
    );
}

#[test]
fn order_handoff_serial_scope_reserve_surface_reaches_stale_reconnect_room_default() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let source_wording_route =
        build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let surface = build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        surface.source_report.sample_id,
        "p08-dice-stale-reconnect-refresh"
    );
    assert_room_default_surface_matches_source_wording_route(
        &surface,
        &source_wording_route,
        &[
            "serial on dice_authority {",
            "  rollback stale_reconnect",
            "  refresh refresh_owner_snapshot@dice_state",
            "  invalidate stale_incompatible_replay@dice_state",
            "}",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.experimental_order_handoff_surface",
            "compare_floor:current_l2.experimental_stage_block_surface",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.order_handoff.surface_artifact_threshold",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.order_handoff.source_wording_emitted_artifact.coupled_later_gate",
            "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption",
            "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface",
        ],
    );
}

#[test]
fn order_handoff_serial_scope_reserve_surface_reaches_delegated_provider_room_default() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let delegated = build_current_l2_source_sample_delegated_rng_service_practical_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let witness_provider_route =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let surface = build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        surface.source_report.sample_id,
        "p09-dice-delegated-rng-provider-placement"
    );
    assert_delegated_provider_surface_matches_prior_routes(
        &surface,
        &delegated,
        &witness_provider_route,
        &[
            "serial on dice_authority {",
            "  fetch fetch_provider_roll@delegated_rng",
            "  publish publish_roll_result@dice_state",
            "  handoff handoff_dice_authority@dice_state",
            "}",
        ],
        &[
            "compare_floor:current_l2.order_handoff.runner_cli",
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_route_actual_adoption",
            "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface",
        ],
    );
}

#[test]
fn order_handoff_serial_scope_reserve_surface_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let surface = build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        surface.source_report.sample_id,
        "p05-dice-owner-guarded-chain"
    );
    assert_eq!(
        surface.surface_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        surface
            .surface_guard_reason
            .as_ref()
            .unwrap()
            .contains("serial-scope reserve surface")
    );
    assert!(surface.profile_axis_refs.is_empty());
    assert!(surface.repo_local_emitted_artifact_refs.is_empty());
    assert!(surface.serial_scope_lines.is_empty());
    assert_eq!(
        surface.compare_floor_refs,
        vec![
            "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface.guard_only"
                .to_string(),
        ]
    );
    assert_eq!(
        surface.guard_refs,
        vec!["guard:serial_scope_reserve_surface_not_reached".to_string()]
    );
}
