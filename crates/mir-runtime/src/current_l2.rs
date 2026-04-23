//! Non-production current L2 checker/runtime skeleton.
//!
//! This module keeps the parser bridge explicit:
//! - `Program` still comes from the parser-free semantic carrier.
//! - optional `CurrentL2ParserBridgeInput` carries actual stage 1 / stage 2 parser evidence.
//! - the runtime skeleton reports both checker-floor summaries and semantic run output.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use mir_ast::current_l2::{
    Stage1DeclGuardSlot, Stage1ParsedChainDecl, Stage1ParsedChainEdge,
    Stage1ParsedLineageAssertion, Stage1ParsedOptionDecl, Stage1ParsedProgram,
    Stage2ParsedTryFallback, Stage2StatementHeadKind, Stage3PredicateFragment,
    parse_stage1_program_text, parse_stage2_try_rollback_text,
    parse_stage3_minimal_predicate_fragment_text,
};
use mir_semantics::{
    ChainEdge, Contract, FixtureHostPlan, FixtureHostStub, InterpreterError, LineageAssertion,
    PlaceStore, Predicate, Program, ProgramKind, RunReport, Statement, StaticGateResult,
    StaticGateVerdict, TraceAuditSink, static_gate_program_detailed,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2ParserBridgeInput {
    pub stage1_program: Option<Stage1ParsedProgram>,
    pub stage2_try_fallback: Option<Stage2ParsedTryFallback>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2Stage1ReconnectClusters {
    pub same_lineage_floor: bool,
    pub missing_option_structure_floor: bool,
    pub capability_strengthening_floor: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentL2TryRollbackStructuralVerdict {
    NoFindings,
    FindingsPresent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentL2TryRollbackStructuralSubjectKind {
    TryFallback,
    AtomicCut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentL2TryRollbackStructuralFindingKind {
    MissingFallbackBody,
    DisallowedFallbackPlacement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2TryRollbackStructuralFindingRow {
    pub subject_kind: CurrentL2TryRollbackStructuralSubjectKind,
    pub finding_kind: CurrentL2TryRollbackStructuralFindingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2TryRollbackStructuralSummary {
    pub verdict: CurrentL2TryRollbackStructuralVerdict,
    pub findings: Vec<CurrentL2TryRollbackStructuralFindingRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2CheckerFloorReport {
    pub stage1_reconnect_clusters: Option<CurrentL2Stage1ReconnectClusters>,
    pub stage2_try_rollback_summary: Option<CurrentL2TryRollbackStructuralSummary>,
    pub static_gate: StaticGateResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentL2RuntimeSkeletonReport {
    pub checker_floor: CurrentL2CheckerFloorReport,
    pub run_report: RunReport,
}

#[derive(Debug, Clone)]
pub struct CurrentL2LoweredSourceProgram {
    pub program: Program,
    pub parser_bridge_input: CurrentL2ParserBridgeInput,
}

#[derive(Debug, Clone)]
pub struct CurrentL2SourceSampleRunReport {
    pub sample_id: String,
    pub sample_path: PathBuf,
    pub lowered: CurrentL2LoweredSourceProgram,
    pub runtime_report: CurrentL2RuntimeSkeletonReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2CheckerRuntimeFirstTrancheManifest {
    pub skeleton_kind: &'static str,
    pub semantic_entry_refs: &'static [&'static str],
    pub runtime_bridge_refs: &'static [&'static str],
    pub parser_bridge_contract_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2CompileReadyVerificationAndFormalHookManifest {
    pub verification_gate_refs: &'static [&'static str],
    pub smoke_gate_refs: &'static [&'static str],
    pub formal_hook_artifact_kind_ref: &'static str,
    pub formal_hook_subject_kind_refs: &'static [&'static str],
    pub formal_hook_contract_row_core_refs: &'static [&'static str],
    pub formal_hook_evidence_ref_family_refs: &'static [&'static str],
    pub formal_hook_obligation_kind_refs: &'static [&'static str],
    pub source_artifact_refs: &'static [&'static str],
    pub validation_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2Phase6NextReopenSequencingManifest {
    pub sequencing_kind_ref: &'static str,
    pub fixed_entry_criteria_refs: &'static [&'static str],
    pub selected_first_reopen_ref: &'static str,
    pub deferred_reopen_refs: &'static [&'static str],
    pub guard_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2Phase6ReserveFormalToolBindingInventoryManifest {
    pub inventory_kind: &'static str,
    pub fixed_entry_criteria_refs: &'static [&'static str],
    pub first_reserve_ref: &'static str,
    pub second_reserve_ref: &'static str,
    pub guard_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2Phase6ParserSideFollowupPackageSequencingManifest {
    pub sequencing_kind: &'static str,
    pub fixed_entry_criteria_refs: &'static [&'static str],
    pub selected_next_package_ref: &'static str,
    pub deferred_reopen_refs: &'static [&'static str],
    pub guard_refs: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest {
    pub scope_kind: &'static str,
    pub source_cluster_refs: &'static [&'static str],
    pub directory_ref: &'static str,
    pub file_layout_ref: &'static str,
    pub file_extension_policy: &'static str,
    pub sample_id_policy: &'static str,
    pub non_goal_refs: &'static [&'static str],
}

const CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_SEMANTIC_ENTRY_REFS: &[&str] = &[
    "static_gate_program_detailed",
    "direct_style_evaluator_from_program",
    "fixture_host_stub_run_program",
];

const CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_RUNTIME_BRIDGE_REFS: &[&str] = &[
    "mir_runtime_current_l2_module",
    "current_l2_runtime_skeleton_report",
];

const CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_PARSER_BRIDGE_CONTRACT_REFS: &[&str] = &[
    "stage1_reconnect_clusters",
    "stage2_try_rollback_structural_summary",
    "parser_bridge_consistency_guard",
];

const CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_RETAINED_LATER_REFS: &[&str] = &[
    "parser_to_program_lowering",
    "stage3_request_predicate_reconnect",
    "richer_host_interface",
    "final_public_runtime_checker_api",
    "formal_hook_concrete_tool_binding",
];

