use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold,
    CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption,
    build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate,
    build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold,
    build_current_l2_source_sample_witness_provider_schema_route_actual_adoption,
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

fn assert_final_public_contract_reopen_threshold(
    threshold: &CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold,
    schema_route_actual_adoption: &CurrentL2SourceSampleWitnessProviderSchemaRouteActualAdoption,
    emitted_contract_coupled_gate: &CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    expected_reopen_sequence_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        threshold.threshold_status,
        schema_route_actual_adoption.actualization_status
    );
    assert_eq!(
        threshold.threshold_status,
        emitted_contract_coupled_gate.coupled_status
    );
    assert_eq!(
        threshold.profile_axis_refs,
        schema_route_actual_adoption.profile_axis_refs
    );
    assert_eq!(
        threshold.repo_local_emitted_artifact_refs,
        schema_route_actual_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        threshold.witness_schema_route_refs,
        schema_route_actual_adoption.witness_schema_route_refs
    );
    assert_eq!(
        threshold.provider_receipt_route_refs,
        schema_route_actual_adoption.provider_receipt_route_refs
    );
    assert_eq!(
        threshold.combined_public_contract_keep_refs,
        schema_route_actual_adoption.combined_public_contract_keep_refs
    );
    assert_eq!(
        threshold.final_public_contract_reopen_sequence_refs,
        expected_reopen_sequence_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.threshold_default_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "witness_provider_final_contract_reopen_default:public_schema_pair_first"
                    .to_string(),
                "witness_provider_final_contract_reopen_default:delegated_attestation_and_combined_contract_second"
                    .to_string(),
                "witness_provider_final_contract_reopen_default:final_emitted_handoff_contract_third"
                    .to_string(),
                "witness_provider_final_contract_reopen_default:exhaustive_shared_space_catalog_later"
                    .to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        threshold.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
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

    match threshold.threshold_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(threshold.threshold_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                threshold
                    .threshold_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("final public contract reopen threshold")
            );
        }
    }
}

#[test]
fn witness_provider_final_public_contract_reopen_threshold_reaches_late_join_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let schema_route_actual_adoption =
        build_current_l2_source_sample_witness_provider_schema_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let emitted_contract_coupled_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &schema_route_actual_adoption,
        &emitted_contract_coupled_gate,
        &[
            "witness_provider_final_contract_reopen:p07-dice-late-join-visible-history:public_schema_pair_first",
            "witness_provider_final_contract_reopen:p07-dice-late-join-visible-history:delegated_attestation_and_combined_contract_second",
            "witness_provider_final_contract_reopen:p07-dice-late-join-visible-history:final_emitted_handoff_contract_third",
            "witness_provider_final_contract_reopen:p07-dice-late-join-visible-history:exhaustive_shared_space_catalog_later",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_schema_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:public_schema_pair_first",
            "guard:delegated_attestation_and_combined_contract_second",
            "guard:final_emitted_handoff_contract_third",
            "guard:exhaustive_shared_space_catalog_later",
        ],
    );
}

#[test]
fn witness_provider_final_public_contract_reopen_threshold_reaches_stale_reconnect_sample() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let schema_route_actual_adoption =
        build_current_l2_source_sample_witness_provider_schema_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let emitted_contract_coupled_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &schema_route_actual_adoption,
        &emitted_contract_coupled_gate,
        &[
            "witness_provider_final_contract_reopen:p08-dice-stale-reconnect-refresh:public_schema_pair_first",
            "witness_provider_final_contract_reopen:p08-dice-stale-reconnect-refresh:delegated_attestation_and_combined_contract_second",
            "witness_provider_final_contract_reopen:p08-dice-stale-reconnect-refresh:final_emitted_handoff_contract_third",
            "witness_provider_final_contract_reopen:p08-dice-stale-reconnect-refresh:exhaustive_shared_space_catalog_later",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_schema_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:public_schema_pair_first",
            "guard:delegated_attestation_and_combined_contract_second",
            "guard:final_emitted_handoff_contract_third",
            "guard:exhaustive_shared_space_catalog_later",
        ],
    );
}

#[test]
fn witness_provider_final_public_contract_reopen_threshold_reaches_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let schema_route_actual_adoption =
        build_current_l2_source_sample_witness_provider_schema_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let emitted_contract_coupled_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &schema_route_actual_adoption,
        &emitted_contract_coupled_gate,
        &[
            "witness_provider_final_contract_reopen:p09-dice-delegated-rng-provider-placement:public_schema_pair_first",
            "witness_provider_final_contract_reopen:p09-dice-delegated-rng-provider-placement:delegated_attestation_and_combined_contract_second",
            "witness_provider_final_contract_reopen:p09-dice-delegated-rng-provider-placement:final_emitted_handoff_contract_third",
            "witness_provider_final_contract_reopen:p09-dice-delegated-rng-provider-placement:exhaustive_shared_space_catalog_later",
        ],
        &[
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_schema_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:public_schema_pair_first",
            "guard:delegated_attestation_and_combined_contract_second",
            "guard:final_emitted_handoff_contract_third",
            "guard:exhaustive_shared_space_catalog_later",
        ],
    );
}

#[test]
fn witness_provider_final_public_contract_reopen_threshold_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let schema_route_actual_adoption =
        build_current_l2_source_sample_witness_provider_schema_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let emitted_contract_coupled_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &schema_route_actual_adoption,
        &emitted_contract_coupled_gate,
        &[],
        &[
            "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold.guard_only",
        ],
        &["guard:witness_provider_final_public_contract_reopen_threshold_not_reached"],
    );
}
