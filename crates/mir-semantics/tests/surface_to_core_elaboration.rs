use mir_semantics::surface_to_core_elaboration::{
    elaborate_surface_to_core_path, elaborate_surface_to_core_source,
    surface_elaboration_diagnostic_codes,
};
use serde_json::Value;
use std::path::PathBuf;

fn assert_no_placeholder_repair_values(value: &Value) {
    match value {
        Value::String(text) => {
            let normalized = text.trim().to_ascii_lowercase();
            assert!(!normalized.is_empty(), "empty repair payload string");
            assert!(
                !matches!(
                    normalized.as_str(),
                    "fixme" | "placeholder" | "tbd" | "todo" | "unknown" | "unresolved"
                ),
                "placeholder repair payload string: {text}"
            );
            assert!(
                ![
                    "fixme",
                    "placeholder",
                    "tbd",
                    "todo",
                    "unknown",
                    "unresolved"
                ]
                .iter()
                .any(|marker| normalized.contains(marker)),
                "placeholder repair payload string: {text}"
            );
        }
        Value::Array(items) => {
            for item in items {
                assert_no_placeholder_repair_values(item);
            }
        }
        Value::Object(map) => {
            for nested in map.values() {
                assert_no_placeholder_repair_values(nested);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn rejected_lab_details_for_source(source: &str) -> Vec<Value> {
    let report = elaborate_surface_to_core_source(source);
    assert!(!report.accepted);
    serde_json::to_value(&report).expect("report serializes")["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted")
        .clone()
}

fn rejected_lab_details_for_sample(path: &str) -> Vec<Value> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let report = elaborate_surface_to_core_path(repo_root.join(path));
    assert!(!report.accepted, "{path} should be rejected");
    serde_json::to_value(&report).expect("report serializes")["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted")
        .clone()
}

fn assert_no_set_insertion_repair(detail: &Value) {
    if let Some(repairs) = detail.get("suggested_repair").and_then(Value::as_array) {
        assert!(
            repairs
                .iter()
                .all(|repair| repair["repair_shape"] != "set_insertion"),
            "unexpected set_insertion repair: {repairs:?}"
        );
    }
}

fn assert_complete_set_insertion_not_bundle_or_partial(detail: &Value, repair: &Value) {
    assert_eq!(repair["repair_shape"], "set_insertion");
    assert_eq!(
        repair["coverage_scope"],
        "complete_missing_set_for_associated_request"
    );
    assert_eq!(
        repair["local_premise_after_edit"],
        "discharged_for_associated_request"
    );
    assert_eq!(repair["insert_failures"], detail["missing_evidence"]);
    assert_eq!(
        repair["declared_failures_before"],
        detail["failure_row_context"]["declared_failures"]
    );
    assert_eq!(
        repair["local_effect"]["declared_failures_after"],
        repair["required_failures"]
    );
    assert_eq!(
        repair["element_insert_count"].as_u64(),
        Some(
            repair["insert_failures"]
                .as_array()
                .expect("set insertion records inserted failures")
                .len() as u64
        )
    );
    assert_eq!(repair["element_insert_count"], 3);
    assert!(
        repair.get("missing_failure").is_none(),
        "ELAB-07 set repair must not be serialized as singleton child repair"
    );
    assert!(
        repair.get("declared_failures").is_none(),
        "ELAB-07 set repair must not reuse singleton declared_failures"
    );
    for forbidden_key in [
        "repair_group_id",
        "bundle_semantics",
        "child_repairs",
        "partiality",
        "guidance_text",
        "textual_guidance",
    ] {
        assert!(
            repair.get(forbidden_key).is_none(),
            "ELAB-07 set repair must not carry {forbidden_key}"
        );
    }
}

fn assert_obl024_diagnostic_soundness_projection(detail: &Value) {
    assert!(
        detail["failure_row_context"]
            .get("association_key")
            .is_none(),
        "internal association_key must not be serialized in LAB JSON"
    );
    assert!(
        detail["failure_row_context"]
            .get("associated_request_count")
            .is_none(),
        "internal associated_request_count must not be serialized in LAB JSON"
    );
    let projection = detail["diagnostic_soundness_projection"]
        .as_object()
        .expect("LAB OBL-024 diagnostic soundness projection is emitted");
    assert_eq!(
        projection["diagnostic_id"],
        serde_json::json!(format!(
            "{}:{}:{}",
            detail["legacy_code"]
                .as_str()
                .expect("legacy_code is a string"),
            detail["canon_id"].as_str().expect("canon_id is a string"),
            detail["request_context"]["request_id"]
                .as_str()
                .expect("request id is a string")
        ))
    );
    assert_eq!(
        projection["lab_association_key"],
        serde_json::json!(format!(
            "{}|request={}",
            detail["failure_row_context"]["target_ref"]
                .as_str()
                .expect("target_ref is a string"),
            detail["request_context"]["request_id"]
                .as_str()
                .expect("request id is a string")
        ))
    );
    assert_eq!(
        projection["reported_rule_instance"],
        detail["rule_instance"]
    );
    assert_eq!(
        projection["reported_failed_premise"],
        detail["failed_premise"]
    );
    assert_eq!(
        projection["reported_bindings"]["request_id"],
        detail["request_context"]["request_id"]
    );
    assert_eq!(
        projection["reported_bindings"]["request_kind"],
        detail["request_context"]["request_kind"]
    );
    assert_eq!(
        projection["reported_bindings"]["generated_from"],
        detail["request_context"]["generated_from"]
    );
    assert_eq!(
        projection["reported_bindings"]["requester_locus"],
        detail["request_context"]["requester_locus"]
    );
    assert_eq!(
        projection["reported_bindings"]["owner_locus"],
        detail["request_context"]["owner_locus"]
    );
    assert_eq!(
        projection["reported_bindings"]["state_name"],
        detail["request_context"]["state_name"]
    );
    assert_eq!(
        projection["reported_bindings"]["key_expr"],
        detail["request_context"]["key_expr"]
    );
    assert_eq!(
        projection["reported_bindings"]["field_name"],
        detail["request_context"]["field_name"]
    );
    assert_eq!(
        projection["reported_bindings"]["target_kind"],
        detail["failure_row_context"]["target_kind"]
    );
    assert_eq!(
        projection["reported_bindings"]["target_ref"],
        detail["failure_row_context"]["target_ref"]
    );
    assert_eq!(
        projection["reported_bindings"]["target_locus"],
        detail["failure_row_context"]["target_locus"]
    );
    assert_eq!(
        projection["reported_bindings"]["event_name"],
        detail["failure_row_context"]["event_name"]
    );
    assert_eq!(
        projection["reported_bindings"]["required_failures"],
        detail["failure_row_context"]["required_failures"]
    );
    assert_eq!(
        projection["reported_bindings"]["declared_failures"],
        detail["failure_row_context"]["declared_failures"]
    );
    assert_eq!(
        projection["reported_bindings"]["missing_failures"],
        detail["failure_row_context"]["missing_failures"]
    );
    assert_eq!(
        projection["trace_local_replay"]["replay_scope"],
        "surface_to_core_elaboration.report_local"
    );
    assert_eq!(
        projection["trace_local_replay"]["replayed_request_id"],
        detail["request_context"]["request_id"]
    );
    assert_eq!(
        projection["trace_local_replay"]["replayed_target_ref"],
        detail["failure_row_context"]["target_ref"]
    );
    assert_eq!(
        projection["trace_local_replay"]["fails_exactly_at"],
        detail["failure_row_context"]["local_premise"]
    );
    assert_eq!(
        projection["trace_local_replay"]["expected_missing_evidence"],
        detail["missing_evidence"]
    );
    assert_eq!(
        projection["trace_local_replay"]["failure_reason"],
        "missing_generated_failures"
    );
    assert_eq!(projection["trace_local_replay"]["replay_non_final"], true);
    assert_eq!(projection["projection_non_final"], true);
    assert_eq!(projection["lab_non_final"], true);
}

#[test]
#[should_panic(expected = "placeholder repair payload string")]
fn placeholder_repair_detector_rejects_marker_substrings() {
    assert_no_placeholder_repair_values(&serde_json::json!({
        "target_ref": "fixme target_ref",
        "span": "tbd span",
        "row": "unknown row",
        "status": "unresolved target"
    }));
}

#[test]
fn elaborates_cross_locus_read_into_remote_request_with_observe_edge() {
    let source = r#"
module Surface.Elab.CrossRead

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.final_public_api_frozen);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let request = &report.core_ir.remote_requests[0];
    assert_eq!(request.request_kind, "read");
    assert_eq!(request.requester_locus, "role:BrowserClient");
    assert_eq!(request.owner_locus, "S");
    assert_eq!(request.state_name, "player");
    assert_eq!(request.key_expr, "self");
    assert!(request.source_span.end > request.source_span.start);
    assert!(
        report.core_ir.generated_edges.iter().any(
            |edge| edge.edge_kind == "observe_request" && edge.request_id == request.request_id
        )
    );
}

#[test]
fn generated_auto_communication_is_explicit_for_visible_remote_read() {
    let source = r#"
module Surface.Elab.AutoCommunicationRead

role BrowserClient
place S

record Player {
  hp: Int64,
  secret_key: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);
    let core_ir = serde_json::to_value(&report.core_ir).expect("core IR serializes");

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(core_ir["message_envelopes"].as_array().unwrap().len(), 1);
    assert_eq!(
        core_ir["message_envelopes"][0]["envelope_kind"],
        Value::String("remote_read".to_string())
    );
    assert_eq!(core_ir["observations"].as_array().unwrap().len(), 1);
    assert_eq!(
        core_ir["observations"][0]["field_name"],
        Value::String("hp".to_string())
    );
    assert!(
        report
            .core_ir
            .generated_edges
            .iter()
            .any(|edge| edge.edge_kind == "message_envelope")
    );
    assert!(
        report
            .core_ir
            .generated_edges
            .iter()
            .any(|edge| edge.edge_kind == "auto_observe")
    );
}

#[test]
fn visible_remote_write_generates_publish_and_observe_rows() {
    let source = r#"
module Surface.Elab.AutoCommunicationWrite

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when heal(target: Participant) fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    S {
      player[target].hp = player[target].hp + 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);
    let core_ir = serde_json::to_value(&report.core_ir).expect("core IR serializes");

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(core_ir["message_envelopes"].as_array().unwrap().len(), 1);
    assert_eq!(core_ir["publications"].as_array().unwrap().len(), 1);
    assert_eq!(
        core_ir["publications"][0]["field_name"],
        Value::String("hp".to_string())
    );
    assert_eq!(core_ir["observations"].as_array().unwrap().len(), 1);
    assert!(
        report
            .core_ir
            .generated_edges
            .iter()
            .any(|edge| edge.edge_kind == "auto_publish")
    );
    assert!(
        report
            .core_ir
            .generated_edges
            .iter()
            .any(|edge| edge.edge_kind == "auto_observe")
    );
}

#[test]
fn whole_record_visible_remote_read_allows_field_auto_observe() {
    let source = r#"
module Surface.Elab.WholeRecordVisibleRead

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);
    let core_ir = serde_json::to_value(&report.core_ir).expect("core IR serializes");

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(core_ir["message_envelopes"].as_array().unwrap().len(), 1);
    assert_eq!(core_ir["observations"].as_array().unwrap().len(), 1);
    assert_eq!(
        core_ir["observations"][0]["field_name"],
        Value::String("hp".to_string())
    );
}

#[test]
fn whole_record_visible_remote_write_generates_publish_and_observe() {
    let source = r#"
module Surface.Elab.WholeRecordVisibleWrite

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe
}

BrowserClient[self] {
  when heal(target: Participant) fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    S {
      player[target].hp = player[target].hp + 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);
    let core_ir = serde_json::to_value(&report.core_ir).expect("core IR serializes");

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(core_ir["message_envelopes"].as_array().unwrap().len(), 1);
    assert_eq!(core_ir["publications"].as_array().unwrap().len(), 1);
    assert_eq!(core_ir["observations"].as_array().unwrap().len(), 1);
}

#[test]
fn private_looking_visible_decl_is_rejected_only_when_communicated() {
    let source = r#"
module Surface.Elab.PrivateVisibleDeclLocalOnly

place S

record Player {
  secret_key: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { secret_key }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(report.core_ir.message_envelopes.is_empty());
}

#[test]
fn rejects_private_field_remote_read_before_auto_observe() {
    let source = r#"
module Surface.Elab.PrivateFieldRead

role BrowserClient
place S

record Player {
  hp: Int64,
  secret_key: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    seen_secret = player[self].secret_key
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["private_field_auto_publish_rejected"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let core_ir = serde_json::to_value(&report.core_ir).expect("core IR serializes");
    assert_eq!(core_ir["publications"].as_array().unwrap().len(), 0);
    assert_eq!(core_ir["observations"].as_array().unwrap().len(), 0);
}

#[test]
fn elaborates_nested_place_write_into_owner_directed_remote_request() {
    let source = r#"
module Surface.Elab.CrossWrite

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    S {
      player[target].hp = player[target].hp - 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let request = &report.core_ir.remote_requests[0];
    assert_eq!(request.request_kind, "write");
    assert_eq!(request.requester_locus, "role:BrowserClient");
    assert_eq!(request.owner_locus, "S");
    assert_eq!(request.state_name, "player");
    assert_eq!(request.key_expr, "target");
    assert_eq!(request.generated_from, "nested_place_block");
    assert!(
        report
            .core_ir
            .transitions
            .iter()
            .any(|transition| transition.kind == "generated_remote_write_request")
    );
}

#[test]
fn records_assignment_rhs_reads_as_dependencies_without_remote_read_materialization() {
    let source = r#"
module Surface.Elab.AttackDependency

role BrowserClient
place S

record Player {
  hp: Int64,
  atk: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    S {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);
    let core_ir = serde_json::to_value(&report.core_ir).expect("core IR serializes");
    let dependencies = core_ir["dependencies"]
        .as_array()
        .expect("core IR exposes dependency rows");

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert_eq!(report.core_ir.remote_requests[0].request_kind, "write");
    assert_eq!(core_ir["message_envelopes"].as_array().unwrap().len(), 1);
    assert_eq!(core_ir["observations"].as_array().unwrap().len(), 0);
    assert_eq!(dependencies.len(), 2);
    assert_eq!(
        dependencies[0]["dependency_kind"],
        Value::String("rhs_indexed_read".to_string())
    );
    assert_eq!(
        dependencies[0]["key_expr"],
        Value::String("target".to_string())
    );
    assert_eq!(
        dependencies[0]["field_name"],
        Value::String("hp".to_string())
    );
    assert_eq!(
        dependencies[1]["key_expr"],
        Value::String("self".to_string())
    );
    assert_eq!(
        dependencies[1]["field_name"],
        Value::String("atk".to_string())
    );
}

#[test]
fn rejects_generated_remote_request_when_failure_row_is_underdeclared() {
    let source = r#"
module Surface.Elab.UnderdeclaredFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["generated_failure_not_declared"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert!(!report.core_ir.remote_requests[0].failure_row_complete);

    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["legacy_code"], "generated_failure_not_declared");
    assert_eq!(details[0]["canon_id"], "E-ROW-001");
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!([
            "MissingWitness",
            "RouteUnavailable",
            "StaleMembership",
            "VisibilityDenied"
        ])
    );
    assert_eq!(
        details[0]["request_context"],
        serde_json::json!({
            "request_id": "req-0001",
            "request_kind": "read",
            "generated_from": "cross_locus_read_expression",
            "requester_locus": "role:BrowserClient",
            "owner_locus": "S",
            "state_name": "player",
            "key_expr": "self",
            "field_name": "hp"
        })
    );
    assert_eq!(
        details[0]["failure_row_context"],
        serde_json::json!({
            "target_kind": "when_fails_row",
            "target_ref": "when_fails_row|locus=role:BrowserClient|event=render",
            "target_locus": "role:BrowserClient",
            "event_name": "render",
            "required_failures": [
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership",
                "VisibilityDenied"
            ],
            "declared_failures": [
                "MissingCapability"
            ],
            "missing_failures": [
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership",
                "VisibilityDenied"
            ],
            "local_premise": "generated_failures_subset_declared_fails"
        })
    );
    assert_obl024_diagnostic_soundness_projection(&details[0]);
    assert!(details[0].get("suggested_repair").is_none());
}

#[test]
fn elab04_sample_fixture_carries_obl024_projection_without_repair() {
    let details = rejected_lab_details_for_sample(
        "samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/main/src/undeclared-generated-failure-negative.mir",
    );

    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["canon_id"], "E-ROW-001");
    assert_obl024_diagnostic_soundness_projection(&details[0]);
    assert!(details[0].get("suggested_repair").is_none());
}

#[test]
fn rejects_generated_write_request_when_failure_row_is_underdeclared() {
    let source = r#"
module Surface.Elab.UnderdeclaredWriteFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["generated_failure_not_declared"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert_eq!(report.core_ir.remote_requests[0].request_kind, "write");
    assert!(!report.core_ir.remote_requests[0].failure_row_complete);

    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["legacy_code"], "generated_failure_not_declared");
    assert_eq!(details[0]["canon_id"], "E-ROW-001");
    assert_eq!(details[0]["severity"], "error");
    assert_eq!(details[0]["rule_instance"], "BND-001.row-containment");
    assert_eq!(
        details[0]["failed_premise"],
        "generated_failures_subset_declared_fails"
    );
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!(["MissingWitness", "RouteUnavailable", "StaleMembership"])
    );
    assert_eq!(
        details[0]["request_context"],
        serde_json::json!({
            "request_id": "req-0001",
            "request_kind": "write",
            "generated_from": "nested_place_block",
            "requester_locus": "role:BrowserClient",
            "owner_locus": "S",
            "state_name": "player",
            "key_expr": "target",
            "field_name": "hp"
        })
    );
    assert_eq!(
        details[0]["failure_row_context"],
        serde_json::json!({
            "target_kind": "when_fails_row",
            "target_ref": "when_fails_row|locus=role:BrowserClient|event=attack",
            "target_locus": "role:BrowserClient",
            "event_name": "attack",
            "required_failures": [
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership"
            ],
            "declared_failures": [
                "MissingCapability"
            ],
            "missing_failures": [
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership"
            ],
            "local_premise": "generated_failures_subset_declared_fails"
        })
    );
    let repairs = details[0]["suggested_repair"]
        .as_array()
        .expect("ELAB-07 emits one LAB set-insertion repair item");
    assert_eq!(repairs.len(), 1);
    let repair = &repairs[0];
    assert_no_placeholder_repair_values(repair);
    assert_eq!(repair["repair_shape"], "set_insertion");
    assert_eq!(repair["repair_family"], "add-to-fails-row");
    assert_eq!(repair["diagnostic_family"], "E-ROW-001");
    assert_eq!(
        repair["edit_atom"],
        "complete_missing_base_failure_set_into_one_existing_when_fails_row"
    );
    assert_eq!(repair["source_locus_edit_count"], 1);
    assert_eq!(repair["element_insert_count"], 3);
    assert!(repair.get("missing_failure").is_none());
    assert!(repair.get("declared_failures").is_none());
    assert_eq!(
        repair["applies_to"],
        serde_json::json!({
            "legacy_code": "generated_failure_not_declared",
            "canon_id": "E-ROW-001",
            "request_id": "req-0001"
        })
    );
    assert_eq!(repair["target_kind"], "when_fails_row");
    assert_eq!(
        repair["target_context"],
        serde_json::json!({
            "target_ref": "when_fails_row|locus=role:BrowserClient|event=attack",
            "locus": "role:BrowserClient",
            "event_name": "attack"
        })
    );
    assert_eq!(
        repair["declared_failures_before"],
        serde_json::json!(["MissingCapability"])
    );
    assert_eq!(
        repair["insert_failures"],
        serde_json::json!(["MissingWitness", "RouteUnavailable", "StaleMembership"])
    );
    assert_eq!(
        repair["required_failures"],
        serde_json::json!([
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
            "StaleMembership"
        ])
    );
    assert_eq!(
        repair["local_effect"]["declared_failures_after"],
        repair["required_failures"]
    );
    assert_eq!(
        repair["coverage_scope"],
        "complete_missing_set_for_associated_request"
    );
    assert_eq!(
        repair["local_premise"],
        details[0]["failure_row_context"]["local_premise"]
    );
    assert_obl024_diagnostic_soundness_projection(&details[0]);
    assert_eq!(
        repair["local_premise_after_edit"],
        "discharged_for_associated_request"
    );
    assert_eq!(
        repair["single_edit_assumption"],
        "erow001_elab07_complete_base_failure_set_source_locus_edit"
    );
    assert_eq!(
        repair["non_goal"],
        "does_not_authorize_capability_witness_route_membership_or_claim_runtime_success"
    );
    assert!(repair["repair_non_final"].as_bool().unwrap_or(false));
    assert!(repair["lab_non_final"].as_bool().unwrap_or(false));
    assert_complete_set_insertion_not_bundle_or_partial(&details[0], repair);
}

#[test]
fn elab07_sample_fixture_carries_obl024_projection_with_exact_set_repair() {
    let details = rejected_lab_details_for_sample(
        "samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/main/src/write-failure-row-negative.mir",
    );

    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["canon_id"], "E-ROW-001");
    assert_obl024_diagnostic_soundness_projection(&details[0]);
    let repairs = details[0]["suggested_repair"]
        .as_array()
        .expect("ELAB-07 sample emits one LAB set-insertion repair item");
    assert_eq!(repairs.len(), 1);
    assert_eq!(repairs[0]["repair_shape"], "set_insertion");
}

#[test]
fn elab07_set_insertion_is_not_child_bundle_or_partial_guidance() {
    let source = r#"
module Surface.Elab.SetInsertionChildBundlePartialGuard

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    let repairs = details[0]["suggested_repair"]
        .as_array()
        .expect("ELAB-07 exact locus emits one complete set repair");
    assert_eq!(
        repairs.len(),
        1,
        "ELAB-07 must not serialize three child alternatives or partial guidance items"
    );
    assert_complete_set_insertion_not_bundle_or_partial(&details[0], &repairs[0]);
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_two_missing_proper_subset() {
    let source = r#"
module Surface.Elab.SetInsertionGuardTwoMissingSubset

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, MissingWitness {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!(["RouteUnavailable", "StaleMembership"])
    );
    assert!(details[0].get("suggested_repair").is_none());
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_padded_declared_failure_row() {
    let source = r#"
module Surface.Elab.SetInsertionGuardPaddedFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, ExtraFailure {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0]["failure_row_context"]["declared_failures"],
        serde_json::json!(["MissingCapability", "ExtraFailure"])
    );
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!(["MissingWitness", "RouteUnavailable", "StaleMembership"])
    );
    assert!(details[0].get("suggested_repair").is_none());
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_duplicate_declared_failure_row() {
    let source = r#"
module Surface.Elab.SetInsertionGuardDuplicateFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0]["failure_row_context"]["declared_failures"],
        serde_json::json!(["MissingCapability", "MissingCapability"])
    );
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!(["MissingWitness", "RouteUnavailable", "StaleMembership"])
    );
    assert!(details[0].get("suggested_repair").is_none());
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_multiple_generated_requests_in_one_row() {
    let source = r#"
module Surface.Elab.SetInsertionGuardMultipleRequests

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant, other: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
      player[other].hp = 2
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(report.core_ir.remote_requests.len(), 2);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec![
            "generated_failure_not_declared",
            "generated_failure_not_declared"
        ]
    );
    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    assert_eq!(details.len(), 2);
    for detail in details {
        assert_eq!(detail["canon_id"], "E-ROW-001");
        assert_eq!(
            detail["failure_row_context"]["target_ref"],
            "when_fails_row|locus=role:BrowserClient|event=attack"
        );
        assert_eq!(
            detail["missing_evidence"],
            serde_json::json!(["MissingWitness", "RouteUnavailable", "StaleMembership"])
        );
        assert!(detail.get("suggested_repair").is_none());
    }
}

