use std::{
    collections::BTreeMap,
    fmt::{self, Write},
    fs,
    path::PathBuf,
};

use mir_ast::current_l2::current_l2_first_tranche_manifest;
use serde::Serialize;

use crate::current_l2::{
    CurrentL2RuntimeSkeletonReport, CurrentL2SourceSampleRunReport,
    CurrentL2TryRollbackStructuralFindingKind, CurrentL2TryRollbackStructuralSubjectKind,
    CurrentL2TryRollbackStructuralSummary, CurrentL2TryRollbackStructuralVerdict,
    current_l2_checker_runtime_first_tranche_manifest,
    current_l2_compile_ready_verification_and_formal_hook_manifest,
    current_l2_phase6_next_reopen_sequencing_manifest,
    current_l2_phase6_parser_side_followup_package_sequencing_manifest,
    current_l2_phase6_reserve_formal_tool_binding_inventory_manifest,
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
    authoritative_room_first_scenario_actual_adoption:
        CurrentL2OperationalCliAuthoritativeRoomFirstScenarioActualAdoptionSummary,
    authoritative_room_reserve_strengthening_lane:
        CurrentL2OperationalCliAuthoritativeRoomReserveStrengtheningLaneSummary,
    order_handoff_source_surface_artifact_actual_adoption:
        CurrentL2OperationalCliOrderHandoffSourceSurfaceArtifactActualAdoptionSummary,
    order_handoff_witness_provider_public_seam_compression:
        CurrentL2OperationalCliOrderHandoffWitnessProviderPublicSeamCompressionSummary,
    theorem_result_object_preview: CurrentL2OperationalCliTheoremResultObjectPreviewSummary,
    model_check_public_checker_preview:
        CurrentL2OperationalCliModelCheckPublicCheckerPreviewSummary,
    theorem_final_public_contract_reopen_threshold:
        CurrentL2OperationalCliTheoremFinalPublicContractReopenThresholdSummary,
    model_check_final_public_contract_reopen_threshold:
        CurrentL2OperationalCliModelCheckFinalPublicContractReopenThresholdSummary,
    typed_checker_hint_preview: CurrentL2OperationalCliTypedCheckerHintPreviewSummary,
    actual_checker_payload_family_threshold:
        CurrentL2OperationalCliActualCheckerPayloadFamilyThresholdSummary,
    actual_checker_payload_row_family_threshold:
        CurrentL2OperationalCliActualCheckerPayloadRowFamilyThresholdSummary,
    actual_checker_payload_row_detail_threshold:
        CurrentL2OperationalCliActualCheckerPayloadRowDetailThresholdSummary,
    actual_checker_payload_row_body_threshold:
        CurrentL2OperationalCliActualCheckerPayloadRowBodyThresholdSummary,
    actual_checker_payload_supported_kind_summary_threshold:
        CurrentL2OperationalCliActualCheckerPayloadSupportedKindSummaryThresholdSummary,
    actual_checker_payload_public_schema_sketch_threshold:
        CurrentL2OperationalCliActualCheckerPayloadPublicSchemaSketchThresholdSummary,
    actual_public_checker_api_sketch_threshold:
        CurrentL2OperationalCliActualPublicCheckerApiSketchThresholdSummary,
    actual_public_checker_entry_criteria_threshold:
        CurrentL2OperationalCliActualPublicCheckerEntryCriteriaThresholdSummary,
    actual_public_checker_command_surface_threshold:
        CurrentL2OperationalCliActualPublicCheckerCommandSurfaceThresholdSummary,
    actual_shared_output_contract_threshold:
        CurrentL2OperationalCliActualSharedOutputContractThresholdSummary,
    actual_public_checker_boundary_threshold:
        CurrentL2OperationalCliActualPublicCheckerBoundaryThresholdSummary,
    actual_verifier_handoff_surface_threshold:
        CurrentL2OperationalCliActualVerifierHandoffSurfaceThresholdSummary,
    actual_minimal_parser_subset_freeze_threshold:
        CurrentL2OperationalCliActualMinimalParserSubsetFreezeThresholdSummary,
    actual_parser_to_checker_reconnect_freeze_threshold:
        CurrentL2OperationalCliActualParserToCheckerReconnectFreezeThresholdSummary,
    actual_phase1_semantics_closeout_threshold:
        CurrentL2OperationalCliActualPhase1SemanticsCloseoutThresholdSummary,
    actual_phase2_parser_free_poc_closeout_threshold:
        CurrentL2OperationalCliActualPhase2ParserFreePocCloseoutThresholdSummary,
    actual_phase4_shared_space_self_driven_closeout_threshold:
        CurrentL2OperationalCliActualPhase4SharedSpaceSelfDrivenCloseoutThresholdSummary,
    actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
        CurrentL2OperationalCliActualPhase5ProofProtocolRuntimePolicyHandoffCloseoutThresholdSummary,
    actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
        CurrentL2OperationalCliActualPhase6ActualParserAstCarrierFirstTrancheThresholdSummary,
    actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
        CurrentL2OperationalCliActualPhase6ActualCheckerRuntimeSkeletonFirstTrancheThresholdSummary,
    actual_phase6_compile_ready_verification_and_formal_hook_threshold:
        CurrentL2OperationalCliActualPhase6CompileReadyVerificationAndFormalHookThresholdSummary,
    actual_phase6_next_reopen_sequencing_threshold:
        CurrentL2OperationalCliActualPhase6NextReopenSequencingThresholdSummary,
    actual_phase6_reserve_formal_tool_binding_inventory_threshold:
        CurrentL2OperationalCliActualPhase6ReserveFormalToolBindingInventoryThresholdSummary,
    actual_phase6_parser_side_followup_package_sequencing_threshold:
        CurrentL2OperationalCliActualPhase6ParserSideFollowupPackageSequencingThresholdSummary,
}

