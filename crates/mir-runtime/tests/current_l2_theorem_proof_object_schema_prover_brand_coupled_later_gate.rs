use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate,
    CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate,
    build_current_l2_source_sample_theorem_result_object_preview_actualization,
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

fn assert_coupled_later_gate_matches_result_object_preview(
    coupled_gate: &CurrentL2SourceSampleTheoremProofObjectSchemaProverBrandCoupledLaterGate,
    result_object_preview: &CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        coupled_gate.actualization_status,
        result_object_preview.actualization_status
    );
    assert_eq!(
        coupled_gate.actualization_subject_kind,
        result_object_preview.actualization_subject_kind
    );
    assert_eq!(
        coupled_gate.actualization_subject_ref,
        result_object_preview.actualization_subject_ref
    );
    assert_eq!(
        coupled_gate.proof_object_schema_candidate_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_proof_object_schema_candidate:{}:result_object_preview_adjacent",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_proof_object_schema_candidate:{}:refs_only_public_schema_candidate",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_proof_object_schema_candidate:{}:public_contract_not_adopted",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.prover_brand_candidate_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_prover_brand_candidate:{}:brand_neutral_preflight_anchor",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_prover_brand_candidate:{}:adapter_boundary_refs_keep",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_prover_brand_candidate:{}:concrete_brand_not_adopted",
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
                "theorem_proof_schema_brand_default:result_object_preview_keep".to_string(),
                "theorem_proof_schema_brand_default:proof_object_schema_candidate_only"
                    .to_string(),
                "theorem_proof_schema_brand_default:prover_brand_candidate_only".to_string(),
                "theorem_proof_schema_brand_default:final_public_contract_later".to_string(),
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
                .contains("theorem proof-object schema / prover-brand coupled later gate"));
        }
    }
}

#[test]
fn theorem_proof_object_schema_prover_brand_coupled_later_gate_reaches_static_underdeclared_sample()
{
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_coupled_later_gate_matches_result_object_preview(
        &coupled_gate,
        &result_object_preview,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
        ],
        &[
            "guard:proof_object_schema_candidate_only",
            "guard:concrete_theorem_prover_brand_candidate_only",
            "guard:final_public_theorem_result_object_later",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn theorem_proof_object_schema_prover_brand_coupled_later_gate_keeps_guarded_prototype_as_not_reached()
{
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_result_object_preview(
        &coupled_gate,
        &result_object_preview,
        &[
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand.guard_only",
        ],
        &["guard:theorem_proof_object_schema_prover_brand_not_reached"],
    );
}

#[test]
fn theorem_proof_object_schema_prover_brand_coupled_later_gate_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_result_object_preview(
        &coupled_gate,
        &result_object_preview,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
        ],
        &[
            "guard:proof_object_schema_candidate_only",
            "guard:concrete_theorem_prover_brand_candidate_only",
            "guard:final_public_theorem_result_object_later",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn theorem_proof_object_schema_prover_brand_coupled_later_gate_reaches_order_handoff_runtime_prototype()
{
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_result_object_preview(
        &coupled_gate,
        &result_object_preview,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
        ],
        &[
            "guard:proof_object_schema_candidate_only",
            "guard:concrete_theorem_prover_brand_candidate_only",
            "guard:final_public_theorem_result_object_later",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn theorem_proof_object_schema_prover_brand_coupled_later_gate_reaches_stale_reconnect_runtime_prototype()
{
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let coupled_gate = build_current_l2_source_sample_theorem_proof_object_schema_prover_brand_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_result_object_preview(
        &coupled_gate,
        &result_object_preview,
        &[
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_proof_object_schema_prover_brand_coupled_later_gate",
        ],
        &[
            "guard:proof_object_schema_candidate_only",
            "guard:concrete_theorem_prover_brand_candidate_only",
            "guard:final_public_theorem_result_object_later",
            "guard:final_public_verifier_contract_later",
        ],
    );
}