#[test]
fn elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows() {
    let source = r#"
module Surface.Elab.SetInsertionGuardDistinctSameEventRows

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(first: Participant) fails MissingCapability {
    S {
      player[first].hp = 1
    }
  }

  when attack(second: Participant) fails MissingCapability {
    S {
      player[second].hp = 2
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(report.core_ir.remote_requests.len(), 2);
    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    assert_eq!(details.len(), 2);
    for detail in details {
        assert_eq!(
            detail["failure_row_context"]["target_ref"],
            "when_fails_row|locus=role:BrowserClient|event=attack"
        );
        assert_eq!(
            detail["suggested_repair"][0]["repair_shape"],
            "set_insertion"
        );
    }
}

#[test]
fn elab07_set_insertion_is_not_emitted_when_failure_row_would_need_creation() {
    let source = r#"
module Surface.Elab.SetInsertionGuardRowCreation

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0]["failure_row_context"]["declared_failures"],
        serde_json::json!([])
    );
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!([
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
            "StaleMembership"
        ])
    );
    assert!(details[0].get("suggested_repair").is_none());
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_event_retargeting() {
    let source = r#"
module Surface.Elab.SetInsertionGuardEventRetargeting

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when heal(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0]["failure_row_context"]["target_ref"],
        "when_fails_row|locus=role:BrowserClient|event=heal"
    );
    assert_no_set_insertion_repair(&details[0]);
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_role_retargeting() {
    let source = r#"
module Surface.Elab.SetInsertionGuardRoleRetargeting

role AdminClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

AdminClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(
        details[0]["failure_row_context"]["target_ref"],
        "when_fails_row|locus=role:AdminClient|event=attack"
    );
    assert_no_set_insertion_repair(&details[0]);
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_state_field_retargeting() {
    let source = r#"
module Surface.Elab.SetInsertionGuardStateFieldRetargeting

role BrowserClient
place S

record Player {
  score: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].score = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["request_context"]["state_name"], "player");
    assert_eq!(details[0]["request_context"]["field_name"], "score");
    assert_no_set_insertion_repair(&details[0]);
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_owner_locus_retargeting() {
    let source = r#"
module Surface.Elab.SetInsertionGuardOwnerLocusRetargeting

role BrowserClient
place S
place T

record Player {
  hp: Int64,
}

T {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    T {
      player[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["request_context"]["owner_locus"], "T");
    assert_eq!(details[0]["request_context"]["state_name"], "player");
    assert_eq!(details[0]["request_context"]["field_name"], "hp");
    assert_no_set_insertion_repair(&details[0]);
}

#[test]
fn elab07_set_insertion_is_not_emitted_for_state_name_retargeting() {
    let source = r#"
module Surface.Elab.SetInsertionGuardStateNameRetargeting

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state enemy[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      enemy[target].hp = 1
    }
  }
}
"#;

    let details = rejected_lab_details_for_source(source);

    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["request_context"]["owner_locus"], "S");
    assert_eq!(details[0]["request_context"]["state_name"], "enemy");
    assert_eq!(details[0]["request_context"]["field_name"], "hp");
    assert_no_set_insertion_repair(&details[0]);
}

