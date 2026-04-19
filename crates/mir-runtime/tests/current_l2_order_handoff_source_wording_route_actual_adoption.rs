use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate,
    CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption,
    CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption,
    build_current_l2_source_sample_order_handoff_source_wording_emitted_artifact_coupled_later_gate,
    build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption,
    build_current_l2_source_sample_order_handoff_surface_actual_adoption,
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

fn assert_actual_adoption_matches_prior_floors(
    actual_adoption: &CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption,
    surface_actual_adoption: &CurrentL2SourceSampleOrderHandoffSurfaceActualAdoption,
    coupled_gate: &CurrentL2SourceSampleOrderHandoffSourceWordingEmittedArtifactCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        actual_adoption.actualization_status,
        surface_actual_adoption.actualization_status
    );
    assert_eq!(actual_adoption.actualization_status, coupled_gate.coupled_status);
    assert_eq!(
        actual_adoption.profile_axis_refs,
        surface_actual_adoption.profile_axis_refs
    );
    assert_eq!(
        actual_adoption.repo_local_emitted_artifact_refs,
        surface_actual_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        actual_adoption.source_wording_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "order_handoff_source_wording_actual_route:{}:edge_row_vertical_continuation_principal",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_source_wording_actual_route:{}:readable_high_level_relation_vocabulary",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_source_wording_actual_route:{}:stage_block_secondary_keep",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_source_wording_actual_route:{}:thread_node_same_causal_language",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_source_wording_actual_route:{}:final_source_surface_handoff_wording_later",
                    actual_adoption.source_report.sample_id
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.emitted_artifact_candidate_keep_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "order_handoff_emitted_artifact_keep:{}:repo_local_emitted_artifact_refs_first",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_emitted_artifact_keep:{}:source_surface_actual_adoption_adjacent",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_emitted_artifact_keep:{}:witness_provider_contract_adjacent_not_collapsed",
                    actual_adoption.source_report.sample_id
                ),
                format!(
                    "order_handoff_emitted_artifact_keep:{}:final_emitted_artifact_schema_later",
                    actual_adoption.source_report.sample_id
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.actual_adoption_default_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "order_handoff_source_wording_actual_adoption_default:edge_row_vertical_continuation_principal"
                    .to_string(),
                "order_handoff_source_wording_actual_adoption_default:readable_high_level_relation_vocabulary"
                    .to_string(),
                "order_handoff_source_wording_actual_adoption_default:stage_block_secondary_keep"
                    .to_string(),
                "order_handoff_source_wording_actual_adoption_default:thread_node_same_causal_language"
                    .to_string(),
                "order_handoff_source_wording_actual_adoption_default:repo_local_emitted_artifact_refs_first"
                    .to_string(),
                "order_handoff_source_wording_actual_adoption_default:emitted_artifact_schema_candidate_keep"
                    .to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.kept_later_refs,
        vec![
            "kept_later:final_parser_grammar".to_string(),
            "kept_later:final_public_parser_checker_runtime_api".to_string(),
            "kept_later:final_source_surface_handoff_wording".to_string(),
            "kept_later:final_emitted_artifact_schema".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:authoritative_room_serial_scope_sugar".to_string(),
            "kept_later:low_level_memory_order_source_surface".to_string(),
            "kept_later:final_modal_foundation_adoption".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );

    match actual_adoption.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(actual_adoption.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(actual_adoption
                .actualization_guard_reason
                .as_ref()
                .unwrap()
                .contains("order-handoff source wording route actual adoption"));
        }
    }
}

#[test]
fn order_handoff_source_wording_route_actual_adoption_reaches_late_join_default() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let surface_actual_adoption = build_current_l2_source_sample_order_handoff_surface_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_order_handoff_source_wording_emitted_artifact_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &surface_actual_adoption,
        &coupled_gate,
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
        ],
        &[
            "guard:edge_row_vertical_continuation_principal",
            "guard:readable_high_level_relation_vocabulary",
            "guard:stage_block_secondary_keep",
            "guard:thread_node_same_causal_language",
            "guard:repo_local_emitted_artifact_refs_first",
            "guard:emitted_artifact_schema_candidate_keep",
            "guard:final_source_surface_handoff_wording_later",
            "guard:final_emitted_artifact_schema_later",
        ],
    );
}

#[test]
fn order_handoff_source_wording_route_actual_adoption_reaches_stale_reconnect_default() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let surface_actual_adoption = build_current_l2_source_sample_order_handoff_surface_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_order_handoff_source_wording_emitted_artifact_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &surface_actual_adoption,
        &coupled_gate,
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
        ],
        &[
            "guard:edge_row_vertical_continuation_principal",
            "guard:readable_high_level_relation_vocabulary",
            "guard:stage_block_secondary_keep",
            "guard:thread_node_same_causal_language",
            "guard:repo_local_emitted_artifact_refs_first",
            "guard:emitted_artifact_schema_candidate_keep",
            "guard:final_source_surface_handoff_wording_later",
            "guard:final_emitted_artifact_schema_later",
        ],
    );
}

#[test]
fn order_handoff_source_wording_route_actual_adoption_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let surface_actual_adoption = build_current_l2_source_sample_order_handoff_surface_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_order_handoff_source_wording_emitted_artifact_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &surface_actual_adoption,
        &coupled_gate,
        &["compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption.guard_only"],
        &["guard:order_handoff_source_wording_route_actual_adoption_not_reached"],
    );
}
