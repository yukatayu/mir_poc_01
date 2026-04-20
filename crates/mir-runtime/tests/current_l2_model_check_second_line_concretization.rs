use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleModelCheckProjectionPrefloor,
    CurrentL2SourceSampleModelCheckSecondLineConcretization,
    build_current_l2_source_sample_model_check_projection_prefloor,
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

fn expected_property_preview_refs(
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
            format!("property_preview:row_local:{subject_ref}:{obligation_kind}")
        })
        .collect()
}

fn expected_repo_local_emitted_artifact_refs(
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
            format!(
                "repo_local_emitted_artifact:model_check_concrete_carrier:{subject_ref}:{obligation_kind}"
            )
        })
        .collect()
}

fn expected_request_preflight_refs(
    second_line: &CurrentL2SourceSampleModelCheckSecondLineConcretization,
) -> Vec<String> {
    match second_line.concretization_subject_ref.as_deref() {
        Some(subject_ref) => vec![
            format!("model_check_request_preflight:{subject_ref}:row_local_property_preview"),
            format!(
                "model_check_request_preflight:{subject_ref}:small_cluster_semantic_projection"
            ),
        ],
        None => Vec::new(),
    }
}

fn assert_second_line_matches_prefloor(
    second_line: &CurrentL2SourceSampleModelCheckSecondLineConcretization,
    prefloor: &CurrentL2SourceSampleModelCheckProjectionPrefloor,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(
        second_line.concretization_status,
        prefloor.projection_status
    );
    assert_eq!(
        second_line.concretization_subject_kind,
        prefloor.projection_subject_kind
    );
    assert_eq!(
        second_line.concretization_subject_ref,
        prefloor.projection_subject_ref
    );
    assert_eq!(
        second_line.principal_machine_carrier_refs,
        prefloor.principal_machine_carrier_refs
    );
    assert_eq!(
        second_line.row_local_property_preview_refs,
        expected_property_preview_refs(prefloor)
    );
    assert_eq!(
        second_line.secondary_projection_refs,
        prefloor.small_cluster_projection_refs
    );
    assert_eq!(
        second_line.request_preflight_refs,
        expected_request_preflight_refs(second_line)
    );
    assert_eq!(
        second_line.public_checker_reserve_refs,
        match second_line.concretization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "public_checker_second_reserve:payload_schema".to_string(),
                "public_checker_second_reserve:api_read_relation".to_string(),
                "public_checker_second_reserve:command_surface".to_string(),
                "public_checker_second_reserve:shared_output_contract".to_string(),
                "public_checker_second_reserve:boundary".to_string(),
                "public_checker_second_reserve:verifier_handoff_surface".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        second_line.repo_local_emitted_artifact_refs,
        expected_repo_local_emitted_artifact_refs(prefloor)
    );
    assert_eq!(
        second_line.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        second_line.excluded_family_refs,
        match second_line.concretization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "excluded_family:theorem_discharge_transport".to_string(),
                "excluded_family:room_protocol_projection".to_string(),
                "excluded_family:provider_receipt_fairness_family".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        second_line.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        second_line.kept_later_refs,
        vec![
            "kept_later:first_settled_property_language".to_string(),
            "kept_later:concrete_model_check_tool_brand".to_string(),
            "kept_later:actual_public_checker_migration".to_string(),
            "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
        ]
    );

    match second_line.concretization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(second_line.concretization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                second_line
                    .concretization_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("model-check second line")
            );
        }
    }
}

#[test]
fn model_check_second_line_concretization_reaches_static_underdeclared_sample() {
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}

#[test]
fn model_check_second_line_concretization_keeps_guarded_prototype_as_not_reached() {
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &["compare_floor:current_l2.model_check.second_line_guarded_preview_only"],
        &["guard:model_check_second_line_not_reached"],
    );
}

#[test]
fn model_check_second_line_concretization_reaches_typed_runtime_prototype() {
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}

#[test]
fn model_check_second_line_concretization_reaches_capture_escape_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p15-typed-capture-escape-rejected.txt");
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}

#[test]
fn model_check_second_line_concretization_reaches_cost_bound_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p16-typed-remote-call-budget-exceeded.txt");
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}

#[test]
fn model_check_second_line_concretization_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}

#[test]
fn model_check_second_line_concretization_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}

#[test]
fn model_check_second_line_concretization_reaches_delegated_provider_runtime_prototype_without_collapsing_fairness_line()
 {
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

    assert_second_line_matches_prefloor(
        &second_line,
        &prefloor,
        &[
            "compare_floor:current_l2.model_check_projection_prefloor",
            "compare_floor:current_l2.model_check.second_line_concretization",
        ],
        &[
            "guard:row_local_property_preview_only",
            "guard:brand_neutral_model_check_request_only",
            "guard:keep_public_checker_chain_docs_only",
        ],
    );
}
