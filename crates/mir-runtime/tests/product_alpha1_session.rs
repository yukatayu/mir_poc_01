use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_runtime::product_alpha1_session::{
    ProductAlpha1SessionErrorKind, attach_product_alpha1_package_to_session_path,
    load_product_alpha1_session, quiescent_save_product_alpha1_session,
    run_product_alpha1_local_session_path, save_product_alpha1_session,
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

fn computational_package_json(
    package_id: &str,
    module_id: &str,
    function_id: &str,
    request_value: i64,
    expected_output: i64,
) -> String {
    format!(
        r#"{{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "{package_id}",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": ["typed_host_io.read_int", "typed_host_io.write_int"],
  "failures": ["AdapterUnavailable", "TypeMismatch"],
  "capabilities": ["RunComputationalRow"],
  "witness_requirements": [],
  "membership_requirements": ["active_participant"],
  "auth_policy": {{
    "policy_id": "{package_id}-auth-policy",
    "required_bindings": ["participant_membership"]
  }},
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {{
      "contract_id": "{package_id}-contract",
      "variance": "invariant",
      "effect_row": ["typed_host_io.read_int", "typed_host_io.write_int"],
      "failure_row": ["AdapterUnavailable", "TypeMismatch"]
    }}
  ],
  "observation_policy": {{
    "view_role": "observer_safe",
    "labels": ["observer_safe_compute_summary"]
  }},
  "redaction_policy": {{
    "level": "observer_safe",
    "redacted_fields": ["raw_auth_evidence"]
  }},
  "retention_policy": {{
    "scope": "computational_session",
    "retained_artifacts": ["checker_report", "runtime_plan", "compute_trace"]
  }},
  "message_recovery_policy": {{
    "handled_failures": ["reject"],
    "recovery": "reject"
  }},
  "savepoint_policy": {{
    "classes": ["R0", "R2"],
    "quiescent_required": true
  }},
  "runtime_input": {{
    "entry_place": "Place[ComputationalHostPlace]",
    "host_input": {{
      "adapter_kind": "ReadInt",
      "effect_ref": "typed_host_io.read_int",
      "request_payload": {{"kind": "int", "value": {request_value}}},
      "expected_response": {{"kind": "int", "value": {request_value}}}
    }},
    "mir_compute": {{
      "module_id": "{module_id}",
      "function_id": "{function_id}",
      "input_type": "Int64",
      "output_type": "Int64",
      "expected_output": {{"kind": "int", "value": {expected_output}}}
    }},
    "host_output": {{
      "adapter_kind": "WriteInt",
      "effect_ref": "typed_host_io.write_int",
      "request_payload": {{"kind": "int", "value": {expected_output}}},
      "expected_response": {{"kind": "int", "value": {expected_output}}}
    }}
  }},
  "native_policy": {{
    "execution_policy": "disabled",
    "provenance_required": true
  }},
  "compatibility": {{
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }}
}}"#
    )
}

