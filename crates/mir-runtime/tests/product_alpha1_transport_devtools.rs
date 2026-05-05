use std::path::Path;

use mir_runtime::{
    product_alpha1_devtools::{export_product_alpha1_devtools, render_product_alpha1_viewer_html},
    product_alpha1_session::{
        attach_product_alpha1_package_to_session_path, quiescent_save_product_alpha1_session,
        run_product_alpha1_local_session_path, save_product_alpha1_session,
    },
    product_alpha1_transport::run_product_alpha1_transport_for_session,
};

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-runtime")
}

#[test]
fn product_alpha1_local_transport_preserves_explicit_fabric_lanes() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");

    let (transported, transport_report) =
        run_product_alpha1_transport_for_session(&report.session, "local")
            .expect("local product transport should execute");

    assert_eq!(
        transport_report.surface_kind,
        "product_alpha1_transport_report"
    );
    assert_eq!(transport_report.mode, "local");
    assert_eq!(transport_report.terminal_outcome, "accepted");
    assert!(transport_report.local_transport_executed);
    assert!(transport_report.wire_roundtrip_executed);
    assert_eq!(transport_report.wire_response_status, "accepted");
    assert!(!transport_report.wan_federation_claimed);
    assert!(transport_report.lane_preservation.auth_lane_preserved);
    assert!(transport_report.lane_preservation.membership_lane_preserved);
    assert!(
        transported
            .route_graph
            .routes
            .iter()
            .any(|route| route.transport_lane == "product_local_loopback")
    );
}

#[test]
fn product_alpha1_docker_transport_has_compose_tcp_boundary_without_wan_claim() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");

    let (_transported, transport_report) =
        run_product_alpha1_transport_for_session(&report.session, "docker")
            .expect("docker product transport should produce a bounded report");

    assert_eq!(transport_report.mode, "docker");
    assert_eq!(transport_report.transport_medium, "docker_compose_tcp");
    assert!(transport_report.docker_compose_tcp_claimed);
    assert!(!transport_report.wan_federation_claimed);
    assert!(
        transport_report
            .non_claims
            .iter()
            .any(|claim| claim.contains("WAN"))
    );
}

#[test]
fn product_alpha1_devtools_bundle_has_required_panels_and_redaction() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let (attached, _) = attach_product_alpha1_package_to_session_path(
        &report.session,
        repo_root().join("samples/product-alpha1/demo/packages/debug-layer"),
    )
    .expect("debug layer should attach");
    let (saved, _) =
        save_product_alpha1_session(&attached, "savepoint#viewer-r0").expect("save should work");
    let (quiescent, _) = quiescent_save_product_alpha1_session(&saved, "savepoint#viewer-r2")
        .expect("quiescent save should work");
    let (transported, _) = run_product_alpha1_transport_for_session(&quiescent, "local")
        .expect("local transport should execute");

    let bundle = export_product_alpha1_devtools(&transported).expect("devtools should export");
    let html = render_product_alpha1_viewer_html(&bundle);

    let expected_panels = [
        "product_overview",
        "place_graph",
        "event_dag",
        "message_route_graph",
        "membership_frontier_timeline",
        "witness_relation_timeline",
        "hotplug_lifecycle",
        "save_load_quiescent_timeline",
        "message_failure_recovery",
        "fallback_degradation",
        "auth_capability_decision",
        "redaction_toggle",
        "retention_trace",
    ];
    for panel in expected_panels {
        assert!(
            bundle.panel_ids.contains(&panel.to_string()),
            "missing panel {panel}"
        );
        assert!(html.contains(panel), "viewer html should contain {panel}");
    }
    let html_lower = html.to_ascii_lowercase();
    assert!(!html_lower.contains("raw_witness_payload"));
    assert!(!html_lower.contains("raw_auth_evidence"));
    let bundle_json = serde_json::to_string(&bundle).expect("bundle should serialize");
    assert!(!bundle_json.contains("witness_refs"));
    assert!(!bundle_json.contains("granted_capabilities"));
    assert!(!bundle_json.contains("ObserveDebugSummary"));
    assert!(!bundle_json.contains("AttachDebugLayer"));
    assert_eq!(
        bundle
            .panels
            .witness_relation_timeline
            .hidden_private_witness_count,
        1
    );
    assert_eq!(
        bundle.panels.auth_capability_decision.capability_decisions[0].granted_capability_count,
        2
    );
    assert_eq!(bundle.admin_debug_view_status, "kept_later");
    assert!(!bundle.final_public_viewer_frozen);
}
