use std::{env, process};

use mir_runtime::full_system_v1_session::run_full_system_v1_session_path;

fn main() {
    let code = run(env::args().skip(1).collect());
    process::exit(code);
}

fn run(args: Vec<String>) -> i32 {
    let (format, entry_function, input, positional) = strip_args(args);
    if !matches!(format.as_str(), "json" | "pretty") {
        eprintln!("unsupported format `{format}`");
        return 2;
    }
    let Some(path) = positional.first() else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example mir_full_system_v1_session -- <source-path> --entry <function> --input <int> [--format json|pretty]"
        );
        return 2;
    };
    if positional.len() != 1 {
        eprintln!("unexpected positional arguments");
        return 2;
    }

    let report = run_full_system_v1_session_path(path, &entry_function, input);
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report should serialize")
        );
    } else {
        println!(
            "FULL SYSTEM V1 SESSION\nsource: {}\nentry: {}\noutcome: {:?}\nobserver_safe_summary: {}",
            report.source_path,
            report.entry_function,
            report.runtime.outcome,
            report.observer_safe_summary
        );
    }

    if report.runtime.accepted { 0 } else { 2 }
}

fn strip_args(args: Vec<String>) -> (String, String, i64, Vec<String>) {
    let mut format = "pretty".to_string();
    let mut entry = "main".to_string();
    let mut input = 0i64;
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
            "--entry" => {
                if let Some(value) = args.get(index + 1) {
                    entry = value.clone();
                    index += 2;
                    continue;
                }
            }
            "--input" => {
                if let Some(value) = args.get(index + 1) {
                    input = value.parse::<i64>().unwrap_or_default();
                    index += 2;
                    continue;
                }
            }
            _ => {}
        }
        positional.push(args[index].clone());
        index += 1;
    }
    (format, entry, input, positional)
}
