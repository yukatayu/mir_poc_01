//! Typed M6/M7 facts for the one finite, source-declared read-only provider
//! effect selected by ADR-0042.
//!
//! These values preserve source coordinates and the fixed profile.  They do
//! not select a provider, issue an effect-use capability, access a host, or
//! execute a request.

use mir_ast::surface_v0::{ReadOnlyProviderEffectDecl, SurfaceV0Span};

use crate::{shared_model::SourceRef, surface_v0_classification::CoreTemplateKind};

pub const READ_ONLY_PROVIDER_FAILURES: [&str; 10] = [
    "StaleMembership",
    "MissingCapability",
    "MissingWitness",
    "VisibilityDenied",
    "RouteUnavailable",
    "ProviderResourceNotFound",
    "AdapterUnavailable",
    "ProviderInvalidResult",
    "ProviderPolicyDenied",
    "ResourceExhausted",
];

pub const READ_ONLY_PROVIDER_MAX_BYTES: u64 = 32;
pub const READ_ONLY_PROVIDER_MAX_CALLS: u64 = 64;
pub const READ_ONLY_PROVIDER_OBSERVATION_LABEL: &str = "observer_safe";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReadOnlyProviderAdapterProfile {
    ReadInt,
}

impl ReadOnlyProviderAdapterProfile {
    pub fn parse(value: &str) -> Option<Self> {
        (value == "read_int").then_some(Self::ReadInt)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadInt => "read_int",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadOnlyProviderAllowance {
    max_bytes: u64,
    max_calls: u64,
}

impl ReadOnlyProviderAllowance {
    pub const fn fixed() -> Self {
        Self {
            max_bytes: READ_ONLY_PROVIDER_MAX_BYTES,
            max_calls: READ_ONLY_PROVIDER_MAX_CALLS,
        }
    }

    pub const fn max_bytes(&self) -> u64 {
        self.max_bytes
    }

    pub const fn max_calls(&self) -> u64 {
        self.max_calls
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadOnlyProviderEffectTemplate {
    name: String,
    requester_principal: String,
    requester_locus: String,
    executor_locus: String,
    logical_resource_slot: String,
    adapter_profile: Option<ReadOnlyProviderAdapterProfile>,
    result_type: String,
    observation_label: String,
    allowance: ReadOnlyProviderAllowance,
    declared_failures: Vec<String>,
    declaration_span: SurfaceV0Span,
    source_ref: SourceRef,
    requester_principal_span: SurfaceV0Span,
    requester_locus_span: SurfaceV0Span,
    executor_locus_span: SurfaceV0Span,
    logical_resource_slot_span: SurfaceV0Span,
    adapter_profile_span: SurfaceV0Span,
    result_type_span: SurfaceV0Span,
    observation_label_span: SurfaceV0Span,
    max_bytes_span: SurfaceV0Span,
    max_calls_span: SurfaceV0Span,
    failures_span: SurfaceV0Span,
}

impl ReadOnlyProviderEffectTemplate {
    pub fn from_decl(decl: &ReadOnlyProviderEffectDecl) -> Self {
        Self {
            name: decl.name().to_string(),
            requester_principal: decl.requester_principal().to_string(),
            requester_locus: decl.requester_locus().to_string(),
            executor_locus: decl.executor_locus().to_string(),
            logical_resource_slot: decl.logical_resource_slot().to_string(),
            adapter_profile: ReadOnlyProviderAdapterProfile::parse(decl.adapter_profile()),
            result_type: decl.result_type().to_string(),
            observation_label: decl.observation_label().to_string(),
            allowance: ReadOnlyProviderAllowance {
                max_bytes: decl.max_bytes(),
                max_calls: decl.max_calls(),
            },
            declared_failures: decl.failures().to_vec(),
            declaration_span: decl.span().clone(),
            source_ref: source_ref_from_span(decl.span()),
            requester_principal_span: decl.requester_principal_span().clone(),
            requester_locus_span: decl.requester_locus_span().clone(),
            executor_locus_span: decl.executor_locus_span().clone(),
            logical_resource_slot_span: decl.logical_resource_slot_span().clone(),
            adapter_profile_span: decl.adapter_profile_span().clone(),
            result_type_span: decl.result_type_span().clone(),
            observation_label_span: decl.observation_label_span().clone(),
            max_bytes_span: decl.max_bytes_span().clone(),
            max_calls_span: decl.max_calls_span().clone(),
            failures_span: decl.failures_span().clone(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub const fn kind(&self) -> CoreTemplateKind {
        CoreTemplateKind::ReadOnlyProviderEffect
    }
    pub fn requester_principal(&self) -> &str {
        &self.requester_principal
    }
    pub fn requester_locus(&self) -> &str {
        &self.requester_locus
    }
    pub fn executor_locus(&self) -> &str {
        &self.executor_locus
    }
    pub fn logical_resource_slot(&self) -> &str {
        &self.logical_resource_slot
    }
    pub fn adapter_profile(&self) -> Option<ReadOnlyProviderAdapterProfile> {
        self.adapter_profile
    }
    pub fn result_type(&self) -> &str {
        &self.result_type
    }
    pub fn observation_label(&self) -> &str {
        &self.observation_label
    }
    pub const fn allowance(&self) -> ReadOnlyProviderAllowance {
        self.allowance
    }
    pub fn declared_failures(&self) -> &[String] {
        &self.declared_failures
    }
    pub fn declaration_span(&self) -> &SurfaceV0Span {
        &self.declaration_span
    }
    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
    pub fn requester_principal_span(&self) -> &SurfaceV0Span {
        &self.requester_principal_span
    }
    pub fn requester_locus_span(&self) -> &SurfaceV0Span {
        &self.requester_locus_span
    }
    pub fn executor_locus_span(&self) -> &SurfaceV0Span {
        &self.executor_locus_span
    }
    pub fn logical_resource_slot_span(&self) -> &SurfaceV0Span {
        &self.logical_resource_slot_span
    }
    pub fn adapter_profile_span(&self) -> &SurfaceV0Span {
        &self.adapter_profile_span
    }
    pub fn result_type_span(&self) -> &SurfaceV0Span {
        &self.result_type_span
    }
    pub fn observation_label_span(&self) -> &SurfaceV0Span {
        &self.observation_label_span
    }
    pub fn max_bytes_span(&self) -> &SurfaceV0Span {
        &self.max_bytes_span
    }
    pub fn max_calls_span(&self) -> &SurfaceV0Span {
        &self.max_calls_span
    }
    pub fn failures_span(&self) -> &SurfaceV0Span {
        &self.failures_span
    }
}

/// M7's source-derived checked Core coordinate.  It deliberately has no
/// provider identity, host path, result value, or authority token.
///
/// ```compile_fail
/// use mir_semantics::surface_v0_provider_effect::CheckedReadOnlyProviderEffectCore;
///
/// let _ = CheckedReadOnlyProviderEffectCore::from_template;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedReadOnlyProviderEffectCore {
    operation: String,
    requester_principal: String,
    requester_locus: String,
    executor_locus: String,
    result_consumer_locus: String,
    adapter_profile: ReadOnlyProviderAdapterProfile,
    logical_resource_slot: String,
    result_type: String,
    observation_label: String,
    allowance: ReadOnlyProviderAllowance,
    declared_failures: Vec<String>,
    source_ref: SourceRef,
}

impl CheckedReadOnlyProviderEffectCore {
    pub(crate) fn from_template(template: &ReadOnlyProviderEffectTemplate) -> Option<Self> {
        let adapter_profile = template.adapter_profile()?;
        Some(Self {
            operation: template.name.clone(),
            requester_principal: template.requester_principal.clone(),
            requester_locus: template.requester_locus.clone(),
            executor_locus: template.executor_locus.clone(),
            // This finite profile fixes the consumer to the requester, but
            // retains it as an explicit Core coordinate.
            result_consumer_locus: template.requester_locus.clone(),
            adapter_profile,
            logical_resource_slot: template.logical_resource_slot.clone(),
            result_type: template.result_type.clone(),
            observation_label: template.observation_label.clone(),
            allowance: template.allowance,
            declared_failures: template.declared_failures.clone(),
            source_ref: template.source_ref.clone(),
        })
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }
    pub fn requester_principal(&self) -> &str {
        &self.requester_principal
    }
    pub fn requester_locus(&self) -> &str {
        &self.requester_locus
    }
    pub fn executor_locus(&self) -> &str {
        &self.executor_locus
    }
    pub fn result_consumer_locus(&self) -> &str {
        &self.result_consumer_locus
    }
    pub const fn adapter_profile(&self) -> ReadOnlyProviderAdapterProfile {
        self.adapter_profile
    }
    pub fn logical_resource_slot(&self) -> &str {
        &self.logical_resource_slot
    }
    pub fn result_type(&self) -> &str {
        &self.result_type
    }
    pub fn observation_label(&self) -> &str {
        &self.observation_label
    }
    pub const fn allowance(&self) -> ReadOnlyProviderAllowance {
        self.allowance
    }
    pub fn declared_failures(&self) -> &[String] {
        &self.declared_failures
    }
    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
    pub const fn effect_use_is_required(&self) -> bool {
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_private_snapshot(
        operation: String,
        requester_principal: String,
        requester_locus: String,
        executor_locus: String,
        result_consumer_locus: String,
        adapter_profile: String,
        logical_resource_slot: String,
        result_type: String,
        observation_label: String,
        max_bytes: u64,
        max_calls: u64,
        declared_failures: Vec<String>,
        source_ref: SourceRef,
    ) -> Option<Self> {
        let exact_failures = declared_failures.len() == READ_ONLY_PROVIDER_FAILURES.len()
            && declared_failures
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                == READ_ONLY_PROVIDER_FAILURES.len()
            && READ_ONLY_PROVIDER_FAILURES
                .iter()
                .all(|failure| declared_failures.iter().any(|declared| declared == failure));
        (requester_principal == "self"
            && is_surface_v0_identifier(&operation)
            && is_surface_v0_identifier(&requester_locus)
            && is_surface_v0_identifier(&executor_locus)
            && is_surface_v0_identifier(&result_consumer_locus)
            && is_surface_v0_identifier(&logical_resource_slot)
            && requester_locus != executor_locus
            && requester_locus == result_consumer_locus
            && adapter_profile == ReadOnlyProviderAdapterProfile::ReadInt.as_str()
            && result_type == "Int"
            && observation_label == READ_ONLY_PROVIDER_OBSERVATION_LABEL
            && max_bytes == READ_ONLY_PROVIDER_MAX_BYTES
            && max_calls == READ_ONLY_PROVIDER_MAX_CALLS
            && exact_failures)
            .then_some(Self {
                operation,
                requester_principal,
                requester_locus,
                executor_locus,
                result_consumer_locus,
                adapter_profile: ReadOnlyProviderAdapterProfile::ReadInt,
                logical_resource_slot,
                result_type,
                observation_label,
                allowance: ReadOnlyProviderAllowance::fixed(),
                declared_failures,
                source_ref,
            })
    }
}

fn is_surface_v0_identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    matches!(bytes.next(), Some(byte) if byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn source_ref_from_span(span: &SurfaceV0Span) -> SourceRef {
    let data = span.source_ref_data();
    let (start_line, start_column, end_line, end_column) = data.line_columns();
    SourceRef::new(
        data.path().to_string(),
        start_line,
        start_column,
        end_line,
        end_column,
    )
}
