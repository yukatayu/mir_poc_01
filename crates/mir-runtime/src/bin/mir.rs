//! Provisional M10 command-line facade.
//!
//! Every command constructs one typed facade command and lets
//! `M10ReferenceSystem` own source parsing, admission, runtime operation, and
//! conformance routing.  The binary holds no second parser, fixture
//! dispatcher, authority constructor, or lifecycle-result shortcut.

use std::{fs, num::NonZeroUsize};

use mir_runtime::m10_reference_system::{M10CliFacadeCommand, M10ReferenceSystem};
use mir_runtime::{
    sys5_local_slice::{Sys5SourceInput, build_project},
    sys5_local_workflow::{
        Sys5LocalWorkflowInput, Sys5LocalWorkflowPatchProject, run_local_workflow_from_project,
    },
    sys6_i2_conformance::{I2ConformanceInput, run_i2_conformance},
};
use serde_json::{Value, json};

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
    if matches!(command_name, "project-loci" | "run-local" | "inspect") {
        return run_sys5_command(command_name, &args[1..]);
    }
    if command_name == "conform-i2" {
        return run_sys6_command(&args[1..]);
    }
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

fn run_sys6_command(args: &[String]) -> Result<(), String> {
    match sys6_command_value(args) {
        Ok(value) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&value)
                    .map_err(|error| format!("unable to serialize SYS-6 report: {error}"))?
            );
            if value.get("status").and_then(Value::as_str) == Some("rejected") {
                Err("SYS-6 conform-i2 rejected: i2_profile_rejected".to_string())
            } else {
                Ok(())
            }
        }
        Err(error) => {
            let value = json!({
                "status": "error",
                "command": "conform-i2",
                "diagnostic_code": error,
                "source_authority": "ordinary_mir_source",
                "public_api_or_wire_contract": false,
                "final_public_api_frozen": false,
                "public_wire_frozen": false,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&value).map_err(|serialization| format!(
                    "unable to serialize SYS-6 error: {serialization}"
                ))?
            );
            Err(format!("SYS-6 conform-i2 rejected: {error}"))
        }
    }
}

fn sys6_command_value(args: &[String]) -> Result<Value, &'static str> {
    let Some(source_path) = args.first().filter(|value| !value.starts_with("--")) else {
        return Err("i2_missing_source_path");
    };
    let mut input = I2ConformanceInput::source_path(std::path::PathBuf::from(source_path));
    let mut selected_ow1_source = false;
    let mut format_json = false;
    let mut index = 1;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + 1) else {
            return Err("i2_unexpected_arguments");
        };
        if value.starts_with("--") {
            return Err("i2_unexpected_arguments");
        }
        match flag {
            "--patch" => input = input.with_patch_path(std::path::PathBuf::from(value)),
            "--selected-ow1-source" if !selected_ow1_source => {
                input = input.with_selected_ow1_source_path(std::path::PathBuf::from(value));
                selected_ow1_source = true;
            }
            "--format" if value == "json" && !format_json => format_json = true,
            _ => return Err("i2_unexpected_arguments"),
        }
        index += 2;
    }
    if !format_json || !selected_ow1_source {
        return Err("i2_unexpected_arguments");
    }
    let report = run_i2_conformance(input).map_err(|error| error.diagnostic_code())?;
    // SYS-6 owns a single fail-closed observer serializer. Never serialize
    // the raw report here: source-controlled identifiers can occur in any
    // selected-source-derived observer field.
    Ok(report.observer_safe_value())
}

fn run_sys5_command(command: &str, args: &[String]) -> Result<(), String> {
    match sys5_command_value(command, args) {
        Ok(value) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&value)
                    .map_err(|error| format!("unable to serialize SYS-5 report: {error}"))?
            );
            Ok(())
        }
        Err(error) => {
            let value = json!({
                "status": "error",
                "command": error.command,
                "diagnostic_code": error.diagnostic_code,
                "source_authority": "ordinary_mir_source",
                "public_api_or_wire_contract": false,
                "final_public_api_frozen": false,
                "public_wire_frozen": false,
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&value).map_err(|serialization| format!(
                    "unable to serialize SYS-5 error: {serialization}"
                ))?
            );
            Err(format!(
                "SYS-5 {} rejected: {}",
                error.command, error.diagnostic_code
            ))
        }
    }
}