fn assert_event_ids_unique(
    session: &mir_runtime::product_alpha1_session::ProductAlpha1SessionCarrier,
) {
    let mut seen = std::collections::BTreeSet::new();
    for node in &session.event_dag.nodes {
        assert!(
            seen.insert(node.event_id.clone()),
            "duplicate event id {}",
            node.event_id
        );
    }
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
    assert!(!report.mir_computation_claimed);
    assert!(!report.product_alpha1_ready);
    assert!(!report.final_public_api_frozen);
    assert!(report.session.mir_compute_history.is_empty());
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
fn product_alpha1_run_local_accepts_operational_sugoroku_root() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/sugoroku-world"),
    )
    .expect("operational sugoroku root should run locally");

    assert_eq!(report.package_id, "operational-sugoroku");
    assert_eq!(report.session.session_id, "session#operational-sugoroku");
    assert_eq!(report.runtime_plan.package_kind, "sugoroku_world");
    assert_eq!(report.runtime_plan.entry_place, "Place[SugorokuGamePlace]");
    assert!(
        report
            .runtime_plan
            .declared_dependencies
            .iter()
            .any(|dependency| dependency == "../membership-chat")
    );
    let projection_inventory = report
        .check_report
        .projection_inventory
        .as_ref()
        .expect("operational sugoroku check should include projection inventory");
    assert_eq!(projection_inventory.target_count, 2);
    assert_eq!(projection_inventory.packet_boundary_count, 2);
    assert_eq!(projection_inventory.ffi_boundary_count, 1);
    assert_eq!(
        report
            .runtime_plan
            .projection_inventory
            .as_ref()
            .expect("runtime plan should keep projection inventory")
            .target_count,
        2
    );
    assert!(report.typed_host_io_claimed);
    assert!(!report.mir_computation_claimed);
    assert!(report.session.mir_compute_history.is_empty());
    assert!(
        report
            .session
            .witness_state
            .witness_refs
            .iter()
            .any(|witness| witness == "draw_pub")
    );
    let event_kinds = report
        .session
        .event_dag
        .nodes
        .iter()
        .map(|node| node.event_kind.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "sugoroku_roll_requested",
        "sugoroku_roll_published",
        "sugoroku_witness_emitted",
        "sugoroku_turn_handoff",
        "sugoroku_stale_membership_rejected",
    ] {
        assert!(
            event_kinds.contains(required),
            "missing sugoroku event kind {required}"
        );
    }
    let route_lanes = report
        .session
        .route_graph
        .routes
        .iter()
        .map(|route| route.transport_lane.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "same_session_sugoroku_roll",
        "same_session_sugoroku_handoff",
        "same_session_sugoroku_membership_reject",
    ] {
        assert!(
            route_lanes.contains(required),
            "missing sugoroku route lane {required}"
        );
    }
    assert!(
        report
            .session
            .message_recovery_state
            .message_state_lane
            .iter()
            .any(|record| {
                record.failure_class.as_deref() == Some("StaleMembership")
                    && record.state == "Rejected"
            })
    );
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_run_local_accepts_operational_portal_worldlink_root() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/portal-worldlink"),
    )
    .expect("operational portal-worldlink root should run locally");

    assert_eq!(report.package_id, "operational-portal-worldlink");
    assert_eq!(
        report.session.session_id,
        "session#operational-portal-worldlink"
    );
    assert_eq!(report.runtime_plan.package_kind, "portal_worldlink");
    assert_eq!(
        report.runtime_plan.entry_place,
        "Place[PortalBoundaryPlace]"
    );
    assert!(
        report
            .runtime_plan
            .declared_dependencies
            .iter()
            .any(|dependency| dependency == "../sugoroku-world")
    );
    assert!(!report.typed_host_io_claimed);
    let event_kinds = report
        .session
        .event_dag
        .nodes
        .iter()
        .map(|node| node.event_kind.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "portal_resolve_requested",
        "portal_handoff_offered",
        "portal_handoff_witness_emitted",
        "portal_admission_requested",
        "portal_admission_accepted",
    ] {
        assert!(
            event_kinds.contains(required),
            "missing portal event kind {required}"
        );
    }
    let route_lanes = report
        .session
        .route_graph
        .routes
        .iter()
        .map(|route| route.transport_lane.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "same_session_portal_resolve",
        "same_session_portal_handoff",
        "same_session_portal_admit",
    ] {
        assert!(
            route_lanes.contains(required),
            "missing portal route lane {required}"
        );
    }
}

#[test]
fn product_alpha1_run_local_accepts_operational_two_shard_hard_boundary_root() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/two-shard-hard-boundary"),
    )
    .expect("operational two-shard hard-boundary root should run locally");

    assert_eq!(report.package_id, "operational-two-shard-hard-boundary");
    assert_eq!(
        report.session.session_id,
        "session#operational-two-shard-hard-boundary"
    );
    assert_eq!(report.runtime_plan.package_kind, "two_shard_hard_boundary");
    assert_eq!(
        report.runtime_plan.entry_place,
        "Place[ShardAuthorityBoundaryPlace]"
    );
    assert!(
        report
            .runtime_plan
            .declared_dependencies
            .iter()
            .any(|dependency| dependency == "../portal-worldlink")
    );
    assert!(!report.typed_host_io_claimed);
    let event_kinds = report
        .session
        .event_dag
        .nodes
        .iter()
        .map(|node| node.event_kind.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "shard_handoff_offer_published",
        "shard_handoff_prepare_accepted",
        "shard_handoff_commit_applied",
        "shard_old_owner_write_rejected",
        "shard_missing_handoff_witness_rejected",
        "shard_stale_config_rejected",
    ] {
        assert!(
            event_kinds.contains(required),
            "missing shard event kind {required}"
        );
    }
    let route_lanes = report
        .session
        .route_graph
        .routes
        .iter()
        .map(|route| route.transport_lane.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "same_session_shard_handoff_offer",
        "same_session_shard_handoff_commit",
        "same_session_shard_old_owner_reject",
        "same_session_shard_missing_witness_reject",
        "same_session_shard_stale_config_reject",
    ] {
        assert!(
            route_lanes.contains(required),
            "missing shard route lane {required}"
        );
    }
    for required in [
        "OldOwnerWriteRejected",
        "MissingHandoffWitness",
        "StaleShardConfig",
    ] {
        assert!(
            report
                .session
                .message_recovery_state
                .message_state_lane
                .iter()
                .any(|record| record.failure_class.as_deref() == Some(required)
                    && record.state == "Rejected"),
            "missing shard rejection {required}"
        );
    }
}

