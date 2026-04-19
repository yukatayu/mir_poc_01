use std::path::{Path, PathBuf};

use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus,
    CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate,
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

fn assert_coupled_later_gate_reached(
    actualization: &CurrentL2SourceSampleTheoremTransportContractCoupledLaterGate,
) {
    assert_eq!(
        actualization.actualization_status,
        CurrentL2EmittedArtifactRouteStatus::Reached
    );
    assert!(actualization.actualization_guard_reason.is_none());

    let subject_ref = actualization.actualization_subject_ref.as_deref().unwrap();
    assert_eq!(
        actualization.transport_candidate_refs,
        vec![
            format!("theorem_transport_candidate:{subject_ref}:review_unit_anchor"),
            format!("theorem_transport_candidate:{subject_ref}:discharge_entry_adjacent"),
            format!("theorem_transport_candidate:{subject_ref}:symbolic_evidence_refs_only"),
            format!(
                "theorem_transport_candidate:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
        ]
    );
    assert_eq!(
        actualization.public_contract_candidate_refs,
        vec![
            format!("theorem_public_contract_candidate:{subject_ref}:notebook_consumer_adjacent"),
            format!("theorem_public_contract_candidate:{subject_ref}:refs_only_reserve_schema"),
            format!(
                "theorem_public_contract_candidate:{subject_ref}:brand_neutral_request_manifest_keep"
            ),
            format!("theorem_public_contract_candidate:{subject_ref}:consumer_payload_later"),
        ]
    );
    assert_eq!(
        actualization.coupled_default_refs,
        vec![
            "theorem_coupled_later_gate_default:transport_and_public_contract_adjacent_distinct"
                .to_string(),
            "theorem_coupled_later_gate_default:review_unit_anchor".to_string(),
            "theorem_coupled_later_gate_default:refs_only_reserve_schema_first".to_string(),
            "theorem_coupled_later_gate_default:proof_object_public_schema_later".to_string(),
        ]
    );
    assert_eq!(
        actualization.compare_floor_refs,
        vec![
            "compare_floor:current_l2.theorem_discharge.actual_format_probe".to_string(),
            "compare_floor:current_l2.theorem_contract_threshold".to_string(),
            "compare_floor:current_l2.theorem_contract_shape_threshold".to_string(),
            "compare_floor:current_l2.theorem_transport_contract_coupled_later_gate".to_string(),
        ]
    );
    assert_eq!(
        actualization.guard_refs,
        vec![
            "guard:transport_public_contract_adjacent_distinct".to_string(),
            "guard:actual_transport_not_yet_adopted".to_string(),
            "guard:public_contract_not_yet_adopted".to_string(),
            "guard:proof_object_public_schema_later".to_string(),
        ]
    );
    assert_eq!(
        actualization.kept_later_refs,
        vec![
            "kept_later:actual_discharge_transport_adoption".to_string(),
            "kept_later:public_theorem_contract_adoption".to_string(),
            "kept_later:theorem_result_public_object".to_string(),
            "kept_later:consumer_shaped_theorem_payload".to_string(),
            "kept_later:concrete_theorem_prover_brand".to_string(),
            "kept_later:proof_object_public_schema".to_string(),
            "kept_later:final_public_verifier_contract".to_string(),
        ]
    );
}

#[test]
fn theorem_transport_contract_coupled_later_gate_reaches_static_underdeclared_sample() {
    let actualization =
        build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
            "e5-underdeclared-lineage",
            FixtureHostPlan::default(),
        )
        .unwrap();

    assert_coupled_later_gate_reached(&actualization);
}

#[test]
fn theorem_transport_contract_coupled_later_gate_keeps_guarded_prototype_as_not_reached() {
    let sample_path = order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt");
    let actualization =
        build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_eq!(
        actualization.source_report.sample_id,
        "p05-dice-owner-guarded-chain"
    );
    assert_eq!(
        actualization.actualization_status,
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached
    );
    assert!(
        actualization
            .actualization_guard_reason
            .as_ref()
            .unwrap()
            .contains("theorem transport/public-contract coupled later gate")
    );
    assert!(actualization.transport_candidate_refs.is_empty());
    assert!(actualization.public_contract_candidate_refs.is_empty());
    assert!(actualization.coupled_default_refs.is_empty());
    assert_eq!(
        actualization.compare_floor_refs,
        vec![
            "compare_floor:current_l2.theorem_transport_contract_coupled_later_gate_guard_only"
                .to_string()
        ]
    );
    assert_eq!(
        actualization.guard_refs,
        vec!["guard:theorem_transport_contract_coupled_later_gate_not_reached".to_string()]
    );
}

#[test]
fn theorem_transport_contract_coupled_later_gate_reaches_typed_runtime_prototype() {
    let sample_path = typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt");
    let actualization =
        build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_reached(&actualization);
}

#[test]
fn theorem_transport_contract_coupled_later_gate_reaches_order_handoff_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    let actualization =
        build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_reached(&actualization);
}

#[test]
fn theorem_transport_contract_coupled_later_gate_reaches_stale_reconnect_runtime_prototype() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    let actualization =
        build_current_l2_source_sample_theorem_transport_contract_coupled_later_gate(
            sample_path.to_str().unwrap(),
            prototype_host_plan(&sample_path),
        )
        .unwrap();

    assert_coupled_later_gate_reached(&actualization);
}