fn sys5_command_value(command: &str, args: &[String]) -> Result<Value, Sys5CliError> {
    let parsed = parse_sys5_args(command, args)?;
    let source_project = project_cli_source(command, &parsed.source_path, "cli-source.mir")?;
    if command == "project-loci" {
        let semantic = source_project.semantic_summary();
        return Ok(json!({
            "status": "ok",
            "command": "project-loci",
            "source_authority": "ordinary_mir_source",
            "profile_name": semantic.profile_name,
            "profile_status": semantic.profile_status,
            "public_api_or_wire_contract": false,
            "final_public_api_frozen": false,
            "public_wire_frozen": false,
            "requires_runtime_execution": false,
            "loci": semantic.loci,
            "locus_programs": semantic.artifacts,
            "generated_communication": semantic.generated_communication,
            "source_core_artifact_mappings": semantic.source_core_artifact_mappings,
            "auth_residuals": semantic.auth_residuals,
            "verification_residuals": semantic.verification_residuals,
            "observer_safety": semantic.observer_safety,
        }));
    }

    let base_admission = source_project
        .prepare_canonical_local_st_admission()
        .map_err(|_| Sys5CliError::new(command, "admission_rejected"))?;
    let mut input =
        Sys5LocalWorkflowInput::from_project_and_admission(source_project, base_admission);
    for (index, patch_path) in parsed.patch_paths.iter().enumerate() {
        // Patch comparison is source/Core-relative to the active checked
        // program, so it retains that semantic logical path. The observer
        // label comes only from the ordinal constructor below; the CLI never
        // forwards a filename/path as provenance.
        let patch_project = project_cli_source(command, patch_path, "cli-source.mir")?;
        let patch_admission = patch_project
            .prepare_canonical_local_st_admission()
            .map_err(|_| Sys5CliError::new(command, "patch_admission_rejected"))?;
        input = input.with_patch_project(
            Sys5LocalWorkflowPatchProject::from_project_and_admission(
                format!("patch-{}", index + 1),
                patch_project,
                patch_admission,
            )
            .with_cli_patch_ordinal(
                NonZeroUsize::new(index + 1).expect("enumerated CLI patch ordinal is nonzero"),
            ),
        );
    }
    let report = run_local_workflow_from_project(input)
        .map_err(|_| Sys5CliError::new(command, "workflow_rejected"))?;
    let mut value = serde_json::to_value(report)
        .map_err(|_| Sys5CliError::new(command, "report_serialization_error"))?;
    let object = value
        .as_object_mut()
        .expect("workflow report serializes to a JSON object");
    object.insert("status".to_string(), Value::String("ok".to_string()));
    object.insert("command".to_string(), Value::String(command.to_string()));
    Ok(value)
}

fn project_cli_source(
    command: &str,
    path: &str,
    logical_source_path: &str,
) -> Result<mir_runtime::sys5_local_slice::Sys5LocalProject, Sys5CliError> {
    let source =
        fs::read_to_string(path).map_err(|_| Sys5CliError::new(command, "source_path_io_error"))?;
    build_project(Sys5SourceInput::inline(logical_source_path, source))
        .map_err(|_| Sys5CliError::new(command, "source_check_or_projection_error"))
}

struct ParsedSys5Args {
    source_path: String,
    patch_paths: Vec<String>,
}

fn parse_sys5_args(command: &str, args: &[String]) -> Result<ParsedSys5Args, Sys5CliError> {
    let Some(source_path) = args.first().filter(|value| !value.starts_with("--")) else {
        return Err(Sys5CliError::new(command, "missing_source_path"));
    };
    let mut patch_paths = Vec::new();
    let mut index = 1;
    let mut format_json = false;
    while index < args.len() {
        let flag = args[index].as_str();
        match flag {
            "--format" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(Sys5CliError::new(command, "unexpected_arguments"));
                };
                if value != "json" || format_json {
                    return Err(Sys5CliError::new(command, "unexpected_arguments"));
                }
                format_json = true;
                index += 2;
            }
            "--patch" if command != "project-loci" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(Sys5CliError::new(command, "unexpected_arguments"));
                };
                if value.starts_with("--") {
                    return Err(Sys5CliError::new(command, "unexpected_arguments"));
                }
                patch_paths.push(value.clone());
                index += 2;
            }
            _ => return Err(Sys5CliError::new(command, "unexpected_arguments")),
        }
    }
    if !format_json {
        return Err(Sys5CliError::new(command, "unexpected_arguments"));
    }
    Ok(ParsedSys5Args {
        source_path: source_path.clone(),
        patch_paths,
    })
}

struct Sys5CliError {
    command: String,
    diagnostic_code: &'static str,
}

impl Sys5CliError {
    fn new(command: &str, diagnostic_code: &'static str) -> Self {
        Self {
            command: command.to_string(),
            diagnostic_code,
        }
    }
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
    "usage: mir parse|check|elab|elaborate|run|trace|project|save|load|patch|conform [source.mir] [--candidate source.mir] [--corpus DIR] [--schedule typed.json] [--patch-intent typed.json] [--carriers typed.json] [--predicates profile.json] | mir project-loci|run-local|inspect source.mir [--patch source.mir] --format json | mir conform-i2 source.mir --selected-ow1-source source.mir --patch source.mir --patch source.mir --format json".to_string()
}