#[test]
fn product_alpha1_run_local_accepts_operational_membership_chat_root() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/membership-chat"),
    )
    .expect("operational membership chat root should run locally");

    assert_eq!(report.package_id, "operational-membership-chat");
    assert_eq!(
        report.session.session_id,
        "session#operational-membership-chat"
    );
    assert_eq!(report.runtime_plan.package_kind, "membership_chat");
    assert_eq!(report.runtime_plan.entry_place, "Place[ChatPlace]");
    assert!(
        report
            .runtime_plan
            .declared_dependencies
            .iter()
            .any(|dependency| dependency == "../world-core")
    );
    assert!(report.typed_host_io_claimed);
    assert_eq!(report.session.host_io_history.len(), 1);
    assert_eq!(report.session.host_io_history[0].adapter_kind, "ChatText");
    assert_eq!(
        report.session.host_io_history[0].request_summary,
        "Text(\"hello room\")"
    );
    assert_eq!(
        report.session.host_io_history[0].response_summary,
        "Text(\"room#lobby message accepted: hello room\")"
    );
    assert_eq!(
        report.session.observer_safe_export.visible_host_io_events,
        vec!["ChatText:Text(\"hello room\")->Text(\"room#lobby message accepted: hello room\")"]
    );
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_run_local_accepts_operational_two_shard_gradient_observation_root() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/two-shard-gradient-observation"),
    )
    .expect("operational two-shard gradient observation root should run locally");

    assert_eq!(
        report.package_id,
        "operational-two-shard-gradient-observation"
    );
    assert_eq!(
        report.session.session_id,
        "session#operational-two-shard-gradient-observation"
    );
    assert_eq!(
        report.runtime_plan.package_kind,
        "two_shard_gradient_observation"
    );
    assert_eq!(
        report.runtime_plan.entry_place,
        "Place[GradientObservationBoundaryPlace]"
    );
    assert!(
        report
            .runtime_plan
            .declared_dependencies
            .iter()
            .any(|dependency| dependency == "../two-shard-hard-boundary")
    );
    assert!(!report.typed_host_io_claimed);
    let event_kinds = report
        .session
        .event_dag
        .nodes
        .iter()
        .map(|node| node.event_kind.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "gradient_observer_view_emitted",
        "gradient_handoff_hint_projected",
        "gradient_write_capability_rejected",
        "gradient_stale_view_dropped",
        "gradient_missing_freshness_rejected",
    ] {
        assert!(
            event_kinds.contains(required),
            "missing gradient event kind {required}"
        );
    }
    let route_lanes = report
        .session
        .route_graph
        .routes
        .iter()
        .map(|route| route.transport_lane.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "same_session_gradient_observe",
        "same_session_gradient_projection",
        "same_session_gradient_write_reject",
        "same_session_gradient_stale_drop",
        "same_session_gradient_missing_freshness_reject",
    ] {
        assert!(
            route_lanes.contains(required),
            "missing gradient route lane {required}"
        );
    }
    for failure_class in [
        "GradientWriteRejected",
        "StaleGradientViewDropped",
        "MissingFreshnessFieldRejected",
    ] {
        assert!(
            report
                .session
                .message_recovery_state
                .message_state_lane
                .iter()
                .any(|record| {
                    record.failure_class.as_deref() == Some(failure_class)
                        && record.state == "Rejected"
                }),
            "missing gradient rejection row {failure_class}"
        );
    }
}

