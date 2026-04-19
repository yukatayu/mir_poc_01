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
    surface_preview: CurrentL2OperationalCliSurfacePreviewSummary,
    theorem_result_object_preview: CurrentL2OperationalCliTheoremResultObjectPreviewSummary,
    model_check_public_checker_preview:
        CurrentL2OperationalCliModelCheckPublicCheckerPreviewSummary,
    typed_checker_hint_preview: CurrentL2OperationalCliTypedCheckerHintPreviewSummary,
}

impl CurrentL2OperationalCliRunSourceSampleSummary {
    fn from_report(host_plan_path: &PathBuf, report: CurrentL2SourceSampleRunReport) -> Self {
        let verification_preview =
            CurrentL2OperationalCliVerificationPreviewSummary::from_source_report(&report);
        let artifact_preview =
            CurrentL2OperationalCliArtifactPreviewSummary::from_source_report(&report);
        let surface_preview =
            CurrentL2OperationalCliSurfacePreviewSummary::from_source_report(&report);
        let theorem_result_object_preview =
            CurrentL2OperationalCliTheoremResultObjectPreviewSummary::from_source_report(
                &report,
                &verification_preview,
            );
        let model_check_public_checker_preview =
            CurrentL2OperationalCliModelCheckPublicCheckerPreviewSummary::from_source_report(
                &report,
                &verification_preview,
            );
        let typed_checker_hint_preview =
            CurrentL2OperationalCliTypedCheckerHintPreviewSummary::from_source_report(
                &report,
                &verification_preview,
            );
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
            surface_preview,
            theorem_result_object_preview,
            model_check_public_checker_preview,
            typed_checker_hint_preview,
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
struct CurrentL2OperationalCliSurfacePreviewSummary {
    minimal_companion: CurrentL2OperationalCliSurfacePreviewSection,
    stage_block_secondary: CurrentL2OperationalCliSurfacePreviewSection,
    serial_scope_reserve: CurrentL2OperationalCliSurfacePreviewSection,
}

impl CurrentL2OperationalCliSurfacePreviewSummary {
    fn from_source_report(report: &CurrentL2SourceSampleRunReport) -> Self {
        let verification_preview =
            CurrentL2OperationalCliVerificationPreviewSummary::from_source_report(report);
        let minimal_companion = CurrentL2OperationalCliSurfacePreviewSection::minimal_companion(
            report,
            &verification_preview,
        );
        let stage_block_secondary =
            CurrentL2OperationalCliSurfacePreviewSection::stage_block_secondary(
                report,
                &verification_preview,
            );
        let serial_scope_reserve =
            CurrentL2OperationalCliSurfacePreviewSection::serial_scope_reserve(
                report,
                &verification_preview,
            );
        Self {
            minimal_companion,
            stage_block_secondary,
            serial_scope_reserve,
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliSurfacePreviewSection {
    status: &'static str,
    guard_reason: Option<String>,
    lines: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
}

impl CurrentL2OperationalCliSurfacePreviewSection {
    fn minimal_companion(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
    ) -> Self {
        if authoritative_room_default_sample_reached(report, verification_preview) {
            return Self {
                status: "reached",
                guard_reason: None,
                lines: minimal_companion_lines(&report.sample_id),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
                    "compare_floor:current_l2.experimental_order_handoff_surface".to_string(),
                ],
                guard_refs: vec![
                    "guard:semantic_honesty_first".to_string(),
                    "guard:helper_local_companion_only".to_string(),
                ],
                kept_later_refs: minimal_companion_kept_later_refs(),
            };
        }

        let guard_detail =
            authoritative_room_vertical_slice_guard_reason(report, verification_preview);
        Self {
            status: "guarded_not_reached",
            guard_reason: Some(format!(
                "current minimal companion surface only actualizes reached authoritative-room defaults: {guard_detail}"
            )),
            lines: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.experimental_order_handoff_guard_only".to_string(),
            ],
            guard_refs: vec!["guard:companion_surface_not_reached".to_string()],
            kept_later_refs: minimal_companion_kept_later_refs(),
        }
    }

    fn stage_block_secondary(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
    ) -> Self {
        if authoritative_room_default_sample_reached(report, verification_preview) {
            return Self {
                status: "reached",
                guard_reason: None,
                lines: stage_block_surface_lines(&report.sample_id),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.experimental_order_handoff_surface".to_string(),
                    "compare_floor:current_l2.experimental_stage_block_surface".to_string(),
                ],
                guard_refs: vec![
                    "guard:stage_block_secondary_candidate".to_string(),
                    "guard:helper_local_companion_only".to_string(),
                ],
                kept_later_refs: stage_block_surface_kept_later_refs(),
            };
        }

        let guard_detail =
            authoritative_room_vertical_slice_guard_reason(report, verification_preview);
        Self {
            status: "guarded_not_reached",
            guard_reason: Some(format!(
                "current stage-block secondary surface only actualizes reached authoritative-room defaults: {guard_detail}"
            )),
            lines: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.experimental_stage_block_guard_only".to_string(),
            ],
            guard_refs: vec!["guard:stage_block_surface_not_reached".to_string()],
            kept_later_refs: stage_block_surface_kept_later_refs(),
        }
    }

    fn serial_scope_reserve(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = match sample_id {
            "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh" => {
                authoritative_room_default_sample_reached(report, verification_preview)
            }
            "p09-dice-delegated-rng-provider-placement" => {
                verification_preview.formal_hook_status == "reached"
                    && report.runtime_report.run_report.terminal_outcome
                        == Some(TerminalOutcome::Success)
            }
            _ => false,
        };

        if reached {
            let compare_floor_refs = match sample_id {
                "p09-dice-delegated-rng-provider-placement" => vec![
                    "compare_floor:current_l2.delegated_rng_service.practical".to_string(),
                    "compare_floor:current_l2.witness_provider_route_actual_adoption".to_string(),
                    "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface"
                        .to_string(),
                ],
                _ => vec![
                    "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption"
                        .to_string(),
                    "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface"
                        .to_string(),
                ],
            };
            return Self {
                status: "reached",
                guard_reason: None,
                lines: order_handoff_serial_scope_reserve_surface_lines(sample_id),
                compare_floor_refs,
                guard_refs: order_handoff_serial_scope_reserve_surface_guard_refs(true),
                kept_later_refs: order_handoff_serial_scope_reserve_surface_kept_later_refs(),
            };
        }

        let guard_detail = verification_preview
            .guard_reason
            .clone()
            .unwrap_or_else(|| {
                format!(
                    "sample `{}` did not reach the authoritative-room serial-scope reserve surface",
                    report.sample_id
                )
            });
        Self {
            status: "guarded_not_reached",
            guard_reason: Some(format!(
                "current serial-scope reserve surface only actualizes authoritative-room-specific reached routes (`p07` / `p08` / `p09`): {guard_detail}"
            )),
            lines: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface.guard_only"
                    .to_string(),
            ],
            guard_refs: order_handoff_serial_scope_reserve_surface_guard_refs(false),
            kept_later_refs: order_handoff_serial_scope_reserve_surface_kept_later_refs(),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliTypedCheckerHintPreviewSummary {
    status: &'static str,
    preview_kind: &'static str,
    cluster_kind: Option<&'static str>,
    case_label: Option<&'static str>,
    typed_reason_family_hint: Option<CurrentL2OperationalCliTypedReasonFamilyHintPreview>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliTypedCheckerHintPreviewSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
    ) -> Self {
        match report.sample_id.as_str() {
            "p10-typed-authorized-fingerprint-declassification"
                if verification_preview.formal_hook_status == "reached" =>
            {
                Self {
                    status: "reached",
                    preview_kind: "sample_local_helper_preview",
                    cluster_kind: Some("ifc_authority_release_cluster"),
                    case_label: Some("authority_sensitive_success"),
                    typed_reason_family_hint: Some(
                        CurrentL2OperationalCliTypedReasonFamilyHintPreview {
                            family_refs: vec![
                                "ifc_label_model_family".to_string(),
                                "explicit_authority_declassification_family".to_string(),
                            ],
                            coverage_state: "partial_cluster",
                        },
                    ),
                    evidence_refs: typed_checker_hint_evidence_refs(&report.sample_id),
                    compare_floor_refs: vec![
                        "compare_floor:current_l2.ifc.source_side_authority_pair".to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_reason_family_hint"
                            .to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_family_coverage_state"
                            .to_string(),
                    ],
                    guard_refs: typed_checker_hint_guard_refs(true),
                    kept_later_refs: typed_checker_hint_kept_later_refs(),
                    guard_reason: None,
                }
            }
            "p11-typed-unauthorized-fingerprint-release"
                if verification_preview.formal_hook_status == "reached" =>
            {
                Self {
                    status: "reached",
                    preview_kind: "sample_local_helper_preview",
                    cluster_kind: Some("ifc_authority_release_cluster"),
                    case_label: Some("authority_miss_negative"),
                    typed_reason_family_hint: Some(
                        CurrentL2OperationalCliTypedReasonFamilyHintPreview {
                            family_refs: vec![
                                "ifc_label_model_family".to_string(),
                                "explicit_authority_declassification_family".to_string(),
                            ],
                            coverage_state: "partial_cluster",
                        },
                    ),
                    evidence_refs: typed_checker_hint_evidence_refs(&report.sample_id),
                    compare_floor_refs: vec![
                        "compare_floor:current_l2.ifc.source_side_authority_pair".to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_reason_family_hint"
                            .to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_family_coverage_state"
                            .to_string(),
                    ],
                    guard_refs: typed_checker_hint_guard_refs(true),
                    kept_later_refs: typed_checker_hint_kept_later_refs(),
                    guard_reason: None,
                }
            }
            "p12-typed-classified-fingerprint-publication-block"
                if verification_preview.formal_hook_status == "reached" =>
            {
                Self {
                    status: "reached",
                    preview_kind: "sample_local_helper_preview",
                    cluster_kind: Some("ifc_label_flow_cluster"),
                    case_label: Some("classified_to_public_negative"),
                    typed_reason_family_hint: Some(
                        CurrentL2OperationalCliTypedReasonFamilyHintPreview {
                            family_refs: vec![
                                "ifc_label_model_family".to_string(),
                                "classified_public_label_flow_family".to_string(),
                            ],
                            coverage_state: "full_cluster",
                        },
                    ),
                    evidence_refs: typed_checker_hint_evidence_refs(&report.sample_id),
                    compare_floor_refs: vec![
                        "compare_floor:current_l2.ifc.source_side_label_flow_negative".to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_reason_family_hint"
                            .to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_family_coverage_state"
                            .to_string(),
                    ],
                    guard_refs: typed_checker_hint_guard_refs(true),
                    kept_later_refs: typed_checker_hint_kept_later_refs(),
                    guard_reason: None,
                }
            }
            _ => Self {
                status: "guarded_not_reached",
                preview_kind: "sample_local_helper_preview",
                cluster_kind: None,
                case_label: None,
                typed_reason_family_hint: None,
                evidence_refs: Vec::new(),
                compare_floor_refs: vec![
                    "compare_floor:current_l2.ifc.sample_local_checker_hint_guard_only".to_string(),
                ],
                guard_refs: typed_checker_hint_guard_refs(false),
                kept_later_refs: typed_checker_hint_kept_later_refs(),
                guard_reason: Some(format!(
                    "current typed checker-hint preview only actualizes the sample-local IFC trio (`p10` / `p11` / `p12`) after verification preview reaches runtime try-cut evidence for `{}`",
                    report.sample_id
                )),
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliTheoremResultObjectPreviewSummary {
    status: &'static str,
    preview_kind: &'static str,
    subject_kind: Option<&'static str>,
    subject_ref: Option<String>,
    result_object_route_refs: Vec<String>,
    notebook_payload_preview_refs: Vec<String>,
    proof_object_schema_reserve_refs: Vec<String>,
    actual_adoption_default_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliTheoremResultObjectPreviewSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
    ) -> Self {
        let reached = matches!(
            report.sample_id.as_str(),
            "e5-underdeclared-lineage"
                | "p06-typed-proof-owner-handoff"
                | "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
        ) && verification_preview.formal_hook_status == "reached";

        if reached {
            let subject_ref = verification_preview.subject_ref.clone();
            return Self {
                status: "reached",
                preview_kind: "helper_local_actualization_manifest",
                subject_kind: verification_preview.subject_kind,
                subject_ref: subject_ref.clone(),
                result_object_route_refs: theorem_result_object_route_refs(subject_ref.as_deref()),
                notebook_payload_preview_refs: theorem_result_payload_preview_refs(
                    subject_ref.as_deref(),
                ),
                proof_object_schema_reserve_refs: theorem_result_proof_object_schema_reserve_refs(),
                actual_adoption_default_refs: theorem_result_object_preview_default_refs(),
                evidence_refs: theorem_result_object_preview_evidence_refs(&report.sample_id),
                compare_floor_refs: theorem_result_object_preview_compare_floor_refs(true),
                guard_refs: theorem_result_object_preview_guard_refs(true),
                kept_later_refs: theorem_result_object_preview_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            preview_kind: "helper_local_actualization_manifest",
            subject_kind: verification_preview.subject_kind,
            subject_ref: verification_preview.subject_ref.clone(),
            result_object_route_refs: Vec::new(),
            notebook_payload_preview_refs: Vec::new(),
            proof_object_schema_reserve_refs: Vec::new(),
            actual_adoption_default_refs: Vec::new(),
            evidence_refs: theorem_result_object_preview_evidence_refs(&report.sample_id),
            compare_floor_refs: theorem_result_object_preview_compare_floor_refs(false),
            guard_refs: theorem_result_object_preview_guard_refs(false),
            kept_later_refs: theorem_result_object_preview_kept_later_refs(),
            guard_reason: Some(format!(
                "current theorem result-object preview only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `{}`",
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliModelCheckPublicCheckerPreviewSummary {
    status: &'static str,
    preview_kind: &'static str,
    subject_kind: Option<&'static str>,
    subject_ref: Option<String>,
    checker_artifact_preview_refs: Vec<String>,
    verifier_handoff_reserve_refs: Vec<String>,
    tool_binding_reserve_refs: Vec<String>,
    actual_adoption_default_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliModelCheckPublicCheckerPreviewSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
    ) -> Self {
        let reached = matches!(
            report.sample_id.as_str(),
            "e5-underdeclared-lineage"
                | "p06-typed-proof-owner-handoff"
                | "p07-dice-late-join-visible-history"
                | "p09-dice-delegated-rng-provider-placement"
        ) && verification_preview.formal_hook_status == "reached";

        if reached {
            let subject_ref = verification_preview.subject_ref.clone();
            return Self {
                status: "reached",
                preview_kind: "helper_local_actualization_manifest",
                subject_kind: verification_preview.subject_kind,
                subject_ref: subject_ref.clone(),
                checker_artifact_preview_refs: model_check_public_checker_preview_refs(
                    subject_ref.as_deref(),
                ),
                verifier_handoff_reserve_refs:
                    model_check_public_checker_verifier_handoff_reserve_refs(),
                tool_binding_reserve_refs: model_check_public_checker_tool_binding_reserve_refs(),
                actual_adoption_default_refs: model_check_public_checker_preview_default_refs(),
                evidence_refs: model_check_public_checker_preview_evidence_refs(&report.sample_id),
                compare_floor_refs: model_check_public_checker_preview_compare_floor_refs(true),
                guard_refs: model_check_public_checker_preview_guard_refs(true),
                kept_later_refs: model_check_public_checker_preview_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            preview_kind: "helper_local_actualization_manifest",
            subject_kind: verification_preview.subject_kind,
            subject_ref: verification_preview.subject_ref.clone(),
            checker_artifact_preview_refs: Vec::new(),
            verifier_handoff_reserve_refs: Vec::new(),
            tool_binding_reserve_refs: Vec::new(),
            actual_adoption_default_refs: Vec::new(),
            evidence_refs: model_check_public_checker_preview_evidence_refs(&report.sample_id),
            compare_floor_refs: model_check_public_checker_preview_compare_floor_refs(false),
            guard_refs: model_check_public_checker_preview_guard_refs(false),
            kept_later_refs: model_check_public_checker_preview_kept_later_refs(),
            guard_reason: Some(format!(
                "current model-check public-checker preview only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `{}`",
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliTypedReasonFamilyHintPreview {
    family_refs: Vec<String>,
    coverage_state: &'static str,
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
    writeln!(output, "surface_preview:").expect("write to string");
    render_surface_preview_section(
        &mut output,
        "minimal_companion",
        &summary.surface_preview.minimal_companion,
    );
    render_surface_preview_section(
        &mut output,
        "stage_block_secondary",
        &summary.surface_preview.stage_block_secondary,
    );
    render_surface_preview_section(
        &mut output,
        "serial_scope_reserve",
        &summary.surface_preview.serial_scope_reserve,
    );
    writeln!(output, "theorem_result_object_preview:").expect("write to string");
    render_theorem_result_object_preview(&mut output, &summary.theorem_result_object_preview);
    writeln!(output, "model_check_public_checker_preview:").expect("write to string");
    render_model_check_public_checker_preview(
        &mut output,
        &summary.model_check_public_checker_preview,
    );
    writeln!(output, "typed_checker_hint_preview:").expect("write to string");
    render_typed_checker_hint_preview(&mut output, &summary.typed_checker_hint_preview);
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

fn typed_checker_hint_evidence_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "lean_foundation:CurrentL2IfcSecretExamples.lean".to_string(),
        "helper_preview:typed_checker_hint_preview".to_string(),
    ]
}

fn theorem_result_object_preview_evidence_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:theorem_result_object_preview".to_string(),
        "compare_floor:current_l2.theorem_result_object_preview_actualization".to_string(),
    ]
}

fn theorem_result_object_route_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_result_object_route:{subject_ref}:notebook_consumer_object_first"),
            format!("theorem_result_object_route:{subject_ref}:review_unit_anchor_bundle"),
            format!(
                "theorem_result_object_route:{subject_ref}:consumer_shaped_payload_preview_only"
            ),
            format!("theorem_result_object_route:{subject_ref}:repo_local_emitted_artifact_refs"),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_payload_preview_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_result_payload_preview:{subject_ref}:notebook_consumer_first"),
            format!("theorem_result_payload_preview:{subject_ref}:review_unit_reference_bundle"),
            format!(
                "theorem_result_payload_preview:{subject_ref}:consumer_shaped_payload_preview_only"
            ),
            format!(
                "theorem_result_payload_preview:{subject_ref}:proof_object_public_schema_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_proof_object_schema_reserve_refs() -> Vec<String> {
    vec![
        "proof_object_schema_reserve:brand_neutral_binding_keep".to_string(),
        "proof_object_schema_reserve:proof_object_public_schema_later".to_string(),
        "proof_object_schema_reserve:final_public_verifier_contract_later".to_string(),
    ]
}

fn theorem_result_object_preview_default_refs() -> Vec<String> {
    vec![
        "theorem_result_object_preview_default:notebook_consumer_object_first".to_string(),
        "theorem_result_object_preview_default:consumer_shaped_payload_preview_only".to_string(),
        "theorem_result_object_preview_default:proof_object_schema_reserve_keep".to_string(),
        "theorem_result_object_preview_default:final_public_contract_later".to_string(),
    ]
}

fn theorem_result_object_preview_compare_floor_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption".to_string(),
            "compare_floor:current_l2.theorem_binding_preflight".to_string(),
            "compare_floor:current_l2.theorem_result_object_preview_actualization".to_string(),
        ]
    } else {
        vec!["compare_floor:current_l2.theorem_result_object_preview.guard_only".to_string()]
    }
}

fn theorem_result_object_preview_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:result_object_preview_actualization_only".to_string(),
            "guard:consumer_shaped_payload_preview_only".to_string(),
            "guard:proof_object_schema_reserve_keep".to_string(),
            "guard:concrete_theorem_prover_brand_later".to_string(),
        ]
    } else {
        vec!["guard:theorem_result_object_preview_not_reached".to_string()]
    }
}

fn theorem_result_object_preview_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_public_checker_preview_evidence_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:model_check_public_checker_preview".to_string(),
        "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization"
            .to_string(),
    ]
}

fn model_check_public_checker_preview_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_public_checker_preview:{subject_ref}:consumer_shaped_artifact_preview_only"
            ),
            format!("model_check_public_checker_preview:{subject_ref}:checker_boundary_bundle"),
            format!(
                "model_check_public_checker_preview:{subject_ref}:row_local_property_route_bundle"
            ),
            format!(
                "model_check_public_checker_preview:{subject_ref}:repo_local_emitted_artifact_refs"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_public_checker_verifier_handoff_reserve_refs() -> Vec<String> {
    vec![
        "model_check_verifier_handoff_reserve:public_checker_migration_later".to_string(),
        "model_check_verifier_handoff_reserve:emitted_handoff_artifact_later".to_string(),
        "model_check_verifier_handoff_reserve:runtime_policy_contract_later".to_string(),
    ]
}

fn model_check_public_checker_tool_binding_reserve_refs() -> Vec<String> {
    vec![
        "model_check_tool_binding_reserve:brand_neutral_request_manifest".to_string(),
        "model_check_tool_binding_reserve:concrete_tool_brand_later".to_string(),
        "model_check_tool_binding_reserve:runtime_policy_contract_later".to_string(),
    ]
}

fn model_check_public_checker_preview_default_refs() -> Vec<String> {
    vec![
        "model_check_public_checker_preview_default:consumer_shaped_artifact_preview_only"
            .to_string(),
        "model_check_public_checker_preview_default:verifier_handoff_reserve_keep".to_string(),
        "model_check_public_checker_preview_default:brand_neutral_tool_binding_reserve_keep"
            .to_string(),
        "model_check_public_checker_preview_default:runtime_policy_contract_later".to_string(),
    ]
}

fn model_check_public_checker_preview_compare_floor_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "compare_floor:current_l2.model_check.row_local_property_actual_adoption".to_string(),
            "compare_floor:current_l2.model_check.second_line_concretization".to_string(),
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization"
                .to_string(),
        ]
    } else {
        vec![
            "compare_floor:current_l2.model_check.public_checker_artifact_preview.guard_only"
                .to_string(),
        ]
    }
}

fn model_check_public_checker_preview_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:public_checker_artifact_preview_actualization_only".to_string(),
            "guard:verifier_handoff_reserve_keep".to_string(),
            "guard:brand_neutral_tool_binding_reserve_keep".to_string(),
            "guard:runtime_policy_contract_later".to_string(),
        ]
    } else {
        vec!["guard:model_check_public_checker_artifact_preview_not_reached".to_string()]
    }
}

