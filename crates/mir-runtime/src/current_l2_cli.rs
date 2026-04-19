use std::{
    collections::BTreeMap,
    fmt::{self, Write},
    fs,
    path::PathBuf,
};

use serde::Serialize;

use crate::current_l2::{
    CurrentL2RuntimeSkeletonReport, CurrentL2SourceSampleRunReport,
    CurrentL2TryRollbackStructuralFindingKind, CurrentL2TryRollbackStructuralSubjectKind,
    CurrentL2TryRollbackStructuralSummary, CurrentL2TryRollbackStructuralVerdict,
    resolve_current_l2_source_sample_path, run_current_l2_source_sample,
};
use mir_semantics::{
    EventKind, NonAdmissibleSubreason, StaticGateVerdict, TerminalOutcome, load_host_plan_from_path,
};

pub const CURRENT_L2_OPERATIONAL_SHELL_NAME: &str = "mir-current-l2";
const RUN_SOURCE_SAMPLE_COMMAND: &str = "run-source-sample";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2OperationalCliError {
    exit_code: i32,
    message: String,
}

impl CurrentL2OperationalCliError {
    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    fn usage(message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            exit_code: 2,
            message: format!("{message}\n{}", current_l2_operational_cli_usage()),
        }
    }

    fn execution(message: impl Into<String>) -> Self {
        Self {
            exit_code: 1,
            message: message.into(),
        }
    }
}

impl fmt::Display for CurrentL2OperationalCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
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

pub fn current_l2_operational_cli_usage() -> String {
    format!(
        "usage: {CURRENT_L2_OPERATIONAL_SHELL_NAME} {RUN_SOURCE_SAMPLE_COMMAND} <sample-or-path> [--host-plan <path>] --format pretty|json"
    )
}

pub fn run_current_l2_operational_cli<I, S>(args: I) -> Result<String, CurrentL2OperationalCliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args = args.into_iter().map(Into::into).collect::<Vec<String>>();
    if args.len() == 1 && matches!(args[0].as_str(), "-h" | "--help") {
        return Ok(current_l2_operational_cli_usage());
    }

    match parse_current_l2_operational_cli(args)? {
        CurrentL2CliCommand::RunSourceSample(command) => run_source_sample_command(command),
    }
}

enum CurrentL2CliCommand {
    RunSourceSample(CurrentL2RunSourceSampleCommand),
}

