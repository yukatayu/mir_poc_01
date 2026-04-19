use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleTheoremFirstPilotActualization,
    CurrentL2SourceSampleTheoremProverBindingPreflight,
    build_current_l2_source_sample_theorem_first_pilot_actualization,
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

fn expected_manifest_refs(
    pilot: &CurrentL2SourceSampleTheoremFirstPilotActualization,
) -> Vec<String> {
    pilot.principal_review_unit_refs
        .iter()
        .map(|entry| {
            let (_, subject_ref, obligation_kind) = entry
                .split_once(':')
                .and_then(|(_, rest)| rest.split_once(':'))
                .map(|(subject_ref, obligation_kind)| ("proof_notebook_review_unit", subject_ref, obligation_kind))
                .unwrap();
            format!("theorem_binding_preflight:{subject_ref}:{obligation_kind}")
        })
        .collect()
}

fn assert_preflight_matches_pilot(
    preflight: &CurrentL2SourceSampleTheoremProverBindingPreflight,
    pilot: &CurrentL2SourceSampleTheoremFirstPilotActualization,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(preflight.preflight_status, pilot.pilot_status);
    assert_eq!(preflight.preflight_subject_kind, pilot.pilot_subject_kind);
    assert_eq!(preflight.preflight_subject_ref, pilot.pilot_subject_ref);
    assert_eq!(
        preflight.principal_review_unit_refs,
        pilot.principal_review_unit_refs
    );
    assert_eq!(preflight.symbolic_evidence_refs, pilot.symbolic_evidence_refs);
    assert_eq!(preflight.binding_preflight_manifest_refs, expected_manifest_refs(pilot));
    assert_eq!(
        preflight.adapter_boundary_refs,
        match preflight.preflight_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "adapter_boundary:principal_review_unit_first".to_string(),
                "adapter_boundary:symbolic_evidence_refs_only".to_string(),
                "adapter_boundary:brand_neutral_theorem_request".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        preflight.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        preflight.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        preflight.kept_later_refs,
        vec![
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:actual_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
        ]
    );

    match preflight.preflight_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(preflight.preflight_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(preflight
                .preflight_guard_reason
                .as_ref()
                .unwrap()
                .contains("binding preflight"));
        }
    }
}

#[test]
fn theorem_binding_preflight_reaches_static_underdeclared_sample() {
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_preflight_matches_pilot(
        &preflight,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_first_pilot_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
        ],
        &[
            "guard:theorem_first_external_target",
            "guard:brand_neutral_preflight_only",
        ],
    );
}

#[test]
fn theorem_binding_preflight_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_preflight_matches_pilot(
        &preflight,
        &pilot,
        &["compare_floor:current_l2.theorem_binding_guarded_preview_only"],
        &["guard:binding_preflight_not_reached"],
    );
}

#[test]
fn theorem_binding_preflight_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_preflight_matches_pilot(
        &preflight,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_first_pilot_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
        ],
        &[
            "guard:theorem_first_external_target",
            "guard:brand_neutral_preflight_only",
        ],
    );
}

#[test]
fn theorem_binding_preflight_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let pilot = build_current_l2_source_sample_theorem_first_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_preflight_matches_pilot(
        &preflight,
        &pilot,
        &[
            "compare_floor:current_l2.theorem_first_pilot_actualization",
            "compare_floor:current_l2.theorem_binding_preflight",
        ],
        &[
            "guard:theorem_first_external_target",
            "guard:brand_neutral_preflight_only",
        ],
    );
}