fn model_check_public_checker_preview_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:first_settled_property_language".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:production_checker_runtime_policy_contract".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn typed_checker_hint_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:sample_local_helper_preview_only".to_string(),
            "guard:checker_adjacent_principal".to_string(),
            "guard:stronger_typed_surface_not_principal".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec!["guard:typed_checker_hint_preview_not_reached".to_string()]
    }
}

fn typed_checker_hint_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:actual_checker_payload_family".to_string(),
        "kept_later:checker_supported_kind_summary".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
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

fn render_theorem_result_object_preview(
    output: &mut String,
    preview: &CurrentL2OperationalCliTheoremResultObjectPreviewSummary,
) {
    writeln!(output, "  status: {}", preview.status).expect("write to string");
    writeln!(output, "  preview_kind: {}", preview.preview_kind).expect("write to string");
    if let Some(subject_kind) = preview.subject_kind {
        writeln!(output, "  subject_kind: {subject_kind}").expect("write to string");
    } else {
        writeln!(output, "  subject_kind: none").expect("write to string");
    }
    if let Some(subject_ref) = &preview.subject_ref {
        writeln!(output, "  subject_ref: {subject_ref}").expect("write to string");
    } else {
        writeln!(output, "  subject_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "result_object_route_refs",
        &preview.result_object_route_refs,
        1,
    );
    render_string_list(
        output,
        "notebook_payload_preview_refs",
        &preview.notebook_payload_preview_refs,
        1,
    );
    render_string_list(
        output,
        "proof_object_schema_reserve_refs",
        &preview.proof_object_schema_reserve_refs,
        1,
    );
    render_string_list(
        output,
        "actual_adoption_default_refs",
        &preview.actual_adoption_default_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &preview.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &preview.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &preview.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &preview.kept_later_refs, 1);
    if let Some(guard_reason) = &preview.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_model_check_public_checker_preview(
    output: &mut String,
    preview: &CurrentL2OperationalCliModelCheckPublicCheckerPreviewSummary,
) {
    writeln!(output, "  status: {}", preview.status).expect("write to string");
    writeln!(output, "  preview_kind: {}", preview.preview_kind).expect("write to string");
    if let Some(subject_kind) = preview.subject_kind {
        writeln!(output, "  subject_kind: {subject_kind}").expect("write to string");
    } else {
        writeln!(output, "  subject_kind: none").expect("write to string");
    }
    if let Some(subject_ref) = &preview.subject_ref {
        writeln!(output, "  subject_ref: {subject_ref}").expect("write to string");
    } else {
        writeln!(output, "  subject_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "checker_artifact_preview_refs",
        &preview.checker_artifact_preview_refs,
        1,
    );
    render_string_list(
        output,
        "verifier_handoff_reserve_refs",
        &preview.verifier_handoff_reserve_refs,
        1,
    );
    render_string_list(
        output,
        "tool_binding_reserve_refs",
        &preview.tool_binding_reserve_refs,
        1,
    );
    render_string_list(
        output,
        "actual_adoption_default_refs",
        &preview.actual_adoption_default_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &preview.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &preview.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &preview.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &preview.kept_later_refs, 1);
    if let Some(guard_reason) = &preview.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
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

fn render_typed_checker_hint_preview(
    output: &mut String,
    preview: &CurrentL2OperationalCliTypedCheckerHintPreviewSummary,
) {
    writeln!(output, "  status: {}", preview.status).expect("write to string");
    writeln!(output, "  preview_kind: {}", preview.preview_kind).expect("write to string");
    if let Some(cluster_kind) = preview.cluster_kind {
        writeln!(output, "  cluster_kind: {cluster_kind}").expect("write to string");
    } else {
        writeln!(output, "  cluster_kind: none").expect("write to string");
    }
    if let Some(case_label) = preview.case_label {
        writeln!(output, "  case_label: {case_label}").expect("write to string");
    } else {
        writeln!(output, "  case_label: none").expect("write to string");
    }
    match &preview.typed_reason_family_hint {
        Some(hint) => {
            writeln!(output, "  typed_reason_family_hint:").expect("write to string");
            render_string_list(output, "family_refs", &hint.family_refs, 2);
            writeln!(output, "    coverage_state: {}", hint.coverage_state)
                .expect("write to string");
        }
        None => {
            writeln!(output, "  typed_reason_family_hint: none").expect("write to string");
        }
    }
    render_string_list(output, "evidence_refs", &preview.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &preview.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &preview.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &preview.kept_later_refs, 1);
    if let Some(guard_reason) = &preview.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_surface_preview_section(
    output: &mut String,
    label: &str,
    section: &CurrentL2OperationalCliSurfacePreviewSection,
) {
    writeln!(output, "  {label}:").expect("write to string");
    writeln!(output, "    status: {}", section.status).expect("write to string");
    if let Some(guard_reason) = &section.guard_reason {
        writeln!(output, "    guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "    guard_reason: none").expect("write to string");
    }
    render_surface_string_list(output, "lines", &section.lines);
    render_surface_string_list(output, "compare_floor_refs", &section.compare_floor_refs);
    render_surface_string_list(output, "guard_refs", &section.guard_refs);
    render_surface_string_list(output, "kept_later_refs", &section.kept_later_refs);
}

fn render_surface_string_list(output: &mut String, label: &str, values: &[String]) {
    if values.is_empty() {
        writeln!(output, "    {label}: []").expect("write to string");
        return;
    }

    writeln!(output, "    {label}:").expect("write to string");
    for value in values {
        writeln!(output, "    - {value}").expect("write to string");
    }
}

fn render_string_list(output: &mut String, label: &str, values: &[String], base_indent: usize) {
    let indent = "  ".repeat(base_indent);
    if values.is_empty() {
        writeln!(output, "{indent}{label}: []").expect("write to string");
        return;
    }

    writeln!(output, "{indent}{label}:").expect("write to string");
    for value in values {
        writeln!(output, "{indent}- {value}").expect("write to string");
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

fn authoritative_room_default_sample_reached(
    report: &CurrentL2SourceSampleRunReport,
    verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
) -> bool {
    matches!(
        report.sample_id.as_str(),
        "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh"
    ) && verification_preview.formal_hook_status == "reached"
}

fn authoritative_room_vertical_slice_guard_reason(
    report: &CurrentL2SourceSampleRunReport,
    verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
) -> String {
    let guard_detail = verification_preview
        .guard_reason
        .clone()
        .unwrap_or_else(|| {
            format!(
                "current default samples (`p07` / `p08`) were not reached for `{}`",
                report.sample_id
            )
        });
    format!(
        "current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): {guard_detail}"
    )
}

fn minimal_companion_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "profile authoritative_room_default".to_string(),
            "activation authority-ack".to_string(),
            "authority single_room_authority".to_string(),
            "consistency authoritative_serial_transition".to_string(),
            "rng authority_rng".to_string(),
            "publication publish_roll_result@dice_state".to_string(),
            "handoff handoff_dice_authority@dice_state".to_string(),
            "late_join published_history_visible_as_past".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "profile authoritative_room_default".to_string(),
            "activation authority-ack".to_string(),
            "authority single_room_authority".to_string(),
            "consistency authoritative_serial_transition".to_string(),
            "rng authority_rng".to_string(),
            "rollback stale_reconnect".to_string(),
            "refresh refresh_owner_snapshot@dice_state".to_string(),
            "replay stale_incompatible_replay_invalidated".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn minimal_companion_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn stage_block_surface_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "transition handoff_turn(dice_owner = player_a)".to_string(),
            "stage publish:".to_string(),
            "  publish publish_roll_result@dice_state".to_string(),
            "stage handoff:".to_string(),
            "  handoff handoff_dice_authority@dice_state".to_string(),
            "    after publish(publish_roll_result@dice_state)".to_string(),
            "    requires witness(publish_roll_result@dice_state)".to_string(),
            "stage observe:".to_string(),
            "  observe late_join_view@dice_state".to_string(),
            "    after handoff(handoff_dice_authority@dice_state)".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "transition reconnect_refresh(dice_owner = player_a)".to_string(),
            "stage rollback:".to_string(),
            "  rollback stale_reconnect".to_string(),
            "stage refresh:".to_string(),
            "  refresh refresh_owner_snapshot@dice_state".to_string(),
            "    after rollback(stale_reconnect)".to_string(),
            "stage replay:".to_string(),
            "  invalidate stale_incompatible_replay@dice_state".to_string(),
            "    after refresh(refresh_owner_snapshot@dice_state)".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn stage_block_surface_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn order_handoff_serial_scope_reserve_surface_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "serial on dice_authority {".to_string(),
            "  publish publish_roll_result@dice_state".to_string(),
            "  handoff handoff_dice_authority@dice_state".to_string(),
            "    requires witness(publish_roll_result@dice_state)".to_string(),
            "  observe late_join_view@dice_state".to_string(),
            "}".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "serial on dice_authority {".to_string(),
            "  rollback stale_reconnect".to_string(),
            "  refresh refresh_owner_snapshot@dice_state".to_string(),
            "  invalidate stale_incompatible_replay@dice_state".to_string(),
            "}".to_string(),
        ],
        "p09-dice-delegated-rng-provider-placement" => vec![
            "serial on dice_authority {".to_string(),
            "  fetch fetch_provider_roll@delegated_rng".to_string(),
            "  publish publish_roll_result@dice_state".to_string(),
            "  handoff handoff_dice_authority@dice_state".to_string(),
            "}".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn order_handoff_serial_scope_reserve_surface_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:serial_scope_room_specific_reserve".to_string(),
            "guard:principal_edge_row_surface_unchanged".to_string(),
            "guard:helper_local_surface_only".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
        ]
    } else {
        vec!["guard:serial_scope_reserve_surface_not_reached".to_string()]
    }
}

fn order_handoff_serial_scope_reserve_surface_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:serial_scope_public_promotion".to_string(),
        "kept_later:serial_scope_beyond_authoritative_room".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_artifact_schema".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
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