impl CurrentL2OperationalCliRunSourceSampleSummary {
    fn from_report(host_plan_path: &PathBuf, report: CurrentL2SourceSampleRunReport) -> Self {
        let verification_preview =
            CurrentL2OperationalCliVerificationPreviewSummary::from_source_report(&report);
        let artifact_preview =
            CurrentL2OperationalCliArtifactPreviewSummary::from_source_report(&report);
        let surface_preview =
            CurrentL2OperationalCliSurfacePreviewSummary::from_source_report(&report);
        let authoritative_room_first_scenario_actual_adoption =
            CurrentL2OperationalCliAuthoritativeRoomFirstScenarioActualAdoptionSummary::from_source_report(
                &report,
                &verification_preview,
                &artifact_preview,
            );
        let authoritative_room_reserve_strengthening_lane =
            CurrentL2OperationalCliAuthoritativeRoomReserveStrengtheningLaneSummary::from_source_report(
                &report,
                &verification_preview,
                &artifact_preview,
            );
        let order_handoff_source_surface_artifact_actual_adoption =
            CurrentL2OperationalCliOrderHandoffSourceSurfaceArtifactActualAdoptionSummary::from_source_report(
                &report,
                &verification_preview,
                &artifact_preview,
            );
        let order_handoff_witness_provider_public_seam_compression =
            CurrentL2OperationalCliOrderHandoffWitnessProviderPublicSeamCompressionSummary::from_source_report(
                &report,
                &verification_preview,
                &artifact_preview,
                &surface_preview,
            );
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
        let theorem_final_public_contract_reopen_threshold =
            CurrentL2OperationalCliTheoremFinalPublicContractReopenThresholdSummary::from_source_report(
                &report,
                &verification_preview,
            );
        let model_check_final_public_contract_reopen_threshold =
            CurrentL2OperationalCliModelCheckFinalPublicContractReopenThresholdSummary::from_source_report(
                &report,
                &verification_preview,
            );
        let typed_checker_hint_preview =
            CurrentL2OperationalCliTypedCheckerHintPreviewSummary::from_source_report(
                &report,
                &verification_preview,
            );
        let actual_checker_payload_family_threshold =
            CurrentL2OperationalCliActualCheckerPayloadFamilyThresholdSummary::from_source_report(
                &report,
                &verification_preview,
                &typed_checker_hint_preview,
            );
        let actual_checker_payload_row_family_threshold =
            CurrentL2OperationalCliActualCheckerPayloadRowFamilyThresholdSummary::from_source_report(
                &report,
                &actual_checker_payload_family_threshold,
            );
        let actual_checker_payload_row_detail_threshold =
            CurrentL2OperationalCliActualCheckerPayloadRowDetailThresholdSummary::from_source_report(
                &report,
                &actual_checker_payload_row_family_threshold,
            );
        let actual_checker_payload_row_body_threshold =
            CurrentL2OperationalCliActualCheckerPayloadRowBodyThresholdSummary::from_source_report(
                &report,
                &actual_checker_payload_row_detail_threshold,
            );
        let actual_checker_payload_supported_kind_summary_threshold =
            CurrentL2OperationalCliActualCheckerPayloadSupportedKindSummaryThresholdSummary::from_source_report(
                &report,
                &actual_checker_payload_row_body_threshold,
            );
        let actual_checker_payload_public_schema_sketch_threshold =
            CurrentL2OperationalCliActualCheckerPayloadPublicSchemaSketchThresholdSummary::from_source_report(
                &report,
                &actual_checker_payload_supported_kind_summary_threshold,
            );
        let actual_public_checker_api_sketch_threshold =
            CurrentL2OperationalCliActualPublicCheckerApiSketchThresholdSummary::from_source_report(
                &report,
                &verification_preview,
                &actual_checker_payload_public_schema_sketch_threshold,
            );
        let actual_public_checker_entry_criteria_threshold =
            CurrentL2OperationalCliActualPublicCheckerEntryCriteriaThresholdSummary::from_source_report(
                &report,
                &actual_public_checker_api_sketch_threshold,
            );
        let actual_public_checker_command_surface_threshold =
            CurrentL2OperationalCliActualPublicCheckerCommandSurfaceThresholdSummary::from_source_report(
                &report,
                &actual_public_checker_entry_criteria_threshold,
            );
        let actual_shared_output_contract_threshold =
            CurrentL2OperationalCliActualSharedOutputContractThresholdSummary::from_source_report(
                &report,
                &actual_public_checker_command_surface_threshold,
            );
        let actual_public_checker_boundary_threshold =
            CurrentL2OperationalCliActualPublicCheckerBoundaryThresholdSummary::from_source_report(
                &report,
                &actual_shared_output_contract_threshold,
            );
        let actual_verifier_handoff_surface_threshold =
            CurrentL2OperationalCliActualVerifierHandoffSurfaceThresholdSummary::from_source_report(
                &report,
                &actual_public_checker_boundary_threshold,
            );
        let actual_minimal_parser_subset_freeze_threshold =
            CurrentL2OperationalCliActualMinimalParserSubsetFreezeThresholdSummary::from_source_report(
                &report,
                &actual_verifier_handoff_surface_threshold,
            );
        let actual_parser_to_checker_reconnect_freeze_threshold =
            CurrentL2OperationalCliActualParserToCheckerReconnectFreezeThresholdSummary::from_source_report(
                &report,
                &actual_minimal_parser_subset_freeze_threshold,
            );
        let actual_phase1_semantics_closeout_threshold =
            CurrentL2OperationalCliActualPhase1SemanticsCloseoutThresholdSummary::from_source_report(
                &report,
                &actual_parser_to_checker_reconnect_freeze_threshold,
            );
        let actual_phase2_parser_free_poc_closeout_threshold =
            CurrentL2OperationalCliActualPhase2ParserFreePocCloseoutThresholdSummary::from_source_report(
                &report,
                &actual_phase1_semantics_closeout_threshold,
            );
        let actual_phase4_shared_space_self_driven_closeout_threshold =
            CurrentL2OperationalCliActualPhase4SharedSpaceSelfDrivenCloseoutThresholdSummary::from_source_report(
                &report,
                &surface_preview,
            );
        let actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold =
            CurrentL2OperationalCliActualPhase5ProofProtocolRuntimePolicyHandoffCloseoutThresholdSummary::from_source_report(
                &report,
                &actual_phase4_shared_space_self_driven_closeout_threshold,
            );
        let actual_phase6_actual_parser_ast_carrier_first_tranche_threshold =
            CurrentL2OperationalCliActualPhase6ActualParserAstCarrierFirstTrancheThresholdSummary::from_source_report(
                &report,
                &actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold,
            );
        let actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold =
            CurrentL2OperationalCliActualPhase6ActualCheckerRuntimeSkeletonFirstTrancheThresholdSummary::from_source_report(
                &report,
                &actual_phase6_actual_parser_ast_carrier_first_tranche_threshold,
            );
        let actual_phase6_compile_ready_verification_and_formal_hook_threshold =
            CurrentL2OperationalCliActualPhase6CompileReadyVerificationAndFormalHookThresholdSummary::from_source_report(
                &report,
                &actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold,
            );
        let actual_phase6_next_reopen_sequencing_threshold =
            CurrentL2OperationalCliActualPhase6NextReopenSequencingThresholdSummary::from_source_report(
                &report,
                &actual_phase6_compile_ready_verification_and_formal_hook_threshold,
            );
        let actual_phase6_reserve_formal_tool_binding_inventory_threshold =
            CurrentL2OperationalCliActualPhase6ReserveFormalToolBindingInventoryThresholdSummary::from_source_report(
                &report,
                &actual_phase6_next_reopen_sequencing_threshold,
            );
        let actual_phase6_parser_side_followup_package_sequencing_threshold =
            CurrentL2OperationalCliActualPhase6ParserSideFollowupPackageSequencingThresholdSummary::from_source_report(
                &report,
                &actual_phase6_reserve_formal_tool_binding_inventory_threshold,
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
            authoritative_room_first_scenario_actual_adoption,
            authoritative_room_reserve_strengthening_lane,
            order_handoff_source_surface_artifact_actual_adoption,
            order_handoff_witness_provider_public_seam_compression,
            theorem_result_object_preview,
            model_check_public_checker_preview,
            theorem_final_public_contract_reopen_threshold,
            model_check_final_public_contract_reopen_threshold,
            typed_checker_hint_preview,
            actual_checker_payload_family_threshold,
            actual_checker_payload_row_family_threshold,
            actual_checker_payload_row_detail_threshold,
            actual_checker_payload_row_body_threshold,
            actual_checker_payload_supported_kind_summary_threshold,
            actual_checker_payload_public_schema_sketch_threshold,
            actual_public_checker_api_sketch_threshold,
            actual_public_checker_entry_criteria_threshold,
            actual_public_checker_command_surface_threshold,
            actual_shared_output_contract_threshold,
            actual_public_checker_boundary_threshold,
            actual_verifier_handoff_surface_threshold,
            actual_minimal_parser_subset_freeze_threshold,
            actual_parser_to_checker_reconnect_freeze_threshold,
            actual_phase1_semantics_closeout_threshold,
            actual_phase2_parser_free_poc_closeout_threshold,
            actual_phase4_shared_space_self_driven_closeout_threshold,
            actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold,
            actual_phase6_actual_parser_ast_carrier_first_tranche_threshold,
            actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold,
            actual_phase6_compile_ready_verification_and_formal_hook_threshold,
            actual_phase6_next_reopen_sequencing_threshold,
            actual_phase6_reserve_formal_tool_binding_inventory_threshold,
            actual_phase6_parser_side_followup_package_sequencing_threshold,
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

#[derive(Clone, Copy)]
struct CurrentL2FirstStrongTypingSampleManifest {
    cluster_kind: &'static str,
    case_label: &'static str,
    family_refs: &'static [&'static str],
    coverage_state: &'static str,
    primary_compare_floor_ref: &'static str,
    foundation_evidence_ref: &'static str,
    selected_option_ref: &'static str,
    visibility_target_ref: &'static str,
}

fn current_l2_first_strong_typing_sample_manifest(
    sample_id: &str,
) -> Option<CurrentL2FirstStrongTypingSampleManifest> {
    match sample_id {
        "p10-typed-authorized-fingerprint-declassification" => {
            Some(CurrentL2FirstStrongTypingSampleManifest {
                cluster_kind: "ifc_authority_release_cluster",
                case_label: "authority_sensitive_success",
                family_refs: &[
                    "ifc_label_model_family",
                    "explicit_authority_declassification_family",
                ],
                coverage_state: "partial_cluster",
                primary_compare_floor_ref: "compare_floor:current_l2.ifc.source_side_authority_pair",
                foundation_evidence_ref: "lean_foundation:CurrentL2IfcSecretExamples.lean",
                selected_option_ref: "release_authority",
                visibility_target_ref: "room_members",
            })
        }
        "p11-typed-unauthorized-fingerprint-release" => {
            Some(CurrentL2FirstStrongTypingSampleManifest {
                cluster_kind: "ifc_authority_release_cluster",
                case_label: "authority_miss_negative",
                family_refs: &[
                    "ifc_label_model_family",
                    "explicit_authority_declassification_family",
                ],
                coverage_state: "partial_cluster",
                primary_compare_floor_ref: "compare_floor:current_l2.ifc.source_side_authority_pair",
                foundation_evidence_ref: "lean_foundation:CurrentL2IfcSecretExamples.lean",
                selected_option_ref: "fingerprint_holder",
                visibility_target_ref: "room_members",
            })
        }
        "p12-typed-classified-fingerprint-publication-block" => {
            Some(CurrentL2FirstStrongTypingSampleManifest {
                cluster_kind: "ifc_label_flow_cluster",
                case_label: "classified_to_public_negative",
                family_refs: &[
                    "ifc_label_model_family",
                    "classified_public_label_flow_family",
                ],
                coverage_state: "full_cluster",
                primary_compare_floor_ref: "compare_floor:current_l2.ifc.source_side_label_flow_negative",
                foundation_evidence_ref: "lean_foundation:CurrentL2IfcSecretExamples.lean",
                selected_option_ref: "classified_holder",
                visibility_target_ref: "public_board",
            })
        }
        "p15-typed-capture-escape-rejected" => Some(CurrentL2FirstStrongTypingSampleManifest {
            cluster_kind: "capture_lifetime_cluster",
            case_label: "capture_escape_negative",
            family_refs: &["capture_set_constraint_family", "lifetime_preorder_family"],
            coverage_state: "partial_cluster",
            primary_compare_floor_ref: "compare_floor:current_l2.typed.capture_lifetime_escape_negative",
            foundation_evidence_ref: "spec_example:current_l2_first_strong_typing_layer_finite_index_spine_default",
            selected_option_ref: "session_owner",
            visibility_target_ref: "room_members",
        }),
        "p16-typed-remote-call-budget-exceeded" => Some(CurrentL2FirstStrongTypingSampleManifest {
            cluster_kind: "simple_cost_bound_cluster",
            case_label: "remote_call_budget_negative",
            family_refs: &["simple_cost_bound_family", "external_effect_budget_family"],
            coverage_state: "partial_cluster",
            primary_compare_floor_ref: "compare_floor:current_l2.typed.simple_cost_bound_negative",
            foundation_evidence_ref: "spec_example:current_l2_first_strong_typing_layer_finite_index_spine_default",
            selected_option_ref: "quota_owner",
            visibility_target_ref: "remote_api_gateway",
        }),
        _ => None,
    }
}

fn current_l2_first_strong_typing_sample_reached(sample_id: &str) -> bool {
    current_l2_first_strong_typing_sample_manifest(sample_id).is_some()
}

fn current_l2_first_strong_typing_sample_guard_label() -> &'static str {
    "first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`)"
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
        if verification_preview.formal_hook_status == "reached" {
            if let Some(manifest) =
                current_l2_first_strong_typing_sample_manifest(report.sample_id.as_str())
            {
                return Self {
                    status: "reached",
                    preview_kind: "sample_local_helper_preview",
                    cluster_kind: Some(manifest.cluster_kind),
                    case_label: Some(manifest.case_label),
                    typed_reason_family_hint: Some(
                        CurrentL2OperationalCliTypedReasonFamilyHintPreview {
                            family_refs: manifest
                                .family_refs
                                .iter()
                                .map(|family_ref| (*family_ref).to_string())
                                .collect(),
                            coverage_state: manifest.coverage_state,
                        },
                    ),
                    evidence_refs: typed_checker_hint_evidence_refs(&report.sample_id),
                    compare_floor_refs: vec![
                        manifest.primary_compare_floor_ref.to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_reason_family_hint"
                            .to_string(),
                        "compare_floor:current_l2.checker_cluster.typed_family_coverage_state"
                            .to_string(),
                    ],
                    guard_refs: typed_checker_hint_guard_refs(true),
                    kept_later_refs: typed_checker_hint_kept_later_refs(),
                    guard_reason: None,
                };
            }
        }

        Self {
            status: "guarded_not_reached",
            preview_kind: "sample_local_helper_preview",
            cluster_kind: None,
            case_label: None,
            typed_reason_family_hint: None,
            evidence_refs: Vec::new(),
            compare_floor_refs: vec![
                "compare_floor:current_l2.typed.sample_local_checker_hint_guard_only".to_string(),
            ],
            guard_refs: typed_checker_hint_guard_refs(false),
            kept_later_refs: typed_checker_hint_kept_later_refs(),
            guard_reason: Some(format!(
                "current typed checker-hint preview only actualizes the sample-local {} after verification preview reaches runtime try-cut evidence for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliAuthoritativeRoomReserveStrengtheningLaneSummary {
    status: &'static str,
    lane_kind: &'static str,
    witness_strengthening_status: &'static str,
    delegated_rng_service_status: &'static str,
    model_check_second_line_status: &'static str,
    witness_strengthening_refs: Vec<String>,
    delegated_rng_service_refs: Vec<String>,
    model_check_second_line_refs: Vec<String>,
    first_line_boundary_refs: Vec<String>,
    reserve_boundary_refs: Vec<String>,
    repo_local_emitted_artifact_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliAuthoritativeRoomReserveStrengtheningLaneSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
        artifact_preview: &CurrentL2OperationalCliArtifactPreviewSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let witness_reached =
            authoritative_room_default_sample_reached(report, verification_preview)
                && sample_id == "p07-dice-late-join-visible-history";
        let delegated_reached = sample_id == "p09-dice-delegated-rng-provider-placement"
            && verification_preview.formal_hook_status == "reached"
            && report.runtime_report.run_report.terminal_outcome == Some(TerminalOutcome::Success);
        let model_check_reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && verification_preview.formal_hook_status == "reached"
            && report.runtime_report.run_report.terminal_outcome == Some(TerminalOutcome::Success);
        let lane_reached = witness_reached || delegated_reached || model_check_reached;

        Self {
            status: if lane_reached {
                "reached"
            } else {
                "guarded_not_reached"
            },
            lane_kind: "helper_local_reserve_strengthening_lane_manifest",
            witness_strengthening_status: if witness_reached {
                "reached"
            } else {
                "guarded_not_reached"
            },
            delegated_rng_service_status: if delegated_reached {
                "reached"
            } else {
                "guarded_not_reached"
            },
            model_check_second_line_status: if model_check_reached {
                "reached"
            } else {
                "guarded_not_reached"
            },
            witness_strengthening_refs: authoritative_room_reserve_strengthening_witness_refs(
                sample_id,
                witness_reached,
            ),
            delegated_rng_service_refs: authoritative_room_reserve_strengthening_delegated_rng_refs(
                sample_id,
                delegated_reached,
            ),
            model_check_second_line_refs: authoritative_room_reserve_strengthening_model_check_refs(
                sample_id,
                model_check_reached,
            ),
            first_line_boundary_refs:
                authoritative_room_reserve_strengthening_first_line_boundary_refs(sample_id),
            reserve_boundary_refs: authoritative_room_reserve_strengthening_boundary_refs(),
            repo_local_emitted_artifact_refs: if lane_reached {
                order_handoff_repo_local_emitted_artifact_refs(sample_id, artifact_preview)
            } else {
                Vec::new()
            },
            compare_floor_refs: authoritative_room_reserve_strengthening_compare_floor_refs(
                sample_id,
                witness_reached,
                delegated_reached,
                model_check_reached,
            ),
            guard_refs: authoritative_room_reserve_strengthening_guard_refs(
                witness_reached,
                delegated_reached,
                model_check_reached,
            ),
            kept_later_refs: authoritative_room_reserve_strengthening_kept_later_refs(),
            guard_reason: if lane_reached {
                None
            } else {
                Some(authoritative_room_reserve_strengthening_guard_reason(
                    report.sample_id.as_str(),
                ))
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliAuthoritativeRoomFirstScenarioActualAdoptionSummary {
    status: &'static str,
    adoption_kind: &'static str,
    profile_axis_refs: Vec<String>,
    relation_refs: Vec<String>,
    authority_handoff_refs: Vec<String>,
    runtime_evidence_refs: Vec<String>,
    repo_local_emitted_artifact_refs: Vec<String>,
    reserve_route_refs: Vec<String>,
    negative_static_stop_refs: Vec<String>,
    contrast_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliAuthoritativeRoomFirstScenarioActualAdoptionSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
        artifact_preview: &CurrentL2OperationalCliArtifactPreviewSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        if authoritative_room_default_sample_reached(report, verification_preview) {
            return Self {
                status: "reached",
                adoption_kind: "helper_local_authoritative_room_first_scenario_manifest",
                profile_axis_refs: authoritative_room_profile_axis_refs(sample_id),
                relation_refs: authoritative_room_first_scenario_relation_refs(sample_id),
                authority_handoff_refs: authoritative_room_first_scenario_handoff_refs(sample_id),
                runtime_evidence_refs: authoritative_room_first_scenario_runtime_evidence_refs(
                    report,
                ),
                repo_local_emitted_artifact_refs: order_handoff_repo_local_emitted_artifact_refs(
                    sample_id,
                    artifact_preview,
                ),
                reserve_route_refs: Vec::new(),
                negative_static_stop_refs: Vec::new(),
                contrast_refs: authoritative_room_first_scenario_contrast_refs(),
                evidence_refs: authoritative_room_first_scenario_evidence_refs(sample_id),
                compare_floor_refs: authoritative_room_first_scenario_compare_floor_refs(
                    sample_id, true,
                ),
                guard_refs: authoritative_room_first_scenario_guard_refs(sample_id, true),
                kept_later_refs: authoritative_room_first_scenario_kept_later_refs(),
                guard_reason: None,
            };
        }

        let negative_static_stop_refs =
            order_handoff_source_surface_artifact_negative_static_stop_refs(sample_id);
        let reserve_route_refs = authoritative_room_first_scenario_reserve_route_refs(sample_id);
        let repo_local_emitted_artifact_refs = if verification_preview.formal_hook_status
            == "reached"
            || !negative_static_stop_refs.is_empty()
            || !reserve_route_refs.is_empty()
        {
            order_handoff_repo_local_emitted_artifact_refs(sample_id, artifact_preview)
        } else {
            Vec::new()
        };

        Self {
            status: "guarded_not_reached",
            adoption_kind: "helper_local_authoritative_room_first_scenario_manifest",
            profile_axis_refs: Vec::new(),
            relation_refs: Vec::new(),
            authority_handoff_refs: Vec::new(),
            runtime_evidence_refs: Vec::new(),
            repo_local_emitted_artifact_refs,
            reserve_route_refs,
            negative_static_stop_refs,
            contrast_refs: authoritative_room_first_scenario_contrast_refs(),
            evidence_refs: authoritative_room_first_scenario_evidence_refs(sample_id),
            compare_floor_refs: authoritative_room_first_scenario_compare_floor_refs(
                sample_id, false,
            ),
            guard_refs: authoritative_room_first_scenario_guard_refs(sample_id, false),
            kept_later_refs: authoritative_room_first_scenario_kept_later_refs(),
            guard_reason: Some(authoritative_room_first_scenario_guard_reason(
                report,
                verification_preview,
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliOrderHandoffSourceSurfaceArtifactActualAdoptionSummary {
    status: &'static str,
    adoption_kind: &'static str,
    profile_axis_refs: Vec<String>,
    principal_surface_lines: Vec<String>,
    secondary_surface_lines: Vec<String>,
    repo_local_emitted_artifact_refs: Vec<String>,
    source_wording_route_refs: Vec<String>,
    emitted_artifact_candidate_keep_refs: Vec<String>,
    negative_static_stop_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliOrderHandoffSourceSurfaceArtifactActualAdoptionSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
        artifact_preview: &CurrentL2OperationalCliArtifactPreviewSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh"
        ) && verification_preview.formal_hook_status == "reached";

        if reached {
            return Self {
                status: "reached",
                adoption_kind: "helper_local_source_surface_artifact_route_manifest",
                profile_axis_refs: authoritative_room_profile_axis_refs(sample_id),
                principal_surface_lines: order_handoff_edge_row_principal_surface_lines(sample_id),
                secondary_surface_lines: order_handoff_stage_block_secondary_lines(sample_id),
                repo_local_emitted_artifact_refs: order_handoff_repo_local_emitted_artifact_refs(
                    sample_id,
                    artifact_preview,
                ),
                source_wording_route_refs: order_handoff_source_wording_actual_route_refs(
                    sample_id,
                ),
                emitted_artifact_candidate_keep_refs: order_handoff_emitted_artifact_keep_refs(
                    sample_id,
                ),
                negative_static_stop_refs: Vec::new(),
                evidence_refs: order_handoff_source_surface_artifact_actual_adoption_evidence_refs(
                    sample_id,
                ),
                compare_floor_refs:
                    order_handoff_source_surface_artifact_actual_adoption_compare_floor_refs(
                        sample_id, true,
                    ),
                guard_refs: order_handoff_source_surface_artifact_actual_adoption_guard_refs(true),
                kept_later_refs:
                    order_handoff_source_surface_artifact_actual_adoption_kept_later_refs(),
                guard_reason: None,
            };
        }

        let negative_static_stop_refs =
            order_handoff_source_surface_artifact_negative_static_stop_refs(sample_id);
        let repo_local_emitted_artifact_refs = if negative_static_stop_refs.is_empty() {
            Vec::new()
        } else {
            order_handoff_repo_local_emitted_artifact_refs(sample_id, artifact_preview)
        };
        let guard_reason =
            order_handoff_source_surface_artifact_guard_reason(report, verification_preview);

        Self {
            status: "guarded_not_reached",
            adoption_kind: "helper_local_source_surface_artifact_route_manifest",
            profile_axis_refs: Vec::new(),
            principal_surface_lines: Vec::new(),
            secondary_surface_lines: Vec::new(),
            repo_local_emitted_artifact_refs,
            source_wording_route_refs: Vec::new(),
            emitted_artifact_candidate_keep_refs: Vec::new(),
            negative_static_stop_refs,
            evidence_refs: order_handoff_source_surface_artifact_actual_adoption_evidence_refs(
                sample_id,
            ),
            compare_floor_refs:
                order_handoff_source_surface_artifact_actual_adoption_compare_floor_refs(
                    sample_id, false,
                ),
            guard_refs: order_handoff_source_surface_artifact_actual_adoption_guard_refs(false),
            kept_later_refs: order_handoff_source_surface_artifact_actual_adoption_kept_later_refs(
            ),
            guard_reason: Some(guard_reason),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualCheckerPayloadFamilyThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    cluster_kind: Option<&'static str>,
    case_label: Option<&'static str>,
    family_refs: Vec<String>,
    coverage_state: Option<&'static str>,
    payload_family_kind: Option<&'static str>,
    source_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualCheckerPayloadFamilyThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
        typed_checker_hint_preview: &CurrentL2OperationalCliTypedCheckerHintPreviewSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && verification_preview.formal_hook_status == "reached"
            && typed_checker_hint_preview.status == "reached";

        if reached {
            let mut compare_floor_refs = typed_checker_hint_preview.compare_floor_refs.clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.actual_checker_payload_family".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_checker_payload_family_threshold"
                    .to_string(),
            );

            let mut evidence_refs = typed_checker_hint_evidence_refs(&report.sample_id);
            evidence_refs
                .push("helper_preview:actual_checker_payload_family_threshold".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_payload_threshold_manifest",
                cluster_kind: typed_checker_hint_preview.cluster_kind,
                case_label: typed_checker_hint_preview.case_label,
                family_refs: typed_checker_hint_preview
                    .typed_reason_family_hint
                    .as_ref()
                    .map(|hint| hint.family_refs.clone())
                    .unwrap_or_default(),
                coverage_state: typed_checker_hint_preview
                    .typed_reason_family_hint
                    .as_ref()
                    .map(|hint| hint.coverage_state),
                payload_family_kind: Some("static_reason_code_row_family"),
                source_refs: actual_checker_payload_family_threshold_source_refs(),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_checker_payload_family_threshold_guard_refs(true),
                kept_later_refs: actual_checker_payload_family_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_payload_threshold_manifest",
            cluster_kind: None,
            case_label: None,
            family_refs: Vec::new(),
            coverage_state: None,
            payload_family_kind: None,
            source_refs: Vec::new(),
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_checker_payload_family_threshold".to_string(),
                "compare_floor:current_l2.checker.actual_checker_payload_family".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.actual_checker_payload_family.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_checker_payload_family_threshold_guard_refs(false),
            kept_later_refs: actual_checker_payload_family_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual checker payload family threshold only actualizes the {} after typed checker-hint preview reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualCheckerPayloadRowFamilyThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    cluster_kind: Option<&'static str>,
    case_label: Option<&'static str>,
    family_refs: Vec<String>,
    coverage_state: Option<&'static str>,
    payload_family_ref: Option<&'static str>,
    row_family_kind: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualCheckerPayloadRowFamilyThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_checker_payload_family_threshold:
            &CurrentL2OperationalCliActualCheckerPayloadFamilyThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_checker_payload_family_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_checker_payload_family_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.checker_payload_row_family".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_checker_payload_row_family_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_checker_payload_family_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_checker_payload_row_family_threshold".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_row_family_threshold_manifest",
                cluster_kind: actual_checker_payload_family_threshold.cluster_kind,
                case_label: actual_checker_payload_family_threshold.case_label,
                family_refs: actual_checker_payload_family_threshold.family_refs.clone(),
                coverage_state: actual_checker_payload_family_threshold.coverage_state,
                payload_family_ref: Some("actual_checker_payload_family"),
                row_family_kind: Some("checked_reason_code_rows"),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_checker_payload_row_family_threshold_guard_refs(true),
                kept_later_refs: actual_checker_payload_row_family_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_row_family_threshold_manifest",
            cluster_kind: None,
            case_label: None,
            family_refs: Vec::new(),
            coverage_state: None,
            payload_family_ref: None,
            row_family_kind: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_checker_payload_row_family_threshold".to_string(),
                "compare_floor:current_l2.checker.checker_payload_row_family".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.checker_payload_row_family.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_checker_payload_row_family_threshold_guard_refs(false),
            kept_later_refs: actual_checker_payload_row_family_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual checker payload row-family threshold only actualizes the {} after actual checker payload family threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualCheckerPayloadRowDetailThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    cluster_kind: Option<&'static str>,
    case_label: Option<&'static str>,
    family_refs: Vec<String>,
    coverage_state: Option<&'static str>,
    payload_row_family_ref: Option<&'static str>,
    row_source_ref: Option<&'static str>,
    row_reason_kind: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualCheckerPayloadRowDetailThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_checker_payload_row_family_threshold:
            &CurrentL2OperationalCliActualCheckerPayloadRowFamilyThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_checker_payload_row_family_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_checker_payload_row_family_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.checker_payload_row_detail".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_checker_payload_row_detail_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_checker_payload_row_family_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_checker_payload_row_detail_threshold".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_row_detail_threshold_manifest",
                cluster_kind: actual_checker_payload_row_family_threshold.cluster_kind,
                case_label: actual_checker_payload_row_family_threshold.case_label,
                family_refs: actual_checker_payload_row_family_threshold
                    .family_refs
                    .clone(),
                coverage_state: actual_checker_payload_row_family_threshold.coverage_state,
                payload_row_family_ref: Some("actual_checker_payload_row_family"),
                row_source_ref: Some(actual_checker_payload_row_detail_source_ref(
                    &report.sample_id,
                )),
                row_reason_kind: actual_checker_payload_row_detail_reason_kind(&report.sample_id),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_checker_payload_row_detail_threshold_guard_refs(true),
                kept_later_refs: actual_checker_payload_row_detail_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_row_detail_threshold_manifest",
            cluster_kind: None,
            case_label: None,
            family_refs: Vec::new(),
            coverage_state: None,
            payload_row_family_ref: None,
            row_source_ref: None,
            row_reason_kind: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_checker_payload_row_detail_threshold".to_string(),
                "compare_floor:current_l2.checker.checker_payload_row_detail".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.checker_payload_row_detail.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_checker_payload_row_detail_threshold_guard_refs(false),
            kept_later_refs: actual_checker_payload_row_detail_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual checker payload row-detail threshold only actualizes the {} after actual checker payload row-family threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualCheckerPayloadRowBodyThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    cluster_kind: Option<&'static str>,
    case_label: Option<&'static str>,
    family_refs: Vec<String>,
    coverage_state: Option<&'static str>,
    payload_row_family_ref: Option<&'static str>,
    row_source_ref: Option<&'static str>,
    row_reason_kind: Option<&'static str>,
    row_body: Option<BTreeMap<String, String>>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualCheckerPayloadRowBodyThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_checker_payload_row_detail_threshold:
            &CurrentL2OperationalCliActualCheckerPayloadRowDetailThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_checker_payload_row_detail_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_checker_payload_row_detail_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.checker_payload_row_body".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_checker_payload_row_body_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_checker_payload_row_detail_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_checker_payload_row_body_threshold".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_row_body_threshold_manifest",
                cluster_kind: actual_checker_payload_row_detail_threshold.cluster_kind,
                case_label: actual_checker_payload_row_detail_threshold.case_label,
                family_refs: actual_checker_payload_row_detail_threshold
                    .family_refs
                    .clone(),
                coverage_state: actual_checker_payload_row_detail_threshold.coverage_state,
                payload_row_family_ref: Some("actual_checker_payload_row_family"),
                row_source_ref: Some(actual_checker_payload_row_detail_source_ref(
                    &report.sample_id,
                )),
                row_reason_kind: actual_checker_payload_row_detail_reason_kind(&report.sample_id),
                row_body: actual_checker_payload_row_body_bundle(&report.sample_id),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_checker_payload_row_body_threshold_guard_refs(true),
                kept_later_refs: actual_checker_payload_row_body_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_row_body_threshold_manifest",
            cluster_kind: None,
            case_label: None,
            family_refs: Vec::new(),
            coverage_state: None,
            payload_row_family_ref: None,
            row_source_ref: None,
            row_reason_kind: None,
            row_body: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_checker_payload_row_body_threshold".to_string(),
                "compare_floor:current_l2.checker.checker_payload_row_body".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.checker_payload_row_body.guard_only".to_string(),
            ],
            guard_refs: actual_checker_payload_row_body_threshold_guard_refs(false),
            kept_later_refs: actual_checker_payload_row_body_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual checker payload row-body threshold only actualizes the {} after actual checker payload row-detail threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualCheckerPayloadSupportedKindSummaryThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    payload_row_family_ref: Option<&'static str>,
    supported_kind_scope: Option<&'static str>,
    supported_kind_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualCheckerPayloadSupportedKindSummaryThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_checker_payload_row_body_threshold:
            &CurrentL2OperationalCliActualCheckerPayloadRowBodyThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_checker_payload_row_body_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_checker_payload_row_body_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.checker_payload_supported_kind_summary"
                    .to_string(),
            );
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_checker_payload_supported_kind_summary_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_checker_payload_row_body_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_checker_payload_supported_kind_summary_threshold"
                    .to_string(),
            );

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_supported_kind_summary_threshold_manifest",
                payload_row_family_ref: Some("actual_checker_payload_row_family"),
                supported_kind_scope: Some("stable_clusters_only"),
                supported_kind_refs: actual_checker_payload_supported_kind_refs(),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_checker_payload_supported_kind_summary_threshold_guard_refs(
                    true,
                ),
                kept_later_refs:
                    actual_checker_payload_supported_kind_summary_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_supported_kind_summary_threshold_manifest",
            payload_row_family_ref: None,
            supported_kind_scope: None,
            supported_kind_refs: Vec::new(),
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_checker_payload_supported_kind_summary_threshold"
                    .to_string(),
                "compare_floor:current_l2.checker.checker_payload_supported_kind_summary"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.checker_payload_supported_kind_summary.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_checker_payload_supported_kind_summary_threshold_guard_refs(false),
            kept_later_refs:
                actual_checker_payload_supported_kind_summary_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual checker payload supported-kind summary threshold only actualizes the {} after actual checker payload row-body threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualCheckerPayloadPublicSchemaSketchThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    actual_checker_payload_family_ref: Option<&'static str>,
    checker_payload_row_family_ref: Option<&'static str>,
    checker_payload_row_detail_ref: Option<&'static str>,
    checker_payload_row_body_ref: Option<&'static str>,
    checker_payload_supported_kind_summary_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualCheckerPayloadPublicSchemaSketchThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_checker_payload_supported_kind_summary_threshold:
            &CurrentL2OperationalCliActualCheckerPayloadSupportedKindSummaryThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_checker_payload_supported_kind_summary_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_checker_payload_supported_kind_summary_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.public_checker_payload_schema".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_public_checker_payload_schema_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_checker_payload_supported_kind_summary_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_checker_payload_public_schema_sketch_threshold".to_string(),
            );

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest",
                actual_checker_payload_family_ref: Some("actual_checker_payload_family"),
                checker_payload_row_family_ref: Some("actual_checker_payload_row_family"),
                checker_payload_row_detail_ref: Some("actual_checker_payload_row_detail"),
                checker_payload_row_body_ref: Some("actual_checker_payload_row_body"),
                checker_payload_supported_kind_summary_ref: Some(
                    "actual_checker_payload_supported_kind_summary",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_checker_payload_public_schema_sketch_threshold_guard_refs(true),
                kept_later_refs:
                    actual_checker_payload_public_schema_sketch_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest",
            actual_checker_payload_family_ref: None,
            checker_payload_row_family_ref: None,
            checker_payload_row_detail_ref: None,
            checker_payload_row_body_ref: None,
            checker_payload_supported_kind_summary_ref: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_checker_payload_public_schema_sketch_threshold".to_string(),
                "compare_floor:current_l2.checker.public_checker_payload_schema".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.public_checker_payload_schema.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_checker_payload_public_schema_sketch_threshold_guard_refs(false),
            kept_later_refs: actual_checker_payload_public_schema_sketch_threshold_kept_later_refs(
            ),
            guard_reason: Some(format!(
                "current actual checker payload public-schema sketch threshold only actualizes the {} after actual checker payload supported-kind summary threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPublicCheckerApiSketchThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    checker_subject: Option<&'static str>,
    public_checker_payload_schema_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPublicCheckerApiSketchThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
        actual_checker_payload_public_schema_sketch_threshold:
            &CurrentL2OperationalCliActualCheckerPayloadPublicSchemaSketchThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_checker_payload_public_schema_sketch_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_checker_payload_public_schema_sketch_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.public_checker_api".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_public_checker_api_threshold".to_string(),
            );

            let mut evidence_refs = actual_checker_payload_public_schema_sketch_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_public_checker_api_sketch_threshold".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_public_checker_api_sketch_threshold_manifest",
                checker_subject: verification_preview.subject_kind,
                public_checker_payload_schema_ref: Some(
                    "public_checker_payload_schema_ready_sketch",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_public_checker_api_sketch_threshold_guard_refs(true),
                kept_later_refs: actual_public_checker_api_sketch_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_public_checker_api_sketch_threshold_manifest",
            checker_subject: None,
            public_checker_payload_schema_ref: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_public_checker_api_sketch_threshold".to_string(),
                "compare_floor:current_l2.checker.public_checker_api".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.public_checker_api.guard_only".to_string(),
            ],
            guard_refs: actual_public_checker_api_sketch_threshold_guard_refs(false),
            kept_later_refs: actual_public_checker_api_sketch_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual public checker API sketch threshold only actualizes the {} after actual checker payload public-schema sketch threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPublicCheckerEntryCriteriaThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    public_checker_api_ref: Option<&'static str>,
    entry_criteria_refs: Vec<String>,
    family_facade_support_ref: Option<&'static str>,
    family_facade_script_refs: Vec<String>,
    smoke_command_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    deferred_boundary_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPublicCheckerEntryCriteriaThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_public_checker_api_sketch_threshold:
            &CurrentL2OperationalCliActualPublicCheckerApiSketchThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_public_checker_api_sketch_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_public_checker_api_sketch_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.public_checker_entry_criteria".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_public_checker_entry_criteria_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_public_checker_api_sketch_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_public_checker_entry_criteria_threshold".to_string());
            evidence_refs.push("source:scripts/current_l2_family_checker_support.py".to_string());
            evidence_refs.push("source:scripts/current_l2_same_lineage_checker.py".to_string());
            evidence_refs.push("source:scripts/current_l2_missing_option_checker.py".to_string());
            evidence_refs.push("source:scripts/current_l2_capability_checker.py".to_string());
            evidence_refs.push("source:detached_loop_smoke_checker_family".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_public_checker_entry_criteria_threshold_manifest",
                public_checker_api_ref: Some("public_checker_api_ready_sketch"),
                entry_criteria_refs: vec![
                    "public_checker_entry_criteria:minimal_api_fixed".to_string(),
                    "public_checker_entry_criteria:command_surface_pressure_present".to_string(),
                    "public_checker_entry_criteria:comparison_target_narrowed".to_string(),
                    "public_checker_entry_criteria:heavier_boundary_deferred".to_string(),
                ],
                family_facade_support_ref: Some("scripts/current_l2_family_checker_support.py"),
                family_facade_script_refs: vec![
                    "scripts/current_l2_same_lineage_checker.py".to_string(),
                    "scripts/current_l2_missing_option_checker.py".to_string(),
                    "scripts/current_l2_capability_checker.py".to_string(),
                ],
                smoke_command_refs: vec![
                    "smoke-same-lineage-checker".to_string(),
                    "smoke-missing-option-checker".to_string(),
                    "smoke-capability-checker".to_string(),
                ],
                next_comparison_target_ref: Some("public_checker_command_surface_comparison"),
                deferred_boundary_refs: vec![
                    "shared_output_contract".to_string(),
                    "parser_front_public_checker_boundary".to_string(),
                    "verifier_handoff_surface".to_string(),
                ],
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_public_checker_entry_criteria_threshold_guard_refs(true),
                kept_later_refs: actual_public_checker_entry_criteria_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_public_checker_entry_criteria_threshold_manifest",
            public_checker_api_ref: None,
            entry_criteria_refs: vec![],
            family_facade_support_ref: None,
            family_facade_script_refs: vec![],
            smoke_command_refs: vec![],
            next_comparison_target_ref: None,
            deferred_boundary_refs: vec![],
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_public_checker_entry_criteria_threshold".to_string(),
                "compare_floor:current_l2.checker.public_checker_entry_criteria".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.public_checker_entry_criteria.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_public_checker_entry_criteria_threshold_guard_refs(false),
            kept_later_refs: actual_public_checker_entry_criteria_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual public checker entry-criteria threshold only actualizes the {} after actual public checker API sketch threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPublicCheckerCommandSurfaceThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    command_surface_kind: Option<&'static str>,
    family_facade_command_refs: Vec<String>,
    public_checker_api_ref: Option<&'static str>,
    next_comparison_target_ref: Option<&'static str>,
    deferred_surface_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPublicCheckerCommandSurfaceThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_public_checker_entry_criteria_threshold:
            &CurrentL2OperationalCliActualPublicCheckerEntryCriteriaThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_public_checker_entry_criteria_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_public_checker_entry_criteria_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.public_checker_command_surface".to_string(),
            );
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_public_checker_command_surface_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_public_checker_entry_criteria_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_public_checker_command_surface_threshold".to_string());
            evidence_refs.push("source:family_facade_checker_commands".to_string());
            evidence_refs.push("source:detached_loop_smoke_checker_family".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_public_checker_command_surface_threshold_manifest",
                command_surface_kind: Some("family_facade_checker_commands"),
                family_facade_command_refs: vec![
                    "same_lineage_checker_command".to_string(),
                    "missing_option_checker_command".to_string(),
                    "capability_checker_command".to_string(),
                ],
                public_checker_api_ref: Some("public_checker_api_ready_sketch"),
                next_comparison_target_ref: Some("shared_output_contract_comparison"),
                deferred_surface_refs: vec![
                    "detached_loop_smoke_same_lineage_checker".to_string(),
                    "detached_loop_smoke_missing_option_checker".to_string(),
                    "detached_loop_smoke_capability_checker".to_string(),
                    "generic_shared_public_checker_entry".to_string(),
                    "shared_output_contract".to_string(),
                    "parser_front_public_checker_boundary".to_string(),
                    "verifier_handoff_surface".to_string(),
                ],
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_public_checker_command_surface_threshold_guard_refs(true),
                kept_later_refs: actual_public_checker_command_surface_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_public_checker_command_surface_threshold_manifest",
            command_surface_kind: None,
            family_facade_command_refs: vec![],
            public_checker_api_ref: None,
            next_comparison_target_ref: None,
            deferred_surface_refs: vec![],
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_public_checker_command_surface_threshold".to_string(),
                "compare_floor:current_l2.checker.public_checker_command_surface".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.public_checker_command_surface.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_public_checker_command_surface_threshold_guard_refs(false),
            kept_later_refs: actual_public_checker_command_surface_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual public checker command-surface threshold only actualizes the {} after actual public checker entry-criteria threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualSharedOutputContractThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    output_contract_kind: Option<&'static str>,
    checker_cluster_name: Option<&'static str>,
    checker_status: Option<&'static str>,
    public_checker_payload_schema_ref: Option<&'static str>,
    next_comparison_target_ref: Option<&'static str>,
    deferred_surface_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualSharedOutputContractThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_public_checker_command_surface_threshold:
            &CurrentL2OperationalCliActualPublicCheckerCommandSurfaceThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_public_checker_command_surface_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_public_checker_command_surface_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.shared_output_contract".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_shared_output_contract_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_public_checker_command_surface_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_shared_output_contract_threshold".to_string());
            evidence_refs.push("source:scripts/current_l2_family_checker_support.py".to_string());
            evidence_refs.push("source:family_checker_row_compare_summary".to_string());
            evidence_refs.push("source:scripts/current_l2_same_lineage_checker.py".to_string());
            evidence_refs.push("source:detached_loop_smoke_same_lineage_checker".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_shared_output_contract_threshold_manifest",
                output_contract_kind: Some("family_checker_row_compare_summary"),
                checker_cluster_name: Some("same_lineage_evidence_floor"),
                checker_status: Some("matched"),
                public_checker_payload_schema_ref: Some(
                    "public_checker_payload_schema_ready_sketch",
                ),
                next_comparison_target_ref: Some("public_checker_boundary_comparison"),
                deferred_surface_refs: vec![
                    "static_gate_artifact_path".to_string(),
                    "fixture_path".to_string(),
                    "artifact_path".to_string(),
                    "fixture_rows_textual_rendering".to_string(),
                    "actual_rows_textual_rendering".to_string(),
                    "generic_shared_public_checker_entry".to_string(),
                    "parser_front_public_checker_boundary".to_string(),
                    "verifier_handoff_surface".to_string(),
                ],
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_shared_output_contract_threshold_guard_refs(true),
                kept_later_refs: actual_shared_output_contract_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_shared_output_contract_threshold_manifest",
            output_contract_kind: None,
            checker_cluster_name: None,
            checker_status: None,
            public_checker_payload_schema_ref: None,
            next_comparison_target_ref: None,
            deferred_surface_refs: vec![],
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_shared_output_contract_threshold".to_string(),
                "compare_floor:current_l2.checker.shared_output_contract".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.shared_output_contract.guard_only".to_string(),
            ],
            guard_refs: actual_shared_output_contract_threshold_guard_refs(false),
            kept_later_refs: actual_shared_output_contract_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual shared output contract threshold only actualizes the {} after actual public checker command-surface threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPublicCheckerBoundaryThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    boundary_kind: Option<&'static str>,
    public_checker_command_surface_ref: Option<&'static str>,
    shared_output_contract_ref: Option<&'static str>,
    next_comparison_target_ref: Option<&'static str>,
    deferred_surface_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPublicCheckerBoundaryThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_shared_output_contract_threshold:
            &CurrentL2OperationalCliActualSharedOutputContractThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_shared_output_contract_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_shared_output_contract_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.public_checker_boundary".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_public_checker_boundary_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_shared_output_contract_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_public_checker_boundary_threshold".to_string());
            evidence_refs.push("source:docs_only_parser_front_checker_boundary".to_string());
            evidence_refs.push("source:public_checker_boundary_ready_sketch".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_public_checker_boundary_threshold_manifest",
                boundary_kind: Some("docs_only_parser_front_checker_boundary"),
                public_checker_command_surface_ref: Some(
                    "public_checker_command_surface_ready_sketch",
                ),
                shared_output_contract_ref: Some("shared_output_contract_ready_sketch"),
                next_comparison_target_ref: Some("verifier_handoff_surface_comparison"),
                deferred_surface_refs: vec![
                    "final_parser_grammar".to_string(),
                    "query_token_and_checker_subject_public_naming".to_string(),
                    "generic_shared_public_checker_entry".to_string(),
                    "detached_loop_wrapper_path_line".to_string(),
                    "verifier_handoff_surface".to_string(),
                ],
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_public_checker_boundary_threshold_guard_refs(true),
                kept_later_refs: actual_public_checker_boundary_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_public_checker_boundary_threshold_manifest",
            boundary_kind: None,
            public_checker_command_surface_ref: None,
            shared_output_contract_ref: None,
            next_comparison_target_ref: None,
            deferred_surface_refs: vec![],
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_public_checker_boundary_threshold".to_string(),
                "compare_floor:current_l2.checker.public_checker_boundary".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.public_checker_boundary.guard_only".to_string(),
            ],
            guard_refs: actual_public_checker_boundary_threshold_guard_refs(false),
            kept_later_refs: actual_public_checker_boundary_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual public checker boundary threshold only actualizes the {} after actual shared output contract threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualVerifierHandoffSurfaceThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    handoff_surface_kind: Option<&'static str>,
    public_checker_boundary_ref: Option<&'static str>,
    proof_obligation_matrix_ref: Option<&'static str>,
    handoff_artifact_mode: Option<&'static str>,
    next_comparison_target_ref: Option<&'static str>,
    deferred_surface_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualVerifierHandoffSurfaceThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_public_checker_boundary_threshold:
            &CurrentL2OperationalCliActualPublicCheckerBoundaryThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_public_checker_boundary_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_public_checker_boundary_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.checker.verifier_handoff_surface".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.checker.minimal_verifier_handoff_surface_threshold"
                    .to_string(),
            );

            let mut evidence_refs = actual_public_checker_boundary_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_verifier_handoff_surface_threshold".to_string());
            evidence_refs.push("source:current_l2_proof_obligation_matrix".to_string());
            evidence_refs.push("source:docs_only_mixed_row_bundle_verifier_surface".to_string());

            return Self {
                status: "reached",
                threshold_kind: "checker_adjacent_verifier_handoff_surface_threshold_manifest",
                handoff_surface_kind: Some("docs_only_mixed_row_bundle_verifier_surface"),
                public_checker_boundary_ref: Some("public_checker_boundary_ready_sketch"),
                proof_obligation_matrix_ref: Some("current_l2_proof_obligation_matrix"),
                handoff_artifact_mode: Some("docs_only_mixed_row_bundle"),
                next_comparison_target_ref: Some("minimal_parser_subset_freeze_comparison"),
                deferred_surface_refs: vec![
                    "subject_kind".to_string(),
                    "subject_ref".to_string(),
                    "proof_obligation_rows".to_string(),
                    "theorem_handoff_artifact_ref".to_string(),
                    "protocol_handoff_artifact_ref".to_string(),
                    "runtime_policy_handoff_artifact_ref".to_string(),
                    "actual_emitted_verifier_handoff_artifact".to_string(),
                    "tool_specific_schema_and_actual_emitter_policy".to_string(),
                    "final_parser_grammar".to_string(),
                    "query_token_and_shared_generic_entry".to_string(),
                ],
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_verifier_handoff_surface_threshold_guard_refs(true),
                kept_later_refs: actual_verifier_handoff_surface_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "checker_adjacent_verifier_handoff_surface_threshold_manifest",
            handoff_surface_kind: None,
            public_checker_boundary_ref: None,
            proof_obligation_matrix_ref: None,
            handoff_artifact_mode: None,
            next_comparison_target_ref: None,
            deferred_surface_refs: vec![],
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_verifier_handoff_surface_threshold".to_string(),
                "compare_floor:current_l2.checker.verifier_handoff_surface".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.checker.verifier_handoff_surface.guard_only".to_string(),
            ],
            guard_refs: actual_verifier_handoff_surface_threshold_guard_refs(false),
            kept_later_refs: actual_verifier_handoff_surface_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual verifier handoff surface threshold only actualizes the {} after actual public checker boundary threshold reaches the checker-adjacent helper floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualMinimalParserSubsetFreezeThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    freeze_kind: Option<&'static str>,
    accepted_cluster_refs: Vec<String>,
    reject_cluster_refs: Vec<String>,
    retention_floor_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualMinimalParserSubsetFreezeThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_verifier_handoff_surface_threshold:
            &CurrentL2OperationalCliActualVerifierHandoffSurfaceThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_verifier_handoff_surface_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_verifier_handoff_surface_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.parser.minimal_parser_subset_freeze".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze".to_string(),
            );

            let mut evidence_refs = actual_verifier_handoff_surface_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_minimal_parser_subset_freeze_threshold".to_string());
            evidence_refs.push("source:stage1_stage2_structural_parser_floor".to_string());
            evidence_refs.push("source:minimal_parser_subset_freeze_ready_sketch".to_string());

            return Self {
                status: "reached",
                threshold_kind: "parser_front_minimal_parser_subset_freeze_threshold_manifest",
                freeze_kind: Some("stage1_stage2_structural_parser_floor"),
                accepted_cluster_refs: vec![
                    "stage1_chain_declaration_structural_floor".to_string(),
                    "stage2_try_rollback_structural_floor".to_string(),
                ],
                reject_cluster_refs: vec![
                    "missing_edge_local_lineage_metadata".to_string(),
                    "missing_fallback_body".to_string(),
                    "atomic_cut_fallback_placement".to_string(),
                ],
                retention_floor_refs: vec![
                    "stage3_admit_slot_branch".to_string(),
                    "stage3_request_clause_branch".to_string(),
                    "stage3_predicate_fragment_branch".to_string(),
                ],
                next_comparison_target_ref: Some("parser_to_checker_reconnect_freeze_comparison"),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_minimal_parser_subset_freeze_threshold_guard_refs(true),
                kept_later_refs: actual_minimal_parser_subset_freeze_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "parser_front_minimal_parser_subset_freeze_threshold_manifest",
            freeze_kind: None,
            accepted_cluster_refs: vec![],
            reject_cluster_refs: vec![],
            retention_floor_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_minimal_parser_subset_freeze_threshold".to_string(),
                "compare_floor:current_l2.parser.minimal_parser_subset_freeze".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.parser.minimal_parser_subset_freeze.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_minimal_parser_subset_freeze_threshold_guard_refs(false),
            kept_later_refs: actual_minimal_parser_subset_freeze_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual minimal parser subset freeze threshold only actualizes the {} after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualParserToCheckerReconnectFreezeThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    reconnect_kind: Option<&'static str>,
    parser_subset_freeze_ref: Option<&'static str>,
    checker_floor_refs: Vec<String>,
    retained_helper_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualParserToCheckerReconnectFreezeThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_minimal_parser_subset_freeze_threshold:
            &CurrentL2OperationalCliActualMinimalParserSubsetFreezeThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_minimal_parser_subset_freeze_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_minimal_parser_subset_freeze_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze".to_string(),
            );
            compare_floor_refs
                .push("compare_floor:current_l2.closeout.phase1_semantics_closeout".to_string());

            let mut evidence_refs = actual_minimal_parser_subset_freeze_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_parser_to_checker_reconnect_freeze_threshold".to_string(),
            );
            evidence_refs.push("source:stage1_stage2_checker_floor_bridge".to_string());
            evidence_refs
                .push("source:parser_to_checker_reconnect_freeze_ready_sketch".to_string());

            return Self {
                status: "reached",
                threshold_kind: "parser_checker_bridge_reconnect_freeze_threshold_manifest",
                reconnect_kind: Some("stage1_stage2_checker_floor_bridge"),
                parser_subset_freeze_ref: Some("minimal_parser_subset_freeze_ready_sketch"),
                checker_floor_refs: vec![
                    "stage1_reconnect_summary_floor".to_string(),
                    "stage2_try_rollback_structural_floor".to_string(),
                ],
                retained_helper_refs: vec![
                    "stage3_request_predicate_reconnect_line".to_string(),
                    "stage1_direct_target_mismatch_redesign_line".to_string(),
                    "runtime_contrast_e21_e22_line".to_string(),
                ],
                next_comparison_target_ref: Some("phase1_semantics_closeout_comparison"),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_parser_to_checker_reconnect_freeze_threshold_guard_refs(true),
                kept_later_refs:
                    actual_parser_to_checker_reconnect_freeze_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "parser_checker_bridge_reconnect_freeze_threshold_manifest",
            reconnect_kind: None,
            parser_subset_freeze_ref: None,
            checker_floor_refs: vec![],
            retained_helper_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_parser_to_checker_reconnect_freeze_threshold".to_string(),
                "compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_parser_to_checker_reconnect_freeze_threshold_guard_refs(false),
            kept_later_refs: actual_parser_to_checker_reconnect_freeze_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual parser-to-checker reconnect freeze threshold only actualizes the {} after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase1SemanticsCloseoutThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    closeout_kind: Option<&'static str>,
    core_semantics_refs: Vec<String>,
    invariant_bridge_refs: Vec<String>,
    notation_status_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase1SemanticsCloseoutThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_parser_to_checker_reconnect_freeze_threshold:
            &CurrentL2OperationalCliActualParserToCheckerReconnectFreezeThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_parser_to_checker_reconnect_freeze_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_parser_to_checker_reconnect_freeze_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs
                .push("compare_floor:current_l2.closeout.phase1_semantics_closeout".to_string());
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout".to_string(),
            );

            let mut evidence_refs = actual_parser_to_checker_reconnect_freeze_threshold
                .evidence_refs
                .clone();
            evidence_refs
                .push("helper_preview:actual_phase1_semantics_closeout_threshold".to_string());
            evidence_refs.push("source:phase1_semantics_closeout_ready_sketch".to_string());
            evidence_refs.push("source:explicit_edge_row_family".to_string());

            return Self {
                status: "reached",
                threshold_kind: "phase1_semantics_closeout_threshold_manifest",
                closeout_kind: Some("current_l2_semantics_closeout"),
                core_semantics_refs: vec![
                    "fallback_lease_chain_semantics".to_string(),
                    "try_atomic_cut_semantics".to_string(),
                ],
                invariant_bridge_refs: vec![
                    "invariants_8_9_to_canonical_normalization_law".to_string(),
                    "invariant_11_to_rollback_cut_non_interference".to_string(),
                ],
                notation_status_refs: vec![
                    "explicit_edge_row_family".to_string(),
                    "a2_polished_first_choice".to_string(),
                    "a1_companion_shorthand".to_string(),
                ],
                next_comparison_target_ref: Some("phase2_parser_free_poc_closeout_comparison"),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_phase1_semantics_closeout_threshold_guard_refs(true),
                kept_later_refs: actual_phase1_semantics_closeout_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase1_semantics_closeout_threshold_manifest",
            closeout_kind: None,
            core_semantics_refs: vec![],
            invariant_bridge_refs: vec![],
            notation_status_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_phase1_semantics_closeout_threshold".to_string(),
                "compare_floor:current_l2.closeout.phase1_semantics_closeout".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase1_semantics_closeout.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_phase1_semantics_closeout_threshold_guard_refs(false),
            kept_later_refs: actual_phase1_semantics_closeout_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase1 semantics closeout threshold only actualizes the {} after actual parser-to-checker reconnect freeze threshold reaches the checker-floor bridge for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase2ParserFreePocCloseoutThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    closeout_kind: Option<&'static str>,
    compile_gate_refs: Vec<String>,
    helper_boundary_refs: Vec<String>,
    detached_loop_policy_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase2ParserFreePocCloseoutThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase1_semantics_closeout_threshold:
            &CurrentL2OperationalCliActualPhase1SemanticsCloseoutThresholdSummary,
    ) -> Self {
        let reached = current_l2_first_strong_typing_sample_reached(report.sample_id.as_str())
            && actual_phase1_semantics_closeout_threshold.status == "reached";

        if reached {
            let mut compare_floor_refs = actual_phase1_semantics_closeout_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout".to_string(),
            );
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout"
                    .to_string(),
            );

            let mut evidence_refs = actual_phase1_semantics_closeout_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_phase2_parser_free_poc_closeout_threshold".to_string(),
            );
            evidence_refs.push("source:phase2_parser_free_closeout_ready_sketch".to_string());
            evidence_refs.push("source:compare_only_non_production_helper".to_string());

            return Self {
                status: "reached",
                threshold_kind: "phase2_parser_free_poc_closeout_threshold_manifest",
                closeout_kind: Some("parser_free_companion_baseline"),
                compile_gate_refs: vec![
                    "interpreter_regression_suite".to_string(),
                    "detached_loop_unit_suite".to_string(),
                    "detached_example_compile_gate".to_string(),
                    "runtime_smoke_fixture_gate".to_string(),
                    "single_fixture_aggregate_compare_gate".to_string(),
                    "static_gate_checker_smoke_gate".to_string(),
                ],
                helper_boundary_refs: vec![
                    "bundle_runtime_path".to_string(),
                    "aggregate_compare_convenience".to_string(),
                    "static_gate_checker_smoke_family".to_string(),
                    "display_only_authoring_assists".to_string(),
                ],
                detached_loop_policy_refs: vec![
                    "compare_only_non_production_helper".to_string(),
                    "target_current_l2_detached_default_candidate".to_string(),
                    "diff_exit_code_one_is_informational".to_string(),
                ],
                next_comparison_target_ref: Some(
                    "phase4_shared_space_self_driven_closeout_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_phase2_parser_free_poc_closeout_threshold_guard_refs(true),
                kept_later_refs: actual_phase2_parser_free_poc_closeout_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase2_parser_free_poc_closeout_threshold_manifest",
            closeout_kind: None,
            compile_gate_refs: vec![],
            helper_boundary_refs: vec![],
            detached_loop_policy_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{}", report.sample_id),
                "helper_preview:actual_phase2_parser_free_poc_closeout_threshold".to_string(),
                "compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_phase2_parser_free_poc_closeout_threshold_guard_refs(false),
            kept_later_refs: actual_phase2_parser_free_poc_closeout_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase2 parser-free PoC closeout threshold only actualizes the {} after actual phase1 semantics closeout threshold reaches the semantics closeout floor for `{}`",
                current_l2_first_strong_typing_sample_guard_label(),
                report.sample_id
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase4SharedSpaceSelfDrivenCloseoutThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    closeout_kind: Option<&'static str>,
    current_package_refs: Vec<String>,
    user_spec_required_catalog_refs: Vec<String>,
    retained_later_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase4SharedSpaceSelfDrivenCloseoutThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        surface_preview: &CurrentL2OperationalCliSurfacePreviewSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && surface_preview.serial_scope_reserve.status == "reached";

        if reached {
            return Self {
                status: "reached",
                threshold_kind: "phase4_shared_space_self_driven_closeout_threshold_manifest",
                closeout_kind: Some("shared_space_practical_boundary_checkpoint"),
                current_package_refs:
                    actual_phase4_shared_space_self_driven_closeout_current_package_refs(),
                user_spec_required_catalog_refs:
                    actual_phase4_shared_space_self_driven_closeout_user_spec_required_catalog_refs(),
                retained_later_refs:
                    actual_phase4_shared_space_self_driven_closeout_retained_later_refs(),
                next_comparison_target_ref: Some(
                    "phase5_proof_protocol_runtime_policy_handoff_closeout_comparison",
                ),
                evidence_refs: vec![
                    format!("sample:{sample_id}"),
                    "helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold"
                        .to_string(),
                    "source:phase4_shared_space_closeout_ready_sketch".to_string(),
                    "source:authoritative_room_baseline_ref".to_string(),
                    "source:control_plane_threshold_ref".to_string(),
                ],
                compare_floor_refs: vec![
                    "compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout"
                        .to_string(),
                    "compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout"
                        .to_string(),
                ],
                guard_refs:
                    actual_phase4_shared_space_self_driven_closeout_threshold_guard_refs(true),
                kept_later_refs:
                    actual_phase4_shared_space_self_driven_closeout_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase4_shared_space_self_driven_closeout_threshold_manifest",
            closeout_kind: None,
            current_package_refs: vec![],
            user_spec_required_catalog_refs: vec![],
            retained_later_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_phase4_shared_space_self_driven_closeout_threshold_guard_refs(
                false,
            ),
            kept_later_refs:
                actual_phase4_shared_space_self_driven_closeout_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase4 shared-space self-driven closeout threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after the helper-local serial-scope reserve surface reaches the authoritative-room/delegated-provider floor for `{sample_id}`"
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase5ProofProtocolRuntimePolicyHandoffCloseoutThresholdSummary
{
    status: &'static str,
    threshold_kind: &'static str,
    closeout_kind: Option<&'static str>,
    verifier_handoff_surface_ref: Option<&'static str>,
    theorem_retained_bridge_stop_ref: Option<&'static str>,
    boundary_inventory_ref: Option<&'static str>,
    retained_later_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase5ProofProtocolRuntimePolicyHandoffCloseoutThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase4_shared_space_self_driven_closeout_threshold:
            &CurrentL2OperationalCliActualPhase4SharedSpaceSelfDrivenCloseoutThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase4_shared_space_self_driven_closeout_threshold.status
            == "reached";

        if reached {
            let mut compare_floor_refs = actual_phase4_shared_space_self_driven_closeout_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout"
                    .to_string(),
            );
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche"
                    .to_string(),
            );

            let mut evidence_refs = actual_phase4_shared_space_self_driven_closeout_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold"
                    .to_string(),
            );
            evidence_refs.push("source:phase5_handoff_closeout_ready_sketch".to_string());
            evidence_refs.push("source:minimal_verifier_handoff_surface".to_string());
            evidence_refs.push(
                "source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body"
                    .to_string(),
            );
            evidence_refs.push("source:small_decidable_core_boundary_inventory".to_string());

            return Self {
                status: "reached",
                threshold_kind:
                    "phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest",
                closeout_kind: Some("proof_protocol_runtime_policy_handoff_stop_line"),
                verifier_handoff_surface_ref: Some("minimal_verifier_handoff_surface"),
                theorem_retained_bridge_stop_ref: Some(
                    "retained_payload_body_materialization_theorem_export_handoff_transport_channel_body",
                ),
                boundary_inventory_ref: Some("small_decidable_core_boundary_inventory"),
                retained_later_refs:
                    actual_phase5_proof_protocol_runtime_policy_handoff_closeout_retained_later_refs(),
                next_comparison_target_ref: Some(
                    "phase6_actual_parser_ast_carrier_first_tranche_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs:
                    actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_guard_refs(
                        true,
                    ),
                kept_later_refs:
                    actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind:
                "phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest",
            closeout_kind: None,
            verifier_handoff_surface_ref: None,
            theorem_retained_bridge_stop_ref: None,
            boundary_inventory_ref: None,
            retained_later_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout.guard_only"
                    .to_string(),
            ],
            guard_refs:
                actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_guard_refs(
                    false,
                ),
            kept_later_refs:
                actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase5 proof/protocol/runtime-policy handoff closeout threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase4 shared-space self-driven closeout threshold reaches the shared-space practical boundary checkpoint for `{sample_id}`"
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase6ActualParserAstCarrierFirstTrancheThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    carrier_kind: Option<&'static str>,
    accepted_surface_refs: Vec<String>,
    code_anchor_refs: Vec<String>,
    retained_later_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase6ActualParserAstCarrierFirstTrancheThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
            &CurrentL2OperationalCliActualPhase5ProofProtocolRuntimePolicyHandoffCloseoutThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
            .status
            == "reached";
        let manifest = current_l2_first_tranche_manifest();

        if reached {
            let mut compare_floor_refs =
                actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
                    .compare_floor_refs
                    .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche"
                    .to_string(),
            );
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche"
                    .to_string(),
            );

            let mut evidence_refs =
                actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
                    .evidence_refs
                    .clone();
            evidence_refs.push(
                "helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold"
                    .to_string(),
            );
            evidence_refs.push(
                "source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch".to_string(),
            );
            evidence_refs.push("source:mir_ast_current_l2_first_tranche".to_string());
            evidence_refs.push("code_anchor:mir_ast_current_l2_module".to_string());
            evidence_refs.push("code_anchor:stage1_stage2_parser_spike_tests".to_string());

            return Self {
                status: "reached",
                threshold_kind: "phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest",
                carrier_kind: Some(manifest.carrier_kind),
                accepted_surface_refs: manifest
                    .accepted_surface_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                code_anchor_refs: manifest
                    .code_anchor_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                retained_later_refs: manifest
                    .retained_later_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                next_comparison_target_ref: Some(
                    "phase6_actual_checker_runtime_skeleton_first_tranche_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs:
                    actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_guard_refs(true),
                kept_later_refs:
                    actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_kept_later_refs(
                    ),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest",
            carrier_kind: None,
            accepted_surface_refs: vec![],
            code_anchor_refs: vec![],
            retained_later_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche.guard_only"
                    .to_string(),
            ],
            guard_refs:
                actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_guard_refs(false),
            kept_later_refs:
                actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase6 parser / AST carrier first tranche threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase5 proof/protocol/runtime-policy handoff closeout threshold reaches the handoff stop line for `{sample_id}`"
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase6ActualCheckerRuntimeSkeletonFirstTrancheThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    skeleton_kind: Option<&'static str>,
    semantic_entry_refs: Vec<String>,
    runtime_bridge_refs: Vec<String>,
    parser_bridge_contract_refs: Vec<String>,
    retained_later_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase6ActualCheckerRuntimeSkeletonFirstTrancheThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
            &CurrentL2OperationalCliActualPhase6ActualParserAstCarrierFirstTrancheThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase6_actual_parser_ast_carrier_first_tranche_threshold.status
            == "reached";
        let manifest = current_l2_checker_runtime_first_tranche_manifest();

        if reached {
            let mut compare_floor_refs =
                actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
                    .compare_floor_refs
                    .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche"
                    .to_string(),
            );
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook"
                    .to_string(),
            );

            let mut evidence_refs = actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold"
                    .to_string(),
            );
            evidence_refs.push(
                "source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch"
                    .to_string(),
            );
            evidence_refs
                .push("source:phase6_current_l2_checker_runtime_first_tranche".to_string());
            evidence_refs.push("code_anchor:mir_runtime_current_l2_module".to_string());
            evidence_refs.push("code_anchor:current_l2_runtime_skeleton_tests".to_string());

            return Self {
                status: "reached",
                threshold_kind:
                    "phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest",
                skeleton_kind: Some(manifest.skeleton_kind),
                semantic_entry_refs: manifest
                    .semantic_entry_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                runtime_bridge_refs: manifest
                    .runtime_bridge_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                parser_bridge_contract_refs: manifest
                    .parser_bridge_contract_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                retained_later_refs: manifest
                    .retained_later_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                next_comparison_target_ref: Some(
                    "phase6_compile_ready_verification_and_formal_hook_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs:
                    actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_guard_refs(
                        true,
                    ),
                kept_later_refs:
                    actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest",
            skeleton_kind: None,
            semantic_entry_refs: vec![],
            runtime_bridge_refs: vec![],
            parser_bridge_contract_refs: vec![],
            retained_later_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche.guard_only"
                    .to_string(),
            ],
            guard_refs:
                actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_guard_refs(
                    false,
                ),
            kept_later_refs:
                actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase6 checker/runtime skeleton first tranche threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 parser / AST carrier first tranche threshold reaches the parser first-tranche minimum for `{sample_id}`"
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase6CompileReadyVerificationAndFormalHookThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    verification_gate_refs: Vec<String>,
    smoke_gate_refs: Vec<String>,
    formal_hook_artifact_kind_ref: Option<&'static str>,
    formal_hook_subject_kind_refs: Vec<String>,
    formal_hook_contract_row_core_refs: Vec<String>,
    formal_hook_evidence_ref_family_refs: Vec<String>,
    formal_hook_obligation_kind_refs: Vec<String>,
    source_artifact_refs: Vec<String>,
    validation_refs: Vec<String>,
    retained_later_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase6NextReopenSequencingThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    sequencing_kind_ref: Option<&'static str>,
    fixed_entry_criteria_refs: Vec<String>,
    selected_first_reopen_ref: Option<&'static str>,
    deferred_reopen_refs: Vec<String>,
    minimum_guard_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase6ReserveFormalToolBindingInventoryThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    inventory_kind: Option<&'static str>,
    fixed_entry_criteria_refs: Vec<String>,
    first_reserve_ref: Option<&'static str>,
    second_reserve_ref: Option<&'static str>,
    minimum_guard_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliActualPhase6ParserSideFollowupPackageSequencingThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    sequencing_kind: Option<&'static str>,
    fixed_entry_criteria_refs: Vec<String>,
    selected_next_package_ref: Option<&'static str>,
    deferred_reopen_refs: Vec<String>,
    minimum_guard_refs: Vec<String>,
    next_comparison_target_ref: Option<&'static str>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliActualPhase6CompileReadyVerificationAndFormalHookThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
            &CurrentL2OperationalCliActualPhase6ActualCheckerRuntimeSkeletonFirstTrancheThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
            .status
            == "reached";
        let manifest = current_l2_compile_ready_verification_and_formal_hook_manifest();

        if reached {
            let mut compare_floor_refs =
                actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
                    .compare_floor_refs
                    .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_next_reopen_sequencing".to_string(),
            );

            let mut evidence_refs =
                actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
                    .evidence_refs
                    .clone();
            evidence_refs.push(
                "helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold"
                    .to_string(),
            );
            evidence_refs.push(
                "source:phase6_compile_ready_verification_and_formal_hook_ready_sketch".to_string(),
            );
            evidence_refs.push(
                "source:phase6_compile_ready_verification_and_formal_hook_minimum".to_string(),
            );
            evidence_refs.push("code_anchor:current_l2_emit_formal_hook_example".to_string());
            evidence_refs.push("code_anchor:current_l2_formal_hook_support_tests".to_string());
            evidence_refs.push("code_anchor:current_l2_detached_loop_smoke_family".to_string());

            return Self {
                status: "reached",
                threshold_kind:
                    "phase6_compile_ready_verification_and_formal_hook_threshold_manifest",
                verification_gate_refs: manifest
                    .verification_gate_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                smoke_gate_refs: manifest
                    .smoke_gate_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                formal_hook_artifact_kind_ref: Some(manifest.formal_hook_artifact_kind_ref),
                formal_hook_subject_kind_refs: manifest
                    .formal_hook_subject_kind_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                formal_hook_contract_row_core_refs: manifest
                    .formal_hook_contract_row_core_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                formal_hook_evidence_ref_family_refs: manifest
                    .formal_hook_evidence_ref_family_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                formal_hook_obligation_kind_refs: manifest
                    .formal_hook_obligation_kind_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                source_artifact_refs: manifest
                    .source_artifact_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                validation_refs: manifest
                    .validation_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                retained_later_refs: manifest
                    .retained_later_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                next_comparison_target_ref: Some("phase6_next_reopen_sequencing_comparison"),
                evidence_refs,
                compare_floor_refs,
                guard_refs:
                    actual_phase6_compile_ready_verification_and_formal_hook_threshold_guard_refs(
                        true,
                    ),
                kept_later_refs:
                    actual_phase6_compile_ready_verification_and_formal_hook_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase6_compile_ready_verification_and_formal_hook_threshold_manifest",
            verification_gate_refs: vec![],
            smoke_gate_refs: vec![],
            formal_hook_artifact_kind_ref: None,
            formal_hook_subject_kind_refs: vec![],
            formal_hook_contract_row_core_refs: vec![],
            formal_hook_evidence_ref_family_refs: vec![],
            formal_hook_obligation_kind_refs: vec![],
            source_artifact_refs: vec![],
            validation_refs: vec![],
            retained_later_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook.guard_only"
                    .to_string(),
            ],
            guard_refs:
                actual_phase6_compile_ready_verification_and_formal_hook_threshold_guard_refs(
                    false,
                ),
            kept_later_refs:
                actual_phase6_compile_ready_verification_and_formal_hook_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase6 compile-ready verification / formal hook threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 checker/runtime skeleton first tranche threshold reaches the checker/runtime first-tranche minimum for `{sample_id}`"
            )),
        }
    }
}

impl CurrentL2OperationalCliActualPhase6NextReopenSequencingThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase6_compile_ready_verification_and_formal_hook_threshold:
            &CurrentL2OperationalCliActualPhase6CompileReadyVerificationAndFormalHookThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase6_compile_ready_verification_and_formal_hook_threshold
            .status
            == "reached";
        let manifest = current_l2_phase6_next_reopen_sequencing_manifest();

        if reached {
            let mut compare_floor_refs =
                actual_phase6_compile_ready_verification_and_formal_hook_threshold
                    .compare_floor_refs
                    .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package"
                    .to_string(),
            );

            let mut evidence_refs =
                actual_phase6_compile_ready_verification_and_formal_hook_threshold
                    .evidence_refs
                    .clone();
            evidence_refs
                .push("helper_preview:actual_phase6_next_reopen_sequencing_threshold".to_string());
            evidence_refs
                .push("source:phase6_next_reopen_sequencing_current_first_choice".to_string());
            evidence_refs.push("source:phase6_next_reopen_sequencing_minimum".to_string());
            evidence_refs.push(
                "source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison"
                    .to_string(),
            );

            return Self {
                status: "reached",
                threshold_kind: "phase6_next_reopen_sequencing_threshold_manifest",
                sequencing_kind_ref: Some(manifest.sequencing_kind_ref),
                fixed_entry_criteria_refs: manifest
                    .fixed_entry_criteria_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                selected_first_reopen_ref: Some(manifest.selected_first_reopen_ref),
                deferred_reopen_refs: manifest
                    .deferred_reopen_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                minimum_guard_refs: manifest
                    .guard_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                next_comparison_target_ref: Some(
                    "phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs: actual_phase6_next_reopen_sequencing_threshold_guard_refs(true),
                kept_later_refs: actual_phase6_next_reopen_sequencing_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase6_next_reopen_sequencing_threshold_manifest",
            sequencing_kind_ref: None,
            fixed_entry_criteria_refs: vec![],
            selected_first_reopen_ref: None,
            deferred_reopen_refs: vec![],
            minimum_guard_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase6_next_reopen_sequencing_threshold".to_string(),
                "compare_floor:current_l2.closeout.phase6_next_reopen_sequencing".to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase6_next_reopen_sequencing.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_phase6_next_reopen_sequencing_threshold_guard_refs(false),
            kept_later_refs: actual_phase6_next_reopen_sequencing_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase6 next-reopen sequencing threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 compile-ready verification / formal hook threshold reaches the compile-ready minimum for `{sample_id}`"
            )),
        }
    }
}