#[test]
fn product_alpha1_run_local_accepts_operational_world_core_starter_template() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/templates/world-core-starter"),
    )
    .expect("operational world-core starter template should run locally");

    assert_eq!(report.package_id, "operational-world-core-starter");
    assert_eq!(
        report.session.session_id,
        "session#operational-world-core-starter"
    );
    assert_eq!(report.runtime_plan.package_kind, "world_core");
    assert_eq!(report.runtime_plan.entry_place, "Place[WorldServerPlace]");
    assert!(report.runtime_plan.declared_dependencies.is_empty());
    assert!(!report.typed_host_io_claimed);
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_run_local_accepts_operational_membership_chat_starter_template() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/templates/membership-chat-starter"),
    )
    .expect("operational membership-chat starter template should run locally");

    assert_eq!(report.package_id, "operational-membership-chat-starter");
    assert_eq!(
        report.session.session_id,
        "session#operational-membership-chat-starter"
    );
    assert_eq!(report.runtime_plan.package_kind, "membership_chat");
    assert_eq!(report.runtime_plan.entry_place, "Place[ChatPlace]");
    assert_eq!(
        report.runtime_plan.declared_dependencies,
        vec!["../world-core-starter".to_owned()]
    );
    assert!(report.typed_host_io_claimed);
    assert_eq!(report.session.host_io_history[0].adapter_kind, "ChatText");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_run_local_accepts_operational_sugoroku_world_starter_template() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/operational/templates/sugoroku-world-starter"),
    )
    .expect("operational sugoroku-world starter template should run locally");

    assert_eq!(report.package_id, "operational-sugoroku-world-starter");
    assert_eq!(
        report.session.session_id,
        "session#operational-sugoroku-world-starter"
    );
    assert_eq!(report.runtime_plan.package_kind, "sugoroku_world");
    assert_eq!(report.runtime_plan.entry_place, "Place[SugorokuGamePlace]");
    assert_eq!(
        report.runtime_plan.declared_dependencies,
        vec!["../membership-chat-starter".to_owned()]
    );
    assert!(report.typed_host_io_claimed);
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_run_local_executes_declared_host_io_payload() {
    let package_json =
        fs::read_to_string(repo_root().join("samples/product-alpha1/demo/package.mir.json"))
            .expect("product demo package should be readable")
            .replace(r#""value": 41"#, r#""value": 7"#)
            .replace(r#""value": 42"#, r#""value": 8"#);
    let package_dir = write_package("product-alpha1-runtime-input-test", &package_json);
    for dependency in [
        "debug-layer",
        "auth-layer",
        "rate-limit-layer",
        "placeholder-object",
        "custom-avatar-preview",
    ] {
        fs::create_dir_all(package_dir.join(format!("packages/{dependency}")))
            .expect("dependency dir should be created");
        fs::write(
            package_dir.join(format!("packages/{dependency}/package.mir.json")),
            fs::read_to_string(repo_root().join(format!(
                "samples/product-alpha1/demo/packages/{dependency}/package.mir.json"
            )))
            .expect("dependency package should be readable"),
        )
        .expect("dependency package should be written");
    }

    let report = run_product_alpha1_local_session_path(&package_dir)
        .expect("runtime input package should run locally");

    assert_eq!(report.session.host_io_history[0].request_summary, "Int(7)");
    assert_eq!(report.session.host_io_history[0].response_summary, "Int(8)");
}

#[test]
fn product_alpha1_run_local_executes_mir_owned_add_one_path() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/computational/add-one-pure-mir"),
    )
    .expect("computational add-one root should run locally");

    assert!(report.typed_host_io_claimed);
    assert!(report.mir_computation_claimed);
    assert_eq!(report.session.host_io_history.len(), 2);
    assert_eq!(report.session.host_io_history[0].adapter_kind, "ReadInt");
    assert_eq!(report.session.host_io_history[0].request_summary, "Int(41)");
    assert_eq!(
        report.session.host_io_history[0].response_summary,
        "Int(41)"
    );
    assert_eq!(report.session.host_io_history[1].adapter_kind, "WriteInt");
    assert_eq!(report.session.host_io_history[1].request_summary, "Int(42)");
    assert_eq!(
        report.session.host_io_history[1].response_summary,
        "Int(42)"
    );
    assert_eq!(report.session.mir_compute_history.len(), 1);
    assert_eq!(report.session.mir_compute_history[0].function_id, "add_one");
    assert_eq!(
        report.session.mir_compute_history[0].input_summary,
        "Int(41)"
    );
    assert_eq!(
        report.session.mir_compute_history[0].output_summary,
        "Int(42)"
    );
    let event_kinds = report
        .session
        .event_dag
        .nodes
        .iter()
        .map(|node| node.event_kind.as_str())
        .collect::<Vec<_>>();
    let input_index = event_kinds
        .iter()
        .position(|kind| *kind == "host_input_received")
        .expect("computational row should emit host_input_received");
    let compute_index = event_kinds
        .iter()
        .position(|kind| *kind == "mir_compute_step")
        .expect("computational row should emit mir_compute_step");
    let output_index = event_kinds
        .iter()
        .position(|kind| *kind == "host_output_emitted")
        .expect("computational row should emit host_output_emitted");
    assert!(input_index < compute_index);
    assert!(compute_index < output_index);
}

