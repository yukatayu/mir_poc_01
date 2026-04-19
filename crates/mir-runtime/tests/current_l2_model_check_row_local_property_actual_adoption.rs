use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleModelCheckPropertyToolThreshold,
    CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption,
    build_current_l2_source_sample_model_check_property_tool_threshold,
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

fn assert_actual_adoption_matches_threshold(
    actual_adoption: &CurrentL2SourceSampleModelCheckRowLocalPropertyActualAdoption,
    threshold: &CurrentL2SourceSampleModelCheckPropertyToolThreshold,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(actual_adoption.actualization_status, threshold.threshold_status);
    assert_eq!(
        actual_adoption.actualization_subject_kind,
        threshold.threshold_subject_kind
    );
    assert_eq!(
        actual_adoption.actualization_subject_ref,
        threshold.threshold_subject_ref
    );
    assert_eq!(
        actual_adoption.property_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_property_route:{}:row_local_preview_bundle",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_property_route:{}:property_language_probe_bundle",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_property_route:{}:small_cluster_projection_second",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_property_route:{}:repo_local_emitted_artifact_refs",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.checker_contract_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_checker_contract_route:{}:checker_boundary_probe_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_contract_route:{}:public_checker_reserve_bundle",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_contract_route:{}:public_checker_contract_later",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_contract_route:{}:verifier_handoff_artifact_later",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.tool_binding_reserve_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_tool_binding_reserve:brand_neutral_request_manifest".to_string(),
                "model_check_tool_binding_reserve:concrete_tool_brand_later".to_string(),
                "model_check_tool_binding_reserve:runtime_policy_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.actual_adoption_default_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_actual_adoption_default:row_local_property_route_first".to_string(),
                "model_check_actual_adoption_default:checker_boundary_contract_first"
                    .to_string(),
                "model_check_actual_adoption_default:brand_neutral_tool_binding_reserve_keep"
                    .to_string(),
                "model_check_actual_adoption_default:public_checker_handoff_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.repo_local_emitted_artifact_refs,
        threshold.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        actual_adoption.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.excluded_family_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "excluded_family:theorem_discharge_actual_format".to_string(),
                "excluded_family:room_protocol_projection".to_string(),
                "excluded_family:provider_receipt_fairness_family".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
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
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:consumer_shaped_public_checker_artifact".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
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
                .contains("model-check row-local property actual adoption"));
        }
    }
}

#[test]
fn model_check_actual_adoption_reaches_static_underdeclared_sample() {
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
        ],
        &[
            "guard:row_local_property_actual_adoption_only",
            "guard:checker_boundary_contract_actual_adoption_only",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:public_checker_handoff_later",
        ],
    );
}

#[test]
fn model_check_actual_adoption_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &["compare_floor:current_l2.model_check.row_local_property_actual_adoption_guard_only"],
        &["guard:model_check_row_local_property_actual_adoption_not_reached"],
    );
}

#[test]
fn model_check_actual_adoption_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
        ],
        &[
            "guard:row_local_property_actual_adoption_only",
            "guard:checker_boundary_contract_actual_adoption_only",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:public_checker_handoff_later",
        ],
    );
}

#[test]
fn model_check_actual_adoption_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
        ],
        &[
            "guard:row_local_property_actual_adoption_only",
            "guard:checker_boundary_contract_actual_adoption_only",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:public_checker_handoff_later",
        ],
    );
}

#[test]
fn model_check_actual_adoption_reaches_delegated_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_row_local_property_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_threshold(
        &actual_adoption,
        &threshold,
        &[
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption",
        ],
        &[
            "guard:row_local_property_actual_adoption_only",
            "guard:checker_boundary_contract_actual_adoption_only",
            "guard:brand_neutral_tool_binding_reserve_keep",
            "guard:public_checker_handoff_later",
        ],
    );
}
