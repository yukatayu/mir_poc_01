use std::{env, fs, path::PathBuf, process};

use mir_semantics::{load_fixture_from_path, static_gate_detailed};

#[path = "support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_static_gate_support::build_detached_static_gate_artifact;

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <fixture-path> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --output /tmp/e4.static-gate.json"
    )
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let mut fixture_path = None;
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output_path = Some(PathBuf::from(value));
            }
            _ if fixture_path.is_none() => {
                fixture_path = Some(PathBuf::from(arg));
            }
            _ => {
                return Err(format!("unexpected argument: {arg}"));
            }
        }
    }

    fixture_path
        .map(|path| (path, output_path))
        .ok_or_else(|| "missing <fixture-path>".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_static_gate".to_string());
    let (fixture_path, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let fixture = load_fixture_from_path(&fixture_path)?;
    let gate = static_gate_detailed(&fixture);
    let artifact = build_detached_static_gate_artifact(fixture_path, &fixture, &gate);
    let payload = serde_json::to_string_pretty(&artifact)?;

    match output_path {
        Some(path) => fs::write(path, payload)?,
        None => println!("{payload}"),
    }

    Ok(())
}
