use mir_runtime::clean_near_end::{
    build_clean_near_end_closeout, build_clean_near_end_matrix, run_clean_near_end_sample,
};

#[test]
fn clean_sample_authorized_declassification_passes() {
    let report = run_clean_near_end_sample("01_authorized_declassification").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("valid"));
    assert_eq!(report.terminal_outcome.as_deref(), Some("success"));
    assert!(report
        .constraints_solved
        .contains(&"authority(Alice) >= FingerprintAuthority.Releaser".to_string()));
}

#[test]
fn clean_sample_unauthorized_declassification_rejected() {
    let report = run_clean_near_end_sample("02_unauthorized_declassification_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(
        report.reason_family.as_deref(),
        Some("authority_preorder_constraint_failed")
    );
    assert!(!report.entered_evaluation);
}

#[test]
fn clean_sample_label_flow_rejected() {
    let report = run_clean_near_end_sample("03_label_flow_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(
        report.reason_family.as_deref(),
        Some("label_flow_constraint_failed")
    );
    assert!(report
        .constraints_failed
        .contains(&"SecurityLabel.KeyMaterial <= SecurityLabel.Public".to_string()));
}

#[test]
fn clean_sample_capture_escape_rejected() {
    let report = run_clean_near_end_sample("04_capture_escape_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(report.reason_family.as_deref(), Some("capture_escape"));
    assert!(report
        .constraints_failed
        .contains(&"{EphemeralToken} <= {RoomHistory}".to_string()));
}

#[test]
fn clean_sample_cost_bound_rejected() {
    let report = run_clean_near_end_sample("05_cost_bound_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(report.reason_family.as_deref(), Some("cost_bound_exceeded"));
    assert!(report
        .constraints_failed
        .contains(&"remote_calls 1 <= 0".to_string()));
}

#[test]
fn clean_sample_handoff_requires_witness() {
    let report = run_clean_near_end_sample("02_missing_witness_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(
        report.reason_family.as_deref(),
        Some("missing_handoff_witness")
    );
    assert!(report
        .constraints_failed
        .contains(&"requires witness(draw_pub)".to_string()));
}

#[test]
fn clean_sample_handoff_before_publication_rejected() {
    let report = run_clean_near_end_sample("03_handoff_before_publication_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(
        report.reason_family.as_deref(),
        Some("handoff_before_publication")
    );
}

#[test]
fn clean_model_peterson_sc_passes() {
    let report = run_clean_near_end_sample("01_peterson_sc_pass").unwrap();
    assert_eq!(report.model_check_result.as_deref(), Some("pass"));
    assert_eq!(
        report.checked_under.as_deref(),
        Some("sequential_consistency")
    );
}

#[test]
fn clean_model_peterson_relaxed_counterexample() {
    let report = run_clean_near_end_sample("02_peterson_relaxed_counterexample").unwrap();
    assert_eq!(report.model_check_result.as_deref(), Some("counterexample"));
    assert!(report
        .counterexample_shape
        .contains(&"A writes flag[A] but B has not observed it".to_string()));
}

#[test]
fn clean_model_broken_mutex_counterexample() {
    let report = run_clean_near_end_sample("03_broken_mutex_counterexample").unwrap();
    assert_eq!(report.model_check_result.as_deref(), Some("counterexample"));
    assert_eq!(
        report.explanation.as_deref(),
        Some("interleaving or visibility permits both actors to enter critical section")
    );
}

#[test]
fn clean_modal_stage_stable_later_minimal_is_valid() {
    let report = run_clean_near_end_sample("01_stage_stable_later_minimal").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("valid"));
    assert!(report
        .mode_constraints
        .contains(&"config : stable".to_string()));
}

#[test]
fn clean_near_end_matrix_counts_all_families() {
    let matrix = build_clean_near_end_matrix().unwrap();
    assert_eq!(matrix.total_samples, 16);
    assert_eq!(matrix.families["typing"], 5);
    assert_eq!(matrix.families["order-handoff"], 6);
    assert_eq!(matrix.families["model-check"], 3);
    assert_eq!(matrix.families["modal"], 2);
}

#[test]
fn clean_sample_delegated_rng_service_emits_term_signatures() {
    let report = run_clean_near_end_sample("05_delegated_rng_service").unwrap();
    assert!(report
        .term_signatures
        .iter()
        .any(|signature| signature.kind == "effect" && signature.name == "rng"));
    assert!(report
        .term_signatures
        .iter()
        .any(|signature| signature.kind == "witness" && signature.name == "provider_receipt"));
    assert!(report
        .term_signatures
        .iter()
        .any(|signature| signature.kind == "relation" && signature.name == "publication_order"));
}

#[test]
fn clean_sample_delegated_rng_service_emits_transport_layer_signature() {
    let report = run_clean_near_end_sample("05_delegated_rng_service").unwrap();
    let layer = report
        .layer_signatures
        .iter()
        .find(|layer| layer.name == "transport_provider_boundary")
        .expect("transport layer signature");
    assert!(layer
        .requires
        .contains(&"runtime_service:delegated_rng_roll".to_string()));
    assert!(layer
        .provides
        .contains(&"evidence:provider_receipt".to_string()));
    assert!(layer
        .checks
        .contains(&"requires witness(provider_receipt)".to_string()));
    assert!(layer.laws.contains(&"no_hidden_effect".to_string()));
}

#[test]
fn clean_sample_authority_witness_emits_auth_layer_signature() {
    let report = run_clean_near_end_sample("06_auditable_authority_witness").unwrap();
    let layer = report
        .layer_signatures
        .iter()
        .find(|layer| layer.name == "auth_authority_witness")
        .expect("auth layer signature");
    assert!(layer
        .provides
        .contains(&"evidence:AuthorityDrawWitness".to_string()));
    assert!(layer
        .emits
        .contains(&"debug_trace:audit(draw_pub)".to_string()));
    assert!(layer.laws.contains(&"evidence_preservation".to_string()));
    assert!(layer.laws.contains(&"no_hidden_authority".to_string()));
}

#[test]
fn clean_model_check_sample_emits_verification_layer_signature() {
    let report = run_clean_near_end_sample("01_peterson_sc_pass").unwrap();
    let layer = report
        .layer_signatures
        .iter()
        .find(|layer| layer.name == "verification_model_check")
        .expect("verification layer signature");
    assert!(layer
        .requires
        .contains(&"property:mutual_exclusion".to_string()));
    assert!(layer
        .checks
        .contains(&"checked_under:sequential_consistency".to_string()));
    assert!(layer
        .provides
        .contains(&"evidence:model_check_result".to_string()));
    assert!(layer
        .laws
        .contains(&"residual_obligations_are_explicit".to_string()));
}

#[test]
fn clean_near_end_closeout_records_signature_inventory() {
    let closeout = build_clean_near_end_closeout().unwrap();
    assert!(closeout.signature_kinds.contains(&"effect".to_string()));
    assert!(closeout.signature_kinds.contains(&"witness".to_string()));
    assert!(closeout
        .reserved_signature_kinds
        .contains(&"adapter".to_string()));
}

#[test]
fn clean_near_end_closeout_records_layer_signature_inventory() {
    let closeout = build_clean_near_end_closeout().unwrap();
    assert_eq!(
        closeout.layer_signature_lanes,
        vec![
            "requires".to_string(),
            "provides".to_string(),
            "transforms".to_string(),
            "checks".to_string(),
            "emits".to_string(),
            "laws".to_string(),
        ]
    );
    assert!(closeout
        .layer_signatures
        .iter()
        .any(|layer| layer.name == "transport_provider_boundary"));
    assert!(closeout
        .layer_signatures
        .iter()
        .any(|layer| layer.name == "auth_authority_witness"));
    assert!(closeout
        .layer_signatures
        .iter()
        .any(|layer| layer.name == "verification_model_check"));
    assert!(closeout
        .reserved_layer_signature_names
        .contains(&"visualization_redacted_debug_view".to_string()));
    assert!(closeout
        .reserved_layer_signature_names
        .contains(&"typed_telemetry_emitter".to_string()));
}