#[test]
fn emits_non_visibility_singleton_erow001_repair_payload() {
    let source = r#"
module Surface.Elab.NonVisibilitySingletonFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, RouteUnavailable, StaleMembership {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["generated_failure_not_declared"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert!(!report.core_ir.remote_requests[0].failure_row_complete);

    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["legacy_code"], "generated_failure_not_declared");
    assert_eq!(details[0]["canon_id"], "E-ROW-001");
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!(["MissingWitness"])
    );
    assert_eq!(
        details[0]["failure_row_context"],
        serde_json::json!({
            "target_kind": "when_fails_row",
            "target_ref": "when_fails_row|locus=role:BrowserClient|event=attack",
            "target_locus": "role:BrowserClient",
            "event_name": "attack",
            "required_failures": [
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership"
            ],
            "declared_failures": [
                "MissingCapability",
                "RouteUnavailable",
                "StaleMembership"
            ],
            "missing_failures": [
                "MissingWitness"
            ],
            "local_premise": "generated_failures_subset_declared_fails"
        })
    );
    let repair = &details[0]["suggested_repair"][0];
    assert_no_placeholder_repair_values(repair);
    assert_eq!(repair["repair_family"], "add-to-fails-row");
    assert_eq!(repair["diagnostic_family"], "E-ROW-001");
    assert_eq!(
        repair["applies_to"],
        serde_json::json!({
            "legacy_code": "generated_failure_not_declared",
            "canon_id": "E-ROW-001",
            "request_id": "req-0001"
        })
    );
    assert_eq!(repair["target_kind"], "when_fails_row");
    assert_eq!(
        repair["target_context"],
        serde_json::json!({
            "target_ref": "when_fails_row|locus=role:BrowserClient|event=attack",
            "locus": "role:BrowserClient",
            "event_name": "attack"
        })
    );
    assert_eq!(repair["missing_failure"], "MissingWitness");
    assert_eq!(
        repair["required_failures"],
        serde_json::json!([
            "MissingCapability",
            "MissingWitness",
            "RouteUnavailable",
            "StaleMembership"
        ])
    );
    assert_eq!(
        repair["declared_failures"],
        serde_json::json!(["MissingCapability", "RouteUnavailable", "StaleMembership"])
    );
    assert_eq!(
        repair["local_effect"]["declared_failures_after"],
        serde_json::json!([
            "MissingCapability",
            "RouteUnavailable",
            "StaleMembership",
            "MissingWitness"
        ])
    );
    assert_eq!(
        repair["local_premise"],
        "generated_failures_subset_declared_fails"
    );
    assert_eq!(
        repair["single_edit_assumption"],
        "erow001_non_visibility_singleton_row_addition_only"
    );
    assert_eq!(
        repair["non_goal"],
        "does_not_authorize_capability_witness_route_membership_or_claim_runtime_success"
    );
    assert_eq!(repair["repair_non_final"], true);
    assert_eq!(repair["lab_non_final"], true);
    assert_obl024_diagnostic_soundness_projection(&details[0]);
}

