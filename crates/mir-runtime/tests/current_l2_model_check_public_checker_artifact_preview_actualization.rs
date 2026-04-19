use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization,
    CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption,
    build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization,
    build_current_l2_source_sample_model_check_row_local_property_actual_adoption,
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

fn assert_checker_artifact_preview_matches_row_local_adoption(
    artifact_preview: &CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization,
    row_local_adoption: &CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        artifact_preview.actualization_status,
        row_local_adoption.actualization_status
    );
    assert_eq!(
        artifact_preview.actualization_subject_kind,
        row_local_adoption.actualization_subject_kind
    );
    assert_eq!(
        artifact_preview.actualization_subject_ref,
        row_local_adoption.actualization_subject_ref
    );
    assert_eq!(
        artifact_preview.checker_artifact_preview_refs,
        match artifact_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_public_checker_preview:{}:consumer_shaped_artifact_preview_only",
                    artifact_preview.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_public_checker_preview:{}:checker_boundary_bundle",
                    artifact_preview.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_public_checker_preview:{}:row_local_property_route_bundle",
                    artifact_preview.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_public_checker_preview:{}:repo_local_emitted_artifact_refs",
                    artifact_preview.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        artifact_preview.verifier_handoff_reserve_refs,
        match artifact_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_verifier_handoff_reserve:public_checker_migration_later".to_string(),
                "model_check_verifier_handoff_reserve:emitted_handoff_artifact_later".to_string(),
                "model_check_verifier_handoff_reserve:runtime_policy_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        artifact_preview.tool_binding_reserve_refs,
        match artifact_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_tool_binding_reserve:brand_neutral_request_manifest".to_string(),
                "model_check_tool_binding_reserve:concrete_tool_brand_later".to_string(),
                "model_check_tool_binding_reserve:runtime_policy_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        artifact_preview.actual_adoption_default_refs,
        match artifact_preview.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_public_checker_preview_default:consumer_shaped_artifact_preview_only"
                    .to_string(),
                "model_check_public_checker_preview_default:verifier_handoff_reserve_keep"
                    .to_string(),
                "model_check_public_checker_preview_default:brand_neutral_tool_binding_reserve_keep"
                    .to_string(),
                "model_check_public_checker_preview_default:runtime_policy_contract_later"
                    .to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        artifact_preview.repo_local_emitted_artifact_refs,
        row_local_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        artifact_preview.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        artifact_preview.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        artifact_preview.kept_later_refs,
        vec![
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:final_public_checker_artifact".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match artifact_preview.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(artifact_preview.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                artifact_preview
                    .actualization_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("model-check public-checker artifact preview actualization")
            );
        }
    }
}

#[test]
fn model_check_public_checker_artifact_preview_actualization_reaches_static_underdeclared_sample() {
    let row_local_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();

    assert_checker_artifact_preview_matches_row_local_adoption(
        &artifact_preview,
        &row_local_adoption,
        &[
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
        ],
        &[
            "guard:public_checker_artifact_preview_actualization_only",
            "guard:verifier_handoff_reserve_keep",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:runtime_policy_contract_later",
        ],
    );
}

#[test]
fn model_check_public_checker_artifact_preview_actualization_keeps_guarded_prototype_as_not_reached()
 {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let row_local_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_checker_artifact_preview_matches_row_local_adoption(
        &artifact_preview,
        &row_local_adoption,
        &["compare_floor:current_l2.model_check.public_checker_artifact_preview.guard_only"],
        &["guard:model_check_public_checker_artifact_preview_not_reached"],
    );
}

#[test]
fn model_check_public_checker_artifact_preview_actualization_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let row_local_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_checker_artifact_preview_matches_row_local_adoption(
        &artifact_preview,
        &row_local_adoption,
        &[
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
        ],
        &[
            "guard:public_checker_artifact_preview_actualization_only",
            "guard:verifier_handoff_reserve_keep",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:runtime_policy_contract_later",
        ],
    );
}

#[test]
fn model_check_public_checker_artifact_preview_actualization_reaches_order_handoff_runtime_prototype()
 {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let row_local_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_checker_artifact_preview_matches_row_local_adoption(
        &artifact_preview,
        &row_local_adoption,
        &[
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
        ],
        &[
            "guard:public_checker_artifact_preview_actualization_only",
            "guard:verifier_handoff_reserve_keep",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:runtime_policy_contract_later",
        ],
    );
}

#[test]
fn model_check_public_checker_artifact_preview_actualization_reaches_delegated_provider_runtime_prototype()
 {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let row_local_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_checker_artifact_preview_matches_row_local_adoption(
        &artifact_preview,
        &row_local_adoption,
        &[
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
        ],
        &[
            "guard:public_checker_artifact_preview_actualization_only",
            "guard:verifier_handoff_reserve_keep",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:runtime_policy_contract_later",
        ],
    );
}