#[test]
fn product_alpha1_run_local_executes_comp03_positive_modules() {
    let cases = [
        (
            "computational-scope-positive",
            "Computational.Scope.Positive",
            "clamp_zero",
            -5,
            0,
        ),
        (
            "computational-arrays-positive",
            "Computational.Arrays.Positive",
            "second",
            5,
            5,
        ),
        (
            "computational-vec3-positive",
            "Computational.Vec3.Positive",
            "length_squared",
            5,
            110,
        ),
        (
            "computational-control-flow-positive",
            "Computational.ControlFlow.Positive",
            "sum_to",
            5,
            15,
        ),
        (
            "computational-compose-positive",
            "Computational.Compose.Positive",
            "add_two",
            40,
            42,
        ),
    ];

    for (package_id, module_id, function_id, request_value, expected_output) in cases {
        let package_dir = write_package(
            "computational-positive-runtime",
            &computational_package_json(
                package_id,
                module_id,
                function_id,
                request_value,
                expected_output,
            ),
        );
        let report = run_product_alpha1_local_session_path(&package_dir)
            .expect("positive computational package should run locally");
        assert!(report.mir_computation_claimed, "package {package_id}");
        assert_eq!(
            report.session.mir_compute_history.len(),
            1,
            "package {package_id}"
        );
        assert_eq!(
            report.session.mir_compute_history[0].function_id, function_id,
            "package {package_id}"
        );
        assert_eq!(
            report.session.mir_compute_history[0].output_summary,
            format!("Int({expected_output})"),
            "package {package_id}"
        );
    }
}

#[test]
fn product_alpha1_run_local_rejects_comp03_negative_modules() {
    let cases = [
        (
            "computational-scope-negative",
            "Computational.Scope.NegativeUseBeforeDeclare",
            "clamp_zero",
            3,
            0,
            "unbound variable",
        ),
        (
            "computational-arrays-negative",
            "Computational.Arrays.NegativeOutOfBounds",
            "second",
            5,
            0,
            "out of bounds",
        ),
        (
            "computational-vec3-negative",
            "Computational.Vec3.NegativeField",
            "length_squared",
            5,
            0,
            "unknown field",
        ),
        (
            "computational-control-flow-negative",
            "Computational.ControlFlow.NegativeCondition",
            "sum_to",
            5,
            0,
            "condition must be Bool",
        ),
        (
            "computational-compose-negative",
            "Computational.Compose.NegativeMissingImport",
            "add_two",
            40,
            0,
            "add_one",
        ),
    ];

    for (package_id, module_id, function_id, request_value, expected_output, expected_detail) in
        cases
    {
        let package_dir = write_package(
            "computational-negative-runtime",
            &computational_package_json(
                package_id,
                module_id,
                function_id,
                request_value,
                expected_output,
            ),
        );
        let error = run_product_alpha1_local_session_path(&package_dir)
            .expect_err("negative computational package should reject");
        assert_eq!(error.kind, ProductAlpha1SessionErrorKind::MirCompute);
        assert!(
            error.detail.contains(expected_detail),
            "package {package_id} detail was {}",
            error.detail
        );
    }
}

#[test]
fn product_alpha1_run_local_executes_declared_echo_text_payload() {
    let package_json = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "product-alpha1-echo-text-runtime",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "membership_chat",
  "dependencies": [],
  "effects": ["typed_host_io.echo_text", "SendRoomMessage"],
  "failures": ["AdapterUnavailable", "RateLimited"],
  "capabilities": ["JoinWorld", "ObserveWorld", "SendRoomMessage"],
  "witness_requirements": [],
  "membership_requirements": ["active_participant"],
  "auth_policy": {
    "policy_id": "echo-text-auth-policy",
    "required_bindings": ["participant_membership"]
  },
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {
      "contract_id": "echo-text-chat-contract",
      "variance": "invariant",
      "effect_row": ["SendRoomMessage"],
      "failure_row": ["RateLimited"]
    }
  ],
  "observation_policy": {
    "view_role": "observer_safe",
    "labels": ["observer_safe_chat_summary"]
  },
  "redaction_policy": {
    "level": "observer_safe",
    "redacted_fields": ["raw_witness_payload", "raw_auth_evidence"]
  },
  "retention_policy": {
    "scope": "echo_text_session",
    "retained_artifacts": ["checker_report", "runtime_plan", "observer_safe_chat_lane"]
  },
  "message_recovery_policy": {
    "handled_failures": ["timeout", "reject"],
    "recovery": "retry_then_reject"
  },
  "savepoint_policy": {
    "classes": ["R0", "R2"],
    "quiescent_required": true
  },
  "runtime_input": {
    "entry_place": "Place[ChatPlace]",
    "host_io": {
      "adapter_kind": "EchoText",
      "effect_ref": "typed_host_io.echo_text",
      "request_payload": {"kind": "text", "value": "Mika"},
      "expected_response": {"kind": "text", "value": "Hello, Mika!"}
    }
  },
  "native_policy": {
    "execution_policy": "disabled",
    "provenance_required": true
  },
  "compatibility": {
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }
}"#;
    let package_dir = write_package("product-alpha1-echo-text-runtime-test", package_json);

    let report = run_product_alpha1_local_session_path(&package_dir)
        .expect("EchoText runtime input package should run locally");

    assert_eq!(
        report.session.host_io_history[0].request_summary,
        "Text(\"Mika\")"
    );
    assert_eq!(
        report.session.host_io_history[0].response_summary,
        "Text(\"Hello, Mika!\")"
    );
}

