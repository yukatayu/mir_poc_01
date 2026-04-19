use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate,
    CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate,
    build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate,
    build_current_l2_source_sample_theorem_result_object_preview_actualization,
    build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate,
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

fn assert_coupled_later_gate_matches_prior_floors(
    coupled_gate: &CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate,
    result_object_preview: &CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    proof_object_gate: &CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        coupled_gate.actualization_status,
        result_object_preview.actualization_status
    );
    assert_eq!(coupled_gate.actualization_status, proof_object_gate.actualization_status);
    assert_eq!(
        coupled_gate.actualization_subject_kind,
        result_object_preview.actualization_subject_kind
    );
    assert_eq!(
        coupled_gate.actualization_subject_ref,
        result_object_preview.actualization_subject_ref
    );
    assert_eq!(
        coupled_gate.repo_local_emitted_artifact_refs,
        result_object_preview.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        coupled_gate.result_object_candidate_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_final_result_candidate:{}:notebook_consumer_object_first",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_final_result_candidate:{}:review_unit_transport_anchor",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_final_result_candidate:{}:repo_local_emitted_artifact_refs_first",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_final_result_candidate:{}:final_public_result_object_later",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.payload_public_contract_candidate_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_payload_public_contract_candidate:{}:consumer_shaped_payload_preview_keep",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_payload_public_contract_candidate:{}:notebook_consumer_contract_first",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_payload_public_contract_candidate:{}:consumer_shaped_payload_candidate_only",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_payload_public_contract_candidate:{}:proof_object_schema_prover_brand_adjacent_not_collapsed",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.coupled_default_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "theorem_result_payload_default:notebook_consumer_object_first".to_string(),
                "theorem_result_payload_default:consumer_shaped_payload_candidate_only"
                    .to_string(),
                "theorem_result_payload_default:proof_object_schema_prover_brand_adjacent_keep"
                    .to_string(),
                "theorem_result_payload_default:final_public_verifier_contract_later".to_string(),
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
            "kept_later:final_public_theorem_result_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match coupled_gate.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(coupled_gate.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(coupled_gate
                .actualization_guard_reason
                .as_ref()
                .unwrap()
                .contains("theorem result/payload public-contract coupled later gate"));
        }
    }
}

#[test]
fn theorem_result_payload_public_contract_coupled_later_gate_reaches_static_underdeclared_sample()
{
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();

    assert_coupled_later_gate_matches_prior_floors(
        &coupled_gate,
        &result_object_preview,
        &proof_object_gate,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
        ],
        &[
            "guard:final_theorem_result_candidate_only",
            "guard:consumer_shaped_payload_candidate_only",
            "guard:proof_object_schema_prover_brand_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn theorem_result_payload_public_contract_coupled_later_gate_keeps_guarded_prototype_as_not_reached()
{
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_prior_floors(
        &coupled_gate,
        &result_object_preview,
        &proof_object_gate,
        &["compare_floor:current_l2.theorem_result_payload_public_contract.guard_only"],
        &["guard:theorem_result_payload_public_contract_not_reached"],
    );
}

#[test]
fn theorem_result_payload_public_contract_coupled_later_gate_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_prior_floors(
        &coupled_gate,
        &result_object_preview,
        &proof_object_gate,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
        ],
        &[
            "guard:final_theorem_result_candidate_only",
            "guard:consumer_shaped_payload_candidate_only",
            "guard:proof_object_schema_prover_brand_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn theorem_result_payload_public_contract_coupled_later_gate_reaches_order_handoff_runtime_prototype()
{
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_prior_floors(
        &coupled_gate,
        &result_object_preview,
        &proof_object_gate,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
        ],
        &[
            "guard:final_theorem_result_candidate_only",
            "guard:consumer_shaped_payload_candidate_only",
            "guard:proof_object_schema_prover_brand_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn theorem_result_payload_public_contract_coupled_later_gate_reaches_stale_reconnect_runtime_prototype(
) {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let proof_object_gate =
        build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_matches_prior_floors(
        &coupled_gate,
        &result_object_preview,
        &proof_object_gate,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
        ],
        &[
            "guard:final_theorem_result_candidate_only",
            "guard:consumer_shaped_payload_candidate_only",
            "guard:proof_object_schema_prover_brand_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}
