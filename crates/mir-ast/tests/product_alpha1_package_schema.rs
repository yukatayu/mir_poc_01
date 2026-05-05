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
  "native_policy": {
    "execution_policy": "disabled",
    "provenance_required": true
  },
  "compatibility": {
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
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
