use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate,
    CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate,
    build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate,
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

fn assert_coupled_later_gate_matches_tool_handoff_floor(
    coupled_gate: &CurrentL2SourceSampleModelCheckPublicCheckerArtifactMigrationCoupledLaterGate,
    tool_handoff: &CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        coupled_gate.actualization_status,
        tool_handoff.actualization_status
    );
    assert_eq!(
        coupled_gate.actualization_subject_kind,
        tool_handoff.actualization_subject_kind
    );
    assert_eq!(
        coupled_gate.actualization_subject_ref,
        tool_handoff.actualization_subject_ref
    );
    assert_eq!(
        coupled_gate.public_checker_artifact_candidate_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_final_public_checker_candidate:{}:consumer_shaped_artifact_preview_keep",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_final_public_checker_candidate:{}:checker_boundary_contract_anchor",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_final_public_checker_candidate:{}:repo_local_emitted_artifact_refs_first",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_final_public_checker_candidate:{}:final_public_checker_artifact_later",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.checker_migration_candidate_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_checker_migration_candidate:{}:verifier_handoff_candidate_keep",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_migration_candidate:{}:tool_brand_candidate_adjacent_keep",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_migration_candidate:{}:actual_public_checker_migration_candidate_only",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_checker_migration_candidate:{}:runtime_policy_contract_adjacent_not_collapsed",
                    coupled_gate.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        coupled_gate.repo_local_emitted_artifact_refs,
        tool_handoff.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        coupled_gate.coupled_default_refs,
        match coupled_gate.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_public_checker_default:consumer_shaped_artifact_candidate_only"
                    .to_string(),
                "model_check_public_checker_default:migration_candidate_only".to_string(),
                "model_check_public_checker_default:tool_brand_verifier_handoff_adjacent_keep"
                    .to_string(),
                "model_check_public_checker_default:final_public_verifier_contract_later"
                    .to_string(),
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
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:final_public_checker_artifact".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match coupled_gate.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(coupled_gate.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                coupled_gate
                    .actualization_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("model-check public-checker artifact / migration coupled later gate")
            );
        }
    }
}

#[test]
fn model_check_public_checker_artifact_migration_coupled_later_gate_reaches_static_underdeclared_sample()
 {
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let coupled_gate = build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_coupled_later_gate_matches_tool_handoff_floor(
        &coupled_gate,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
        ],
        &[
            "guard:consumer_shaped_public_checker_candidate_only",
            "guard:actual_public_checker_migration_candidate_only",
            "guard:tool_brand_verifier_handoff_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn model_check_public_checker_artifact_migration_coupled_later_gate_keeps_guarded_prototype_as_not_reached()
 {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate = build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_tool_handoff_floor(
        &coupled_gate,
        &tool_handoff,
        &["compare_floor:current_l2.model_check.public_checker_artifact_migration.guard_only"],
        &["guard:model_check_public_checker_artifact_migration_not_reached"],
    );
}

#[test]
fn model_check_public_checker_artifact_migration_coupled_later_gate_reaches_typed_runtime_prototype()
 {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate = build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_tool_handoff_floor(
        &coupled_gate,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
        ],
        &[
            "guard:consumer_shaped_public_checker_candidate_only",
            "guard:actual_public_checker_migration_candidate_only",
            "guard:tool_brand_verifier_handoff_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn model_check_public_checker_artifact_migration_coupled_later_gate_reaches_order_handoff_runtime_prototype()
 {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate = build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_tool_handoff_floor(
        &coupled_gate,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
        ],
        &[
            "guard:consumer_shaped_public_checker_candidate_only",
            "guard:actual_public_checker_migration_candidate_only",
            "guard:tool_brand_verifier_handoff_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}

#[test]
fn model_check_public_checker_artifact_migration_coupled_later_gate_reaches_delegated_provider_runtime_prototype()
 {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let tool_handoff =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let coupled_gate = build_current_l2_source_sample_model_check_public_checker_artifact_migration_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_coupled_later_gate_matches_tool_handoff_floor(
        &coupled_gate,
        &tool_handoff,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
        ],
        &[
            "guard:consumer_shaped_public_checker_candidate_only",
            "guard:actual_public_checker_migration_candidate_only",
            "guard:tool_brand_verifier_handoff_adjacent_keep",
            "guard:final_public_verifier_contract_later",
        ],
    );
}
