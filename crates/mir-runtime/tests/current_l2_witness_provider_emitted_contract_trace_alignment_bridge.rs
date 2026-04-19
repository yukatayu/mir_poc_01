use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge,
    CurrentL2SourceSampleWitnessProviderRouteActualAdoption,
    build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate,
    build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge,
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

fn assert_trace_alignment_matches_prior_routes(
    bridge: &CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge,
    route_actual_adoption: &CurrentL2SourceSampleWitnessProviderRouteActualAdoption,
    coupled_later_gate: &CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    expected_pairs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        bridge.alignment_status,
        route_actual_adoption.actualization_status
    );
    assert_eq!(bridge.alignment_status, coupled_later_gate.coupled_status);
    assert_eq!(
        bridge.profile_axis_refs,
        route_actual_adoption.profile_axis_refs
    );
    assert_eq!(
        bridge.profile_axis_refs,
        coupled_later_gate.profile_axis_refs
    );
    assert_eq!(
        bridge.repo_local_emitted_artifact_refs,
        route_actual_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        bridge.repo_local_emitted_artifact_refs,
        coupled_later_gate.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        bridge.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        bridge.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        bridge.kept_later_refs,
        vec![
            "kept_later:trace_alignment_beyond_representative_corpus".to_string(),
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );

    match bridge.alignment_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let expected_pairs = expected_pairs
                .iter()
                .map(|entry| entry.to_string())
                .collect::<Vec<_>>();
            assert!(bridge.alignment_guard_reason.is_none());
            assert_eq!(bridge.route_pair_refs, expected_pairs);
            assert_eq!(bridge.emitted_contract_pair_refs, expected_pairs);
            assert_eq!(bridge.matched_pair_refs, expected_pairs);
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                bridge
                    .alignment_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("trace alignment bridge")
            );
            assert!(bridge.route_pair_refs.is_empty());
            assert!(bridge.emitted_contract_pair_refs.is_empty());
            assert!(bridge.matched_pair_refs.is_empty());
        }
    }
}

#[test]
fn witness_provider_trace_alignment_bridge_reaches_late_join_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let route_actual_adoption =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_later_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let bridge =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_trace_alignment_matches_prior_routes(
        &bridge,
        &route_actual_adoption,
        &coupled_later_gate,
        &[
            "witness_provider_emitted_contract_trace_alignment_pair:p07-dice-late-join-visible-history:rollback_cut_non_interference",
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
            "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_final_public_contract_promotion",
            "guard:no_final_public_schema_promotion",
        ],
    );
}

#[test]
fn witness_provider_trace_alignment_bridge_reaches_stale_reconnect_sample() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let route_actual_adoption =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_later_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let bridge =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_trace_alignment_matches_prior_routes(
        &bridge,
        &route_actual_adoption,
        &coupled_later_gate,
        &[
            "witness_provider_emitted_contract_trace_alignment_pair:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_final_public_contract_promotion",
            "guard:no_final_public_schema_promotion",
        ],
    );
}

#[test]
fn witness_provider_trace_alignment_bridge_reaches_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let route_actual_adoption =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_later_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let bridge =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_trace_alignment_matches_prior_routes(
        &bridge,
        &route_actual_adoption,
        &coupled_later_gate,
        &[
            "witness_provider_emitted_contract_trace_alignment_pair:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_route_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_final_public_contract_promotion",
            "guard:no_final_public_schema_promotion",
        ],
    );
}

#[test]
fn witness_provider_trace_alignment_bridge_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let route_actual_adoption =
        build_current_l2_source_sample_witness_provider_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_later_gate =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let bridge =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_trace_alignment_matches_prior_routes(
        &bridge,
        &route_actual_adoption,
        &coupled_later_gate,
        &[],
        &[
            "compare_floor:current_l2.witness_provider_emitted_contract.trace_alignment_bridge_guard_only",
        ],
        &["guard:witness_provider_emitted_contract_trace_alignment_not_reached"],
    );
}
