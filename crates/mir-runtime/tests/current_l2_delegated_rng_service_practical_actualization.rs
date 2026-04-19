use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
    build_current_l2_source_sample_delegated_rng_service_practical_actualization,
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

fn assert_practical_actualization(
    actualization: &CurrentL2SourceSampleDelegatedRngServicePracticalActualization,
    expected_profile_axis_refs: &[&str],
    expected_provider_boundary_refs: &[&str],
    expected_optional_attachment_refs: &[&str],
    expected_runtime_evidence_refs: &[&str],
    expected_repo_local_emitted_artifact_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        actualization.practical_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(actualization.practical_guard_reason.is_none());
    assert_eq!(
        actualization.profile_axis_refs,
        expected_profile_axis_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.provider_boundary_refs,
        expected_provider_boundary_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.optional_attachment_refs,
        expected_optional_attachment_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.runtime_evidence_refs,
        expected_runtime_evidence_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.contrast_refs,
        vec![
            "contrast_target:authority_rng_baseline".to_string(),
            "contrast_target:delegated_provider_attestation".to_string(),
            "contrast_target:distributed_randomness_provider".to_string(),
        ]
    );
    assert_eq!(
        actualization.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actualization.kept_later_refs,
        vec![
            "kept_later:auditable_authority_witness_combination".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:distributed_randomness_provider".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:control_plane_separated_carrier".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );
}

#[test]
fn delegated_rng_service_practical_actualization_reaches_provider_placement_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let actualization =
        build_current_l2_source_sample_delegated_rng_service_practical_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        actualization.source_report.sample_id,
        "p09-dice-delegated-rng-provider-placement"
    );
    assert_practical_actualization(
        &actualization,
        &[
            "profile_axis:activation:authority-ack",
            "profile_axis:authority_placement:single_room_authority",
            "profile_axis:consistency_mode:authoritative_serial_transition",
            "profile_axis:rng_source:delegated_rng_service",
            "profile_axis:fairness_claim:opaque_authority_trust",
        ],
        &[
            "provider_boundary:placement:delegated_rng_service",
            "provider_boundary:authority_keeps_commit",
            "provider_boundary:provider_returns_draw_not_state_transition",
            "provider_boundary:room_state_mutation_by_authority",
            "provider_boundary:witness_core_unchanged",
        ],
        &[
            "optional_attachment:provider_draw_ref",
            "optional_attachment:provider_receipt_ref",
        ],
        &[
            "runtime_event:perform-success",
            "runtime_event:atomic-cut",
            "place_record:dice_state:fetch_provider_roll@delegated_rng",
            "place_record:dice_state:provider_draw_ref@delegated_rng",
            "place_record:dice_state:publish_roll_result@dice_state",
            "place_record:dice_state:handoff_dice_authority@dice_state",
            "debug_output:provider_debug_text_output:delegated_rng_service.draw -> primary",
        ],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.order_handoff.runner_cli",
            "compare_floor:current_l2.delegated_rng_service.practical",
        ],
        &[
            "guard:provider_placement_only",
            "guard:authority_keeps_commit",
            "guard:provider_receipt_optional",
            "guard:fairness_claim_not_collapsed_with_attestation",
        ],
    );
}

#[test]
fn delegated_rng_service_practical_actualization_keeps_authority_rng_baseline_as_guard_only() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let actualization =
        build_current_l2_source_sample_delegated_rng_service_practical_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        actualization.source_report.sample_id,
        "p07-dice-late-join-visible-history"
    );
    assert_eq!(
        actualization.practical_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(actualization
        .practical_guard_reason
        .as_ref()
        .unwrap()
        .contains("delegated provider"));
    assert!(actualization.profile_axis_refs.is_empty());
    assert!(actualization.provider_boundary_refs.is_empty());
    assert!(actualization.optional_attachment_refs.is_empty());
    assert!(actualization.runtime_evidence_refs.is_empty());
    assert!(actualization.repo_local_emitted_artifact_refs.is_empty());
    assert_eq!(
        actualization.compare_floor_refs,
        vec!["compare_floor:current_l2.delegated_rng_service.guard_only".to_string()]
    );
    assert_eq!(
        actualization.guard_refs,
        vec!["guard:delegated_provider_slice_not_reached".to_string()]
    );
}

#[test]
fn delegated_rng_service_practical_actualization_keeps_stale_reconnect_baseline_as_guard_only() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let actualization =
        build_current_l2_source_sample_delegated_rng_service_practical_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        actualization.source_report.sample_id,
        "p08-dice-stale-reconnect-refresh"
    );
    assert_eq!(
        actualization.practical_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(actualization
        .practical_guard_reason
        .as_ref()
        .unwrap()
        .contains("delegated provider"));
    assert!(actualization.profile_axis_refs.is_empty());
    assert!(actualization.provider_boundary_refs.is_empty());
    assert!(actualization.optional_attachment_refs.is_empty());
    assert!(actualization.runtime_evidence_refs.is_empty());
    assert!(actualization.repo_local_emitted_artifact_refs.is_empty());
    assert_eq!(
        actualization.compare_floor_refs,
        vec!["compare_floor:current_l2.delegated_rng_service.guard_only".to_string()]
    );
    assert_eq!(
        actualization.guard_refs,
        vec!["guard:delegated_provider_slice_not_reached".to_string()]
    );
}
