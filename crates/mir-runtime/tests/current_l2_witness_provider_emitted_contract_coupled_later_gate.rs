use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption,
    CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption,
    build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate,
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

fn assert_coupled_later_gate_matches_public_shape_adoption(
    coupled_later_gate: &CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    public_shape_adoption: &CurrentL2SourceSampleWitnessProviderArtifactPublicShapeActualAdoption,
    expected_witness_contract_candidate_refs: &[&str],
    expected_provider_contract_candidate_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        coupled_later_gate.coupled_status,
        public_shape_adoption.actualization_status
    );
    assert_eq!(
        coupled_later_gate.profile_axis_refs,
        public_shape_adoption.profile_axis_refs
    );
    assert_eq!(
        coupled_later_gate.repo_local_emitted_artifact_refs,
        public_shape_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        coupled_later_gate.witness_contract_candidate_refs,
        expected_witness_contract_candidate_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_later_gate.provider_contract_candidate_refs,
        expected_provider_contract_candidate_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_later_gate.emitted_contract_candidate_refs,
        match coupled_later_gate.coupled_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "emitted_handoff_contract_candidate:{}:repo_local_emitted_artifact_refs_first",
                    coupled_later_gate.source_report.sample_id
                ),
                format!(
                    "emitted_handoff_contract_candidate:{}:public_contract_adjacent_not_collapsed",
                    coupled_later_gate.source_report.sample_id
                ),
                format!(
                    "emitted_handoff_contract_candidate:{}:final_emitted_handoff_contract_later",
                    coupled_later_gate.source_report.sample_id
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_later_gate.coupled_default_refs,
        match coupled_later_gate.coupled_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "public_contract_coupled_later_default:claim_payload_split_first".to_string(),
                "public_contract_coupled_later_default:repo_local_emitted_artifact_refs_first"
                    .to_string(),
                "public_contract_coupled_later_default:witness_provider_routes_noncollapsed"
                    .to_string(),
                "public_contract_coupled_later_default:combined_public_contract_later".to_string(),
                "public_contract_coupled_later_default:final_emitted_handoff_contract_later"
                    .to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_later_gate.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_later_gate.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_later_gate.kept_later_refs,
        vec![
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:final_source_surface_handoff_wording".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );

    match coupled_later_gate.coupled_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(coupled_later_gate.coupled_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                coupled_later_gate
                    .coupled_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("witness/provider emitted-contract coupled later gate")
            );
        }
    }
}

#[test]
fn witness_provider_emitted_contract_coupled_later_gate_reaches_late_join_witness_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let public_shape_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
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

    assert_coupled_later_gate_matches_public_shape_adoption(
        &coupled_later_gate,
        &public_shape_adoption,
        &[
            "witness_public_contract_candidate:p07-dice-late-join-visible-history:claim_payload_split_first",
            "witness_public_contract_candidate:p07-dice-late-join-visible-history:witness_route_noncollapsed",
            "witness_public_contract_candidate:p07-dice-late-join-visible-history:final_public_witness_schema_later",
            "witness_public_contract_candidate:p07-dice-late-join-visible-history:combined_public_contract_later",
        ],
        &[],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
        ],
        &[
            "guard:public_contract_coupled_later_gate_only",
            "guard:claim_payload_split_first",
            "guard:repo_local_emitted_artifact_refs_first",
            "guard:witness_provider_routes_noncollapsed",
            "guard:combined_public_contract_later",
            "guard:final_emitted_handoff_contract_later",
        ],
    );
}

#[test]
fn witness_provider_emitted_contract_coupled_later_gate_reaches_stale_reconnect_baseline() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let public_shape_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
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

    assert_coupled_later_gate_matches_public_shape_adoption(
        &coupled_later_gate,
        &public_shape_adoption,
        &[],
        &[],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
        ],
        &[
            "guard:public_contract_coupled_later_gate_only",
            "guard:claim_payload_split_first",
            "guard:repo_local_emitted_artifact_refs_first",
            "guard:witness_provider_routes_noncollapsed",
            "guard:combined_public_contract_later",
            "guard:final_emitted_handoff_contract_later",
        ],
    );
}

#[test]
fn witness_provider_emitted_contract_coupled_later_gate_reaches_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let public_shape_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
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

    assert_coupled_later_gate_matches_public_shape_adoption(
        &coupled_later_gate,
        &public_shape_adoption,
        &[],
        &[
            "provider_public_contract_candidate:p09-dice-delegated-rng-provider-placement:optional_provider_attachment_keep",
            "provider_public_contract_candidate:p09-dice-delegated-rng-provider-placement:provider_route_noncollapsed",
            "provider_public_contract_candidate:p09-dice-delegated-rng-provider-placement:delegated_provider_attestation_later",
            "provider_public_contract_candidate:p09-dice-delegated-rng-provider-placement:combined_public_contract_later",
        ],
        &[
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
        ],
        &[
            "guard:public_contract_coupled_later_gate_only",
            "guard:claim_payload_split_first",
            "guard:repo_local_emitted_artifact_refs_first",
            "guard:witness_provider_routes_noncollapsed",
            "guard:combined_public_contract_later",
            "guard:final_emitted_handoff_contract_later",
        ],
    );
}

#[test]
fn witness_provider_emitted_contract_coupled_later_gate_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let public_shape_adoption =
        build_current_l2_source_sample_witness_provider_artifact_public_shape_actual_adoption(
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

    assert_coupled_later_gate_matches_public_shape_adoption(
        &coupled_later_gate,
        &public_shape_adoption,
        &[],
        &[],
        &[
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate_guard_only",
        ],
        &["guard:witness_provider_emitted_contract_coupled_later_gate_not_reached"],
    );
}
