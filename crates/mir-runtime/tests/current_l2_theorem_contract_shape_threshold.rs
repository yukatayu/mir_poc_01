use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleTheoremContractShapeThreshold,
    build_current_l2_source_sample_theorem_contract_shape_threshold,
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

fn assert_shape_threshold_reached(
    threshold: &CurrentL2SourceSampleTheoremContractShapeThreshold,
    expected_compare_floor_refs: &[&str],
) {
    assert_eq!(
        threshold.threshold_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(threshold.threshold_guard_reason.is_none());

    let subject_ref = threshold.threshold_subject_ref.as_deref().unwrap();
    assert_eq!(
        threshold.transport_shape_refs,
        vec![
            format!("theorem_transport_shape:{subject_ref}:review_unit_refs_only"),
            format!("theorem_transport_shape:{subject_ref}:discharge_entry_refs_only"),
            format!("theorem_transport_shape:{subject_ref}:brand_neutral_request_manifest_only"),
            format!("theorem_transport_shape:{subject_ref}:repo_local_emitted_artifact_refs_first"),
        ]
    );
    assert_eq!(
        threshold.public_contract_shape_refs,
        vec![
            format!("theorem_public_contract_shape:{subject_ref}:refs_only_reserve_schema"),
            format!("theorem_public_contract_shape:{subject_ref}:symbolic_evidence_refs_only"),
            format!("theorem_public_contract_shape:{subject_ref}:discharge_entry_adjacent"),
            format!("theorem_public_contract_shape:{subject_ref}:proof_object_later"),
        ]
    );
    assert_eq!(
        threshold.threshold_default_refs,
        vec![
            "theorem_shape_threshold_default:refs_only_reserve_schema_first".to_string(),
            "theorem_shape_threshold_default:review_unit_transport_anchor".to_string(),
            "theorem_shape_threshold_default:brand_neutral_request_manifest_keep".to_string(),
            "theorem_shape_threshold_default:consumer_shaped_payload_later".to_string(),
        ]
    );
    assert_eq!(
        threshold.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        threshold.contrast_refs,
        vec![
            "contrast_target:consumer_shaped_theorem_payload".to_string(),
            "contrast_target:concrete_theorem_prover_payload".to_string(),
            "contrast_target:proof_object_public_schema".to_string(),
            "contrast_target:source_surface_first_theorem_contract".to_string(),
        ]
    );
    assert_eq!(
        threshold.guard_refs,
        vec![
            "guard:theorem_shape_threshold_only".to_string(),
            "guard:refs_only_reserve_schema_first".to_string(),
            "guard:consumer_shaped_payload_later".to_string(),
            "guard:proof_object_public_schema_later".to_string(),
        ]
    );
    assert_eq!(
        threshold.kept_later_refs,
        vec![
            "kept_later:actual_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:theorem_result_public_object".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );
}

#[test]
fn theorem_contract_shape_threshold_reaches_static_underdeclared_sample() {
    let threshold = build_current_l2_source_sample_theorem_contract_shape_threshold(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_shape_threshold_reached(
        &threshold,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
            "compare_floor:current_l2.theorem_contract_shape_threshold",
        ],
    );
}

#[test]
fn theorem_contract_shape_threshold_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let threshold = build_current_l2_source_sample_theorem_contract_shape_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        threshold.source_report.sample_id,
        "p05-dice-owner-guarded-chain"
    );
    assert_eq!(
        threshold.threshold_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        threshold
            .threshold_guard_reason
            .as_ref()
            .unwrap()
            .contains("theorem contract shape threshold")
    );
    assert!(threshold.transport_shape_refs.is_empty());
    assert!(threshold.public_contract_shape_refs.is_empty());
    assert!(threshold.threshold_default_refs.is_empty());
    assert_eq!(
        threshold.compare_floor_refs,
        vec!["compare_floor:current_l2.theorem_contract_shape_threshold_guard_only".to_string()]
    );
    assert_eq!(
        threshold.guard_refs,
        vec!["guard:theorem_contract_shape_threshold_not_reached".to_string()]
    );
}

#[test]
fn theorem_contract_shape_threshold_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let threshold = build_current_l2_source_sample_theorem_contract_shape_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_shape_threshold_reached(
        &threshold,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
            "compare_floor:current_l2.theorem_contract_shape_threshold",
        ],
    );
}

#[test]
fn theorem_contract_shape_threshold_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let threshold = build_current_l2_source_sample_theorem_contract_shape_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_shape_threshold_reached(
        &threshold,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
            "compare_floor:current_l2.theorem_contract_shape_threshold",
        ],
    );
}

#[test]
fn theorem_contract_shape_threshold_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let threshold = build_current_l2_source_sample_theorem_contract_shape_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_shape_threshold_reached(
        &threshold,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
            "compare_floor:current_l2.theorem_contract_shape_threshold",
        ],
    );
}