impl CurrentL2OperationalCliActualPhase6ReserveFormalToolBindingInventoryThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase6_next_reopen_sequencing_threshold:
            &CurrentL2OperationalCliActualPhase6NextReopenSequencingThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase6_next_reopen_sequencing_threshold.status == "reached";
        let manifest = current_l2_phase6_reserve_formal_tool_binding_inventory_manifest();

        if reached {
            let mut compare_floor_refs = actual_phase6_next_reopen_sequencing_threshold
                .compare_floor_refs
                .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory"
                    .to_string(),
            );

            let mut evidence_refs = actual_phase6_next_reopen_sequencing_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold"
                    .to_string(),
            );
            evidence_refs.push(
                "source:phase6_reserve_formal_tool_binding_inventory_current_first_choice"
                    .to_string(),
            );
            evidence_refs
                .push("source:phase6_reserve_formal_tool_binding_inventory_minimum".to_string());

            return Self {
                status: "reached",
                threshold_kind: "phase6_reserve_formal_tool_binding_inventory_threshold_manifest",
                inventory_kind: Some(manifest.inventory_kind),
                fixed_entry_criteria_refs: manifest
                    .fixed_entry_criteria_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                first_reserve_ref: Some(manifest.first_reserve_ref),
                second_reserve_ref: Some(manifest.second_reserve_ref),
                minimum_guard_refs: manifest
                    .guard_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                next_comparison_target_ref: Some(
                    "phase6_parser_side_follow_up_package_sequencing_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs:
                    actual_phase6_reserve_formal_tool_binding_inventory_threshold_guard_refs(true),
                kept_later_refs:
                    actual_phase6_reserve_formal_tool_binding_inventory_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase6_reserve_formal_tool_binding_inventory_threshold_manifest",
            inventory_kind: None,
            fixed_entry_criteria_refs: vec![],
            first_reserve_ref: None,
            second_reserve_ref: None,
            minimum_guard_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory.guard_only"
                    .to_string(),
            ],
            guard_refs: actual_phase6_reserve_formal_tool_binding_inventory_threshold_guard_refs(
                false,
            ),
            kept_later_refs:
                actual_phase6_reserve_formal_tool_binding_inventory_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase6 reserve formal tool binding inventory threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 next-reopen sequencing threshold reaches the sequencing minimum for `{sample_id}`"
            )),
        }
    }
}

