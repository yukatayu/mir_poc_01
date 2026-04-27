use std::{fmt, path::PathBuf};

use serde::Serialize;

use crate::{
    clean_near_end::{
        CleanNearEndSampleReport, clean_near_end_json_value, run_clean_near_end_sample,
    },
    current_l2::run_current_l2_source_sample,
};
use mir_semantics::{RunReport, StaticGateVerdict, load_host_plan_from_path};

pub const CURRENT_L2_OPERATIONAL_SHELL_NAME: &str = "mir-current-l2";
const RUN_SOURCE_SAMPLE_COMMAND: &str = "run-source-sample";
const CHECK_SOURCE_SAMPLE_COMMAND: &str = "check-source-sample";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2OperationalCliError {
    message: String,
    exit_code: i32,
}

impl CurrentL2OperationalCliError {
    fn usage(message: impl Into<String>) -> Self {
        Self {
            message: format!("{}\n{}", message.into(), current_l2_operational_cli_usage()),
            exit_code: 2,
        }
    }

    fn execution(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl fmt::Display for CurrentL2OperationalCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for CurrentL2OperationalCliError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CurrentL2CliOutputFormat {
    Pretty,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CurrentL2RunSourceSampleCommand {
    sample: String,
    host_plan_path: Option<PathBuf>,
    format: CurrentL2CliOutputFormat,
}

enum CurrentL2CliCommand {
    RunSourceSample(CurrentL2RunSourceSampleCommand),
    CheckSourceSample(CurrentL2RunSourceSampleCommand),
}

pub fn current_l2_operational_cli_usage() -> String {
    format!(
        "usage: {CURRENT_L2_OPERATIONAL_SHELL_NAME} {RUN_SOURCE_SAMPLE_COMMAND} <sample-or-path> [--host-plan <path>] --format pretty|json\n       {CURRENT_L2_OPERATIONAL_SHELL_NAME} {CHECK_SOURCE_SAMPLE_COMMAND} <sample-or-path> [--host-plan <path>] --format pretty|json"
    )
}

pub fn run_current_l2_operational_cli<I, S>(args: I) -> Result<String, CurrentL2OperationalCliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args = args.into_iter().map(Into::into).collect::<Vec<_>>();
    if args.len() == 1 && matches!(args[0].as_str(), "-h" | "--help") {
        return Ok(current_l2_operational_cli_usage());
    }
    match parse_current_l2_operational_cli(args)? {
        CurrentL2CliCommand::RunSourceSample(command) => run_source_sample_command(command),
        CurrentL2CliCommand::CheckSourceSample(command) => check_source_sample_command(command),
    }
}

fn parse_current_l2_operational_cli(
    args: Vec<String>,
) -> Result<CurrentL2CliCommand, CurrentL2OperationalCliError> {
    let Some((command, rest)) = args.split_first() else {
        return Err(CurrentL2OperationalCliError::usage("missing command"));
    };
    match command.as_str() {
        RUN_SOURCE_SAMPLE_COMMAND => Ok(CurrentL2CliCommand::RunSourceSample(
            parse_run_source_sample_command(rest)?,
        )),
        CHECK_SOURCE_SAMPLE_COMMAND => Ok(CurrentL2CliCommand::CheckSourceSample(
            parse_run_source_sample_command(rest)?,
        )),
        other => Err(CurrentL2OperationalCliError::usage(format!(
            "unknown command `{other}`"
        ))),
    }
}

fn parse_run_source_sample_command(
    args: &[String],
) -> Result<CurrentL2RunSourceSampleCommand, CurrentL2OperationalCliError> {
    let Some((sample, rest)) = args.split_first() else {
        return Err(CurrentL2OperationalCliError::usage(
            "missing required <sample-or-path> argument",
        ));
    };
    if sample.trim().is_empty() {
        return Err(CurrentL2OperationalCliError::usage(
            "sample argument must not be empty",
        ));
    }

    let mut host_plan_path = None;
    let mut format = None;
    let mut cursor = 0;
    while cursor < rest.len() {
        match rest[cursor].as_str() {
            "--host-plan" => {
                let value = rest.get(cursor + 1).ok_or_else(|| {
                    CurrentL2OperationalCliError::usage("missing value for --host-plan <path>")
                })?;
                host_plan_path = Some(PathBuf::from(value));
                cursor += 2;
            }
            "--format" => {
                let value = rest.get(cursor + 1).ok_or_else(|| {
                    CurrentL2OperationalCliError::usage("missing value for --format pretty|json")
                })?;
                format = Some(parse_output_format(value)?);
                cursor += 2;
            }
            other => {
                return Err(CurrentL2OperationalCliError::usage(format!(
                    "unexpected argument `{other}`"
                )));
            }
        }
    }

    Ok(CurrentL2RunSourceSampleCommand {
        sample: sample.clone(),
        host_plan_path,
        format: format.ok_or_else(|| {
            CurrentL2OperationalCliError::usage("missing required --format pretty|json")
        })?,
    })
}

fn parse_output_format(
    value: &str,
) -> Result<CurrentL2CliOutputFormat, CurrentL2OperationalCliError> {
    match value {
        "pretty" => Ok(CurrentL2CliOutputFormat::Pretty),
        "json" => Ok(CurrentL2CliOutputFormat::Json),
        other => Err(CurrentL2OperationalCliError::usage(format!(
            "unsupported format `{other}`"
        ))),
    }
}

fn run_source_sample_command(
    command: CurrentL2RunSourceSampleCommand,
) -> Result<String, CurrentL2OperationalCliError> {
    if is_clean_near_end_sample(&command.sample) {
        let report = run_clean_near_end_sample(&command.sample)
            .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string()))?;
        return render_clean_report(RUN_SOURCE_SAMPLE_COMMAND, report, command.format);
    }