#[test]
fn sample_fixtures_cover_each_non_visibility_singleton_with_repair_payload() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let cases = [
        (
            "samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/main/src/non-visibility-singleton-failure-row-negative.mir",
            "MissingWitness",
            serde_json::json!(["MissingCapability", "RouteUnavailable", "StaleMembership"]),
        ),
        (
            "samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/main/src/missing-capability-singleton-failure-row-negative.mir",
            "MissingCapability",
            serde_json::json!(["MissingWitness", "RouteUnavailable", "StaleMembership"]),
        ),
        (
            "samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/main/src/route-unavailable-singleton-failure-row-negative.mir",
            "RouteUnavailable",
            serde_json::json!(["MissingCapability", "MissingWitness", "StaleMembership"]),
        ),
        (
            "samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/main/src/stale-membership-singleton-failure-row-negative.mir",
            "StaleMembership",
            serde_json::json!(["MissingCapability", "MissingWitness", "RouteUnavailable"]),
        ),
    ];

    for (path, missing_failure, declared_failures) in cases {
        let report = elaborate_surface_to_core_path(repo_root.join(path));
        let report_json = serde_json::to_value(&report).expect("report serializes");
        let details = report_json["lab_diagnostic_details"]
            .as_array()
            .expect("LAB diagnostic details are emitted");

        assert!(!report.accepted, "{path} should be rejected");
        assert_eq!(
            surface_elaboration_diagnostic_codes(&report),
            vec!["generated_failure_not_declared"],
            "{path}"
        );
        assert_eq!(details.len(), 1, "{path}");
        assert_eq!(details[0]["canon_id"], "E-ROW-001", "{path}");
        assert_eq!(
            details[0]["missing_evidence"],
            serde_json::json!([missing_failure]),
            "{path}"
        );
        assert_eq!(
            details[0]["failure_row_context"]["declared_failures"], declared_failures,
            "{path}"
        );
        assert_eq!(
            details[0]["failure_row_context"]["missing_failures"],
            serde_json::json!([missing_failure]),
            "{path}"
        );
        assert_eq!(
            details[0]["request_context"]["request_id"],
            serde_json::json!("req-0001"),
            "{path}"
        );
        assert_eq!(
            details[0]["failure_row_context"]["target_kind"],
            serde_json::json!("when_fails_row"),
            "{path}"
        );
        assert!(
            details[0]["failure_row_context"]["target_ref"]
                .as_str()
                .expect("target_ref is a string")
                .starts_with("when_fails_row|"),
            "{path}"
        );
        assert_no_placeholder_repair_values(&details[0]["failure_row_context"]);
        let repair = &details[0]["suggested_repair"][0];
        assert_no_placeholder_repair_values(repair);
        assert_eq!(repair["repair_family"], "add-to-fails-row", "{path}");
        assert_eq!(repair["diagnostic_family"], "E-ROW-001", "{path}");
        assert_eq!(
            repair["applies_to"],
            serde_json::json!({
                "legacy_code": "generated_failure_not_declared",
                "canon_id": "E-ROW-001",
                "request_id": "req-0001"
            }),
            "{path}"
        );
        assert_eq!(repair["target_kind"], "when_fails_row", "{path}");
        assert_eq!(
            repair["target_context"],
            serde_json::json!({
                "target_ref": "when_fails_row|locus=role:BrowserClient|event=attack",
                "locus": "role:BrowserClient",
                "event_name": "attack"
            }),
            "{path}"
        );
        assert_eq!(
            repair["missing_failure"],
            serde_json::json!(missing_failure),
            "{path}"
        );
        assert_eq!(
            repair["required_failures"],
            serde_json::json!([
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership"
            ]),
            "{path}"
        );
        assert_eq!(repair["declared_failures"], declared_failures, "{path}");
        let mut declared_failures_after = repair["declared_failures"]
            .as_array()
            .expect("declared_failures is an array")
            .clone();
        declared_failures_after.push(repair["missing_failure"].clone());
        assert_eq!(
            repair["local_effect"]["declared_failures_after"],
            Value::Array(declared_failures_after),
            "{path}"
        );
        assert_eq!(
            repair["local_premise"], "generated_failures_subset_declared_fails",
            "{path}"
        );
        assert_eq!(
            repair["single_edit_assumption"], "erow001_non_visibility_singleton_row_addition_only",
            "{path}"
        );
        assert_eq!(
            repair["non_goal"],
            "does_not_authorize_capability_witness_route_membership_or_claim_runtime_success",
            "{path}"
        );
        assert_eq!(repair["repair_non_final"], true, "{path}");
        assert_eq!(repair["lab_non_final"], true, "{path}");
        assert_obl024_diagnostic_soundness_projection(&details[0]);
    }
}

