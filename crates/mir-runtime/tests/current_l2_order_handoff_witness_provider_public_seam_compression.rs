use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface,
    CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption,
    CurrentL2SourceSampleOrderHandoffWitnessProviderPublicSeamCompression,
    CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge,
    CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold,
    build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface,
    build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption,
    build_current_l2_source_sample_order_handoff_witness_provider_public_seam_compression,
    build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge,
    build_current_l2_source_sample_witness_provider_final_public_contract_reopen_threshold,
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

fn extend_unique_refs(target: &mut Vec<String>, refs: &[String]) {
    for reference in refs {
        if !target.contains(reference) {
            target.push(reference.clone());
        }
    }
}

fn assert_public_seam_compression_matches_prior_floors(
    compression: &CurrentL2SourceSampleOrderHandoffWitnessProviderPublicSeamCompression,
    route: &CurrentL2SourceSampleOrderHandoffSourceWordingRouteActualAdoption,
    serial: &CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface,
    trace: &CurrentL2SourceSampleWitnessProviderEmittedContractTraceAlignmentBridge,
    threshold: &CurrentL2SourceSampleWitnessProviderFinalPublicContractReopenThreshold,
    expected_trace_alignment_pairs: &[&str],
) {
    assert_eq!(compression.compression_status, route.actualization_status);
    assert_eq!(compression.compression_status, serial.surface_status);
    assert_eq!(compression.compression_status, trace.alignment_status);
    assert_eq!(compression.compression_status, threshold.threshold_status);
    assert_eq!(compression.profile_axis_refs, route.profile_axis_refs);
    assert_eq!(
        compression.source_wording_route_refs,
        route.source_wording_route_refs
    );
    assert_eq!(
        compression.emitted_artifact_candidate_keep_refs,
        route.emitted_artifact_candidate_keep_refs
    );
    assert_eq!(compression.serial_scope_lines, serial.serial_scope_lines);
    assert_eq!(
        compression.witness_schema_route_refs,
        threshold.witness_schema_route_refs
    );
    assert_eq!(
        compression.provider_receipt_route_refs,
        threshold.provider_receipt_route_refs
    );
    assert_eq!(
        compression.combined_public_contract_keep_refs,
        threshold.combined_public_contract_keep_refs
    );
    assert_eq!(
        compression.trace_alignment_pair_refs,
        expected_trace_alignment_pairs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );

    let mut expected_repo_local_emitted_artifact_refs =
        route.repo_local_emitted_artifact_refs.clone();
    extend_unique_refs(
        &mut expected_repo_local_emitted_artifact_refs,
        &serial.repo_local_emitted_artifact_refs,
    );
    extend_unique_refs(
        &mut expected_repo_local_emitted_artifact_refs,
        &trace.repo_local_emitted_artifact_refs,
    );
    extend_unique_refs(
        &mut expected_repo_local_emitted_artifact_refs,
        &threshold.repo_local_emitted_artifact_refs,
    );
    assert_eq!(
        compression.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
    );

    let mut expected_compare_floor_refs = route.compare_floor_refs.clone();
    extend_unique_refs(&mut expected_compare_floor_refs, &serial.compare_floor_refs);
    extend_unique_refs(&mut expected_compare_floor_refs, &trace.compare_floor_refs);
    extend_unique_refs(
        &mut expected_compare_floor_refs,
        &threshold.compare_floor_refs,
    );
    extend_unique_refs(
        &mut expected_compare_floor_refs,
        &[
            "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression"
                .to_string(),
        ],
    );
    assert_eq!(compression.compare_floor_refs, expected_compare_floor_refs);

    let sample_id = &compression.source_report.sample_id;
    assert_eq!(
        compression.public_seam_residual_refs,
        vec![
            format!(
                "order_handoff_public_seam_residual:{sample_id}:final_source_surface_handoff_wording_later"
            ),
            format!(
                "order_handoff_public_seam_residual:{sample_id}:final_emitted_artifact_schema_later"
            ),
            format!("shared_space_public_seam_residual:{sample_id}:public_schema_pair_first"),
            format!(
                "shared_space_public_seam_residual:{sample_id}:delegated_attestation_and_combined_contract_second"
            ),
            format!(
                "shared_space_public_seam_residual:{sample_id}:final_emitted_handoff_contract_third"
            ),
        ]
    );
    assert_eq!(
        compression.guard_refs,
        vec![
            "guard:edge_row_vertical_continuation_principal".to_string(),
            "guard:serial_scope_reserve_surface_only".to_string(),
            "guard:witness_provider_trace_alignment_bridge".to_string(),
            "guard:public_schema_pair_first".to_string(),
            "guard:delegated_attestation_and_combined_contract_second".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
            "guard:final_emitted_artifact_schema_later".to_string(),
            "guard:final_emitted_handoff_contract_third".to_string(),
        ]
    );
    assert_eq!(
        compression.kept_later_refs,
        vec![
            "kept_later:final_parser_grammar".to_string(),
            "kept_later:final_public_parser_checker_runtime_api".to_string(),
            "kept_later:final_source_surface_handoff_wording".to_string(),
            "kept_later:final_emitted_artifact_schema".to_string(),
            "kept_later:final_public_witness_schema".to_string(),
            "kept_later:final_public_provider_receipt_schema".to_string(),
            "kept_later:delegated_provider_attestation".to_string(),
            "kept_later:combined_provider_witness_public_contract".to_string(),
            "kept_later:final_emitted_handoff_contract".to_string(),
            "kept_later:authoritative_room_serial_scope_sugar".to_string(),
            "kept_later:low_level_memory_order_source_surface".to_string(),
            "kept_later:final_modal_foundation_adoption".to_string(),
            "kept_later:exhaustive_shared_space_catalog".to_string(),
        ]
    );
    assert!(compression.compression_guard_reason.is_none());
}

