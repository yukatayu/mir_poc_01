use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::product_alpha1_session::{
    ProductAlpha1SessionErrorKind, attach_product_alpha1_package_to_session_path,
    run_product_alpha1_local_session_path,
};

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-runtime")
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn write_package(dir_name: &str, package_json: &str) -> PathBuf {
    let dir = unique_temp_dir(dir_name);
    fs::create_dir_all(&dir).expect("temp package dir should be created");
    fs::write(dir.join("package.mir.json"), package_json).expect("package should be written");
    dir
}

fn layer_package_json(
    package_id: &str,
    capabilities: &[&str],
    membership: &[&str],
    witnesses: &[&str],
    view_role: &str,
    redaction_level: &str,
    retention_scope: &str,
) -> String {
    format!(
        r#"{{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "{package_id}",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "layer",
  "dependencies": [],
  "effects": ["observe.event_dag"],
  "failures": [],
  "capabilities": {capabilities},
  "witness_requirements": {witnesses},
  "membership_requirements": {membership},
  "auth_policy": {{
    "policy_id": "temp-layer-auth-policy",
    "required_bindings": ["admin_membership"]
  }},
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {{
      "contract_id": "temp-layer-contract",
      "variance": "invariant",
      "effect_row": ["observe.event_dag"],
      "failure_row": []
    }}
  ],
  "observation_policy": {{
    "view_role": "{view_role}",
    "labels": ["debug_summary_redacted"]
  }},
  "redaction_policy": {{
    "level": "{redaction_level}",
    "redacted_fields": ["raw_witness_payload", "raw_auth_evidence"]
  }},
  "retention_policy": {{
    "scope": "{retention_scope}",
    "retained_artifacts": ["hotplug_lifecycle"]
  }},
  "message_recovery_policy": {{
    "handled_failures": ["reject"],
    "recovery": "reject"
  }},
  "savepoint_policy": {{
    "classes": ["R0"],
    "quiescent_required": false
  }},
  "native_policy": {{
    "execution_policy": "disabled",
    "provenance_required": true
  }},
  "compatibility": {{
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }}
}}"#,
        capabilities = json_array(capabilities),
        membership = json_array(membership),
        witnesses = json_array(witnesses),
    )
}

fn json_array(values: &[&str]) -> String {
    let items = values
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{items}]")
}

#[test]
fn product_alpha1_run_local_builds_same_session_carrier_with_required_lanes() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");

    assert_eq!(report.surface_kind, "product_alpha1_run_local_report");
    assert_eq!(report.session.session_id, "session#product-alpha1-demo");
    assert_eq!(report.session.phase, "run_local");
    assert!(report.runtime_plan_emitted);
    assert!(!report.local_transport_claimed);
    assert!(report.typed_host_io_claimed);
    assert!(!report.product_alpha1_ready);
    assert!(!report.final_public_api_frozen);
    assert!(!report.session.event_dag.nodes.is_empty());
    assert!(!report.session.route_graph.routes.is_empty());
    assert!(!report.session.membership.active_members.is_empty());
    assert!(!report.session.witness_state.witness_refs.is_empty());
    assert!(report.session.hotplug_lifecycle.is_empty());
    assert_eq!(report.session.save_load_state.ordinary_save_ready, false);
    assert_eq!(report.session.save_load_state.quiescent_save_ready, false);
    assert_eq!(
        report.session.message_recovery_state.recovery_policy,
        "retry_then_reject"
    );
}

