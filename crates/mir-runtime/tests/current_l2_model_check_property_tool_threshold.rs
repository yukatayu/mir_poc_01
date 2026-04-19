use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleModelCheckPropertyToolSeamProbe,
    CurrentL2SourceSampleModelCheckPropertyToolThreshold,
    CurrentL2SourceSampleModelCheckSecondLineConcretization,
    build_current_l2_source_sample_model_check_property_tool_seam_probe,
    build_current_l2_source_sample_model_check_property_tool_threshold,
    build_current_l2_source_sample_model_check_second_line_concretization,
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

fn assert_threshold_matches_second_line_and_probe(
    threshold: &CurrentL2SourceSampleModelCheckPropertyToolThreshold,
    second_line: &CurrentL2SourceSampleModelCheckSecondLineConcretization,
    probe: &CurrentL2SourceSampleModelCheckPropertyToolSeamProbe,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(threshold.threshold_status, second_line.concretization_status);
    assert_eq!(threshold.threshold_status, probe.probe_status);
    assert_eq!(
        threshold.threshold_subject_kind,
        second_line.concretization_subject_kind
    );
    assert_eq!(
        threshold.threshold_subject_ref,
        second_line.concretization_subject_ref
    );
    assert_eq!(
        threshold.principal_machine_carrier_refs,
        second_line.principal_machine_carrier_refs
    );
    assert_eq!(
        threshold.row_local_property_preview_refs,
        second_line.row_local_property_preview_refs
    );
    assert_eq!(
        threshold.secondary_projection_refs,
        second_line.secondary_projection_refs
    );
    assert_eq!(
        threshold.property_language_probe_refs,
        probe.property_language_probe_refs
    );
    assert_eq!(threshold.tool_seam_probe_refs, probe.tool_seam_probe_refs);
    assert_eq!(
        threshold.checker_boundary_probe_refs,
        probe.checker_boundary_probe_refs
    );
    assert_eq!(threshold.request_preflight_refs, second_line.request_preflight_refs);
    assert_eq!(
        threshold.public_checker_reserve_refs,
        second_line.public_checker_reserve_refs
    );
    assert_eq!(
        threshold.threshold_default_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "model_check_threshold_default:row_local_property_preview_first".to_string(),
                "model_check_threshold_default:small_cluster_semantic_projection_second"
                    .to_string(),
                "model_check_threshold_default:brand_neutral_model_check_request".to_string(),
                "model_check_threshold_default:public_checker_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        threshold.repo_local_emitted_artifact_refs,
        second_line.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        threshold.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.excluded_family_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "excluded_family:theorem_discharge_transport".to_string(),
                "excluded_family:room_protocol_projection".to_string(),
                "excluded_family:provider_receipt_fairness_family".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
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
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
        ]
    );

    match threshold.threshold_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(threshold.threshold_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(threshold
                .threshold_guard_reason
                .as_ref()
                .unwrap()
                .contains("model-check property/tool threshold"));
        }
    }
}

#[test]
fn model_check_property_tool_threshold_reaches_static_underdeclared_sample() {
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_threshold_matches_second_line_and_probe(
        &threshold,
        &second_line,
        &probe,
        &[
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
            "compare_floor:current_l2.model_check.property_tool_threshold",
        ],
        &[
            "guard:row_local_property_preview_threshold_only",
            "guard:property_language_probe_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:public_checker_contract_later",
        ],
    );
}

#[test]
fn model_check_property_tool_threshold_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_second_line_and_probe(
        &threshold,
        &second_line,
        &probe,
        &["compare_floor:current_l2.model_check.property_tool_threshold_guard_only"],
        &["guard:model_check_property_tool_threshold_not_reached"],
    );
}

#[test]
fn model_check_property_tool_threshold_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_second_line_and_probe(
        &threshold,
        &second_line,
        &probe,
        &[
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
            "compare_floor:current_l2.model_check.property_tool_threshold",
        ],
        &[
            "guard:row_local_property_preview_threshold_only",
            "guard:property_language_probe_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:public_checker_contract_later",
        ],
    );
}

#[test]
fn model_check_property_tool_threshold_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_second_line_and_probe(
        &threshold,
        &second_line,
        &probe,
        &[
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
            "compare_floor:current_l2.model_check.property_tool_threshold",
        ],
        &[
            "guard:row_local_property_preview_threshold_only",
            "guard:property_language_probe_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:public_checker_contract_later",
        ],
    );
}

#[test]
fn model_check_property_tool_threshold_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_second_line_and_probe(
        &threshold,
        &second_line,
        &probe,
        &[
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
            "compare_floor:current_l2.model_check.property_tool_threshold",
        ],
        &[
            "guard:row_local_property_preview_threshold_only",
            "guard:property_language_probe_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:public_checker_contract_later",
        ],
    );
}

#[test]
fn model_check_property_tool_threshold_reaches_delegated_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let second_line = build_current_l2_source_sample_model_check_second_line_concretization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_model_check_property_tool_seam_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_model_check_property_tool_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_second_line_and_probe(
        &threshold,
        &second_line,
        &probe,
        &[
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
            "compare_floor:current_l2.model_check.property_tool_threshold",
        ],
        &[
            "guard:row_local_property_preview_threshold_only",
            "guard:property_language_probe_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:public_checker_contract_later",
        ],
    );
}
