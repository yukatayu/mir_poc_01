//! Internal SYS-1 semantic runtime kernel.
//!
//! This module owns the bounded typed request lifecycle that sits below the
//! M10 conformance facade and above later projection/dispatch work.  It does
//! not define a public wire format, interpret un-checked source, issue
//! authority, or select a transport.  Every accepted owner request remains
//! tied to checked Core and to a sealed M9 membership/capability/witness
//! lineage.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_classification::SourceToCoreKind,
    surface_v0_pipeline::{
        CheckedBinaryOperator, CheckedExpressionTree, CheckedSurfaceV0, EffectKind, TypedStateRead,
    },
};

use crate::{
    m8_runtime_local_cut::{M8LeaseRecord, M8LocalRuntime, M8LocalRuntimeSeed},
    m8_runtime_owner_queue::{M8AuthorityUse, M8DeclaredFailure, M8OwnerRequest, M8StateKey},
    m9_auth_verification::{
        M9_REMOTE_INPUT_VISIBILITY_RESTRICTED_REDACTED, M9KernelDesignatedRemoteInputLineage,
        M9KernelOwnerLineage, M9RuntimeExecutionSeam,
        canonical_designated_remote_input_release_label,
    },
};

/// A state key is a checked Core-local coordinate, not a remote-store handle.
pub(crate) type KernelStateKey = M8StateKey;

macro_rules! opaque_ref {
    ($name:ident) => {
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub(crate) struct $name(String);

        impl $name {
            pub(crate) fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[allow(dead_code)]
            pub(crate) fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(concat!(stringify!($name), "(<sealed>)"))
            }
        }
    };
}

