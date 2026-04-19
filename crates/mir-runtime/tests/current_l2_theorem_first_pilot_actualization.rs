use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSamplePreviewArtifactRoute,
    CurrentL2SourceSampleTheoremFirstPilotActualization,
    build_current_l2_source_sample_preview_artifact_route,
    build_current_l2_source_sample_theorem_first_pilot_actualization,
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

fn expected_review_unit_refs(route: &CurrentL2SourceSamplePreviewArtifactRoute) -> Vec<String> {
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

fn expected_symbolic_evidence_refs(
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
) -> Vec<String> {
    let mut refs = Vec::new();

    for unit in &route.proof_notebook_review_units {
        for evidence in &unit.row.evidence_refs {
            let formatted = format!("{}:{}", evidence.ref_kind, evidence.ref_id);
            if !refs.contains(&formatted) {
                refs.push(formatted);
            }
        }
    }

    refs
}

fn assert_pilot_matches_preview_route(
    pilot: &CurrentL2SourceSampleTheoremFirstPilotActualization,
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
    expected_repo_local_artifact_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(pilot.pilot_status, route.formal_hook_status);
    assert_eq!(
        pilot.pilot_subject_kind,
        route
            .formal_hook_artifact
            .as_ref()
            .map(|artifact| artifact.subject_kind.clone())
    );
    assert_eq!(
        pilot.pilot_subject_ref,
        route
            .formal_hook_artifact
            .as_ref()
            .map(|artifact| artifact.subject_ref.clone())
    );
    assert_eq!(pilot.principal_review_unit_refs, expected_review_unit_refs(route));
    assert_eq!(pilot.symbolic_evidence_refs, expected_symbolic_evidence_refs(route));
    assert_eq!(
        pilot.repo_local_emitted_artifact_refs,
        expected_repo_local_artifact_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        pilot.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        pilot.kept_later_refs,
        vec![
            "kept_later:theorem_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:concrete_theorem_prover_binding".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
        ]
    );
    assert_eq!(
        pilot.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );

    match pilot.pilot_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(pilot.pilot_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(pilot
                .pilot_guard_reason
                .as_ref()
                .unwrap()
                .contains("runtime_try_cut_cluster"));
        }
    }
}

#[test]
fn theorem_first_pilot_actualization_reaches_static_underdeclared_sample() {
    let route = build_current_l2_source_sample_preview_artifact_route(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_pilot_matches_preview_route(
        &pilot,
        &route,
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:e5-underdeclared-lineage:canonical_normalization_law",
            "repo_local_emitted_artifact:proof_notebook_review_unit:e5-underdeclared-lineage:no_re_promotion",
        ],
        &[
            "compare_floor:sample_local_preview_aligned_typed_artifact_route",
            "compare_floor:current_l2.theorem_discharge_prefloor",
        ],
        &["guard:notebook_consumer_threshold_ready"],
    );
}

#[test]
fn theorem_first_pilot_actualization_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preview_route(
        &pilot,
        &route,
        &[],
        &["compare_floor:current_l2.theorem_guarded_preview_only"],
        &["guard:pilot_not_reached"],
    );
}

#[test]
fn theorem_first_pilot_actualization_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preview_route(
        &pilot,
        &route,
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p06-typed-proof-owner-handoff:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:sample_local_preview_aligned_typed_artifact_route",
            "compare_floor:current_l2.theorem_discharge_prefloor",
        ],
        &["guard:notebook_consumer_threshold_ready"],
    );
}

#[test]
fn theorem_first_pilot_actualization_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preview_route(
        &pilot,
        &route,
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:sample_local_preview_aligned_typed_artifact_route",
            "compare_floor:current_l2.theorem_discharge_prefloor",
        ],
        &["guard:notebook_consumer_threshold_ready"],
    );
}

#[test]
fn theorem_first_pilot_actualization_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preview_route(
        &pilot,
        &route,
        &[
            "repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:sample_local_preview_aligned_typed_artifact_route",
            "compare_floor:current_l2.theorem_discharge_prefloor",
        ],
        &["guard:notebook_consumer_threshold_ready"],
    );
}
