use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use mir_ast::{
    surface_alpha::{
        SurfaceModule, SurfacePlaceItem, SurfaceRoleInstanceBlock, SurfaceStmt,
        parse_surface_mir_report, parse_surface_mir_report_path,
    },
    textual_alpha::{SourceSpan, TextualMirDiagnostic},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedStateSemanticsReport {
    pub accepted: bool,
    pub module_path: Option<String>,
    pub indexed_states: Vec<IndexedStateDecl>,
    pub access_checks: Vec<IndexedStateAccessCheck>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub accepted_obligations: Vec<IndexedStateObligation>,
    pub residual_obligations: Vec<IndexedStateObligation>,
    pub source_authority: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedStateDecl {
    pub owner_locus: String,
    pub state_name: String,
    pub key_name: String,
    pub keyspace_type: String,
    pub value_type: String,
    pub visible_fields: Vec<String>,
    pub authority_model: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedStateAccessCheck {
    pub state_name: String,
    pub owner_locus: String,
    pub access_locus: String,
    pub key_expr: String,
    pub access_kind: String,
    pub accepted: bool,
    pub reason_code: Option<String>,
    pub key_authority_granted: bool,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedStateObligation {
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexedTarget {
    state_name: String,
    key_expr: String,
}

#[derive(Debug, Default)]
struct CheckContext {
    indexed_states: BTreeMap<(String, String), IndexedStateDecl>,
    diagnostics: Vec<TextualMirDiagnostic>,
    access_checks: Vec<IndexedStateAccessCheck>,
}

#[derive(Debug, Default)]
struct EventState {
    stale_keys: BTreeSet<String>,
    retained_keys: BTreeSet<String>,
}

pub fn check_surface_indexed_state_source(source: &str) -> IndexedStateSemanticsReport {
    let parse_report = parse_surface_mir_report(source);
    match parse_report.module {
        Some(module) => check_surface_indexed_state_module(module),
        None => IndexedStateSemanticsReport {
            accepted: false,
            module_path: None,
            indexed_states: Vec::new(),
            access_checks: Vec::new(),
            diagnostics: parse_report.diagnostics,
            accepted_obligations: Vec::new(),
            residual_obligations: residual_obligations(),
            source_authority: ".mir".to_string(),
            final_public_api_frozen: false,
        },
    }
}

pub fn check_surface_indexed_state_path(path: impl AsRef<Path>) -> IndexedStateSemanticsReport {
    let parse_report = parse_surface_mir_report_path(path);
    match parse_report.module {
        Some(module) => check_surface_indexed_state_module(module),
        None => IndexedStateSemanticsReport {
            accepted: false,
            module_path: None,
            indexed_states: Vec::new(),
            access_checks: Vec::new(),
            diagnostics: parse_report.diagnostics,
            accepted_obligations: Vec::new(),
            residual_obligations: residual_obligations(),
            source_authority: ".mir".to_string(),
            final_public_api_frozen: false,
        },
    }
}

pub fn indexed_state_diagnostic_codes(report: &IndexedStateSemanticsReport) -> Vec<String> {
    report
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.clone())
        .collect()
}

pub fn check_surface_indexed_state_module(module: SurfaceModule) -> IndexedStateSemanticsReport {
    let module_path = module.module_path.clone();
    let mut context = CheckContext::default();

    collect_indexed_states(&module, &mut context);
    check_accesses(&module, &mut context);

    let indexed_states = context.indexed_states.values().cloned().collect::<Vec<_>>();
    let accepted_obligations = if context.diagnostics.is_empty() {
        vec![
            IndexedStateObligation {
                code: "indexed_state_owner_locus_recorded".to_string(),
                detail: "indexed state owner is the declaring place block".to_string(),
            },
            IndexedStateObligation {
                code: "indexed_state_key_not_authority".to_string(),
                detail: "keyspace is represented separately from authority".to_string(),
            },
        ]
    } else {
        Vec::new()
    };

    IndexedStateSemanticsReport {
        accepted: context.diagnostics.is_empty(),
        module_path: Some(module_path),
        indexed_states,
        access_checks: context.access_checks,
        diagnostics: context.diagnostics,
        accepted_obligations,
        residual_obligations: residual_obligations(),
        source_authority: ".mir".to_string(),
        final_public_api_frozen: false,
    }
}

fn collect_indexed_states(module: &SurfaceModule, context: &mut CheckContext) {
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::State(state) = item else {
                continue;
            };
            let Some(index) = &state.index else {
                continue;
            };
            if index.key_type_text != "Participant" {
                context.diagnostics.push(diagnostic(
                    "unsupported_indexed_state_keyspace",
                    "Surface alpha indexed state only supports Participant keyspace",
                    index.span.clone(),
                ));
                continue;
            }
            let key = (block.place_ref.clone(), state.state_name.clone());
            if context.indexed_states.contains_key(&key) {
                context.diagnostics.push(diagnostic(
                    "ambiguous_indexed_state_name",
                    "indexed state names must be unique within an owner locus",
                    state.span.clone(),
                ));
                continue;
            }
            context.indexed_states.insert(
                key,
                IndexedStateDecl {
                    owner_locus: block.place_ref.clone(),
                    state_name: state.state_name.clone(),
                    key_name: index.name.clone(),
                    keyspace_type: index.key_type_text.clone(),
                    value_type: state.value_type_text.clone(),
                    visible_fields: state
                        .visible
                        .as_ref()
                        .map(|visible| visible.fields.clone())
                        .unwrap_or_default(),
                    authority_model: "owner_locus_or_explicit_capability".to_string(),
                    span: state.span.clone(),
                },
            );
        }
    }
}

fn check_accesses(module: &SurfaceModule, context: &mut CheckContext) {
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::When(when) = item else {
                continue;
            };
            let mut event_state = EventState::default();
            check_statements(
                &when.body,
                &block.place_ref,
                Some(&block.place_ref),
                false,
                context,
                &mut event_state,
            );
        }
    }
    for block in &module.role_instance_blocks {
        check_role_instance_block(block, context);
    }
}

