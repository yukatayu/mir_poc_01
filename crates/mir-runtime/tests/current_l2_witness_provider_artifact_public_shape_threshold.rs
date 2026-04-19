use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold,
    build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold,
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
    threshold: &CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold,
    expected_profile_axis_refs: &[&str],
    expected_witness_attachment_refs: &[&str],
    expected_provider_attachment_refs: &[&str],
    expected_emitted_artifact_reserve_refs: &[&str],
    expected_compare_floor_refs: &[&str],
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
        threshold.witness_attachment_refs,
        expected_witness_attachment_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.provider_attachment_refs,
        expected_provider_attachment_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.emitted_artifact_reserve_refs,
        expected_emitted_artifact_reserve_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.threshold_default_refs,
        vec![
            "public_shape_threshold_default:claim_payload_split_first".to_string(),
            "public_shape_threshold_default:repo_local_emitted_artifact_refs_first".to_string(),
            "public_shape_threshold_default:optional_attachment_refs_only".to_string(),
            "public_shape_threshold_default:combined_public_contract_later".to_string(),
        ]
    );
    assert_eq!(
        threshold.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.contrast_refs,
        vec![
            "contrast_target:append_friendly_notice_room".to_string(),
            "contrast_target:note_only_witness".to_string(),
            "contrast_target:delegated_provider_attestation".to_string(),
            "contrast_target:combined_public_contract".to_string(),
        ]
    );
    assert_eq!(
        threshold.guard_refs,
        vec![
            "guard:public_shape_threshold_only".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:optional_attachment_refs_only".to_string(),
            "guard:combined_public_contract_later".to_string(),
        ]
    );
    assert_eq!(
        threshold.kept_later_refs,
        vec![
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );
}

#[test]
fn witness_provider_artifact_public_shape_threshold_reaches_late_join_witness_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
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
            "witness_public_shape_reserve:witness_kind",
            "witness_public_shape_reserve:action_ref",
            "witness_public_shape_reserve:draw_slot",
            "witness_public_shape_reserve:draw_result_binding",
        ],
        &[],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
        ],
    );
}

#[test]
fn witness_provider_artifact_public_shape_threshold_reaches_stale_reconnect_baseline() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        threshold.source_report.sample_id,
        "p08-dice-stale-reconnect-refresh"
    );
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
        &[],
        &[],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
        ],
    );
}

#[test]
fn witness_provider_artifact_public_shape_threshold_reaches_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        threshold.source_report.sample_id,
        "p09-dice-delegated-rng-provider-placement"
    );
    assert_threshold_reached(
        &threshold,
        &[
            "profile_axis:activation:authority-ack",
            "profile_axis:authority_placement:single_room_authority",
            "profile_axis:consistency_mode:authoritative_serial_transition",
            "profile_axis:rng_source:delegated_rng_service",
            "profile_axis:fairness_claim:opaque_authority_trust",
        ],
        &[],
        &[
            "optional_attachment:provider_draw_ref",
            "optional_attachment:provider_receipt_ref",
        ],
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference",
            "repo_local_emitted_artifact:model_check_concrete_carrier:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
        ],
    );
}

#[test]
fn witness_provider_artifact_public_shape_threshold_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        threshold.source_report.sample_id,
        "p05-dice-owner-guarded-chain"
    );
    assert_eq!(
        threshold.threshold_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        threshold
            .threshold_guard_reason
            .as_ref()
            .unwrap()
            .contains("public-shape threshold")
    );
    assert!(threshold.profile_axis_refs.is_empty());
    assert!(threshold.witness_attachment_refs.is_empty());
    assert!(threshold.provider_attachment_refs.is_empty());
    assert!(threshold.emitted_artifact_reserve_refs.is_empty());
    assert!(threshold.threshold_default_refs.is_empty());
    assert_eq!(
        threshold.compare_floor_refs,
        vec![
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold_guard_only"
                .to_string()
        ]
    );
    assert_eq!(
        threshold.guard_refs,
        vec!["guard:witness_provider_artifact_threshold_not_reached".to_string()]
    );
}
