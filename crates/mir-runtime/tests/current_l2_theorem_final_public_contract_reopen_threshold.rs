use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold,
    CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate,
    CurrentL2SourceSampleTheoremResultObjectActualAdoption,
    build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold,
    build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate,
    build_current_l2_source_sample_theorem_result_object_actual_adoption,
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

fn assert_final_public_contract_reopen_threshold(
    threshold: &CurrentL2SourceSampleTheoremFinalPublicContractReopenThreshold,
    result_object_actual_adoption: &CurrentL2SourceSampleTheoremResultObjectActualAdoption,
    proof_object_schema_coupled_gate: &CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        threshold.threshold_status,
        result_object_actual_adoption.actualization_status
    );
    assert_eq!(
        threshold.threshold_status,
        proof_object_schema_coupled_gate.actualization_status
    );
    assert_eq!(
        threshold.actualization_subject_kind,
        result_object_actual_adoption.actualization_subject_kind
    );
    assert_eq!(
        threshold.actualization_subject_ref,
        result_object_actual_adoption.actualization_subject_ref
    );
    assert_eq!(
        threshold.repo_local_emitted_artifact_refs,
        result_object_actual_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        threshold.result_object_route_refs,
        result_object_actual_adoption.result_object_route_refs
    );
    assert_eq!(
        threshold.payload_preview_keep_refs,
        result_object_actual_adoption.payload_preview_keep_refs
    );
    assert_eq!(
        threshold.proof_object_schema_candidate_refs,
        proof_object_schema_coupled_gate.proof_object_schema_candidate_refs
    );
    assert_eq!(
        threshold.prover_brand_candidate_refs,
        proof_object_schema_coupled_gate.prover_brand_candidate_refs
    );
    assert_eq!(
        threshold.final_public_contract_reopen_sequence_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_final_public_contract_reopen:{}:result_object_and_payload_first",
                    threshold.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_final_public_contract_reopen:{}:prover_brand_and_proof_schema_second",
                    threshold.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_final_public_contract_reopen:{}:final_public_verifier_contract_third",
                    threshold.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        threshold.threshold_default_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "theorem_final_public_contract_reopen_default:result_object_and_payload_first"
                    .to_string(),
                "theorem_final_public_contract_reopen_default:prover_brand_and_proof_schema_second"
                    .to_string(),
                "theorem_final_public_contract_reopen_default:final_public_verifier_contract_third"
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
            "kept_later:final_public_theorem_result_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
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
                    .contains("theorem final public contract reopen threshold")
            );
        }
    }
}

#[test]
fn theorem_final_public_contract_reopen_threshold_reaches_static_underdeclared_sample() {
    let result_object_actual_adoption =
        build_current_l2_source_sample_theorem_result_object_actual_adoption(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let proof_object_schema_coupled_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &result_object_actual_adoption,
        &proof_object_schema_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:result_object_and_payload_first",
            "guard:prover_brand_and_proof_schema_second",
            "guard:final_public_verifier_contract_third",
        ],
    );
}

#[test]
fn theorem_final_public_contract_reopen_threshold_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let result_object_actual_adoption =
        build_current_l2_source_sample_theorem_result_object_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let proof_object_schema_coupled_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &result_object_actual_adoption,
        &proof_object_schema_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:result_object_and_payload_first",
            "guard:prover_brand_and_proof_schema_second",
            "guard:final_public_verifier_contract_third",
        ],
    );
}

#[test]
fn theorem_final_public_contract_reopen_threshold_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let result_object_actual_adoption =
        build_current_l2_source_sample_theorem_result_object_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let proof_object_schema_coupled_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &result_object_actual_adoption,
        &proof_object_schema_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:result_object_and_payload_first",
            "guard:prover_brand_and_proof_schema_second",
            "guard:final_public_verifier_contract_third",
        ],
    );
}

#[test]
fn theorem_final_public_contract_reopen_threshold_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let result_object_actual_adoption =
        build_current_l2_source_sample_theorem_result_object_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let proof_object_schema_coupled_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &result_object_actual_adoption,
        &proof_object_schema_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold",
        ],
        &[
            "guard:result_object_and_payload_first",
            "guard:prover_brand_and_proof_schema_second",
            "guard:final_public_verifier_contract_third",
        ],
    );
}

#[test]
fn theorem_final_public_contract_reopen_threshold_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let result_object_actual_adoption =
        build_current_l2_source_sample_theorem_result_object_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let proof_object_schema_coupled_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold = build_current_l2_source_sample_theorem_final_public_contract_reopen_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &result_object_actual_adoption,
        &proof_object_schema_coupled_gate,
        &["compare_floor:current_l2.theorem_final_public_contract_reopen_threshold.guard_only"],
        &["guard:theorem_final_public_contract_reopen_threshold_not_reached"],
    );
}
