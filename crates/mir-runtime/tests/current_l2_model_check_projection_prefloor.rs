use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleModelCheckProjectionPrefloor,
    CurrentL2SourceSamplePreviewArtifactRoute,
    build_current_l2_source_sample_model_check_projection_prefloor,
    build_current_l2_source_sample_preview_artifact_route,
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

fn expected_carrier_refs(route: &CurrentL2SourceSamplePreviewArtifactRoute) -> Vec<String> {
    route
        .model_check_concrete_carriers
        .iter()
        .map(|carrier| {
            format!(
                "model_check_concrete_carrier:{}:{}",
                carrier.subject_ref, carrier.case.obligation_kind
            )
        })
        .collect()
}

fn assert_prefloor_matches_preview_route(
    prefloor: &CurrentL2SourceSampleModelCheckProjectionPrefloor,
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
    expected_projection_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(prefloor.projection_status, route.formal_hook_status);
    assert_eq!(
        prefloor.projection_subject_kind,
        route
            .formal_hook_artifact
            .as_ref()
            .map(|artifact| artifact.subject_kind.clone())
    );
    assert_eq!(
        prefloor.projection_subject_ref,
        route
            .formal_hook_artifact
            .as_ref()
            .map(|artifact| artifact.subject_ref.clone())
    );
    assert_eq!(
        prefloor.principal_machine_carrier_refs,
        expected_carrier_refs(route)
    );
    assert_eq!(
        prefloor.small_cluster_projection_refs,
        expected_projection_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        prefloor.property_language_seam_refs,
        vec!["property_language_seam:current_l2.model_check.small_cluster.semantic".to_string()]
    );
    assert_eq!(
        prefloor.tool_binding_seam_refs,
        vec!["tool_binding_seam:current_l2.model_check.reserve".to_string()]
    );
    assert_eq!(
        prefloor.excluded_family_refs,
        vec![
            "excluded_family:typed_reserve_cluster".to_string(),
            "excluded_family:theorem_discharge_transport".to_string(),
            "excluded_family:room_protocol_projection".to_string(),
        ]
    );
    assert_eq!(
        prefloor.kept_later_refs,
        vec![
            "kept_later:fairness_replay_operational_profile".to_string(),
            "kept_later:production_checker_runtime_policy_contract".to_string(),
        ]
    );
    assert_eq!(
        prefloor.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );

    match prefloor.projection_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(prefloor.projection_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                prefloor
                    .projection_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("runtime_try_cut_cluster")
            );
        }
    }
}

#[test]
fn model_check_projection_prefloor_reaches_static_underdeclared_sample() {
    let route = build_current_l2_source_sample_preview_artifact_route(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["small_cluster_projection:e5-underdeclared-lineage:fixture_static_rejection"],
        &["guard:static_rejection_cluster"],
    );
}

#[test]
fn model_check_projection_prefloor_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[],
        &["guard:projection_not_reached"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_typed_runtime_prototype_without_collapsing_theorem_or_typed_lines()
 {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["small_cluster_projection:p06-typed-proof-owner-handoff:runtime_try_cut_local"],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_authorized_ifc_typed_runtime_prototype() {
    let sample_path =
        typed_prototype_sample_path("p10-typed-authorized-fingerprint-declassification.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[
            "small_cluster_projection:p10-typed-authorized-fingerprint-declassification:runtime_try_cut_local",
        ],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_unauthorized_ifc_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p11-typed-unauthorized-fingerprint-release.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[
            "small_cluster_projection:p11-typed-unauthorized-fingerprint-release:runtime_try_cut_local",
        ],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_label_flow_negative_ifc_typed_runtime_prototype() {
    let sample_path =
        typed_prototype_sample_path("p12-typed-classified-fingerprint-publication-block.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[
            "small_cluster_projection:p12-typed-classified-fingerprint-publication-block:runtime_try_cut_local",
        ],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_capture_escape_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p15-typed-capture-escape-rejected.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["small_cluster_projection:p15-typed-capture-escape-rejected:runtime_try_cut_local"],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_cost_bound_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p16-typed-remote-call-budget-exceeded.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["small_cluster_projection:p16-typed-remote-call-budget-exceeded:runtime_try_cut_local"],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["small_cluster_projection:p07-dice-late-join-visible-history:runtime_try_cut_local"],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_delegated_rng_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[
            "small_cluster_projection:p09-dice-delegated-rng-provider-placement:runtime_try_cut_local",
        ],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}

#[test]
fn model_check_projection_prefloor_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_model_check_projection_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["small_cluster_projection:p08-dice-stale-reconnect-refresh:runtime_try_cut_local"],
        &["guard:rollback_or_atomic_cut_evidence_present"],
    );
}
