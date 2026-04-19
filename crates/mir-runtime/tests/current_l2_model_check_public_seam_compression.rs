use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold,
    CurrentL2SourceSampleModelCheckPropertyToolSeamProbe,
    CurrentL2SourceSampleModelCheckPublicSeamCompression,
    build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold,
    build_current_l2_source_sample_model_check_property_tool_seam_probe,
    build_current_l2_source_sample_model_check_public_seam_compression,
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

fn extend_unique_refs(target: &mut Vec<String>, refs: &[String]) {
    for reference in refs {
        if !target.contains(reference) {
            target.push(reference.clone());
        }
    }
}

fn assert_model_check_public_seam_compression_matches_prior_floors(
    compression: &CurrentL2SourceSampleModelCheckPublicSeamCompression,
    threshold: &CurrentL2SourceSampleModelCheckFinalPublicContractReopenThreshold,
    probe: &CurrentL2SourceSampleModelCheckPropertyToolSeamProbe,
) {
    assert_eq!(compression.compression_status, threshold.threshold_status);
    assert_eq!(compression.compression_status, probe.probe_status);
    assert_eq!(
        compression.actualization_subject_kind,
        threshold.actualization_subject_kind
    );
    assert_eq!(
        compression.actualization_subject_ref,
        threshold.actualization_subject_ref
    );
    assert_eq!(
        compression.actualization_subject_kind,
        probe.probe_subject_kind
    );
    assert_eq!(
        compression.actualization_subject_ref,
        probe.probe_subject_ref
    );
    assert_eq!(
        compression.checker_artifact_route_refs,
        threshold.checker_artifact_route_refs
    );
    assert_eq!(
        compression.migration_candidate_keep_refs,
        threshold.migration_candidate_keep_refs
    );
    assert_eq!(
        compression.verifier_handoff_candidate_refs,
        threshold.verifier_handoff_candidate_refs
    );
    assert_eq!(
        compression.tool_brand_candidate_refs,
        threshold.tool_brand_candidate_refs
    );
    assert_eq!(
        compression.property_language_probe_refs,
        probe.property_language_probe_refs
    );
    assert_eq!(compression.tool_seam_probe_refs, probe.tool_seam_probe_refs);
    assert_eq!(
        compression.checker_boundary_probe_refs,
        probe.checker_boundary_probe_refs
    );

    let mut expected_repo_local_emitted_artifact_refs =
        threshold.repo_local_emitted_artifact_refs.clone();
    extend_unique_refs(
        &mut expected_repo_local_emitted_artifact_refs,
        &probe.repo_local_emitted_artifact_refs,
    );
    assert_eq!(
        compression.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs
    );

    let mut expected_compare_floor_refs = threshold.compare_floor_refs.clone();
    extend_unique_refs(&mut expected_compare_floor_refs, &probe.compare_floor_refs);
    extend_unique_refs(
        &mut expected_compare_floor_refs,
        &["compare_floor:current_l2.model_check.public_seam_compression".to_string()],
    );
    assert_eq!(compression.compare_floor_refs, expected_compare_floor_refs);

    let subject_ref = compression.actualization_subject_ref.as_ref().unwrap();
    assert_eq!(
        compression.public_seam_residual_refs,
        vec![
            format!(
                "model_check_public_seam_residual:{subject_ref}:property_language_and_tool_brand_first"
            ),
            format!(
                "model_check_public_seam_residual:{subject_ref}:public_checker_artifact_and_migration_second"
            ),
            format!(
                "model_check_public_seam_residual:{subject_ref}:verifier_handoff_and_runtime_policy_contract_third"
            ),
            format!(
                "model_check_public_seam_residual:{subject_ref}:final_public_verifier_contract_fourth"
            ),
        ]
    );
    assert_eq!(
        compression.guard_refs,
        vec![
            "guard:model_check_property_language_and_tool_brand_first".to_string(),
            "guard:model_check_public_checker_artifact_and_migration_second".to_string(),
            "guard:model_check_verifier_handoff_and_runtime_policy_contract_third".to_string(),
            "guard:model_check_final_public_verifier_contract_fourth".to_string(),
        ]
    );
    assert_eq!(
        compression.kept_later_refs,
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
    assert!(compression.compression_guard_reason.is_none());
}

#[test]
fn model_check_public_seam_compression_reaches_static_underdeclared_sample() {
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_model_check_public_seam_compression(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_model_check_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &probe,
    );
}

#[test]
fn model_check_public_seam_compression_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_model_check_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_model_check_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &probe,
    );
}

#[test]
fn model_check_public_seam_compression_reaches_late_join_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_model_check_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_model_check_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &probe,
    );
}

#[test]
fn model_check_public_seam_compression_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_model_check_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_model_check_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &probe,
    );
}

#[test]
fn model_check_public_seam_compression_reaches_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let threshold =
        build_current_l2_source_sample_model_check_final_public_contract_reopen_threshold(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let compression = build_current_l2_source_sample_model_check_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_model_check_public_seam_compression_matches_prior_floors(
        &compression,
        &threshold,
        &probe,
    );
}

#[test]
fn model_check_public_seam_compression_keeps_guarded_prototype_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let compression = build_current_l2_source_sample_model_check_public_seam_compression(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        compression.compression_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        compression
            .compression_guard_reason
            .as_ref()
            .unwrap()
            .contains("model-check public seam compression")
    );
    assert!(compression.repo_local_emitted_artifact_refs.is_empty());
    assert!(compression.property_language_probe_refs.is_empty());
    assert!(compression.public_seam_residual_refs.is_empty());
    assert_eq!(
        compression.compare_floor_refs,
        vec!["compare_floor:current_l2.model_check.public_seam_compression.guard_only".to_string()]
    );
    assert_eq!(
        compression.guard_refs,
        vec!["guard:model_check_public_seam_compression_not_reached".to_string()]
    );
}