impl CurrentL2OperationalCliActualPhase6ParserSideFollowupPackageSequencingThresholdSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        actual_phase6_reserve_formal_tool_binding_inventory_threshold:
            &CurrentL2OperationalCliActualPhase6ReserveFormalToolBindingInventoryThresholdSummary,
    ) -> Self {
        let sample_id = report.sample_id.as_str();
        let reached = matches!(
            sample_id,
            "p07-dice-late-join-visible-history"
                | "p08-dice-stale-reconnect-refresh"
                | "p09-dice-delegated-rng-provider-placement"
        ) && actual_phase6_reserve_formal_tool_binding_inventory_threshold.status
            == "reached";
        let manifest = current_l2_phase6_parser_side_followup_package_sequencing_manifest();

        if reached {
            let mut compare_floor_refs =
                actual_phase6_reserve_formal_tool_binding_inventory_threshold
                    .compare_floor_refs
                    .clone();
            compare_floor_refs.push(
                "compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing"
                    .to_string(),
            );

            let mut evidence_refs = actual_phase6_reserve_formal_tool_binding_inventory_threshold
                .evidence_refs
                .clone();
            evidence_refs.push(
                "helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold"
                    .to_string(),
            );
            evidence_refs.push(
                "source:phase6_parser_side_followup_package_sequencing_current_first_choice"
                    .to_string(),
            );
            evidence_refs
                .push("source:phase6_parser_side_followup_package_sequencing_minimum".to_string());

            return Self {
                status: "reached",
                threshold_kind: "phase6_parser_side_followup_package_sequencing_threshold_manifest",
                sequencing_kind: Some(manifest.sequencing_kind),
                fixed_entry_criteria_refs: manifest
                    .fixed_entry_criteria_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                selected_next_package_ref: Some(manifest.selected_next_package_ref),
                deferred_reopen_refs: manifest
                    .deferred_reopen_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                minimum_guard_refs: manifest
                    .guard_refs
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                next_comparison_target_ref: Some(
                    "phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison",
                ),
                evidence_refs,
                compare_floor_refs,
                guard_refs:
                    actual_phase6_parser_side_followup_package_sequencing_threshold_guard_refs(true),
                kept_later_refs:
                    actual_phase6_parser_side_followup_package_sequencing_threshold_kept_later_refs(
                    ),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            threshold_kind: "phase6_parser_side_followup_package_sequencing_threshold_manifest",
            sequencing_kind: None,
            fixed_entry_criteria_refs: vec![],
            selected_next_package_ref: None,
            deferred_reopen_refs: vec![],
            minimum_guard_refs: vec![],
            next_comparison_target_ref: None,
            evidence_refs: vec![
                format!("sample:{sample_id}"),
                "helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold"
                    .to_string(),
                "compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing"
                    .to_string(),
            ],
            compare_floor_refs: vec![
                "compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing.guard_only"
                    .to_string(),
            ],
            guard_refs:
                actual_phase6_parser_side_followup_package_sequencing_threshold_guard_refs(false),
            kept_later_refs:
                actual_phase6_parser_side_followup_package_sequencing_threshold_kept_later_refs(),
            guard_reason: Some(format!(
                "current actual phase6 parser-side follow-up package sequencing threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 reserve formal tool binding inventory threshold reaches the reserve-inventory minimum for `{sample_id}`"
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliOrderHandoffWitnessProviderPublicSeamCompressionSummary {
    status: &'static str,
    compression_kind: &'static str,
    profile_axis_refs: Vec<String>,
    repo_local_emitted_artifact_refs: Vec<String>,
    source_wording_route_refs: Vec<String>,
    emitted_artifact_candidate_keep_refs: Vec<String>,
    serial_scope_lines: Vec<String>,
    witness_schema_route_refs: Vec<String>,
    provider_receipt_route_refs: Vec<String>,
    combined_public_contract_keep_refs: Vec<String>,
    trace_alignment_pair_refs: Vec<String>,
    public_seam_residual_refs: Vec<String>,
    evidence_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliOrderHandoffWitnessProviderPublicSeamCompressionSummary {
    fn from_source_report(
        report: &CurrentL2SourceSampleRunReport,
        verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
        artifact_preview: &CurrentL2OperationalCliArtifactPreviewSummary,
        _surface_preview: &CurrentL2OperationalCliSurfacePreviewSummary,
    ) -> Self {
        let reached = matches!(
            report.sample_id.as_str(),
            "p07-dice-late-join-visible-history" | "p08-dice-stale-reconnect-refresh"
        ) && verification_preview.formal_hook_status == "reached";

        if reached {
            let sample_id = report.sample_id.as_str();
            let repo_local_emitted_artifact_refs =
                order_handoff_repo_local_emitted_artifact_refs(sample_id, artifact_preview);
            return Self {
                status: "reached",
                compression_kind: "helper_local_public_seam_manifest",
                profile_axis_refs: authoritative_room_profile_axis_refs(sample_id),
                repo_local_emitted_artifact_refs,
                source_wording_route_refs: order_handoff_source_wording_actual_route_refs(
                    sample_id,
                ),
                emitted_artifact_candidate_keep_refs: order_handoff_emitted_artifact_keep_refs(
                    sample_id,
                ),
                serial_scope_lines: order_handoff_serial_scope_reserve_surface_lines(sample_id),
                witness_schema_route_refs: witness_provider_schema_route_witness_refs(sample_id),
                provider_receipt_route_refs: witness_provider_schema_route_provider_refs(sample_id),
                combined_public_contract_keep_refs:
                    witness_provider_combined_public_contract_keep_refs(sample_id),
                trace_alignment_pair_refs: witness_provider_trace_alignment_pair_refs(
                    sample_id,
                    artifact_preview,
                ),
                public_seam_residual_refs: order_handoff_witness_provider_public_seam_residual_refs(
                    sample_id,
                ),
                evidence_refs: order_handoff_witness_provider_public_seam_evidence_refs(sample_id),
                compare_floor_refs: order_handoff_witness_provider_public_seam_compare_floor_refs(
                    true,
                ),
                guard_refs: order_handoff_witness_provider_public_seam_guard_refs(true),
                kept_later_refs: order_handoff_witness_provider_public_seam_kept_later_refs(),
                guard_reason: None,
            };
        }

        Self {
            status: "guarded_not_reached",
            compression_kind: "helper_local_public_seam_manifest",
            profile_axis_refs: Vec::new(),
            repo_local_emitted_artifact_refs: Vec::new(),
            source_wording_route_refs: Vec::new(),
            emitted_artifact_candidate_keep_refs: Vec::new(),
            serial_scope_lines: Vec::new(),
            witness_schema_route_refs: Vec::new(),
            provider_receipt_route_refs: Vec::new(),
            combined_public_contract_keep_refs: Vec::new(),
            trace_alignment_pair_refs: Vec::new(),
            public_seam_residual_refs: Vec::new(),
            evidence_refs: order_handoff_witness_provider_public_seam_evidence_refs(
                &report.sample_id,
            ),
            compare_floor_refs: order_handoff_witness_provider_public_seam_compare_floor_refs(
                false,
            ),
            guard_refs: order_handoff_witness_provider_public_seam_guard_refs(false),
            kept_later_refs: order_handoff_witness_provider_public_seam_kept_later_refs(),
            guard_reason: Some(format!(
                "current order-handoff/witness-provider public seam compression only actualizes the representative authoritative-room pair (`p07` / `p08`) after route/reserve/bridge/threshold floors align for `{}`",
                report.sample_id
            )),
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
    bridge_floor_refs: Vec<String>,
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
                bridge_floor_refs: Vec::new(),
                compare_floor_refs: theorem_result_object_preview_compare_floor_refs(true),
                guard_refs: theorem_result_object_preview_guard_refs(true),
                kept_later_refs: theorem_result_object_preview_kept_later_refs(),
                guard_reason: None,
            };
        }

        let bridge_floor_refs = theorem_guard_bridge_floor_refs(&report.sample_id);
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
            bridge_floor_refs,
            compare_floor_refs: theorem_result_object_preview_compare_floor_refs(false),
            guard_refs: theorem_result_object_preview_guard_refs(false),
            kept_later_refs: theorem_result_object_preview_kept_later_refs(),
            guard_reason: Some(theorem_result_object_preview_guard_reason(
                &report.sample_id,
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
    bridge_floor_refs: Vec<String>,
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
                bridge_floor_refs: Vec::new(),
                compare_floor_refs: model_check_public_checker_preview_compare_floor_refs(true),
                guard_refs: model_check_public_checker_preview_guard_refs(true),
                kept_later_refs: model_check_public_checker_preview_kept_later_refs(),
                guard_reason: None,
            };
        }

        let bridge_floor_refs = model_check_guard_bridge_floor_refs(
            verification_preview.subject_ref.as_deref(),
            &report.sample_id,
        );
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
            bridge_floor_refs,
            compare_floor_refs: model_check_public_checker_preview_compare_floor_refs(false),
            guard_refs: model_check_public_checker_preview_guard_refs(false),
            kept_later_refs: model_check_public_checker_preview_kept_later_refs(),
            guard_reason: Some(model_check_public_checker_preview_guard_reason(
                &report.sample_id,
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliTheoremFinalPublicContractReopenThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    subject_kind: Option<&'static str>,
    subject_ref: Option<String>,
    result_object_route_refs: Vec<String>,
    payload_preview_keep_refs: Vec<String>,
    proof_object_schema_candidate_refs: Vec<String>,
    prover_brand_candidate_refs: Vec<String>,
    final_public_contract_reopen_sequence_refs: Vec<String>,
    threshold_default_refs: Vec<String>,
    evidence_refs: Vec<String>,
    bridge_floor_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliTheoremFinalPublicContractReopenThresholdSummary {
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
                threshold_kind: "helper_local_reopen_threshold_manifest",
                subject_kind: verification_preview.subject_kind,
                subject_ref: subject_ref.clone(),
                result_object_route_refs: theorem_result_object_actual_route_refs(
                    subject_ref.as_deref(),
                ),
                payload_preview_keep_refs: theorem_result_object_payload_preview_keep_refs(
                    subject_ref.as_deref(),
                ),
                proof_object_schema_candidate_refs: theorem_proof_object_schema_candidate_refs(
                    subject_ref.as_deref(),
                ),
                prover_brand_candidate_refs: theorem_prover_brand_candidate_refs(
                    subject_ref.as_deref(),
                ),
                final_public_contract_reopen_sequence_refs:
                    theorem_final_public_contract_reopen_sequence_refs(subject_ref.as_deref()),
                threshold_default_refs: theorem_final_public_contract_reopen_threshold_default_refs(
                ),
                evidence_refs: theorem_final_public_contract_reopen_threshold_evidence_refs(
                    &report.sample_id,
                ),
                bridge_floor_refs: Vec::new(),
                compare_floor_refs:
                    theorem_final_public_contract_reopen_threshold_compare_floor_refs(true),
                guard_refs: theorem_final_public_contract_reopen_threshold_guard_refs(true),
                kept_later_refs: theorem_final_public_contract_reopen_threshold_kept_later_refs(),
                guard_reason: None,
            };
        }

        let bridge_floor_refs = theorem_guard_bridge_floor_refs(&report.sample_id);
        Self {
            status: "guarded_not_reached",
            threshold_kind: "helper_local_reopen_threshold_manifest",
            subject_kind: verification_preview.subject_kind,
            subject_ref: verification_preview.subject_ref.clone(),
            result_object_route_refs: Vec::new(),
            payload_preview_keep_refs: Vec::new(),
            proof_object_schema_candidate_refs: Vec::new(),
            prover_brand_candidate_refs: Vec::new(),
            final_public_contract_reopen_sequence_refs: Vec::new(),
            threshold_default_refs: Vec::new(),
            evidence_refs: theorem_final_public_contract_reopen_threshold_evidence_refs(
                &report.sample_id,
            ),
            bridge_floor_refs,
            compare_floor_refs: theorem_final_public_contract_reopen_threshold_compare_floor_refs(
                false,
            ),
            guard_refs: theorem_final_public_contract_reopen_threshold_guard_refs(false),
            kept_later_refs: theorem_final_public_contract_reopen_threshold_kept_later_refs(),
            guard_reason: Some(theorem_final_public_contract_reopen_threshold_guard_reason(
                &report.sample_id,
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct CurrentL2OperationalCliModelCheckFinalPublicContractReopenThresholdSummary {
    status: &'static str,
    threshold_kind: &'static str,
    subject_kind: Option<&'static str>,
    subject_ref: Option<String>,
    checker_artifact_route_refs: Vec<String>,
    migration_candidate_keep_refs: Vec<String>,
    verifier_handoff_candidate_refs: Vec<String>,
    tool_brand_candidate_refs: Vec<String>,
    final_public_contract_reopen_sequence_refs: Vec<String>,
    threshold_default_refs: Vec<String>,
    evidence_refs: Vec<String>,
    bridge_floor_refs: Vec<String>,
    compare_floor_refs: Vec<String>,
    guard_refs: Vec<String>,
    kept_later_refs: Vec<String>,
    guard_reason: Option<String>,
}

impl CurrentL2OperationalCliModelCheckFinalPublicContractReopenThresholdSummary {
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
                threshold_kind: "helper_local_reopen_threshold_manifest",
                subject_kind: verification_preview.subject_kind,
                subject_ref: subject_ref.clone(),
                checker_artifact_route_refs: model_check_checker_artifact_actual_route_refs(
                    subject_ref.as_deref(),
                ),
                migration_candidate_keep_refs: model_check_checker_artifact_migration_keep_refs(
                    subject_ref.as_deref(),
                ),
                verifier_handoff_candidate_refs: model_check_verifier_handoff_candidate_refs(
                    subject_ref.as_deref(),
                ),
                tool_brand_candidate_refs: model_check_tool_brand_candidate_refs(
                    subject_ref.as_deref(),
                ),
                final_public_contract_reopen_sequence_refs:
                    model_check_final_public_contract_reopen_sequence_refs(subject_ref.as_deref()),
                threshold_default_refs:
                    model_check_final_public_contract_reopen_threshold_default_refs(),
                evidence_refs: model_check_final_public_contract_reopen_threshold_evidence_refs(
                    &report.sample_id,
                ),
                bridge_floor_refs: Vec::new(),
                compare_floor_refs:
                    model_check_final_public_contract_reopen_threshold_compare_floor_refs(true),
                guard_refs: model_check_final_public_contract_reopen_threshold_guard_refs(true),
                kept_later_refs: model_check_final_public_contract_reopen_threshold_kept_later_refs(
                ),
                guard_reason: None,
            };
        }

        let bridge_floor_refs = model_check_guard_bridge_floor_refs(
            verification_preview.subject_ref.as_deref(),
            &report.sample_id,
        );
        Self {
            status: "guarded_not_reached",
            threshold_kind: "helper_local_reopen_threshold_manifest",
            subject_kind: verification_preview.subject_kind,
            subject_ref: verification_preview.subject_ref.clone(),
            checker_artifact_route_refs: Vec::new(),
            migration_candidate_keep_refs: Vec::new(),
            verifier_handoff_candidate_refs: Vec::new(),
            tool_brand_candidate_refs: Vec::new(),
            final_public_contract_reopen_sequence_refs: Vec::new(),
            threshold_default_refs: Vec::new(),
            evidence_refs: model_check_final_public_contract_reopen_threshold_evidence_refs(
                &report.sample_id,
            ),
            bridge_floor_refs,
            compare_floor_refs:
                model_check_final_public_contract_reopen_threshold_compare_floor_refs(false),
            guard_refs: model_check_final_public_contract_reopen_threshold_guard_refs(false),
            kept_later_refs: model_check_final_public_contract_reopen_threshold_kept_later_refs(),
            guard_reason: Some(
                model_check_final_public_contract_reopen_threshold_guard_reason(&report.sample_id),
            ),
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
    writeln!(output, "authoritative_room_first_scenario_actual_adoption:")
        .expect("write to string");
    render_authoritative_room_first_scenario_actual_adoption(
        &mut output,
        &summary.authoritative_room_first_scenario_actual_adoption,
    );
    writeln!(output, "authoritative_room_reserve_strengthening_lane:").expect("write to string");
    render_authoritative_room_reserve_strengthening_lane(
        &mut output,
        &summary.authoritative_room_reserve_strengthening_lane,
    );
    writeln!(
        output,
        "order_handoff_source_surface_artifact_actual_adoption:"
    )
    .expect("write to string");
    render_order_handoff_source_surface_artifact_actual_adoption(
        &mut output,
        &summary.order_handoff_source_surface_artifact_actual_adoption,
    );
    writeln!(
        output,
        "order_handoff_witness_provider_public_seam_compression:"
    )
    .expect("write to string");
    render_order_handoff_witness_provider_public_seam_compression(
        &mut output,
        &summary.order_handoff_witness_provider_public_seam_compression,
    );
    writeln!(output, "theorem_result_object_preview:").expect("write to string");
    render_theorem_result_object_preview(&mut output, &summary.theorem_result_object_preview);
    writeln!(output, "model_check_public_checker_preview:").expect("write to string");
    render_model_check_public_checker_preview(
        &mut output,
        &summary.model_check_public_checker_preview,
    );
    writeln!(output, "theorem_final_public_contract_reopen_threshold:").expect("write to string");
    render_theorem_final_public_contract_reopen_threshold(
        &mut output,
        &summary.theorem_final_public_contract_reopen_threshold,
    );
    writeln!(
        output,
        "model_check_final_public_contract_reopen_threshold:"
    )
    .expect("write to string");
    render_model_check_final_public_contract_reopen_threshold(
        &mut output,
        &summary.model_check_final_public_contract_reopen_threshold,
    );
    writeln!(output, "typed_checker_hint_preview:").expect("write to string");
    render_typed_checker_hint_preview(&mut output, &summary.typed_checker_hint_preview);
    writeln!(output, "actual_checker_payload_family_threshold:").expect("write to string");
    render_actual_checker_payload_family_threshold(
        &mut output,
        &summary.actual_checker_payload_family_threshold,
    );
    writeln!(output, "actual_checker_payload_row_family_threshold:").expect("write to string");
    render_actual_checker_payload_row_family_threshold(
        &mut output,
        &summary.actual_checker_payload_row_family_threshold,
    );
    writeln!(output, "actual_checker_payload_row_detail_threshold:").expect("write to string");
    render_actual_checker_payload_row_detail_threshold(
        &mut output,
        &summary.actual_checker_payload_row_detail_threshold,
    );
    writeln!(output, "actual_checker_payload_row_body_threshold:").expect("write to string");
    render_actual_checker_payload_row_body_threshold(
        &mut output,
        &summary.actual_checker_payload_row_body_threshold,
    );
    writeln!(
        output,
        "actual_checker_payload_supported_kind_summary_threshold:"
    )
    .expect("write to string");
    render_actual_checker_payload_supported_kind_summary_threshold(
        &mut output,
        &summary.actual_checker_payload_supported_kind_summary_threshold,
    );
    writeln!(
        output,
        "actual_checker_payload_public_schema_sketch_threshold:"
    )
    .expect("write to string");
    render_actual_checker_payload_public_schema_sketch_threshold(
        &mut output,
        &summary.actual_checker_payload_public_schema_sketch_threshold,
    );
    writeln!(output, "actual_public_checker_api_sketch_threshold:").expect("write to string");
    render_actual_public_checker_api_sketch_threshold(
        &mut output,
        &summary.actual_public_checker_api_sketch_threshold,
    );
    writeln!(output, "actual_public_checker_entry_criteria_threshold:").expect("write to string");
    render_actual_public_checker_entry_criteria_threshold(
        &mut output,
        &summary.actual_public_checker_entry_criteria_threshold,
    );
    writeln!(output, "actual_public_checker_command_surface_threshold:").expect("write to string");
    render_actual_public_checker_command_surface_threshold(
        &mut output,
        &summary.actual_public_checker_command_surface_threshold,
    );
    writeln!(output, "actual_shared_output_contract_threshold:").expect("write to string");
    render_actual_shared_output_contract_threshold(
        &mut output,
        &summary.actual_shared_output_contract_threshold,
    );
    writeln!(output, "actual_public_checker_boundary_threshold:").expect("write to string");
    render_actual_public_checker_boundary_threshold(
        &mut output,
        &summary.actual_public_checker_boundary_threshold,
    );
    writeln!(output, "actual_verifier_handoff_surface_threshold:").expect("write to string");
    render_actual_verifier_handoff_surface_threshold(
        &mut output,
        &summary.actual_verifier_handoff_surface_threshold,
    );
    writeln!(output, "actual_minimal_parser_subset_freeze_threshold:").expect("write to string");
    render_actual_minimal_parser_subset_freeze_threshold(
        &mut output,
        &summary.actual_minimal_parser_subset_freeze_threshold,
    );
    writeln!(
        output,
        "actual_parser_to_checker_reconnect_freeze_threshold:"
    )
    .expect("write to string");
    render_actual_parser_to_checker_reconnect_freeze_threshold(
        &mut output,
        &summary.actual_parser_to_checker_reconnect_freeze_threshold,
    );
    writeln!(output, "actual_phase1_semantics_closeout_threshold:").expect("write to string");
    render_actual_phase1_semantics_closeout_threshold(
        &mut output,
        &summary.actual_phase1_semantics_closeout_threshold,
    );
    writeln!(output, "actual_phase2_parser_free_poc_closeout_threshold:").expect("write to string");
    render_actual_phase2_parser_free_poc_closeout_threshold(
        &mut output,
        &summary.actual_phase2_parser_free_poc_closeout_threshold,
    );
    writeln!(
        output,
        "actual_phase4_shared_space_self_driven_closeout_threshold:"
    )
    .expect("write to string");
    render_actual_phase4_shared_space_self_driven_closeout_threshold(
        &mut output,
        &summary.actual_phase4_shared_space_self_driven_closeout_threshold,
    );
    writeln!(
        output,
        "actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:"
    )
    .expect("write to string");
    render_actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold(
        &mut output,
        &summary.actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold,
    );
    writeln!(
        output,
        "actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:"
    )
    .expect("write to string");
    render_actual_phase6_actual_parser_ast_carrier_first_tranche_threshold(
        &mut output,
        &summary.actual_phase6_actual_parser_ast_carrier_first_tranche_threshold,
    );
    writeln!(
        output,
        "actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:"
    )
    .expect("write to string");
    render_actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold(
        &mut output,
        &summary.actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold,
    );
    writeln!(
        output,
        "actual_phase6_compile_ready_verification_and_formal_hook_threshold:"
    )
    .expect("write to string");
    render_actual_phase6_compile_ready_verification_and_formal_hook_threshold(
        &mut output,
        &summary.actual_phase6_compile_ready_verification_and_formal_hook_threshold,
    );
    writeln!(output, "actual_phase6_next_reopen_sequencing_threshold:").expect("write to string");
    render_actual_phase6_next_reopen_sequencing_threshold(
        &mut output,
        &summary.actual_phase6_next_reopen_sequencing_threshold,
    );
    writeln!(
        output,
        "actual_phase6_reserve_formal_tool_binding_inventory_threshold:"
    )
    .expect("write to string");
    render_actual_phase6_reserve_formal_tool_binding_inventory_threshold(
        &mut output,
        &summary.actual_phase6_reserve_formal_tool_binding_inventory_threshold,
    );
    writeln!(
        output,
        "actual_phase6_parser_side_followup_package_sequencing_threshold:"
    )
    .expect("write to string");
    render_actual_phase6_parser_side_followup_package_sequencing_threshold(
        &mut output,
        &summary.actual_phase6_parser_side_followup_package_sequencing_threshold,
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

fn typed_checker_hint_evidence_refs(sample_id: &str) -> Vec<String> {
    let foundation_evidence_ref = current_l2_first_strong_typing_sample_manifest(sample_id)
        .map(|manifest| manifest.foundation_evidence_ref)
        .unwrap_or("spec_example:current_l2_first_strong_typing_layer_finite_index_spine_default");

    vec![
        format!("sample:{sample_id}"),
        foundation_evidence_ref.to_string(),
        "helper_preview:typed_checker_hint_preview".to_string(),
    ]
}

fn theorem_guard_bridge_floor_refs(sample_id: &str) -> Vec<String> {
    let Some(manifest) = current_l2_first_strong_typing_sample_manifest(sample_id) else {
        return Vec::new();
    };

    vec![
        manifest.foundation_evidence_ref.to_string(),
        manifest.primary_compare_floor_ref.to_string(),
        "helper_preview:typed_checker_hint_preview".to_string(),
        format!("proof_notebook_review_unit:{sample_id}:rollback_cut_non_interference"),
        format!("theorem_binding_preflight:{sample_id}:rollback_cut_non_interference"),
        format!("theorem_lean_stub_pilot:{sample_id}:lean_first_principal"),
        format!(
            "repo_local_emitted_artifact:lean_theorem_stub:{sample_id}:rollback_cut_non_interference"
        ),
    ]
}

fn theorem_result_object_preview_guard_reason(sample_id: &str) -> String {
    let base = format!(
        "current theorem result-object preview only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `{sample_id}`"
    );

    if current_l2_first_strong_typing_sample_reached(sample_id) {
        format!(
            "{base}; current first strong typing sample stays on checker-adjacent / Lean-first theorem bridge floors until theorem result-object preview itself is widened"
        )
    } else {
        base
    }
}

fn model_check_guard_bridge_floor_refs(subject_ref: Option<&str>, sample_id: &str) -> Vec<String> {
    let Some(manifest) = current_l2_first_strong_typing_sample_manifest(sample_id) else {
        return Vec::new();
    };
    let subject_ref = subject_ref.unwrap_or(sample_id);

    vec![
        manifest.primary_compare_floor_ref.to_string(),
        "helper_preview:typed_checker_hint_preview".to_string(),
        "compare_floor:current_l2.model_check_projection_prefloor".to_string(),
        "compare_floor:current_l2.model_check.second_line_concretization".to_string(),
        format!("small_cluster_projection:{subject_ref}:runtime_try_cut_local"),
        format!("model_check_request_preflight:{subject_ref}:row_local_property_preview"),
        format!("model_check_property_route:{subject_ref}:row_local_preview_bundle"),
        format!(
            "model_check_checker_contract_route:{subject_ref}:checker_boundary_contract_anchor"
        ),
    ]
}

fn model_check_public_checker_preview_guard_reason(sample_id: &str) -> String {
    let base = format!(
        "current model-check public-checker preview only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `{sample_id}`"
    );

    if current_l2_first_strong_typing_sample_reached(sample_id) {
        format!(
            "{base}; current first strong typing sample stays on reached row-local carrier / property-tool bridge floors until public-checker preview itself is widened"
        )
    } else {
        base
    }
}

fn theorem_final_public_contract_reopen_threshold_guard_reason(sample_id: &str) -> String {
    let base = format!(
        "current theorem final public-contract reopen threshold only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `{sample_id}`"
    );

    if current_l2_first_strong_typing_sample_reached(sample_id) {
        format!(
            "{base}; current first strong typing sample stays on checker-adjacent / Lean-first theorem bridge floors before theorem public-seam reopen is widened"
        )
    } else {
        base
    }
}

fn model_check_final_public_contract_reopen_threshold_guard_reason(sample_id: &str) -> String {
    let base = format!(
        "current model-check final public-contract reopen threshold only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `{sample_id}`"
    );

    if current_l2_first_strong_typing_sample_reached(sample_id) {
        format!(
            "{base}; current first strong typing sample stays on reached row-local carrier / property-tool bridge floors before model-check public-contract reopen is widened"
        )
    } else {
        base
    }
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

fn order_handoff_witness_provider_public_seam_evidence_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:order_handoff_witness_provider_public_seam_compression".to_string(),
        "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression"
            .to_string(),
    ]
}

fn authoritative_room_profile_axis_refs(sample_id: &str) -> Vec<String> {
    let mut refs = vec![
        "profile_axis:activation:authority-ack".to_string(),
        "profile_axis:authority_placement:single_room_authority".to_string(),
        "profile_axis:consistency_mode:authoritative_serial_transition".to_string(),
        "profile_axis:rng_source:authority_rng".to_string(),
    ];

    match sample_id {
        "p07-dice-late-join-visible-history" => {
            refs.push("profile_axis:late_join:published_history_visible_as_past".to_string())
        }
        "p08-dice-stale-reconnect-refresh" => {
            refs.push("profile_axis:stale_reconnect:fail_then_refresh".to_string());
            refs.push("profile_axis:replay:stale_incompatible_replay_invalidated".to_string());
        }
        _ => {}
    }

    refs.push("profile_axis:fairness_claim:no_distributed_fairness_theorem_required".to_string());
    refs
}

fn authoritative_room_first_scenario_relation_refs(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "relation_family:program_order".to_string(),
            "relation_family:publication_order".to_string(),
            "relation_family:observation_order".to_string(),
            "relation_family:witness_order".to_string(),
            "relation_family:finalization_order".to_string(),
            "relation_family:scoped_happens_before".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "relation_family:program_order".to_string(),
            "relation_family:observation_order".to_string(),
            "relation_family:witness_order".to_string(),
            "relation_family:finalization_order".to_string(),
            "relation_family:scoped_happens_before".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn authoritative_room_first_scenario_handoff_refs(sample_id: &str) -> Vec<String> {
    let mut refs = vec![
        "authority_handoff:owner_slot:single_room_authority".to_string(),
        "authority_handoff:stage_family:authoritative_serial_transition".to_string(),
        "authority_handoff:payload_ref:dice_state".to_string(),
    ];

    match sample_id {
        "p07-dice-late-join-visible-history" => {
            refs.insert(
                2,
                "authority_handoff:stage_sequence:publish_then_handoff".to_string(),
            );
        }
        "p08-dice-stale-reconnect-refresh" => {
            refs.insert(
                2,
                "authority_handoff:stage_sequence:fail_then_refresh".to_string(),
            );
        }
        _ => {}
    }

    refs
}

fn authoritative_room_first_scenario_runtime_evidence_refs(
    report: &CurrentL2SourceSampleRunReport,
) -> Vec<String> {
    let mut refs = Vec::new();

    for event in &report.runtime_report.run_report.trace_audit_sink.events {
        let formatted = format!(
            "runtime_event:{}",
            authoritative_room_first_scenario_event_kind_ref(event)
        );
        if !refs.contains(&formatted) {
            refs.push(formatted);
        }
    }

    match report.sample_id.as_str() {
        "p07-dice-late-join-visible-history" => {
            authoritative_room_first_scenario_extend_place_records(
                &mut refs,
                &report.runtime_report.run_report.final_place_store,
                "dice_state",
            );
            authoritative_room_first_scenario_extend_place_records(
                &mut refs,
                &report.runtime_report.run_report.final_place_store,
                "observer_debug_text_output",
            );
        }
        "p08-dice-stale-reconnect-refresh" => {
            authoritative_room_first_scenario_extend_place_records(
                &mut refs,
                &report.runtime_report.run_report.final_place_store,
                "dice_state",
            );
            authoritative_room_first_scenario_extend_place_records(
                &mut refs,
                &report.runtime_report.run_report.final_place_store,
                "reconnect_debug_text_output",
            );
        }
        _ => {}
    }

    refs
}

fn authoritative_room_first_scenario_reserve_route_refs(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p09-dice-delegated-rng-provider-placement" => vec![
            format!("reserve_route:delegated_rng_service_practical:{sample_id}"),
            format!("reserve_route:serial_scope_reserve_surface:{sample_id}"),
            format!("reserve_route:witness_provider_route_actual_adoption:{sample_id}"),
        ],
        _ => Vec::new(),
    }
}

fn authoritative_room_first_scenario_contrast_refs() -> Vec<String> {
    vec!["contrast_target:append_friendly_notice_room".to_string()]
}

fn authoritative_room_first_scenario_evidence_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:authoritative_room_first_scenario_actual_adoption".to_string(),
        "compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption".to_string(),
    ]
}

fn authoritative_room_first_scenario_compare_floor_refs(
    sample_id: &str,
    reached: bool,
) -> Vec<String> {
    if reached {
        return vec![
            "compare_floor:current_l2.authoritative_room.vertical_slice".to_string(),
            "compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption"
                .to_string(),
            "compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout"
                .to_string(),
            "compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption"
                .to_string(),
        ];
    }

    if sample_id == "p09-dice-delegated-rng-provider-placement" {
        return vec![
            "compare_floor:current_l2.delegated_rng_service.practical".to_string(),
            "compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout"
                .to_string(),
            "compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption.guard_only"
                .to_string(),
        ];
    }

    if matches!(
        sample_id,
        "p13-dice-late-join-missing-publication-witness"
            | "p14-dice-late-join-handoff-before-publication"
    ) {
        return vec![
            "compare_floor:current_l2.order_handoff.negative_static_stop_actualization".to_string(),
            "compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption.guard_only"
                .to_string(),
        ];
    }

    vec![
        "compare_floor:current_l2.authoritative_room.guard_only".to_string(),
        "compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption.guard_only"
            .to_string(),
    ]
}

fn authoritative_room_first_scenario_guard_refs(sample_id: &str, reached: bool) -> Vec<String> {
    if reached {
        let mut refs = vec![
            "guard:authoritative_room_first_default_profile".to_string(),
            "guard:representative_first_scenario_pair".to_string(),
            "guard:no_distributed_fairness_theorem_required".to_string(),
            "guard:minimal_working_subset_only".to_string(),
        ];
        match sample_id {
            "p07-dice-late-join-visible-history" => {
                refs.push("guard:late_join_history_visible_as_past".to_string());
            }
            "p08-dice-stale-reconnect-refresh" => {
                refs.push("guard:stale_reconnect_fail_then_refresh".to_string());
                refs.push("guard:stale_replay_invalidated_not_merged".to_string());
            }
            _ => {}
        }
        return refs;
    }

    match sample_id {
        "p09-dice-delegated-rng-provider-placement" => vec![
            "guard:delegated_rng_service_practical_reserve".to_string(),
            "guard:first_scenario_pair_unchanged".to_string(),
        ],
        "p13-dice-late-join-missing-publication-witness"
        | "p14-dice-late-join-handoff-before-publication" => vec![
            "guard:late_join_negative_static_stop".to_string(),
            "guard:first_scenario_pair_unchanged".to_string(),
        ],
        _ => vec!["guard:authoritative_room_first_scenario_not_reached".to_string()],
    }
}

fn authoritative_room_first_scenario_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:auditable_authority_witness".to_string(),
        "kept_later:delegated_rng_service".to_string(),
        "kept_later:distributed_randomness_provider".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
        "kept_later:final_consistency_fairness_catalog".to_string(),
    ]
}

fn authoritative_room_reserve_strengthening_witness_refs(
    sample_id: &str,
    reached: bool,
) -> Vec<String> {
    if !reached {
        return Vec::new();
    }

    vec![
        "fairness_claim:auditable_authority_witness".to_string(),
        "witness_field:witness_kind".to_string(),
        "witness_field:action_ref".to_string(),
        "witness_field:draw_slot".to_string(),
        "witness_field:draw_result".to_string(),
        format!("witness_binding:{sample_id}:authority_draw_witness"),
    ]
}

fn authoritative_room_reserve_strengthening_delegated_rng_refs(
    sample_id: &str,
    reached: bool,
) -> Vec<String> {
    if !reached {
        return Vec::new();
    }

    vec![
        "profile_axis:rng_source:delegated_rng_service".to_string(),
        "provider_boundary:placement:delegated_rng_service".to_string(),
        "provider_boundary:authority_keeps_commit".to_string(),
        "optional_attachment:provider_draw_ref".to_string(),
        format!("provider_sample:{sample_id}:delegated_rng_draw_route"),
    ]
}

fn authoritative_room_reserve_strengthening_model_check_refs(
    sample_id: &str,
    reached: bool,
) -> Vec<String> {
    if !reached {
        return Vec::new();
    }

    vec![
        format!("property_preview:row_local:{sample_id}:canonical_normalization_law"),
        format!("property_preview:row_local:{sample_id}:no_re_promotion"),
        format!("model_check_request_preflight:{sample_id}:row_local_property_preview"),
        format!("model_check_request_preflight:{sample_id}:small_cluster_semantic_projection"),
        "public_checker_second_reserve:boundary".to_string(),
    ]
}

fn authoritative_room_reserve_strengthening_first_line_boundary_refs(
    sample_id: &str,
) -> Vec<String> {
    let mut refs = vec![
        "first_line_boundary:representative_pair_kept_at_p07_p08".to_string(),
        "first_line_boundary:authoritative_room_default_profile_stays_principal".to_string(),
        "first_line_boundary:authority_rng_default_profile_unchanged".to_string(),
    ];

    if sample_id == "p09-dice-delegated-rng-provider-placement" {
        refs.push("first_line_boundary:delegated_rng_not_promoted_into_default_pair".to_string());
    }

    refs
}

fn authoritative_room_reserve_strengthening_boundary_refs() -> Vec<String> {
    vec![
        "reserve_boundary:auditable_authority_witness_second_strengthening".to_string(),
        "reserve_boundary:delegated_rng_service_second_practical".to_string(),
        "reserve_boundary:model_check_second_line_not_room_profile".to_string(),
        "reserve_boundary:public_checker_contract_kept_later".to_string(),
        "reserve_boundary:witness_provider_combined_public_contract_later".to_string(),
    ]
}

fn authoritative_room_reserve_strengthening_compare_floor_refs(
    sample_id: &str,
    witness_reached: bool,
    delegated_reached: bool,
    model_check_reached: bool,
) -> Vec<String> {
    let mut refs = Vec::new();

    if witness_reached {
        refs.push("compare_floor:current_l2.auditable_authority_witness.strengthening".to_string());
    }
    if delegated_reached {
        refs.push("compare_floor:current_l2.delegated_rng_service.practical".to_string());
    }
    if model_check_reached {
        refs.push("compare_floor:current_l2.model_check.second_line_concretization".to_string());
    }
    if refs.is_empty() {
        refs.push("compare_floor:current_l2.reserve_strengthening_lane.guard_only".to_string());
    }
    refs.push(format!(
        "compare_floor:current_l2.reserve_strengthening_lane:{sample_id}"
    ));

    refs
}

fn authoritative_room_reserve_strengthening_guard_refs(
    witness_reached: bool,
    delegated_reached: bool,
    model_check_reached: bool,
) -> Vec<String> {
    if witness_reached || delegated_reached || model_check_reached {
        return vec![
            "guard:first_line_boundary_preserved".to_string(),
            "guard:reserve_components_kept_separate".to_string(),
            "guard:model_check_second_line_not_room_profile".to_string(),
            "guard:witness_provider_public_contract_later".to_string(),
        ];
    }

    vec![
        "guard:representative_reserve_strengthening_sample_set".to_string(),
        "guard:first_line_pair_unchanged".to_string(),
    ]
}

fn authoritative_room_reserve_strengthening_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:combined_witness_provider_public_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:concrete_model_check_tool_brand".to_string(),
        "kept_later:actual_public_checker_migration".to_string(),
        "kept_later:distributed_fairness_theorem".to_string(),
    ]
}

fn authoritative_room_reserve_strengthening_guard_reason(sample_id: &str) -> String {
    format!(
        "current authoritative-room reserve strengthening lane only actualizes the representative reserve strengthening sample set (`p07` witness, `p08` reconnect-model-check, `p09` delegated RNG); `{sample_id}` stays guard-only until one of those reserve routes is actually exercised"
    )
}

fn order_handoff_source_surface_artifact_actual_adoption_evidence_refs(
    sample_id: &str,
) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:order_handoff_source_surface_artifact_actual_adoption".to_string(),
        "compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption"
            .to_string(),
    ]
}

fn order_handoff_repo_local_emitted_artifact_refs(
    sample_id: &str,
    artifact_preview: &CurrentL2OperationalCliArtifactPreviewSummary,
) -> Vec<String> {
    let mut refs = Vec::new();
    for unit in &artifact_preview.proof_notebook_review_units {
        let artifact_ref = format!(
            "repo_local_emitted_artifact:proof_notebook_review_unit:{sample_id}:{}",
            unit.obligation_kind
        );
        if !refs.contains(&artifact_ref) {
            refs.push(artifact_ref);
        }
    }
    for carrier in &artifact_preview.model_check_concrete_carriers {
        let artifact_ref = format!(
            "repo_local_emitted_artifact:model_check_concrete_carrier:{sample_id}:{}",
            carrier.obligation_kind
        );
        if !refs.contains(&artifact_ref) {
            refs.push(artifact_ref);
        }
    }
    refs
}

fn order_handoff_source_surface_artifact_actual_adoption_compare_floor_refs(
    sample_id: &str,
    reached: bool,
) -> Vec<String> {
    if reached {
        vec![
            "compare_floor:current_l2.order_handoff.surface_actual_adoption".to_string(),
            "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption"
                .to_string(),
            "compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption"
                .to_string(),
        ]
    } else if matches!(
        sample_id,
        "p13-dice-late-join-missing-publication-witness"
            | "p14-dice-late-join-handoff-before-publication"
    ) {
        vec![
            "compare_floor:current_l2.order_handoff.negative_static_stop_actualization"
                .to_string(),
            "compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption.guard_only"
                .to_string(),
        ]
    } else {
        vec![
            "compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption.guard_only"
                .to_string(),
        ]
    }
}

fn order_handoff_source_surface_artifact_actual_adoption_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:edge_row_vertical_continuation_principal".to_string(),
            "guard:stage_block_secondary_keep".to_string(),
            "guard:repo_local_emitted_artifact_refs_first".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
            "guard:final_emitted_artifact_schema_later".to_string(),
        ]
    } else {
        vec![
            "guard:source_surface_artifact_actual_adoption_not_reached".to_string(),
            "guard:negative_static_stop_pair_kept_helper_local".to_string(),
        ]
    }
}

