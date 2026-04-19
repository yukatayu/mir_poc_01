use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate,
    build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate,
    build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate,
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

fn assert_coupled_later_gate_matches_public_contract_floor(
    coupled_gate: &CurrentL2SourceSampleWitnessProviderPublicSchemaCoupledLaterGate,
    public_contract: &CurrentL2SourceSampleWitnessProviderEmittedContractCoupledLaterGate,
    expected_witness_schema_candidate_refs: &[&str],
    expected_provider_receipt_candidate_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(coupled_gate.coupled_status, public_contract.coupled_status);
    assert_eq!(coupled_gate.profile_axis_refs, public_contract.profile_axis_refs);
    assert_eq!(
        coupled_gate.witness_schema_candidate_refs,
        expected_witness_schema_candidate_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_gate.provider_receipt_candidate_refs,
        expected_provider_receipt_candidate_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_gate.combined_public_contract_candidate_refs,
        match coupled_gate.coupled_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "combined_public_contract_candidate:{}:witness_provider_routes_noncollapsed",
                    coupled_gate.source_report.sample_id
                ),
                format!(
                    "combined_public_contract_candidate:{}:repo_local_emitted_artifact_refs_first",
                    coupled_gate.source_report.sample_id
                ),
                format!(
                    "combined_public_contract_candidate:{}:combined_public_contract_candidate_only",
                    coupled_gate.source_report.sample_id
                ),
                format!(
                    "combined_public_contract_candidate:{}:final_emitted_handoff_contract_adjacent_keep",
                    coupled_gate.source_report.sample_id
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.coupled_default_refs,
        match coupled_gate.coupled_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "public_schema_coupled_default:claim_payload_split_first".to_string(),
                "public_schema_coupled_default:witness_provider_routes_noncollapsed".to_string(),
                "public_schema_coupled_default:combined_public_contract_candidate_only"
                    .to_string(),
                "public_schema_coupled_default:final_emitted_handoff_contract_adjacent_keep"
                    .to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_gate.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        coupled_gate.kept_later_refs,
        vec![
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );

    match coupled_gate.coupled_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(coupled_gate.coupled_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(coupled_gate
                .coupled_guard_reason
                .as_ref()
                .unwrap()
                .contains("witness/provider public-schema coupled later gate"));
        }
    }
}

#[test]
fn witness_provider_public_schema_coupled_later_gate_reaches_late_join_witness_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let public_contract =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_public_contract_floor(
        &coupled_gate,
        &public_contract,
        &[
            "witness_schema_candidate:p07-dice-late-join-visible-history:claim_payload_split_first",
            "witness_schema_candidate:p07-dice-late-join-visible-history:witness_route_noncollapsed",
            "witness_schema_candidate:p07-dice-late-join-visible-history:symbolic_binding_ref_keep",
            "witness_schema_candidate:p07-dice-late-join-visible-history:final_public_witness_schema_later",
        ],
        &[],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.auditable_authority_witness.strengthening",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
        ],
        &[
            "guard:final_public_witness_provider_schema_candidate_only",
            "guard:combined_public_contract_candidate_only",
            "guard:final_emitted_handoff_contract_adjacent_keep",
            "guard:exhaustive_shared_space_catalog_later",
        ],
    );
}

#[test]
fn witness_provider_public_schema_coupled_later_gate_reaches_stale_reconnect_baseline() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let public_contract =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_public_contract_floor(
        &coupled_gate,
        &public_contract,
        &[],
        &[],
        &[
            "compare_floor:current_l2.authoritative_room.vertical_slice",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.order_handoff.surface_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
        ],
        &[
            "guard:final_public_witness_provider_schema_candidate_only",
            "guard:combined_public_contract_candidate_only",
            "guard:final_emitted_handoff_contract_adjacent_keep",
            "guard:exhaustive_shared_space_catalog_later",
        ],
    );
}

#[test]
fn witness_provider_public_schema_coupled_later_gate_reaches_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let public_contract =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_public_contract_floor(
        &coupled_gate,
        &public_contract,
        &[],
        &[
            "provider_receipt_candidate:p09-dice-delegated-rng-provider-placement:provider_route_noncollapsed",
            "provider_receipt_candidate:p09-dice-delegated-rng-provider-placement:optional_provider_attachment_keep",
            "provider_receipt_candidate:p09-dice-delegated-rng-provider-placement:delegated_provider_attestation_adjacent_keep",
            "provider_receipt_candidate:p09-dice-delegated-rng-provider-placement:final_public_provider_receipt_schema_later",
        ],
        &[
            "compare_floor:current_l2.delegated_rng_service.practical",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_threshold",
            "compare_floor:current_l2.witness_provider_artifact.public_shape_actual_adoption",
            "compare_floor:current_l2.witness_provider_emitted_contract.coupled_later_gate",
            "compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate",
        ],
        &[
            "guard:final_public_witness_provider_schema_candidate_only",
            "guard:combined_public_contract_candidate_only",
            "guard:final_emitted_handoff_contract_adjacent_keep",
            "guard:exhaustive_shared_space_catalog_later",
        ],
    );
}

#[test]
fn witness_provider_public_schema_coupled_later_gate_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let public_contract =
        build_current_l2_source_sample_witness_provider_emitted_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_witness_provider_public_schema_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_public_contract_floor(
        &coupled_gate,
        &public_contract,
        &[],
        &[],
        &["compare_floor:current_l2.witness_provider_public_schema.coupled_later_gate_guard_only"],
        &["guard:witness_provider_public_schema_coupled_later_gate_not_reached"],
    );
}
