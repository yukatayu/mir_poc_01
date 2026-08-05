//! Provisional M10 command-line facade.
//!
//! Every command constructs one typed facade command and lets
//! `M10ReferenceSystem` own source parsing, admission, runtime operation, and
//! conformance routing.  The binary holds no second parser, fixture
//! dispatcher, authority constructor, or lifecycle-result shortcut.

use std::fs;

use mir_runtime::m10_reference_system::{M10CliFacadeCommand, M10ReferenceSystem};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let Some(command_name) = args.first().map(String::as_str) else {
        return Err(usage());
    };
    let command = command_from_name(command_name)?;
    let command = populate_command(command, &args[1..])?;
    let mut system = M10ReferenceSystem::deterministic_profile("m10-reference-profile");
    let report = system.run_cli(command)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report)
            .map_err(|error| format!("unable to serialize M10 report: {error}"))?
    );
    Ok(())
}

fn command_from_name(name: &str) -> Result<M10CliFacadeCommand, String> {
    match name {
        "parse" => Ok(M10CliFacadeCommand::parse()),
        "check" => Ok(M10CliFacadeCommand::check()),
        "elab" | "elaborate" => Ok(M10CliFacadeCommand::elaborate()),
        "run" => Ok(M10CliFacadeCommand::run()),
        "trace" => Ok(M10CliFacadeCommand::trace()),
        "project" => Ok(M10CliFacadeCommand::project()),
        "save" => Ok(M10CliFacadeCommand::save()),
        "load" => Ok(M10CliFacadeCommand::load()),
        "patch" => Ok(M10CliFacadeCommand::patch()),
        "conform" => Ok(M10CliFacadeCommand::conform()),
        _ => Err(usage()),
    }
}

fn populate_command(
    mut command: M10CliFacadeCommand,
    args: &[String],
) -> Result<M10CliFacadeCommand, String> {
    let mut index = 0;
    if let Some(path) = args.first().filter(|value| !value.starts_with("--")) {
        command = command.source_path(path);
        index = 1;
    }
    while index < args.len() {
        let flag = args[index].as_str();
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        command = match flag {
            "--candidate" => command.candidate_source_path(value),
            "--corpus" => command.corpus_path(value),
            "--schedule" => {
                let source = fs::read_to_string(value)
                    .map_err(|error| format!("unable to read typed schedule {value}: {error}"))?;
                let schedule = serde_json::from_str(&source)
                    .map_err(|error| format!("invalid typed schedule {value}: {error}"))?;
                command.typed_schedule_json(schedule)
            }
            "--patch-intent" => {
                let source = fs::read_to_string(value)
                    .map_err(|error| format!("unable to read patch intent {value}: {error}"))?;
                let carrier = serde_json::from_str(&source)
                    .map_err(|error| format!("invalid patch intent {value}: {error}"))?;
                command.patch_intent_json(carrier)
            }
            "--carriers" => {
                let source = fs::read_to_string(value)
                    .map_err(|error| format!("unable to read typed carriers {value}: {error}"))?;
                let carriers = serde_json::from_str(&source)
                    .map_err(|error| format!("invalid typed carriers {value}: {error}"))?;
                command.typed_carriers_json(carriers)
            }
            "--predicates" => {
                let source = fs::read_to_string(value).map_err(|error| {
                    format!("unable to read correspondence predicates {value}: {error}")
                })?;
                let profile = serde_json::from_str(&source).map_err(|error| {
                    format!("invalid correspondence predicates {value}: {error}")
                })?;
                command.predicate_profile_json(profile)
            }
            "--expected-output" => command.expected_output_json(value),
            "--artifact" => command.checked_artifact_without_source(value),
            _ => return Err(format!("unknown option {flag}")),
        };
        index += 2;
    }
    Ok(command)
}

fn usage() -> String {
    "usage: mir parse|check|elab|elaborate|run|trace|project|save|load|patch|conform [source.mir] [--candidate source.mir] [--corpus DIR] [--schedule typed.json] [--patch-intent typed.json] [--carriers typed.json] [--predicates profile.json]".to_string()
}
