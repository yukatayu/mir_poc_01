use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSamplePreviewArtifactRoute,
    CurrentL2SourceSampleTheoremDischargePrefloor,
    build_current_l2_source_sample_preview_artifact_route,
    build_current_l2_source_sample_theorem_discharge_prefloor,
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

fn expected_review_unit_refs(
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
) -> Vec<String> {
    route
        .proof_notebook_review_units
        .iter()
        .map(|unit| {
            format!(
                "proof_notebook_review_unit:{}:{}",
                unit.subject_ref, unit.row.obligation_kind
            )
        })
        .collect()
}

fn assert_prefloor_matches_preview_route(
    prefloor: &CurrentL2SourceSampleTheoremDischargePrefloor,
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
    expected_discharge_entry_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(prefloor.discharge_status, route.formal_hook_status);
    assert_eq!(
        prefloor.discharge_subject_kind,
        route
            .formal_hook_artifact
            .as_ref()
            .map(|artifact| artifact.subject_kind.clone())
    );
    assert_eq!(
        prefloor.discharge_subject_ref,
        route
            .formal_hook_artifact
            .as_ref()
            .map(|artifact| artifact.subject_ref.clone())
    );
    assert_eq!(
        prefloor.principal_review_unit_refs,
        expected_review_unit_refs(route)
    );
    assert_eq!(
        prefloor.discharge_entry_reserve_refs,
        expected_discharge_entry_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        prefloor.transport_seam_refs,
        vec!["transport_seam:current_l2.theorem.discharge.reserve".to_string()]
    );
    assert_eq!(
        prefloor.public_contract_seam_refs,
        vec!["public_contract_seam:current_l2.theorem.notebook.reserve".to_string()]
    );
    assert_eq!(
        prefloor.kept_later_refs,
        vec![
            "kept_later:concrete_theorem_prover_binding".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
        ]
    );
    assert_eq!(
        prefloor.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );

    match prefloor.discharge_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(prefloor.discharge_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(prefloor
                .discharge_guard_reason
                .as_ref()
                .unwrap()
                .contains("runtime_try_cut_cluster"));
        }
    }
}

#[test]
fn theorem_discharge_prefloor_reaches_static_underdeclared_sample() {
    let route = build_current_l2_source_sample_preview_artifact_route(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[
            "discharge_entry_reserve:e5-underdeclared-lineage:canonical_normalization_law",
            "discharge_entry_reserve:e5-underdeclared-lineage:no_re_promotion",
        ],
        &["guard:notebook_consumer_threshold_ready"],
    );
}

#[test]
fn theorem_discharge_prefloor_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &[],
        &["guard:discharge_not_reached"],
    );
}

#[test]
fn theorem_discharge_prefloor_reaches_typed_runtime_prototype_without_premature_public_contract_adoption() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["discharge_entry_reserve:p06-typed-proof-owner-handoff:rollback_cut_non_interference"],
        &["guard:notebook_consumer_threshold_ready"],
    );
}

#[test]
fn theorem_discharge_prefloor_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["discharge_entry_reserve:p07-dice-late-join-visible-history:rollback_cut_non_interference"],
        &["guard:notebook_consumer_threshold_ready"],
    );
}

#[test]
fn theorem_discharge_prefloor_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_prefloor_matches_preview_route(
        &prefloor,
        &route,
        &["discharge_entry_reserve:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference"],
        &["guard:notebook_consumer_threshold_ready"],
    );
}