#[test]
fn rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail() {
    let source = r#"
module Surface.Elab.VisibilityOnlyFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["generated_failure_not_declared"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert!(!report.core_ir.remote_requests[0].failure_row_complete);

    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["legacy_code"], "generated_failure_not_declared");
    assert_eq!(details[0]["canon_id"], "E-ROW-002");
    assert_eq!(
        details[0]["missing_evidence"],
        serde_json::json!(["VisibilityDenied"])
    );
    assert_eq!(
        details[0]["request_context"],
        serde_json::json!({
            "request_id": "req-0001",
            "request_kind": "read",
            "generated_from": "cross_locus_read_expression",
            "requester_locus": "role:BrowserClient",
            "owner_locus": "S",
            "state_name": "player",
            "key_expr": "self",
            "field_name": "hp"
        })
    );
    assert_eq!(
        details[0]["failure_row_context"],
        serde_json::json!({
            "target_kind": "when_fails_row",
            "target_ref": "when_fails_row|locus=role:BrowserClient|event=render",
            "target_locus": "role:BrowserClient",
            "event_name": "render",
            "required_failures": [
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership",
                "VisibilityDenied"
            ],
            "declared_failures": [
                "MissingCapability",
                "MissingWitness",
                "RouteUnavailable",
                "StaleMembership"
            ],
            "missing_failures": [
                "VisibilityDenied"
            ],
            "local_premise": "generated_failures_subset_declared_fails"
        })
    );
    assert_eq!(
        details[0]["suggested_repair"],
        serde_json::json!([
            {
                "repair_family": "add-to-fails-row",
                "diagnostic_family": "E-ROW-002",
                "applies_to": {
                    "legacy_code": "generated_failure_not_declared",
                    "canon_id": "E-ROW-002",
                    "request_id": "req-0001"
                },
                "target_kind": "when_fails_row",
                "target_context": {
                    "target_ref": "when_fails_row|locus=role:BrowserClient|event=render",
                    "locus": "role:BrowserClient",
                    "event_name": "render"
                },
                "missing_failure": "VisibilityDenied",
                "required_failures": [
                    "MissingCapability",
                    "MissingWitness",
                    "RouteUnavailable",
                    "StaleMembership",
                    "VisibilityDenied"
                ],
                "declared_failures": [
                    "MissingCapability",
                    "MissingWitness",
                    "RouteUnavailable",
                    "StaleMembership"
                ],
                "local_effect": {
                    "declared_failures_after": [
                        "MissingCapability",
                        "MissingWitness",
                        "RouteUnavailable",
                        "StaleMembership",
                        "VisibilityDenied"
                    ]
                },
                "local_premise": "generated_failures_subset_declared_fails",
                "single_edit_assumption": "erow002_visibility_single_row_addition_only",
                "non_goal": "does_not_authorize_visibility_or_claim_runtime_success",
                "repair_non_final": true,
                "lab_non_final": true
            }
        ])
    );
    assert_obl024_diagnostic_soundness_projection(&details[0]);
}

