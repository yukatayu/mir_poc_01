use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremResultObjectActualAdoption,
    CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate,
    CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption,
    build_current_l2_source_sample_theorem_result_object_actual_adoption,
    build_current_l2_source_sample_theorem_result_object_preview_actualization,
    build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate,
    build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption,
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

fn assert_actual_adoption_matches_prior_floors(
    actual_adoption: &CurrentL2SourceSampleTheoremResultObjectActualAdoption,
    review_unit_actual_adoption: &CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption,
    result_object_preview: &CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    result_payload_coupled_gate: &CurrentL2SourceSampleTheoremResultPayloadPublicContractCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        actual_adoption.actualization_status,
        review_unit_actual_adoption.actualization_status
    );
    assert_eq!(
        actual_adoption.actualization_status,
        result_object_preview.actualization_status
    );
    assert_eq!(
        actual_adoption.actualization_status,
        result_payload_coupled_gate.actualization_status
    );
    assert_eq!(
        actual_adoption.actualization_subject_kind,
        review_unit_actual_adoption.actualization_subject_kind
    );
    assert_eq!(
        actual_adoption.actualization_subject_ref,
        review_unit_actual_adoption.actualization_subject_ref
    );
    assert_eq!(
        actual_adoption.repo_local_emitted_artifact_refs,
        review_unit_actual_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        actual_adoption.result_object_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_result_object_actual_route:{}:review_unit_transport_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_result_object_actual_route:{}:notebook_consumer_object_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_result_object_actual_route:{}:repo_local_emitted_artifact_refs_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_result_object_actual_route:{}:consumer_shaped_payload_preview_keep",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_result_object_actual_route:{}:proof_object_schema_prover_brand_later",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.payload_preview_keep_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_result_object_payload_preview_keep:{}:notebook_consumer_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_result_object_payload_preview_keep:{}:consumer_shaped_payload_preview_only",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_result_object_payload_preview_keep:{}:payload_public_contract_later",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.actual_adoption_default_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "theorem_result_object_actual_adoption_default:review_unit_transport_first"
                    .to_string(),
                "theorem_result_object_actual_adoption_default:notebook_consumer_object_first"
                    .to_string(),
                "theorem_result_object_actual_adoption_default:consumer_shaped_payload_preview_keep"
                    .to_string(),
                "theorem_result_object_actual_adoption_default:proof_object_schema_prover_brand_later"
                    .to_string(),
                "theorem_result_object_actual_adoption_default:final_public_result_object_later"
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
            "kept_later:final_public_theorem_result_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match actual_adoption.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(actual_adoption.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(actual_adoption
                .actualization_guard_reason
                .as_ref()
                .unwrap()
                .contains("theorem result-object actual adoption"));
        }
    }
}

#[test]
fn theorem_result_object_actual_adoption_reaches_static_underdeclared_sample() {
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let result_payload_coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_result_object_actual_adoption(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &review_unit_actual_adoption,
        &result_object_preview,
        &result_payload_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_first",
            "guard:notebook_consumer_object_first",
            "guard:consumer_shaped_payload_preview_keep",
            "guard:proof_object_schema_prover_brand_later",
            "guard:final_public_theorem_result_object_later",
        ],
    );
}

#[test]
fn theorem_result_object_actual_adoption_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_payload_coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_result_object_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &review_unit_actual_adoption,
        &result_object_preview,
        &result_payload_coupled_gate,
        &["compare_floor:current_l2.theorem_result_object_actual_adoption.guard_only"],
        &["guard:theorem_result_object_actual_adoption_not_reached"],
    );
}

#[test]
fn theorem_result_object_actual_adoption_reaches_typed_runtime_prototype() {
    let sample_path =
        typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_payload_coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_result_object_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &review_unit_actual_adoption,
        &result_object_preview,
        &result_payload_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_first",
            "guard:notebook_consumer_object_first",
            "guard:consumer_shaped_payload_preview_keep",
            "guard:proof_object_schema_prover_brand_later",
            "guard:final_public_theorem_result_object_later",
        ],
    );
}

#[test]
fn theorem_result_object_actual_adoption_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_payload_coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_result_object_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &review_unit_actual_adoption,
        &result_object_preview,
        &result_payload_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_first",
            "guard:notebook_consumer_object_first",
            "guard:consumer_shaped_payload_preview_keep",
            "guard:proof_object_schema_prover_brand_later",
            "guard:final_public_theorem_result_object_later",
        ],
    );
}

#[test]
fn theorem_result_object_actual_adoption_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let review_unit_actual_adoption =
        build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_object_preview =
        build_current_l2_source_sample_theorem_result_object_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let result_payload_coupled_gate =
        build_current_l2_source_sample_theorem_result_payload_public_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_result_object_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &review_unit_actual_adoption,
        &result_object_preview,
        &result_payload_coupled_gate,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_result_object_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_first",
            "guard:notebook_consumer_object_first",
            "guard:consumer_shaped_payload_preview_keep",
            "guard:proof_object_schema_prover_brand_later",
            "guard:final_public_theorem_result_object_later",
        ],
    );
}
