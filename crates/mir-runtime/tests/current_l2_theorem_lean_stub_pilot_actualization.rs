use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremLeanStubPilotActualization,
    CurrentL2SourceSampleTheoremProverBindingPreflight,
    build_current_l2_source_sample_theorem_lean_stub_pilot_actualization,
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

fn assert_pilot_matches_preflight(
    pilot: &CurrentL2SourceSampleTheoremLeanStubPilotActualization,
    preflight: &CurrentL2SourceSampleTheoremProverBindingPreflight,
    expected_binding_refs: &[&str],
    expected_artifact_refs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(pilot.pilot_status, preflight.preflight_status);
    assert_eq!(pilot.pilot_subject_kind, preflight.preflight_subject_kind);
    assert_eq!(pilot.pilot_subject_ref, preflight.preflight_subject_ref);
    assert_eq!(
        pilot.principal_review_unit_refs,
        preflight.principal_review_unit_refs
    );
    assert_eq!(
        pilot.binding_preflight_manifest_refs,
        preflight.binding_preflight_manifest_refs
    );
    assert_eq!(
        pilot.pilot_binding_refs,
        expected_binding_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        pilot.repo_local_emitted_artifact_refs,
        expected_artifact_refs
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
        pilot.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        pilot.code_anchor_refs,
        match pilot.pilot_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "code_anchor:current_l2_proof_notebook_review_unit_support".to_string(),
                "code_anchor:current_l2_lean_theorem_stub_support".to_string(),
                "code_anchor:current_l2_emit_lean_theorem_stub".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        pilot.kept_later_refs,
        vec![
            "kept_later:actual_lean_tool_execution".to_string(),
            "kept_later:actual_discharge_transport".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
            "kept_later:rocq_iris_fallback".to_string(),
        ]
    );

    match pilot.pilot_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(pilot.pilot_guard_reason.is_none());
            assert!(!pilot.lean_stub_artifacts.is_empty());
            for artifact in &pilot.lean_stub_artifacts {
                assert_eq!(artifact.tool_family, "lean-first");
                assert!(artifact.source_text.contains("namespace CurrentL2"));
                assert!(artifact.source_text.contains("theorem "));
                assert!(artifact.source_text.contains("sorry"));
            }
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(pilot
                .pilot_guard_reason
                .as_ref()
                .unwrap()
                .contains("Lean stub"));
            assert!(pilot.lean_stub_artifacts.is_empty());
        }
    }
}

#[test]
fn theorem_lean_stub_pilot_reaches_static_underdeclared_sample() {
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_pilot_matches_preflight(
        &pilot,
        &preflight,
        &[
            "theorem_lean_stub_pilot:e5-underdeclared-lineage:lean_first_principal",
            "theorem_lean_stub_pilot:e5-underdeclared-lineage:review_unit_input_only",
            "theorem_lean_stub_pilot:e5-underdeclared-lineage:nonproduction_stub_only",
            "theorem_lean_stub_pilot:e5-underdeclared-lineage:rocq_iris_fallback_retained",
        ],
        &[
            "repo_local_emitted_artifact:lean_theorem_stub:e5-underdeclared-lineage:canonical_normalization_law",
            "repo_local_emitted_artifact:lean_theorem_stub:e5-underdeclared-lineage:no_re_promotion",
        ],
        &[
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
        ],
        &[
            "guard:lean_first_nonproduction_stub_only",
            "guard:review_unit_principal_input",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_lean_stub_pilot_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preflight(
        &pilot,
        &preflight,
        &[],
        &[],
        &["compare_floor:current_l2.theorem_lean_stub_pilot.guard_only"],
        &["guard:theorem_lean_stub_pilot_not_reached"],
    );
}

#[test]
fn theorem_lean_stub_pilot_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preflight(
        &pilot,
        &preflight,
        &[
            "theorem_lean_stub_pilot:p06-typed-proof-owner-handoff:lean_first_principal",
            "theorem_lean_stub_pilot:p06-typed-proof-owner-handoff:review_unit_input_only",
            "theorem_lean_stub_pilot:p06-typed-proof-owner-handoff:nonproduction_stub_only",
            "theorem_lean_stub_pilot:p06-typed-proof-owner-handoff:rocq_iris_fallback_retained",
        ],
        &[
            "repo_local_emitted_artifact:lean_theorem_stub:p06-typed-proof-owner-handoff:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
        ],
        &[
            "guard:lean_first_nonproduction_stub_only",
            "guard:review_unit_principal_input",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_lean_stub_pilot_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preflight(
        &pilot,
        &preflight,
        &[
            "theorem_lean_stub_pilot:p07-dice-late-join-visible-history:lean_first_principal",
            "theorem_lean_stub_pilot:p07-dice-late-join-visible-history:review_unit_input_only",
            "theorem_lean_stub_pilot:p07-dice-late-join-visible-history:nonproduction_stub_only",
            "theorem_lean_stub_pilot:p07-dice-late-join-visible-history:rocq_iris_fallback_retained",
        ],
        &[
            "repo_local_emitted_artifact:lean_theorem_stub:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
        ],
        &[
            "guard:lean_first_nonproduction_stub_only",
            "guard:review_unit_principal_input",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_lean_stub_pilot_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let preflight = build_current_l2_source_sample_theorem_prover_binding_preflight(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_pilot_matches_preflight(
        &pilot,
        &preflight,
        &[
            "theorem_lean_stub_pilot:p08-dice-stale-reconnect-refresh:lean_first_principal",
            "theorem_lean_stub_pilot:p08-dice-stale-reconnect-refresh:review_unit_input_only",
            "theorem_lean_stub_pilot:p08-dice-stale-reconnect-refresh:nonproduction_stub_only",
            "theorem_lean_stub_pilot:p08-dice-stale-reconnect-refresh:rocq_iris_fallback_retained",
        ],
        &[
            "repo_local_emitted_artifact:lean_theorem_stub:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
        ],
        &[
            "guard:lean_first_nonproduction_stub_only",
            "guard:review_unit_principal_input",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}
