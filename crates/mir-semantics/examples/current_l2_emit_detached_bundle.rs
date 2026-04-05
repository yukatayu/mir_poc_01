use std::{env, fs, path::PathBuf, process};

use mir_semantics::{
    load_bundle_from_fixture_path, run_bundle,
};
#[path = "support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;

use current_l2_detached_bundle_support::build_detached_bundle_artifact;

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <fixture-path> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --output /tmp/e3.json"
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
        .unwrap_or_else(|| "current_l2_emit_detached_bundle".to_string());
    let (fixture_path, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let bundle = load_bundle_from_fixture_path(&fixture_path)?;
    let report = run_bundle(&bundle)?;
    let artifact = build_detached_bundle_artifact(&bundle, &report);
    let payload = serde_json::to_string_pretty(&artifact)?;

    match output_path {
        Some(path) => fs::write(path, payload)?,
        None => println!("{payload}"),
    }

    Ok(())
}
