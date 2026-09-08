//! M8's bounded admission boundary for checked M7 Surface v0 artifacts.
//!
//! This module does not execute the checked program.  It only establishes a
//! source-bound runtime admission instance after every M8-owned residual is
//! matched by typed evidence.  Deferred authorization and verification remain
//! explicitly outside this boundary and are reported to M9.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use mir_semantics::m9_finite_refinement::M9ReadOnlyProviderEffectCoverage;
use mir_semantics::surface_v0_pipeline::private_snapshot::{
    SnapshotCheckedProgramIdentity, SnapshotDesignatedCheckedCore,
    SnapshotOwnerAdmissionBudgetCondition, SnapshotRelationCheckedCore, SnapshotSourceRef,
    SnapshotTypedExpression, SnapshotTypedStateRead,
};
use mir_semantics::{
    evaluation_materialization::{EvaluationPolicy, InputFrontier, ObservationPolicy, PolicyStamp},
    shared_model::{ResultFrontier, SourceRef},
    surface_v0_classification::{OwnerAdmissionBudgetCondition, SourceToCoreKind},
    surface_v0_pipeline::{
        CheckedEvaluationKind, CheckedProgramIdentity, CheckedSurfaceV0, DesignatedCheckedCore,
        RelationCheckedCore, ResidualObligation, ResidualObligationKind, TypedExpression,
        TypedStateRead,
    },
};

/// The bounded information-flow order retained by the M8 runtime boundary.
///
/// Labels created without an explicit class default to `Private`: admission
/// evidence never acquires a weaker visibility class through an omission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum M8SecurityClass {
    Public,
    Restricted,
    Private,
}

impl M8SecurityClass {
    pub fn is_at_least(self, other: Self) -> bool {
        self >= other
    }