fn order_handoff_source_surface_artifact_actual_adoption_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_artifact_schema".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
    ]
}

fn order_handoff_source_wording_actual_route_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!(
            "order_handoff_source_wording_actual_route:{sample_id}:edge_row_vertical_continuation_principal"
        ),
        format!(
            "order_handoff_source_wording_actual_route:{sample_id}:readable_high_level_relation_vocabulary"
        ),
        format!("order_handoff_source_wording_actual_route:{sample_id}:stage_block_secondary_keep"),
        format!(
            "order_handoff_source_wording_actual_route:{sample_id}:thread_node_same_causal_language"
        ),
        format!(
            "order_handoff_source_wording_actual_route:{sample_id}:final_source_surface_handoff_wording_later"
        ),
    ]
}

fn order_handoff_emitted_artifact_keep_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!(
            "order_handoff_emitted_artifact_keep:{sample_id}:repo_local_emitted_artifact_refs_first"
        ),
        format!(
            "order_handoff_emitted_artifact_keep:{sample_id}:source_surface_actual_adoption_adjacent"
        ),
        format!(
            "order_handoff_emitted_artifact_keep:{sample_id}:witness_provider_contract_adjacent_not_collapsed"
        ),
        format!(
            "order_handoff_emitted_artifact_keep:{sample_id}:final_emitted_artifact_schema_later"
        ),
    ]
}