opaque_ref!(PrincipalRef);
opaque_ref!(LocusRef);
opaque_ref!(CapabilityRef);
opaque_ref!(WitnessRef);
opaque_ref!(MembershipEpoch);
opaque_ref!(MembershipIncarnation);
opaque_ref!(OperationId);
opaque_ref!(RequestIdentity);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SemanticValue {
    Int(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FailureKind {
    StaleMembership,
    RouteUnavailable,
    MissingCapability,
    MissingWitness,
    VisibilityDenied,
    /// An accepted checked Core row can conservatively carry a future
    /// declared failure name without granting a new runtime behavior.
    Declared(String),
}

impl FailureKind {
    fn from_checked_name(name: &str) -> Self {
        match name {
            "StaleMembership" => Self::StaleMembership,
            "MissingCapability" => Self::MissingCapability,
            "MissingWitness" => Self::MissingWitness,
            "RouteUnavailable" => Self::RouteUnavailable,
            "VisibilityDenied" => Self::VisibilityDenied,
            other => Self::Declared(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FailureRow(Vec<FailureKind>);

impl FailureRow {
    pub(crate) fn new(kinds: impl IntoIterator<Item = FailureKind>) -> Self {
        let mut entries = kinds.into_iter().collect::<Vec<_>>();
        entries.sort_unstable();
        entries.dedup();
        Self(entries)
    }

    pub(crate) fn contains(&self, kind: FailureKind) -> bool {
        self.0.contains(&kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct EffectRow(Vec<EffectKind>);

impl EffectRow {
    pub(crate) fn new(kinds: impl IntoIterator<Item = EffectKind>) -> Self {
        let mut entries = Vec::new();
        for kind in kinds {
            if !entries.contains(&kind) {
                entries.push(kind);
            }
        }
        Self(entries)
    }

    pub(crate) fn contains(&self, kind: EffectKind) -> bool {
        self.0.contains(&kind)
    }

    pub(crate) fn entries(&self) -> &[EffectKind] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VisibilityClass {
    ObserverSafeRedacted,
    RestrictedRedacted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RedactionPolicy {
    visibility: VisibilityClass,
}

impl RedactionPolicy {
    fn from_visibility(visibility: VisibilityClass) -> Self {
        Self { visibility }
    }

    pub(crate) const fn is_observer_safe(&self) -> bool {
        matches!(self.visibility, VisibilityClass::ObserverSafeRedacted)
    }

    pub(crate) const fn visibility_class(&self) -> VisibilityClass {
        self.visibility
    }
}

/// Checked source/Core lineage retained by an internal carrier.  This is a
/// runtime validation value, never a Core constructor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceCoreProvenance {
    program_identity: String,
    source_ref: SourceRef,
    core_ref: String,
    effect_row: EffectRow,
    failure_row: FailureRow,
    redaction: RedactionPolicy,
}

impl SourceCoreProvenance {
    /// Construct canonical owner provenance only from an already checked
    /// owner operation.  This is fallible because a malformed internal
    /// operation/source-map pairing is a diagnostic boundary, not an excuse
    /// to panic while constructing a carrier.
    pub(crate) fn try_from_checked_owner_operation(
        checked: &CheckedSurfaceV0,
        operation: &str,
    ) -> Result<Self, KernelDiagnostics> {
        let evaluation = checked
            .evaluation(operation)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownOperation))?;
        let source_ref = evaluation.source_ref().clone();
        let core_ref = checked
            .source_map()
            .entries()
            .iter()
            .find(|entry| {
                entry.kind() == SourceToCoreKind::OwnerRmw && entry.source_ref() == &source_ref
            })
            .ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::SourceCoreProvenanceMismatch)
            })?
            .core_ref()
            .to_string();
        let effect_row = EffectRow::new(
            evaluation
                .effect_row()
                .entries()
                .iter()
                .map(|entry| entry.kind()),
        );
        let failure_row = FailureRow::new(
            evaluation
                .declared_failure_row()
                .names()
                .into_iter()
                .chain(evaluation.generated_failure_row().names())
                .map(|name| FailureKind::from_checked_name(&name)),
        );
        let redaction = RedactionPolicy::from_visibility(
            if evaluation.effect_row().entries().iter().any(|entry| {
                entry.kind() == EffectKind::ObserverPublish
                    && entry.redaction_label() == "observer_safe"
            }) {
                VisibilityClass::ObserverSafeRedacted
            } else {
                VisibilityClass::RestrictedRedacted
            },
        );
        Ok(Self {
            program_identity: checked.program_identity().stable_key(),
            source_ref,
            core_ref,
            effect_row,
            failure_row,
            redaction,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_checked_owner_operation(
        checked: &CheckedSurfaceV0,
        operation: &str,
    ) -> Self {
        Self::try_from_checked_owner_operation(checked, operation)
            .expect("test fixture supplies a checked owner operation")
    }

    pub(crate) fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = source_ref;
        self
    }

    pub(crate) fn with_visibility(mut self, visibility: VisibilityClass) -> Self {
        self.redaction = RedactionPolicy::from_visibility(visibility);
        self
    }

    #[cfg(test)]
    pub(crate) fn without_failure_for_test(mut self, failure: FailureKind) -> Self {
        self.failure_row.0.retain(|candidate| candidate != &failure);
        self
    }

    pub(crate) fn core_ref(&self) -> &str {
        &self.core_ref
    }

    fn matches(&self, expected: &Self) -> bool {
        self == expected
    }
}

fn canonical_checked_owner_provenance(
    checked: &CheckedSurfaceV0,
    operation: &str,
) -> Result<SourceCoreProvenance, KernelDiagnostics> {
    SourceCoreProvenance::try_from_checked_owner_operation(checked, operation)
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct OwnerRequestCarrier {
    operation: OperationId,
    request_identity: Option<RequestIdentity>,
    request_occurrence: Option<OccurrenceRef>,
    origin_principal: Option<PrincipalRef>,
    origin_locus: Option<LocusRef>,
    target_owner: Option<LocusRef>,
    arguments: BTreeMap<String, String>,
    membership_ref: Option<String>,
    membership_epoch: Option<MembershipEpoch>,
    membership_incarnation: Option<MembershipIncarnation>,
    capability_ref: Option<CapabilityRef>,
    witness_ref: Option<WitnessRef>,
    provenance: Option<SourceCoreProvenance>,
}

impl std::fmt::Debug for OwnerRequestCarrier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OwnerRequestCarrier")
            .field("operation", &self.operation.as_str())
            .field("request_identity", &self.request_identity)
            .field("source_bound", &self.provenance.is_some())
            .finish_non_exhaustive()
    }
}

impl OwnerRequestCarrier {
    pub(crate) fn new(operation: OperationId) -> Self {
        Self {
            operation,
            request_identity: None,
            request_occurrence: None,
            origin_principal: None,
            origin_locus: None,
            target_owner: None,
            arguments: BTreeMap::new(),
            membership_ref: None,
            membership_epoch: None,
            membership_incarnation: None,
            capability_ref: None,
            witness_ref: None,
            provenance: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn source_free_for_test(operation: OperationId) -> Self {
        Self::new(operation)
    }

    pub(crate) fn with_origin(mut self, principal: PrincipalRef, locus: LocusRef) -> Self {
        self.origin_principal = Some(principal);
        self.origin_locus = Some(locus);
        self
    }

    pub(crate) fn with_target_owner(mut self, locus: LocusRef) -> Self {
        self.target_owner = Some(locus);
        self
    }

    pub(crate) fn with_argument(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.arguments.insert(name.into(), value.into());
        self
    }

    pub(crate) fn with_membership_ref(mut self, reference: impl Into<String>) -> Self {
        self.membership_ref = Some(reference.into());
        self
    }

    pub(crate) fn with_membership_epoch(mut self, epoch: MembershipEpoch) -> Self {
        self.membership_epoch = Some(epoch);
        self
    }

    pub(crate) fn with_membership_incarnation(
        mut self,
        incarnation: MembershipIncarnation,
    ) -> Self {
        self.membership_incarnation = Some(incarnation);
        self
    }

    pub(crate) fn with_capability_ref(mut self, reference: CapabilityRef) -> Self {
        self.capability_ref = Some(reference);
        self
    }

    pub(crate) fn with_witness_ref(mut self, reference: WitnessRef) -> Self {
        self.witness_ref = Some(reference);
        self
    }

    pub(crate) fn with_provenance(mut self, provenance: SourceCoreProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }

    pub(crate) fn with_request_identity(mut self, identity: RequestIdentity) -> Self {
        self.request_identity = Some(identity);
        self
    }

    pub(crate) fn without_capability_for_kernel_falsifier(mut self) -> Self {
        self.capability_ref = None;
        self
    }

    pub(crate) fn with_kernel_falsifier_capability(mut self, reference: CapabilityRef) -> Self {
        self.capability_ref = Some(reference);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QueuePosition(u64);

impl QueuePosition {
    pub(crate) fn stable_debug_id(&self) -> String {
        format!("queue:{:020}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QueuedOwnerRequest {
    carrier: OwnerRequestCarrier,
    queue_position: QueuePosition,
}

impl QueuedOwnerRequest {
    pub(crate) fn request_identity(&self) -> &RequestIdentity {
        self.carrier
            .request_identity
            .as_ref()
            .expect("queued carrier always has an assigned identity")
    }

    pub(crate) fn queue_position(&self) -> &QueuePosition {
        &self.queue_position
    }

    pub(crate) fn carrier(&self) -> &OwnerRequestCarrier {
        &self.carrier
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OccurrenceRef {
    stable_id: String,
    sequence: u64,
}

impl OccurrenceRef {
    fn new(kind: &str, sequence: u64) -> Self {
        Self {
            stable_id: format!("{kind}:{sequence:020}"),
            sequence,
        }
    }

    fn is_concrete(&self) -> bool {
        !self.stable_id.is_empty()
    }
}

/// The request lifecycle is retained as occurrence references rather than as
/// queue positions.  It is an internal diagnostic carrier, never a scheduler
/// authority or public transport receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OccurrenceLifecycle {
    request: OccurrenceRef,
    serve: OccurrenceRef,
    reply: OccurrenceRef,
    receive: OccurrenceRef,
}

impl OccurrenceLifecycle {
    fn new(
        request: OccurrenceRef,
        serve: OccurrenceRef,
        reply: OccurrenceRef,
        receive: OccurrenceRef,
    ) -> Self {
        Self {
            request,
            serve,
            reply,
            receive,
        }
    }

    pub(crate) fn all_ids_are_concrete(&self) -> bool {
        self.request.is_concrete()
            && self.serve.is_concrete()
            && self.reply.is_concrete()
            && self.receive.is_concrete()
    }

    pub(crate) fn strictly_orders_request_serve_reply_receive(&self) -> bool {
        self.request.sequence < self.serve.sequence
            && self.serve.sequence < self.reply.sequence
            && self.reply.sequence < self.receive.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ServedOwnerRequest {
    request_identity: RequestIdentity,
    serve_occurrence: OccurrenceRef,
}

impl ServedOwnerRequest {
    pub(crate) fn serve_occurrence(&self) -> &OccurrenceRef {
        &self.serve_occurrence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KernelOutcome {
    Success(SemanticValue),
    DeclaredFailure(FailureKind),
}

impl KernelOutcome {
    fn failure(&self) -> Option<FailureKind> {
        match self {
            Self::Success(_) => None,
            Self::DeclaredFailure(failure) => Some(failure.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KernelReply {
    request_identity: RequestIdentity,
    serve_occurrence: OccurrenceRef,
    reply_occurrence: OccurrenceRef,
    outcome: KernelOutcome,
}

impl KernelReply {
    pub(crate) fn failure(&self) -> Option<FailureKind> {
        self.outcome.failure()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct KernelReceipt {
    request_identity: RequestIdentity,
    origin_principal: PrincipalRef,
    origin_locus: LocusRef,
    target_owner: LocusRef,
    operation: OperationId,
    source_ref: SourceRef,
    core_ref: String,
    effect_row: EffectRow,
    failure_row: FailureRow,
    capability_refs: Vec<CapabilityRef>,
    witness_refs: Vec<WitnessRef>,
    membership_epoch: MembershipEpoch,
    membership_incarnation: MembershipIncarnation,
    redaction: RedactionPolicy,
    outcome: KernelOutcome,
    occurrences: OccurrenceLifecycle,
}

impl std::fmt::Debug for KernelReceipt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("KernelReceipt")
            .field("request_identity", &self.request_identity)
            .field("operation", &self.operation.as_str())
            .field("source_ref", &self.source_ref)
            .field("redaction", &self.redaction.is_observer_safe())
            .finish()
    }
}

impl KernelReceipt {
    pub(crate) fn request_identity(&self) -> &RequestIdentity {
        &self.request_identity
    }

    pub(crate) fn origin_principal(&self) -> &PrincipalRef {
        &self.origin_principal
    }

    pub(crate) fn origin_locus(&self) -> &LocusRef {
        &self.origin_locus
    }

    pub(crate) fn target_owner(&self) -> &LocusRef {
        &self.target_owner
    }

    pub(crate) fn operation(&self) -> &OperationId {
        &self.operation
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub(crate) fn effect_row(&self) -> &EffectRow {
        &self.effect_row
    }

    pub(crate) fn failure_row(&self) -> &FailureRow {
        &self.failure_row
    }

    pub(crate) fn capability_refs(&self) -> Vec<CapabilityRef> {
        self.capability_refs.clone()
    }

    pub(crate) fn witness_refs(&self) -> Vec<WitnessRef> {
        self.witness_refs.clone()
    }

    pub(crate) fn membership_epoch(&self) -> &MembershipEpoch {
        &self.membership_epoch
    }

    pub(crate) fn membership_incarnation(&self) -> &MembershipIncarnation {
        &self.membership_incarnation
    }

    pub(crate) const fn visibility_class(&self) -> VisibilityClass {
        self.redaction.visibility_class()
    }

    pub(crate) fn failure(&self) -> Option<FailureKind> {
        self.outcome.failure()
    }

    pub(crate) fn occurrences(&self) -> OccurrenceLifecycle {
        self.occurrences.clone()
    }
}

opaque_ref!(InputFrontier);
opaque_ref!(RemoteInputReceiptId);

/// Exact release contract for one source-owner input read.  It is an internal
/// semantic tuple, not a transport header and not an authority issuer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputReleaseTuple {
    origin_principal: PrincipalRef,
    source_owner: LocusRef,
    target_evaluator: LocusRef,
    label: String,
    visibility: VisibilityClass,
}

impl RemoteInputReleaseTuple {
    pub(crate) fn new(
        origin_principal: PrincipalRef,
        source_owner: LocusRef,
        target_evaluator: LocusRef,
        label: impl Into<String>,
    ) -> Self {
        Self {
            origin_principal,
            source_owner,
            target_evaluator,
            label: label.into(),
            visibility: VisibilityClass::RestrictedRedacted,
        }
    }

    pub(crate) fn with_visibility(mut self, visibility: VisibilityClass) -> Self {
        self.visibility = visibility;
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputRequestCarrier {
    evaluator: String,
    result: String,
    dependency_index: usize,
    request_identity: Option<RequestIdentity>,
    request_occurrence: Option<OccurrenceRef>,
    origin_principal: Option<PrincipalRef>,
    origin_locus: Option<LocusRef>,
    source_owner: Option<LocusRef>,
    target_evaluator: Option<LocusRef>,
    input_frontier: Option<InputFrontier>,
    release_tuple: Option<RemoteInputReleaseTuple>,
    membership_ref: Option<String>,
    membership_epoch: Option<MembershipEpoch>,
    membership_incarnation: Option<MembershipIncarnation>,
    capability_ref: Option<CapabilityRef>,
    witness_ref: Option<WitnessRef>,
    source_ref: Option<SourceRef>,
}

impl std::fmt::Debug for RemoteInputRequestCarrier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RemoteInputRequestCarrier")
            .field("evaluator", &self.evaluator)
            .field("result", &self.result)
            .field("dependency_index", &self.dependency_index)
            .field("request_identity", &self.request_identity)
            .field("source_bound", &self.source_ref.is_some())
            .finish_non_exhaustive()
    }
}

impl RemoteInputRequestCarrier {
    /// Build the skeletal internal carrier for exactly one checked remote
    /// input dependency.  The caller still cannot choose its sealed lineage;
    /// malformed checked references become typed diagnostics rather than a
    /// panic in crate-visible construction code.
    pub(crate) fn try_from_checked_designated_dependency(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Result<Self, KernelDiagnostics> {
        let dependency = checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| {
                core.generated_remote_input_dependencies()
                    .get(dependency_index)
            })
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownOperation))?;
        Ok(Self {
            evaluator: evaluator.to_string(),
            result: result.to_string(),
            dependency_index,
            request_identity: None,
            request_occurrence: None,
            origin_principal: None,
            origin_locus: None,
            source_owner: Some(LocusRef::new(dependency.source_owner_locus())),
            target_evaluator: Some(LocusRef::new(evaluator)),
            input_frontier: None,
            release_tuple: None,
            membership_ref: None,
            membership_epoch: None,
            membership_incarnation: None,
            capability_ref: None,
            witness_ref: None,
            source_ref: Some(dependency.typed_state_read().source_ref()),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_checked_designated_dependency(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Self {
        Self::try_from_checked_designated_dependency(checked, evaluator, result, dependency_index)
            .expect("test fixture supplies a checked designated dependency")
    }

    pub(crate) fn with_origin(mut self, principal: PrincipalRef, locus: LocusRef) -> Self {
        self.origin_principal = Some(principal);
        self.origin_locus = Some(locus);
        self
    }

    pub(crate) fn with_source_owner(mut self, source_owner: LocusRef) -> Self {
        self.source_owner = Some(source_owner);
        self
    }

    pub(crate) fn with_target_evaluator(mut self, target_evaluator: LocusRef) -> Self {
        self.target_evaluator = Some(target_evaluator);
        self
    }

    pub(crate) fn with_input_frontier(mut self, input_frontier: InputFrontier) -> Self {
        self.input_frontier = Some(input_frontier);
        self
    }

    pub(crate) fn with_release_tuple(mut self, release_tuple: RemoteInputReleaseTuple) -> Self {
        self.release_tuple = Some(release_tuple);
        self
    }

    pub(crate) fn with_membership_ref(mut self, reference: impl Into<String>) -> Self {
        self.membership_ref = Some(reference.into());
        self
    }

    pub(crate) fn with_membership_epoch(mut self, epoch: MembershipEpoch) -> Self {
        self.membership_epoch = Some(epoch);
        self
    }

    pub(crate) fn with_membership_incarnation(
        mut self,
        incarnation: MembershipIncarnation,
    ) -> Self {
        self.membership_incarnation = Some(incarnation);
        self
    }

    pub(crate) fn with_capability_ref(mut self, reference: CapabilityRef) -> Self {
        self.capability_ref = Some(reference);
        self
    }

    pub(crate) fn with_witness_ref(mut self, reference: WitnessRef) -> Self {
        self.witness_ref = Some(reference);
        self
    }

    pub(crate) fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QueuedRemoteInputRequest {
    carrier: RemoteInputRequestCarrier,
    queue_position: QueuePosition,
}

impl QueuedRemoteInputRequest {
    pub(crate) fn request_identity(&self) -> &RequestIdentity {
        self.carrier
            .request_identity
            .as_ref()
            .expect("queued remote input carrier has an identity")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ServedRemoteInputRequest {
    request_identity: RequestIdentity,
    serve_occurrence: OccurrenceRef,
}

impl ServedRemoteInputRequest {
    pub(crate) fn serve_occurrence(&self) -> &OccurrenceRef {
        &self.serve_occurrence
    }
}

#[derive(Clone, PartialEq, Eq)]
enum RemoteInputResultInner {
    Success(SemanticValue),
    #[cfg(test)]
    PanicIfInspected(String),
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputResult(RemoteInputResultInner);

impl RemoteInputResult {
    pub(crate) fn success(value: SemanticValue) -> Self {
        Self(RemoteInputResultInner::Success(value))
    }

    #[cfg(test)]
    pub(crate) fn panic_if_inspected_for_test(message: impl Into<String>) -> Self {
        Self(RemoteInputResultInner::PanicIfInspected(message.into()))
    }

    fn into_value(self) -> SemanticValue {
        match self.0 {
            RemoteInputResultInner::Success(value) => value,
            #[cfg(test)]
            RemoteInputResultInner::PanicIfInspected(message) => panic!("{message}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputReply {
    request_identity: RequestIdentity,
    serve_occurrence: OccurrenceRef,
    reply_occurrence: OccurrenceRef,
    outcome: KernelOutcome,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputReceipt {
    receipt_id: RemoteInputReceiptId,
    request_identity: RequestIdentity,
    origin_principal: PrincipalRef,
    source_owner: LocusRef,
    target_evaluator: LocusRef,
    release_tuple: RemoteInputReleaseTuple,
    input_frontier: InputFrontier,
    source_ref: SourceRef,
    core_ref: String,
    effect_row: EffectRow,
    failure_row: FailureRow,
    outcome: KernelOutcome,
    membership_epoch: MembershipEpoch,
    membership_incarnation: MembershipIncarnation,
    capability_refs: Vec<CapabilityRef>,
    witness_refs: Vec<WitnessRef>,
    occurrences: OccurrenceLifecycle,
}

impl std::fmt::Debug for RemoteInputReceipt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RemoteInputReceipt")
            .field("receipt_id", &self.receipt_id)
            .field("request_identity", &self.request_identity)
            .field("source_owner", &self.source_owner)
            .field("target_evaluator", &self.target_evaluator)
            .field("visibility", &self.release_tuple.visibility)
            .finish()
    }
}

impl RemoteInputReceipt {
    pub(crate) fn receipt_id(&self) -> &RemoteInputReceiptId {
        &self.receipt_id
    }

    pub(crate) fn request_identity(&self) -> &RequestIdentity {
        &self.request_identity
    }

    pub(crate) fn source_owner(&self) -> &LocusRef {
        &self.source_owner
    }

    pub(crate) fn target_evaluator(&self) -> &LocusRef {
        &self.target_evaluator
    }

    pub(crate) fn release_tuple(&self) -> &RemoteInputReleaseTuple {
        &self.release_tuple
    }

    pub(crate) fn input_frontier(&self) -> &InputFrontier {
        &self.input_frontier
    }

    pub(crate) const fn visibility_class(&self) -> VisibilityClass {
        self.release_tuple.visibility
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn effect_row(&self) -> &EffectRow {
        &self.effect_row
    }

    pub(crate) fn failure_row(&self) -> &FailureRow {
        &self.failure_row
    }

    pub(crate) fn value(&self) -> Option<&SemanticValue> {
        match &self.outcome {
            KernelOutcome::Success(value) => Some(value),
            KernelOutcome::DeclaredFailure(_) => None,
        }
    }

    pub(crate) fn membership_incarnation(&self) -> &MembershipIncarnation {
        &self.membership_incarnation
    }

    pub(crate) fn failure(&self) -> Option<FailureKind> {
        self.outcome.failure()
    }

    pub(crate) fn occurrences(&self) -> OccurrenceLifecycle {
        self.occurrences.clone()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputConsumeRequest {
    evaluator: String,
    result: String,
    dependency_index: usize,
    receipt_id: Option<RemoteInputReceiptId>,
    evaluator_locus: Option<LocusRef>,
}

impl RemoteInputConsumeRequest {
    pub(crate) fn try_from_checked_designated_dependency(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Result<Self, KernelDiagnostics> {
        checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| {
                core.generated_remote_input_dependencies()
                    .get(dependency_index)
            })
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownOperation))?;
        Ok(Self {
            evaluator: evaluator.to_string(),
            result: result.to_string(),
            dependency_index,
            receipt_id: None,
            evaluator_locus: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_checked_designated_dependency(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Self {
        Self::try_from_checked_designated_dependency(checked, evaluator, result, dependency_index)
            .expect("test fixture supplies a checked designated dependency")
    }

    pub(crate) fn with_receipt(mut self, receipt_id: &RemoteInputReceiptId) -> Self {
        self.receipt_id = Some(receipt_id.clone());
        self
    }

    pub(crate) fn with_evaluator(mut self, evaluator_locus: LocusRef) -> Self {
        self.evaluator_locus = Some(evaluator_locus);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConsumedRemoteInput {
    value: Option<SemanticValue>,
}

impl ConsumedRemoteInput {
    pub(crate) fn value(&self) -> Option<&SemanticValue> {
        self.value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ReceiptStore {
    owner: BTreeMap<RequestIdentity, KernelReceipt>,
    remote_input: BTreeMap<RemoteInputReceiptId, RemoteInputReceipt>,
}

impl ReceiptStore {
    pub(crate) fn contains(&self, identity: &RequestIdentity) -> bool {
        self.owner.contains_key(identity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct SemanticSnapshot {
    ints: BTreeMap<KernelStateKey, i64>,
}

impl SemanticSnapshot {
    pub(crate) fn int(&self, key: &KernelStateKey) -> Option<i64> {
        self.ints.get(key).copied()
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub(crate) struct AuthorityView {
    membership_refs: BTreeSet<String>,
    capability_refs: BTreeSet<CapabilityRef>,
    witness_refs: BTreeSet<WitnessRef>,
}

impl std::fmt::Debug for AuthorityView {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthorityView")
            .field("membership_count", &self.membership_refs.len())
            .field("capability_count", &self.capability_refs.len())
            .field("witness_count", &self.witness_refs.len())
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct KernelTrace {
    lifecycles: BTreeMap<RequestIdentity, Vec<&'static str>>,
    occurrences: BTreeMap<RequestIdentity, OccurrenceLifecycle>,
    typed_failures: BTreeMap<RequestIdentity, FailureKind>,
}

impl KernelTrace {
    pub(crate) fn lifecycle_for(&self, identity: &RequestIdentity) -> Vec<&'static str> {
        self.lifecycles.get(identity).cloned().unwrap_or_default()
    }

    fn append(&mut self, identity: &RequestIdentity, event: &'static str) {
        self.lifecycles
            .entry(identity.clone())
            .or_default()
            .push(event);
    }

    fn install_occurrences(
        &mut self,
        identity: &RequestIdentity,
        occurrences: OccurrenceLifecycle,
    ) {
        self.occurrences.insert(identity.clone(), occurrences);
    }

    fn record_typed_failure(&mut self, identity: &RequestIdentity, failure: FailureKind) {
        self.typed_failures.insert(identity.clone(), failure);
    }

    pub(crate) fn occurrences_for(
        &self,
        identity: &RequestIdentity,
    ) -> Option<OccurrenceLifecycle> {
        self.occurrences.get(identity).cloned()
    }

    pub(crate) fn contains_typed_failure_receipt(
        &self,
        identity: &RequestIdentity,
        failure: FailureKind,
    ) -> bool {
        self.typed_failures.get(identity) == Some(&failure)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KernelDiagnosticKind {
    WrongTargetOwner,
    SourceCoreProvenanceMismatch,
    UnknownRequestIdentity,
    DuplicateReply,
    DuplicateReceipt,
    StaleMembershipIncarnation,
    InputFrontierMismatch,
    ReleaseTupleMismatch,
    RemoteInputValueMismatch,
    SourceFreeCarrierRejected,
    AuthorityLineageRejected,
    UnknownOperation,
    QueueEmpty,
    OutOfOrderOwnerRequest,
    RouteUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KernelDiagnostic {
    kind: KernelDiagnosticKind,
}

impl KernelDiagnostic {
    pub(crate) const fn kind(&self) -> KernelDiagnosticKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KernelDiagnostics {
    primary: KernelDiagnostic,
}

impl KernelDiagnostics {
    fn one(kind: KernelDiagnosticKind) -> Self {
        Self {
            primary: KernelDiagnostic { kind },
        }
    }

    pub(crate) fn primary(&self) -> &KernelDiagnostic {
        &self.primary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct KernelSeed {
    ints: BTreeMap<KernelStateKey, i64>,
    live_leases: Vec<M8LeaseRecord>,
}

impl KernelSeed {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_int(mut self, key: KernelStateKey, value: i64) -> Self {
        self.ints.insert(key, value);
        self
    }

    /// Lease material remains an already-checked M8 runtime seed.  The
    /// semantic kernel carries it through without interpreting relation
    /// lifetime or projection policy itself.
    pub(crate) fn with_live_lease(mut self, lease: M8LeaseRecord) -> Self {
        self.live_leases.push(lease);
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
struct OwnerLineage {
    principal: PrincipalRef,
    owner_locus: LocusRef,
    membership_ref: String,
    membership_epoch: MembershipEpoch,
    membership_incarnation: MembershipIncarnation,
    capability_ref: CapabilityRef,
    witness_ref: WitnessRef,
    m8_authority_use: Option<M8AuthorityUse>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RemoteInputKey {
    evaluator: String,
    result: String,
    dependency_index: usize,
}

impl RemoteInputKey {
    fn new(
        evaluator: impl Into<String>,
        result: impl Into<String>,
        dependency_index: usize,
    ) -> Self {
        Self {
            evaluator: evaluator.into(),
            result: result.into(),
            dependency_index,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct RemoteInputLineage {
    origin_principal: PrincipalRef,
    source_owner: LocusRef,
    target_evaluator: LocusRef,
    membership_ref: String,
    membership_epoch: MembershipEpoch,
    membership_incarnation: MembershipIncarnation,
    capability_ref: CapabilityRef,
    witness_ref: WitnessRef,
    input_frontier: InputFrontier,
    release_tuple: RemoteInputReleaseTuple,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct SealedM9RuntimeAdmission {
    program_identity: String,
    owner_lineages: BTreeMap<OperationId, OwnerLineage>,
    remote_input_lineages: BTreeMap<RemoteInputKey, RemoteInputLineage>,
    #[cfg(test)]
    test_only: bool,
}

impl std::fmt::Debug for SealedM9RuntimeAdmission {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SealedM9RuntimeAdmission")
            .field("owner_operation_count", &self.owner_lineages.len())
            .field("remote_input_count", &self.remote_input_lineages.len())
            .finish()
    }
}

impl SealedM9RuntimeAdmission {
    /// Build an internal kernel admission only from final M9 facts carried by
    /// an execution seam.  A checked operation without a corresponding M9
    /// lineage is deliberately absent from this admission; it cannot be
    /// scheduled through the kernel.
    pub(crate) fn from_m9_execution_seam(
        checked: &CheckedSurfaceV0,
        seam: &M9RuntimeExecutionSeam,
    ) -> Result<Self, KernelDiagnostics> {
        let mut owner_lineages = BTreeMap::new();
        for evaluation in checked.evaluations() {
            let Some(owner) = evaluation.owner_rmw_core() else {
                continue;
            };
            let Some(lineage) = seam.kernel_owner_lineage(
                evaluation.name(),
                evaluation.actor_authority_origin(),
                owner.owner_locus(),
            ) else {
                continue;
            };
            let m8_authority_use = seam
                .owner_authority_use(
                    evaluation.name(),
                    evaluation.actor_authority_origin(),
                    owner.owner_locus(),
                )
                .ok_or_else(|| {
                    KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected)
                })?;
            owner_lineages.insert(
                OperationId::new(evaluation.name()),
                OwnerLineage::from_m9(lineage, Some(m8_authority_use)),
            );
        }
        let mut remote_input_lineages = BTreeMap::new();
        for evaluation in checked.evaluations() {
            let Some(designated) = evaluation.designated_core() else {
                continue;
            };
            let frontier = designated.trigger().frontier().ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::InputFrontierMismatch)
            })?;
            for (dependency_index, dependency) in designated
                .generated_remote_input_dependencies()
                .iter()
                .enumerate()
            {
                let Some(m9_lineage) = seam.kernel_designated_remote_input_lineage(
                    dependency.source_owner_locus(),
                    designated.evaluator(),
                    designated.result(),
                    dependency_index,
                    frontier,
                ) else {
                    continue;
                };
                let read = dependency.typed_state_read();
                let expected_release_label = canonical_designated_remote_input_release_label(
                    read.namespace(),
                    read.index(),
                    read.field(),
                    dependency.source_owner_locus(),
                    frontier,
                );
                // The scope has already been admitted by M9, but the kernel
                // still verifies the source-derived tuple before it turns the
                // sealed lineage into a carrier contract.  The carrier never
                // supplies producer or release metadata.
                if m9_lineage.producer_locus() != dependency.source_owner_locus()
                    || m9_lineage.evaluator() != designated.evaluator()
                    || m9_lineage.result() != designated.result()
                    || m9_lineage.dependency_index() != dependency_index
                    || m9_lineage.input_frontier() != frontier
                    || m9_lineage.release_label() != expected_release_label
                    || m9_lineage.visibility() != M9_REMOTE_INPUT_VISIBILITY_RESTRICTED_REDACTED
                {
                    return Err(KernelDiagnostics::one(
                        KernelDiagnosticKind::AuthorityLineageRejected,
                    ));
                }
                let release_tuple = RemoteInputReleaseTuple::new(
                    PrincipalRef::new(m9_lineage.principal()),
                    LocusRef::new(m9_lineage.producer_locus()),
                    LocusRef::new(m9_lineage.evaluator()),
                    m9_lineage.release_label(),
                )
                .with_visibility(VisibilityClass::RestrictedRedacted);
                remote_input_lineages.insert(
                    RemoteInputKey::new(
                        designated.evaluator(),
                        designated.result(),
                        dependency_index,
                    ),
                    RemoteInputLineage::from_m9(m9_lineage, release_tuple),
                );
            }
        }
        Ok(Self {
            program_identity: checked.program_identity().stable_key(),
            owner_lineages,
            remote_input_lineages,
            #[cfg(test)]
            test_only: false,
        })
    }

    /// Test-only visibility into the already M9-sealed lineage.  This cannot
    /// create or alter a producer-side release capability.
    #[cfg(test)]
    pub(crate) fn m9_sealed_remote_input_lineage_for_test(
        &self,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
    ) -> Option<&RemoteInputLineage> {
        self.remote_input_lineages
            .get(&RemoteInputKey::new(evaluator, result, dependency_index))
    }

    #[cfg(test)]
    pub(crate) fn test_seal_checked_owner_lineage(
        checked: &CheckedSurfaceV0,
        principal: PrincipalRef,
        owner_locus: LocusRef,
        membership_epoch: MembershipEpoch,
        membership_incarnation: MembershipIncarnation,
        capability_ref: CapabilityRef,
        witness_ref: WitnessRef,
    ) -> Self {
        let evaluation = checked
            .evaluations()
            .iter()
            .find(|evaluation| evaluation.owner_rmw_core().is_some())
            .expect("test checked source contains an owner operation");
        let membership_ref = format!(
            "membership:{}:{}:{}",
            principal.as_str(),
            owner_locus.as_str(),
            membership_epoch.as_str()
        );
        Self {
            program_identity: checked.program_identity().stable_key(),
            owner_lineages: BTreeMap::from([(
                OperationId::new(evaluation.name()),
                OwnerLineage {
                    membership_incarnation,
                    principal,
                    owner_locus,
                    membership_ref,
                    membership_epoch,
                    capability_ref,
                    witness_ref,
                    m8_authority_use: None,
                },
            )]),
            remote_input_lineages: BTreeMap::new(),
            test_only: true,
        }
    }

    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn test_seal_checked_designated_remote_input_lineage(
        checked: &CheckedSurfaceV0,
        evaluator: &str,
        result: &str,
        dependency_index: usize,
        origin_principal: PrincipalRef,
        source_owner: LocusRef,
        target_evaluator: LocusRef,
        membership_epoch: MembershipEpoch,
        membership_incarnation: MembershipIncarnation,
        release_tuple: RemoteInputReleaseTuple,
    ) -> Self {
        let dependency = checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| {
                core.generated_remote_input_dependencies()
                    .get(dependency_index)
            })
            .expect("test checked source contains the designated remote input");
        assert_eq!(dependency.source_owner_locus(), source_owner.as_str());
        assert_eq!(dependency.designated_evaluator(), target_evaluator.as_str());
        let input_frontier = checked
            .designated_result(evaluator, result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| core.trigger().frontier())
            .expect("test checked source has the designated input frontier");
        let membership_ref = format!(
            "membership:{}:{}:{}",
            origin_principal.as_str(),
            source_owner.as_str(),
            membership_epoch.as_str()
        );
        let capability_ref = CapabilityRef::new(format!(
            "cap:attack:{}:{}:{}",
            source_owner.as_str(),
            origin_principal.as_str(),
            membership_epoch.as_str()
        ));
        let witness_ref = WitnessRef::new(format!(
            "witness:attack:{}:{}:{}",
            source_owner.as_str(),
            origin_principal.as_str(),
            membership_epoch.as_str()
        ));
        Self {
            program_identity: checked.program_identity().stable_key(),
            owner_lineages: BTreeMap::new(),
            remote_input_lineages: BTreeMap::from([(
                RemoteInputKey::new(evaluator, result, dependency_index),
                RemoteInputLineage {
                    origin_principal,
                    source_owner,
                    target_evaluator,
                    membership_ref,
                    membership_epoch,
                    membership_incarnation,
                    capability_ref,
                    witness_ref,
                    input_frontier: InputFrontier::new(input_frontier),
                    release_tuple,
                },
            )]),
            test_only: true,
        }
    }
}

impl OwnerLineage {
    fn from_m9(lineage: M9KernelOwnerLineage, m8_authority_use: Option<M8AuthorityUse>) -> Self {
        Self {
            principal: PrincipalRef::new(lineage.principal()),
            owner_locus: LocusRef::new(lineage.owner_locus()),
            membership_ref: lineage.membership_ref().to_string(),
            membership_epoch: MembershipEpoch::new(lineage.membership_epoch()),
            membership_incarnation: MembershipIncarnation::new(lineage.membership_incarnation()),
            capability_ref: CapabilityRef::new(lineage.capability_ref()),
            witness_ref: WitnessRef::new(lineage.witness_ref()),
            m8_authority_use,
        }
    }
}

impl RemoteInputLineage {
    fn from_m9(
        lineage: M9KernelDesignatedRemoteInputLineage,
        release_tuple: RemoteInputReleaseTuple,
    ) -> Self {
        Self {
            origin_principal: PrincipalRef::new(lineage.principal()),
            source_owner: LocusRef::new(lineage.producer_locus()),
            target_evaluator: LocusRef::new(lineage.evaluator()),
            membership_ref: lineage.membership_ref().to_string(),
            membership_epoch: MembershipEpoch::new(lineage.membership_epoch()),
            membership_incarnation: MembershipIncarnation::new(lineage.membership_incarnation()),
            capability_ref: CapabilityRef::new(lineage.capability_ref()),
            witness_ref: WitnessRef::new(lineage.witness_ref()),
            input_frontier: InputFrontier::new(lineage.input_frontier()),
            release_tuple,
        }
    }

    #[cfg(test)]
    pub(crate) fn source_owner(&self) -> &LocusRef {
        &self.source_owner
    }

    #[cfg(test)]
    pub(crate) fn target_evaluator(&self) -> &LocusRef {
        &self.target_evaluator
    }

    #[cfg(test)]
    pub(crate) fn input_frontier(&self) -> &InputFrontier {
        &self.input_frontier
    }

    #[cfg(test)]
    pub(crate) fn release_tuple(&self) -> &RemoteInputReleaseTuple {
        &self.release_tuple
    }

    #[cfg(test)]
    pub(crate) const fn visibility_class(&self) -> VisibilityClass {
        self.release_tuple.visibility
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ServedOwnerState {
    carrier: OwnerRequestCarrier,
    outcome: KernelOutcome,
}

/// The reference constructor is used only by crate tests that pin kernel
/// semantics.  The production M9 entry owns the admitted M8 local runtime
/// for ordinary `run_source` and generic OwnerEvent paths.  Specialized M10
/// scenario runners remain legacy regression evidence outside this SYS-1
/// kernel claim.
enum OwnerExecutionBackend {
    Reference,
    AdmittedM8(Box<M8LocalRuntime>),
}

fn m8_declared_failure(failure: Option<M8DeclaredFailure>) -> FailureKind {
    match failure {
        Some(M8DeclaredFailure::MissingCapability) => FailureKind::MissingCapability,
        Some(M8DeclaredFailure::MissingWitness) => FailureKind::MissingWitness,
        Some(M8DeclaredFailure::StaleMembership) => FailureKind::StaleMembership,
        Some(M8DeclaredFailure::RouteUnavailable) | None => FailureKind::RouteUnavailable,
    }
}

/// The typed kernel state.  Profile/conformance/release orchestration lives
/// outside this type and depends on this kernel rather than the reverse.
pub(crate) struct SemanticRuntimeKernel {
    checked: CheckedSurfaceV0,
    admission: SealedM9RuntimeAdmission,
    semantic_snapshot: SemanticSnapshot,
    authority_view: AuthorityView,
    owner_backend: OwnerExecutionBackend,
    owner_queues: BTreeMap<LocusRef, VecDeque<QueuedOwnerRequest>>,
    served: BTreeMap<OccurrenceRef, ServedOwnerState>,
    replies: BTreeMap<OccurrenceRef, KernelReply>,
    remote_input_queues: BTreeMap<LocusRef, VecDeque<QueuedRemoteInputRequest>>,
    served_remote_inputs: BTreeMap<OccurrenceRef, RemoteInputRequestCarrier>,
    remote_input_replies: BTreeMap<OccurrenceRef, RemoteInputReply>,
    consumed_remote_receipts: BTreeSet<RemoteInputReceiptId>,
    receipt_store: ReceiptStore,
    trace: KernelTrace,
    next_request: u64,
    next_queue_position: u64,
    next_occurrence: u64,
}

impl SemanticRuntimeKernel {
    pub(crate) fn from_checked_m9(
        checked: CheckedSurfaceV0,
        admission: SealedM9RuntimeAdmission,
        seed: KernelSeed,
    ) -> Result<Self, KernelDiagnostics> {
        if checked.program_identity().stable_key() != admission.program_identity {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::SourceCoreProvenanceMismatch,
            ));
        }
        let authority_view = AuthorityView {
            membership_refs: admission
                .owner_lineages
                .values()
                .map(|lineage| lineage.membership_ref.clone())
                .chain(
                    admission
                        .remote_input_lineages
                        .values()
                        .map(|lineage| lineage.membership_ref.clone()),
                )
                .collect(),
            capability_refs: admission
                .owner_lineages
                .values()
                .map(|lineage| lineage.capability_ref.clone())
                .chain(
                    admission
                        .remote_input_lineages
                        .values()
                        .map(|lineage| lineage.capability_ref.clone()),
                )
                .collect(),
            witness_refs: admission
                .owner_lineages
                .values()
                .map(|lineage| lineage.witness_ref.clone())
                .chain(
                    admission
                        .remote_input_lineages
                        .values()
                        .map(|lineage| lineage.witness_ref.clone()),
                )
                .collect(),
        };
        Ok(Self {
            checked,
            admission,
            semantic_snapshot: SemanticSnapshot { ints: seed.ints },
            authority_view,
            owner_backend: OwnerExecutionBackend::Reference,
            owner_queues: BTreeMap::new(),
            served: BTreeMap::new(),
            replies: BTreeMap::new(),
            remote_input_queues: BTreeMap::new(),
            served_remote_inputs: BTreeMap::new(),
            remote_input_replies: BTreeMap::new(),
            consumed_remote_receipts: BTreeSet::new(),
            receipt_store: ReceiptStore::default(),
            trace: KernelTrace::default(),
            next_request: 0,
            next_queue_position: 0,
            next_occurrence: 0,
        })
    }

    /// Production entry point from an already-admitted M9 execution seam.
    pub(crate) fn from_m9_execution_seam(
        checked: CheckedSurfaceV0,
        seam: M9RuntimeExecutionSeam,
        seed: KernelSeed,
    ) -> Result<Self, KernelDiagnostics> {
        let admission = SealedM9RuntimeAdmission::from_m9_execution_seam(&checked, &seam)?;
        let seed_ints = seed.ints.clone();
        let seed_live_leases = seed.live_leases.clone();
        let (instance, authority_state) = seam.into_parts();
        let mut m8_seed = M8LocalRuntimeSeed::new().with_authority_state(authority_state);
        for (key, value) in seed_ints {
            m8_seed = m8_seed.with_owner_int(key, value);
        }
        for lease in seed_live_leases {
            m8_seed = m8_seed.with_live_lease(lease);
        }
        let mut kernel = Self::from_checked_m9(checked, admission, seed)?;
        kernel.owner_backend = OwnerExecutionBackend::AdmittedM8(Box::new(
            M8LocalRuntime::from_admitted(instance, m8_seed),
        ));
        Ok(kernel)
    }

    pub(crate) fn into_m8_runtime(self) -> Result<M8LocalRuntime, KernelDiagnostics> {
        match self.owner_backend {
            OwnerExecutionBackend::AdmittedM8(runtime) => Ok(*runtime),
            OwnerExecutionBackend::Reference => Err(KernelDiagnostics::one(
                KernelDiagnosticKind::AuthorityLineageRejected,
            )),
        }
    }

    pub(crate) fn m8_runtime_snapshot(&self) -> Result<M8LocalRuntime, KernelDiagnostics> {
        match &self.owner_backend {
            OwnerExecutionBackend::AdmittedM8(runtime) => Ok((**runtime).clone()),
            OwnerExecutionBackend::Reference => Err(KernelDiagnostics::one(
                KernelDiagnosticKind::AuthorityLineageRejected,
            )),
        }
    }

    pub(crate) fn enqueue_owner_request(
        &mut self,
        carrier: OwnerRequestCarrier,
    ) -> Result<QueuedOwnerRequest, KernelDiagnostics> {
        // Complete every carrier, identity, authority, and Core-provenance
        // check before touching M8.  A rejected request must not be left in
        // the M8 FIFO where a later serve could mutate owner state.
        self.validate_owner_carrier(&carrier, false)?;
        if carrier.request_occurrence.is_some() {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::UnknownRequestIdentity,
            ));
        }
        let identity = RequestIdentity::new(format!("request:{:020}", self.next_request));
        let target_owner = carrier
            .target_owner
            .clone()
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::WrongTargetOwner))?;
        let m8_request = match &self.owner_backend {
            OwnerExecutionBackend::AdmittedM8(_) => Some(self.m8_request_from_carrier(&carrier)?),
            OwnerExecutionBackend::Reference => None,
        };

        if let (OwnerExecutionBackend::AdmittedM8(runtime), Some(m8_request)) =
            (&mut self.owner_backend, m8_request)
        {
            runtime
                .enqueue_owner(m8_request)
                .map_err(|_| KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable))?;
        }

        let mut carrier = carrier;
        self.next_request += 1;
        carrier.request_identity = Some(identity.clone());
        carrier.request_occurrence = Some(self.next_occurrence("request"));
        let queued = QueuedOwnerRequest {
            carrier,
            queue_position: QueuePosition(self.next_queue_position),
        };
        self.next_queue_position += 1;
        self.owner_queues
            .entry(target_owner)
            .or_default()
            .push_back(queued.clone());
        self.trace.append(&identity, "request");
        Ok(queued)
    }

    /// Materialize an internal owner carrier from an already sealed M9
    /// lineage.  The caller supplies only source-schedule arguments and an
    /// origin locus; it cannot choose authority, target owner, or provenance.
    pub(crate) fn owner_request_from_admitted_lineage(
        &self,
        operation: &str,
        origin_locus: LocusRef,
        arguments: BTreeMap<String, String>,
    ) -> Result<OwnerRequestCarrier, KernelDiagnostics> {
        let operation_id = OperationId::new(operation);
        let lineage = self
            .admission
            .owner_lineages
            .get(&operation_id)
            .ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected)
            })?;
        let mut carrier = OwnerRequestCarrier::new(operation_id)
            .with_origin(lineage.principal.clone(), origin_locus)
            .with_target_owner(lineage.owner_locus.clone())
            .with_membership_ref(lineage.membership_ref.clone())
            .with_membership_epoch(lineage.membership_epoch.clone())
            .with_membership_incarnation(lineage.membership_incarnation.clone())
            .with_capability_ref(lineage.capability_ref.clone())
            .with_witness_ref(lineage.witness_ref.clone())
            .with_provenance(self.expected_owner_provenance(operation)?);
        for (name, value) in arguments {
            carrier = carrier.with_argument(name, value);
        }
        Ok(carrier)
    }

    pub(crate) fn serve_next_owner(
        &mut self,
        owner: LocusRef,
    ) -> Result<ServedOwnerRequest, KernelDiagnostics> {
        let Some(carrier) = self
            .owner_queues
            .get_mut(&owner)
            .and_then(VecDeque::pop_front)
            .map(|queued| queued.carrier)
        else {
            return Err(KernelDiagnostics::one(KernelDiagnosticKind::QueueEmpty));
        };
        self.serve_validated_owner_carrier(carrier)
    }

    /// Direct carrier service is deliberately defensive: it can only consume
    /// the exact carrier already issued by this kernel's queue.
    pub(crate) fn serve_owner_carrier(
        &mut self,
        carrier: OwnerRequestCarrier,
    ) -> Result<ServedOwnerRequest, KernelDiagnostics> {
        self.validate_owner_carrier(&carrier, true)?;
        let identity = carrier
            .request_identity
            .as_ref()
            .expect("validated served carrier has request identity")
            .clone();
        let target_owner = carrier
            .target_owner
            .as_ref()
            .expect("validated served carrier has target")
            .clone();
        // The M8 backend always serves the owner FIFO head.  Direct service
        // therefore may name only the same kernel FIFO head; selecting a
        // later carrier would misattribute M8's mutation and receipt.
        let queue = self
            .owner_queues
            .get(&target_owner)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownRequestIdentity))?;
        if queue.front().map(QueuedOwnerRequest::request_identity) != Some(&identity) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::OutOfOrderOwnerRequest,
            ));
        }
        let queued = self
            .owner_queues
            .get_mut(&target_owner)
            .and_then(VecDeque::pop_front)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::QueueEmpty))?;
        self.serve_validated_owner_carrier(queued.carrier)
    }

    pub(crate) fn reply_to_served_request(
        &mut self,
        serve_occurrence: &OccurrenceRef,
    ) -> Result<KernelReply, KernelDiagnostics> {
        if self.replies.contains_key(serve_occurrence) {
            return Err(KernelDiagnostics::one(KernelDiagnosticKind::DuplicateReply));
        }
        let (identity, outcome) = self
            .served
            .get(serve_occurrence)
            .map(|served| {
                (
                    served
                        .carrier
                        .request_identity
                        .as_ref()
                        .expect("served carrier has identity")
                        .clone(),
                    served.outcome.clone(),
                )
            })
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownRequestIdentity))?;
        let reply = KernelReply {
            request_identity: identity.clone(),
            serve_occurrence: serve_occurrence.clone(),
            reply_occurrence: self.next_occurrence("reply"),
            outcome,
        };
        self.replies.insert(serve_occurrence.clone(), reply.clone());
        self.trace.append(&identity, "reply");
        Ok(reply)
    }

    pub(crate) fn receive_reply(
        &mut self,
        reply: KernelReply,
    ) -> Result<KernelReceipt, KernelDiagnostics> {
        if self.receipt_store.contains(&reply.request_identity) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::DuplicateReceipt,
            ));
        }
        let Some(stored) = self.replies.get(&reply.serve_occurrence) else {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::UnknownRequestIdentity,
            ));
        };
        if stored != &reply {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::SourceCoreProvenanceMismatch,
            ));
        }
        let served = self
            .served
            .get(&reply.serve_occurrence)
            .expect("reply was made only for a served carrier")
            .clone();
        let receive_occurrence = self.next_occurrence("receive");
        let receipt = self.receipt_from_carrier(
            &served.carrier,
            reply.outcome.clone(),
            OccurrenceLifecycle::new(
                served
                    .carrier
                    .request_occurrence
                    .clone()
                    .expect("served carrier retains request occurrence"),
                reply.serve_occurrence.clone(),
                reply.reply_occurrence.clone(),
                receive_occurrence,
            ),
        )?;
        self.receipt_store
            .owner
            .insert(reply.request_identity.clone(), receipt.clone());
        self.trace
            .append(&reply.request_identity, "receive_receipt");
        self.trace
            .install_occurrences(&reply.request_identity, receipt.occurrences());
        if let Some(failure) = receipt.failure() {
            self.trace
                .record_typed_failure(&reply.request_identity, failure);
        }
        Ok(receipt)
    }

    /// Enqueue the one source-owner read generated by a checked designated
    /// dependency.  This is deliberately not a generic effect/provider API.
    pub(crate) fn enqueue_remote_input_request(
        &mut self,
        carrier: RemoteInputRequestCarrier,
    ) -> Result<QueuedRemoteInputRequest, KernelDiagnostics> {
        self.validate_remote_input_carrier(&carrier, false)?;
        let mut carrier = carrier;
        let identity = RequestIdentity::new(format!("remote-input:{:020}", self.next_request));
        self.next_request += 1;
        carrier.request_identity = Some(identity.clone());
        carrier.request_occurrence = Some(self.next_occurrence("remote-input-request"));
        let source_owner = carrier
            .source_owner
            .clone()
            .expect("validated remote input has source owner");
        let queued = QueuedRemoteInputRequest {
            carrier,
            queue_position: QueuePosition(self.next_queue_position),
        };
        self.next_queue_position += 1;
        self.remote_input_queues
            .entry(source_owner)
            .or_default()
            .push_back(queued.clone());
        self.trace.append(&identity, "request");
        Ok(queued)
    }

    /// Serve one generated source-owner read.  The operation only snapshots
    /// the checked source state; it has no semantic owner-write path.
    pub(crate) fn serve_next_remote_input(
        &mut self,
        source_owner: LocusRef,
    ) -> Result<ServedRemoteInputRequest, KernelDiagnostics> {
        let Some(carrier) = self
            .remote_input_queues
            .get_mut(&source_owner)
            .and_then(VecDeque::pop_front)
            .map(|queued| queued.carrier)
        else {
            return Err(KernelDiagnostics::one(KernelDiagnosticKind::QueueEmpty));
        };
        self.validate_remote_input_payload(&carrier)?;
        let identity = carrier
            .request_identity
            .as_ref()
            .expect("queued remote input has identity")
            .clone();
        let occurrence = self.next_occurrence("remote-input-serve");
        self.served_remote_inputs
            .insert(occurrence.clone(), carrier);
        self.trace.append(&identity, "serve");
        Ok(ServedRemoteInputRequest {
            request_identity: identity,
            serve_occurrence: occurrence,
        })
    }

    /// Attach the source-owner read result to an already-served remote input.
    /// Duplicate replies are rejected before inspecting their payload.
    pub(crate) fn reply_to_remote_input(
        &mut self,
        serve_occurrence: &OccurrenceRef,
        result: RemoteInputResult,
    ) -> Result<RemoteInputReply, KernelDiagnostics> {
        if self.remote_input_replies.contains_key(serve_occurrence) {
            return Err(KernelDiagnostics::one(KernelDiagnosticKind::DuplicateReply));
        }
        let carrier = self
            .served_remote_inputs
            .get(serve_occurrence)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownRequestIdentity))?;
        let outcome = match self.remote_input_source_value(carrier) {
            Ok(expected_value) => {
                let value = result.into_value();
                if value != expected_value {
                    return Err(KernelDiagnostics::one(
                        KernelDiagnosticKind::RemoteInputValueMismatch,
                    ));
                }
                KernelOutcome::Success(value)
            }
            Err(_) => KernelOutcome::DeclaredFailure(FailureKind::RouteUnavailable),
        };
        let identity = carrier
            .request_identity
            .as_ref()
            .expect("served remote input has identity")
            .clone();
        let reply = RemoteInputReply {
            request_identity: identity.clone(),
            serve_occurrence: serve_occurrence.clone(),
            reply_occurrence: self.next_occurrence("remote-input-reply"),
            outcome,
        };
        self.remote_input_replies
            .insert(serve_occurrence.clone(), reply.clone());
        self.trace.append(&identity, "reply");
        Ok(reply)
    }

    pub(crate) fn receive_remote_input_reply(
        &mut self,
        reply: RemoteInputReply,
    ) -> Result<RemoteInputReceipt, KernelDiagnostics> {
        let Some(stored) = self.remote_input_replies.get(&reply.serve_occurrence) else {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::UnknownRequestIdentity,
            ));
        };
        if stored != &reply {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::SourceCoreProvenanceMismatch,
            ));
        }
        let receipt_id = RemoteInputReceiptId::new(format!(
            "remote-input-receipt:{}",
            reply.request_identity.as_str()
        ));
        if self.receipt_store.remote_input.contains_key(&receipt_id) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::DuplicateReceipt,
            ));
        }
        let carrier = self
            .served_remote_inputs
            .get(&reply.serve_occurrence)
            .expect("stored remote reply retains its served carrier")
            .clone();
        let receive_occurrence = self.next_occurrence("remote-input-receive");
        let receipt = self.remote_input_receipt_from(
            &carrier,
            receipt_id.clone(),
            reply.outcome.clone(),
            OccurrenceLifecycle::new(
                carrier
                    .request_occurrence
                    .clone()
                    .expect("served remote input retains request occurrence"),
                reply.serve_occurrence.clone(),
                reply.reply_occurrence.clone(),
                receive_occurrence,
            ),
        )?;
        self.receipt_store
            .remote_input
            .insert(receipt_id, receipt.clone());
        self.trace
            .append(&reply.request_identity, "receive_receipt");
        self.trace
            .install_occurrences(&reply.request_identity, receipt.occurrences());
        if let Some(failure) = receipt.failure() {
            self.trace
                .record_typed_failure(&reply.request_identity, failure);
        }
        Ok(receipt)
    }

    /// Consume a receipt only at the checked designated evaluator and only
    /// for the exact source-derived dependency that requested it.
    pub(crate) fn consume_remote_input_receipt(
        &mut self,
        request: RemoteInputConsumeRequest,
    ) -> Result<ConsumedRemoteInput, KernelDiagnostics> {
        let receipt_id = request
            .receipt_id
            .as_ref()
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownRequestIdentity))?;
        if self.consumed_remote_receipts.contains(receipt_id) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::DuplicateReceipt,
            ));
        }
        let receipt = self
            .receipt_store
            .remote_input
            .get(receipt_id)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownRequestIdentity))?;
        let key = RemoteInputKey::new(
            &request.evaluator,
            &request.result,
            request.dependency_index,
        );
        let lineage = self
            .admission
            .remote_input_lineages
            .get(&key)
            .ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected)
            })?;
        let dependency = self.checked_remote_dependency(&key)?;
        let expected_frontier = InputFrontier::new(
            self.checked
                .designated_result(&key.evaluator, &key.result)
                .and_then(|evaluation| evaluation.designated_core())
                .and_then(|core| core.trigger().frontier())
                .ok_or_else(|| {
                    KernelDiagnostics::one(KernelDiagnosticKind::InputFrontierMismatch)
                })?,
        );
        if request.evaluator_locus.as_ref() != Some(&lineage.target_evaluator)
            || receipt.target_evaluator() != &lineage.target_evaluator
            || receipt.source_owner() != &lineage.source_owner
            || receipt.release_tuple() != &lineage.release_tuple
            || receipt.input_frontier() != &lineage.input_frontier
            || receipt.input_frontier() != &expected_frontier
            || receipt.source_ref() != &dependency.typed_state_read().source_ref()
        {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::SourceCoreProvenanceMismatch,
            ));
        }
        self.consumed_remote_receipts.insert(receipt_id.clone());
        self.trace.append(receipt.request_identity(), "consume");
        Ok(ConsumedRemoteInput {
            value: receipt.value().cloned(),
        })
    }

    pub(crate) fn semantic_snapshot(&self) -> &SemanticSnapshot {
        &self.semantic_snapshot
    }

    pub(crate) fn authority_view(&self) -> &AuthorityView {
        &self.authority_view
    }

    pub(crate) fn trace(&self) -> &KernelTrace {
        &self.trace
    }

    pub(crate) fn receipt_store(&self) -> &ReceiptStore {
        &self.receipt_store
    }

    fn validate_owner_carrier(
        &self,
        carrier: &OwnerRequestCarrier,
        require_identity: bool,
    ) -> Result<(), KernelDiagnostics> {
        let Some(provenance) = &carrier.provenance else {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::SourceFreeCarrierRejected,
            ));
        };
        let expected_provenance = self.expected_owner_provenance(carrier.operation.as_str())?;
        if !provenance.matches(&expected_provenance) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::SourceCoreProvenanceMismatch,
            ));
        }
        let Some(lineage) = self.admission.owner_lineages.get(&carrier.operation) else {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::AuthorityLineageRejected,
            ));
        };
        if carrier.target_owner.as_ref() != Some(&lineage.owner_locus) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::WrongTargetOwner,
            ));
        }
        if carrier.membership_incarnation.as_ref() != Some(&lineage.membership_incarnation) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::StaleMembershipIncarnation,
            ));
        }
        let expected_origin_locus = self
            .checked
            .evaluation(carrier.operation.as_str())
            .map(|evaluation| LocusRef::new(evaluation.authority_origin_locus()))
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownOperation))?;
        if carrier.origin_principal.as_ref() != Some(&lineage.principal)
            || carrier.origin_locus.as_ref() != Some(&expected_origin_locus)
            || carrier.membership_ref.as_deref() != Some(lineage.membership_ref.as_str())
            || carrier.membership_epoch.as_ref() != Some(&lineage.membership_epoch)
            || carrier.capability_ref.as_ref() != Some(&lineage.capability_ref)
            || carrier.witness_ref.as_ref() != Some(&lineage.witness_ref)
        {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::AuthorityLineageRejected,
            ));
        }
        if require_identity {
            let Some(identity) = &carrier.request_identity else {
                return Err(KernelDiagnostics::one(
                    KernelDiagnosticKind::UnknownRequestIdentity,
                ));
            };
            if !self.request_is_queued(identity) {
                return Err(KernelDiagnostics::one(
                    KernelDiagnosticKind::UnknownRequestIdentity,
                ));
            }
        } else if carrier.request_identity.is_some() {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::UnknownRequestIdentity,
            ));
        }
        Ok(())
    }

    fn m8_request_from_carrier(
        &self,
        carrier: &OwnerRequestCarrier,
    ) -> Result<M8OwnerRequest, KernelDiagnostics> {
        let lineage = self
            .admission
            .owner_lineages
            .get(&carrier.operation)
            .ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected)
            })?;
        let authority = lineage.m8_authority_use.clone().ok_or_else(|| {
            KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected)
        })?;
        let mut request = M8OwnerRequest::new(carrier.operation.as_str());
        for (name, value) in &carrier.arguments {
            request = request.with_argument(name, value);
        }
        Ok(request.with_authority_use(authority))
    }

    fn serve_validated_owner_carrier(
        &mut self,
        carrier: OwnerRequestCarrier,
    ) -> Result<ServedOwnerRequest, KernelDiagnostics> {
        // Both callers validate while the carrier is still resident in this
        // kernel's queue.  Requiring queue residency again after pop would
        // incorrectly turn an admitted request into an unknown request.
        let operation = self
            .checked
            .evaluation(carrier.operation.as_str())
            .expect("validated carrier names checked owner operation");
        let owner = operation
            .owner_rmw_core()
            .expect("validated carrier names checked owner Core")
            .clone();
        let outcome = match (
            self.materialize_key(owner.target(), &carrier.arguments),
            owner.target().owner_locus() == owner.owner_locus(),
        ) {
            (Some(target), true) => match &mut self.owner_backend {
                OwnerExecutionBackend::Reference => match self.evaluate_expression(
                    owner.expression().tree(),
                    &carrier.arguments,
                    owner.owner_locus(),
                ) {
                    Ok(value) => {
                        self.semantic_snapshot.ints.insert(target, value);
                        KernelOutcome::Success(SemanticValue::Int(value))
                    }
                    Err(_) => KernelOutcome::DeclaredFailure(FailureKind::RouteUnavailable),
                },
                OwnerExecutionBackend::AdmittedM8(runtime) => {
                    match runtime.serve_next_owner(owner.owner_locus()) {
                        Ok(served) => match served.written_int(&target) {
                            Some(value) => {
                                // On the kernel-admitted ordinary owner path,
                                // M8 is the owner executor.  This bounded
                                // snapshot is a kernel diagnostic mirror used
                                // by that typed carrier path; it is updated
                                // solely from M8's committed outcome.
                                self.semantic_snapshot.ints.insert(target, value);
                                KernelOutcome::Success(SemanticValue::Int(value))
                            }
                            None => KernelOutcome::DeclaredFailure(FailureKind::RouteUnavailable),
                        },
                        Err(diagnostics) => KernelOutcome::DeclaredFailure(m8_declared_failure(
                            diagnostics.outcome().failure(),
                        )),
                    }
                }
            },
            _ => KernelOutcome::DeclaredFailure(FailureKind::RouteUnavailable),
        };
        let identity = carrier
            .request_identity
            .as_ref()
            .expect("validated served carrier has identity")
            .clone();
        let occurrence = self.next_occurrence("serve");
        self.served
            .insert(occurrence.clone(), ServedOwnerState { carrier, outcome });
        self.trace.append(&identity, "serve");
        Ok(ServedOwnerRequest {
            request_identity: identity,
            serve_occurrence: occurrence,
        })
    }

    fn expected_owner_provenance(
        &self,
        operation: &str,
    ) -> Result<SourceCoreProvenance, KernelDiagnostics> {
        canonical_checked_owner_provenance(&self.checked, operation)
    }

    fn checked_remote_dependency(
        &self,
        key: &RemoteInputKey,
    ) -> Result<
        &mir_semantics::surface_v0_pipeline::DesignatedRemoteInputDependency,
        KernelDiagnostics,
    > {
        self.checked
            .designated_result(&key.evaluator, &key.result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| {
                core.generated_remote_input_dependencies()
                    .get(key.dependency_index)
            })
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownOperation))
    }

    fn validate_remote_input_payload(
        &self,
        carrier: &RemoteInputRequestCarrier,
    ) -> Result<(), KernelDiagnostics> {
        let key = RemoteInputKey::new(
            carrier.evaluator.clone(),
            carrier.result.clone(),
            carrier.dependency_index,
        );
        let lineage = self
            .admission
            .remote_input_lineages
            .get(&key)
            .ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected)
            })?;
        let dependency = self.checked_remote_dependency(&key)?;
        if carrier.membership_incarnation.as_ref() != Some(&lineage.membership_incarnation) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::StaleMembershipIncarnation,
            ));
        }
        let expected_frontier = self
            .checked
            .designated_result(&key.evaluator, &key.result)
            .and_then(|evaluation| evaluation.designated_core())
            .and_then(|core| core.trigger().frontier())
            .map(InputFrontier::new)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::InputFrontierMismatch))?;
        if lineage.input_frontier != expected_frontier
            || carrier.input_frontier.as_ref() != Some(&expected_frontier)
        {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::InputFrontierMismatch,
            ));
        }
        if carrier.release_tuple.as_ref() != Some(&lineage.release_tuple) {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::ReleaseTupleMismatch,
            ));
        }
        if carrier.origin_principal.as_ref() != Some(&lineage.origin_principal)
            || carrier.origin_locus.as_ref() != Some(&lineage.target_evaluator)
            || carrier.source_owner.as_ref() != Some(&lineage.source_owner)
            || carrier.target_evaluator.as_ref() != Some(&lineage.target_evaluator)
            || carrier.membership_ref.as_deref() != Some(lineage.membership_ref.as_str())
            || carrier.membership_epoch.as_ref() != Some(&lineage.membership_epoch)
            || carrier.source_ref.as_ref() != Some(&dependency.typed_state_read().source_ref())
            || carrier.capability_ref.as_ref() != Some(&lineage.capability_ref)
            || carrier.witness_ref.as_ref() != Some(&lineage.witness_ref)
        {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::AuthorityLineageRejected,
            ));
        }
        Ok(())
    }

    fn validate_remote_input_carrier(
        &self,
        carrier: &RemoteInputRequestCarrier,
        require_queued_identity: bool,
    ) -> Result<(), KernelDiagnostics> {
        self.validate_remote_input_payload(carrier)?;
        if require_queued_identity {
            let Some(identity) = &carrier.request_identity else {
                return Err(KernelDiagnostics::one(
                    KernelDiagnosticKind::UnknownRequestIdentity,
                ));
            };
            if !self.remote_input_is_queued(identity) {
                return Err(KernelDiagnostics::one(
                    KernelDiagnosticKind::UnknownRequestIdentity,
                ));
            }
        } else if carrier.request_identity.is_some() {
            return Err(KernelDiagnostics::one(
                KernelDiagnosticKind::UnknownRequestIdentity,
            ));
        }
        Ok(())
    }

    fn remote_input_source_value(
        &self,
        carrier: &RemoteInputRequestCarrier,
    ) -> Result<SemanticValue, KernelDiagnostics> {
        let key = RemoteInputKey::new(
            carrier.evaluator.clone(),
            carrier.result.clone(),
            carrier.dependency_index,
        );
        let dependency = self.checked_remote_dependency(&key)?;
        let state_key = self
            .materialize_key(dependency.typed_state_read(), &BTreeMap::new())
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable))?;
        self.semantic_snapshot
            .int(&state_key)
            .map(SemanticValue::Int)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable))
    }

    fn remote_input_receipt_from(
        &self,
        carrier: &RemoteInputRequestCarrier,
        receipt_id: RemoteInputReceiptId,
        outcome: KernelOutcome,
        occurrences: OccurrenceLifecycle,
    ) -> Result<RemoteInputReceipt, KernelDiagnostics> {
        self.validate_remote_input_payload(carrier)?;
        Ok(RemoteInputReceipt {
            receipt_id,
            request_identity: carrier
                .request_identity
                .as_ref()
                .expect("served remote input has identity")
                .clone(),
            origin_principal: carrier
                .origin_principal
                .as_ref()
                .expect("validated remote input has origin principal")
                .clone(),
            source_owner: carrier
                .source_owner
                .as_ref()
                .expect("validated remote input has source owner")
                .clone(),
            target_evaluator: carrier
                .target_evaluator
                .as_ref()
                .expect("validated remote input has evaluator")
                .clone(),
            release_tuple: carrier
                .release_tuple
                .as_ref()
                .expect("validated remote input has release tuple")
                .clone(),
            input_frontier: carrier
                .input_frontier
                .as_ref()
                .expect("validated remote input has frontier")
                .clone(),
            source_ref: self
                .checked_remote_dependency(&RemoteInputKey::new(
                    &carrier.evaluator,
                    &carrier.result,
                    carrier.dependency_index,
                ))?
                .typed_state_read()
                .source_ref(),
            core_ref: self.remote_input_core_ref(carrier)?,
            effect_row: EffectRow::new([EffectKind::DesignatedRemoteRequest]),
            failure_row: FailureRow::new([FailureKind::RouteUnavailable]),
            outcome,
            membership_epoch: carrier
                .membership_epoch
                .as_ref()
                .expect("validated remote input has membership epoch")
                .clone(),
            membership_incarnation: carrier
                .membership_incarnation
                .as_ref()
                .expect("validated remote input has membership incarnation")
                .clone(),
            capability_refs: vec![
                self.remote_input_lineage_for(carrier)?
                    .capability_ref
                    .clone(),
            ],
            witness_refs: vec![self.remote_input_lineage_for(carrier)?.witness_ref.clone()],
            occurrences,
        })
    }

    fn remote_input_lineage_for(
        &self,
        carrier: &RemoteInputRequestCarrier,
    ) -> Result<&RemoteInputLineage, KernelDiagnostics> {
        self.admission
            .remote_input_lineages
            .get(&RemoteInputKey::new(
                &carrier.evaluator,
                &carrier.result,
                carrier.dependency_index,
            ))
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::AuthorityLineageRejected))
    }

    fn remote_input_core_ref(
        &self,
        carrier: &RemoteInputRequestCarrier,
    ) -> Result<String, KernelDiagnostics> {
        let evaluation = self
            .checked
            .designated_result(&carrier.evaluator, &carrier.result)
            .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::UnknownOperation))?;
        self.checked
            .source_map()
            .entries()
            .iter()
            .find(|entry| {
                entry.kind() == SourceToCoreKind::DesignatedDecision
                    && entry.source_ref() == evaluation.source_ref()
            })
            .map(|entry| entry.core_ref().to_string())
            .ok_or_else(|| {
                KernelDiagnostics::one(KernelDiagnosticKind::SourceCoreProvenanceMismatch)
            })
    }

    fn materialize_key(
        &self,
        read: &TypedStateRead,
        arguments: &BTreeMap<String, String>,
    ) -> Option<KernelStateKey> {
        let index = read
            .index()
            .and_then(|index| arguments.get(index).map(String::as_str).or(Some(index)))?;
        Some(KernelStateKey::indexed_field(
            read.namespace(),
            index,
            read.field()?,
        ))
    }

    fn evaluate_expression(
        &self,
        tree: &CheckedExpressionTree,
        arguments: &BTreeMap<String, String>,
        owner_locus: &str,
    ) -> Result<i64, KernelDiagnostics> {
        match tree {
            CheckedExpressionTree::StateRead(read) => {
                if read.owner_locus() != owner_locus {
                    return Err(KernelDiagnostics::one(
                        KernelDiagnosticKind::RouteUnavailable,
                    ));
                }
                let key = self.materialize_key(read, arguments).ok_or_else(|| {
                    KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable)
                })?;
                self.semantic_snapshot
                    .int(&key)
                    .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable))
            }
            CheckedExpressionTree::ParameterRead { name, .. } => arguments
                .get(name)
                .and_then(|value| value.parse::<i64>().ok())
                .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable)),
            CheckedExpressionTree::IntegerLiteral(literal) => Ok(literal.value()),
            CheckedExpressionTree::Binary {
                operator,
                left,
                right,
                ..
            } => {
                let left = self.evaluate_expression(left, arguments, owner_locus)?;
                let right = self.evaluate_expression(right, arguments, owner_locus)?;
                match operator {
                    CheckedBinaryOperator::Add => left.checked_add(right),
                    CheckedBinaryOperator::Subtract => left.checked_sub(right),
                }
                .ok_or_else(|| KernelDiagnostics::one(KernelDiagnosticKind::RouteUnavailable))
            }
        }
    }

    fn receipt_from_carrier(
        &self,
        carrier: &OwnerRequestCarrier,
        outcome: KernelOutcome,
        occurrences: OccurrenceLifecycle,
    ) -> Result<KernelReceipt, KernelDiagnostics> {
        let provenance = self.expected_owner_provenance(carrier.operation.as_str())?;
        Ok(KernelReceipt {
            request_identity: carrier
                .request_identity
                .as_ref()
                .expect("served carrier has identity")
                .clone(),
            origin_principal: carrier
                .origin_principal
                .as_ref()
                .expect("validated carrier has origin")
                .clone(),
            origin_locus: carrier
                .origin_locus
                .as_ref()
                .expect("validated carrier has origin locus")
                .clone(),
            target_owner: carrier
                .target_owner
                .as_ref()
                .expect("validated carrier has target")
                .clone(),
            operation: carrier.operation.clone(),
            source_ref: provenance.source_ref,
            core_ref: provenance.core_ref,
            effect_row: provenance.effect_row,
            failure_row: provenance.failure_row,
            capability_refs: carrier.capability_ref.clone().into_iter().collect(),
            witness_refs: carrier.witness_ref.clone().into_iter().collect(),
            membership_epoch: carrier
                .membership_epoch
                .as_ref()
                .expect("validated carrier has membership epoch")
                .clone(),
            membership_incarnation: carrier
                .membership_incarnation
                .as_ref()
                .expect("validated carrier has membership incarnation")
                .clone(),
            redaction: provenance.redaction.clone(),
            outcome,
            occurrences,
        })
    }

    fn request_is_queued(&self, identity: &RequestIdentity) -> bool {
        self.owner_queues.values().any(|queue| {
            queue
                .iter()
                .any(|queued| queued.request_identity() == identity)
        })
    }

    fn remote_input_is_queued(&self, identity: &RequestIdentity) -> bool {
        self.remote_input_queues.values().any(|queue| {
            queue
                .iter()
                .any(|queued| queued.carrier.request_identity.as_ref() == Some(identity))
        })
    }

    fn next_occurrence(&mut self, kind: &str) -> OccurrenceRef {
        let occurrence = OccurrenceRef::new(kind, self.next_occurrence);
        self.next_occurrence += 1;
        occurrence
    }
}
