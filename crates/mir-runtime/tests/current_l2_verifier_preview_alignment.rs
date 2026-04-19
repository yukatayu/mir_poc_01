use std::path::{Path, PathBuf};

use mir_runtime::current_l2_cli::run_current_l2_operational_cli;
use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};
use serde_json::Value;

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSamplePreviewArtifactRoute,
    build_current_l2_source_sample_preview_artifact_route,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn host_plan_path(name: &str) -> PathBuf {
    fixture_path(name).with_extension("host-plan.json")
}

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

fn cli_json(sample_argument: &str, host_plan_path: Option<&Path>) -> Value {
    let mut args = vec!["run-source-sample".to_string(), sample_argument.to_string()];
    if let Some(host_plan_path) = host_plan_path {
        args.push("--host-plan".to_string());
        args.push(host_plan_path.to_str().unwrap().to_string());
    }
    args.push("--format".to_string());
    args.push("json".to_string());

    let output = run_current_l2_operational_cli(args).unwrap();
    serde_json::from_str(&output).unwrap()
}

fn expected_route_status(route: &CurrentL2SourceSamplePreviewArtifactRoute) -> &'static str {
    match route.formal_hook_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => "reached",
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => "guarded_not_reached",
    }
}

fn expected_subject_kind(route: &CurrentL2SourceSamplePreviewArtifactRoute) -> Value {
    route
        .formal_hook_artifact
        .as_ref()
        .map_or(Value::Null, |artifact| {
            Value::String(artifact.subject_kind.clone())
        })
}

fn expected_subject_ref(route: &CurrentL2SourceSamplePreviewArtifactRoute) -> Value {
    route
        .formal_hook_artifact
        .as_ref()
        .map_or(Value::Null, |artifact| {
            Value::String(artifact.subject_ref.clone())
        })
}

fn preview_string_list(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap().to_string())
        .collect()
}

fn expected_proof_obligations(route: &CurrentL2SourceSamplePreviewArtifactRoute) -> Vec<String> {
    route
        .proof_notebook_review_units
        .iter()
        .map(|unit| unit.row.obligation_kind.clone())
        .collect()
}

fn expected_model_check_obligations(
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
) -> Vec<String> {
    route
        .model_check_concrete_carriers
        .iter()
        .map(|carrier| carrier.case.obligation_kind.clone())
        .collect()
}

fn expected_evidence_refs_from_proof_unit(
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
) -> Vec<Vec<String>> {
    route
        .proof_notebook_review_units
        .iter()
        .map(|unit| {
            unit.row
                .evidence_refs
                .iter()
                .map(|evidence| format!("{}:{}", evidence.ref_kind, evidence.ref_id))
                .collect()
        })
        .collect()
}

fn expected_evidence_refs_from_model_check_carrier(
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
) -> Vec<Vec<String>> {
    route
        .model_check_concrete_carriers
        .iter()
        .map(|carrier| {
            carrier
                .case
                .evidence_refs
                .iter()
                .map(|evidence| format!("{}:{}", evidence.ref_kind, evidence.ref_id))
                .collect()
        })
        .collect()
}

fn preview_row_evidence_refs(rows: &[Value]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| preview_string_list(&row["evidence_refs"]))
        .collect()
}

fn assert_cli_preview_matches_preview_route(
    preview_json: &Value,
    route: &CurrentL2SourceSamplePreviewArtifactRoute,
) {
    assert_eq!(
        preview_json["verification_preview"]["formal_hook_status"],
        expected_route_status(route)
    );
    assert_eq!(
        preview_json["verification_preview"]["subject_kind"],
        expected_subject_kind(route)
    );
    assert_eq!(
        preview_json["verification_preview"]["subject_ref"],
        expected_subject_ref(route)
    );
    assert_eq!(
        preview_string_list(
            &preview_json["verification_preview"]["proof_notebook_review_unit_obligations"]
        ),
        expected_proof_obligations(route)
    );
    assert_eq!(
        preview_string_list(
            &preview_json["verification_preview"]["model_check_concrete_carrier_obligations"]
        ),
        expected_model_check_obligations(route)
    );

    let proof_rows = preview_json["artifact_preview"]["proof_notebook_review_units"]
        .as_array()
        .unwrap();
    let expected_proof_goal_texts = route
        .proof_notebook_review_units
        .iter()
        .map(|unit| unit.row.goal_text.clone())
        .collect::<Vec<_>>();
    let actual_proof_goal_texts = proof_rows
        .iter()
        .map(|row| row["goal_text"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        proof_rows
            .iter()
            .map(|row| row["obligation_kind"].as_str().unwrap().to_string())
            .collect::<Vec<_>>(),
        expected_proof_obligations(route)
    );
    assert_eq!(actual_proof_goal_texts, expected_proof_goal_texts);
    assert_eq!(
        preview_row_evidence_refs(proof_rows),
        expected_evidence_refs_from_proof_unit(route)
    );

    let model_check_rows = preview_json["artifact_preview"]["model_check_concrete_carriers"]
        .as_array()
        .unwrap();
    assert_eq!(
        model_check_rows
            .iter()
            .map(|row| row["obligation_kind"].as_str().unwrap().to_string())
            .collect::<Vec<_>>(),
        expected_model_check_obligations(route)
    );
    assert_eq!(
        preview_row_evidence_refs(model_check_rows),
        expected_evidence_refs_from_model_check_carrier(route)
    );

    match route.formal_hook_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(preview_json["verification_preview"]["guard_reason"].is_null());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            let guard_reason = preview_json["verification_preview"]["guard_reason"]
                .as_str()
                .unwrap();
            assert!(guard_reason.contains("runtime_try_cut_cluster"));
        }
    }
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_static_underdeclared_sample() {
    let preview_json = cli_json(
        "e5-underdeclared-lineage",
        Some(host_plan_path("e2-try-fallback.json").as_path()),
    );
    let route = build_current_l2_source_sample_preview_artifact_route(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_guarded_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_authorized_ifc_typed_runtime_prototype() {
    let sample_path =
        typed_prototype_sample_path("p10-typed-authorized-fingerprint-declassification.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_unauthorized_ifc_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p11-typed-unauthorized-fingerprint-release.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_label_flow_negative_ifc_typed_runtime_prototype()
 {
    let sample_path =
        typed_prototype_sample_path("p12-typed-classified-fingerprint-publication-block.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}

#[test]
fn verifier_preview_alignment_matches_emitted_route_for_delegated_rng_provider_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let route = build_current_l2_source_sample_preview_artifact_route(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_cli_preview_matches_preview_route(&preview_json, &route);
}
