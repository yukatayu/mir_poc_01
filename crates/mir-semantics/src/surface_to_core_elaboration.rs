use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use mir_ast::{
    surface_alpha::{
        SurfaceAssignStmt, SurfaceJoinStmt, SurfaceModule, SurfacePlaceItem,
        SurfaceRoleInstanceBlock, SurfaceStmt, SurfaceVisibilityDecl, SurfaceWhenBlock,
        parse_surface_mir_report, parse_surface_mir_report_path,
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
const VISIBILITY_FAILURE: &str = "VisibilityDenied";

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
    pub dependencies: Vec<SurfaceCoreDependency>,
    pub message_envelopes: Vec<SurfaceCoreMessageEnvelope>,
    pub publications: Vec<SurfaceCorePublication>,
    pub observations: Vec<SurfaceCoreObservation>,
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
pub struct SurfaceCoreDependency {
    pub dependency_id: String,
    pub dependency_kind: String,
    pub write_request_id: Option<String>,
    pub requester_locus: String,
    pub owner_locus: String,
    pub state_name: String,
    pub key_expr: String,
    pub field_name: Option<String>,
    pub access_text: String,
    pub generated_from: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreMessageEnvelope {
    pub envelope_id: String,
    pub request_id: String,
    pub envelope_kind: String,
    pub from_locus: String,
    pub to_locus: String,
    pub state_name: String,
    pub key_expr: String,
    pub field_name: Option<String>,
    pub visibility_channel: Option<String>,
    pub redaction_label: String,
    pub retention_scope: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCorePublication {
    pub publish_id: String,
    pub request_id: String,
    pub envelope_id: String,
    pub publisher_locus: String,
    pub channel: String,
    pub state_name: String,
    pub key_expr: String,
    pub field_name: Option<String>,
    pub redaction_label: String,
    pub retention_scope: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCoreObservation {
    pub observe_id: String,
    pub request_id: String,
    pub envelope_id: String,
    pub observer_locus: String,
    pub owner_locus: String,
    pub channel: String,
    pub state_name: String,
    pub key_expr: String,
    pub field_name: Option<String>,
    pub redaction_label: String,
    pub retention_scope: String,
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
    visible: Option<SurfaceVisibilityDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexedTarget {
    state_name: String,
    key_expr: String,
    field_name: Option<String>,
    access_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommunicationDecision {
    visibility_channel: Option<String>,
    generate_publish: bool,
    generate_observe: bool,
    visibility_failure_required: bool,
}

#[derive(Debug, Default)]
struct ElaborationContext {
    indexed_states: BTreeMap<(String, String), IndexedStateDecl>,
    diagnostics: Vec<TextualMirDiagnostic>,
    core_ir: SurfaceCoreIr,
    next_transition: usize,
    next_request: usize,
    next_dependency: usize,
    next_envelope: usize,
    next_publish: usize,
    next_observe: usize,
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
                    visible: state.visible.clone(),
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
            SurfaceStmt::Join(join) => push_join_transition(context, access_locus, join),
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
    let mut generated_write_request_id = None;
    if let Some(target) = parse_indexed_target(&assign.target_text)
        && let Some(state) = resolve_indexed_state(
            context,
            &target,
            owner_hint,
            access_locus,
            assign.span.clone(),
        )
        && state.owner_locus != access_locus
    {
        let generated_from = if owner_hint == Some(state.owner_locus.as_str()) {
            "nested_place_block"
        } else {
            "cross_locus_write_target"
        };
        let request_id = push_remote_request(
            context,
            "write",
            generated_from,
            access_locus,
            state,
            target,
            when,
            assign.span.clone(),
        );
        generated_write_request_id = Some(request_id);
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
        if let Some(write_request_id) = &generated_write_request_id {
            let generated_from = if owner_hint == Some(state.owner_locus.as_str()) {
                "nested_place_block_rhs"
            } else {
                "cross_locus_rhs_expression"
            };
            push_rhs_read_dependency(
                context,
                generated_from,
                write_request_id,
                access_locus,
                state,
                target,
                assign.value.span.clone(),
            );
        } else {
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
}

fn push_rhs_read_dependency(
    context: &mut ElaborationContext,
    generated_from: &str,
    write_request_id: &str,
    requester_locus: &str,
    state: IndexedStateDecl,
    target: IndexedTarget,
    span: SourceSpan,
) {
    let dependency_id = context.dependency_id();
    context.core_ir.dependencies.push(SurfaceCoreDependency {
        dependency_id: dependency_id.clone(),
        dependency_kind: "rhs_indexed_read".to_string(),
        write_request_id: Some(write_request_id.to_string()),
        requester_locus: requester_locus.to_string(),
        owner_locus: state.owner_locus,
        state_name: state.state_name,
        key_expr: target.key_expr,
        field_name: target.field_name,
        access_text: target.access_text,
        generated_from: generated_from.to_string(),
        source_span: span.clone(),
    });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: dependency_id,
        entity_kind: "dependency".to_string(),
        span,
    });
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

fn push_join_transition(
    context: &mut ElaborationContext,
    access_locus: &str,
    join: &SurfaceJoinStmt,
) {
    let transition_id = context.transition_id();
    context.core_ir.transitions.push(SurfaceCoreTransition {
        transition_id: transition_id.clone(),
        locus: access_locus.to_string(),
        trigger: format!(
            "join {} as {} via {}",
            join.target_place, join.role_ref, join.admission_place
        ),
        kind: "surface_role_join_admission".to_string(),
        source_span: join.span.clone(),
    });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: transition_id,
        entity_kind: "transition".to_string(),
        span: join.span.clone(),
    });
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

#[allow(clippy::too_many_arguments)]
fn push_remote_request(
    context: &mut ElaborationContext,
    request_kind: &str,
    generated_from: &str,
    requester_locus: &str,
    state: IndexedStateDecl,
    target: IndexedTarget,
    when: &SurfaceWhenBlock,
    span: SourceSpan,
) -> String {
    let request_id = context.request_id();
    let communication =
        communication_decision(context, request_kind, &state, &target, span.clone());
    let required_failures = required_failures(communication.visibility_failure_required);
    let declared_failures = when.failure_row.clone();
    let failure_row_complete = required_failure_set(communication.visibility_failure_required)
        .is_subset(&declared_failures.iter().cloned().collect::<BTreeSet<_>>());
    let owner_locus = state.owner_locus.clone();
    let state_name = state.state_name.clone();
    let key_expr = target.key_expr.clone();
    let field_name = target.field_name.clone();
    let access_text = target.access_text.clone();
    context
        .core_ir
        .remote_requests
        .push(SurfaceCoreRemoteRequest {
            request_id: request_id.clone(),
            request_kind: request_kind.to_string(),
            requester_locus: requester_locus.to_string(),
            owner_locus: owner_locus.clone(),
            state_name: state_name.clone(),
            key_expr: key_expr.clone(),
            access_text,
            generated_from: generated_from.to_string(),
            required_failures,
            declared_failures,
            failure_row_complete,
            source_span: span.clone(),
        });
    push_generated_communication(
        context,
        request_kind,
        requester_locus,
        &owner_locus,
        &state_name,
        &key_expr,
        field_name.as_deref(),
        &request_id,
        &communication,
        span.clone(),
    );
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
            request_id: request_id.clone(),
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
    request_id
}

fn communication_decision(
    context: &mut ElaborationContext,
    request_kind: &str,
    state: &IndexedStateDecl,
    target: &IndexedTarget,
    span: SourceSpan,
) -> CommunicationDecision {
    let request_span = span.clone();
    let visible_channel = visible_channel_for_target(context, state, target, span);
    match request_kind {
        "read" => {
            if state.visible.is_none() {
                context.diagnostics.push(diagnostic(
                    "private_field_auto_publish_rejected",
                    "cross-locus observe requires an explicit visible declaration",
                    request_span,
                ));
            }
            CommunicationDecision {
                visibility_channel: visible_channel,
                generate_publish: false,
                generate_observe: true,
                visibility_failure_required: true,
            }
        }
        "write" => CommunicationDecision {
            visibility_channel: visible_channel.clone(),
            generate_publish: visible_channel.is_some(),
            generate_observe: visible_channel.is_some(),
            visibility_failure_required: visible_channel.is_some(),
        },
        _ => CommunicationDecision {
            visibility_channel: None,
            generate_publish: false,
            generate_observe: false,
            visibility_failure_required: false,
        },
    }
}

fn visible_channel_for_target(
    context: &mut ElaborationContext,
    state: &IndexedStateDecl,
    target: &IndexedTarget,
    span: SourceSpan,
) -> Option<String> {
    let Some(visible) = &state.visible else {
        return None;
    };

    match (&target.field_name, visible.fields.as_slice()) {
        (Some(field), []) => {
            if is_private_field_name(field) {
                context.diagnostics.push(diagnostic(
                    "private_field_auto_publish_rejected",
                    "observer-safe auto publish cannot expose a private-looking field",
                    span,
                ));
                None
            } else {
                Some(visible.channel.clone())
            }
        }
        (Some(field), fields) if fields.iter().any(|visible_field| visible_field == field) => {
            if is_private_field_name(field) {
                context.diagnostics.push(diagnostic(
                    "private_field_auto_publish_rejected",
                    "observer-safe auto publish cannot expose a private-looking field",
                    span,
                ));
                None
            } else {
                Some(visible.channel.clone())
            }
        }
        (Some(_), _) => {
            context.diagnostics.push(diagnostic(
                "private_field_auto_publish_rejected",
                "only fields declared visible may generate observer-safe auto communication",
                span,
            ));
            None
        }
        (None, []) => Some(visible.channel.clone()),
        (None, _) => {
            context.diagnostics.push(diagnostic(
                "private_field_auto_publish_rejected",
                "whole-record observer-safe auto communication is blocked when only selected fields are visible",
                span,
            ));
            None
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn push_generated_communication(
    context: &mut ElaborationContext,
    request_kind: &str,
    requester_locus: &str,
    owner_locus: &str,
    state_name: &str,
    key_expr: &str,
    field_name: Option<&str>,
    request_id: &str,
    communication: &CommunicationDecision,
    span: SourceSpan,
) {
    let envelope_id = context.envelope_id();
    context
        .core_ir
        .message_envelopes
        .push(SurfaceCoreMessageEnvelope {
            envelope_id: envelope_id.clone(),
            request_id: request_id.to_string(),
            envelope_kind: format!("remote_{request_kind}"),
            from_locus: requester_locus.to_string(),
            to_locus: owner_locus.to_string(),
            state_name: state_name.to_string(),
            key_expr: key_expr.to_string(),
            field_name: field_name.map(str::to_string),
            visibility_channel: communication.visibility_channel.clone(),
            redaction_label: "observer_safe".to_string(),
            retention_scope: "report_local".to_string(),
            source_span: span.clone(),
        });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: envelope_id.clone(),
        entity_kind: "message_envelope".to_string(),
        span: span.clone(),
    });
    push_generated_edge(
        context,
        "message_envelope",
        requester_locus,
        owner_locus,
        request_id,
        span.clone(),
    );

    if let Some(channel) = &communication.visibility_channel {
        if communication.generate_publish {
            let publish_id = context.publish_id();
            context.core_ir.publications.push(SurfaceCorePublication {
                publish_id: publish_id.clone(),
                request_id: request_id.to_string(),
                envelope_id: envelope_id.clone(),
                publisher_locus: owner_locus.to_string(),
                channel: channel.clone(),
                state_name: state_name.to_string(),
                key_expr: key_expr.to_string(),
                field_name: field_name.map(str::to_string),
                redaction_label: "observer_safe".to_string(),
                retention_scope: "report_local".to_string(),
                source_span: span.clone(),
            });
            context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
                entity_id: publish_id,
                entity_kind: "publication".to_string(),
                span: span.clone(),
            });
            push_generated_edge(
                context,
                "auto_publish",
                owner_locus,
                "observer_safe:*",
                request_id,
                span.clone(),
            );
        }

        if communication.generate_observe {
            let observe_id = context.observe_id();
            let observer_locus = if request_kind == "read" {
                requester_locus
            } else {
                "observer_safe:*"
            };
            context.core_ir.observations.push(SurfaceCoreObservation {
                observe_id: observe_id.clone(),
                request_id: request_id.to_string(),
                envelope_id,
                observer_locus: observer_locus.to_string(),
                owner_locus: owner_locus.to_string(),
                channel: channel.clone(),
                state_name: state_name.to_string(),
                key_expr: key_expr.to_string(),
                field_name: field_name.map(str::to_string),
                redaction_label: "observer_safe".to_string(),
                retention_scope: "report_local".to_string(),
                source_span: span.clone(),
            });
            context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
                entity_id: observe_id,
                entity_kind: "observation".to_string(),
                span: span.clone(),
            });
            push_generated_edge(
                context,
                "auto_observe",
                owner_locus,
                observer_locus,
                request_id,
                span,
            );
        }
    }
}

fn push_generated_edge(
    context: &mut ElaborationContext,
    edge_kind: &str,
    from_locus: &str,
    to_locus: &str,
    request_id: &str,
    span: SourceSpan,
) {
    let edge_id = context.edge_id();
    context
        .core_ir
        .generated_edges
        .push(SurfaceCoreGeneratedEdge {
            edge_id: edge_id.clone(),
            edge_kind: edge_kind.to_string(),
            from_locus: from_locus.to_string(),
            to_locus: to_locus.to_string(),
            request_id: request_id.to_string(),
            source_span: span.clone(),
        });
    context.core_ir.source_spans.push(SurfaceCoreSourceSpan {
        entity_id: edge_id,
        entity_kind: "generated_edge".to_string(),
        span,
    });
}

fn resolve_indexed_state(
    context: &mut ElaborationContext,
    target: &IndexedTarget,
    owner_hint: Option<&str>,
    access_locus: &str,
    span: SourceSpan,
) -> Option<IndexedStateDecl> {
    if let Some(owner_locus) = owner_hint
        && let Some(state) = context
            .indexed_states
            .get(&(owner_locus.to_string(), target.state_name.clone()))
    {
        return Some(state.clone());
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
            let mut field_name = None;
            if normalized[access_end..].starts_with('.') {
                access_end += 1;
                let field_start = access_end;
                while access_end < bytes.len() && is_ident_byte(bytes[access_end]) {
                    access_end += 1;
                }
                if field_start < access_end {
                    field_name = Some(normalized[field_start..access_end].to_string());
                }
            }
            targets.push(IndexedTarget {
                state_name,
                key_expr,
                field_name,
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

fn required_failures(include_visibility: bool) -> Vec<String> {
    let mut failures = REMOTE_REQUEST_FAILURES
        .iter()
        .map(|failure| (*failure).to_string())
        .collect::<Vec<_>>();
    if include_visibility {
        failures.push(VISIBILITY_FAILURE.to_string());
    }
    failures
}

fn required_failure_set(include_visibility: bool) -> BTreeSet<String> {
    required_failures(include_visibility)
        .iter()
        .cloned()
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
        obligation(
            "surface_core_message_envelopes_explicit",
            "generated remote requests carry explicit MessageEnvelope rows",
        ),
        obligation(
            "surface_core_auto_publish_observe_explicit",
            "visible state access lowers to explicit publish / observe rows",
        ),
        obligation(
            "surface_core_private_fields_not_auto_published",
            "private or non-visible fields are blocked from observer-safe auto communication",
        ),
    ]
}

fn residual_obligations() -> Vec<SurfaceCoreObligation> {
    vec![
        obligation(
            "role_admission_runtime_integration_pending_after_p_surf_05",
            "P-SURF-05 supplies report-level admission evidence; later runtime integration must attach grants to executable requests",
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

    fn dependency_id(&mut self) -> String {
        self.next_dependency += 1;
        format!("dep-{:04}", self.next_dependency)
    }

    fn envelope_id(&mut self) -> String {
        self.next_envelope += 1;
        format!("env-{:04}", self.next_envelope)
    }

    fn publish_id(&mut self) -> String {
        self.next_publish += 1;
        format!("pub-{:04}", self.next_publish)
    }

    fn observe_id(&mut self) -> String {
        self.next_observe += 1;
        format!("obs-{:04}", self.next_observe)
    }

    fn edge_id(&mut self) -> String {
        self.next_edge += 1;
        format!("edge-{:04}", self.next_edge)
    }
}

fn is_private_field_name(field_name: &str) -> bool {
    let normalized = field_name.to_ascii_lowercase();
    normalized == "secret"
        || normalized == "private"
        || normalized == "password"
        || normalized == "credential"
        || normalized == "token"
        || normalized.starts_with("secret_")
        || normalized.starts_with("private_")
        || normalized.starts_with("password_")
        || normalized.starts_with("credential_")
        || normalized.ends_with("_secret")
        || normalized.ends_with("_private")
        || normalized.ends_with("_password")
        || normalized.ends_with("_credential")
        || normalized.ends_with("_token")
}
