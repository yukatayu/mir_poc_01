use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremLeanStubPilotActualization,
    CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge,
    build_current_l2_source_sample_theorem_lean_stub_pilot_actualization,
    build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge,
};

fn order_handoff_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn host_plan_path(name: &str) -> PathBuf {
    fixture_path(name).with_extension("host-plan.json")
}

fn source_sample_host_plan(name: &str) -> FixtureHostPlan {
    load_host_plan_from_path(&host_plan_path(name)).unwrap()
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

fn assert_trace_alignment_matches_pilot(
    bridge: &CurrentL2SourceSampleTheoremLeanStubTraceAlignmentBridge,
    pilot: &CurrentL2SourceSampleTheoremLeanStubPilotActualization,
    expected_pairs: &[&str],
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(bridge.alignment_status, pilot.pilot_status);
    assert_eq!(bridge.alignment_subject_kind, pilot.pilot_subject_kind);
    assert_eq!(bridge.alignment_subject_ref, pilot.pilot_subject_ref);
    assert_eq!(
        bridge.principal_review_unit_refs,
        pilot.principal_review_unit_refs
    );
    assert_eq!(
        bridge.repo_local_emitted_artifact_refs,
        pilot.repo_local_emitted_artifact_refs
    );
    assert_eq!(
        bridge.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        bridge.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        bridge.kept_later_refs,
        vec![
            "kept_later:actual_lean_tool_execution".to_string(),
            "kept_later:prototype_wide_trace_alignment".to_string(),
            "kept_later:public_theorem_contract".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:cross_tool_public_artifact_conformance_contract".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match bridge.alignment_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            let expected_pairs = expected_pairs
                .iter()
                .map(|entry| entry.to_string())
                .collect::<Vec<_>>();
            assert!(bridge.alignment_guard_reason.is_none());
            assert_eq!(bridge.review_unit_pair_refs, expected_pairs);
            assert_eq!(bridge.lean_stub_pair_refs, expected_pairs);
            assert_eq!(bridge.matched_pair_refs, expected_pairs);
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(bridge
                .alignment_guard_reason
                .as_ref()
                .unwrap()
                .contains("trace alignment"));
            assert!(bridge.review_unit_pair_refs.is_empty());
            assert!(bridge.lean_stub_pair_refs.is_empty());
            assert!(bridge.matched_pair_refs.is_empty());
        }
    }
}

#[test]
fn theorem_trace_alignment_bridge_reaches_runtime_sample() {
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        "e2-try-fallback",
        source_sample_host_plan("e2-try-fallback.json"),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        "e2-try-fallback",
        source_sample_host_plan("e2-try-fallback.json"),
    )
    .unwrap();

    assert_trace_alignment_matches_pilot(
        &bridge,
        &pilot,
        &["theorem_trace_alignment_pair:e2-try-fallback:rollback_cut_non_interference"],
        &[
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_actual_lean_execution",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_trace_alignment_bridge_reaches_static_underdeclared_sample() {
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_trace_alignment_matches_pilot(
        &bridge,
        &pilot,
        &[
            "theorem_trace_alignment_pair:e5-underdeclared-lineage:canonical_normalization_law",
            "theorem_trace_alignment_pair:e5-underdeclared-lineage:no_re_promotion",
        ],
        &[
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_actual_lean_execution",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_trace_alignment_bridge_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_trace_alignment_matches_pilot(
        &bridge,
        &pilot,
        &[
            "theorem_trace_alignment_pair:p06-typed-proof-owner-handoff:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_actual_lean_execution",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_trace_alignment_bridge_reaches_late_join_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_trace_alignment_matches_pilot(
        &bridge,
        &pilot,
        &[
            "theorem_trace_alignment_pair:p07-dice-late-join-visible-history:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_actual_lean_execution",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_trace_alignment_bridge_reaches_stale_reconnect_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_trace_alignment_matches_pilot(
        &bridge,
        &pilot,
        &[
            "theorem_trace_alignment_pair:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference",
        ],
        &[
            "compare_floor:current_l2.theorem_lean_stub_pilot_actualization",
            "compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge",
        ],
        &[
            "guard:repo_local_trace_alignment_only",
            "guard:no_actual_lean_execution",
            "guard:no_public_theorem_contract_promotion",
        ],
    );
}

#[test]
fn theorem_trace_alignment_bridge_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let pilot = build_current_l2_source_sample_theorem_lean_stub_pilot_actualization(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let bridge = build_current_l2_source_sample_theorem_lean_stub_trace_alignment_bridge(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_trace_alignment_matches_pilot(
        &bridge,
        &pilot,
        &[],
        &["compare_floor:current_l2.theorem_lean_stub_trace_alignment_bridge.guard_only"],
        &["guard:theorem_lean_stub_trace_alignment_bridge_not_reached"],
    );
}
