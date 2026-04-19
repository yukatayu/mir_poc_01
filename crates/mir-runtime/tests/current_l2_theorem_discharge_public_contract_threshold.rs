use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleTheoremDischargeActualFormatProbe,
    CurrentL2SourceSampleTheoremDischargePublicContractThreshold,
    CurrentL2SourceSampleTheoremProverBindingPreflight,
    build_current_l2_source_sample_theorem_discharge_actual_format_probe,
    build_current_l2_source_sample_theorem_discharge_public_contract_threshold,
    build_current_l2_source_sample_theorem_prover_binding_preflight,
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

fn assert_threshold_matches_probe_and_preflight(
    threshold: &CurrentL2SourceSampleTheoremDischargePublicContractThreshold,
    probe: &CurrentL2SourceSampleTheoremDischargeActualFormatProbe,
    preflight: &CurrentL2SourceSampleTheoremProverBindingPreflight,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(threshold.threshold_status, probe.probe_status);
    assert_eq!(threshold.threshold_status, preflight.preflight_status);
    assert_eq!(threshold.threshold_subject_kind, probe.probe_subject_kind);
    assert_eq!(threshold.threshold_subject_ref, probe.probe_subject_ref);
    assert_eq!(
        threshold.principal_review_unit_refs,
        probe.principal_review_unit_refs
    );
    assert_eq!(
        threshold.discharge_entry_reserve_refs,
        probe.discharge_entry_reserve_refs
    );
    assert_eq!(threshold.symbolic_evidence_refs, probe.symbolic_evidence_refs);
    assert_eq!(threshold.transport_preview_refs, probe.transport_preview_refs);
    assert_eq!(
        threshold.public_contract_preview_refs,
        probe.public_contract_preview_refs
    );
    assert_eq!(threshold.consumer_boundary_refs, probe.consumer_boundary_refs);
    assert_eq!(
        threshold.binding_preflight_manifest_refs,
        preflight.binding_preflight_manifest_refs
    );
    assert_eq!(threshold.adapter_boundary_refs, preflight.adapter_boundary_refs);
    assert_eq!(
        threshold.threshold_default_refs,
        match threshold.threshold_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "theorem_contract_default:review_unit_first".to_string(),
                "theorem_contract_default:discharge_entry_adjacent".to_string(),
                "theorem_contract_default:notebook_consumer_first".to_string(),
                "theorem_contract_default:brand_neutral_theorem_request".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        threshold.repo_local_emitted_artifact_refs,
        probe.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        threshold.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
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
            "kept_later:actual_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
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
                .contains("theorem discharge/public-contract threshold"));
        }
    }
}

#[test]
fn theorem_contract_threshold_reaches_static_underdeclared_sample() {
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_threshold_matches_probe_and_preflight(
        &threshold,
        &probe,
        &preflight,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
        ],
        &[
            "guard:review_unit_first_threshold_only",
            "guard:transport_preview_only",
            "guard:public_contract_preview_only",
            "guard:brand_neutral_request_only",
        ],
    );
}

#[test]
fn theorem_contract_threshold_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_probe_and_preflight(
        &threshold,
        &probe,
        &preflight,
        &["compare_floor:current_l2.theorem_contract_threshold_guard_only"],
        &["guard:theorem_contract_threshold_not_reached"],
    );
}

#[test]
fn theorem_contract_threshold_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_probe_and_preflight(
        &threshold,
        &probe,
        &preflight,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
        ],
        &[
            "guard:review_unit_first_threshold_only",
            "guard:transport_preview_only",
            "guard:public_contract_preview_only",
            "guard:brand_neutral_request_only",
        ],
    );
}

#[test]
fn theorem_contract_threshold_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_probe_and_preflight(
        &threshold,
        &probe,
        &preflight,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
        ],
        &[
            "guard:review_unit_first_threshold_only",
            "guard:transport_preview_only",
            "guard:public_contract_preview_only",
            "guard:brand_neutral_request_only",
        ],
    );
}

#[test]
fn theorem_contract_threshold_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let threshold = build_current_l2_source_sample_theorem_discharge_public_contract_threshold(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_threshold_matches_probe_and_preflight(
        &threshold,
        &probe,
        &preflight,
        &[
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_contract_threshold",
        ],
        &[
            "guard:review_unit_first_threshold_only",
            "guard:transport_preview_only",
            "guard:public_contract_preview_only",
            "guard:brand_neutral_request_only",
        ],
    );
}