pub const CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_MANIFEST:
    CurrentL2CheckerRuntimeFirstTrancheManifest = CurrentL2CheckerRuntimeFirstTrancheManifest {
    skeleton_kind: "current_l2_nonproduction_checker_runtime_skeleton",
    semantic_entry_refs: CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_SEMANTIC_ENTRY_REFS,
    runtime_bridge_refs: CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_RUNTIME_BRIDGE_REFS,
    parser_bridge_contract_refs:
        CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_PARSER_BRIDGE_CONTRACT_REFS,
    retained_later_refs: CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_RETAINED_LATER_REFS,
};

pub fn current_l2_checker_runtime_first_tranche_manifest()
-> &'static CurrentL2CheckerRuntimeFirstTrancheManifest {
    &CURRENT_L2_CHECKER_RUNTIME_FIRST_TRANCHE_MANIFEST
}

const CURRENT_L2_COMPILE_READY_VERIFICATION_GATE_REFS: &[&str] = &[
    "cargo_test_mir_ast",
    "cargo_test_mir_runtime",
    "cargo_test_mir_semantics_current_l2_minimal_interpreter",
    "cargo_test_mir_semantics_current_l2_static_gate_support",
    "cargo_test_mir_semantics_current_l2_detached_bundle_support",
    "cargo_test_mir_semantics_current_l2_formal_hook_support",
    "python_unittest_current_l2_static_and_detached_loop",
];

const CURRENT_L2_COMPILE_READY_SMOKE_GATE_REFS: &[&str] =
    &["smoke_formal_hook_static", "smoke_formal_hook_runtime"];

const CURRENT_L2_COMPILE_READY_FORMAL_HOOK_SUBJECT_KIND_REFS: &[&str] =
    &["fixture_static_cluster", "runtime_try_cut_cluster"];

const CURRENT_L2_COMPILE_READY_FORMAL_HOOK_CONTRACT_ROW_CORE_REFS: &[&str] =
    &["obligation_kind", "evidence_refs"];

const CURRENT_L2_COMPILE_READY_FORMAL_HOOK_EVIDENCE_REF_FAMILY_REFS: &[&str] =
    &["ref_kind", "ref_id"];

const CURRENT_L2_COMPILE_READY_FORMAL_HOOK_OBLIGATION_KIND_REFS: &[&str] = &[
    "canonical_normalization_law",
    "no_re_promotion",
    "rollback_cut_non_interference",
];

const CURRENT_L2_COMPILE_READY_SOURCE_ARTIFACT_REFS: &[&str] =
    &["detached_static_gate_artifact", "detached_bundle_artifact"];

const CURRENT_L2_COMPILE_READY_VALIDATION_REFS: &[&str] =
    &["input_schema_version_guard", "input_artifact_kind_guard"];

const CURRENT_L2_COMPILE_READY_RETAINED_LATER_REFS: &[&str] = &[
    "concrete_theorem_tool_binding",
    "concrete_model_check_tool_binding",
    "parser_second_tranche_widen",
    "final_public_surface",
];

pub const CURRENT_L2_COMPILE_READY_VERIFICATION_AND_FORMAL_HOOK_MANIFEST:
    CurrentL2CompileReadyVerificationAndFormalHookManifest =
    CurrentL2CompileReadyVerificationAndFormalHookManifest {
        verification_gate_refs: CURRENT_L2_COMPILE_READY_VERIFICATION_GATE_REFS,
        smoke_gate_refs: CURRENT_L2_COMPILE_READY_SMOKE_GATE_REFS,
        formal_hook_artifact_kind_ref: "current_l2_tool_neutral_formal_hook",
        formal_hook_subject_kind_refs: CURRENT_L2_COMPILE_READY_FORMAL_HOOK_SUBJECT_KIND_REFS,
        formal_hook_contract_row_core_refs:
            CURRENT_L2_COMPILE_READY_FORMAL_HOOK_CONTRACT_ROW_CORE_REFS,
        formal_hook_evidence_ref_family_refs:
            CURRENT_L2_COMPILE_READY_FORMAL_HOOK_EVIDENCE_REF_FAMILY_REFS,
        formal_hook_obligation_kind_refs: CURRENT_L2_COMPILE_READY_FORMAL_HOOK_OBLIGATION_KIND_REFS,
        source_artifact_refs: CURRENT_L2_COMPILE_READY_SOURCE_ARTIFACT_REFS,
        validation_refs: CURRENT_L2_COMPILE_READY_VALIDATION_REFS,
        retained_later_refs: CURRENT_L2_COMPILE_READY_RETAINED_LATER_REFS,
    };

pub fn current_l2_compile_ready_verification_and_formal_hook_manifest()
-> &'static CurrentL2CompileReadyVerificationAndFormalHookManifest {
    &CURRENT_L2_COMPILE_READY_VERIFICATION_AND_FORMAL_HOOK_MANIFEST
}

const CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_FIXED_ENTRY_CRITERIA_REFS: &[&str] = &[
    "phase6_parser_first_tranche",
    "phase6_checker_runtime_first_tranche",
    "phase6_compile_ready_formal_hook",
];

const CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_DEFERRED_REOPEN_REFS: &[&str] = &[
    "theorem_first_concrete_tool_binding_route",
    "concrete_model_check_tool_binding",
];

const CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_GUARD_REFS: &[&str] = &[
    "keep_tool_neutral_formal_hook_as_entry_criteria",
    "avoid_request_head_clause_suite_richer_diagnostics_bulk_widen",
    "keep_model_check_line_reserve_only",
];

pub const CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_MANIFEST:
    CurrentL2Phase6NextReopenSequencingManifest = CurrentL2Phase6NextReopenSequencingManifest {
    sequencing_kind_ref: "phase6_checkpoint_postclose_next_reopen",
    fixed_entry_criteria_refs: CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_FIXED_ENTRY_CRITERIA_REFS,
    selected_first_reopen_ref: "phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route",
    deferred_reopen_refs: CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_DEFERRED_REOPEN_REFS,
    guard_refs: CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_GUARD_REFS,
};

pub fn current_l2_phase6_next_reopen_sequencing_manifest()
-> &'static CurrentL2Phase6NextReopenSequencingManifest {
    &CURRENT_L2_PHASE6_NEXT_REOPEN_SEQUENCING_MANIFEST
}

const CURRENT_L2_PHASE6_RESERVE_FORMAL_TOOL_BINDING_INVENTORY_FIXED_ENTRY_CRITERIA_REFS: &[&str] =
    &[
        "phase5_handoff_closeout",
        "phase6_compile_ready_formal_hook",
        "phase6_parser_second_tranche_first_package",
    ];

