use std::{env, fs, path::PathBuf, process};

#[allow(dead_code)]
#[path = "support/current_l2_detached_bundle_support.rs"]
mod current_l2_detached_bundle_support;
#[allow(dead_code)]
#[path = "support/current_l2_formal_hook_support.rs"]
mod current_l2_formal_hook_support;
#[allow(dead_code)]
#[path = "support/current_l2_static_gate_support.rs"]
mod current_l2_static_gate_support;

use current_l2_detached_bundle_support::DetachedBundleArtifact;
use current_l2_formal_hook_support::{
    build_formal_hook_from_detached_bundle_artifact, build_formal_hook_from_static_gate_artifact,
};
use current_l2_static_gate_support::DetachedStaticGateArtifact;

fn usage(program: &str) -> String {
    format!(
        "usage: {program} <static-gate|detached-bundle> <artifact-path> [--output <artifact-path>]\n\
         example: cargo run -p mir-semantics --example {program} -- \\\n\
         static-gate /tmp/e5.static-gate.json --output /tmp/e5.formal-hook.json"
    )
}

fn parse_args() -> Result<(String, PathBuf, Option<PathBuf>), String> {
    let mut args = env::args().skip(1);
    let source_kind = args
        .next()
        .ok_or_else(|| "missing <static-gate|detached-bundle>".to_string())?;
    let artifact_path = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "missing <artifact-path>".to_string())?;
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

    Ok((source_kind, artifact_path, output_path))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = env::args()
        .next()
        .unwrap_or_else(|| "current_l2_emit_formal_hook".to_string());
    let (source_kind, artifact_path, output_path) = match parse_args() {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };

    let payload = fs::read_to_string(&artifact_path)?;
    let formal_hook = match source_kind.as_str() {
        "static-gate" => {
            let artifact: DetachedStaticGateArtifact = serde_json::from_str(&payload)?;
            build_formal_hook_from_static_gate_artifact(&artifact)?
        }
        "detached-bundle" => {
            let artifact: DetachedBundleArtifact = serde_json::from_str(&payload)?;
            build_formal_hook_from_detached_bundle_artifact(&artifact)?
        }
        _ => {
            eprintln!("unsupported source kind: {source_kind}");
            eprintln!("{}", usage(&program));
            process::exit(2);
        }
    };
    let output = serde_json::to_string_pretty(&formal_hook)?;

    match output_path {
        Some(path) => fs::write(path, output)?,
        None => println!("{output}"),
    }

    Ok(())
}