fn witness_provider_schema_route_witness_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("witness_provider_schema_route_actual:{sample_id}:witness_schema_candidate_keep"),
        format!("witness_provider_schema_route_actual:{sample_id}:witness_route_first"),
        format!(
            "witness_provider_schema_route_actual:{sample_id}:repo_local_emitted_artifact_refs_first"
        ),
        format!("witness_provider_schema_route_actual:{sample_id}:combined_public_contract_later"),
    ]
}

fn witness_provider_schema_route_provider_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("witness_provider_schema_route_actual:{sample_id}:provider_receipt_candidate_keep"),
        format!("witness_provider_schema_route_actual:{sample_id}:provider_route_first"),
        format!(
            "witness_provider_schema_route_actual:{sample_id}:repo_local_emitted_artifact_refs_first"
        ),
        format!(
            "witness_provider_schema_route_actual:{sample_id}:delegated_provider_attestation_later"
        ),
        format!("witness_provider_schema_route_actual:{sample_id}:combined_public_contract_later"),
    ]
}

fn witness_provider_combined_public_contract_keep_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!(
            "witness_provider_combined_contract_keep:{sample_id}:combined_public_contract_candidate_only"
        ),
        format!(
            "witness_provider_combined_contract_keep:{sample_id}:final_emitted_handoff_contract_adjacent_keep"
        ),
    ]
}

fn witness_provider_trace_alignment_pair_refs(
    sample_id: &str,
    artifact_preview: &CurrentL2OperationalCliArtifactPreviewSummary,
) -> Vec<String> {
    let mut obligation_kinds = Vec::new();
    for unit in &artifact_preview.proof_notebook_review_units {
        let obligation_kind = unit.obligation_kind.to_string();
        if !obligation_kinds.contains(&obligation_kind) {
            obligation_kinds.push(obligation_kind);
        }
    }
    for carrier in &artifact_preview.model_check_concrete_carriers {
        let obligation_kind = carrier.obligation_kind.to_string();
        if !obligation_kinds.contains(&obligation_kind) {
            obligation_kinds.push(obligation_kind);
        }
    }
    obligation_kinds.sort();
    obligation_kinds
        .into_iter()
        .map(|obligation_kind| {
            format!(
                "witness_provider_emitted_contract_trace_alignment_pair:{sample_id}:{obligation_kind}"
            )
        })
        .collect()
}

fn order_handoff_witness_provider_public_seam_residual_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!(
            "order_handoff_public_seam_residual:{sample_id}:final_source_surface_handoff_wording_later"
        ),
        format!(
            "order_handoff_public_seam_residual:{sample_id}:final_emitted_artifact_schema_later"
        ),
        format!("shared_space_public_seam_residual:{sample_id}:public_schema_pair_first"),
        format!(
            "shared_space_public_seam_residual:{sample_id}:delegated_attestation_and_combined_contract_second"
        ),
        format!(
            "shared_space_public_seam_residual:{sample_id}:final_emitted_handoff_contract_third"
        ),
    ]
}

fn order_handoff_witness_provider_public_seam_compare_floor_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption"
                .to_string(),
            "compare_floor:current_l2.order_handoff.serial_scope_reserve_surface".to_string(),
            "compare_floor:current_l2.witness_provider_emitted_contract_trace_alignment_bridge"
                .to_string(),
            "compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold"
                .to_string(),
            "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression"
                .to_string(),
        ]
    } else {
        vec![
            "compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression.guard_only"
                .to_string(),
        ]
    }
}

fn order_handoff_witness_provider_public_seam_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:edge_row_vertical_continuation_principal".to_string(),
            "guard:serial_scope_reserve_surface_only".to_string(),
            "guard:witness_provider_trace_alignment_bridge".to_string(),
            "guard:public_schema_pair_first".to_string(),
            "guard:delegated_attestation_and_combined_contract_second".to_string(),
            "guard:final_source_surface_handoff_wording_later".to_string(),
            "guard:final_emitted_artifact_schema_later".to_string(),
            "guard:final_emitted_handoff_contract_third".to_string(),
        ]
    } else {
        vec!["guard:order_handoff_witness_provider_public_seam_compression_not_reached".to_string()]
    }
}

fn order_handoff_witness_provider_public_seam_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_runtime_api".to_string(),
        "kept_later:final_source_surface_handoff_wording".to_string(),
        "kept_later:final_emitted_artifact_schema".to_string(),
        "kept_later:final_public_witness_schema".to_string(),
        "kept_later:final_public_provider_receipt_schema".to_string(),
        "kept_later:delegated_provider_attestation".to_string(),
        "kept_later:combined_provider_witness_public_contract".to_string(),
        "kept_later:final_emitted_handoff_contract".to_string(),
        "kept_later:authoritative_room_serial_scope_sugar".to_string(),
        "kept_later:low_level_memory_order_source_surface".to_string(),
        "kept_later:final_modal_foundation_adoption".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
    ]
}

fn theorem_final_public_contract_reopen_threshold_evidence_refs(sample_id: &str) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:theorem_final_public_contract_reopen_threshold".to_string(),
        "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold".to_string(),
    ]
}

fn theorem_result_object_actual_route_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_result_object_actual_route:{subject_ref}:review_unit_transport_first"),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:notebook_consumer_object_first"
            ),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:consumer_shaped_payload_preview_keep"
            ),
            format!(
                "theorem_result_object_actual_route:{subject_ref}:proof_object_schema_prover_brand_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_result_object_payload_preview_keep_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_result_object_payload_preview_keep:{subject_ref}:notebook_consumer_first"
            ),
            format!(
                "theorem_result_object_payload_preview_keep:{subject_ref}:consumer_shaped_payload_preview_only"
            ),
            format!(
                "theorem_result_object_payload_preview_keep:{subject_ref}:payload_public_contract_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_proof_object_schema_candidate_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_proof_object_schema_candidate:{subject_ref}:result_object_preview_adjacent"
            ),
            format!(
                "theorem_proof_object_schema_candidate:{subject_ref}:refs_only_public_schema_candidate"
            ),
            format!(
                "theorem_proof_object_schema_candidate:{subject_ref}:public_contract_not_adopted"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_prover_brand_candidate_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!("theorem_prover_brand_candidate:{subject_ref}:brand_neutral_preflight_anchor"),
            format!("theorem_prover_brand_candidate:{subject_ref}:adapter_boundary_refs_keep"),
            format!("theorem_prover_brand_candidate:{subject_ref}:concrete_brand_not_adopted"),
        ],
        None => Vec::new(),
    }
}

fn theorem_final_public_contract_reopen_sequence_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "theorem_final_public_contract_reopen:{subject_ref}:result_object_and_payload_first"
            ),
            format!(
                "theorem_final_public_contract_reopen:{subject_ref}:prover_brand_and_proof_schema_second"
            ),
            format!(
                "theorem_final_public_contract_reopen:{subject_ref}:final_public_verifier_contract_third"
            ),
        ],
        None => Vec::new(),
    }
}

fn theorem_final_public_contract_reopen_threshold_default_refs() -> Vec<String> {
    vec![
        "theorem_final_public_contract_reopen_default:result_object_and_payload_first".to_string(),
        "theorem_final_public_contract_reopen_default:prover_brand_and_proof_schema_second"
            .to_string(),
        "theorem_final_public_contract_reopen_default:final_public_verifier_contract_third"
            .to_string(),
    ]
}

fn theorem_final_public_contract_reopen_threshold_compare_floor_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "compare_floor:current_l2.theorem_review_unit_transport_actual_adoption".to_string(),
            "compare_floor:current_l2.theorem_result_object_preview_actualization".to_string(),
            "compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate"
                .to_string(),
            "compare_floor:current_l2.theorem_result_object_actual_adoption".to_string(),
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold".to_string(),
        ]
    } else {
        vec![
            "compare_floor:current_l2.theorem_final_public_contract_reopen_threshold.guard_only"
                .to_string(),
        ]
    }
}

fn theorem_final_public_contract_reopen_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:result_object_and_payload_first".to_string(),
            "guard:prover_brand_and_proof_schema_second".to_string(),
            "guard:final_public_verifier_contract_third".to_string(),
        ]
    } else {
        vec!["guard:theorem_final_public_contract_reopen_threshold_not_reached".to_string()]
    }
}

fn theorem_final_public_contract_reopen_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_theorem_result_object".to_string(),
        "kept_later:consumer_shaped_theorem_payload".to_string(),
        "kept_later:concrete_theorem_prover_brand".to_string(),
        "kept_later:proof_object_public_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn model_check_final_public_contract_reopen_threshold_evidence_refs(
    sample_id: &str,
) -> Vec<String> {
    vec![
        format!("sample:{sample_id}"),
        "helper_preview:model_check_final_public_contract_reopen_threshold".to_string(),
        "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold".to_string(),
    ]
}

fn model_check_checker_artifact_actual_route_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:row_local_property_route_first"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:checker_boundary_contract_anchor"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:consumer_shaped_checker_artifact_candidate_only"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:repo_local_emitted_artifact_refs_first"
            ),
            format!(
                "model_check_checker_artifact_actual_route:{subject_ref}:final_public_checker_artifact_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_checker_artifact_migration_keep_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:verifier_handoff_candidate_adjacent_keep"
            ),
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:tool_brand_candidate_adjacent_keep"
            ),
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:actual_public_checker_migration_candidate_only"
            ),
            format!(
                "model_check_checker_artifact_migration_keep:{subject_ref}:runtime_policy_contract_later"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_verifier_handoff_candidate_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_verifier_handoff_candidate:{subject_ref}:public_checker_preview_adjacent"
            ),
            format!(
                "model_check_verifier_handoff_candidate:{subject_ref}:emitted_handoff_artifact_candidate"
            ),
            format!(
                "model_check_verifier_handoff_candidate:{subject_ref}:runtime_policy_contract_candidate"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_tool_brand_candidate_refs(subject_ref: Option<&str>) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_tool_brand_candidate:{subject_ref}:brand_neutral_request_manifest_keep"
            ),
            format!("model_check_tool_brand_candidate:{subject_ref}:concrete_tool_brand_candidate"),
            format!(
                "model_check_tool_brand_candidate:{subject_ref}:public_checker_artifact_not_adopted"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_final_public_contract_reopen_sequence_refs(
    subject_ref: Option<&str>,
) -> Vec<String> {
    match subject_ref {
        Some(subject_ref) => vec![
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:property_language_and_tool_brand_first"
            ),
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:public_checker_artifact_and_migration_second"
            ),
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:verifier_handoff_and_runtime_policy_contract_third"
            ),
            format!(
                "model_check_final_public_contract_reopen:{subject_ref}:final_public_verifier_contract_fourth"
            ),
        ],
        None => Vec::new(),
    }
}

fn model_check_final_public_contract_reopen_threshold_default_refs() -> Vec<String> {
    vec![
        "model_check_final_public_contract_reopen_default:property_language_and_tool_brand_first"
            .to_string(),
        "model_check_final_public_contract_reopen_default:public_checker_artifact_and_migration_second"
            .to_string(),
        "model_check_final_public_contract_reopen_default:verifier_handoff_and_runtime_policy_contract_third"
            .to_string(),
        "model_check_final_public_contract_reopen_default:final_public_verifier_contract_fourth"
            .to_string(),
    ]
}

fn model_check_final_public_contract_reopen_threshold_compare_floor_refs(
    reached: bool,
) -> Vec<String> {
    if reached {
        vec![
            "compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization"
                .to_string(),
            "compare_floor:current_l2.model_check.property_tool_threshold".to_string(),
            "compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate"
                .to_string(),
            "compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate"
                .to_string(),
            "compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption"
                .to_string(),
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold"
                .to_string(),
        ]
    } else {
        vec![
            "compare_floor:current_l2.model_check.final_public_contract_reopen_threshold.guard_only"
                .to_string(),
        ]
    }
}

fn model_check_final_public_contract_reopen_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:property_language_and_tool_brand_first".to_string(),
            "guard:public_checker_artifact_and_migration_second".to_string(),
            "guard:verifier_handoff_and_runtime_policy_contract_third".to_string(),
            "guard:final_public_verifier_contract_fourth".to_string(),
        ]
    } else {
        vec!["guard:model_check_final_public_contract_reopen_threshold_not_reached".to_string()]
    }
}

fn model_check_final_public_contract_reopen_threshold_kept_later_refs() -> Vec<String> {
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
        "kept_later:final_finite_index_surface".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:actual_checker_payload_family".to_string(),
        "kept_later:checker_supported_kind_summary".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_checker_payload_family_threshold_source_refs() -> Vec<String> {
    vec![
        "fixture_checked_reason_codes".to_string(),
        "detached_static_gate_reason_codes".to_string(),
    ]
}

fn actual_checker_payload_family_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_payload_threshold_only".to_string(),
            "guard:actual_checker_payload_family_docs_first_bridge".to_string(),
            "guard:supported_kind_summary_later".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec!["guard:actual_checker_payload_family_threshold_not_reached".to_string()]
    }
}

fn actual_checker_payload_family_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:checker_payload_row_family".to_string(),
        "kept_later:checker_payload_row_detail".to_string(),
        "kept_later:checker_payload_row_body".to_string(),
        "kept_later:checker_supported_kind_summary".to_string(),
        "kept_later:public_checker_payload_schema".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_checker_payload_row_family_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_row_family_threshold_only".to_string(),
            "guard:checker_payload_row_family_docs_first_bridge".to_string(),
            "guard:checker_payload_row_detail_later".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec!["guard:actual_checker_payload_row_family_threshold_not_reached".to_string()]
    }
}

fn actual_checker_payload_row_family_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:checker_payload_row_detail".to_string(),
        "kept_later:checker_payload_row_body".to_string(),
        "kept_later:checker_supported_kind_summary".to_string(),
        "kept_later:public_checker_payload_schema".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_checker_payload_row_detail_source_ref(_sample_id: &str) -> &'static str {
    "fixture_checked_reason_codes"
}

fn actual_checker_payload_row_detail_reason_kind(sample_id: &str) -> Option<&'static str> {
    current_l2_first_strong_typing_sample_manifest(sample_id).map(|manifest| manifest.case_label)
}

fn actual_checker_payload_row_detail_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_row_detail_threshold_only".to_string(),
            "guard:checker_payload_row_detail_docs_first_bridge".to_string(),
            "guard:checker_payload_row_body_later".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec!["guard:actual_checker_payload_row_detail_threshold_not_reached".to_string()]
    }
}

fn actual_checker_payload_row_detail_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:checker_payload_row_body".to_string(),
        "kept_later:checker_supported_kind_summary".to_string(),
        "kept_later:public_checker_payload_schema".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_checker_payload_row_body_bundle(sample_id: &str) -> Option<BTreeMap<String, String>> {
    let manifest = current_l2_first_strong_typing_sample_manifest(sample_id)?;

    Some(BTreeMap::from([
        (
            "selected_option_ref".to_string(),
            manifest.selected_option_ref.to_string(),
        ),
        (
            "visibility_target_ref".to_string(),
            manifest.visibility_target_ref.to_string(),
        ),
    ]))
}

fn actual_checker_payload_row_body_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_row_body_threshold_only".to_string(),
            "guard:checker_payload_row_body_docs_first_bridge".to_string(),
            "guard:checker_supported_kind_summary_later".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec!["guard:actual_checker_payload_row_body_threshold_not_reached".to_string()]
    }
}

fn actual_checker_payload_row_body_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:checker_supported_kind_summary".to_string(),
        "kept_later:public_checker_payload_schema".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_checker_payload_supported_kind_refs() -> Vec<String> {
    vec![
        "missing_lineage_assertion".to_string(),
        "lineage_assertion_edge_mismatch".to_string(),
        "declared_target_missing".to_string(),
        "declared_target_mismatch".to_string(),
        "capability_strengthens".to_string(),
        "missing_chain_head_option".to_string(),
        "missing_predecessor_option".to_string(),
        "missing_successor_option".to_string(),
    ]
}

fn actual_checker_payload_supported_kind_summary_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_supported_kind_summary_threshold_only".to_string(),
            "guard:checker_supported_kind_summary_docs_first_bridge".to_string(),
            "guard:public_checker_payload_schema_later".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec![
            "guard:actual_checker_payload_supported_kind_summary_threshold_not_reached".to_string(),
        ]
    }
}

fn actual_checker_payload_supported_kind_summary_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:public_checker_payload_schema".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_checker_payload_public_schema_sketch_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_public_checker_payload_schema_threshold_only".to_string(),
            "guard:checker_payload_public_schema_docs_first_bridge".to_string(),
            "guard:public_checker_api_later".to_string(),
            "guard:final_public_checker_payload_later".to_string(),
        ]
    } else {
        vec!["guard:actual_checker_payload_public_schema_sketch_threshold_not_reached".to_string()]
    }
}

fn actual_checker_payload_public_schema_sketch_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:public_checker_api".to_string(),
        "kept_later:final_public_checker_artifact".to_string(),
        "kept_later:final_typed_source_principal".to_string(),
        "kept_later:final_ifc_syntax".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_public_checker_api_sketch_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_public_checker_api_threshold_only".to_string(),
            "guard:public_checker_api_docs_first_bridge".to_string(),
            "guard:public_checker_entry_criteria_later".to_string(),
            "guard:shared_output_contract_later".to_string(),
        ]
    } else {
        vec!["guard:actual_public_checker_api_sketch_threshold_not_reached".to_string()]
    }
}

fn actual_public_checker_api_sketch_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:public_checker_entry_criteria".to_string(),
        "kept_later:public_checker_command_surface".to_string(),
        "kept_later:shared_output_contract".to_string(),
        "kept_later:parser_front_public_checker_boundary".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_public_checker_entry_criteria_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_public_checker_entry_criteria_threshold_only".to_string(),
            "guard:public_checker_command_surface_comparison_next".to_string(),
            "guard:shared_output_contract_later".to_string(),
            "guard:parser_front_public_checker_boundary_later".to_string(),
        ]
    } else {
        vec!["guard:actual_public_checker_entry_criteria_threshold_not_reached".to_string()]
    }
}

fn actual_public_checker_entry_criteria_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:public_checker_command_surface".to_string(),
        "kept_later:shared_output_contract".to_string(),
        "kept_later:parser_front_public_checker_boundary".to_string(),
        "kept_later:verifier_handoff_surface".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_public_checker_command_surface_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_public_checker_command_surface_threshold_only".to_string(),
            "guard:shared_output_contract_comparison_next".to_string(),
            "guard:detached_loop_smoke_wrapper_later".to_string(),
            "guard:generic_shared_public_checker_entry_later".to_string(),
        ]
    } else {
        vec!["guard:actual_public_checker_command_surface_threshold_not_reached".to_string()]
    }
}

fn actual_public_checker_command_surface_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:detached_loop_smoke_wrapper".to_string(),
        "kept_later:generic_shared_public_checker_entry".to_string(),
        "kept_later:shared_output_contract".to_string(),
        "kept_later:parser_front_public_checker_boundary".to_string(),
        "kept_later:verifier_handoff_surface".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_shared_output_contract_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_shared_output_contract_threshold_only".to_string(),
            "guard:public_checker_boundary_comparison_next".to_string(),
            "guard:detached_loop_wrapper_paths_later".to_string(),
            "guard:row_snippet_textual_rendering_later".to_string(),
        ]
    } else {
        vec!["guard:actual_shared_output_contract_threshold_not_reached".to_string()]
    }
}

fn actual_shared_output_contract_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:detached_loop_wrapper_paths".to_string(),
        "kept_later:fixture_and_actual_rows_textual_rendering".to_string(),
        "kept_later:generic_shared_public_checker_entry".to_string(),
        "kept_later:parser_front_public_checker_boundary".to_string(),
        "kept_later:verifier_handoff_surface".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_public_checker_boundary_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_public_checker_boundary_threshold_only".to_string(),
            "guard:verifier_handoff_surface_comparison_next".to_string(),
            "guard:final_parser_grammar_later".to_string(),
            "guard:generic_shared_public_checker_entry_later".to_string(),
        ]
    } else {
        vec!["guard:actual_public_checker_boundary_threshold_not_reached".to_string()]
    }
}

fn actual_public_checker_boundary_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:query_token_and_checker_subject_public_naming".to_string(),
        "kept_later:generic_shared_public_checker_entry".to_string(),
        "kept_later:detached_loop_wrapper_path_line".to_string(),
        "kept_later:verifier_handoff_surface".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_verifier_handoff_surface_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:checker_adjacent_verifier_handoff_surface_threshold_only".to_string(),
            "guard:minimal_parser_subset_freeze_comparison_next".to_string(),
            "guard:actual_emitted_verifier_handoff_artifact_later".to_string(),
            "guard:dedicated_handoff_artifact_family_later".to_string(),
        ]
    } else {
        vec!["guard:actual_verifier_handoff_surface_threshold_not_reached".to_string()]
    }
}

