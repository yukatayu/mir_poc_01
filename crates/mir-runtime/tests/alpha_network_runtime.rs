use mir_runtime::alpha_network_runtime::{
    build_auth_evidence_lane_preserved_report, build_closeout_reports,
    build_docker_two_process_envelope_report, build_missing_capability_rejection_report,
    build_missing_witness_rejection_report, build_observer_safe_route_trace_report,
    build_stale_membership_rejection_report,
};

#[test]
fn net_02_accepts_two_process_exchange_with_explicit_lanes() {
    let report = build_docker_two_process_envelope_report().expect("NET-02 report");

    assert_eq!(report.sample_id, "NET-02");
    assert_eq!(report.terminal_outcome, "accepted");
    assert_eq!(report.transport_surface, "tcp_process_boundary");
    assert_eq!(report.transport_medium, "loopback_tcp");
    assert_eq!(report.bridge_kind, "tcp_json_socket");
    assert_eq!(report.bridge_processes.len(), 2);
    assert!(
        report.admission_checks.iter().all(|check| check.passed),
        "all admission checks should pass for NET-02"
    );
    assert_eq!(report.observer_route_trace.len(), 2);
    assert_eq!(report.observer_route_trace[0].witness_ref_count, 1);
    assert!(
        report
            .what_it_proves
            .iter()
            .any(|line| line.contains("real TCP process boundary"))
    );
}

#[test]
fn net_03_rejects_stale_membership_without_hidden_refresh() {
    let report = build_stale_membership_rejection_report().expect("NET-03 report");

    assert_eq!(report.sample_id, "NET-03");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(
        report.reason_family.as_deref(),
        Some("membership_freshness")
    );
    assert!(
        report
            .rejection_reason_refs
            .contains(&"membership_epoch_drift".to_string())
    );
    assert!(
        report
            .rejection_reason_refs
            .contains(&"member_incarnation_drift".to_string())
    );
    assert_eq!(report.world_membership_epoch, 1);
    assert_eq!(report.world_active_participants, vec!["Bob".to_string()]);
}

#[test]
fn net_04_rejects_missing_capability_explicitly() {
    let report = build_missing_capability_rejection_report().expect("NET-04 report");

    assert_eq!(report.sample_id, "NET-04");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("capability"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"missing_capability:HandoffTurn".to_string())
    );
    assert!(
        report
            .admission_checks
            .iter()
            .any(|check| check.check_name == "capability_sufficient" && !check.passed)
    );
}

#[test]
fn net_05_rejects_missing_required_witness_explicitly() {
    let report = build_missing_witness_rejection_report().expect("NET-05 report");

    assert_eq!(report.sample_id, "NET-05");
    assert_eq!(report.terminal_outcome, "rejected");
    assert_eq!(report.reason_family.as_deref(), Some("witness"));
    assert!(
        report
            .rejection_reason_refs
            .contains(&"missing_witness:draw_pub#1".to_string())
    );
    assert!(
        report
            .admission_checks
            .iter()
            .any(|check| check.check_name == "required_witness_present" && !check.passed)
    );
}

#[test]
fn net_07_route_trace_stays_observer_safe_and_redacted() {
    let report = build_observer_safe_route_trace_report().expect("NET-07 report");

    assert_eq!(report.sample_id, "NET-07");
    assert_eq!(report.terminal_outcome, "accepted");
    assert_eq!(report.observer_route_trace.len(), 2);
    for row in &report.observer_route_trace {
        assert_eq!(row.authority, "ObserveRouteTrace(NetworkTransportLane)");
        assert_eq!(row.retention_scope, "helper_local_ephemeral");
        assert_eq!(row.redaction, "observer_safe_route_trace");
    }
}

#[test]
fn net_09_keeps_auth_evidence_in_a_separate_lane() {
    let report = build_auth_evidence_lane_preserved_report().expect("NET-09 report");

    assert_eq!(report.sample_id, "NET-09");
    assert_eq!(report.terminal_outcome, "accepted");
    let auth_lane = report.auth_lane.expect("NET-09 auth lane");
    assert!(auth_lane.auth_present);
    assert_eq!(auth_lane.subject, "Alice");
    assert_eq!(auth_lane.issuer, "LocalAuthService");
    assert!(auth_lane.preserved_separately);
    assert!(
        auth_lane
            .bindings
            .contains(&"transport=network_transport_lane".to_string())
    );
}

#[test]
fn closeout_reports_cover_all_implemented_network_rows() {
    let reports = build_closeout_reports().expect("network closeout reports");
    let sample_ids: Vec<_> = reports
        .iter()
        .map(|report| report.sample_id.as_str())
        .collect();

    assert_eq!(
        sample_ids,
        vec!["NET-02", "NET-03", "NET-04", "NET-05", "NET-07", "NET-09"]
    );
    for report in &reports {
        assert!(
            report
                .retained_later_refs
                .contains(&"network_partition_explicit_failure".to_string())
        );
        assert!(
            report
                .retained_later_refs
                .contains(&"final_public_transport_abi".to_string())
        );
    }
}
