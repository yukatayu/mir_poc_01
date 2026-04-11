//! Non-production current L2 checker/runtime skeleton.
//!
//! This module keeps the parser bridge explicit:
//! - `Program` still comes from the parser-free semantic carrier.
//! - optional `CurrentL2ParserBridgeInput` carries actual stage 1 / stage 2 parser evidence.
//! - the runtime skeleton reports both checker-floor summaries and semantic run output.

use std::collections::BTreeMap;

use mir_ast::current_l2::{
    Stage1DeclGuardSlot, Stage1ParsedChainDecl, Stage1ParsedChainEdge, Stage1ParsedLineageAssertion,
    Stage1ParsedOptionDecl, Stage1ParsedProgram, Stage2ParsedTryFallback,
    Stage2StatementHeadKind,
};
use mir_semantics::{
    ChainEdge, FixtureHostPlan, FixtureHostStub, InterpreterError, LineageAssertion, Program,
    RunReport, Statement, StaticGateResult, static_gate_program_detailed,
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
        let (expected_body, expected_fallback_body) = single_stage2_try_fallback_signature(program)?;
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
