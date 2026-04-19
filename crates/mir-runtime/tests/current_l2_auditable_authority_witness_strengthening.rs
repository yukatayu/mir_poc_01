use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening,
    build_current_l2_source_sample_auditable_authority_witness_strengthening,
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

fn assert_strengthening(
    strengthening: &CurrentL2SourceSampleAuditableAuthorityWitnessStrengthening,
    expected_fairness_claim_refs: &[&str],
    expected_witness_core_refs: &[&str],
    expected_witness_binding_refs: &[&str],
    expected_runtime_evidence_refs: &[&str],
    expected_repo_local_emitted_artifact_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        strengthening.strengthening_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(strengthening.strengthening_guard_reason.is_none());
    assert_eq!(
        strengthening.fairness_claim_refs,
        expected_fairness_claim_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.witness_core_refs,
        expected_witness_core_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.witness_binding_refs,
        expected_witness_binding_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.runtime_evidence_refs,
        expected_runtime_evidence_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.contrast_refs,
        vec![
            "contrast_target:append_friendly_notice_room".to_string(),
            "contrast_target:note_only_witness".to_string(),
            "contrast_target:expanded_attested_receipt".to_string(),
        ]
    );
    assert_eq!(
        strengthening.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        strengthening.kept_later_refs,
        vec![
            "kept_later:delegated_rng_service".to_string(),
            "kept_later:provider_receipt_optional_attachment".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:distributed_randomness_provider".to_string(),
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );
}

#[test]
fn auditable_authority_witness_strengthening_reaches_late_join_authoritative_profile() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let strengthening = build_current_l2_source_sample_auditable_authority_witness_strengthening(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        strengthening.source_report.sample_id,
        "p07-dice-late-join-visible-history"
    );
    assert_strengthening(
        &strengthening,
        &["fairness_claim:auditable_authority_witness"],
        &[
            "witness_field:witness_kind",
            "witness_field:action_ref",
            "witness_field:draw_slot",
            "witness_field:draw_result",
        ],
        &[
            "witness_binding:witness_kind:authority_draw_witness",
            "witness_binding:action_ref:handoff_dice_authority@dice_state",
            "witness_binding:draw_slot:primary",
            "witness_binding:draw_result_binding:publish_roll_result@dice_state",
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
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
        ],
        &[
            "guard:room_profile_claim_payload_split",
            "guard:minimal_witness_core_only",
            "guard:witness_bearing_authoritative_sample_reached",
        ],
    );
}

#[test]
fn auditable_authority_witness_strengthening_keeps_stale_reconnect_profile_guard_only() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let strengthening = build_current_l2_source_sample_auditable_authority_witness_strengthening(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(strengthening.source_report.sample_id, "p08-dice-stale-reconnect-refresh");
    assert_eq!(
        strengthening.strengthening_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(strengthening
        .strengthening_guard_reason
        .as_ref()
        .unwrap()
        .contains("witness-bearing authoritative draw"));
    assert!(strengthening.fairness_claim_refs.is_empty());
    assert!(strengthening.witness_core_refs.is_empty());
    assert!(strengthening.witness_binding_refs.is_empty());
    assert!(strengthening.runtime_evidence_refs.is_empty());
    assert!(strengthening.repo_local_emitted_artifact_refs.is_empty());
    assert_eq!(
        strengthening.compare_floor_refs,
        vec!["compare_floor:current_l2.auditable_authority_witness.guard_only".to_string()]
    );
    assert_eq!(
        strengthening.guard_refs,
        vec!["guard:witness_strengthening_not_reached".to_string()]
    );
}

#[test]
fn auditable_authority_witness_strengthening_keeps_guarded_chain_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let strengthening = build_current_l2_source_sample_auditable_authority_witness_strengthening(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(strengthening.source_report.sample_id, "p05-dice-owner-guarded-chain");
    assert_eq!(
        strengthening.strengthening_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(strengthening
        .strengthening_guard_reason
        .as_ref()
        .unwrap()
        .contains("witness strengthening"));
    assert!(strengthening.fairness_claim_refs.is_empty());
    assert!(strengthening.witness_core_refs.is_empty());
    assert!(strengthening.witness_binding_refs.is_empty());
    assert!(strengthening.runtime_evidence_refs.is_empty());
    assert!(strengthening.repo_local_emitted_artifact_refs.is_empty());
    assert_eq!(
        strengthening.compare_floor_refs,
        vec!["compare_floor:current_l2.auditable_authority_witness.guard_only".to_string()]
    );
    assert_eq!(
        strengthening.guard_refs,
        vec!["guard:witness_strengthening_not_reached".to_string()]
    );
}
