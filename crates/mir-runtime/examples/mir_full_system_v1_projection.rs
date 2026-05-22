use std::{env, process};

use mir_runtime::full_system_v1_projection::project_full_system_v1_path;

fn main() {
    let code = run(env::args().skip(1).collect());
    process::exit(code);
}

fn run(args: Vec<String>) -> i32 {
    let (format, request_path, positional) = strip_args(args);
    if !matches!(format.as_str(), "json" | "pretty") {
        eprintln!("unsupported format `{format}`");
        return 2;
    }
    let Some(source_path) = positional.first() else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example mir_full_system_v1_projection -- <source-path> --request <projection-request> [--format json|pretty]"
        );
        return 2;
    };
    if positional.len() != 1 || request_path.is_empty() {
        eprintln!("unexpected arguments or missing --request");
        return 2;
    }

    let report = project_full_system_v1_path(source_path, &request_path);
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report should serialize")
        );
    } else {
        println!(
            "FULL SYSTEM V1 PROJECTION\nsource: {}\nrequest: {}\naccepted: {}\ntargets: {}\ndiagnostics: {}",
            report.source_path,
            report.request_path,
            report.accepted,
            report.target_manifests.len(),
            report.diagnostics.len()
        );
    }

    if report.accepted { 0 } else { 2 }
}

fn strip_args(args: Vec<String>) -> (String, String, Vec<String>) {
    let mut format = "pretty".to_string();
    let mut request_path = String::new();
    let mut positional = Vec::new();
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                if let Some(value) = args.get(index + 1) {
                    format = value.clone();
                    index += 2;
                    continue;
                }
            }
            "--request" => {
                if let Some(value) = args.get(index + 1) {
                    request_path = value.clone();
                    index += 2;
                    continue;
                }
            }
            _ => {}
        }
        positional.push(args[index].clone());
        index += 1;
    }
    (format, request_path, positional)
}