#[test]
fn product_alpha1_run_local_executes_declared_chat_text_payload() {
    let package_json = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "product-alpha1-chat-text-runtime",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "membership_chat",
  "dependencies": [],
  "effects": ["typed_host_io.chat_text", "SendRoomMessage"],
  "failures": ["AdapterUnavailable", "RateLimited"],
  "capabilities": ["JoinWorld", "ObserveWorld", "SendRoomMessage"],
  "witness_requirements": [],
  "membership_requirements": ["active_participant"],
  "auth_policy": {
    "policy_id": "chat-text-auth-policy",
    "required_bindings": ["participant_membership"]
  },
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {
      "contract_id": "chat-text-chat-contract",
      "variance": "invariant",
      "effect_row": ["SendRoomMessage"],
      "failure_row": ["RateLimited"]
    }
  ],
  "observation_policy": {
    "view_role": "observer_safe",
    "labels": ["observer_safe_chat_summary"]
  },
  "redaction_policy": {
    "level": "observer_safe",
    "redacted_fields": ["raw_witness_payload", "raw_auth_evidence"]
  },
  "retention_policy": {
    "scope": "chat_text_session",
    "retained_artifacts": ["checker_report", "runtime_plan", "observer_safe_chat_lane"]
  },
  "message_recovery_policy": {
    "handled_failures": ["timeout", "reject"],
    "recovery": "retry_then_reject"
  },
  "savepoint_policy": {
    "classes": ["R0", "R2"],
    "quiescent_required": true
  },
  "runtime_input": {
    "entry_place": "Place[ChatPlace]",
    "host_io": {
      "adapter_kind": "ChatText",
      "effect_ref": "typed_host_io.chat_text",
      "request_payload": {"kind": "text", "value": "ready to play"},
      "expected_response": {"kind": "text", "value": "room#lobby message accepted: ready to play"}
    }
  },
  "native_policy": {
    "execution_policy": "disabled",
    "provenance_required": true
  },
  "compatibility": {
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }
}"#;
    let package_dir = write_package("product-alpha1-chat-text-runtime-test", package_json);

    let report = run_product_alpha1_local_session_path(&package_dir)
        .expect("ChatText runtime input package should run locally");

    assert_eq!(
        report.session.host_io_history[0].request_summary,
        "Text(\"ready to play\")"
    );
    assert_eq!(
        report.session.host_io_history[0].response_summary,
        "Text(\"room#lobby message accepted: ready to play\")"
    );
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
            .contains("run-local requires a product alpha-1 world-like package")
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

#[test]
fn product_alpha1_run_local_records_message_recovery_contract_rows() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");

    let recovery = &report.session.message_recovery_state;
    assert!(recovery.runtime_recovery_claimed);
    assert!(
        recovery
            .transport_contracts
            .iter()
            .any(|contract| contract.contract_kind == "TimeoutBounded")
    );
    assert!(
        recovery
            .recovery_policies
            .iter()
            .any(|policy| policy.policy_kind == "RetryThenReject")
    );
    assert!(
        recovery
            .failure_observations
            .iter()
            .any(|failure| failure.failure_class == "timeout"
                && failure.terminal_state == "Rejected")
    );
    for failure in &recovery.failure_observations {
        assert!(
            report
                .session
                .event_dag
                .nodes
                .iter()
                .any(|node| node.envelope_ref.as_deref() == Some(failure.envelope_id.as_str())),
            "failure observation {} should be linked from the event DAG",
            failure.envelope_id
        );
        assert!(
            report
                .session
                .route_graph
                .routes
                .iter()
                .any(|route| route.envelope_id == failure.envelope_id),
            "failure observation {} should be linked from the route graph",
            failure.envelope_id
        );
    }
}