fn check_role_instance_block(block: &SurfaceRoleInstanceBlock, context: &mut CheckContext) {
    let access_locus = format!("role:{}", block.role_ref);
    for when in &block.whens {
        let mut event_state = EventState::default();
        check_statements(
            &when.body,
            &access_locus,
            None,
            false,
            context,
            &mut event_state,
        );
    }
}

fn check_statements(
    body: &[SurfaceStmt],
    access_locus: &str,
    owner_hint: Option<&str>,
    owner_hint_is_nested: bool,
    context: &mut CheckContext,
    event_state: &mut EventState,
) {
    for stmt in body {
        match stmt {
            SurfaceStmt::Assign(assign) => {
                if let Some(target) = parse_indexed_target(&assign.target_text) {
                    check_indexed_write(
                        target,
                        access_locus,
                        owner_hint,
                        owner_hint_is_nested,
                        assign.span.clone(),
                        context,
                        event_state,
                    );
                }
            }
            SurfaceStmt::NestedPlaceBlock(block) => {
                check_statements(
                    &block.body,
                    access_locus,
                    Some(&block.place_ref),
                    true,
                    context,
                    event_state,
                );
            }
            SurfaceStmt::Raw(raw) => {
                check_raw_lifecycle_stmt(
                    &raw.text,
                    raw.span.clone(),
                    access_locus,
                    owner_hint,
                    owner_hint_is_nested,
                    context,
                    event_state,
                );
            }
            _ => {}
        }
    }
}

fn check_indexed_write(
    target: IndexedTarget,
    access_locus: &str,
    owner_hint: Option<&str>,
    owner_hint_is_nested: bool,
    span: SourceSpan,
    context: &mut CheckContext,
    event_state: &mut EventState,
) {
    let Some(state) =
        resolve_indexed_state(context, &target, owner_hint, access_locus, span.clone())
    else {
        return;
    };
    if event_state.stale_keys.contains(&target.key_expr) {
        push_rejected_access(
            context,
            state,
            access_locus,
            target.key_expr,
            "write",
            "stale_indexed_state_key",
            "indexed state key is stale in the current membership/incarnation epoch",
            span,
        );
        return;
    }
    if owner_hint_is_nested && access_locus != state.owner_locus {
        push_rejected_access(
            context,
            state,
            access_locus,
            target.key_expr,
            "write",
            "indexed_state_nested_place_requires_generated_request",
            "nested place block access must elaborate to an owner-directed generated request before it can write indexed state",
            span,
        );
        return;
    }
    if access_locus == state.owner_locus {
        context.access_checks.push(IndexedStateAccessCheck {
            state_name: state.state_name,
            owner_locus: state.owner_locus,
            access_locus: access_locus.to_string(),
            key_expr: target.key_expr,
            access_kind: "write".to_string(),
            accepted: true,
            reason_code: None,
            key_authority_granted: false,
            span,
        });
        return;
    }
    push_rejected_access(
        context,
        state,
        access_locus,
        target.key_expr,
        "write",
        "indexed_state_key_is_not_authority",
        "indexed state key does not grant write authority",
        span,
    );
}

