//! Non-production current L2 parser carrier floor.
//!
//! This module intentionally exposes only the stage 1 declaration/lineage subset and
//! the stage 2 try/fallback structural subset that were frozen in the Phase 3/6 docs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2FirstTrancheManifest {
    pub carrier_kind: &'static str,
    pub accepted_surface_refs: &'static [&'static str],
    pub code_anchor_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

const CURRENT_L2_FIRST_TRANCHE_ACCEPTED_SURFACE_REFS: &[&str] = &[
    "stage1_option_decl_chain_surface",
    "stage2_try_fallback_structural_surface",
];

const CURRENT_L2_FIRST_TRANCHE_CODE_ANCHOR_REFS: &[&str] = &[
    "mir_ast_current_l2_module",
    "stage1_stage2_parser_spike_tests",
];

const CURRENT_L2_FIRST_TRANCHE_RETAINED_LATER_REFS: &[&str] = &[
    "stage3_admit_slot_surface",
    "stage3_request_clause_suite",
    "stage3_predicate_fragment",
    "perform_head_final_public_api",
    "span_rich_diagnostics",
    "final_grammar",
];

pub const CURRENT_L2_FIRST_TRANCHE_MANIFEST: CurrentL2FirstTrancheManifest =
    CurrentL2FirstTrancheManifest {
        carrier_kind: "current_l2_nonproduction_parser_carrier",
        accepted_surface_refs: CURRENT_L2_FIRST_TRANCHE_ACCEPTED_SURFACE_REFS,
        code_anchor_refs: CURRENT_L2_FIRST_TRANCHE_CODE_ANCHOR_REFS,
        retained_later_refs: CURRENT_L2_FIRST_TRANCHE_RETAINED_LATER_REFS,
    };

pub fn current_l2_first_tranche_manifest() -> &'static CurrentL2FirstTrancheManifest {
    &CURRENT_L2_FIRST_TRANCHE_MANIFEST
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2SecondTrancheManifest {
    pub carrier_kind: &'static str,
    pub accepted_surface_refs: &'static [&'static str],
    pub code_anchor_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

const CURRENT_L2_SECOND_TRANCHE_ACCEPTED_SURFACE_REFS: &[&str] = &[
    "stage3_decl_admit_slot_surface",
    "stage3_minimal_predicate_fragment_surface",
];

const CURRENT_L2_SECOND_TRANCHE_CODE_ANCHOR_REFS: &[&str] = &[
    "mir_ast_current_l2_module",
    "stage3_admit_slot_tests",
    "stage3_predicate_fragment_tests",
    "stage3_multiline_and_suite_tests_reusing_fragment_parser",
];

const CURRENT_L2_SECOND_TRANCHE_RETAINED_LATER_REFS: &[&str] = &[
    "shared_single_attachment_frame",
    "request_clause_suite_publicization",
    "perform_head_final_public_api",
    "span_rich_diagnostics",
    "final_grammar",
    "theorem_model_check_concrete_binding",
];

pub const CURRENT_L2_SECOND_TRANCHE_MANIFEST: CurrentL2SecondTrancheManifest =
    CurrentL2SecondTrancheManifest {
        carrier_kind: "current_l2_nonproduction_parser_second_tranche_carrier",
        accepted_surface_refs: CURRENT_L2_SECOND_TRANCHE_ACCEPTED_SURFACE_REFS,
        code_anchor_refs: CURRENT_L2_SECOND_TRANCHE_CODE_ANCHOR_REFS,
        retained_later_refs: CURRENT_L2_SECOND_TRANCHE_RETAINED_LATER_REFS,
    };

pub fn current_l2_second_tranche_manifest() -> &'static CurrentL2SecondTrancheManifest {
    &CURRENT_L2_SECOND_TRANCHE_MANIFEST
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2SharedSingleAttachmentFrameManifest {
    pub carrier_kind: &'static str,
    pub accepted_surface_refs: &'static [&'static str],
    pub code_anchor_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

const CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_ACCEPTED_SURFACE_REFS: &[&str] = &[
    "stage3_option_admit_multiline_extraction_surface",
    "stage3_request_clause_multiline_extraction_surface",
    "stage3_minimal_predicate_fragment_surface",
];

const CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_CODE_ANCHOR_REFS: &[&str] = &[
    "mir_ast_current_l2_module",
    "mir_ast_crate_surface_note",
    "stage3_multiline_attachment_tests",
];

const CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_RETAINED_LATER_REFS: &[&str] = &[
    "request_clause_suite_publicization",
    "perform_head_final_public_parser_api",
    "span_rich_diagnostics",
    "final_grammar",
    "fixed_subset_source_sample_corpus_scope_and_file_layout",
];

