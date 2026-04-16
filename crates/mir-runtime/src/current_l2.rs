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
    Predicate, Program, ProgramKind, RunReport, Statement, StaticGateResult,
    static_gate_program_detailed,
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

    let checker_floor = CurrentL2CheckerFloorReport {
        stage1_reconnect_clusters: parser_bridge_input
            .as_ref()
            .and_then(|bridge| bridge.stage1_program.as_ref())
            .map(summarize_stage1_reconnect_clusters),
        stage2_try_rollback_summary: parser_bridge_input
            .as_ref()
            .and_then(|bridge| bridge.stage2_try_fallback.as_ref())
            .map(summarize_stage2_try_rollback_findings),
        static_gate: static_gate_program_detailed(&program),
    };
    let runtime = FixtureHostStub::new(host_plan)?.run_program(program)?;

    Ok(CurrentL2RuntimeSkeletonReport {
        checker_floor,
        run_report: runtime,
    })
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
    source
        .lines()
        .enumerate()
        .map(|(line_no, raw_line)| {
            let trimmed = raw_line.trim();
            CurrentL2SourceLine {
                line_no: line_no + 1,
                indent: raw_line.chars().take_while(|ch| *ch == ' ').count(),
                text: trimmed.to_string(),
                is_blank: trimmed.is_empty(),
            }
        })
        .collect()
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
        if tokens.len() < 8
            || tokens[0] != "option"
            || tokens[2] != "on"
            || tokens[4] != "capability"
            || tokens[6] != "lease"
        {
            return Err(self.line_error(
                line.line_no,
                format!("unsupported option declaration `{}`", line.text),
            ));
        }

        let admit = match tokens.len() {
            8 => {
                self.stage1_lines.push(line.text.clone());
                None
            }
            9 if tokens[8] == "admit" => {
                return Err(self.line_error(line.line_no, "missing declaration-side admit payload"));
            }
            10 if tokens[8] == "admit" => {
                self.stage1_supported = false;
                Some(self.parse_predicate_fragment(tokens[9], line.line_no)?)
            }
            _ => {
                return Err(self.line_error(
                    line.line_no,
                    format!("unsupported option declaration `{}`", line.text),
                ));
            }
        };

        self.cursor += 1;
        Ok(Statement::OptionDecl {
            name: tokens[1].to_string(),
            target: tokens[3].to_string(),
            capability: tokens[5].to_string(),
            lease: tokens[7].to_string(),
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
    let (successor_part, lineage_part) = rest
        .split_once(" @ lineage(")
        .ok_or_else(|| "missing edge-local lineage metadata".to_string())?;
    let lineage_inner = lineage_part
        .strip_suffix(')')
        .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
    let (lineage_pred, lineage_succ) = lineage_inner
        .split_once(" -> ")
        .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
    let successor = successor_part.trim().to_string();

    Ok((
        ChainEdge {
            predecessor: previous.to_string(),
            successor: successor.clone(),
            lineage_assertion: Some(LineageAssertion {
                predecessor: lineage_pred.trim().to_string(),
                successor: lineage_succ.trim().to_string(),
            }),
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
            summary.same_lineage_floor = true;

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
    let lineage_assertion = edge
        .lineage_assertion
        .as_ref()
        .map(stage1_lineage_from_semantic_lineage)
        .ok_or_else(|| {
            InterpreterError::InvalidProgram(
                "stage 1 parser bridge cannot represent a semantic chain edge without lineage assertion"
                    .to_string(),
            )
        })?;

    Ok(Stage1ParsedChainEdge {
        predecessor: edge.predecessor.clone(),
        successor: edge.successor.clone(),
        lineage_assertion,
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
