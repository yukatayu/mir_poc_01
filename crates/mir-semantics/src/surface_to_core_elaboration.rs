use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use mir_ast::{
    surface_alpha::{
        SurfaceAssignStmt, SurfaceModule, SurfacePlaceItem, SurfaceRoleInstanceBlock, SurfaceStmt,
        SurfaceWhenBlock, parse_surface_mir_report, parse_surface_mir_report_path,
    },
    textual_alpha::{SourceSpan, TextualMirDiagnostic},
};
use serde::{Deserialize, Serialize};

const REMOTE_REQUEST_FAILURES: &[&str] = &[
    "MissingCapability",
    "MissingWitness",
    "RouteUnavailable",
    "StaleMembership",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceToCoreElaborationReport {
    pub accepted: bool,
    pub module_path: Option<String>,
    pub core_ir: SurfaceCoreIr,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub accepted_obligations: Vec<SurfaceCoreObligation>,
    pub residual_obligations: Vec<SurfaceCoreObligation>,
    pub source_authority: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SurfaceCoreIr {
    pub transitions: Vec<SurfaceCoreTransition>,
    pub remote_requests: Vec<SurfaceCoreRemoteRequest>,
    pub generated_edges: Vec<SurfaceCoreGeneratedEdge>,
    pub source_spans: Vec<SurfaceCoreSourceSpan>,
    pub obligations: Vec<SurfaceCoreObligation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreTransition {
    pub transition_id: String,
    pub locus: String,
    pub trigger: String,
    pub kind: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreRemoteRequest {
    pub request_id: String,
    pub request_kind: String,
    pub requester_locus: String,
    pub owner_locus: String,
    pub state_name: String,
    pub key_expr: String,
    pub access_text: String,
    pub generated_from: String,
    pub required_failures: Vec<String>,
    pub declared_failures: Vec<String>,
    pub failure_row_complete: bool,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreGeneratedEdge {
    pub edge_id: String,
    pub edge_kind: String,
    pub from_locus: String,
    pub to_locus: String,
    pub request_id: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreSourceSpan {
    pub entity_id: String,
    pub entity_kind: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreObligation {
    pub code: String,
    pub detail: String,
    pub source_span: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexedStateDecl {
    owner_locus: String,
    state_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexedTarget {
    state_name: String,
    key_expr: String,
    access_text: String,
}

#[derive(Debug, Default)]
struct ElaborationContext {
    indexed_states: BTreeMap<(String, String), IndexedStateDecl>,
    diagnostics: Vec<TextualMirDiagnostic>,
    core_ir: SurfaceCoreIr,
    next_transition: usize,
    next_request: usize,
    next_edge: usize,
}

pub fn elaborate_surface_to_core_source(source: &str) -> SurfaceToCoreElaborationReport {
    let parse_report = parse_surface_mir_report(source);
    match parse_report.module {
        Some(module) => elaborate_surface_to_core_module(module),
        None => rejected_parse_report(parse_report.diagnostics),
    }
}

pub fn elaborate_surface_to_core_path(path: impl AsRef<Path>) -> SurfaceToCoreElaborationReport {
    let parse_report = parse_surface_mir_report_path(path);
    match parse_report.module {
        Some(module) => elaborate_surface_to_core_module(module),
        None => rejected_parse_report(parse_report.diagnostics),
    }
}

pub fn elaborate_surface_to_core_module(module: SurfaceModule) -> SurfaceToCoreElaborationReport {
    let module_path = module.module_path.clone();
    let mut context = ElaborationContext::default();
    collect_indexed_states(&module, &mut context);
    elaborate_whens(&module, &mut context);

    let accepted = context.diagnostics.is_empty();
    let accepted_obligations = if accepted {
        accepted_obligations()
    } else {
        Vec::new()
    };
    let residual_obligations = residual_obligations();
    context
        .core_ir
        .obligations
        .extend(accepted_obligations.iter().cloned());
    context
        .core_ir
        .obligations
        .extend(residual_obligations.iter().cloned());

    SurfaceToCoreElaborationReport {
        accepted,
        module_path: Some(module_path),
        core_ir: context.core_ir,
        diagnostics: context.diagnostics,
        accepted_obligations,
        residual_obligations,
        source_authority: ".mir".to_string(),
        final_public_api_frozen: false,
    }
}

pub fn surface_elaboration_diagnostic_codes(
    report: &SurfaceToCoreElaborationReport,
) -> Vec<String> {
    report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.clone())
        .collect()
}

fn rejected_parse_report(diagnostics: Vec<TextualMirDiagnostic>) -> SurfaceToCoreElaborationReport {
    SurfaceToCoreElaborationReport {
        accepted: false,
        module_path: None,
        core_ir: SurfaceCoreIr {
            obligations: residual_obligations(),
            ..SurfaceCoreIr::default()
        },
        diagnostics,
        accepted_obligations: Vec::new(),
        residual_obligations: residual_obligations(),
        source_authority: ".mir".to_string(),
        final_public_api_frozen: false,
    }
}

fn collect_indexed_states(module: &SurfaceModule, context: &mut ElaborationContext) {
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::State(state) = item else {
                continue;
            };
            if state.index.is_none() {
                continue;
            }
            let key = (block.place_ref.clone(), state.state_name.clone());
            if context.indexed_states.contains_key(&key) {
                context.diagnostics.push(diagnostic(
                    "ambiguous_indexed_state_name",
                    "indexed state names must be unique within an owner locus before elaboration",
                    state.span.clone(),
                ));
                continue;
            }
            context.indexed_states.insert(
                key,
                IndexedStateDecl {
                    owner_locus: block.place_ref.clone(),
                    state_name: state.state_name.clone(),
                },
            );
        }
    }
}

fn elaborate_whens(module: &SurfaceModule, context: &mut ElaborationContext) {
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::When(when) = item else {
                continue;
            };
            push_when_transition(context, &block.place_ref, when, "surface_place_when");
            elaborate_statements(
                &when.body,
                &block.place_ref,
                Some(&block.place_ref),
                when,
                context,
            );
        }
    }
    for block in &module.role_instance_blocks {
        elaborate_role_instance_block(block, context);
    }
}

fn elaborate_role_instance_block(
    block: &SurfaceRoleInstanceBlock,
    context: &mut ElaborationContext,
) {
    let access_locus = format!("role:{}", block.role_ref);
    for when in &block.whens {
        push_when_transition(context, &access_locus, when, "surface_role_when");
        elaborate_statements(&when.body, &access_locus, None, when, context);
    }
}

fn elaborate_statements(
    body: &[SurfaceStmt],
    access_locus: &str,
    owner_hint: Option<&str>,
    when: &SurfaceWhenBlock,
    context: &mut ElaborationContext,
) {
    for stmt in body {
        match stmt {
            SurfaceStmt::Assign(assign) => {
                elaborate_assign(assign, access_locus, owner_hint, when, context);
            }
            SurfaceStmt::NestedPlaceBlock(block) => {
                elaborate_statements(
                    &block.body,
                    access_locus,
                    Some(&block.place_ref),
                    when,
                    context,
                );
            }
            SurfaceStmt::Join(join) => push_unsupported_statement(
                context,
                "join",
                "P-SURF-05 role admission owns join lowering",
                join.span.clone(),
            ),
            SurfaceStmt::Require(raw) => push_unsupported_statement(
                context,
                "require",
                "P-SURF-03 does not lower arbitrary require statements",
                raw.span.clone(),
            ),
            SurfaceStmt::Grant(raw) => push_unsupported_statement(
                context,
                "grant",
                "P-SURF-05 role admission owns grant lowering",
                raw.span.clone(),
            ),
            SurfaceStmt::Publish(raw) => push_unsupported_statement(
                context,
                "publish",
                "P-SURF-04 auto communication owns publish lowering",
                raw.span.clone(),
            ),
            SurfaceStmt::Raw(raw) => push_unsupported_statement(
                context,
                "raw",
                "P-SURF-03 only lowers assignments and nested place blocks",
                raw.span.clone(),
            ),
        }
    }
}

fn elaborate_assign(
    assign: &SurfaceAssignStmt,
    access_locus: &str,
    owner_hint: Option<&str>,
    when: &SurfaceWhenBlock,
    context: &mut ElaborationContext,
) {
    if let Some(target) = parse_indexed_target(&assign.target_text) {
        if let Some(state) = resolve_indexed_state(
            context,
            &target,
            owner_hint,
            access_locus,
            assign.span.clone(),
        ) {
            if state.owner_locus != access_locus {
                let generated_from = if owner_hint == Some(state.owner_locus.as_str()) {
                    "nested_place_block"
                } else {
                    "cross_locus_write_target"
                };
                push_remote_request(
                    context,
                    "write",
                    generated_from,
                    access_locus,
                    state,
                    target,
                    when,
                    assign.span.clone(),
                );
                return;
            }
        }
    }

    for target in indexed_targets_in_text(&assign.value.text) {
        let Some(state) = resolve_indexed_state(
            context,
            &target,
            owner_hint,
            access_locus,
            assign.value.span.clone(),
        ) else {
            continue;
        };
        if state.owner_locus == access_locus {
            continue;
        }
        let generated_from = if owner_hint == Some(state.owner_locus.as_str()) {
            "nested_place_block"
        } else {
            "cross_locus_read_expression"
        };
        push_remote_request(
            context,
            "read",
            generated_from,
            access_locus,
            state,
            target,
            when,
            assign.value.span.clone(),
        );
    }
}

fn push_unsupported_statement(
    context: &mut ElaborationContext,
    statement_kind: &str,
    detail: &str,
    span: SourceSpan,
) {
    context.diagnostics.push(diagnostic(
        "unsupported_surface_statement_for_elaboration",
        &format!("unsupported `{statement_kind}` statement in P-SURF-03 elaboration: {detail}"),
        span,
    ));
}

fn push_when_transition(
    context: &mut ElaborationContext,
    locus: &str,
    when: &SurfaceWhenBlock,
    kind: &str,
) {
    let transition_id = context.transition_id();
    context.core_ir.transitions.push(SurfaceCoreTransition {
        transition_id: transition_id.clone(),
        locus: locus.to_string(),
        trigger: when.event_name.clone(),
        kind: kind.to_string(),
        source_span: when.span.clone(),
    });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: transition_id,
        entity_kind: "transition".to_string(),
        span: when.span.clone(),
    });
}

fn push_remote_request(
    context: &mut ElaborationContext,
    request_kind: &str,
    generated_from: &str,
    requester_locus: &str,
    state: IndexedStateDecl,
    target: IndexedTarget,
    when: &SurfaceWhenBlock,
    span: SourceSpan,
) {
    let request_id = context.request_id();
    let required_failures = required_failures();
    let declared_failures = when.failure_row.clone();
    let failure_row_complete = required_failure_set()
        .is_subset(&declared_failures.iter().cloned().collect::<BTreeSet<_>>());
    context
        .core_ir
        .remote_requests
        .push(SurfaceCoreRemoteRequest {
            request_id: request_id.clone(),
            request_kind: request_kind.to_string(),
            requester_locus: requester_locus.to_string(),
            owner_locus: state.owner_locus.clone(),
            state_name: state.state_name,
            key_expr: target.key_expr,
            access_text: target.access_text,
            generated_from: generated_from.to_string(),
            required_failures,
            declared_failures,
            failure_row_complete,
            source_span: span.clone(),
        });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: request_id.clone(),
        entity_kind: "remote_request".to_string(),
        span: span.clone(),
    });

    let transition_kind = format!("generated_remote_{request_kind}_request");
    let transition_id = context.transition_id();
    context.core_ir.transitions.push(SurfaceCoreTransition {
        transition_id: transition_id.clone(),
        locus: state.owner_locus.clone(),
        trigger: when.event_name.clone(),
        kind: transition_kind,
        source_span: span.clone(),
    });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: transition_id,
        entity_kind: "transition".to_string(),
        span: span.clone(),
    });

    let edge_id = context.edge_id();
    context
        .core_ir
        .generated_edges
        .push(SurfaceCoreGeneratedEdge {
            edge_id: edge_id.clone(),
            edge_kind: if request_kind == "read" {
                "observe_request".to_string()
            } else {
                "remote_write_request".to_string()
            },
            from_locus: requester_locus.to_string(),
            to_locus: state.owner_locus,
            request_id,
            source_span: span.clone(),
        });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: edge_id,
        entity_kind: "generated_edge".to_string(),
        span: span.clone(),
    });

    if !failure_row_complete {
        context.diagnostics.push(diagnostic(
            "generated_failure_not_declared",
            "generated remote requests must be contained in a when failure row before admission",
            span,
        ));
    }
}