fn actual_verifier_handoff_surface_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:subject_rows".to_string(),
        "kept_later:theorem_protocol_runtime_dedicated_handoff_artifact_family".to_string(),
        "kept_later:actual_emitted_verifier_handoff_artifact".to_string(),
        "kept_later:tool_specific_schema_and_actual_emitter_policy".to_string(),
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:query_token_and_shared_generic_entry".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_minimal_parser_subset_freeze_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:parser_front_minimal_parser_subset_freeze_threshold_only".to_string(),
            "guard:parser_to_checker_reconnect_freeze_comparison_next".to_string(),
            "guard:stage3_request_admit_predicate_retained_later".to_string(),
            "guard:final_parser_grammar_later".to_string(),
        ]
    } else {
        vec!["guard:actual_minimal_parser_subset_freeze_threshold_not_reached".to_string()]
    }
}

fn actual_minimal_parser_subset_freeze_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:stage3_admit_slot_branch".to_string(),
        "kept_later:stage3_request_clause_branch".to_string(),
        "kept_later:stage3_predicate_fragment_branch".to_string(),
        "kept_later:public_parser_api".to_string(),
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:parser_to_checker_reconnect_freeze".to_string(),
        "kept_later:final_public_parser_checker_api".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_parser_to_checker_reconnect_freeze_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:parser_checker_reconnect_freeze_threshold_only".to_string(),
            "guard:phase1_semantics_closeout_comparison_next".to_string(),
            "guard:stage3_request_reconnect_later".to_string(),
            "guard:runtime_proof_boundary_actualization_later".to_string(),
        ]
    } else {
        vec!["guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached".to_string()]
    }
}

fn actual_parser_to_checker_reconnect_freeze_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:stage3_request_predicate_reconnect_line".to_string(),
        "kept_later:stage1_direct_target_mismatch_redesign_line".to_string(),
        "kept_later:runtime_contrast_e21_e22_line".to_string(),
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_public_parser_checker_api".to_string(),
        "kept_later:actual_external_verifier_schema".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_phase1_semantics_closeout_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:phase1_semantics_closeout_threshold_only".to_string(),
            "guard:phase2_parser_free_poc_closeout_comparison_next".to_string(),
            "guard:final_parser_grammar_later".to_string(),
            "guard:final_type_system_and_schema_later".to_string(),
        ]
    } else {
        vec!["guard:actual_phase1_semantics_closeout_threshold_not_reached".to_string()]
    }
}

fn actual_phase1_semantics_closeout_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_parser_grammar".to_string(),
        "kept_later:final_reserved_keyword_and_punctuation".to_string(),
        "kept_later:final_type_system".to_string(),
        "kept_later:actual_external_verifier_schema".to_string(),
        "kept_later:actual_emitted_verifier_artifact".to_string(),
        "kept_later:final_public_verifier_contract".to_string(),
    ]
}

fn actual_phase2_parser_free_poc_closeout_threshold_guard_refs(reached: bool) -> Vec<String> {
    if reached {
        vec![
            "guard:phase2_parser_free_poc_closeout_threshold_only".to_string(),
            "guard:phase4_shared_space_self_driven_closeout_comparison_next".to_string(),
            "guard:reference_update_bless_workflow_later".to_string(),
            "guard:retention_and_public_exporter_policy_later".to_string(),
        ]
    } else {
        vec!["guard:actual_phase2_parser_free_poc_closeout_threshold_not_reached".to_string()]
    }
}

fn actual_phase2_parser_free_poc_closeout_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:reference_update_bless_workflow".to_string(),
        "kept_later:final_retention_path_policy".to_string(),
        "kept_later:public_exporter_api".to_string(),
        "kept_later:production_host_interface".to_string(),
    ]
}

fn actual_phase4_shared_space_self_driven_closeout_current_package_refs() -> Vec<String> {
    vec![
        "authoritative_room_baseline_ref".to_string(),
        "working_subset_catalog_ref".to_string(),
        "minimal_authority_witness_core_ref".to_string(),
        "authoritative_delegated_provider_cut_ref".to_string(),
        "control_plane_threshold_ref".to_string(),
    ]
}

fn actual_phase4_shared_space_self_driven_closeout_user_spec_required_catalog_refs() -> Vec<String>
{
    vec![
        "final_activation_overlay_catalog".to_string(),
        "final_authority_auth_identity_admission_catalog".to_string(),
        "final_consistency_fairness_catalog".to_string(),
    ]
}

fn actual_phase4_shared_space_self_driven_closeout_retained_later_refs() -> Vec<String> {
    vec![
        "append_friendly_optional_provider_attestation".to_string(),
        "control_plane_separated_carrier_actualization".to_string(),
        "distributed_fairness_protocol".to_string(),
        "final_operational_realization".to_string(),
    ]
}

fn actual_phase4_shared_space_self_driven_closeout_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    if reached {
        vec![
            "guard:phase4_shared_space_self_driven_closeout_threshold_only".to_string(),
            "guard:phase5_proof_protocol_runtime_policy_handoff_closeout_comparison_next"
                .to_string(),
            "guard:user_spec_required_final_catalog_later".to_string(),
            "guard:distributed_fairness_protocol_later".to_string(),
        ]
    } else {
        vec![
            "guard:actual_phase4_shared_space_self_driven_closeout_threshold_not_reached"
                .to_string(),
        ]
    }
}

fn actual_phase4_shared_space_self_driven_closeout_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "kept_later:final_public_witness_provider_artifact_contract".to_string(),
        "kept_later:exhaustive_shared_space_catalog".to_string(),
        "kept_later:control_plane_separated_carrier_actualization".to_string(),
        "kept_later:distributed_fairness_protocol".to_string(),
        "kept_later:final_operational_realization".to_string(),
    ]
}

fn actual_phase5_proof_protocol_runtime_policy_handoff_closeout_retained_later_refs() -> Vec<String>
{
    vec![
        "actual_subject_row_materialization".to_string(),
        "boundary_specific_handoff_artifact_family".to_string(),
        "actual_emitted_verifier_artifact".to_string(),
        "concrete_tool_binding".to_string(),
        "public_checker_migration".to_string(),
        "low_level_memory_order_family".to_string(),
    ]
}

fn actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    if reached {
        vec![
            "guard:phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_only"
                .to_string(),
            "guard:phase6_actual_parser_ast_carrier_first_tranche_comparison_next".to_string(),
            "guard:actual_subject_row_and_artifact_family_later".to_string(),
            "guard:tool_binding_public_checker_migration_and_low_level_memory_order_later"
                .to_string(),
        ]
    } else {
        vec![
            "guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_not_reached"
                .to_string(),
        ]
    }
}

fn actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_kept_later_refs()
-> Vec<String> {
    vec![
        "kept_later:actual_subject_row_materialization".to_string(),
        "kept_later:boundary_specific_handoff_artifact_family".to_string(),
        "kept_later:actual_emitted_verifier_artifact".to_string(),
        "kept_later:concrete_tool_binding".to_string(),
        "kept_later:public_checker_migration".to_string(),
        "kept_later:low_level_memory_order_family".to_string(),
    ]
}

fn actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    let mut refs = vec![
        "guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_required"
            .to_string(),
    ];
    if reached {
        refs.push(
            "guard:phase6_actual_checker_runtime_skeleton_first_tranche_comparison_next"
                .to_string(),
        );
    } else {
        refs.push(
            "guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_not_reached"
                .to_string(),
        );
    }
    refs
}

fn actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_kept_later_refs() -> Vec<String>
{
    current_l2_first_tranche_manifest()
        .retained_later_refs
        .iter()
        .map(|item| (*item).to_string())
        .collect()
}

fn actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    let mut refs = vec![
        "guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_required"
            .to_string(),
    ];
    if reached {
        refs.push(
            "guard:phase6_compile_ready_verification_and_formal_hook_comparison_next".to_string(),
        );
    } else {
        refs.push(
            "guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_not_reached"
                .to_string(),
        );
    }
    refs
}

fn actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_kept_later_refs()
-> Vec<String> {
    current_l2_checker_runtime_first_tranche_manifest()
        .retained_later_refs
        .iter()
        .map(|item| (*item).to_string())
        .collect()
}

fn actual_phase6_compile_ready_verification_and_formal_hook_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    let mut refs = vec![
        "guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_required"
            .to_string(),
    ];
    if reached {
        refs.push("guard:phase6_next_reopen_sequencing_comparison_next".to_string());
    } else {
        refs.push(
            "guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_not_reached"
                .to_string(),
        );
    }
    refs
}

fn actual_phase6_compile_ready_verification_and_formal_hook_threshold_kept_later_refs()
-> Vec<String> {
    current_l2_compile_ready_verification_and_formal_hook_manifest()
        .retained_later_refs
        .iter()
        .map(|item| (*item).to_string())
        .collect()
}

fn actual_phase6_next_reopen_sequencing_threshold_guard_refs(reached: bool) -> Vec<String> {
    let mut refs = vec![
        "guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_required"
            .to_string(),
    ];
    if reached {
        refs.push(
            "guard:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison_next"
                .to_string(),
        );
    } else {
        refs.push("guard:actual_phase6_next_reopen_sequencing_threshold_not_reached".to_string());
    }
    refs
}

fn actual_phase6_next_reopen_sequencing_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "request_clause_suite_bulk_widen".to_string(),
        "perform_head_final_public_api".to_string(),
        "concrete_theorem_tool_binding".to_string(),
        "concrete_model_check_tool_binding".to_string(),
        "final_public_surface".to_string(),
    ]
}

fn actual_phase6_reserve_formal_tool_binding_inventory_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    let mut refs =
        vec!["guard:actual_phase6_next_reopen_sequencing_threshold_required".to_string()];
    if reached {
        refs.push(
            "guard:phase6_parser_side_follow_up_package_sequencing_comparison_next".to_string(),
        );
    } else {
        refs.push(
            "guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_not_reached"
                .to_string(),
        );
    }
    refs
}

fn actual_phase6_reserve_formal_tool_binding_inventory_threshold_kept_later_refs() -> Vec<String> {
    vec![
        "concrete_theorem_tool_name".to_string(),
        "concrete_model_check_tool_name".to_string(),
        "actual_ci_artifact_retention_policy".to_string(),
        "parser_side_followup_package_selection".to_string(),
        "final_public_parser_checker_runtime_surface".to_string(),
    ]
}

fn actual_phase6_parser_side_followup_package_sequencing_threshold_guard_refs(
    reached: bool,
) -> Vec<String> {
    let mut refs = vec![
        "guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_required".to_string(),
    ];
    if reached {
        refs.push(
            "guard:phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison_next"
                .to_string(),
        );
    } else {
        refs.push(
            "guard:actual_phase6_parser_side_followup_package_sequencing_threshold_not_reached"
                .to_string(),
        );
    }
    refs
}

