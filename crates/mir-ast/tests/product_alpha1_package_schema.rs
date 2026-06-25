use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_ast::product_alpha1::{
    ProductAlpha1ErrorKind, check_product_alpha1_package, check_product_alpha1_package_path,
    load_product_alpha1_package_path, parse_product_alpha1_package_text,
};

const MINIMAL_PRODUCT_PACKAGE: &str = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "product-alpha1-demo",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": ["typed_host_io.add_one"],
  "failures": ["AdapterUnavailable"],
  "capabilities": ["RollDice", "PublishRoll"],
  "witness_requirements": ["game_started_witness"],
  "membership_requirements": ["active_participant"],
  "auth_policy": {
    "policy_id": "demo-auth-policy",
    "required_bindings": ["participant_membership"]
  },
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {
      "contract_id": "demo-world-contract",
      "variance": "invariant",
      "effect_row": ["typed_host_io.add_one"],
      "failure_row": ["AdapterUnavailable"]
    }
  ],
  "observation_policy": {
    "view_role": "observer_safe",
    "labels": ["observer_safe_summary"]
  },
  "redaction_policy": {
    "level": "observer_safe",
    "redacted_fields": ["raw_witness_payload", "raw_auth_evidence"]
  },
  "retention_policy": {
    "scope": "demo_session",
    "retained_artifacts": ["checker_report", "runtime_plan"]
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
    "entry_place": "Place[ProductDemoRoom]",
    "host_io": {
      "adapter_kind": "AddOne",
      "effect_ref": "typed_host_io.add_one",
      "request_payload": {"kind": "int", "value": 41},
      "expected_response": {"kind": "int", "value": 42}
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

const MINIMAL_ECHO_TEXT_PRODUCT_PACKAGE: &str = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "product-alpha1-echo-text",
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
      "request_payload": {"kind": "text", "value": "Taro"},
      "expected_response": {"kind": "text", "value": "Hello, Taro!"}
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

const MINIMAL_CHAT_TEXT_PRODUCT_PACKAGE: &str = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "product-alpha1-chat-text",
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
      "request_payload": {"kind": "text", "value": "hello room"},
      "expected_response": {"kind": "text", "value": "room#lobby message accepted: hello room"}
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

const MINIMAL_COMPUTATIONAL_ADD_ONE_PACKAGE: &str = r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "computational-add-one-pure-mir",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": ["typed_host_io.read_int", "typed_host_io.write_int"],
  "failures": ["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
  "capabilities": ["RunComputationalAddOne"],
  "witness_requirements": [],
  "membership_requirements": ["active_participant"],
  "auth_policy": {
    "policy_id": "computational-add-one-auth-policy",
    "required_bindings": ["participant_membership"]
  },
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [
    {
      "contract_id": "computational-add-one-contract",
      "variance": "invariant",
      "effect_row": ["typed_host_io.read_int", "typed_host_io.write_int"],
      "failure_row": ["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"]
    }
  ],
  "observation_policy": {
    "view_role": "observer_safe",
    "labels": ["observer_safe_compute_summary"]
  },
  "redaction_policy": {
    "level": "observer_safe",
    "redacted_fields": ["raw_auth_evidence"]
  },
  "retention_policy": {
    "scope": "computational_session",
    "retained_artifacts": ["checker_report", "runtime_plan", "compute_trace"]
  },
  "message_recovery_policy": {
    "handled_failures": ["reject"],
    "recovery": "reject"
  },
  "savepoint_policy": {
    "classes": ["R0", "R2"],
    "quiescent_required": true
  },
  "runtime_input": {
    "entry_place": "Place[ComputationalHostPlace]",
    "host_input": {
      "adapter_kind": "ReadInt",
      "effect_ref": "typed_host_io.read_int",
      "request_payload": {"kind": "int", "value": 41},
      "expected_response": {"kind": "int", "value": 41}
    },
    "mir_compute": {
      "module_id": "Computational.AddOne",
      "function_id": "add_one",
      "input_type": "Int64",
      "output_type": "Int64",
      "required_capabilities": ["RunComputationalAddOne"],
      "failure_tag": "MirComputeRejected",
      "expected_output": {"kind": "int", "value": 42}
    },
    "host_output": {
      "adapter_kind": "WriteInt",
      "effect_ref": "typed_host_io.write_int",
      "request_payload": {"kind": "int", "value": 42},
      "expected_response": {"kind": "int", "value": 42}
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

fn computational_product_package(
    package_id: &str,
    module_id: &str,
    function_id: &str,
    request_value: i64,
    expected_output: i64,
) -> String {
    computational_product_package_with_boundary(
        package_id,
        module_id,
        function_id,
        request_value,
        expected_output,
        &["typed_host_io.read_int", "typed_host_io.write_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalRow"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["reject"],
        "\"required_capabilities\": [\"RunComputationalRow\"],\n      \"failure_tag\": \"MirComputeRejected\",\n      ",
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

#[allow(clippy::too_many_arguments)]
fn computational_product_package_with_boundary(
    package_id: &str,
    module_id: &str,
    function_id: &str,
    request_value: i64,
    expected_output: i64,
    effects: &[&str],
    failures: &[&str],
    capabilities: &[&str],
    contract_failures: &[&str],
    handled_failures: &[&str],
    mir_compute_extra_fields: &str,
) -> String {
    format!(
        r#"{{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "{package_id}",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": {effects},
  "failures": {failures},
  "capabilities": {capabilities},
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
      "failure_row": {contract_failures}
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
    "handled_failures": {handled_failures},
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
      {mir_compute_extra_fields}
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
}}"#,
        effects = json_array(effects),
        failures = json_array(failures),
        capabilities = json_array(capabilities),
        contract_failures = json_array(contract_failures),
        handled_failures = json_array(handled_failures),
    )
}

const MINIMAL_PROJECTION_PROFILE: &str = r#"{
  "projection_profile_version": "ops-product-projection-v0",
  "non_final": true,
  "source_package": "projection-world-core",
  "targets": [
    {
      "target_id": "server",
      "target_kind": "server_host",
      "places": ["WorldServerPlace"],
      "outputs": {
        "native_binary_emitted": false,
        "host_launch_bundle_part": true
      }
    },
    {
      "target_id": "participant-client",
      "target_kind": "client_host",
      "places": ["ParticipantPlace[*]", "ClientViewPlace"],
      "outputs": {
        "native_binary_emitted": false,
        "host_launch_bundle_part": true
      }
    }
  ],
  "packet_boundaries": [
    {
      "name": "roll_request_packet",
      "fields": ["message_id", "payload", "membership_epoch", "witness_refs"]
    },
    {
      "name": "chat_message_packet",
      "fields": ["message_id", "payload", "membership_epoch", "redaction_policy"]
    }
  ],
  "ffi_boundaries": [
    {
      "name": "host_io_adapter",
      "input_schema": "typed_payload",
      "output_schema": "typed_payload",
      "effect_row": ["typed_host_io.add_one"],
      "failure_row": ["AdapterUnavailable"]
    }
  ],
  "backend": {
    "llvm_codegen_claimed": false,
    "direct_mir_to_machine_code_claimed": false,
    "future_backend_requirements_documented": true
  }
}"#;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

#[test]
fn product_alpha1_package_schema_accepts_minimal_world_with_explicit_evidence() {
    let package = parse_product_alpha1_package_text(MINIMAL_PRODUCT_PACKAGE)
        .expect("minimal product alpha package should parse");
    let report =
        check_product_alpha1_package(&package).expect("minimal product alpha package should check");

    assert_eq!(report.surface_kind, "mirrorea_product_alpha1_check_report");
    assert_eq!(report.schema_version, "mirrorea-product-alpha1-v0");
    assert_eq!(report.package_id, "product-alpha1-demo");
    assert_eq!(report.verdict, "accepted");
    assert!(!report.accepted_obligations.is_empty());
    assert!(report.accepted_obligations.iter().any(|row| {
        row.kind == "package_schema_version" && row.evidence == "schema version accepted"
    }));
    assert!(report.residual_obligations.iter().any(|row| {
        row.line == "runtime_preflight" && row.kind == "quiescent_save_runtime_evidence"
    }));
    assert!(!report.product_alpha1_ready);
    assert!(!report.final_public_api_frozen);
}

#[test]
fn product_alpha1_package_schema_accepts_echo_text_host_io_package() {
    let package = parse_product_alpha1_package_text(MINIMAL_ECHO_TEXT_PRODUCT_PACKAGE)
        .expect("EchoText product alpha package should parse");
    let report = check_product_alpha1_package(&package)
        .expect("EchoText product alpha package should check");

    assert_eq!(report.package_kind, "membership_chat");
    assert!(report.accepted_obligations.iter().any(|row| {
        row.kind == "runtime_input_host_io"
            && row.evidence == "typed host-I/O runtime input declaration accepted"
    }));
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_rejects_invalid_echo_text_expected_response() {
    let error = parse_product_alpha1_package_text(
        &MINIMAL_ECHO_TEXT_PRODUCT_PACKAGE.replace("Hello, Taro!", "Goodbye, Taro!"),
    )
    .expect_err("EchoText host-I/O shape should be validated");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("EchoText"));
}

#[test]
fn product_alpha1_package_schema_accepts_chat_text_host_io_package() {
    let package = parse_product_alpha1_package_text(MINIMAL_CHAT_TEXT_PRODUCT_PACKAGE)
        .expect("ChatText product alpha package should parse");
    let report = check_product_alpha1_package(&package)
        .expect("ChatText product alpha package should check");

    assert_eq!(report.package_kind, "membership_chat");
    assert!(report.accepted_obligations.iter().any(|row| {
        row.kind == "runtime_input_host_io"
            && row.evidence == "typed host-I/O runtime input declaration accepted"
    }));
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_rejects_invalid_chat_text_expected_response() {
    let error = parse_product_alpha1_package_text(&MINIMAL_CHAT_TEXT_PRODUCT_PACKAGE.replace(
        "room#lobby message accepted: hello room",
        "room#lobby rejected",
    ))
    .expect_err("ChatText host-I/O shape should be validated");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("ChatText"));
}

#[test]
fn product_alpha1_package_schema_accepts_computational_add_one_package() {
    let package = parse_product_alpha1_package_text(MINIMAL_COMPUTATIONAL_ADD_ONE_PACKAGE)
        .expect("computational add-one package should parse");
    let report =
        check_product_alpha1_package(&package).expect("computational add-one package should check");

    assert_eq!(report.package_id, "computational-add-one-pure-mir");
    assert!(report.accepted_obligations.iter().any(|row| {
        row.kind == "runtime_input_mir_compute"
            && row.evidence == "Mir-owned computational runtime input declaration accepted"
    }));
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_rejects_computational_add_one_missing_mir_compute() {
    let error = parse_product_alpha1_package_text(&MINIMAL_COMPUTATIONAL_ADD_ONE_PACKAGE.replace(
        r#",
    "mir_compute": {
      "module_id": "Computational.AddOne",
      "function_id": "add_one",
      "input_type": "Int64",
      "output_type": "Int64",
      "required_capabilities": ["RunComputationalAddOne"],
      "failure_tag": "MirComputeRejected",
      "expected_output": {"kind": "int", "value": 42}
    }"#,
        "",
    ))
    .expect_err("computational add-one package without mir_compute should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("runtime_input.mir_compute"));
}

#[test]
fn product_alpha1_package_schema_accepts_comp03_registry_module_shapes() {
    let package = parse_product_alpha1_package_text(&computational_product_package(
        "computational-arrays-positive",
        "Computational.Arrays.Positive",
        "second",
        5,
        5,
    ))
    .expect("comp03 arrays positive package should parse");
    let report =
        check_product_alpha1_package(&package).expect("comp03 arrays positive should check");

    assert_eq!(report.package_id, "computational-arrays-positive");
    assert!(report.accepted_obligations.iter().any(|row| {
        row.kind == "runtime_input_mir_compute"
            && row.evidence == "Mir-owned computational runtime input declaration accepted"
    }));
}

#[test]
fn product_alpha1_package_schema_accepts_runtime_reject_rows_for_known_modules() {
    let package = parse_product_alpha1_package_text(&computational_product_package(
        "computational-compose-negative",
        "Computational.Compose.NegativeMissingImport",
        "add_two",
        40,
        42,
    ))
    .expect("known negative computational package should parse");
    let report = check_product_alpha1_package(&package)
        .expect("known negative computational package should check");

    assert_eq!(report.package_id, "computational-compose-negative");
    assert_eq!(report.verdict, "accepted");
}

#[test]
fn product_alpha1_package_schema_accepts_comp04_boundary_contract_positive() {
    let package = parse_product_alpha1_package_text(&computational_product_package_with_boundary(
        "computational-boundary-positive",
        "Computational.Compose.Positive",
        "add_two",
        40,
        42,
        &["typed_host_io.read_int", "typed_host_io.write_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalTransform"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["reject"],
        "\"required_capabilities\": [\"RunComputationalTransform\"],\n      \"failure_tag\": \"MirComputeRejected\",\n      ",
    ))
    .expect("comp04 boundary-positive package should parse");
    let report =
        check_product_alpha1_package(&package).expect("comp04 boundary-positive should check");

    assert_eq!(report.package_id, "computational-boundary-positive");
    assert_eq!(report.verdict, "accepted");
}

#[test]
fn product_alpha1_package_schema_rejects_comp04_undeclared_host_effect() {
    let error = parse_product_alpha1_package_text(&computational_product_package_with_boundary(
        "computational-boundary-undeclared-effect",
        "Computational.Compose.Positive",
        "add_two",
        40,
        42,
        &["typed_host_io.read_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalTransform"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["reject"],
        "\"required_capabilities\": [\"RunComputationalTransform\"],\n      \"failure_tag\": \"MirComputeRejected\",\n      ",
    ))
    .expect_err("undeclared host effect should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("typed_host_io.write_int"));
}

#[test]
fn product_alpha1_package_schema_rejects_comp04_undeclared_failure_row() {
    let error = parse_product_alpha1_package_text(&computational_product_package_with_boundary(
        "computational-boundary-undeclared-failure",
        "Computational.Arrays.NegativeOutOfBounds",
        "second",
        5,
        0,
        &["typed_host_io.read_int", "typed_host_io.write_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalTransform"],
        &["AdapterUnavailable", "TypeMismatch"],
        &["reject"],
        "\"required_capabilities\": [\"RunComputationalTransform\"],\n      \"failure_tag\": \"MirComputeRejected\",\n      ",
    ))
    .expect_err("undeclared computational failure row should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("MirComputeRejected"));
}

#[test]
fn product_alpha1_package_schema_rejects_comp04_missing_required_capability() {
    let error = parse_product_alpha1_package_text(&computational_product_package_with_boundary(
        "computational-boundary-missing-capability",
        "Computational.Compose.Positive",
        "add_two",
        40,
        42,
        &["typed_host_io.read_int", "typed_host_io.write_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalRow"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["reject"],
        "\"required_capabilities\": [\"RunComputationalTransform\"],\n      \"failure_tag\": \"MirComputeRejected\",\n      ",
    ))
    .expect_err("missing required capability should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("RunComputationalTransform"));
}

#[test]
fn product_alpha1_package_schema_rejects_missing_computational_boundary_fields() {
    let error = parse_product_alpha1_package_text(&computational_product_package_with_boundary(
        "computational-boundary-missing-fields",
        "Computational.Compose.Positive",
        "add_two",
        40,
        42,
        &["typed_host_io.read_int", "typed_host_io.write_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalTransform"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["reject"],
        "",
    ))
    .expect_err("missing computational boundary fields should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(
        error.detail.contains("missing field `failure_tag`")
            || error
                .detail
                .contains("runtime_input.mir_compute.required_capabilities")
    );
}

#[test]
fn product_alpha1_package_schema_rejects_failure_tag_on_unrelated_contract() {
    let base = computational_product_package_with_boundary(
        "computational-boundary-unrelated-contract",
        "Computational.Arrays.NegativeOutOfBounds",
        "second",
        5,
        0,
        &["typed_host_io.read_int", "typed_host_io.write_int"],
        &["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
        &["RunComputationalTransform"],
        &["AdapterUnavailable", "TypeMismatch"],
        &["reject"],
        "\"required_capabilities\": [\"RunComputationalTransform\"],\n      \"failure_tag\": \"MirComputeRejected\",\n      ",
    );
    let patched = base.replace(
        r#""contracts": [
    {
      "contract_id": "computational-boundary-unrelated-contract-contract",
      "variance": "invariant",
      "effect_row": ["typed_host_io.read_int", "typed_host_io.write_int"],
      "failure_row": ["AdapterUnavailable", "TypeMismatch"]
    }
  ],"#,
        r#""contracts": [
    {
      "contract_id": "computational-boundary-unrelated-contract-contract",
      "variance": "invariant",
      "effect_row": ["typed_host_io.read_int", "typed_host_io.write_int"],
      "failure_row": ["AdapterUnavailable", "TypeMismatch"]
    },
    {
      "contract_id": "dummy-unrelated-contract",
      "variance": "observe_only",
      "effect_row": [],
      "failure_row": ["MirComputeRejected"]
    }
  ],"#,
    );
    let error = parse_product_alpha1_package_text(&patched)
        .expect_err("failure tag on unrelated contract should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("computational contract"));
}

#[test]
fn product_alpha1_package_schema_rejects_unknown_computational_module() {
    let error = parse_product_alpha1_package_text(&computational_product_package(
        "computational-unknown-module",
        "Computational.Unknown.Module",
        "mystery",
        1,
        1,
    ))
    .expect_err("unknown computational module should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(
        error
            .detail
            .contains("current computational sample registry")
    );
}

#[test]
fn product_alpha1_package_schema_rejects_mixed_legacy_and_computational_runtime_inputs() {
    let mixed = MINIMAL_COMPUTATIONAL_ADD_ONE_PACKAGE.replace(
        r#""host_input": {
      "adapter_kind": "ReadInt",
      "effect_ref": "typed_host_io.read_int",
      "request_payload": {"kind": "int", "value": 41},
      "expected_response": {"kind": "int", "value": 41}
    },"#,
        r#""host_io": {
      "adapter_kind": "AddOne",
      "effect_ref": "typed_host_io.add_one",
      "request_payload": {"kind": "int", "value": 41},
      "expected_response": {"kind": "int", "value": 42}
    },
    "host_input": {
      "adapter_kind": "ReadInt",
      "effect_ref": "typed_host_io.read_int",
      "request_payload": {"kind": "int", "value": 41},
      "expected_response": {"kind": "int", "value": 41}
    },"#,
    )
    .replace(
        r#""effects": ["typed_host_io.read_int", "typed_host_io.write_int"]"#,
        r#""effects": ["typed_host_io.add_one", "typed_host_io.read_int", "typed_host_io.write_int"]"#,
    );
    let error = parse_product_alpha1_package_text(&mixed)
        .expect_err("mixed runtime input shape should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("cannot be mixed"));
}

#[test]
fn product_alpha1_package_schema_rejects_wrong_declared_function_for_known_module() {
    let error = parse_product_alpha1_package_text(&computational_product_package(
        "computational-arrays-wrong-function",
        "Computational.Arrays.Positive",
        "add_one",
        5,
        5,
    ))
    .expect_err("known module with wrong function should reject");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("must equal `second`"));
}

#[test]
fn product_alpha1_package_schema_loads_product_demo_root_fixture() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-ast");
    let package = load_product_alpha1_package_path(repo_root.join("samples/product-alpha1/demo"))
        .expect("product demo package root should load");
    let report = check_product_alpha1_package(&package)
        .expect("product demo package root should pass schema check");

    assert_eq!(report.package_id, "product-alpha1-demo");
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_accepts_operational_sample_suite_roots() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-ast");

    for relative in [
        "samples/product-alpha1/operational/world-core",
        "samples/product-alpha1/operational/membership-chat",
        "samples/product-alpha1/operational/sugoroku-world",
        "samples/product-alpha1/operational/portal-worldlink",
        "samples/product-alpha1/operational/two-shard-hard-boundary",
        "samples/product-alpha1/operational/two-shard-gradient-observation",
    ] {
        let package = load_product_alpha1_package_path(repo_root.join(relative))
            .unwrap_or_else(|_| panic!("operational fixture should load: {relative}"));
        let report = check_product_alpha1_package(&package)
            .unwrap_or_else(|_| panic!("operational fixture should check: {relative}"));

        assert_eq!(report.verdict, "accepted");
        assert!(!report.product_alpha1_ready);
    }
}

#[test]
fn product_alpha1_package_schema_accepts_operational_template_world_core_starter() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-ast");

    let package = load_product_alpha1_package_path(
        repo_root.join("samples/product-alpha1/operational/templates/world-core-starter"),
    )
    .expect("operational authoring starter should load");
    let report = check_product_alpha1_package(&package)
        .expect("operational authoring starter should pass schema check");

    assert_eq!(report.package_id, "operational-world-core-starter");
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_accepts_operational_template_membership_chat_starter() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-ast");

    let package = load_product_alpha1_package_path(
        repo_root.join("samples/product-alpha1/operational/templates/membership-chat-starter"),
    )
    .expect("operational membership-chat starter should load");
    let report = check_product_alpha1_package(&package)
        .expect("operational membership-chat starter should pass schema check");

    assert_eq!(report.package_id, "operational-membership-chat-starter");
    assert_eq!(report.package_kind, "membership_chat");
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_accepts_operational_template_sugoroku_world_starter() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-ast");

    let package = load_product_alpha1_package_path(
        repo_root.join("samples/product-alpha1/operational/templates/sugoroku-world-starter"),
    )
    .expect("operational sugoroku-world starter should load");
    let report = check_product_alpha1_package(&package)
        .expect("operational sugoroku-world starter should pass schema check");

    assert_eq!(report.package_id, "operational-sugoroku-world-starter");
    assert_eq!(report.package_kind, "sugoroku_world");
    assert_eq!(report.verdict, "accepted");
    assert!(!report.product_alpha1_ready);
}

#[test]
fn product_alpha1_package_schema_check_report_includes_operational_projection_inventory() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-ast");

    let report = check_product_alpha1_package_path(
        repo_root.join("samples/product-alpha1/operational/sugoroku-world"),
    )
    .expect("operational sugoroku root should include projection inventory");

    let inventory = report
        .projection_inventory
        .as_ref()
        .expect("projection inventory should be present");
    assert_eq!(inventory.source_package, "operational-sugoroku");
    assert_eq!(inventory.target_count, 2);
    assert_eq!(inventory.packet_boundary_count, 2);
    assert_eq!(inventory.ffi_boundary_count, 1);
    assert_eq!(inventory.packet_boundary_names.len(), 2);
    assert!(
        inventory
            .packet_boundary_names
            .iter()
            .any(|name| name == "roll_request_packet")
    );
    assert!(
        inventory
            .ffi_boundary_names
            .iter()
            .any(|name| name == "host_io_adapter")
    );
}

#[test]
fn product_alpha1_package_schema_rejects_projection_inventory_that_claims_native_binary() {
    let suite_dir = unique_temp_dir("product-alpha1-projection-inventory-test");
    let package_dir = suite_dir.join("world-core");
    fs::create_dir_all(&package_dir).expect("temp package dir should be created");
    fs::create_dir_all(suite_dir.join("deployments/projection"))
        .expect("projection dir should be created");
    fs::write(
        package_dir.join("package.mir.json"),
        MINIMAL_PRODUCT_PACKAGE
            .replace("product-alpha1-demo", "projection-world-core")
            .replace(
                r#""package_kind": "world""#,
                r#""package_kind": "world_core""#,
            ),
    )
    .expect("temp package should be written");
    fs::write(
        suite_dir.join("deployments/projection/projection.profile.json"),
        MINIMAL_PROJECTION_PROFILE.replace(
            r#""native_binary_emitted": false"#,
            r#""native_binary_emitted": true"#,
        ),
    )
    .expect("projection profile should be written");

    let error = check_product_alpha1_package_path(&package_dir)
        .expect_err("native binary claim should be rejected in projection inventory");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("native_binary_emitted"));
}

#[test]
fn product_alpha1_package_schema_rejects_missing_dependency_package() {
    let dir = unique_temp_dir("product-alpha1-missing-dependency-test");
    fs::create_dir_all(&dir).expect("temp package dir should be created");
    fs::write(
        dir.join("package.mir.json"),
        MINIMAL_PRODUCT_PACKAGE.replace(
            r#""dependencies": []"#,
            r#""dependencies": ["packages/missing-layer"]"#,
        ),
    )
    .expect("temp package should be written");

    let error = check_product_alpha1_package_path(&dir)
        .expect_err("missing dependency package should be rejected");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::MissingPackageFile);
    assert!(error.detail.contains("declared dependency"));
}

#[test]
fn product_alpha1_package_schema_rejects_missing_required_policy() {
    let error = parse_product_alpha1_package_text(
        r#"{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "missing-message-policy",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": [],
  "failures": [],
  "capabilities": [],
  "witness_requirements": [],
  "membership_requirements": [],
  "auth_policy": {"policy_id": "auth", "required_bindings": []},
  "auth_stack": [],
  "contracts": [],
  "observation_policy": {"view_role": "observer_safe", "labels": []},
  "redaction_policy": {"level": "observer_safe", "redacted_fields": []},
  "retention_policy": {"scope": "demo_session", "retained_artifacts": []},
  "savepoint_policy": {"classes": ["R0"], "quiescent_required": false},
  "native_policy": {"execution_policy": "disabled", "provenance_required": true},
  "compatibility": {
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }
}"#,
    )
    .expect_err("missing message_recovery_policy should be rejected");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
}

#[test]
fn product_alpha1_package_schema_rejects_missing_required_alpha_field() {
    let error = parse_product_alpha1_package_text(&MINIMAL_PRODUCT_PACKAGE.replace(
        r#",
  "auth_stack": ["membership_auth", "capability_auth"]"#,
        "",
    ))
    .expect_err("missing auth_stack should be rejected");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
}

#[test]
fn product_alpha1_package_schema_rejects_unknown_nested_fields() {
    let error = parse_product_alpha1_package_text(
        &MINIMAL_PRODUCT_PACKAGE.replace("redacted_fields", "redacted_fieldz"),
    )
    .expect_err("unknown nested redaction policy field should be rejected");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
}

#[test]
fn product_alpha1_package_schema_rejects_unknown_contract_variance() {
    let error =
        parse_product_alpha1_package_text(&MINIMAL_PRODUCT_PACKAGE.replace("invariant", "banana"))
            .expect_err("unknown contract variance should be rejected");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("unsupported contract variance"));
}

#[test]
fn product_alpha1_package_schema_rejects_r4_savepoint_policy() {
    let error = parse_product_alpha1_package_text(
        &MINIMAL_PRODUCT_PACKAGE.replace(r#""classes": ["R0", "R2"]"#, r#""classes": ["R4"]"#),
    )
    .expect_err("R4 savepoint policy is not admitted for product alpha-1 schema");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(error.detail.contains("unsupported savepoint class"));
}

#[test]
fn product_alpha1_package_schema_rejects_native_execution_enabled() {
    let error = parse_product_alpha1_package_text(&MINIMAL_PRODUCT_PACKAGE.replace(
        r#""execution_policy": "disabled""#,
        r#""execution_policy": "host_native_execution""#,
    ))
    .expect_err("alpha-1 schema must reject arbitrary native package execution");

    assert_eq!(error.kind, ProductAlpha1ErrorKind::SchemaDecode);
    assert!(
        error
            .detail
            .contains("NativeExecutionPolicy must remain `disabled`")
    );
}