const CURRENT_L2_PHASE6_RESERVE_FORMAL_TOOL_BINDING_INVENTORY_GUARD_REFS: &[&str] = &[
    "keep_tool_neutral_formal_hook_as_current_entry_criteria",
    "keep_parser_followup_package_as_current_mainline",
    "avoid_dual_tool_choice_single_package",
    "avoid_public_checker_runtime_surface_backpressure",
];

pub const CURRENT_L2_PHASE6_RESERVE_FORMAL_TOOL_BINDING_INVENTORY_MANIFEST:
    CurrentL2Phase6ReserveFormalToolBindingInventoryManifest =
    CurrentL2Phase6ReserveFormalToolBindingInventoryManifest {
        inventory_kind: "phase6_postclose_formal_reserve_inventory",
        fixed_entry_criteria_refs:
            CURRENT_L2_PHASE6_RESERVE_FORMAL_TOOL_BINDING_INVENTORY_FIXED_ENTRY_CRITERIA_REFS,
        first_reserve_ref: "theorem_first_notebook_pressure_concrete_tool_binding_route",
        second_reserve_ref: "model_check_protocol_property_concrete_tool_binding_route",
        guard_refs: CURRENT_L2_PHASE6_RESERVE_FORMAL_TOOL_BINDING_INVENTORY_GUARD_REFS,
    };

pub fn current_l2_phase6_reserve_formal_tool_binding_inventory_manifest()
-> &'static CurrentL2Phase6ReserveFormalToolBindingInventoryManifest {
    &CURRENT_L2_PHASE6_RESERVE_FORMAL_TOOL_BINDING_INVENTORY_MANIFEST
}

const CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_FIXED_ENTRY_CRITERIA_REFS:
    &[&str] = &[
    "phase6_parser_second_tranche_first_package",
    "phase6_reserve_formal_tool_binding_inventory",
    "stage3_multiline_attachment_first_tranche_actualization",
];

const CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_DEFERRED_REOPEN_REFS: &[&str] = &[
    "phase6_request_clause_suite_publicization",
    "phase6_perform_head_final_public_parser_api",
    "phase6_span_rich_diagnostics",
    "phase6_final_grammar",
];

const CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_GUARD_REFS: &[&str] = &[
    "reuse_existing_stage3_minimal_predicate_fragment_surface",
    "keep_request_head_and_suite_ordering_out_of_scope",
    "keep_source_sample_path_after_parser_followup_cut",
];

pub const CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_MANIFEST:
    CurrentL2Phase6ParserSideFollowupPackageSequencingManifest =
    CurrentL2Phase6ParserSideFollowupPackageSequencingManifest {
        sequencing_kind: "phase6_parser_followup_next_package_selection",
        fixed_entry_criteria_refs:
            CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_FIXED_ENTRY_CRITERIA_REFS,
        selected_next_package_ref: "phase6_parser_second_tranche_shared_single_attachment_frame_first_package",
        deferred_reopen_refs:
            CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_DEFERRED_REOPEN_REFS,
        guard_refs: CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_GUARD_REFS,
    };

pub fn current_l2_phase6_parser_side_followup_package_sequencing_manifest()
-> &'static CurrentL2Phase6ParserSideFollowupPackageSequencingManifest {
    &CURRENT_L2_PHASE6_PARSER_SIDE_FOLLOWUP_PACKAGE_SEQUENCING_MANIFEST
}

const CURRENT_L2_FIXED_SUBSET_SOURCE_SAMPLE_CORPUS_SCOPE_SOURCE_CLUSTER_REFS: &[&str] = &[
    "e1_place_atomic_cut",
    "e2_try_fallback",
    "e3_option_admit_chain",
    "e4_malformed_lineage",
    "e21_try_atomic_cut_frontier",
    "e23_malformed_try_fallback_missing_fallback_body",
];

const CURRENT_L2_FIXED_SUBSET_SOURCE_SAMPLE_CORPUS_SCOPE_NON_GOAL_REFS: &[&str] = &[
    "not_final_parser_grammar",
    "not_fixture_reverse_generation",
    "not_verdict_or_stage_in_filename",
];

pub const CURRENT_L2_FIXED_SUBSET_SOURCE_SAMPLE_CORPUS_SCOPE_AND_FILE_LAYOUT_MANIFEST:
    CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest =
    CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest {
        scope_kind: "current_l2_fixed_subset_source_sample_corpus_scope",
        source_cluster_refs: CURRENT_L2_FIXED_SUBSET_SOURCE_SAMPLE_CORPUS_SCOPE_SOURCE_CLUSTER_REFS,
        directory_ref: "repo_root_samples_current_l2_directory",
        file_layout_ref: "flat_one_file_per_sample_layout",
        file_extension_policy: "neutral_text_dot_txt_until_final_grammar",
        sample_id_policy: "fixture_stem_aligned_kebab_case_sample_id",
        non_goal_refs: CURRENT_L2_FIXED_SUBSET_SOURCE_SAMPLE_CORPUS_SCOPE_NON_GOAL_REFS,
    };

pub fn current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest()
-> &'static CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest {
    &CURRENT_L2_FIXED_SUBSET_SOURCE_SAMPLE_CORPUS_SCOPE_AND_FILE_LAYOUT_MANIFEST
}

pub fn current_l2_default_source_sample_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/current-l2")
}

fn current_l2_runner_accepted_sample_paths() -> Vec<PathBuf> {
    let root = current_l2_default_source_sample_directory();
    [
        "e1-place-atomic-cut.txt",
        "e2-try-fallback.txt",
        "e3-option-admit-chain.txt",
        "e4-malformed-lineage.txt",
        "e5-underdeclared-lineage.txt",
        "e12-underdeclared-target-missing.txt",
        "e14-malformed-duplicate-option-declaration.txt",
        "e15-malformed-duplicate-chain-declaration.txt",
        "e16-malformed-missing-chain-head-option.txt",
        "e13-malformed-capability-strengthening.txt",
        "e19-malformed-target-mismatch.txt",
        "e21-try-atomic-cut-frontier.txt",
        "e22-try-atomic-cut-place-mismatch.txt",
        "e18-malformed-missing-successor-option.txt",
        "e20-malformed-late-capability-strengthening.txt",
        "e23-malformed-try-fallback-missing-fallback-body.txt",
    ]
    .into_iter()
    .map(|name| root.join(name))
    .collect()
}