fn actual_phase6_parser_side_followup_package_sequencing_threshold_kept_later_refs() -> Vec<String>
{
    vec![
        "request_clause_suite_publicization".to_string(),
        "perform_head_final_public_parser_api".to_string(),
        "span_rich_diagnostics".to_string(),
        "source_sample_corpus_scope".to_string(),
        "final_public_parser_checker_runtime_surface".to_string(),
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
    render_string_list(output, "bridge_floor_refs", &preview.bridge_floor_refs, 1);
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
    render_string_list(output, "bridge_floor_refs", &preview.bridge_floor_refs, 1);
    render_string_list(output, "compare_floor_refs", &preview.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &preview.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &preview.kept_later_refs, 1);
    if let Some(guard_reason) = &preview.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_theorem_final_public_contract_reopen_threshold(
    output: &mut String,
    threshold: &CurrentL2OperationalCliTheoremFinalPublicContractReopenThresholdSummary,
) {
    writeln!(output, "  status: {}", threshold.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", threshold.threshold_kind).expect("write to string");
    if let Some(subject_kind) = threshold.subject_kind {
        writeln!(output, "  subject_kind: {subject_kind}").expect("write to string");
    } else {
        writeln!(output, "  subject_kind: none").expect("write to string");
    }
    if let Some(subject_ref) = &threshold.subject_ref {
        writeln!(output, "  subject_ref: {subject_ref}").expect("write to string");
    } else {
        writeln!(output, "  subject_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "result_object_route_refs",
        &threshold.result_object_route_refs,
        1,
    );
    render_string_list(
        output,
        "payload_preview_keep_refs",
        &threshold.payload_preview_keep_refs,
        1,
    );
    render_string_list(
        output,
        "proof_object_schema_candidate_refs",
        &threshold.proof_object_schema_candidate_refs,
        1,
    );
    render_string_list(
        output,
        "prover_brand_candidate_refs",
        &threshold.prover_brand_candidate_refs,
        1,
    );
    render_string_list(
        output,
        "final_public_contract_reopen_sequence_refs",
        &threshold.final_public_contract_reopen_sequence_refs,
        1,
    );
    render_string_list(
        output,
        "threshold_default_refs",
        &threshold.threshold_default_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &threshold.evidence_refs, 1);
    render_string_list(output, "bridge_floor_refs", &threshold.bridge_floor_refs, 1);
    render_string_list(
        output,
        "compare_floor_refs",
        &threshold.compare_floor_refs,
        1,
    );
    render_string_list(output, "guard_refs", &threshold.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &threshold.kept_later_refs, 1);
    if let Some(guard_reason) = &threshold.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_order_handoff_witness_provider_public_seam_compression(
    output: &mut String,
    compression: &CurrentL2OperationalCliOrderHandoffWitnessProviderPublicSeamCompressionSummary,
) {
    writeln!(output, "  status: {}", compression.status).expect("write to string");
    writeln!(
        output,
        "  compression_kind: {}",
        compression.compression_kind
    )
    .expect("write to string");
    render_string_list(
        output,
        "profile_axis_refs",
        &compression.profile_axis_refs,
        1,
    );
    render_string_list(
        output,
        "repo_local_emitted_artifact_refs",
        &compression.repo_local_emitted_artifact_refs,
        1,
    );
    render_string_list(
        output,
        "source_wording_route_refs",
        &compression.source_wording_route_refs,
        1,
    );
    render_string_list(
        output,
        "emitted_artifact_candidate_keep_refs",
        &compression.emitted_artifact_candidate_keep_refs,
        1,
    );
    render_string_list(
        output,
        "serial_scope_lines",
        &compression.serial_scope_lines,
        1,
    );
    render_string_list(
        output,
        "witness_schema_route_refs",
        &compression.witness_schema_route_refs,
        1,
    );
    render_string_list(
        output,
        "provider_receipt_route_refs",
        &compression.provider_receipt_route_refs,
        1,
    );
    render_string_list(
        output,
        "combined_public_contract_keep_refs",
        &compression.combined_public_contract_keep_refs,
        1,
    );
    render_string_list(
        output,
        "trace_alignment_pair_refs",
        &compression.trace_alignment_pair_refs,
        1,
    );
    render_string_list(
        output,
        "public_seam_residual_refs",
        &compression.public_seam_residual_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &compression.evidence_refs, 1);
    render_string_list(
        output,
        "compare_floor_refs",
        &compression.compare_floor_refs,
        1,
    );
    render_string_list(output, "guard_refs", &compression.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &compression.kept_later_refs, 1);
    if let Some(guard_reason) = &compression.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_order_handoff_source_surface_artifact_actual_adoption(
    output: &mut String,
    summary: &CurrentL2OperationalCliOrderHandoffSourceSurfaceArtifactActualAdoptionSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  adoption_kind: {}", summary.adoption_kind).expect("write to string");
    render_string_list(output, "profile_axis_refs", &summary.profile_axis_refs, 1);
    render_string_list(
        output,
        "principal_surface_lines",
        &summary.principal_surface_lines,
        1,
    );
    render_string_list(
        output,
        "secondary_surface_lines",
        &summary.secondary_surface_lines,
        1,
    );
    render_string_list(
        output,
        "repo_local_emitted_artifact_refs",
        &summary.repo_local_emitted_artifact_refs,
        1,
    );
    render_string_list(
        output,
        "source_wording_route_refs",
        &summary.source_wording_route_refs,
        1,
    );
    render_string_list(
        output,
        "emitted_artifact_candidate_keep_refs",
        &summary.emitted_artifact_candidate_keep_refs,
        1,
    );
    render_string_list(
        output,
        "negative_static_stop_refs",
        &summary.negative_static_stop_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_authoritative_room_first_scenario_actual_adoption(
    output: &mut String,
    summary: &CurrentL2OperationalCliAuthoritativeRoomFirstScenarioActualAdoptionSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  adoption_kind: {}", summary.adoption_kind).expect("write to string");
    render_string_list(output, "profile_axis_refs", &summary.profile_axis_refs, 1);
    render_string_list(output, "relation_refs", &summary.relation_refs, 1);
    render_string_list(
        output,
        "authority_handoff_refs",
        &summary.authority_handoff_refs,
        1,
    );
    render_string_list(
        output,
        "runtime_evidence_refs",
        &summary.runtime_evidence_refs,
        1,
    );
    render_string_list(
        output,
        "repo_local_emitted_artifact_refs",
        &summary.repo_local_emitted_artifact_refs,
        1,
    );
    render_string_list(output, "reserve_route_refs", &summary.reserve_route_refs, 1);
    render_string_list(
        output,
        "negative_static_stop_refs",
        &summary.negative_static_stop_refs,
        1,
    );
    render_string_list(output, "contrast_refs", &summary.contrast_refs, 1);
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_authoritative_room_reserve_strengthening_lane(
    output: &mut String,
    summary: &CurrentL2OperationalCliAuthoritativeRoomReserveStrengtheningLaneSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  lane_kind: {}", summary.lane_kind).expect("write to string");
    writeln!(
        output,
        "  witness_strengthening_status: {}",
        summary.witness_strengthening_status
    )
    .expect("write to string");
    writeln!(
        output,
        "  delegated_rng_service_status: {}",
        summary.delegated_rng_service_status
    )
    .expect("write to string");
    writeln!(
        output,
        "  model_check_second_line_status: {}",
        summary.model_check_second_line_status
    )
    .expect("write to string");
    render_string_list(
        output,
        "witness_strengthening_refs",
        &summary.witness_strengthening_refs,
        1,
    );
    render_string_list(
        output,
        "delegated_rng_service_refs",
        &summary.delegated_rng_service_refs,
        1,
    );
    render_string_list(
        output,
        "model_check_second_line_refs",
        &summary.model_check_second_line_refs,
        1,
    );
    render_string_list(
        output,
        "first_line_boundary_refs",
        &summary.first_line_boundary_refs,
        1,
    );
    render_string_list(
        output,
        "reserve_boundary_refs",
        &summary.reserve_boundary_refs,
        1,
    );
    render_string_list(
        output,
        "repo_local_emitted_artifact_refs",
        &summary.repo_local_emitted_artifact_refs,
        1,
    );
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_model_check_final_public_contract_reopen_threshold(
    output: &mut String,
    threshold: &CurrentL2OperationalCliModelCheckFinalPublicContractReopenThresholdSummary,
) {
    writeln!(output, "  status: {}", threshold.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", threshold.threshold_kind).expect("write to string");
    if let Some(subject_kind) = threshold.subject_kind {
        writeln!(output, "  subject_kind: {subject_kind}").expect("write to string");
    } else {
        writeln!(output, "  subject_kind: none").expect("write to string");
    }
    if let Some(subject_ref) = &threshold.subject_ref {
        writeln!(output, "  subject_ref: {subject_ref}").expect("write to string");
    } else {
        writeln!(output, "  subject_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "checker_artifact_route_refs",
        &threshold.checker_artifact_route_refs,
        1,
    );
    render_string_list(
        output,
        "migration_candidate_keep_refs",
        &threshold.migration_candidate_keep_refs,
        1,
    );
    render_string_list(
        output,
        "verifier_handoff_candidate_refs",
        &threshold.verifier_handoff_candidate_refs,
        1,
    );
    render_string_list(
        output,
        "tool_brand_candidate_refs",
        &threshold.tool_brand_candidate_refs,
        1,
    );
    render_string_list(
        output,
        "final_public_contract_reopen_sequence_refs",
        &threshold.final_public_contract_reopen_sequence_refs,
        1,
    );
    render_string_list(
        output,
        "threshold_default_refs",
        &threshold.threshold_default_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &threshold.evidence_refs, 1);
    render_string_list(output, "bridge_floor_refs", &threshold.bridge_floor_refs, 1);
    render_string_list(
        output,
        "compare_floor_refs",
        &threshold.compare_floor_refs,
        1,
    );
    render_string_list(output, "guard_refs", &threshold.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &threshold.kept_later_refs, 1);
    if let Some(guard_reason) = &threshold.guard_reason {
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

fn render_actual_checker_payload_family_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualCheckerPayloadFamilyThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(cluster_kind) = summary.cluster_kind {
        writeln!(output, "  cluster_kind: {cluster_kind}").expect("write to string");
    } else {
        writeln!(output, "  cluster_kind: none").expect("write to string");
    }
    if let Some(case_label) = summary.case_label {
        writeln!(output, "  case_label: {case_label}").expect("write to string");
    } else {
        writeln!(output, "  case_label: none").expect("write to string");
    }
    render_string_list(output, "family_refs", &summary.family_refs, 1);
    if let Some(coverage_state) = summary.coverage_state {
        writeln!(output, "  coverage_state: {coverage_state}").expect("write to string");
    } else {
        writeln!(output, "  coverage_state: none").expect("write to string");
    }
    if let Some(payload_family_kind) = summary.payload_family_kind {
        writeln!(output, "  payload_family_kind: {payload_family_kind}").expect("write to string");
    } else {
        writeln!(output, "  payload_family_kind: none").expect("write to string");
    }
    render_string_list(output, "source_refs", &summary.source_refs, 1);
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_checker_payload_row_family_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualCheckerPayloadRowFamilyThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(cluster_kind) = summary.cluster_kind {
        writeln!(output, "  cluster_kind: {cluster_kind}").expect("write to string");
    } else {
        writeln!(output, "  cluster_kind: none").expect("write to string");
    }
    if let Some(case_label) = summary.case_label {
        writeln!(output, "  case_label: {case_label}").expect("write to string");
    } else {
        writeln!(output, "  case_label: none").expect("write to string");
    }
    render_string_list(output, "family_refs", &summary.family_refs, 1);
    if let Some(coverage_state) = summary.coverage_state {
        writeln!(output, "  coverage_state: {coverage_state}").expect("write to string");
    } else {
        writeln!(output, "  coverage_state: none").expect("write to string");
    }
    if let Some(payload_family_ref) = summary.payload_family_ref {
        writeln!(output, "  payload_family_ref: {payload_family_ref}").expect("write to string");
    } else {
        writeln!(output, "  payload_family_ref: none").expect("write to string");
    }
    if let Some(row_family_kind) = summary.row_family_kind {
        writeln!(output, "  row_family_kind: {row_family_kind}").expect("write to string");
    } else {
        writeln!(output, "  row_family_kind: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_checker_payload_row_detail_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualCheckerPayloadRowDetailThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(cluster_kind) = summary.cluster_kind {
        writeln!(output, "  cluster_kind: {cluster_kind}").expect("write to string");
    } else {
        writeln!(output, "  cluster_kind: none").expect("write to string");
    }
    if let Some(case_label) = summary.case_label {
        writeln!(output, "  case_label: {case_label}").expect("write to string");
    } else {
        writeln!(output, "  case_label: none").expect("write to string");
    }
    render_string_list(output, "family_refs", &summary.family_refs, 1);
    if let Some(coverage_state) = summary.coverage_state {
        writeln!(output, "  coverage_state: {coverage_state}").expect("write to string");
    } else {
        writeln!(output, "  coverage_state: none").expect("write to string");
    }
    if let Some(payload_row_family_ref) = summary.payload_row_family_ref {
        writeln!(output, "  payload_row_family_ref: {payload_row_family_ref}")
            .expect("write to string");
    } else {
        writeln!(output, "  payload_row_family_ref: none").expect("write to string");
    }
    if let Some(row_source_ref) = summary.row_source_ref {
        writeln!(output, "  row_source_ref: {row_source_ref}").expect("write to string");
    } else {
        writeln!(output, "  row_source_ref: none").expect("write to string");
    }
    if let Some(row_reason_kind) = summary.row_reason_kind {
        writeln!(output, "  row_reason_kind: {row_reason_kind}").expect("write to string");
    } else {
        writeln!(output, "  row_reason_kind: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_checker_payload_row_body_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualCheckerPayloadRowBodyThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(cluster_kind) = summary.cluster_kind {
        writeln!(output, "  cluster_kind: {cluster_kind}").expect("write to string");
    } else {
        writeln!(output, "  cluster_kind: none").expect("write to string");
    }
    if let Some(case_label) = summary.case_label {
        writeln!(output, "  case_label: {case_label}").expect("write to string");
    } else {
        writeln!(output, "  case_label: none").expect("write to string");
    }
    render_string_list(output, "family_refs", &summary.family_refs, 1);
    if let Some(coverage_state) = summary.coverage_state {
        writeln!(output, "  coverage_state: {coverage_state}").expect("write to string");
    } else {
        writeln!(output, "  coverage_state: none").expect("write to string");
    }
    if let Some(payload_row_family_ref) = summary.payload_row_family_ref {
        writeln!(output, "  payload_row_family_ref: {payload_row_family_ref}")
            .expect("write to string");
    } else {
        writeln!(output, "  payload_row_family_ref: none").expect("write to string");
    }
    if let Some(row_source_ref) = summary.row_source_ref {
        writeln!(output, "  row_source_ref: {row_source_ref}").expect("write to string");
    } else {
        writeln!(output, "  row_source_ref: none").expect("write to string");
    }
    if let Some(row_reason_kind) = summary.row_reason_kind {
        writeln!(output, "  row_reason_kind: {row_reason_kind}").expect("write to string");
    } else {
        writeln!(output, "  row_reason_kind: none").expect("write to string");
    }
    if let Some(row_body) = &summary.row_body {
        render_string_map(output, "row_body", row_body, 1);
    } else {
        writeln!(output, "  row_body: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_checker_payload_supported_kind_summary_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualCheckerPayloadSupportedKindSummaryThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(payload_row_family_ref) = summary.payload_row_family_ref {
        writeln!(output, "  payload_row_family_ref: {payload_row_family_ref}")
            .expect("write to string");
    } else {
        writeln!(output, "  payload_row_family_ref: none").expect("write to string");
    }
    if let Some(supported_kind_scope) = summary.supported_kind_scope {
        writeln!(output, "  supported_kind_scope: {supported_kind_scope}")
            .expect("write to string");
    } else {
        writeln!(output, "  supported_kind_scope: none").expect("write to string");
    }
    render_string_list(
        output,
        "supported_kind_refs",
        &summary.supported_kind_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_checker_payload_public_schema_sketch_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualCheckerPayloadPublicSchemaSketchThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(actual_checker_payload_family_ref) = summary.actual_checker_payload_family_ref {
        writeln!(
            output,
            "  actual_checker_payload_family_ref: {actual_checker_payload_family_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  actual_checker_payload_family_ref: none").expect("write to string");
    }
    if let Some(checker_payload_row_family_ref) = summary.checker_payload_row_family_ref {
        writeln!(
            output,
            "  checker_payload_row_family_ref: {checker_payload_row_family_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  checker_payload_row_family_ref: none").expect("write to string");
    }
    if let Some(checker_payload_row_detail_ref) = summary.checker_payload_row_detail_ref {
        writeln!(
            output,
            "  checker_payload_row_detail_ref: {checker_payload_row_detail_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  checker_payload_row_detail_ref: none").expect("write to string");
    }
    if let Some(checker_payload_row_body_ref) = summary.checker_payload_row_body_ref {
        writeln!(
            output,
            "  checker_payload_row_body_ref: {checker_payload_row_body_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  checker_payload_row_body_ref: none").expect("write to string");
    }
    if let Some(checker_payload_supported_kind_summary_ref) =
        summary.checker_payload_supported_kind_summary_ref
    {
        writeln!(
            output,
            "  checker_payload_supported_kind_summary_ref: {checker_payload_supported_kind_summary_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  checker_payload_supported_kind_summary_ref: none")
            .expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_public_checker_api_sketch_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPublicCheckerApiSketchThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(checker_subject) = summary.checker_subject {
        writeln!(output, "  checker_subject: {checker_subject}").expect("write to string");
    } else {
        writeln!(output, "  checker_subject: none").expect("write to string");
    }
    if let Some(public_checker_payload_schema_ref) = summary.public_checker_payload_schema_ref {
        writeln!(
            output,
            "  public_checker_payload_schema_ref: {public_checker_payload_schema_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  public_checker_payload_schema_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_public_checker_entry_criteria_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPublicCheckerEntryCriteriaThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(public_checker_api_ref) = summary.public_checker_api_ref {
        writeln!(output, "  public_checker_api_ref: {public_checker_api_ref}")
            .expect("write to string");
    } else {
        writeln!(output, "  public_checker_api_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "entry_criteria_refs",
        &summary.entry_criteria_refs,
        1,
    );
    if let Some(family_facade_support_ref) = summary.family_facade_support_ref {
        writeln!(
            output,
            "  family_facade_support_ref: {family_facade_support_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  family_facade_support_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "family_facade_script_refs",
        &summary.family_facade_script_refs,
        1,
    );
    render_string_list(output, "smoke_command_refs", &summary.smoke_command_refs, 1);
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_boundary_refs",
        &summary.deferred_boundary_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_public_checker_command_surface_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPublicCheckerCommandSurfaceThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(command_surface_kind) = summary.command_surface_kind {
        writeln!(output, "  command_surface_kind: {command_surface_kind}")
            .expect("write to string");
    } else {
        writeln!(output, "  command_surface_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "family_facade_command_refs",
        &summary.family_facade_command_refs,
        1,
    );
    if let Some(public_checker_api_ref) = summary.public_checker_api_ref {
        writeln!(output, "  public_checker_api_ref: {public_checker_api_ref}")
            .expect("write to string");
    } else {
        writeln!(output, "  public_checker_api_ref: none").expect("write to string");
    }
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_surface_refs",
        &summary.deferred_surface_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_shared_output_contract_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualSharedOutputContractThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(output_contract_kind) = summary.output_contract_kind {
        writeln!(output, "  output_contract_kind: {output_contract_kind}")
            .expect("write to string");
    } else {
        writeln!(output, "  output_contract_kind: none").expect("write to string");
    }
    if let Some(checker_cluster_name) = summary.checker_cluster_name {
        writeln!(output, "  checker_cluster_name: {checker_cluster_name}")
            .expect("write to string");
    } else {
        writeln!(output, "  checker_cluster_name: none").expect("write to string");
    }
    if let Some(checker_status) = summary.checker_status {
        writeln!(output, "  checker_status: {checker_status}").expect("write to string");
    } else {
        writeln!(output, "  checker_status: none").expect("write to string");
    }
    if let Some(public_checker_payload_schema_ref) = summary.public_checker_payload_schema_ref {
        writeln!(
            output,
            "  public_checker_payload_schema_ref: {public_checker_payload_schema_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  public_checker_payload_schema_ref: none").expect("write to string");
    }
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_surface_refs",
        &summary.deferred_surface_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_public_checker_boundary_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPublicCheckerBoundaryThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(boundary_kind) = summary.boundary_kind {
        writeln!(output, "  boundary_kind: {boundary_kind}").expect("write to string");
    } else {
        writeln!(output, "  boundary_kind: none").expect("write to string");
    }
    if let Some(public_checker_command_surface_ref) = summary.public_checker_command_surface_ref {
        writeln!(
            output,
            "  public_checker_command_surface_ref: {public_checker_command_surface_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  public_checker_command_surface_ref: none").expect("write to string");
    }
    if let Some(shared_output_contract_ref) = summary.shared_output_contract_ref {
        writeln!(
            output,
            "  shared_output_contract_ref: {shared_output_contract_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  shared_output_contract_ref: none").expect("write to string");
    }
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_surface_refs",
        &summary.deferred_surface_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_verifier_handoff_surface_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualVerifierHandoffSurfaceThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(handoff_surface_kind) = summary.handoff_surface_kind {
        writeln!(output, "  handoff_surface_kind: {handoff_surface_kind}")
            .expect("write to string");
    } else {
        writeln!(output, "  handoff_surface_kind: none").expect("write to string");
    }
    if let Some(public_checker_boundary_ref) = summary.public_checker_boundary_ref {
        writeln!(
            output,
            "  public_checker_boundary_ref: {public_checker_boundary_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  public_checker_boundary_ref: none").expect("write to string");
    }
    if let Some(proof_obligation_matrix_ref) = summary.proof_obligation_matrix_ref {
        writeln!(
            output,
            "  proof_obligation_matrix_ref: {proof_obligation_matrix_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  proof_obligation_matrix_ref: none").expect("write to string");
    }
    if let Some(handoff_artifact_mode) = summary.handoff_artifact_mode {
        writeln!(output, "  handoff_artifact_mode: {handoff_artifact_mode}")
            .expect("write to string");
    } else {
        writeln!(output, "  handoff_artifact_mode: none").expect("write to string");
    }
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_surface_refs",
        &summary.deferred_surface_refs,
        1,
    );
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_minimal_parser_subset_freeze_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualMinimalParserSubsetFreezeThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(freeze_kind) = summary.freeze_kind {
        writeln!(output, "  freeze_kind: {freeze_kind}").expect("write to string");
    } else {
        writeln!(output, "  freeze_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "accepted_cluster_refs",
        &summary.accepted_cluster_refs,
        1,
    );
    render_string_list(
        output,
        "reject_cluster_refs",
        &summary.reject_cluster_refs,
        1,
    );
    render_string_list(
        output,
        "retention_floor_refs",
        &summary.retention_floor_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_parser_to_checker_reconnect_freeze_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualParserToCheckerReconnectFreezeThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(reconnect_kind) = summary.reconnect_kind {
        writeln!(output, "  reconnect_kind: {reconnect_kind}").expect("write to string");
    } else {
        writeln!(output, "  reconnect_kind: none").expect("write to string");
    }
    if let Some(parser_subset_freeze_ref) = summary.parser_subset_freeze_ref {
        writeln!(
            output,
            "  parser_subset_freeze_ref: {parser_subset_freeze_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  parser_subset_freeze_ref: none").expect("write to string");
    }
    render_string_list(output, "checker_floor_refs", &summary.checker_floor_refs, 1);
    render_string_list(
        output,
        "retained_helper_refs",
        &summary.retained_helper_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase1_semantics_closeout_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase1SemanticsCloseoutThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(closeout_kind) = summary.closeout_kind {
        writeln!(output, "  closeout_kind: {closeout_kind}").expect("write to string");
    } else {
        writeln!(output, "  closeout_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "core_semantics_refs",
        &summary.core_semantics_refs,
        1,
    );
    render_string_list(
        output,
        "invariant_bridge_refs",
        &summary.invariant_bridge_refs,
        1,
    );
    render_string_list(
        output,
        "notation_status_refs",
        &summary.notation_status_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase2_parser_free_poc_closeout_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase2ParserFreePocCloseoutThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(closeout_kind) = summary.closeout_kind {
        writeln!(output, "  closeout_kind: {closeout_kind}").expect("write to string");
    } else {
        writeln!(output, "  closeout_kind: none").expect("write to string");
    }
    render_string_list(output, "compile_gate_refs", &summary.compile_gate_refs, 1);
    render_string_list(
        output,
        "helper_boundary_refs",
        &summary.helper_boundary_refs,
        1,
    );
    render_string_list(
        output,
        "detached_loop_policy_refs",
        &summary.detached_loop_policy_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase4_shared_space_self_driven_closeout_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase4SharedSpaceSelfDrivenCloseoutThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(closeout_kind) = summary.closeout_kind {
        writeln!(output, "  closeout_kind: {closeout_kind}").expect("write to string");
    } else {
        writeln!(output, "  closeout_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "current_package_refs",
        &summary.current_package_refs,
        1,
    );
    render_string_list(
        output,
        "user_spec_required_catalog_refs",
        &summary.user_spec_required_catalog_refs,
        1,
    );
    render_string_list(
        output,
        "retained_later_refs",
        &summary.retained_later_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold(
    output: &mut String,
    summary:
        &CurrentL2OperationalCliActualPhase5ProofProtocolRuntimePolicyHandoffCloseoutThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(closeout_kind) = summary.closeout_kind {
        writeln!(output, "  closeout_kind: {closeout_kind}").expect("write to string");
    } else {
        writeln!(output, "  closeout_kind: none").expect("write to string");
    }
    if let Some(verifier_handoff_surface_ref) = summary.verifier_handoff_surface_ref {
        writeln!(
            output,
            "  verifier_handoff_surface_ref: {verifier_handoff_surface_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  verifier_handoff_surface_ref: none").expect("write to string");
    }
    if let Some(theorem_retained_bridge_stop_ref) = summary.theorem_retained_bridge_stop_ref {
        writeln!(
            output,
            "  theorem_retained_bridge_stop_ref: {theorem_retained_bridge_stop_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  theorem_retained_bridge_stop_ref: none").expect("write to string");
    }
    if let Some(boundary_inventory_ref) = summary.boundary_inventory_ref {
        writeln!(output, "  boundary_inventory_ref: {boundary_inventory_ref}")
            .expect("write to string");
    } else {
        writeln!(output, "  boundary_inventory_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "retained_later_refs",
        &summary.retained_later_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase6_actual_parser_ast_carrier_first_tranche_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase6ActualParserAstCarrierFirstTrancheThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(carrier_kind) = summary.carrier_kind {
        writeln!(output, "  carrier_kind: {carrier_kind}").expect("write to string");
    } else {
        writeln!(output, "  carrier_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "accepted_surface_refs",
        &summary.accepted_surface_refs,
        1,
    );
    render_string_list(output, "code_anchor_refs", &summary.code_anchor_refs, 1);
    render_string_list(
        output,
        "retained_later_refs",
        &summary.retained_later_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold(
    output: &mut String,
    summary:
        &CurrentL2OperationalCliActualPhase6ActualCheckerRuntimeSkeletonFirstTrancheThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(skeleton_kind) = summary.skeleton_kind {
        writeln!(output, "  skeleton_kind: {skeleton_kind}").expect("write to string");
    } else {
        writeln!(output, "  skeleton_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "semantic_entry_refs",
        &summary.semantic_entry_refs,
        1,
    );
    render_string_list(
        output,
        "runtime_bridge_refs",
        &summary.runtime_bridge_refs,
        1,
    );
    render_string_list(
        output,
        "parser_bridge_contract_refs",
        &summary.parser_bridge_contract_refs,
        1,
    );
    render_string_list(
        output,
        "retained_later_refs",
        &summary.retained_later_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase6_compile_ready_verification_and_formal_hook_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase6CompileReadyVerificationAndFormalHookThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    render_string_list(
        output,
        "verification_gate_refs",
        &summary.verification_gate_refs,
        1,
    );
    render_string_list(output, "smoke_gate_refs", &summary.smoke_gate_refs, 1);
    if let Some(formal_hook_artifact_kind_ref) = summary.formal_hook_artifact_kind_ref {
        writeln!(
            output,
            "  formal_hook_artifact_kind_ref: {formal_hook_artifact_kind_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  formal_hook_artifact_kind_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "formal_hook_subject_kind_refs",
        &summary.formal_hook_subject_kind_refs,
        1,
    );
    render_string_list(
        output,
        "formal_hook_contract_row_core_refs",
        &summary.formal_hook_contract_row_core_refs,
        1,
    );
    render_string_list(
        output,
        "formal_hook_evidence_ref_family_refs",
        &summary.formal_hook_evidence_ref_family_refs,
        1,
    );
    render_string_list(
        output,
        "formal_hook_obligation_kind_refs",
        &summary.formal_hook_obligation_kind_refs,
        1,
    );
    render_string_list(
        output,
        "source_artifact_refs",
        &summary.source_artifact_refs,
        1,
    );
    render_string_list(output, "validation_refs", &summary.validation_refs, 1);
    render_string_list(
        output,
        "retained_later_refs",
        &summary.retained_later_refs,
        1,
    );
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase6_next_reopen_sequencing_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase6NextReopenSequencingThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(sequencing_kind_ref) = summary.sequencing_kind_ref {
        writeln!(output, "  sequencing_kind_ref: {sequencing_kind_ref}").expect("write to string");
    } else {
        writeln!(output, "  sequencing_kind_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "fixed_entry_criteria_refs",
        &summary.fixed_entry_criteria_refs,
        1,
    );
    if let Some(selected_first_reopen_ref) = summary.selected_first_reopen_ref {
        writeln!(
            output,
            "  selected_first_reopen_ref: {selected_first_reopen_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  selected_first_reopen_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_reopen_refs",
        &summary.deferred_reopen_refs,
        1,
    );
    render_string_list(output, "minimum_guard_refs", &summary.minimum_guard_refs, 1);
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase6_reserve_formal_tool_binding_inventory_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase6ReserveFormalToolBindingInventoryThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(inventory_kind) = summary.inventory_kind {
        writeln!(output, "  inventory_kind: {inventory_kind}").expect("write to string");
    } else {
        writeln!(output, "  inventory_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "fixed_entry_criteria_refs",
        &summary.fixed_entry_criteria_refs,
        1,
    );
    if let Some(first_reserve_ref) = summary.first_reserve_ref {
        writeln!(output, "  first_reserve_ref: {first_reserve_ref}").expect("write to string");
    } else {
        writeln!(output, "  first_reserve_ref: none").expect("write to string");
    }
    if let Some(second_reserve_ref) = summary.second_reserve_ref {
        writeln!(output, "  second_reserve_ref: {second_reserve_ref}").expect("write to string");
    } else {
        writeln!(output, "  second_reserve_ref: none").expect("write to string");
    }
    render_string_list(output, "minimum_guard_refs", &summary.minimum_guard_refs, 1);
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
        writeln!(output, "  guard_reason: {guard_reason}").expect("write to string");
    } else {
        writeln!(output, "  guard_reason: none").expect("write to string");
    }
}

fn render_actual_phase6_parser_side_followup_package_sequencing_threshold(
    output: &mut String,
    summary: &CurrentL2OperationalCliActualPhase6ParserSideFollowupPackageSequencingThresholdSummary,
) {
    writeln!(output, "  status: {}", summary.status).expect("write to string");
    writeln!(output, "  threshold_kind: {}", summary.threshold_kind).expect("write to string");
    if let Some(sequencing_kind) = summary.sequencing_kind {
        writeln!(output, "  sequencing_kind: {sequencing_kind}").expect("write to string");
    } else {
        writeln!(output, "  sequencing_kind: none").expect("write to string");
    }
    render_string_list(
        output,
        "fixed_entry_criteria_refs",
        &summary.fixed_entry_criteria_refs,
        1,
    );
    if let Some(selected_next_package_ref) = summary.selected_next_package_ref {
        writeln!(
            output,
            "  selected_next_package_ref: {selected_next_package_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  selected_next_package_ref: none").expect("write to string");
    }
    render_string_list(
        output,
        "deferred_reopen_refs",
        &summary.deferred_reopen_refs,
        1,
    );
    render_string_list(output, "minimum_guard_refs", &summary.minimum_guard_refs, 1);
    if let Some(next_comparison_target_ref) = summary.next_comparison_target_ref {
        writeln!(
            output,
            "  next_comparison_target_ref: {next_comparison_target_ref}"
        )
        .expect("write to string");
    } else {
        writeln!(output, "  next_comparison_target_ref: none").expect("write to string");
    }
    render_string_list(output, "evidence_refs", &summary.evidence_refs, 1);
    render_string_list(output, "compare_floor_refs", &summary.compare_floor_refs, 1);
    render_string_list(output, "guard_refs", &summary.guard_refs, 1);
    render_string_list(output, "kept_later_refs", &summary.kept_later_refs, 1);
    if let Some(guard_reason) = &summary.guard_reason {
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

fn render_string_map(
    output: &mut String,
    label: &str,
    values: &BTreeMap<String, String>,
    base_indent: usize,
) {
    let indent = "  ".repeat(base_indent);
    if values.is_empty() {
        writeln!(output, "{indent}{label}: []").expect("write to string");
        return;
    }

    writeln!(output, "{indent}{label}:").expect("write to string");
    for (key, value) in values {
        writeln!(output, "{}  {}: {}", indent, key, value).expect("write to string");
    }
}

fn authoritative_room_first_scenario_event_kind_ref(event: &EventKind) -> &'static str {
    match event {
        EventKind::PerformSuccess => "perform-success",
        EventKind::PerformFailure => "perform-failure",
        EventKind::Rollback => "rollback",
        EventKind::AtomicCut => "atomic-cut",
        EventKind::Reject => "reject",
    }
}

fn authoritative_room_first_scenario_extend_place_records(
    refs: &mut Vec<String>,
    place_store: &BTreeMap<String, Vec<String>>,
    key: &str,
) {
    if let Some(records) = place_store.get(key) {
        for record in records {
            let prefix = if key.contains("debug") {
                format!("debug_output:{key}:")
            } else {
                format!("place_record:{key}:")
            };
            let formatted = format!("{prefix}{record}");
            if !refs.contains(&formatted) {
                refs.push(formatted);
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

fn authoritative_room_first_scenario_guard_reason(
    report: &CurrentL2SourceSampleRunReport,
    verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
) -> String {
    let sample_id = report.sample_id.as_str();
    if sample_id == "p09-dice-delegated-rng-provider-placement" {
        return format!(
            "current authoritative-room first scenario keeps delegated RNG placement on the practical reserve route and does not promote it into the representative default pair (`p07` / `p08`) for `{sample_id}`"
        );
    }

    if sample_id == "p13-dice-late-join-missing-publication-witness" {
        let reason = report
            .runtime_report
            .checker_floor
            .static_gate
            .reasons
            .first()
            .cloned()
            .unwrap_or_else(|| "missing publication witness before handoff".to_string());
        return format!(
            "current authoritative-room first scenario keeps the late-join negative pair helper-local and guarded for `{sample_id}`: missing publication witness before handoff; {reason}"
        );
    }

    if sample_id == "p14-dice-late-join-handoff-before-publication" {
        let reason = report
            .runtime_report
            .checker_floor
            .static_gate
            .reasons
            .first()
            .cloned()
            .unwrap_or_else(|| "handoff-before-publish breaks late-join visibility".to_string());
        return format!(
            "current authoritative-room first scenario keeps the late-join negative pair helper-local and guarded for `{sample_id}`: handoff-before-publish breaks late-join visibility; {reason}"
        );
    }

    authoritative_room_vertical_slice_guard_reason(report, verification_preview)
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

fn order_handoff_edge_row_principal_surface_lines(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p07-dice-late-join-visible-history" => vec![
            "publish publish_roll_result@dice_state".to_string(),
            "handoff handoff_dice_authority@dice_state".to_string(),
            "  after publish(publish_roll_result@dice_state)".to_string(),
            "  requires witness(publish_roll_result@dice_state)".to_string(),
            "observe late_join_view@dice_state".to_string(),
            "  after handoff(handoff_dice_authority@dice_state)".to_string(),
        ],
        "p08-dice-stale-reconnect-refresh" => vec![
            "rollback stale_reconnect".to_string(),
            "refresh refresh_owner_snapshot@dice_state".to_string(),
            "  after rollback(stale_reconnect)".to_string(),
            "invalidate stale_incompatible_replay@dice_state".to_string(),
            "  after refresh(refresh_owner_snapshot@dice_state)".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn order_handoff_stage_block_secondary_lines(sample_id: &str) -> Vec<String> {
    stage_block_surface_lines(sample_id)
        .into_iter()
        .skip(1)
        .collect()
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

fn order_handoff_source_surface_artifact_negative_static_stop_refs(sample_id: &str) -> Vec<String> {
    match sample_id {
        "p13-dice-late-join-missing-publication-witness" => vec![
            "negative_static_stop:p13-dice-late-join-missing-publication-witness:publish_witness_required_before_handoff".to_string(),
            "negative_static_stop:p13-dice-late-join-missing-publication-witness:publish_then_handoff_then_observe_order_required".to_string(),
        ],
        "p14-dice-late-join-handoff-before-publication" => vec![
            "negative_static_stop:p14-dice-late-join-handoff-before-publication:handoff_before_publish_for_late_join_visibility".to_string(),
            "negative_static_stop:p14-dice-late-join-handoff-before-publication:publish_then_handoff_then_observe_order_required".to_string(),
        ],
        _ => Vec::new(),
    }
}

fn order_handoff_source_surface_artifact_guard_reason(
    report: &CurrentL2SourceSampleRunReport,
    verification_preview: &CurrentL2OperationalCliVerificationPreviewSummary,
) -> String {
    let sample_id = report.sample_id.as_str();
    if matches!(
        sample_id,
        "p13-dice-late-join-missing-publication-witness"
            | "p14-dice-late-join-handoff-before-publication"
    ) {
        let reason = report
            .runtime_report
            .checker_floor
            .static_gate
            .reasons
            .first()
            .cloned()
            .unwrap_or_else(|| {
                format!("late-join visibility negative pair remained rejected for `{sample_id}`")
            });
        return format!(
            "current order-handoff source surface / emitted-artifact actual adoption keeps the late-join negative pair helper-local and guarded for `{sample_id}`: {reason}"
        );
    }

    if sample_id == "p09-dice-delegated-rng-provider-placement" {
        return format!(
            "current order-handoff source surface / emitted-artifact actual adoption keeps delegated RNG placement on the serial-scope practical route and does not promote it into the representative principal pair (`p07` / `p08`) for `{sample_id}`"
        );
    }

    verification_preview.guard_reason.clone().unwrap_or_else(|| {
        format!(
            "current order-handoff source surface / emitted-artifact actual adoption only actualizes the representative authoritative-room pair (`p07` / `p08`) for `{sample_id}`"
        )
    })
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
