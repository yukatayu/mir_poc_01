use mir_runtime::clean_near_end::{
    build_clean_near_end_closeout, build_clean_near_end_matrix, run_clean_near_end_sample,
};

#[test]
fn clean_sample_authorized_declassification_passes() {
    let report = run_clean_near_end_sample("01_authorized_declassification").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("valid"));
    assert_eq!(report.terminal_outcome.as_deref(), Some("success"));
    assert!(
        report
            .constraints_solved
            .contains(&"authority(Alice) >= FingerprintAuthority.Releaser".to_string())
    );
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
    assert!(
        report
            .constraints_failed
            .contains(&"SecurityLabel.KeyMaterial <= SecurityLabel.Public".to_string())
    );
}

#[test]
fn clean_sample_capture_escape_rejected() {
    let report = run_clean_near_end_sample("04_capture_escape_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(report.reason_family.as_deref(), Some("capture_escape"));
    assert!(
        report
            .constraints_failed
            .contains(&"{EphemeralToken} <= {RoomHistory}".to_string())
    );
}

#[test]
fn clean_sample_cost_bound_rejected() {
    let report = run_clean_near_end_sample("05_cost_bound_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(report.reason_family.as_deref(), Some("cost_bound_exceeded"));
    assert!(
        report
            .constraints_failed
            .contains(&"remote_calls 1 <= 0".to_string())
    );
}

#[test]
fn clean_sample_handoff_requires_witness() {
    let report = run_clean_near_end_sample("02_missing_witness_rejected").unwrap();
    assert_eq!(report.static_verdict.as_deref(), Some("malformed"));
    assert_eq!(
        report.reason_family.as_deref(),
        Some("missing_handoff_witness")
    );
    assert!(
        report
            .constraints_failed
            .contains(&"requires witness(draw_pub)".to_string())
    );
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
    assert!(
        report
            .counterexample_shape
            .contains(&"A writes flag[A] but B has not observed it".to_string())
    );
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
    assert!(
        report
            .mode_constraints
            .contains(&"config : stable".to_string())
    );
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
    assert!(
        report
            .term_signatures
            .iter()
            .any(|signature| signature.kind == "effect" && signature.name == "rng")
    );
    assert!(
        report
            .term_signatures
            .iter()
            .any(|signature| signature.kind == "witness" && signature.name == "provider_receipt")
    );
    assert!(
        report
            .term_signatures
            .iter()
            .any(|signature| signature.kind == "relation" && signature.name == "publication_order")
    );
}

#[test]
fn clean_sample_delegated_rng_service_emits_transport_layer_signature() {
    let report = run_clean_near_end_sample("05_delegated_rng_service").unwrap();
    let layer = report
        .layer_signatures
        .iter()
        .find(|layer| layer.name == "transport_provider_boundary")
        .expect("transport layer signature");
    assert!(
        layer
            .requires
            .contains(&"runtime_service:delegated_rng_roll".to_string())
    );
    assert!(
        layer
            .provides
            .contains(&"evidence:provider_receipt".to_string())
    );
    assert!(
        layer
            .checks
            .contains(&"requires witness(provider_receipt)".to_string())
    );
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
    assert!(
        layer
            .provides
            .contains(&"evidence:AuthorityDrawWitness".to_string())
    );
    assert!(
        layer
            .emits
            .contains(&"debug_trace:audit(draw_pub)".to_string())
    );
    assert!(layer.laws.contains(&"evidence_preservation".to_string()));
    assert!(layer.laws.contains(&"no_hidden_authority".to_string()));
}

#[test]
fn clean_sample_delegated_rng_service_emits_message_envelopes() {
    let report = run_clean_near_end_sample("05_delegated_rng_service").unwrap();
    let envelope = report
        .message_envelopes
        .iter()
        .find(|envelope| envelope.envelope_id == "provider_request#1")
        .expect("provider request envelope");
    assert_eq!(envelope.auth_evidence, None);
    assert_eq!(envelope.transport, "provider_boundary");
    assert!(
        envelope
            .capability_requirements
            .contains(&"RequestDelegatedRngRoll".to_string())
    );
    assert!(
        envelope
            .witness_refs
            .contains(&"provider_receipt".to_string())
    );
}

#[test]
fn clean_sample_authority_witness_emits_auth_envelope() {
    let report = run_clean_near_end_sample("06_auditable_authority_witness").unwrap();
    let envelope = report
        .message_envelopes
        .iter()
        .find(|envelope| envelope.envelope_id == "audit_trace_request#1")
        .expect("audit trace envelope");
    assert_eq!(envelope.dispatch_outcome, "accepted");
    assert_eq!(envelope.auth_evidence, None);
    assert_eq!(envelope.principal_claim.principal, "Alice");
    assert!(
        envelope
            .authorization_checks
            .contains(&"requires witness(draw_pub)".to_string())
    );
}

#[test]
fn clean_sample_delegated_rng_service_emits_visualization_view() {
    let report = run_clean_near_end_sample("05_delegated_rng_service").unwrap();
    let view = report
        .visualization_views
        .iter()
        .find(|view| view.view_name == "provider_boundary_redacted_flow")
        .expect("provider boundary visualization view");
    assert!(
        view.layer_signature_refs
            .contains(&"transport_provider_boundary".to_string())
    );
    assert!(
        view.message_envelope_refs
            .contains(&"provider_request#1".to_string())
    );
    assert!(
        view.message_envelope_refs
            .contains(&"provider_receipt#1".to_string())
    );
    assert!(
        view.redaction_rules
            .contains(&"auth_evidence:none_baseline".to_string())
    );
}

#[test]
fn clean_sample_authority_witness_emits_telemetry_row() {
    let report = run_clean_near_end_sample("06_auditable_authority_witness").unwrap();
    let row = report
        .telemetry_rows
        .iter()
        .find(|row| row.row_name == "audit_trace_dispatch")
        .expect("audit trace telemetry row");
    assert_eq!(row.channel, "audit_trace_boundary");
    assert!(
        row.layer_signature_refs
            .contains(&"auth_authority_witness".to_string())
    );
    assert!(
        row.message_envelope_refs
            .contains(&"audit_trace_request#1".to_string())
    );
    assert_eq!(row.measurement, "dispatch_outcome");
    assert_eq!(row.value, "accepted");
}

#[test]
fn clean_model_check_sample_emits_verification_layer_signature() {
    let report = run_clean_near_end_sample("01_peterson_sc_pass").unwrap();
    let layer = report
        .layer_signatures
        .iter()
        .find(|layer| layer.name == "verification_model_check")
        .expect("verification layer signature");
    assert!(
        layer
            .requires
            .contains(&"property:mutual_exclusion".to_string())
    );
    assert!(
        layer
            .checks
            .contains(&"checked_under:sequential_consistency".to_string())
    );
    assert!(
        layer
            .provides
            .contains(&"evidence:model_check_result".to_string())
    );
    assert!(
        layer
            .laws
            .contains(&"residual_obligations_are_explicit".to_string())
    );
}

#[test]
fn clean_near_end_closeout_records_signature_inventory() {
    let closeout = build_clean_near_end_closeout().unwrap();
    assert!(closeout.signature_kinds.contains(&"effect".to_string()));
    assert!(closeout.signature_kinds.contains(&"witness".to_string()));
    assert!(
        closeout
            .reserved_signature_kinds
            .contains(&"adapter".to_string())
    );
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
    assert!(
        closeout
            .layer_signatures
            .iter()
            .any(|layer| layer.name == "transport_provider_boundary")
    );
    assert!(
        closeout
            .layer_signatures
            .iter()
            .any(|layer| layer.name == "auth_authority_witness")
    );
    assert!(
        closeout
            .layer_signatures
            .iter()
            .any(|layer| layer.name == "verification_model_check")
    );
    assert!(
        closeout
            .reserved_layer_signature_names
            .contains(&"visualization_redacted_debug_view".to_string())
    );
    assert!(
        closeout
            .reserved_layer_signature_names
            .contains(&"typed_telemetry_emitter".to_string())
    );
}

#[test]
fn clean_near_end_closeout_records_message_envelope_inventory() {
    let closeout = build_clean_near_end_closeout().unwrap();
    assert!(
        closeout
            .message_envelope_lanes
            .contains(&"auth_evidence".to_string())
    );
    assert!(closeout.auth_evidence_kinds.contains(&"none".to_string()));
    assert!(
        closeout
            .reserved_auth_evidence_kinds
            .contains(&"session_token".to_string())
    );
    assert!(
        closeout
            .transport_seams
            .contains(&"provider_boundary".to_string())
    );
    assert!(
        closeout
            .reserved_transport_seams
            .contains(&"loopback_socket".to_string())
    );
}

#[test]
fn clean_near_end_closeout_records_visualization_and_telemetry_inventory() {
    let closeout = build_clean_near_end_closeout().unwrap();
    assert!(
        closeout
            .visualization_views
            .iter()
            .any(|view| view.view_name == "provider_boundary_redacted_flow")
    );
    assert!(
        closeout
            .visualization_view_lanes
            .contains(&"message_envelope_refs".to_string())
    );
    assert!(
        closeout
            .reserved_visualization_view_names
            .contains(&"cross_place_projection".to_string())
    );
    assert!(
        closeout
            .telemetry_rows
            .iter()
            .any(|row| row.row_name == "audit_trace_dispatch")
    );
    assert!(
        closeout
            .telemetry_row_lanes
            .contains(&"channel".to_string())
    );
    assert!(
        closeout
            .telemetry_channels
            .contains(&"provider_boundary".to_string())
    );
    assert!(
        closeout
            .reserved_telemetry_channels
            .contains(&"typed_effect_adapter".to_string())
    );
}