pub fn resolve_current_l2_source_sample_path(
    sample_argument: &str,
) -> Result<PathBuf, InterpreterError> {
    let trimmed = sample_argument.trim();
    if trimmed.is_empty() {
        return Err(InterpreterError::InvalidProgram(
            "source sample argument must not be empty".to_string(),
        ));
    }

    let direct_path = PathBuf::from(trimmed);
    let accepted_paths = current_l2_runner_accepted_sample_paths();
    if direct_path.exists() {
        let normalized_direct = canonicalize_existing_source_sample_path(&direct_path)?;
        for accepted_path in &accepted_paths {
            if canonicalize_existing_source_sample_path(accepted_path)? == normalized_direct {
                return Ok(normalized_direct);
            }
        }
        return Err(InterpreterError::InvalidProgram(format!(
            "source sample path is outside the current accepted sample set: {}",
            direct_path.display()
        )));
    }

    let shorthand_name = if Path::new(trimmed).extension().is_some() {
        trimmed.to_string()
    } else {
        format!("{trimmed}.txt")
    };
    let shorthand_path = current_l2_default_source_sample_directory().join(shorthand_name);
    if accepted_paths.iter().any(|path| path == &shorthand_path) && shorthand_path.is_file() {
        return Ok(shorthand_path);
    }

    Err(InterpreterError::InvalidProgram(format!(
        "source sample not found: {trimmed} (checked direct path and {})",
        shorthand_path.display()
    )))
}

fn canonicalize_existing_source_sample_path(path: &Path) -> Result<PathBuf, InterpreterError> {
    fs::canonicalize(path).map_err(|error| {
        InterpreterError::InvalidProgram(format!(
            "failed to canonicalize source sample path {}: {error}",
            path.display()
        ))
    })
}

pub fn run_current_l2_source_sample(
    sample_argument: &str,
    host_plan: FixtureHostPlan,
) -> Result<CurrentL2SourceSampleRunReport, InterpreterError> {
    let sample_path = resolve_current_l2_source_sample_path(sample_argument)?;
    let source = fs::read_to_string(&sample_path).map_err(|error| {
        InterpreterError::InvalidProgram(format!(
            "failed to read source sample {}: {error}",
            sample_path.display()
        ))
    })?;
    let lowered = lower_current_l2_fixed_source_text(&source)?;
    let sample_id = current_l2_source_sample_id(&sample_path)?;
    let runtime_report = run_current_l2_runtime_skeleton(
        lowered.program.clone(),
        host_plan,
        Some(lowered.parser_bridge_input.clone()),
    )?;

    Ok(CurrentL2SourceSampleRunReport {
        sample_id,
        sample_path,
        lowered,
        runtime_report,
    })
}

pub fn run_current_l2_runtime_skeleton(
    program: Program,
    host_plan: FixtureHostPlan,
    parser_bridge_input: Option<CurrentL2ParserBridgeInput>,
) -> Result<CurrentL2RuntimeSkeletonReport, InterpreterError> {
    if let Some(bridge) = parser_bridge_input.as_ref() {
        validate_parser_bridge_input(&program, bridge)?;
    }

    let static_gate = current_l2_source_static_gate_program_detailed(&program);
    let checker_floor = CurrentL2CheckerFloorReport {
        stage1_reconnect_clusters: parser_bridge_input
            .as_ref()
            .and_then(|bridge| bridge.stage1_program.as_ref())
            .map(summarize_stage1_reconnect_clusters),
        stage2_try_rollback_summary: parser_bridge_input
            .as_ref()
            .and_then(|bridge| bridge.stage2_try_fallback.as_ref())
            .map(summarize_stage2_try_rollback_findings),
        static_gate: static_gate.clone(),
    };
    let runtime = if static_gate.verdict == StaticGateVerdict::Valid {
        FixtureHostStub::new(host_plan)?.run_program(program)?
    } else {
        RunReport {
            static_verdict: static_gate.verdict,
            entered_evaluation: false,
            terminal_outcome: None,
            trace_audit_sink: TraceAuditSink::default(),
            steps_executed: 0,
            final_place_store: PlaceStore::new(),
        }
    };

    Ok(CurrentL2RuntimeSkeletonReport {
        checker_floor,
        run_report: runtime,
    })
}

fn current_l2_source_static_gate_program_detailed(program: &Program) -> StaticGateResult {
    static_gate_program_detailed(program)
}

pub fn lower_current_l2_fixed_source_text(
    source: &str,
) -> Result<CurrentL2LoweredSourceProgram, InterpreterError> {
    let mut parser = CurrentL2FixedSourceParser::new(collect_current_l2_source_lines(source));
    let body = parser.parse_program_body()?;
    let stage1_program = parser.stage1_program()?;
    let stage2_try_fallback = parser.stage2_try_fallback()?;

    Ok(CurrentL2LoweredSourceProgram {
        program: Program {
            kind: ProgramKind::Program,
            body,
        },
        parser_bridge_input: CurrentL2ParserBridgeInput {
            stage1_program,
            stage2_try_fallback,
        },
    })
}

#[derive(Debug, Clone)]
struct CurrentL2SourceLine {
    line_no: usize,
    indent: usize,
    text: String,
    is_blank: bool,
}

fn collect_current_l2_source_lines(source: &str) -> Vec<CurrentL2SourceLine> {
    let mut lines = Vec::new();
    let mut in_leading_comment_block = true;

    for (line_no, raw_line) in source.lines().enumerate() {
        let trimmed = raw_line.trim();
        let is_leading_comment = in_leading_comment_block && trimmed.starts_with('#');
        let is_blank = trimmed.is_empty() || is_leading_comment;
        if !trimmed.is_empty() && !is_leading_comment {
            in_leading_comment_block = false;
        }
        lines.push(CurrentL2SourceLine {
            line_no: line_no + 1,
            indent: raw_line.chars().take_while(|ch| *ch == ' ').count(),
            text: trimmed.to_string(),
            is_blank,
        });
    }

    lines
}

struct CurrentL2FixedSourceParser {
    lines: Vec<CurrentL2SourceLine>,
    cursor: usize,
    stage1_lines: Vec<String>,
    stage1_supported: bool,
    try_source: Option<String>,
}

