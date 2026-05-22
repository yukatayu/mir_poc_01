use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use mir_semantics::full_system_v1::check_textual_mir_module_path;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn write_module(root: &Path, relative_path: &str, source: &str) -> PathBuf {
    let path = root.join(relative_path);
    fs::create_dir_all(path.parent().expect("module path should have parent"))
        .expect("parent directory should be created");
    fs::write(&path, source).expect("module source should be written");
    path
}

fn diagnostics(report: &mir_semantics::full_system_v1::FullSystemV1CheckReport) -> Vec<&str> {
    report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect()
}

#[test]
fn typed_ir_checker_accepts_pure_and_host_boundary_modules() {
    let root = unique_temp_dir("mir-full-system-v1-check-accept");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");

    write_module(
        &root,
        "add-one-positive/src/add-one.mir",
        r#"module Computational.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}
"#,
    );
    let host_boundary = write_module(
        &root,
        "host-boundary-positive/src/host-boundary-add-one.mir",
        r#"module Computational.HostIoAddOne

import Computational.AddOne

capability HostRead
capability HostWrite

effect read_int {
  requires HostRead
  output x: Int64
  failure AdapterUnavailable
}

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition main at ComputationalHostPlace requires HostRead, HostWrite {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
    ensure y = x + 1
}
"#,
    );

    let report = check_textual_mir_module_path(host_boundary);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.final_public_api_frozen);
    assert!(report.module.is_some());
    assert!(
        report
            .accepted_obligations
            .iter()
            .any(|row| row.code == "imports_resolved")
    );
    assert!(
        report
            .accepted_obligations
            .iter()
            .any(|row| row.code == "effect_failure_rows_explicit")
    );
    assert!(
        report
            .residual_obligations
            .iter()
            .any(|row| row.code == "ambient_effect_row_containment_not_modeled")
    );
}

#[test]
fn typed_ir_checker_rejects_unresolved_import() {
    let root = unique_temp_dir("mir-full-system-v1-check-import");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_module(
        &root,
        "broken-import/src/unresolved-import.mir",
        r#"module Broken.Import

import Missing.Module

fn add_one(x: Int64) -> Int64 {
  return x
}
"#,
    );

    let report = check_textual_mir_module_path(source);

    assert!(!report.accepted);
    assert_eq!(diagnostics(&report), vec!["unresolved_import"]);
}

#[test]
fn typed_ir_checker_rejects_type_scope_array_effect_failure_and_capability_rows() {
    let root = unique_temp_dir("mir-full-system-v1-check-reject");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");

    let type_mismatch = write_module(
        &root,
        "type-mismatch-negative/src/type-mismatch.mir",
        r#"module Computational.TypeMismatch

fn bad(x: Int64) -> Int64 {
  return true
}
"#,
    );
    let scope_unbound = write_module(
        &root,
        "scope-unbound-negative/src/scope-unbound.mir",
        r#"module Computational.ScopeNegative

fn bad(x: Int64) -> Int64 {
  let y: Int64 = z
  return y
}
"#,
    );
    let array_bounds = write_module(
        &root,
        "static-array-bounds-negative/src/static-array-bounds.mir",
        r#"module Computational.ArrayBoundsNegative

fn second(x: Int64) -> Int64 {
  let xs: [Int64; 1] = [x]
  return xs[1]
}
"#,
    );
    let undeclared_effect = write_module(
        &root,
        "undeclared-effect-negative/src/undeclared-effect.mir",
        r#"module Computational.EffectNegative

transition main at HostPlace {
  perform read_int via host_input
}
"#,
    );
    let missing_failure = write_module(
        &root,
        "effect-failure-missing-negative/src/effect-failure-missing.mir",
        r#"module Computational.FailureNegative

capability HostWrite

effect write_int(y: Int64) {
  requires HostWrite
}

transition main at HostPlace requires HostWrite {
  perform write_int(1) via host_output
}
"#,
    );
    let undeclared_capability = write_module(
        &root,
        "undeclared-capability-negative/src/undeclared-capability.mir",
        r#"module Computational.CapabilityNegative

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition main at HostPlace {
  perform write_int(1) via host_output
}
"#,
    );

    let cases = [
        (type_mismatch, "return_type_mismatch"),
        (scope_unbound, "unbound_variable"),
        (array_bounds, "static_index_out_of_bounds"),
        (undeclared_effect, "effect_not_declared"),
        (missing_failure, "effect_failure_row_missing"),
        (undeclared_capability, "capability_not_declared"),
    ];

    for (path, expected_code) in cases {
        let report = check_textual_mir_module_path(path);
        assert!(!report.accepted, "expected failure for {expected_code}");
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == expected_code),
            "expected diagnostic `{expected_code}`, got {:?}",
            diagnostics(&report)
        );
    }
}

#[test]
fn typed_ir_checker_rejects_semantically_broken_imported_modules() {
    let root = unique_temp_dir("mir-full-system-v1-check-imported-bad");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");

    let source = write_module(
        &root,
        "main/src/imported-semantic-negative.mir",
        r#"module Computational.ImportedSemanticNegative

import Shared.BadFunction
import Shared.BadEffect

capability HostWrite

fn use_imported(x: Int64) -> Int64 {
  return imported_bad(x)
}

transition main at HostPlace requires HostWrite {
  perform write_shared(1) via host_output
}
"#,
    );
    write_module(
        &root,
        "shared/src/bad-function.mir",
        r#"module Shared.BadFunction

fn imported_bad(x: Int64) -> Int64 {
  return true
}
"#,
    );
    write_module(
        &root,
        "shared/src/bad-effect.mir",
        r#"module Shared.BadEffect

capability HostWrite

effect write_shared(y: Int64) {
  requires HostWrite
}
"#,
    );

    let report = check_textual_mir_module_path(source);

    assert!(!report.accepted);
    assert!(diagnostics(&report).contains(&"return_type_mismatch"));
    assert!(diagnostics(&report).contains(&"effect_failure_row_missing"));
}

#[test]
fn typed_ir_checker_rejects_ambiguous_import_resolution() {
    let root = unique_temp_dir("mir-full-system-v1-check-ambiguous-import");
    fs::create_dir_all(&root).expect("temp root should be created");
    fs::write(root.join("matrix.json"), "{}").expect("matrix marker should be written");
    let source = write_module(
        &root,
        "main/src/duplicate-import.mir",
        r#"module Main.Test

import Shared.Dup

fn use_dup(x: Int64) -> Int64 {
  return dup(x)
}
"#,
    );
    write_module(
        &root,
        "dup-a/src/dup-a.mir",
        r#"module Shared.Dup

fn dup(x: Int64) -> Int64 {
  return x
}
"#,
    );
    write_module(
        &root,
        "dup-b/src/dup-b.mir",
        r#"module Shared.Dup

fn dup(x: Int64) -> Int64 {
  return x + 1
}
"#,
    );

    let report = check_textual_mir_module_path(source);

    assert!(!report.accepted);
    assert!(diagnostics(&report).contains(&"ambiguous_import_resolution"));
}
