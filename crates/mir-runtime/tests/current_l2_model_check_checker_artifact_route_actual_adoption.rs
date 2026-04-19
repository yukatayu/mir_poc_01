use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption,
    CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate,
    CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization,
    CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate,
    build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption,
    build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate,
    build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization,
    build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate,
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
    actual_adoption: &CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption,
    artifact_migration: &CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate,
    artifact_preview: &CurrentL2SourceSampleModelCheckPublicCheckerArtifactPreviewActualization,
    tool_handoff: &CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        actual_adoption.actualization_status,
        artifact_migration.actualization_status
    );
    assert_eq!(
        actual_adoption.actualization_status,
        artifact_preview.actualization_status
    );
    assert_eq!(
        actual_adoption.actualization_status,
        tool_handoff.actualization_status
    );
    assert_eq!(
        actual_adoption.actualization_subject_kind,
        artifact_migration.actualization_subject_kind
    );
    assert_eq!(
        actual_adoption.actualization_subject_ref,
        artifact_migration.actualization_subject_ref
    );
    assert_eq!(
        actual_adoption.repo_local_emitted_artifact_refs,
        artifact_migration.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        actual_adoption.checker_artifact_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_checker_artifact_actual_route:{}:row_local_property_route_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_actual_route:{}:checker_boundary_contract_anchor",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_actual_route:{}:consumer_shaped_checker_artifact_candidate_only",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_actual_route:{}:repo_local_emitted_artifact_refs_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_actual_route:{}:final_public_checker_artifact_later",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.migration_candidate_keep_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_checker_artifact_migration_keep:{}:verifier_handoff_candidate_adjacent_keep",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_migration_keep:{}:tool_brand_candidate_adjacent_keep",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_migration_keep:{}:actual_public_checker_migration_candidate_only",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_artifact_migration_keep:{}:runtime_policy_contract_later",
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
                "model_check_checker_artifact_actual_adoption_default:row_local_property_route_first"
                    .to_string(),
                "model_check_checker_artifact_actual_adoption_default:checker_boundary_contract_anchor"
                    .to_string(),
                "model_check_checker_artifact_actual_adoption_default:consumer_shaped_checker_artifact_candidate_only"
                    .to_string(),
                "model_check_checker_artifact_actual_adoption_default:migration_candidate_adjacent_keep"
                    .to_string(),
                "model_check_checker_artifact_actual_adoption_default:final_public_checker_artifact_later"
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
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:final_public_checker_artifact".to_string(),
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
            assert!(
                actual_adoption
                    .actualization_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("model-check checker-artifact route actual adoption")
            );
        }
    }
}

#[test]
fn model_check_checker_artifact_route_actual_adoption_reaches_static_underdeclared_sample() {
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let artifact_migration =
        build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &artifact_migration,
        &artifact_preview,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
        ],
        &[
            "guard:row_local_property_route_first",
            "guard:checker_boundary_contract_anchor",
            "guard:consumer_shaped_checker_artifact_candidate_only",
            "guard:migration_candidate_adjacent_keep",
            "guard:final_public_checker_artifact_later",
        ],
    );
}

#[test]
fn model_check_checker_artifact_route_actual_adoption_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_migration =
        build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &artifact_migration,
        &artifact_preview,
        &tool_handoff,
        &["compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption.guard_only"],
        &["guard:model_check_checker_artifact_route_actual_adoption_not_reached"],
    );
}

#[test]
fn model_check_checker_artifact_route_actual_adoption_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_migration =
        build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &artifact_migration,
        &artifact_preview,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
        ],
        &[
            "guard:row_local_property_route_first",
            "guard:checker_boundary_contract_anchor",
            "guard:consumer_shaped_checker_artifact_candidate_only",
            "guard:migration_candidate_adjacent_keep",
            "guard:final_public_checker_artifact_later",
        ],
    );
}

#[test]
fn model_check_checker_artifact_route_actual_adoption_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_migration =
        build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &artifact_migration,
        &artifact_preview,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
        ],
        &[
            "guard:row_local_property_route_first",
            "guard:checker_boundary_contract_anchor",
            "guard:consumer_shaped_checker_artifact_candidate_only",
            "guard:migration_candidate_adjacent_keep",
            "guard:final_public_checker_artifact_later",
        ],
    );
}

#[test]
fn model_check_checker_artifact_route_actual_adoption_reaches_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let artifact_preview =
        build_current_l2_source_sample_model_check_public_checker_artifact_preview_actualization(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let artifact_migration =
        build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_actual_adoption_matches_prior_floors(
        &actual_adoption,
        &artifact_migration,
        &artifact_preview,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
        ],
        &[
            "guard:row_local_property_route_first",
            "guard:checker_boundary_contract_anchor",
            "guard:consumer_shaped_checker_artifact_candidate_only",
            "guard:migration_candidate_adjacent_keep",
            "guard:final_public_checker_artifact_later",
        ],
    );
}