fn resolve_indexed_state(
    context: &mut ElaborationContext,
    target: &IndexedTarget,
    owner_hint: Option<&str>,
    access_locus: &str,
    span: SourceSpan,
) -> Option<IndexedStateDecl> {
    if let Some(owner_locus) = owner_hint {
        if let Some(state) = context
            .indexed_states
            .get(&(owner_locus.to_string(), target.state_name.clone()))
        {
            return Some(state.clone());
        }
    }
    if let Some(state) = context
        .indexed_states
        .get(&(access_locus.to_string(), target.state_name.clone()))
    {
        return Some(state.clone());
    }

    let matches = context
        .indexed_states
        .values()
        .filter(|state| state.state_name == target.state_name)
        .cloned()
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [state] => Some(state.clone()),
        [] => None,
        _ => {
            context.diagnostics.push(diagnostic(
                "ambiguous_indexed_state_reference",
                "indexed state reference is ambiguous without an owner locus",
                span,
            ));
            None
        }
    }
}

fn parse_indexed_target(text: &str) -> Option<IndexedTarget> {
    indexed_targets_in_text(text).into_iter().next()
}

fn indexed_targets_in_text(text: &str) -> Vec<IndexedTarget> {
    let normalized = text.split_whitespace().collect::<String>();
    let bytes = normalized.as_bytes();
    let mut targets = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'[' {
            index += 1;
            continue;
        }
        let mut name_start = index;
        while name_start > 0 && is_ident_byte(bytes[name_start - 1]) {
            name_start -= 1;
        }
        if name_start == index {
            index += 1;
            continue;
        }
        let Some(right) = normalized[index + 1..]
            .find(']')
            .map(|offset| index + 1 + offset)
        else {
            break;
        };
        let state_name = normalized[name_start..index].to_string();
        let key_expr = normalized[index + 1..right].to_string();
        if !state_name.is_empty() && !key_expr.is_empty() {
            let mut access_end = right + 1;
            if normalized[access_end..].starts_with('.') {
                access_end += 1;
                while access_end < bytes.len() && is_ident_byte(bytes[access_end]) {
                    access_end += 1;
                }
            }
            targets.push(IndexedTarget {
                state_name,
                key_expr,
                access_text: normalized[name_start..access_end].to_string(),
            });
        }
        index = right + 1;
    }
    targets
}