impl CurrentL2FixedSourceParser {
    fn new(lines: Vec<CurrentL2SourceLine>) -> Self {
        Self {
            lines,
            cursor: 0,
            stage1_lines: Vec::new(),
            stage1_supported: true,
            try_source: None,
        }
    }

    fn parse_program_body(&mut self) -> Result<Vec<Statement>, InterpreterError> {
        let mut body = Vec::new();

        loop {
            self.skip_blank_lines();
            let Some(line) = self.peek() else {
                break;
            };
            if line.indent != 0 {
                return Err(
                    self.line_error(line.line_no, "top-level statements must start at indent 0")
                );
            }
            body.push(self.parse_statement_at_indent(0)?);
        }

        Ok(body)
    }

    fn stage1_program(&self) -> Result<Option<Stage1ParsedProgram>, InterpreterError> {
        if !self.stage1_supported || self.stage1_lines.is_empty() {
            return Ok(None);
        }

        parse_stage1_program_text(&self.stage1_lines.join("\n"))
            .map(Some)
            .map_err(|message| {
                InterpreterError::InvalidProgram(format!(
                    "stage 1 bridge parse failed after source lowering: {message}"
                ))
            })
    }

    fn stage2_try_fallback(&self) -> Result<Option<Stage2ParsedTryFallback>, InterpreterError> {
        let Some(source) = self.try_source.as_ref() else {
            return Ok(None);
        };

        parse_stage2_try_rollback_text(source)
            .map(Some)
            .map_err(|message| {
                InterpreterError::InvalidProgram(format!(
                    "stage 2 bridge parse failed after source lowering: {message}"
                ))
            })
    }

    fn parse_statement_at_indent(
        &mut self,
        expected_indent: usize,
    ) -> Result<Statement, InterpreterError> {
        self.skip_blank_lines();
        let line = self.peek().ok_or_else(|| {
            InterpreterError::InvalidProgram(
                "unexpected end of input while parsing statement".into(),
            )
        })?;

        if line.indent != expected_indent {
            return Err(self.line_error(
                line.line_no,
                format!("expected indent {expected_indent}, got {}", line.indent),
            ));
        }

        if line.text.starts_with("place ") {
            return self.parse_place_block();
        }
        if line.text == "try {" {
            return self.parse_try_fallback();
        }
        if line.text.starts_with("option ") {
            return self.parse_option_decl();
        }
        if line.text.starts_with("chain ") {
            return self.parse_chain_decl();
        }
        if line.text.starts_with("perform ") {
            return self.parse_perform_statement();
        }
        if line.text == "atomic_cut" {
            self.cursor += 1;
            return Ok(Statement::AtomicCut);
        }
        if line.text.starts_with("fallback ") {
            return Err(self.line_error(
                line.line_no,
                "fallback row must follow a chain declaration at the same indentation",
            ));
        }

        Err(self.line_error(
            line.line_no,
            format!("unsupported fixed-subset source row `{}`", line.text),
        ))
    }

    fn parse_place_block(&mut self) -> Result<Statement, InterpreterError> {
        let line = self.peek().expect("place line should exist").clone();
        let tokens: Vec<&str> = line.text.split_whitespace().collect();
        if tokens.len() != 3 || tokens[0] != "place" || tokens[2] != "{" {
            return Err(self.line_error(
                line.line_no,
                format!("unsupported place block head `{}`", line.text),
            ));
        }

        self.cursor += 1;
        let (body, _) = self.parse_brace_block_with_indent(line.indent, line.line_no, "}")?;
        Ok(Statement::PlaceBlock {
            place: tokens[1].to_string(),
            body,
        })
    }

    fn parse_try_fallback(&mut self) -> Result<Statement, InterpreterError> {
        let line = self.peek().expect("try line should exist").clone();
        let start = self.cursor;
        self.cursor += 1;

        let (body, body_indent) = self.parse_try_body(line.indent)?;
        let (fallback_body, fallback_indent) =
            self.parse_brace_block_with_indent(line.indent, line.line_no, "}")?;
        let end = self.cursor.saturating_sub(1);

        if self.try_source.is_some() {
            return Err(self.line_error(
                line.line_no,
                "fixed-subset lowering first cut supports at most one try/fallback block",
            ));
        }
        self.try_source = Some(render_stage2_bridge_source(
            &self.lines[start..=end],
            line.indent,
            body_indent,
            fallback_indent,
        ));

        Ok(Statement::TryFallback {
            body,
            fallback_body,
        })
    }

    fn parse_try_body(
        &mut self,
        opener_indent: usize,
    ) -> Result<(Vec<Statement>, Option<usize>), InterpreterError> {
        let Some(first_indent) = self.next_child_indent(opener_indent) else {
            let line = self.peek().ok_or_else(|| {
                InterpreterError::InvalidProgram(
                    "missing `} fallback {` delimiter for try block".to_string(),
                )
            })?;
            if line.indent == opener_indent && line.text == "} fallback {" {
                self.cursor += 1;
                return Ok((Vec::new(), None));
            }
            return Err(self.line_error(
                line.line_no,
                "missing `} fallback {` delimiter for try block",
            ));
        };

        let mut body = Vec::new();
        loop {
            self.skip_blank_lines();
            let line = self.peek().ok_or_else(|| {
                InterpreterError::InvalidProgram(
                    "missing `} fallback {` delimiter for try block".to_string(),
                )
            })?;
            if line.indent <= opener_indent {
                if line.indent == opener_indent && line.text == "} fallback {" {
                    self.cursor += 1;
                    break;
                }
                return Err(self.line_error(
                    line.line_no,
                    "missing `} fallback {` delimiter for try block",
                ));
            }
            if line.indent != first_indent {
                return Err(self.line_error(
                    line.line_no,
                    format!(
                        "unexpected indentation inside try body: expected {first_indent}, got {}",
                        line.indent
                    ),
                ));
            }
            body.push(self.parse_statement_at_indent(first_indent)?);
        }

        Ok((body, Some(first_indent)))
    }

