use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption,
    CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold,
    build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption,
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

fn assert_actual_adoption_matches_threshold(
    actual_adoption: &CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption,
    threshold: &CurrentL2SourceSampleWitnessProviderArtifactPublicShapeThreshold,
    expected_witness_route_refs: &[&str],
    expected_provider_route_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        actual_adoption.actualization_status,
        threshold.threshold_status
    );
    assert_eq!(
        actual_adoption.profile_axis_refs,
        threshold.profile_axis_refs
    );
    assert_eq!(
        actual_adoption.repo_local_emitted_artifact_refs,
        threshold.emitted_artifact_reserve_refs
    );
    assert_eq!(
        actual_adoption.witness_route_refs,
        expected_witness_route_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.provider_route_refs,
        expected_provider_route_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.actual_adoption_default_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "public_shape_actual_adoption_default:claim_payload_split_first".to_string(),
                "public_shape_actual_adoption_default:repo_local_emitted_artifact_refs_first"
                    .to_string(),
                "public_shape_actual_adoption_default:optional_attachment_refs_only".to_string(),
                "public_shape_actual_adoption_default:combined_public_contract_later".to_string(),
                "public_shape_actual_adoption_default:witness_provider_routes_noncollapsed"
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
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );

    match actual_adoption.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(actual_adoption.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                actual_adoption
                    .actualization_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("witness/provider/artifact public-shape actual adoption")
            );
        }
    }
}

#[test]
fn witness_provider_artifact_actual_adoption_reaches_late_join_witness_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[
            "witness_public_shape_route:p07-dice-late-join-visible-history:claim_payload_split_first",
            "witness_public_shape_route:p07-dice-late-join-visible-history:witness_attachment_refs_only",
            "witness_public_shape_route:p07-dice-late-join-visible-history:symbolic_binding_ref_keep",
            "witness_public_shape_route:p07-dice-late-join-visible-history:combined_public_contract_later",
        ],
        &[],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
        ],
        &[
            "guard:public_shape_actual_adoption_only",
            "guard:claim_payload_split_first",
            "guard:optional_attachment_refs_only",
            "guard:combined_public_contract_later",
            "guard:witness_provider_routes_noncollapsed",
        ],
    );
}

#[test]
fn witness_provider_artifact_actual_adoption_reaches_stale_reconnect_baseline() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[],
        &[],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
        ],
        &[
            "guard:public_shape_actual_adoption_only",
            "guard:claim_payload_split_first",
            "guard:optional_attachment_refs_only",
            "guard:combined_public_contract_later",
            "guard:witness_provider_routes_noncollapsed",
        ],
    );
}

#[test]
fn witness_provider_artifact_actual_adoption_reaches_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[],
        &[
            "provider_public_shape_route:p09-dice-delegated-rng-provider-placement:provider_attachment_refs_only",
            "provider_public_shape_route:p09-dice-delegated-rng-provider-placement:optional_provider_attachment_keep",
            "provider_public_shape_route:p09-dice-delegated-rng-provider-placement:delegated_provider_attestation_later",
            "provider_public_shape_route:p09-dice-delegated-rng-provider-placement:combined_public_contract_later",
        ],
        &[
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
        ],
        &[
            "guard:public_shape_actual_adoption_only",
            "guard:claim_payload_split_first",
            "guard:optional_attachment_refs_only",
            "guard:combined_public_contract_later",
            "guard:witness_provider_routes_noncollapsed",
        ],
    );
}

#[test]
fn witness_provider_artifact_actual_adoption_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let threshold =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[],
        &[],
        &[
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption_guard_only",
        ],
        &["guard:witness_provider_artifact_actual_adoption_not_reached"],
    );
}