pub const CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_MANIFEST:
    CurrentL2SharedSingleAttachmentFrameManifest = CurrentL2SharedSingleAttachmentFrameManifest {
    carrier_kind: "current_l2_nonproduction_parser_followup_carrier",
    accepted_surface_refs: CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_ACCEPTED_SURFACE_REFS,
    code_anchor_refs: CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_CODE_ANCHOR_REFS,
    retained_later_refs: CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_RETAINED_LATER_REFS,
};

pub fn current_l2_shared_single_attachment_frame_manifest()
-> &'static CurrentL2SharedSingleAttachmentFrameManifest {
    &CURRENT_L2_SHARED_SINGLE_ATTACHMENT_FRAME_MANIFEST
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2RequestClauseSuiteManifest {
    pub carrier_kind: &'static str,
    pub accepted_surface_refs: &'static [&'static str],
    pub code_anchor_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

const CURRENT_L2_REQUEST_CLAUSE_SUITE_ACCEPTED_SURFACE_REFS: &[&str] = &[
    "stage3_request_clause_suite_surface",
    "stage3_request_clause_multiline_extraction_surface",
    "stage3_minimal_predicate_fragment_surface",
];

const CURRENT_L2_REQUEST_CLAUSE_SUITE_CODE_ANCHOR_REFS: &[&str] = &[
    "mir_ast_current_l2_module",
    "stage3_request_clause_suite_tests",
];

const CURRENT_L2_REQUEST_CLAUSE_SUITE_RETAINED_LATER_REFS: &[&str] = &[
    "perform_head_final_public_parser_api",
    "span_rich_diagnostics",
    "final_grammar",
];

pub const CURRENT_L2_REQUEST_CLAUSE_SUITE_MANIFEST: CurrentL2RequestClauseSuiteManifest =
    CurrentL2RequestClauseSuiteManifest {
        carrier_kind: "current_l2_nonproduction_request_clause_suite_carrier",
        accepted_surface_refs: CURRENT_L2_REQUEST_CLAUSE_SUITE_ACCEPTED_SURFACE_REFS,
        code_anchor_refs: CURRENT_L2_REQUEST_CLAUSE_SUITE_CODE_ANCHOR_REFS,
        retained_later_refs: CURRENT_L2_REQUEST_CLAUSE_SUITE_RETAINED_LATER_REFS,
    };

