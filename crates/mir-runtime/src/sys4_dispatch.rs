//! SYS-4 in-process dispatch of already projected locus artifacts.
//!
//! This module starts at the checked SYS-3 projection and a sealed M9 runtime
//! admission.  It deliberately has no source parsing, conformance facade, or
//! precomputed-result selection path: routes, endpoint ownership, and Core
//! provenance are taken only from the projection result.

// SYS-4 remains crate-internal until its CLI facade is introduced.  Its
// entrypoints are consumed by the crate's SYS-4 conformance tests today.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use mir_semantics::{
    evaluation_materialization::{InputFrontier, ObservationPolicy, PolicyStamp},
    shared_model::{ResultFrontier, ResultVersion, SourceRef},
    surface_v0_pipeline::{CheckedProgramIdentity, TypedStateRead},
};

use crate::{
    m8_runtime_admission::{EvidenceSecurityLabel, M8RuntimeInstance, M8SecurityClass},
    m8_runtime_designated_value::{
        M8ConsumeRequest, M8DesignatedEvaluationRequest, M8DesignatedTick, M8InputReceipt,
        M8InputReceiptSet,
    },
    m8_runtime_local_cut::{
        M8LocalDesignatedTraceContext, M8LocalRuntime, M8LocalRuntimeSeed, M8LocalTrace,
        M8LocalTraceObservation,
    },
    m8_runtime_owner_queue::{M8OwnerRequest, M8ServeOutcome, M8StateKey},
    m9_auth_verification::{
        M9AdmissionErrorKind, M9AuthorityGeneration, M9AuthorityInspection,
        M9AuthoritySuccessorPublisher, M9AuthorityTransitionKind, M9CacheValidationInspection,
        M9DesignatedSourceReleaseLineage, M9KernelAuthorityView, M9RuntimeExecutionSeam,
        M9SealedFailureInspection, M9SealedTransitionInspection,
        M9SourceReleaseValidationInspection,
    },
    sys2_execution_backend::{Ow1ContextualM8Execution, Ow1WorkerBackend, Ow1WorkerFailure},
    sys3_projection::{
        BackendEligibility, BackendProfile, CarrierContract, CommunicationEdge,
        CommunicationEdgeKind, GlobalProjectionResult, LocusProgram, ProjectedOperationFragment,
        ProjectedOperationFragmentKind, ReferenceOnlyRedactionPolicy, RuntimeAdmissionStatus,
        SourceRefView,
    },
};

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
    CarrierPolicyMismatch,
    MissingSourceReleaseAuthority,
    UnknownProjectedEdge,
    UnavailableEnvelope,
    FaultEnvelopeRouteMismatch,
    UnknownRetargetLocus,
    BackendIneligible,
    M8ExecutionRejected,
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
    m9_failure_inspection: Option<Box<M9SealedFailureInspection>>,
    m8_non_consuming_validation_node_id: Option<String>,
    local_store_read_audit_id: Option<String>,
    backend_m8_failure: Option<Box<M8LocalTraceObservation>>,
    retarget_fault: Option<Box<RetargetFaultInspection>>,
    cache_projection_mismatch: Option<Box<CacheProjectionMismatchInspection>>,
}

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
    owner_lineages: BTreeSet<(String, String, String, String)>,
    designated_evaluators: BTreeSet<(String, String)>,
    designated_consumers: BTreeSet<(String, String)>,
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
pub(crate) struct SealedFabricAdmission {
    program_identity: CheckedProgramIdentity,
    program_fingerprint: BTreeSet<(FabricRouteKey, String)>,
    summary: ObserverSafeM9Summary,
    instance: M8RuntimeInstance,
    authority_generation: M9AuthorityGeneration,
    authority_successor: M9AuthoritySuccessorPublisher,
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
                    generation
                        .kernel_designated_remote_input_lineage(
                            dependency.source_owner_locus(),
                            core.evaluator(),
                            core.result(),
                            0,
                            core.trigger().frontier().unwrap_or_default(),
                        )
                        .is_some()
                })(
                ),
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
        let mut designated_evaluators = BTreeSet::new();
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
                _ => {}
            }
        }
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
                owner_lineages,
                designated_evaluators,
                designated_consumers,
            },
            instance,
            authority_generation,
            authority_successor,
            initial_state_seed,
        })
    }

    pub(crate) fn observer_safe_m9_summary(&self) -> &ObserverSafeM9Summary {
        &self.summary
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
    kind: FaultInjectionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FaultInjectionKind {
    RouteUnavailable,
    Retarget,
    CorruptVisibilityRedaction,
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
            kind: FaultInjectionKind::CorruptM8PublicationId,
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
            | MailboxPayload::DesignatedDelivery { value: Some(value) } => RuntimeValue::int(value),
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
        self.predecessors.insert(occurrence_id.into(), predecessors);
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
                node.kind == "OwnerRequest"
                    && node.semantic_identity.as_deref() == Some(operation)
                    && node.consumer_locus.as_deref() == Some(owner_locus)
            })
            .count()
    }

    pub(crate) fn designated_evaluation_count(&self, value_name: &str) -> usize {
        self.nodes
            .iter()
            .filter(|node| {
                node.kind == "DesignatedValueEvaluated"
                    && node.semantic_identity.as_deref() == Some(value_name)
            })
            .count()
    }

    pub(crate) fn non_consuming_designated_cache_validation(
        &self,
        node_id: &str,
    ) -> Option<&ActualM8TraceNode> {
        self.nodes
            .iter()
            .find(|node| node.node_id == node_id && node.kind == "DesignatedCacheValidated")
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
                node.kind == "DesignatedValueConsumed"
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
                node.kind == "DesignatedValueConsumed"
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
        self.nodes.push(ActualM8TraceNode {
            node_id: node_id.into(),
            kind: kind.into(),
            request_id,
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

enum M8ExecutionBackend {
    St(Box<M8LocalRuntime>),
    Ow1(Ow1WorkerBackend),
}

struct M8OwnerExecution {
    outcome: M8ServeOutcome,
    request_node_id: String,
    serve_node_id: String,
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
    fn enqueue_and_serve(
        &mut self,
        owner_locus: &str,
        request: M8OwnerRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<M8OwnerExecution, M8BackendFailure> {
        match self {
            Self::St(runtime) => runtime
                .execute_owner_with_context(owner_locus, request, context)
                .map(
                    |(outcome, request_observation, serve_observation)| M8OwnerExecution {
                        outcome,
                        request_node_id: request_observation.node_id().to_string(),
                        serve_node_id: serve_observation.node_id().to_string(),
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
                        request_node_id: receipt.request_observation().node_id().to_string(),
                        serve_node_id: receipt.serve_observation().node_id().to_string(),
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
        request: M8DesignatedEvaluationRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<M8DesignatedEvaluation, M8BackendFailure> {
        match self {
            Self::St(runtime) => runtime
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
            Self::St(runtime) => runtime
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
        key: M8StateKey,
        source_ref: SourceRef,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<Option<(i64, M8LocalTraceObservation)>, Sys4DiagnosticKind> {
        match self {
            Self::St(runtime) => Ok(runtime.read_owner_int_with_context(key, source_ref, context)),
            Self::Ow1(worker) => worker
                .read_owner_int_with_context(key, source_ref, context)
                .map_err(map_worker_failure),
        }
    }

    fn has_designated_publication_id(
        &self,
        value_name: &str,
        value_id: &str,
    ) -> Result<bool, Sys4DiagnosticKind> {
        match self {
            Self::St(runtime) => Ok(runtime.has_designated_publication_id(value_name, value_id)),
            Self::Ow1(worker) => worker
                .has_designated_publication_id(value_name, value_id)
                .map_err(map_worker_failure),
        }
    }

    fn validate_designated_non_consuming(
        &mut self,
        request: M8ConsumeRequest,
        context: M8LocalDesignatedTraceContext,
    ) -> Result<String, Sys4DiagnosticKind> {
        match self {
            Self::St(runtime) => runtime
                .validate_published_value_non_consuming(request, context)
                .map_err(|_| Sys4DiagnosticKind::M8ExecutionRejected),
            Self::Ow1(worker) => worker
                .validate_designated_non_consuming(request, context)
                .map_err(map_worker_failure),
        }
    }

    fn local_trace_snapshot(&self) -> Option<M8LocalTrace> {
        match self {
            Self::St(runtime) => Some(runtime.trace()),
            Self::Ow1(worker) => worker.local_trace_snapshot().ok(),
        }
    }

    fn designated_publication_snapshot(&self, value_name: &str) -> Option<String> {
        match self {
            Self::St(runtime) => runtime
                .designated_result_store()
                .published_values(value_name)
                .first()
                .map(|value| format!("{value:?}")),
            Self::Ow1(worker) => worker
                .designated_publication_snapshot(value_name)
                .ok()
                .flatten(),
        }
    }

    fn replace_designated_input_receipts(
        &mut self,
        receipts: M8InputReceiptSet,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(runtime) => {
                runtime.replace_designated_input_receipts(receipts);
                Ok(())
            }
            Self::Ow1(worker) => worker
                .replace_designated_input_receipts(receipts)
                .map_err(map_worker_failure),
        }
    }

    fn refresh_authority(
        &mut self,
        generation: &M9AuthorityGeneration,
    ) -> Result<(), Sys4DiagnosticKind> {
        match self {
            Self::St(runtime) => {
                runtime.refresh_m9_authority_state(generation.authority_state());
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
            Self::St(runtime) => {
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
            Self::St(runtime) => {
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
            Self::St(runtime) => {
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

fn map_worker_failure(failure: Ow1WorkerFailure) -> Sys4DiagnosticKind {
    match failure {
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

pub(crate) struct M9AuthorityLifecycle {
    publisher: M9AuthoritySuccessorPublisher,
}

impl M9AuthorityLifecycle {
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
        })
    }
}

pub(crate) struct M9AuthorityTransition {
    generation: M9AuthorityGeneration,
    sealed_m9_inspection: M9SealedTransitionInspection,
}

impl M9AuthorityTransition {
    pub(crate) fn sealed_m9_inspection(&self) -> &M9SealedTransitionInspection {
        &self.sealed_m9_inspection
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
    trace: FabricTrace,
    m8_trace: FabricM8Trace,
    actual_m8_trace: ActualM8Trace,
    m8_local_runtime_trace: M8LocalTrace,
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
    next_request: u64,
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

impl LocalFabric {
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
        if !matches!(
            program.backend_eligibility(backend_profile),
            BackendEligibility::Eligible
        ) {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::BackendIneligible,
            ));
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
        let mut m8_seed = M8LocalRuntimeSeed::new()
            .with_authority_state(admission.authority_generation.authority_state());
        for ((_, state, index, field), value) in &admission.initial_state_seed.ints {
            m8_seed =
                m8_seed.with_owner_int(M8StateKey::indexed_field(state, index, field), *value);
        }
        // A designated evaluator must receive its source-owner inputs through
        // generated endpoints at dispatch time.  In particular, boot must not
        // manufacture an input receipt from the state seed.
        m8_seed = m8_seed.with_designated_input_receipts(M8InputReceiptSet::new());
        let runtime = M8LocalRuntime::from_admitted(admission.instance, m8_seed);
        let backend = match backend_profile {
            BackendProfile::St => M8ExecutionBackend::St(Box::new(runtime)),
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
                M8ExecutionBackend::Ow1(Ow1WorkerBackend::spawn(owner, runtime))
            }
        };
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
            trace: FabricTrace::default(),
            m8_trace: FabricM8Trace::default(),
            actual_m8_trace: ActualM8Trace::default(),
            m8_local_runtime_trace: M8LocalTrace::default(),
            causality: CausalityGraph::default(),
            next_endpoint_occurrence: 0,
            route_faults: BTreeSet::new(),
            in_transit_faults: InTransitFaults::default(),
            completed_receipts: BTreeMap::new(),
            local_store_read_audits,
            consumption_state: DesignatedConsumptionState::default(),
            evaluator_publication_bindings: EvaluatorPublicationBindingRegistry::default(),
            cache: BTreeMap::new(),
            next_request: 0,
        })
    }

    pub(crate) fn locus_names(&self) -> Vec<String> {
        self.loci.keys().cloned().collect()
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
    pub(crate) fn trace(&self) -> &FabricTrace {
        &self.trace
    }
    pub(crate) fn m8_local_trace(&self) -> &FabricM8Trace {
        &self.m8_trace
    }

    pub(crate) fn m8_actual_trace(&self) -> &ActualM8Trace {
        &self.actual_m8_trace
    }

    #[cfg(test)]
    pub(crate) fn m8_backend_test_support_mut(&mut self) -> M8BackendTestSupport<'_> {
        M8BackendTestSupport {
            backend: &mut self.backend,
        }
    }

    pub(crate) fn m8_local_runtime_trace(&self) -> &M8LocalTrace {
        &self.m8_local_runtime_trace
    }

    fn refresh_m8_local_runtime_trace(&mut self) {
        if let Some(trace) = self.backend.local_trace_snapshot() {
            self.m8_local_runtime_trace = trace;
        }
    }

    pub(crate) fn causality(&self) -> &CausalityGraph {
        &self.causality
    }

    pub(crate) fn m8_owner_queue_depth(&self, owner_locus: &str) -> usize {
        match &self.backend {
            M8ExecutionBackend::St(runtime) => runtime.pending_owner_fifo(owner_locus).len(),
            M8ExecutionBackend::Ow1(_) => 0,
        }
    }

    pub(crate) fn designated_cache_entry(&self, identity: &str) -> Option<&CachedDelivery> {
        self.cache.get(identity)
    }

    pub(crate) fn designated_cache_snapshot(&self) -> BTreeMap<String, CachedDelivery> {
        self.cache.clone()
    }

    pub(crate) fn m8_designated_publication_snapshot(&self, value_name: &str) -> Option<String> {
        self.backend.designated_publication_snapshot(value_name)
    }
    pub(crate) fn designated_consumption_state(&self) -> &DesignatedConsumptionState {
        &self.consumption_state
    }
    pub(crate) fn projected_artifact_identity(&self) -> &CheckedProgramIdentity {
        self.program.checked_program_identity()
    }
    pub(crate) fn m9_authority_lifecycle_mut(&mut self) -> &mut M9AuthorityLifecycle {
        &mut self.authority_lifecycle
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
        {
            return Err(Sys4DispatchDiagnostics::one(
                Sys4DiagnosticKind::ProgramAdmissionMismatch,
            ));
        }
        self.backend
            .refresh_authority(&transition.generation)
            .map_err(Sys4DispatchDiagnostics::one)?;
        self.authority_generation = transition.generation;
        Ok(())
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

    fn next_request_id(&mut self) -> String {
        let next = self.next_request;
        self.next_request += 1;
        format!("sys4-request-{next:020}")
    }
}

impl LocalFabric {
    pub(crate) fn current_m9_authority_inspection(&self) -> M9AuthorityInspection {
        self.authority_generation.sealed_inspection()
    }

    pub(crate) fn in_transit_faults(&self) -> &InTransitFaults {
        &self.in_transit_faults
    }

    pub(crate) fn m8_authority_state_digest(&self, locus: &str) -> String {
        format!("{}:{}", self.authority_generation.generation_ref(), locus)
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

    fn next_mailbox_token(&mut self, label: &str) -> String {
        let next = self.next_endpoint_occurrence;
        self.next_endpoint_occurrence = self.next_endpoint_occurrence.saturating_add(1);
        format!("sys4-{label}-{next:020}")
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
        let envelope_id = self.next_mailbox_token("envelope");
        let carrier_id = self.next_mailbox_token("carrier");
        let mailbox_record_id = self.next_mailbox_token("outbox-record");
        let occurrence = self.next_mailbox_token("outbox-enqueue");
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
        let envelope_id = self.next_mailbox_token("envelope");
        let carrier_id = self.next_mailbox_token("carrier");
        let mailbox_record_id = self.next_mailbox_token("inbox-record");
        let occurrence = self.next_mailbox_token("inbox-enqueue");
        self.causality.record(occurrence.clone(), Vec::new());
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
        self.loci
            .get_mut(edge.target_locus())
            .ok_or_else(|| Sys4DispatchDiagnostics::one(Sys4DiagnosticKind::WrongTargetLocus))?
            .incoming_mailbox
            .pending
            .push_back(envelope.clone());
        Ok(envelope)
    }

    pub(crate) fn submit_source_action(
        &mut self,
        action: SourceAction,
    ) -> Sys4Result<FabricSubmission> {
        let request_id = self.next_request_id();
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
                    MailboxPayload::DesignatedDelivery { value } => *value = None,
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
        let dequeue_occurrence = self.next_mailbox_token("outbox-dequeue");
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
        let target_record_id = self.next_mailbox_token("inbox-record");
        let enqueue_occurrence = self.next_mailbox_token("inbox-enqueue");
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
        let occurrence = self.next_mailbox_token("locus-dequeue");
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
                        self.refresh_m8_local_runtime_trace();
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
                self.refresh_m8_local_runtime_trace();
                self.causality.record(
                    execution.request_node_id.clone(),
                    vec![dequeue_occurrence.clone()],
                );
                self.causality.record(
                    execution.serve_node_id.clone(),
                    vec![execution.request_node_id.clone()],
                );
                self.actual_m8_trace.append(
                    execution.request_node_id.clone(),
                    "OwnerRequest",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(&execution.request_node_id),
                );
                self.actual_m8_trace.append(
                    execution.serve_node_id.clone(),
                    "OwnerServe",
                    Some(envelope.request_id.clone()),
                    Some(envelope.operation_id.clone()),
                    Some(locus.to_string()),
                    self.causality.predecessor_ids(&execution.serve_node_id),
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
                    diagnostic.context.m8_trace_node_id = Some(execution.serve_node_id.clone());
                    diagnostic.context.backend_m8_failure =
                        Some(Box::new(execution.serve_observation.clone()));
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
                    vec![execution.serve_node_id.clone()],
                )?;
                let mut step = base(
                    LocusM9Validation::Owner {
                        owner_lineage_ref: lineage,
                    },
                    None,
                );
                step.m8_request_node_id = Some(execution.request_node_id);
                step.m8_serve_node_id = Some(execution.serve_node_id);
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
                        self.refresh_m8_local_runtime_trace();
                        return Err(self.quarantine(locus, &envelope, kind, &envelope.request_id));
                    }
                };
                self.refresh_m8_local_runtime_trace();
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
                if let Err(kind) = self.backend.replace_designated_input_receipts(receipts) {
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
                        self.refresh_m8_local_runtime_trace();
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
                let published = execution.published;
                let input_node = execution.input_observation.node_id().to_string();
                self.causality
                    .record(input_node.clone(), vec![dequeue_occurrence.clone()]);
                let evaluation_node = execution.evaluation_observation.node_id().to_string();
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
                        .map(|value| format!("{value:?}")),
                    result_frontier: delivery_edge
                        .carrier_contract()
                        .result_frontier()
                        .map(|value| format!("{value:?}")),
                    result_version: delivery_edge.carrier_contract().result_version(),
                    consumer_locus: consumer_core.consumer_locus().to_string(),
                    policy_stamp: delivery_edge
                        .carrier_contract()
                        .policy_stamp()
                        .map(|value| format!("{value:?}")),
                    visibility_policy: delivery_edge.carrier_contract().visibility_policy().clone(),
                    redaction_policy: format!(
                        "{:?}",
                        delivery_edge.carrier_contract().visibility_policy()
                    ),
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
                self.refresh_m8_local_runtime_trace();
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
                MailboxPayload::DesignatedDelivery { value },
            ) => {
                let binding = envelope.immutable_delivery_binding().clone();
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
                if !self.evaluator_publication_bindings.matches(
                    &envelope.operation_id,
                    envelope.m8_publication_id(),
                    &binding,
                ) {
                    return Err(self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::DeliveryPublicationIdentityMismatch,
                        &envelope.request_id,
                    ));
                }
                let value = value.ok_or_else(|| {
                    self.quarantine(
                        locus,
                        &envelope,
                        Sys4DiagnosticKind::MissingTypedDesignatedValue,
                        &envelope.request_id,
                    )
                })?;
                self.consume_delivery(locus, &envelope, &dequeue_occurrence, value, binding, false)
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
        value: i64,
        binding: SealedDeliveryBinding,
        cache_retry: bool,
    ) -> Sys4Result<LocusStep> {
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
        let publication_exists = match self
            .backend
            .has_designated_publication_id(&envelope.operation_id, envelope.m8_publication_id())
        {
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
        let (consumed, consumption_observation) = match self.backend.consume_designated(
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
                self.refresh_m8_local_runtime_trace();
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
        self.refresh_m8_local_runtime_trace();
        let node = consumption_observation.node_id().to_string();
        self.causality.record(
            node.clone(),
            vec![
                dequeue_occurrence.to_string(),
                validation.occurrence_id().to_string(),
            ],
        );
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
        let node = self
            .backend
            .validate_designated_non_consuming(
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
        self.refresh_m8_local_runtime_trace();
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
                    let fault_id = self.next_request_id();
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
                    });
                }
                let fault_id = self.next_request_id();
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
            .map(|value| format!("{value:?}")),
        result_frontier: edge
            .carrier_contract()
            .result_frontier()
            .map(|value| format!("{value:?}")),
        result_version: edge.carrier_contract().result_version(),
        consumer_locus: edge.target_locus().to_string(),
        policy_stamp: edge
            .carrier_contract()
            .policy_stamp()
            .map(|value| format!("{value:?}")),
        visibility_policy: edge.carrier_contract().visibility_policy().clone(),
        redaction_policy: format!("{:?}", edge.carrier_contract().visibility_policy()),
        m8_publication_id: observed.m8_publication_id.clone(),
        logical_tick_id: observed.logical_tick_id.clone(),
        logical_tick_frontier: observed.logical_tick_frontier.clone(),
    }
}
