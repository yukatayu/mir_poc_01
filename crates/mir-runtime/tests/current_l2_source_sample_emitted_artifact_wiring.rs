use std::path::PathBuf;

use mir_semantics::{FixtureHostPlan, load_bundle_from_fixture_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, build_current_l2_source_sample_emitted_artifact_route,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

#[test]
fn emitted_artifact_wiring_reaches_runtime_review_unit_and_model_check_carrier() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e2-try-fallback.json")).unwrap();
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e2-try-fallback",
        bundle.host_plan.unwrap(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(route.source_report.sample_id, "e2-try-fallback");
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "runtime_try_cut_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 1);
    assert_eq!(route.model_check_concrete_carriers.len(), 1);
}

#[test]
fn emitted_artifact_wiring_reaches_static_review_units_and_model_check_carriers() {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e4-malformed-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(route.source_report.sample_id, "e4-malformed-lineage");
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_reaches_underdeclared_lineage_static_review_units_and_model_check_carriers()
 {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(route.source_report.sample_id, "e5-underdeclared-lineage");
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_reaches_underdeclared_target_static_review_units_and_model_check_carriers()
 {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e12-underdeclared-target-missing",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(
        route.source_report.sample_id,
        "e12-underdeclared-target-missing"
    );
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_reaches_duplicate_option_static_review_units_and_model_check_carriers() {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e14-malformed-duplicate-option-declaration",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(
        route.source_report.sample_id,
        "e14-malformed-duplicate-option-declaration"
    );
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_reaches_duplicate_chain_static_review_units_and_model_check_carriers() {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e15-malformed-duplicate-chain-declaration",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(
        route.source_report.sample_id,
        "e15-malformed-duplicate-chain-declaration"
    );
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_reaches_capability_static_review_units_and_model_check_carriers() {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e13-malformed-capability-strengthening",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(
        route.source_report.sample_id,
        "e13-malformed-capability-strengthening"
    );
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_reaches_late_capability_static_review_units_and_model_check_carriers() {
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e20-malformed-late-capability-strengthening",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert_eq!(
        route.source_report.sample_id,
        "e20-malformed-late-capability-strengthening"
    );
    assert_eq!(
        route.formal_hook_artifact.as_ref().unwrap().subject_kind,
        "fixture_static_cluster"
    );
    assert_eq!(route.proof_notebook_review_units.len(), 2);
    assert_eq!(route.model_check_concrete_carriers.len(), 2);
}

#[test]
fn emitted_artifact_wiring_keeps_e3_guarded_and_emits_no_followup_artifacts() {
    let bundle = load_bundle_from_fixture_path(fixture_path("e3-option-admit-chain.json")).unwrap();
    let route = build_current_l2_source_sample_emitted_artifact_route(
        "e3-option-admit-chain",
        bundle.host_plan.unwrap(),
    )
    .unwrap();

    assert_eq!(
        route.formal_hook_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert_eq!(route.source_report.sample_id, "e3-option-admit-chain");
    assert!(route.formal_hook_artifact.is_none());
    assert!(
        route
            .formal_hook_guard_reason
            .as_ref()
            .unwrap()
            .contains("runtime_try_cut_cluster")
    );
    assert!(route.proof_notebook_review_units.is_empty());
    assert!(route.model_check_concrete_carriers.is_empty());
}
