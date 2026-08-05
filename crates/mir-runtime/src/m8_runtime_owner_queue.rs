//! Deterministic M8 owner-queue execution over an admitted checked artifact.
//!
//! This is a bounded, source-free execution facade.  It owns one semantic
//! snapshot and evaluates an owner RMW only when that owner's FIFO request is
//! served.  It is not a transport, credential provider, or observer export.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedBinaryOperator, CheckedExpressionTree, SurfaceV0PipelineDiagnostics,
    },
};

use crate::{
    m8_runtime_admission::{M8OwnerExecutionPlan, M8RuntimeInstance},
    m8_runtime_authority::{M8AuthorityState, M8AuthorityValidationFailure},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct M8StateKey {
    namespace: String,
    index: String,
    field: String,
}

impl M8StateKey {
    pub fn indexed_field(
        namespace: impl Into<String>,
        index: impl Into<String>,
        field: impl Into<String>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            index: index.into(),
            field: field.into(),
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn index(&self) -> &str {
        &self.index
    }

    pub fn field(&self) -> &str {
        &self.field
    }
}

/// Typed use of a previously issued authority.  It names neither an
/// authentication provider nor a credential: membership, capability, and
/// witness remain distinct values checked at owner service time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8AuthorityUse {
    principal: String,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    witness_ref: Option<String>,
}

impl M8AuthorityUse {
    pub fn for_principal(principal: impl Into<String>) -> Self {
        Self {
            principal: principal.into(),
            membership_ref: None,
            capability_ref: None,
            witness_ref: None,
        }
    }

    pub fn with_membership_ref(mut self, membership_ref: impl Into<String>) -> Self {
        self.membership_ref = Some(membership_ref.into());
        self
    }

    pub fn with_capability_ref(mut self, capability_ref: impl Into<String>) -> Self {
        self.capability_ref = Some(capability_ref.into());
        self
    }

    pub fn with_witness_ref(mut self, witness_ref: impl Into<String>) -> Self {
        self.witness_ref = Some(witness_ref.into());
        self
    }

    pub fn principal(&self) -> &str {
        &self.principal
    }

    pub fn membership_ref(&self) -> Option<&str> {
        self.membership_ref.as_deref()
    }

    pub fn capability_ref(&self) -> Option<&str> {
        self.capability_ref.as_deref()
    }

    pub fn witness_ref(&self) -> Option<&str> {
        self.witness_ref.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8OwnerRequest {
    evaluation: String,
    arguments: BTreeMap<String, String>,
    authority_use: Option<M8AuthorityUse>,
}

impl M8OwnerRequest {
    pub fn new(evaluation: impl Into<String>) -> Self {
        Self {
            evaluation: evaluation.into(),
            arguments: BTreeMap::new(),
            authority_use: None,
        }
    }

    pub fn with_argument(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.arguments.insert(name.into(), value.into());
        self
    }

    pub fn with_authority_use(mut self, authority_use: M8AuthorityUse) -> Self {
        self.authority_use = Some(authority_use);
        self
    }

    pub fn evaluation(&self) -> &str {
        &self.evaluation
    }

    pub fn arguments(&self) -> &BTreeMap<String, String> {
        &self.arguments
    }

    pub fn authority_use(&self) -> Option<&M8AuthorityUse> {
        self.authority_use.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ExecutionSeed {
    ints: BTreeMap<M8StateKey, i64>,
    membership_epoch: u64,
    authority_state: M8AuthorityState,
}

impl M8ExecutionSeed {
    pub fn new() -> Self {
        Self {
            ints: BTreeMap::new(),
            membership_epoch: 1,
            authority_state: M8AuthorityState::new(),
        }
    }

    pub fn with_int(mut self, key: M8StateKey, value: i64) -> Self {
        self.ints.insert(key, value);
        self
    }

    pub fn with_membership_epoch(mut self, membership_epoch: u64) -> Self {
        self.membership_epoch = membership_epoch;
        self
    }

    pub fn with_authority_state(mut self, authority_state: M8AuthorityState) -> Self {
        self.authority_state = authority_state;
        self
    }
}

impl Default for M8ExecutionSeed {
    fn default() -> Self {
        Self::new()
    }
}

/// The sole mutable semantic state for this bounded execution facade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8SemanticSnapshot {
    ints: BTreeMap<M8StateKey, i64>,
    membership_epoch: u64,
    authority_state: M8AuthorityState,
    pub(crate) relations: BTreeMap<String, M8SemanticRelation>,
    published_values: BTreeMap<String, Vec<String>>,
}

impl M8SemanticSnapshot {
    fn from_seed(seed: M8ExecutionSeed) -> Self {
        Self {
            ints: seed.ints,
            membership_epoch: seed.membership_epoch,
            authority_state: seed.authority_state,
            relations: BTreeMap::new(),
            published_values: BTreeMap::new(),
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            ints: BTreeMap::new(),
            membership_epoch: 1,
            authority_state: M8AuthorityState::new(),
            relations: BTreeMap::new(),
            published_values: BTreeMap::new(),
        }
    }

    pub fn int(&self, key: &M8StateKey) -> Option<i64> {
        self.ints.get(key).copied()
    }

    pub fn contains_presentation_contexts(&self) -> bool {
        false
    }

    pub fn published_values_for(&self, subject: &str) -> &[String] {
        self.published_values
            .get(subject)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(crate) fn insert_relation(&mut self, relation: M8SemanticRelation) {
        self.relations.insert(relation.name.clone(), relation);
    }

    pub(crate) fn relation(&self, name: &str) -> Option<&M8SemanticRelation> {
        self.relations.get(name)
    }

    pub(crate) fn relation_mut(&mut self, name: &str) -> Option<&mut M8SemanticRelation> {
        self.relations.get_mut(name)
    }

    pub(crate) fn authority_state(&self) -> &M8AuthorityState {
        &self.authority_state
    }

    /// Crate-internal M9 bridge refresh.  The surrounding M8 local session
    /// retains its store, relation, and configuration state; only the
    /// authority inventory is replaced from the sealed upstream snapshot.
    pub(crate) fn replace_authority_state(&mut self, authority_state: M8AuthorityState) {
        self.authority_state = authority_state;
    }

    /// Patch activation may introduce a checked Int field with the finite-v0
    /// default.  This crate-private operation is intentionally narrower than
    /// an owner write: it cannot set an arbitrary value or replace an existing
    /// field, and M8PatchRuntime records the resulting occurrence separately.
    pub(crate) fn initialize_int_default(&mut self, key: M8StateKey) -> bool {
        if self.ints.contains_key(&key) {
            return false;
        }
        self.ints.insert(key, 0);
        true
    }

    /// Exact mutable store domain for crate-internal runtime receipts.  This
    /// intentionally excludes maintained relations and admission/config
    /// state, so a relation transition cannot be represented as a store
    /// change merely by hashing a combined snapshot.
    pub(crate) fn canonical_store_projection(&self) -> String {
        let ints = self.ints.iter().map(|(key, value)| {
            format!(
                "int|{}|{}|{}|{}",
                key.namespace(),
                key.index(),
                key.field(),
                value
            )
        });
        let published_values = self
            .published_values
            .iter()
            .map(|(subject, values)| format!("published|{subject}|{}", values.join(",")));
        ints.chain(published_values).collect::<Vec<_>>().join("\n")
    }

    /// Exact maintained-relation domain for crate-internal runtime receipts.
    pub(crate) fn canonical_relation_projection(&self) -> String {
        self.relations
            .values()
            .map(|relation| {
                format!(
                    "relation|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                    relation.name,
                    relation.owner_locus,
                    relation.selected_option_index,
                    relation.selected_floor.as_str(),
                    relation.selected_anchor,
                    relation.selected_option_epoch,
                    relation.primary_epoch,
                    relation.binding_epoch,
                    relation.binding_frontier,
                    relation.active_lease_ref,
                    relation.activation_frontier,
                    relation.lineage.join(","),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The semantic snapshot has no independent configuration domain: its
    /// membership/authority caches are M9-derived and are intentionally not
    /// reclassified as M8 configuration receipt input.
    pub(crate) fn canonical_configuration_projection(&self) -> String {
        String::new()
    }

    /// Ordered semantic projection used only by crate-internal diagnostics
    /// that genuinely need every M8 domain.  Receipt hashing uses the domain
    /// accessors above instead of this aggregate projection.
    pub(crate) fn canonical_projection(&self) -> String {
        [
            self.canonical_configuration_projection(),
            self.canonical_store_projection(),
            self.canonical_relation_projection(),
        ]
        .join("\n")
    }

    pub(crate) fn empty_with_authority_state(authority_state: M8AuthorityState) -> Self {
        Self {
            authority_state,
            ..Self::empty()
        }
    }

    pub(crate) fn absorb_relations_from(&mut self, other: &mut Self) {
        self.relations.append(&mut other.relations);
    }
}

/// M8's semantic relation state lives in the same snapshot as owner state.
/// Presentation contexts, samples, and derived poses are intentionally absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8RelationFloor {
    Live,
    Anchor,
    Frozen,
}

impl M8RelationFloor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Anchor => "anchor",
            Self::Frozen => "frozen",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8SemanticRelation {
    pub(crate) name: String,
    pub(crate) owner_locus: String,
    pub(crate) selected_option_index: usize,
    pub(crate) selected_floor: M8RelationFloor,
    pub(crate) selected_anchor: String,
    pub(crate) selected_option_epoch: String,
    pub(crate) primary_epoch: String,
    pub(crate) binding_epoch: String,
    pub(crate) binding_frontier: String,
    pub(crate) active_lease_ref: String,
    pub(crate) activation_frontier: String,
    pub(crate) lineage: Vec<String>,
}

/// Internal initial state for one admitted M7 maintained relation.  Keeping it
/// as a named carrier avoids a parallel relation snapshot while making each
/// semantic field explicit at the M8 admission boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8SemanticRelationInitialState {
    pub(crate) name: String,
    pub(crate) owner_locus: String,
    pub(crate) selected_option_index: usize,
    pub(crate) selected_anchor: String,
    pub(crate) primary_epoch: String,
    pub(crate) binding_epoch: String,
    pub(crate) binding_frontier: String,
    pub(crate) active_lease_ref: String,
    pub(crate) activation_frontier: String,
    pub(crate) lineage: Vec<String>,
}

impl M8SemanticRelation {
    pub(crate) fn from_initial_state(initial: M8SemanticRelationInitialState) -> Self {
        Self {
            name: initial.name,
            owner_locus: initial.owner_locus,
            selected_option_index: initial.selected_option_index,
            selected_floor: M8RelationFloor::Live,
            selected_anchor: initial.selected_anchor,
            selected_option_epoch: initial.primary_epoch.clone(),
            primary_epoch: initial.primary_epoch,
            binding_epoch: initial.binding_epoch,
            binding_frontier: initial.binding_frontier,
            active_lease_ref: initial.active_lease_ref,
            activation_frontier: initial.activation_frontier,
            lineage: initial.lineage,
        }
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub const fn selected_option_index(&self) -> usize {
        self.selected_option_index
    }

    pub const fn selected_floor(&self) -> M8RelationFloor {
        self.selected_floor
    }

    pub fn selected_anchor(&self) -> &str {
        &self.selected_anchor
    }

    pub fn selected_option_epoch(&self) -> &str {
        &self.selected_option_epoch
    }

    pub fn primary_epoch(&self) -> &str {
        &self.primary_epoch
    }

    pub fn binding_epoch(&self) -> &str {
        &self.binding_epoch
    }

    pub fn binding_frontier(&self) -> &str {
        &self.binding_frontier
    }

    pub fn active_lease_ref(&self) -> &str {
        &self.active_lease_ref
    }

    pub fn activation_frontier(&self) -> &str {
        &self.activation_frontier
    }

    pub fn lineage(&self) -> &[String] {
        &self.lineage
    }

    pub fn lineage_contains(&self, fragment: &str) -> bool {
        self.lineage.iter().any(|entry| entry.contains(fragment))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8Occurrence {
    id: String,
}

impl M8Occurrence {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8OwnerQueue {
    occurrence_ids: Vec<String>,
}

impl M8OwnerQueue {
    pub fn occurrence_ids(&self) -> Vec<String> {
        self.occurrence_ids.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8DeclaredFailure {
    StaleMembership,
    MissingCapability,
    MissingWitness,
    RouteUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8ServeDiagnosticKind {
    DeclaredFailure(M8DeclaredFailure),
    OwnerQueueEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8EnqueueDiagnosticKind {
    UnknownEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8EnqueueDiagnostic {
    kind: M8EnqueueDiagnosticKind,
    evaluation: String,
    source_ref: SourceRef,
}

impl M8EnqueueDiagnostic {
    pub const fn kind(&self) -> M8EnqueueDiagnosticKind {
        self.kind
    }

    pub fn evaluation(&self) -> &str {
        &self.evaluation
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8EnqueueDiagnostics {
    entries: Vec<M8EnqueueDiagnostic>,
}

impl M8EnqueueDiagnostics {
    fn unknown_evaluation(evaluation: &str, source_ref: SourceRef) -> Self {
        Self {
            entries: vec![M8EnqueueDiagnostic {
                kind: M8EnqueueDiagnosticKind::UnknownEvaluation,
                evaluation: evaluation.to_string(),
                source_ref,
            }],
        }
    }

    pub fn primary(&self) -> &M8EnqueueDiagnostic {
        self.entries
            .first()
            .expect("M8 enqueue diagnostics always have a primary entry")
    }

    pub fn entries(&self) -> &[M8EnqueueDiagnostic] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ServeDiagnostic {
    kind: M8ServeDiagnosticKind,
    source_ref: SourceRef,
}

impl M8ServeDiagnostic {
    pub const fn kind(&self) -> M8ServeDiagnosticKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ServeDiagnostics {
    entries: Vec<M8ServeDiagnostic>,
    outcome: Box<M8ServeOutcome>,
}

impl M8ServeDiagnostics {
    fn failure(
        failure: M8DeclaredFailure,
        evaluation: &str,
        owner_locus: &str,
        source_ref: SourceRef,
    ) -> Self {
        Self {
            entries: vec![M8ServeDiagnostic {
                kind: M8ServeDiagnosticKind::DeclaredFailure(failure),
                source_ref: source_ref.clone(),
            }],
            outcome: Box::new(M8ServeOutcome::from_failure(
                evaluation,
                owner_locus,
                failure,
                source_ref,
            )),
        }
    }

    fn queue_empty(owner_locus: &str, source_ref: SourceRef) -> Self {
        Self {
            entries: vec![M8ServeDiagnostic {
                kind: M8ServeDiagnosticKind::OwnerQueueEmpty,
                source_ref: source_ref.clone(),
            }],
            outcome: Box::new(M8ServeOutcome::from_failure(
                "",
                owner_locus,
                M8DeclaredFailure::RouteUnavailable,
                source_ref,
            )),
        }
    }

    pub fn primary(&self) -> &M8ServeDiagnostic {
        self.entries
            .first()
            .expect("M8 serve diagnostics always have a primary entry")
    }

    pub fn entries(&self) -> &[M8ServeDiagnostic] {
        &self.entries
    }

    pub fn outcome(&self) -> &M8ServeOutcome {
        &self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ServeOutcome {
    evaluation: String,
    owner_locus: String,
    failure: Option<M8DeclaredFailure>,
    reads: BTreeMap<M8StateKey, i64>,
    writes: BTreeMap<M8StateKey, i64>,
    source_ref: SourceRef,
}

impl M8ServeOutcome {
    fn success(
        evaluation: &str,
        owner_locus: &str,
        reads: BTreeMap<M8StateKey, i64>,
        writes: BTreeMap<M8StateKey, i64>,
        source_ref: SourceRef,
    ) -> Self {
        Self {
            evaluation: evaluation.to_string(),
            owner_locus: owner_locus.to_string(),
            failure: None,
            reads,
            writes,
            source_ref,
        }
    }

    fn from_failure(
        evaluation: &str,
        owner_locus: &str,
        failure: M8DeclaredFailure,
        source_ref: SourceRef,
    ) -> Self {
        Self {
            evaluation: evaluation.to_string(),
            owner_locus: owner_locus.to_string(),
            failure: Some(failure),
            reads: BTreeMap::new(),
            writes: BTreeMap::new(),
            source_ref,
        }
    }

    pub fn evaluation(&self) -> &str {
        &self.evaluation
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub const fn failure(&self) -> Option<M8DeclaredFailure> {
        self.failure
    }

    pub fn read_int(&self, key: &M8StateKey) -> Option<i64> {
        self.reads.get(key).copied()
    }

    pub fn written_int(&self, key: &M8StateKey) -> Option<i64> {
        self.writes.get(key).copied()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8QueueTraceKind {
    Enqueued,
    TypedEnqueueRejected,
    AuthorityValidated,
    OwnerRead,
    OwnerWrite,
    DeclaredFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8QueueTraceEntry {
    kind: M8QueueTraceKind,
    trace_node_id: String,
    trace_node_index: u64,
    request_occurrence_id: Option<String>,
    dependencies: BTreeSet<String>,
    authority: M8AuthorityUse,
    failure: Option<M8DeclaredFailure>,
    enqueue_diagnostic_kind: Option<M8EnqueueDiagnosticKind>,
    read_values: BTreeMap<M8StateKey, i64>,
    written_values: BTreeMap<M8StateKey, i64>,
    source_ref: SourceRef,
}

impl M8QueueTraceEntry {
    pub const fn kind(&self) -> M8QueueTraceKind {
        self.kind
    }

    pub fn trace_node_id(&self) -> &str {
        &self.trace_node_id
    }

    pub const fn trace_node_index(&self) -> u64 {
        self.trace_node_index
    }

    pub fn request_occurrence_id(&self) -> Option<&str> {
        self.request_occurrence_id.as_deref()
    }

    pub fn dependencies(&self) -> &BTreeSet<String> {
        &self.dependencies
    }

    pub fn authority(&self) -> &M8AuthorityUse {
        &self.authority
    }

    pub const fn failure(&self) -> Option<M8DeclaredFailure> {
        self.failure
    }

    pub const fn enqueue_diagnostic_kind(&self) -> Option<M8EnqueueDiagnosticKind> {
        self.enqueue_diagnostic_kind
    }

    pub fn read_int(&self, key: &M8StateKey) -> Option<i64> {
        self.read_values.get(key).copied()
    }

    pub fn written_int(&self, key: &M8StateKey) -> Option<i64> {
        self.written_values.get(key).copied()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

/// Internal semantic trace for the bounded M8 queue.  It deliberately retains
/// authority values for local evidence; it is not an observer-safe export.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8QueueTrace {
    entries: Vec<M8QueueTraceEntry>,
}

impl M8QueueTrace {
    pub fn entries(&self) -> &[M8QueueTraceEntry] {
        &self.entries
    }

    pub fn kinds(&self) -> Vec<M8QueueTraceKind> {
        self.entries.iter().map(M8QueueTraceEntry::kind).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueuedOwnerRequest {
    occurrence: M8Occurrence,
    enqueue_trace_node_id: String,
    request: M8OwnerRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RuntimeExecution {
    owner_plans: Vec<M8OwnerExecutionPlan>,
    pub(crate) snapshot: M8SemanticSnapshot,
    owner_queues: BTreeMap<String, VecDeque<QueuedOwnerRequest>>,
    trace: M8QueueTrace,
    pub(crate) next_occurrence: u64,
    pub(crate) next_trace_node: u64,
    rejection_source_ref: SourceRef,
}

impl M8RuntimeExecution {
    pub fn from_admitted(instance: M8RuntimeInstance, seed: M8ExecutionSeed) -> Self {
        Self {
            owner_plans: instance.owner_execution_plans().to_vec(),
            snapshot: M8SemanticSnapshot::from_seed(seed),
            owner_queues: BTreeMap::new(),
            trace: M8QueueTrace::default(),
            next_occurrence: 0,
            next_trace_node: 0,
            rejection_source_ref: instance.program_identity().root_source_ref().clone(),
        }
    }

    /// Exact replay-relevant owner execution state for M8 store receipts.
    /// Pending FIFO contents, occurrence counters, and queue trace nodes are
    /// mutable state and therefore cannot be omitted from save/load evidence.
    pub(crate) fn canonical_store_projection(&self) -> String {
        let queues = self.owner_queues.iter().flat_map(|(locus, queue)| {
            queue.iter().map(move |queued| {
                format!(
                    "queue|{}|{}|{}|{}|{}|{}|{}|{}",
                    locus,
                    queued.occurrence.id(),
                    queued.enqueue_trace_node_id,
                    queued.request.evaluation(),
                    queued
                        .request
                        .arguments()
                        .iter()
                        .map(|(name, value)| format!("{name}={value}"))
                        .collect::<Vec<_>>()
                        .join(","),
                    queued
                        .request
                        .authority_use()
                        .and_then(M8AuthorityUse::membership_ref)
                        .unwrap_or(""),
                    queued
                        .request
                        .authority_use()
                        .and_then(M8AuthorityUse::capability_ref)
                        .unwrap_or(""),
                    queued
                        .request
                        .authority_use()
                        .and_then(M8AuthorityUse::witness_ref)
                        .unwrap_or(""),
                )
            })
        });
        let trace = self.trace.entries().iter().map(|entry| {
            format!(
                "queue_trace|{:?}|{}|{}",
                entry.kind(),
                entry.trace_node_id(),
                entry.request_occurrence_id().unwrap_or(""),
            )
        });
        std::iter::once(format!(
            "owner_counters|{}|{}",
            self.next_occurrence, self.next_trace_node,
        ))
        .chain(queues)
        .chain(trace)
        .collect::<Vec<_>>()
        .join("\n")
    }

    /// M7 rejection is terminal for this queue facade: it never manufactures a
    /// checked owner plan, request, or receipt path from a rejected source.
    pub fn from_rejected_m7(
        _diagnostics: SurfaceV0PipelineDiagnostics,
    ) -> Result<Self, M8RejectedM7> {
        Err(M8RejectedM7)
    }

    pub fn enqueue(&mut self, request: M8OwnerRequest) -> M8Occurrence {
        self.try_enqueue(request).expect(
            "M8RuntimeExecution::enqueue is only a convenience for evaluations retained by the admitted checked artifact",
        )
    }

    pub fn try_enqueue(
        &mut self,
        request: M8OwnerRequest,
    ) -> Result<M8Occurrence, M8EnqueueDiagnostics> {
        let Some(plan) = self.owner_plan(request.evaluation()).cloned() else {
            let authority = request
                .authority_use()
                .cloned()
                .unwrap_or_else(|| M8AuthorityUse::for_principal(""));
            let diagnostics = M8EnqueueDiagnostics::unknown_evaluation(
                request.evaluation(),
                self.rejection_source_ref.clone(),
            );
            self.append_trace(
                M8QueueTraceKind::TypedEnqueueRejected,
                None,
                BTreeSet::new(),
                authority,
                None,
                Some(M8EnqueueDiagnosticKind::UnknownEvaluation),
                BTreeMap::new(),
                BTreeMap::new(),
                self.rejection_source_ref.clone(),
            );
            return Err(diagnostics);
        };
        let occurrence = M8Occurrence::new(format!("m8-occurrence-{:020}", self.next_occurrence));
        self.next_occurrence += 1;
        let authority = request
            .authority_use()
            .cloned()
            .unwrap_or_else(|| M8AuthorityUse::for_principal(""));
        let enqueue_trace_node_id = self.append_trace(
            M8QueueTraceKind::Enqueued,
            Some(&occurrence),
            BTreeSet::new(),
            authority,
            None,
            None,
            BTreeMap::new(),
            BTreeMap::new(),
            plan.source_ref().clone(),
        );
        self.owner_queues
            .entry(plan.owner_locus().to_string())
            .or_default()
            .push_back(QueuedOwnerRequest {
                occurrence: occurrence.clone(),
                enqueue_trace_node_id,
                request,
            });
        Ok(occurrence)
    }

    pub fn owner_queue(&self, owner_locus: &str) -> M8OwnerQueue {
        M8OwnerQueue {
            occurrence_ids: self
                .owner_queues
                .get(owner_locus)
                .map(|queue| {
                    queue
                        .iter()
                        .map(|queued| queued.occurrence.id().to_string())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    pub fn snapshot(&self) -> M8SemanticSnapshot {
        self.snapshot.clone()
    }

    pub fn trace(&self) -> &M8QueueTrace {
        &self.trace
    }

    pub(crate) fn replace_admitted_plans(&mut self, instance: &M8RuntimeInstance) {
        self.owner_plans = instance.owner_execution_plans().to_vec();
        self.rejection_source_ref = instance.program_identity().root_source_ref().clone();
    }

    pub(crate) fn has_pending_requests(&self) -> bool {
        self.owner_queues.values().any(|queue| !queue.is_empty())
    }

    pub(crate) fn equivalent_without_plans(&self, other: &Self) -> bool {
        self.snapshot == other.snapshot
            && self.owner_queues == other.owner_queues
            && self.trace == other.trace
            && self.next_occurrence == other.next_occurrence
            && self.next_trace_node == other.next_trace_node
    }

    pub fn serve_next_owner(
        &mut self,
        owner_locus: &str,
    ) -> Result<M8ServeOutcome, M8ServeDiagnostics> {
        let queued = match self
            .owner_queues
            .get_mut(owner_locus)
            .and_then(VecDeque::pop_front)
        {
            Some(queued) => queued,
            None => {
                return Err(M8ServeDiagnostics::queue_empty(
                    owner_locus,
                    SourceRef::new("<m8-owner-queue>", 1, 1, 1, 1),
                ));
            }
        };
        let plan = self
            .owner_plan(queued.request.evaluation())
            .expect("queued request was accepted only from an admitted owner plan")
            .clone();
        let authority = queued
            .request
            .authority_use()
            .cloned()
            .unwrap_or_else(|| M8AuthorityUse::for_principal(""));

        if let Some(failure) = self.authority_failure(&plan, &authority) {
            let diagnostics = M8ServeDiagnostics::failure(
                failure,
                plan.evaluation(),
                plan.owner_locus(),
                plan.source_ref().clone(),
            );
            self.append_trace(
                M8QueueTraceKind::DeclaredFailure,
                Some(&queued.occurrence),
                BTreeSet::from([queued.enqueue_trace_node_id.clone()]),
                authority,
                Some(failure),
                None,
                BTreeMap::new(),
                BTreeMap::new(),
                plan.source_ref().clone(),
            );
            return Err(diagnostics);
        }

        let target = match self.materialize_key(plan.target(), queued.request.arguments()) {
            Some(target) if plan.target().owner_locus() == plan.owner_locus() => target,
            _ => {
                return Err(self.route_failure(
                    &queued.occurrence,
                    &queued.enqueue_trace_node_id,
                    authority,
                    &plan,
                ));
            }
        };
        let mut reads = BTreeMap::new();
        let value = match self.evaluate_expression(
            plan.expression().tree(),
            queued.request.arguments(),
            plan.owner_locus(),
            &mut reads,
        ) {
            Ok(value) => value,
            Err(()) => {
                return Err(self.route_failure(
                    &queued.occurrence,
                    &queued.enqueue_trace_node_id,
                    authority,
                    &plan,
                ));
            }
        };
        let mut writes = BTreeMap::new();
        writes.insert(target.clone(), value);

        // The read set and write set are completely formed before the one
        // mutation below, so this owner RMW observes service-time state and
        // commits atomically with respect to this deterministic queue.
        self.snapshot.ints.insert(target, value);
        let validation_trace_node_id = self.append_trace(
            M8QueueTraceKind::AuthorityValidated,
            Some(&queued.occurrence),
            BTreeSet::from([queued.enqueue_trace_node_id.clone()]),
            authority.clone(),
            None,
            None,
            BTreeMap::new(),
            BTreeMap::new(),
            plan.source_ref().clone(),
        );
        let read_trace_node_id = self.append_trace(
            M8QueueTraceKind::OwnerRead,
            Some(&queued.occurrence),
            BTreeSet::from([validation_trace_node_id]),
            authority.clone(),
            None,
            None,
            reads.clone(),
            BTreeMap::new(),
            plan.source_ref().clone(),
        );
        self.append_trace(
            M8QueueTraceKind::OwnerWrite,
            Some(&queued.occurrence),
            BTreeSet::from([read_trace_node_id]),
            authority,
            None,
            None,
            reads.clone(),
            writes.clone(),
            plan.source_ref().clone(),
        );
        Ok(M8ServeOutcome::success(
            plan.evaluation(),
            plan.owner_locus(),
            reads,
            writes,
            plan.source_ref().clone(),
        ))
    }

    pub fn run_replay(mut self, requests: Vec<M8OwnerRequest>) -> M8ReplayReport {
        let mut outcomes = Vec::with_capacity(requests.len());
        for request in requests {
            let owner_locus = self
                .owner_plan(request.evaluation())
                .expect("replay requests must name an admitted owner evaluation")
                .owner_locus()
                .to_string();
            self.enqueue(request);
            match self.serve_next_owner(&owner_locus) {
                Ok(outcome) => outcomes.push(outcome),
                Err(diagnostics) => outcomes.push(diagnostics.outcome().clone()),
            }
        }
        M8ReplayReport {
            outcomes,
            snapshot: self.snapshot,
            trace: self.trace,
        }
    }

    fn owner_plan(&self, evaluation: &str) -> Option<&M8OwnerExecutionPlan> {
        self.owner_plans
            .iter()
            .find(|plan| plan.evaluation() == evaluation)
    }

    fn authority_failure(
        &self,
        plan: &M8OwnerExecutionPlan,
        authority: &M8AuthorityUse,
    ) -> Option<M8DeclaredFailure> {
        if authority.principal() != plan.actor() {
            return Some(M8DeclaredFailure::StaleMembership);
        }
        match self.snapshot.authority_state().validate_owner_use(
            authority.principal(),
            authority.membership_ref(),
            authority.capability_ref(),
            authority.witness_ref(),
            plan.owner_locus(),
            plan.evaluation(),
        ) {
            Ok(()) => None,
            Err(M8AuthorityValidationFailure::StaleMembership) => {
                Some(M8DeclaredFailure::StaleMembership)
            }
            Err(M8AuthorityValidationFailure::MissingCapability) => {
                Some(M8DeclaredFailure::MissingCapability)
            }
            Err(M8AuthorityValidationFailure::MissingWitness) => {
                Some(M8DeclaredFailure::MissingWitness)
            }
        }
    }

    fn materialize_key(
        &self,
        read: &mir_semantics::surface_v0_pipeline::TypedStateRead,
        arguments: &BTreeMap<String, String>,
    ) -> Option<M8StateKey> {
        Some(M8StateKey::indexed_field(
            read.namespace(),
            read.index()
                .map(|index| arguments.get(index).map(String::as_str).unwrap_or(index))?,
            read.field()?,
        ))
    }

    fn evaluate_expression(
        &self,
        tree: &CheckedExpressionTree,
        arguments: &BTreeMap<String, String>,
        owner_locus: &str,
        reads: &mut BTreeMap<M8StateKey, i64>,
    ) -> Result<i64, ()> {
        match tree {
            CheckedExpressionTree::StateRead(read) => {
                if read.owner_locus() != owner_locus {
                    return Err(());
                }
                let key = self.materialize_key(read, arguments).ok_or(())?;
                let value = self.snapshot.int(&key).ok_or(())?;
                reads.insert(key, value);
                Ok(value)
            }
            CheckedExpressionTree::ParameterRead { name, .. } => arguments
                .get(name)
                .and_then(|value| value.parse::<i64>().ok())
                .ok_or(()),
            CheckedExpressionTree::IntegerLiteral(literal) => Ok(literal.value()),
            CheckedExpressionTree::Binary {
                operator,
                left,
                right,
                ..
            } => {
                let left = self.evaluate_expression(left, arguments, owner_locus, reads)?;
                let right = self.evaluate_expression(right, arguments, owner_locus, reads)?;
                match operator {
                    CheckedBinaryOperator::Add => left.checked_add(right).ok_or(()),
                    CheckedBinaryOperator::Subtract => left.checked_sub(right).ok_or(()),
                }
            }
        }
    }

    fn route_failure(
        &mut self,
        occurrence: &M8Occurrence,
        enqueue_trace_node_id: &str,
        authority: M8AuthorityUse,
        plan: &M8OwnerExecutionPlan,
    ) -> M8ServeDiagnostics {
        let failure = M8DeclaredFailure::RouteUnavailable;
        let diagnostics = M8ServeDiagnostics::failure(
            failure,
            plan.evaluation(),
            plan.owner_locus(),
            plan.source_ref().clone(),
        );
        self.append_trace(
            M8QueueTraceKind::DeclaredFailure,
            Some(occurrence),
            BTreeSet::from([enqueue_trace_node_id.to_string()]),
            authority,
            Some(failure),
            None,
            BTreeMap::new(),
            BTreeMap::new(),
            plan.source_ref().clone(),
        );
        diagnostics
    }

    #[allow(clippy::too_many_arguments)]
    fn append_trace(
        &mut self,
        kind: M8QueueTraceKind,
        request_occurrence: Option<&M8Occurrence>,
        dependencies: BTreeSet<String>,
        authority: M8AuthorityUse,
        failure: Option<M8DeclaredFailure>,
        enqueue_diagnostic_kind: Option<M8EnqueueDiagnosticKind>,
        read_values: BTreeMap<M8StateKey, i64>,
        written_values: BTreeMap<M8StateKey, i64>,
        source_ref: SourceRef,
    ) -> String {
        let trace_node_index = self.next_trace_node;
        self.next_trace_node += 1;
        let trace_node_id = format!("m8-trace-node-{trace_node_index:020}");
        self.trace.entries.push(M8QueueTraceEntry {
            kind,
            trace_node_id: trace_node_id.clone(),
            trace_node_index,
            request_occurrence_id: request_occurrence.map(|occurrence| occurrence.id().to_string()),
            dependencies,
            authority,
            failure,
            enqueue_diagnostic_kind,
            read_values,
            written_values,
            source_ref,
        });
        trace_node_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M8RejectedM7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ReplayReport {
    outcomes: Vec<M8ServeOutcome>,
    snapshot: M8SemanticSnapshot,
    trace: M8QueueTrace,
}

impl M8ReplayReport {
    pub fn outcomes(&self) -> &[M8ServeOutcome] {
        &self.outcomes
    }

    pub fn snapshot(&self) -> &M8SemanticSnapshot {
        &self.snapshot
    }

    pub fn trace(&self) -> &M8QueueTrace {
        &self.trace
    }
}
