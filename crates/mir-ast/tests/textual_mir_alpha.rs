use mir_ast::textual_alpha::{
    AstExprKind, AstStmt, AstTopLevel, TextualMirDiagnostic, parse_textual_mir_module,
    parse_textual_mir_module_path,
};
use std::path::PathBuf;

fn diagnostic_codes(diagnostics: &[TextualMirDiagnostic]) -> Vec<&str> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect()
}

#[test]
fn parses_pure_add_one_module_with_function_body() {
    let source = r#"
module Computational.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}
"#;

    let module = parse_textual_mir_module(source).expect("add_one source should parse");

    assert_eq!(module.module_path, "Computational.AddOne");
    assert!(module.imports.is_empty());
    assert_eq!(module.items.len(), 1);
    let AstTopLevel::Function(function) = &module.items[0] else {
        panic!("expected function item");
    };
    assert_eq!(function.function_name, "add_one");
    assert_eq!(function.parameter_name, "x");
    assert_eq!(function.body.len(), 2);
    assert!(matches!(function.body[0], AstStmt::Let { .. }));
    assert!(matches!(function.body[1], AstStmt::Return { .. }));
    let AstStmt::Let { value, .. } = &function.body[0] else {
        panic!("expected let statement");
    };
    assert_eq!(value.span.line, 5);
    let AstExprKind::Binary { left, right, .. } = &value.kind else {
        panic!("expected binary expression");
    };
    assert_eq!(left.span.line, 5);
    assert_eq!(right.span.line, 5);
}

#[test]
fn parses_effect_and_transition_surface_for_host_boundary_sample() {
    let source = r#"
module Computational.HostIoAddOne

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
"#;

    let module = parse_textual_mir_module(source).expect("host boundary source should parse");

    assert_eq!(module.imports.len(), 1);
    assert_eq!(module.capabilities.len(), 2);
    assert_eq!(module.effects.len(), 2);
    assert_eq!(module.transitions.len(), 1);
    let transition = &module.transitions[0];
    assert_eq!(transition.transition_name, "main");
    assert_eq!(transition.place_ref, "ComputationalHostPlace");
    assert_eq!(
        transition.required_capabilities,
        vec!["HostRead", "HostWrite"]
    );
    assert_eq!(transition.body.len(), 3);
}

#[test]
fn malformed_function_signature_reports_source_spanned_diagnostic() {
    let source = r#"
module Broken.Function

fn add_one(x Int64) -> Int64 {
  return x
}
"#;

    let diagnostics =
        parse_textual_mir_module(source).expect_err("missing colon should reject parsing");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "expected_colon"
            && diagnostic.span.line == 4
            && diagnostic.span.column > 0
    }));
}

#[test]
fn malformed_perform_boundary_reports_specific_diagnostic() {
    let source = r#"
module Broken.Perform

effect read_int {
  output x: Int64
  failure AdapterUnavailable
}

transition main at HostPlace {
  x <- perform read_int host_input
}
"#;

    let diagnostics =
        parse_textual_mir_module(source).expect_err("missing via should reject parsing");

    assert_eq!(
        diagnostic_codes(&diagnostics),
        vec!["expected_via_after_perform"]
    );
}

#[test]
fn unresolved_import_is_rejected_in_path_aware_parse() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir");

    let diagnostics = parse_textual_mir_module_path(path)
        .expect_err("missing import should reject path-aware parsing");

    assert_eq!(diagnostic_codes(&diagnostics), vec!["unresolved_import"]);
    assert_eq!(diagnostics[0].span.line, 3);
}
