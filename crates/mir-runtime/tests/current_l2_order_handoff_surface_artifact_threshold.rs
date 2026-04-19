use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold,
    build_current_l2_source_sample_order_handoff_surface_artifact_threshold,
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

fn assert_threshold_reached(
    threshold: &CurrentL2SourceSampleOrderHandoffSurfaceArtifactThreshold,
    expected_profile_axis_refs: &[&str],
    expected_principal_surface_lines: &[&str],
    expected_secondary_surface_lines: &[&str],
    expected_repo_local_emitted_artifact_refs: &[&str],
) {
    assert_eq!(
        threshold.threshold_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(threshold.threshold_guard_reason.is_none());
    assert_eq!(
        threshold.profile_axis_refs,
        expected_profile_axis_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.principal_surface_lines,
        expected_principal_surface_lines
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.secondary_surface_lines,
        expected_secondary_surface_lines
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.threshold_default_refs,
        vec![
            "surface_threshold_default:edge_row_vertical_continuation_principal".to_string(),
            "surface_threshold_default:readable_high_level_relation_vocabulary".to_string(),
            "surface_threshold_default:stage_block_secondary_candidate".to_string(),
            "surface_threshold_default:repo_local_emitted_artifact_refs_first".to_string(),
            "surface_threshold_default:witness_clause_public_row_later".to_string(),
        ]
    );
    assert_eq!(
        threshold.compare_floor_refs,
        vec![
            "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
            "compare_floor:current_l2.experimental_order_handoff_surface".to_string(),
            "compare_floor:current_l2.experimental_stage_block_surface".to_string(),
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold"
                .to_string(),
            "compare_floor:current_l2.order_handoff.surface_artifact_threshold".to_string(),
        ]
    );
    assert_eq!(
        threshold.contrast_refs,
        vec![
            "contrast_target:authoritative_room_serial_scope_sugar".to_string(),
            "contrast_target:low_level_memory_order_source_surface".to_string(),
            "contrast_target:packed_metadata_line".to_string(),
            "contrast_target:combined_public_contract".to_string(),
        ]
    );
    assert_eq!(
        threshold.guard_refs,
        vec![
            "guard:surface_threshold_only".to_string(),
            "guard:edge_row_principal".to_string(),
            "guard:stage_block_secondary_only".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:final_wording_later".to_string(),
        ]
    );
    assert_eq!(
        threshold.kept_later_refs,
        vec![
            "kept_later:final_parser_grammar".to_string(),
            "kept_later:final_public_parser_checker_runtime_api".to_string(),
            "kept_later:final_source_surface_handoff_wording".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:authoritative_room_serial_scope_sugar".to_string(),
            "kept_later:low_level_memory_order_source_surface".to_string(),
            "kept_later:final_modal_foundation_adoption".to_string(),
        ]
    );
}

#[test]
fn order_handoff_surface_artifact_threshold_reaches_late_join_default() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold = build_current_l2_source_sample_order_handoff_surface_artifact_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        threshold.source_report.sample_id,
        "p07-dice-late-join-visible-history"
    );
    assert_threshold_reached(
        &threshold,
        &[
            "profile_axis:activation:authority-ack",
            "profile_axis:authority_placement:single_room_authority",
            "profile_axis:consistency_mode:authoritative_serial_transition",
            "profile_axis:rng_source:authority_rng",
            "profile_axis:late_join:published_history_visible_as_past",
            "profile_axis:fairness_claim:no_distributed_fairness_theorem_required",
        ],
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
            "transition handoff_turn(dice_owner = player_a)",
            "stage publish:",
            "  publish publish_roll_result@dice_state",
            "stage handoff:",
            "  handoff handoff_dice_authority@dice_state",
            "    after publish(publish_roll_result@dice_state)",
            "    requires witness(publish_roll_result@dice_state)",
            "stage observe:",
            "  observe late_join_view@dice_state",
            "    after handoff(handoff_dice_authority@dice_state)",
        ],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn order_handoff_surface_artifact_threshold_reaches_stale_reconnect_default() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let threshold = build_current_l2_source_sample_order_handoff_surface_artifact_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(threshold.source_report.sample_id, "p08-dice-stale-reconnect-refresh");
    assert_threshold_reached(
        &threshold,
        &[
            "profile_axis:activation:authority-ack",
            "profile_axis:authority_placement:single_room_authority",
            "profile_axis:consistency_mode:authoritative_serial_transition",
            "profile_axis:rng_source:authority_rng",
            "profile_axis:stale_reconnect:fail_then_refresh",
            "profile_axis:replay:stale_incompatible_replay_invalidated",
            "profile_axis:fairness_claim:no_distributed_fairness_theorem_required",
        ],
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
            "transition reconnect_refresh(dice_owner = player_a)",
            "stage rollback:",
            "  rollback stale_reconnect",
            "stage refresh:",
            "  refresh refresh_owner_snapshot@dice_state",
            "    after rollback(stale_reconnect)",
            "stage replay:",
            "  invalidate stale_incompatible_replay@dice_state",
            "    after refresh(refresh_owner_snapshot@dice_state)",
        ],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn order_handoff_surface_artifact_threshold_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let threshold = build_current_l2_source_sample_order_handoff_surface_artifact_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(threshold.source_report.sample_id, "p05-dice-owner-guarded-chain");
    assert_eq!(
        threshold.threshold_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(threshold
        .threshold_guard_reason
        .as_ref()
        .unwrap()
        .contains("order-handoff surface/artifact threshold"));
    assert!(threshold.profile_axis_refs.is_empty());
    assert!(threshold.principal_surface_lines.is_empty());
    assert!(threshold.secondary_surface_lines.is_empty());
    assert!(threshold.repo_local_emitted_artifact_refs.is_empty());
    assert!(threshold.threshold_default_refs.is_empty());
    assert_eq!(
        threshold.compare_floor_refs,
        vec!["compare_floor:current_l2.order_handoff.surface_artifact_threshold_guard_only"
            .to_string()]
    );
    assert_eq!(
        threshold.guard_refs,
        vec!["guard:surface_artifact_threshold_not_reached".to_string()]
    );
}