#[test]
fn elab10_sample_fixture_carries_obl024_projection_with_visibility_repair() {
    let details = rejected_lab_details_for_sample(
        "samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/main/src/visibility-failure-row-negative.mir",
    );

    assert_eq!(details.len(), 1);
    assert_eq!(details[0]["canon_id"], "E-ROW-002");
    assert_obl024_diagnostic_soundness_projection(&details[0]);
    let repairs = details[0]["suggested_repair"]
        .as_array()
        .expect("ELAB-10 sample emits one LAB visibility repair item");
    assert_eq!(repairs.len(), 1);
    assert_eq!(repairs[0]["repair_family"], "add-to-fails-row");
    assert_eq!(repairs[0]["diagnostic_family"], "E-ROW-002");
    assert_eq!(repairs[0]["missing_failure"], "VisibilityDenied");
}

#[test]
fn suggested_repair_payloads_are_non_placeholder_local_witnesses() {
    let source = r#"
module Surface.Elab.VisibilityRepairPayload

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);
    let report_json = serde_json::to_value(&report).expect("report serializes");
    let details = report_json["lab_diagnostic_details"]
        .as_array()
        .expect("LAB diagnostic details are emitted");
    let repair = &details[0]["suggested_repair"][0];

    assert!(!report.accepted);
    assert_no_placeholder_repair_values(repair);
    assert_eq!(repair["diagnostic_family"], details[0]["canon_id"]);
    assert_eq!(
        repair["applies_to"]["legacy_code"],
        details[0]["legacy_code"]
    );
    assert_eq!(repair["applies_to"]["canon_id"], details[0]["canon_id"]);
    assert_eq!(
        repair["applies_to"]["request_id"],
        details[0]["request_context"]["request_id"]
    );
    assert_eq!(
        repair["target_kind"],
        details[0]["failure_row_context"]["target_kind"]
    );
    assert_eq!(
        repair["target_context"]["target_ref"],
        details[0]["failure_row_context"]["target_ref"]
    );
    assert_eq!(
        repair["target_context"]["locus"],
        details[0]["failure_row_context"]["target_locus"]
    );
    assert_eq!(
        repair["target_context"]["event_name"],
        details[0]["failure_row_context"]["event_name"]
    );
    assert_eq!(repair["missing_failure"], details[0]["missing_evidence"][0]);
    assert_eq!(
        details[0]["failure_row_context"]["missing_failures"],
        serde_json::json!([repair["missing_failure"].clone()])
    );
    assert_eq!(
        repair["required_failures"],
        details[0]["failure_row_context"]["required_failures"]
    );
    assert_eq!(
        repair["declared_failures"],
        details[0]["failure_row_context"]["declared_failures"]
    );
    let mut declared_failures_after = repair["declared_failures"]
        .as_array()
        .expect("declared_failures is an array")
        .clone();
    declared_failures_after.push(repair["missing_failure"].clone());
    assert_eq!(
        repair["local_effect"]["declared_failures_after"],
        Value::Array(declared_failures_after)
    );
    assert_eq!(
        repair["local_premise"],
        details[0]["failure_row_context"]["local_premise"]
    );
    assert!(
        repair["single_edit_assumption"]
            .as_str()
            .expect("single_edit_assumption is a string")
            .contains("single_row_addition")
    );
    assert!(
        repair["non_goal"]
            .as_str()
            .expect("non_goal is a string")
            .contains("does_not")
    );
    assert_eq!(repair["repair_non_final"], true);
    assert_eq!(repair["lab_non_final"], true);
}