    fn parse_brace_block_with_indent(
        &mut self,
        opener_indent: usize,
        opener_line_no: usize,
        close_text: &str,
    ) -> Result<(Vec<Statement>, Option<usize>), InterpreterError> {
        let Some(first_indent) = self.next_child_indent(opener_indent) else {
            let line = self.peek().ok_or_else(|| {
                InterpreterError::InvalidProgram(format!(
                    "missing `{close_text}` to close block opened at line {opener_line_no}"
                ))
            })?;
            if line.indent == opener_indent && line.text == close_text {
                self.cursor += 1;
                return Ok((Vec::new(), None));
            }
            return Err(self.line_error(
                line.line_no,
                format!("missing `{close_text}` to close block opened at line {opener_line_no}"),
            ));
        };

        let mut body = Vec::new();
        loop {
            self.skip_blank_lines();
            let line = self.peek().ok_or_else(|| {
                InterpreterError::InvalidProgram(format!(
                    "missing `{close_text}` to close block opened at line {opener_line_no}"
                ))
            })?;
            if line.indent <= opener_indent {
                if line.indent == opener_indent && line.text == close_text {
                    self.cursor += 1;
                    break;
                }
                return Err(self.line_error(
                    line.line_no,
                    format!(
                        "missing `{close_text}` to close block opened at line {opener_line_no}"
                    ),
                ));
            }
            if line.indent != first_indent {
                return Err(self.line_error(
                    line.line_no,
                    format!(
                        "unexpected indentation inside block: expected {first_indent}, got {}",
                        line.indent
                    ),
                ));
            }
            body.push(self.parse_statement_at_indent(first_indent)?);
        }

        Ok((body, Some(first_indent)))
    }

    fn parse_option_decl(&mut self) -> Result<Statement, InterpreterError> {
        let line = self.peek().expect("option line should exist").clone();
        let tokens: Vec<&str> = line.text.split_whitespace().collect();
        let (name, target, capability, lease, admit_slot) = match tokens.as_slice() {
            [
                "option",
                name,
                "on",
                target,
                "capability",
                capability,
                "lease",
                lease,
            ] => (*name, (*target).to_string(), *capability, *lease, None),
            [
                "option",
                _name,
                "on",
                _target,
                "capability",
                _capability,
                "lease",
                _lease,
                "admit",
            ] => {
                return Err(self.line_error(line.line_no, "missing declaration-side admit payload"));
            }
            [
                "option",
                name,
                "on",
                target,
                "capability",
                capability,
                "lease",
                lease,
                "admit",
                admit_slot,
            ] => (
                *name,
                (*target).to_string(),
                *capability,
                *lease,
                Some(*admit_slot),
            ),
            [
                "option",
                name,
                "on",
                "capability",
                capability,
                "lease",
                lease,
            ] => (*name, String::new(), *capability, *lease, None),
            [
                "option",
                _name,
                "on",
                "capability",
                _capability,
                "lease",
                _lease,
                "admit",
            ] => {
                return Err(self.line_error(line.line_no, "missing declaration-side admit payload"));
            }
            [
                "option",
                name,
                "on",
                "capability",
                capability,
                "lease",
                lease,
                "admit",
                admit_slot,
            ] => (*name, String::new(), *capability, *lease, Some(*admit_slot)),
            _ => {
                return Err(self.line_error(
                    line.line_no,
                    format!("unsupported option declaration `{}`", line.text),
                ));
            }
        };

        let admit = match admit_slot {
            None => {
                self.stage1_lines.push(line.text.clone());
                None
            }
            Some(slot) => {
                self.stage1_supported = false;
                Some(self.parse_predicate_fragment(slot, line.line_no)?)
            }
        };

        self.cursor += 1;
        Ok(Statement::OptionDecl {
            name: name.to_string(),
            target,
            capability: capability.to_string(),
            lease: lease.to_string(),
            admit,
        })
    }

    fn parse_chain_decl(&mut self) -> Result<Statement, InterpreterError> {
        let line = self.peek().expect("chain line should exist").clone();
        let tokens: Vec<&str> = line.text.split_whitespace().collect();
        if tokens.len() != 4 || tokens[0] != "chain" || tokens[2] != "=" {
            return Err(self.line_error(
                line.line_no,
                format!("unsupported chain declaration `{}`", line.text),
            ));
        }

        self.stage1_lines.push(line.text.clone());
        self.cursor += 1;

        let name = tokens[1].to_string();
        let head = tokens[3].to_string();
        let mut previous = head.clone();
        let mut edges = Vec::new();

        loop {
            self.skip_blank_lines();
            let Some(next_line) = self.peek() else {
                break;
            };
            if next_line.indent != line.indent || !next_line.text.starts_with("fallback ") {
                break;
            }
            let (edge, successor) = parse_source_fallback_edge(&next_line.text, &previous)
                .map_err(|message| self.line_error(next_line.line_no, message))?;
            self.stage1_lines.push(next_line.text.clone());
            edges.push(edge);
            previous = successor;
            self.cursor += 1;
        }

        Ok(Statement::ChainDecl { name, head, edges })
    }

    fn parse_perform_statement(&mut self) -> Result<Statement, InterpreterError> {
        let line = self.peek().expect("perform line should exist").clone();
        let tokens: Vec<&str> = line.text.split_whitespace().collect();
        if tokens.len() != 4 || tokens[0] != "perform" {
            return Err(self.line_error(
                line.line_no,
                format!("unsupported perform head `{}`", line.text),
            ));
        }

        let op = tokens[1].to_string();
        let subject = tokens[3].to_string();
        let via = match tokens[2] {
            "on" => false,
            "via" => true,
            _ => {
                return Err(self.line_error(
                    line.line_no,
                    format!("unsupported perform head `{}`", line.text),
                ));
            }
        };

        self.cursor += 1;
        let contract = self.parse_contract_clause_block(line.indent)?;

        if via {
            Ok(Statement::PerformVia {
                op,
                chain_ref: subject,
                contract,
            })
        } else {
            Ok(Statement::PerformOn {
                op,
                target: subject,
                contract,
            })
        }
    }

