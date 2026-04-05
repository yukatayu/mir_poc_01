use std::{env, fs, path::PathBuf, process};

use mir_semantics::run_directory;

#[path = "support/current_l2_detached_aggregate_support.rs"]
mod current_l2_detached_aggregate_support;

use current_l2_detached_aggregate_support::build_detached_aggregate_artifact;

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <fixture-directory> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         crates/mir-ast/tests/fixtures/current-l2 --output /tmp/current-l2-batch.json"
    )
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let mut fixture_directory = None;
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output_path = Some(PathBuf::from(value));
            }
            _ if fixture_directory.is_none() => {
                fixture_directory = Some(PathBuf::from(arg));
            }
            _ => {
                return Err(format!("unexpected argument: {arg}"));
            }
        }
    }

    fixture_directory
        .map(|path| (path, output_path))
        .ok_or_else(|| "missing <fixture-directory>".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_detached_aggregate".to_string());
    let (fixture_directory, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let summary = run_directory(&fixture_directory)?;
    let artifact = build_detached_aggregate_artifact(fixture_directory, &summary);
    let payload = serde_json::to_string_pretty(&artifact)?;

    match output_path {
        Some(path) => fs::write(path, payload)?,
        None => println!("{payload}"),
    }

    Ok(())
}
