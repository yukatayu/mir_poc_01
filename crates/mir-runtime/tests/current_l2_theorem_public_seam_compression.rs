use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold,
    CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge,
    CurrentL2SourceSampleTheoremPublicSeamCompression,
    build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold,
    build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge,
    build_current_l2_source_sample_theorem_public_seam_compression,
};

fn order_handoff_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

fn typed_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-typed-proof-model-check")
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

fn assert_theorem_public_seam_compression_matches_prior_floors(
    compression: &CurrentL2SourceSampleTheoremPublicSeamCompression,
    threshold: &CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold,
    bridge: &CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge,
    expected_alignment_pairs: &[&str],
) {
    assert_eq!(compression.compression_status, threshold.threshold_status);
    assert_eq!(compression.compression_status, bridge.alignment_status);
    assert_eq!(
        compression.actualization_subject_kind,
        threshold.actualization_subject_kind
    );
    assert_eq!(
        compression.actualization_subject_ref,
        threshold.actualization_subject_ref
    );
    assert_eq!(
        compression.actualization_subject_kind,
        bridge.alignment_subject_kind
    );
    assert_eq!(
        compression.actualization_subject_ref,
        bridge.alignment_subject_ref
    );
    assert_eq!(
        compression.result_object_route_refs,
        threshold.result_object_route_refs
    );
    assert_eq!(
        compression.payload_preview_keep_refs,
        threshold.payload_preview_keep_refs
    );
    assert_eq!(
        compression.proof_object_schema_candidate_refs,
        threshold.proof_object_schema_candidate_refs
    );
    assert_eq!(
        compression.prover_brand_candidate_refs,
        threshold.prover_brand_candidate_refs
    );
    assert_eq!(
        compression.lean_stub_alignment_refs,
        expected_alignment_pairs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );

    let mut expected_repo_local_emitted_artifact_refs =
        threshold.repo_local_emitted_artifact_refs.clone();
    extend_unique_refs(
        &mut expected_repo_local_emitted_artifact_refs,
        &bridge.repo_local_emitted_artifact_refs,
    );
    assert_eq!(
        compression.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
    );

    let mut expected_compare_floor_refs = threshold.compare_floor_refs.clone();
    extend_unique_refs(&mut expected_compare_floor_refs, &bridge.compare_floor_refs);
    extend_unique_refs(
        &mut expected_compare_floor_refs,
        &[
            "compare_floor:current_l2.theorem_actual_lean_execution_availability_probe".to_string(),
            "compare_floor:current_l2.theorem_public_seam_compression".to_string(),
        ],
    );
    assert_eq!(compression.compare_floor_refs, expected_compare_floor_refs);

    let subject_ref = compression.actualization_subject_ref.as_ref().unwrap();
    assert_eq!(
        compression.public_seam_residual_refs,
        vec![
            format!("theorem_public_seam_residual:{subject_ref}:result_object_and_payload_first"),
            format!(
                "theorem_public_seam_residual:{subject_ref}:prover_brand_and_proof_schema_second"
            ),
            format!(
                "theorem_public_seam_residual:{subject_ref}:final_public_verifier_contract_third"
            ),
            format!(
                "theorem_public_seam_residual:{subject_ref}:actual_lean_execution_environment_conditional"
            ),
        ]
    );
    assert_eq!(
        compression.environment_stop_line_refs,
        vec![
            "environment_probe:current_l2.theorem_actual_lean_execution_availability_probe"
                .to_string(),
            "environment_stop_line:actual_lean_execution_requires_local_toolchain".to_string(),
            "environment_stop_line:lean_stub_bridge_kept_until_local_toolchain_available"
                .to_string(),
        ]
    );
    assert_eq!(
        compression.guard_refs,
        vec![
            "guard:theorem_result_object_payload_pair_first".to_string(),
            "guard:theorem_prover_brand_proof_schema_pair_second".to_string(),
            "guard:final_public_verifier_contract_third".to_string(),
            "guard:actual_lean_execution_environment_conditional".to_string(),
            "guard:lean_stub_bridge_kept_until_local_toolchain_available".to_string(),
        ]
    );
    assert_eq!(
        compression.kept_later_refs,
        vec![
            "kept_later:final_public_theorem_result_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
            "kept_later:actual_lean_execution_with_concrete_toolchain".to_string(),
        ]
    );
    assert!(compression.compression_guard_reason.is_none());
}

#[test]
fn theorem_public_seam_compression_reaches_static_underdeclared_sample() {
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_theorem_public_seam_compression(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_theorem_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &bridge,
        &[
            "theorem_trace_alignment_pair:e5-underdeclared-lineage:canonical_normalization_law",
            "theorem_trace_alignment_pair:e5-underdeclared-lineage:no_re_promotion",
        ],
    );
}

#[test]
fn theorem_public_seam_compression_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_theorem_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_theorem_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &bridge,
        &[
            "theorem_trace_alignment_pair:p06-typed-proof-owner-handoff:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn theorem_public_seam_compression_reaches_late_join_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_theorem_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_theorem_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &bridge,
        &[
            "theorem_trace_alignment_pair:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn theorem_public_seam_compression_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_theorem_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_theorem_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &bridge,
        &[
            "theorem_trace_alignment_pair:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
    );
}

#[test]
fn theorem_public_seam_compression_keeps_guarded_prototype_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let compression = build_current_l2_source_sample_theorem_public_seam_compression(
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
            .contains("theorem public seam compression")
    );
    assert!(compression.repo_local_emitted_artifact_refs.is_empty());
    assert!(compression.lean_stub_alignment_refs.is_empty());
    assert!(compression.public_seam_residual_refs.is_empty());
    assert!(compression.environment_stop_line_refs.is_empty());
    assert_eq!(
        compression.compare_floor_refs,
        vec!["compare_floor:current_l2.theorem_public_seam_compression.guard_only".to_string()]
    );
    assert_eq!(
        compression.guard_refs,
        vec!["guard:theorem_public_seam_compression_not_reached".to_string()]
    );
}
