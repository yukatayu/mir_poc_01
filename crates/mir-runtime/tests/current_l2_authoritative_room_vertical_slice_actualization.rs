use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization,
    build_current_l2_source_sample_authoritative_room_vertical_slice_actualization,
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

fn assert_vertical_slice(
    slice: &CurrentL2SourceSampleAuthoritativeRoomVerticalSliceActualization,
    expected_profile_axis_refs: &[&str],
    expected_relation_refs: &[&str],
    expected_authority_handoff_refs: &[&str],
    expected_runtime_evidence_refs: &[&str],
    expected_repo_local_emitted_artifact_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        slice.slice_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(slice.slice_guard_reason.is_none());
    assert_eq!(
        slice.profile_axis_refs,
        expected_profile_axis_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.relation_refs,
        expected_relation_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.authority_handoff_refs,
        expected_authority_handoff_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.runtime_evidence_refs,
        expected_runtime_evidence_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.contrast_refs,
        vec!["contrast_target:append_friendly_notice_room".to_string()]
    );
    assert_eq!(
        slice.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        slice.kept_later_refs,
        vec![
            "kept_later:auditable_authority_witness".to_string(),
            "kept_later:delegated_rng_service".to_string(),
            "kept_later:distributed_randomness_provider".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );
}

#[test]
fn authoritative_room_vertical_slice_actualization_reaches_late_join_history_profile() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let slice = build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        slice.source_report.sample_id,
        "p07-dice-late-join-visible-history"
    );
    assert_vertical_slice(
        &slice,
        &[
            "profile_axis:activation:authority-ack",
            "profile_axis:authority_placement:single_room_authority",
            "profile_axis:consistency_mode:authoritative_serial_transition",
            "profile_axis:rng_source:authority_rng",
            "profile_axis:late_join:published_history_visible_as_past",
            "profile_axis:fairness_claim:no_distributed_fairness_theorem_required",
        ],
        &[
            "relation_family:program_order",
            "relation_family:publication_order",
            "relation_family:observation_order",
            "relation_family:witness_order",
            "relation_family:finalization_order",
            "relation_family:scoped_happens_before",
        ],
        &[
            "authority_handoff:owner_slot:single_room_authority",
            "authority_handoff:stage_family:authoritative_serial_transition",
            "authority_handoff:stage_sequence:publish_then_handoff",
            "authority_handoff:payload_ref:dice_state",
        ],
        &[
            "runtime_event:perform-success",
            "runtime_event:atomic-cut",
            "place_record:dice_state:publish_roll_result@dice_state",
            "place_record:dice_state:handoff_dice_authority@dice_state",
            "debug_output:observer_debug_text_output:late_join_view: player_c sees result+owner history",
        ],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.order_handoff.runner_cli",
            "compare_floor:current_l2.verifier_preview_alignment",
            "compare_floor:current_l2.authoritative_room.vertical_slice",
        ],
        &[
            "guard:authoritative_room_default_profile",
            "guard:late_join_history_visible_as_past",
        ],
    );
}

#[test]
fn authoritative_room_vertical_slice_actualization_reaches_stale_reconnect_profile() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let slice = build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        slice.source_report.sample_id,
        "p08-dice-stale-reconnect-refresh"
    );
    assert_vertical_slice(
        &slice,
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
            "relation_family:program_order",
            "relation_family:observation_order",
            "relation_family:witness_order",
            "relation_family:finalization_order",
            "relation_family:scoped_happens_before",
        ],
        &[
            "authority_handoff:owner_slot:single_room_authority",
            "authority_handoff:stage_family:authoritative_serial_transition",
            "authority_handoff:stage_sequence:fail_then_refresh",
            "authority_handoff:payload_ref:dice_state",
        ],
        &[
            "runtime_event:perform-failure",
            "runtime_event:rollback",
            "runtime_event:perform-success",
            "place_record:dice_state:refresh_owner_snapshot@dice_state",
            "debug_output:reconnect_debug_text_output:refresh_owner_snapshot: stale reconnect redirected",
        ],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.order_handoff.runner_cli",
            "compare_floor:current_l2.verifier_preview_alignment",
            "compare_floor:current_l2.authoritative_room.vertical_slice",
        ],
        &[
            "guard:authoritative_room_default_profile",
            "guard:stale_reconnect_fail_then_refresh",
        ],
    );
}

#[test]
fn authoritative_room_vertical_slice_actualization_keeps_guarded_chain_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let slice = build_current_l2_source_sample_authoritative_room_vertical_slice_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        slice.source_report.sample_id,
        "p05-dice-owner-guarded-chain"
    );
    assert_eq!(
        slice.slice_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        slice
            .slice_guard_reason
            .as_ref()
            .unwrap()
            .contains("authoritative-room vertical slice")
    );
    assert!(slice.profile_axis_refs.is_empty());
    assert!(slice.relation_refs.is_empty());
    assert!(slice.authority_handoff_refs.is_empty());
    assert!(slice.runtime_evidence_refs.is_empty());
    assert!(slice.repo_local_emitted_artifact_refs.is_empty());
    assert_eq!(
        slice.compare_floor_refs,
        vec!["compare_floor:current_l2.authoritative_room.guard_only".to_string()]
    );
    assert_eq!(
        slice.guard_refs,
        vec!["guard:room_default_slice_not_reached".to_string()]
    );
}
