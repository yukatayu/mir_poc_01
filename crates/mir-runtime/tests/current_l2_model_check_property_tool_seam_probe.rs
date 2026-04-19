use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleModelCheckProjectionPrefloor,
    CurrentL2SourceSampleModelCheckPropertyToolSeamProbe,
    CurrentL2SourceSampleModelCheckSecondLineConcretization,
    build_current_l2_source_sample_model_check_projection_prefloor,
    build_current_l2_source_sample_model_check_property_tool_seam_probe,
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

fn expected_property_language_probe_refs(
    prefloor: &CurrentL2SourceSampleModelCheckProjectionPrefloor,
) -> Vec<String> {
    prefloor
        .principal_machine_carrier_refs
        .iter()
        .map(|entry| {
            let (_, subject_ref, obligation_kind) = entry
                .split_once(':')
                .and_then(|(_, rest)| rest.split_once(':'))
                .map(|(subject_ref, obligation_kind)| {
                    ("model_check_concrete_carrier", subject_ref, obligation_kind)
                })
                .unwrap();
            format!("property_language_probe:row_local:{subject_ref}:{obligation_kind}")
        })
        .collect()
}

fn expected_tool_seam_probe_refs(
    second_line: &CurrentL2SourceSampleModelCheckSecondLineConcretization,
) -> Vec<String> {
    match second_line.concretization_subject_ref.as_deref() {
        Some(subject_ref) => vec![
            format!("tool_seam_probe:{subject_ref}:brand_neutral_model_check_request"),
            format!("tool_seam_probe:{subject_ref}:small_cluster_semantic_projection"),
        ],
        None => Vec::new(),
    }
}

fn assert_probe_matches_prefloor_and_second_line(
    probe: &CurrentL2SourceSampleModelCheckPropertyToolSeamProbe,
    prefloor: &CurrentL2SourceSampleModelCheckProjectionPrefloor,
    second_line: &CurrentL2SourceSampleModelCheckSecondLineConcretization,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(probe.probe_status, second_line.concretization_status);
    assert_eq!(
        probe.probe_subject_kind,
        second_line.concretization_subject_kind
    );
    assert_eq!(
        probe.probe_subject_ref,
        second_line.concretization_subject_ref
    );
    assert_eq!(
        probe.principal_machine_carrier_refs,
        prefloor.principal_machine_carrier_refs
    );
    assert_eq!(
        probe.row_local_property_preview_refs,
        second_line.row_local_property_preview_refs
    );
    assert_eq!(
        probe.secondary_projection_refs,
        second_line.secondary_projection_refs
    );
    assert_eq!(
        probe.property_language_probe_refs,
        expected_property_language_probe_refs(prefloor)
    );
    assert_eq!(
        probe.tool_seam_probe_refs,
        expected_tool_seam_probe_refs(second_line)
    );
    assert_eq!(
        probe.checker_boundary_probe_refs,
        match probe.probe_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "checker_boundary_probe:row_local_property_preview_first".to_string(),
                "checker_boundary_probe:brand_neutral_tool_probe_only".to_string(),
                "checker_boundary_probe:public_checker_contract_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        probe.repo_local_emitted_artifact_refs,
        second_line.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        probe.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        probe.excluded_family_refs,
        match probe.probe_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "excluded_family:theorem_discharge_actual_format".to_string(),
                "excluded_family:room_protocol_projection".to_string(),
                "excluded_family:provider_receipt_fairness_family".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        probe.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        probe.kept_later_refs,
        vec![
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
        ]
    );

    match probe.probe_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(probe.probe_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(probe
                .probe_guard_reason
                .as_ref()
                .unwrap()
                .contains("property/tool-seam"));
        }
    }
}

#[test]
fn model_check_property_tool_seam_probe_reaches_static_underdeclared_sample() {
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
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

    assert_probe_matches_prefloor_and_second_line(
        &probe,
        &prefloor,
        &second_line,
        &[
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
        ],
        &[
            "guard:settled_property_language_not_adopted",
            "guard:brand_neutral_tool_probe_only",
            "guard:public_checker_contract_later_gate",
        ],
    );
}

#[test]
fn model_check_property_tool_seam_probe_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
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

    assert_probe_matches_prefloor_and_second_line(
        &probe,
        &prefloor,
        &second_line,
        &["compare_floor:current_l2.model_check.property_tool_seam_guard_only"],
        &["guard:model_check_property_tool_seam_not_reached"],
    );
}

#[test]
fn model_check_property_tool_seam_probe_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
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

    assert_probe_matches_prefloor_and_second_line(
        &probe,
        &prefloor,
        &second_line,
        &[
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
        ],
        &[
            "guard:settled_property_language_not_adopted",
            "guard:brand_neutral_tool_probe_only",
            "guard:public_checker_contract_later_gate",
        ],
    );
}

#[test]
fn model_check_property_tool_seam_probe_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
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

    assert_probe_matches_prefloor_and_second_line(
        &probe,
        &prefloor,
        &second_line,
        &[
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
        ],
        &[
            "guard:settled_property_language_not_adopted",
            "guard:brand_neutral_tool_probe_only",
            "guard:public_checker_contract_later_gate",
        ],
    );
}

#[test]
fn model_check_property_tool_seam_probe_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
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

    assert_probe_matches_prefloor_and_second_line(
        &probe,
        &prefloor,
        &second_line,
        &[
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
        ],
        &[
            "guard:settled_property_language_not_adopted",
            "guard:brand_neutral_tool_probe_only",
            "guard:public_checker_contract_later_gate",
        ],
    );
}

#[test]
fn model_check_property_tool_seam_probe_reaches_delegated_provider_runtime_prototype_without_collapsing_fairness_line() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
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

    assert_probe_matches_prefloor_and_second_line(
        &probe,
        &prefloor,
        &second_line,
        &[
            "compare_floor:current_l2.model_check.second_line_concretization",
            "compare_floor:current_l2.model_check.property_tool_seam_probe",
        ],
        &[
            "guard:settled_property_language_not_adopted",
            "guard:brand_neutral_tool_probe_only",
            "guard:public_checker_contract_later_gate",
        ],
    );
}