#[test]
fn order_handoff_witness_provider_public_seam_compression_reaches_late_join_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let route = build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let serial = build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let trace =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
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
    let compression =
        build_current_l2_source_sample_order_handoff_witness_provider_public_seam_compression(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_public_seam_compression_matches_prior_floors(
        &compression,
        &route,
        &serial,
        &trace,
        &threshold,
        &[
            "witness_provider_emitted_contract_trace_alignment_pair:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn order_handoff_witness_provider_public_seam_compression_reaches_stale_reconnect_sample() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let route = build_current_l2_source_sample_order_handoff_source_wording_route_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let serial = build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let trace =
        build_current_l2_source_sample_witness_provider_emitted_contract_trace_alignment_bridge(
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
    let compression =
        build_current_l2_source_sample_order_handoff_witness_provider_public_seam_compression(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_public_seam_compression_matches_prior_floors(
        &compression,
        &route,
        &serial,
        &trace,
        &threshold,
        &[
            "witness_provider_emitted_contract_trace_alignment_pair:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn order_handoff_witness_provider_public_seam_compression_keeps_guarded_chain_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let compression =
        build_current_l2_source_sample_order_handoff_witness_provider_public_seam_compression(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        compression.compression_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        compression
            .compression_guard_reason
            .as_ref()
            .unwrap()
            .contains("order-handoff/witness-provider public seam compression")
    );
    assert!(compression.profile_axis_refs.is_empty());
    assert!(compression.repo_local_emitted_artifact_refs.is_empty());
    assert!(compression.trace_alignment_pair_refs.is_empty());
    assert!(compression.public_seam_residual_refs.is_empty());
    assert_eq!(
        compression.compare_floor_refs,
        vec![
            "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression.guard_only"
                .to_string()
        ]
    );
    assert_eq!(
        compression.guard_refs,
        vec![
            "guard:order_handoff_witness_provider_public_seam_compression_not_reached".to_string()
        ]
    );
}
