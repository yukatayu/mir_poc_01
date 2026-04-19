use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption,
    CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold,
    CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate,
    build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption,
    build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold,
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

fn assert_final_public_contract_reopen_threshold(
    threshold: &CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold,
    checker_artifact_route_actual_adoption: &CurrentL2SourceSampleModelCheckCheckerArtifactRouteActualAdoption,
    tool_brand_verifier_handoff_coupled_gate: &CurrentL2SourceSampleModelCheckToolBrandVerifierHandoffCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        threshold.threshold_status,
        checker_artifact_route_actual_adoption.actualization_status
    );
    assert_eq!(
        threshold.threshold_status,
        tool_brand_verifier_handoff_coupled_gate.actualization_status
    );
    assert_eq!(
        threshold.actualization_subject_kind,
        checker_artifact_route_actual_adoption.actualization_subject_kind
    );
    assert_eq!(
        threshold.actualization_subject_ref,
        checker_artifact_route_actual_adoption.actualization_subject_ref
    );
    assert_eq!(
        threshold.repo_local_emitted_artifact_refs,
        checker_artifact_route_actual_adoption.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        threshold.checker_artifact_route_refs,
        checker_artifact_route_actual_adoption.checker_artifact_route_refs
    );
    assert_eq!(
        threshold.migration_candidate_keep_refs,
        checker_artifact_route_actual_adoption.migration_candidate_keep_refs
    );
    assert_eq!(
        threshold.verifier_handoff_candidate_refs,
        tool_brand_verifier_handoff_coupled_gate.verifier_handoff_candidate_refs
    );
    assert_eq!(
        threshold.tool_brand_candidate_refs,
        tool_brand_verifier_handoff_coupled_gate.tool_brand_candidate_refs
    );
    assert_eq!(
        threshold.final_public_contract_reopen_sequence_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "model_check_final_public_contract_reopen:{}:property_language_and_tool_brand_first",
                    threshold.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_final_public_contract_reopen:{}:public_checker_artifact_and_migration_second",
                    threshold.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_final_public_contract_reopen:{}:verifier_handoff_and_runtime_policy_contract_third",
                    threshold.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "model_check_final_public_contract_reopen:{}:final_public_verifier_contract_fourth",
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
                "model_check_final_public_contract_reopen_default:property_language_and_tool_brand_first"
                    .to_string(),
                "model_check_final_public_contract_reopen_default:public_checker_artifact_and_migration_second"
                    .to_string(),
                "model_check_final_public_contract_reopen_default:verifier_handoff_and_runtime_policy_contract_third"
                    .to_string(),
                "model_check_final_public_contract_reopen_default:final_public_verifier_contract_fourth"
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
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:final_public_checker_artifact".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
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
                    .contains("model-check final public contract reopen threshold")
            );
        }
    }
}

#[test]
fn model_check_final_public_contract_reopen_threshold_reaches_static_underdeclared_sample() {
    let checker_artifact_route_actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let tool_brand_verifier_handoff_coupled_gate =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &checker_artifact_route_actual_adoption,
        &tool_brand_verifier_handoff_coupled_gate,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold",
        ],
        &[
            "guard:property_language_and_tool_brand_first",
            "guard:public_checker_artifact_and_migration_second",
            "guard:verifier_handoff_and_runtime_policy_contract_third",
            "guard:final_public_verifier_contract_fourth",
        ],
    );
}

#[test]
fn model_check_final_public_contract_reopen_threshold_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let checker_artifact_route_actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_brand_verifier_handoff_coupled_gate =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &checker_artifact_route_actual_adoption,
        &tool_brand_verifier_handoff_coupled_gate,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold",
        ],
        &[
            "guard:property_language_and_tool_brand_first",
            "guard:public_checker_artifact_and_migration_second",
            "guard:verifier_handoff_and_runtime_policy_contract_third",
            "guard:final_public_verifier_contract_fourth",
        ],
    );
}

#[test]
fn model_check_final_public_contract_reopen_threshold_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let checker_artifact_route_actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_brand_verifier_handoff_coupled_gate =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &checker_artifact_route_actual_adoption,
        &tool_brand_verifier_handoff_coupled_gate,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold",
        ],
        &[
            "guard:property_language_and_tool_brand_first",
            "guard:public_checker_artifact_and_migration_second",
            "guard:verifier_handoff_and_runtime_policy_contract_third",
            "guard:final_public_verifier_contract_fourth",
        ],
    );
}

#[test]
fn model_check_final_public_contract_reopen_threshold_reaches_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let checker_artifact_route_actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_brand_verifier_handoff_coupled_gate =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &checker_artifact_route_actual_adoption,
        &tool_brand_verifier_handoff_coupled_gate,
        &[
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization",
            "compare_floor:current_l2.model_check.property_tool_threshold",
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate",
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate",
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption",
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold",
        ],
        &[
            "guard:property_language_and_tool_brand_first",
            "guard:public_checker_artifact_and_migration_second",
            "guard:verifier_handoff_and_runtime_policy_contract_third",
            "guard:final_public_verifier_contract_fourth",
        ],
    );
}

#[test]
fn model_check_final_public_contract_reopen_threshold_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let checker_artifact_route_actual_adoption =
        build_current_l2_source_sample_model_check_checker_artifact_route_actual_adoption(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let tool_brand_verifier_handoff_coupled_gate =
        build_current_l2_source_sample_model_check_tool_brand_verifier_handoff_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_final_public_contract_reopen_threshold(
        &threshold,
        &checker_artifact_route_actual_adoption,
        &tool_brand_verifier_handoff_coupled_gate,
        &["compare_floor:current_l2.model_check.final_public_contract_reopen_threshold.guard_only"],
        &["guard:model_check_final_public_contract_reopen_threshold_not_reached"],
    );
}
