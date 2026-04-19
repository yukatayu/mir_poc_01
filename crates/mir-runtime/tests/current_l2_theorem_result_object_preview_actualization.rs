use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption,
    build_current_l2_source_sample_theorem_result_object_preview_actualization,
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

fn assert_result_object_preview_matches_review_unit_adoption(
    result_object_preview: &CurrentL2SourceSampleTheoremResultObjectPreviewActualization,
    review_unit_adoption: &CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        result_object_preview.actualization_status,
        review_unit_adoption.actualization_status
    );
    assert_eq!(
        result_object_preview.actualization_subject_kind,
        review_unit_adoption.actualization_subject_kind
    );
    assert_eq!(
        result_object_preview.actualization_subject_ref,
        review_unit_adoption.actualization_subject_ref
    );
    assert_eq!(
        result_object_preview.result_object_route_refs,
        match result_object_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_result_object_route:{}:notebook_consumer_object_first",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
                format!(
                    "theorem_result_object_route:{}:review_unit_anchor_bundle",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
                format!(
                    "theorem_result_object_route:{}:consumer_shaped_payload_preview_only",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
                format!(
                    "theorem_result_object_route:{}:repo_local_emitted_artifact_refs",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        result_object_preview.notebook_payload_preview_refs,
        match result_object_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_result_payload_preview:{}:notebook_consumer_first",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
                format!(
                    "theorem_result_payload_preview:{}:review_unit_reference_bundle",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
                format!(
                    "theorem_result_payload_preview:{}:consumer_shaped_payload_preview_only",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
                format!(
                    "theorem_result_payload_preview:{}:proof_object_public_schema_later",
                    result_object_preview
                        .actualization_subject_ref
                        .as_ref()
                        .unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        result_object_preview.proof_object_schema_reserve_refs,
        match result_object_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "proof_object_schema_reserve:brand_neutral_binding_keep".to_string(),
                "proof_object_schema_reserve:proof_object_public_schema_later".to_string(),
                "proof_object_schema_reserve:final_public_verifier_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        result_object_preview.actual_adoption_default_refs,
        match result_object_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "theorem_result_object_preview_default:notebook_consumer_object_first"
                    .to_string(),
                "theorem_result_object_preview_default:consumer_shaped_payload_preview_only"
                    .to_string(),
                "theorem_result_object_preview_default:proof_object_schema_reserve_keep"
                    .to_string(),
                "theorem_result_object_preview_default:final_public_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        result_object_preview.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result_object_preview.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result_object_preview.kept_later_refs,
        vec![
            "kept_later:final_public_theorem_result_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match result_object_preview.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(result_object_preview.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(result_object_preview
                .actualization_guard_reason
                .as_ref()
                .unwrap()
                .contains("theorem result-object preview actualization"));
        }
    }
}

#[test]
fn theorem_result_object_preview_actualization_reaches_static_underdeclared_sample() {
    let review_unit_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_result_object_preview_matches_review_unit_adoption(
        &result_object_preview,
        &review_unit_adoption,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
        ],
        &[
            "guard:result_object_preview_actualization_only",
            "guard:consumer_shaped_payload_preview_only",
            "guard:proof_object_schema_reserve_keep",
            "guard:concrete_theorem_prover_brand_later",
        ],
    );
}

#[test]
fn theorem_result_object_preview_actualization_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let review_unit_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_result_object_preview_matches_review_unit_adoption(
        &result_object_preview,
        &review_unit_adoption,
        &["compare_floor:current_l2.theorem_result_object_preview.guard_only"],
        &["guard:theorem_result_object_preview_not_reached"],
    );
}

#[test]
fn theorem_result_object_preview_actualization_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let review_unit_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_result_object_preview_matches_review_unit_adoption(
        &result_object_preview,
        &review_unit_adoption,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
        ],
        &[
            "guard:result_object_preview_actualization_only",
            "guard:consumer_shaped_payload_preview_only",
            "guard:proof_object_schema_reserve_keep",
            "guard:concrete_theorem_prover_brand_later",
        ],
    );
}

#[test]
fn theorem_result_object_preview_actualization_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let review_unit_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_result_object_preview_matches_review_unit_adoption(
        &result_object_preview,
        &review_unit_adoption,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
        ],
        &[
            "guard:result_object_preview_actualization_only",
            "guard:consumer_shaped_payload_preview_only",
            "guard:proof_object_schema_reserve_keep",
            "guard:concrete_theorem_prover_brand_later",
        ],
    );
}

#[test]
fn theorem_result_object_preview_actualization_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let review_unit_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let result_object_preview = build_current_l2_source_sample_theorem_result_object_preview_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_result_object_preview_matches_review_unit_adoption(
        &result_object_preview,
        &review_unit_adoption,
        &[
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_result_object_preview_actualization",
        ],
        &[
            "guard:result_object_preview_actualization_only",
            "guard:consumer_shaped_payload_preview_only",
            "guard:proof_object_schema_reserve_keep",
            "guard:concrete_theorem_prover_brand_later",
        ],
    );
}
