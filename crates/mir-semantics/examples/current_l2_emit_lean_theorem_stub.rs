use std::{env, fs, path::PathBuf, process};

#[allow(dead_code)]
#[path = "support/current_l2_lean_theorem_stub_support.rs"]
mod current_l2_lean_theorem_stub_support;
#[allow(dead_code)]
#[path = "support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[allow(dead_code)]
#[path = "support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[allow(dead_code)]
#[path = "support/current_l2_proof_notebook_review_unit_support.rs"]
mod current_l2_proof_notebook_review_unit_support;
#[allow(dead_code)]
#[path = "support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_lean_theorem_stub_support::build_lean_theorem_stub_artifacts;
use current_l2_proof_notebook_review_unit_support::ProofNotebookReviewUnitArtifact;

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <review-unit-path> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         /tmp/e2.proof-notebook-review-unit.json --output /tmp/e2.lean-theorem-stub.json"
    )
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let artifact_path = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "missing <review-unit-path>".to_string())?;
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                output_path = Some(PathBuf::from(value));
            }
            _ => return Err(format!("unexpected argument: {arg}")),
        }
    }

    Ok((artifact_path, output_path))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_lean_theorem_stub".to_string());
    let (artifact_path, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let payload = fs::read_to_string(&artifact_path)?;
    let review_units: Vec<ProofNotebookReviewUnitArtifact> = serde_json::from_str(&payload)?;
    let stubs = build_lean_theorem_stub_artifacts(&review_units)?;
    let output = serde_json::to_string_pretty(&stubs)?;

    match output_path {
        Some(path) => fs::write(path, output)?,
        None => println!("{output}"),
    }

    Ok(())
}