    fn parse_contract_clause_block(
        &mut self,
        head_indent: usize,
    ) -> Result<Contract, InterpreterError> {
        let Some(clause_indent) = self.next_child_indent(head_indent) else {
            return Ok(Contract {
                require: Vec::new(),
                ensure: Vec::new(),
            });
        };

        let mut require = Vec::new();
        let mut ensure = Vec::new();

        loop {
            self.skip_blank_lines();
            let Some(line) = self.peek() else {
                break;
            };
            if line.indent <= head_indent {
                break;
            }
            if line.indent != clause_indent {
                return Err(self.line_error(
                    line.line_no,
                    format!(
                        "unexpected indentation inside contract clause block: expected {clause_indent}, got {}",
                        line.indent
                    ),
                ));
            }
            if line.text.starts_with("require:") || line.text.starts_with("ensure:") {
                return Err(self.line_error(
                    line.line_no,
                    "multiline contract clauses are outside the lowering first cut",
                ));
            }
            if let Some(fragment) = line.text.strip_prefix("require ") {
                require.push(self.parse_predicate_fragment(fragment, line.line_no)?);
                self.cursor += 1;
                continue;
            }
            if let Some(fragment) = line.text.strip_prefix("ensure ") {
                ensure.push(self.parse_predicate_fragment(fragment, line.line_no)?);
                self.cursor += 1;
                continue;
            }
            return Err(self.line_error(
                line.line_no,
                format!("unsupported contract clause `{}`", line.text),
            ));
        }

        Ok(Contract { require, ensure })
    }

    fn parse_predicate_fragment(
        &self,
        fragment_text: &str,
        line_no: usize,
    ) -> Result<Predicate, InterpreterError> {
        let trimmed = fragment_text.trim();
        if trimmed.is_empty() {
            return Err(self.line_error(line_no, "missing predicate fragment"));
        }

        let fragment = parse_stage3_minimal_predicate_fragment_text(trimmed)
            .map_err(|message| self.line_error(line_no, message))?;
        Ok(predicate_from_stage3_fragment(fragment))
    }

    fn skip_blank_lines(&mut self) {
        while self
            .lines
            .get(self.cursor)
            .is_some_and(|line| line.is_blank)
        {
            self.cursor += 1;
        }
    }

    fn next_child_indent(&self, parent_indent: usize) -> Option<usize> {
        self.lines
            .iter()
            .skip(self.cursor)
            .find(|line| !line.is_blank)
            .and_then(|line| (line.indent > parent_indent).then_some(line.indent))
    }

    fn peek(&self) -> Option<&CurrentL2SourceLine> {
        self.lines.get(self.cursor)
    }

    fn line_error(&self, line_no: usize, message: impl Into<String>) -> InterpreterError {
        InterpreterError::InvalidProgram(format!("line {line_no}: {}", message.into()))
    }
}

fn parse_source_fallback_edge(line: &str, previous: &str) -> Result<(ChainEdge, String), String> {
    let rest = line
        .strip_prefix("fallback ")
        .ok_or_else(|| format!("unsupported fallback row `{line}`"))?;
    let (successor, lineage_assertion) =
        if let Some((successor_part, lineage_part)) = rest.split_once(" @ lineage(") {
            let lineage_inner = lineage_part
                .strip_suffix(')')
                .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
            let (lineage_pred, lineage_succ) = lineage_inner
                .split_once(" -> ")
                .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
            (
                successor_part.trim().to_string(),
                Some(LineageAssertion {
                    predecessor: lineage_pred.trim().to_string(),
                    successor: lineage_succ.trim().to_string(),
                }),
            )
        } else {
            (rest.trim().to_string(), None)
        };

    Ok((
        ChainEdge {
            predecessor: previous.to_string(),
            successor: successor.clone(),
            lineage_assertion,
        },
        successor,
    ))
}

fn predicate_from_stage3_fragment(fragment: Stage3PredicateFragment) -> Predicate {
    match fragment {
        Stage3PredicateFragment::Atom { name } => Predicate::Atom { name },
        Stage3PredicateFragment::Call { callee, args } => Predicate::Call { callee, args },
        Stage3PredicateFragment::And { terms } => Predicate::And {
            terms: terms
                .into_iter()
                .map(predicate_from_stage3_fragment)
                .collect(),
        },
    }
}

fn render_stage2_bridge_source(
    lines: &[CurrentL2SourceLine],
    opener_indent: usize,
    body_indent: Option<usize>,
    fallback_indent: Option<usize>,
) -> String {
    let mut rendered = Vec::new();
    let mut in_fallback = false;

    for line in lines {
        if line.is_blank {
            continue;
        }
        if line.indent == opener_indent && line.text == "try {" {
            rendered.push(line.text.clone());
            continue;
        }
        if line.indent == opener_indent && line.text == "} fallback {" {
            rendered.push(line.text.clone());
            in_fallback = true;
            continue;
        }
        if line.indent == opener_indent && line.text == "}" {
            rendered.push(line.text.clone());
            continue;
        }

        let direct_indent = if in_fallback {
            fallback_indent
        } else {
            body_indent
        };
        if direct_indent.is_some_and(|indent| line.indent == indent) {
            rendered.push(line.text.clone());
        }
    }

    rendered.join("\n")
}

fn validate_parser_bridge_input(
    program: &Program,
    bridge: &CurrentL2ParserBridgeInput,
) -> Result<(), InterpreterError> {
    if let Some(stage1_program) = bridge.stage1_program.as_ref() {
        let expected = stage1_subset_from_program(program)?;
        if stage1_program != &expected {
            return Err(InterpreterError::InvalidProgram(
                "stage 1 parser bridge does not match semantic program subset".to_string(),
            ));
        }
    }

    if let Some(stage2_try_fallback) = bridge.stage2_try_fallback.as_ref() {
        let (expected_body, expected_fallback_body) =
            single_stage2_try_fallback_signature(program)?;
        if stage2_try_fallback.body() != expected_body.as_slice()
            || stage2_try_fallback.fallback_body() != expected_fallback_body.as_slice()
        {
            return Err(InterpreterError::InvalidProgram(
                "stage 2 parser bridge does not match semantic try/fallback subset".to_string(),
            ));
        }
    }

    Ok(())
}

fn current_l2_source_sample_id(sample_path: &Path) -> Result<String, InterpreterError> {
    sample_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(ToString::to_string)
        .ok_or_else(|| {
            InterpreterError::InvalidProgram(format!(
                "source sample path does not have a valid file stem: {}",
                sample_path.display()
            ))
        })
}

