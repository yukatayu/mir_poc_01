use std::{env, process};

use mir_runtime::full_system_v1_renderer_pose_backend::run_full_system_v1_renderer_pose_backend_path;

fn main() {
    let code = run(env::args().skip(1).collect());
    process::exit(code);
}

fn run(args: Vec<String>) -> i32 {
    let (format, request_path, provider_path, posegraph_package_path, input, positional) =
        strip_args(args);
    if !matches!(format.as_str(), "json" | "pretty") {
        eprintln!("unsupported format `{format}`");
        return 2;
    }
    let Some(source_path) = positional.first() else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example mir_full_system_v1_renderer_pose_backend -- <source-path> --request <projection-request> --provider <provider-manifest> --posegraph-package <package.mir.json> [--input <int>] [--format json|pretty]"
        );
        return 2;
    };
    if positional.len() != 1
        || request_path.is_empty()
        || provider_path.is_empty()
        || posegraph_package_path.is_empty()
    {
        eprintln!("unexpected arguments or missing --request/--provider/--posegraph-package");
        return 2;
    }

    let report = run_full_system_v1_renderer_pose_backend_path(
        source_path,
        &request_path,
        &provider_path,
        &posegraph_package_path,
        input,
    );
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report should serialize")
        );
    } else {
        println!(
            "FULL SYSTEM V1 RENDERER POSE BACKEND\nsource: {}\nrequest: {}\nprovider: {}\nposegraph: {}\naccepted: {}\noutcome: {}",
            report.source_path,
            report.request_path,
            report.provider_manifest_path,
            report.posegraph_package_path,
            report.accepted,
            report.terminal_outcome
        );
    }

    if report.accepted { 0 } else { 2 }
}

fn strip_args(args: Vec<String>) -> (String, String, String, String, i64, Vec<String>) {
    let mut format = "pretty".to_string();
    let mut request_path = String::new();
    let mut provider_path = String::new();
    let mut posegraph_package_path = String::new();
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
            "--request" => {
                if let Some(value) = args.get(index + 1) {
                    request_path = value.clone();
                    index += 2;
                    continue;
                }
            }
            "--provider" => {
                if let Some(value) = args.get(index + 1) {
                    provider_path = value.clone();
                    index += 2;
                    continue;
                }
            }
            "--posegraph-package" => {
                if let Some(value) = args.get(index + 1) {
                    posegraph_package_path = value.clone();
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
    (
        format,
        request_path,
        provider_path,
        posegraph_package_path,
        input,
        positional,
    )
}