fn check_raw_lifecycle_stmt(
    text: &str,
    span: SourceSpan,
    access_locus: &str,
    owner_hint: Option<&str>,
    owner_hint_is_nested: bool,
    context: &mut CheckContext,
    event_state: &mut EventState,
) {
    let normalized = normalize_text(text);
    if let Some(key) = normalized.strip_prefix("leave ") {
        event_state.stale_keys.insert(key.trim().to_string());
        return;
    }
    if let Some(key) = normalized.strip_prefix("retain savepoint ") {
        event_state.retained_keys.insert(key.trim().to_string());
        return;
    }
    if let Some(target_text) = normalized.strip_prefix("compact ") {
        let Some(target) = parse_indexed_target(target_text) else {
            return;
        };
        let Some(state) =
            resolve_indexed_state(context, &target, owner_hint, access_locus, span.clone())
        else {
            return;
        };
        if owner_hint_is_nested && access_locus != state.owner_locus {
            push_rejected_access(
                context,
                state,
                access_locus,
                target.key_expr,
                "compact",
                "indexed_state_nested_place_requires_generated_request",
                "nested place block access must elaborate to an owner-directed generated request before it can compact indexed state",
                span,
            );
            return;
        }
        if event_state.retained_keys.contains(&target.key_expr) {
            push_rejected_access(
                context,
                state,
                access_locus,
                target.key_expr,
                "compact",
                "indexed_state_compaction_blocked_by_retained_evidence",
                "indexed state compaction is blocked by retained savepoint evidence in the P-SURF-02 semantics floor",
                span,
            );
        }
    }
}

fn resolve_indexed_state(
    context: &mut CheckContext,
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

fn push_rejected_access(
    context: &mut CheckContext,
    state: IndexedStateDecl,
    access_locus: &str,
    key_expr: String,
    access_kind: &str,
    reason_code: &str,
    message: &str,
    span: SourceSpan,
) {
    context.access_checks.push(IndexedStateAccessCheck {
        state_name: state.state_name.clone(),
        owner_locus: state.owner_locus,
        access_locus: access_locus.to_string(),
        key_expr,
        access_kind: access_kind.to_string(),
        accepted: false,
        reason_code: Some(reason_code.to_string()),
        key_authority_granted: false,
        span: span.clone(),
    });
    context
        .diagnostics
        .push(diagnostic(reason_code, message, span));
}

fn parse_indexed_target(text: &str) -> Option<IndexedTarget> {
    let normalized = normalize_text(text).replace(' ', "");
    let left = normalized.find('[')?;
    let right = normalized[left + 1..].find(']')? + left + 1;
    let state_name = normalized[..left].to_string();
    if state_name.is_empty() {
        return None;
    }
    let key_expr = normalized[left + 1..right].to_string();
    if key_expr.is_empty() {
        return None;
    }
    Some(IndexedTarget {
        state_name,
        key_expr,
    })
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn diagnostic(code: &str, message: &str, span: SourceSpan) -> TextualMirDiagnostic {
    TextualMirDiagnostic {
        code: code.to_string(),
        message: message.to_string(),
        span,
    }
}

fn residual_obligations() -> Vec<IndexedStateObligation> {
    vec![
        IndexedStateObligation {
            code: "cross_locus_generated_request_elaboration_pending".to_string(),
            detail: "P-SURF-03 must lower remote indexed access into Core obligations".to_string(),
        },
        IndexedStateObligation {
            code: "runtime_membership_epoch_carrier_pending".to_string(),
            detail: "P-SURF-07/P-SURF-08 must carry active/tombstoned keys in runtime/devtools"
                .to_string(),
        },
    ]
}
