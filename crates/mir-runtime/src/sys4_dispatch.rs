//! SYS-4 in-process dispatch of already projected locus artifacts.
//!
//! This module starts at the checked SYS-3 projection and a sealed M9 runtime
//! admission.  It deliberately has no source parsing, conformance facade, or
//! precomputed-result selection path: routes, endpoint ownership, and Core
//! provenance are taken only from the projection result.

// SYS-4 remains crate-internal until its CLI facade is introduced.  Its
// entrypoints are consumed by the crate's SYS-4 conformance tests today.

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{Arc, Mutex, MutexGuard},
};

use sha2::{Digest, Sha256};

use mir_semantics::{
    evaluation_materialization::{InputFrontier, ObservationPolicy, PolicyStamp},
    shared_model::{ResultFrontier, ResultVersion, SourceRef},
    surface_v0_pipeline::{CheckedProgramIdentity, TypedStateRead},
};

use crate::{
    m8_runtime_admission::{EvidenceSecurityLabel, M8RuntimeInstance, M8SecurityClass},
    m8_runtime_designated_value::{
        M8ConsumeRequest, M8DesignatedEvaluationRequest, M8DesignatedTick, M8InputReceipt,
        M8InputReceiptSet, M8PublishedDesignatedValue,
    },
    m8_runtime_local_cut::{
        M8LiveFloor, M8LocalCut, M8LocalDesignatedTraceContext, M8LocalRuntime, M8LocalRuntimeSeed,
        M8LocalTrace, M8LocalTraceKind, M8LocalTraceObservation,
    },
    m8_runtime_owner_queue::{M8OwnerRequest, M8ServeOutcome, M8StateKey},
    m8_runtime_relation_projection::{
        M8BindingInvalidation, M8LeaseRecord, M8ObservedRelationShadow, M8Point,
        M8PresentationContext, M8PresentationFallback, M8PublishedRelationState,
        M8RelationAuthorityUse, M8RelationProjection, M8RelationReacquire, M8RestrictionPolicy,
    },
    m9_auth_verification::{
        M9AdmissionErrorKind, M9AuthorityGeneration, M9AuthorityInspection,
        M9AuthoritySuccessorPublisher, M9AuthorityTransitionKind, M9CacheValidationInspection,
        M9CheckedPatchAuthorityBinding, M9DesignatedSourceReleaseLineage, M9KernelAuthorityView,
        M9RelationPublicationAdmission, M9RuntimeExecutionSeam,
        M9RuntimeValidationObservationSnapshot, M9SealedFailureInspection, M9SealedGeneration,
        M9SealedTransitionInspection, M9SourceReleaseValidationInspection,
    },
    sys2_execution_backend::{
        Ow1ContextualM8Execution, Ow1ObserverDesignatedPublication, Ow1WorkerBackend,
        Ow1WorkerFailure,
    },
    sys3_projection::{
        BackendEligibility, BackendIneligibilityReason, BackendProfile, CarrierContract,
        CommunicationEdge, CommunicationEdgeKind, GlobalProjectionResult, LocusProgram,
        ProjectedOperationFragment, ProjectedOperationFragmentKind, ReferenceOnlyRedactionPolicy,
        RuntimeAdmissionStatus, SourceRefView,
    },
};

/// One fully successful relation publication consumes four source-outbox,
/// three transport, one target-dequeue, and one relation-serve occurrence.
/// Capacity is preflighted before any owner or consumer M8 transition.
const RELATION_DISPATCH_ENDPOINT_OCCURRENCES: u64 = 9;

#[cfg(test)]
use crate::m8_runtime_local_cut::M8LocalSessionObserver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys4DiagnosticKind {
    IncompleteM9ResidualDischarge,
    IncompleteM9AuthorityInventory,
    ProgramAdmissionMismatch,
    ProgramProjectionMismatch,
    ForeignSeedLocus,
    ForeignSeedState,
    ForeignSeedIndex,
    ForeignSeedField,
    RouteUnavailable,
    WrongTargetLocus,
    ExternalTargetOverrideRejected,
    ExternalAuthorityOverrideRejected,
    MissingConsumerCapability,
    MissingConsumerWitness,
    MissingConsumerMembership,
    MissingEvaluatorAuthority,
    MissingPublishedResult,
    MissingTypedDesignatedValue,
    MissingDesignatedTick,
    DeliveryPublicationIdentityMismatch,
    CacheBindingDigestMismatch,
    CacheProjectionMismatch,
    CarrierRedactionMismatch,
    CarrierProvenanceMismatch,
    CarrierVisibilityMismatch,
    CarrierPolicyMismatch,
    MissingSourceReleaseAuthority,
    UnknownProjectedEdge,
    UnavailableEnvelope,
    FaultEnvelopeRouteMismatch,
    UnknownRetargetLocus,
    BackendIneligible,
    M8ExecutionRejected,
    /// A finite in-process identifier namespace cannot advance without
    /// wrapping.  Fail closed before producing a duplicate carrier or trace
    /// occurrence.
    IdentifierExhausted,
    /// A live OW1 worker failed to provide a clone-only observer snapshot.
    /// This never rewrites the semantic result of an already committed M8
    /// operation and is distinct from a genuinely absent observation.
    ObserverSnapshotUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4Diagnostic {
    kind: Sys4DiagnosticKind,
    typed_success: Option<RuntimeValue>,
}

impl Sys4Diagnostic {
    fn new(kind: Sys4DiagnosticKind) -> Self {
        Self {
            kind,
            typed_success: None,
        }
    }

    pub(crate) const fn kind(&self) -> Sys4DiagnosticKind {
        self.kind
    }

    pub(crate) fn typed_success(&self) -> Option<&RuntimeValue> {
        self.typed_success.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4DispatchDiagnostics {
    entries: Vec<Sys4Diagnostic>,
    context: Box<Sys4DiagnosticContext>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Sys4DiagnosticContext {
    endpoint_dequeue_occurrence_id: Option<String>,
    m8_trace_node_id: Option<String>,
    rejected_envelope_id: Option<String>,
    rejected_request_id: Option<String>,
    relation_publication_failure_disposition: Option<RelationPublicationFailureDisposition>,
    m9_failure_inspection: Option<Box<M9SealedFailureInspection>>,
    m8_non_consuming_validation_node_id: Option<String>,
    local_store_read_audit_id: Option<String>,
    backend_m8_failure: Option<Box<M8LocalTraceObservation>>,
    retarget_fault: Option<Box<RetargetFaultInspection>>,
    cache_projection_mismatch: Option<Box<CacheProjectionMismatchInspection>>,
    backend_ineligibility_reason: Option<BackendIneligibilityReason>,
    observer_snapshot_failure: Option<Box<ObserverSnapshotFailure>>,
}

/// Observer-safe disposition of an uncommitted generated relation carrier.
/// It distinguishes an explicit retry-safe discard from a carrier the
/// transport has already terminalized; neither state grants authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RelationPublicationFailureDisposition {
    DiscardedUndelivered,
    AlreadyRemovedByTransport,
}

/// Observer-only availability state.  It deliberately contains neither a
/// payload nor M9 authority material, and is not a transport failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ObserverSnapshotChannel {
    LocalTrace,
    DesignatedPublication,
    #[cfg(test)]
    ObserverSafeSession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObserverSnapshotFailure {
    session_id: String,
    channel: ObserverSnapshotChannel,
    diagnostic: Sys4DiagnosticKind,
}

impl ObserverSnapshotFailure {
    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    pub(crate) const fn channel(&self) -> ObserverSnapshotChannel {
        self.channel
    }

    pub(crate) const fn diagnostic(&self) -> Sys4DiagnosticKind {
        self.diagnostic
    }
}

const DESIGNATED_PUBLICATION_OBSERVER_SESSION: &str = "observer:designated-publication";

/// Observer-safe evidence that a retarget policy was applied to one exact
/// projected carrier.  It carries locus names only, never payload or grants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RetargetFaultInspection {
    evidence_id: String,
    edge_ref: String,
    envelope_id: String,
    attempted_target_locus: String,
    rejected_at_fault_admission: bool,
    target_enqueue_occurrence_id: Option<String>,
}

impl RetargetFaultInspection {
    pub(crate) fn evidence_id(&self) -> &str {
        &self.evidence_id
    }
    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }
    pub(crate) fn envelope_id(&self) -> &str {
        &self.envelope_id
    }
    pub(crate) fn attempted_target_locus(&self) -> &str {
        &self.attempted_target_locus
    }
    pub(crate) const fn rejected_at_fault_admission(&self) -> bool {
        self.rejected_at_fault_admission
    }
    pub(crate) fn target_enqueue_occurrence_id(&self) -> Option<&str> {
        self.target_enqueue_occurrence_id.as_deref()
    }
}

/// Typed comparison between a retry carrier and the currently projected
/// delivery edge.  The booleans prove validation occurred before M9/M8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CacheProjectionMismatchInspection {
    envelope_id: String,
    expected_edge_ref: String,
    expected_source_ref: SourceRefView,
    expected_core_ref: Option<String>,
    carrier_core_ref: Option<String>,
    rejected_before_m9_validation: bool,
    rejected_before_m8_validation: bool,
}

impl CacheProjectionMismatchInspection {
    pub(crate) fn envelope_id(&self) -> &str {
        &self.envelope_id
    }
    pub(crate) fn expected_edge_ref(&self) -> &str {
        &self.expected_edge_ref
    }
    pub(crate) fn expected_source_ref(&self) -> SourceRefView {
        self.expected_source_ref.clone()
    }
    pub(crate) fn expected_core_ref(&self) -> Option<&str> {
        self.expected_core_ref.as_deref()
    }
    pub(crate) fn carrier_core_ref(&self) -> Option<&str> {
        self.carrier_core_ref.as_deref()
    }
    pub(crate) const fn rejected_before_m9_validation(&self) -> bool {
        self.rejected_before_m9_validation
    }
    pub(crate) const fn rejected_before_m8_validation(&self) -> bool {
        self.rejected_before_m8_validation
    }
}

impl Sys4DispatchDiagnostics {
    fn one(kind: Sys4DiagnosticKind) -> Self {
        Self {
            entries: vec![Sys4Diagnostic::new(kind)],
            context: Box::default(),
        }
    }

    pub(crate) fn primary(&self) -> &Sys4Diagnostic {
        self.entries
            .first()
            .expect("SYS-4 diagnostics always retain one primary row")
    }

    pub(crate) const fn partial_fabric(&self) -> Option<()> {
        None
    }

    pub(crate) const fn exposes_raw_payload(&self) -> bool {
        false
    }

    pub(crate) fn endpoint_dequeue_occurrence_id(&self) -> Option<&str> {
        self.context.endpoint_dequeue_occurrence_id.as_deref()
    }

    pub(crate) fn m8_trace_node_id(&self) -> Option<&str> {
        self.context.m8_trace_node_id.as_deref()
    }

    pub(crate) fn rejected_envelope_id(&self) -> Option<&str> {
        self.context.rejected_envelope_id.as_deref()
    }

    pub(crate) const fn relation_publication_failure_disposition(
        &self,
    ) -> Option<RelationPublicationFailureDisposition> {
        self.context.relation_publication_failure_disposition
    }

    pub(crate) fn backend_ineligibility_reason(&self) -> Option<&BackendIneligibilityReason> {
        self.context.backend_ineligibility_reason.as_ref()
    }

    pub(crate) fn rejected_request_id(&self) -> Option<&str> {
        self.context.rejected_request_id.as_deref()
    }

    pub(crate) fn m9_failure_inspection(&self) -> Option<&M9SealedFailureInspection> {
        self.context.m9_failure_inspection.as_deref()
    }

    pub(crate) fn m8_non_consuming_validation_node_id(&self) -> Option<&str> {
        self.context.m8_non_consuming_validation_node_id.as_deref()
    }

    pub(crate) fn local_store_read_audit_id(&self) -> Option<&str> {
        self.context.local_store_read_audit_id.as_deref()
    }

    pub(crate) fn backend_m8_failure_inspection(&self) -> Option<&M8LocalTraceObservation> {
        self.context.backend_m8_failure.as_deref()
    }

    pub(crate) fn retarget_fault_inspection(&self) -> Option<&RetargetFaultInspection> {
        self.context.retarget_fault.as_deref()
    }

    pub(crate) fn cache_projection_mismatch_inspection(
        &self,
    ) -> Option<&CacheProjectionMismatchInspection> {
        self.context.cache_projection_mismatch.as_deref()
    }
}

type Sys4Result<T> = Result<T, Sys4DispatchDiagnostics>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeValue {
    Int(i64),
    Unit,
}

impl RuntimeValue {
    pub(crate) const fn int(value: i64) -> Self {
        Self::Int(value)
    }

    pub(crate) const fn unit() -> Self {
        Self::Unit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct FabricRouteKey {
    operation: String,
    kind: CommunicationEdgeKind,
    source_locus: String,
    target_locus: String,
}

impl FabricRouteKey {
    pub(crate) fn owner_request(
        operation: impl Into<String>,
        source_locus: impl Into<String>,
        target_locus: impl Into<String>,
    ) -> Self {
        Self {
            operation: operation.into(),
            kind: CommunicationEdgeKind::OwnerRequest,
            source_locus: source_locus.into(),
            target_locus: target_locus.into(),
        }
    }

    fn from_edge(edge: &CommunicationEdge) -> Self {
        Self {
            operation: edge.operation_id().to_string(),
            kind: edge.kind(),
            source_locus: edge.source_locus().to_string(),
            target_locus: edge.target_locus().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FabricRoute {
    key: FabricRouteKey,
    edge_ref: String,
}

impl FabricRoute {
    pub(crate) fn key(&self) -> &FabricRouteKey {
        &self.key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FabricRouteIndex {
    routes: BTreeMap<FabricRouteKey, FabricRoute>,
}

impl FabricRouteIndex {
    pub(crate) fn edge_refs(&self) -> BTreeSet<String> {
        self.routes
            .values()
            .map(|route| route.edge_ref.clone())
            .collect()
    }

    pub(crate) fn all_routes_derive_from_plan(
        &self,
        plan: &crate::sys3_projection::CommunicationPlan,
    ) -> bool {
        self.routes.values().all(|route| {
            plan.edges().iter().any(|edge| {
                edge.edge_ref() == route.edge_ref && FabricRouteKey::from_edge(edge) == route.key
            })
        })
    }

    fn route(&self, key: &FabricRouteKey) -> Option<&FabricRoute> {
        self.routes.get(key)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FabricProgram {
    projection: GlobalProjectionResult,
    route_index: FabricRouteIndex,
}

impl FabricProgram {
    pub(crate) fn from_projection(projection: GlobalProjectionResult) -> Sys4Result<Self> {
        let mut routes = BTreeMap::new();
        for edge in projection.communication_plan().edges() {
            let key = FabricRouteKey::from_edge(edge);
            if routes
                .insert(
                    key.clone(),
                    FabricRoute {
                        key,
                        edge_ref: edge.edge_ref().to_string(),
                    },
                )
                .is_some()
            {
                return Err(Sys4DispatchDiagnostics::one(
                    Sys4DiagnosticKind::ProgramProjectionMismatch,
                ));
            }
        }
        Ok(Self {
            projection,
            route_index: FabricRouteIndex { routes },
        })
    }

    pub(crate) const fn runtime_admission_status(&self) -> RuntimeAdmissionStatus {
        self.projection.runtime_admission_status()
    }

    pub(crate) fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        self.projection.checked_program_identity()
    }

    pub(crate) fn locus_names(&self) -> Vec<String> {
        self.projection
            .locus_order()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect()
    }

    pub(crate) fn locus_count(&self) -> usize {
        self.projection.locus_order().len()
    }

    pub(crate) fn route_index(&self) -> &FabricRouteIndex {
        &self.route_index
    }

    pub(crate) fn projected_authority_grants(&self) -> Vec<()> {
        Vec::new()
    }

    pub(crate) fn backend_eligibility(&self, profile: BackendProfile) -> BackendEligibility {
        self.projection.backend_requirements().eligibility(profile)
    }

    pub(crate) fn derive_route_for_external_action(
        &self,
        external: &ExternalAction,
    ) -> Sys4Result<&FabricRoute> {
        if external.target_locus_override().is_some() {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ExternalTargetOverrideRejected,
            ));
        }
        if external.authority_principal_override().is_some() {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ExternalAuthorityOverrideRejected,
            ));
        }
        let ExternalActionKind::Source(source) = &external.kind else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::RouteUnavailable,
            ));
        };
        let Some(fragment) = self.owner_request_fragment(source.operation_id()) else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::RouteUnavailable,
            ));
        };
        let (Some(origin), Some(target)) = (fragment.origin_locus(), fragment.target_owner_locus())
        else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramProjectionMismatch,
            ));
        };
        self.route_index
            .route(&FabricRouteKey::owner_request(
                source.operation_id(),
                origin,
                target,
            ))
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable))
    }

    fn owner_request_fragment(&self, operation: &str) -> Option<ProjectedOperationFragment> {
        self.projection
            .sys4_artifact_fragments()
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == operation
                    && fragment.fragment_kind()
                        == ProjectedOperationFragmentKind::OwnerRequestInvocation
            })
            .cloned()
    }

    fn owner_execution_fragment(&self, operation: &str) -> Option<ProjectedOperationFragment> {
        self.projection
            .sys4_artifact_fragments()
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == operation
                    && fragment.fragment_kind() == ProjectedOperationFragmentKind::OwnerRmwExecution
            })
            .cloned()
    }

    fn designated_evaluator_fragment(
        &self,
        value_name: &str,
    ) -> Option<ProjectedOperationFragment> {
        self.projection
            .sys4_artifact_fragments()
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == value_name
                    && fragment.fragment_kind()
                        == ProjectedOperationFragmentKind::DesignatedEvaluation
            })
            .cloned()
    }

    fn designated_consumer_fragment(&self, value_name: &str) -> Option<ProjectedOperationFragment> {
        self.projection
            .sys4_artifact_fragments()
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == value_name
                    && fragment.fragment_kind()
                        == ProjectedOperationFragmentKind::DesignatedResultConsumer
            })
            .cloned()
    }

    /// The dependency ordinal is part of the checked projection identity.  A
    /// runtime admission must use that exact ordinal rather than collapsing
    /// every designated source service onto dependency zero.
    fn designated_remote_input_dependency_index(
        &self,
        fragment: &ProjectedOperationFragment,
    ) -> Option<usize> {
        let dependency = fragment.designated_remote_input_dependency()?;
        let ordinal = fragment.checked_core_identity().dependency_ordinal()?;
        let evaluator = self.designated_evaluator_fragment(fragment.operation_id())?;
        evaluator
            .designated_checked_core()?
            .generated_remote_input_dependencies()
            .get(ordinal)
            .filter(|candidate| *candidate == dependency)
            .map(|_| ordinal)
    }

    fn projected_fingerprint(&self) -> BTreeSet<(FabricRouteKey, String)> {
        self.route_index
            .routes
            .iter()
            .map(|(key, route)| (key.clone(), route.edge_ref.clone()))
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn for_test_remove_route(&mut self, key: FabricRouteKey) {
        self.route_index.routes.remove(&key);
    }

    #[cfg(test)]
    pub(crate) fn for_test_retarget_route(&mut self, key: FabricRouteKey, target: &str) {
        let Some(route) = self.route_index.routes.remove(&key) else {
            return;
        };
        let mut changed_key = route.key;
        changed_key.target_locus = target.to_string();
        self.route_index.routes.insert(
            key,
            FabricRoute {
                key: changed_key,
                edge_ref: route.edge_ref,
            },
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4InitialStateSeed {
    checked_program_identity: CheckedProgramIdentity,
    ints: BTreeMap<(String, String, String, String), i64>,
}

impl Sys4InitialStateSeed {
    pub(crate) fn for_checked_program(checked_program_identity: CheckedProgramIdentity) -> Self {
        Self {
            checked_program_identity,
            ints: BTreeMap::new(),
        }
    }

    pub(crate) fn with_int(
        mut self,
        locus: impl Into<String>,
        state: impl Into<String>,
        index: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Self {
        self.ints.insert(
            (locus.into(), state.into(), index.into(), field.into()),
            value,
        );
        self
    }

    pub(crate) fn int(&self, locus: &str, state: &str, index: &str, field: &str) -> Option<i64> {
        self.ints
            .get(&(
                locus.to_string(),
                state.to_string(),
                index.to_string(),
                field.to_string(),
            ))
            .copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObserverSafeM9Summary {
    checked_program_identity: CheckedProgramIdentity,
    complete_final: bool,
    inventory_digest: String,
    owner_lineages: BTreeSet<(String, String, String, String)>,
    relation_transitions: BTreeSet<(String, String)>,
    designated_evaluators: BTreeSet<(String, String)>,
    designated_remote_input_lineages: BTreeSet<(String, String, String, usize, String)>,
    designated_consumers: BTreeSet<(String, String)>,
}

/// Canonical source-semantic rows retained by a sealed admission.  These rows
/// contain only operation/locus/version identities already safe for observer
/// tooling; they deliberately omit all membership, capability, witness, and
/// provider references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObserverSafeM9SemanticRowSets {
    owner_lineages: BTreeSet<(String, String, String, String)>,
    relation_transitions: BTreeSet<(String, String)>,
    designated_evaluators: BTreeSet<(String, String)>,
    designated_remote_input_lineages: BTreeSet<(String, String, String, usize, String)>,
    designated_consumers: BTreeSet<(String, String)>,
}

impl ObserverSafeM9SemanticRowSets {
    pub(crate) fn owner_lineages(&self) -> &BTreeSet<(String, String, String, String)> {
        &self.owner_lineages
    }

    pub(crate) fn relation_transitions(&self) -> &BTreeSet<(String, String)> {
        &self.relation_transitions
    }

    pub(crate) fn designated_evaluators(&self) -> &BTreeSet<(String, String)> {
        &self.designated_evaluators
    }

    pub(crate) fn designated_remote_input_lineages(
        &self,
    ) -> &BTreeSet<(String, String, String, usize, String)> {
        &self.designated_remote_input_lineages
    }

    pub(crate) fn designated_consumers(&self) -> &BTreeSet<(String, String)> {
        &self.designated_consumers
    }
}

impl ObserverSafeM9Summary {
    pub(crate) fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        &self.checked_program_identity
    }

    pub(crate) const fn is_complete_final_m9_runtime_seam(&self) -> bool {
        self.complete_final
    }

    pub(crate) const fn residuals_discharged_for_static_program(&self) -> bool {
        self.complete_final
    }

    pub(crate) fn semantic_row_sets_clone(&self) -> ObserverSafeM9SemanticRowSets {
        ObserverSafeM9SemanticRowSets {
            owner_lineages: self.owner_lineages.clone(),
            relation_transitions: self.relation_transitions.clone(),
            designated_evaluators: self.designated_evaluators.clone(),
            designated_remote_input_lineages: self.designated_remote_input_lineages.clone(),
            designated_consumers: self.designated_consumers.clone(),
        }
    }

    pub(crate) fn contains_owner_lineage(
        &self,
        operation: &str,
        principal: &str,
        origin: &str,
        owner: &str,
    ) -> bool {
        self.owner_lineages.contains(&(
            operation.to_string(),
            principal.to_string(),
            origin.to_string(),
            owner.to_string(),
        ))
    }

    pub(crate) fn generated_by_projection(&self) -> BTreeSet<String> {
        BTreeSet::new()
    }

    pub(crate) fn contains_designated_evaluator(&self, value_name: &str, locus: &str) -> bool {
        self.designated_evaluators
            .contains(&(value_name.to_string(), locus.to_string()))
    }

    pub(crate) fn contains_designated_consumer(&self, value_name: &str, locus: &str) -> bool {
        self.designated_consumers
            .contains(&(value_name.to_string(), locus.to_string()))
    }

    pub(crate) fn contains_designated_consumer_lineage(
        &self,
        value_name: &str,
        locus: &str,
    ) -> bool {
        self.contains_designated_consumer(value_name, locus)
    }
}

#[derive(Clone)]
struct M9AuthorityLiveFloor {
    current: Arc<Mutex<M9AuthorityGeneration>>,
}

/// A short critical section over one M9-owned monotone authority floor.
///
/// A successor is not published until its sealed authority inventory has
/// reached the backend. This prevents two fabrics admitted from one M9 seam
/// from both refreshing M8 from the same predecessor generation.
struct M9AuthorityLiveFloorGuard<'a> {
    current: MutexGuard<'a, M9AuthorityGeneration>,
}

impl M9AuthorityLiveFloorGuard<'_> {
    fn current_generation(&self) -> M9AuthorityGeneration {
        self.current.clone()
    }

    /// Runtime validation observations may advance independently of the
    /// monotone authority generation.  They are synchronized only while the
    /// shared floor still carries the exact same program/generation/authority
    /// facts, so a publisher can never overwrite a competing successor with
    /// an older observer snapshot.
    fn matches_runtime_authority_facts(&self, live: &M9AuthorityGeneration) -> bool {
        self.current
            .has_same_runtime_authority_facts_ignoring_validation_observations(live)
    }

    /// Re-key the *same* shared live floor to M9's checked-program successor
    /// only when M9 has established exact authority equivalence.  This is not
    /// a new generation or a candidate-owned floor replacement: state,
    /// lineages, and tombstones remain unchanged while later M9 transitions
    /// become bound to the active patched program identity.
    fn rebind_checked_patch_program(
        &mut self,
        active: &M9AuthorityGeneration,
        rebased: &M9AuthorityGeneration,
    ) -> bool {
        self.current.matches_for_restore(active)
            && rebased.is_checked_patch_rebase_equivalent_to(active)
            && {
                *self.current = rebased.clone();
                true
            }
    }

    fn accepts_successor(
        &self,
        prior: &M9AuthorityGeneration,
        successor: &M9AuthorityGeneration,
    ) -> bool {
        self.current.matches_for_restore(prior)
            && successor.generation() > self.current.generation()
            && successor.preserves_tombstones_from(&self.current)
    }

    fn commit_successor(&mut self, successor: &M9AuthorityGeneration) {
        *self.current = successor.clone();
    }
}

impl M9AuthorityLiveFloor {
    fn new(generation: M9AuthorityGeneration) -> Self {
        Self {
            current: Arc::new(Mutex::new(generation)),
        }
    }

    /// A staged local transition needs an isolated floor.  Its candidate may
    /// advance M9 while proving generated endpoint delivery, but that advance
    /// must not become visible to sibling/live fabrics before the whole
    /// transition commits through the canonical floor.
    fn detached_candidate(generation: M9AuthorityGeneration) -> Self {
        Self::new(generation)
    }

    fn matches_generation(&self, generation: &M9AuthorityGeneration) -> bool {
        self.current
            .lock()
            .ok()
            .is_some_and(|current| current.matches_for_restore(generation))
    }

    fn identity_snapshot(&self) -> usize {
        Arc::as_ptr(&self.current) as usize
    }

    /// Hash the live M9 generation held by this shared floor. The address of
    /// the `Arc` is deliberately excluded: only semantic continuation state
    /// can bind a local cut, never a process allocation identity.
    fn private_restore_integrity_digest(&self) -> Option<String> {
        self.current
            .lock()
            .ok()
            .map(|current| current.private_restore_integrity_digest())
    }

    /// Lock the floor only when it still names the exact sealed generation
    /// supplied by the caller. The guard is held across backend refresh or
    /// restore and the fabric-side generation install.
    fn guard_matching(
        &self,
        generation: &M9AuthorityGeneration,
    ) -> Option<M9AuthorityLiveFloorGuard<'_>> {
        let current = self.current.lock().ok()?;
        current
            .matches_for_restore(generation)
            .then_some(M9AuthorityLiveFloorGuard { current })
    }
}

#[derive(Clone)]
pub(crate) struct SealedFabricAdmission {
    program_identity: CheckedProgramIdentity,
    program_fingerprint: BTreeSet<(FabricRouteKey, String)>,
    summary: ObserverSafeM9Summary,
    instance: M8RuntimeInstance,
    authority_generation: M9AuthorityGeneration,
    authority_successor: M9AuthoritySuccessorPublisher,
    authority_live_floor: M9AuthorityLiveFloor,
    initial_state_seed: Sys4InitialStateSeed,
}

impl std::fmt::Debug for SealedFabricAdmission {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SealedFabricAdmission")
            .field("program_identity", &self.program_identity)
            .field("complete_m9_final", &self.summary.complete_final)
            .finish_non_exhaustive()
    }
}

impl SealedFabricAdmission {
    pub(crate) fn from_m9_execution_seam(
        program: &FabricProgram,
        seam: M9RuntimeExecutionSeam,
        initial_state_seed: Sys4InitialStateSeed,
    ) -> Sys4Result<Self> {
        if !seam.has_complete_final_residual_discharge() {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::IncompleteM9ResidualDischarge,
            ));
        }
        if seam.initial_authority_generation().program_identity()
            != program.checked_program_identity().stable_key()
            || initial_state_seed.checked_program_identity != *program.checked_program_identity()
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        let generation = seam.initial_authority_generation();
        // Admission is inventory-complete, not summary-complete: each
        // projected operation family must have an opaque M9 lineage before a
        // locus runtime is booted.  In particular a designated evaluator
        // alone cannot admit its source release or consumer.
        for fragment in program.projection.sys4_artifact_fragments().entries() {
            let complete = match fragment.fragment_kind() {
                ProjectedOperationFragmentKind::OwnerRmwExecution => {
                    fragment.owner_rmw_checked_core().is_some_and(|core| {
                        generation
                            .owner_authority_for_operation(
                                fragment.operation_id(),
                                core.owner_locus(),
                            )
                            .is_some()
                    })
                }
                ProjectedOperationFragmentKind::DesignatedEvaluation => {
                    fragment.designated_checked_core().is_some_and(|core| {
                        generation
                            .designated_evaluation_authority_use(core.evaluator(), core.result())
                            .is_some()
                    })
                }
                ProjectedOperationFragmentKind::DesignatedResultConsumer => generation
                    .designated_consumption_authority_use(
                        fragment.locus_tag().as_str(),
                        fragment.operation_id(),
                    )
                    .is_some(),
                ProjectedOperationFragmentKind::DesignatedRemoteInputService => (|| {
                    let Some(dependency) = fragment.designated_remote_input_dependency() else {
                        return false;
                    };
                    let Some(evaluator) =
                        program.designated_evaluator_fragment(fragment.operation_id())
                    else {
                        return false;
                    };
                    let Some(core) = evaluator.designated_checked_core() else {
                        return false;
                    };
                    let Some(dependency_index) =
                        program.designated_remote_input_dependency_index(fragment)
                    else {
                        return false;
                    };
                    generation
                        .kernel_designated_remote_input_lineage(
                            dependency.source_owner_locus(),
                            core.evaluator(),
                            core.result(),
                            dependency_index,
                            core.trigger().frontier().unwrap_or_default(),
                        )
                        .is_some()
                })(
                ),
                ProjectedOperationFragmentKind::RelationPublication => {
                    fragment.relation_checked_core().is_some_and(|_| {
                        [
                            "publish_relation",
                            "invalidate_primary",
                            "reacquire_primary",
                        ]
                        .into_iter()
                        .all(|transition| {
                            generation
                                .relation_authority_use(fragment.operation_id(), transition)
                                .is_some()
                        })
                    })
                }
                ProjectedOperationFragmentKind::ConsumerLocalRelationProjection => fragment
                    .consumer_relation_projection()
                    .is_some_and(|descriptor| {
                        [
                            "publish_relation",
                            "invalidate_primary",
                            "reacquire_primary",
                        ]
                        .into_iter()
                        .all(|transition| {
                            generation
                                .relation_authority_use(descriptor.source_relation(), transition)
                                .is_some()
                        })
                    }),
                _ => true,
            };
            if !complete {
                return Err(Sys4DispatchDiagnostics::one(
                    Sys4DiagnosticKind::IncompleteM9AuthorityInventory,
                ));
            }
        }
        validate_seed(program, &initial_state_seed)?;
        let mut owner_lineages = BTreeSet::new();
        let mut relation_transitions = BTreeSet::new();
        let mut designated_evaluators = BTreeSet::new();
        let mut designated_remote_input_lineages = BTreeSet::new();
        let mut designated_consumers = BTreeSet::new();
        for fragment in program.projection.sys4_artifact_fragments().entries() {
            match fragment.fragment_kind() {
                ProjectedOperationFragmentKind::OwnerRmwExecution => {
                    let core = fragment
                        .owner_rmw_checked_core()
                        .expect("SYS-3 owner fragment retains checked Core");
                    if let Some((principal, _authority)) = generation
                        .owner_authority_for_operation(fragment.operation_id(), core.owner_locus())
                    {
                        owner_lineages.insert((
                            fragment.operation_id().to_string(),
                            principal,
                            fragment
                                .origin_locus()
                                .unwrap_or(core.authority_origin_locus())
                                .to_string(),
                            core.owner_locus().to_string(),
                        ));
                    }
                }
                ProjectedOperationFragmentKind::DesignatedEvaluation => {
                    if generation
                        .designated_evaluation_authority_use(
                            fragment
                                .operation_id()
                                .split('.')
                                .next()
                                .unwrap_or_default(),
                            fragment
                                .operation_id()
                                .split('.')
                                .nth(1)
                                .unwrap_or_default(),
                        )
                        .is_some()
                    {
                        designated_evaluators.insert((
                            fragment.operation_id().to_string(),
                            fragment.locus_tag().as_str().to_string(),
                        ));
                    }
                }
                ProjectedOperationFragmentKind::DesignatedResultConsumer => {
                    if generation
                        .designated_consumption_authority_use(
                            fragment.locus_tag().as_str(),
                            fragment.operation_id(),
                        )
                        .is_some()
                    {
                        designated_consumers.insert((
                            fragment.operation_id().to_string(),
                            fragment.locus_tag().as_str().to_string(),
                        ));
                    }
                }
                ProjectedOperationFragmentKind::DesignatedRemoteInputService => {
                    let dependency = fragment
                        .designated_remote_input_dependency()
                        .expect("complete designated remote input retains its dependency");
                    let evaluator = program
                        .designated_evaluator_fragment(fragment.operation_id())
                        .expect("complete designated remote input retains evaluator");
                    let core = evaluator
                        .designated_checked_core()
                        .expect("complete designated remote input retains checked Core");
                    let dependency_index = program
                        .designated_remote_input_dependency_index(fragment)
                        .expect("complete designated remote input retains exact ordinal");
                    designated_remote_input_lineages.insert((
                        dependency.source_owner_locus().to_string(),
                        core.evaluator().to_string(),
                        core.result().to_string(),
                        dependency_index,
                        core.trigger().frontier().unwrap_or_default().to_string(),
                    ));
                }
                ProjectedOperationFragmentKind::RelationPublication => {
                    for transition in ["invalidate_primary", "reacquire_primary"] {
                        relation_transitions
                            .insert((fragment.operation_id().to_string(), transition.to_string()));
                    }
                }
                _ => {}
            }
        }
        let inventory_digest = generation.observer_safe_inventory_digest();
        let (instance, _authority_state, authority_generation, authority_successor) =
            seam.into_kernel_parts().ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramAdmissionMismatch)
            })?;
        Ok(Self {
            program_identity: program.checked_program_identity().clone(),
            program_fingerprint: program.projected_fingerprint(),
            summary: ObserverSafeM9Summary {
                checked_program_identity: program.checked_program_identity().clone(),
                complete_final: true,
                inventory_digest,
                owner_lineages,
                relation_transitions,
                designated_evaluators,
                designated_remote_input_lineages,
                designated_consumers,
            },
            instance,
            authority_live_floor: M9AuthorityLiveFloor::new(authority_generation.clone()),
            authority_generation,
            authority_successor,
            initial_state_seed,
        })
    }

    pub(crate) fn observer_safe_m9_summary(&self) -> &ObserverSafeM9Summary {
        &self.summary
    }

    pub(crate) fn observer_safe_m9_summary_clone(&self) -> ObserverSafeM9Summary {
        self.summary.clone()
    }

    pub(crate) fn observer_safe_m9_semantic_row_sets_clone(&self) -> ObserverSafeM9SemanticRowSets {
        self.summary.semantic_row_sets_clone()
    }

    pub(crate) fn initial_state_seed(&self) -> &Sys4InitialStateSeed {
        &self.initial_state_seed
    }
}

fn validate_seed(program: &FabricProgram, seed: &Sys4InitialStateSeed) -> Sys4Result<()> {
    let known_loci = program.locus_names().into_iter().collect::<BTreeSet<_>>();
    let mut schemas = BTreeMap::<(String, String), BTreeSet<String>>::new();
    let mut indices = BTreeMap::<(String, String), BTreeSet<String>>::new();
    for fragment in program.projection.sys4_artifact_fragments().entries() {
        for schema in fragment.local_state_schemas() {
            let schema_key = (schema.owner_locus().to_string(), schema.name().to_string());
            schemas.insert(
                schema_key.clone(),
                schema
                    .fields()
                    .iter()
                    .map(|field| field.name().to_string())
                    .collect(),
            );
            for read in fragment
                .owner_rmw_checked_core()
                .into_iter()
                .flat_map(|core| std::iter::once(core.target()).chain(core.same_owner_reads()))
            {
                if read.namespace() == schema.name()
                    && let Some(index) = read.index()
                {
                    indices
                        .entry(schema_key.clone())
                        .or_default()
                        .insert(index.to_string());
                }
            }
            if let Some(dependency) = fragment.designated_remote_input_dependency() {
                let read = dependency.typed_state_read();
                if read.namespace() == schema.name()
                    && let Some(index) = read.index()
                {
                    indices
                        .entry(schema_key.clone())
                        .or_default()
                        .insert(index.to_string());
                }
            }
        }
    }
    for (locus, state, index, field) in seed.ints.keys() {
        if !known_loci.contains(locus) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ForeignSeedLocus,
            ));
        }
        let schema_key = (locus.clone(), state.clone());
        let Some(fields) = schemas.get(&schema_key) else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ForeignSeedState,
            ));
        };
        if !indices
            .get(&schema_key)
            .is_some_and(|accepted| accepted.contains(index))
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ForeignSeedIndex,
            ));
        }
        if !fields.contains(field) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ForeignSeedField,
            ));
        }
    }
    Ok(())
}

/// Bounded SYS-4 patch activation is intentionally a private, source-first
/// hand-off.  The runtime receives this already checked/projected/M9-admitted
/// carrier; it never receives source text, an AST, manual routes, or grants.
#[derive(Clone)]
pub(crate) struct Sys4CheckedPatchCandidate {
    patch_id: String,
    base_frontier: Sys4PatchFrontier,
    patch_program: FabricProgram,
    patch_admission: SealedFabricAdmission,
    compatibility: Sys4PatchCompatibility,
}

impl std::fmt::Debug for Sys4CheckedPatchCandidate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys4CheckedPatchCandidate")
            .field("patch_id", &self.patch_id)
            .field("compatible", &self.compatibility.matches())
            .finish_non_exhaustive()
    }
}

impl Sys4CheckedPatchCandidate {
    pub(crate) fn from_prechecked_projected_admitted(
        patch_id: impl Into<String>,
        base_program: &FabricProgram,
        patch_program: FabricProgram,
        patch_admission: SealedFabricAdmission,
    ) -> Sys4Result<Self> {
        let patch_id = patch_id.into();
        if patch_id.is_empty()
            || patch_admission.program_identity != *patch_program.checked_program_identity()
            || patch_admission.program_fingerprint != patch_program.projected_fingerprint()
            || !patch_admission.summary.complete_final
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        let base_frontier = Sys4PatchFrontier::for_candidate_base(
            base_program,
            &patch_admission.authority_generation,
        );
        Ok(Self {
            patch_id,
            base_frontier,
            compatibility: Sys4PatchCompatibility::between(base_program, &patch_program),
            patch_program,
            patch_admission,
        })
    }

    fn patch_admission_is_complete(&self) -> bool {
        self.patch_admission.summary.complete_final
            && self.patch_admission.program_identity
                == *self.patch_program.checked_program_identity()
            && self.patch_admission.program_fingerprint
                == self.patch_program.projected_fingerprint()
            && self
                .patch_admission
                .authority_successor
                .current_generation_for_restore()
                .matches_for_restore(&self.patch_admission.authority_generation)
    }

    #[cfg(test)]
    pub(crate) fn for_test_with_stale_activation_frontier(
        mut self,
        stale_frontier: impl Into<String>,
    ) -> Self {
        self.base_frontier.nonce = stale_frontier.into();
        self
    }
}

/// Private activation identity.  It binds a candidate to the exact projected
/// program and M9 authority inventory currently installed in the fabric.
/// It is not a save format, public API, or wire identity.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Sys4PatchFrontier {
    program_identity: CheckedProgramIdentity,
    program_fingerprint: BTreeSet<(FabricRouteKey, String)>,
    authority_binding: M9CheckedPatchAuthorityBinding,
    activation_generation: u64,
    nonce: String,
}

impl std::fmt::Debug for Sys4PatchFrontier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys4PatchFrontier")
            .field("program_identity", &self.program_identity)
            .field("authority_generation", &self.authority_binding.generation())
            .field("activation_generation", &self.activation_generation)
            .finish_non_exhaustive()
    }
}

impl Sys4PatchFrontier {
    fn for_candidate_base(
        program: &FabricProgram,
        authority_generation: &M9AuthorityGeneration,
    ) -> Self {
        Self {
            program_identity: program.checked_program_identity().clone(),
            program_fingerprint: program.projected_fingerprint(),
            authority_binding: authority_generation.checked_patch_authority_binding(),
            activation_generation: 0,
            nonce: Self::nonce_for(program, authority_generation.generation(), 0),
        }
    }

    fn for_active(
        program: &FabricProgram,
        authority_generation: &M9AuthorityGeneration,
        activation_generation: u64,
    ) -> Self {
        Self {
            program_identity: program.checked_program_identity().clone(),
            program_fingerprint: program.projected_fingerprint(),
            authority_binding: authority_generation.checked_patch_authority_binding(),
            activation_generation,
            nonce: Self::nonce_for(
                program,
                authority_generation.generation(),
                activation_generation,
            ),
        }
    }

    fn nonce_for(
        program: &FabricProgram,
        authority_generation: u64,
        activation_generation: u64,
    ) -> String {
        format!(
            "sys4-patch-frontier:{}:{authority_generation}:{activation_generation}",
            program.checked_program_identity().stable_key()
        )
    }

    pub(crate) const fn is_exact_successor_of(&self, base: &Self) -> bool {
        self.activation_generation == base.activation_generation.saturating_add(1)
    }

    fn has_same_program_projection_and_activation(&self, other: &Self) -> bool {
        self.program_identity == other.program_identity
            && self.program_fingerprint == other.program_fingerprint
            && self.activation_generation == other.activation_generation
    }

    fn has_well_formed_nonce(&self) -> bool {
        self.nonce
            == format!(
                "sys4-patch-frontier:{}:{}:{}",
                self.program_identity.stable_key(),
                self.authority_binding.generation(),
                self.activation_generation,
            )
    }

    fn authority_binding(&self) -> &M9CheckedPatchAuthorityBinding {
        &self.authority_binding
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys4PatchCompatibility {
    mismatch: Option<Sys4PatchDiagnosticKind>,
}

impl Sys4PatchCompatibility {
    fn between(base: &FabricProgram, patch: &FabricProgram) -> Self {
        let base_shape = patch_compatibility_shape(base);
        let patch_shape = patch_compatibility_shape(patch);
        let mismatch = if !base_shape.has_same_topology_and_schema(&patch_shape) {
            Some(Sys4PatchDiagnosticKind::TopologyOwnerRouteMismatch)
        } else if base_shape.owner_rmw_fragments != patch_shape.owner_rmw_fragments {
            Some(Sys4PatchDiagnosticKind::OwnerRmwExpressionChanged)
        } else if base_shape.non_designated_fragments != patch_shape.non_designated_fragments
            || base_shape.non_designated_edges != patch_shape.non_designated_edges
            || base_shape.non_designated_effect_handlers
                != patch_shape.non_designated_effect_handlers
            || base_shape.relation_graph != patch_shape.relation_graph
        {
            Some(Sys4PatchDiagnosticKind::NonDesignatedCoreMaterialChanged)
        } else {
            None
        };
        Self { mismatch }
    }

    fn matches(&self) -> bool {
        self.mismatch.is_none()
    }

    fn diagnostic(&self) -> Sys4PatchDiagnosticKind {
        self.mismatch
            .expect("checked patch compatibility diagnostic exists only for a mismatch")
    }
}

type Sys4PatchStateSchemaFields = Vec<(String, String, Option<String>)>;
type Sys4PatchStateSchemaShape = (String, String, String, String, Sys4PatchStateSchemaFields);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys4PatchCompatibilityShape {
    loci: BTreeSet<String>,
    fragments: BTreeSet<(String, String, String, String, String, String)>,
    edges: BTreeSet<(String, String, String, String)>,
    state_schemas: BTreeSet<Sys4PatchStateSchemaShape>,
    owner_rmw_fragments: Vec<String>,
    non_designated_fragments: Vec<String>,
    non_designated_edges: Vec<String>,
    non_designated_effect_handlers: Vec<String>,
    relation_graph: crate::sys3_projection::ProjectionRelationGraph,
}

impl Sys4PatchCompatibilityShape {
    fn has_same_topology_and_schema(&self, other: &Self) -> bool {
        self.loci == other.loci
            && self.fragments == other.fragments
            && self.edges == other.edges
            && self.state_schemas == other.state_schemas
    }
}

fn designated_patch_operation_ids(program: &FabricProgram) -> BTreeSet<String> {
    program
        .projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .filter(|fragment| {
            matches!(
                fragment.fragment_kind(),
                ProjectedOperationFragmentKind::DesignatedEvaluation
                    | ProjectedOperationFragmentKind::DesignatedResultConsumer
            )
        })
        .map(|fragment| fragment.operation_id().to_string())
        .collect()
}

fn local_fragment_shape(fragment: &ProjectedOperationFragment) -> String {
    let placement = if let Some(signature) = fragment.typed_input_signature() {
        format!("owner-request:{signature:?}")
    } else if let Some(core) = fragment.owner_rmw_checked_core() {
        format!("owner-rmw:{core:?}")
    } else if let Some(core) = fragment.relation_checked_core() {
        format!("relation-checked-core:{core:?}")
    } else if let Some(descriptor) = fragment.consumer_relation_projection() {
        format!("consumer-relation-projection:{descriptor:?}")
    } else if let Some(dependency) = fragment.designated_remote_input_dependency() {
        format!("designated-remote-input:{dependency:?}")
    } else if let Some(core) = fragment.designated_checked_core() {
        format!("designated-evaluator:{core:?}")
    } else if let Some(core) = fragment.designated_result_consumer_core() {
        format!("designated-result-consumer:{core:?}")
    } else {
        "placement:none".to_string()
    };
    // Do not compare `checked_core_identity` wholesale: it contains the
    // checked-program identity, which necessarily changes for an allowed
    // designated expression patch. Everything below is the fragment's local
    // Core, authority/failure/effect material, and source provenance.
    format!(
        "operation={};kind={:?};locus={};source_ref={:?};core_ref={};fragment_ref={};authority={:?};declared_failure={:?};generated_failure={:?};obligations={:?};runtime_seam={:?};state_schemas={:?};placement={placement}",
        fragment.operation_id(),
        fragment.fragment_kind(),
        fragment.locus_tag().as_str(),
        fragment.source_ref(),
        fragment.core_ref().unwrap_or_default(),
        fragment.fragment_ref(),
        fragment.authority_requirements(),
        fragment.declared_failure_names(),
        fragment.generated_failure_names(),
        fragment.semantic_obligations().rows(),
        fragment.runtime_seam_requirements().rows(),
        fragment.local_state_schemas(),
    )
}

fn sorted_owner_rmw_fragment_shapes(program: &FabricProgram) -> Vec<String> {
    let mut fragments = program
        .projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .filter(|fragment| fragment.owner_rmw_checked_core().is_some())
        .map(local_fragment_shape)
        .collect::<Vec<_>>();
    fragments.sort();
    fragments
}

fn sorted_non_designated_fragments(program: &FabricProgram) -> Vec<String> {
    let designated_operations = designated_patch_operation_ids(program);
    let mut fragments = program
        .projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .filter(|fragment| !designated_operations.contains(fragment.operation_id()))
        .map(local_fragment_shape)
        .collect::<Vec<_>>();
    fragments.sort();
    fragments
}

fn local_edge_shape(edge: &CommunicationEdge) -> String {
    // As for fragments, checked-core identity is intentionally decomposed:
    // its whole-program member changes for an allowed designated delta while
    // the generated edge's local source/Core/contract content must not.
    format!(
        "operation={};kind={:?};source={};target={};source_ref={:?};core_ref={:?};edge_ref={};source_fragment_ref={};target_fragment_ref={};derived={};transfers_authority={};carrier_contract={:?}",
        edge.operation_id(),
        edge.kind(),
        edge.source_locus(),
        edge.target_locus(),
        edge.source_ref(),
        edge.core_ref(),
        edge.edge_ref(),
        edge.source_fragment_ref(),
        edge.target_fragment_ref(),
        edge.is_derived_from_checked_core(),
        edge.transfers_authority(),
        edge.carrier_contract(),
    )
}

fn sorted_non_designated_edges(program: &FabricProgram) -> Vec<String> {
    let designated_operations = designated_patch_operation_ids(program);
    let mut edges = program
        .projection
        .communication_plan()
        .edges()
        .iter()
        .filter(|edge| !designated_operations.contains(edge.operation_id()))
        .map(local_edge_shape)
        .collect::<Vec<_>>();
    edges.sort();
    edges
}

fn non_designated_effect_handlers(program: &FabricProgram) -> Vec<String> {
    let designated_operations = designated_patch_operation_ids(program);
    program
        .projection
        .effect_handler_plan()
        .entries()
        .iter()
        .filter(|handler| !designated_operations.contains(handler.operation()))
        .map(|handler| {
            format!(
                "operation={};kind={:?};locus={};source_ref={:?};core_ref={:?};handler_ref={};source_bound={};effect={:?};declared_failure={:?};generated_failure={:?}",
                handler.operation(),
                handler.kind(),
                handler.locus(),
                handler.source_ref(),
                handler.core_ref(),
                handler.handler_ref(),
                handler.is_source_bound(),
                handler.effect_row().kinds(),
                handler.declared_failure_row().names(),
                handler.generated_failure_row().names(),
            )
        })
        .collect::<Vec<_>>()
}

fn patch_compatibility_shape(program: &FabricProgram) -> Sys4PatchCompatibilityShape {
    // `non_designated_fragments` below invokes `local_fragment_shape`, which
    // binds every relation_checked_core's primary/fallback lineage together
    // with its local source_ref and core_ref.  Those local Core/provenance
    // fields remain stable across a permitted designated-only expression
    // delta, unlike the enclosing checked-program identity.
    let fragments = program
        .projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .map(|fragment| {
            let placement = if let Some(core) = fragment.owner_rmw_checked_core() {
                format!(
                    "owner:{}:{}",
                    core.authority_origin_locus(),
                    core.owner_locus()
                )
            } else if let Some(dependency) = fragment.designated_remote_input_dependency() {
                format!(
                    "designated-source:{}:{}",
                    dependency.source_owner_locus(),
                    dependency.designated_evaluator()
                )
            } else if let Some(core) = fragment.designated_checked_core() {
                format!(
                    "designated-evaluator:{}:{}",
                    core.evaluator(),
                    core.result()
                )
            } else if let Some(core) = fragment.designated_result_consumer_core() {
                format!(
                    "designated-consumer:{}:{}",
                    core.evaluator(),
                    core.consumer_locus()
                )
            } else if fragment.relation_checked_core().is_some() {
                "RelationPublication".to_string()
            } else if fragment.consumer_relation_projection().is_some() {
                "ConsumerLocalRelationProjection".to_string()
            } else {
                "placement:none".to_string()
            };
            (
                fragment.operation_id().to_string(),
                format!("{:?}", fragment.fragment_kind()),
                fragment.locus_tag().as_str().to_string(),
                fragment.origin_locus().unwrap_or_default().to_string(),
                fragment
                    .target_owner_locus()
                    .unwrap_or_default()
                    .to_string(),
                placement,
            )
        })
        .collect();
    let edges = program
        .projection
        .communication_plan()
        .edges()
        .iter()
        .map(|edge| {
            (
                edge.operation_id().to_string(),
                format!("{:?}", edge.kind()),
                edge.source_locus().to_string(),
                edge.target_locus().to_string(),
            )
        })
        .collect();
    let state_schemas = program
        .projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .flat_map(|fragment| fragment.local_state_schemas().iter())
        .map(|schema| {
            (
                schema.owner_locus().to_string(),
                schema.name().to_string(),
                schema.index_name().to_string(),
                schema.index_type().to_string(),
                schema
                    .fields()
                    .iter()
                    .map(|field| {
                        (
                            field.name().to_string(),
                            field.type_name().to_string(),
                            field.visibility_channel().map(ToOwned::to_owned),
                        )
                    })
                    .collect(),
            )
        })
        .collect();
    Sys4PatchCompatibilityShape {
        loci: program.locus_names().into_iter().collect(),
        fragments,
        edges,
        state_schemas,
        owner_rmw_fragments: sorted_owner_rmw_fragment_shapes(program),
        non_designated_fragments: sorted_non_designated_fragments(program),
        non_designated_edges: sorted_non_designated_edges(program),
        non_designated_effect_handlers: non_designated_effect_handlers(program),
        relation_graph: program.projection.relation_graph().clone(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys4PatchVerdict {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys4PatchDiagnosticKind {
    StaleFrontier,
    NonQuiescentPendingCarrier,
    TopologyOwnerRouteMismatch,
    OwnerRmwExpressionChanged,
    NonDesignatedCoreMaterialChanged,
    M9AuthorityLineageMismatch,
    IncompleteCandidateAdmission,
    BackendIneligible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4PatchBoundaryInspection {
    candidate_was_prechecked_projected_and_m9_admitted: bool,
}

impl Sys4PatchBoundaryInspection {
    pub(crate) const fn candidate_was_prechecked_projected_and_m9_admitted(&self) -> bool {
        self.candidate_was_prechecked_projected_and_m9_admitted
    }

    pub(crate) const fn runtime_received_only_checked_patch_candidate(&self) -> bool {
        self.candidate_was_prechecked_projected_and_m9_admitted
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4PatchLifecycle {
    verdict: Sys4PatchVerdict,
    diagnostic: Option<Sys4PatchDiagnosticKind>,
    source_first_checked_projection_and_m9_admission: bool,
}

impl Sys4PatchLifecycle {
    pub(crate) const fn contains_source_first_checked_projection_and_m9_admission(&self) -> bool {
        self.source_first_checked_projection_and_m9_admission
    }

    pub(crate) fn is_lifecycle_only_rejection(&self) -> bool {
        self.verdict == Sys4PatchVerdict::Rejected && self.diagnostic.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4PatchOutcome {
    verdict: Sys4PatchVerdict,
    primary_diagnostic_kind: Option<Sys4PatchDiagnosticKind>,
    lifecycle: Sys4PatchLifecycle,
    boundary_inspection: Sys4PatchBoundaryInspection,
    base_frontier: Sys4PatchFrontier,
    activation_frontier: Sys4PatchFrontier,
    m9_authority_frontier_mismatch: Option<M9AuthorityFrontierMismatchInspection>,
    m9_live_floor_recheck: Option<M9LiveFloorRecheckInspection>,
}

/// Observer-safe explanation of an exact M9 authority mismatch at the
/// checked-patch boundary.  It carries sealed identifiers and digests only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9AuthorityFrontierMismatchInspection {
    active_generation: M9SealedGeneration,
    candidate_generation: u64,
    candidate_generation_ref: String,
    active_authority_lineage_digest: String,
    candidate_authority_lineage_digest: String,
}

impl M9AuthorityFrontierMismatchInspection {
    pub(crate) fn same_numeric_generation(&self) -> bool {
        self.active_generation.generation() == self.candidate_generation
    }

    pub(crate) fn active_generation(&self) -> M9SealedGeneration {
        self.active_generation.clone()
    }

    pub(crate) fn active_generation_ref(&self) -> &str {
        self.active_generation.generation_ref()
    }

    pub(crate) fn candidate_generation_ref(&self) -> &str {
        &self.candidate_generation_ref
    }

    pub(crate) fn active_authority_lineage_digest(&self) -> &str {
        &self.active_authority_lineage_digest
    }

    pub(crate) fn candidate_authority_lineage_digest(&self) -> &str {
        &self.candidate_authority_lineage_digest
    }

    pub(crate) const fn compared_exact_m9_identity_and_lineage_not_numeric_generation_only(
        &self,
    ) -> bool {
        true
    }
}

/// Observer-safe record that patch activation rechecked the shared M9 live
/// floor while holding the same floor guard used for the final swap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M9LiveFloorRecheckInspection {
    current_generation: M9SealedGeneration,
    shared_live_floor_identity: usize,
}

impl M9LiveFloorRecheckInspection {
    pub(crate) fn current_generation(&self) -> M9SealedGeneration {
        self.current_generation.clone()
    }

    pub(crate) const fn shared_live_floor_identity(&self) -> usize {
        self.shared_live_floor_identity
    }

    pub(crate) const fn checked_exact_current_generation_and_lineage(&self) -> bool {
        true
    }
}

impl Sys4PatchOutcome {
    pub(crate) const fn verdict(&self) -> Sys4PatchVerdict {
        self.verdict
    }

    pub(crate) const fn primary_diagnostic_kind(&self) -> Option<Sys4PatchDiagnosticKind> {
        self.primary_diagnostic_kind
    }

    pub(crate) fn lifecycle(&self) -> &Sys4PatchLifecycle {
        &self.lifecycle
    }

    pub(crate) fn boundary_inspection(&self) -> &Sys4PatchBoundaryInspection {
        &self.boundary_inspection
    }

    pub(crate) fn base_frontier(&self) -> &Sys4PatchFrontier {
        &self.base_frontier
    }

    pub(crate) fn activation_frontier(&self) -> &Sys4PatchFrontier {
        &self.activation_frontier
    }

    pub(crate) fn m9_authority_frontier_mismatch_inspection(
        &self,
    ) -> Option<&M9AuthorityFrontierMismatchInspection> {
        self.m9_authority_frontier_mismatch.as_ref()
    }

    pub(crate) fn m9_live_floor_recheck_inspection(&self) -> Option<&M9LiveFloorRecheckInspection> {
        self.m9_live_floor_recheck.as_ref()
    }

    pub(crate) const fn exposes_raw_source_or_authority_material(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sys4PatchLifecycleRow {
    Accepted,
    Rejected(Sys4PatchDiagnosticKind),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Sys4PatchLifecycleLog {
    rows: Vec<Sys4PatchLifecycleRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4PatchLifecycleSnapshot {
    rows: Vec<Sys4PatchLifecycleRow>,
}

impl Sys4PatchLifecycleSnapshot {
    pub(crate) fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn extends_only_with_lifecycle_rows_since(&self, prior: &Self) -> bool {
        self.rows.starts_with(&prior.rows)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceAction {
    kind: SourceActionKind,
    arguments: BTreeMap<String, String>,
    tick: Option<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SourceActionKind {
    OwnerOperation(String),
    DesignatedTick(String),
    ConsumeDesignatedResult(String),
}

impl SourceAction {
    pub(crate) fn owner_operation(operation: impl Into<String>) -> Self {
        Self {
            kind: SourceActionKind::OwnerOperation(operation.into()),
            arguments: BTreeMap::new(),
            tick: None,
        }
    }

    pub(crate) fn designated_tick(value_name: impl Into<String>) -> Self {
        Self {
            kind: SourceActionKind::DesignatedTick(value_name.into()),
            arguments: BTreeMap::new(),
            tick: None,
        }
    }

    pub(crate) fn consume_designated_result(value_name: impl Into<String>) -> Self {
        Self {
            kind: SourceActionKind::ConsumeDesignatedResult(value_name.into()),
            arguments: BTreeMap::new(),
            tick: None,
        }
    }

    pub(crate) fn with_argument(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.arguments.insert(name.into(), value.into());
        self
    }

    pub(crate) fn with_tick(
        mut self,
        frontier: impl Into<String>,
        tick: impl Into<String>,
    ) -> Self {
        self.tick = Some((frontier.into(), tick.into()));
        self
    }

    pub(crate) fn operation_id(&self) -> &str {
        match &self.kind {
            SourceActionKind::OwnerOperation(operation)
            | SourceActionKind::DesignatedTick(operation)
            | SourceActionKind::ConsumeDesignatedResult(operation) => operation,
        }
    }

    pub(crate) const fn origin_locus_override(&self) -> Option<&str> {
        None
    }
    pub(crate) const fn authority_principal_override(&self) -> Option<&str> {
        None
    }
    pub(crate) const fn target_locus_override(&self) -> Option<&str> {
        None
    }
    pub(crate) const fn can_carry_checked_core_identity(&self) -> bool {
        false
    }
    pub(crate) const fn can_carry_authority_grant(&self) -> bool {
        false
    }
    pub(crate) const fn can_carry_state_delta(&self) -> bool {
        false
    }
    pub(crate) const fn can_carry_expected_result(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FaultInjection {
    edge_ref: String,
    envelope_id: Option<String>,
    target_locus: Option<String>,
    replacement_core_ref: Option<String>,
    replacement_policy_stamp: Option<String>,
    replacement_redaction_policy: Option<String>,
    replacement_m8_publication_id: Option<String>,
    replacement_source_ref: Option<SourceRefView>,
    replacement_visibility_label: Option<String>,
    kind: FaultInjectionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FaultInjectionKind {
    RouteUnavailable,
    Retarget,
    CorruptVisibilityRedaction,
    CorruptSourceRef,
    CorruptVisibility,
    CorruptPolicyStamp,
    StripIntPayload,
    CorruptM8PublicationId,
    CorruptCacheBindingDigest,
    RewriteCacheRetryProjectionBinding,
}

impl FaultInjection {
    pub(crate) fn route_unavailable(operation: impl Into<String>) -> Self {
        Self {
            edge_ref: operation.into(),
            envelope_id: None,
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::RouteUnavailable,
        }
    }

    pub(crate) fn route_unavailable_for_edge(edge_ref: impl Into<String>) -> Self {
        Self::route_unavailable(edge_ref)
    }

    pub(crate) fn retarget_in_transit_envelope_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
        target_locus: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: Some(target_locus.into()),
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::Retarget,
        }
    }

    pub(crate) fn drop_in_transit_envelope_payload_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::StripIntPayload,
        }
    }

    pub(crate) fn corrupt_in_transit_envelope_policy_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::CorruptPolicyStamp,
        }
    }

    pub(crate) fn corrupt_in_transit_envelope_redaction_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::CorruptVisibilityRedaction,
        }
    }

    pub(crate) fn corrupt_in_transit_envelope_m8_publication_id_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
        forged_publication_id: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: Some(forged_publication_id.into()),
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::CorruptM8PublicationId,
        }
    }

    pub(crate) fn corrupt_in_transit_envelope_source_ref_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
        forged_source_path: impl Into<String>,
    ) -> Self {
        let forged_source = SourceRef::new(forged_source_path.into(), 1, 1, 1, 1);
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: Some(SourceRefView::new(&forged_source)),
            replacement_visibility_label: None,
            kind: FaultInjectionKind::CorruptSourceRef,
        }
    }

    pub(crate) fn corrupt_in_transit_envelope_visibility_for_edge(
        edge_ref: impl Into<String>,
        envelope_id: impl Into<String>,
        forged_visibility_label: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: Some(forged_visibility_label.into()),
            kind: FaultInjectionKind::CorruptVisibility,
        }
    }

    pub(crate) fn corrupt_local_cache_retry_binding_digest(envelope_id: impl Into<String>) -> Self {
        Self {
            edge_ref: String::new(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: None,
            replacement_policy_stamp: None,
            replacement_redaction_policy: None,
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::CorruptCacheBindingDigest,
        }
    }

    pub(crate) fn rewrite_local_cache_retry_projection_binding_for_edge(
        envelope_id: impl Into<String>,
        edge_ref: impl Into<String>,
        core_ref: impl Into<String>,
        policy_stamp: impl Into<String>,
        redaction_policy: impl Into<String>,
    ) -> Self {
        Self {
            edge_ref: edge_ref.into(),
            envelope_id: Some(envelope_id.into()),
            target_locus: None,
            replacement_core_ref: Some(core_ref.into()),
            replacement_policy_stamp: Some(policy_stamp.into()),
            replacement_redaction_policy: Some(redaction_policy.into()),
            replacement_m8_publication_id: None,
            replacement_source_ref: None,
            replacement_visibility_label: None,
            kind: FaultInjectionKind::RewriteCacheRetryProjectionBinding,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InTransitFault {
    edge_ref: String,
    envelope_id: Option<String>,
    kind: FaultInjectionKind,
    target_locus: Option<String>,
    replacement_m8_publication_id: Option<String>,
    replacement_source_ref: Option<SourceRefView>,
    replacement_visibility_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct InTransitFaults {
    entries: Vec<InTransitFault>,
}
impl InTransitFaults {
    pub(crate) fn affects_edge(&self, edge_ref: &str) -> bool {
        self.entries.iter().any(|entry| entry.edge_ref == edge_ref)
    }
    pub(crate) fn affects_exact_envelope(&self, edge_ref: &str, envelope_id: &str) -> bool {
        self.entries.iter().any(|entry| {
            entry.edge_ref == edge_ref && entry.envelope_id.as_deref() == Some(envelope_id)
        })
    }
    fn take_exact(&mut self, edge_ref: &str, envelope_id: &str) -> Option<InTransitFault> {
        let index = self.entries.iter().position(|entry| {
            entry.edge_ref == edge_ref && entry.envelope_id.as_deref() == Some(envelope_id)
        })?;
        Some(self.entries.remove(index))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExternalAction {
    kind: ExternalActionKind,
    target_override: Option<String>,
    authority_override: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExternalActionKind {
    Source(SourceAction),
    Fault(FaultInjection),
}

impl ExternalAction {
    pub(crate) fn source_operation(source: SourceAction) -> Self {
        Self {
            kind: ExternalActionKind::Source(source),
            target_override: None,
            authority_override: None,
        }
    }

    pub(crate) fn fault_event(fault: FaultInjection) -> Self {
        Self {
            kind: ExternalActionKind::Fault(fault),
            target_override: None,
            authority_override: None,
        }
    }

    pub(crate) const fn is_source_operation(&self) -> bool {
        matches!(self.kind, ExternalActionKind::Source(_))
    }
    pub(crate) const fn is_fault_event(&self) -> bool {
        matches!(self.kind, ExternalActionKind::Fault(_))
    }
    pub(crate) fn target_locus_override(&self) -> Option<&str> {
        self.target_override.as_deref()
    }
    pub(crate) fn authority_principal_override(&self) -> Option<&str> {
        self.authority_override.as_deref()
    }
    pub(crate) const fn can_carry_checked_core_identity(&self) -> bool {
        false
    }
    pub(crate) const fn can_carry_authority_grant(&self) -> bool {
        false
    }
    pub(crate) const fn can_carry_state_delta(&self) -> bool {
        false
    }
    pub(crate) const fn can_carry_expected_result(&self) -> bool {
        false
    }

    #[cfg(test)]
    pub(crate) fn for_test_attempt_target_override(source: SourceAction, target: &str) -> Self {
        Self {
            kind: ExternalActionKind::Source(source),
            target_override: Some(target.to_string()),
            authority_override: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_attempt_authority_override(
        source: SourceAction,
        authority: &str,
    ) -> Self {
        Self {
            kind: ExternalActionKind::Source(source),
            target_override: None,
            authority_override: Some(authority.to_string()),
        }
    }
}

/// One opaque generated carrier record held by the endpoint that actually
/// enqueued or dequeued it.  It carries only source/Core-derived routing and
/// provenance; authority facts remain sealed in M9/M8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndpointCarrierRecord {
    record_id: String,
    carrier_id: String,
    request_id: String,
    edge_kind: CommunicationEdgeKind,
    edge_ref: String,
    source_locus: String,
    target_locus: String,
    enqueue_occurrence_id: Option<String>,
    dequeue_occurrence_id: Option<String>,
    request_carrier_id: Option<String>,
    input_receipt_carrier_id: Option<String>,
    source_ref: SourceRefView,
    core_ref: Option<String>,
    source_fragment_ref: String,
    target_fragment_ref: String,
}

impl EndpointCarrierRecord {
    pub(crate) fn carrier_id(&self) -> &str {
        &self.carrier_id
    }

    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub(crate) fn source_ref(&self) -> SourceRefView {
        self.source_ref.clone()
    }
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }
    pub(crate) fn source_fragment_ref(&self) -> &String {
        &self.source_fragment_ref
    }
    pub(crate) fn target_fragment_ref(&self) -> &String {
        &self.target_fragment_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct EndpointCarrierHistory {
    records: Vec<EndpointCarrierRecord>,
}

impl EndpointCarrierHistory {
    pub(crate) fn carrier_history_len(&self) -> usize {
        self.records.len()
    }

    pub(crate) fn carrier_history_for_request(&self, request_id: &str) -> EndpointCarrierHistory {
        EndpointCarrierHistory {
            records: self
                .records
                .iter()
                .filter(|record| record.request_id == request_id)
                .cloned()
                .collect(),
        }
    }

    pub(crate) fn single(
        &self,
        edge_kind: CommunicationEdgeKind,
        source_locus: &str,
        target_locus: &str,
    ) -> EndpointCarrierRecord {
        self.records
            .iter()
            .find(|record| {
                record.edge_kind == edge_kind
                    && record.source_locus == source_locus
                    && record.target_locus == target_locus
            })
            .cloned()
            .expect("generated endpoint history has the requested unique carrier record")
    }

    fn append(&mut self, record: EndpointCarrierRecord) {
        self.records.push(record);
    }
}

/// An immutable, projection-derived carrier payload.  The only mutable
/// mailbox metadata is replaced when the envelope changes ownership from an
/// outbox to an inbox; source/Core/contract facts themselves are copied from
/// the exact communication edge and never recomputed by a receiver.
#[derive(Debug, Clone, PartialEq, Eq)]
enum MailboxPayload {
    OwnerRequest {
        arguments: BTreeMap<String, String>,
    },
    OwnerReply {
        receipt: Box<FabricReceipt>,
    },
    DesignatedInputRequest {
        frontier: String,
        tick: String,
    },
    DesignatedInputReceipt {
        source_value: Option<i64>,
        frontier: String,
        tick: String,
    },
    DesignatedDelivery {
        value: Option<i64>,
        publication: Box<M8PublishedDesignatedValue>,
    },
    /// An immutable owner relation-state publication.  It is accepted only
    /// on the generated `RelationProjectionPublication` endpoint and is
    /// imported as a consumer shadow rather than as remote mutable state.
    RelationPublication {
        publication: Box<M8PublishedRelationState>,
        target_admission: M9RelationPublicationAdmission,
    },
    CacheRetry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SealedDeliveryBinding {
    carrier_contract: CarrierContract,
    source_ref: SourceRefView,
    core_ref: Option<String>,
    source_fragment_ref: String,
    target_fragment_ref: String,
    input_frontier: Option<String>,
    result_frontier: Option<String>,
    result_version: Option<ResultVersion>,
    consumer_locus: String,
    policy_stamp: Option<String>,
    visibility_policy: ReferenceOnlyRedactionPolicy,
    redaction_policy: String,
    // Exact M8-produced visibility evidence travels with the generated
    // carrier.  It is checked again before consumer admission/import and is
    // not reconstructed from a later publication lookup.
    m8_visibility_label: String,
    m8_visibility_class: M8SecurityClass,
    m8_redaction: String,
    m8_source_ref: SourceRefView,
    m8_publication_id: String,
    logical_tick_id: String,
    logical_tick_frontier: String,
}

impl SealedDeliveryBinding {
    pub(crate) fn source_ref(&self) -> SourceRefView {
        self.source_ref.clone()
    }
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }
    pub(crate) fn source_fragment_ref(&self) -> &String {
        &self.source_fragment_ref
    }
    pub(crate) fn target_fragment_ref(&self) -> &String {
        &self.target_fragment_ref
    }
    pub(crate) fn input_frontier(&self) -> Option<&InputFrontier> {
        self.carrier_contract.input_frontier()
    }
    pub(crate) fn result_frontier(&self) -> Option<&ResultFrontier> {
        self.carrier_contract.result_frontier()
    }
    pub(crate) fn result_version(&self) -> ResultVersion {
        self.carrier_contract
            .result_version()
            .expect("designated delivery binding has result version")
    }
    pub(crate) fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }
    pub(crate) fn policy_stamp(&self) -> Option<&PolicyStamp> {
        self.carrier_contract.policy_stamp()
    }
    pub(crate) fn visibility_policy(&self) -> &ReferenceOnlyRedactionPolicy {
        &self.visibility_policy
    }
    pub(crate) fn redaction_policy(&self) -> &str {
        &self.redaction_policy
    }
    pub(crate) fn m8_publication_id(&self) -> &str {
        &self.m8_publication_id
    }
    pub(crate) fn logical_tick_id(&self) -> &str {
        &self.logical_tick_id
    }
    pub(crate) fn logical_tick_frontier(&self) -> &str {
        &self.logical_tick_frontier
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MailboxEnvelope {
    envelope_id: String,
    carrier_id: String,
    request_id: String,
    operation_id: String,
    edge_kind: CommunicationEdgeKind,
    edge_ref: String,
    source_locus: String,
    target_locus: String,
    carrier_contract: CarrierContract,
    source_ref: SourceRefView,
    core_ref: Option<String>,
    source_fragment_ref: String,
    target_fragment_ref: String,
    mailbox_record_id: String,
    mailbox_enqueue_occurrence_id: String,
    request_carrier_id: Option<String>,
    input_receipt_carrier_id: Option<String>,
    m9_owner_lineage_ref: Option<String>,
    m9_source_release_lineage: Option<M9DesignatedSourceReleaseLineage>,
    semantic_identity: Option<String>,
    immutable_delivery_binding: Option<SealedDeliveryBinding>,
    immutable_delivery_digest: Option<String>,
    m8_publication_id: Option<String>,
    m8_evaluation_node_id: Option<String>,
    logical_tick_id: Option<String>,
    logical_tick_frontier: Option<String>,
    payload: MailboxPayload,
}

impl MailboxEnvelope {
    pub(crate) fn envelope_id(&self) -> &str {
        &self.envelope_id
    }
    pub(crate) fn carrier_id(&self) -> &str {
        &self.carrier_id
    }
    pub(crate) const fn edge_kind(&self) -> CommunicationEdgeKind {
        self.edge_kind
    }
    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }
    pub(crate) fn carrier_contract(&self) -> &CarrierContract {
        &self.carrier_contract
    }
    pub(crate) fn source_ref(&self) -> SourceRefView {
        self.source_ref.clone()
    }
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }
    pub(crate) fn source_fragment_ref(&self) -> &String {
        &self.source_fragment_ref
    }
    pub(crate) fn target_fragment_ref(&self) -> &String {
        &self.target_fragment_ref
    }
    pub(crate) fn mailbox_record_id(&self) -> &str {
        &self.mailbox_record_id
    }
    pub(crate) fn request_carrier_id(&self) -> &str {
        self.request_carrier_id.as_deref().unwrap_or("")
    }
    pub(crate) fn input_receipt_carrier_id(&self) -> &str {
        self.input_receipt_carrier_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m9_owner_lineage_ref(&self) -> &str {
        self.m9_owner_lineage_ref.as_deref().unwrap_or("")
    }
    pub(crate) fn m9_source_release_lineage(&self) -> &M9DesignatedSourceReleaseLineage {
        self.m9_source_release_lineage
            .as_ref()
            .expect("only designated input requests carry a source-release lineage")
    }
    pub(crate) fn semantic_identity(&self) -> &str {
        self.semantic_identity.as_deref().unwrap_or("")
    }
    pub(crate) fn immutable_delivery_binding(&self) -> &SealedDeliveryBinding {
        self.immutable_delivery_binding
            .as_ref()
            .expect("only designated deliveries carry immutable binding")
    }
    pub(crate) fn immutable_delivery_digest(&self) -> &str {
        self.immutable_delivery_digest
            .as_deref()
            .expect("only designated deliveries carry immutable digest")
    }
    pub(crate) fn m8_publication_id(&self) -> &str {
        self.m8_publication_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m8_evaluation_node_id(&self) -> &str {
        self.m8_evaluation_node_id.as_deref().unwrap_or("")
    }
    pub(crate) fn logical_tick_id(&self) -> &str {
        self.logical_tick_id.as_deref().unwrap_or("")
    }
    pub(crate) fn logical_tick_frontier(&self) -> &str {
        self.logical_tick_frontier.as_deref().unwrap_or("")
    }
    pub(crate) fn designated_tick_id(&self) -> &str {
        self.logical_tick_id()
    }
    pub(crate) fn designated_tick_frontier(&self) -> &str {
        self.logical_tick_frontier()
    }
    pub(crate) const fn is_local_cache_retry(&self) -> bool {
        matches!(self.payload, MailboxPayload::CacheRetry)
    }
    pub(crate) fn input_frontier(&self) -> Option<&InputFrontier> {
        self.carrier_contract.input_frontier()
    }
    pub(crate) fn result_frontier(&self) -> Option<&ResultFrontier> {
        self.carrier_contract.result_frontier()
    }
    pub(crate) fn observation_policy(&self) -> Option<&ObservationPolicy> {
        self.carrier_contract.observation_policy()
    }
    pub(crate) fn policy_stamp(&self) -> Option<&PolicyStamp> {
        self.carrier_contract.policy_stamp()
    }
    pub(crate) fn visibility_policy(&self) -> &ReferenceOnlyRedactionPolicy {
        self.carrier_contract.visibility_policy()
    }
    pub(crate) fn redaction_policy(&self) -> &str {
        self.immutable_delivery_binding
            .as_ref()
            .map_or("", SealedDeliveryBinding::redaction_policy)
    }
    pub(crate) fn typed_value(&self) -> RuntimeValue {
        match self.payload {
            MailboxPayload::DesignatedInputReceipt {
                source_value: Some(value),
                ..
            }
            | MailboxPayload::DesignatedDelivery {
                value: Some(value), ..
            } => RuntimeValue::int(value),
            _ => RuntimeValue::unit(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PendingMailboxEnvelopes {
    entries: Vec<MailboxEnvelope>,
}

impl PendingMailboxEnvelopes {
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub(crate) fn single(&self) -> MailboxEnvelope {
        self.entries
            .first()
            .cloned()
            .expect("mailbox has the requested single envelope")
    }
    pub(crate) fn for_request(&self, request_id: &str) -> MailboxEnvelope {
        self.entries
            .iter()
            .find(|entry| entry.request_id == request_id)
            .cloned()
            .expect("mailbox retains the requested carrier")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct OutgoingMailbox {
    pending: VecDeque<MailboxEnvelope>,
}

impl OutgoingMailbox {
    pub(crate) fn pending_envelopes(&self) -> PendingMailboxEnvelopes {
        PendingMailboxEnvelopes {
            entries: self.pending.iter().cloned().collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MailboxEnvelopeTerminalState {
    RejectedQuarantined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObserverSafeMailboxAudit;
impl ObserverSafeMailboxAudit {
    pub(crate) const fn is_observer_safe(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalRejectedMailboxEnvelope {
    envelope_id: String,
    terminal_state: MailboxEnvelopeTerminalState,
    diagnostic_kind: Sys4DiagnosticKind,
    observer_safe_audit: ObserverSafeMailboxAudit,
}
impl TerminalRejectedMailboxEnvelope {
    pub(crate) const fn terminal_state(&self) -> MailboxEnvelopeTerminalState {
        self.terminal_state
    }
    pub(crate) const fn diagnostic_kind(&self) -> Sys4DiagnosticKind {
        self.diagnostic_kind
    }
    pub(crate) fn observer_safe_audit(&self) -> &ObserverSafeMailboxAudit {
        &self.observer_safe_audit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct IncomingMailbox {
    pending: VecDeque<MailboxEnvelope>,
    terminal: BTreeMap<String, TerminalRejectedMailboxEnvelope>,
}
impl IncomingMailbox {
    pub(crate) fn pending_envelopes(&self) -> PendingMailboxEnvelopes {
        PendingMailboxEnvelopes {
            entries: self.pending.iter().cloned().collect(),
        }
    }
    pub(crate) fn terminal_rejected_envelope(
        &self,
        envelope_id: &str,
    ) -> Option<&TerminalRejectedMailboxEnvelope> {
        self.terminal.get(envelope_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct CausalityGraph {
    predecessors: BTreeMap<String, Vec<String>>,
}

impl CausalityGraph {
    pub(crate) fn predecessor_ids(&self, occurrence_id: &str) -> Vec<String> {
        self.predecessors
            .get(occurrence_id)
            .cloned()
            .unwrap_or_default()
    }

    fn record(&mut self, occurrence_id: impl Into<String>, predecessors: Vec<String>) {
        let entry = self.predecessors.entry(occurrence_id.into()).or_default();
        for predecessor in predecessors {
            if !entry.contains(&predecessor) {
                entry.push(predecessor);
            }
        }
    }

    /// Install the complete predecessor set for one occurrence.  Snapshot
    /// recovery uses replacement rather than `record`'s monotone union so a
    /// provisional worker trace cannot retain stale M8 edges.
    fn replace(&mut self, occurrence_id: impl Into<String>, predecessors: Vec<String>) {
        self.predecessors.insert(occurrence_id.into(), predecessors);
    }

    pub(crate) fn contains_occurrence(&self, occurrence_id: &str) -> bool {
        self.predecessors.contains_key(occurrence_id)
    }

    /// Exact finite reachability used by the observer-safe SYS-5 join.  This
    /// walks only retained occurrence dependencies and never invents a
    /// relationship from operation identity or queue position.
    pub(crate) fn reaches(&self, descendant: &str, ancestor: &str) -> bool {
        let mut pending = self.predecessor_ids(descendant);
        let mut seen = BTreeSet::new();
        while let Some(current) = pending.pop() {
            if current == ancestor {
                return true;
            }
            if seen.insert(current.clone()) {
                pending.extend(self.predecessor_ids(&current));
            }
        }
        false
    }

    /// Return one retained direct predecessor only when the graph records
    /// exactly one.  Observer joins use this to retain a real mailbox enqueue
    /// occurrence rather than substituting a request identity or queue
    /// position.
    pub(crate) fn sole_predecessor(&self, occurrence_id: &str) -> Option<&str> {
        let predecessors = self.predecessors.get(occurrence_id)?;
        (predecessors.len() == 1).then(|| predecessors[0].as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActualM8TraceNode {
    node_id: String,
    kind: String,
    request_id: Option<String>,
    semantic_identity: Option<String>,
    consumer_locus: Option<String>,
    predecessors: Vec<String>,
}

impl ActualM8TraceNode {
    pub(crate) fn node_id(&self) -> &str {
        &self.node_id
    }
    pub(crate) fn semantic_identity(&self) -> &str {
        self.semantic_identity.as_deref().unwrap_or("")
    }

    pub(crate) fn consumer_locus(&self) -> &str {
        self.consumer_locus.as_deref().unwrap_or("")
    }

    pub(crate) fn kind(&self) -> M8LocalTraceKind {
        match self.kind.as_str() {
            "OwnerRequest" | "OwnerEnqueued" => M8LocalTraceKind::OwnerEnqueued,
            "OwnerAuthorityValidated" => M8LocalTraceKind::OwnerAuthorityValidated,
            "OwnerRead" => M8LocalTraceKind::OwnerRead,
            "OwnerServe" | "OwnerWrite" => M8LocalTraceKind::OwnerWrite,
            "RelationPrimaryInvalidated" => M8LocalTraceKind::RelationPrimaryInvalidated,
            "RelationOptionAdvanced" => M8LocalTraceKind::RelationOptionAdvanced,
            "RelationFallbackFrozen" => M8LocalTraceKind::RelationFallbackFrozen,
            "RelationPrimaryReturnIgnored" => M8LocalTraceKind::RelationPrimaryReturnIgnored,
            "RelationFreshLineageReacquired" => M8LocalTraceKind::RelationFreshLineageReacquired,
            "RelationPublished" => M8LocalTraceKind::RelationPublished,
            "RelationPublicationObserved" => M8LocalTraceKind::RelationPublicationObserved,
            "DesignatedAuthorityValidated" => M8LocalTraceKind::DesignatedAuthorityValidated,
            "DesignatedInputReceipt" | "DesignatedInputReceiptValidated" => {
                M8LocalTraceKind::DesignatedInputReceiptValidated
            }
            "DesignatedValueEvaluated" | "DesignatedValuePublished" => {
                M8LocalTraceKind::DesignatedValuePublished
            }
            "DesignatedPublicationImported" => M8LocalTraceKind::DesignatedPublicationImported,
            "DesignatedEvaluationIdempotent" => M8LocalTraceKind::DesignatedEvaluationIdempotent,
            "DesignatedConsumerAuthorityValidated" => {
                M8LocalTraceKind::DesignatedConsumerAuthorityValidated
            }
            "DesignatedValueConsumed" => M8LocalTraceKind::DesignatedValueConsumed,
            "DesignatedCacheValidated" => M8LocalTraceKind::DesignatedCacheValidated,
            "OwnerOperationRejected" => M8LocalTraceKind::OwnerOperationRejected,
            "OwnerEnqueueRejected" => M8LocalTraceKind::OwnerEnqueueRejected,
            "OwnerServeRejected" => M8LocalTraceKind::OwnerServeRejected,
            "DesignatedEvaluationRejected" => M8LocalTraceKind::DesignatedEvaluationRejected,
            "DesignatedConsumptionRejected" => M8LocalTraceKind::DesignatedConsumptionRejected,
            "PatchStateInitialized" => M8LocalTraceKind::PatchStateInitialized,
            "LocalCutSaved" => M8LocalTraceKind::LocalCutSaved,
            "RestoreRejected" => M8LocalTraceKind::RestoreRejected,
            "RelationTransitionRejected" => M8LocalTraceKind::RelationTransitionRejected,
            "EntityPresenceSynchronized" => M8LocalTraceKind::EntityPresenceSynchronized,
            other => panic!("unknown actual M8 trace kind: {other}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ActualM8Trace {
    nodes: Vec<ActualM8TraceNode>,
}

impl ActualM8Trace {
    pub(crate) fn owner_request_node_count(&self, operation: &str, owner_locus: &str) -> usize {
        self.nodes
            .iter()
            .filter(|node| {
                node.kind() == M8LocalTraceKind::OwnerEnqueued
                    && node.semantic_identity.as_deref() == Some(operation)
                    && node.consumer_locus.as_deref() == Some(owner_locus)
            })
            .count()
    }

    pub(crate) fn designated_evaluation_count(&self, value_name: &str) -> usize {
        self.nodes
            .iter()
            .filter(|node| {
                node.kind() == M8LocalTraceKind::DesignatedValuePublished
                    && node.semantic_identity.as_deref() == Some(value_name)
            })
            .count()
    }

    /// Return one concrete M8 occurrence only when request and trace kind
    /// identify it uniquely.  Observer joins fail closed on missing or
    /// ambiguous correspondence rather than selecting a first same-operation
    /// row.
    pub(crate) fn observer_exact_node_ref_for_request_kind(
        &self,
        request_id: &str,
        kind: M8LocalTraceKind,
    ) -> Option<&str> {
        let mut matches = self
            .nodes
            .iter()
            .filter(|node| node.request_id.as_deref() == Some(request_id) && node.kind() == kind);
        let node = matches.next()?;
        matches.next().is_none().then_some(node.node_id.as_str())
    }

    pub(crate) fn non_consuming_designated_cache_validation(
        &self,
        node_id: &str,
    ) -> Option<&ActualM8TraceNode> {
        self.nodes.iter().find(|node| {
            node.node_id == node_id && node.kind() == M8LocalTraceKind::DesignatedCacheValidated
        })
    }

    pub(crate) fn stable_digest(&self) -> String {
        format!("{:?}", self.nodes)
    }
    pub(crate) fn designated_consume_node_id(
        &self,
        semantic_identity: &str,
        consumer_locus: &str,
    ) -> Option<String> {
        self.nodes
            .iter()
            .find(|node| {
                node.kind() == M8LocalTraceKind::DesignatedValueConsumed
                    && node.semantic_identity.as_deref() == Some(semantic_identity)
                    && node.consumer_locus.as_deref() == Some(consumer_locus)
            })
            .map(|node| node.node_id.clone())
    }

    pub(crate) fn value_consumed_count(
        &self,
        semantic_identity: &str,
        consumer_locus: &str,
    ) -> usize {
        self.nodes
            .iter()
            .filter(|node| {
                node.kind() == M8LocalTraceKind::DesignatedValueConsumed
                    && node.semantic_identity.as_deref() == Some(semantic_identity)
                    && node.consumer_locus.as_deref() == Some(consumer_locus)
            })
            .count()
    }

    pub(crate) fn node(&self, node_id: &str) -> &ActualM8TraceNode {
        self.nodes
            .iter()
            .find(|node| node.node_id == node_id)
            .expect("requested M8 trace node exists")
    }

    fn append(
        &mut self,
        node_id: impl Into<String>,
        kind: impl Into<String>,
        request_id: Option<String>,
        semantic_identity: Option<String>,
        consumer_locus: Option<String>,
        predecessors: Vec<String>,
    ) {
        let node_id = node_id.into();
        // A fabric view may first learn a row from a session trace and later
        // add endpoint-specific predecessors.  It must never create a second
        // synthetic M8 node for that same M8-owned occurrence.
        if let Some(existing) = self.nodes.iter_mut().find(|node| node.node_id == node_id) {
            for predecessor in predecessors {
                if !existing.predecessors.contains(&predecessor) {
                    existing.predecessors.push(predecessor);
                }
            }
            if existing.request_id.is_none() {
                existing.request_id = request_id;
            }
            if existing.semantic_identity.is_none() {
                existing.semantic_identity = semantic_identity;
            }
            if existing.consumer_locus.is_none() {
                existing.consumer_locus = consumer_locus;
            }
            return;
        }
        self.nodes.push(ActualM8TraceNode {
            node_id,
            kind: kind.into(),
            request_id,
            semantic_identity,
            consumer_locus,
            predecessors,
        });
    }

    /// Reconcile a row learned from a complete M8 session snapshot.  Existing
    /// endpoint/request annotations remain authoritative, while the M8
    /// predecessor projection is replaced exactly by the recovered causal
    /// set.
    fn reconcile_snapshot_node(
        &mut self,
        node_id: String,
        kind: String,
        semantic_identity: Option<String>,
        consumer_locus: Option<String>,
        predecessors: Vec<String>,
    ) {
        if let Some(existing) = self.nodes.iter_mut().find(|node| node.node_id == node_id) {
            existing.predecessors = predecessors;
            if existing.semantic_identity.is_none() {
                existing.semantic_identity = semantic_identity;
            }
            if existing.consumer_locus.is_none() {
                existing.consumer_locus = consumer_locus;
            }
            return;
        }
        self.nodes.push(ActualM8TraceNode {
            node_id,
            kind,
            request_id: None,
            semantic_identity,
            consumer_locus,
            predecessors,
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeStoreRead {
    locus: String,
    state: String,
    index: String,
    field: String,
    value: i64,
}

impl RuntimeStoreRead {
    pub(crate) fn int(
        locus: impl Into<String>,
        state: impl Into<String>,
        index: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Self {
        Self {
            locus: locus.into(),
            state: state.into(),
            index: index.into(),
            field: field.into(),
            value,
        }
    }

    pub(crate) fn matches_int(
        &self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
        value: i64,
    ) -> bool {
        self.locus == locus
            && self.state == state
            && self.index == index
            && self.field == field
            && self.value == value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeStoreWrite(RuntimeStoreRead);

impl RuntimeStoreWrite {
    pub(crate) fn int(
        locus: impl Into<String>,
        state: impl Into<String>,
        index: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Self {
        Self(RuntimeStoreRead::int(locus, state, index, field, value))
    }

    pub(crate) fn matches_int(
        &self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
        value: i64,
    ) -> bool {
        self.0.matches_int(locus, state, index, field, value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnerRmwReport {
    reads: Vec<RuntimeStoreRead>,
    writes: Vec<RuntimeStoreWrite>,
    source_ref: String,
    core_ref: String,
}

impl OwnerRmwReport {
    pub(crate) fn m8_reads(&self) -> Vec<RuntimeStoreRead> {
        self.reads.clone()
    }
    pub(crate) fn m8_writes(&self) -> Vec<RuntimeStoreWrite> {
        self.writes.clone()
    }
    pub(crate) fn has_checked_source_core_provenance(&self) -> bool {
        !self.source_ref.is_empty() && !self.core_ref.is_empty()
    }
    pub(crate) fn has_exact_int_write(
        &self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
        value: i64,
    ) -> bool {
        self.writes
            .iter()
            .any(|write| write.matches_int(locus, state, index, field, value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FabricReceipt {
    request_id: String,
    delivery_id: String,
    operation_id: String,
    origin_locus: String,
    target_locus: String,
    typed_value: RuntimeValue,
    result_version: Option<ResultVersion>,
    owner_rmw: Option<OwnerRmwReport>,
    performed_m8_consumption: bool,
    returned_from_cache: bool,
    semantic_consumption_identity: Option<String>,
    fault_id: Option<String>,
    m9_cache_validation: Option<M9CacheValidationInspection>,
    m8_non_consuming_validation_node_id: Option<String>,
    m8_publication_id: Option<String>,
    logical_tick_id: Option<String>,
    logical_tick_frontier: Option<String>,
}

impl FabricReceipt {
    pub(crate) fn request_id(&self) -> &str {
        &self.request_id
    }
    pub(crate) fn delivery_id(&self) -> &str {
        &self.delivery_id
    }
    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }
    pub(crate) fn origin_locus(&self) -> &str {
        &self.origin_locus
    }
    pub(crate) fn target_locus(&self) -> &str {
        &self.target_locus
    }
    pub(crate) fn typed_value(&self) -> RuntimeValue {
        self.typed_value.clone()
    }
    pub(crate) const fn result_version(&self) -> Option<ResultVersion> {
        self.result_version
    }
    pub(crate) fn owner_rmw_report(&self) -> Option<&OwnerRmwReport> {
        self.owner_rmw.as_ref()
    }
    pub(crate) const fn performed_m8_semantic_consumption(&self) -> bool {
        self.performed_m8_consumption
    }
    pub(crate) const fn returned_from_designated_cache_after_authority_revalidation(&self) -> bool {
        self.returned_from_cache
    }
    pub(crate) fn semantic_consumption_identity(&self) -> &str {
        self.semantic_consumption_identity
            .as_deref()
            .expect("only designated consumption receipts have a semantic identity")
    }
    pub(crate) fn is_fault(&self) -> bool {
        self.fault_id.is_some()
    }
    pub(crate) fn fault_id(&self) -> &str {
        self.fault_id
            .as_deref()
            .expect("only fault receipts carry a fault identity")
    }
    pub(crate) const fn is_observer_safe(&self) -> bool {
        true
    }
    pub(crate) fn source_derived_from_edge(&self, edge_ref: &str) -> bool {
        self.operation_id == edge_ref
    }
    pub(crate) const fn exposes_raw_payload(&self) -> bool {
        false
    }
    pub(crate) fn m9_cache_validation(&self) -> Option<&M9CacheValidationInspection> {
        self.m9_cache_validation.as_ref()
    }
    pub(crate) fn m8_non_consuming_validation_node_id(&self) -> Option<&str> {
        self.m8_non_consuming_validation_node_id.as_deref()
    }
    pub(crate) fn m8_publication_id(&self) -> &str {
        self.m8_publication_id.as_deref().unwrap_or("")
    }
    pub(crate) fn logical_tick_id(&self) -> &str {
        self.logical_tick_id.as_deref().unwrap_or("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FabricSubmission {
    request_id: String,
    envelope_id: String,
    carrier_id: String,
    operation_id: String,
    origin_locus: String,
    target_locus: String,
}
impl FabricSubmission {
    pub(crate) fn request_id(&self) -> &str {
        &self.request_id
    }
    pub(crate) fn envelope_id(&self) -> &str {
        &self.envelope_id
    }
    pub(crate) fn carrier_id(&self) -> &str {
        &self.carrier_id
    }
    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }
    pub(crate) fn origin_locus(&self) -> &str {
        &self.origin_locus
    }
    pub(crate) fn target_locus(&self) -> &str {
        &self.target_locus
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TransportStep {
    envelope_id: String,
    carrier_id: String,
    source_outbox_dequeue_record_id: String,
    source_outbox_dequeue_occurrence_id: String,
    target_inbox_enqueue_record_id: String,
    target_inbox_enqueue_occurrence_id: String,
}
impl TransportStep {
    pub(crate) fn envelope_id(&self) -> &str {
        &self.envelope_id
    }
    pub(crate) fn carrier_id(&self) -> &str {
        &self.carrier_id
    }
    pub(crate) fn source_outbox_dequeue_record_id(&self) -> &str {
        &self.source_outbox_dequeue_record_id
    }
    pub(crate) fn source_outbox_dequeue_occurrence_id(&self) -> &str {
        &self.source_outbox_dequeue_occurrence_id
    }
    pub(crate) fn target_inbox_enqueue_record_id(&self) -> &str {
        &self.target_inbox_enqueue_record_id
    }
    pub(crate) fn target_inbox_enqueue_occurrence_id(&self) -> &str {
        &self.target_inbox_enqueue_occurrence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalStoreReadAudit {
    occurrence_id: String,
    reads: Vec<RuntimeStoreRead>,
}
impl LocalStoreReadAudit {
    pub(crate) fn occurrence_id(&self) -> &str {
        &self.occurrence_id
    }
    pub(crate) fn stable_digest(&self) -> String {
        format!("{}:{:?}", self.occurrence_id, self.reads)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocusM9Validation {
    Owner {
        owner_lineage_ref: String,
    },
    SourceRelease {
        inspection: M9SourceReleaseValidationInspection,
    },
    Consumer {
        inspection: M9CacheValidationInspection,
    },
    None,
}
impl LocusM9Validation {
    pub(crate) fn owner_lineage_ref(&self) -> &str {
        match self {
            Self::Owner { owner_lineage_ref } => owner_lineage_ref,
            _ => "",
        }
    }
    pub(crate) fn source_release_inspection(&self) -> &M9SourceReleaseValidationInspection {
        match self {
            Self::SourceRelease { inspection } => inspection,
            _ => panic!("locus step has no source-release inspection"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocusStep {
    consumed_envelope_id: String,
    locus_dequeue_record_id: String,
    locus_dequeue_occurrence_id: String,
    m9_validation: LocusM9Validation,
    m8_request_node_id: Option<String>,
    m8_serve_node_id: Option<String>,
    m8_input_receipt_node_id: Option<String>,
    m8_evaluation_node_id: Option<String>,
    m8_non_consuming_validation_node_id: Option<String>,
    reply_envelope_id: Option<String>,
    local_store_read_audit: Option<LocalStoreReadAudit>,
    local_store_reads: Vec<RuntimeStoreRead>,
    receipt: Option<FabricReceipt>,
    request_id: String,
    semantic_identity: Option<String>,
    m9_cache_validation: Option<M9CacheValidationInspection>,
}
impl LocusStep {
    pub(crate) fn consumed_envelope_id(&self) -> &str {
        &self.consumed_envelope_id
    }
    pub(crate) fn locus_dequeue_record_id(&self) -> &str {
        &self.locus_dequeue_record_id
    }
    pub(crate) fn locus_dequeue_occurrence_id(&self) -> &str {
        &self.locus_dequeue_occurrence_id
    }
    pub(crate) fn m9_validation(&self) -> &LocusM9Validation {
        &self.m9_validation
    }
    pub(crate) fn m8_request_node_id(&self) -> &str {
        self.m8_request_node_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m8_serve_node_id(&self) -> &str {
        self.m8_serve_node_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m8_input_receipt_node_id(&self) -> &str {
        self.m8_input_receipt_node_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m8_evaluation_node_id(&self) -> &str {
        self.m8_evaluation_node_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m8_consume_node_id(&self) -> &str {
        self.m8_request_node_id.as_deref().unwrap_or("")
    }
    pub(crate) fn m8_non_consuming_validation_node_id(&self) -> Option<&str> {
        self.m8_non_consuming_validation_node_id.as_deref()
    }
    pub(crate) fn reply_envelope_id(&self) -> &str {
        self.reply_envelope_id.as_deref().unwrap_or("")
    }
    pub(crate) fn local_store_read_audit(&self) -> Option<&LocalStoreReadAudit> {
        self.local_store_read_audit.as_ref()
    }
    pub(crate) fn local_store_reads(&self) -> Vec<RuntimeStoreRead> {
        self.local_store_reads.clone()
    }
    pub(crate) fn receipt(&self) -> Option<&FabricReceipt> {
        self.receipt.as_ref()
    }
    pub(crate) fn request_id(&self) -> &str {
        &self.request_id
    }
    pub(crate) fn semantic_identity(&self) -> &str {
        self.semantic_identity.as_deref().unwrap_or("")
    }
    pub(crate) fn m9_cache_validation(&self) -> Option<&M9CacheValidationInspection> {
        self.m9_cache_validation.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sys4TraceKind {
    RequestAdmitted,
    Dispatched,
    Received,
    Served,
    M8OwnerRead,
    M8OwnerWrite,
    ReplyDispatched,
    ReplyReceived,
    DesignatedResultPublished,
    DesignatedResultDispatched,
    DesignatedResultReceived,
    DesignatedResultConsumed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4TraceEntry {
    request_id: String,
    delivery_id: Option<String>,
    operation: String,
    kind: Sys4TraceKind,
    edge_kind: Option<CommunicationEdgeKind>,
    edge_ref: Option<String>,
    source_ref: Option<SourceRefView>,
    core_ref: Option<String>,
    source_fragment_ref: Option<String>,
    target_fragment_ref: Option<String>,
    endpoint_carrier_id: Option<String>,
    endpoint_record_id: Option<String>,
    endpoint_occurrence_id: Option<String>,
    source_locus: Option<String>,
    target_locus: Option<String>,
    semantic_identity: Option<String>,
    consumer_locus: Option<String>,
    fault_id: Option<String>,
    observer_safe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FabricTrace {
    entries: Vec<Sys4TraceEntry>,
}

impl FabricTrace {
    fn append(
        &mut self,
        request_id: impl Into<String>,
        delivery_id: Option<String>,
        operation: impl Into<String>,
        kind: Sys4TraceKind,
        edge_kind: Option<CommunicationEdgeKind>,
    ) {
        self.entries.push(Sys4TraceEntry {
            request_id: request_id.into(),
            delivery_id,
            operation: operation.into(),
            kind,
            edge_kind,
            edge_ref: None,
            source_ref: None,
            core_ref: None,
            source_fragment_ref: None,
            target_fragment_ref: None,
            endpoint_carrier_id: None,
            endpoint_record_id: None,
            endpoint_occurrence_id: None,
            source_locus: None,
            target_locus: None,
            semantic_identity: None,
            consumer_locus: None,
            fault_id: None,
            observer_safe: true,
        });
    }

    fn append_endpoint(
        &mut self,
        request_id: impl Into<String>,
        operation: impl Into<String>,
        kind: Sys4TraceKind,
        record: &EndpointCarrierRecord,
        occurrence_id: impl Into<String>,
    ) {
        self.entries.push(Sys4TraceEntry {
            request_id: request_id.into(),
            delivery_id: None,
            operation: operation.into(),
            kind,
            edge_kind: Some(record.edge_kind),
            edge_ref: Some(record.edge_ref.clone()),
            source_ref: Some(record.source_ref.clone()),
            core_ref: record.core_ref.clone(),
            source_fragment_ref: Some(record.source_fragment_ref.clone()),
            target_fragment_ref: Some(record.target_fragment_ref.clone()),
            endpoint_carrier_id: Some(record.carrier_id.clone()),
            endpoint_record_id: Some(record.record_id.clone()),
            endpoint_occurrence_id: Some(occurrence_id.into()),
            source_locus: Some(record.source_locus.clone()),
            target_locus: Some(record.target_locus.clone()),
            semantic_identity: None,
            consumer_locus: None,
            fault_id: None,
            observer_safe: true,
        });
    }

    fn append_designated_delivery_endpoint(
        &mut self,
        request_id: impl Into<String>,
        delivery_id: impl Into<String>,
        operation: impl Into<String>,
        kind: Sys4TraceKind,
        record: &EndpointCarrierRecord,
        occurrence_id: impl Into<String>,
    ) {
        self.append_endpoint(request_id, operation, kind, record, occurrence_id);
        self.entries
            .last_mut()
            .expect("endpoint append retains a trace entry")
            .delivery_id = Some(delivery_id.into());
    }

    fn append_fault(&mut self, fault_id: impl Into<String>, operation: impl Into<String>) {
        let fault_id = fault_id.into();
        self.entries.push(Sys4TraceEntry {
            request_id: fault_id.clone(),
            delivery_id: None,
            operation: operation.into(),
            kind: Sys4TraceKind::RequestAdmitted,
            edge_kind: None,
            edge_ref: None,
            source_ref: None,
            core_ref: None,
            source_fragment_ref: None,
            target_fragment_ref: None,
            endpoint_carrier_id: None,
            endpoint_record_id: None,
            endpoint_occurrence_id: None,
            source_locus: None,
            target_locus: None,
            semantic_identity: None,
            consumer_locus: None,
            fault_id: Some(fault_id),
            observer_safe: true,
        });
    }

    fn append_actual_m8_consumption(
        &mut self,
        request_id: impl Into<String>,
        delivery_id: impl Into<String>,
        operation: impl Into<String>,
        semantic_identity: impl Into<String>,
        consumer_locus: impl Into<String>,
    ) {
        self.entries.push(Sys4TraceEntry {
            request_id: request_id.into(),
            delivery_id: Some(delivery_id.into()),
            operation: operation.into(),
            kind: Sys4TraceKind::DesignatedResultConsumed,
            edge_kind: Some(CommunicationEdgeKind::DesignatedResultDelivery),
            edge_ref: None,
            source_ref: None,
            core_ref: None,
            source_fragment_ref: None,
            target_fragment_ref: None,
            endpoint_carrier_id: None,
            endpoint_record_id: None,
            endpoint_occurrence_id: None,
            source_locus: None,
            target_locus: None,
            semantic_identity: Some(semantic_identity.into()),
            consumer_locus: Some(consumer_locus.into()),
            fault_id: None,
            observer_safe: true,
        });
    }

    pub(crate) fn for_request(&self, request_id: &str) -> FabricTraceView {
        FabricTraceView {
            entries: self
                .entries
                .iter()
                .filter(|entry| entry.request_id == request_id)
                .cloned()
                .collect(),
        }
    }

    pub(crate) fn for_designated_delivery(
        &self,
        operation: &str,
        delivery_id: &str,
    ) -> FabricTraceView {
        FabricTraceView {
            entries: self
                .entries
                .iter()
                .filter(|entry| {
                    entry.operation == operation
                        && entry.delivery_id.as_deref() == Some(delivery_id)
                })
                .cloned()
                .collect(),
        }
    }

    pub(crate) fn canonical_correspondence_excluding_debug_worker_tokens(
        &self,
    ) -> Vec<(String, Sys4TraceKind, Option<CommunicationEdgeKind>)> {
        self.entries
            .iter()
            .map(|entry| (entry.operation.clone(), entry.kind, entry.edge_kind))
            .collect()
    }

    pub(crate) fn endpoint_row_for_carrier(
        &self,
        carrier_id: &str,
        kind: Sys4TraceKind,
        record_id: &str,
        source_locus: &str,
        target_locus: &str,
    ) -> &Sys4TraceEntry {
        self.entries
            .iter()
            .find(|entry| {
                entry.endpoint_carrier_id.as_deref() == Some(carrier_id)
                    && entry.kind == kind
                    && entry.endpoint_record_id.as_deref() == Some(record_id)
                    && entry.source_locus.as_deref() == Some(source_locus)
                    && entry.target_locus.as_deref() == Some(target_locus)
            })
            .expect("endpoint trace row is derived from the concrete endpoint record")
    }

    pub(crate) fn for_fault(&self, fault_id: &str) -> FabricTraceView {
        FabricTraceView {
            entries: self
                .entries
                .iter()
                .filter(|entry| entry.fault_id.as_deref() == Some(fault_id))
                .cloned()
                .collect(),
        }
    }
}

impl Sys4TraceEntry {
    pub(crate) fn edge_ref(&self) -> &str {
        self.edge_ref.as_deref().unwrap_or("")
    }
    pub(crate) fn source_ref(&self) -> SourceRefView {
        self.source_ref
            .clone()
            .expect("endpoint trace rows retain checked source provenance")
    }
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }
    pub(crate) fn source_fragment_ref(&self) -> &str {
        self.source_fragment_ref.as_deref().unwrap_or("")
    }
    pub(crate) fn target_fragment_ref(&self) -> &str {
        self.target_fragment_ref.as_deref().unwrap_or("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FabricTraceView {
    entries: Vec<Sys4TraceEntry>,
}

/// A redacted, concrete endpoint occurrence retained by SYS-4.  It contains
/// only checked provenance and occurrence identifiers; carrier payloads,
/// local store values, and M9 material are deliberately absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4ObserverTraceSegment {
    occurrence_ref: String,
    source_ref: SourceRefView,
    core_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    edge_ref: String,
}

impl Sys4ObserverTraceSegment {
    pub(crate) fn occurrence_ref(&self) -> &str {
        &self.occurrence_ref
    }
    pub(crate) fn source_ref(&self) -> &SourceRefView {
        &self.source_ref
    }
    pub(crate) fn core_ref(&self) -> &str {
        &self.core_ref
    }
    pub(crate) fn source_fragment_ref(&self) -> &str {
        &self.source_fragment_ref
    }
    pub(crate) fn target_fragment_ref(&self) -> &str {
        &self.target_fragment_ref
    }
    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }
}

/// A request-scoped pair of exact generated endpoint rows.  Its occurrence
/// identifiers are all retained runtime occurrences; the request identity is
/// deliberately not an occurrence.  Construction checks the complete checked
/// provenance copied through dispatch and receive, so a consumer cannot join
/// a source row to an unrelated target row by operation name alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4ObserverEndpointOccurrences {
    request_enqueue_occurrence_id: String,
    dispatch_occurrence_id: String,
    receive_occurrence_id: String,
    source_ref: SourceRefView,
    core_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    edge_ref: String,
}

impl Sys4ObserverEndpointOccurrences {
    pub(crate) fn request_enqueue_occurrence_id(&self) -> &str {
        &self.request_enqueue_occurrence_id
    }
    pub(crate) fn dispatch_occurrence_id(&self) -> &str {
        &self.dispatch_occurrence_id
    }
    pub(crate) fn receive_occurrence_id(&self) -> &str {
        &self.receive_occurrence_id
    }
    pub(crate) fn source_ref(&self) -> &SourceRefView {
        &self.source_ref
    }
    pub(crate) fn core_ref(&self) -> &str {
        &self.core_ref
    }
    pub(crate) fn source_fragment_ref(&self) -> &str {
        &self.source_fragment_ref
    }
    pub(crate) fn target_fragment_ref(&self) -> &str {
        &self.target_fragment_ref
    }
    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }
}

impl FabricTraceView {
    pub(crate) fn kinds(&self) -> Vec<Sys4TraceKind> {
        self.entries.iter().map(|entry| entry.kind).collect()
    }
    pub(crate) fn contains_edge_kind(&self, kind: CommunicationEdgeKind) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.edge_kind == Some(kind))
    }
    pub(crate) fn m8_value_consumed_count_for(&self, identity: &str, consumer: &str) -> usize {
        self.entries
            .iter()
            .filter(|entry| {
                entry.kind == Sys4TraceKind::DesignatedResultConsumed
                    && entry.semantic_identity.as_deref() == Some(identity)
                    && entry.consumer_locus.as_deref() == Some(consumer)
            })
            .count()
    }

    pub(crate) fn all_entries_observer_safe(&self) -> bool {
        self.entries.iter().all(|entry| entry.observer_safe)
    }

    pub(crate) const fn target_locus_override(&self) -> Option<&str> {
        None
    }
}

impl FabricTrace {
    /// Find a single exact endpoint row.  All selection keys are caller
    /// independent facts from one completed request; missing provenance and
    /// ambiguity are represented by `None` so higher layers must fail closed.
    fn observer_exact_endpoint_segment(
        &self,
        request_id: &str,
        kind: Sys4TraceKind,
        edge_kind: CommunicationEdgeKind,
        source_locus: &str,
        target_locus: &str,
    ) -> Option<Sys4ObserverTraceSegment> {
        let mut matches = self.entries.iter().filter(|entry| {
            entry.request_id == request_id
                && entry.kind == kind
                && entry.edge_kind == Some(edge_kind)
                && entry.source_locus.as_deref() == Some(source_locus)
                && entry.target_locus.as_deref() == Some(target_locus)
        });
        let entry = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        Some(Sys4ObserverTraceSegment {
            occurrence_ref: entry.endpoint_occurrence_id.clone()?,
            source_ref: entry.source_ref.clone()?,
            core_ref: entry.core_ref.clone()?,
            source_fragment_ref: entry.source_fragment_ref.clone()?,
            target_fragment_ref: entry.target_fragment_ref.clone()?,
            edge_ref: entry.edge_ref.clone()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FabricM8Trace {
    consumption_counts: BTreeMap<(String, String), usize>,
}

impl FabricM8Trace {
    fn record_actual_consumption(&mut self, identity: &str, consumer: &str) {
        *self
            .consumption_counts
            .entry((identity.to_string(), consumer.to_string()))
            .or_default() += 1;
    }

    pub(crate) fn value_consumed_count(&self, identity: &str, consumer: &str) -> usize {
        self.consumption_counts
            .get(&(identity.to_string(), consumer.to_string()))
            .copied()
            .unwrap_or_default()
    }

    pub(crate) fn new_entries_since(&self, prior: &Self) -> Self {
        let mut counts = BTreeMap::new();
        for (key, count) in &self.consumption_counts {
            let prior_count = prior
                .consumption_counts
                .get(key)
                .copied()
                .unwrap_or_default();
            counts.insert(key.clone(), count.saturating_sub(prior_count));
        }
        Self {
            consumption_counts: counts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct DesignatedConsumptionState {
    counts: BTreeMap<(String, String), usize>,
}

impl DesignatedConsumptionState {
    pub(crate) fn semantic_consumption_count(&self, identity: &str, consumer: &str) -> usize {
        self.counts
            .get(&(identity.to_string(), consumer.to_string()))
            .copied()
            .unwrap_or_default()
    }
}

/// SYS-4's bounded local-cut receipt evidence.  It records only sealed
/// carrier identities, never an M8 payload or authority fact; concrete M8
/// state remains in the per-locus `M8LocalCut` below.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ImportedDesignatedPublicationState {
    entries: BTreeSet<(String, String, String)>,
}

impl ImportedDesignatedPublicationState {
    pub(crate) fn contains_exact(
        &self,
        semantic_identity: &str,
        publication_id: &str,
        sealed_delivery_digest: &str,
    ) -> bool {
        self.entries.contains(&(
            semantic_identity.to_string(),
            publication_id.to_string(),
            sealed_delivery_digest.to_string(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct DesignatedReceiptState {
    entries: BTreeSet<(String, String, String)>,
}

impl DesignatedReceiptState {
    pub(crate) fn contains_exact_receipt(
        &self,
        semantic_identity: &str,
        publication_id: &str,
        logical_tick_id: &str,
    ) -> bool {
        self.entries.contains(&(
            semantic_identity.to_string(),
            publication_id.to_string(),
            logical_tick_id.to_string(),
        ))
    }
}

/// Mutable state of one generated locus endpoint.  The checked artifact and
/// program identity are deliberately *not* copied into a cut; restore binds
/// these fields back to the supplied checked projection.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LocusRuntimeCut {
    locus: String,
    local_store: LocusLocalStore,
    incoming_endpoint: EndpointCarrierHistory,
    outgoing_endpoint: EndpointCarrierHistory,
    incoming_mailbox: IncomingMailbox,
    outgoing_mailbox: OutgoingMailbox,
}

impl LocusRuntimeCut {
    fn capture(runtime: &LocusRuntime) -> Self {
        Self {
            locus: runtime.locus.clone(),
            local_store: runtime.local_store.clone(),
            incoming_endpoint: runtime.incoming_endpoint.clone(),
            outgoing_endpoint: runtime.outgoing_endpoint.clone(),
            incoming_mailbox: runtime.incoming_mailbox.clone(),
            outgoing_mailbox: runtime.outgoing_mailbox.clone(),
        }
    }

    fn restore_into(&self, runtime: &mut LocusRuntime) {
        runtime.local_store = self.local_store.clone();
        runtime.incoming_endpoint = self.incoming_endpoint.clone();
        runtime.outgoing_endpoint = self.outgoing_endpoint.clone();
        runtime.incoming_mailbox = self.incoming_mailbox.clone();
        runtime.outgoing_mailbox = self.outgoing_mailbox.clone();
    }
}

/// A whole-fabric, process-local cut.  This internal bounded representation
/// retains actual locus endpoints/mailboxes, fabric traces, M8 session cuts,
/// and the M9 successor lifecycle needed to continue exactly from the cut.
/// It is neither a durable save format nor a public transport/wire contract.
#[derive(Clone)]
pub(crate) struct Sys4LocalCut {
    cut_id: String,
    program_identity: CheckedProgramIdentity,
    program_fingerprint: BTreeSet<(FabricRouteKey, String)>,
    backend_profile: BackendProfile,
    loci: BTreeMap<String, LocusRuntimeCut>,
    m8_cuts: BTreeMap<String, M8LocalCut>,
    authority_generation: M9AuthorityGeneration,
    authority_lifecycle: M9AuthorityLifecycle,
    authority_live_floor: M9AuthorityLiveFloor,
    trace: FabricTrace,
    route_faults: BTreeSet<String>,
    in_transit_faults: InTransitFaults,
    completed_receipts: BTreeMap<String, FabricReceipt>,
    local_store_read_audits: BTreeMap<String, LocalStoreReadAudit>,
    cache: BTreeMap<String, CachedDelivery>,
    /// Observer-safe digests of relation states delivered through generated
    /// relation-publication endpoints.  The M8 cuts retain the semantic
    /// source; this map is the local fabric's read-only devtools index.
    relation_semantic_digests: BTreeMap<String, String>,
    /// One-shot finite M9 bindings already consumed by an accepted fresh
    /// relation reacquisition.  Retaining this set prevents restore from
    /// reactivating a dormant binding a second time.
    used_fresh_relation_bindings: BTreeSet<String>,
    consumption_state: DesignatedConsumptionState,
    evaluator_publication_bindings: EvaluatorPublicationBindingRegistry,
    imported_designated_publication_state: ImportedDesignatedPublicationState,
    designated_receipt_state: DesignatedReceiptState,
    m8_trace: FabricM8Trace,
    actual_m8_trace: ActualM8Trace,
    m8_local_runtime_trace: M8LocalTrace,
    m8_trace_offsets: BTreeMap<String, usize>,
    m8_qualified_trace_nodes: BTreeMap<String, BTreeMap<String, String>>,
    m8_qualified_trace_dependencies: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    m8_raw_node_loci: BTreeMap<String, BTreeMap<String, String>>,
    m8_locus_trace_sequences: BTreeMap<String, u64>,
    m8_locus_sessions: BTreeMap<String, String>,
    observer_snapshot_failures:
        BTreeMap<(String, ObserverSnapshotChannel), ObserverSnapshotFailure>,
    causality: CausalityGraph,
    next_endpoint_occurrence: u64,
    next_request: u64,
    patch_generation: u64,
    patch_lifecycle: Sys4PatchLifecycleLog,
    patch_lifecycle_snapshot: Sys4PatchLifecycleSnapshot,
    active_patch_frontier: Sys4PatchFrontier,
    /// Private seal over every field restored into the fresh fabric. This is
    /// intentionally not a public/local-observer projection or wire format.
    private_restore_integrity_digest: String,
}

impl std::fmt::Debug for Sys4LocalCut {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys4LocalCut")
            .field("cut_id", &self.cut_id)
            .field("program_identity", &self.program_identity)
            .field("backend_profile", &self.backend_profile)
            .field("loci", &self.loci.keys().collect::<Vec<_>>())
            .field(
                "authority_generation",
                &self.authority_generation.generation(),
            )
            .finish_non_exhaustive()
    }
}

impl Sys4LocalCut {
    /// Opaque SYS-4 local-cut seal. It is a private value: callers can bind
    /// the digest but cannot inspect its M8/M9, store, mailbox, or trace
    /// material.
    pub(crate) fn private_restore_integrity_digest(&self) -> &str {
        &self.private_restore_integrity_digest
    }

    /// Recompute the exact private seal before a fresh fabric is constructed.
    /// A poisoned M9 floor also fails closed rather than substituting a
    /// process identity or an incomplete observer projection.
    pub(crate) fn has_valid_private_restore_integrity(&self) -> bool {
        self.compute_private_restore_integrity_digest()
            .is_some_and(|digest| digest == self.private_restore_integrity_digest)
    }

    /// Crate-private observer-safe material for a SYS-5 wrapper. The only
    /// returned value is the opaque private seal; raw M9 credential,
    /// capability, witness, owner-store, M8, queue, endpoint, or trace data
    /// never enters the observer material.
    pub(crate) fn observer_safe_integrity_material(&self) -> String {
        self.private_restore_integrity_digest().to_string()
    }

    /// Deterministically bind every `Sys4LocalCut` field that restore can
    /// install or use for later continuation. The inputs remain private to
    /// this function; only the SHA-256 digest above may leave SYS-4.
    fn compute_private_restore_integrity_digest(&self) -> Option<String> {
        let authority_live_floor = self
            .authority_live_floor
            .private_restore_integrity_digest()?;
        let components = vec![
            ("cut_id", self.cut_id.clone()),
            ("program_identity", format!("{:?}", self.program_identity)),
            (
                "program_fingerprint",
                format!("{:?}", self.program_fingerprint),
            ),
            ("backend_profile", format!("{:?}", self.backend_profile)),
            ("loci", format!("{:?}", self.loci)),
            ("m8_cuts", format!("{:?}", self.m8_cuts)),
            (
                "authority_generation",
                self.authority_generation.private_restore_integrity_digest(),
            ),
            (
                "authority_lifecycle",
                self.authority_lifecycle.private_restore_integrity_digest(),
            ),
            ("authority_live_floor", authority_live_floor),
            ("trace", format!("{:?}", self.trace)),
            ("route_faults", format!("{:?}", self.route_faults)),
            ("in_transit_faults", format!("{:?}", self.in_transit_faults)),
            (
                "completed_receipts",
                format!("{:?}", self.completed_receipts),
            ),
            (
                "local_store_read_audits",
                format!("{:?}", self.local_store_read_audits),
            ),
            ("cache", format!("{:?}", self.cache)),
            (
                "relation_semantic_digests",
                format!("{:?}", self.relation_semantic_digests),
            ),
            (
                "used_fresh_relation_bindings",
                format!("{:?}", self.used_fresh_relation_bindings),
            ),
            ("consumption_state", format!("{:?}", self.consumption_state)),
            (
                "evaluator_publication_bindings",
                format!("{:?}", self.evaluator_publication_bindings),
            ),
            (
                "imported_designated_publication_state",
                format!("{:?}", self.imported_designated_publication_state),
            ),
            (
                "designated_receipt_state",
                format!("{:?}", self.designated_receipt_state),
            ),
            ("m8_trace", format!("{:?}", self.m8_trace)),
            ("actual_m8_trace", format!("{:?}", self.actual_m8_trace)),
            (
                "m8_local_runtime_trace",
                format!("{:?}", self.m8_local_runtime_trace),
            ),
            ("m8_trace_offsets", format!("{:?}", self.m8_trace_offsets)),
            (
                "m8_qualified_trace_nodes",
                format!("{:?}", self.m8_qualified_trace_nodes),
            ),
            (
                "m8_qualified_trace_dependencies",
                format!("{:?}", self.m8_qualified_trace_dependencies),
            ),
            ("m8_raw_node_loci", format!("{:?}", self.m8_raw_node_loci)),
            (
                "m8_locus_trace_sequences",
                format!("{:?}", self.m8_locus_trace_sequences),
            ),
            ("m8_locus_sessions", format!("{:?}", self.m8_locus_sessions)),
            (
                "observer_snapshot_failures",
                format!("{:?}", self.observer_snapshot_failures),
            ),
            ("causality", format!("{:?}", self.causality)),
            (
                "next_endpoint_occurrence",
                self.next_endpoint_occurrence.to_string(),
            ),
            ("next_request", self.next_request.to_string()),
            ("patch_generation", self.patch_generation.to_string()),
            ("patch_lifecycle", format!("{:?}", self.patch_lifecycle)),
            (
                "patch_lifecycle_snapshot",
                format!("{:?}", self.patch_lifecycle_snapshot),
            ),
            (
                "active_patch_frontier",
                format!("{:?}", self.active_patch_frontier),
            ),
        ];
        Some(sys4_private_restore_integrity_digest(&components))
    }

    pub(crate) fn patch_lifecycle_snapshot(&self) -> &Sys4PatchLifecycleSnapshot {
        &self.patch_lifecycle_snapshot
    }

    pub(crate) fn active_patch_frontier_snapshot(&self) -> &Sys4PatchFrontier {
        &self.active_patch_frontier
    }

    fn patch_frontier_lifecycle_generation_is_consistent(&self) -> bool {
        self.patch_lifecycle
            .rows
            .iter()
            .filter(|row| matches!(row, Sys4PatchLifecycleRow::Accepted))
            .count()
            == self.patch_generation as usize
            && self.patch_lifecycle_snapshot.rows == self.patch_lifecycle.rows
            && self.active_patch_frontier.activation_generation == self.patch_generation
    }

    #[cfg(test)]
    pub(crate) fn for_test_rewind_patch_generation_below_lifecycle_frontier(&mut self) {
        self.patch_generation = self.patch_generation.saturating_sub(1);
    }

    #[cfg(test)]
    pub(crate) fn patch_frontier_lifecycle_generation_is_inconsistent(&self) -> bool {
        !self.patch_frontier_lifecycle_generation_is_consistent()
    }

    pub(crate) fn imported_designated_publication_state(
        &self,
    ) -> &ImportedDesignatedPublicationState {
        &self.imported_designated_publication_state
    }

    pub(crate) fn designated_receipt_state(&self) -> &DesignatedReceiptState {
        &self.designated_receipt_state
    }

    /// Test-only malformed-cut constructor.  It removes one concrete source
    /// endpoint send record while retaining the target receive, so restore
    /// must reject the incomplete generated carrier lifecycle.
    #[cfg(test)]
    pub(crate) fn for_test_drop_outgoing_endpoint_record(
        &mut self,
        locus: &str,
        request_id: &str,
        carrier_id: &str,
    ) {
        if let Some(locus_cut) = self.loci.get_mut(locus) {
            locus_cut.outgoing_endpoint.records.retain(|record| {
                record.request_id != request_id || record.carrier_id != carrier_id
            });
        }
    }

    /// Test-only malformed-cut constructor for the target half of one moved
    /// carrier.  A pending inbox envelope without this exact receive record
    /// must never restore, even when a matching source send remains present.
    #[cfg(test)]
    pub(crate) fn for_test_drop_incoming_endpoint_record(
        &mut self,
        locus: &str,
        request_id: &str,
        carrier_id: &str,
        record_id: &str,
    ) {
        if let Some(locus_cut) = self.loci.get_mut(locus) {
            locus_cut.incoming_endpoint.records.retain(|record| {
                record.request_id != request_id
                    || record.carrier_id != carrier_id
                    || record.record_id != record_id
            });
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_duplicate_pending_inbox_envelope_and_receive_record(
        &mut self,
        locus: &str,
        request_id: &str,
        carrier_id: &str,
        record_id: &str,
    ) {
        let Some(locus_cut) = self.loci.get_mut(locus) else {
            return;
        };
        if let Some(envelope) = locus_cut
            .incoming_mailbox
            .pending
            .iter()
            .find(|envelope| {
                envelope.request_id == request_id
                    && envelope.carrier_id == carrier_id
                    && envelope.mailbox_record_id == record_id
            })
            .cloned()
        {
            locus_cut.incoming_mailbox.pending.push_back(envelope);
        }
        if let Some(record) = locus_cut
            .incoming_endpoint
            .records
            .iter()
            .find(|record| {
                record.request_id == request_id
                    && record.carrier_id == carrier_id
                    && record.record_id == record_id
            })
            .cloned()
        {
            locus_cut.incoming_endpoint.records.push(record);
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_set_next_request_below_retained_max(&mut self, request_id: &str) {
        self.next_request = request_id
            .strip_prefix("sys4-request-")
            .or_else(|| request_id.strip_prefix("sys5-relation-request:"))
            .and_then(|suffix| suffix.parse().ok())
            .unwrap_or_default();
    }

    /// Test-only corrupt-cut seam for the observer-safe relation digest index.
    /// Restore must reject any nonempty value that is not re-derived from the
    /// target M8 imported shadow.
    #[cfg(test)]
    pub(crate) fn for_test_set_relation_semantic_digest(
        &mut self,
        relation: &str,
        digest: impl Into<String>,
    ) {
        self.relation_semantic_digests
            .insert(relation.to_string(), digest.into());
    }

    /// Test-only bounded corruption seam. It alters one owner-local value
    /// inside the retained SYS-4 cut without exposing the store through any
    /// production or observer API; the saved private seal is deliberately
    /// not recomputed.
    #[cfg(test)]
    pub(crate) fn for_test_tamper_owner_state_value(
        &mut self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
        value: i64,
    ) {
        if let Some(locus_cut) = self.loci.get_mut(locus) {
            locus_cut.local_store.set_int(state, index, field, value);
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_set_next_endpoint_occurrence_below_retained_max(
        &mut self,
        identifier: &str,
    ) {
        self.next_endpoint_occurrence = sys4_counter_suffix(identifier).unwrap_or_default();
    }

    #[cfg(test)]
    pub(crate) fn for_test_drop_actual_m8_trace_row(&mut self, node_id: &str) {
        self.actual_m8_trace
            .nodes
            .retain(|node| node.node_id != node_id);
    }

    /// Add a predecessor that is a real fabric occurrence but not one of the
    /// saved causal predecessors for this M8 node. Restore must reject the
    /// copied ActualM8 view rather than accepting a merely well-formed ID.
    #[cfg(test)]
    pub(crate) fn for_test_add_actual_m8_trace_predecessor(
        &mut self,
        node_id: &str,
        forged_predecessor_id: &str,
    ) {
        if let Some(node) = self
            .actual_m8_trace
            .nodes
            .iter_mut()
            .find(|node| node.node_id == node_id)
        {
            node.predecessors.push(forged_predecessor_id.to_string());
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_drop_causality_row(&mut self, occurrence_id: &str) {
        self.causality.predecessors.remove(occurrence_id);
    }
}

fn cut_projection_mismatch() -> Sys4DispatchDiagnostics {
    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramProjectionMismatch)
}

/// Internal whole-cut seal. Every field is tagged and length-delimited before
/// hashing; this is private integrity material, not an observer export.
fn sys4_private_restore_integrity_digest(components: &[(&str, String)]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys4/local-cut-restore-integrity/v1\0");
    for (name, value) in components {
        for component in [*name, value.as_str()] {
            hasher.update(
                u64::try_from(component.len())
                    .expect("SYS-4 local-cut integrity component length fits u64")
                    .to_le_bytes(),
            );
            hasher.update(component.as_bytes());
        }
    }
    format!("sys4-local-cut-restore-sha256-v1:{:x}", hasher.finalize())
}

fn validate_sys4_local_cut(
    program: &FabricProgram,
    backend_profile: BackendProfile,
    cut: &Sys4LocalCut,
) -> Sys4Result<()> {
    if cut.program_identity != *program.checked_program_identity()
        || cut.program_fingerprint != program.projected_fingerprint()
        || cut.backend_profile != backend_profile
    {
        return Err(cut_projection_mismatch());
    }
    let expected_loci: BTreeSet<_> = program.locus_names().into_iter().collect();
    let relation_publications: BTreeSet<_> = program
        .projection
        .sys4_artifact_fragments()
        .entries()
        .iter()
        .filter(|fragment| {
            fragment.fragment_kind() == ProjectedOperationFragmentKind::RelationPublication
        })
        .map(|fragment| fragment.operation_id().to_string())
        .collect();
    if cut.loci.keys().cloned().collect::<BTreeSet<_>>() != expected_loci
        || cut.m8_cuts.keys().cloned().collect::<BTreeSet<_>>() != expected_loci
        || cut
            .local_store_read_audits
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            != expected_loci
    {
        return Err(cut_projection_mismatch());
    }
    if cut
        .relation_semantic_digests
        .iter()
        .any(|(relation, digest)| !relation_publications.contains(relation) || digest.is_empty())
        || cut.used_fresh_relation_bindings.iter().any(|relation| {
            !relation_publications.contains(relation)
                || cut
                    .authority_generation
                    .fresh_relation_reacquire_binding(relation)
                    .is_none()
        })
    {
        return Err(cut_projection_mismatch());
    }
    let expected_relation_digests = program
        .projection
        .communication_plan()
        .edges()
        .iter()
        .filter(|edge| edge.kind() == CommunicationEdgeKind::RelationProjectionPublication)
        .filter_map(|edge| {
            cut.m8_cuts
                .get(edge.target_locus())
                .and_then(|m8_cut| {
                    m8_cut.relation_observed_shadow(edge.operation_id(), edge.target_locus())
                })
                .map(|shadow| (edge.operation_id().to_string(), shadow.semantic_digest()))
        })
        .collect::<BTreeMap<_, _>>();
    if cut.relation_semantic_digests != expected_relation_digests {
        return Err(cut_projection_mismatch());
    }
    if !cut
        .authority_lifecycle
        .matches_generation_for_restore(&cut.authority_generation)
        || !cut
            .authority_live_floor
            .matches_generation(&cut.authority_generation)
        || cut.m8_cuts.values().any(|m8_cut| {
            m8_cut.program_identity() != program.checked_program_identity()
                || m8_cut.authority_inventory() != &cut.authority_generation.authority_state()
        })
    {
        return Err(Sys4DispatchDiagnostics::one(
            Sys4DiagnosticKind::ProgramAdmissionMismatch,
        ));
    }
    // Retain the pre-existing M9 stale-generation diagnostic precedence, but
    // validate the private whole-cut seal before any fresh fabric is built or
    // any other restored state can be installed.
    if !cut.has_valid_private_restore_integrity() {
        return Err(cut_projection_mismatch());
    }
    let expected_patch_frontier =
        Sys4PatchFrontier::for_active(program, &cut.authority_generation, cut.patch_generation);
    if !cut.patch_frontier_lifecycle_generation_is_consistent()
        || cut.active_patch_frontier != expected_patch_frontier
    {
        return Err(cut_projection_mismatch());
    }

    for (locus, locus_cut) in &cut.loci {
        if locus_cut.locus != *locus || !locus_cut.local_store.is_owned_by_locus(locus) {
            return Err(cut_projection_mismatch());
        }
        validate_locus_runtime_cut(program, locus, locus_cut, &cut.causality)?;
    }

    let sent_records: Vec<_> = cut
        .loci
        .values()
        .flat_map(|locus_cut| locus_cut.outgoing_endpoint.records.iter())
        .collect();
    if cut.loci.values().any(|locus_cut| {
        locus_cut.incoming_endpoint.records.iter().any(|received| {
            !sent_records
                .iter()
                .any(|sent| endpoint_transfer_pair_matches(sent, received))
        })
    }) {
        return Err(cut_projection_mismatch());
    }
    // Endpoint history is symmetric with a live inbox: every pending target
    // envelope must retain its exact receive record and that record must in
    // turn retain the matching source send. A receive-only history check
    // would allow a corrupt cut to preserve the carrier payload while
    // erasing the target endpoint occurrence that made it admissible.
    if cut.loci.values().any(|locus_cut| {
        locus_cut.incoming_mailbox.pending.iter().any(|envelope| {
            let Some(received) = locus_cut
                .incoming_endpoint
                .records
                .iter()
                .find(|record| endpoint_receive_record_matches_envelope(record, envelope))
            else {
                return true;
            };
            !sent_records
                .iter()
                .any(|sent| endpoint_transfer_pair_matches(sent, received))
        })
    }) {
        return Err(cut_projection_mismatch());
    }
    if !cut_endpoint_inventory_is_bijective(cut)
        || !cut_counters_are_fresh(cut)
        || !cut_m8_views_are_derived(cut)
    {
        return Err(cut_projection_mismatch());
    }

    if !cut
        .completed_receipts
        .iter()
        .all(|(request_id, receipt)| receipt.request_id == *request_id)
        || !cut
            .route_faults
            .iter()
            .all(|edge_ref| projected_edge_for_ref(program, edge_ref).is_some())
        || !cut.in_transit_faults.entries.iter().all(|fault| {
            projected_edge_for_ref(program, &fault.edge_ref).is_some()
                && fault.envelope_id.as_ref().is_none_or(|envelope_id| {
                    cut.loci.values().any(|locus_cut| {
                        locus_cut.outgoing_mailbox.pending.iter().any(|envelope| {
                            envelope.envelope_id == *envelope_id
                                && envelope.edge_ref == fault.edge_ref
                        })
                    })
                })
        })
    {
        return Err(cut_projection_mismatch());
    }
    Ok(())
}

fn projected_edge_for_ref<'a>(
    program: &'a FabricProgram,
    edge_ref: &str,
) -> Option<&'a CommunicationEdge> {
    program
        .projection
        .communication_plan()
        .edges()
        .iter()
        .find(|edge| edge.edge_ref() == edge_ref)
}

fn envelope_matches_projected_edge(program: &FabricProgram, envelope: &MailboxEnvelope) -> bool {
    let Some(edge) = projected_edge_for_ref(program, &envelope.edge_ref) else {
        return false;
    };
    FabricRouteKey::from_edge(edge)
        == FabricRouteKey {
            operation: envelope.operation_id.clone(),
            kind: envelope.edge_kind,
            source_locus: envelope.source_locus.clone(),
            target_locus: envelope.target_locus.clone(),
        }
        && program
            .route_index
            .route(&FabricRouteKey::from_edge(edge))
            .is_some_and(|route| route.edge_ref == envelope.edge_ref)
        && envelope.carrier_contract == *edge.carrier_contract()
        && envelope.source_ref == edge.source_ref()
        && envelope.core_ref.as_deref() == edge.core_ref()
        && envelope.source_fragment_ref == *edge.source_fragment_ref()
        && envelope.target_fragment_ref == *edge.target_fragment_ref()
        && !envelope.envelope_id.is_empty()
        && !envelope.carrier_id.is_empty()
        && !envelope.mailbox_record_id.is_empty()
        && !envelope.mailbox_enqueue_occurrence_id.is_empty()
}

fn endpoint_record_matches_projected_edge(
    program: &FabricProgram,
    record: &EndpointCarrierRecord,
) -> bool {
    let Some(edge) = projected_edge_for_ref(program, &record.edge_ref) else {
        return false;
    };
    record.edge_kind == edge.kind()
        && record.source_locus == edge.source_locus()
        && record.target_locus == edge.target_locus()
        && record.source_ref == edge.source_ref()
        && record.core_ref.as_deref() == edge.core_ref()
        && record.source_fragment_ref == *edge.source_fragment_ref()
        && record.target_fragment_ref == *edge.target_fragment_ref()
        && !record.record_id.is_empty()
        && !record.carrier_id.is_empty()
        && !record.request_id.is_empty()
}

fn endpoint_transfer_pair_matches(
    sent: &EndpointCarrierRecord,
    received: &EndpointCarrierRecord,
) -> bool {
    sent.carrier_id == received.carrier_id
        && sent.request_id == received.request_id
        && sent.edge_kind == received.edge_kind
        && sent.edge_ref == received.edge_ref
        && sent.source_locus == received.source_locus
        && sent.target_locus == received.target_locus
        && sent.request_carrier_id == received.request_carrier_id
        && sent.input_receipt_carrier_id == received.input_receipt_carrier_id
        && sent.source_ref == received.source_ref
        && sent.core_ref == received.core_ref
        && sent.source_fragment_ref == received.source_fragment_ref
        && sent.target_fragment_ref == received.target_fragment_ref
        && sent.dequeue_occurrence_id.is_some()
        && received.enqueue_occurrence_id.is_some()
}

fn endpoint_receive_record_matches_envelope(
    received: &EndpointCarrierRecord,
    envelope: &MailboxEnvelope,
) -> bool {
    received.record_id == envelope.mailbox_record_id
        && received.carrier_id == envelope.carrier_id
        && received.request_id == envelope.request_id
        && received.edge_kind == envelope.edge_kind
        && received.edge_ref == envelope.edge_ref
        && received.source_locus == envelope.source_locus
        && received.target_locus == envelope.target_locus
        && received.request_carrier_id == envelope.request_carrier_id
        && received.input_receipt_carrier_id == envelope.input_receipt_carrier_id
        && received.source_ref == envelope.source_ref
        && received.core_ref == envelope.core_ref
        && received.source_fragment_ref == envelope.source_fragment_ref
        && received.target_fragment_ref == envelope.target_fragment_ref
        && received.dequeue_occurrence_id.is_none()
        && received.enqueue_occurrence_id.as_deref()
            == Some(envelope.mailbox_enqueue_occurrence_id.as_str())
}

/// Verify the finite carrier inventory as identities, rather than merely as
/// matching field shapes.  A transported carrier has one source send and one
/// target receive; a pending target inbox also names exactly that receive.
/// These checks deliberately permit the two endpoint records to share their
/// one carrier ID while prohibiting duplicate logical carriers or endpoint
/// occurrence IDs.
fn cut_endpoint_inventory_is_bijective(cut: &Sys4LocalCut) -> bool {
    let sent_records: Vec<_> = cut
        .loci
        .values()
        .flat_map(|locus_cut| locus_cut.outgoing_endpoint.records.iter())
        .collect();
    let received_records: Vec<_> = cut
        .loci
        .values()
        .flat_map(|locus_cut| locus_cut.incoming_endpoint.records.iter())
        .collect();
    if sent_records.iter().any(|sent| {
        received_records
            .iter()
            .filter(|received| endpoint_transfer_pair_matches(sent, received))
            .count()
            != 1
    }) || received_records.iter().any(|received| {
        sent_records
            .iter()
            .filter(|sent| endpoint_transfer_pair_matches(sent, received))
            .count()
            != 1
    }) {
        return false;
    }

    let mut envelope_ids = BTreeSet::new();
    let mut pending_carrier_ids = BTreeSet::new();
    let mut mailbox_record_ids = BTreeSet::new();
    let mut mailbox_enqueue_ids = BTreeSet::new();
    for locus_cut in cut.loci.values() {
        for envelope in locus_cut
            .outgoing_mailbox
            .pending
            .iter()
            .chain(locus_cut.incoming_mailbox.pending.iter())
        {
            if !envelope_ids.insert(envelope.envelope_id.clone())
                || !pending_carrier_ids.insert(envelope.carrier_id.clone())
                || !mailbox_record_ids.insert(envelope.mailbox_record_id.clone())
                || !mailbox_enqueue_ids.insert(envelope.mailbox_enqueue_occurrence_id.clone())
            {
                return false;
            }
        }
    }

    let mut endpoint_record_ids = BTreeSet::new();
    let mut endpoint_occurrence_ids = BTreeSet::new();
    let mut endpoint_carrier_counts = BTreeMap::<String, usize>::new();
    for record in sent_records.iter().chain(received_records.iter()) {
        if !endpoint_record_ids.insert(record.record_id.clone()) {
            return false;
        }
        *endpoint_carrier_counts
            .entry(record.carrier_id.clone())
            .or_default() += 1;
        for occurrence in [
            record.enqueue_occurrence_id.as_deref(),
            record.dequeue_occurrence_id.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if !endpoint_occurrence_ids.insert(occurrence.to_string()) {
                return false;
            }
        }
    }
    if endpoint_carrier_counts.values().any(|count| *count != 2) {
        return false;
    }

    for locus_cut in cut.loci.values() {
        for envelope in &locus_cut.outgoing_mailbox.pending {
            // An outbox envelope has not crossed an endpoint yet. Reusing a
            // historical endpoint record or occurrence would make the next
            // transport step ambiguous after restore.
            if endpoint_record_ids.contains(&envelope.mailbox_record_id)
                || endpoint_occurrence_ids.contains(&envelope.mailbox_enqueue_occurrence_id)
                || endpoint_carrier_counts.contains_key(&envelope.carrier_id)
            {
                return false;
            }
        }
        for envelope in &locus_cut.incoming_mailbox.pending {
            let matching_receives = locus_cut
                .incoming_endpoint
                .records
                .iter()
                .filter(|record| endpoint_receive_record_matches_envelope(record, envelope))
                .collect::<Vec<_>>();
            if matching_receives.len() != 1
                || sent_records
                    .iter()
                    .filter(|sent| endpoint_transfer_pair_matches(sent, matching_receives[0]))
                    .count()
                    != 1
            {
                return false;
            }
        }
    }
    true
}

fn sys4_counter_suffix(identifier: &str) -> Option<u64> {
    identifier
        .strip_prefix("sys4-")?
        .rsplit_once('-')?
        .1
        .parse()
        .ok()
}

fn cut_counters_are_fresh(cut: &Sys4LocalCut) -> bool {
    let mut max_endpoint = None;
    let mut max_request = None;
    let mut observe_endpoint = |identifier: &str| {
        if let Some(value) = sys4_counter_suffix(identifier) {
            max_endpoint = Some(max_endpoint.map_or(value, |current: u64| current.max(value)));
        }
    };
    let mut observe_request = |identifier: &str| {
        if let Some(value) = identifier
            .strip_prefix("sys4-request-")
            .or_else(|| identifier.strip_prefix("sys5-relation-request:"))
            .and_then(|suffix| suffix.parse::<u64>().ok())
        {
            max_request = Some(max_request.map_or(value, |current: u64| current.max(value)));
        }
    };

    for locus_cut in cut.loci.values() {
        for envelope in locus_cut
            .outgoing_mailbox
            .pending
            .iter()
            .chain(locus_cut.incoming_mailbox.pending.iter())
        {
            observe_endpoint(&envelope.envelope_id);
            observe_endpoint(&envelope.carrier_id);
            observe_endpoint(&envelope.mailbox_record_id);
            observe_endpoint(&envelope.mailbox_enqueue_occurrence_id);
            if let Some(carrier_id) = &envelope.request_carrier_id {
                observe_endpoint(carrier_id);
            }
            if let Some(carrier_id) = &envelope.input_receipt_carrier_id {
                observe_endpoint(carrier_id);
            }
            observe_request(&envelope.request_id);
        }
        for record in locus_cut
            .outgoing_endpoint
            .records
            .iter()
            .chain(locus_cut.incoming_endpoint.records.iter())
        {
            observe_endpoint(&record.record_id);
            observe_endpoint(&record.carrier_id);
            if let Some(occurrence) = &record.enqueue_occurrence_id {
                observe_endpoint(occurrence);
            }
            if let Some(occurrence) = &record.dequeue_occurrence_id {
                observe_endpoint(occurrence);
            }
            if let Some(carrier_id) = &record.request_carrier_id {
                observe_endpoint(carrier_id);
            }
            if let Some(carrier_id) = &record.input_receipt_carrier_id {
                observe_endpoint(carrier_id);
            }
            observe_request(&record.request_id);
        }
    }
    for (request_id, receipt) in &cut.completed_receipts {
        observe_request(request_id);
        observe_request(&receipt.request_id);
    }
    // Fault-only actions have neither an endpoint carrier nor a completed
    // semantic receipt, but still consume the next request ID. Their retained
    // FabricTrace rows therefore participate in the same freshness floor.
    for entry in &cut.trace.entries {
        observe_request(&entry.request_id);
        if let Some(fault_id) = &entry.fault_id {
            observe_request(fault_id);
        }
    }
    max_endpoint.is_none_or(|maximum| cut.next_endpoint_occurrence > maximum)
        && max_request.is_none_or(|maximum| cut.next_request > maximum)
}

/// `m8_local_runtime_trace`, `actual_m8_trace`, the fabric consumption view,
/// and their M8 causality roots are observer projections.  They are retained
/// for devtools/replay correspondence, but the per-session M8 cuts remain
/// their source.  Reject a cut when a copied projection has been removed,
/// duplicated, or detached from its owning M8 row.
fn cut_m8_views_are_derived(cut: &Sys4LocalCut) -> bool {
    let mut expected = BTreeMap::<String, M8LocalTraceObservation>::new();
    for (locus, m8_cut) in &cut.m8_cuts {
        let Some(session_id) = cut.m8_locus_sessions.get(locus) else {
            return false;
        };
        // SYS-4 cuts are ST-only today, so every semantic locus owns the
        // physical M8 session whose cut it carries.
        if session_id != locus {
            return false;
        }
        let raw_trace = m8_cut.trace_prefix();
        let raw_observations = raw_trace.observations();
        if cut.m8_trace_offsets.get(session_id).copied() != Some(raw_observations.len())
            || cut
                .m8_locus_trace_sequences
                .get(locus)
                .copied()
                .unwrap_or_default()
                != raw_observations.len() as u64
        {
            return false;
        }
        // A session without any M8 occurrence is still part of the whole
        // fabric cut, but has no qualified-node projection to retain.  SYS4
        // captures it without minting a synthetic M8 save occurrence.
        if raw_observations.is_empty() {
            if cut
                .m8_qualified_trace_nodes
                .get(session_id)
                .is_some_and(|nodes| !nodes.is_empty())
                || cut
                    .m8_qualified_trace_dependencies
                    .get(session_id)
                    .is_some_and(|dependencies| !dependencies.is_empty())
                || cut
                    .m8_raw_node_loci
                    .get(session_id)
                    .is_some_and(|loci| !loci.is_empty())
            {
                return false;
            }
            continue;
        }
        let Some(qualified_nodes) = cut.m8_qualified_trace_nodes.get(session_id) else {
            return false;
        };
        let Some(dependencies) = cut.m8_qualified_trace_dependencies.get(session_id) else {
            return false;
        };
        let Some(raw_node_loci) = cut.m8_raw_node_loci.get(session_id) else {
            return false;
        };
        if qualified_nodes.len() != raw_observations.len()
            || dependencies.len() != raw_observations.len()
            || raw_node_loci.len() != raw_observations.len()
        {
            return false;
        }
        for (index, observation) in raw_observations.iter().enumerate() {
            let raw_node_id = observation.node_id();
            let expected_node_id = format!("sys4-m8:{locus}:m8-fabric-trace-{index:020}");
            if qualified_nodes.get(raw_node_id) != Some(&expected_node_id)
                || raw_node_loci.get(raw_node_id) != Some(locus)
            {
                return false;
            }
            let expected_dependencies = observation
                .predecessor_ids()
                .iter()
                .map(|raw| qualified_nodes.get(raw).cloned())
                .collect::<Option<Vec<_>>>();
            let Some(expected_dependencies) = expected_dependencies else {
                return false;
            };
            if dependencies.get(raw_node_id) != Some(&expected_dependencies) {
                return false;
            }
            let qualified =
                observation.fabric_rekeyed(expected_node_id.clone(), expected_dependencies);
            if expected.insert(expected_node_id, qualified).is_some() {
                return false;
            }
        }
    }

    let aggregate = cut.m8_local_runtime_trace.observations();
    if aggregate.len() != expected.len() {
        return false;
    }
    let mut aggregate_ids = BTreeSet::new();
    for observation in &aggregate {
        if !aggregate_ids.insert(observation.node_id().to_string()) {
            return false;
        }
        let Some(expected_observation) = expected.get(observation.node_id()) else {
            return false;
        };
        if !same_derived_m8_observation(observation, expected_observation) {
            return false;
        }
    }

    if cut.actual_m8_trace.nodes.len() != expected.len() {
        return false;
    }
    let mut actual_ids = BTreeSet::new();
    let mut expected_consumptions = BTreeMap::<(String, String), usize>::new();
    for expected_observation in expected.values() {
        if expected_observation.kind() == M8LocalTraceKind::DesignatedValueConsumed {
            let semantic_identity = expected_observation.semantic_identity();
            let consumer = expected_observation.consumer_locus();
            if semantic_identity.is_empty() || consumer.is_empty() {
                return false;
            }
            *expected_consumptions
                .entry((semantic_identity.to_string(), consumer.to_string()))
                .or_default() += 1;
        }
    }
    if cut.m8_trace.consumption_counts != expected_consumptions {
        return false;
    }
    for node in &cut.actual_m8_trace.nodes {
        if !actual_ids.insert(node.node_id.clone()) {
            return false;
        }
        let Some(expected_observation) = expected.get(&node.node_id) else {
            return false;
        };
        if node.kind != format!("{:?}", expected_observation.kind())
            || !expected_observation
                .predecessor_ids()
                .iter()
                .all(|predecessor| node.predecessors.contains(predecessor))
            || !cut.causality.contains_occurrence(&node.node_id)
            || node.predecessors != cut.causality.predecessor_ids(&node.node_id)
            || !node
                .predecessors
                .iter()
                .all(|predecessor| cut.causality.contains_occurrence(predecessor))
        {
            return false;
        }
    }
    true
}

fn same_derived_m8_observation(
    actual: &M8LocalTraceObservation,
    expected: &M8LocalTraceObservation,
) -> bool {
    actual.node_id() == expected.node_id()
        && actual.kind() == expected.kind()
        && actual.predecessor_ids() == expected.predecessor_ids()
        && actual.source_ref == expected.source_ref
        && actual.occurrence_id == expected.occurrence_id
        && actual.designated_context_digest() == expected.designated_context_digest()
}

fn validate_locus_runtime_cut(
    program: &FabricProgram,
    locus: &str,
    locus_cut: &LocusRuntimeCut,
    causality: &CausalityGraph,
) -> Sys4Result<()> {
    if locus_cut.outgoing_mailbox.pending.iter().any(|envelope| {
        envelope.source_locus != locus
            || !envelope_matches_projected_edge(program, envelope)
            || !causality.contains_occurrence(&envelope.mailbox_enqueue_occurrence_id)
    }) || locus_cut.incoming_mailbox.pending.iter().any(|envelope| {
        envelope.target_locus != locus
            || !envelope_matches_projected_edge(program, envelope)
            || !causality.contains_occurrence(&envelope.mailbox_enqueue_occurrence_id)
    }) || locus_cut.outgoing_endpoint.records.iter().any(|record| {
        record.source_locus != locus
            || !endpoint_record_matches_projected_edge(program, record)
            || record
                .dequeue_occurrence_id
                .as_ref()
                .is_none_or(|occurrence| !causality.contains_occurrence(occurrence))
    }) || locus_cut.incoming_endpoint.records.iter().any(|record| {
        record.target_locus != locus
            || !endpoint_record_matches_projected_edge(program, record)
            || record
                .enqueue_occurrence_id
                .as_ref()
                .is_none_or(|occurrence| !causality.contains_occurrence(occurrence))
    }) {
        return Err(cut_projection_mismatch());
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FabricArtifact {
    designated_consumers: BTreeSet<String>,
    designated_evaluation_expressions: BTreeSet<String>,
}

impl FabricArtifact {
    pub(crate) fn has_designated_result_consumer(&self, value_name: &str) -> bool {
        self.designated_consumers.contains(value_name)
    }
    pub(crate) fn has_designated_evaluation_expression(&self, value_name: &str) -> bool {
        self.designated_evaluation_expressions.contains(value_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LocusLocalStore {
    locus: String,
    ints: BTreeMap<(String, String, String), i64>,
}

impl LocusLocalStore {
    fn owned(locus: impl Into<String>) -> Self {
        Self {
            locus: locus.into(),
            ints: BTreeMap::new(),
        }
    }
    pub(crate) fn is_owned_by_locus(&self, locus: &str) -> bool {
        self.locus == locus
    }
    pub(crate) const fn contains_remote_locus_state(&self) -> bool {
        false
    }
    fn int(&self, state: &str, index: &str, field: &str) -> Option<i64> {
        self.ints
            .get(&(state.to_string(), index.to_string(), field.to_string()))
            .copied()
    }
    fn set_int(&mut self, state: &str, index: &str, field: &str, value: i64) {
        self.ints.insert(
            (state.to_string(), index.to_string(), field.to_string()),
            value,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocusRuntime {
    locus: String,
    program_identity: CheckedProgramIdentity,
    local_store: LocusLocalStore,
    artifact: FabricArtifact,
    incoming_endpoint: EndpointCarrierHistory,
    outgoing_endpoint: EndpointCarrierHistory,
    incoming_mailbox: IncomingMailbox,
    outgoing_mailbox: OutgoingMailbox,
}

impl LocusRuntime {
    pub(crate) fn locus(&self) -> &str {
        &self.locus
    }
    pub(crate) fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }
    pub(crate) fn local_store(&self) -> &LocusLocalStore {
        &self.local_store
    }
    pub(crate) fn artifact(&self) -> &FabricArtifact {
        &self.artifact
    }

    pub(crate) fn incoming_endpoint(&self) -> &EndpointCarrierHistory {
        &self.incoming_endpoint
    }

    pub(crate) fn outgoing_endpoint(&self) -> &EndpointCarrierHistory {
        &self.outgoing_endpoint
    }

    pub(crate) fn incoming_mailbox(&self) -> &IncomingMailbox {
        &self.incoming_mailbox
    }

    pub(crate) fn outgoing_mailbox(&self) -> &OutgoingMailbox {
        &self.outgoing_mailbox
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FabricSemanticSnapshot {
    loci: BTreeMap<String, LocusLocalStore>,
}

impl FabricSemanticSnapshot {
    pub(crate) fn int(&self, locus: &str, state: &str, index: &str, field: &str) -> Option<i64> {
        self.loci
            .get(locus)
            .and_then(|store| store.int(state, index, field))
    }
    pub(crate) fn changed_loci_since(&self, prior: &Self) -> Vec<String> {
        self.loci
            .iter()
            .filter_map(|(locus, store)| {
                (prior.loci.get(locus) != Some(store)).then_some(locus.clone())
            })
            .collect()
    }
    pub(crate) fn locus_unchanged_since(&self, locus: &str, prior: &Self) -> bool {
        self.loci.get(locus) == prior.loci.get(locus)
    }
    pub(crate) fn same_state(&self, other: &Self) -> bool {
        self == other
    }
}

/// Observer-safe identity of the active checked runtime configuration.  It
/// deliberately excludes local values and M9 credential material while still
/// covering the program, generated artifacts, routes, and derived cache
/// identity that a rejected patch must leave untouched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActiveRuntimeIdentitySnapshot {
    checked_program_identity: CheckedProgramIdentity,
    projection_fingerprint: BTreeSet<(FabricRouteKey, String)>,
    artifacts: BTreeMap<String, FabricArtifact>,
    route_refs: BTreeSet<String>,
    cache_bindings: BTreeSet<(String, String, String)>,
    patch_generation: u64,
}

impl ActiveRuntimeIdentitySnapshot {
    pub(crate) const fn includes_program_artifact_projection_route_and_cache_identity(
        &self,
    ) -> bool {
        true
    }
}

enum M8ExecutionBackend {
    /// Independent deterministic M8 sessions, one per admitted logical
    /// locus.  No ST operation may borrow another locus's semantic snapshot.
    St(BTreeMap<String, Box<M8LocalRuntime>>),
    Ow1(Ow1WorkerBackend),
}

struct M8OwnerExecution {
    outcome: M8ServeOutcome,
    request_observation: M8LocalTraceObservation,
    serve_observation: M8LocalTraceObservation,
}

struct M8DesignatedEvaluation {
    published: crate::m8_runtime_designated_value::M8PublishedDesignatedValue,
    input_observation: M8LocalTraceObservation,
    evaluation_observation: M8LocalTraceObservation,
}

struct M8BackendFailure {
    kind: Sys4DiagnosticKind,
    observation: Option<Box<M8LocalTraceObservation>>,
}

impl M8BackendFailure {
    fn observed(observation: Box<M8LocalTraceObservation>) -> Self {
        Self {
            kind: Sys4DiagnosticKind::M8ExecutionRejected,
            observation: Some(observation),
        }
    }

    fn unobserved(kind: Sys4DiagnosticKind) -> Self {
        Self {
            kind,
            observation: None,
        }
    }
}

impl M8ExecutionBackend {
    fn profile(&self) -> BackendProfile {
        match self {
            Self::St(_) => BackendProfile::St,
            Self::Ow1(_) => BackendProfile::Ow1,
        }
    }

    fn is_ow1(&self) -> bool {
        matches!(self, Self::Ow1(_))
    }

    /// A checked SYS-4 patch is prepared against clone-only ST sessions.  OW1
    /// has no worker snapshot/clone command yet, so it remains explicitly
    /// ineligible rather than extracting worker-owned mutable M8 state.
    fn clone_for_checked_patch(&self) -> Result<Self, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(Self::St(
                sessions
                    .iter()
                    .map(|(locus, runtime)| (locus.clone(), Box::new((**runtime).clone())))
                    .collect(),
            )),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn install_checked_patch(
        &mut self,
        instance: M8RuntimeInstance,
        authority_generation: &M9AuthorityGeneration,
        patch_id: &str,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => {
                for runtime in sessions.values_mut() {
                    runtime.install_admitted_sys4_checked_patch(instance.clone(), patch_id);
                    runtime.refresh_m9_authority_state(authority_generation.authority_state());
                }
                Ok(())
            }
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn has_pending_owner_requests(&self) -> bool {
        match self {
            Self::St(sessions) => sessions
                .values()
                .any(|runtime| runtime.has_pending_owner_requests()),
            // OW1 checked patches fail closed before clone/preflight. Do not
            // claim a worker-owned FIFO has been observed by the coordinator.
            Self::Ow1(_) => true,
        }
    }

    fn enqueue_and_serve(
        &mut self,
        owner_locus: &str,
        request: M8OwnerRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<M8OwnerExecution, M8BackendFailure> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or_else(|| M8BackendFailure::unobserved(Sys4DiagnosticKind::BackendIneligible))?
                .execute_owner_with_context(owner_locus, request, context)
                .map(
                    |(outcome, request_observation, serve_observation)| M8OwnerExecution {
                        outcome,
                        request_observation,
                        serve_observation,
                    },
                )
                .map_err(M8BackendFailure::observed),
            Self::Ow1(worker) => {
                let execution = worker
                    .execute_owner_with_context(owner_locus, request, context)
                    .map_err(|failure| M8BackendFailure::unobserved(map_worker_failure(failure)))?;
                match execution {
                    Ow1ContextualM8Execution::Served(receipt) => Ok(M8OwnerExecution {
                        request_observation: receipt.request_observation().clone(),
                        serve_observation: receipt.serve_observation().clone(),
                        outcome: receipt.outcome().clone(),
                    }),
                    Ow1ContextualM8Execution::Rejected { observation } => {
                        Err(M8BackendFailure::observed(observation))
                    }
                }
            }
        }
    }

    fn evaluate_designated(
        &mut self,
        evaluator_locus: &str,
        request: M8DesignatedEvaluationRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<M8DesignatedEvaluation, M8BackendFailure> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(evaluator_locus)
                .ok_or_else(|| M8BackendFailure::unobserved(Sys4DiagnosticKind::BackendIneligible))?
                .evaluate_designated_with_context(request, context)
                .map(|(published, input_observation, evaluation_observation)| {
                    M8DesignatedEvaluation {
                        published,
                        input_observation,
                        evaluation_observation,
                    }
                })
                .map_err(M8BackendFailure::observed),
            Self::Ow1(worker) => worker
                .evaluate_designated_with_context(request, context)
                .map_err(|failure| M8BackendFailure::unobserved(map_worker_failure(failure)))?
                .map(|(published, input_observation, evaluation_observation)| {
                    M8DesignatedEvaluation {
                        published,
                        input_observation,
                        evaluation_observation,
                    }
                })
                .map_err(M8BackendFailure::observed),
        }
    }

    fn consume_designated(
        &mut self,
        consumer_locus: &str,
        request: M8ConsumeRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<
        (
            crate::m8_runtime_designated_value::M8ConsumedDesignatedValue,
            M8LocalTraceObservation,
        ),
        M8BackendFailure,
    > {
        match self {
            Self::St(sessions) => sessions
                .get_mut(consumer_locus)
                .ok_or_else(|| M8BackendFailure::unobserved(Sys4DiagnosticKind::BackendIneligible))?
                .consume_published_value_with_context(request, context)
                .map_err(M8BackendFailure::observed),
            Self::Ow1(worker) => worker
                .consume_designated_with_context(request, context)
                .map_err(|failure| M8BackendFailure::unobserved(map_worker_failure(failure)))?
                .map_err(M8BackendFailure::observed),
        }
    }

    /// Read a source-owner value in the backend-owned M8 runtime. The
    /// returned observation is allocated by M8 for the exact dequeued carrier
    /// context; SYS-4 only places it in the fabric causality graph.
    fn read_owner_int_with_context(
        &mut self,
        source_locus: &str,
        key: M8StateKey,
        source_ref: SourceRef,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<Option<(i64, M8LocalTraceObservation)>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .get_mut(source_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .read_owner_int_with_context(key, source_ref, context)),
            Self::Ow1(worker) => worker
                .read_owner_int_with_context(key, source_ref, context)
                .map_err(map_worker_failure),
        }
    }

    /// Relation lifecycle support is intentionally ST-only for the SYS-5
    /// local profile.  The methods still target the independent per-locus M8
    /// session; they never borrow or mutate a remote session directly.
    fn install_relation_bootstrap(
        &mut self,
        owner_locus: &str,
        relation: &str,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .install_finite_local_bootstrap_chain(relation)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn publish_relation(
        &mut self,
        owner_locus: &str,
        relation: &str,
        authority: M8RelationAuthorityUse,
    ) -> Result<M8PublishedRelationState, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .publish_semantic_relation(relation, owner_locus, authority)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn commit_relation_publication(
        &mut self,
        owner_locus: &str,
        publication: &M8PublishedRelationState,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .commit_semantic_relation_publication(publication)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn relation_requires_fresh_reacquire(
        &mut self,
        owner_locus: &str,
        relation: &str,
    ) -> Result<bool, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .relation_requires_fresh_reacquire(relation)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn invalidate_relation(
        &mut self,
        owner_locus: &str,
        relation: &str,
        authority: M8RelationAuthorityUse,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => {
                let anchor = sessions
                    .get(owner_locus)
                    .and_then(|runtime| runtime.relation_state(relation))
                    .map(|state| state.selected_anchor().to_string())
                    .ok_or(Sys4DiagnosticKind::M8ExecutionRejected)?;
                sessions
                    .get_mut(owner_locus)
                    .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                    .invalidate_primary(
                        relation,
                        authority,
                        M8BindingInvalidation::anchor_unavailable(anchor),
                    )
                    .map(|_| ())
                    .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected)
            }
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn install_fresh_relation_lease(
        &mut self,
        owner_locus: &str,
        relation: &str,
        lease: M8LeaseRecord,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .install_sealed_fresh_relation_lease(relation, lease)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn reacquire_relation(
        &mut self,
        owner_locus: &str,
        relation: &str,
        authority: M8RelationAuthorityUse,
        reacquire: M8RelationReacquire,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(owner_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .reacquire_primary(relation, authority, reacquire)
                .map(|_| ())
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn import_relation_shadow(
        &mut self,
        consumer_locus: &str,
        publication: M8PublishedRelationState,
    ) -> Result<M8ObservedRelationShadow, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(consumer_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .import_semantic_relation_shadow(consumer_locus, publication)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn qualify_relation_shadow_observe_occurrence(
        &mut self,
        consumer_locus: &str,
        shadow: &M8ObservedRelationShadow,
        qualified_occurrence: &str,
    ) -> Result<M8ObservedRelationShadow, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(consumer_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .qualify_observed_relation_shadow_occurrence(shadow, qualified_occurrence)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn relation_shadow(
        &self,
        consumer_locus: &str,
        relation: &str,
    ) -> Result<Option<M8ObservedRelationShadow>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .get(consumer_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .observed_relation_shadow(relation, consumer_locus)),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn project_relation_shadow(
        &self,
        consumer_locus: &str,
        relation: &str,
        context: M8PresentationContext,
    ) -> Result<M8RelationProjection, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get(consumer_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .project_observed_relation_shadow(relation, context)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn has_designated_publication_id(
        &self,
        locus: &str,
        value_name: &str,
        value_id: &str,
    ) -> Result<bool, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .get(locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .has_designated_publication_id(value_name, value_id)),
            Self::Ow1(worker) => worker
                .has_designated_publication_id(value_name, value_id)
                .map_err(map_worker_failure),
        }
    }

    fn validate_designated_non_consuming(
        &mut self,
        consumer_locus: &str,
        request: M8ConsumeRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<String, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => sessions
                .get_mut(consumer_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .validate_published_value_non_consuming(request, context)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(worker) => worker
                .validate_designated_non_consuming(request, context)
                .map_err(map_worker_failure),
        }
    }

    fn local_trace_snapshot(
        &self,
        locus: &str,
    ) -> Result<Option<(String, M8LocalTrace)>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .get(locus)
                .map(|runtime| (locus.to_string(), runtime.trace()))),
            Self::Ow1(worker) => match worker.local_trace_snapshot() {
                Ok(trace) => Ok(Some(("ow1".to_string(), trace))),
                Err(failure) => Err(map_worker_failure(failure)),
            },
        }
    }

    fn save_local_cuts(
        &mut self,
        cut_id: &str,
    ) -> Result<BTreeMap<String, M8LocalCut>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .iter_mut()
                .map(|(locus, runtime)| {
                    (
                        locus.clone(),
                        runtime.capture_for_sys4_local_cut(format!("{cut_id}:{locus}")),
                    )
                })
                .collect()),
            // A SYS-4 whole-fabric cut has not yet acquired an acknowledged
            // worker-cut command.  Fail closed rather than extracting the
            // worker-owned M8 runtime through the coordinator.
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn restore_local_cuts(
        &mut self,
        cuts: &BTreeMap<String, M8LocalCut>,
        live_authority: &crate::m8_runtime_authority::M8AuthorityState,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => {
                if sessions.len() != cuts.len()
                    || !sessions.keys().all(|locus| cuts.contains_key(locus))
                    || !cuts
                        .values()
                        .all(|cut| cut.authority_inventory() == live_authority)
                {
                    return Err(Sys4DiagnosticKind::ProgramProjectionMismatch);
                }
                for (locus, runtime) in sessions {
                    let cut = cuts
                        .get(locus)
                        .expect("validated session cut inventory is total");
                    let floor = M8LiveFloor::for_restoration_with_live_authority(
                        cut,
                        live_authority.clone(),
                    );
                    runtime
                        .try_restore_local_cut(cut, &floor)
                        .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected)?;
                }
                Ok(())
            }
            Self::Ow1(_) => Err(Sys4DiagnosticKind::BackendIneligible),
        }
    }

    fn designated_publication_snapshot(
        &self,
        value_name: &str,
    ) -> Result<Option<Ow1ObserverDesignatedPublication>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions.values().find_map(|runtime| {
                runtime
                    .designated_result_store()
                    .published_values(value_name)
                    .first()
                    .map(|value| Ow1ObserverDesignatedPublication::from_published(value))
            })),
            Self::Ow1(worker) => match worker.designated_publication_snapshot(value_name) {
                Ok(publication) => Ok(publication),
                Err(failure) => Err(map_worker_failure(failure)),
            },
        }
    }

    fn replace_designated_input_receipts(
        &mut self,
        evaluator_locus: &str,
        receipts: M8InputReceiptSet,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => {
                let runtime = sessions
                    .get_mut(evaluator_locus)
                    .ok_or(Sys4DiagnosticKind::BackendIneligible)?;
                runtime.replace_designated_input_receipts(receipts);
                Ok(())
            }
            Self::Ow1(worker) => worker
                .replace_designated_input_receipts(receipts)
                .map_err(map_worker_failure),
        }
    }

    fn import_designated_publication(
        &mut self,
        consumer_locus: &str,
        publication: M8PublishedDesignatedValue,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<Option<M8LocalTraceObservation>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => {
                let source_ref = publication.source_ref().clone();
                sessions
                    .get_mut(consumer_locus)
                    .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                    .import_designated_publication(publication, source_ref, context)
                    .map_err(|_| Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch)
            }
            Self::Ow1(worker) => worker
                .import_designated_publication_with_context(publication, context)
                .map_err(map_worker_failure)?
                .map_err(|_| Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch),
        }
    }

    /// Validate publication provenance and visibility/redaction against the
    /// consumer session's admitted M8 plan before the carrier can reach M9,
    /// M8 import, cache, or consumption.
    fn validates_generated_designated_publication(
        &self,
        consumer_locus: &str,
        publication: M8PublishedDesignatedValue,
    ) -> Result<bool, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .get(consumer_locus)
                .ok_or(Sys4DiagnosticKind::BackendIneligible)?
                .accepts_generated_designated_publication(&publication)),
            Self::Ow1(worker) => worker
                .validates_generated_designated_publication(publication)
                .map_err(map_worker_failure),
        }
    }

    #[cfg(test)]
    fn observer_safe_session(
        &self,
        locus: &str,
    ) -> Result<Option<M8LocalSessionObserver>, Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => Ok(sessions
                .get(locus)
                .map(|runtime| runtime.observer_safe_session())),
            // OW1 has exactly one worker-owned M8 session.  Its redacted
            // observer snapshot remains available only at that semantic
            // worker locus; other fabric loci never acquire a surrogate
            // partition from the physical worker.
            Self::Ow1(worker) if worker.evidence().target_owner().as_str() == locus => {
                match worker.observer_safe_session() {
                    Ok(observer) => Ok(Some(observer)),
                    Err(failure) => Err(map_worker_failure(failure)),
                }
            }
            Self::Ow1(_) => Ok(None),
        }
    }

    #[cfg(test)]
    fn fail_next_local_trace_snapshot_once(&mut self) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(_) => Err(Sys4DiagnosticKind::BackendIneligible),
            Self::Ow1(worker) => worker
                .fail_next_local_trace_snapshot_once()
                .map_err(map_worker_failure),
        }
    }

    #[cfg(test)]
    fn fail_next_designated_publication_snapshot_once(&mut self) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(_) => Err(Sys4DiagnosticKind::BackendIneligible),
            Self::Ow1(worker) => worker
                .fail_next_designated_publication_snapshot_once()
                .map_err(map_worker_failure),
        }
    }

    #[cfg(test)]
    fn fail_next_observer_safe_session_once(&mut self) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(_) => Err(Sys4DiagnosticKind::BackendIneligible),
            Self::Ow1(worker) => worker
                .fail_next_observer_safe_session_once()
                .map_err(map_worker_failure),
        }
    }

    fn refresh_authority(
        &mut self,
        generation: &M9AuthorityGeneration,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(sessions) => {
                for runtime in sessions.values_mut() {
                    runtime.refresh_m9_authority_state(generation.authority_state());
                }
                Ok(())
            }
            Self::Ow1(worker) => worker
                .refresh_authority_and_ack(generation.authority_state())
                .map_err(map_worker_failure),
        }
    }

    #[cfg(test)]
    fn arm_designated_consume_rejection(
        &mut self,
        envelope_id: &str,
        publication_id: &str,
        consumer: &str,
    ) -> Sys4Result<()> {
        match self {
            Self::St(sessions) => {
                let runtime = sessions.get_mut(consumer).ok_or_else(|| {
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::BackendIneligible)
                })?;
                runtime.arm_designated_consume_rejection(M8LocalDesignatedTraceContext::new(
                    envelope_id,
                    "m8-test-armed",
                    consumer,
                    publication_id,
                    "m8-test-armed",
                    "m8-test-armed",
                ));
                Ok(())
            }
            Self::Ow1(_) => Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::BackendIneligible,
            )),
        }
    }

    #[cfg(test)]
    fn arm_owner_operation_rejection(
        &mut self,
        envelope_id: &str,
        operation: &str,
        owner_locus: &str,
    ) -> Sys4Result<()> {
        match self {
            Self::St(sessions) => {
                let runtime = sessions.get_mut(owner_locus).ok_or_else(|| {
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::BackendIneligible)
                })?;
                runtime.arm_owner_operation_rejection(
                    M8LocalDesignatedTraceContext::new(
                        envelope_id,
                        "m8-test-armed-owner",
                        "",
                        "",
                        "",
                        "",
                    )
                    .with_operation_id(operation)
                    .with_owner_locus(owner_locus),
                );
                Ok(())
            }
            Self::Ow1(worker) => worker
                .arm_owner_operation_rejection(
                    M8LocalDesignatedTraceContext::new(
                        envelope_id,
                        "m8-test-armed-owner",
                        "",
                        "",
                        "",
                        "",
                    )
                    .with_operation_id(operation)
                    .with_owner_locus(owner_locus),
                )
                .map_err(|failure| Sys4DispatchDiagnostics::one(map_worker_failure(failure))),
        }
    }

    #[cfg(test)]
    fn arm_designated_evaluation_rejection(
        &mut self,
        envelope_id: &str,
        operation: &str,
        evaluator_locus: &str,
        tick: &str,
    ) -> Sys4Result<()> {
        match self {
            Self::St(sessions) => {
                let runtime = sessions.get_mut(evaluator_locus).ok_or_else(|| {
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::BackendIneligible)
                })?;
                runtime.arm_designated_evaluation_rejection(
                    M8LocalDesignatedTraceContext::new(
                        envelope_id,
                        "m8-test-armed-evaluation",
                        "",
                        "",
                        tick,
                        "",
                    )
                    .with_operation_id(operation)
                    .with_evaluator_locus(evaluator_locus),
                );
                Ok(())
            }
            Self::Ow1(_) => Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::BackendIneligible,
            )),
        }
    }
}

#[cfg(test)]
pub(crate) struct M8BackendTestSupport<'a> {
    backend: &'a mut M8ExecutionBackend,
}

#[cfg(test)]
impl M8BackendTestSupport<'_> {
    pub(crate) fn fail_next_local_trace_snapshot_once(&mut self) -> Sys4Result<()> {
        self.backend
            .fail_next_local_trace_snapshot_once()
            .map_err(Sys4DispatchDiagnostics::one)
    }

    pub(crate) fn fail_next_designated_publication_snapshot_once(&mut self) -> Sys4Result<()> {
        self.backend
            .fail_next_designated_publication_snapshot_once()
            .map_err(Sys4DispatchDiagnostics::one)
    }

    pub(crate) fn fail_next_observer_safe_session_once(&mut self) -> Sys4Result<()> {
        self.backend
            .fail_next_observer_safe_session_once()
            .map_err(Sys4DispatchDiagnostics::one)
    }

    pub(crate) fn reject_next_designated_consume_after_validation(
        &mut self,
        envelope_id: &str,
        publication_id: &str,
        consumer: &str,
    ) -> Sys4Result<()> {
        self.backend
            .arm_designated_consume_rejection(envelope_id, publication_id, consumer)
    }

    pub(crate) fn reject_next_owner_operation_after_dequeue(
        &mut self,
        envelope_id: &str,
        operation: &str,
        owner_locus: &str,
    ) -> Sys4Result<()> {
        self.backend
            .arm_owner_operation_rejection(envelope_id, operation, owner_locus)
    }

    pub(crate) fn reject_next_designated_evaluation_after_input_receipt(
        &mut self,
        envelope_id: &str,
        operation: &str,
        evaluator_locus: &str,
        tick: &str,
    ) -> Sys4Result<()> {
        self.backend.arm_designated_evaluation_rejection(
            envelope_id,
            operation,
            evaluator_locus,
            tick,
        )
    }
}

/// Test/devtools evidence derived from the actual M8 sessions which back an
/// ST fabric.  It deliberately contains no authority, witness, credential,
/// payload, or raw M8 identifier material.
#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8RuntimePartitionEvidence {
    partitions: BTreeMap<String, M8RuntimePartition>,
}

#[cfg(test)]
impl M8RuntimePartitionEvidence {
    pub(crate) const fn is_observer_safe(&self) -> bool {
        true
    }

    pub(crate) fn locus_names(&self) -> Vec<String> {
        self.partitions.keys().cloned().collect()
    }

    pub(crate) fn partition(&self, locus: &str) -> Option<&M8RuntimePartition> {
        self.partitions.get(locus)
    }

    pub(crate) fn changed_partitions_since(&self, before: &Self) -> Vec<String> {
        self.partitions
            .iter()
            .filter(|(locus, current)| before.partitions.get(*locus) != Some(*current))
            .map(|(locus, _)| locus.clone())
            .collect()
    }

    pub(crate) fn all_m8_trace_occurrence_ids(&self) -> Vec<String> {
        self.partitions
            .values()
            .flat_map(|partition| {
                partition
                    .trace_occurrences
                    .iter()
                    .map(|occurrence| occurrence.fabric_qualified_id.clone())
            })
            .collect()
    }

    pub(crate) fn all_m8_trace_occurrence_ids_are_unique(&self) -> bool {
        let ids = self.all_m8_trace_occurrence_ids();
        ids.iter().collect::<BTreeSet<_>>().len() == ids.len()
    }

    pub(crate) fn all_m8_trace_occurrences_resolve_in(
        &self,
        actual_trace: &ActualM8Trace,
        causality: &CausalityGraph,
    ) -> bool {
        self.partitions.values().all(|partition| {
            partition.trace_occurrences.iter().all(|occurrence| {
                actual_trace
                    .nodes
                    .iter()
                    .find(|node| node.node_id == occurrence.fabric_qualified_id)
                    .is_some_and(|node| node.kind() == occurrence.kind())
                    && causality.contains_occurrence(occurrence.fabric_qualified_id())
            })
        })
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8RuntimePartition {
    session_id: String,
    partition_id: String,
    state_inventory: M8RuntimeStateInventory,
    publication_inventory: M8RuntimePublicationInventory,
    trace_occurrences: Vec<M8RuntimeOccurrence>,
}

#[cfg(test)]
impl M8RuntimePartition {
    fn from_m8_session(
        locus: &str,
        observer: M8LocalSessionObserver,
        fabric_node_ids: &BTreeMap<String, String>,
        request_ids: &BTreeMap<String, String>,
        dependencies: &BTreeMap<String, Vec<String>>,
    ) -> Self {
        let trace_occurrences: Vec<_> = observer
            .trace_observations()
            .iter()
            .map(|observation| {
                let node_id = fabric_node_ids
                    .get(observation.node_id())
                    .cloned()
                    .expect("refreshed M8 session retains each observer row's fabric identity");
                M8RuntimeOccurrence {
                    fabric_qualified_id: node_id.clone(),
                    kind: observation.kind(),
                    m8_publication_id: observation.m8_publication_id().to_string(),
                    envelope_id: observation.envelope_id().to_string(),
                    operation_id: observation.operation_id().to_string(),
                    edge_ref: observation.edge_ref().to_string(),
                    evaluator_locus: observation.evaluator_locus().to_string(),
                    consumer_locus: observation.consumer_locus().to_string(),
                    request_id: request_ids.get(&node_id).cloned(),
                    qualified_dependency_graph: QualifiedM8DependencyGraph::for_root(
                        node_id,
                        dependencies,
                    ),
                    observer_safe_read_key_refs: if observation.kind()
                        == M8LocalTraceKind::OwnerRead
                    {
                        observer.owner_read_key_refs_for_node(observation.node_id())
                    } else {
                        Vec::new()
                    },
                }
            })
            .collect();
        let publications = trace_occurrences
            .iter()
            .filter(|occurrence| {
                !occurrence.m8_publication_id.is_empty()
                    && matches!(
                        occurrence.kind,
                        M8LocalTraceKind::DesignatedValuePublished
                            | M8LocalTraceKind::DesignatedPublicationImported
                    )
            })
            .map(|occurrence| (occurrence.m8_publication_id.clone(), occurrence.clone()))
            .collect();
        Self {
            session_id: format!("sys4-m8-session:{locus}"),
            partition_id: format!("sys4-m8-partition:{locus}"),
            state_inventory: M8RuntimeStateInventory {
                key_refs: observer.state_key_refs().to_vec(),
                state_digest: observer.state_digest().to_string(),
            },
            publication_inventory: M8RuntimePublicationInventory {
                publications,
                published_value_refs: observer.published_value_refs().to_vec(),
            },
            trace_occurrences,
        }
    }

    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    pub(crate) fn partition_id(&self) -> &str {
        &self.partition_id
    }

    pub(crate) fn authoritative_state_key_refs(&self) -> Vec<String> {
        self.state_inventory.key_refs()
    }

    pub(crate) fn state_digest(&self) -> &str {
        self.state_inventory.state_digest()
    }

    pub(crate) fn state_inventory(&self) -> &M8RuntimeStateInventory {
        &self.state_inventory
    }

    pub(crate) fn publication_inventory(&self) -> &M8RuntimePublicationInventory {
        &self.publication_inventory
    }

    pub(crate) fn m8_trace_occurrences_for_operation(
        &self,
        operation: &str,
    ) -> M8RuntimeOccurrences {
        M8RuntimeOccurrences(
            self.trace_occurrences
                .iter()
                .filter(|occurrence| occurrence.operation_id() == operation)
                .cloned()
                .collect(),
        )
    }

    pub(crate) fn m8_trace_occurrences_by_kind(
        &self,
        kind: M8LocalTraceKind,
    ) -> M8RuntimeOccurrences {
        M8RuntimeOccurrences(
            self.trace_occurrences
                .iter()
                .filter(|occurrence| occurrence.kind() == kind)
                .cloned()
                .collect(),
        )
    }

    pub(crate) fn m8_trace_occurrence_count_for_operation(
        &self,
        operation: &str,
        kind: M8LocalTraceKind,
    ) -> usize {
        self.m8_trace_occurrences_for_operation(operation)
            .count_kind(kind)
    }

    pub(crate) fn has_m8_trace_occurrences_for_operation(&self, operation: &str) -> bool {
        !self
            .m8_trace_occurrences_for_operation(operation)
            .is_empty()
    }

    pub(crate) fn single_m8_trace_occurrence_for_operation(
        &self,
        operation: &str,
        kind: M8LocalTraceKind,
    ) -> &M8RuntimeOccurrence {
        let matches: Vec<_> = self
            .trace_occurrences
            .iter()
            .filter(|occurrence| {
                occurrence.operation_id() == operation && occurrence.kind() == kind
            })
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "expected exactly one matching M8 occurrence"
        );
        matches[0]
    }

    pub(crate) fn single_m8_trace_occurrence_for_request(
        &self,
        request_id: &str,
        kind: M8LocalTraceKind,
    ) -> &M8RuntimeOccurrence {
        let matches: Vec<_> = self
            .trace_occurrences
            .iter()
            .filter(|occurrence| {
                occurrence.request_id.as_deref() == Some(request_id) && occurrence.kind() == kind
            })
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "expected exactly one request-keyed M8 occurrence"
        );
        matches[0]
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8RuntimeStateInventory {
    key_refs: Vec<String>,
    state_digest: String,
}

#[cfg(test)]
impl M8RuntimeStateInventory {
    pub(crate) const fn is_derived_from_m8_session(&self) -> bool {
        true
    }

    pub(crate) fn key_refs(&self) -> Vec<String> {
        self.key_refs.clone()
    }

    pub(crate) fn state_digest(&self) -> &str {
        &self.state_digest
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct M8RuntimePublicationInventory {
    publications: BTreeMap<String, M8RuntimeOccurrence>,
    published_value_refs: Vec<String>,
}

#[cfg(test)]
impl M8RuntimePublicationInventory {
    pub(crate) const fn is_derived_from_m8_session(&self) -> bool {
        true
    }

    pub(crate) fn published_value_refs(&self) -> &[String] {
        &self.published_value_refs
    }

    pub(crate) fn contains_publication_id(&self, publication_id: &str) -> bool {
        self.publications.contains_key(publication_id)
    }

    pub(crate) fn publication(&self, publication_id: &str) -> Option<&M8RuntimeOccurrence> {
        self.publications.get(publication_id)
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8RuntimeOccurrence {
    fabric_qualified_id: String,
    kind: M8LocalTraceKind,
    m8_publication_id: String,
    envelope_id: String,
    operation_id: String,
    edge_ref: String,
    evaluator_locus: String,
    consumer_locus: String,
    request_id: Option<String>,
    qualified_dependency_graph: QualifiedM8DependencyGraph,
    observer_safe_read_key_refs: Vec<String>,
}

#[cfg(test)]
impl M8RuntimeOccurrence {
    pub(crate) fn fabric_qualified_id(&self) -> &str {
        &self.fabric_qualified_id
    }

    pub(crate) fn node_id(&self) -> &str {
        &self.fabric_qualified_id
    }

    pub(crate) const fn kind(&self) -> M8LocalTraceKind {
        self.kind
    }

    pub(crate) fn m8_publication_id(&self) -> &str {
        &self.m8_publication_id
    }

    pub(crate) fn envelope_id(&self) -> &str {
        &self.envelope_id
    }

    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub(crate) fn evaluator_locus(&self) -> &str {
        &self.evaluator_locus
    }

    pub(crate) fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub(crate) fn qualified_dependency_graph(&self) -> &QualifiedM8DependencyGraph {
        &self.qualified_dependency_graph
    }

    pub(crate) fn observer_safe_read_key_refs(&self) -> Vec<String> {
        self.observer_safe_read_key_refs.clone()
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QualifiedM8DependencyGraph {
    root: String,
    predecessors: BTreeMap<String, Vec<String>>,
}

#[cfg(test)]
impl QualifiedM8DependencyGraph {
    fn for_root(root: String, all_predecessors: &BTreeMap<String, Vec<String>>) -> Self {
        let mut pending = vec![root.clone()];
        let mut predecessors = BTreeMap::new();
        while let Some(node) = pending.pop() {
            if predecessors.contains_key(&node) {
                continue;
            }
            let node_predecessors = all_predecessors.get(&node).cloned().unwrap_or_default();
            pending.extend(node_predecessors.iter().cloned());
            predecessors.insert(node, node_predecessors);
        }
        Self { root, predecessors }
    }

    pub(crate) fn reaches(&self, predecessor: &str) -> bool {
        let mut pending = self
            .predecessors
            .get(&self.root)
            .cloned()
            .unwrap_or_default();
        let mut seen = BTreeSet::new();
        while let Some(current) = pending.pop() {
            if current == predecessor {
                return true;
            }
            if seen.insert(current.clone())
                && let Some(next) = self.predecessors.get(&current)
            {
                pending.extend(next.iter().cloned());
            }
        }
        false
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct M8RuntimeOccurrences(Vec<M8RuntimeOccurrence>);

#[cfg(test)]
impl M8RuntimeOccurrences {
    pub(crate) const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) const fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn single(&self) -> &M8RuntimeOccurrence {
        assert_eq!(self.0.len(), 1, "expected exactly one M8 occurrence");
        &self.0[0]
    }

    pub(crate) fn count_kind(&self, kind: M8LocalTraceKind) -> usize {
        self.0
            .iter()
            .filter(|occurrence| occurrence.kind() == kind)
            .count()
    }
}

fn map_worker_failure(failure: Ow1WorkerFailure) -> Sys4DiagnosticKind {
    match failure {
        Ow1WorkerFailure::ObserverSnapshotUnavailable => {
            Sys4DiagnosticKind::ObserverSnapshotUnavailable
        }
        Ow1WorkerFailure::Designated(diagnostics) => {
            let _ = diagnostics.primary();
            Sys4DiagnosticKind::M8ExecutionRejected
        }
        Ow1WorkerFailure::Disconnected
        | Ow1WorkerFailure::WorkerPanicked
        | Ow1WorkerFailure::Enqueue(_)
        | Ow1WorkerFailure::Serve(_)
        | Ow1WorkerFailure::FifoIdentityMismatch => Sys4DiagnosticKind::M8ExecutionRejected,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CachedDelivery {
    value: RuntimeValue,
    result_version: ResultVersion,
    delivery_id: String,
    semantic_identity: String,
    operation: String,
    consumer_locus: String,
    visibility_redaction: String,
    policy_stamp: String,
    sealed_delivery_binding: SealedDeliveryBinding,
    sealed_delivery_binding_digest: String,
}

impl CachedDelivery {
    pub(crate) fn matches_semantic_identity_source_core_frontiers_version_policy_visibility_redaction(
        &self,
        semantic_identity: &str,
        operation: &str,
        consumer_locus: &str,
        result_version: ResultVersion,
    ) -> bool {
        self.semantic_identity == semantic_identity
            && self.operation == operation
            && self.consumer_locus == consumer_locus
            && self.result_version == result_version
            && !self.visibility_redaction.is_empty()
            && !self.policy_stamp.is_empty()
    }

    pub(crate) fn sealed_delivery_binding(&self) -> &SealedDeliveryBinding {
        &self.sealed_delivery_binding
    }

    pub(crate) fn sealed_delivery_binding_digest(&self) -> &str {
        &self.sealed_delivery_binding_digest
    }
}

#[derive(Clone)]
pub(crate) struct M9AuthorityLifecycle {
    publisher: M9AuthoritySuccessorPublisher,
}

impl M9AuthorityLifecycle {
    fn matches_generation_for_restore(&self, generation: &M9AuthorityGeneration) -> bool {
        self.publisher
            .current_generation_for_restore()
            .matches_for_restore(generation)
    }

    /// Private M9 publisher continuation state retained by a SYS-4 local
    /// cut. It is opaque outside this module and cannot create authority.
    fn private_restore_integrity_digest(&self) -> String {
        self.publisher.private_restore_integrity_digest()
    }

    /// M9, not SYS-4, adopts exact runtime validation observations before a
    /// successor operation.  The caller has already held the matching shared
    /// authority-floor guard; a false result is a fail-closed identity or
    /// lineage mismatch, never an invitation to reconstruct observations.
    fn synchronize_from_live_generation(&mut self, live: &M9AuthorityGeneration) -> bool {
        self.publisher.synchronize_runtime_observations_from(live)
    }

    fn transition(
        &mut self,
        value_name: &str,
        consumer: &str,
        kind: M9AuthorityTransitionKind,
        operation: impl FnOnce(
            &mut M9AuthoritySuccessorPublisher,
        ) -> Result<
            M9AuthorityGeneration,
            crate::m9_auth_verification::M9AdmissionDiagnostics,
        >,
    ) -> Sys4Result<M9AuthorityTransition> {
        let prior = self.publisher.current_inspection();
        let prior_publisher = self.publisher.clone();
        let prior_runtime_validation_observations = self
            .publisher
            .current_runtime_validation_observation_snapshot();
        let lineage = prior
            .designated_consumer_lineage(value_name, consumer)
            .cloned()
            .ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::MissingConsumerCapability)
            })?;
        let generation = operation(&mut self.publisher).map_err(|_| {
            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::MissingConsumerCapability)
        })?;
        let sealed_m9_inspection =
            generation.transition_inspection(&prior, kind, Some(lineage.clone()), None);
        Ok(M9AuthorityTransition {
            generation,
            sealed_m9_inspection,
            prior_runtime_validation_observations,
            prior_publisher,
        })
    }
    pub(crate) fn revoke_designated_consumer_capability(
        &mut self,
        value_name: &str,
        consumer: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.transition(
            value_name,
            consumer,
            M9AuthorityTransitionKind::DesignatedConsumerCapabilityRevoked,
            |publisher| publisher.revoke_designated_consumption_capability(consumer, value_name),
        )
    }

    pub(crate) fn retire_designated_consumer_membership(
        &mut self,
        value_name: &str,
        consumer: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.transition(
            value_name,
            consumer,
            M9AuthorityTransitionKind::DesignatedConsumerMembershipRetired,
            |publisher| publisher.retire_designated_consumption_membership(consumer, value_name),
        )
    }

    pub(crate) fn retire_designated_consumer_witness(
        &mut self,
        value_name: &str,
        consumer: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.transition(
            value_name,
            consumer,
            M9AuthorityTransitionKind::DesignatedConsumerWitnessRetired,
            |publisher| publisher.retire_designated_consumption_witness(consumer, value_name),
        )
    }

    pub(crate) fn revoke_designated_source_release(
        &mut self,
        lineage: &M9DesignatedSourceReleaseLineage,
    ) -> Sys4Result<M9AuthorityTransition> {
        let prior = self.publisher.current_inspection();
        let prior_publisher = self.publisher.clone();
        let prior_runtime_validation_observations = self
            .publisher
            .current_runtime_validation_observation_snapshot();
        let generation = self
            .publisher
            .revoke_designated_source_release(lineage)
            .map_err(|_| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::MissingSourceReleaseAuthority)
            })?;
        let sealed_m9_inspection = generation.transition_inspection(
            &prior,
            M9AuthorityTransitionKind::DesignatedSourceReleaseRevoked,
            None,
            Some(lineage.clone()),
        );
        Ok(M9AuthorityTransition {
            generation,
            sealed_m9_inspection,
            prior_runtime_validation_observations,
            prior_publisher,
        })
    }

    /// Retire a relation's source-declared primary membership only through
    /// the M9 publisher.  The caller supplies the relation plus its explicit
    /// checked anchor locus; M9 derives the principal and resolves the live
    /// membership/capability/witness lineage internally.
    fn retire_source_declared_primary_anchor(
        &mut self,
        relation: &str,
        checked_primary_locus: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        let prior = self.publisher.current_inspection();
        let prior_publisher = self.publisher.clone();
        let prior_runtime_validation_observations = self
            .publisher
            .current_runtime_validation_observation_snapshot();
        let (generation, retirement) = self
            .publisher
            .retire_source_declared_primary_anchor(relation, checked_primary_locus)
            .map_err(|_| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))?;
        let sealed_m9_inspection =
            generation.membership_retirement_transition_inspection(&prior, retirement);
        Ok(M9AuthorityTransition {
            generation,
            sealed_m9_inspection,
            prior_runtime_validation_observations,
            prior_publisher,
        })
    }

    /// Re-admit only the checked primary anchor selected by M9's finite
    /// template.  The relation name is source-derived at the caller; no
    /// principal, locus, epoch, membership, capability, or witness crosses
    /// this SYS-4 boundary.
    fn reacquire_source_declared_primary_anchor(
        &mut self,
        relation: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        let prior = self.publisher.current_inspection();
        let prior_publisher = self.publisher.clone();
        let prior_runtime_validation_observations = self
            .publisher
            .current_runtime_validation_observation_snapshot();
        let (generation, reacquire) = self
            .publisher
            .reacquire_source_declared_primary_anchor(relation)
            .map_err(|_| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))?;
        let sealed_m9_inspection =
            generation.membership_reacquire_transition_inspection(&prior, reacquire);
        Ok(M9AuthorityTransition {
            generation,
            sealed_m9_inspection,
            prior_runtime_validation_observations,
            prior_publisher,
        })
    }

    /// Restore the M9 publisher only when this fabric owns the exact
    /// uninstalled successor. A stale or foreign transition cannot roll back
    /// a publisher that has since advanced.
    fn rollback_uninstalled_transition(&mut self, transition: &M9AuthorityTransition) -> bool {
        self.publisher.restore_uninstalled_successor(
            transition.prior_publisher.clone(),
            &transition.generation,
        )
    }
}

/// Borrowed authority-transition boundary. It keeps the shared M9 live-floor
/// guard from the final M9-owned observation synchronization through exactly
/// one successor publication request. The subsequent backend/fabric install
/// remains in `apply_admitted_authority_lifecycle`, where the floor is
/// rechecked before any semantic state changes.
pub(crate) struct M9AuthorityLifecycleAccess<'a> {
    lifecycle: &'a mut M9AuthorityLifecycle,
    live_generation: &'a M9AuthorityGeneration,
    floor_guard: Option<M9AuthorityLiveFloorGuard<'a>>,
}

impl M9AuthorityLifecycleAccess<'_> {
    fn synchronize_before_successor(&mut self) -> Sys4Result<()> {
        let Some(floor_guard) = self.floor_guard.as_ref() else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        };
        if !floor_guard.matches_runtime_authority_facts(self.live_generation)
            || !self
                .lifecycle
                .synchronize_from_live_generation(self.live_generation)
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        Ok(())
    }

    pub(crate) fn revoke_designated_consumer_capability(
        &mut self,
        value_name: &str,
        consumer: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.synchronize_before_successor()?;
        self.lifecycle
            .revoke_designated_consumer_capability(value_name, consumer)
    }

    pub(crate) fn retire_designated_consumer_membership(
        &mut self,
        value_name: &str,
        consumer: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.synchronize_before_successor()?;
        self.lifecycle
            .retire_designated_consumer_membership(value_name, consumer)
    }

    pub(crate) fn retire_designated_consumer_witness(
        &mut self,
        value_name: &str,
        consumer: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.synchronize_before_successor()?;
        self.lifecycle
            .retire_designated_consumer_witness(value_name, consumer)
    }

    pub(crate) fn revoke_designated_source_release(
        &mut self,
        lineage: &M9DesignatedSourceReleaseLineage,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.synchronize_before_successor()?;
        self.lifecycle.revoke_designated_source_release(lineage)
    }

    fn retire_source_declared_primary_anchor(
        &mut self,
        relation: &str,
        checked_primary_locus: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.synchronize_before_successor()?;
        self.lifecycle
            .retire_source_declared_primary_anchor(relation, checked_primary_locus)
    }

    fn reacquire_source_declared_primary_anchor(
        &mut self,
        relation: &str,
    ) -> Sys4Result<M9AuthorityTransition> {
        self.synchronize_before_successor()?;
        self.lifecycle
            .reacquire_source_declared_primary_anchor(relation)
    }
}

pub(crate) struct M9AuthorityTransition {
    generation: M9AuthorityGeneration,
    sealed_m9_inspection: M9SealedTransitionInspection,
    prior_runtime_validation_observations: M9RuntimeValidationObservationSnapshot,
    prior_publisher: M9AuthoritySuccessorPublisher,
}

impl M9AuthorityTransition {
    pub(crate) fn sealed_m9_inspection(&self) -> &M9SealedTransitionInspection {
        &self.sealed_m9_inspection
    }

    /// Observer-safe M9 evidence for the exact pre-transition validation
    /// snapshot. It reveals neither counter keys nor authority material.
    pub(crate) fn prior_runtime_validation_observation_digest(&self) -> &str {
        self.prior_runtime_validation_observations.opaque_digest()
    }

    /// Observer-safe identity for the exact admitted M9 transition.  This is
    /// deliberately an M9 lifecycle reference, not a source/Core operation
    /// reference: the sealed consumer lineage is already M9-owned opaque
    /// material and does not disclose a credential or witness payload.
    pub(crate) fn observer_transition_ref(&self) -> Option<String> {
        let inspection = self.sealed_m9_inspection();
        let kind = match inspection.transition_kind() {
            M9AuthorityTransitionKind::DesignatedConsumerCapabilityRevoked => {
                "designated-consumer-capability-revoked"
            }
            M9AuthorityTransitionKind::DesignatedConsumerMembershipRetired => {
                "designated-consumer-membership-retired"
            }
            M9AuthorityTransitionKind::DesignatedConsumerWitnessRetired => {
                "designated-consumer-witness-retired"
            }
            M9AuthorityTransitionKind::DesignatedSourceReleaseRevoked => {
                return None;
            }
            M9AuthorityTransitionKind::SourceDeclaredMembershipRetired => {
                "source-declared-membership-retired"
            }
            M9AuthorityTransitionKind::SourceDeclaredPrimaryAnchorReacquired => {
                "source-declared-primary-anchor-reacquired"
            }
        };
        let sealed_lineage_ref = match inspection.transition_kind() {
            M9AuthorityTransitionKind::SourceDeclaredMembershipRetired => {
                inspection.membership_retirement().successor_tombstone_ref()
            }
            M9AuthorityTransitionKind::SourceDeclaredPrimaryAnchorReacquired => {
                inspection.membership_reacquire().fresh_membership_ref()
            }
            M9AuthorityTransitionKind::DesignatedSourceReleaseRevoked => unreachable!(),
            _ => inspection.consumer_lineage().opaque_lineage_ref(),
        };
        Some(format!(
            "m9-admitted-transition:{kind}:{sealed_lineage_ref}"
        ))
    }

    /// The successor M9 generation is sealed authority evidence and is kept
    /// separate from a source/Core provenance claim.
    pub(crate) fn observer_successor_generation_ref(&self) -> String {
        self.sealed_m9_inspection()
            .successor_generation()
            .generation_ref()
            .to_string()
    }

    fn matches_live_runtime_validation_observations(
        &self,
        generation: &M9AuthorityGeneration,
    ) -> bool {
        self.prior_runtime_validation_observations
            .matches_generation(generation)
    }
}

/// Immutable delivery bindings retained when E evaluates a designated value.
/// The first successful evaluator binding for an operation/publication pair is
/// the only binding that pair may have in this finite fabric.  A fixed-version
/// idempotent evaluation may emit another occurrence, but it cannot rewrite
/// the publication's already-established tick/frontier.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct EvaluatorPublicationBindingRegistry {
    entries: BTreeMap<(String, String), SealedDeliveryBinding>,
}

impl EvaluatorPublicationBindingRegistry {
    fn retain(&mut self, operation_id: &str, publication_id: &str, binding: SealedDeliveryBinding) {
        self.entries
            .entry((operation_id.to_string(), publication_id.to_string()))
            .or_insert(binding);
    }

    fn matches(
        &self,
        operation_id: &str,
        publication_id: &str,
        binding: &SealedDeliveryBinding,
    ) -> bool {
        self.entries
            .get(&(operation_id.to_string(), publication_id.to_string()))
            == Some(binding)
    }
}

/// The bounded OW1 profile requires exactly one logical worker locus.  This
/// pure projection-only derivation covers every currently admitted semantic
/// owner role; bootstrap remains fail-closed when the resulting set is empty
/// or contains more than one locus.
pub(crate) fn ow1_worker_locus_candidates(projection: &GlobalProjectionResult) -> BTreeSet<String> {
    let mut worker_loci = BTreeSet::new();
    for fragment in projection.sys4_artifact_fragments().entries() {
        if let Some(core) = fragment.owner_rmw_checked_core() {
            worker_loci.insert(core.owner_locus().to_string());
        }
        if let Some(dependency) = fragment.designated_remote_input_dependency() {
            worker_loci.insert(dependency.source_owner_locus().to_string());
        }
        if let Some(core) = fragment.relation_checked_core() {
            worker_loci.insert(core.owner_locus().to_string());
        }
    }
    worker_loci
}

pub(crate) struct LocalFabric {
    program: FabricProgram,
    loci: BTreeMap<String, LocusRuntime>,
    backend: M8ExecutionBackend,
    authority_generation: M9AuthorityGeneration,
    authority_lifecycle: M9AuthorityLifecycle,
    authority_live_floor: M9AuthorityLiveFloor,
    trace: FabricTrace,
    m8_trace: FabricM8Trace,
    actual_m8_trace: ActualM8Trace,
    m8_local_runtime_trace: M8LocalTrace,
    m8_trace_offsets: BTreeMap<String, usize>,
    m8_qualified_trace_nodes: BTreeMap<String, BTreeMap<String, String>>,
    // The backend trace may be one physical OW1 history, while this fabric
    // trace is a semantic per-locus projection.  Keep the exact dependency
    // projection assigned when each raw row first becomes visible so later
    // handler outcomes cannot reintroduce worker-sequencing edges.
    m8_qualified_trace_dependencies: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    // Raw M8 node identity is scoped to a physical session. In OW1 a single
    // session serves multiple semantic loci, so retain the locus that caused
    // each actual row before projecting dependencies.
    m8_raw_node_loci: BTreeMap<String, BTreeMap<String, String>>,
    m8_locus_trace_sequences: BTreeMap<String, u64>,
    m8_locus_sessions: BTreeMap<String, String>,
    observer_snapshot_failures:
        BTreeMap<(String, ObserverSnapshotChannel), ObserverSnapshotFailure>,
    causality: CausalityGraph,
    next_endpoint_occurrence: u64,
    route_faults: BTreeSet<String>,
    in_transit_faults: InTransitFaults,
    completed_receipts: BTreeMap<String, FabricReceipt>,
    local_store_read_audits: BTreeMap<String, LocalStoreReadAudit>,
    consumption_state: DesignatedConsumptionState,
    // Evaluator-established, immutable publication facts.  This fabric state
    // is intentionally separate from C's consumption cache so a reordered
    // delivery cannot become valid merely because C has not seen its sibling
    // carrier yet.  Its owned, cloneable representation is suitable for a
    // later whole-fabric cut without consulting a latest M8 trace row.
    evaluator_publication_bindings: EvaluatorPublicationBindingRegistry,
    cache: BTreeMap<String, CachedDelivery>,
    relation_semantic_digests: BTreeMap<String, String>,
    /// A sealed M9 fresh-reacquire binding may activate one time in this
    /// finite profile.  The schedule never holds the binding itself.
    used_fresh_relation_bindings: BTreeSet<String>,
    next_request: u64,
    patch_generation: u64,
    patch_lifecycle: Sys4PatchLifecycleLog,
}

impl std::fmt::Debug for LocalFabric {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocalFabric")
            .field("program", self.program.checked_program_identity())
            .field("loci", &self.loci.keys().collect::<Vec<_>>())
            .field(
                "authority_generation",
                &self.authority_generation.generation(),
            )
            .finish_non_exhaustive()
    }
}

/// Exact endpoint evidence for one generated relation publication.  It keeps
/// the SYS-4 transport stages and the M8-imported consumer shadow together so
/// SYS-5 devtools do not reconstruct causality by joining unrelated logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4RelationEndpointReceipt {
    request_id: String,
    owner_publish_occurrence_id: String,
    request_enqueue_occurrence_id: String,
    transport: TransportStep,
    consumer_observe_occurrence_id: String,
    consumer_serve_occurrence_id: String,
    edge: CommunicationEdge,
    shadow: M8ObservedRelationShadow,
    fresh_reacquire: Option<Sys4FreshReacquireEvidence>,
}

/// Observer-safe evidence for a source-derived fresh primary reacquire.  It
/// records the lifecycle causality and opaque M9 successor lineages but never
/// exposes a credential, capability, witness, or membership payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4FreshReacquireEvidence {
    source_derived: bool,
    lifecycle_request_identity: String,
    lifecycle_enqueue_occurrence_id: String,
    m9_reacquire_occurrence_id: String,
    lifecycle_receipt_occurrence_id: String,
    checked_primary_anchor_ref: String,
    prior_membership_ref: String,
    prior_membership_epoch_ref: String,
    prior_incarnation_ref: String,
    retired_membership_ref: String,
    retired_membership_epoch_ref: String,
    retired_incarnation_ref: String,
    fresh_membership_ref: String,
    fresh_membership_epoch_ref: String,
    fresh_incarnation_ref: String,
    capability_lineage_ref: String,
    witness_lineage_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    m9_transition_ref: String,
}

impl Sys4FreshReacquireEvidence {
    pub(crate) const fn source_derived(&self) -> bool {
        self.source_derived
    }

    pub(crate) fn lifecycle_request_identity(&self) -> &str {
        &self.lifecycle_request_identity
    }

    pub(crate) fn lifecycle_enqueue_occurrence_id(&self) -> &str {
        &self.lifecycle_enqueue_occurrence_id
    }

    pub(crate) fn m9_reacquire_occurrence_id(&self) -> &str {
        &self.m9_reacquire_occurrence_id
    }

    pub(crate) fn lifecycle_receipt_occurrence_id(&self) -> &str {
        &self.lifecycle_receipt_occurrence_id
    }

    pub(crate) fn checked_primary_anchor_ref(&self) -> &str {
        &self.checked_primary_anchor_ref
    }

    pub(crate) fn prior_membership_ref(&self) -> &str {
        &self.prior_membership_ref
    }

    pub(crate) fn prior_membership_epoch_ref(&self) -> &str {
        &self.prior_membership_epoch_ref
    }

    pub(crate) fn prior_incarnation_ref(&self) -> &str {
        &self.prior_incarnation_ref
    }

    pub(crate) fn retired_membership_ref(&self) -> &str {
        &self.retired_membership_ref
    }

    pub(crate) fn retired_membership_epoch_ref(&self) -> &str {
        &self.retired_membership_epoch_ref
    }

    pub(crate) fn retired_incarnation_ref(&self) -> &str {
        &self.retired_incarnation_ref
    }

    pub(crate) fn fresh_membership_ref(&self) -> &str {
        &self.fresh_membership_ref
    }

    pub(crate) fn fresh_membership_epoch_ref(&self) -> &str {
        &self.fresh_membership_epoch_ref
    }

    pub(crate) fn fresh_incarnation_ref(&self) -> &str {
        &self.fresh_incarnation_ref
    }

    pub(crate) fn capability_lineage_ref(&self) -> &str {
        &self.capability_lineage_ref
    }

    pub(crate) fn witness_lineage_ref(&self) -> &str {
        &self.witness_lineage_ref
    }

    pub(crate) fn prior_generation_ref(&self) -> &str {
        &self.prior_generation_ref
    }

    pub(crate) fn successor_generation_ref(&self) -> &str {
        &self.successor_generation_ref
    }

    pub(crate) fn m9_transition_ref(&self) -> &str {
        &self.m9_transition_ref
    }
}

/// Observer-safe result of the finite source-derived participant leave path.
/// The M9 membership transition is deliberately separate from the B-owned
/// relation operation: the receipt proves their causal order without exposing
/// membership, capability, witness, or payload values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys4ParticipantLeaveReceipt {
    relation: String,
    participant_locus: String,
    lifecycle_request_identity: String,
    lifecycle_enqueue_occurrence_id: String,
    m9_retire_occurrence_id: String,
    lifecycle_receipt_occurrence_id: String,
    checked_membership_identity_ref: String,
    prior_membership_ref: String,
    successor_tombstone_ref: String,
    membership_epoch_before_ref: String,
    membership_epoch_after_ref: String,
    incarnation_before_ref: String,
    incarnation_after_ref: String,
    capability_lineage_ref: String,
    witness_lineage_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    m9_transition_ref: String,
    relation_endpoint: Sys4RelationEndpointReceipt,
}

impl Sys4ParticipantLeaveReceipt {
    pub(crate) fn relation(&self) -> &str {
        &self.relation
    }

    pub(crate) fn participant_locus(&self) -> &str {
        &self.participant_locus
    }

    /// The identity of an admitted external lifecycle request.  This is not
    /// a Core operation identity or a transport carrier identity.
    pub(crate) fn lifecycle_request_identity(&self) -> &str {
        &self.lifecycle_request_identity
    }

    pub(crate) fn lifecycle_enqueue_occurrence_id(&self) -> &str {
        &self.lifecycle_enqueue_occurrence_id
    }

    pub(crate) fn m9_retire_occurrence_id(&self) -> &str {
        &self.m9_retire_occurrence_id
    }

    pub(crate) fn lifecycle_receipt_occurrence_id(&self) -> &str {
        &self.lifecycle_receipt_occurrence_id
    }

    pub(crate) fn checked_membership_identity_ref(&self) -> &str {
        &self.checked_membership_identity_ref
    }

    pub(crate) fn prior_membership_ref(&self) -> &str {
        &self.prior_membership_ref
    }

    pub(crate) fn successor_tombstone_ref(&self) -> &str {
        &self.successor_tombstone_ref
    }

    pub(crate) fn membership_epoch_before_ref(&self) -> &str {
        &self.membership_epoch_before_ref
    }

    pub(crate) fn membership_epoch_after_ref(&self) -> &str {
        &self.membership_epoch_after_ref
    }

    pub(crate) fn incarnation_before_ref(&self) -> &str {
        &self.incarnation_before_ref
    }

    pub(crate) fn incarnation_after_ref(&self) -> &str {
        &self.incarnation_after_ref
    }

    pub(crate) fn capability_lineage_ref(&self) -> &str {
        &self.capability_lineage_ref
    }

    pub(crate) fn witness_lineage_ref(&self) -> &str {
        &self.witness_lineage_ref
    }

    pub(crate) fn prior_generation_ref(&self) -> &str {
        &self.prior_generation_ref
    }

    pub(crate) fn successor_generation_ref(&self) -> &str {
        &self.successor_generation_ref
    }

    pub(crate) fn m9_transition_ref(&self) -> &str {
        &self.m9_transition_ref
    }

    pub(crate) fn relation_endpoint(&self) -> &Sys4RelationEndpointReceipt {
        &self.relation_endpoint
    }
}

impl Sys4RelationEndpointReceipt {
    pub(crate) fn request_id(&self) -> &str {
        &self.request_id
    }

    pub(crate) fn owner_publish_occurrence_id(&self) -> &str {
        &self.owner_publish_occurrence_id
    }

    pub(crate) fn fresh_reacquire(&self) -> Option<&Sys4FreshReacquireEvidence> {
        self.fresh_reacquire.as_ref()
    }

    /// The actual generated request occurrence at the source outbox.  The
    /// request identity remains distinct so a devtools view can show the
    /// causal publish -> request-enqueue edge without treating an identifier
    /// as an occurrence.
    pub(crate) fn request_enqueue_occurrence_id(&self) -> &str {
        &self.request_enqueue_occurrence_id
    }

    pub(crate) fn transport(&self) -> &TransportStep {
        &self.transport
    }

    pub(crate) fn consumer_observe_occurrence_id(&self) -> &str {
        &self.consumer_observe_occurrence_id
    }

    pub(crate) fn consumer_serve_occurrence_id(&self) -> &str {
        &self.consumer_serve_occurrence_id
    }

    pub(crate) fn edge(&self) -> &CommunicationEdge {
        &self.edge
    }

    pub(crate) fn shadow(&self) -> &M8ObservedRelationShadow {
        &self.shadow
    }
}

impl LocalFabric {
    /// Validate the full retained occurrence chain of one generated relation
    /// publication.  This is an observer-safe check over the receipt already
    /// produced by SYS-4; it neither re-evaluates the relation nor joins logs
    /// by operation name.  Every stage must remain a distinct actual
    /// occurrence and preserve the one checked endpoint provenance.
    pub(crate) fn observer_exact_relation_endpoint_receipt(
        &self,
        receipt: &Sys4RelationEndpointReceipt,
    ) -> bool {
        let edge = receipt.edge();
        let Some(endpoint) = self.observer_exact_endpoint_occurrences(
            receipt.request_id(),
            Sys4TraceKind::Dispatched,
            Sys4TraceKind::Received,
            CommunicationEdgeKind::RelationProjectionPublication,
            edge.source_locus(),
            edge.target_locus(),
        ) else {
            return false;
        };
        let occurrences = [
            receipt.owner_publish_occurrence_id(),
            receipt.request_enqueue_occurrence_id(),
            endpoint.dispatch_occurrence_id(),
            endpoint.receive_occurrence_id(),
            receipt.consumer_observe_occurrence_id(),
            receipt.consumer_serve_occurrence_id(),
        ];
        if receipt.request_id().is_empty()
            || occurrences.iter().any(|occurrence| occurrence.is_empty())
            || occurrences
                .iter()
                .any(|occurrence| *occurrence == receipt.request_id())
            || receipt.request_enqueue_occurrence_id() != endpoint.request_enqueue_occurrence_id()
            || receipt.transport().source_outbox_dequeue_occurrence_id()
                != endpoint.dispatch_occurrence_id()
            || receipt.transport().target_inbox_enqueue_occurrence_id()
                != endpoint.receive_occurrence_id()
            || endpoint.core_ref().is_empty()
            || endpoint.source_fragment_ref().is_empty()
            || endpoint.target_fragment_ref().is_empty()
            || endpoint.edge_ref() != edge.edge_ref()
            || endpoint.core_ref() != edge.core_ref().unwrap_or("")
            || endpoint.source_fragment_ref() != edge.source_fragment_ref()
            || endpoint.target_fragment_ref() != edge.target_fragment_ref()
            || endpoint.source_ref() != &edge.source_ref()
        {
            return false;
        }
        self.causality.reaches(
            receipt.request_enqueue_occurrence_id(),
            receipt.owner_publish_occurrence_id(),
        ) && self.causality.reaches(
            endpoint.dispatch_occurrence_id(),
            receipt.request_enqueue_occurrence_id(),
        ) && self.causality.reaches(
            endpoint.receive_occurrence_id(),
            endpoint.dispatch_occurrence_id(),
        ) && self.causality.reaches(
            receipt.consumer_observe_occurrence_id(),
            endpoint.receive_occurrence_id(),
        ) && self.causality.reaches(
            receipt.consumer_serve_occurrence_id(),
            receipt.consumer_observe_occurrence_id(),
        )
    }

    /// Dispatch the current M8 owner relation state through the exact
    /// projection-derived relation-publication endpoint.
    pub(crate) fn publish_relation_current(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        let edge = self.relation_publication_edge(relation)?;
        self.ensure_relation_dispatch_identifier_capacity()?;
        let authority = self.relation_publication_authority(&edge)?;
        let _target_admission = self.relation_publication_target_admission(&edge)?;
        let publication = self
            .backend
            .publish_relation(edge.source_locus(), relation, authority)
            .map_err(Sys4DispatchDiagnostics::one)?;
        let publication = self.qualify_owner_relation_publication(&edge, publication)?;
        self.dispatch_relation_publication(edge, publication)
    }

    /// The only local invalidation route consumes a live M9 relation use and
    /// mutates the owner M8 session before a new immutable publication is
    /// created.  It cannot be invoked with caller-supplied authority.
    pub(crate) fn invalidate_relation_primary(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        self.run_relation_transition_atomically(relation, Self::invalidate_relation_primary_staged)
    }

    /// Retire the membership at a relation's explicitly checked primary
    /// anchor, then have the relation owner degrade and publish the fallback.
    /// The schedule names only the checked relation: the participant locus,
    /// principal, M9 lineage, authority successor, and generated relation
    /// endpoint are all derived internally.  The bounded ST transition is
    /// failure-atomic; duplicate/stale membership, missing explicit anchor,
    /// and endpoint failure leave the live fabric unchanged.
    pub(crate) fn participant_leave_relation_primary(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4ParticipantLeaveReceipt> {
        if self.backend.profile() != BackendProfile::St {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::BackendIneligible,
            ));
        }
        self.run_participant_leave_transition_atomically(
            relation,
            Self::participant_leave_relation_primary_staged,
        )
    }

    fn participant_leave_relation_primary_staged(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4ParticipantLeaveReceipt> {
        let participant_locus = self.explicit_primary_anchor_locus(relation)?;
        self.ensure_participant_leave_identifier_capacity()?;

        let lifecycle_request_identity =
            self.next_request_id_with_prefix("sys4-external-lifecycle-request-")?;
        let lifecycle_enqueue_occurrence_id =
            self.next_mailbox_token("participant-leave-enqueue")?;
        self.causality
            .record(lifecycle_enqueue_occurrence_id.clone(), Vec::new());
        let transition = self
            .m9_authority_lifecycle_mut()
            .retire_source_declared_primary_anchor(relation, &participant_locus)?;
        let inspection = transition.sealed_m9_inspection();
        let retirement = inspection.membership_retirement().clone();
        let prior_generation_ref = inspection.prior_generation().generation_ref().to_string();
        let successor_generation_ref = inspection
            .successor_generation()
            .generation_ref()
            .to_string();
        let m9_transition_ref = transition.observer_transition_ref().ok_or_else(|| {
            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramAdmissionMismatch)
        })?;
        let m9_retire_occurrence_id = self.next_mailbox_token("participant-leave-retire")?;
        self.causality.record(
            m9_retire_occurrence_id.clone(),
            vec![lifecycle_enqueue_occurrence_id.clone()],
        );
        self.apply_admitted_authority_lifecycle(transition)?;
        let relation_endpoint = self.invalidate_relation_primary_staged(relation)?;
        self.causality.record(
            relation_endpoint.owner_publish_occurrence_id().to_string(),
            vec![m9_retire_occurrence_id.clone()],
        );
        if !self.causality.reaches(
            relation_endpoint.owner_publish_occurrence_id(),
            &m9_retire_occurrence_id,
        ) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        let lifecycle_receipt_occurrence_id =
            self.next_mailbox_token("participant-leave-receipt")?;
        self.causality.record(
            lifecycle_receipt_occurrence_id.clone(),
            vec![relation_endpoint.consumer_serve_occurrence_id().to_string()],
        );
        Ok(Sys4ParticipantLeaveReceipt {
            relation: relation.to_string(),
            participant_locus,
            lifecycle_request_identity,
            lifecycle_enqueue_occurrence_id,
            m9_retire_occurrence_id,
            lifecycle_receipt_occurrence_id,
            checked_membership_identity_ref: retirement
                .checked_membership_identity_ref()
                .to_string(),
            prior_membership_ref: retirement.prior_membership_ref().to_string(),
            successor_tombstone_ref: retirement.successor_tombstone_ref().to_string(),
            membership_epoch_before_ref: retirement.membership_epoch_before_ref().to_string(),
            membership_epoch_after_ref: retirement.membership_epoch_after_ref().to_string(),
            incarnation_before_ref: retirement.incarnation_before_ref().to_string(),
            incarnation_after_ref: retirement.incarnation_after_ref().to_string(),
            capability_lineage_ref: retirement.capability_lineage_ref().to_string(),
            witness_lineage_ref: retirement.witness_lineage_ref().to_string(),
            prior_generation_ref,
            successor_generation_ref,
            m9_transition_ref,
            relation_endpoint,
        })
    }

    /// Execute the owner-side invalidation on a cloneable ST candidate.  The
    /// generated relation endpoint is part of the same semantic transition:
    /// if its dispatch fails, no owner mutation, M8 occurrence, local carrier,
    /// or one-shot binding may become observable in the live fabric.
    fn invalidate_relation_primary_staged(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        let edge = self.relation_publication_edge(relation)?;
        self.ensure_relation_dispatch_identifier_capacity()?;
        let publish_authority = self.relation_publication_authority(&edge)?;
        let _target_admission = self.relation_publication_target_admission(&edge)?;
        let authority = self
            .authority_generation
            .relation_authority_use(relation, "invalidate_primary")
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))?;
        self.backend
            .invalidate_relation(edge.source_locus(), relation, authority)
            .map_err(Sys4DispatchDiagnostics::one)?;
        let publication = self
            .backend
            .publish_relation(edge.source_locus(), relation, publish_authority)
            .map_err(Sys4DispatchDiagnostics::one)?;
        let publication = self.qualify_owner_relation_publication(&edge, publication)?;
        self.dispatch_relation_publication(edge, publication)
    }

    /// Activate exactly one dormant M9-sealed fresh binding, then re-acquire
    /// the owner relation's primary anchor and publish the new lineage.
    pub(crate) fn fresh_reacquire_relation_primary(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        self.run_relation_transition_atomically(
            relation,
            Self::fresh_reacquire_relation_primary_staged,
        )
    }

    /// Activate a fresh relation binding only in a candidate ST fabric.  If a
    /// source-declared primary was retired, the absent binding is restored
    /// only by a new M9-issued primary membership/incarnation and its sealed
    /// checked scope inventory; ordinary legacy bindings retain their M10
    /// behavior.  The binding is consumed when, and only when, its derived
    /// publication has crossed the generated endpoint successfully.
    fn fresh_reacquire_relation_primary_staged(
        &mut self,
        relation: &str,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        if self.used_fresh_relation_bindings.contains(relation) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::M8ExecutionRejected,
            ));
        }
        let edge = self.relation_publication_edge(relation)?;
        if !self
            .backend
            .relation_requires_fresh_reacquire(edge.source_locus(), relation)
            .map_err(Sys4DispatchDiagnostics::one)?
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::M8ExecutionRejected,
            ));
        }
        let mut fresh_reacquire = None;
        if self
            .authority_generation
            .fresh_relation_reacquire_binding(relation)
            .is_none()
        {
            self.ensure_fresh_anchor_reacquire_identifier_capacity()?;
            let lifecycle_request_identity =
                self.next_request_id_with_prefix("sys4-external-lifecycle-request-")?;
            let lifecycle_enqueue_occurrence_id =
                self.next_mailbox_token("primary-reacquire-enqueue")?;
            self.causality
                .record(lifecycle_enqueue_occurrence_id.clone(), Vec::new());
            let transition = self
                .m9_authority_lifecycle_mut()
                .reacquire_source_declared_primary_anchor(relation)?;
            let inspection = transition.sealed_m9_inspection();
            let reacquire_lineage = inspection.membership_reacquire().clone();
            let prior_generation_ref = inspection.prior_generation().generation_ref().to_string();
            let successor_generation_ref = inspection
                .successor_generation()
                .generation_ref()
                .to_string();
            let m9_transition_ref = transition.observer_transition_ref().ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramAdmissionMismatch)
            })?;
            let m9_reacquire_occurrence_id = self.next_mailbox_token("primary-reacquire-m9")?;
            self.causality.record(
                m9_reacquire_occurrence_id.clone(),
                vec![lifecycle_enqueue_occurrence_id.clone()],
            );
            self.apply_admitted_authority_lifecycle(transition)?;
            fresh_reacquire = Some((
                lifecycle_request_identity,
                lifecycle_enqueue_occurrence_id,
                m9_reacquire_occurrence_id,
                reacquire_lineage,
                prior_generation_ref,
                successor_generation_ref,
                m9_transition_ref,
            ));
        } else {
            self.ensure_relation_dispatch_identifier_capacity()?;
        }
        let publish_authority = self.relation_publication_authority(&edge)?;
        let _target_admission = self.relation_publication_target_admission(&edge)?;
        let binding = self
            .authority_generation
            .fresh_relation_reacquire_binding(relation)
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))?;
        let (authority, reacquire, lease) = binding.m8_activation_material();
        self.backend
            .install_fresh_relation_lease(edge.source_locus(), relation, lease)
            .map_err(Sys4DispatchDiagnostics::one)?;
        self.backend
            .reacquire_relation(edge.source_locus(), relation, authority, reacquire)
            .map_err(Sys4DispatchDiagnostics::one)?;
        self.used_fresh_relation_bindings
            .insert(relation.to_string());
        let publication = self
            .backend
            .publish_relation(edge.source_locus(), relation, publish_authority)
            .map_err(Sys4DispatchDiagnostics::one)?;
        let publication = self.qualify_owner_relation_publication(&edge, publication)?;
        let mut receipt = self.dispatch_relation_publication(edge, publication)?;
        if let Some((
            lifecycle_request_identity,
            lifecycle_enqueue_occurrence_id,
            m9_reacquire_occurrence_id,
            reacquire_lineage,
            prior_generation_ref,
            successor_generation_ref,
            m9_transition_ref,
        )) = fresh_reacquire
        {
            self.causality.record(
                receipt.owner_publish_occurrence_id().to_string(),
                vec![m9_reacquire_occurrence_id.clone()],
            );
            if !self.causality.reaches(
                receipt.owner_publish_occurrence_id(),
                &m9_reacquire_occurrence_id,
            ) {
                return Err(Sys4DispatchDiagnostics::one(
                    Sys4DiagnosticKind::ProgramAdmissionMismatch,
                ));
            }
            let lifecycle_receipt_occurrence_id =
                self.next_mailbox_token("primary-reacquire-receipt")?;
            self.causality.record(
                lifecycle_receipt_occurrence_id.clone(),
                vec![receipt.consumer_serve_occurrence_id().to_string()],
            );
            receipt.fresh_reacquire = Some(Sys4FreshReacquireEvidence {
                source_derived: true,
                lifecycle_request_identity,
                lifecycle_enqueue_occurrence_id,
                m9_reacquire_occurrence_id,
                lifecycle_receipt_occurrence_id,
                checked_primary_anchor_ref: reacquire_lineage
                    .checked_primary_anchor_ref()
                    .to_string(),
                prior_membership_ref: reacquire_lineage.prior_membership_ref().to_string(),
                prior_membership_epoch_ref: reacquire_lineage
                    .prior_membership_epoch_ref()
                    .to_string(),
                prior_incarnation_ref: reacquire_lineage.prior_incarnation_ref().to_string(),
                retired_membership_ref: reacquire_lineage.retired_membership_ref().to_string(),
                retired_membership_epoch_ref: reacquire_lineage
                    .retired_membership_epoch_ref()
                    .to_string(),
                retired_incarnation_ref: reacquire_lineage.retired_incarnation_ref().to_string(),
                fresh_membership_ref: reacquire_lineage.fresh_membership_ref().to_string(),
                fresh_membership_epoch_ref: reacquire_lineage
                    .fresh_membership_epoch_ref()
                    .to_string(),
                fresh_incarnation_ref: reacquire_lineage.fresh_incarnation_ref().to_string(),
                capability_lineage_ref: reacquire_lineage.capability_lineage_ref().to_string(),
                witness_lineage_ref: reacquire_lineage.witness_lineage_ref().to_string(),
                prior_generation_ref,
                successor_generation_ref,
                m9_transition_ref,
            });
        }
        Ok(receipt)
    }

    /// Relation invalidation and fresh-reacquire are one semantic operation
    /// together with their required generated publication.  ST owns all of
    /// the affected M8 state locally, so stage that whole operation on a
    /// cloned candidate and publish the candidate only after endpoint
    /// dispatch succeeds.  In OW1 relation operations are not admitted by
    /// this finite backend; its transition returns before any M8 mutation, so
    /// retaining the direct path preserves that fail-closed behavior without
    /// pretending a worker snapshot exists.
    fn run_relation_transition_atomically(
        &mut self,
        relation: &str,
        transition: fn(&mut Self, &str) -> Sys4Result<Sys4RelationEndpointReceipt>,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        if self.backend.profile() != BackendProfile::St {
            return transition(self, relation);
        }

        let mut candidate = self
            .clone_for_checked_patch()
            .map_err(Sys4DispatchDiagnostics::one)?;
        let receipt = transition(&mut candidate, relation)?;
        self.commit_staged_authority_candidate(candidate)?;
        Ok(receipt)
    }

    /// The membership retirement must be in the same cloneable ST candidate
    /// as the relation owner's invalidation/publication.  M9 advances only
    /// inside that candidate; if routing or publication fails, dropping it
    /// also drops the uninstalled successor publisher and leaves the live
    /// membership lineage untouched.
    fn run_participant_leave_transition_atomically(
        &mut self,
        relation: &str,
        transition: fn(&mut Self, &str) -> Sys4Result<Sys4ParticipantLeaveReceipt>,
    ) -> Sys4Result<Sys4ParticipantLeaveReceipt> {
        let mut candidate = self
            .clone_for_checked_patch()
            .map_err(Sys4DispatchDiagnostics::one)?;
        let receipt = transition(&mut candidate, relation)?;
        self.commit_staged_authority_candidate(candidate)?;
        Ok(receipt)
    }

    /// Complete the second phase of a staged relation/lifecycle operation.
    /// Candidate execution uses a private floor; only after generated
    /// dispatch has succeeded do we validate and advance the canonical floor
    /// shared by live fabrics, then install that same floor into the winning
    /// candidate.  Any error leaves `self` untouched.
    fn commit_staged_authority_candidate(&mut self, mut candidate: Self) -> Sys4Result<()> {
        let live_floor = self.authority_live_floor.clone();
        // The candidate may preserve this fabric's numeric generation (for
        // example, an M8-only relation fallback).  It still cannot install
        // after a sibling sharing this floor has advanced M9 while the
        // candidate endpoint work was in flight. Hold the exact matching
        // guard through either install path.
        let Some(mut floor_guard) = live_floor.guard_matching(&self.authority_generation) else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        };
        if candidate
            .authority_generation
            .matches_for_restore(&self.authority_generation)
        {
            candidate.authority_live_floor = live_floor.clone();
            *self = candidate;
            return Ok(());
        }

        if !floor_guard
            .accepts_successor(&self.authority_generation, &candidate.authority_generation)
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        floor_guard.commit_successor(&candidate.authority_generation);
        candidate.authority_live_floor = live_floor.clone();
        *self = candidate;
        Ok(())
    }

    pub(crate) fn relation_imported_shadow(
        &self,
        relation: &str,
        consumer_locus: &str,
    ) -> Sys4Result<Option<M8ObservedRelationShadow>> {
        self.backend
            .relation_shadow(consumer_locus, relation)
            .map_err(Sys4DispatchDiagnostics::one)
    }

    /// Execute the M8 consumer-local presentation fallback against the
    /// imported shadow only.  It adds no endpoint carrier and cannot mutate
    /// owner semantic relation state.
    pub(crate) fn project_relation_presentation_gap(
        &self,
        relation: &str,
    ) -> Sys4Result<M8RelationProjection> {
        let edge = self.relation_publication_edge(relation)?;
        let shadow = self
            .backend
            .relation_shadow(edge.target_locus(), relation)
            .map_err(Sys4DispatchDiagnostics::one)?
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))?;
        let subject = self.relation_projection_subject(relation)?;
        let context = M8PresentationContext::for_consumer(edge.target_locus())
            .with_frontier(shadow.semantic().activation_frontier())
            .with_presentation_fallback(
                M8PresentationFallback::hold_last_local(subject, M8Point::new(0, 0))
                    .with_policy(M8RestrictionPolicy::Restricted),
            );
        self.backend
            .project_relation_shadow(edge.target_locus(), relation, context)
            .map_err(Sys4DispatchDiagnostics::one)
    }

    pub(crate) fn relation_semantic_digest(&self, relation: &str) -> Option<&str> {
        self.relation_semantic_digests
            .get(relation)
            .map(String::as_str)
    }

    pub(crate) fn endpoint_carrier_count_for_relation(&self, relation: &str) -> usize {
        self.program
            .projection
            .communication_plan()
            .edges()
            .iter()
            .filter(|edge| {
                edge.operation_id() == relation
                    && edge.kind() == CommunicationEdgeKind::RelationProjectionPublication
            })
            .map(|edge| {
                self.loci.get(edge.source_locus()).map_or(0, |runtime| {
                    runtime
                        .outgoing_endpoint
                        .records
                        .iter()
                        .filter(|record| record.edge_ref == edge.edge_ref())
                        .count()
                })
            })
            .sum()
    }

    pub(crate) fn total_endpoint_carrier_count(&self) -> usize {
        self.loci
            .values()
            .map(|runtime| {
                runtime.outgoing_endpoint.carrier_history_len()
                    + runtime.incoming_endpoint.carrier_history_len()
            })
            .sum()
    }

    /// A one-shot fresh relation binding is semantic state owned by the
    /// fabric, not by the SYS-5 schedule.  This narrow observer is used only
    /// to classify a rejected repeat request before it can enter an endpoint.
    pub(crate) fn relation_fresh_binding_is_consumed(&self, relation: &str) -> bool {
        self.used_fresh_relation_bindings.contains(relation)
    }

    fn relation_publication_edge(&self, relation: &str) -> Sys4Result<CommunicationEdge> {
        let fragments = self.program.projection.sys4_artifact_fragments();
        let fragment = fragments
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == relation
                    && fragment.fragment_kind()
                        == ProjectedOperationFragmentKind::RelationPublication
            })
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable))?;
        let core = fragment.relation_checked_core().ok_or_else(|| {
            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramProjectionMismatch)
        })?;
        let consumer = core
            .consumer_projection_locus()
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable))?;
        self.edge_for(
            relation,
            CommunicationEdgeKind::RelationProjectionPublication,
            core.owner_locus(),
            consumer,
        )
    }

    fn relation_projection_subject(&self, relation: &str) -> Sys4Result<String> {
        let fragments = self.program.projection.sys4_artifact_fragments();
        fragments
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == relation
                    && fragment.fragment_kind()
                        == ProjectedOperationFragmentKind::RelationPublication
            })
            .and_then(|fragment| fragment.relation_checked_core())
            .map(|core| core.subject().to_string())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable))
    }

    /// A participant leave is admitted only where checked Core carries an
    /// explicit primary-anchor locus.  Legacy anchors without a locus cannot
    /// be inferred from names or topology, and therefore fail closed.
    fn explicit_primary_anchor_locus(&self, relation: &str) -> Sys4Result<String> {
        let fragments = self.program.projection.sys4_artifact_fragments();
        let core = fragments
            .entries()
            .iter()
            .find(|fragment| {
                fragment.operation_id() == relation
                    && fragment.fragment_kind()
                        == ProjectedOperationFragmentKind::RelationPublication
            })
            .and_then(|fragment| fragment.relation_checked_core())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable))?;
        let locus = core.primary().anchor_locus().ok_or_else(|| {
            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramProjectionMismatch)
        })?;
        if locus.is_empty() || !self.loci.contains_key(locus) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramProjectionMismatch,
            ));
        }
        Ok(locus.to_string())
    }

    fn relation_publication_authority(
        &self,
        edge: &CommunicationEdge,
    ) -> Sys4Result<M8RelationAuthorityUse> {
        self.authority_generation
            .relation_authority_use(edge.operation_id(), "publish_relation")
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))
    }

    /// Check all identifiers used by one relation publication before an M8
    /// owner invalidation/reacquire/publication can mutate semantic state.
    /// The ST profile has no interleaving point inside this dispatch path, so
    /// this finite reservation is exact without introducing a separate
    /// mutable reservation carrier.
    fn ensure_relation_dispatch_identifier_capacity(&self) -> Sys4Result<()> {
        self.next_request
            .checked_add(1)
            .and(
                self.next_endpoint_occurrence
                    .checked_add(RELATION_DISPATCH_ENDPOINT_OCCURRENCES),
            )
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::IdentifierExhausted))
            .map(|_| ())
    }

    /// Reserve the leave occurrence plus its one generated relation
    /// publication before M9 can retire the member in the ST candidate.
    fn ensure_participant_leave_identifier_capacity(&self) -> Sys4Result<()> {
        self.next_request
            .checked_add(2)
            .and(
                self.next_endpoint_occurrence
                    .checked_add(RELATION_DISPATCH_ENDPOINT_OCCURRENCES + 3),
            )
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::IdentifierExhausted))
            .map(|_| ())
    }

    /// A source-declared primary reacquire adds one external lifecycle
    /// identity and three retained lifecycle occurrences to its generated
    /// relation publication.  Reserve the complete ST transition before M9
    /// issues the fresh membership so exhaustion remains failure-atomic.
    fn ensure_fresh_anchor_reacquire_identifier_capacity(&self) -> Sys4Result<()> {
        self.next_request
            .checked_add(2)
            .and(
                self.next_endpoint_occurrence
                    .checked_add(RELATION_DISPATCH_ENDPOINT_OCCURRENCES + 3),
            )
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::IdentifierExhausted))
            .map(|_| ())
    }

    fn relation_publication_target_admission(
        &self,
        edge: &CommunicationEdge,
    ) -> Sys4Result<M9RelationPublicationAdmission> {
        self.authority_generation
            .admit_relation_publication_target(
                edge.operation_id(),
                edge.source_locus(),
                edge.target_locus(),
            )
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))
    }

    /// Turn the owner M8-local publication occurrence into the fabric's
    /// qualified occurrence namespace before it becomes a generated request
    /// predecessor.  The source occurrence is still M8-owned; SYS-4 only
    /// associates it with the source-derived endpoint.
    fn qualify_owner_relation_publication(
        &mut self,
        edge: &CommunicationEdge,
        publication: M8PublishedRelationState,
    ) -> Sys4Result<M8PublishedRelationState> {
        let raw = publication.owner_publish_occurrence_id().ok_or_else(|| {
            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::CarrierProvenanceMismatch)
        })?;
        self.refresh_m8_local_runtime_trace(edge.source_locus());
        let qualified = self.fabric_qualified_m8_node_for_locus(edge.source_locus(), raw);
        Ok(publication.with_owner_publish_occurrence_id(qualified))
    }

    fn dispatch_relation_publication(
        &mut self,
        edge: CommunicationEdge,
        publication: M8PublishedRelationState,
    ) -> Sys4Result<Sys4RelationEndpointReceipt> {
        if publication.relation() != edge.operation_id()
            || publication.owner_locus() != edge.source_locus()
            || SourceRefView::new(publication.source_ref()) != edge.source_ref()
            || publication.owner_publish_occurrence_id().is_none()
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::CarrierProvenanceMismatch,
            ));
        }
        let target_admission = self.relation_publication_target_admission(&edge)?;
        let owner_publish_occurrence = publication
            .owner_publish_occurrence_id()
            .expect("publication was checked for an owner occurrence")
            .to_string();
        let request_id = self.next_relation_request_id()?;
        let envelope = self.enqueue_outbox(
            &edge,
            &request_id,
            MailboxPayload::RelationPublication {
                publication: Box::new(publication),
                target_admission,
            },
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![owner_publish_occurrence.clone()],
        )?;
        let request_enqueue_occurrence_id = envelope.mailbox_enqueue_occurrence_id.clone();
        let transport = match self.step_transport(
            edge.source_locus(),
            edge.target_locus(),
            envelope.envelope_id(),
        ) {
            Ok(transport) => transport,
            Err(mut diagnostics) => {
                // The owner publication has not committed its sequence yet.
                // Remove this undelivered attempt so a later retry can use
                // the same immutable semantic publication rather than leave
                // a duplicate stale carrier in the source mailbox.
                let discarded = self.discard_undelivered_relation_publication(
                    edge.source_locus(),
                    envelope.envelope_id(),
                );
                diagnostics.context.relation_publication_failure_disposition = Some(if discarded {
                    RelationPublicationFailureDisposition::DiscardedUndelivered
                } else {
                    RelationPublicationFailureDisposition::AlreadyRemovedByTransport
                });
                return Err(diagnostics);
            }
        };
        let (received, locus_dequeue) = self.dequeue_locus(edge.target_locus())?;
        let (publication, target_admission) = match &received.payload {
            MailboxPayload::RelationPublication {
                publication,
                target_admission,
            } => ((**publication).clone(), target_admission.clone()),
            _ => {
                return Err(self.quarantine(
                    edge.target_locus(),
                    &received,
                    Sys4DiagnosticKind::CarrierProvenanceMismatch,
                    &request_id,
                ));
            }
        };
        if received.edge_kind != CommunicationEdgeKind::RelationProjectionPublication
            || received.edge_ref != edge.edge_ref()
            || received.source_ref != edge.source_ref()
            || received.core_ref != edge.core_ref().map(ToOwned::to_owned)
            || received.source_fragment_ref != *edge.source_fragment_ref()
            || received.target_fragment_ref != *edge.target_fragment_ref()
        {
            return Err(self.quarantine(
                edge.target_locus(),
                &received,
                Sys4DiagnosticKind::CarrierProvenanceMismatch,
                &request_id,
            ));
        }
        if target_admission.relation() != edge.operation_id()
            || target_admission.owner_locus() != edge.source_locus()
            || target_admission.consumer_locus() != edge.target_locus()
            || !self
                .authority_generation
                .revalidate_relation_publication_target(&target_admission)
        {
            return Err(self.quarantine(
                edge.target_locus(),
                &received,
                Sys4DiagnosticKind::M8ExecutionRejected,
                &request_id,
            ));
        }
        let publication_for_commit = publication.clone();
        let shadow = self
            .backend
            .import_relation_shadow(edge.target_locus(), publication)
            .map_err(|kind| self.quarantine(edge.target_locus(), &received, kind, &request_id))?;
        if shadow.relation() != edge.operation_id()
            || shadow.owner_locus() != edge.source_locus()
            || shadow.consumer_locus() != edge.target_locus()
            || SourceRefView::new(shadow.source_ref()) != edge.source_ref()
            || shadow.core_ref().is_empty()
        {
            return Err(self.quarantine(
                edge.target_locus(),
                &received,
                Sys4DiagnosticKind::CarrierProvenanceMismatch,
                &request_id,
            ));
        }
        self.refresh_m8_local_runtime_trace(edge.target_locus());
        let raw_observe = shadow
            .consumer_observe_occurrence_id()
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))?;
        let observe = self.fabric_qualified_m8_node_for_locus(edge.target_locus(), raw_observe);
        let shadow = self
            .backend
            .qualify_relation_shadow_observe_occurrence(edge.target_locus(), &shadow, &observe)
            .map_err(Sys4DispatchDiagnostics::one)?;
        self.causality
            .record(observe.clone(), vec![locus_dequeue.clone()]);
        // This endpoint completion is a retained SYS-4 occurrence.  Do not
        // manufacture a SYS-5 label from a request or publication identity:
        // the observer chain must continue from the M8 consumer observation
        // through a real local-fabric serve occurrence.
        let serve = self.next_mailbox_token("relation-serve")?;
        self.causality.record(serve.clone(), vec![observe.clone()]);
        let receipt = Sys4RelationEndpointReceipt {
            request_id,
            owner_publish_occurrence_id: owner_publish_occurrence,
            request_enqueue_occurrence_id,
            transport,
            consumer_observe_occurrence_id: observe,
            consumer_serve_occurrence_id: serve,
            edge,
            shadow,
            fresh_reacquire: None,
        };
        if !self.observer_exact_relation_endpoint_receipt(&receipt) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::CarrierProvenanceMismatch,
            ));
        }
        self.backend
            .commit_relation_publication(receipt.edge().source_locus(), &publication_for_commit)
            .map_err(Sys4DispatchDiagnostics::one)?;
        self.relation_semantic_digests.insert(
            receipt.edge().operation_id().to_string(),
            receipt.shadow().semantic_digest(),
        );
        Ok(receipt)
    }

    /// An endpoint failure before target enqueue leaves no committed relation
    /// publication.  The exact envelope is only an unaccepted attempt, so
    /// it must not survive beside the retry of the same sequence.
    fn discard_undelivered_relation_publication(
        &mut self,
        source_locus: &str,
        envelope_id: &str,
    ) -> bool {
        let Some(runtime) = self.loci.get_mut(source_locus) else {
            return false;
        };
        let Some(position) = runtime.outgoing_mailbox.pending.iter().position(|entry| {
            entry.envelope_id == envelope_id
                && matches!(entry.payload, MailboxPayload::RelationPublication { .. })
        }) else {
            return false;
        };
        let _ = runtime.outgoing_mailbox.pending.remove(position);
        true
    }

    pub(crate) fn bootstrap(
        program: FabricProgram,
        admission: SealedFabricAdmission,
        backend_profile: BackendProfile,
    ) -> Sys4Result<Self> {
        if admission.program_identity != *program.checked_program_identity()
            || admission.program_fingerprint != program.projected_fingerprint()
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramProjectionMismatch,
            ));
        }
        if let BackendEligibility::Ineligible { reason } =
            program.backend_eligibility(backend_profile)
        {
            let mut diagnostics =
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::BackendIneligible);
            diagnostics.context.backend_ineligibility_reason = Some(reason);
            return Err(diagnostics);
        }
        let mut loci = BTreeMap::new();
        for locus in program.locus_names() {
            let artifact = fabric_artifact_for(program.projection.locus_program(&locus));
            loci.insert(
                locus.clone(),
                LocusRuntime {
                    local_store: LocusLocalStore::owned(locus.clone()),
                    locus,
                    program_identity: program.checked_program_identity().clone(),
                    artifact,
                    incoming_endpoint: EndpointCarrierHistory::default(),
                    outgoing_endpoint: EndpointCarrierHistory::default(),
                    incoming_mailbox: IncomingMailbox::default(),
                    outgoing_mailbox: OutgoingMailbox::default(),
                },
            );
        }
        for ((locus, state, index, field), value) in &admission.initial_state_seed.ints {
            loci.get_mut(locus)
                .expect("validated SYS-4 seed locus remains present")
                .local_store
                .set_int(state, index, field, *value);
        }
        let session_for_locus = |locus: &str| {
            let mut seed = M8LocalRuntimeSeed::new()
                .with_authority_state(admission.authority_generation.authority_state());
            for ((seed_locus, state, index, field), value) in &admission.initial_state_seed.ints {
                if seed_locus == locus {
                    seed =
                        seed.with_owner_int(M8StateKey::indexed_field(state, index, field), *value);
                }
            }
            // A designated evaluator receives source-owner inputs only after
            // the generated receipt carrier is dequeued. Boot never mints one
            // from a seed, even when this locus owns unrelated state.
            seed = seed.with_designated_input_receipts(M8InputReceiptSet::new());
            M8LocalRuntime::from_admitted(admission.instance.clone(), seed)
        };
        let mut backend = match backend_profile {
            BackendProfile::St => M8ExecutionBackend::St(
                program
                    .locus_names()
                    .into_iter()
                    .map(|locus| {
                        let runtime = session_for_locus(&locus);
                        (locus, Box::new(runtime))
                    })
                    .collect(),
            ),
            BackendProfile::Ow1 => {
                // SYS-3 has already admitted OW1 only when one logical locus
                // combines semantic owner and designated source-owner duties.
                // Recover that unique locus from either generated owner-RMW or
                // generated designated-source fragments; do not require a
                // synthetic owner RMW in an otherwise designated-only fabric.
                let worker_loci = ow1_worker_locus_candidates(&program.projection);
                let owner = (worker_loci.len() == 1)
                    .then(|| {
                        crate::semantic_runtime_kernel::LocusRef::new(
                            worker_loci
                                .into_iter()
                                .next()
                                .expect("one validated OW1 worker locus"),
                        )
                    })
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::BackendIneligible)
                    })?;
                let runtime = session_for_locus(owner.as_str());
                M8ExecutionBackend::Ow1(Ow1WorkerBackend::spawn(owner, runtime))
            }
        };
        // The finite local relation fallback chain is constructed inside the
        // owner M8 session from its admitted plan and M9-derived bootstrap
        // lease.  No schedule or SYS-5 caller provides chain material.
        for fragment in program.projection.sys4_artifact_fragments().entries() {
            if fragment.fragment_kind() != ProjectedOperationFragmentKind::RelationPublication {
                continue;
            }
            let relation = fragment
                .relation_checked_core()
                .expect("relation publication fragment retains checked Core");
            backend
                .install_relation_bootstrap(relation.owner_locus(), fragment.operation_id())
                .map_err(Sys4DispatchDiagnostics::one)?;
        }
        let local_store_read_audits = program
            .locus_names()
            .into_iter()
            .map(|locus| {
                (
                    locus,
                    LocalStoreReadAudit {
                        occurrence_id: "sys4-read-audit:empty".to_string(),
                        reads: Vec::new(),
                    },
                )
            })
            .collect();
        Ok(Self {
            program,
            loci,
            backend,
            authority_generation: admission.authority_generation,
            authority_lifecycle: M9AuthorityLifecycle {
                publisher: admission.authority_successor,
            },
            authority_live_floor: admission.authority_live_floor,
            trace: FabricTrace::default(),
            m8_trace: FabricM8Trace::default(),
            actual_m8_trace: ActualM8Trace::default(),
            m8_local_runtime_trace: M8LocalTrace::default(),
            m8_trace_offsets: BTreeMap::new(),
            m8_qualified_trace_nodes: BTreeMap::new(),
            m8_qualified_trace_dependencies: BTreeMap::new(),
            m8_raw_node_loci: BTreeMap::new(),
            m8_locus_trace_sequences: BTreeMap::new(),
            m8_locus_sessions: BTreeMap::new(),
            observer_snapshot_failures: BTreeMap::new(),
            causality: CausalityGraph::default(),
            next_endpoint_occurrence: 0,
            route_faults: BTreeSet::new(),
            in_transit_faults: InTransitFaults::default(),
            completed_receipts: BTreeMap::new(),
            local_store_read_audits,
            consumption_state: DesignatedConsumptionState::default(),
            evaluator_publication_bindings: EvaluatorPublicationBindingRegistry::default(),
            cache: BTreeMap::new(),
            relation_semantic_digests: BTreeMap::new(),
            used_fresh_relation_bindings: BTreeSet::new(),
            next_request: 0,
            patch_generation: 0,
            patch_lifecycle: Sys4PatchLifecycleLog::default(),
        })
    }

    pub(crate) fn locus_names(&self) -> Vec<String> {
        self.loci.keys().cloned().collect()
    }

    /// Crate-private access to the active projection only for construction of
    /// a checked patch candidate.  The caller cannot mutate it or inject a
    /// route, Core, authority, or activation frontier.
    pub(crate) fn active_program_for_checked_patch(&self) -> &FabricProgram {
        &self.program
    }
    pub(crate) fn locus_runtime(&self, locus: &str) -> Option<&LocusRuntime> {
        self.loci.get(locus)
    }
    pub(crate) fn semantic_snapshot(&self) -> FabricSemanticSnapshot {
        FabricSemanticSnapshot {
            loci: self
                .loci
                .iter()
                .map(|(locus, runtime)| (locus.clone(), runtime.local_store.clone()))
                .collect(),
        }
    }

    /// Activate an opaque candidate built by the checked-source → projection
    /// → M9-admission pipeline. All checks and plan replacement run against
    /// a clone-only candidate; rejected candidates mutate only lifecycle
    /// evidence on this fabric.
    pub(crate) fn activate_checked_patch(
        &mut self,
        candidate: Sys4CheckedPatchCandidate,
    ) -> Sys4Result<Sys4PatchOutcome> {
        let active_frontier = self.current_patch_frontier();
        let boundary_inspection = Sys4PatchBoundaryInspection {
            candidate_was_prechecked_projected_and_m9_admitted: true,
        };
        // Static compatibility is checked before authority binding so a
        // topology/owner-route mismatch remains a topology diagnostic rather
        // than being masked by an independently admitted candidate inventory.
        if !candidate.compatibility.matches() {
            return Ok(self.reject_checked_patch(
                &candidate,
                active_frontier,
                candidate.compatibility.diagnostic(),
                boundary_inspection,
            ));
        }
        if !candidate
            .base_frontier
            .has_same_program_projection_and_activation(&active_frontier)
            || !candidate.base_frontier.has_well_formed_nonce()
        {
            return Ok(self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::StaleFrontier,
                boundary_inspection,
            ));
        }
        // The active floor is shared between fabrics that began from the same
        // admitted M9 seam.  Keep its guard through the clone/preflight/swap;
        // a candidate may prove equivalence but can never replace that floor.
        let live_floor = self.authority_live_floor.clone();
        let Some(mut floor_guard) = live_floor.guard_matching(&self.authority_generation) else {
            let recheck = M9LiveFloorRecheckInspection {
                current_generation: self.current_m9_authority_inspection().generation(),
                shared_live_floor_identity: live_floor.identity_snapshot(),
            };
            let mut outcome = self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::StaleFrontier,
                boundary_inspection,
            );
            outcome.m9_live_floor_recheck = Some(recheck);
            return Ok(outcome);
        };
        let floor_generation = floor_guard.current_generation();
        let live_floor_recheck = M9LiveFloorRecheckInspection {
            current_generation: floor_generation.sealed_inspection().generation(),
            shared_live_floor_identity: live_floor.identity_snapshot(),
        };
        if !self
            .authority_lifecycle
            .matches_generation_for_restore(&self.authority_generation)
            || !floor_generation.matches_for_restore(&self.authority_generation)
        {
            drop(floor_guard);
            let mut outcome = self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::StaleFrontier,
                boundary_inspection,
            );
            outcome.m9_live_floor_recheck = Some(live_floor_recheck);
            return Ok(outcome);
        }
        if candidate.base_frontier.authority_binding().generation()
            != active_frontier.authority_binding().generation()
        {
            drop(floor_guard);
            let mut outcome = self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::StaleFrontier,
                boundary_inspection,
            );
            outcome.m9_live_floor_recheck = Some(live_floor_recheck);
            return Ok(outcome);
        }
        if candidate.base_frontier.authority_binding() != active_frontier.authority_binding() {
            let mismatch = M9AuthorityFrontierMismatchInspection {
                active_generation: self.current_m9_authority_inspection().generation(),
                candidate_generation: candidate.base_frontier.authority_binding().generation(),
                candidate_generation_ref: candidate
                    .base_frontier
                    .authority_binding()
                    .generation_ref()
                    .to_string(),
                active_authority_lineage_digest: active_frontier
                    .authority_binding()
                    .lineage_digest()
                    .to_string(),
                candidate_authority_lineage_digest: candidate
                    .base_frontier
                    .authority_binding()
                    .lineage_digest()
                    .to_string(),
            };
            drop(floor_guard);
            let mut outcome = self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::M9AuthorityLineageMismatch,
                boundary_inspection,
            );
            outcome.m9_authority_frontier_mismatch = Some(mismatch);
            return Ok(outcome);
        }
        if self.backend.is_ow1() {
            drop(floor_guard);
            return Ok(self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::BackendIneligible,
                boundary_inspection,
            ));
        }
        if !self.is_quiescent_for_checked_patch() {
            drop(floor_guard);
            return Ok(self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::NonQuiescentPendingCarrier,
                boundary_inspection,
            ));
        }
        if !candidate.patch_admission_is_complete() {
            drop(floor_guard);
            return Ok(self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::IncompleteCandidateAdmission,
                boundary_inspection,
            ));
        }
        // The candidate publisher is produced by normal M9 final admission
        // for the checked patched program.  Retain it only after M9 confirms
        // that its current authority inventory is an exact rebase of the
        // active one; a raw candidate authority generation never replaces the
        // fabric's state, lineages, tombstones, or shared floor.
        let Some((rebased_authority_generation, rebased_authority_publisher)) = candidate
            .patch_admission
            .authority_successor
            .for_checked_patch_rebase_of(&self.authority_generation)
        else {
            let mismatch = M9AuthorityFrontierMismatchInspection {
                active_generation: self.current_m9_authority_inspection().generation(),
                candidate_generation: candidate.patch_admission.authority_generation.generation(),
                candidate_generation_ref: candidate
                    .patch_admission
                    .authority_generation
                    .generation_ref()
                    .to_string(),
                active_authority_lineage_digest: self
                    .authority_generation
                    .checked_patch_authority_binding()
                    .lineage_digest()
                    .to_string(),
                candidate_authority_lineage_digest: candidate
                    .patch_admission
                    .authority_generation
                    .checked_patch_authority_binding()
                    .lineage_digest()
                    .to_string(),
            };
            drop(floor_guard);
            let mut outcome = self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::M9AuthorityLineageMismatch,
                boundary_inspection,
            );
            outcome.m9_authority_frontier_mismatch = Some(mismatch);
            outcome.m9_live_floor_recheck = Some(live_floor_recheck);
            return Ok(outcome);
        };

        let mut prepared = match self.clone_for_checked_patch() {
            Ok(fabric) => fabric,
            Err(_) => {
                drop(floor_guard);
                return Ok(self.reject_checked_patch(
                    &candidate,
                    active_frontier,
                    Sys4PatchDiagnosticKind::BackendIneligible,
                    boundary_inspection,
                ));
            }
        };
        if prepared
            .install_checked_patch_candidate(&candidate, &rebased_authority_generation)
            .is_err()
        {
            drop(floor_guard);
            return Ok(self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::BackendIneligible,
                boundary_inspection,
            ));
        }

        // The backend install above was clone-only. Commit the M9-provided
        // program rebase atomically through the pre-existing shared floor,
        // then install that same sealed generation/publisher into the fabric.
        // If this exact guard has advanced, neither the active fabric nor its
        // authority lifecycle has changed.
        if !floor_guard
            .rebind_checked_patch_program(&self.authority_generation, &rebased_authority_generation)
        {
            drop(floor_guard);
            let mut outcome = self.reject_checked_patch(
                &candidate,
                active_frontier,
                Sys4PatchDiagnosticKind::StaleFrontier,
                boundary_inspection,
            );
            outcome.m9_live_floor_recheck = Some(live_floor_recheck);
            return Ok(outcome);
        }
        prepared.authority_generation = rebased_authority_generation;
        prepared.authority_lifecycle = M9AuthorityLifecycle {
            publisher: rebased_authority_publisher,
        };
        // `clone_for_checked_patch` intentionally detached this candidate's
        // floor.  The canonical floor has now been rebased under its guard,
        // so the installed fabric must rejoin that canonical identity rather
        // than retaining a private candidate floor.
        prepared.authority_live_floor = self.authority_live_floor.clone();

        let activation_frontier = prepared.current_patch_frontier();
        let lifecycle = Sys4PatchLifecycle {
            verdict: Sys4PatchVerdict::Accepted,
            diagnostic: None,
            source_first_checked_projection_and_m9_admission: true,
        };
        *self = prepared;
        drop(floor_guard);
        Ok(Sys4PatchOutcome {
            verdict: Sys4PatchVerdict::Accepted,
            primary_diagnostic_kind: None,
            lifecycle,
            boundary_inspection,
            base_frontier: candidate.base_frontier,
            activation_frontier,
            m9_authority_frontier_mismatch: None,
            m9_live_floor_recheck: Some(live_floor_recheck),
        })
    }

    fn current_patch_frontier(&self) -> Sys4PatchFrontier {
        Sys4PatchFrontier::for_active(
            &self.program,
            &self.authority_generation,
            self.patch_generation,
        )
    }

    fn reject_checked_patch(
        &mut self,
        candidate: &Sys4CheckedPatchCandidate,
        active_frontier: Sys4PatchFrontier,
        diagnostic: Sys4PatchDiagnosticKind,
        boundary_inspection: Sys4PatchBoundaryInspection,
    ) -> Sys4PatchOutcome {
        self.patch_lifecycle
            .rows
            .push(Sys4PatchLifecycleRow::Rejected(diagnostic));
        Sys4PatchOutcome {
            verdict: Sys4PatchVerdict::Rejected,
            primary_diagnostic_kind: Some(diagnostic),
            lifecycle: Sys4PatchLifecycle {
                verdict: Sys4PatchVerdict::Rejected,
                diagnostic: Some(diagnostic),
                source_first_checked_projection_and_m9_admission: true,
            },
            boundary_inspection,
            base_frontier: candidate.base_frontier.clone(),
            activation_frontier: active_frontier,
            m9_authority_frontier_mismatch: None,
            m9_live_floor_recheck: None,
        }
    }

    fn is_quiescent_for_checked_patch(&self) -> bool {
        self.loci.values().all(|runtime| {
            runtime.incoming_mailbox.pending.is_empty()
                && runtime.outgoing_mailbox.pending.is_empty()
        }) && !self.backend.has_pending_owner_requests()
    }

    fn clone_for_checked_patch(&self) -> Result<Self, Sys4DiagnosticKind> {
        Ok(Self {
            program: self.program.clone(),
            loci: self.loci.clone(),
            backend: self.backend.clone_for_checked_patch()?,
            authority_generation: self.authority_generation.clone(),
            authority_lifecycle: self.authority_lifecycle.clone(),
            // Candidate execution must not share the canonical live floor:
            // an M9 successor is committed there only after all generated
            // routing/publication stages have succeeded.
            authority_live_floor: M9AuthorityLiveFloor::detached_candidate(
                self.authority_generation.clone(),
            ),
            trace: self.trace.clone(),
            m8_trace: self.m8_trace.clone(),
            actual_m8_trace: self.actual_m8_trace.clone(),
            m8_local_runtime_trace: self.m8_local_runtime_trace.clone(),
            m8_trace_offsets: self.m8_trace_offsets.clone(),
            m8_qualified_trace_nodes: self.m8_qualified_trace_nodes.clone(),
            m8_qualified_trace_dependencies: self.m8_qualified_trace_dependencies.clone(),
            m8_raw_node_loci: self.m8_raw_node_loci.clone(),
            m8_locus_trace_sequences: self.m8_locus_trace_sequences.clone(),
            m8_locus_sessions: self.m8_locus_sessions.clone(),
            observer_snapshot_failures: self.observer_snapshot_failures.clone(),
            causality: self.causality.clone(),
            next_endpoint_occurrence: self.next_endpoint_occurrence,
            route_faults: self.route_faults.clone(),
            in_transit_faults: self.in_transit_faults.clone(),
            completed_receipts: self.completed_receipts.clone(),
            local_store_read_audits: self.local_store_read_audits.clone(),
            consumption_state: self.consumption_state.clone(),
            evaluator_publication_bindings: self.evaluator_publication_bindings.clone(),
            cache: self.cache.clone(),
            relation_semantic_digests: self.relation_semantic_digests.clone(),
            used_fresh_relation_bindings: self.used_fresh_relation_bindings.clone(),
            next_request: self.next_request,
            patch_generation: self.patch_generation,
            patch_lifecycle: self.patch_lifecycle.clone(),
        })
    }

    fn install_checked_patch_candidate(
        &mut self,
        candidate: &Sys4CheckedPatchCandidate,
        rebased_authority_generation: &M9AuthorityGeneration,
    ) -> Result<(), Sys4DiagnosticKind> {
        self.backend.install_checked_patch(
            candidate.patch_admission.instance.clone(),
            rebased_authority_generation,
            &candidate.patch_id,
        )?;
        for locus in self.program.locus_names() {
            let runtime = self
                .loci
                .get_mut(&locus)
                .ok_or(Sys4DiagnosticKind::ProgramProjectionMismatch)?;
            runtime.program_identity = candidate.patch_program.checked_program_identity().clone();
            runtime.artifact =
                fabric_artifact_for(candidate.patch_program.projection.locus_program(&locus));
        }
        self.program = candidate.patch_program.clone();
        // Cache and publication bindings are derived from the preceding
        // evaluator plan. They cannot be carried across a fixed-version plan
        // replacement. The active M9 authority state, lineages, tombstones,
        // and shared live floor are retained.  Activation may separately
        // re-key that same floor to M9's normally admitted publisher for the
        // patched checked program, after exact equivalence validation.
        self.cache.clear();
        self.consumption_state = DesignatedConsumptionState::default();
        self.evaluator_publication_bindings = EvaluatorPublicationBindingRegistry::default();
        self.patch_generation = self.patch_generation.saturating_add(1);
        self.patch_lifecycle
            .rows
            .push(Sys4PatchLifecycleRow::Accepted);
        Ok(())
    }

    /// Save a bounded, process-local SYS-4 cut.  The per-locus M8 snapshots
    /// remain the source of truth for imported designated publications;
    /// SYS-4 retains only their sealed carrier/receipt identities for cache
    /// replay.  OW1 deliberately fails closed until it has an acknowledged
    /// worker-cut command rather than exposing worker-owned mutable state.
    pub(crate) fn save_local_cut(&mut self, cut_id: impl Into<String>) -> Sys4Result<Sys4LocalCut> {
        let cut_id = cut_id.into();
        let m8_cuts = match self.backend.save_local_cuts(&cut_id) {
            Ok(cuts) => cuts,
            Err(Sys4DiagnosticKind::BackendIneligible)
                if self.backend.profile() == BackendProfile::Ow1 =>
            {
                let mut diagnostics =
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::BackendIneligible);
                diagnostics.context.backend_ineligibility_reason =
                    Some(BackendIneligibilityReason::Ow1WorkerCutDeferred);
                return Err(diagnostics);
            }
            Err(kind) => return Err(Sys4DispatchDiagnostics::one(kind)),
        };
        let imported_designated_publication_state = ImportedDesignatedPublicationState {
            entries: self
                .cache
                .values()
                .map(|cached| {
                    (
                        cached.semantic_identity.clone(),
                        cached.delivery_id.clone(),
                        cached.sealed_delivery_binding_digest.clone(),
                    )
                })
                .collect(),
        };
        let designated_receipt_state = DesignatedReceiptState {
            entries: self
                .cache
                .values()
                .map(|cached| {
                    (
                        cached.semantic_identity.clone(),
                        cached.delivery_id.clone(),
                        cached.sealed_delivery_binding.logical_tick_id().to_string(),
                    )
                })
                .collect(),
        };
        let loci: Vec<_> = self.program.locus_names();
        for locus in loci {
            self.refresh_m8_local_runtime_trace(&locus);
        }
        let mut cut = Sys4LocalCut {
            cut_id,
            program_identity: self.program.checked_program_identity().clone(),
            program_fingerprint: self.program.projected_fingerprint(),
            backend_profile: self.backend.profile(),
            loci: self
                .loci
                .iter()
                .map(|(locus, runtime)| (locus.clone(), LocusRuntimeCut::capture(runtime)))
                .collect(),
            m8_cuts,
            authority_generation: self.authority_generation.clone(),
            authority_lifecycle: self.authority_lifecycle.clone(),
            authority_live_floor: self.authority_live_floor.clone(),
            trace: self.trace.clone(),
            route_faults: self.route_faults.clone(),
            in_transit_faults: self.in_transit_faults.clone(),
            completed_receipts: self.completed_receipts.clone(),
            local_store_read_audits: self.local_store_read_audits.clone(),
            cache: self.cache.clone(),
            relation_semantic_digests: self.relation_semantic_digests.clone(),
            used_fresh_relation_bindings: self.used_fresh_relation_bindings.clone(),
            consumption_state: self.consumption_state.clone(),
            evaluator_publication_bindings: self.evaluator_publication_bindings.clone(),
            imported_designated_publication_state,
            designated_receipt_state,
            m8_trace: self.m8_trace.clone(),
            actual_m8_trace: self.actual_m8_trace.clone(),
            m8_local_runtime_trace: self.m8_local_runtime_trace.clone(),
            m8_trace_offsets: self.m8_trace_offsets.clone(),
            m8_qualified_trace_nodes: self.m8_qualified_trace_nodes.clone(),
            m8_qualified_trace_dependencies: self.m8_qualified_trace_dependencies.clone(),
            m8_raw_node_loci: self.m8_raw_node_loci.clone(),
            m8_locus_trace_sequences: self.m8_locus_trace_sequences.clone(),
            m8_locus_sessions: self.m8_locus_sessions.clone(),
            observer_snapshot_failures: self.observer_snapshot_failures.clone(),
            causality: self.causality.clone(),
            next_endpoint_occurrence: self.next_endpoint_occurrence,
            next_request: self.next_request,
            patch_generation: self.patch_generation,
            patch_lifecycle: self.patch_lifecycle.clone(),
            patch_lifecycle_snapshot: self.patch_lifecycle_snapshot(),
            active_patch_frontier: self.current_patch_frontier(),
            private_restore_integrity_digest: String::new(),
        };
        cut.private_restore_integrity_digest = cut
            .compute_private_restore_integrity_digest()
            .ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramAdmissionMismatch)
            })?;
        Ok(cut)
    }

    pub(crate) fn restore_local_cut(
        program: FabricProgram,
        admission: SealedFabricAdmission,
        backend_profile: BackendProfile,
        cut: &Sys4LocalCut,
    ) -> Sys4Result<Self> {
        validate_sys4_local_cut(&program, backend_profile, cut)?;
        // Re-check and hold the shared monotone floor for the whole restore
        // critical section. Preflight alone is insufficient: another fabric
        // could otherwise install a successor between validation and the M8
        // candidate restore below.
        let live_floor = cut.authority_live_floor.clone();
        let Some(_floor_guard) = live_floor.guard_matching(&cut.authority_generation) else {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        };
        let mut fabric = Self::bootstrap(program, admission, backend_profile)?;
        fabric
            .backend
            .restore_local_cuts(&cut.m8_cuts, &cut.authority_generation.authority_state())
            .map_err(Sys4DispatchDiagnostics::one)?;
        for (locus, locus_cut) in &cut.loci {
            let runtime = fabric.loci.get_mut(locus).ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramProjectionMismatch)
            })?;
            locus_cut.restore_into(runtime);
        }
        // This replacement happens only after every M8 session has restored
        // into the fresh candidate.  A failed preflight or restore drops that
        // candidate, so no partial fabric becomes observable.
        fabric.authority_generation = cut.authority_generation.clone();
        fabric.authority_lifecycle = cut.authority_lifecycle.clone();
        fabric.authority_live_floor = live_floor.clone();
        fabric.trace = cut.trace.clone();
        fabric.route_faults = cut.route_faults.clone();
        fabric.in_transit_faults = cut.in_transit_faults.clone();
        fabric.completed_receipts = cut.completed_receipts.clone();
        fabric.local_store_read_audits = cut.local_store_read_audits.clone();
        fabric.cache = cut.cache.clone();
        fabric.relation_semantic_digests = cut.relation_semantic_digests.clone();
        fabric.used_fresh_relation_bindings = cut.used_fresh_relation_bindings.clone();
        fabric.consumption_state = cut.consumption_state.clone();
        fabric.evaluator_publication_bindings = cut.evaluator_publication_bindings.clone();
        fabric.m8_trace = cut.m8_trace.clone();
        fabric.actual_m8_trace = cut.actual_m8_trace.clone();
        fabric.m8_local_runtime_trace = cut.m8_local_runtime_trace.clone();
        fabric.m8_trace_offsets = cut.m8_trace_offsets.clone();
        fabric.m8_qualified_trace_nodes = cut.m8_qualified_trace_nodes.clone();
        fabric.m8_qualified_trace_dependencies = cut.m8_qualified_trace_dependencies.clone();
        fabric.m8_raw_node_loci = cut.m8_raw_node_loci.clone();
        fabric.m8_locus_trace_sequences = cut.m8_locus_trace_sequences.clone();
        fabric.m8_locus_sessions = cut.m8_locus_sessions.clone();
        fabric.observer_snapshot_failures = cut.observer_snapshot_failures.clone();
        fabric.causality = cut.causality.clone();
        fabric.next_endpoint_occurrence = cut.next_endpoint_occurrence;
        fabric.next_request = cut.next_request;
        fabric.patch_generation = cut.patch_generation;
        fabric.patch_lifecycle = cut.patch_lifecycle.clone();
        Ok(fabric)
    }

    /// Test-only interleaving seam: validate an old cut, let an independent
    /// admitted fabric commit a successor through the same M9 floor, then
    /// attempt the old restore. This never mutates a caller-owned fabric and
    /// exercises the restore-side floor recheck rather than manufacturing an
    /// authority generation in SYS-4.
    #[cfg(test)]
    pub(crate) fn for_test_restore_local_cut_with_authority_floor_interleaving(
        program: FabricProgram,
        admission: SealedFabricAdmission,
        backend_profile: BackendProfile,
        cut: &Sys4LocalCut,
        transition: M9AuthorityTransition,
    ) -> Sys4Result<Self> {
        validate_sys4_local_cut(&program, backend_profile, cut)?;
        let mut competing = Self::bootstrap(program.clone(), admission.clone(), backend_profile)?;
        competing.apply_admitted_authority_lifecycle(transition)?;
        Self::restore_local_cut(program, admission, backend_profile, cut)
    }
    pub(crate) fn trace(&self) -> &FabricTrace {
        &self.trace
    }
    pub(crate) fn m8_local_trace(&self) -> Sys4Result<&FabricM8Trace> {
        self.require_current_m8_trace_observer()?;
        Ok(&self.m8_trace)
    }

    pub(crate) fn m8_actual_trace(&self) -> Sys4Result<&ActualM8Trace> {
        self.require_current_m8_trace_observer()?;
        Ok(&self.actual_m8_trace)
    }

    /// Narrow observer-only lookup for one projection-derived endpoint row.
    /// No schedule input can change its selection key: callers must supply
    /// facts already returned by the completed generated dispatch.
    pub(crate) fn observer_exact_endpoint_segment(
        &self,
        request_id: &str,
        kind: Sys4TraceKind,
        edge_kind: CommunicationEdgeKind,
        source_locus: &str,
        target_locus: &str,
    ) -> Option<Sys4ObserverTraceSegment> {
        self.trace.observer_exact_endpoint_segment(
            request_id,
            kind,
            edge_kind,
            source_locus,
            target_locus,
        )
    }

    /// Return the exact generated dispatch/receive pair for one completed
    /// request.  This is intentionally narrow: callers must already know the
    /// request identity, trace kinds, edge kind, and both endpoint loci from
    /// the generated dispatch they are observing.  Missing, duplicate, or
    /// provenance-mismatched rows fail closed as `None`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn observer_exact_endpoint_occurrences(
        &self,
        request_id: &str,
        dispatch_kind: Sys4TraceKind,
        receive_kind: Sys4TraceKind,
        edge_kind: CommunicationEdgeKind,
        source_locus: &str,
        target_locus: &str,
    ) -> Option<Sys4ObserverEndpointOccurrences> {
        let dispatched = self.observer_exact_endpoint_segment(
            request_id,
            dispatch_kind,
            edge_kind,
            source_locus,
            target_locus,
        )?;
        let received = self.observer_exact_endpoint_segment(
            request_id,
            receive_kind,
            edge_kind,
            source_locus,
            target_locus,
        )?;
        if dispatched.source_ref() != received.source_ref()
            || dispatched.core_ref() != received.core_ref()
            || dispatched.source_fragment_ref() != received.source_fragment_ref()
            || dispatched.target_fragment_ref() != received.target_fragment_ref()
            || dispatched.edge_ref() != received.edge_ref()
            || dispatched.source_ref().path.is_empty()
            || dispatched.core_ref().is_empty()
            || dispatched.source_fragment_ref().is_empty()
            || dispatched.target_fragment_ref().is_empty()
            || dispatched.edge_ref().is_empty()
        {
            return None;
        }
        let request_enqueue_occurrence_id = self
            .causality
            .sole_predecessor(dispatched.occurrence_ref())?;
        if request_enqueue_occurrence_id.is_empty()
            || dispatched.occurrence_ref().is_empty()
            || received.occurrence_ref().is_empty()
            || !self
                .causality
                .reaches(dispatched.occurrence_ref(), request_enqueue_occurrence_id)
            || !self
                .causality
                .reaches(received.occurrence_ref(), dispatched.occurrence_ref())
        {
            return None;
        }
        Some(Sys4ObserverEndpointOccurrences {
            request_enqueue_occurrence_id: request_enqueue_occurrence_id.to_string(),
            dispatch_occurrence_id: dispatched.occurrence_ref().to_string(),
            receive_occurrence_id: received.occurrence_ref().to_string(),
            source_ref: dispatched.source_ref().clone(),
            core_ref: dispatched.core_ref().to_string(),
            source_fragment_ref: dispatched.source_fragment_ref().to_string(),
            target_fragment_ref: dispatched.target_fragment_ref().to_string(),
            edge_ref: dispatched.edge_ref().to_string(),
        })
    }

    /// Exact M8 occurrence for a completed request.  Ambiguity is rejected
    /// rather than hidden behind a first-match observer projection.
    pub(crate) fn observer_exact_m8_occurrence(
        &self,
        request_id: &str,
        kind: M8LocalTraceKind,
    ) -> Option<&str> {
        self.m8_actual_trace()
            .ok()
            .and_then(|trace| trace.observer_exact_node_ref_for_request_kind(request_id, kind))
    }

    pub(crate) fn observer_causally_reaches(
        &self,
        descendant_occurrence: &str,
        ancestor_occurrence: &str,
    ) -> bool {
        self.causality
            .reaches(descendant_occurrence, ancestor_occurrence)
    }

    #[cfg(test)]
    pub(crate) fn m8_backend_test_support_mut(&mut self) -> M8BackendTestSupport<'_> {
        M8BackendTestSupport {
            backend: &mut self.backend,
        }
    }

    /// Typed observer status. A failed worker snapshot does not invalidate an
    /// already committed semantic operation, but it does make aggregate
    /// devtools evidence fail closed until a fresh snapshot is incorporated.
    pub(crate) fn observer_snapshot_failures(&self) -> Vec<ObserverSnapshotFailure> {
        self.observer_snapshot_failures.values().cloned().collect()
    }

    pub(crate) fn m8_local_runtime_trace(&self) -> Sys4Result<&M8LocalTrace> {
        if let Some(failure) = self
            .observer_snapshot_failures
            .values()
            .find(|failure| failure.channel() == ObserverSnapshotChannel::LocalTrace)
        {
            return Err(self.observer_snapshot_diagnostic(failure));
        }
        Ok(&self.m8_local_runtime_trace)
    }

    /// Request a fresh trace snapshot for one semantic locus.  This is the
    /// only recovery path that clears its local-trace unavailable status.
    pub(crate) fn recover_m8_local_trace_observer(&mut self, locus: &str) -> Sys4Result<()> {
        if !self.is_bound_m8_observer_locus(locus) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::WrongTargetLocus,
            ));
        }
        self.refresh_m8_local_runtime_trace(locus);
        let session_id = self.observer_session_id(locus);
        if let Some(failure) = self
            .observer_snapshot_failures
            .get(&(session_id, ObserverSnapshotChannel::LocalTrace))
        {
            return Err(self.observer_snapshot_diagnostic(failure));
        }
        Ok(())
    }

    /// Typed observer-safe session probe.  This is intentionally separate
    /// from semantic dispatch: a failed redacted-session read reports only
    /// observer unavailability and cannot replay or roll back M8 work.
    #[cfg(test)]
    pub(crate) fn try_m8_partition_evidence(&mut self) -> Sys4Result<M8RuntimePartitionEvidence> {
        // A worker snapshot failure invalidates every aggregate view derived
        // from that worker trace.  Do not join a fresh session observer to
        // stale qualified/actual rows; recovery rebuilds the full session
        // projection before this view becomes available again.
        self.require_current_m8_trace_observer()?;
        let mut observer_sessions = Vec::new();
        for locus in self.program.locus_names() {
            let session_id = self.observer_session_id(&locus);
            match self.backend.observer_safe_session(&locus) {
                Ok(Some(observer)) => {
                    self.clear_observer_snapshot_failure(
                        &session_id,
                        ObserverSnapshotChannel::ObserverSafeSession,
                    );
                    observer_sessions.push((locus, observer));
                }
                Ok(None) => {}
                Err(diagnostic) => {
                    let failure = ObserverSnapshotFailure {
                        session_id: session_id.clone(),
                        channel: ObserverSnapshotChannel::ObserverSafeSession,
                        diagnostic,
                    };
                    self.observer_snapshot_failures.insert(
                        (session_id, ObserverSnapshotChannel::ObserverSafeSession),
                        failure.clone(),
                    );
                    return Err(self.observer_snapshot_diagnostic(&failure));
                }
            }
        }
        let partitions = observer_sessions
            .into_iter()
            .map(|(locus, observer)| {
                let session_id = self
                    .m8_locus_sessions
                    .get(&locus)
                    .cloned()
                    .unwrap_or_else(|| locus.clone());
                let fabric_node_ids = self
                    .m8_qualified_trace_nodes
                    .get(&session_id)
                    .cloned()
                    .unwrap_or_default();
                let request_ids = self
                    .actual_m8_trace
                    .nodes
                    .iter()
                    .filter_map(|node| {
                        node.request_id
                            .as_ref()
                            .map(|request_id| (node.node_id.clone(), request_id.clone()))
                    })
                    .collect();
                let dependencies = self
                    .m8_qualified_trace_dependencies
                    .get(&session_id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|(raw, predecessors)| {
                        fabric_node_ids
                            .get(&raw)
                            .cloned()
                            .map(|node| (node, predecessors))
                    })
                    .collect();
                (
                    locus.clone(),
                    M8RuntimePartition::from_m8_session(
                        &locus,
                        observer,
                        &fabric_node_ids,
                        &request_ids,
                        &dependencies,
                    ),
                )
            })
            .collect();
        Ok(M8RuntimePartitionEvidence { partitions })
    }

    fn observer_session_id(&self, locus: &str) -> String {
        if self.backend.is_ow1() {
            "ow1".to_string()
        } else {
            locus.to_string()
        }
    }

    fn require_current_m8_trace_observer(&self) -> Sys4Result<()> {
        if let Some(failure) = self
            .observer_snapshot_failures
            .values()
            .find(|failure| failure.channel() == ObserverSnapshotChannel::LocalTrace)
        {
            return Err(self.observer_snapshot_diagnostic(failure));
        }
        Ok(())
    }

    /// A recovery operation names a semantic session owner, not merely a
    /// locus whose work happened to run on an OW1 worker.  The latter would
    /// let callers re-key the one worker session through an arbitrary E/C
    /// locus after an observer failure.
    fn is_bound_m8_observer_locus(&self, locus: &str) -> bool {
        match &self.backend {
            M8ExecutionBackend::St(sessions) => sessions.contains_key(locus),
            M8ExecutionBackend::Ow1(worker) => worker.evidence().target_owner().as_str() == locus,
        }
    }

    fn record_observer_snapshot_failure(
        &mut self,
        session_id: String,
        channel: ObserverSnapshotChannel,
        diagnostic: Sys4DiagnosticKind,
    ) {
        let failure = ObserverSnapshotFailure {
            session_id: session_id.clone(),
            channel,
            diagnostic,
        };
        self.observer_snapshot_failures
            .insert((session_id, channel), failure);
    }

    fn clear_observer_snapshot_failure(
        &mut self,
        session_id: &str,
        channel: ObserverSnapshotChannel,
    ) {
        self.observer_snapshot_failures
            .remove(&(session_id.to_string(), channel));
    }

    fn observer_snapshot_diagnostic(
        &self,
        failure: &ObserverSnapshotFailure,
    ) -> Sys4DispatchDiagnostics {
        let mut diagnostics = Sys4DispatchDiagnostics::one(failure.diagnostic());
        diagnostics.context.observer_snapshot_failure = Some(Box::new(failure.clone()));
        diagnostics
    }

    fn refresh_m8_local_runtime_trace(&mut self, locus: &str) {
        let observer_session_id = self.observer_session_id(locus);
        let (session_id, trace) = match self.backend.local_trace_snapshot(locus) {
            Ok(Some(snapshot)) => snapshot,
            // A caller asked for a locus with no session. This is a genuine
            // absence, not an unavailable OW1 observer, and cannot clear a
            // prior worker-snapshot failure.
            Ok(None) => return,
            Err(diagnostic) => {
                self.record_observer_snapshot_failure(
                    observer_session_id,
                    ObserverSnapshotChannel::LocalTrace,
                    diagnostic,
                );
                return;
            }
        };
        let observations = trace.observations();
        if observations.is_empty() {
            self.m8_locus_sessions
                .insert(locus.to_string(), session_id.clone());
            self.m8_trace_offsets.insert(session_id, 0);
            self.clear_observer_snapshot_failure(
                &observer_session_id,
                ObserverSnapshotChannel::LocalTrace,
            );
            return;
        }
        let recovering = self.observer_snapshot_failures.contains_key(&(
            observer_session_id.clone(),
            ObserverSnapshotChannel::LocalTrace,
        ));
        // A successful worker response is an exact session snapshot.  Always
        // reconcile all of its rows, not only a delta: a prior unavailable
        // snapshot may have left provisional qualified identities and endpoint
        // associations that must be replaced before any aggregate observer
        // view is made available again.
        let mut missing = Vec::new();
        let mut semantic_loci = BTreeMap::new();
        let prior_raw_node_loci = self
            .m8_raw_node_loci
            .get(&session_id)
            .cloned()
            .unwrap_or_default();
        for observation in &observations {
            let semantic_locus = prior_raw_node_loci
                .get(observation.node_id())
                .cloned()
                .unwrap_or_else(|| {
                    if self.backend.is_ow1() {
                        Self::semantic_locus_for_m8_observation(locus, observation)
                    } else {
                        locus.to_string()
                    }
                });
            if !self
                .m8_qualified_trace_nodes
                .get(&session_id)
                .is_some_and(|known| known.contains_key(observation.node_id()))
            {
                missing.push(observation.node_id().to_string());
            }
            semantic_loci.insert(observation.node_id().to_string(), semantic_locus);
        }
        for raw_node_id in missing {
            let sequence = self
                .m8_locus_trace_sequences
                .entry(locus.to_string())
                .or_default();
            let qualified = format!("sys4-m8:{locus}:m8-fabric-trace-{sequence:020}");
            *sequence += 1;
            self.m8_qualified_trace_nodes
                .entry(session_id.clone())
                .or_default()
                .insert(raw_node_id, qualified);
        }
        self.m8_locus_sessions
            .insert(locus.to_string(), session_id.clone());
        self.m8_raw_node_loci
            .insert(session_id.clone(), semantic_loci);

        let Some(known_nodes) = self.m8_qualified_trace_nodes.get(&session_id).cloned() else {
            self.record_observer_snapshot_failure(
                observer_session_id,
                ObserverSnapshotChannel::LocalTrace,
                Sys4DiagnosticKind::ObserverSnapshotUnavailable,
            );
            return;
        };
        let Some(raw_node_loci) = self.m8_raw_node_loci.get(&session_id).cloned() else {
            self.record_observer_snapshot_failure(
                observer_session_id,
                ObserverSnapshotChannel::LocalTrace,
                Sys4DiagnosticKind::ObserverSnapshotUnavailable,
            );
            return;
        };
        let dependency_projection: BTreeMap<_, _> = observations
            .iter()
            .map(|observation| {
                let dependencies = raw_node_loci
                    .get(observation.node_id())
                    .map(|semantic_locus| {
                        observation
                            .predecessor_ids()
                            .iter()
                            .filter(|raw| raw_node_loci.get(*raw) == Some(semantic_locus))
                            .filter_map(|raw| known_nodes.get(raw).cloned())
                            .collect()
                    })
                    .unwrap_or_default();
                (observation.node_id().to_string(), dependencies)
            })
            .collect();
        if dependency_projection.len() != observations.len()
            || observations
                .iter()
                .any(|observation| !known_nodes.contains_key(observation.node_id()))
        {
            self.record_observer_snapshot_failure(
                observer_session_id,
                ObserverSnapshotChannel::LocalTrace,
                Sys4DiagnosticKind::ObserverSnapshotUnavailable,
            );
            return;
        }
        let prior_dependency_projection = self
            .m8_qualified_trace_dependencies
            .get(&session_id)
            .cloned()
            .unwrap_or_default();
        self.m8_qualified_trace_dependencies
            .insert(session_id.clone(), dependency_projection.clone());
        self.m8_local_runtime_trace
            .reconcile_fabric_qualified_session(&trace, &known_nodes, &dependency_projection);

        let owner_request_nodes: BTreeMap<_, _> = observations
            .iter()
            .filter(|observation| observation.kind() == M8LocalTraceKind::OwnerEnqueued)
            .filter_map(|observation| {
                observation.occurrence_id.clone().and_then(|occurrence| {
                    known_nodes
                        .get(observation.node_id())
                        .cloned()
                        .map(|node_id| (occurrence, node_id))
                })
            })
            .collect();
        for observation in observations {
            let Some(node_id) = known_nodes.get(observation.node_id()).cloned() else {
                self.record_observer_snapshot_failure(
                    observer_session_id,
                    ObserverSnapshotChannel::LocalTrace,
                    Sys4DiagnosticKind::ObserverSnapshotUnavailable,
                );
                return;
            };
            let recovered_owner_request = recovering
                .then(|| {
                    (observation.kind() == M8LocalTraceKind::OwnerWrite)
                        .then(|| {
                            observation
                                .occurrence_id
                                .as_ref()
                                .and_then(|occurrence| owner_request_nodes.get(occurrence).cloned())
                        })
                        .flatten()
                })
                .flatten();
            let external_predecessors: Vec<_> = self
                .causality
                .predecessor_ids(&node_id)
                .into_iter()
                // Replace only M8's own prior projected edges. Explicit
                // generated endpoint edges may name a qualified M8 node in
                // the same physical OW1 session (E publication → C import),
                // and are semantic SYS-4 causality rather than worker FIFO.
                .filter(|predecessor| {
                    !prior_dependency_projection
                        .get(observation.node_id())
                        .is_some_and(|prior| prior.contains(predecessor))
                })
                .filter(|predecessor| {
                    recovered_owner_request
                        .as_ref()
                        .is_none_or(|request| predecessor != request)
                })
                .collect();
            let mut predecessors = dependency_projection
                .get(observation.node_id())
                .cloned()
                .unwrap_or_default();
            for predecessor in external_predecessors {
                if !predecessors.contains(&predecessor) {
                    predecessors.push(predecessor);
                }
            }
            self.causality
                .replace(node_id.clone(), predecessors.clone());
            let semantic_identity = match observation.kind() {
                M8LocalTraceKind::DesignatedValuePublished
                | M8LocalTraceKind::DesignatedEvaluationIdempotent => {
                    Some(observation.operation_id().to_string())
                }
                _ => (!observation.semantic_identity().is_empty())
                    .then(|| observation.semantic_identity().to_string()),
            };
            self.actual_m8_trace.reconcile_snapshot_node(
                node_id,
                format!("{:?}", observation.kind()),
                semantic_identity,
                (!observation.consumer_locus().is_empty())
                    .then(|| observation.consumer_locus().to_string()),
                predecessors,
            );
        }
        if recovering {
            self.reconcile_m8_consumption_trace();
        }
        self.m8_trace_offsets
            .insert(session_id.clone(), trace.len());
        // Clear only after the exact worker/session trace snapshot was
        // received and incorporated into the aggregate observer projection.
        self.clear_observer_snapshot_failure(
            &observer_session_id,
            ObserverSnapshotChannel::LocalTrace,
        );
    }

    /// A sole OW1 worker can execute M8 rows for more than one semantic
    /// locus.  Preserve M8 predecessors when they remain in the same
    /// semantic locus, but never turn physical worker FIFO order into a
    /// cross-locus Mir dependency.  ST's independently owned sessions need
    /// no suppression, so this classification is applied only to OW1 rows.
    fn semantic_locus_for_m8_observation(
        default_locus: &str,
        observation: &M8LocalTraceObservation,
    ) -> String {
        match observation.kind() {
            M8LocalTraceKind::DesignatedAuthorityValidated => {
                if !observation.evaluator_locus().is_empty() {
                    observation.evaluator_locus().to_string()
                } else {
                    default_locus.to_string()
                }
            }
            M8LocalTraceKind::DesignatedPublicationImported
            | M8LocalTraceKind::DesignatedConsumerAuthorityValidated
            | M8LocalTraceKind::DesignatedValueConsumed
            | M8LocalTraceKind::DesignatedConsumptionRejected
            | M8LocalTraceKind::DesignatedCacheValidated => {
                if !observation.consumer_locus().is_empty() {
                    observation.consumer_locus().to_string()
                } else {
                    default_locus.to_string()
                }
            }
            M8LocalTraceKind::DesignatedInputReceiptValidated
            | M8LocalTraceKind::DesignatedValuePublished
            | M8LocalTraceKind::DesignatedEvaluationIdempotent
            | M8LocalTraceKind::DesignatedEvaluationRejected => {
                if !observation.evaluator_locus().is_empty() {
                    observation.evaluator_locus().to_string()
                } else {
                    default_locus.to_string()
                }
            }
            // A remote input read executes at its source owner but belongs to
            // neither that owner's RMW lane nor the evaluator's M8 decision
            // lane.  Give it a stable generated service lane so OW1 physical
            // FIFO cannot manufacture an S-RMW→read or read→E-evaluation M8
            // dependency.  The explicit generated carrier/receipt supplies
            // the semantic S→E order instead.
            M8LocalTraceKind::OwnerRead if !observation.evaluator_locus().is_empty() => {
                format!(
                    "designated-source-read:{}→{}",
                    observation.owner_locus(),
                    observation.evaluator_locus()
                )
            }
            _ => {
                if !observation.owner_locus().is_empty() {
                    observation.owner_locus().to_string()
                } else {
                    default_locus.to_string()
                }
            }
        }
    }

    fn fabric_qualified_m8_observation(
        &self,
        session_id: &str,
        observation: &M8LocalTraceObservation,
    ) -> M8LocalTraceObservation {
        let known_nodes = self
            .m8_qualified_trace_nodes
            .get(session_id)
            .expect("refreshed M8 session retains fabric node registry");
        let node_id = known_nodes
            .get(observation.node_id())
            .cloned()
            .expect("refreshed M8 observation has a fabric node identity");
        let dependencies = self
            .m8_qualified_trace_dependencies
            .get(session_id)
            .and_then(|known| known.get(observation.node_id()))
            .cloned()
            .expect("refreshed M8 observation retains dependency projection");
        observation.fabric_rekeyed(node_id, dependencies)
    }

    /// Register an M8 node returned directly by a typed backend outcome when
    /// a later clone-only trace snapshot is unavailable.  The node identity
    /// remains M8-owned; this only assigns the usual fabric qualification so
    /// the already-committed semantic operation can finish without replay.
    /// A later successful snapshot replaces the provisional dependency view
    /// with the complete worker trace before aggregate observation is served.
    fn ensure_fabric_qualified_m8_node_for_locus(&mut self, locus: &str, raw_node_id: &str) {
        let session_id = self.observer_session_id(locus);
        self.m8_locus_sessions
            .entry(locus.to_string())
            .or_insert_with(|| session_id.clone());
        if self
            .m8_qualified_trace_nodes
            .get(&session_id)
            .is_some_and(|nodes| nodes.contains_key(raw_node_id))
        {
            return;
        }
        let sequence = self
            .m8_locus_trace_sequences
            .entry(locus.to_string())
            .or_default();
        let qualified = format!("sys4-m8:{locus}:m8-fabric-trace-{sequence:020}");
        *sequence += 1;
        self.m8_qualified_trace_nodes
            .entry(session_id.clone())
            .or_default()
            .insert(raw_node_id.to_string(), qualified);
        self.m8_raw_node_loci
            .entry(session_id.clone())
            .or_default()
            .insert(raw_node_id.to_string(), locus.to_string());
        self.m8_qualified_trace_dependencies
            .entry(session_id)
            .or_default()
            .entry(raw_node_id.to_string())
            .or_default();
    }

    fn ensure_fabric_qualified_m8_observation_for_locus(
        &mut self,
        locus: &str,
        observation: &M8LocalTraceObservation,
    ) {
        self.ensure_fabric_qualified_m8_node_for_locus(locus, observation.node_id());
        let session_id = self.observer_session_id(locus);
        let semantic_locus = if self.backend.is_ow1() {
            Self::semantic_locus_for_m8_observation(locus, observation)
        } else {
            locus.to_string()
        };
        self.m8_raw_node_loci
            .entry(session_id.clone())
            .or_default()
            .insert(observation.node_id().to_string(), semantic_locus.clone());
        let predecessors = observation
            .predecessor_ids()
            .iter()
            .filter(|raw| {
                self.m8_raw_node_loci
                    .get(&session_id)
                    .and_then(|loci| loci.get(*raw))
                    == Some(&semantic_locus)
            })
            .filter_map(|raw| {
                self.m8_qualified_trace_nodes
                    .get(&session_id)
                    .and_then(|nodes| nodes.get(raw))
                    .cloned()
            })
            .collect();
        self.m8_qualified_trace_dependencies
            .entry(session_id)
            .or_default()
            .entry(observation.node_id().to_string())
            .or_insert(predecessors);
    }

    fn fabric_qualified_m8_observation_for_locus(
        &mut self,
        locus: &str,
        observation: &M8LocalTraceObservation,
    ) -> M8LocalTraceObservation {
        self.ensure_fabric_qualified_m8_observation_for_locus(locus, observation);
        let session_id = self
            .m8_locus_sessions
            .get(locus)
            .expect("M8 result is qualified only after its session refresh");
        self.fabric_qualified_m8_observation(session_id, observation)
    }

    /// Associate every M8 row that carries this exact immutable envelope
    /// context with the fabric request.  In particular, owner queue reads are
    /// emitted by M8 between the returned enqueue/serve rows, so request
    /// evidence must come from the row's context rather than an operation-wide
    /// summary or a latest-observation lookup.
    fn associate_m8_envelope_request(&mut self, envelope: &MailboxEnvelope) {
        let observations: Vec<_> = self
            .m8_local_runtime_trace
            .observations()
            .into_iter()
            .filter(|observation| observation.envelope_id() == envelope.envelope_id())
            .collect();
        for observation in observations {
            let node_id = observation.node_id().to_string();
            self.causality
                .record(node_id.clone(), observation.predecessor_ids().to_vec());
            self.actual_m8_trace.append(
                node_id,
                format!("{:?}", observation.kind()),
                Some(envelope.request_id.clone()),
                (!observation.semantic_identity().is_empty())
                    .then(|| observation.semantic_identity().to_string()),
                (!observation.consumer_locus().is_empty())
                    .then(|| observation.consumer_locus().to_string()),
                observation.predecessor_ids().to_vec(),
            );
        }
    }

    fn fabric_qualified_m8_node_for_locus(&mut self, locus: &str, raw_node_id: &str) -> String {
        self.ensure_fabric_qualified_m8_node_for_locus(locus, raw_node_id);
        let session_id = self
            .m8_locus_sessions
            .get(locus)
            .expect("M8 node is qualified only after its session refresh");
        self.m8_qualified_trace_nodes
            .get(session_id)
            .and_then(|known| known.get(raw_node_id))
            .cloned()
            .expect("refreshed M8 node has a fabric identity")
    }

    /// Backend outcomes retain the raw M8 observation until their owner
    /// session has been refreshed.  Once refreshed, SYS-4 can only expose the
    /// fabric-qualified occurrence, never a second raw-id view of the same
    /// M8-owned event.
    fn fabric_qualified_m8_failure_for_locus(
        &mut self,
        locus: &str,
        failure: M8BackendFailure,
    ) -> M8BackendFailure {
        M8BackendFailure {
            kind: failure.kind,
            observation: failure.observation.map(|observation| {
                Box::new(self.fabric_qualified_m8_observation_for_locus(locus, &observation))
            }),
        }
    }

    fn reconcile_m8_consumption_trace(&mut self) {
        let mut counts = BTreeMap::new();
        for observation in self.m8_local_runtime_trace.observations() {
            if observation.kind() == M8LocalTraceKind::DesignatedValueConsumed
                && !observation.semantic_identity().is_empty()
                && !observation.consumer_locus().is_empty()
            {
                *counts
                    .entry((
                        observation.semantic_identity().to_string(),
                        observation.consumer_locus().to_string(),
                    ))
                    .or_default() += 1;
            }
        }
        self.m8_trace.consumption_counts = counts;
    }

    pub(crate) fn causality(&self) -> &CausalityGraph {
        &self.causality
    }

    pub(crate) fn m8_owner_queue_depth(&self, owner_locus: &str) -> usize {
        match &self.backend {
            M8ExecutionBackend::St(sessions) => sessions
                .get(owner_locus)
                .map_or(0, |runtime| runtime.pending_owner_fifo(owner_locus).len()),
            M8ExecutionBackend::Ow1(_) => 0,
        }
    }

    pub(crate) fn designated_cache_entry(&self, identity: &str) -> Option<&CachedDelivery> {
        self.cache.get(identity)
    }

    pub(crate) fn designated_cache_snapshot(&self) -> BTreeMap<String, CachedDelivery> {
        self.cache.clone()
    }

    /// Observer-safe cache cardinality for one checked designated operation
    /// and consumer.  It returns neither a cached value nor a delivery
    /// binding, so callers can use it only as a mutation-free status fact.
    pub(crate) fn designated_cache_entry_count_for_value(
        &self,
        value_name: &str,
        consumer: &str,
    ) -> usize {
        self.cache
            .values()
            .filter(|entry| entry.operation == value_name && entry.consumer_locus == consumer)
            .count()
    }

    /// Typed worker-backed designated-publication observer. `Ok(None)` is a
    /// genuine absence; `Err(ObserverSnapshotUnavailable)` means no observer
    /// result is available and must never be silently read as absence.
    pub(crate) fn try_m8_designated_publication_snapshot(
        &mut self,
        value_name: &str,
    ) -> Sys4Result<Option<Ow1ObserverDesignatedPublication>> {
        match self.backend.designated_publication_snapshot(value_name) {
            Ok(publication) => {
                debug_assert!(publication.as_ref().is_none_or(|publication| {
                    publication.is_observer_safe() && publication.value_name() == value_name
                }));
                self.clear_observer_snapshot_failure(
                    DESIGNATED_PUBLICATION_OBSERVER_SESSION,
                    ObserverSnapshotChannel::DesignatedPublication,
                );
                Ok(publication)
            }
            Err(diagnostic) => {
                let failure = ObserverSnapshotFailure {
                    session_id: DESIGNATED_PUBLICATION_OBSERVER_SESSION.to_string(),
                    channel: ObserverSnapshotChannel::DesignatedPublication,
                    diagnostic,
                };
                self.observer_snapshot_failures.insert(
                    (
                        DESIGNATED_PUBLICATION_OBSERVER_SESSION.to_string(),
                        ObserverSnapshotChannel::DesignatedPublication,
                    ),
                    failure.clone(),
                );
                Err(self.observer_snapshot_diagnostic(&failure))
            }
        }
    }

    pub(crate) fn designated_consumption_state(&self) -> &DesignatedConsumptionState {
        &self.consumption_state
    }

    /// Observer-safe aggregate for one projected designated value at one
    /// consumer locus.  The semantic-state key remains the sealed internal
    /// identity; this accessor merely joins it to the already cached checked
    /// operation name and exposes no delivery payload or M9 material.
    pub(crate) fn designated_semantic_consumption_count_for_value(
        &self,
        value_name: &str,
        consumer: &str,
    ) -> usize {
        self.consumption_state
            .counts
            .iter()
            .filter(|((semantic_identity, locus), _)| {
                locus == consumer
                    && self.cache.get(semantic_identity).is_some_and(|entry| {
                        entry.operation == value_name && entry.consumer_locus == consumer
                    })
            })
            .map(|(_, count)| *count)
            .sum()
    }
    pub(crate) fn projected_artifact_identity(&self) -> &CheckedProgramIdentity {
        self.program.checked_program_identity()
    }
    pub(crate) fn m9_authority_lifecycle_mut(&mut self) -> M9AuthorityLifecycleAccess<'_> {
        let Self {
            authority_lifecycle,
            authority_generation,
            authority_live_floor,
            ..
        } = self;
        let floor_guard = authority_live_floor.guard_matching(authority_generation);
        M9AuthorityLifecycleAccess {
            lifecycle: authority_lifecycle,
            live_generation: authority_generation,
            floor_guard,
        }
    }

    pub(crate) fn apply_admitted_authority_lifecycle(
        &mut self,
        transition: M9AuthorityTransition,
    ) -> Sys4Result<()> {
        if transition.generation.program_identity()
            != self.program.checked_program_identity().stable_key()
            || transition.generation.generation() <= self.authority_generation.generation()
            || !transition
                .generation
                .preserves_tombstones_from(&self.authority_generation)
            || !self
                .authority_lifecycle
                .matches_generation_for_restore(&transition.generation)
            || !transition.matches_live_runtime_validation_observations(&self.authority_generation)
        {
            return Err(self.reject_uninstalled_authority_transition(
                &transition,
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        // Clone the Arc before taking its guard so Rust can keep the floor
        // lock while this fabric mutates its disjoint backend/generation
        // fields. A competing fabric must therefore lose before any local M8
        // authority inventory is refreshed.
        let live_floor = self.authority_live_floor.clone();
        let Some(mut floor_guard) = live_floor.guard_matching(&self.authority_generation) else {
            return Err(self.reject_uninstalled_authority_transition(
                &transition,
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        };
        if !floor_guard.accepts_successor(&self.authority_generation, &transition.generation) {
            drop(floor_guard);
            return Err(self.reject_uninstalled_authority_transition(
                &transition,
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        if let Err(kind) = self.backend.refresh_authority(&transition.generation) {
            drop(floor_guard);
            return Err(self.reject_uninstalled_authority_transition(&transition, kind));
        }
        floor_guard.commit_successor(&transition.generation);
        self.authority_generation = transition.generation;
        Ok(())
    }

    fn reject_uninstalled_authority_transition(
        &mut self,
        transition: &M9AuthorityTransition,
        kind: Sys4DiagnosticKind,
    ) -> Sys4DispatchDiagnostics {
        // The publisher was advanced only to construct the sealed transition.
        // If install cannot proceed, restore its M9-owned prior snapshot when
        // this fabric still owns that exact uninstalled successor. A foreign
        // transition cannot affect this fabric's lifecycle.
        let _restored = self
            .authority_lifecycle
            .rollback_uninstalled_transition(transition);
        Sys4DispatchDiagnostics::one(kind)
    }

    fn route_for(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source_locus: &str,
        target_locus: &str,
    ) -> Sys4Result<FabricRoute> {
        self.program
            .route_index
            .route(&FabricRouteKey {
                operation: operation.to_string(),
                kind,
                source_locus: source_locus.to_string(),
                target_locus: target_locus.to_string(),
            })
            .cloned()
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable))
    }

    fn next_request_id(&mut self) -> Sys4Result<String> {
        self.next_request_id_with_prefix("sys4-request-")
    }

    fn next_relation_request_id(&mut self) -> Sys4Result<String> {
        self.next_request_id_with_prefix("sys5-relation-request:")
    }

    fn next_request_id_with_prefix(&mut self, prefix: &str) -> Sys4Result<String> {
        let next = self.next_request;
        self.next_request = next
            .checked_add(1)
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::IdentifierExhausted))?;
        Ok(format!("{prefix}{next:020}"))
    }
}

impl LocalFabric {
    pub(crate) fn current_m9_authority_inspection(&self) -> M9AuthorityInspection {
        self.authority_generation.sealed_inspection()
    }

    pub(crate) fn current_patch_frontier_snapshot(&self) -> Sys4PatchFrontier {
        self.current_patch_frontier()
    }

    pub(crate) fn m9_authority_live_floor_identity_snapshot(&self) -> usize {
        self.authority_live_floor.identity_snapshot()
    }

    pub(crate) fn in_transit_faults(&self) -> &InTransitFaults {
        &self.in_transit_faults
    }

    pub(crate) fn m8_authority_state_digest(&self, locus: &str) -> String {
        format!("{}:{}", self.authority_generation.generation_ref(), locus)
    }

    #[cfg(test)]
    pub(crate) fn for_test_set_relation_identifier_counters(
        &mut self,
        next_request: u64,
        next_endpoint_occurrence: u64,
    ) {
        self.next_request = next_request;
        self.next_endpoint_occurrence = next_endpoint_occurrence;
    }

    #[cfg(test)]
    pub(crate) fn for_test_clear_route_fault(&mut self, edge_ref: &str) {
        self.route_faults.remove(edge_ref);
    }

    /// Test-only seam for the two-phase candidate commit boundary. It runs
    /// the real source-derived M8 fallback and generated endpoint work on an
    /// isolated candidate, but deliberately withholds the canonical-floor
    /// commit so a competing sibling transition can be placed deterministically
    /// between the two phases.
    #[cfg(test)]
    pub(crate) fn for_test_stage_relation_invalidation_candidate(
        &self,
        relation: &str,
    ) -> Sys4Result<Self> {
        let mut candidate = self
            .clone_for_checked_patch()
            .map_err(Sys4DispatchDiagnostics::one)?;
        candidate.invalidate_relation_primary_staged(relation)?;
        Ok(candidate)
    }

    /// Test-only completion for a candidate built by
    /// `for_test_stage_relation_invalidation_candidate`. It exposes no
    /// authority input and shares the production two-phase commit check.
    #[cfg(test)]
    pub(crate) fn for_test_commit_staged_relation_candidate(
        &mut self,
        candidate: Self,
    ) -> Sys4Result<()> {
        self.commit_staged_authority_candidate(candidate)
    }

    #[cfg(test)]
    pub(crate) fn for_test_remove_relation_publish_authority(&mut self, relation: &str) {
        self.authority_generation
            .for_test_remove_relation_authority_use(relation, "publish_relation");
    }

    #[cfg(test)]
    pub(crate) fn for_test_pending_relation_publication_count(&self, locus: &str) -> usize {
        self.loci.get(locus).map_or(0, |runtime| {
            runtime
                .outgoing_mailbox
                .pending
                .iter()
                .filter(|envelope| {
                    matches!(envelope.payload, MailboxPayload::RelationPublication { .. })
                })
                .count()
        })
    }

    pub(crate) fn local_store_read_audit(&self, locus: &str) -> &LocalStoreReadAudit {
        self.local_store_read_audits
            .get(locus)
            .expect("booted locus retains read audit")
    }

    fn edge_for(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source_locus: &str,
        target_locus: &str,
    ) -> Sys4Result<CommunicationEdge> {
        let route = self.route_for(operation, kind, source_locus, target_locus)?;
        self.program
            .projection
            .communication_plan()
            .edges()
            .iter()
            .find(|edge| edge.edge_ref() == route.edge_ref)
            .cloned()
            .ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramProjectionMismatch)
            })
    }

    pub(crate) fn active_runtime_identity_snapshot(&self) -> ActiveRuntimeIdentitySnapshot {
        ActiveRuntimeIdentitySnapshot {
            checked_program_identity: self.program.checked_program_identity().clone(),
            projection_fingerprint: self.program.projected_fingerprint(),
            artifacts: self
                .loci
                .iter()
                .map(|(locus, runtime)| (locus.clone(), runtime.artifact.clone()))
                .collect(),
            route_refs: self.program.route_index().edge_refs(),
            cache_bindings: self
                .cache
                .values()
                .map(|cached| {
                    (
                        cached.semantic_identity.clone(),
                        cached.delivery_id.clone(),
                        cached.sealed_delivery_binding_digest.clone(),
                    )
                })
                .collect(),
            patch_generation: self.patch_generation,
        }
    }

    pub(crate) fn patch_lifecycle_snapshot(&self) -> Sys4PatchLifecycleSnapshot {
        Sys4PatchLifecycleSnapshot {
            rows: self.patch_lifecycle.rows.clone(),
        }
    }

    fn next_endpoint_occurrence(&mut self) -> Sys4Result<u64> {
        let next = self.next_endpoint_occurrence;
        self.next_endpoint_occurrence = next
            .checked_add(1)
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::IdentifierExhausted))?;
        Ok(next)
    }

    fn next_mailbox_token(&mut self, label: &str) -> Sys4Result<String> {
        let next = self.next_endpoint_occurrence()?;
        Ok(format!("sys4-{label}-{next:020}"))
    }

    #[allow(clippy::too_many_arguments)]
    fn enqueue_outbox(
        &mut self,
        edge: &CommunicationEdge,
        request_id: &str,
        payload: MailboxPayload,
        request_carrier_id: Option<String>,
        input_receipt_carrier_id: Option<String>,
        m9_owner_lineage_ref: Option<String>,
        m9_source_release_lineage: Option<M9DesignatedSourceReleaseLineage>,
        semantic_identity: Option<String>,
        immutable_delivery_binding: Option<SealedDeliveryBinding>,
        immutable_delivery_digest: Option<String>,
        predecessors: Vec<String>,
    ) -> Sys4Result<MailboxEnvelope> {
        let envelope_id = self.next_mailbox_token("envelope")?;
        let carrier_id = self.next_mailbox_token("carrier")?;
        let mailbox_record_id = self.next_mailbox_token("outbox-record")?;
        let occurrence = self.next_mailbox_token("outbox-enqueue")?;
        self.causality.record(occurrence.clone(), predecessors);
        let m8_publication_id = immutable_delivery_binding
            .as_ref()
            .map(|binding| binding.m8_publication_id().to_string());
        let (logical_tick_id, logical_tick_frontier) = match &payload {
            MailboxPayload::DesignatedInputRequest { frontier, tick }
            | MailboxPayload::DesignatedInputReceipt { frontier, tick, .. } => {
                (Some(tick.clone()), Some(frontier.clone()))
            }
            _ => (
                immutable_delivery_binding
                    .as_ref()
                    .map(|binding| binding.logical_tick_id().to_string()),
                immutable_delivery_binding
                    .as_ref()
                    .map(|binding| binding.logical_tick_frontier().to_string()),
            ),
        };
        let envelope = MailboxEnvelope {
            envelope_id,
            carrier_id,
            request_id: request_id.to_string(),
            operation_id: edge.operation_id().to_string(),
            edge_kind: edge.kind(),
            edge_ref: edge.edge_ref().to_string(),
            source_locus: edge.source_locus().to_string(),
            target_locus: edge.target_locus().to_string(),
            carrier_contract: edge.carrier_contract().clone(),
            source_ref: edge.source_ref(),
            core_ref: edge.core_ref().map(ToOwned::to_owned),
            source_fragment_ref: edge.source_fragment_ref().clone(),
            target_fragment_ref: edge.target_fragment_ref().clone(),
            mailbox_record_id,
            mailbox_enqueue_occurrence_id: occurrence,
            request_carrier_id,
            input_receipt_carrier_id,
            m9_owner_lineage_ref,
            m9_source_release_lineage,
            semantic_identity,
            immutable_delivery_binding,
            immutable_delivery_digest,
            m8_publication_id,
            m8_evaluation_node_id: None,
            logical_tick_id,
            logical_tick_frontier,
            payload,
        };
        self.loci
            .get_mut(edge.source_locus())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus))?
            .outgoing_mailbox
            .pending
            .push_back(envelope.clone());
        Ok(envelope)
    }

    fn enqueue_local_inbox(
        &mut self,
        edge: &CommunicationEdge,
        request_id: &str,
        payload: MailboxPayload,
        semantic_identity: Option<String>,
        immutable_delivery_binding: Option<SealedDeliveryBinding>,
        immutable_delivery_digest: Option<String>,
    ) -> Sys4Result<MailboxEnvelope> {
        let envelope_id = self.next_mailbox_token("envelope")?;
        let carrier_id = self.next_mailbox_token("carrier")?;
        // A cache retry is consumer-local execution, but it still retains the
        // exact E→C delivery edge and sealed publication binding that it
        // revalidates. Record that local ingress as an endpoint pair so a
        // whole-fabric cut never contains an unaccounted pending inbox.
        let source_record_id = self.next_mailbox_token("cache-retry-source-record")?;
        let source_dispatch_occurrence = self.next_mailbox_token("cache-retry-source-dispatch")?;
        let mailbox_record_id = self.next_mailbox_token("inbox-record")?;
        let occurrence = self.next_mailbox_token("inbox-enqueue")?;
        self.causality
            .record(source_dispatch_occurrence.clone(), Vec::new());
        self.causality
            .record(occurrence.clone(), vec![source_dispatch_occurrence.clone()]);
        let m8_publication_id = immutable_delivery_binding
            .as_ref()
            .map(|binding| binding.m8_publication_id().to_string());
        let logical_tick_id = immutable_delivery_binding
            .as_ref()
            .map(|binding| binding.logical_tick_id().to_string());
        let logical_tick_frontier = immutable_delivery_binding
            .as_ref()
            .map(|binding| binding.logical_tick_frontier().to_string());
        let envelope = MailboxEnvelope {
            envelope_id,
            carrier_id,
            request_id: request_id.to_string(),
            operation_id: edge.operation_id().to_string(),
            edge_kind: edge.kind(),
            edge_ref: edge.edge_ref().to_string(),
            source_locus: edge.source_locus().to_string(),
            target_locus: edge.target_locus().to_string(),
            carrier_contract: edge.carrier_contract().clone(),
            source_ref: edge.source_ref(),
            core_ref: edge.core_ref().map(ToOwned::to_owned),
            source_fragment_ref: edge.source_fragment_ref().clone(),
            target_fragment_ref: edge.target_fragment_ref().clone(),
            mailbox_record_id,
            mailbox_enqueue_occurrence_id: occurrence,
            request_carrier_id: None,
            input_receipt_carrier_id: None,
            m9_owner_lineage_ref: None,
            m9_source_release_lineage: None,
            semantic_identity,
            immutable_delivery_binding,
            immutable_delivery_digest,
            m8_publication_id,
            m8_evaluation_node_id: None,
            logical_tick_id,
            logical_tick_frontier,
            payload,
        };
        let source_record = EndpointCarrierRecord {
            record_id: source_record_id,
            carrier_id: envelope.carrier_id.clone(),
            request_id: envelope.request_id.clone(),
            edge_kind: envelope.edge_kind,
            edge_ref: envelope.edge_ref.clone(),
            source_locus: envelope.source_locus.clone(),
            target_locus: envelope.target_locus.clone(),
            enqueue_occurrence_id: None,
            dequeue_occurrence_id: Some(source_dispatch_occurrence),
            request_carrier_id: None,
            input_receipt_carrier_id: None,
            source_ref: envelope.source_ref.clone(),
            core_ref: envelope.core_ref.clone(),
            source_fragment_ref: envelope.source_fragment_ref.clone(),
            target_fragment_ref: envelope.target_fragment_ref.clone(),
        };
        let target_record = EndpointCarrierRecord {
            record_id: envelope.mailbox_record_id.clone(),
            carrier_id: envelope.carrier_id.clone(),
            request_id: envelope.request_id.clone(),
            edge_kind: envelope.edge_kind,
            edge_ref: envelope.edge_ref.clone(),
            source_locus: envelope.source_locus.clone(),
            target_locus: envelope.target_locus.clone(),
            enqueue_occurrence_id: Some(envelope.mailbox_enqueue_occurrence_id.clone()),
            dequeue_occurrence_id: None,
            request_carrier_id: None,
            input_receipt_carrier_id: None,
            source_ref: envelope.source_ref.clone(),
            core_ref: envelope.core_ref.clone(),
            source_fragment_ref: envelope.source_fragment_ref.clone(),
            target_fragment_ref: envelope.target_fragment_ref.clone(),
        };
        self.loci
            .get_mut(edge.source_locus())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus))?
            .outgoing_endpoint
            .append(source_record);
        self.loci
            .get_mut(edge.target_locus())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus))?
            .incoming_endpoint
            .append(target_record);
        self.loci
            .get_mut(edge.target_locus())
            .expect("validated target retains its local inbox")
            .incoming_mailbox
            .pending
            .push_back(envelope.clone());
        Ok(envelope)
    }

    pub(crate) fn submit_source_action(
        &mut self,
        action: SourceAction,
    ) -> Sys4Result<FabricSubmission> {
        let request_id = self.next_request_id()?;
        let (edge, payload, owner_lineage, release_lineage, semantic_identity) = match &action.kind
        {
            SourceActionKind::OwnerOperation(_) => {
                let route = self
                    .program
                    .derive_route_for_external_action(&ExternalAction::source_operation(
                        action.clone(),
                    ))?
                    .clone();
                let edge = self
                    .program
                    .projection
                    .communication_plan()
                    .edges()
                    .iter()
                    .find(|edge| edge.edge_ref() == route.edge_ref)
                    .cloned()
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::ProgramProjectionMismatch)
                    })?;
                if edge.target_locus() != route.key.target_locus
                    || edge.source_locus() != route.key.source_locus
                {
                    return Err(Sys4DispatchDiagnostics::one(
                        Sys4DiagnosticKind::WrongTargetLocus,
                    ));
                }
                let owner_lineage = self
                    .authority_generation
                    .owner_lineage_ref(action.operation_id(), edge.target_locus())
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected)
                    })?;
                (
                    edge,
                    MailboxPayload::OwnerRequest {
                        arguments: action.arguments.clone(),
                    },
                    Some(owner_lineage),
                    None,
                    None,
                )
            }
            SourceActionKind::DesignatedTick(_) => {
                let evaluator = self
                    .program
                    .designated_evaluator_fragment(action.operation_id())
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable)
                    })?;
                let core = evaluator
                    .designated_checked_core()
                    .expect("projected evaluator retains Core");
                let source_owner = self
                    .program
                    .projection
                    .sys4_artifact_fragments()
                    .entries()
                    .iter()
                    .find(|fragment| {
                        fragment.operation_id() == action.operation_id()
                            && fragment.fragment_kind()
                                == ProjectedOperationFragmentKind::DesignatedRemoteInputService
                    })
                    .and_then(|fragment| {
                        fragment
                            .designated_remote_input_dependency()
                            .map(|dependency| dependency.source_owner_locus().to_string())
                    })
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable)
                    })?;
                let edge = self.edge_for(
                    action.operation_id(),
                    CommunicationEdgeKind::DesignatedInputRequest,
                    core.evaluator(),
                    &source_owner,
                )?;
                let frontier = core.trigger().frontier().unwrap_or_default().to_string();
                let release = self
                    .current_m9_authority_inspection()
                    .designated_source_release_lineage(
                        core.evaluator(),
                        core.result(),
                        &source_owner,
                        0,
                        &frontier,
                    )
                    .cloned()
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(
                            Sys4DiagnosticKind::MissingSourceReleaseAuthority,
                        )
                    })?;
                let (given_frontier, tick) = action.tick.clone().ok_or_else(|| {
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::MissingDesignatedTick)
                })?;
                if given_frontier != frontier || tick.is_empty() {
                    return Err(Sys4DispatchDiagnostics::one(
                        Sys4DiagnosticKind::MissingDesignatedTick,
                    ));
                }
                (
                    edge,
                    MailboxPayload::DesignatedInputRequest {
                        frontier: given_frontier,
                        tick,
                    },
                    None,
                    Some(release),
                    None,
                )
            }
            SourceActionKind::ConsumeDesignatedResult(_) => {
                let consumer = self
                    .program
                    .designated_consumer_fragment(action.operation_id())
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable)
                    })?;
                let core = consumer
                    .designated_result_consumer_core()
                    .expect("projected consumer retains Core");
                let evaluator = self
                    .program
                    .designated_evaluator_fragment(action.operation_id())
                    .ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::RouteUnavailable)
                    })?;
                let evaluator_core = evaluator
                    .designated_checked_core()
                    .expect("projected evaluator retains Core");
                let edge = self.edge_for(
                    action.operation_id(),
                    CommunicationEdgeKind::DesignatedResultDelivery,
                    evaluator_core.evaluator(),
                    core.consumer_locus(),
                )?;
                let cached = self
                    .cache
                    .values()
                    .find(|entry| {
                        entry.operation == action.operation_id()
                            && entry.consumer_locus == core.consumer_locus()
                    })
                    .cloned();
                if cached.is_none() {
                    let delivery = self
                        .loci
                        .values()
                        .flat_map(|runtime| runtime.outgoing_mailbox.pending.iter())
                        .find(|envelope| {
                            envelope.operation_id == action.operation_id()
                                && envelope.edge_kind
                                    == CommunicationEdgeKind::DesignatedResultDelivery
                        })
                        .cloned()
                        .ok_or_else(|| {
                            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::MissingPublishedResult)
                        })?;
                    return Ok(FabricSubmission {
                        request_id: delivery.request_id,
                        envelope_id: delivery.envelope_id,
                        carrier_id: delivery.carrier_id,
                        operation_id: delivery.operation_id,
                        origin_locus: delivery.source_locus,
                        target_locus: delivery.target_locus,
                    });
                }
                let cached = cached.expect("cache branch checked above");
                let envelope = self.enqueue_local_inbox(
                    &edge,
                    &request_id,
                    MailboxPayload::CacheRetry,
                    Some(cached.semantic_identity.clone()),
                    Some(cached.sealed_delivery_binding.clone()),
                    Some(cached.sealed_delivery_binding_digest.clone()),
                )?;
                self.trace.append(
                    &request_id,
                    Some(cached.delivery_id.clone()),
                    action.operation_id(),
                    Sys4TraceKind::RequestAdmitted,
                    Some(edge.kind()),
                );
                return Ok(FabricSubmission {
                    request_id,
                    envelope_id: envelope.envelope_id,
                    carrier_id: envelope.carrier_id,
                    operation_id: action.operation_id().to_string(),
                    origin_locus: edge.source_locus().to_string(),
                    target_locus: edge.target_locus().to_string(),
                });
            }
        };
        self.trace.append(
            &request_id,
            None,
            action.operation_id(),
            Sys4TraceKind::RequestAdmitted,
            Some(edge.kind()),
        );
        let envelope = self.enqueue_outbox(
            &edge,
            &request_id,
            payload,
            None,
            None,
            owner_lineage,
            release_lineage,
            semantic_identity,
            None,
            None,
            Vec::new(),
        )?;
        Ok(FabricSubmission {
            request_id,
            envelope_id: envelope.envelope_id,
            carrier_id: envelope.carrier_id,
            operation_id: action.operation_id().to_string(),
            origin_locus: edge.source_locus().to_string(),
            target_locus: edge.target_locus().to_string(),
        })
    }

    pub(crate) fn step_transport(
        &mut self,
        source: &str,
        target: &str,
        envelope_id: &str,
    ) -> Sys4Result<TransportStep> {
        let (index, envelope) = {
            let runtime = self.loci.get(source).ok_or_else(|| {
                Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus)
            })?;
            let index = runtime
                .outgoing_mailbox
                .pending
                .iter()
                .position(|entry| entry.envelope_id == envelope_id)
                .ok_or_else(|| {
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::UnavailableEnvelope)
                })?;
            (index, runtime.outgoing_mailbox.pending[index].clone())
        };
        if envelope.target_locus != target || envelope.source_locus != source {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::WrongTargetLocus,
            ));
        }
        let known_edge = self
            .program
            .projection
            .communication_plan()
            .edges()
            .iter()
            .any(|edge| edge.edge_ref() == envelope.edge_ref);
        if !known_edge {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::UnknownProjectedEdge,
            ));
        }
        if self.route_faults.contains(&envelope.edge_ref) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::RouteUnavailable,
            ));
        }
        if let Some(fault) = self
            .in_transit_faults
            .take_exact(&envelope.edge_ref, &envelope.envelope_id)
        {
            if matches!(fault.kind, FaultInjectionKind::Retarget) {
                let _ = self
                    .loci
                    .get_mut(source)
                    .expect("source checked above")
                    .outgoing_mailbox
                    .pending
                    .remove(index);
                let mut diagnostic =
                    Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus);
                diagnostic.context.rejected_envelope_id = Some(envelope.envelope_id.clone());
                diagnostic.context.retarget_fault = Some(Box::new(RetargetFaultInspection {
                    evidence_id: format!(
                        "sys4-retarget:{}:{}:{}",
                        fault.edge_ref,
                        envelope.envelope_id,
                        fault.target_locus.as_deref().unwrap_or("")
                    ),
                    edge_ref: fault.edge_ref,
                    envelope_id: envelope.envelope_id.clone(),
                    attempted_target_locus: fault.target_locus.unwrap_or_default(),
                    rejected_at_fault_admission: false,
                    target_enqueue_occurrence_id: None,
                }));
                return Err(diagnostic);
            }
            let mut moved = envelope.clone();
            match fault.kind {
                FaultInjectionKind::StripIntPayload => match &mut moved.payload {
                    MailboxPayload::DesignatedDelivery { value, .. } => *value = None,
                    MailboxPayload::DesignatedInputReceipt { source_value, .. } => {
                        *source_value = None
                    }
                    _ => {}
                },
                FaultInjectionKind::CorruptPolicyStamp => {
                    moved.immutable_delivery_digest = Some("sys4-corrupted-policy".to_string())
                }
                FaultInjectionKind::CorruptVisibilityRedaction => {
                    if let Some(binding) = &mut moved.immutable_delivery_binding {
                        binding.redaction_policy = "sys4-corrupted-redaction".to_string();
                    }
                }
                FaultInjectionKind::CorruptM8PublicationId => {
                    moved.m8_publication_id = fault.replacement_m8_publication_id;
                }
                FaultInjectionKind::CorruptSourceRef => {
                    moved.source_ref = fault
                        .replacement_source_ref
                        .expect("source-ref fault retains its replacement");
                }
                FaultInjectionKind::CorruptVisibility => {
                    if let Some(binding) = &mut moved.immutable_delivery_binding {
                        binding.m8_visibility_label = fault
                            .replacement_visibility_label
                            .expect("visibility fault retains its replacement");
                        moved.immutable_delivery_digest = Some(format!("{binding:?}"));
                    }
                }
                FaultInjectionKind::CorruptCacheBindingDigest => {
                    moved.immutable_delivery_digest =
                        Some("sys4-corrupted-cache-binding".to_string())
                }
                FaultInjectionKind::RouteUnavailable
                | FaultInjectionKind::Retarget
                | FaultInjectionKind::RewriteCacheRetryProjectionBinding => {}
            }
            return self.move_envelope(source, target, index, moved);
        }
        self.move_envelope(source, target, index, envelope)
    }

    fn move_envelope(
        &mut self,
        source: &str,
        target: &str,
        index: usize,
        mut envelope: MailboxEnvelope,
    ) -> Sys4Result<TransportStep> {
        let old_record_id = envelope.mailbox_record_id.clone();
        let dequeue_occurrence = self.next_mailbox_token("outbox-dequeue")?;
        self.causality.record(
            dequeue_occurrence.clone(),
            vec![envelope.mailbox_enqueue_occurrence_id.clone()],
        );
        let _ = self
            .loci
            .get_mut(source)
            .expect("validated source")
            .outgoing_mailbox
            .pending
            .remove(index);
        let target_record_id = self.next_mailbox_token("inbox-record")?;
        let enqueue_occurrence = self.next_mailbox_token("inbox-enqueue")?;
        self.causality
            .record(enqueue_occurrence.clone(), vec![dequeue_occurrence.clone()]);
        envelope.mailbox_record_id = target_record_id.clone();
        envelope.mailbox_enqueue_occurrence_id = enqueue_occurrence.clone();
        let out_record = EndpointCarrierRecord {
            record_id: old_record_id.clone(),
            carrier_id: envelope.carrier_id.clone(),
            request_id: envelope.request_id.clone(),
            edge_kind: envelope.edge_kind,
            edge_ref: envelope.edge_ref.clone(),
            source_locus: source.to_string(),
            target_locus: target.to_string(),
            enqueue_occurrence_id: None,
            dequeue_occurrence_id: Some(dequeue_occurrence.clone()),
            request_carrier_id: envelope.request_carrier_id.clone(),
            input_receipt_carrier_id: envelope.input_receipt_carrier_id.clone(),
            source_ref: envelope.source_ref.clone(),
            core_ref: envelope.core_ref.clone(),
            source_fragment_ref: envelope.source_fragment_ref.clone(),
            target_fragment_ref: envelope.target_fragment_ref.clone(),
        };
        let in_record = EndpointCarrierRecord {
            record_id: target_record_id.clone(),
            carrier_id: envelope.carrier_id.clone(),
            request_id: envelope.request_id.clone(),
            edge_kind: envelope.edge_kind,
            edge_ref: envelope.edge_ref.clone(),
            source_locus: source.to_string(),
            target_locus: target.to_string(),
            enqueue_occurrence_id: Some(enqueue_occurrence.clone()),
            dequeue_occurrence_id: None,
            request_carrier_id: envelope.request_carrier_id.clone(),
            input_receipt_carrier_id: envelope.input_receipt_carrier_id.clone(),
            source_ref: envelope.source_ref.clone(),
            core_ref: envelope.core_ref.clone(),
            source_fragment_ref: envelope.source_fragment_ref.clone(),
            target_fragment_ref: envelope.target_fragment_ref.clone(),
        };
        self.loci
            .get_mut(source)
            .expect("validated source retains its outgoing endpoint")
            .outgoing_endpoint
            .append(out_record.clone());
        self.loci
            .get_mut(target)
            .expect("validated target retains its incoming endpoint")
            .incoming_endpoint
            .append(in_record.clone());
        self.loci
            .get_mut(target)
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus))?
            .incoming_mailbox
            .pending
            .push_back(envelope.clone());
        match envelope.edge_kind {
            CommunicationEdgeKind::OwnerReplyReceipt => {
                self.trace.append_endpoint(
                    &envelope.request_id,
                    &envelope.operation_id,
                    Sys4TraceKind::ReplyDispatched,
                    &out_record,
                    &dequeue_occurrence,
                );
                self.trace.append_endpoint(
                    &envelope.request_id,
                    &envelope.operation_id,
                    Sys4TraceKind::ReplyReceived,
                    &in_record,
                    &enqueue_occurrence,
                );
            }
            CommunicationEdgeKind::DesignatedResultDelivery => {
                self.trace.append_designated_delivery_endpoint(
                    &envelope.request_id,
                    envelope.m8_publication_id(),
                    &envelope.operation_id,
                    Sys4TraceKind::DesignatedResultDispatched,
                    &out_record,
                    &dequeue_occurrence,
                );
                self.trace.append_designated_delivery_endpoint(
                    &envelope.request_id,
                    envelope.m8_publication_id(),
                    &envelope.operation_id,
                    Sys4TraceKind::DesignatedResultReceived,
                    &in_record,
                    &enqueue_occurrence,
                );
            }
            _ => {
                self.trace.append_endpoint(
                    &envelope.request_id,
                    &envelope.operation_id,
                    Sys4TraceKind::Dispatched,
                    &out_record,
                    &dequeue_occurrence,
                );
                self.trace.append_endpoint(
                    &envelope.request_id,
                    &envelope.operation_id,
                    Sys4TraceKind::Received,
                    &in_record,
                    &enqueue_occurrence,
                );
            }
        }
        Ok(TransportStep {
            envelope_id: envelope.envelope_id,
            carrier_id: envelope.carrier_id,
            source_outbox_dequeue_record_id: old_record_id,
            source_outbox_dequeue_occurrence_id: dequeue_occurrence,
            target_inbox_enqueue_record_id: target_record_id,
            target_inbox_enqueue_occurrence_id: enqueue_occurrence,
        })
    }

    fn dequeue_locus(&mut self, locus: &str) -> Sys4Result<(MailboxEnvelope, String)> {
        let envelope = self
            .loci
            .get_mut(locus)
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus))?
            .incoming_mailbox
            .pending
            .pop_front()
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::UnavailableEnvelope))?;
        let occurrence = self.next_mailbox_token("locus-dequeue")?;
        self.causality.record(
            occurrence.clone(),
            vec![envelope.mailbox_enqueue_occurrence_id.clone()],
        );
        Ok((envelope, occurrence))
    }

    fn quarantine(
        &mut self,
        locus: &str,
        envelope: &MailboxEnvelope,
        kind: Sys4DiagnosticKind,
        request_id: &str,
    ) -> Sys4DispatchDiagnostics {
        self.loci
            .get_mut(locus)
            .expect("validated locus")
            .incoming_mailbox
            .terminal
            .insert(
                envelope.envelope_id.clone(),
                TerminalRejectedMailboxEnvelope {
                    envelope_id: envelope.envelope_id.clone(),
                    terminal_state: MailboxEnvelopeTerminalState::RejectedQuarantined,
                    diagnostic_kind: kind,
                    observer_safe_audit: ObserverSafeMailboxAudit,
                },
            );
        let mut diagnostic = Sys4DispatchDiagnostics::one(kind);
        diagnostic.context.rejected_envelope_id = Some(envelope.envelope_id.clone());
        diagnostic.context.rejected_request_id = Some(request_id.to_string());
        diagnostic
    }

    pub(crate) fn step_locus(&mut self, locus: &str) -> Sys4Result<LocusStep> {
        let (envelope, dequeue_occurrence) = self.dequeue_locus(locus)?;
        let base = |m9_validation, receipt: Option<FabricReceipt>| LocusStep {
            consumed_envelope_id: envelope.envelope_id.clone(),
            locus_dequeue_record_id: envelope.mailbox_record_id.clone(),
            locus_dequeue_occurrence_id: dequeue_occurrence.clone(),
            m9_validation,
            m8_request_node_id: None,
            m8_serve_node_id: None,
            m8_input_receipt_node_id: None,
            m8_evaluation_node_id: None,
            m8_non_consuming_validation_node_id: None,
            reply_envelope_id: None,
            local_store_read_audit: None,
            local_store_reads: Vec::new(),
            receipt,
            request_id: envelope.request_id.clone(),
            semantic_identity: envelope.semantic_identity.clone(),
            m9_cache_validation: None,
        };
        match (&envelope.edge_kind, &envelope.payload) {
            (CommunicationEdgeKind::OwnerRequest, MailboxPayload::OwnerRequest { arguments }) => {
                let fragment = self
                    .program
                    .owner_execution_fragment(&envelope.operation_id)
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::RouteUnavailable,
                            &envelope.request_id,
                        )
                    })?;
                let core = fragment
                    .owner_rmw_checked_core()
                    .expect("projected owner Core");
                if locus != core.owner_locus() {
                    return Err(self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::WrongTargetLocus,
                        &envelope.request_id,
                    ));
                }
                // Pure carrier/provenance checks must finish before M9
                // records an admitted owner-operation validation occurrence.
                // A forged lineage is not a successful validation merely
                // because the projected operation itself is authority-bound.
                self.authority_generation
                    .owner_authority_for_operation(&envelope.operation_id, locus)
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::M8ExecutionRejected,
                            &envelope.request_id,
                        )
                    })?;
                let lineage = self
                    .authority_generation
                    .owner_lineage_ref(&envelope.operation_id, locus)
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::M8ExecutionRejected,
                            &envelope.request_id,
                        )
                    })?;
                if envelope.m9_owner_lineage_ref.as_deref() != Some(lineage.as_str()) {
                    return Err(self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::M8ExecutionRejected,
                        &envelope.request_id,
                    ));
                }
                let (_, authority) = self
                    .authority_generation
                    .validate_owner_operation(&envelope.operation_id, locus, &envelope.request_id)
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::M8ExecutionRejected,
                            &envelope.request_id,
                        )
                    })?;
                let mut request =
                    M8OwnerRequest::new(&envelope.operation_id).with_authority_use(authority);
                for (name, value) in arguments {
                    request = request.with_argument(name, value);
                }
                let context =
                    M8LocalDesignatedTraceContext::new(envelope.envelope_id(), "", "", "", "", "")
                        .with_operation_id(&envelope.operation_id)
                        .with_owner_locus(locus)
                        .with_edge_ref(&envelope.edge_ref);
                let execution = match self.backend.enqueue_and_serve(locus, request, context) {
                    Ok(execution) => execution,
                    Err(failure) => {
                        self.refresh_m8_local_runtime_trace(locus);
                        let failure = self.fabric_qualified_m8_failure_for_locus(locus, failure);
                        let mut diagnostic =
                            self.quarantine(locus, &envelope, failure.kind, &envelope.request_id);
                        diagnostic.context.endpoint_dequeue_occurrence_id =
                            Some(dequeue_occurrence.clone());
                        if let Some(failure) = failure.observation {
                            self.causality.record(
                                failure.node_id().to_string(),
                                vec![dequeue_occurrence.clone()],
                            );
                            self.actual_m8_trace.append(
                                failure.node_id().to_string(),
                                "OwnerOperationRejected",
                                Some(envelope.request_id.clone()),
                                Some(envelope.operation_id.clone()),
                                Some(locus.to_string()),
                                self.causality.predecessor_ids(failure.node_id()),
                            );
                            diagnostic.context.m8_trace_node_id =
                                Some(failure.node_id().to_string());
                            diagnostic.context.backend_m8_failure = Some(failure);
                        }
                        return Err(diagnostic);
                    }
                };
                // The worker owns its M8 trace.  Refresh the observer snapshot
                // before interpreting either a successful RMW or an M8
                // declared failure so both backends expose the exact rows that
                // were returned by the typed backend outcome.
                self.refresh_m8_local_runtime_trace(locus);
                self.associate_m8_envelope_request(&envelope);
                let request_observation = self.fabric_qualified_m8_observation_for_locus(
                    locus,
                    &execution.request_observation,
                );
                let serve_observation = self
                    .fabric_qualified_m8_observation_for_locus(locus, &execution.serve_observation);
                let request_node_id = request_observation.node_id().to_string();
                let serve_node_id = serve_observation.node_id().to_string();
                self.causality
                    .record(request_node_id.clone(), vec![dequeue_occurrence.clone()]);
                // The generated owner request → serve relation is explicit
                // fabric causality in addition to M8's owner-local read
                // chain. A later full recovery keeps it for ordinary live
                // execution, while replacing stale worker-only edges.
                self.causality
                    .record(serve_node_id.clone(), vec![request_node_id.clone()]);
                self.actual_m8_trace.append(
                    request_node_id.clone(),
                    "OwnerRequest",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(&request_node_id),
                );
                self.actual_m8_trace.append(
                    serve_node_id.clone(),
                    "OwnerServe",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(&serve_node_id),
                );
                if execution.outcome.failure().is_some() {
                    let mut diagnostic = self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::M8ExecutionRejected,
                        &envelope.request_id,
                    );
                    diagnostic.context.endpoint_dequeue_occurrence_id =
                        Some(dequeue_occurrence.clone());
                    diagnostic.context.m8_trace_node_id = Some(serve_node_id.clone());
                    diagnostic.context.backend_m8_failure = Some(Box::new(serve_observation));
                    return Err(diagnostic);
                }
                let mut reads = Vec::new();
                let mut writes = Vec::new();
                let mut seen_reads = BTreeSet::new();
                for read in std::iter::once(core.target()).chain(core.same_owner_reads().iter()) {
                    let key = m8_key_for_read(read, arguments);
                    if seen_reads.insert(key.clone()) {
                        let value = execution.outcome.read_int(&key).unwrap_or_default();
                        reads.push(RuntimeStoreRead::int(
                            locus,
                            key.namespace(),
                            key.index(),
                            key.field(),
                            value,
                        ));
                    }
                }
                let target_key = m8_key_for_read(core.target(), arguments);
                if let Some(value) = execution.outcome.written_int(&target_key) {
                    self.loci
                        .get_mut(locus)
                        .expect("owner local runtime")
                        .local_store
                        .set_int(
                            target_key.namespace(),
                            target_key.index(),
                            target_key.field(),
                            value,
                        );
                    writes.push(RuntimeStoreWrite::int(
                        locus,
                        target_key.namespace(),
                        target_key.index(),
                        target_key.field(),
                        value,
                    ));
                }
                self.trace.append(
                    &envelope.request_id,
                    None,
                    &envelope.operation_id,
                    Sys4TraceKind::Served,
                    Some(envelope.edge_kind),
                );
                self.trace.append(
                    &envelope.request_id,
                    None,
                    &envelope.operation_id,
                    Sys4TraceKind::M8OwnerRead,
                    Some(envelope.edge_kind),
                );
                self.trace.append(
                    &envelope.request_id,
                    None,
                    &envelope.operation_id,
                    Sys4TraceKind::M8OwnerWrite,
                    Some(envelope.edge_kind),
                );
                let receipt = FabricReceipt {
                    request_id: envelope.request_id.clone(),
                    delivery_id: envelope.carrier_id.clone(),
                    operation_id: envelope.operation_id.clone(),
                    origin_locus: envelope.source_locus.clone(),
                    target_locus: locus.to_string(),
                    typed_value: RuntimeValue::unit(),
                    result_version: None,
                    owner_rmw: Some(OwnerRmwReport {
                        reads,
                        writes,
                        source_ref: envelope.source_ref.path.clone(),
                        core_ref: envelope.core_ref.clone().unwrap_or_default(),
                    }),
                    performed_m8_consumption: false,
                    returned_from_cache: false,
                    semantic_consumption_identity: None,
                    fault_id: None,
                    m9_cache_validation: None,
                    m8_non_consuming_validation_node_id: None,
                    m8_publication_id: None,
                    logical_tick_id: None,
                    logical_tick_frontier: None,
                };
                let reply_edge = self.edge_for(
                    &envelope.operation_id,
                    CommunicationEdgeKind::OwnerReplyReceipt,
                    locus,
                    &envelope.source_locus,
                )?;
                let reply = self.enqueue_outbox(
                    &reply_edge,
                    &envelope.request_id,
                    MailboxPayload::OwnerReply {
                        receipt: Box::new(receipt.clone()),
                    },
                    Some(envelope.carrier_id.clone()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    vec![serve_node_id.clone()],
                )?;
                let mut step = base(
                    LocusM9Validation::Owner {
                        owner_lineage_ref: lineage,
                    },
                    None,
                );
                step.m8_request_node_id = Some(request_node_id);
                step.m8_serve_node_id = Some(serve_node_id);
                step.reply_envelope_id = Some(reply.envelope_id);
                Ok(step)
            }
            (CommunicationEdgeKind::OwnerReplyReceipt, MailboxPayload::OwnerReply { receipt }) => {
                self.completed_receipts
                    .insert(envelope.request_id.clone(), (**receipt).clone());
                Ok(base(LocusM9Validation::None, Some((**receipt).clone())))
            }
            (
                CommunicationEdgeKind::DesignatedInputRequest,
                MailboxPayload::DesignatedInputRequest { frontier, tick },
            ) => {
                let fragments = self.program.projection.sys4_artifact_fragments();
                let service = fragments
                    .entries()
                    .iter()
                    .find(|fragment| {
                        fragment.operation_id() == envelope.operation_id
                            && fragment.fragment_kind()
                                == ProjectedOperationFragmentKind::DesignatedRemoteInputService
                    })
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::RouteUnavailable,
                            &envelope.request_id,
                        )
                    })?;
                let dependency = service
                    .designated_remote_input_dependency()
                    .expect("source service retains dependency");
                let evaluator = self
                    .program
                    .designated_evaluator_fragment(&envelope.operation_id)
                    .expect("source service has evaluator");
                let core = evaluator
                    .designated_checked_core()
                    .expect("evaluator retains Core");
                let release = self
                    .authority_generation
                    .validate_designated_source_release(
                        &envelope.operation_id,
                        core.evaluator(),
                        core.result(),
                        locus,
                        0,
                        frontier,
                        envelope.m9_source_release_lineage(),
                        envelope.envelope_id(),
                    )
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::MissingSourceReleaseAuthority,
                            &envelope.request_id,
                        )
                    })?;
                self.causality.record(
                    release.occurrence_id().to_string(),
                    vec![dequeue_occurrence.clone()],
                );
                let read = dependency.typed_state_read();
                let source_key = m8_key_for_read(read, &BTreeMap::new());
                let read_context = M8LocalDesignatedTraceContext::new(
                    envelope.envelope_id(),
                    envelope.semantic_identity(),
                    "",
                    "",
                    tick,
                    frontier,
                )
                .with_operation_id(&envelope.operation_id)
                .with_owner_locus(locus)
                .with_evaluator_locus(core.evaluator())
                .with_edge_ref(&envelope.edge_ref);
                let (source_value, read_observation) = match self
                    .backend
                    .read_owner_int_with_context(
                        locus,
                        source_key.clone(),
                        read.source_ref().clone(),
                        read_context,
                    ) {
                    Ok(Some(read)) => read,
                    Ok(None) => {
                        return Err(self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::MissingTypedDesignatedValue,
                            &envelope.request_id,
                        ));
                    }
                    Err(kind) => {
                        self.refresh_m8_local_runtime_trace(locus);
                        return Err(self.quarantine(locus, &envelope, kind, &envelope.request_id));
                    }
                };
                self.refresh_m8_local_runtime_trace(locus);
                let read_observation =
                    self.fabric_qualified_m8_observation_for_locus(locus, &read_observation);
                let audit = LocalStoreReadAudit {
                    occurrence_id: read_observation.node_id().to_string(),
                    reads: vec![RuntimeStoreRead::int(
                        locus,
                        read.namespace(),
                        source_key.index(),
                        source_key.field(),
                        source_value,
                    )],
                };
                self.causality.record(
                    audit.occurrence_id.clone(),
                    vec![release.occurrence_id().to_string()],
                );
                self.actual_m8_trace.append(
                    read_observation.node_id().to_string(),
                    "OwnerRead",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(read_observation.node_id()),
                );
                self.local_store_read_audits
                    .insert(locus.to_string(), audit.clone());
                let receipt_edge = self.edge_for(
                    &envelope.operation_id,
                    CommunicationEdgeKind::DesignatedInputReceipt,
                    locus,
                    core.evaluator(),
                )?;
                let reply = self.enqueue_outbox(
                    &receipt_edge,
                    &envelope.request_id,
                    MailboxPayload::DesignatedInputReceipt {
                        source_value: Some(source_value),
                        frontier: frontier.clone(),
                        tick: tick.clone(),
                    },
                    Some(envelope.carrier_id.clone()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    vec![audit.occurrence_id.clone()],
                )?;
                let mut step = base(
                    LocusM9Validation::SourceRelease {
                        inspection: release,
                    },
                    None,
                );
                step.local_store_read_audit = Some(audit.clone());
                step.local_store_reads = audit.reads;
                step.reply_envelope_id = Some(reply.envelope_id);
                Ok(step)
            }
            (
                CommunicationEdgeKind::DesignatedInputReceipt,
                MailboxPayload::DesignatedInputReceipt {
                    source_value,
                    frontier,
                    tick,
                },
            ) => {
                let value = source_value.ok_or_else(|| {
                    self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::MissingTypedDesignatedValue,
                        &envelope.request_id,
                    )
                })?;
                let evaluator = self
                    .program
                    .designated_evaluator_fragment(&envelope.operation_id)
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::RouteUnavailable,
                            &envelope.request_id,
                        )
                    })?;
                let core = evaluator
                    .designated_checked_core()
                    .expect("projected evaluator Core");
                let fragments = self.program.projection.sys4_artifact_fragments();
                let service = fragments
                    .entries()
                    .iter()
                    .find(|fragment| {
                        fragment.operation_id() == envelope.operation_id
                            && fragment.fragment_kind()
                                == ProjectedOperationFragmentKind::DesignatedRemoteInputService
                    })
                    .expect("input receipt has service");
                let dependency = service
                    .designated_remote_input_dependency()
                    .expect("service retains dependency");
                let read = dependency.typed_state_read();
                let receipts = M8InputReceiptSet::new().with_receipt(
                    M8InputReceipt::live(format!("sys4-received-input:{}", envelope.carrier_id))
                        .for_state_read(m8_key_for_read(read, &BTreeMap::new()))
                        .with_source_owner_locus(dependency.source_owner_locus())
                        .with_evaluator(core.evaluator())
                        .with_input_frontier(core.trigger().frontier().unwrap_or_default())
                        .with_source_ref(read.source_ref())
                        .with_label(
                            EvidenceSecurityLabel::new("sys4:designated-input")
                                .with_class(M8SecurityClass::Restricted),
                        )
                        .with_int_value(value),
                );
                if let Err(kind) = self
                    .backend
                    .replace_designated_input_receipts(locus, receipts)
                {
                    return Err(self.quarantine(locus, &envelope, kind, &envelope.request_id));
                }
                let authority = self
                    .authority_generation
                    .designated_evaluation_authority_use(core.evaluator(), core.result())
                    .ok_or_else(|| {
                        self.quarantine(
                            locus,
                            &envelope,
                            Sys4DiagnosticKind::MissingEvaluatorAuthority,
                            &envelope.request_id,
                        )
                    })?;
                let consumer = self
                    .program
                    .designated_consumer_fragment(&envelope.operation_id)
                    .expect("evaluator has consumer");
                let consumer_core = consumer
                    .designated_result_consumer_core()
                    .expect("consumer Core");
                let delivery_edge = self.edge_for(
                    &envelope.operation_id,
                    CommunicationEdgeKind::DesignatedResultDelivery,
                    locus,
                    consumer_core.consumer_locus(),
                )?;
                let semantic_identity = semantic_consumption_identity(
                    self.program.checked_program_identity(),
                    core.evaluator(),
                    core.result(),
                    consumer_core.consumer_locus(),
                    core.input_frontier(),
                    core.result_frontier(),
                    core.result_version(),
                    core.observation_policy(),
                    core.policy_stamp(),
                );
                let evaluation_context = M8LocalDesignatedTraceContext::new(
                    envelope.envelope_id(),
                    &semantic_identity,
                    consumer_core.consumer_locus(),
                    "",
                    tick.clone(),
                    frontier.clone(),
                )
                .with_operation_id(&envelope.operation_id)
                .with_evaluator_locus(locus)
                .with_edge_ref(delivery_edge.edge_ref());
                let execution = match self.backend.evaluate_designated(
                    locus,
                    M8DesignatedEvaluationRequest::for_value(&envelope.operation_id)
                        .with_tick(
                            M8DesignatedTick::new(tick.clone())
                                .with_input_frontier(frontier.clone()),
                        )
                        .with_authority(authority),
                    evaluation_context,
                ) {
                    Ok(execution) => execution,
                    Err(failure) => {
                        self.refresh_m8_local_runtime_trace(locus);
                        let failure = self.fabric_qualified_m8_failure_for_locus(locus, failure);
                        let mut diagnostic =
                            self.quarantine(locus, &envelope, failure.kind, &envelope.request_id);
                        diagnostic.context.endpoint_dequeue_occurrence_id =
                            Some(dequeue_occurrence.clone());
                        if let Some(failure) = failure.observation {
                            self.causality.record(
                                failure.node_id().to_string(),
                                vec![dequeue_occurrence.clone()],
                            );
                            self.actual_m8_trace.append(
                                failure.node_id().to_string(),
                                "DesignatedEvaluationRejected",
                                Some(envelope.request_id.clone()),
                                Some(envelope.operation_id.clone()),
                                Some(locus.to_string()),
                                self.causality.predecessor_ids(failure.node_id()),
                            );
                            diagnostic.context.m8_trace_node_id =
                                Some(failure.node_id().to_string());
                            diagnostic.context.backend_m8_failure = Some(failure);
                        }
                        return Err(diagnostic);
                    }
                };
                self.refresh_m8_local_runtime_trace(locus);
                let published = execution.published;
                let input_observation = self
                    .fabric_qualified_m8_observation_for_locus(locus, &execution.input_observation);
                let evaluation_observation = self.fabric_qualified_m8_observation_for_locus(
                    locus,
                    &execution.evaluation_observation,
                );
                let input_node = input_observation.node_id().to_string();
                self.causality
                    .record(input_node.clone(), vec![dequeue_occurrence.clone()]);
                let evaluation_node = evaluation_observation.node_id().to_string();
                self.causality
                    .record(evaluation_node.clone(), vec![input_node.clone()]);
                self.actual_m8_trace.append(
                    input_node.clone(),
                    "DesignatedInputReceipt",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(&input_node),
                );
                self.actual_m8_trace.append(
                    evaluation_node.clone(),
                    "DesignatedValueEvaluated",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(&evaluation_node),
                );
                let binding = SealedDeliveryBinding {
                    carrier_contract: delivery_edge.carrier_contract().clone(),
                    source_ref: delivery_edge.source_ref(),
                    core_ref: delivery_edge.core_ref().map(ToOwned::to_owned),
                    source_fragment_ref: delivery_edge.source_fragment_ref().clone(),
                    target_fragment_ref: delivery_edge.target_fragment_ref().clone(),
                    input_frontier: delivery_edge
                        .carrier_contract()
                        .input_frontier()
                        .map(|value| format!("{:?}", value)),
                    result_frontier: delivery_edge
                        .carrier_contract()
                        .result_frontier()
                        .map(|value| format!("{:?}", value)),
                    result_version: delivery_edge.carrier_contract().result_version(),
                    consumer_locus: consumer_core.consumer_locus().to_string(),
                    policy_stamp: delivery_edge
                        .carrier_contract()
                        .policy_stamp()
                        .map(|value| format!("{:?}", value)),
                    visibility_policy: delivery_edge.carrier_contract().visibility_policy().clone(),
                    redaction_policy: format!(
                        "{:?}",
                        delivery_edge.carrier_contract().visibility_policy()
                    ),
                    m8_visibility_label: published.visibility_label().as_str().to_string(),
                    m8_visibility_class: published.visibility_label().security_class(),
                    m8_redaction: published.redaction().as_str().to_string(),
                    m8_source_ref: SourceRefView::new(published.source_ref()),
                    m8_publication_id: published.value_id().to_string(),
                    logical_tick_id: tick.clone(),
                    logical_tick_frontier: frontier.clone(),
                };
                let digest = format!("{:?}", binding);
                let mut delivery = self.enqueue_outbox(
                    &delivery_edge,
                    &envelope.request_id,
                    MailboxPayload::DesignatedDelivery {
                        value: published.int_value(),
                        publication: Box::new(published.clone()),
                    },
                    None,
                    Some(envelope.carrier_id.clone()),
                    None,
                    None,
                    Some(semantic_identity.clone()),
                    Some(binding.clone()),
                    Some(digest),
                    vec![evaluation_node.clone()],
                )?;
                self.evaluator_publication_bindings.retain(
                    &envelope.operation_id,
                    published.value_id(),
                    binding,
                );
                delivery.m8_evaluation_node_id = Some(evaluation_node.clone());
                if let Some(runtime) = self.loci.get_mut(locus)
                    && let Some(queued) = runtime
                        .outgoing_mailbox
                        .pending
                        .iter_mut()
                        .find(|queued| queued.envelope_id == delivery.envelope_id)
                {
                    queued.m8_evaluation_node_id = Some(evaluation_node.clone());
                }
                self.trace.append(
                    format!("publish:{}", published.value_id()),
                    Some(published.value_id().to_string()),
                    &envelope.operation_id,
                    Sys4TraceKind::DesignatedResultPublished,
                    Some(CommunicationEdgeKind::DesignatedResultDelivery),
                );
                let receipt = FabricReceipt {
                    request_id: envelope.request_id.clone(),
                    delivery_id: published.value_id().to_string(),
                    operation_id: envelope.operation_id.clone(),
                    origin_locus: locus.to_string(),
                    target_locus: locus.to_string(),
                    typed_value: RuntimeValue::int(published.int_value().ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(
                            Sys4DiagnosticKind::MissingTypedDesignatedValue,
                        )
                    })?),
                    result_version: Some(published.result_version()),
                    owner_rmw: None,
                    performed_m8_consumption: false,
                    returned_from_cache: false,
                    semantic_consumption_identity: None,
                    fault_id: None,
                    m9_cache_validation: None,
                    m8_non_consuming_validation_node_id: None,
                    m8_publication_id: Some(published.value_id().to_string()),
                    logical_tick_id: Some(tick.clone()),
                    logical_tick_frontier: Some(frontier.clone()),
                };
                self.completed_receipts
                    .insert(envelope.request_id.clone(), receipt.clone());
                let mut step = base(LocusM9Validation::None, Some(receipt));
                step.m8_input_receipt_node_id = Some(input_node);
                step.m8_evaluation_node_id = Some(evaluation_node);
                step.reply_envelope_id = Some(delivery.envelope_id);
                Ok(step)
            }
            (
                CommunicationEdgeKind::DesignatedResultDelivery,
                MailboxPayload::DesignatedDelivery { .. },
            ) => {
                let binding = envelope.immutable_delivery_binding().clone();
                let expected_edge = self
                    .program
                    .projection
                    .communication_plan()
                    .edges()
                    .iter()
                    .find(|edge| edge.edge_ref() == envelope.edge_ref)
                    .expect("dequeued generated carrier retains its projected edge");
                if envelope.source_ref != expected_edge.source_ref() {
                    return Err(self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::CarrierProvenanceMismatch,
                        &envelope.request_id,
                    ));
                }
                if binding.redaction_policy
                    != format!("{:?}", envelope.carrier_contract.visibility_policy())
                {
                    return Err(self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::CarrierRedactionMismatch,
                        &envelope.request_id,
                    ));
                }
                if envelope.immutable_delivery_digest() != format!("{:?}", binding) {
                    return Err(self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::CarrierPolicyMismatch,
                        &envelope.request_id,
                    ));
                }
                self.consume_delivery(locus, &envelope, &dequeue_occurrence, false)
                    .map(|mut step| {
                        step.consumed_envelope_id = envelope.envelope_id.clone();
                        step.locus_dequeue_record_id = envelope.mailbox_record_id.clone();
                        step.locus_dequeue_occurrence_id = dequeue_occurrence;
                        step
                    })
            }
            (CommunicationEdgeKind::DesignatedResultDelivery, MailboxPayload::CacheRetry) => {
                self.consume_cache_retry(locus, &envelope, &dequeue_occurrence)
            }
            _ => Err(self.quarantine(
                locus,
                &envelope,
                Sys4DiagnosticKind::M8ExecutionRejected,
                &envelope.request_id,
            )),
        }
    }

    fn consume_delivery(
        &mut self,
        locus: &str,
        envelope: &MailboxEnvelope,
        dequeue_occurrence: &str,
        cache_retry: bool,
    ) -> Sys4Result<LocusStep> {
        let (value, publication) = match &envelope.payload {
            MailboxPayload::DesignatedDelivery {
                value: Some(value),
                publication,
            } => (*value, publication.as_ref()),
            MailboxPayload::DesignatedDelivery { value: None, .. } => {
                return Err(self.quarantine(
                    locus,
                    envelope,
                    Sys4DiagnosticKind::MissingTypedDesignatedValue,
                    &envelope.request_id,
                ));
            }
            _ => {
                return Err(self.quarantine(
                    locus,
                    envelope,
                    Sys4DiagnosticKind::M8ExecutionRejected,
                    &envelope.request_id,
                ));
            }
        };
        let binding = envelope.immutable_delivery_binding().clone();
        let semantic_identity = envelope.semantic_identity().to_string();
        // A fixed result version can legitimately make a second evaluator
        // occurrence idempotent, but it cannot make a new tick/frontier a
        // second interpretation of the already-consumed publication.  Compare
        // against the retained immutable carrier binding rather than any
        // operation-global/latest M8 lookup: this is a C-side frame-integrity
        // check and must run before M9 or M8 consumption.
        let cached_publication_tick_split =
            self.cache.get(&semantic_identity).is_some_and(|cached| {
                cached.delivery_id == envelope.m8_publication_id()
                    && cached.sealed_delivery_binding.m8_publication_id()
                        == binding.m8_publication_id()
                    && cached.result_version == binding.result_version()
                    && (cached.sealed_delivery_binding.logical_tick_id()
                        != binding.logical_tick_id()
                        || cached.sealed_delivery_binding.logical_tick_frontier()
                            != binding.logical_tick_frontier())
            });
        if envelope.m8_publication_id().is_empty()
            || envelope.m8_publication_id() != binding.m8_publication_id()
            || envelope.logical_tick_id() != binding.logical_tick_id()
            || envelope.logical_tick_frontier() != binding.logical_tick_frontier()
            || cached_publication_tick_split
        {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch,
                &envelope.request_id,
            ));
        }
        if binding.m8_source_ref != publication.source_ref() {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::CarrierProvenanceMismatch,
                &envelope.request_id,
            ));
        }
        if binding.m8_visibility_label != publication.visibility_label().as_str()
            || binding.m8_visibility_class != publication.visibility_label().security_class()
        {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::CarrierVisibilityMismatch,
                &envelope.request_id,
            ));
        }
        if binding.m8_redaction != publication.redaction().as_str()
            || binding.redaction_policy
                != format!("{:?}", envelope.carrier_contract.visibility_policy())
        {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::CarrierRedactionMismatch,
                &envelope.request_id,
            ));
        }
        // A designated delivery is a sealed concrete publication, not a
        // lookup of the evaluator's latest value.  Validate every value that
        // crossed this exact endpoint before M9, M8 consumption, cache, or
        // local semantic state can observe it.
        let publication_matches_carrier = publication.value_name() == envelope.operation_id
            && publication.value_id() == envelope.m8_publication_id()
            && publication.evaluator() == envelope.source_locus
            && publication.int_value() == Some(value)
            && publication.logical_tick().id() == envelope.logical_tick_id()
            && publication.logical_tick().input_frontier() == envelope.logical_tick_frontier()
            && binding
                .input_frontier()
                .is_some_and(|frontier| frontier == publication.input_frontier())
            && binding
                .result_frontier()
                .is_some_and(|frontier| frontier == publication.result_frontier())
            && publication.result_version() == binding.result_version()
            && binding
                .policy_stamp()
                .is_some_and(|stamp| stamp == publication.policy_stamp())
            && envelope
                .carrier_contract
                .observation_policy()
                .is_some_and(|policy| policy == publication.observation_policy());
        if !publication_matches_carrier {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch,
                &envelope.request_id,
            ));
        }
        if !self.evaluator_publication_bindings.matches(
            &envelope.operation_id,
            envelope.m8_publication_id(),
            &binding,
        ) {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch,
                &envelope.request_id,
            ));
        }
        let admitted_m8_contract_matches = self
            .backend
            .validates_generated_designated_publication(locus, publication.clone())
            .map_err(|kind| self.quarantine(locus, envelope, kind, &envelope.request_id))?;
        if !admitted_m8_contract_matches {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch,
                &envelope.request_id,
            ));
        }
        let validation = match self.authority_generation.validate_designated_consumer(
            &envelope.operation_id,
            locus,
            &envelope.request_id,
            &semantic_identity,
        ) {
            Ok(value) => value,
            Err(failure) => {
                let kind = match failure.admission_error_kind() {
                    M9AdmissionErrorKind::InvalidMembershipLineage => {
                        Sys4DiagnosticKind::MissingConsumerMembership
                    }
                    _ if self
                        .authority_generation
                        .designated_consumer_witness_is_retired(locus, &envelope.operation_id) =>
                    {
                        Sys4DiagnosticKind::MissingConsumerWitness
                    }
                    _ => Sys4DiagnosticKind::MissingConsumerCapability,
                };
                let mut diagnostic = self.quarantine(locus, envelope, kind, &envelope.request_id);
                diagnostic.context.m9_failure_inspection = Some(Box::new(failure));
                return Err(diagnostic);
            }
        };
        self.causality.record(
            validation.occurrence_id().to_string(),
            vec![dequeue_occurrence.to_string()],
        );
        let authority = self
            .authority_generation
            .designated_consumption_authority_use(locus, &envelope.operation_id)
            .ok_or_else(|| {
                self.quarantine(
                    locus,
                    envelope,
                    Sys4DiagnosticKind::MissingConsumerCapability,
                    &envelope.request_id,
                )
            })?;
        // Consumer authority must be checked before importing the payload:
        // a revoked/missing membership, capability, or witness has no C-side
        // M8 row, cache mutation, or publication materialization.
        let mut imported_node_id = None;
        if !cache_retry {
            let import_context = M8LocalDesignatedTraceContext::new(
                envelope.envelope_id(),
                &semantic_identity,
                locus,
                envelope.m8_publication_id(),
                envelope.logical_tick_id(),
                envelope.logical_tick_frontier(),
            )
            .with_operation_id(&envelope.operation_id)
            .with_evaluator_locus(&envelope.source_locus)
            .with_edge_ref(&envelope.edge_ref);
            let import_observation = match self.backend.import_designated_publication(
                locus,
                publication.clone(),
                import_context,
            ) {
                Ok(observation) => observation,
                Err(kind) => {
                    self.refresh_m8_local_runtime_trace(locus);
                    return Err(self.quarantine(locus, envelope, kind, &envelope.request_id));
                }
            };
            self.refresh_m8_local_runtime_trace(locus);
            if let Some(import_observation) = import_observation {
                let import_observation =
                    self.fabric_qualified_m8_observation_for_locus(locus, &import_observation);
                let import_node = import_observation.node_id().to_string();
                self.causality.record(
                    import_node.clone(),
                    vec![
                        dequeue_occurrence.to_string(),
                        envelope.m8_evaluation_node_id().to_string(),
                    ],
                );
                self.actual_m8_trace.append(
                    import_node.clone(),
                    "DesignatedPublicationImported",
                    Some(envelope.request_id.clone()),
                    Some(semantic_identity.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(import_observation.node_id()),
                );
                imported_node_id = Some(import_node);
            }
        }
        let publication_exists = match self.backend.has_designated_publication_id(
            locus,
            &envelope.operation_id,
            envelope.m8_publication_id(),
        ) {
            Ok(exists) => exists,
            Err(kind) => {
                return Err(self.quarantine(locus, envelope, kind, &envelope.request_id));
            }
        };
        if !publication_exists {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch,
                &envelope.request_id,
            ));
        }
        let (consumed, consumption_observation) = match self.backend.consume_designated(
            locus,
            M8ConsumeRequest::for_value(&envelope.operation_id)
                .with_consumer(locus)
                .with_delivery_id(envelope.m8_publication_id())
                .with_authority(authority),
            M8LocalDesignatedTraceContext::new(
                envelope.envelope_id(),
                &semantic_identity,
                locus,
                envelope.m8_publication_id(),
                envelope.logical_tick_id(),
                envelope.logical_tick_frontier(),
            )
            .with_operation_id(&envelope.operation_id)
            .with_edge_ref(&envelope.edge_ref),
        ) {
            Ok(value) => value,
            Err(failure) => {
                self.refresh_m8_local_runtime_trace(locus);
                let failure = self.fabric_qualified_m8_failure_for_locus(locus, failure);
                let mut diagnostic =
                    self.quarantine(locus, envelope, failure.kind, &envelope.request_id);
                diagnostic.context.endpoint_dequeue_occurrence_id =
                    Some(dequeue_occurrence.to_string());
                if let Some(failure) = failure.observation {
                    self.causality.record(
                        failure.node_id().to_string(),
                        vec![dequeue_occurrence.to_string()],
                    );
                    self.actual_m8_trace.append(
                        failure.node_id().to_string(),
                        "DesignatedConsumptionRejected",
                        Some(envelope.request_id.clone()),
                        Some(semantic_identity.clone()),
                        Some(locus.to_string()),
                        self.causality.predecessor_ids(failure.node_id()),
                    );
                    diagnostic.context.m8_trace_node_id = Some(failure.node_id().to_string());
                    diagnostic.context.backend_m8_failure = Some(failure);
                }
                return Err(diagnostic);
            }
        };
        self.refresh_m8_local_runtime_trace(locus);
        let consumption_observation =
            self.fabric_qualified_m8_observation_for_locus(locus, &consumption_observation);
        let node = consumption_observation.node_id().to_string();
        let mut consumption_predecessors = vec![
            dequeue_occurrence.to_string(),
            validation.occurrence_id().to_string(),
        ];
        if let Some(import_node) = imported_node_id {
            consumption_predecessors.push(import_node);
        }
        self.causality
            .record(node.clone(), consumption_predecessors);
        self.actual_m8_trace.append(
            node.clone(),
            "DesignatedValueConsumed",
            Some(envelope.request_id.clone()),
            Some(semantic_identity.clone()),
            Some(locus.to_string()),
            self.causality.predecessor_ids(&node),
        );
        self.trace.append_actual_m8_consumption(
            &envelope.request_id,
            envelope.m8_publication_id(),
            &envelope.operation_id,
            semantic_identity.clone(),
            locus,
        );
        self.m8_trace
            .record_actual_consumption(&semantic_identity, locus);
        *self
            .consumption_state
            .counts
            .entry((semantic_identity.clone(), locus.to_string()))
            .or_default() += 1;
        self.cache.insert(
            semantic_identity.clone(),
            CachedDelivery {
                value: RuntimeValue::int(value),
                result_version: consumed.result_version(),
                delivery_id: envelope.m8_publication_id().to_string(),
                semantic_identity: semantic_identity.clone(),
                operation: envelope.operation_id.clone(),
                consumer_locus: locus.to_string(),
                visibility_redaction: binding.redaction_policy.clone(),
                policy_stamp: binding.policy_stamp.clone().unwrap_or_default(),
                sealed_delivery_binding_digest: envelope.immutable_delivery_digest().to_string(),
                sealed_delivery_binding: binding,
            },
        );
        let receipt = FabricReceipt {
            request_id: envelope.request_id.clone(),
            delivery_id: envelope.m8_publication_id().to_string(),
            operation_id: envelope.operation_id.clone(),
            origin_locus: envelope.source_locus.clone(),
            target_locus: locus.to_string(),
            typed_value: RuntimeValue::int(value),
            result_version: Some(consumed.result_version()),
            owner_rmw: None,
            performed_m8_consumption: !cache_retry,
            returned_from_cache: cache_retry,
            semantic_consumption_identity: Some(semantic_identity.clone()),
            fault_id: None,
            m9_cache_validation: Some(validation.clone()),
            m8_non_consuming_validation_node_id: None,
            m8_publication_id: Some(envelope.m8_publication_id().to_string()),
            logical_tick_id: Some(envelope.logical_tick_id().to_string()),
            logical_tick_frontier: Some(envelope.logical_tick_frontier().to_string()),
        };
        self.completed_receipts
            .insert(envelope.request_id.clone(), receipt.clone());
        Ok(LocusStep {
            consumed_envelope_id: envelope.envelope_id.clone(),
            locus_dequeue_record_id: envelope.mailbox_record_id.clone(),
            locus_dequeue_occurrence_id: dequeue_occurrence.to_string(),
            m9_validation: LocusM9Validation::Consumer {
                inspection: validation.clone(),
            },
            m8_request_node_id: Some(node),
            m8_serve_node_id: None,
            m8_input_receipt_node_id: None,
            m8_evaluation_node_id: None,
            m8_non_consuming_validation_node_id: None,
            reply_envelope_id: None,
            local_store_read_audit: None,
            local_store_reads: Vec::new(),
            receipt: Some(receipt),
            request_id: envelope.request_id.clone(),
            semantic_identity: Some(semantic_identity),
            m9_cache_validation: Some(validation),
        })
    }

    fn consume_cache_retry(
        &mut self,
        locus: &str,
        envelope: &MailboxEnvelope,
        dequeue_occurrence: &str,
    ) -> Sys4Result<LocusStep> {
        let semantic_identity = envelope.semantic_identity().to_string();
        let cached = self.cache.get(&semantic_identity).cloned().ok_or_else(|| {
            self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::MissingPublishedResult,
                &envelope.request_id,
            )
        })?;
        let binding = envelope.immutable_delivery_binding().clone();
        let expected_edge = match self.edge_for(
            &envelope.operation_id,
            CommunicationEdgeKind::DesignatedResultDelivery,
            &envelope.source_locus,
            locus,
        ) {
            Ok(edge) => edge,
            Err(_) => {
                return Err(self.quarantine(
                    locus,
                    envelope,
                    Sys4DiagnosticKind::CacheProjectionMismatch,
                    &envelope.request_id,
                ));
            }
        };
        let expected_binding = projection_delivery_binding(&expected_edge, &binding);
        if binding != expected_binding
            || envelope.source_ref != expected_edge.source_ref()
            || envelope.core_ref.as_deref() != expected_edge.core_ref()
            || envelope.source_fragment_ref != *expected_edge.source_fragment_ref()
            || envelope.target_fragment_ref != *expected_edge.target_fragment_ref()
        {
            let mut diagnostic = self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::CacheProjectionMismatch,
                &envelope.request_id,
            );
            diagnostic.context.cache_projection_mismatch =
                Some(Box::new(CacheProjectionMismatchInspection {
                    envelope_id: envelope.envelope_id.clone(),
                    expected_edge_ref: expected_edge.edge_ref().to_string(),
                    expected_source_ref: expected_edge.source_ref(),
                    expected_core_ref: expected_edge.core_ref().map(ToOwned::to_owned),
                    carrier_core_ref: binding.core_ref.clone(),
                    rejected_before_m9_validation: true,
                    rejected_before_m8_validation: true,
                }));
            return Err(diagnostic);
        }
        let binding_digest = format!("{binding:?}");
        if envelope.immutable_delivery_digest() != cached.sealed_delivery_binding_digest
            || envelope.immutable_delivery_digest() != binding_digest
            || binding != cached.sealed_delivery_binding
            || envelope.m8_publication_id() != cached.sealed_delivery_binding.m8_publication_id()
            || envelope.logical_tick_id() != cached.sealed_delivery_binding.logical_tick_id()
            || envelope.logical_tick_frontier()
                != cached.sealed_delivery_binding.logical_tick_frontier()
        {
            return Err(self.quarantine(
                locus,
                envelope,
                Sys4DiagnosticKind::CacheBindingDigestMismatch,
                &envelope.request_id,
            ));
        }
        let validation = match self.authority_generation.validate_designated_consumer(
            &envelope.operation_id,
            locus,
            &envelope.request_id,
            &semantic_identity,
        ) {
            Ok(value) => value,
            Err(failure) => {
                let kind = match failure.admission_error_kind() {
                    M9AdmissionErrorKind::InvalidMembershipLineage => {
                        Sys4DiagnosticKind::MissingConsumerMembership
                    }
                    _ if self
                        .authority_generation
                        .designated_consumer_witness_is_retired(locus, &envelope.operation_id) =>
                    {
                        Sys4DiagnosticKind::MissingConsumerWitness
                    }
                    _ => Sys4DiagnosticKind::MissingConsumerCapability,
                };
                let mut diagnostic = self.quarantine(locus, envelope, kind, &envelope.request_id);
                diagnostic.context.m9_failure_inspection = Some(Box::new(failure));
                return Err(diagnostic);
            }
        };
        self.causality.record(
            validation.occurrence_id().to_string(),
            vec![dequeue_occurrence.to_string()],
        );
        let authority = self
            .authority_generation
            .designated_consumption_authority_use(locus, &envelope.operation_id)
            .ok_or_else(|| {
                self.quarantine(
                    locus,
                    envelope,
                    Sys4DiagnosticKind::MissingConsumerCapability,
                    &envelope.request_id,
                )
            })?;
        let raw_node = self
            .backend
            .validate_designated_non_consuming(
                locus,
                M8ConsumeRequest::for_value(&envelope.operation_id)
                    .with_consumer(locus)
                    .with_delivery_id(&cached.delivery_id)
                    .with_authority(authority),
                M8LocalDesignatedTraceContext::new(
                    envelope.envelope_id(),
                    &semantic_identity,
                    locus,
                    cached.sealed_delivery_binding.m8_publication_id(),
                    cached.sealed_delivery_binding.logical_tick_id(),
                    cached.sealed_delivery_binding.logical_tick_frontier(),
                ),
            )
            .map_err(|kind| self.quarantine(locus, envelope, kind, &envelope.request_id))?;
        self.refresh_m8_local_runtime_trace(locus);
        let node = self.fabric_qualified_m8_node_for_locus(locus, &raw_node);
        self.causality.record(
            node.clone(),
            vec![
                dequeue_occurrence.to_string(),
                validation.occurrence_id().to_string(),
            ],
        );
        self.actual_m8_trace.append(
            node.clone(),
            "DesignatedCacheValidated",
            Some(envelope.request_id.clone()),
            Some(semantic_identity.clone()),
            Some(locus.to_string()),
            self.causality.predecessor_ids(&node),
        );
        let receipt = FabricReceipt {
            request_id: envelope.request_id.clone(),
            delivery_id: cached.delivery_id.clone(),
            operation_id: envelope.operation_id.clone(),
            origin_locus: envelope.source_locus.clone(),
            target_locus: locus.to_string(),
            typed_value: cached.value.clone(),
            result_version: Some(cached.result_version),
            owner_rmw: None,
            performed_m8_consumption: false,
            returned_from_cache: true,
            semantic_consumption_identity: Some(semantic_identity.clone()),
            fault_id: None,
            m9_cache_validation: Some(validation.clone()),
            m8_non_consuming_validation_node_id: Some(node.clone()),
            m8_publication_id: Some(
                cached
                    .sealed_delivery_binding
                    .m8_publication_id()
                    .to_string(),
            ),
            logical_tick_id: Some(cached.sealed_delivery_binding.logical_tick_id().to_string()),
            logical_tick_frontier: Some(
                cached
                    .sealed_delivery_binding
                    .logical_tick_frontier()
                    .to_string(),
            ),
        };
        self.completed_receipts
            .insert(envelope.request_id.clone(), receipt.clone());
        Ok(LocusStep {
            consumed_envelope_id: envelope.envelope_id.clone(),
            locus_dequeue_record_id: envelope.mailbox_record_id.clone(),
            locus_dequeue_occurrence_id: dequeue_occurrence.to_string(),
            m9_validation: LocusM9Validation::Consumer {
                inspection: validation.clone(),
            },
            m8_request_node_id: None,
            m8_serve_node_id: None,
            m8_input_receipt_node_id: None,
            m8_evaluation_node_id: None,
            m8_non_consuming_validation_node_id: Some(node),
            reply_envelope_id: None,
            local_store_read_audit: None,
            local_store_reads: Vec::new(),
            receipt: Some(receipt),
            request_id: envelope.request_id.clone(),
            semantic_identity: Some(semantic_identity),
            m9_cache_validation: Some(validation),
        })
    }

    pub(crate) fn dispatch_source_action(
        &mut self,
        action: SourceAction,
    ) -> Sys4Result<FabricReceipt> {
        let submitted = self.submit_source_action(action)?;
        for _ in 0..32 {
            if let Some(receipt) = self.completed_receipts.remove(submitted.request_id()) {
                return Ok(receipt);
            }
            let pending_transport = self.loci.iter().find_map(|(locus, runtime)| {
                runtime.outgoing_mailbox.pending.front().map(|envelope| {
                    (
                        locus.clone(),
                        envelope.target_locus.clone(),
                        envelope.envelope_id.clone(),
                    )
                })
            });
            if let Some((source, target, envelope_id)) = pending_transport {
                self.step_transport(&source, &target, &envelope_id)?;
                continue;
            }
            let pending_locus = self.loci.iter().find_map(|(locus, runtime)| {
                (!runtime.incoming_mailbox.pending.is_empty()).then(|| locus.clone())
            });
            if let Some(locus) = pending_locus {
                self.step_locus(&locus)?;
                continue;
            }
            break;
        }
        self.completed_receipts
            .remove(submitted.request_id())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::M8ExecutionRejected))
    }

    pub(crate) fn dispatch_external_action(
        &mut self,
        action: ExternalAction,
    ) -> Sys4Result<FabricReceipt> {
        match action.kind {
            ExternalActionKind::Source(source) => self.dispatch_source_action(source),
            ExternalActionKind::Fault(fault) => {
                if matches!(
                    fault.kind,
                    FaultInjectionKind::CorruptCacheBindingDigest
                        | FaultInjectionKind::RewriteCacheRetryProjectionBinding
                ) {
                    let envelope_id = fault.envelope_id.as_deref().ok_or_else(|| {
                        Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::UnavailableEnvelope)
                    })?;
                    let live_cache_retry = self
                        .loci
                        .values()
                        .flat_map(|runtime| runtime.incoming_mailbox.pending.iter())
                        .find(|entry| {
                            entry.envelope_id == envelope_id && entry.is_local_cache_retry()
                        })
                        .ok_or_else(|| {
                            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::UnavailableEnvelope)
                        })?;
                    if matches!(
                        fault.kind,
                        FaultInjectionKind::RewriteCacheRetryProjectionBinding
                    ) && live_cache_retry.edge_ref != fault.edge_ref
                    {
                        return Err(Sys4DispatchDiagnostics::one(
                            Sys4DiagnosticKind::FaultEnvelopeRouteMismatch,
                        ));
                    }
                    let edge_ref = self
                        .loci
                        .values_mut()
                        .find_map(|runtime| {
                            runtime
                                .incoming_mailbox
                                .pending
                                .iter_mut()
                                .find(|entry| {
                                    entry.envelope_id == envelope_id && entry.is_local_cache_retry()
                                })
                                .map(|entry| {
                                    match fault.kind {
                                        FaultInjectionKind::CorruptCacheBindingDigest => {
                                            entry.immutable_delivery_digest =
                                                Some("sys4-corrupted-cache-binding".to_string());
                                        }
                                        FaultInjectionKind::RewriteCacheRetryProjectionBinding => {
                                            let binding = entry
                                                .immutable_delivery_binding
                                                .as_mut()
                                                .expect("cache retry carries sealed binding");
                                            binding.core_ref = fault.replacement_core_ref.clone();
                                            binding.policy_stamp =
                                                fault.replacement_policy_stamp.clone();
                                            binding.redaction_policy = fault
                                                .replacement_redaction_policy
                                                .clone()
                                                .unwrap_or_default();
                                            entry.immutable_delivery_digest =
                                                Some(format!("{binding:?}"));
                                        }
                                        _ => {
                                            unreachable!("cache retry branch selects cache faults")
                                        }
                                    }
                                    entry.edge_ref.clone()
                                })
                        })
                        .ok_or_else(|| {
                            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::UnavailableEnvelope)
                        })?;
                    let fault_id = self.next_request_id()?;
                    self.trace.append_fault(&fault_id, &edge_ref);
                    return Ok(FabricReceipt {
                        request_id: fault_id.clone(),
                        delivery_id: fault_id.clone(),
                        operation_id: edge_ref,
                        origin_locus: "external".to_string(),
                        target_locus: String::new(),
                        typed_value: RuntimeValue::unit(),
                        result_version: None,
                        owner_rmw: None,
                        performed_m8_consumption: false,
                        returned_from_cache: false,
                        semantic_consumption_identity: None,
                        fault_id: Some(fault_id),
                        m9_cache_validation: None,
                        m8_non_consuming_validation_node_id: None,
                        m8_publication_id: None,
                        logical_tick_id: None,
                        logical_tick_frontier: None,
                    });
                }
                let known = self
                    .program
                    .projection
                    .communication_plan()
                    .edges()
                    .iter()
                    .any(|edge| edge.edge_ref() == fault.edge_ref);
                if !known {
                    return Err(Sys4DispatchDiagnostics::one(
                        Sys4DiagnosticKind::UnknownProjectedEdge,
                    ));
                }
                if let Some(envelope_id) = &fault.envelope_id {
                    let live = self
                        .loci
                        .values()
                        .flat_map(|runtime| runtime.outgoing_mailbox.pending.iter())
                        .find(|entry| entry.envelope_id == *envelope_id);
                    let Some(live) = live else {
                        return Err(Sys4DispatchDiagnostics::one(
                            Sys4DiagnosticKind::UnavailableEnvelope,
                        ));
                    };
                    if live.edge_ref != fault.edge_ref {
                        return Err(Sys4DispatchDiagnostics::one(
                            Sys4DiagnosticKind::FaultEnvelopeRouteMismatch,
                        ));
                    }
                }
                if matches!(fault.kind, FaultInjectionKind::Retarget) {
                    let attempted_target = fault.target_locus.as_deref().unwrap_or("");
                    if attempted_target.is_empty() || !self.loci.contains_key(attempted_target) {
                        let mut diagnostic =
                            Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::UnknownRetargetLocus);
                        diagnostic.context.retarget_fault =
                            Some(Box::new(RetargetFaultInspection {
                                evidence_id: format!(
                                    "sys4-retarget:{}:{}:{}",
                                    fault.edge_ref,
                                    fault.envelope_id.as_deref().unwrap_or(""),
                                    attempted_target,
                                ),
                                edge_ref: fault.edge_ref.clone(),
                                envelope_id: fault.envelope_id.clone().unwrap_or_default(),
                                attempted_target_locus: attempted_target.to_string(),
                                rejected_at_fault_admission: true,
                                target_enqueue_occurrence_id: None,
                            }));
                        return Err(diagnostic);
                    }
                }
                if matches!(fault.kind, FaultInjectionKind::RouteUnavailable) {
                    self.route_faults.insert(fault.edge_ref.clone());
                } else {
                    self.in_transit_faults.entries.push(InTransitFault {
                        edge_ref: fault.edge_ref.clone(),
                        envelope_id: fault.envelope_id.clone(),
                        kind: fault.kind,
                        target_locus: fault.target_locus.clone(),
                        replacement_m8_publication_id: fault.replacement_m8_publication_id.clone(),
                        replacement_source_ref: fault.replacement_source_ref.clone(),
                        replacement_visibility_label: fault.replacement_visibility_label.clone(),
                    });
                }
                let fault_id = self.next_request_id()?;
                self.trace.append_fault(&fault_id, &fault.edge_ref);
                Ok(FabricReceipt {
                    request_id: fault_id.clone(),
                    delivery_id: fault_id.clone(),
                    operation_id: fault.edge_ref,
                    origin_locus: "external".to_string(),
                    target_locus: String::new(),
                    typed_value: RuntimeValue::unit(),
                    result_version: None,
                    owner_rmw: None,
                    performed_m8_consumption: false,
                    returned_from_cache: false,
                    semantic_consumption_identity: None,
                    fault_id: Some(fault_id),
                    m9_cache_validation: None,
                    m8_non_consuming_validation_node_id: None,
                    m8_publication_id: None,
                    logical_tick_id: None,
                    logical_tick_frontier: None,
                })
            }
        }
    }
}

fn fabric_artifact_for(program: Option<&LocusProgram>) -> FabricArtifact {
    let Some(program) = program else {
        return FabricArtifact {
            designated_consumers: BTreeSet::new(),
            designated_evaluation_expressions: BTreeSet::new(),
        };
    };
    FabricArtifact {
        designated_consumers: program
            .operation_fragments()
            .iter()
            .filter(|fragment| {
                fragment.fragment_kind() == ProjectedOperationFragmentKind::DesignatedResultConsumer
            })
            .map(|fragment| fragment.operation_id().to_string())
            .collect(),
        designated_evaluation_expressions: program
            .operation_fragments()
            .iter()
            .filter(|fragment| {
                fragment.fragment_kind() == ProjectedOperationFragmentKind::DesignatedEvaluation
            })
            .map(|fragment| fragment.operation_id().to_string())
            .collect(),
    }
}

fn m8_key_for_read(read: &TypedStateRead, arguments: &BTreeMap<String, String>) -> M8StateKey {
    M8StateKey::indexed_field(
        read.namespace(),
        resolved_index(read, arguments),
        read.field().unwrap_or_default(),
    )
}

fn resolved_index(read: &TypedStateRead, arguments: &BTreeMap<String, String>) -> String {
    read.index()
        .and_then(|index| {
            arguments
                .get(index)
                .cloned()
                .or_else(|| Some(index.to_string()))
        })
        .unwrap_or_default()
}

#[allow(clippy::too_many_arguments)]
fn semantic_consumption_identity(
    identity: &CheckedProgramIdentity,
    evaluator: &str,
    result: &str,
    consumer: &str,
    input_frontier: &impl std::fmt::Debug,
    result_frontier: &impl std::fmt::Debug,
    result_version: ResultVersion,
    observation_policy: &impl std::fmt::Debug,
    policy_stamp: &impl std::fmt::Debug,
) -> String {
    format!(
        "program:{}|evaluator:{evaluator}|result:{result}|consumer:{consumer}|input:{input_frontier:?}|result_frontier:{result_frontier:?}|version:{}|policy:{observation_policy:?}|stamp:{policy_stamp:?}",
        identity.stable_key(),
        result_version.value(),
    )
}

/// Rebuild only the projection-owned portion of a retry binding.  Dynamic
/// publication/tick facts remain the exact immutable facts originally carried
/// by the M8-produced delivery; SYS-4 never substitutes a later publication.
fn projection_delivery_binding(
    edge: &CommunicationEdge,
    observed: &SealedDeliveryBinding,
) -> SealedDeliveryBinding {
    SealedDeliveryBinding {
        carrier_contract: edge.carrier_contract().clone(),
        source_ref: edge.source_ref(),
        core_ref: edge.core_ref().map(ToOwned::to_owned),
        source_fragment_ref: edge.source_fragment_ref().clone(),
        target_fragment_ref: edge.target_fragment_ref().clone(),
        input_frontier: edge
            .carrier_contract()
            .input_frontier()
            .map(|value| format!("{:?}", value)),
        result_frontier: edge
            .carrier_contract()
            .result_frontier()
            .map(|value| format!("{:?}", value)),
        result_version: edge.carrier_contract().result_version(),
        consumer_locus: edge.target_locus().to_string(),
        policy_stamp: edge
            .carrier_contract()
            .policy_stamp()
            .map(|value| format!("{:?}", value)),
        visibility_policy: edge.carrier_contract().visibility_policy().clone(),
        redaction_policy: format!("{:?}", edge.carrier_contract().visibility_policy()),
        m8_visibility_label: observed.m8_visibility_label.clone(),
        m8_visibility_class: observed.m8_visibility_class,
        m8_redaction: observed.m8_redaction.clone(),
        m8_source_ref: observed.m8_source_ref.clone(),
        m8_publication_id: observed.m8_publication_id.clone(),
        logical_tick_id: observed.logical_tick_id.clone(),
        logical_tick_frontier: observed.logical_tick_frontier.clone(),
    }
}

#[cfg(test)]
mod relation_dispatch_p1_tests {
    use super::*;

    use crate::sys5_local_slice::{
        Sys5LocalAdmissionRequest, Sys5LocalRuntimeProfile, Sys5RelationBootstrapPolicy,
        Sys5SourceInput, build_project,
    };

    const RELATION_FIXTURE_PATH: &str = "tests/inline/sys4_relation_dispatch_p1.mir";
    const RELATION_FIXTURE_SOURCE: &str = r#"
module Mirrorea.Sys4.RelationDispatchP1

locus WorldAuthority
locus ParticipantA
locus ParticipantB
locus ViewerC
principal self
principal target
type Player
type Bird

state avatar[id: Player] at WorldAuthority {
  hp: Int
  atk: Int
}

state participant_input[id: Player] at ParticipantA {
  focus: Int
}

Role[self] at ParticipantA {
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
    }
  }
}

relation bird_follow at ParticipantB {
  subject bird: Bird
  primary participant_a_shoulder epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = participant_input[self].focus + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

    fn boot_relation_fabric() -> (FabricProgram, SealedFabricAdmission, LocalFabric) {
        let project = build_project(Sys5SourceInput::inline(
            RELATION_FIXTURE_PATH,
            RELATION_FIXTURE_SOURCE,
        ))
        .expect("bounded relation fixture is checked before projection");
        let request = Sys5LocalAdmissionRequest::source_declared(
            "self",
            "WorldAuthority",
            "epoch:sys4-relation-p1-world",
            "incarnation:self:WorldAuthority:epoch:sys4-relation-p1-world",
            Sys5LocalRuntimeProfile::St,
        )
        .with_source_declared_membership(
            "self",
            "ParticipantA",
            "epoch:sys4-relation-p1-a",
            "incarnation:self:ParticipantA:epoch:sys4-relation-p1-a",
        )
        .with_source_declared_membership(
            "self",
            "ParticipantB",
            "epoch:sys4-relation-p1-b",
            "incarnation:self:ParticipantB:epoch:sys4-relation-p1-b",
        )
        .with_source_declared_membership(
            "self",
            "ViewerC",
            "epoch:sys4-relation-p1-c",
            "incarnation:self:ViewerC:epoch:sys4-relation-p1-c",
        )
        .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
        .with_auth_discharge("MembershipAuth")
        .with_optional_verification_discharge("finite_refinement");
        let prepared = project
            .prepare_finite_admission(request)
            .expect("sealed M9 admission remains source-bound");
        let (program, admission) = prepared.into_parts_for_sys4();
        let fabric = LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St)
            .expect("ST relation fabric boots from its generated program");
        (program, admission, fabric)
    }

    fn assert_diagnostic<T: std::fmt::Debug>(
        result: Sys4Result<T>,
        expected: Sys4DiagnosticKind,
    ) -> Sys4DispatchDiagnostics {
        let diagnostic = result.expect_err("operation must fail closed");
        assert_eq!(diagnostic.primary().kind(), expected);
        assert_eq!(diagnostic.partial_fabric(), None);
        diagnostic
    }

    #[test]
    fn relation_route_failure_discards_pending_carrier_and_recovers_same_publication_sequence() {
        let (_program, _admission, mut fabric) = boot_relation_fabric();
        let edge = fabric
            .relation_publication_edge("bird_follow")
            .expect("generated relation edge exists");
        fabric.route_faults.insert(edge.edge_ref().to_string());

        let diagnostic = assert_diagnostic(
            fabric.publish_relation_current("bird_follow"),
            Sys4DiagnosticKind::RouteUnavailable,
        );
        assert_eq!(
            diagnostic.relation_publication_failure_disposition(),
            Some(RelationPublicationFailureDisposition::DiscardedUndelivered),
            "the failed uncommitted attempt is explicitly retry-safe"
        );
        assert_eq!(
            fabric.for_test_pending_relation_publication_count("ParticipantB"),
            0,
            "a retry must not coexist with a stale pending carrier"
        );

        fabric.for_test_clear_route_fault(edge.edge_ref());
        let recovered = fabric
            .publish_relation_current("bird_follow")
            .expect("clearing the route fault permits the immutable retry");
        assert_eq!(recovered.shadow().publication_occurrence(), 0);
        assert_eq!(
            fabric.for_test_pending_relation_publication_count("ParticipantB"),
            0,
            "the recovered carrier is consumed rather than duplicated"
        );
    }

    #[test]
    fn relation_publish_authority_and_identifier_preflight_fail_before_semantic_or_endpoint_mutation()
     {
        let (_program, _admission, mut fabric) = boot_relation_fabric();
        fabric.for_test_remove_relation_publish_authority("bird_follow");
        assert_diagnostic(
            fabric.publish_relation_current("bird_follow"),
            Sys4DiagnosticKind::M8ExecutionRejected,
        );
        assert_eq!(fabric.endpoint_carrier_count_for_relation("bird_follow"), 0);
        assert_eq!(
            fabric.for_test_pending_relation_publication_count("ParticipantB"),
            0
        );

        let (_program, _admission, mut fabric) = boot_relation_fabric();
        fabric.for_test_set_relation_identifier_counters(u64::MAX, 0);
        assert_diagnostic(
            fabric.publish_relation_current("bird_follow"),
            Sys4DiagnosticKind::IdentifierExhausted,
        );
        assert_eq!(fabric.endpoint_carrier_count_for_relation("bird_follow"), 0);
        assert_eq!(
            fabric.for_test_pending_relation_publication_count("ParticipantB"),
            0
        );

        fabric.for_test_set_relation_identifier_counters(0, 0);
        let recovered = fabric
            .publish_relation_current("bird_follow")
            .expect("preflight failure neither commits nor advances relation publication");
        assert_eq!(recovered.shadow().publication_occurrence(), 0);
        assert!(recovered.request_id().ends_with("00000000000000000000"));
    }

    #[test]
    fn relation_cut_rederives_digest_and_preserves_qualified_observe_provenance() {
        let (program, admission, mut fabric) = boot_relation_fabric();
        fabric
            .publish_relation_current("bird_follow")
            .expect("relation publication crosses the generated endpoint");
        fabric
            .invalidate_relation_primary("bird_follow")
            .expect("invalidation publishes the fallback before fresh reacquire");
        let receipt = fabric
            .fresh_reacquire_relation_primary("bird_follow")
            .expect("fresh sealed binding republishes the primary relation");
        let live_shadow = fabric
            .relation_imported_shadow("bird_follow", "ViewerC")
            .expect("consumer M8 session is available")
            .expect("publication installs its consumer shadow");
        assert_eq!(
            live_shadow.consumer_observe_occurrence_id(),
            Some(receipt.consumer_observe_occurrence_id()),
            "the stored shadow, not merely the returned receipt clone, keeps the qualified observe"
        );
        assert!(
            live_shadow
                .consumer_observe_occurrence_id()
                .expect("imported shadow has an observe occurrence")
                .starts_with("sys4-m8:ViewerC:"),
            "stored observe occurrence must use the fabric namespace: {live_shadow:?}"
        );

        let cut = fabric
            .save_local_cut("relation-p1-provenance")
            .expect("relation state is cut-compatible");
        let restored = LocalFabric::restore_local_cut(
            program.clone(),
            admission.clone(),
            BackendProfile::St,
            &cut,
        )
        .expect("untampered relation cut restores");
        let restored_shadow = restored
            .relation_imported_shadow("bird_follow", "ViewerC")
            .expect("restored consumer session is available")
            .expect("restored cut retains consumer shadow");
        assert_eq!(
            restored_shadow.consumer_observe_occurrence_id(),
            Some(receipt.consumer_observe_occurrence_id())
        );
        assert_eq!(
            restored.relation_semantic_digest("bird_follow"),
            Some(live_shadow.semantic_digest().as_str())
        );

        let mut tampered = cut;
        tampered.for_test_set_relation_semantic_digest("bird_follow", "tampered-nonempty-digest");
        assert_diagnostic(
            LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &tampered),
            Sys4DiagnosticKind::ProgramProjectionMismatch,
        );
    }
}