    let report = load_current_l2_source_report(&command)?;
    render_current_l2_report(RUN_SOURCE_SAMPLE_COMMAND, report, command.format)
}

fn check_source_sample_command(
    command: CurrentL2RunSourceSampleCommand,
) -> Result<String, CurrentL2OperationalCliError> {
    if is_clean_near_end_sample(&command.sample) {
        let report = run_clean_near_end_sample(&command.sample)
            .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string()))?;
        return render_clean_report(CHECK_SOURCE_SAMPLE_COMMAND, report, command.format);
    }

    let report = load_current_l2_source_report(&command)?;
    render_current_l2_report(CHECK_SOURCE_SAMPLE_COMMAND, report, command.format)
}

fn is_clean_near_end_sample(sample: &str) -> bool {
    sample.ends_with(".mir")
        || sample.contains("samples/clean-near-end/")
        || sample.contains("authorized_declassification")
        || sample.contains("peterson")
        || sample.starts_with("typing/")
        || sample.starts_with("order-handoff/")
        || sample.starts_with("model-check/")
        || sample.starts_with("modal/")
}

#[derive(Debug, Serialize)]
struct CurrentL2CompactReport {
    shell: &'static str,
    command: &'static str,
    sample: String,
    sample_path: String,
    host_plan_path: String,
    static_verdict: String,
    runtime: CurrentL2CompactRuntime,
}

#[derive(Debug, Serialize)]
struct CurrentL2CompactRuntime {
    entered_evaluation: bool,
    terminal_outcome: Option<String>,
    steps_executed: usize,
    events: Vec<String>,
}

fn load_current_l2_source_report(
    command: &CurrentL2RunSourceSampleCommand,
) -> Result<CurrentL2CompactReport, CurrentL2OperationalCliError> {
    let host_plan_path = resolve_host_plan_path(command)?;
    let host_plan = load_host_plan_from_path(&host_plan_path).map_err(|error| {
        CurrentL2OperationalCliError::execution(format!(
            "failed to load host plan {}: {error}",
            host_plan_path.display()
        ))
    })?;
    let source_report = run_current_l2_source_sample(&command.sample, host_plan)
        .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string()))?;
    Ok(CurrentL2CompactReport {
        shell: CURRENT_L2_OPERATIONAL_SHELL_NAME,
        command: RUN_SOURCE_SAMPLE_COMMAND,
        sample: source_report.sample_id,
        sample_path: source_report.sample_path.display().to_string(),
        host_plan_path: host_plan_path.display().to_string(),
        static_verdict: match source_report
            .runtime_report
            .checker_floor
            .static_gate
            .verdict
        {
            StaticGateVerdict::Valid => "valid".to_string(),
            StaticGateVerdict::Malformed => "malformed".to_string(),
            StaticGateVerdict::Underdeclared => "underdeclared".to_string(),
        },
        runtime: compact_runtime_report(source_report.runtime_report.run_report),
    })
}