#[test]
fn product_alpha1_save_and_load_restore_same_session_frontier() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let (attached, _) = attach_product_alpha1_package_to_session_path(
        &report.session,
        repo_root().join("samples/product-alpha1/demo/packages/debug-layer"),
    )
    .expect("debug layer should attach before save");
    let (saved, save_report) =
        save_product_alpha1_session(&attached, "savepoint#r0").expect("R0 local save should work");

    let mut mutated_after_save = saved.clone();
    mutated_after_save.phase = "mutated_after_save".to_string();
    mutated_after_save
        .active_layers
        .push("post-save-test-layer".to_string());
    let (loaded, load_report) = load_product_alpha1_session(&mutated_after_save, "savepoint#r0")
        .expect("R0 local load should work");

    assert_eq!(save_report.savepoint_class, "R0_Local");
    assert!(save_report.state_roundtrip_equal);
    assert_eq!(load_report.terminal_outcome, "loaded");
    assert_eq!(loaded.phase, "loaded");
    assert_eq!(loaded.active_layers, vec!["product-alpha1-debug-layer"]);
    assert_eq!(loaded.hotplug_lifecycle.len(), 1);
    assert_eq!(
        loaded.save_load_state.local_savepoint_refs,
        vec!["savepoint#r0"]
    );
    assert_event_ids_unique(&loaded);
}

#[test]
fn product_alpha1_save_and_load_preserve_mir_compute_history() {
    let report = run_product_alpha1_local_session_path(
        repo_root().join("samples/product-alpha1/computational/add-one-pure-mir"),
    )
    .expect("computational add-one root should run locally");
    let (saved, save_report) = save_product_alpha1_session(&report.session, "savepoint#comp-r0")
        .expect("computational R0 local save should work");
    let (loaded, load_report) = load_product_alpha1_session(&saved, "savepoint#comp-r0")
        .expect("computational R0 local load should work");

    assert_eq!(save_report.savepoint_class, "R0_Local");
    assert_eq!(load_report.terminal_outcome, "loaded");
    assert_eq!(saved.savepoints.len(), 1);
    assert_eq!(saved.savepoints[0].saved_mir_compute_history.len(), 1);
    assert_eq!(
        loaded.mir_compute_history,
        report.session.mir_compute_history
    );
    assert_eq!(loaded.host_io_history, report.session.host_io_history);
    assert_event_ids_unique(&loaded);
}

#[test]
fn product_alpha1_load_rejects_stale_membership_and_activation_cut_rewind() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let (saved_before_attach, _) = save_product_alpha1_session(&report.session, "savepoint#pre")
        .expect("R0 local save should work");
    let (attached_after_save, _) = attach_product_alpha1_package_to_session_path(
        &saved_before_attach,
        repo_root().join("samples/product-alpha1/demo/packages/debug-layer"),
    )
    .expect("debug layer should attach after save");

    let activation_error = load_product_alpha1_session(&attached_after_save, "savepoint#pre")
        .expect_err("load should not rewind across an accepted activation cut");
    assert_eq!(
        activation_error.kind,
        ProductAlpha1SessionErrorKind::LoadAdmissibility
    );

    let (saved, _) = save_product_alpha1_session(&report.session, "savepoint#r0")
        .expect("R0 local save should work");
    let mut stale_membership = saved.clone();
    stale_membership.membership.membership_epoch += 1;
    stale_membership
        .membership
        .active_members
        .push("stale_member_after_save".to_string());

    let stale_error = load_product_alpha1_session(&stale_membership, "savepoint#r0")
        .expect_err("load should reject stale membership resurrection");
    assert_eq!(
        stale_error.kind,
        ProductAlpha1SessionErrorKind::LoadAdmissibility
    );
}

#[test]
fn product_alpha1_repeated_save_load_and_quiescent_save_keep_event_ids_unique() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let (saved_once, _) = save_product_alpha1_session(&report.session, "savepoint#repeat")
        .expect("first save should work");
    let (saved_twice, _) = save_product_alpha1_session(&saved_once, "savepoint#repeat")
        .expect("second save with same savepoint should not duplicate event ids");
    let (quiescent_once, _) =
        quiescent_save_product_alpha1_session(&saved_twice, "savepoint#r2-repeat")
            .expect("first quiescent save should work");
    let (quiescent_twice, _) =
        quiescent_save_product_alpha1_session(&quiescent_once, "savepoint#r2-repeat")
            .expect("second quiescent save should not duplicate event ids");
    let (loaded, _) = load_product_alpha1_session(&quiescent_twice, "savepoint#repeat")
        .expect("load should work after repeated saves");

    assert_event_ids_unique(&quiescent_twice);
    assert_event_ids_unique(&loaded);
}

