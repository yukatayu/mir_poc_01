use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use mir_ast::{
    surface_alpha::{
        SurfaceAssignStmt, SurfaceModule, SurfacePlaceItem, SurfaceRoleDecl,
        SurfaceRoleInstanceBlock, SurfaceStmt, parse_surface_mir_report,
        parse_surface_mir_report_path,
    },
    textual_alpha::{SourceSpan, TextualMirDiagnostic},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoleAdmissionReport {
    pub accepted: bool,
    pub module_path: Option<String>,
    pub role_claims: Vec<SurfaceRoleClaim>,
    pub admission_requests: Vec<SurfaceAdmissionRequest>,
    pub admission_verdicts: Vec<SurfaceAdmissionVerdict>,
    pub capability_grants: Vec<SurfaceCapabilityGrant>,
    pub admission_witnesses: Vec<SurfaceAdmissionWitness>,
    pub authority_checks: Vec<SurfaceRoleAuthorityCheck>,
    pub stale_rejections: Vec<SurfaceStaleMembershipRejection>,
    pub optional_hash_bindings: Vec<SurfaceOptionalHashBinding>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub accepted_obligations: Vec<SurfaceRoleAdmissionObligation>,
    pub residual_obligations: Vec<SurfaceRoleAdmissionObligation>,
    pub source_authority: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoleClaim {
    pub claim_id: String,
    pub principal: String,
    pub claimed_role: String,
    pub supported_features: Vec<String>,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAdmissionRequest {
    pub request_id: String,
    pub principal: String,
    pub claimed_role: String,
    pub target_place: String,
    pub admission_locus: String,
    pub requested_capabilities: Vec<String>,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAdmissionVerdict {
    pub verdict_id: String,
    pub request_id: String,
    pub verdict: String,
    pub principal: String,
    pub admitted_role: String,
    pub target_place: String,
    pub membership_epoch: String,
    pub member_incarnation: String,
    pub granted_capabilities: Vec<String>,
    pub admission_witness_ref: String,
    pub failure_or_reason: Option<String>,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCapabilityGrant {
    pub grant_id: String,
    pub verdict_id: String,
    pub principal: String,
    pub role: String,
    pub target_place: String,
    pub capability: String,
    pub membership_epoch: String,
    pub member_incarnation: String,
    pub admission_witness_ref: String,
    pub authority_source: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAdmissionWitness {
    pub witness_id: String,
    pub verdict_id: String,
    pub principal: String,
    pub role: String,
    pub target_place: String,
    pub redaction_label: String,
    pub retention_scope: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoleAuthorityCheck {
    pub check_id: String,
    pub principal: String,
    pub claimed_role: String,
    pub target_place: String,
    pub operation: String,
    pub required_capability: String,
    pub accepted: bool,
    pub authority_source: Option<String>,
    pub reason_code: Option<String>,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceStaleMembershipRejection {
    pub rejection_id: String,
    pub principal: String,
    pub claimed_role: String,
    pub target_place: String,
    pub observed_membership_epoch: String,
    pub observed_member_incarnation: String,
    pub reason_code: String,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceOptionalHashBinding {
    pub binding_id: String,
    pub principal: String,
    pub claimed_role: String,
    pub package_hash: String,
    pub runtime_hash: String,
    pub semantic_safety_proof: bool,
    pub source_span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoleAdmissionObligation {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GrantKey {
    principal: String,
    target_place: String,
    capability: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StaleGrantFence {
    principal: String,
    target_place: String,
}

#[derive(Debug, Default)]
struct RoleAdmissionContext {
    role_supports: BTreeMap<String, Vec<String>>,
    indexed_states: BTreeMap<(String, String), IndexedStateDecl>,
    claims: Vec<SurfaceRoleClaim>,
    requests: Vec<SurfaceAdmissionRequest>,
    verdicts: Vec<SurfaceAdmissionVerdict>,
    grants: Vec<SurfaceCapabilityGrant>,
    witnesses: Vec<SurfaceAdmissionWitness>,
    authority_checks: Vec<SurfaceRoleAuthorityCheck>,
    stale_rejections: Vec<SurfaceStaleMembershipRejection>,
    hash_bindings: Vec<SurfaceOptionalHashBinding>,
    diagnostics: Vec<TextualMirDiagnostic>,
    active_grants: BTreeSet<GrantKey>,
    stale_grant_fences: BTreeSet<StaleGrantFence>,
    next_claim: usize,
    next_request: usize,
    next_verdict: usize,
    next_grant: usize,
    next_witness: usize,
    next_check: usize,
    next_rejection: usize,
    next_binding: usize,
}

pub fn check_surface_role_admission_source(source: &str) -> SurfaceRoleAdmissionReport {
    let parse_report = parse_surface_mir_report(source);
    match parse_report.module {
        Some(module) => check_surface_role_admission_module(module),
        None => rejected_report(parse_report.diagnostics),
    }
}

pub fn check_surface_role_admission_path(path: impl AsRef<Path>) -> SurfaceRoleAdmissionReport {
    let parse_report = parse_surface_mir_report_path(path);
    match parse_report.module {
        Some(module) => check_surface_role_admission_module(module),
        None => rejected_report(parse_report.diagnostics),
    }
}

pub fn check_surface_role_admission_module(module: SurfaceModule) -> SurfaceRoleAdmissionReport {
    let module_path = module.module_path.clone();
    let mut context = RoleAdmissionContext::default();
    collect_roles(&module.roles, &mut context);
    collect_indexed_states(&module, &mut context);
    check_role_instance_blocks(&module.role_instance_blocks, &mut context);

    let accepted = context.diagnostics.is_empty();
    let accepted_obligations = if accepted {
        accepted_obligations()
    } else {
        Vec::new()
    };

    SurfaceRoleAdmissionReport {
        accepted,
        module_path: Some(module_path),
        role_claims: context.claims,
        admission_requests: context.requests,
        admission_verdicts: context.verdicts,
        capability_grants: context.grants,
        admission_witnesses: context.witnesses,
        authority_checks: context.authority_checks,
        stale_rejections: context.stale_rejections,
        optional_hash_bindings: context.hash_bindings,
        diagnostics: context.diagnostics,
        accepted_obligations,
        residual_obligations: Vec::new(),
        source_authority: ".mir".to_string(),
        final_public_api_frozen: false,
    }
}

pub fn surface_role_admission_diagnostic_codes(report: &SurfaceRoleAdmissionReport) -> Vec<String> {
    report
        .diagnostics
        .iter()
        .map(|row| row.code.clone())
        .collect()
}

fn rejected_report(diagnostics: Vec<TextualMirDiagnostic>) -> SurfaceRoleAdmissionReport {
    SurfaceRoleAdmissionReport {
        accepted: false,
        module_path: None,
        role_claims: Vec::new(),
        admission_requests: Vec::new(),
        admission_verdicts: Vec::new(),
        capability_grants: Vec::new(),
        admission_witnesses: Vec::new(),
        authority_checks: Vec::new(),
        stale_rejections: Vec::new(),
        optional_hash_bindings: Vec::new(),
        diagnostics,
        accepted_obligations: Vec::new(),
        residual_obligations: Vec::new(),
        source_authority: ".mir".to_string(),
        final_public_api_frozen: false,
    }
}

fn collect_roles(roles: &[SurfaceRoleDecl], context: &mut RoleAdmissionContext) {
    for role in roles {
        context
            .role_supports
            .insert(role.role_name.clone(), role.supports.clone());
    }
}

fn collect_indexed_states(module: &SurfaceModule, context: &mut RoleAdmissionContext) {
    for block in &module.place_blocks {
        for item in &block.items {
            let SurfacePlaceItem::State(state) = item else {
                continue;
            };
            if state.index.is_none() {
                continue;
            }
            let key = (block.place_ref.clone(), state.state_name.clone());
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

fn check_role_instance_blocks(
    blocks: &[SurfaceRoleInstanceBlock],
    context: &mut RoleAdmissionContext,
) {
    for block in blocks {
        context.active_grants.clear();
        context.stale_grant_fences.clear();
        let supports = context
            .role_supports
            .get(&block.role_ref)
            .cloned()
            .unwrap_or_default();
        let claim_id = context.claim_id();
        context.claims.push(SurfaceRoleClaim {
            claim_id,
            principal: block.instance_ref.clone(),
            claimed_role: block.role_ref.clone(),
            supported_features: supports,
            source_span: block.span.clone(),
        });
        for when in &block.whens {
            check_statements(
                &when.body,
                &block.instance_ref,
                &block.role_ref,
                None,
                context,
            );
        }
    }
}

fn check_statements(
    body: &[SurfaceStmt],
    principal: &str,
    claimed_role: &str,
    owner_hint: Option<&str>,
    context: &mut RoleAdmissionContext,
) {
    for stmt in body {
        match stmt {
            SurfaceStmt::Join(join) => {
                push_admission(
                    principal,
                    claimed_role,
                    &join.target_place,
                    &join.admission_place,
                    join.span.clone(),
                    context,
                );
            }
            SurfaceStmt::NestedPlaceBlock(block) => check_statements(
                &block.body,
                principal,
                claimed_role,
                Some(&block.place_ref),
                context,
            ),
            SurfaceStmt::Assign(assign) => {
                check_assign(assign, principal, claimed_role, owner_hint, context);
            }
            SurfaceStmt::Raw(raw) => {
                check_raw_statement(
                    &raw.text,
                    principal,
                    claimed_role,
                    raw.span.clone(),
                    context,
                );
            }
            SurfaceStmt::Require(_) | SurfaceStmt::Grant(_) | SurfaceStmt::Publish(_) => {}
        }
    }
}

fn push_admission(
    principal: &str,
    claimed_role: &str,
    target_place: &str,
    admission_locus: &str,
    source_span: SourceSpan,
    context: &mut RoleAdmissionContext,
) {
    let request_id = context.request_id();
    let verdict_id = context.verdict_id();
    let witness_id = context.witness_id();
    let membership_epoch = format!("epoch-{:04}", context.next_verdict);
    let member_incarnation = format!("incarnation-{:04}", context.next_verdict);
    let mut requested_capabilities = vec![
        format!("Member({target_place})"),
        format!("ObserveState({target_place})"),
        format!("WriteState({target_place})"),
    ];
    if let Some(supports) = context.role_supports.get(claimed_role) {
        requested_capabilities.extend(supports.iter().map(|support| format!("support:{support}")));
    }
    let granted_capabilities = vec![
        format!("Member({target_place})"),
        format!("ObserveState({target_place})"),
        format!("WriteState({target_place})"),
    ];

    context.requests.push(SurfaceAdmissionRequest {
        request_id: request_id.clone(),
        principal: principal.to_string(),
        claimed_role: claimed_role.to_string(),
        target_place: target_place.to_string(),
        admission_locus: admission_locus.to_string(),
        requested_capabilities,
        source_span: source_span.clone(),
    });
    context.verdicts.push(SurfaceAdmissionVerdict {
        verdict_id: verdict_id.clone(),
        request_id: request_id.clone(),
        verdict: "accepted".to_string(),
        principal: principal.to_string(),
        admitted_role: claimed_role.to_string(),
        target_place: target_place.to_string(),
        membership_epoch: membership_epoch.clone(),
        member_incarnation: member_incarnation.clone(),
        granted_capabilities: granted_capabilities.clone(),
        admission_witness_ref: witness_id.clone(),
        failure_or_reason: None,
        source_span: source_span.clone(),
    });
    context.witnesses.push(SurfaceAdmissionWitness {
        witness_id: witness_id.clone(),
        verdict_id: verdict_id.clone(),
        principal: principal.to_string(),
        role: claimed_role.to_string(),
        target_place: target_place.to_string(),
        redaction_label: "observer_safe".to_string(),
        retention_scope: "role_admission_report".to_string(),
        source_span: source_span.clone(),
    });

    for capability in granted_capabilities {
        let grant_id = context.grant_id();
        context.active_grants.insert(GrantKey {
            principal: principal.to_string(),
            target_place: target_place.to_string(),
            capability: capability.clone(),
        });
        context.grants.push(SurfaceCapabilityGrant {
            grant_id,
            verdict_id: verdict_id.clone(),
            principal: principal.to_string(),
            role: claimed_role.to_string(),
            target_place: target_place.to_string(),
            capability,
            membership_epoch: membership_epoch.clone(),
            member_incarnation: member_incarnation.clone(),
            admission_witness_ref: witness_id.clone(),
            authority_source: "admission_grant".to_string(),
            source_span: source_span.clone(),
        });
    }
}

fn check_assign(
    assign: &SurfaceAssignStmt,
    principal: &str,
    claimed_role: &str,
    owner_hint: Option<&str>,
    context: &mut RoleAdmissionContext,
) {
    let Some(target) = parse_indexed_target(&assign.target_text) else {
        return;
    };
    let Some(state) = resolve_indexed_state(context, &target, owner_hint) else {
        return;
    };
    let required_capability = format!("WriteState({})", state.owner_locus);
    let grant_key = GrantKey {
        principal: principal.to_string(),
        target_place: state.owner_locus.clone(),
        capability: required_capability.clone(),
    };
    let stale_fenced = context.stale_grant_fences.contains(&StaleGrantFence {
        principal: principal.to_string(),
        target_place: state.owner_locus.clone(),
    });
    let has_grant = context.active_grants.contains(&grant_key);
    let accepted = has_grant && !stale_fenced;
    let reason_code = if stale_fenced {
        Some("stale_membership".to_string())
    } else if !has_grant {
        Some("missing_capability_grant".to_string())
    } else {
        None
    };
    let check_id = context.check_id();
    context.authority_checks.push(SurfaceRoleAuthorityCheck {
        check_id,
        principal: principal.to_string(),
        claimed_role: claimed_role.to_string(),
        target_place: state.owner_locus.clone(),
        operation: "write_indexed_state".to_string(),
        required_capability,
        accepted,
        authority_source: accepted.then(|| "admission_grant".to_string()),
        reason_code: reason_code.clone(),
        source_span: assign.span.clone(),
    });
    if stale_fenced {
        context.diagnostics.push(diagnostic(
            "stale_membership_authority_rejected",
            "stale membership fence rejects authority derived from an earlier grant",
            assign.span.clone(),
        ));
    } else if !accepted {
        context.diagnostics.push(diagnostic(
            "role_claim_without_capability_grant",
            "role claim does not grant authority; missing admission capability grant",
            assign.span.clone(),
        ));
    }
}

fn check_raw_statement(
    text: &str,
    principal: &str,
    claimed_role: &str,
    span: SourceSpan,
    context: &mut RoleAdmissionContext,
) {
    let parts = text.split_whitespace().collect::<Vec<_>>();
    if parts.first() == Some(&"stale_message") {
        let target_place = parts.get(1).copied().unwrap_or("unknown").to_string();
        let observed_membership_epoch = parts.get(2).copied().unwrap_or("unknown").to_string();
        let observed_member_incarnation = parts.get(3).copied().unwrap_or("unknown").to_string();
        let rejection_id = context.rejection_id();
        context
            .stale_rejections
            .push(SurfaceStaleMembershipRejection {
                rejection_id,
                principal: principal.to_string(),
                claimed_role: claimed_role.to_string(),
                target_place: target_place.clone(),
                observed_membership_epoch,
                observed_member_incarnation,
                reason_code: "stale_membership".to_string(),
                source_span: span.clone(),
            });
        context
            .active_grants
            .retain(|grant| !(grant.principal == principal && grant.target_place == target_place));
        context.stale_grant_fences.insert(StaleGrantFence {
            principal: principal.to_string(),
            target_place,
        });
        context.diagnostics.push(diagnostic(
            "stale_membership_message_rejected",
            "stale membership epoch/incarnation cannot resurrect authority",
            span,
        ));
        return;
    }

    if parts.first() == Some(&"bind_hash") {
        let package_hash = value_after(&parts, "package").unwrap_or("unknown");
        let runtime_hash = value_after(&parts, "runtime").unwrap_or("unknown");
        let binding_id = context.binding_id();
        context.hash_bindings.push(SurfaceOptionalHashBinding {
            binding_id,
            principal: principal.to_string(),
            claimed_role: claimed_role.to_string(),
            package_hash: package_hash.to_string(),
            runtime_hash: runtime_hash.to_string(),
            semantic_safety_proof: false,
            source_span: span,
        });
    }
}

fn value_after<'a>(parts: &'a [&str], needle: &str) -> Option<&'a str> {
    parts
        .iter()
        .position(|part| *part == needle)
        .and_then(|index| parts.get(index + 1))
        .copied()
}

fn resolve_indexed_state(
    context: &RoleAdmissionContext,
    target: &IndexedTarget,
    owner_hint: Option<&str>,
) -> Option<IndexedStateDecl> {
    if let Some(owner_hint) = owner_hint {
        let key = (owner_hint.to_string(), target.state_name.clone());
        if let Some(state) = context.indexed_states.get(&key) {
            return Some(state.clone());
        }
    }
    let mut matches = context
        .indexed_states
        .values()
        .filter(|state| state.state_name == target.state_name);
    let first = matches.next()?.clone();
    if matches.next().is_some() {
        None
    } else {
        Some(first)
    }
}

fn parse_indexed_target(text: &str) -> Option<IndexedTarget> {
    indexed_targets_in_text(text).into_iter().next()
}

fn indexed_targets_in_text(text: &str) -> Vec<IndexedTarget> {
    let normalized = text.split_whitespace().collect::<String>();
    let bytes = normalized.as_bytes();
    let mut targets = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if !is_ident_start(bytes[index]) {
            index += 1;
            continue;
        }
        let name_start = index;
        index += 1;
        while index < bytes.len() && is_ident_byte(bytes[index]) {
            index += 1;
        }
        if index >= bytes.len() || bytes[index] != b'[' {
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

fn is_ident_start(value: u8) -> bool {
    value.is_ascii_alphabetic() || value == b'_'
}

fn is_ident_byte(value: u8) -> bool {
    value.is_ascii_alphanumeric() || value == b'_'
}

fn accepted_obligations() -> Vec<SurfaceRoleAdmissionObligation> {
    vec![
        obligation(
            "surface_role_claim_not_authority",
            "role claims are recorded separately from authority-bearing capability grants",
        ),
        obligation(
            "surface_admission_requests_explicit",
            "join statements lower to explicit admission request and verdict rows",
        ),
        obligation(
            "surface_capability_grants_authority_source",
            "write authority comes from admission capability grants, not role strings",
        ),
        obligation(
            "surface_stale_membership_rejected",
            "stale membership epoch/incarnation rows reject instead of resurrecting authority",
        ),
        obligation(
            "surface_hash_binding_metadata_only",
            "package/runtime hash binding is metadata and not semantic safety proof",
        ),
    ]
}

fn obligation(code: &str, detail: &str) -> SurfaceRoleAdmissionObligation {
    SurfaceRoleAdmissionObligation {
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

impl RoleAdmissionContext {
    fn claim_id(&mut self) -> String {
        self.next_claim += 1;
        format!("claim-{:04}", self.next_claim)
    }

    fn request_id(&mut self) -> String {
        self.next_request += 1;
        format!("adm-req-{:04}", self.next_request)
    }

    fn verdict_id(&mut self) -> String {
        self.next_verdict += 1;
        format!("adm-verdict-{:04}", self.next_verdict)
    }

    fn grant_id(&mut self) -> String {
        self.next_grant += 1;
        format!("grant-{:04}", self.next_grant)
    }

    fn witness_id(&mut self) -> String {
        self.next_witness += 1;
        format!("admission-witness-{:04}", self.next_witness)
    }

    fn check_id(&mut self) -> String {
        self.next_check += 1;
        format!("authority-check-{:04}", self.next_check)
    }

    fn rejection_id(&mut self) -> String {
        self.next_rejection += 1;
        format!("stale-reject-{:04}", self.next_rejection)
    }

    fn binding_id(&mut self) -> String {
        self.next_binding += 1;
        format!("hash-binding-{:04}", self.next_binding)
    }
}