fn summarize_stage1_reconnect_clusters(
    parsed: &Stage1ParsedProgram,
) -> CurrentL2Stage1ReconnectClusters {
    let option_map: BTreeMap<&str, _> = parsed
        .options
        .iter()
        .map(|option| (option.name.as_str(), option))
        .collect();
    let mut summary = CurrentL2Stage1ReconnectClusters {
        same_lineage_floor: false,
        missing_option_structure_floor: false,
        capability_strengthening_floor: false,
    };

    for chain in &parsed.chains {
        if !option_map.contains_key(chain.head.as_str()) {
            summary.missing_option_structure_floor = true;
        }

        for edge in &chain.edges {
            if edge.lineage_assertion.is_some() {
                summary.same_lineage_floor = true;
            }

            let predecessor = option_map.get(edge.predecessor.as_str()).copied();
            let successor = option_map.get(edge.successor.as_str()).copied();
            if predecessor.is_none() || successor.is_none() {
                summary.missing_option_structure_floor = true;
            }

            if let (Some(predecessor), Some(successor)) = (predecessor, successor) {
                if capability_strengthens(
                    predecessor.capability.as_str(),
                    successor.capability.as_str(),
                ) {
                    summary.capability_strengthening_floor = true;
                }
            }
        }
    }

    summary
}

fn summarize_stage2_try_rollback_findings(
    parsed: &Stage2ParsedTryFallback,
) -> CurrentL2TryRollbackStructuralSummary {
    let mut findings = Vec::new();

    if parsed.fallback_body().is_empty() {
        findings.push(CurrentL2TryRollbackStructuralFindingRow {
            subject_kind: CurrentL2TryRollbackStructuralSubjectKind::TryFallback,
            finding_kind: CurrentL2TryRollbackStructuralFindingKind::MissingFallbackBody,
        });
    }

    for statement in parsed.fallback_body() {
        if statement.is_atomic_cut() {
            findings.push(CurrentL2TryRollbackStructuralFindingRow {
                subject_kind: CurrentL2TryRollbackStructuralSubjectKind::AtomicCut,
                finding_kind:
                    CurrentL2TryRollbackStructuralFindingKind::DisallowedFallbackPlacement,
            });
        }
    }

    let verdict = if findings.is_empty() {
        CurrentL2TryRollbackStructuralVerdict::NoFindings
    } else {
        CurrentL2TryRollbackStructuralVerdict::FindingsPresent
    };

    CurrentL2TryRollbackStructuralSummary { verdict, findings }
}

fn capability_strengthens(from: &str, to: &str) -> bool {
    matches!((from, to), ("read", "write"))
}

fn stage1_subset_from_program(program: &Program) -> Result<Stage1ParsedProgram, InterpreterError> {
    let mut options = Vec::new();
    let mut chains = Vec::new();
    collect_stage1_subset(&program.body, &mut options, &mut chains)?;
    Ok(Stage1ParsedProgram { options, chains })
}

fn collect_stage1_subset(
    statements: &[Statement],
    options: &mut Vec<Stage1ParsedOptionDecl>,
    chains: &mut Vec<Stage1ParsedChainDecl>,
) -> Result<(), InterpreterError> {
    for statement in statements {
        match statement {
            Statement::PlaceBlock { body, .. } => collect_stage1_subset(body, options, chains)?,
            Statement::OptionDecl {
                name,
                target,
                capability,
                lease,
                ..
            } => options.push(Stage1ParsedOptionDecl {
                name: name.clone(),
                target: target.clone(),
                capability: capability.clone(),
                decl_guard_slot: Stage1DeclGuardSlot {
                    surface_text: lease.clone(),
                },
            }),
            Statement::ChainDecl { name, head, edges } => {
                let edges = edges
                    .iter()
                    .map(stage1_edge_from_semantic_edge)
                    .collect::<Result<Vec<_>, _>>()?;
                chains.push(Stage1ParsedChainDecl {
                    name: name.clone(),
                    head: head.clone(),
                    edges,
                });
            }
            Statement::PerformOn { .. }
            | Statement::PerformVia { .. }
            | Statement::TryFallback { .. }
            | Statement::AtomicCut => {}
        }
    }

    Ok(())
}

fn stage1_edge_from_semantic_edge(
    edge: &ChainEdge,
) -> Result<Stage1ParsedChainEdge, InterpreterError> {
    Ok(Stage1ParsedChainEdge {
        predecessor: edge.predecessor.clone(),
        successor: edge.successor.clone(),
        lineage_assertion: edge
            .lineage_assertion
            .as_ref()
            .map(stage1_lineage_from_semantic_lineage),
    })
}

fn stage1_lineage_from_semantic_lineage(
    lineage: &LineageAssertion,
) -> Stage1ParsedLineageAssertion {
    Stage1ParsedLineageAssertion {
        predecessor: lineage.predecessor.clone(),
        successor: lineage.successor.clone(),
    }
}

fn single_stage2_try_fallback_signature(
    program: &Program,
) -> Result<(Vec<Stage2StatementHeadKind>, Vec<Stage2StatementHeadKind>), InterpreterError> {
    let mut try_fallbacks = Vec::new();
    collect_try_fallback_statements(&program.body, &mut try_fallbacks);

    match try_fallbacks.as_slice() {
        [Statement::TryFallback { body, fallback_body }] => Ok((
            statement_heads(body),
            statement_heads(fallback_body),
        )),
        [] => Err(InterpreterError::InvalidProgram(
            "stage 2 parser bridge requires exactly one TryFallback in the semantic program, found 0"
                .to_string(),
        )),
        _ => Err(InterpreterError::InvalidProgram(format!(
            "stage 2 parser bridge requires exactly one TryFallback in the semantic program, found {}",
            try_fallbacks.len()
        ))),
    }
}

fn collect_try_fallback_statements<'a>(
    statements: &'a [Statement],
    collected: &mut Vec<&'a Statement>,
) {
    for statement in statements {
        match statement {
            Statement::PlaceBlock { body, .. } => collect_try_fallback_statements(body, collected),
            Statement::TryFallback { .. } => collected.push(statement),
            Statement::PerformOn { .. }
            | Statement::PerformVia { .. }
            | Statement::OptionDecl { .. }
            | Statement::ChainDecl { .. }
            | Statement::AtomicCut => {}
        }
    }
}

fn statement_heads(statements: &[Statement]) -> Vec<Stage2StatementHeadKind> {
    statements
        .iter()
        .map(|statement| match statement {
            Statement::AtomicCut => Stage2StatementHeadKind::AtomicCut,
            Statement::PlaceBlock { .. }
            | Statement::PerformOn { .. }
            | Statement::PerformVia { .. }
            | Statement::OptionDecl { .. }
            | Statement::ChainDecl { .. }
            | Statement::TryFallback { .. } => Stage2StatementHeadKind::Other,
        })
        .collect()
}