fn resolve_host_plan_path(
    command: &CurrentL2RunSourceSampleCommand,
) -> Result<PathBuf, CurrentL2OperationalCliError> {
    if let Some(path) = command.host_plan_path.clone() {
        return Ok(path);
    }
    let sample_path = PathBuf::from(&command.sample);
    if sample_path.exists() {
        let candidate = sample_path.with_extension("host-plan.json");
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(CurrentL2OperationalCliError::usage(format!(
        "missing --host-plan <path> and no adjacent host plan was found for `{}`",
        command.sample
    )))
}

fn compact_runtime_report(report: RunReport) -> CurrentL2CompactRuntime {
    CurrentL2CompactRuntime {
        entered_evaluation: report.entered_evaluation,
        terminal_outcome: report
            .terminal_outcome
            .map(|outcome| format!("{outcome:?}").to_lowercase()),
        steps_executed: report.steps_executed,
        events: report
            .trace_audit_sink
            .events
            .into_iter()
            .map(|event| format!("{event:?}").to_lowercase())
            .collect(),
    }
}

fn render_current_l2_report(
    command: &'static str,
    mut report: CurrentL2CompactReport,
    format: CurrentL2CliOutputFormat,
) -> Result<String, CurrentL2OperationalCliError> {
    report.command = command;
    match format {
        CurrentL2CliOutputFormat::Pretty => Ok(format!(
            "shell: {}\ncommand: {}\nsample: {}\nsample_path: {}\nhost_plan_path: {}\nstatic_verdict: {}\nentered_evaluation: {}\nterminal_outcome: {}\nsteps_executed: {}\nevents:\n{}",
            report.shell,
            report.command,
            report.sample,
            report.sample_path,
            report.host_plan_path,
            report.static_verdict,
            report.runtime.entered_evaluation,
            report
                .runtime
                .terminal_outcome
                .clone()
                .unwrap_or_else(|| "null".to_string()),
            report.runtime.steps_executed,
            format_lines(&report.runtime.events),
        )),
        CurrentL2CliOutputFormat::Json => serde_json::to_string_pretty(&report)
            .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string())),
    }
}

fn render_clean_report(
    command: &'static str,
    report: CleanNearEndSampleReport,
    format: CurrentL2CliOutputFormat,
) -> Result<String, CurrentL2OperationalCliError> {
    match format {
        CurrentL2CliOutputFormat::Pretty => Ok(format!(
            "shell: {}\ncommand: {}\nsample: {}\nfamily: {}\nsource_path: {}\nstatic_verdict: {}\nentered_evaluation: {}\nterminal_outcome: {}\nreason_family: {}\nconstraints_solved:\n{}\nconstraints_failed:\n{}",
            CURRENT_L2_OPERATIONAL_SHELL_NAME,
            command,
            report.sample,
            report.family.as_str(),
            report.source_path,
            report
                .static_verdict
                .clone()
                .unwrap_or_else(|| "null".to_string()),
            report.entered_evaluation,
            report
                .terminal_outcome
                .clone()
                .unwrap_or_else(|| "null".to_string()),
            report
                .reason_family
                .clone()
                .unwrap_or_else(|| "null".to_string()),
            format_lines(&report.constraints_solved),
            format_lines(&report.constraints_failed),
        )),
        CurrentL2CliOutputFormat::Json => {
            serde_json::to_string_pretty(&clean_near_end_json_value(&report))
                .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string()))
        }
    }
}

fn format_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        "- <none>".to_string()
    } else {
        lines
            .iter()
            .map(|line| format!("- {line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
