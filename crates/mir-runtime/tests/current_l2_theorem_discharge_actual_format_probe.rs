use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleTheoremDischargeActualFormatProbe,
    CurrentL2SourceSampleTheoremDischargePrefloor,
    CurrentL2SourceSampleTheoremFirstPilotActualization,
    build_current_l2_source_sample_theorem_discharge_actual_format_probe,
    build_current_l2_source_sample_theorem_discharge_prefloor,
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

fn expected_transport_preview_refs(
    prefloor: &CurrentL2SourceSampleTheoremDischargePrefloor,
) -> Vec<String> {
    prefloor
        .discharge_entry_reserve_refs
        .iter()
        .map(|entry| {
            let (_, subject_ref, obligation_kind) = entry
                .split_once(':')
                .and_then(|(_, rest)| rest.split_once(':'))
                .map(|(subject_ref, obligation_kind)| {
                    ("discharge_entry_reserve", subject_ref, obligation_kind)
                })
                .unwrap();
            format!("theorem_discharge_transport_preview:{subject_ref}:{obligation_kind}")
        })
        .collect()
}

fn expected_public_contract_preview_refs(
    probe: &CurrentL2SourceSampleTheoremDischargeActualFormatProbe,
) -> Vec<String> {
    match probe.probe_subject_ref.as_deref() {
        Some(subject_ref) => vec![
            format!("theorem_public_contract_preview:{subject_ref}:review_unit_first"),
            format!("theorem_public_contract_preview:{subject_ref}:discharge_entry_adjacent"),
        ],
        None => Vec::new(),
    }
}

fn assert_probe_matches_prefloor_and_pilot(
    probe: &CurrentL2SourceSampleTheoremDischargeActualFormatProbe,
    prefloor: &CurrentL2SourceSampleTheoremDischargePrefloor,
    pilot: &CurrentL2SourceSampleTheoremFirstPilotActualization,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(probe.probe_status, prefloor.discharge_status);
    assert_eq!(probe.probe_subject_kind, prefloor.discharge_subject_kind);
    assert_eq!(probe.probe_subject_ref, prefloor.discharge_subject_ref);
    assert_eq!(
        probe.principal_review_unit_refs,
        prefloor.principal_review_unit_refs
    );
    assert_eq!(
        probe.discharge_entry_reserve_refs,
        prefloor.discharge_entry_reserve_refs
    );
    assert_eq!(probe.symbolic_evidence_refs, pilot.symbolic_evidence_refs);
    assert_eq!(
        probe.transport_preview_refs,
        expected_transport_preview_refs(prefloor)
    );
    assert_eq!(
        probe.public_contract_preview_refs,
        expected_public_contract_preview_refs(probe)
    );
    assert_eq!(
        probe.consumer_boundary_refs,
        match probe.probe_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "consumer_boundary:notebook_consumer_first".to_string(),
                "consumer_boundary:abstract_discharge_entry_only".to_string(),
                "consumer_boundary:brand_neutral_contract_probe".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        probe.repo_local_emitted_artifact_refs,
        pilot.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        probe.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
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
            "kept_later:actual_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
        ]
    );

    match probe.probe_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(probe.probe_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(
                probe
                    .probe_guard_reason
                    .as_ref()
                    .unwrap()
                    .contains("theorem discharge actual-format")
            );
        }
    }
}

#[test]
fn theorem_discharge_actual_format_probe_reaches_static_underdeclared_sample() {
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_probe_matches_prefloor_and_pilot(
        &probe,
        &prefloor,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_discharge_prefloor",
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
        ],
        &[
            "guard:transport_public_contract_coupled_later_gate",
            "guard:brand_neutral_contract_probe_only",
            "guard:review_unit_not_theorem_result",
        ],
    );
}

#[test]
fn theorem_discharge_actual_format_probe_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_probe_matches_prefloor_and_pilot(
        &probe,
        &prefloor,
        &pilot,
        &["compare_floor:current_l2.theorem_discharge.actual_format_guard_only"],
        &["guard:theorem_discharge_actual_format_not_reached"],
    );
}

#[test]
fn theorem_discharge_actual_format_probe_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_probe_matches_prefloor_and_pilot(
        &probe,
        &prefloor,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_discharge_prefloor",
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
        ],
        &[
            "guard:transport_public_contract_coupled_later_gate",
            "guard:brand_neutral_contract_probe_only",
            "guard:review_unit_not_theorem_result",
        ],
    );
}

#[test]
fn theorem_discharge_actual_format_probe_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_probe_matches_prefloor_and_pilot(
        &probe,
        &prefloor,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_discharge_prefloor",
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
        ],
        &[
            "guard:transport_public_contract_coupled_later_gate",
            "guard:brand_neutral_contract_probe_only",
            "guard:review_unit_not_theorem_result",
        ],
    );
}

#[test]
fn theorem_discharge_actual_format_probe_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let prefloor = build_current_l2_source_sample_theorem_discharge_prefloor(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let probe = build_current_l2_source_sample_theorem_discharge_actual_format_probe(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_probe_matches_prefloor_and_pilot(
        &probe,
        &prefloor,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_discharge_prefloor",
            "compare_floor:current_l2.theorem_discharge.actual_format_probe",
        ],
        &[
            "guard:transport_public_contract_coupled_later_gate",
            "guard:brand_neutral_contract_probe_only",
            "guard:review_unit_not_theorem_result",
        ],
    );
}
