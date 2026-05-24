use std::{env, path::PathBuf, process};

use mir_ast::surface_alpha::parse_surface_mir_report_path;
use serde_json::json;

fn main() {
    let code = run(env::args().skip(1).collect());
    process::exit(code);
}

fn run(args: Vec<String>) -> i32 {
    let (format, args) = strip_format(args);
    if !matches!(format.as_str(), "json" | "pretty") {
        print_payload(
            "json",
            json!({
                "status": "error",
                "diagnostic_code": "unsupported_format",
                "implemented": true,
                "final_public_grammar_frozen": false
            }),
        );
        return 2;
    }

    let Some(path) = args.first() else {
        print_payload(
            &format,
            json!({
                "status": "error",
                "diagnostic_code": "missing_source_path",
                "implemented": true,
                "final_public_grammar_frozen": false
            }),
        );
        return 2;
    };

    if args.len() != 1 {
        print_payload(
            &format,
            json!({
                "status": "error",
                "diagnostic_code": "unexpected_arguments",
                "implemented": true,
                "final_public_grammar_frozen": false
            }),
        );
        return 2;
    }

    let path = PathBuf::from(path);
    let report = parse_surface_mir_report_path(&path);
    let payload = json!({
        "surface_kind": "surface_mir_alpha_parse_report",
        "source_path": path.display().to_string(),
        "accepted": report.accepted,
        "module": report.module,
        "diagnostics": report.diagnostics,
        "canonical_place_scope_syntax": report.canonical_place_scope_syntax,
        "implemented": true,
        "final_public_grammar_frozen": report.final_public_grammar_frozen,
    });

    let accepted = payload["accepted"].as_bool().unwrap_or(false);
    print_payload(&format, payload);
    if accepted { 0 } else { 2 }
}

fn strip_format(args: Vec<String>) -> (String, Vec<String>) {
    let mut format = "pretty".to_string();
    let mut stripped = Vec::new();
    let mut index = 0;
    while index < args.len() {
        if args[index] == "--format" {
            if let Some(value) = args.get(index + 1) {
                format = value.clone();
                index += 2;
                continue;
            }
        }
        stripped.push(args[index].clone());
        index += 1;
    }
    (format, stripped)
}

fn print_payload(format: &str, payload: serde_json::Value) {
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("payload should serialize")
        );
        return;
    }
    println!("{payload:#}");
}