fn is_ident_byte(value: u8) -> bool {
    value.is_ascii_alphanumeric() || value == b'_'
}

fn required_failures() -> Vec<String> {
    REMOTE_REQUEST_FAILURES
        .iter()
        .map(|failure| (*failure).to_string())
        .collect()
}

fn required_failure_set() -> BTreeSet<String> {
    REMOTE_REQUEST_FAILURES
        .iter()
        .map(|failure| (*failure).to_string())
        .collect()
}

fn accepted_obligations() -> Vec<SurfaceCoreObligation> {
    vec![
        obligation(
            "surface_core_transitions_explicit",
            "Surface when blocks and generated remote requests are represented as Core IR transitions",
        ),
        obligation(
            "surface_core_remote_requests_explicit",
            "cross-locus indexed state access elaborates to explicit remote request rows",
        ),
        obligation(
            "surface_core_source_spans_preserved",
            "generated transitions, requests, and edges retain source spans",
        ),
        obligation(
            "surface_core_generated_failure_rows_contained",
            "generated remote request failures are checked against the surrounding when failure row",
        ),
    ]
}

fn residual_obligations() -> Vec<SurfaceCoreObligation> {
    vec![
        obligation(
            "auto_publish_observe_expansion_pending_p_surf_04",
            "P-SURF-04 must expand visibility declarations into generated communication rows",
        ),
        obligation(
            "role_admission_capability_grants_pending_p_surf_05",
            "P-SURF-05 must connect role admission to capability grants before authority claims",
        ),
        obligation(
            "runtime_hotplug_activation_pending_p_surf_06",
            "P-SURF-06 must route source patches through compatibility verdicts and activation cuts",
        ),
    ]
}

fn obligation(code: &str, detail: &str) -> SurfaceCoreObligation {
    SurfaceCoreObligation {
        code: code.to_string(),
        detail: detail.to_string(),
        source_span: None,
    }
}

fn diagnostic(code: &str, message: &str, span: SourceSpan) -> TextualMirDiagnostic {
    TextualMirDiagnostic {
        code: code.to_string(),
        message: message.to_string(),
        span,
    }
}

impl ElaborationContext {
    fn transition_id(&mut self) -> String {
        self.next_transition += 1;
        format!("tr-{:04}", self.next_transition)
    }

    fn request_id(&mut self) -> String {
        self.next_request += 1;
        format!("req-{:04}", self.next_request)
    }

    fn edge_id(&mut self) -> String {
        self.next_edge += 1;
        format!("edge-{:04}", self.next_edge)
    }
}
