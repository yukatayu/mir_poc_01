//! Syntax-only carrier for the finite source-declared read-only provider
//! effect.  This module retains source facts and spans; it neither assigns
//! authority nor reaches a host/provider boundary.

use super::{SurfaceV0Span, SyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadOnlyProviderEffectDecl {
    name: String,
    requester_principal: String,
    requester_principal_span: SurfaceV0Span,
    requester_locus: String,
    requester_locus_span: SurfaceV0Span,
    adapter_profile: String,
    adapter_profile_span: SurfaceV0Span,
    executor_locus: String,
    executor_locus_span: SurfaceV0Span,
    logical_resource_slot: String,
    logical_resource_slot_span: SurfaceV0Span,
    result_type: String,
    result_type_span: SurfaceV0Span,
    observation_label: String,
    observation_label_span: SurfaceV0Span,
    max_bytes: u64,
    max_bytes_span: SurfaceV0Span,
    max_calls: u64,
    max_calls_span: SurfaceV0Span,
    failures: Vec<String>,
    failures_span: SurfaceV0Span,
    node: SyntaxNode,
}

impl ReadOnlyProviderEffectDecl {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        name: String,
        requester_principal: String,
        requester_principal_span: SurfaceV0Span,
        requester_locus: String,
        requester_locus_span: SurfaceV0Span,
        adapter_profile: String,
        adapter_profile_span: SurfaceV0Span,
        executor_locus: String,
        executor_locus_span: SurfaceV0Span,
        logical_resource_slot: String,
        logical_resource_slot_span: SurfaceV0Span,
        result_type: String,
        result_type_span: SurfaceV0Span,
        observation_label: String,
        observation_label_span: SurfaceV0Span,
        max_bytes: u64,
        max_bytes_span: SurfaceV0Span,
        max_calls: u64,
        max_calls_span: SurfaceV0Span,
        failures: Vec<String>,
        failures_span: SurfaceV0Span,
        node: SyntaxNode,
    ) -> Self {
        Self {
            name,
            requester_principal,
            requester_principal_span,
            requester_locus,
            requester_locus_span,
            adapter_profile,
            adapter_profile_span,
            executor_locus,
            executor_locus_span,
            logical_resource_slot,
            logical_resource_slot_span,
            result_type,
            result_type_span,
            observation_label,
            observation_label_span,
            max_bytes,
            max_bytes_span,
            max_calls,
            max_calls_span,
            failures,
            failures_span,
            node,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn requester_principal(&self) -> &str {
        &self.requester_principal
    }
    pub fn requester_principal_span(&self) -> &SurfaceV0Span {
        &self.requester_principal_span
    }
    pub fn requester_locus(&self) -> &str {
        &self.requester_locus
    }
    pub fn requester_locus_span(&self) -> &SurfaceV0Span {
        &self.requester_locus_span
    }
    pub fn adapter_profile(&self) -> &str {
        &self.adapter_profile
    }
    pub fn adapter_profile_span(&self) -> &SurfaceV0Span {
        &self.adapter_profile_span
    }
    pub fn executor_locus(&self) -> &str {
        &self.executor_locus
    }
    pub fn executor_locus_span(&self) -> &SurfaceV0Span {
        &self.executor_locus_span
    }
    pub fn logical_resource_slot(&self) -> &str {
        &self.logical_resource_slot
    }
    pub fn logical_resource_slot_span(&self) -> &SurfaceV0Span {
        &self.logical_resource_slot_span
    }
    pub fn result_type(&self) -> &str {
        &self.result_type
    }
    pub fn result_type_span(&self) -> &SurfaceV0Span {
        &self.result_type_span
    }
    pub fn observation_label(&self) -> &str {
        &self.observation_label
    }
    pub fn observation_label_span(&self) -> &SurfaceV0Span {
        &self.observation_label_span
    }
    pub const fn max_bytes(&self) -> u64 {
        self.max_bytes
    }
    pub fn max_bytes_span(&self) -> &SurfaceV0Span {
        &self.max_bytes_span
    }
    pub const fn max_calls(&self) -> u64 {
        self.max_calls
    }
    pub fn max_calls_span(&self) -> &SurfaceV0Span {
        &self.max_calls_span
    }
    pub fn failures(&self) -> &[String] {
        &self.failures
    }
    pub fn failures_span(&self) -> &SurfaceV0Span {
        &self.failures_span
    }
    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
    pub(super) fn node(&self) -> &SyntaxNode {
        &self.node
    }
}