pub fn current_l2_request_clause_suite_manifest() -> &'static CurrentL2RequestClauseSuiteManifest {
    &CURRENT_L2_REQUEST_CLAUSE_SUITE_MANIFEST
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3RequestClauseSuite {
    pub require_fragment_text: Option<String>,
    pub ensure_fragment_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentL2PerformHeadManifest {
    pub carrier_kind: &'static str,
    pub accepted_surface_refs: &'static [&'static str],
    pub code_anchor_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

const CURRENT_L2_PERFORM_HEAD_ACCEPTED_SURFACE_REFS: &[&str] = &[
    "stage3_perform_owner_surface",
    "stage3_perform_on_head_surface",
    "stage3_perform_via_head_surface",
];

const CURRENT_L2_PERFORM_HEAD_CODE_ANCHOR_REFS: &[&str] =
    &["mir_ast_current_l2_module", "stage3_perform_head_tests"];

const CURRENT_L2_PERFORM_HEAD_RETAINED_LATER_REFS: &[&str] = &[
    "request_clause_suite_bundle_attachment",
    "span_rich_diagnostics",
    "final_grammar",
];

pub const CURRENT_L2_PERFORM_HEAD_MANIFEST: CurrentL2PerformHeadManifest =
    CurrentL2PerformHeadManifest {
        carrier_kind: "current_l2_nonproduction_perform_head_carrier",
        accepted_surface_refs: CURRENT_L2_PERFORM_HEAD_ACCEPTED_SURFACE_REFS,
        code_anchor_refs: CURRENT_L2_PERFORM_HEAD_CODE_ANCHOR_REFS,
        retained_later_refs: CURRENT_L2_PERFORM_HEAD_RETAINED_LATER_REFS,
    };

pub fn current_l2_perform_head_manifest() -> &'static CurrentL2PerformHeadManifest {
    &CURRENT_L2_PERFORM_HEAD_MANIFEST
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage3PerformTargetRef {
    On(String),
    Via(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3PerformHead {
    pub op: String,
    pub target_ref: Stage3PerformTargetRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1DeclGuardSlot {
    pub surface_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedOptionDecl {
    pub name: String,
    pub target: String,
    pub capability: String,
    pub decl_guard_slot: Stage1DeclGuardSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedLineageAssertion {
    pub predecessor: String,
    pub successor: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedChainEdge {
    pub predecessor: String,
    pub successor: String,
    pub lineage_assertion: Option<Stage1ParsedLineageAssertion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedChainDecl {
    pub name: String,
    pub head: String,
    pub edges: Vec<Stage1ParsedChainEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedProgram {
    pub options: Vec<Stage1ParsedOptionDecl>,
    pub chains: Vec<Stage1ParsedChainDecl>,
}

pub fn parse_stage1_program_text(source: &str) -> Result<Stage1ParsedProgram, String> {
    let mut options = Vec::new();
    let mut chains = Vec::new();
    let mut active_chain: Option<(usize, String)> = None;

    for (line_no, raw_line) in source.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("option ") {
            active_chain = None;
            options.push(
                parse_option_decl(line)
                    .map_err(|message| format!("line {}: {}", line_no + 1, message))?,
            );
            continue;
        }

        if line.starts_with("chain ") {
            let chain = parse_chain_decl(line)
                .map_err(|message| format!("line {}: {}", line_no + 1, message))?;
            let head = chain.head.clone();
            chains.push(chain);
            active_chain = Some((chains.len() - 1, head));
            continue;
        }

        if line.starts_with("fallback ") {
            let (chain_index, previous) = active_chain.take().ok_or_else(|| {
                format!("line {}: fallback row without active chain", line_no + 1)
            })?;
            let (edge, next_predecessor) = parse_fallback_edge(line, &previous)
                .map_err(|message| format!("line {}: {}", line_no + 1, message))?;
            chains[chain_index].edges.push(edge);
            active_chain = Some((chain_index, next_predecessor));
            continue;
        }

        return Err(format!(
            "line {}: unsupported stage 1 input `{}`",
            line_no + 1,
            line
        ));
    }

    Ok(Stage1ParsedProgram { options, chains })
}

fn parse_option_decl(line: &str) -> Result<Stage1ParsedOptionDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() > 8 && tokens[8] == "admit" {
        return Err("option-local admit is outside stage 1 accepted cluster".to_string());
    }
    match tokens.as_slice() {
        [
            "option",
            name,
            "on",
            target,
            "capability",
            capability,
            "lease",
            lease,
        ] => Ok(Stage1ParsedOptionDecl {
            name: (*name).to_string(),
            target: (*target).to_string(),
            capability: (*capability).to_string(),
            decl_guard_slot: Stage1DeclGuardSlot {
                surface_text: (*lease).to_string(),
            },
        }),
        [
            "option",
            name,
            "on",
            "capability",
            capability,
            "lease",
            lease,
        ] => Ok(Stage1ParsedOptionDecl {
            name: (*name).to_string(),
            target: String::new(),
            capability: (*capability).to_string(),
            decl_guard_slot: Stage1DeclGuardSlot {
                surface_text: (*lease).to_string(),
            },
        }),
        _ => Err(format!("unsupported option declaration `{line}`")),
    }
}

fn parse_chain_decl(line: &str) -> Result<Stage1ParsedChainDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 4 || tokens[0] != "chain" || tokens[2] != "=" {
        return Err(format!("unsupported chain declaration `{line}`"));
    }

    Ok(Stage1ParsedChainDecl {
        name: tokens[1].to_string(),
        head: tokens[3].to_string(),
        edges: Vec::new(),
    })
}

fn parse_fallback_edge(
    line: &str,
    previous: &str,
) -> Result<(Stage1ParsedChainEdge, String), String> {
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
                Some(Stage1ParsedLineageAssertion {
                    predecessor: lineage_pred.trim().to_string(),
                    successor: lineage_succ.trim().to_string(),
                }),
            )
        } else {
            (rest.trim().to_string(), None)
        };

    Ok((
        Stage1ParsedChainEdge {
            predecessor: previous.to_string(),
            successor: successor.clone(),
            lineage_assertion,
        },
        successor,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage2StatementHeadKind {
    AtomicCut,
    Other,
}

impl Stage2StatementHeadKind {
    pub fn is_atomic_cut(&self) -> bool {
        matches!(self, Self::AtomicCut)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage2ParsedTryFallback {
    body: Vec<Stage2StatementHeadKind>,
    fallback_body: Vec<Stage2StatementHeadKind>,
}

impl Stage2ParsedTryFallback {
    pub fn body(&self) -> &[Stage2StatementHeadKind] {
        &self.body
    }

    pub fn fallback_body(&self) -> &[Stage2StatementHeadKind] {
        &self.fallback_body
    }
}

pub fn parse_stage2_try_rollback_text(source: &str) -> Result<Stage2ParsedTryFallback, String> {
    let lines: Vec<&str> = source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    if lines.is_empty() {
        return Err("stage 2 try/rollback input must not be empty".to_string());
    }

    if lines[0] != "try {" {
        return Err(format!(
            "stage 2 input must start with `try {{`, got `{}`",
            lines[0]
        ));
    }

    let mut body = Vec::new();
    let mut fallback_body = Vec::new();
    let mut in_fallback = false;
    let mut closed = false;
    let mut nested_block_depth = 0usize;

    for line in lines.iter().skip(1) {
        if nested_block_depth > 0 {
            if line.ends_with('{') {
                nested_block_depth += 1;
                continue;
            }
            if *line == "}" {
                nested_block_depth = nested_block_depth.saturating_sub(1);
                continue;
            }
            continue;
        }

        if !in_fallback {
            if *line == "} fallback {" {
                in_fallback = true;
                continue;
            }
            if line.ends_with('{') {
                body.push(Stage2StatementHeadKind::Other);
                nested_block_depth = 1;
                continue;
            }
            body.push(parse_statement_head(line)?);
            continue;
        }

        if *line == "}" {
            if closed {
                return Err(format!("unexpected content after fallback close `{line}`"));
            }
            closed = true;
            continue;
        }

        if closed {
            return Err(format!("unexpected content after fallback close `{line}`"));
        }

        if line.ends_with('{') {
            fallback_body.push(Stage2StatementHeadKind::Other);
            nested_block_depth = 1;
            continue;
        }

        fallback_body.push(parse_statement_head(line)?);
    }

    if !in_fallback {
        return Err("stage 2 input is missing `} fallback {` delimiter".to_string());
    }

    if nested_block_depth != 0 {
        return Err("stage 2 input is missing closing `}` for nested block".to_string());
    }

    if !closed {
        return Err("stage 2 input is missing closing `}` for fallback block".to_string());
    }

    Ok(Stage2ParsedTryFallback {
        body,
        fallback_body,
    })
}

fn parse_statement_head(line: &str) -> Result<Stage2StatementHeadKind, String> {
    if line == "atomic_cut" {
        return Ok(Stage2StatementHeadKind::AtomicCut);
    }
    if line.ends_with('{') || line == "}" || line.starts_with("fallback ") {
        return Err(format!("unsupported stage 2 statement head `{line}`"));
    }
    Ok(Stage2StatementHeadKind::Other)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3DeclAdmitSlot {
    pub surface_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedOptionDecl {
    pub name: String,
    pub target: String,
    pub capability: String,
    pub decl_guard_slot: Stage1DeclGuardSlot,
    pub decl_admit_slot: Option<Stage3DeclAdmitSlot>,
}

pub type Stage3ParsedLineageAssertion = Stage1ParsedLineageAssertion;
pub type Stage3ParsedChainEdge = Stage1ParsedChainEdge;
pub type Stage3ParsedChainDecl = Stage1ParsedChainDecl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedProgram {
    pub options: Vec<Stage3ParsedOptionDecl>,
    pub chains: Vec<Stage3ParsedChainDecl>,
}

/// Retained-later stage 3 helper. This is comparison / bridge evidence, not the
/// current Phase 6 parser first tranche.
pub fn parse_stage3_admit_slot_program_text(source: &str) -> Result<Stage3ParsedProgram, String> {
    let mut options = Vec::new();
    let mut chains = Vec::new();
    let mut active_chain: Option<(usize, String)> = None;

    for (line_no, raw_line) in source.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("option ") {
            active_chain = None;
            options.push(
                parse_stage3_option_decl(line)
                    .map_err(|message| format!("line {}: {}", line_no + 1, message))?,
            );
            continue;
        }

        if line.starts_with("chain ") {
            let chain = parse_chain_decl(line)
                .map_err(|message| format!("line {}: {}", line_no + 1, message))?;
            let head = chain.head.clone();
            chains.push(chain);
            active_chain = Some((chains.len() - 1, head));
            continue;
        }

        if line.starts_with("fallback ") {
            let (chain_index, previous) = active_chain.take().ok_or_else(|| {
                format!("line {}: fallback row without active chain", line_no + 1)
            })?;
            let (edge, next_predecessor) = parse_fallback_edge(line, &previous)
                .map_err(|message| format!("line {}: {}", line_no + 1, message))?;
            chains[chain_index].edges.push(edge);
            active_chain = Some((chain_index, next_predecessor));
            continue;
        }

        if line.starts_with("perform ") {
            return Err(format!(
                "line {}: request head is outside stage 3 admit-slot first tranche",
                line_no + 1
            ));
        }

        if line.starts_with("require ") {
            return Err(format!(
                "line {}: request-local require clause is outside stage 3 admit-slot first tranche",
                line_no + 1
            ));
        }

        if line.starts_with("ensure ") {
            return Err(format!(
                "line {}: request-local ensure clause is outside stage 3 admit-slot first tranche",
                line_no + 1
            ));
        }

        return Err(format!(
            "line {}: unsupported stage 3 admit-slot input `{}`",
            line_no + 1,
            line
        ));
    }

    Ok(Stage3ParsedProgram { options, chains })
}

fn parse_stage3_option_decl(line: &str) -> Result<Stage3ParsedOptionDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.as_slice() {
        [
            "option",
            name,
            "on",
            target,
            "capability",
            capability,
            "lease",
            lease,
        ] => Ok(Stage3ParsedOptionDecl {
            name: (*name).to_string(),
            target: (*target).to_string(),
            capability: (*capability).to_string(),
            decl_guard_slot: Stage1DeclGuardSlot {
                surface_text: (*lease).to_string(),
            },
            decl_admit_slot: None,
        }),
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
        ] => Err("missing declaration-side admit slot payload".to_string()),
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
        ] => Ok(Stage3ParsedOptionDecl {
            name: (*name).to_string(),
            target: (*target).to_string(),
            capability: (*capability).to_string(),
            decl_guard_slot: Stage1DeclGuardSlot {
                surface_text: (*lease).to_string(),
            },
            decl_admit_slot: Some(Stage3DeclAdmitSlot {
                surface_text: (*admit_slot).to_string(),
            }),
        }),
        [
            "option",
            name,
            "on",
            "capability",
            capability,
            "lease",
            lease,
        ] => Ok(Stage3ParsedOptionDecl {
            name: (*name).to_string(),
            target: String::new(),
            capability: (*capability).to_string(),
            decl_guard_slot: Stage1DeclGuardSlot {
                surface_text: (*lease).to_string(),
            },
            decl_admit_slot: None,
        }),
        [
            "option",
            _name,
            "on",
            "capability",
            _capability,
            "lease",
            _lease,
            "admit",
        ] => Err("missing declaration-side admit slot payload".to_string()),
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
        ] => Ok(Stage3ParsedOptionDecl {
            name: (*name).to_string(),
            target: String::new(),
            capability: (*capability).to_string(),
            decl_guard_slot: Stage1DeclGuardSlot {
                surface_text: (*lease).to_string(),
            },
            decl_admit_slot: Some(Stage3DeclAdmitSlot {
                surface_text: (*admit_slot).to_string(),
            }),
        }),
        _ => Err(format!("unsupported option declaration `{line}`")),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stage3SourceLine {
    indent: usize,
    text: String,
    is_blank: bool,
}

/// Retained-later stage 3 helper for multiline attachment extraction.
pub fn extract_stage3_option_admit_multiline_fragment_text(source: &str) -> Result<String, String> {
    let lines = collect_stage3_source_lines(source);
    let head_index = find_first_stage3_head(&lines, "option ")
        .ok_or_else(|| "missing option declaration head".to_string())?;

    extract_stage3_multiline_clause_from_head(&lines, head_index, "admit")
}

/// Retained-later stage 3 helper for request-clause attachment extraction.
pub fn extract_stage3_request_clause_multiline_fragment_text(
    source: &str,
    clause_name: &str,
) -> Result<String, String> {
    let lines = collect_stage3_source_lines(source);
    let head_index = find_first_stage3_head(&lines, "perform ")
        .ok_or_else(|| "missing perform request head".to_string())?;

    extract_stage3_multiline_clause_from_head(&lines, head_index, clause_name)
}

/// Non-production stage 3 helper for fixed two-slot request clause suites.
pub fn parse_stage3_request_clause_suite_text(
    source: &str,
) -> Result<Stage3RequestClauseSuite, String> {
    let lines = collect_stage3_source_lines(source);
    let head_index = find_first_stage3_head(&lines, "perform ")
        .ok_or_else(|| "missing perform request head".to_string())?;
    let head_indent = lines[head_index].indent;
    let child_indent = lines.iter().skip(head_index + 1).find_map(|line| {
        if line.is_blank || line.indent <= head_indent {
            None
        } else {
            Some(line.indent)
        }
    });

    let Some(child_indent) = child_indent else {
        return Ok(Stage3RequestClauseSuite {
            require_fragment_text: None,
            ensure_fragment_text: None,
        });
    };

    let mut suite = Stage3RequestClauseSuite {
        require_fragment_text: None,
        ensure_fragment_text: None,
    };
    let mut index = head_index + 1;
    let mut saw_clause = false;
    let mut pending_blank_between_clauses = false;

    while index < lines.len() {
        let line = &lines[index];

        if line.is_blank {
            if saw_clause {
                pending_blank_between_clauses = true;
            }
            index += 1;
            continue;
        }

        if line.indent <= head_indent {
            break;
        }

        if line.indent > child_indent {
            return Err(
                "unexpected nested continuation outside request-local clause block".to_string(),
            );
        }

        if line.indent < child_indent {
            break;
        }

        if pending_blank_between_clauses {
            return Err("blank line is not allowed between request-local clauses".to_string());
        }

        if line.text == "require:" {
            if suite.require_fragment_text.is_some() {
                return Err("duplicate `require` clause is not allowed".to_string());
            }
            if suite.ensure_fragment_text.is_some() {
                return Err("require clause cannot appear after ensure clause".to_string());
            }
            let (fragment, next_index) =
                extract_stage3_suite_multiline_block(&lines, index, "require:")?;
            suite.require_fragment_text = Some(fragment);
            saw_clause = true;
            index = next_index;
            continue;
        }

        if let Some(fragment) =
            extract_stage3_single_line_request_clause_fragment(&line.text, "require")?
        {
            if suite.require_fragment_text.is_some() {
                return Err("duplicate `require` clause is not allowed".to_string());
            }
            if suite.ensure_fragment_text.is_some() {
                return Err("require clause cannot appear after ensure clause".to_string());
            }
            suite.require_fragment_text = Some(fragment);
            saw_clause = true;
            index += 1;
            continue;
        }

        if line.text == "ensure:" {
            if suite.ensure_fragment_text.is_some() {
                return Err("duplicate `ensure` clause is not allowed".to_string());
            }
            let (fragment, next_index) =
                extract_stage3_suite_multiline_block(&lines, index, "ensure:")?;
            suite.ensure_fragment_text = Some(fragment);
            saw_clause = true;
            index = next_index;
            continue;
        }

        if let Some(fragment) =
            extract_stage3_single_line_request_clause_fragment(&line.text, "ensure")?
        {
            if suite.ensure_fragment_text.is_some() {
                return Err("duplicate `ensure` clause is not allowed".to_string());
            }
            suite.ensure_fragment_text = Some(fragment);
            saw_clause = true;
            index += 1;
            continue;
        }

        return Err(format!(
            "unsupported request-local clause line inside fixed two-slot suite: `{}`",
            line.text
        ));
    }

    Ok(suite)
}

/// Non-production stage 3 helper for perform-head structural extraction.
pub fn parse_stage3_perform_head_text(source: &str) -> Result<Stage3PerformHead, String> {
    let lines = collect_stage3_source_lines(source);
    let head_index = find_first_stage3_head(&lines, "perform ")
        .ok_or_else(|| "missing perform request head".to_string())?;

    parse_stage3_perform_head_line(&lines[head_index].text)
}

fn collect_stage3_source_lines(source: &str) -> Vec<Stage3SourceLine> {
    source
        .lines()
        .map(|raw_line| {
            let trimmed = raw_line.trim();
            Stage3SourceLine {
                indent: raw_line.chars().take_while(|ch| *ch == ' ').count(),
                text: trimmed.to_string(),
                is_blank: trimmed.is_empty(),
            }
        })
        .collect()
}

fn find_first_stage3_head(lines: &[Stage3SourceLine], prefix: &str) -> Option<usize> {
    lines
        .iter()
        .position(|line| !line.is_blank && line.text.starts_with(prefix))
}

fn parse_stage3_perform_head_line(line: &str) -> Result<Stage3PerformHead, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    match tokens.as_slice() {
        ["perform"] => Err("missing perform operation after `perform`".to_string()),
        ["perform", _op, "on"] => Err("missing perform-on target after `on`".to_string()),
        ["perform", _op, "via"] => Err("missing perform-via chain ref after `via`".to_string()),
        ["perform", op, "on", target] => Ok(Stage3PerformHead {
            op: (*op).to_string(),
            target_ref: Stage3PerformTargetRef::On((*target).to_string()),
        }),
        ["perform", op, "via", chain_ref] => Ok(Stage3PerformHead {
            op: (*op).to_string(),
            target_ref: Stage3PerformTargetRef::Via((*chain_ref).to_string()),
        }),
        ["perform", _op, relation, ..] if *relation != "on" && *relation != "via" => Err(format!(
            "perform head must use `on` or `via`, got `{relation}`"
        )),
        ["perform", ..] => Err(format!("unsupported perform head shape `{line}`")),
        _ => Err(format!("unsupported perform head shape `{line}`")),
    }
}

fn extract_stage3_single_line_request_clause_fragment(
    text: &str,
    clause_name: &str,
) -> Result<Option<String>, String> {
    let prefix = format!("{clause_name} ");
    if !text.starts_with(&prefix) {
        return Ok(None);
    }

    let fragment = text[prefix.len()..].trim();
    if fragment.is_empty() {
        return Err(format!("missing predicate fragment after `{clause_name}`"));
    }

    Ok(Some(fragment.to_string()))
}

fn extract_stage3_multiline_clause_from_head(
    lines: &[Stage3SourceLine],
    head_index: usize,
    clause_name: &str,
) -> Result<String, String> {
    let head_indent = lines[head_index].indent;
    let header = format!("{clause_name}:");
    let first_child_indent = lines
        .iter()
        .enumerate()
        .skip(head_index + 1)
        .find_map(|(_, line)| {
            if line.is_blank || line.indent <= head_indent {
                None
            } else {
                Some(line.indent)
            }
        })
        .ok_or_else(|| format!("missing `{header}` attachment header"))?;

    let mut index = head_index + 1;
    while index < lines.len() {
        let line = &lines[index];

        if line.is_blank {
            index += 1;
            continue;
        }
        if line.indent <= head_indent {
            break;
        }
        if line.indent == first_child_indent {
            if line.text == header {
                return extract_stage3_multiline_block(lines, index, &header);
            }

            index += 1;
            while index < lines.len() {
                let nested = &lines[index];
                if nested.is_blank {
                    index += 1;
                    continue;
                }
                if nested.indent <= first_child_indent {
                    break;
                }
                index += 1;
            }
            continue;
        }

        index += 1;
    }

    Err(format!("missing `{header}` attachment header"))
}

fn extract_stage3_multiline_block(
    lines: &[Stage3SourceLine],
    header_index: usize,
    header: &str,
) -> Result<String, String> {
    let header_indent = lines[header_index].indent;
    let mut block_lines = Vec::new();
    let mut index = header_index + 1;
    let blank_line_error =
        format!("blank line is not allowed inside multiline predicate block after {header}");

    while index < lines.len() {
        let line = &lines[index];
        if line.is_blank {
            return Err(blank_line_error);
        }
        if line.indent <= header_indent {
            break;
        }
        block_lines.push(line);
        index += 1;
    }

    if block_lines.is_empty() {
        return Err(format!("missing multiline predicate block after {header}"));
    }

    let min_indent = block_lines
        .iter()
        .map(|line| line.indent)
        .min()
        .expect("block_lines is not empty");

    Ok(block_lines
        .iter()
        .map(|line| line.dedented_text(min_indent))
        .collect::<Vec<_>>()
        .join("\n"))
}

fn extract_stage3_suite_multiline_block(
    lines: &[Stage3SourceLine],
    header_index: usize,
    header: &str,
) -> Result<(String, usize), String> {
    let header_indent = lines[header_index].indent;
    let mut block_lines = Vec::new();
    let mut index = header_index + 1;
    let blank_line_error =
        format!("blank line is not allowed inside multiline predicate block after {header}");

    while index < lines.len() {
        let line = &lines[index];
        if line.is_blank {
            return Err(blank_line_error);
        }
        if line.indent <= header_indent {
            break;
        }
        block_lines.push(line);
        index += 1;
    }

    if block_lines.is_empty() {
        return Err(format!("missing multiline predicate block after {header}"));
    }

    let min_indent = block_lines
        .iter()
        .map(|line| line.indent)
        .min()
        .expect("block_lines is not empty");

    let fragment = block_lines
        .iter()
        .map(|line| line.dedented_text(min_indent))
        .collect::<Vec<_>>()
        .join("\n");

    Ok((fragment, index))
}

impl Stage3SourceLine {
    fn dedented_text(&self, min_indent: usize) -> String {
        let relative_indent = self.indent.saturating_sub(min_indent);
        format!("{}{}", " ".repeat(relative_indent), self.text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage3PredicateFragment {
    Atom { name: String },
    Call { callee: String, args: Vec<String> },
    And { terms: Vec<Stage3PredicateFragment> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stage3PredicateToken {
    Ident(String),
    And,
    LParen,
    RParen,
    Comma,
}

/// Retained-later stage 3 helper for narrow predicate fragments.
pub fn parse_stage3_minimal_predicate_fragment_text(
    source: &str,
) -> Result<Stage3PredicateFragment, String> {
    let tokens = tokenize_stage3_predicate_fragment(source)?;
    let mut parser = Stage3PredicateParser::new(tokens);
    let fragment = parser.parse_expression()?;

    if !parser.is_eof() {
        return Err(format!(
            "unexpected trailing token {:?} after predicate fragment",
            parser.peek()
        ));
    }

    Ok(fragment)
}

fn tokenize_stage3_predicate_fragment(source: &str) -> Result<Vec<Stage3PredicateToken>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(ch) = chars.peek().copied() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        match ch {
            '(' => {
                chars.next();
                tokens.push(Stage3PredicateToken::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Stage3PredicateToken::RParen);
            }
            ',' => {
                chars.next();
                tokens.push(Stage3PredicateToken::Comma);
            }
            _ if is_stage3_ident_start(ch) => {
                let mut ident = String::new();
                while let Some(next) = chars.peek().copied() {
                    if is_stage3_ident_continue(next) {
                        ident.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if ident == "and" {
                    tokens.push(Stage3PredicateToken::And);
                } else {
                    tokens.push(Stage3PredicateToken::Ident(ident));
                }
            }
            _ => {
                return Err(format!(
                    "unsupported character `{ch}` in predicate fragment"
                ));
            }
        }
    }

    Ok(tokens)
}

fn is_stage3_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_stage3_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

struct Stage3PredicateParser {
    tokens: Vec<Stage3PredicateToken>,
    cursor: usize,
}

impl Stage3PredicateParser {
    fn new(tokens: Vec<Stage3PredicateToken>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Stage3PredicateToken> {
        self.tokens.get(self.cursor)
    }

    fn bump(&mut self) -> Option<Stage3PredicateToken> {
        let token = self.tokens.get(self.cursor).cloned();
        if token.is_some() {
            self.cursor += 1;
        }
        token
    }

    fn parse_expression(&mut self) -> Result<Stage3PredicateFragment, String> {
        let first = self.parse_term()?;
        let mut terms = vec![first];

        while matches!(self.peek(), Some(Stage3PredicateToken::And)) {
            self.bump();
            terms.push(self.parse_term()?);
        }

        if terms.len() == 1 {
            Ok(terms.pop().expect("one term should remain"))
        } else {
            Ok(Stage3PredicateFragment::And { terms })
        }
    }

    fn parse_term(&mut self) -> Result<Stage3PredicateFragment, String> {
        match self.peek() {
            Some(Stage3PredicateToken::LParen) => {
                self.bump();
                let inner = self.parse_expression()?;
                match self.bump() {
                    Some(Stage3PredicateToken::RParen) => Ok(inner),
                    other => Err(format!(
                        "expected `)` after grouped fragment, got {other:?}"
                    )),
                }
            }
            Some(Stage3PredicateToken::Ident(_)) => self.parse_atom_or_call(),
            other => Err(format!("expected predicate term, got {other:?}")),
        }
    }

    fn parse_atom_or_call(&mut self) -> Result<Stage3PredicateFragment, String> {
        let ident = match self.bump() {
            Some(Stage3PredicateToken::Ident(ident)) => ident,
            other => return Err(format!("expected identifier, got {other:?}")),
        };

        if !matches!(self.peek(), Some(Stage3PredicateToken::LParen)) {
            return Ok(Stage3PredicateFragment::Atom { name: ident });
        }

        self.bump();
        let mut args = Vec::new();
        if !matches!(self.peek(), Some(Stage3PredicateToken::RParen)) {
            loop {
                match self.bump() {
                    Some(Stage3PredicateToken::Ident(arg)) => args.push(arg),
                    other => return Err(format!("expected call argument, got {other:?}")),
                }

                match self.peek() {
                    Some(Stage3PredicateToken::Comma) => {
                        self.bump();
                    }
                    Some(Stage3PredicateToken::RParen) => break,
                    other => {
                        return Err(format!(
                            "expected `,` or `)` after call argument, got {other:?}"
                        ));
                    }
                }
            }
        }

        match self.bump() {
            Some(Stage3PredicateToken::RParen) => Ok(Stage3PredicateFragment::Call {
                callee: ident,
                args,
            }),
            other => Err(format!("expected `)` after call arguments, got {other:?}")),
        }
    }
}