#[test]
fn product_alpha1_run_local_executes_declared_host_io_payload() {
    let package_json =
        fs::read_to_string(repo_root().join("samples/product-alpha1/demo/package.mir.json"))
            .expect("product demo package should be readable")
            .replace(r#""value": 41"#, r#""value": 7"#)
            .replace(r#""value": 42"#, r#""value": 8"#);
    let package_dir = write_package("product-alpha1-runtime-input-test", &package_json);
    fs::create_dir_all(package_dir.join("packages/debug-layer"))
        .expect("dependency dir should be created");
    fs::write(
        package_dir.join("packages/debug-layer/package.mir.json"),
        fs::read_to_string(
            repo_root().join("samples/product-alpha1/demo/packages/debug-layer/package.mir.json"),
        )
        .expect("debug layer package should be readable"),
    )
    .expect("dependency package should be written");

    let report = run_product_alpha1_local_session_path(&package_dir)
        .expect("runtime input package should run locally");

    assert_eq!(report.session.host_io_history[0].request_summary, "Int(7)");
    assert_eq!(report.session.host_io_history[0].response_summary, "Int(8)");
}

#[test]
fn product_alpha1_run_local_rejects_non_world_package() {
    let error = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/demo/packages/debug-layer"),
    )
    .expect_err("run-local should only admit product world packages");

    assert_eq!(
        error.kind,
        ProductAlpha1SessionErrorKind::UnsupportedPackage
    );
    assert!(
        error
            .detail
            .contains("run-local requires a product alpha-1 world package")
    );
}

#[test]
fn product_alpha1_attach_mutates_same_session_hotplug_state() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let session_before = report.session;
    let event_count_before = session_before.event_dag.nodes.len();

    let (session_after, attach_report) = attach_product_alpha1_package_to_session_path(
        &session_before,
        repo_root().join("samples/product-alpha1/demo/packages/debug-layer"),
    )
    .expect("debug layer should attach to product alpha session");

    assert_eq!(attach_report.surface_kind, "product_alpha1_attach_report");
    assert_eq!(attach_report.session_id, session_before.session_id);
    assert_eq!(attach_report.package_id, "product-alpha1-debug-layer");
    assert_eq!(attach_report.terminal_outcome, "accepted");
    assert_eq!(
        attach_report.auth_decision.overlay_transparency_claimed,
        false
    );
    assert!(attach_report.session_mutated);
    assert_eq!(session_after.phase, "attached");
    assert_eq!(
        session_after.active_layers,
        vec!["product-alpha1-debug-layer"]
    );
    assert_eq!(session_after.hotplug_lifecycle.len(), 1);
    assert!(session_after.event_dag.nodes.len() > event_count_before);
    assert!(
        session_after
            .event_dag
            .nodes
            .iter()
            .any(|node| node.event_kind == "hotplug_verdict")
    );
    assert!(
        session_after
            .event_dag
            .nodes
            .iter()
            .any(|node| node.event_kind == "activation_cut")
    );
    assert!(
        session_after
            .route_graph
            .routes
            .iter()
            .any(|route| route.envelope_id == "envelope#attach#product-alpha1-debug-layer")
    );
    assert_eq!(session_after.auth_decisions.len(), 2);
    assert_eq!(session_after.save_load_state.quiescent_save_ready, false);
    assert!(!session_after.product_alpha1_ready);
}

#[test]
fn product_alpha1_attach_rejects_missing_membership_capability_or_witness() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let bad_layer = write_package(
        "product-alpha1-bad-layer",
        &layer_package_json(
            "product-alpha1-bad-layer",
            &["ImpossibleCapability"],
            &["nonexistent_admin"],
            &["missing_witness"],
            "observer_safe",
            "observer_safe",
            "demo_session",
        ),
    );

    let (session_after, attach_report) =
        attach_product_alpha1_package_to_session_path(&report.session, &bad_layer)
            .expect("rejected attach should still return an observable report");

    assert_eq!(attach_report.terminal_outcome, "rejected");
    assert!(!attach_report.active_runtime_mutated);
    assert!(attach_report.session_mutated);
    assert!(session_after.active_layers.is_empty());
    assert_eq!(
        session_after.hotplug_lifecycle[0].terminal_outcome,
        "rejected"
    );
    assert!(
        attach_report
            .auth_decision
            .notes
            .iter()
            .any(|note| note.contains("missing_membership"))
    );
}

#[test]
fn product_alpha1_attach_cannot_weaken_observer_safe_policy() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let broad_layer = write_package(
        "product-alpha1-broad-layer",
        &layer_package_json(
            "product-alpha1-broad-layer",
            &["ObserveDebugSummary"],
            &["active_admin_participant"],
            &[],
            "admin_debug",
            "admin_full",
            "durable_audit",
        ),
    );

    let (session_after, attach_report) =
        attach_product_alpha1_package_to_session_path(&report.session, &broad_layer)
            .expect("valid broad-view layer should attach without weakening observer policy");

    assert_eq!(attach_report.terminal_outcome, "accepted");
    assert_eq!(
        session_after.observer_safe_export.view_role,
        "observer_safe"
    );
    assert_eq!(
        session_after.observer_safe_export.redaction_level,
        "observer_safe"
    );
    assert_eq!(
        session_after.observer_safe_export.retention_scope,
        "demo_session"
    );
}