    pub(crate) fn join(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSecurityLabel {
    value: String,
    security_class: M8SecurityClass,
}

impl EvidenceSecurityLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            security_class: M8SecurityClass::Private,
        }
    }

    pub fn with_class(mut self, security_class: M8SecurityClass) -> Self {
        self.security_class = security_class;
        self
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub const fn security_class(&self) -> M8SecurityClass {
        self.security_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRedaction(String);

impl EvidenceRedaction {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Evidence admitted at the M8 boundary.  Each variant names exactly one
/// residual family and retains its source location; there is intentionally no
/// boolean success carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M8AdmissionEvidence {
    RelationVisibility {
        relation: String,
        label: EvidenceSecurityLabel,
        redaction: EvidenceRedaction,
        source_ref: SourceRef,
    },
    RelationLifetime {
        relation: String,
        live_lease: String,
        binding_frontier: String,
        source_ref: SourceRef,
    },
    RelationFallbackValidity {
        relation: String,
        primary_epoch: String,
        fallback_epoch: String,
        source_ref: SourceRef,
    },
    ValueVisibilityRedaction {
        value: String,
        label: EvidenceSecurityLabel,
        redaction: EvidenceRedaction,
        source_ref: SourceRef,
    },
    AuthDeferred {
        name: String,
        authority_label: String,
        source_ref: SourceRef,
    },
    VerifyDeferred {
        name: String,
        theorem: String,
        witness_schema: String,
        source_ref: SourceRef,
    },
}

impl M8AdmissionEvidence {
    fn residual_kind(&self) -> ResidualObligationKind {
        match self {
            Self::RelationVisibility { .. } => ResidualObligationKind::Visibility,
            Self::RelationLifetime { .. } => ResidualObligationKind::RelationLifetime,
            Self::RelationFallbackValidity { .. } => ResidualObligationKind::FallbackValidity,
            Self::ValueVisibilityRedaction { .. } => {
                ResidualObligationKind::ValueVisibilityRedaction
            }
            Self::AuthDeferred { .. } => ResidualObligationKind::AuthDeferred,
            Self::VerifyDeferred { .. } => ResidualObligationKind::VerifyDeferred,
        }
    }

    fn residual_name(&self) -> &str {
        match self {
            Self::RelationVisibility { relation, .. }
            | Self::RelationLifetime { relation, .. }
            | Self::RelationFallbackValidity { relation, .. } => relation,
            Self::ValueVisibilityRedaction { value, .. } => value,
            Self::AuthDeferred { name, .. } | Self::VerifyDeferred { name, .. } => name,
        }
    }

    fn source_ref(&self) -> &SourceRef {
        match self {
            Self::RelationVisibility { source_ref, .. }
            | Self::RelationLifetime { source_ref, .. }
            | Self::RelationFallbackValidity { source_ref, .. }
            | Self::ValueVisibilityRedaction { source_ref, .. }
            | Self::AuthDeferred { source_ref, .. }
            | Self::VerifyDeferred { source_ref, .. } => source_ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RuntimeAdmission {
    program_identity: CheckedProgramIdentity,
    evidence: Vec<M8AdmissionEvidence>,
}

impl M8RuntimeAdmission {
    pub fn new(program_identity: CheckedProgramIdentity) -> Self {
        Self {
            program_identity,
            evidence: Vec::new(),
        }
    }

    pub fn with_evidence(mut self, evidence: M8AdmissionEvidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn evidence(&self) -> &[M8AdmissionEvidence] {
        &self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8AdmissionDiagnosticKind {
    ProgramIdentityMismatch,
    MissingResidualEvidence,
    SourceRefMismatch,
    DuplicateResidualEvidence,
    ConflictingResidualEvidence,
    DeferredToM9,
    RelationEvidencePayloadMismatch,
    UnsupportedReadOnlyProviderEffectProfile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8AdmissionDiagnostic {
    kind: M8AdmissionDiagnosticKind,
    residual_kind: Option<ResidualObligationKind>,
    residual_name: Option<String>,
    source_ref: SourceRef,
    expected_source_ref: Option<SourceRef>,
}

impl M8AdmissionDiagnostic {
    fn for_residual(kind: M8AdmissionDiagnosticKind, residual: &ResidualObligation) -> Self {
        Self {
            kind,
            residual_kind: Some(residual.kind()),
            residual_name: Some(residual.name().to_string()),
            source_ref: residual.source_ref().clone(),
            expected_source_ref: None,
        }
    }

    fn source_ref_mismatch(residual: &ResidualObligation) -> Self {
        Self {
            kind: M8AdmissionDiagnosticKind::SourceRefMismatch,
            residual_kind: Some(residual.kind()),
            residual_name: Some(residual.name().to_string()),
            source_ref: residual.source_ref().clone(),
            expected_source_ref: Some(residual.source_ref().clone()),
        }
    }

    fn program_identity_mismatch(checked: &CheckedSurfaceV0) -> Self {
        Self {
            kind: M8AdmissionDiagnosticKind::ProgramIdentityMismatch,
            residual_kind: None,
            residual_name: None,
            source_ref: checked.program_identity().root_source_ref().clone(),
            expected_source_ref: None,
        }
    }

    fn relation_payload_mismatch(residual: &ResidualObligation) -> Self {
        Self::for_residual(
            M8AdmissionDiagnosticKind::RelationEvidencePayloadMismatch,
            residual,
        )
    }

    fn unsupported_read_only_provider_effect_profile(checked: &CheckedSurfaceV0) -> Option<Self> {
        if !checked
            .evaluations()
            .iter()
            .any(|evaluation| evaluation.kind() == CheckedEvaluationKind::ReadOnlyProviderEffect)
        {
            return None;
        }

        if let Some(residual) = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| {
                residual.kind() == ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported
            })
        {
            return Some(Self::for_residual(
                M8AdmissionDiagnosticKind::UnsupportedReadOnlyProviderEffectProfile,
                residual,
            ));
        }

        let evaluation = checked.evaluations().iter().find(|evaluation| {
            evaluation.kind() == CheckedEvaluationKind::ReadOnlyProviderEffect
        })?;
        Some(Self {
            kind: M8AdmissionDiagnosticKind::UnsupportedReadOnlyProviderEffectProfile,
            residual_kind: None,
            residual_name: None,
            source_ref: evaluation.source_ref().clone(),
            expected_source_ref: None,
        })
    }

    pub const fn kind(&self) -> M8AdmissionDiagnosticKind {
        self.kind
    }

    pub const fn residual_kind(&self) -> Option<ResidualObligationKind> {
        self.residual_kind
    }

    pub fn residual_name(&self) -> Option<&str> {
        self.residual_name.as_deref()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn expected_source_ref(&self) -> Option<&SourceRef> {
        self.expected_source_ref.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8AdmissionDiagnostics {
    entries: Vec<M8AdmissionDiagnostic>,
}

impl M8AdmissionDiagnostics {
    fn one(diagnostic: M8AdmissionDiagnostic) -> Self {
        Self {
            entries: vec![diagnostic],
        }
    }

    pub fn primary(&self) -> &M8AdmissionDiagnostic {
        self.entries
            .first()
            .expect("M8 admission diagnostics always have a primary entry")
    }

    pub fn entries(&self) -> &[M8AdmissionDiagnostic] {
        &self.entries
    }

    pub const fn has_runtime_success(&self) -> bool {
        false
    }

    pub const fn grants_authority(&self) -> bool {
        false
    }

    pub const fn emits_verdict(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8Runtime {
    _admission_boundary: (),
}

impl M8Runtime {
    /// Admit one checked M7 artifact using only its structural identity and
    /// typed, source-bound evidence.  No source AST, fixture name, authority,
    /// or verification verdict participates in this boundary.
    pub fn admit(
        &self,
        checked: CheckedSurfaceV0,
        admission: M8RuntimeAdmission,
    ) -> Result<M8RuntimeInstance, M8AdmissionDiagnostics> {
        if admission.program_identity() != checked.program_identity() {
            return Err(M8AdmissionDiagnostics::one(
                M8AdmissionDiagnostic::program_identity_mismatch(&checked),
            ));
        }

        if let Some(diagnostic) =
            M8AdmissionDiagnostic::unsupported_read_only_provider_effect_profile(&checked)
        {
            return Err(M8AdmissionDiagnostics::one(diagnostic));
        }

        if let Some(residual) = checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| {
                matches!(
                    residual.kind(),
                    ResidualObligationKind::AuthDeferred | ResidualObligationKind::VerifyDeferred
                )
            })
        {
            return Err(M8AdmissionDiagnostics::one(
                M8AdmissionDiagnostic::for_residual(
                    M8AdmissionDiagnosticKind::DeferredToM9,
                    residual,
                ),
            ));
        }

        for residual in checked.residual_obligations().entries() {
            let evidence: Vec<&M8AdmissionEvidence> = admission
                .evidence()
                .iter()
                .filter(|evidence| {
                    evidence.residual_kind() == residual.kind()
                        && evidence.residual_name() == residual.name()
                })
                .collect();
            let Some(first) = evidence.first() else {
                return Err(M8AdmissionDiagnostics::one(
                    M8AdmissionDiagnostic::for_residual(
                        M8AdmissionDiagnosticKind::MissingResidualEvidence,
                        residual,
                    ),
                ));
            };
            if evidence
                .iter()
                .any(|evidence| evidence.source_ref() != residual.source_ref())
            {
                return Err(M8AdmissionDiagnostics::one(
                    M8AdmissionDiagnostic::source_ref_mismatch(residual),
                ));
            }
            if evidence.len() > 1 {
                let diagnostic_kind = if evidence.iter().all(|evidence| *evidence == *first) {
                    M8AdmissionDiagnosticKind::DuplicateResidualEvidence
                } else {
                    M8AdmissionDiagnosticKind::ConflictingResidualEvidence
                };
                return Err(M8AdmissionDiagnostics::one(
                    M8AdmissionDiagnostic::for_residual(diagnostic_kind, residual),
                ));
            }
        }

        if let Some(diagnostic) = relation_payload_diagnostic(&checked, &admission) {
            return Err(M8AdmissionDiagnostics::one(diagnostic));
        }

        Ok(M8RuntimeInstance::from_admitted(checked, admission))
    }
}

/// Lossless, crate-private M8 plan view retained by the M9 outer judgment.
///
/// This is deliberately neither `M8RuntimeInstance` nor an M8 admission
/// result.  It retains the checked artifact, source-bound M8 evidence, ordered
/// lowering, and exact M9 residual descriptors so that a fully resolved M9
/// carrier can later cross the private M8 execution seam without reparsing
/// source, reconstructing Core, or making M8 direct admission succeed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8DeferredM9Base {
    program_identity: CheckedProgramIdentity,
    checked_surface: CheckedSurfaceV0,
    admission: M8RuntimeAdmission,
    ordered_lowering: OrderedRuntimeLowering,
    deferred_residuals: Vec<M8DeferredM9Residual>,
    provider_coverage: Option<M9ReadOnlyProviderEffectCoverage>,
    provider_binding_context: Option<M8ReadOnlyProviderEffectBindingContext>,
}

impl M8DeferredM9Base {
    pub(crate) fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub(crate) fn deferred_residuals(&self) -> &[M8DeferredM9Residual] {
        &self.deferred_residuals
    }

    pub(crate) fn has_read_only_provider_effect_scope(&self) -> bool {
        self.provider_coverage.is_some() || self.provider_binding_context.is_some()
    }
}

/// Opaque, T0-internal binding coordinate retained only by the dedicated M8
/// provider component. It exposes neither a resource nonce nor a native
/// handle.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct M8ReadOnlyProviderEffectBindingContext {
    resource_runtime_nonce: [u8; 32],
}

impl std::fmt::Debug for M8ReadOnlyProviderEffectBindingContext {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M8ReadOnlyProviderEffectBindingContext(..)")
    }
}

pub(crate) fn read_only_provider_effect_binding_context(
    resource_runtime_nonce: [u8; 32],
) -> M8ReadOnlyProviderEffectBindingContext {
    M8ReadOnlyProviderEffectBindingContext {
        resource_runtime_nonce,
    }
}

impl M8ReadOnlyProviderEffectBindingContext {
    fn matches_resource_runtime_nonce(&self, resource_runtime_nonce: &[u8; 32]) -> bool {
        &self.resource_runtime_nonce == resource_runtime_nonce
    }

    fn resource_runtime_nonce_ref(&self) -> String {
        resource_runtime_nonce_ref(&self.resource_runtime_nonce)
    }
}

fn resource_runtime_nonce_ref(resource_runtime_nonce: &[u8; 32]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/m8/i3/provider-resource-runtime-nonce/v1\0");
    hasher.update(resource_runtime_nonce);
    format!(
        "m8-i3-provider-resource-runtime-nonce-sha256-v1:{:x}",
        hasher.finalize()
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8DeferredM9Residual {
    kind: ResidualObligationKind,
    name: String,
    source_ref: SourceRef,
}

impl M8DeferredM9Residual {
    pub(crate) const fn kind(&self) -> ResidualObligationKind {
        self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

/// Validate only M8-owned residual evidence before an outer M9 judgment.
///
/// M8 direct admission intentionally keeps its earlier `DeferredToM9`
/// outcome.  This helper is the sole crate-private seam that lets M9 prove
/// that all non-M9 M8 evidence is complete while retaining the M7 residual
/// row unchanged.
pub(crate) fn prepare_deferred_m9_base(
    checked: &CheckedSurfaceV0,
    admission: &M8RuntimeAdmission,
) -> Result<M8DeferredM9Base, M8AdmissionDiagnostics> {
    prepare_deferred_m9_base_for_provider_coverage(checked, admission, None, None)
}

/// The composite provider path may carry the complete checked provider
/// coverage while retaining the source residual.  This is deliberately a
/// separate crate-private seam: ordinary M8 admission remains rejecting.
pub(crate) fn prepare_deferred_m9_base_for_provider_coverage(
    checked: &CheckedSurfaceV0,
    admission: &M8RuntimeAdmission,
    provider_coverage: Option<&M9ReadOnlyProviderEffectCoverage>,
    provider_binding_context: Option<M8ReadOnlyProviderEffectBindingContext>,
) -> Result<M8DeferredM9Base, M8AdmissionDiagnostics> {
    if admission.program_identity() != checked.program_identity() {
        return Err(M8AdmissionDiagnostics::one(
            M8AdmissionDiagnostic::program_identity_mismatch(checked),
        ));
    }

    if provider_coverage.is_some() != provider_binding_context.is_some() {
        return Err(M8AdmissionDiagnostics::one(
            M8AdmissionDiagnostic::program_identity_mismatch(checked),
        ));
    }

    if let Some(diagnostic) =
        M8AdmissionDiagnostic::unsupported_read_only_provider_effect_profile(checked)
        && provider_coverage.is_none_or(|coverage| {
            coverage.program_identity() != checked.program_identity()
                || !coverage.matches_checked(checked)
        })
    {
        return Err(M8AdmissionDiagnostics::one(diagnostic));
    }

    for residual in checked.residual_obligations().entries() {
        if residual.kind() == ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported
            && provider_coverage.is_some()
        {
            continue;
        }
        if matches!(
            residual.kind(),
            ResidualObligationKind::AuthDeferred | ResidualObligationKind::VerifyDeferred
        ) {
            continue;
        }
        let evidence: Vec<&M8AdmissionEvidence> = admission
            .evidence()
            .iter()
            .filter(|evidence| {
                evidence.residual_kind() == residual.kind()
                    && evidence.residual_name() == residual.name()
            })
            .collect();
        let Some(first) = evidence.first() else {
            return Err(M8AdmissionDiagnostics::one(
                M8AdmissionDiagnostic::for_residual(
                    M8AdmissionDiagnosticKind::MissingResidualEvidence,
                    residual,
                ),
            ));
        };
        if evidence
            .iter()
            .any(|evidence| evidence.source_ref() != residual.source_ref())
        {
            return Err(M8AdmissionDiagnostics::one(
                M8AdmissionDiagnostic::source_ref_mismatch(residual),
            ));
        }
        if evidence.len() > 1 {
            let diagnostic_kind = if evidence.iter().all(|evidence| *evidence == *first) {
                M8AdmissionDiagnosticKind::DuplicateResidualEvidence
            } else {
                M8AdmissionDiagnosticKind::ConflictingResidualEvidence
            };
            return Err(M8AdmissionDiagnostics::one(
                M8AdmissionDiagnostic::for_residual(diagnostic_kind, residual),
            ));
        }
    }

    if let Some(diagnostic) = relation_payload_diagnostic(checked, admission) {
        return Err(M8AdmissionDiagnostics::one(diagnostic));
    }

    Ok(M8DeferredM9Base {
        program_identity: checked.program_identity().clone(),
        checked_surface: checked.clone(),
        admission: admission.clone(),
        ordered_lowering: OrderedRuntimeLowering::from_checked(checked),
        deferred_residuals: checked
            .residual_obligations()
            .entries()
            .iter()
            .filter(|residual| {
                matches!(
                    residual.kind(),
                    ResidualObligationKind::AuthDeferred | ResidualObligationKind::VerifyDeferred
                )
            })
            .map(|residual| M8DeferredM9Residual {
                kind: residual.kind(),
                name: residual.name().to_string(),
                source_ref: residual.source_ref().clone(),
            })
            .collect(),
        provider_coverage: provider_coverage.cloned(),
        provider_binding_context,
    })
}

/// Cross the M8 plan seam only after the outer M9 judgment has resolved its
/// retained residuals.  This is crate-private so neither direct M8 admission
/// nor the prepared M9 base can be mistaken for a public runtime success.
#[allow(dead_code)] // Reserved crate-private M10 seam; no public M8 success route exists.
pub(crate) fn materialize_m9_resolved_base(
    base: M8DeferredM9Base,
) -> Result<M8RuntimeInstance, M8AdmissionDiagnostics> {
    if base.has_read_only_provider_effect_scope() {
        let diagnostic = M8AdmissionDiagnostic::unsupported_read_only_provider_effect_profile(
            &base.checked_surface,
        )
        .unwrap_or_else(|| M8AdmissionDiagnostic::program_identity_mismatch(&base.checked_surface));
        return Err(M8AdmissionDiagnostics::one(diagnostic));
    }
    let M8DeferredM9Base {
        checked_surface,
        admission,
        ..
    } = base;
    Ok(M8RuntimeInstance::from_admitted(checked_surface, admission))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeLoweringKind {
    OwnerRequest,
    OwnerLocalRead,
    OwnerWrite,
    ObserverPublish,
    DesignatedDecision,
    DesignatedResultConsume,
    RelationPublish,
    ConsumerLocalProjection,
    DeferredPolicy,
    ReadOnlyProviderEffectRequest,
    ReadOnlyProviderEffectInvocation,
    ReadOnlyProviderEffectResult,
    ReadOnlyProviderEffectResultConsume,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLoweringEntry {
    ordinal: usize,
    kind: RuntimeLoweringKind,
    source_ref: SourceRef,
    core_ref: String,
}

impl RuntimeLoweringEntry {
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub const fn kind(&self) -> RuntimeLoweringKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OrderedRuntimeLowering {
    entries: Vec<RuntimeLoweringEntry>,
}

impl OrderedRuntimeLowering {
    fn from_checked(checked: &CheckedSurfaceV0) -> Self {
        Self {
            entries: checked
                .source_map()
                .entries()
                .iter()
                .map(|entry| RuntimeLoweringEntry {
                    ordinal: entry.ordinal(),
                    kind: match entry.kind() {
                        SourceToCoreKind::OwnerRmw => RuntimeLoweringKind::OwnerRequest,
                        SourceToCoreKind::OwnerLocalRead => RuntimeLoweringKind::OwnerLocalRead,
                        SourceToCoreKind::OwnerLocalWrite => RuntimeLoweringKind::OwnerWrite,
                        SourceToCoreKind::ObserverPublish => RuntimeLoweringKind::ObserverPublish,
                        SourceToCoreKind::DesignatedDecision => {
                            RuntimeLoweringKind::DesignatedDecision
                        }
                        SourceToCoreKind::DesignatedResultConsume => {
                            RuntimeLoweringKind::DesignatedResultConsume
                        }
                        SourceToCoreKind::PublishRelation => RuntimeLoweringKind::RelationPublish,
                        SourceToCoreKind::ConsumerLocalProjection => {
                            RuntimeLoweringKind::ConsumerLocalProjection
                        }
                        SourceToCoreKind::DeferredPolicy => RuntimeLoweringKind::DeferredPolicy,
                        SourceToCoreKind::ReadOnlyProviderEffectRequest => {
                            RuntimeLoweringKind::ReadOnlyProviderEffectRequest
                        }
                        SourceToCoreKind::ReadOnlyProviderEffectInvocation => {
                            RuntimeLoweringKind::ReadOnlyProviderEffectInvocation
                        }
                        SourceToCoreKind::ReadOnlyProviderEffectResult => {
                            RuntimeLoweringKind::ReadOnlyProviderEffectResult
                        }
                        SourceToCoreKind::ReadOnlyProviderEffectResultConsume => {
                            RuntimeLoweringKind::ReadOnlyProviderEffectResultConsume
                        }
                    },
                    source_ref: entry.source_ref().clone(),
                    core_ref: entry.core_ref().to_string(),
                })
                .collect(),
        }
    }

    pub fn entries(&self) -> &[RuntimeLoweringEntry] {
        &self.entries
    }

    pub fn kinds(&self) -> Vec<RuntimeLoweringKind> {
        self.entries
            .iter()
            .map(RuntimeLoweringEntry::kind)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8AdmissionEvidenceRow {
    entries: Vec<M8AdmissionEvidence>,
}

impl M8AdmissionEvidenceRow {
    pub fn entries(&self) -> &[M8AdmissionEvidence] {
        &self.entries
    }

    pub fn contains_residual(&self, kind: ResidualObligationKind, name: &str) -> bool {
        self.entries
            .iter()
            .any(|evidence| evidence.residual_kind() == kind && evidence.residual_name() == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8AdmittedDesignatedValue {
    name: String,
    result_frontier: ResultFrontier,
    input_frontier: InputFrontier,
    evaluation_policy: EvaluationPolicy,
    observation_policy: ObservationPolicy,
    policy_stamp: PolicyStamp,
    visibility_label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
}

impl M8AdmittedDesignatedValue {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn result_frontier(&self) -> &ResultFrontier {
        &self.result_frontier
    }

    pub fn input_frontier(&self) -> &InputFrontier {
        &self.input_frontier
    }

    pub fn evaluation_policy(&self) -> &EvaluationPolicy {
        &self.evaluation_policy
    }

    pub fn observation_policy(&self) -> &ObservationPolicy {
        &self.observation_policy
    }

    pub fn policy_stamp(&self) -> &PolicyStamp {
        &self.policy_stamp
    }

    pub fn visibility_label(&self) -> &EvidenceSecurityLabel {
        &self.visibility_label
    }

    pub fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }
}

/// Checked M7 designated Core and its exact M8 visibility/redaction evidence.
/// This remains a source-free Core carrier for the receipt-only runtime facade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8DesignatedExecutionPlan {
    name: String,
    source_ref: SourceRef,
    core: DesignatedCheckedCore,
    visibility_label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
}

impl M8DesignatedExecutionPlan {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core(&self) -> &DesignatedCheckedCore {
        &self.core
    }

    pub(crate) fn visibility_label(&self) -> &EvidenceSecurityLabel {
        &self.visibility_label
    }

    pub(crate) fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }
}

/// Checked M7 owner data retained by an admitted instance for the M8 owner
/// queue.  It is a source-free Core carrier, not a request reconstructed from
/// a parser or fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8OwnerExecutionPlan {
    evaluation: String,
    actor: String,
    owner_locus: String,
    source_ref: SourceRef,
    target: TypedStateRead,
    expression: TypedExpression,
    owner_admission_budget: Option<OwnerAdmissionBudgetCondition>,
}

impl M8OwnerExecutionPlan {
    pub(crate) fn evaluation(&self) -> &str {
        &self.evaluation
    }

    pub(crate) fn actor(&self) -> &str {
        &self.actor
    }

    pub(crate) fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn target(&self) -> &TypedStateRead {
        &self.target
    }

    pub(crate) fn expression(&self) -> &TypedExpression {
        &self.expression
    }

    pub(crate) fn owner_admission_budget(&self) -> Option<&OwnerAdmissionBudgetCondition> {
        self.owner_admission_budget.as_ref()
    }
}

/// Checked relation data and its exact Phase 1 admission evidence, retained
/// for the later M8 semantic relation/projection facade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationExecutionPlan {
    name: String,
    source_ref: SourceRef,
    core: RelationCheckedCore,
    visibility_label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
    live_lease_ref: String,
    binding_frontier: String,
    primary_epoch: String,
    fallback_epoch: String,
}

impl M8RelationExecutionPlan {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core(&self) -> &RelationCheckedCore {
        &self.core
    }

    pub fn visibility_label(&self) -> &EvidenceSecurityLabel {
        &self.visibility_label
    }

    pub fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }

    pub fn live_lease_ref(&self) -> &str {
        &self.live_lease_ref
    }

    pub fn binding_frontier(&self) -> &str {
        &self.binding_frontier
    }

    pub fn primary_epoch(&self) -> &str {
        &self.primary_epoch
    }

    pub fn fallback_epoch(&self) -> &str {
        &self.fallback_epoch
    }

    pub(crate) fn has_exact_admission_evidence(&self) -> bool {
        self.binding_frontier
            == self
                .core
                .binding_frontier()
                .as_slice()
                .first()
                .expect("M7 relation binding frontier is finite and nonempty")
                .as_str()
            && self.primary_epoch == self.core.primary().epoch()
            && self.fallback_epoch == self.core.fallback().epoch()
            && !self.live_lease_ref.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RuntimeInstance {
    program_identity: CheckedProgramIdentity,
    runtime_alias: String,
    ordered_lowering: OrderedRuntimeLowering,
    admission: M8RuntimeAdmission,
    admission_evidence: M8AdmissionEvidenceRow,
    designated_values: Vec<M8AdmittedDesignatedValue>,
    designated_execution_plans: Vec<M8DesignatedExecutionPlan>,
    owner_execution_plans: Vec<M8OwnerExecutionPlan>,
    relation_execution_plans: Vec<M8RelationExecutionPlan>,
    scope: M8RuntimeInstanceScope,
}

/// The ordinary instance is the only form allowed to leave M8 through the
/// existing private I3 snapshot.  The provider composite marker is retained
/// across restrictions and can only be observed through its sealed wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
enum M8RuntimeInstanceScope {
    Ordinary,
    ReadOnlyProviderEffectComposite {
        coverage: Box<M9ReadOnlyProviderEffectCoverage>,
        binding_context: M8ReadOnlyProviderEffectBindingContext,
    },
    /// A decoded I3 provider component may enter this state only through the
    /// dedicated inherited-control restore below. It remains non-ordinary:
    /// generic M8 admission and private ordinary snapshots reject it.
    ReadOnlyProviderEffectInstalled {
        binding_context: M8ReadOnlyProviderEffectBindingContext,
        component_binding_ref: String,
    },
}

/// Opaque M8 component retained by the private read-only provider composite.
/// It deliberately exposes neither the bare runtime instance nor an ordinary
/// process image.
#[derive(Clone)]
pub(crate) struct M8ReadOnlyProviderEffectComponent {
    instance: M8RuntimeInstance,
    coverage: M9ReadOnlyProviderEffectCoverage,
    binding_context: M8ReadOnlyProviderEffectBindingContext,
    declared_provider_lowering_count: usize,
}

impl std::fmt::Debug for M8ReadOnlyProviderEffectComponent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M8ReadOnlyProviderEffectComponent(..)")
    }
}

/// A component-only snapshot.  It is not an ordinary I3 process image and
/// can restore only through the same provider-scoped wrapper.
#[derive(Clone)]
pub(crate) struct M8ReadOnlyProviderEffectComponentSnapshot {
    component: M8ReadOnlyProviderEffectComponent,
}

impl std::fmt::Debug for M8ReadOnlyProviderEffectComponentSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M8ReadOnlyProviderEffectComponentSnapshot(..)")
    }
}

/// Versioned, private image DTO for the non-executable portion of a scoped
/// provider component. It retains an explicit nested private-snapshot scope
/// discriminator, so the component cannot be decoded as an ordinary M8
/// instance. The current binding remains checked against the parent-held
/// component at the SYS-5 expected-start boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct M8I3PrivateProviderComponentSnapshot {
    version: u32,
    component: M8I3PrivateSnapshot,
    component_binding_ref: String,
    resource_runtime_nonce_ref: String,
    declared_provider_lowering_count: usize,
}

/// One private, installed M8 provider component. It is returned only after a
/// tagged component snapshot has matched the resource-runtime nonce carried
/// through inherited provider control; it is not an ordinary M8 instance.
pub(crate) struct M8InstalledReadOnlyProviderEffectComponent {
    instance: M8RuntimeInstance,
    component_binding_ref: String,
}

impl std::fmt::Debug for M8InstalledReadOnlyProviderEffectComponent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M8InstalledReadOnlyProviderEffectComponent(..)")
    }
}

impl M8InstalledReadOnlyProviderEffectComponent {
    /// Copy one still-scoped M8 instance only for the dedicated local
    /// provider bootstrap.  The returned instance remains marked as an
    /// inherited provider component, so the ordinary I3 snapshot/export
    /// path continues to reject it.
    pub(crate) fn scoped_instance_for_local_provider_runtime(
        &self,
        expected_component_binding_ref: &str,
    ) -> Option<M8RuntimeInstance> {
        matches!(
            &self.instance.scope,
            M8RuntimeInstanceScope::ReadOnlyProviderEffectInstalled {
                component_binding_ref,
                ..
            } if component_binding_ref == expected_component_binding_ref
        )
        .then(|| self.instance.clone())
    }

    pub(crate) fn component_binding_ref(&self) -> &str {
        &self.component_binding_ref
    }
}

/// T0-internal, non-authorizing inventory for a sealed provider component. It
/// retains only checked identity and ordered-lowering correspondence; it
/// exposes neither the underlying M8 instance nor any authority, witness, or
/// native binding.
pub(crate) struct M8ReadOnlyProviderEffectComponentInventory {
    program_identity: CheckedProgramIdentity,
    ordered_lowering: OrderedRuntimeLowering,
    declared_provider_lowering_count: usize,
}

impl std::fmt::Debug for M8ReadOnlyProviderEffectComponentInventory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("M8ReadOnlyProviderEffectComponentInventory(..)")
    }
}

pub(crate) fn materialize_read_only_provider_effect_component(
    base: M8DeferredM9Base,
    coverage: M9ReadOnlyProviderEffectCoverage,
) -> Result<M8ReadOnlyProviderEffectComponent, M8AdmissionDiagnostics> {
    let M8DeferredM9Base {
        program_identity,
        checked_surface,
        admission,
        ordered_lowering,
        provider_coverage,
        provider_binding_context,
        ..
    } = base;
    if coverage.program_identity() != &program_identity
        || !coverage.matches_checked(&checked_surface)
    {
        return Err(M8AdmissionDiagnostics::one(
            M8AdmissionDiagnostic::program_identity_mismatch(&checked_surface),
        ));
    }
    let (Some(base_coverage), Some(binding_context)) =
        (provider_coverage, provider_binding_context)
    else {
        return Err(M8AdmissionDiagnostics::one(
            M8AdmissionDiagnostic::program_identity_mismatch(&checked_surface),
        ));
    };
    if base_coverage != coverage {
        return Err(M8AdmissionDiagnostics::one(
            M8AdmissionDiagnostic::program_identity_mismatch(&checked_surface),
        ));
    }
    let declared_provider_lowering_count = ordered_lowering
        .entries()
        .iter()
        .filter(|entry| is_read_only_provider_effect_lowering(entry.kind()))
        .count();
    if declared_provider_lowering_count != 4 {
        return Err(M8AdmissionDiagnostics::one(
            M8AdmissionDiagnostic::program_identity_mismatch(&checked_surface),
        ));
    }
    let retained_ordered_lowering = OrderedRuntimeLowering {
        entries: ordered_lowering
            .entries()
            .iter()
            .filter(|entry| !is_read_only_provider_effect_lowering(entry.kind()))
            .cloned()
            .collect(),
    };
    let mut instance = M8RuntimeInstance::from_admitted(checked_surface, admission);
    instance.ordered_lowering = retained_ordered_lowering;
    instance.scope = M8RuntimeInstanceScope::ReadOnlyProviderEffectComposite {
        coverage: Box::new(coverage.clone()),
        binding_context: binding_context.clone(),
    };
    Ok(M8ReadOnlyProviderEffectComponent {
        instance,
        coverage,
        binding_context,
        declared_provider_lowering_count,
    })
}

impl M8ReadOnlyProviderEffectComponent {
    pub(crate) fn restricted_to_loci(&self, assigned_loci: &BTreeSet<String>) -> Self {
        let mut instance = self.instance.restricted_to_loci(assigned_loci);
        // This sealed component retains its already-filtered static
        // non-provider lowering correspondence across a locus restriction.
        // The ordinary execution plans above remain restricted; this private
        // inventory cannot become an ordinary M8 instance or process image.
        instance.ordered_lowering = self.instance.ordered_lowering.clone();
        Self {
            instance,
            coverage: self.coverage.clone(),
            binding_context: self.binding_context.clone(),
            declared_provider_lowering_count: self.declared_provider_lowering_count,
        }
    }

    pub(crate) fn ordinary_i3_private_snapshot(
        &self,
    ) -> Result<M8I3PrivateSnapshot, M8I3PrivateSnapshotError> {
        self.instance.i3_private_snapshot()
    }

    pub(crate) fn scoped_snapshot(&self) -> M8ReadOnlyProviderEffectComponentSnapshot {
        M8ReadOnlyProviderEffectComponentSnapshot {
            component: self.clone(),
        }
    }

    /// Export only the already restricted, non-provider M8 plan inventory for
    /// a tagged inactive provider image.  This is deliberately distinct from
    /// the ordinary I3 snapshot route, which continues to reject the scoped
    /// component rather than erasing its provider identity.
    pub(crate) fn i3_private_provider_component_snapshot(
        &self,
    ) -> M8I3PrivateProviderComponentSnapshot {
        M8I3PrivateProviderComponentSnapshot {
            version: M8I3PrivateProviderComponentSnapshot::VERSION,
            component: M8I3PrivateSnapshot::from_read_only_provider_component(&self.instance),
            component_binding_ref: self.i3_inactive_component_binding_ref(),
            resource_runtime_nonce_ref: self.binding_context.resource_runtime_nonce_ref(),
            declared_provider_lowering_count: self.declared_provider_lowering_count,
        }
    }

    pub(crate) fn is_component_scoped(&self) -> bool {
        matches!(
            &self.instance.scope,
            M8RuntimeInstanceScope::ReadOnlyProviderEffectComposite {
                coverage,
                binding_context,
            } if coverage.as_ref() == &self.coverage && binding_context == &self.binding_context
        )
    }

    pub(crate) fn inventory(&self) -> M8ReadOnlyProviderEffectComponentInventory {
        M8ReadOnlyProviderEffectComponentInventory {
            program_identity: self.instance.program_identity().clone(),
            ordered_lowering: self.instance.ordered_lowering().clone(),
            declared_provider_lowering_count: self.declared_provider_lowering_count,
        }
    }

    /// Opaque private correspondence reference for an already restricted
    /// inactive component. It is not a credential and exposes neither the
    /// resource nonce nor any authority/witness material to an image.
    pub(crate) fn i3_inactive_component_binding_ref(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/m8/i3/inactive-provider-component/v1\0");
        hasher.update(self.instance.program_identity().stable_key());
        hasher.update(format!("{:?}", self.instance.ordered_lowering()));
        hasher.update(format!("{:?}", self.coverage));
        hasher.update(self.binding_context.resource_runtime_nonce);
        hasher.update(self.declared_provider_lowering_count.to_be_bytes());
        format!(
            "m8-i3-inactive-provider-component-sha256-v1:{:x}",
            hasher.finalize()
        )
    }
}

impl M8ReadOnlyProviderEffectComponentInventory {
    pub(crate) fn matches_checked_program_identity(&self, checked: &CheckedSurfaceV0) -> bool {
        self.program_identity == *checked.program_identity()
    }

    /// Count the provider rows declared by the original checked profile. This
    /// remains four after the component execution inventory excludes them.
    pub(crate) const fn provider_lowering_count(&self) -> usize {
        self.declared_provider_lowering_count
    }

    pub(crate) fn retained_lowering_ordinals(&self) -> Vec<usize> {
        self.ordered_lowering
            .entries()
            .iter()
            .map(RuntimeLoweringEntry::ordinal)
            .collect()
    }

    /// Compare the retained inventory against every non-provider checked
    /// lowering internally, including ordinal, source association, and Core
    /// association, without exporting those references to T0 inspection.
    pub(crate) fn retains_exact_non_provider_ordered_lowering_associations(
        &self,
        checked: &CheckedSurfaceV0,
    ) -> bool {
        self.program_identity == *checked.program_identity()
            && self.ordered_lowering.entries()
                == OrderedRuntimeLowering::from_checked(checked)
                    .entries()
                    .iter()
                    .filter(|entry| !is_read_only_provider_effect_lowering(entry.kind()))
                    .cloned()
                    .collect::<Vec<_>>()
    }
}

fn is_read_only_provider_effect_lowering(kind: RuntimeLoweringKind) -> bool {
    matches!(
        kind,
        RuntimeLoweringKind::ReadOnlyProviderEffectRequest
            | RuntimeLoweringKind::ReadOnlyProviderEffectInvocation
            | RuntimeLoweringKind::ReadOnlyProviderEffectResult
            | RuntimeLoweringKind::ReadOnlyProviderEffectResultConsume
    )
}

impl M8ReadOnlyProviderEffectComponentSnapshot {
    pub(crate) fn is_component_scoped(&self) -> bool {
        self.component.is_component_scoped()
    }

    pub(crate) fn restore_for_resource_runtime_nonce(
        self,
        resource_runtime_nonce: &[u8; 32],
    ) -> Result<M8ReadOnlyProviderEffectComponent, M8I3PrivateSnapshotError> {
        if !self
            .component
            .binding_context
            .matches_resource_runtime_nonce(resource_runtime_nonce)
        {
            return Err(M8I3PrivateSnapshotError::StructuralMismatch);
        }
        self.restore()
    }

    pub(crate) fn restore(
        self,
    ) -> Result<M8ReadOnlyProviderEffectComponent, M8I3PrivateSnapshotError> {
        match &self.component.instance.scope {
            M8RuntimeInstanceScope::ReadOnlyProviderEffectComposite {
                coverage,
                binding_context,
            } if coverage.as_ref() == &self.component.coverage
                && binding_context == &self.component.binding_context
                && coverage.program_identity() == self.component.instance.program_identity() =>
            {
                Ok(self.component)
            }
            M8RuntimeInstanceScope::Ordinary
            | M8RuntimeInstanceScope::ReadOnlyProviderEffectComposite { .. }
            | M8RuntimeInstanceScope::ReadOnlyProviderEffectInstalled { .. } => {
                Err(M8I3PrivateSnapshotError::StructuralMismatch)
            }
        }
    }
}

impl M8I3PrivateProviderComponentSnapshot {
    // Version 2 made `resource_runtime_nonce_ref` mandatory.  This is a
    // private, provider-only DTO; older/missing-field bytes are rejected
    // rather than being interpreted as an ordinary or unbound component.
    const VERSION: u32 = 2;

    /// Verify that a private provider image retained the required nested
    /// scoped discriminator. This is structural only: it neither restores an
    /// ordinary M8 instance nor issues any authority.
    pub(crate) fn is_scoped_component_snapshot(&self) -> bool {
        self.version == Self::VERSION
            && self.declared_provider_lowering_count == 4
            && !self.component_binding_ref.is_empty()
            && !self.resource_runtime_nonce_ref.is_empty()
            && self.component.version == M8I3PrivateSnapshot::VERSION
            && self.component.scope == M8I3PrivateSnapshotScope::ReadOnlyProviderEffectComponent
    }

    /// Compare a decoded image DTO to the exact parent-held restricted
    /// component.  The comparison is structural and non-authorizing: it does
    /// not restore an M8 instance, issue a capability, or make the binding
    /// current.
    pub(crate) fn matches_parent_component(
        &self,
        parent: &M8ReadOnlyProviderEffectComponent,
    ) -> bool {
        self.version == Self::VERSION
            && self.declared_provider_lowering_count == parent.declared_provider_lowering_count
            && self.component_binding_ref == parent.i3_inactive_component_binding_ref()
            && self.resource_runtime_nonce_ref
                == parent.binding_context.resource_runtime_nonce_ref()
            && self.component
                == M8I3PrivateSnapshot::from_read_only_provider_component(&parent.instance)
            && parent.is_component_scoped()
    }

    pub(crate) const fn declared_provider_lowering_count(&self) -> usize {
        self.declared_provider_lowering_count
    }

    pub(crate) fn component_binding_ref(&self) -> &str {
        &self.component_binding_ref
    }

    /// Restore only the explicitly tagged scoped component for a child whose
    /// inherited control carries the exact resource-runtime nonce. This does
    /// not construct an ordinary snapshot, invoke M8 admission, or supply a
    /// provider-effect capability; M9 installs and revalidates that lineage
    /// separately at the caller's preceding boundary.
    pub(crate) fn restore_for_inherited_provider_install(
        self,
        resource_runtime_nonce: &[u8; 32],
    ) -> Result<M8InstalledReadOnlyProviderEffectComponent, M8I3PrivateSnapshotError> {
        if !self.is_scoped_component_snapshot()
            || self.resource_runtime_nonce_ref != resource_runtime_nonce_ref(resource_runtime_nonce)
        {
            return Err(M8I3PrivateSnapshotError::StructuralMismatch);
        }
        let component_binding_ref = self.component_binding_ref;
        let instance = self.component.into_provider_component_instance(
            M8ReadOnlyProviderEffectBindingContext {
                resource_runtime_nonce: *resource_runtime_nonce,
            },
            component_binding_ref.clone(),
        )?;
        Ok(M8InstalledReadOnlyProviderEffectComponent {
            instance,
            component_binding_ref,
        })
    }

    #[cfg(test)]
    pub(crate) fn test_only_change_retained_lowering_association(&mut self) -> bool {
        let Some(entry) = self.component.ordered_lowering.first_mut() else {
            return false;
        };
        entry.core_ref.push_str(":i3-provider-tampered");
        true
    }
}

impl M8RuntimeInstance {
    fn from_admitted(checked: CheckedSurfaceV0, admission: M8RuntimeAdmission) -> Self {
        let designated_execution_plans: Vec<M8DesignatedExecutionPlan> = checked
            .evaluations()
            .iter()
            .filter_map(|evaluation| {
                let core = evaluation.designated_core()?;
                let name = format!("{}.{}", core.evaluator(), core.result());
                let (visibility_label, redaction) =
                    admission
                        .evidence
                        .iter()
                        .find_map(|evidence| match evidence {
                            M8AdmissionEvidence::ValueVisibilityRedaction {
                                value,
                                label,
                                redaction,
                                source_ref,
                            } if value == &name && source_ref == evaluation.source_ref() => {
                                Some((label.clone(), redaction.clone()))
                            }
                            _ => None,
                        })?;
                Some(M8DesignatedExecutionPlan {
                    name,
                    source_ref: evaluation.source_ref().clone(),
                    core: core.clone(),
                    visibility_label,
                    redaction,
                })
            })
            .collect();
        let designated_values = designated_execution_plans
            .iter()
            .map(|plan| M8AdmittedDesignatedValue {
                name: plan.name.clone(),
                result_frontier: plan.core.result_frontier().clone(),
                input_frontier: plan.core.input_frontier().clone(),
                evaluation_policy: plan.core.evaluation_policy().clone(),
                observation_policy: plan.core.observation_policy().clone(),
                policy_stamp: plan.core.policy_stamp().clone(),
                visibility_label: plan.visibility_label.clone(),
                redaction: plan.redaction.clone(),
            })
            .collect();
        let owner_execution_plans = checked
            .evaluations()
            .iter()
            .filter_map(|evaluation| {
                let owner = evaluation.owner_rmw_core()?;
                Some(M8OwnerExecutionPlan {
                    evaluation: evaluation.name().to_string(),
                    actor: evaluation.actor_authority_origin().to_string(),
                    owner_locus: owner.owner_locus().to_string(),
                    source_ref: evaluation.source_ref().clone(),
                    target: owner.target().clone(),
                    expression: owner.expression().clone(),
                    owner_admission_budget: owner.owner_admission_budget().cloned(),
                })
            })
            .collect();
        let relation_execution_plans = checked
            .evaluations()
            .iter()
            .filter_map(|evaluation| {
                let core = evaluation.relation_core()?;
                let name = evaluation.name().to_string();
                let source_ref = evaluation.source_ref();
                let (visibility_label, redaction) = admission
                    .evidence
                    .iter()
                    .find_map(|evidence| match evidence {
                        M8AdmissionEvidence::RelationVisibility {
                            relation,
                            label,
                            redaction,
                            source_ref: evidence_ref,
                        } if relation == &name && evidence_ref == source_ref => {
                            Some((label.clone(), redaction.clone()))
                        }
                        _ => None,
                    })
                    .expect("validated relation visibility evidence exists");
                let (live_lease_ref, binding_frontier) = admission
                    .evidence
                    .iter()
                    .find_map(|evidence| match evidence {
                        M8AdmissionEvidence::RelationLifetime {
                            relation,
                            live_lease,
                            binding_frontier,
                            source_ref: evidence_ref,
                        } if relation == &name && evidence_ref == source_ref => {
                            Some((live_lease.clone(), binding_frontier.clone()))
                        }
                        _ => None,
                    })
                    .expect("validated relation lifetime evidence exists");
                let (primary_epoch, fallback_epoch) = admission
                    .evidence
                    .iter()
                    .find_map(|evidence| match evidence {
                        M8AdmissionEvidence::RelationFallbackValidity {
                            relation,
                            primary_epoch,
                            fallback_epoch,
                            source_ref: evidence_ref,
                        } if relation == &name && evidence_ref == source_ref => {
                            Some((primary_epoch.clone(), fallback_epoch.clone()))
                        }
                        _ => None,
                    })
                    .expect("validated relation fallback evidence exists");
                Some(M8RelationExecutionPlan {
                    source_ref: source_ref.clone(),
                    name,
                    core: core.clone(),
                    visibility_label,
                    redaction,
                    live_lease_ref,
                    binding_frontier,
                    primary_epoch,
                    fallback_epoch,
                })
            })
            .collect();
        let program_identity = checked.program_identity().clone();
        let runtime_alias = format!("runtime-admitted:{}", program_identity.stable_key());
        let ordered_lowering = OrderedRuntimeLowering::from_checked(&checked);

        Self {
            program_identity,
            runtime_alias,
            ordered_lowering,
            admission_evidence: M8AdmissionEvidenceRow {
                entries: admission.evidence.clone(),
            },
            admission,
            designated_values,
            designated_execution_plans,
            owner_execution_plans,
            relation_execution_plans,
            scope: M8RuntimeInstanceScope::Ordinary,
        }
    }

    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn runtime_alias(&self) -> &str {
        &self.runtime_alias
    }

    pub const fn is_runtime_admitted(&self) -> bool {
        true
    }

    pub fn ordered_lowering(&self) -> &OrderedRuntimeLowering {
        &self.ordered_lowering
    }

    pub fn admission_evidence(&self) -> &M8AdmissionEvidenceRow {
        &self.admission_evidence
    }

    pub fn admission(&self) -> &M8RuntimeAdmission {
        &self.admission
    }

    pub fn designated_value(&self, name: &str) -> Option<&M8AdmittedDesignatedValue> {
        self.designated_values
            .iter()
            .find(|value| value.name == name)
    }

    /// Move the admitted checked Core into M8's single-state owner execution
    /// facade.  This accepts no source AST or fixture identity.
    pub fn into_execution(
        self,
        seed: crate::m8_runtime_owner_queue::M8ExecutionSeed,
    ) -> crate::m8_runtime_owner_queue::M8RuntimeExecution {
        crate::m8_runtime_owner_queue::M8RuntimeExecution::from_admitted(self, seed)
    }

    pub(crate) fn owner_execution_plans(&self) -> &[M8OwnerExecutionPlan] {
        &self.owner_execution_plans
    }

    /// Move admitted designated Core and exact visibility evidence into M8's
    /// receipt-only designated-value facade.
    pub fn into_designated_values(
        self,
        seed: crate::m8_runtime_designated_value::M8DesignatedSeed,
    ) -> crate::m8_runtime_designated_value::M8DesignatedRuntime {
        crate::m8_runtime_designated_value::M8DesignatedRuntime::from_admitted(self, seed)
    }

    pub(crate) fn designated_execution_plans(&self) -> &[M8DesignatedExecutionPlan] {
        &self.designated_execution_plans
    }

    /// Move the admitted checked relation Core into M8's bounded semantic
    /// relation/projection facade.  Presentation contexts are supplied later
    /// and remain outside this admitted semantic state.
    pub fn into_relation_projection(
        self,
        seed: crate::m8_runtime_relation_projection::M8RelationProjectionSeed,
    ) -> crate::m8_runtime_relation_projection::M8RelationProjectionRuntime {
        crate::m8_runtime_relation_projection::M8RelationProjectionRuntime::from_admitted(
            self, seed,
        )
    }

    pub(crate) fn relation_execution_plans(&self) -> &[M8RelationExecutionPlan] {
        &self.relation_execution_plans
    }

    /// Restrict this already-admitted, source-free execution inventory to
    /// plans evaluated at the assigned loci.  This filter retains the parent
    /// checked-program identity and never performs a second M8 admission.
    pub(crate) fn restricted_to_loci(&self, assigned_loci: &BTreeSet<String>) -> Self {
        let owner_execution_plans = self
            .owner_execution_plans
            .iter()
            .filter(|plan| assigned_loci.contains(plan.owner_locus()))
            .cloned()
            .collect::<Vec<_>>();
        let designated_execution_plans = self
            .designated_execution_plans
            .iter()
            .filter(|plan| assigned_loci.contains(plan.core().evaluator()))
            .cloned()
            .collect::<Vec<_>>();
        let relation_execution_plans = self
            .relation_execution_plans
            .iter()
            .filter(|plan| assigned_loci.contains(plan.core().owner_locus()))
            .cloned()
            .collect::<Vec<_>>();
        let designated_values = self
            .designated_values
            .iter()
            .filter(|value| {
                designated_execution_plans
                    .iter()
                    .any(|plan| plan.name() == value.name())
            })
            .cloned()
            .collect::<Vec<_>>();
        let retained_source_refs = owner_execution_plans
            .iter()
            .map(M8OwnerExecutionPlan::source_ref)
            .chain(
                designated_execution_plans
                    .iter()
                    .map(M8DesignatedExecutionPlan::source_ref),
            )
            .chain(
                relation_execution_plans
                    .iter()
                    .map(M8RelationExecutionPlan::source_ref),
            )
            .collect::<Vec<_>>();
        let has_retained_source_ref =
            |source_ref: &SourceRef| retained_source_refs.contains(&source_ref);
        let admission_evidence = self
            .admission_evidence
            .entries
            .iter()
            .filter(|evidence| has_retained_source_ref(evidence.source_ref()))
            .cloned()
            .collect::<Vec<_>>();

        Self {
            program_identity: self.program_identity.clone(),
            runtime_alias: self.runtime_alias.clone(),
            ordered_lowering: OrderedRuntimeLowering {
                entries: self
                    .ordered_lowering
                    .entries
                    .iter()
                    .filter(|entry| has_retained_source_ref(entry.source_ref()))
                    .cloned()
                    .collect(),
            },
            admission: M8RuntimeAdmission {
                program_identity: self.admission.program_identity.clone(),
                evidence: admission_evidence.clone(),
            },
            admission_evidence: M8AdmissionEvidenceRow {
                entries: admission_evidence,
            },
            designated_values,
            designated_execution_plans,
            owner_execution_plans,
            relation_execution_plans,
            scope: self.scope.clone(),
        }
    }

    pub fn relation_plan(&self, name: &str) -> Option<&M8RelationExecutionPlan> {
        self.relation_execution_plans
            .iter()
            .find(|plan| plan.name() == name)
    }

    /// Export an already-admitted M8 execution image for the private I3
    /// process bootstrap.  This copies the restricted, source-free execution
    /// facts only; it neither invokes M8 admission nor exposes a public
    /// serialization contract.
    pub(crate) fn i3_private_snapshot(
        &self,
    ) -> Result<M8I3PrivateSnapshot, M8I3PrivateSnapshotError> {
        match &self.scope {
            M8RuntimeInstanceScope::Ordinary => Ok(M8I3PrivateSnapshot::from_instance(self)),
            M8RuntimeInstanceScope::ReadOnlyProviderEffectComposite { .. }
            | M8RuntimeInstanceScope::ReadOnlyProviderEffectInstalled { .. } => {
                Err(M8I3PrivateSnapshotError::StructuralMismatch)
            }
        }
    }

    /// Restore an exact, already-admitted M8 execution image.  This is a
    /// structural reconstruction of the sealed restricted facts, not a
    /// source check, lowering, or M8 admission path.
    pub(crate) fn from_i3_private_snapshot(
        snapshot: M8I3PrivateSnapshot,
    ) -> Result<Self, M8I3PrivateSnapshotError> {
        snapshot.into_instance()
    }
}

/// Fail-closed rejection of a private M8 execution snapshot.  The public
/// runtime API never receives this carrier or its schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum M8I3PrivateSnapshotError {
    SemanticSnapshot,
    StructuralMismatch,
}

/// Versioned DTO for an already restricted M8 instance.  It deliberately
/// retains no `CheckedSurfaceV0`, source text, or admission capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct M8I3PrivateSnapshot {
    version: u32,
    /// Required private discriminator. It deliberately has no serde default:
    /// pre-discriminator bytes and unknown scopes cannot restore as ordinary.
    scope: M8I3PrivateSnapshotScope,
    program_identity: SnapshotCheckedProgramIdentity,
    runtime_alias: String,
    ordered_lowering: Vec<PrivateRuntimeLoweringEntrySnapshot>,
    admission: PrivateM8RuntimeAdmissionSnapshot,
    designated_execution_plans: Vec<PrivateM8DesignatedExecutionPlanSnapshot>,
    owner_execution_plans: Vec<PrivateM8OwnerExecutionPlanSnapshot>,
    relation_execution_plans: Vec<PrivateM8RelationExecutionPlanSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum M8I3PrivateSnapshotScope {
    Ordinary,
    ReadOnlyProviderEffectComponent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateRuntimeLoweringKindSnapshot {
    OwnerRequest,
    OwnerLocalRead,
    OwnerWrite,
    ObserverPublish,
    DesignatedDecision,
    DesignatedResultConsume,
    RelationPublish,
    ConsumerLocalProjection,
    DeferredPolicy,
    ReadOnlyProviderEffectRequest,
    ReadOnlyProviderEffectInvocation,
    ReadOnlyProviderEffectResult,
    ReadOnlyProviderEffectResultConsume,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateRuntimeLoweringEntrySnapshot {
    ordinal: usize,
    kind: PrivateRuntimeLoweringKindSnapshot,
    source_ref: SnapshotSourceRef,
    core_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateM8SecurityClassSnapshot {
    Public,
    Restricted,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateEvidenceSecurityLabelSnapshot {
    value: String,
    security_class: PrivateM8SecurityClassSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateM8RuntimeAdmissionSnapshot {
    program_identity: SnapshotCheckedProgramIdentity,
    evidence: Vec<PrivateM8AdmissionEvidenceSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum PrivateM8AdmissionEvidenceSnapshot {
    RelationVisibility {
        relation: String,
        label: PrivateEvidenceSecurityLabelSnapshot,
        redaction: String,
        source_ref: SnapshotSourceRef,
    },
    RelationLifetime {
        relation: String,
        live_lease: String,
        binding_frontier: String,
        source_ref: SnapshotSourceRef,
    },
    RelationFallbackValidity {
        relation: String,
        primary_epoch: String,
        fallback_epoch: String,
        source_ref: SnapshotSourceRef,
    },
    ValueVisibilityRedaction {
        value: String,
        label: PrivateEvidenceSecurityLabelSnapshot,
        redaction: String,
        source_ref: SnapshotSourceRef,
    },
    AuthDeferred {
        name: String,
        authority_label: String,
        source_ref: SnapshotSourceRef,
    },
    VerifyDeferred {
        name: String,
        theorem: String,
        witness_schema: String,
        source_ref: SnapshotSourceRef,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateM8DesignatedExecutionPlanSnapshot {
    name: String,
    source_ref: SnapshotSourceRef,
    core: SnapshotDesignatedCheckedCore,
    visibility_label: PrivateEvidenceSecurityLabelSnapshot,
    redaction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateM8OwnerExecutionPlanSnapshot {
    evaluation: String,
    actor: String,
    owner_locus: String,
    source_ref: SnapshotSourceRef,
    target: SnapshotTypedStateRead,
    expression: SnapshotTypedExpression,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_admission_budget: Option<SnapshotOwnerAdmissionBudgetCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateM8RelationExecutionPlanSnapshot {
    name: String,
    source_ref: SnapshotSourceRef,
    core: SnapshotRelationCheckedCore,
    visibility_label: PrivateEvidenceSecurityLabelSnapshot,
    redaction: String,
    live_lease_ref: String,
    binding_frontier: String,
    primary_epoch: String,
    fallback_epoch: String,
}

impl M8I3PrivateSnapshot {
    const VERSION: u32 = 2;

    fn from_instance(instance: &M8RuntimeInstance) -> Self {
        Self {
            version: Self::VERSION,
            scope: M8I3PrivateSnapshotScope::Ordinary,
            program_identity: SnapshotCheckedProgramIdentity::from_checked(
                &instance.program_identity,
            ),
            runtime_alias: instance.runtime_alias.clone(),
            ordered_lowering: instance
                .ordered_lowering
                .entries
                .iter()
                .map(PrivateRuntimeLoweringEntrySnapshot::from_entry)
                .collect(),
            admission: PrivateM8RuntimeAdmissionSnapshot::from_admission(&instance.admission),
            designated_execution_plans: instance
                .designated_execution_plans
                .iter()
                .map(PrivateM8DesignatedExecutionPlanSnapshot::from_plan)
                .collect(),
            owner_execution_plans: instance
                .owner_execution_plans
                .iter()
                .map(PrivateM8OwnerExecutionPlanSnapshot::from_plan)
                .collect(),
            relation_execution_plans: instance
                .relation_execution_plans
                .iter()
                .map(PrivateM8RelationExecutionPlanSnapshot::from_plan)
                .collect(),
        }
    }

    fn from_read_only_provider_component(instance: &M8RuntimeInstance) -> Self {
        let mut snapshot = Self::from_instance(instance);
        snapshot.scope = M8I3PrivateSnapshotScope::ReadOnlyProviderEffectComponent;
        snapshot
    }

    fn into_instance(self) -> Result<M8RuntimeInstance, M8I3PrivateSnapshotError> {
        self.into_instance_with_scope(
            M8I3PrivateSnapshotScope::Ordinary,
            M8RuntimeInstanceScope::Ordinary,
        )
    }

    fn into_provider_component_instance(
        self,
        binding_context: M8ReadOnlyProviderEffectBindingContext,
        component_binding_ref: String,
    ) -> Result<M8RuntimeInstance, M8I3PrivateSnapshotError> {
        self.into_instance_with_scope(
            M8I3PrivateSnapshotScope::ReadOnlyProviderEffectComponent,
            M8RuntimeInstanceScope::ReadOnlyProviderEffectInstalled {
                binding_context,
                component_binding_ref,
            },
        )
    }

    fn into_instance_with_scope(
        self,
        expected_scope: M8I3PrivateSnapshotScope,
        restored_scope: M8RuntimeInstanceScope,
    ) -> Result<M8RuntimeInstance, M8I3PrivateSnapshotError> {
        if self.version != Self::VERSION || self.scope != expected_scope {
            return Err(M8I3PrivateSnapshotError::StructuralMismatch);
        }
        if self.ordered_lowering.iter().any(|entry| {
            matches!(
                entry.kind,
                PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectRequest
                    | PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectInvocation
                    | PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectResult
                    | PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectResultConsume
            )
        }) {
            return Err(M8I3PrivateSnapshotError::StructuralMismatch);
        }
        let program_identity = self
            .program_identity
            .into_checked()
            .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?;
        let admission = self.admission.into_admission()?;
        if admission.program_identity != program_identity {
            return Err(M8I3PrivateSnapshotError::StructuralMismatch);
        }
        let designated_execution_plans = self
            .designated_execution_plans
            .into_iter()
            .map(PrivateM8DesignatedExecutionPlanSnapshot::into_plan)
            .collect::<Result<Vec<_>, _>>()?;
        let owner_execution_plans = self
            .owner_execution_plans
            .into_iter()
            .map(PrivateM8OwnerExecutionPlanSnapshot::into_plan)
            .collect::<Result<Vec<_>, _>>()?;
        let relation_execution_plans = self
            .relation_execution_plans
            .into_iter()
            .map(PrivateM8RelationExecutionPlanSnapshot::into_plan)
            .collect::<Result<Vec<_>, _>>()?;

        let designated_values = designated_execution_plans
            .iter()
            .map(|plan| M8AdmittedDesignatedValue {
                name: plan.name.clone(),
                result_frontier: plan.core.result_frontier().clone(),
                input_frontier: plan.core.input_frontier().clone(),
                evaluation_policy: plan.core.evaluation_policy().clone(),
                observation_policy: plan.core.observation_policy().clone(),
                policy_stamp: plan.core.policy_stamp().clone(),
                visibility_label: plan.visibility_label.clone(),
                redaction: plan.redaction.clone(),
            })
            .collect();
        let admission_evidence = M8AdmissionEvidenceRow {
            entries: admission.evidence.clone(),
        };
        Ok(M8RuntimeInstance {
            program_identity,
            runtime_alias: self.runtime_alias,
            ordered_lowering: OrderedRuntimeLowering {
                entries: self
                    .ordered_lowering
                    .into_iter()
                    .map(PrivateRuntimeLoweringEntrySnapshot::into_entry)
                    .collect::<Result<Vec<_>, _>>()?,
            },
            admission,
            admission_evidence,
            designated_values,
            designated_execution_plans,
            owner_execution_plans,
            relation_execution_plans,
            scope: restored_scope,
        })
    }
}

impl PrivateRuntimeLoweringEntrySnapshot {
    fn from_entry(entry: &RuntimeLoweringEntry) -> Self {
        Self {
            ordinal: entry.ordinal,
            kind: match entry.kind {
                RuntimeLoweringKind::OwnerRequest => {
                    PrivateRuntimeLoweringKindSnapshot::OwnerRequest
                }
                RuntimeLoweringKind::OwnerLocalRead => {
                    PrivateRuntimeLoweringKindSnapshot::OwnerLocalRead
                }
                RuntimeLoweringKind::OwnerWrite => PrivateRuntimeLoweringKindSnapshot::OwnerWrite,
                RuntimeLoweringKind::ObserverPublish => {
                    PrivateRuntimeLoweringKindSnapshot::ObserverPublish
                }
                RuntimeLoweringKind::DesignatedDecision => {
                    PrivateRuntimeLoweringKindSnapshot::DesignatedDecision
                }
                RuntimeLoweringKind::DesignatedResultConsume => {
                    PrivateRuntimeLoweringKindSnapshot::DesignatedResultConsume
                }
                RuntimeLoweringKind::RelationPublish => {
                    PrivateRuntimeLoweringKindSnapshot::RelationPublish
                }
                RuntimeLoweringKind::ConsumerLocalProjection => {
                    PrivateRuntimeLoweringKindSnapshot::ConsumerLocalProjection
                }
                RuntimeLoweringKind::DeferredPolicy => {
                    PrivateRuntimeLoweringKindSnapshot::DeferredPolicy
                }
                RuntimeLoweringKind::ReadOnlyProviderEffectRequest => {
                    PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectRequest
                }
                RuntimeLoweringKind::ReadOnlyProviderEffectInvocation => {
                    PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectInvocation
                }
                RuntimeLoweringKind::ReadOnlyProviderEffectResult => {
                    PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectResult
                }
                RuntimeLoweringKind::ReadOnlyProviderEffectResultConsume => {
                    PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectResultConsume
                }
            },
            source_ref: SnapshotSourceRef::from_checked(&entry.source_ref),
            core_ref: entry.core_ref.clone(),
        }
    }

    fn into_entry(self) -> Result<RuntimeLoweringEntry, M8I3PrivateSnapshotError> {
        Ok(RuntimeLoweringEntry {
            ordinal: self.ordinal,
            kind: match self.kind {
                PrivateRuntimeLoweringKindSnapshot::OwnerRequest => {
                    RuntimeLoweringKind::OwnerRequest
                }
                PrivateRuntimeLoweringKindSnapshot::OwnerLocalRead => {
                    RuntimeLoweringKind::OwnerLocalRead
                }
                PrivateRuntimeLoweringKindSnapshot::OwnerWrite => RuntimeLoweringKind::OwnerWrite,
                PrivateRuntimeLoweringKindSnapshot::ObserverPublish => {
                    RuntimeLoweringKind::ObserverPublish
                }
                PrivateRuntimeLoweringKindSnapshot::DesignatedDecision => {
                    RuntimeLoweringKind::DesignatedDecision
                }
                PrivateRuntimeLoweringKindSnapshot::DesignatedResultConsume => {
                    RuntimeLoweringKind::DesignatedResultConsume
                }
                PrivateRuntimeLoweringKindSnapshot::RelationPublish => {
                    RuntimeLoweringKind::RelationPublish
                }
                PrivateRuntimeLoweringKindSnapshot::ConsumerLocalProjection => {
                    RuntimeLoweringKind::ConsumerLocalProjection
                }
                PrivateRuntimeLoweringKindSnapshot::DeferredPolicy => {
                    RuntimeLoweringKind::DeferredPolicy
                }
                PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectRequest => {
                    RuntimeLoweringKind::ReadOnlyProviderEffectRequest
                }
                PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectInvocation => {
                    RuntimeLoweringKind::ReadOnlyProviderEffectInvocation
                }
                PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectResult => {
                    RuntimeLoweringKind::ReadOnlyProviderEffectResult
                }
                PrivateRuntimeLoweringKindSnapshot::ReadOnlyProviderEffectResultConsume => {
                    RuntimeLoweringKind::ReadOnlyProviderEffectResultConsume
                }
            },
            source_ref: self
                .source_ref
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            core_ref: self.core_ref,
        })
    }
}

impl PrivateEvidenceSecurityLabelSnapshot {
    fn from_label(label: &EvidenceSecurityLabel) -> Self {
        Self {
            value: label.value.clone(),
            security_class: match label.security_class {
                M8SecurityClass::Public => PrivateM8SecurityClassSnapshot::Public,
                M8SecurityClass::Restricted => PrivateM8SecurityClassSnapshot::Restricted,
                M8SecurityClass::Private => PrivateM8SecurityClassSnapshot::Private,
            },
        }
    }

    fn into_label(self) -> EvidenceSecurityLabel {
        EvidenceSecurityLabel::new(self.value).with_class(match self.security_class {
            PrivateM8SecurityClassSnapshot::Public => M8SecurityClass::Public,
            PrivateM8SecurityClassSnapshot::Restricted => M8SecurityClass::Restricted,
            PrivateM8SecurityClassSnapshot::Private => M8SecurityClass::Private,
        })
    }
}

impl PrivateM8RuntimeAdmissionSnapshot {
    fn from_admission(admission: &M8RuntimeAdmission) -> Self {
        Self {
            program_identity: SnapshotCheckedProgramIdentity::from_checked(
                &admission.program_identity,
            ),
            evidence: admission
                .evidence
                .iter()
                .map(PrivateM8AdmissionEvidenceSnapshot::from_evidence)
                .collect(),
        }
    }

    fn into_admission(self) -> Result<M8RuntimeAdmission, M8I3PrivateSnapshotError> {
        Ok(M8RuntimeAdmission {
            program_identity: self
                .program_identity
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            evidence: self
                .evidence
                .into_iter()
                .map(PrivateM8AdmissionEvidenceSnapshot::into_evidence)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl PrivateM8AdmissionEvidenceSnapshot {
    fn from_evidence(evidence: &M8AdmissionEvidence) -> Self {
        match evidence {
            M8AdmissionEvidence::RelationVisibility {
                relation,
                label,
                redaction,
                source_ref,
            } => Self::RelationVisibility {
                relation: relation.clone(),
                label: PrivateEvidenceSecurityLabelSnapshot::from_label(label),
                redaction: redaction.0.clone(),
                source_ref: SnapshotSourceRef::from_checked(source_ref),
            },
            M8AdmissionEvidence::RelationLifetime {
                relation,
                live_lease,
                binding_frontier,
                source_ref,
            } => Self::RelationLifetime {
                relation: relation.clone(),
                live_lease: live_lease.clone(),
                binding_frontier: binding_frontier.clone(),
                source_ref: SnapshotSourceRef::from_checked(source_ref),
            },
            M8AdmissionEvidence::RelationFallbackValidity {
                relation,
                primary_epoch,
                fallback_epoch,
                source_ref,
            } => Self::RelationFallbackValidity {
                relation: relation.clone(),
                primary_epoch: primary_epoch.clone(),
                fallback_epoch: fallback_epoch.clone(),
                source_ref: SnapshotSourceRef::from_checked(source_ref),
            },
            M8AdmissionEvidence::ValueVisibilityRedaction {
                value,
                label,
                redaction,
                source_ref,
            } => Self::ValueVisibilityRedaction {
                value: value.clone(),
                label: PrivateEvidenceSecurityLabelSnapshot::from_label(label),
                redaction: redaction.0.clone(),
                source_ref: SnapshotSourceRef::from_checked(source_ref),
            },
            M8AdmissionEvidence::AuthDeferred {
                name,
                authority_label,
                source_ref,
            } => Self::AuthDeferred {
                name: name.clone(),
                authority_label: authority_label.clone(),
                source_ref: SnapshotSourceRef::from_checked(source_ref),
            },
            M8AdmissionEvidence::VerifyDeferred {
                name,
                theorem,
                witness_schema,
                source_ref,
            } => Self::VerifyDeferred {
                name: name.clone(),
                theorem: theorem.clone(),
                witness_schema: witness_schema.clone(),
                source_ref: SnapshotSourceRef::from_checked(source_ref),
            },
        }
    }

    fn into_evidence(self) -> Result<M8AdmissionEvidence, M8I3PrivateSnapshotError> {
        let source = |source_ref: SnapshotSourceRef| {
            source_ref
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)
        };
        match self {
            Self::RelationVisibility {
                relation,
                label,
                redaction,
                source_ref,
            } => Ok(M8AdmissionEvidence::RelationVisibility {
                relation,
                label: label.into_label(),
                redaction: EvidenceRedaction(redaction),
                source_ref: source(source_ref)?,
            }),
            Self::RelationLifetime {
                relation,
                live_lease,
                binding_frontier,
                source_ref,
            } => Ok(M8AdmissionEvidence::RelationLifetime {
                relation,
                live_lease,
                binding_frontier,
                source_ref: source(source_ref)?,
            }),
            Self::RelationFallbackValidity {
                relation,
                primary_epoch,
                fallback_epoch,
                source_ref,
            } => Ok(M8AdmissionEvidence::RelationFallbackValidity {
                relation,
                primary_epoch,
                fallback_epoch,
                source_ref: source(source_ref)?,
            }),
            Self::ValueVisibilityRedaction {
                value,
                label,
                redaction,
                source_ref,
            } => Ok(M8AdmissionEvidence::ValueVisibilityRedaction {
                value,
                label: label.into_label(),
                redaction: EvidenceRedaction(redaction),
                source_ref: source(source_ref)?,
            }),
            Self::AuthDeferred {
                name,
                authority_label,
                source_ref,
            } => Ok(M8AdmissionEvidence::AuthDeferred {
                name,
                authority_label,
                source_ref: source(source_ref)?,
            }),
            Self::VerifyDeferred {
                name,
                theorem,
                witness_schema,
                source_ref,
            } => Ok(M8AdmissionEvidence::VerifyDeferred {
                name,
                theorem,
                witness_schema,
                source_ref: source(source_ref)?,
            }),
        }
    }
}

impl PrivateM8DesignatedExecutionPlanSnapshot {
    fn from_plan(plan: &M8DesignatedExecutionPlan) -> Self {
        Self {
            name: plan.name.clone(),
            source_ref: SnapshotSourceRef::from_checked(&plan.source_ref),
            core: SnapshotDesignatedCheckedCore::from_checked(&plan.core),
            visibility_label: PrivateEvidenceSecurityLabelSnapshot::from_label(
                &plan.visibility_label,
            ),
            redaction: plan.redaction.0.clone(),
        }
    }

    fn into_plan(self) -> Result<M8DesignatedExecutionPlan, M8I3PrivateSnapshotError> {
        Ok(M8DesignatedExecutionPlan {
            name: self.name,
            source_ref: self
                .source_ref
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            core: self
                .core
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            visibility_label: self.visibility_label.into_label(),
            redaction: EvidenceRedaction(self.redaction),
        })
    }
}

impl PrivateM8OwnerExecutionPlanSnapshot {
    fn from_plan(plan: &M8OwnerExecutionPlan) -> Self {
        Self {
            evaluation: plan.evaluation.clone(),
            actor: plan.actor.clone(),
            owner_locus: plan.owner_locus.clone(),
            source_ref: SnapshotSourceRef::from_checked(&plan.source_ref),
            target: SnapshotTypedStateRead::from_checked(&plan.target),
            expression: SnapshotTypedExpression::from_checked(&plan.expression),
            owner_admission_budget: plan
                .owner_admission_budget
                .as_ref()
                .map(SnapshotOwnerAdmissionBudgetCondition::from_checked),
        }
    }

    fn into_plan(self) -> Result<M8OwnerExecutionPlan, M8I3PrivateSnapshotError> {
        let owner_locus = self.owner_locus;
        let owner_admission_budget = self
            .owner_admission_budget
            .map(|condition| condition.into_checked(&owner_locus))
            .transpose()
            .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?;
        Ok(M8OwnerExecutionPlan {
            evaluation: self.evaluation,
            actor: self.actor,
            owner_locus,
            source_ref: self
                .source_ref
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            target: self
                .target
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            expression: self
                .expression
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            owner_admission_budget,
        })
    }
}

impl PrivateM8RelationExecutionPlanSnapshot {
    fn from_plan(plan: &M8RelationExecutionPlan) -> Self {
        Self {
            name: plan.name.clone(),
            source_ref: SnapshotSourceRef::from_checked(&plan.source_ref),
            core: SnapshotRelationCheckedCore::from_checked(&plan.core),
            visibility_label: PrivateEvidenceSecurityLabelSnapshot::from_label(
                &plan.visibility_label,
            ),
            redaction: plan.redaction.0.clone(),
            live_lease_ref: plan.live_lease_ref.clone(),
            binding_frontier: plan.binding_frontier.clone(),
            primary_epoch: plan.primary_epoch.clone(),
            fallback_epoch: plan.fallback_epoch.clone(),
        }
    }

    fn into_plan(self) -> Result<M8RelationExecutionPlan, M8I3PrivateSnapshotError> {
        let plan = M8RelationExecutionPlan {
            name: self.name,
            source_ref: self
                .source_ref
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            core: self
                .core
                .into_checked()
                .map_err(|_| M8I3PrivateSnapshotError::SemanticSnapshot)?,
            visibility_label: self.visibility_label.into_label(),
            redaction: EvidenceRedaction(self.redaction),
            live_lease_ref: self.live_lease_ref,
            binding_frontier: self.binding_frontier,
            primary_epoch: self.primary_epoch,
            fallback_epoch: self.fallback_epoch,
        };
        plan.has_exact_admission_evidence()
            .then_some(plan)
            .ok_or(M8I3PrivateSnapshotError::StructuralMismatch)
    }
}

fn relation_payload_diagnostic(
    checked: &CheckedSurfaceV0,
    admission: &M8RuntimeAdmission,
) -> Option<M8AdmissionDiagnostic> {
    for evaluation in checked.evaluations() {
        let Some(core) = evaluation.relation_core() else {
            continue;
        };
        let relation = evaluation.name();
        let source_ref = evaluation.source_ref();
        let residual = |kind| {
            checked
                .residual_obligations()
                .entries()
                .iter()
                .find(|entry| {
                    entry.kind() == kind
                        && entry.name() == relation
                        && entry.source_ref() == source_ref
                })
        };
        let lifetime = admission.evidence.iter().find(|evidence| {
            matches!(
                evidence,
                M8AdmissionEvidence::RelationLifetime {
                    relation: evidence_relation,
                    source_ref: evidence_ref,
                    ..
                } if evidence_relation == relation && evidence_ref == source_ref
            )
        });
        if !matches!(
            lifetime,
            Some(M8AdmissionEvidence::RelationLifetime {
                live_lease,
                binding_frontier,
                ..
            }) if !live_lease.is_empty()
                && binding_frontier
                    == core
                        .binding_frontier()
                        .as_slice()
                        .first()
                        .expect("M7 relation binding frontier is finite and nonempty")
                        .as_str()
        ) {
            return residual(ResidualObligationKind::RelationLifetime)
                .map(M8AdmissionDiagnostic::relation_payload_mismatch);
        }
        let fallback = admission.evidence.iter().find(|evidence| {
            matches!(
                evidence,
                M8AdmissionEvidence::RelationFallbackValidity {
                    relation: evidence_relation,
                    source_ref: evidence_ref,
                    ..
                } if evidence_relation == relation && evidence_ref == source_ref
            )
        });
        if !matches!(
            fallback,
            Some(M8AdmissionEvidence::RelationFallbackValidity {
                primary_epoch,
                fallback_epoch,
                ..
            }) if primary_epoch == core.primary().epoch()
                && fallback_epoch == core.fallback().epoch()
        ) {
            return residual(ResidualObligationKind::FallbackValidity)
                .map(M8AdmissionDiagnostic::relation_payload_mismatch);
        }
        let visibility = admission.evidence.iter().find(|evidence| {
            matches!(
                evidence,
                M8AdmissionEvidence::RelationVisibility {
                    relation: evidence_relation,
                    source_ref: evidence_ref,
                    ..
                } if evidence_relation == relation && evidence_ref == source_ref
            )
        });
        if !matches!(
            visibility,
            Some(M8AdmissionEvidence::RelationVisibility {
                label, redaction, ..
            }) if !label.as_str().is_empty() && !redaction.as_str().is_empty()
        ) {
            return residual(ResidualObligationKind::Visibility)
                .map(M8AdmissionDiagnostic::relation_payload_mismatch);
        }
    }
    None
}

#[cfg(test)]
#[path = "m8_owner_budget_snapshot_tests.rs"]
mod m8_owner_budget_snapshot_tests;