#[test]
fn elaborates_nested_place_read_as_owner_directed_request() {
    let source = r#"
module Surface.Elab.NestedRead

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    S {
      seen_hp = player[self].hp
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let request = &report.core_ir.remote_requests[0];
    assert_eq!(request.request_kind, "read");
    assert_eq!(request.generated_from, "nested_place_block");
    assert_eq!(request.owner_locus, "S");
}

#[test]
fn rejects_unsupported_surface_statements_instead_of_dropping_them() {
    let source = r#"
module Surface.Elab.UnsupportedPublish

role BrowserClient
place World
place WorldAdmission

BrowserClient[self] {
  when start {
    publish World.last_message
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["unsupported_surface_statement_for_elaboration"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 0);
}

#[test]
fn represents_join_as_admission_transition_after_role_admission_floor() {
    let source = r#"
module Surface.Elab.JoinTransition

role BrowserClient
place World
place WorldAdmission

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(report.core_ir.remote_requests.is_empty());
    assert!(
        report
            .core_ir
            .transitions
            .iter()
            .any(|transition| transition.kind == "surface_role_join_admission")
    );
}

#[test]
fn generated_core_ir_carries_source_spans_for_transitions_and_requests() {
    let source = r#"
module Surface.Elab.SourceSpans

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.core_ir.source_spans.is_empty());
    assert!(
        report
            .core_ir
            .source_spans
            .iter()
            .all(|row| row.span.end > row.span.start)
    );
    assert!(
        report
            .accepted_obligations
            .iter()
            .any(|obligation| obligation.code == "surface_core_source_spans_preserved")
    );
}
