use std::{env, process};

use mir_runtime::posegraph_runtime::run_posegraph_runtime_package_path;

fn main() {
    let code = run(env::args().skip(1).collect());
    process::exit(code);
}

fn run(args: Vec<String>) -> i32 {
    let (format, positional) = strip_args(args);
    if !matches!(format.as_str(), "json" | "pretty") {
        eprintln!("unsupported format `{format}`");
        return 2;
    }
    let Some(path) = positional.first() else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example posegraph_runtime_session -- <package-path> [--format json|pretty]"
        );
        return 2;
    };
    if positional.len() != 1 {
        eprintln!("unexpected positional arguments");
        return 2;
    }

    let report = run_posegraph_runtime_package_path(path);
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report should serialize")
        );
    } else {
        println!(
            "POSEGRAPH RUNTIME\npackage: {}\ntransition: {}.{}\noutcome: {:?}\nobserver_safe_summary: {}",
            report.package_path,
            report.module_id,
            report.transition_id,
            report.terminal_outcome,
            report.observer_safe_summary
        );
    }

    if report.accepted { 0 } else { 2 }
}

fn strip_args(args: Vec<String>) -> (String, Vec<String>) {
    let mut format = "pretty".to_string();
    let mut positional = Vec::new();
    let mut index = 0usize;
    while index < args.len() {
        if args[index].as_str() == "--format"
            && let Some(value) = args.get(index + 1)
        {
            format = value.clone();
            index += 2;
            continue;
        }
        positional.push(args[index].clone());
        index += 1;
    }
    (format, positional)
}