fn parse_current_l2_operational_cli(
    args: Vec<String>,
) -> Result<CurrentL2CliCommand, CurrentL2OperationalCliError> {
    let Some((command, rest)) = args.split_first() else {
        return Err(CurrentL2OperationalCliError::usage(
            "missing command `run-source-sample`",
        ));
    };

    match command.as_str() {
        RUN_SOURCE_SAMPLE_COMMAND => Ok(CurrentL2CliCommand::RunSourceSample(
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
            "missing required <sample> argument",
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
            flag => {
                return Err(CurrentL2OperationalCliError::usage(format!(
                    "unexpected argument `{flag}`"
                )));
            }
        }
    }

    let format = format.ok_or_else(|| {
        CurrentL2OperationalCliError::usage("missing required --format pretty|json")
    })?;

    Ok(CurrentL2RunSourceSampleCommand {
        sample: sample.clone(),
        host_plan_path,
        format,
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
    let sample_path = resolve_current_l2_source_sample_path(&command.sample)
        .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string()))?;
    let host_plan_path = command
        .host_plan_path
        .unwrap_or_else(|| sample_path.with_extension("host-plan.json"));
    if !host_plan_path.is_file() {
        return Err(CurrentL2OperationalCliError::usage(format!(
            "missing --host-plan <path> and no adjacent host plan was found for {}",
            sample_path.display()
        )));
    }
    let host_plan = load_host_plan_from_path(&host_plan_path).map_err(|error| {
        CurrentL2OperationalCliError::execution(format!(
            "failed to load host plan {}: {error}",
            host_plan_path.display()
        ))
    })?;
    let report =
        run_current_l2_source_sample(sample_path.to_str().unwrap_or(&command.sample), host_plan)
            .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string()))?;
    let summary =
        CurrentL2OperationalCliRunSourceSampleSummary::from_report(&host_plan_path, report);

    match command.format {
        CurrentL2CliOutputFormat::Pretty => Ok(render_pretty_summary(&summary)),
        CurrentL2CliOutputFormat::Json => serde_json::to_string_pretty(&summary)
            .map_err(|error| CurrentL2OperationalCliError::execution(error.to_string())),
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliRunSourceSampleSummary {
    shell: &'static str,
    command: &'static str,
    sample: String,
    sample_path: String,
    host_plan_path: String,
    checker_floor: CurrentL2OperationalCliCheckerFloorSummary,
    runtime: CurrentL2OperationalCliRuntimeSummary,
    verification_preview: CurrentL2OperationalCliVerificationPreviewSummary,
    artifact_preview: CurrentL2OperationalCliArtifactPreviewSummary,
}

impl CurrentL2OperationalCliRunSourceSampleSummary {
    fn from_report(host_plan_path: &PathBuf, report: CurrentL2SourceSampleRunReport) -> Self {
        let verification_preview =
            CurrentL2OperationalCliVerificationPreviewSummary::from_source_report(&report);
        let artifact_preview =
            CurrentL2OperationalCliArtifactPreviewSummary::from_source_report(&report);
        Self {
            shell: CURRENT_L2_OPERATIONAL_SHELL_NAME,
            command: RUN_SOURCE_SAMPLE_COMMAND,
            sample: report.sample_id,
            sample_path: display_path(&report.sample_path),
            host_plan_path: display_path(host_plan_path),
            checker_floor: CurrentL2OperationalCliCheckerFloorSummary::from_runtime_report(
                &report.runtime_report,
            ),
            runtime: CurrentL2OperationalCliRuntimeSummary::from_runtime_report(
                &report.runtime_report,
            ),
            verification_preview,
            artifact_preview,
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliCheckerFloorSummary {
    stage1_reconnect_clusters: Option<CurrentL2OperationalCliStage1ReconnectClusters>,
    stage2_try_rollback_summary: Option<CurrentL2OperationalCliTryRollbackSummary>,
    static_gate: CurrentL2OperationalCliStaticGateSummary,
}

impl CurrentL2OperationalCliCheckerFloorSummary {
    fn from_runtime_report(report: &CurrentL2RuntimeSkeletonReport) -> Self {
        Self {
            stage1_reconnect_clusters: report
                .checker_floor
                .stage1_reconnect_clusters
                .as_ref()
                .map(CurrentL2OperationalCliStage1ReconnectClusters::from_stage1),
            stage2_try_rollback_summary: report
                .checker_floor
                .stage2_try_rollback_summary
                .as_ref()
                .map(CurrentL2OperationalCliTryRollbackSummary::from_stage2),
            static_gate: CurrentL2OperationalCliStaticGateSummary {
                verdict: static_gate_verdict_name(report.checker_floor.static_gate.verdict),
                reasons: report.checker_floor.static_gate.reasons.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliStage1ReconnectClusters {
    same_lineage_floor: bool,
    missing_option_structure_floor: bool,
    capability_strengthening_floor: bool,
}

impl CurrentL2OperationalCliStage1ReconnectClusters {
    fn from_stage1(stage1: &crate::current_l2::CurrentL2Stage1ReconnectClusters) -> Self {
        Self {
            same_lineage_floor: stage1.same_lineage_floor,
            missing_option_structure_floor: stage1.missing_option_structure_floor,
            capability_strengthening_floor: stage1.capability_strengthening_floor,
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliTryRollbackSummary {
    verdict: &'static str,
    findings: Vec<CurrentL2OperationalCliTryRollbackFinding>,
}

impl CurrentL2OperationalCliTryRollbackSummary {
    fn from_stage2(stage2: &CurrentL2TryRollbackStructuralSummary) -> Self {
        Self {
            verdict: try_rollback_verdict_name(stage2.verdict),
            findings: stage2
                .findings
                .iter()
                .map(|finding| CurrentL2OperationalCliTryRollbackFinding {
                    subject_kind: try_rollback_subject_kind_name(finding.subject_kind),
                    finding_kind: try_rollback_finding_kind_name(finding.finding_kind),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliTryRollbackFinding {
    subject_kind: &'static str,
    finding_kind: &'static str,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliStaticGateSummary {
    verdict: &'static str,
    reasons: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliRuntimeSummary {
    entered_evaluation: bool,
    terminal_outcome: Option<&'static str>,
    steps_executed: usize,
    events: Vec<&'static str>,
    debug_outputs: BTreeMap<String, Vec<String>>,
    non_admissible_metadata: Vec<CurrentL2OperationalCliNonAdmissibleMetadata>,
    narrative_explanations: Vec<String>,
}

impl CurrentL2OperationalCliRuntimeSummary {
    fn from_runtime_report(report: &CurrentL2RuntimeSkeletonReport) -> Self {
        let run = &report.run_report;
        Self {
            entered_evaluation: run.entered_evaluation,
            terminal_outcome: run.terminal_outcome.map(terminal_outcome_name),
            steps_executed: run.steps_executed,
            events: run
                .trace_audit_sink
                .events
                .iter()
                .copied()
                .map(event_kind_name)
                .collect(),
            debug_outputs: collect_debug_outputs(&run.final_place_store),
            non_admissible_metadata: run
                .trace_audit_sink
                .non_admissible_metadata
                .iter()
                .map(|metadata| CurrentL2OperationalCliNonAdmissibleMetadata {
                    option_ref: metadata.option_ref.clone(),
                    subreason: non_admissible_subreason_name(metadata.subreason),
                })
                .collect(),
            narrative_explanations: run.trace_audit_sink.narrative_explanations.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliVerificationPreviewSummary {
    formal_hook_status: &'static str,
    subject_kind: Option<&'static str>,
    subject_ref: Option<String>,
    proof_notebook_review_unit_obligations: Vec<&'static str>,
    model_check_concrete_carrier_obligations: Vec<&'static str>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliVerificationPreviewSummary {
    fn from_source_report(report: &CurrentL2SourceSampleRunReport) -> Self {
        let static_verdict = report.runtime_report.checker_floor.static_gate.verdict;

        match static_verdict {
            StaticGateVerdict::Malformed | StaticGateVerdict::Underdeclared => Self {
                formal_hook_status: "reached",
                subject_kind: Some("fixture_static_cluster"),
                subject_ref: Some(report.sample_id.clone()),
                proof_notebook_review_unit_obligations: vec![
                    "canonical_normalization_law",
                    "no_re_promotion",
                ],
                model_check_concrete_carrier_obligations: vec![
                    "canonical_normalization_law",
                    "no_re_promotion",
                ],
                guard_reason: None,
            },
            StaticGateVerdict::Valid => {
                let has_try_cut = report
                    .runtime_report
                    .run_report
                    .trace_audit_sink
                    .events
                    .iter()
                    .any(|event| matches!(event, EventKind::Rollback | EventKind::AtomicCut));

                if has_try_cut {
                    Self {
                        formal_hook_status: "reached",
                        subject_kind: Some("runtime_try_cut_cluster"),
                        subject_ref: Some(report.sample_id.clone()),
                        proof_notebook_review_unit_obligations: vec![
                            "rollback_cut_non_interference",
                        ],
                        model_check_concrete_carrier_obligations: vec![
                            "rollback_cut_non_interference",
                        ],
                        guard_reason: None,
                    }
                } else {
                    Self {
                        formal_hook_status: "guarded_not_reached",
                        subject_kind: None,
                        subject_ref: None,
                        proof_notebook_review_unit_obligations: Vec::new(),
                        model_check_concrete_carrier_obligations: Vec::new(),
                        guard_reason: Some(format!(
                            "current helper preview only reaches runtime_try_cut_cluster when rollback or atomic-cut evidence exists for `{}`",
                            report.sample_id
                        )),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliArtifactPreviewSummary {
    proof_notebook_review_units: Vec<CurrentL2OperationalCliProofNotebookReviewUnitPreview>,
    model_check_concrete_carriers: Vec<CurrentL2OperationalCliModelCheckCarrierPreview>,
}

impl CurrentL2OperationalCliArtifactPreviewSummary {
    fn from_source_report(report: &CurrentL2SourceSampleRunReport) -> Self {
        let static_verdict = report.runtime_report.checker_floor.static_gate.verdict;

        match static_verdict {
            StaticGateVerdict::Malformed | StaticGateVerdict::Underdeclared => Self {
                proof_notebook_review_units: vec![
                    CurrentL2OperationalCliProofNotebookReviewUnitPreview {
                        obligation_kind: "canonical_normalization_law",
                        goal_text: format!(
                            "Review that canonical normalization preserves the rejected static shape for `{}`.",
                            report.sample_id
                        ),
                        evidence_refs: vec![
                            format!("fixture:{}", report.sample_id),
                            format!("static_gate_artifact:{}", report.sample_id),
                        ],
                    },
                    CurrentL2OperationalCliProofNotebookReviewUnitPreview {
                        obligation_kind: "no_re_promotion",
                        goal_text: format!(
                            "Review that `{}` remains rejected and is not re-promoted by later tooling.",
                            report.sample_id
                        ),
                        evidence_refs: vec![format!("fixture:{}", report.sample_id)],
                    },
                ],
                model_check_concrete_carriers: vec![
                    CurrentL2OperationalCliModelCheckCarrierPreview {
                        obligation_kind: "canonical_normalization_law",
                        evidence_refs: vec![
                            format!("fixture:{}", report.sample_id),
                            format!("static_gate_artifact:{}", report.sample_id),
                        ],
                    },
                    CurrentL2OperationalCliModelCheckCarrierPreview {
                        obligation_kind: "no_re_promotion",
                        evidence_refs: vec![format!("fixture:{}", report.sample_id)],
                    },
                ],
            },
            StaticGateVerdict::Valid => {
                let has_try_cut = report
                    .runtime_report
                    .run_report
                    .trace_audit_sink
                    .events
                    .iter()
                    .any(|event| matches!(event, EventKind::Rollback | EventKind::AtomicCut));

                if has_try_cut {
                    Self {
                        proof_notebook_review_units: vec![
                            CurrentL2OperationalCliProofNotebookReviewUnitPreview {
                                obligation_kind: "rollback_cut_non_interference",
                                goal_text: format!(
                                    "Review that rollback / atomic-cut evidence does not interfere with surviving runtime behavior for `{}`.",
                                    report.sample_id
                                ),
                                evidence_refs: vec![
                                    format!("fixture:{}", report.sample_id),
                                    format!("runtime_cluster:{}", report.sample_id),
                                ],
                            },
                        ],
                        model_check_concrete_carriers: vec![
                            CurrentL2OperationalCliModelCheckCarrierPreview {
                                obligation_kind: "rollback_cut_non_interference",
                                evidence_refs: vec![
                                    format!("fixture:{}", report.sample_id),
                                    format!("runtime_cluster:{}", report.sample_id),
                                ],
                            },
                        ],
                    }
                } else {
                    Self {
                        proof_notebook_review_units: Vec::new(),
                        model_check_concrete_carriers: Vec::new(),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliProofNotebookReviewUnitPreview {
    obligation_kind: &'static str,
    goal_text: String,
    evidence_refs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliModelCheckCarrierPreview {
    obligation_kind: &'static str,
    evidence_refs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliNonAdmissibleMetadata {
    option_ref: String,
    subreason: &'static str,
}

fn render_pretty_summary(summary: &CurrentL2OperationalCliRunSourceSampleSummary) -> String {
    let mut output = String::new();
    writeln!(output, "shell: {}", summary.shell).expect("write to string");
    writeln!(output, "command: {}", summary.command).expect("write to string");
    writeln!(output, "sample: {}", summary.sample).expect("write to string");
    writeln!(output, "sample_path: {}", summary.sample_path).expect("write to string");
    writeln!(output, "host_plan_path: {}", summary.host_plan_path).expect("write to string");
    writeln!(
        output,
        "static_gate: {}",
        summary.checker_floor.static_gate.verdict
    )
    .expect("write to string");
    if !summary.checker_floor.static_gate.reasons.is_empty() {
        writeln!(output, "static_reasons:").expect("write to string");
        for reason in &summary.checker_floor.static_gate.reasons {
            writeln!(output, "- {reason}").expect("write to string");
        }
    }

    match &summary.checker_floor.stage1_reconnect_clusters {
        Some(stage1) => {
            writeln!(output, "stage1_reconnect_clusters:").expect("write to string");
            writeln!(
                output,
                "  same_lineage_floor: {}",
                stage1.same_lineage_floor
            )
            .expect("write to string");
            writeln!(
                output,
                "  missing_option_structure_floor: {}",
                stage1.missing_option_structure_floor
            )
            .expect("write to string");
            writeln!(
                output,
                "  capability_strengthening_floor: {}",
                stage1.capability_strengthening_floor
            )
            .expect("write to string");
        }
        None => {
            writeln!(output, "stage1_reconnect_clusters: not_available").expect("write to string");
        }
    }

    match &summary.checker_floor.stage2_try_rollback_summary {
        Some(stage2) => {
            writeln!(output, "stage2_try_rollback_verdict: {}", stage2.verdict)
                .expect("write to string");
            if stage2.findings.is_empty() {
                writeln!(output, "stage2_try_rollback_findings: []").expect("write to string");
            } else {
                writeln!(output, "stage2_try_rollback_findings:").expect("write to string");
                for finding in &stage2.findings {
                    writeln!(
                        output,
                        "- {}:{}",
                        finding.subject_kind, finding.finding_kind
                    )
                    .expect("write to string");
                }
            }
        }
        None => {
            writeln!(output, "stage2_try_rollback_verdict: not_available")
                .expect("write to string");
        }
    }

    writeln!(
        output,
        "entered_evaluation: {}",
        summary.runtime.entered_evaluation
    )
    .expect("write to string");
    writeln!(
        output,
        "terminal_outcome: {}",
        summary.runtime.terminal_outcome.unwrap_or("none")
    )
    .expect("write to string");
    writeln!(output, "steps_executed: {}", summary.runtime.steps_executed)
        .expect("write to string");
    writeln!(output, "events:").expect("write to string");
    for event in &summary.runtime.events {
        writeln!(output, "- {event}").expect("write to string");
    }
    if summary.runtime.debug_outputs.is_empty() {
        writeln!(output, "debug_outputs: []").expect("write to string");
    } else {
        writeln!(output, "debug_outputs:").expect("write to string");
        for (target, records) in &summary.runtime.debug_outputs {
            writeln!(output, "- {target}:").expect("write to string");
            for record in records {
                writeln!(output, "  - {record}").expect("write to string");
            }
        }
    }
    writeln!(output, "verification_preview:").expect("write to string");
    writeln!(
        output,
        "  formal_hook_status: {}",
        summary.verification_preview.formal_hook_status
    )
    .expect("write to string");
    if let Some(subject_kind) = summary.verification_preview.subject_kind {
        writeln!(output, "  subject_kind: {subject_kind}").expect("write to string");
    } else {
        writeln!(output, "  subject_kind: none").expect("write to string");
    }
    if let Some(subject_ref) = &summary.verification_preview.subject_ref {
        writeln!(output, "  subject_ref: {subject_ref}").expect("write to string");
    } else {
        writeln!(output, "  subject_ref: none").expect("write to string");
    }
    render_obligation_list(
        &mut output,
        "proof_notebook_review_unit_obligations",
        &summary
            .verification_preview
            .proof_notebook_review_unit_obligations,
    );
    render_obligation_list(
        &mut output,
        "model_check_concrete_carrier_obligations",
        &summary
            .verification_preview
            .model_check_concrete_carrier_obligations,
    );
    if let Some(guard_reason) = &summary.verification_preview.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
    writeln!(output, "artifact_preview:").expect("write to string");
    render_proof_review_units(
        &mut output,
        &summary.artifact_preview.proof_notebook_review_units,
    );
    render_model_check_carriers(
        &mut output,
        &summary.artifact_preview.model_check_concrete_carriers,
    );
    if summary.runtime.non_admissible_metadata.is_empty() {
        writeln!(output, "non_admissible_metadata: []").expect("write to string");
    } else {
        writeln!(output, "non_admissible_metadata:").expect("write to string");
        for metadata in &summary.runtime.non_admissible_metadata {
            writeln!(output, "- {}:{}", metadata.option_ref, metadata.subreason)
                .expect("write to string");
        }
    }
    if !summary.runtime.narrative_explanations.is_empty() {
        writeln!(output, "narrative_explanations:").expect("write to string");
        for narrative in &summary.runtime.narrative_explanations {
            writeln!(output, "- {narrative}").expect("write to string");
        }
    }

    output.trim_end().to_string()
}

fn display_path(path: &PathBuf) -> String {
    fs::canonicalize(path)
        .unwrap_or_else(|_| path.clone())
        .display()
        .to_string()
}

fn render_obligation_list(output: &mut String, label: &str, values: &[&'static str]) {
    if values.is_empty() {
        writeln!(output, "  {label}: []").expect("write to string");
    } else {
        writeln!(output, "  {label}:").expect("write to string");
        for value in values {
            writeln!(output, "  - {value}").expect("write to string");
        }
    }
}

fn render_proof_review_units(
    output: &mut String,
    values: &[CurrentL2OperationalCliProofNotebookReviewUnitPreview],
) {
    if values.is_empty() {
        writeln!(output, "  proof_notebook_review_units: []").expect("write to string");
        return;
    }

    writeln!(output, "  proof_notebook_review_units:").expect("write to string");
    for value in values {
        writeln!(output, "  - obligation_kind: {}", value.obligation_kind)
            .expect("write to string");
        writeln!(output, "    goal_text: {}", value.goal_text).expect("write to string");
        if value.evidence_refs.is_empty() {
            writeln!(output, "    evidence_refs: []").expect("write to string");
        } else {
            writeln!(output, "    evidence_refs:").expect("write to string");
            for evidence_ref in &value.evidence_refs {
                writeln!(output, "    - {evidence_ref}").expect("write to string");
            }
        }
    }
}

fn render_model_check_carriers(
    output: &mut String,
    values: &[CurrentL2OperationalCliModelCheckCarrierPreview],
) {
    if values.is_empty() {
        writeln!(output, "  model_check_concrete_carriers: []").expect("write to string");
        return;
    }

    writeln!(output, "  model_check_concrete_carriers:").expect("write to string");
    for value in values {
        writeln!(output, "  - obligation_kind: {}", value.obligation_kind)
            .expect("write to string");
        if value.evidence_refs.is_empty() {
            writeln!(output, "    evidence_refs: []").expect("write to string");
        } else {
            writeln!(output, "    evidence_refs:").expect("write to string");
            for evidence_ref in &value.evidence_refs {
                writeln!(output, "    - {evidence_ref}").expect("write to string");
            }
        }
    }
}

fn static_gate_verdict_name(verdict: StaticGateVerdict) -> &'static str {
    match verdict {
        StaticGateVerdict::Valid => "valid",
        StaticGateVerdict::Malformed => "malformed",
        StaticGateVerdict::Underdeclared => "underdeclared",
    }
}

fn terminal_outcome_name(outcome: TerminalOutcome) -> &'static str {
    match outcome {
        TerminalOutcome::Success => "success",
        TerminalOutcome::ExplicitFailure => "explicit_failure",
        TerminalOutcome::Reject => "Reject",
    }
}

fn event_kind_name(event: EventKind) -> &'static str {
    match event {
        EventKind::PerformSuccess => "perform-success",
        EventKind::PerformFailure => "perform-failure",
        EventKind::Rollback => "rollback",
        EventKind::AtomicCut => "atomic-cut",
        EventKind::Reject => "Reject",
    }
}

fn non_admissible_subreason_name(subreason: NonAdmissibleSubreason) -> &'static str {
    match subreason {
        NonAdmissibleSubreason::AdmitMiss => "admit-miss",
        NonAdmissibleSubreason::LeaseExpired => "lease-expired",
    }
}

fn collect_debug_outputs(
    place_store: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>> {
    place_store
        .iter()
        .filter(|(target, _)| is_debug_output_target(target))
        .map(|(target, records)| (target.clone(), records.clone()))
        .collect()
}

fn is_debug_output_target(target: &str) -> bool {
    target.starts_with("debug_")
        || (target.contains("_debug_")
            && (target.ends_with("_output") || target.ends_with("_pipe")))
}

fn try_rollback_verdict_name(verdict: CurrentL2TryRollbackStructuralVerdict) -> &'static str {
    match verdict {
        CurrentL2TryRollbackStructuralVerdict::NoFindings => "no_findings",
        CurrentL2TryRollbackStructuralVerdict::FindingsPresent => "findings_present",
    }
}

fn try_rollback_subject_kind_name(
    subject_kind: CurrentL2TryRollbackStructuralSubjectKind,
) -> &'static str {
    match subject_kind {
        CurrentL2TryRollbackStructuralSubjectKind::TryFallback => "TryFallback",
        CurrentL2TryRollbackStructuralSubjectKind::AtomicCut => "AtomicCut",
    }
}

fn try_rollback_finding_kind_name(
    finding_kind: CurrentL2TryRollbackStructuralFindingKind,
) -> &'static str {
    match finding_kind {
        CurrentL2TryRollbackStructuralFindingKind::MissingFallbackBody => "missing_fallback_body",
        CurrentL2TryRollbackStructuralFindingKind::DisallowedFallbackPlacement => {
            "disallowed_fallback_placement"
        }
    }
}