#[test]
fn product_alpha1_quiescent_save_emits_r2_obligations() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let (saved, quiescent_report) =
        quiescent_save_product_alpha1_session(&report.session, "savepoint#r2")
            .expect("R2 quiescent save should succeed on a quiet local session");

    assert_eq!(quiescent_report.terminal_outcome, "saved");
    assert_eq!(quiescent_report.savepoint_class, "R2_Quiescent");
    assert!(quiescent_report.no_inflight);
    assert!(quiescent_report.all_places_sealed);
    assert!(quiescent_report.no_post_cut_send);
    assert_eq!(
        quiescent_report
            .rejected_post_cut_sends
            .iter()
            .map(|record| record.outcome.as_str())
            .collect::<Vec<_>>(),
        vec!["rejected"]
    );
    assert!(saved.save_load_state.quiescent_save_ready);
    assert_eq!(
        saved.save_load_state.local_savepoint_refs,
        vec!["savepoint#r2"]
    );
    assert!(
        saved
            .event_dag
            .nodes
            .iter()
            .any(|node| node.event_kind == "quiescent_save")
    );
    assert_eq!(
        saved
            .save_load_state
            .quiescence_state
            .sealed_place_refs
            .len(),
        saved.runtime_plan.place_graph.nodes.len()
    );
    assert!(
        saved
            .save_load_state
            .quiescence_state
            .rejected_post_cut_sends
            .iter()
            .any(|record| record.savepoint_id == "savepoint#r2" && record.outcome == "rejected")
    );
    assert_event_ids_unique(&saved);
}

#[test]
fn product_alpha1_quiescent_save_rejects_inflight_messages() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let mut session = report.session;
    session.message_recovery_state.message_state_lane.push(
        mir_runtime::product_alpha1_session::ProductAlpha1MessageStateRecord {
            envelope_id: "envelope#inflight-test".to_string(),
            state: "InFlight".to_string(),
            failure_class: None,
            recovery_action: None,
        },
    );

    let (rejected, quiescent_report) =
        quiescent_save_product_alpha1_session(&session, "savepoint#blocked")
            .expect("rejected quiescent save should still return an observable report");

    assert_eq!(quiescent_report.terminal_outcome, "rejected");
    assert!(!quiescent_report.no_inflight);
    assert!(quiescent_report.all_places_sealed);
    assert!(
        quiescent_report
            .failed_messages
            .contains(&"envelope#inflight-test".to_string())
    );
    assert!(rejected.save_load_state.local_savepoint_refs.is_empty());
    assert!(!rejected.save_load_state.quiescent_save_ready);
}

#[test]
fn product_alpha1_quiescent_save_rejects_missing_r2_or_missing_preflight_gate() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");

    let mut no_r2 = report.session.clone();
    no_r2.save_load_state.declared_savepoint_classes = vec!["R0".to_string()];
    no_r2.runtime_plan.savepoint_classes = vec!["R0".to_string()];
    no_r2.runtime_plan.quiescent_save_requested = false;
    let (rejected_no_r2, no_r2_report) =
        quiescent_save_product_alpha1_session(&no_r2, "savepoint#no-r2")
            .expect("missing R2 should be reported as an observable reject");
    assert_eq!(no_r2_report.terminal_outcome, "rejected");
    assert!(!rejected_no_r2.save_load_state.quiescent_save_ready);

    let mut guard_disabled = report.session;
    guard_disabled
        .save_load_state
        .quiescence_state
        .post_cut_send_guard_enabled = false;
    let (rejected_guard, guard_report) =
        quiescent_save_product_alpha1_session(&guard_disabled, "savepoint#no-guard")
            .expect("missing post-cut-send guard should be reported as an observable reject");
    assert_eq!(guard_report.terminal_outcome, "rejected");
    assert!(!guard_report.no_post_cut_send);
    assert!(
        rejected_guard
            .save_load_state
            .local_savepoint_refs
            .is_empty()
    );
}

#[test]
fn product_alpha1_load_restores_save_load_timeline_to_saved_frontier() {
    let report =
        run_product_alpha1_local_session_path(repo_root().join("samples/product-alpha1/demo"))
            .expect("product alpha demo should run locally");
    let (saved_r0, _) = save_product_alpha1_session(&report.session, "savepoint#r0")
        .expect("R0 local save should work");
    let (saved_r2, _) = quiescent_save_product_alpha1_session(&saved_r0, "savepoint#r2")
        .expect("R2 local quiescent save should work");
    let (loaded_r0, _) = load_product_alpha1_session(&saved_r2, "savepoint#r0")
        .expect("load should restore the R0 frontier");

    assert_eq!(
        loaded_r0.save_load_state.local_savepoint_refs,
        vec!["savepoint#r0"]
    );
    assert!(!loaded_r0.save_load_state.quiescent_save_ready);
    assert!(
        loaded_r0
            .event_dag
            .nodes
            .iter()
            .all(|node| node.event_kind != "quiescent_save")
    );
}
