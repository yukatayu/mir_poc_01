use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption,
    CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate,
    build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption,
    build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate,
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

fn assert_actual_adoption_matches_coupled_floor(
    actual_adoption: &CurrentL2SourceSampleTheoremReviewUnitTransportActualAdoption,
    coupled_gate: &CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate,
    expected_compare_floor_refs: &[&str],
    expected_guard_refs: &[&str],
) {
    assert_eq!(actual_adoption.actualization_status, coupled_gate.actualization_status);
    assert_eq!(
        actual_adoption.actualization_subject_kind,
        coupled_gate.actualization_subject_kind
    );
    assert_eq!(
        actual_adoption.actualization_subject_ref,
        coupled_gate.actualization_subject_ref
    );
    assert_eq!(
        actual_adoption.transport_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_transport_route:{}:review_unit_anchor",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_transport_route:{}:discharge_entry_ref_bundle",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_transport_route:{}:symbolic_evidence_refs_only",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_transport_route:{}:repo_local_emitted_artifact_refs",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.notebook_contract_route_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                format!(
                    "theorem_notebook_contract_route:{}:notebook_consumer_first",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_notebook_contract_route:{}:review_unit_reference_bundle",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_notebook_contract_route:{}:discharge_entry_adjacent",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
                format!(
                    "theorem_notebook_contract_route:{}:proof_object_public_schema_later",
                    actual_adoption.actualization_subject_ref.as_ref().unwrap()
                ),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.external_binding_reserve_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "binding_reserve:brand_neutral_request_manifest".to_string(),
                "binding_reserve:adapter_boundary_refs_keep".to_string(),
                "binding_reserve:concrete_theorem_prover_brand_later".to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.actual_adoption_default_refs,
        match actual_adoption.actualization_status {
            CurrentL2EmittedArtifactRouteStatus::Reached => vec![
                "theorem_actual_adoption_default:review_unit_transport_first".to_string(),
                "theorem_actual_adoption_default:notebook_consumer_contract_first".to_string(),
                "theorem_actual_adoption_default:transport_contract_adjacent_distinct"
                    .to_string(),
                "theorem_actual_adoption_default:proof_object_public_schema_later"
                    .to_string(),
            ],
            CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => Vec::new(),
        }
    );
    assert_eq!(
        actual_adoption.compare_floor_refs,
        expected_compare_floor_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.guard_refs,
        expected_guard_refs
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        actual_adoption.kept_later_refs,
        vec![
            "kept_later:theorem_result_public_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );

    match actual_adoption.actualization_status {
        CurrentL2EmittedArtifactRouteStatus::Reached => {
            assert!(actual_adoption.actualization_guard_reason.is_none());
        }
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => {
            assert!(actual_adoption
                .actualization_guard_reason
                .as_ref()
                .unwrap()
                .contains("theorem review-unit transport actual adoption"));
        }
    }
}

#[test]
fn theorem_review_unit_actual_adoption_reaches_static_underdeclared_sample() {
    let coupled_gate = build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        "e5-underdeclared-lineage",
        FixtureHostPlan::default(),
    )
    .unwrap();

    assert_actual_adoption_matches_coupled_floor(
        &actual_adoption,
        &coupled_gate,
        &[
            "compare_floor:current_l2.theorem_transport_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_actual_adoption_only",
            "guard:notebook_contract_actual_adoption_only",
            "guard:brand_neutral_binding_reserve_keep",
            "guard:proof_object_public_schema_later",
        ],
    );
}

#[test]
fn theorem_review_unit_actual_adoption_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let coupled_gate = build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_coupled_floor(
        &actual_adoption,
        &coupled_gate,
        &["compare_floor:current_l2.theorem_review_unit_transport.guard_only"],
        &["guard:theorem_review_unit_transport_not_reached"],
    );
}

#[test]
fn theorem_review_unit_actual_adoption_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let coupled_gate = build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_coupled_floor(
        &actual_adoption,
        &coupled_gate,
        &[
            "compare_floor:current_l2.theorem_transport_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_actual_adoption_only",
            "guard:notebook_contract_actual_adoption_only",
            "guard:brand_neutral_binding_reserve_keep",
            "guard:proof_object_public_schema_later",
        ],
    );
}

#[test]
fn theorem_review_unit_actual_adoption_reaches_order_handoff_runtime_prototype() {
    let sample_path =
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let coupled_gate = build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_coupled_floor(
        &actual_adoption,
        &coupled_gate,
        &[
            "compare_floor:current_l2.theorem_transport_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_actual_adoption_only",
            "guard:notebook_contract_actual_adoption_only",
            "guard:brand_neutral_binding_reserve_keep",
            "guard:proof_object_public_schema_later",
        ],
    );
}

#[test]
fn theorem_review_unit_actual_adoption_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let coupled_gate = build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();
    let actual_adoption = build_current_l2_source_sample_theorem_review_unit_transport_actual_adoption(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_actual_adoption_matches_coupled_floor(
        &actual_adoption,
        &coupled_gate,
        &[
            "compare_floor:current_l2.theorem_transport_contract.coupled_later_gate",
            "compare_floor:current_l2.theorem_binding_preflight",
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption",
        ],
        &[
            "guard:review_unit_transport_actual_adoption_only",
            "guard:notebook_contract_actual_adoption_only",
            "guard:brand_neutral_binding_reserve_keep",
            "guard:proof_object_public_schema_later",
        ],
    );
}
