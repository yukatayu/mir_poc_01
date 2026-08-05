//! Unified local M8 session and finite cut/restore boundary.
//!
//! The session keeps one mutable semantic snapshot and moves it through the
//! existing owner, relation, and designated execution carriers for an
//! operation.  The carriers therefore share the actual authority inventory
//! and relation/owner state rather than reconciling facade snapshots later.

use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use mir_semantics::{
    shared_model::{OccurrenceId, ResultVersion, SourceRef, TraceEntry, TraceKind},
    surface_v0_pipeline::CheckedProgramIdentity,
};

use crate::{
    m8_runtime_admission::{
        EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionEvidence, M8RuntimeInstance,
        M8SecurityClass,
    },
    m8_runtime_authority::M8AuthorityState,
    m8_runtime_designated_value::{
        M8ConsumeRequest, M8ConsumedDesignatedValue, M8ConsumptionState, M8DesignatedAuthorityUse,
        M8DesignatedDiagnosticKind, M8DesignatedDiagnostics, M8DesignatedEvaluationRequest,
        M8DesignatedResultStore, M8DesignatedRuntime, M8DesignatedSeed, M8DesignatedTrace,
        M8DesignatedTraceKind, M8InputReceiptSet, M8PresentationInterpolation, M8ReceiptState,
        M8ResultVersionStore,
    },
    m8_runtime_owner_queue::{
        M8AuthorityUse, M8EnqueueDiagnosticKind, M8EnqueueDiagnostics, M8EntityPresenceRegistry,
        M8ExecutionSeed, M8Occurrence, M8OwnerRequest, M8QueueTraceKind, M8RuntimeExecution,
        M8SemanticRelation, M8SemanticSnapshot, M8ServeDiagnosticKind, M8ServeDiagnostics,
        M8ServeOutcome, M8StateKey,
    },
    m8_runtime_relation_projection::{
        M8BindingInvalidation, M8FiniteFallbackChain, M8FiniteFallbackSelection,
        M8PresentationContext, M8ProjectionDiagnostics, M8RelationAuthorityUse,
        M8RelationDiagnosticKind, M8RelationDiagnostics, M8RelationProjection,
        M8RelationProjectionRuntime, M8RelationProjectionSeed, M8RelationReacquire,
        M8RelationTrace, M8RelationTraceKind,
    },
    m9_auth_verification::{M9M8EntityPresenceBridge, M9M8EntityPresenceStatus},
};

pub use crate::m8_runtime_relation_projection::{M8LeaseInventory, M8LeaseRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalRuntimeSeed {
    owner_ints: BTreeMap<M8StateKey, i64>,
    authority_state: M8AuthorityState,
    live_leases: M8LeaseInventory,
    designated_input_receipts: M8InputReceiptSet,
}

impl M8LocalRuntimeSeed {
    pub fn new() -> Self {
        Self {
            owner_ints: BTreeMap::new(),
            authority_state: M8AuthorityState::new(),
            live_leases: M8LeaseInventory::default(),
            designated_input_receipts: M8InputReceiptSet::new(),
        }
    }

    pub fn with_owner_int(mut self, key: M8StateKey, value: i64) -> Self {
        self.owner_ints.insert(key, value);
        self
    }

    pub fn with_authority_state(mut self, authority_state: M8AuthorityState) -> Self {
        self.authority_state = authority_state;
        self
    }

    pub fn with_live_lease(mut self, lease: M8LeaseRecord) -> Self {
        self.live_leases
            .records
            .insert(lease.reference.clone(), lease);
        self
    }

    pub fn with_designated_input_receipts(
        mut self,
        designated_input_receipts: M8InputReceiptSet,
    ) -> Self {
        self.designated_input_receipts = designated_input_receipts;
        self
    }
}

impl Default for M8LocalRuntimeSeed {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalAdmissionProvenance {
    program_identity: CheckedProgramIdentity,
    evidence: Vec<M8AdmissionEvidence>,
}

impl M8LocalAdmissionProvenance {
    fn from_instance(instance: &M8RuntimeInstance) -> Self {
        Self {
            program_identity: instance.program_identity().clone(),
            evidence: instance.admission_evidence().entries().to_vec(),
        }
    }

    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn evidence(&self) -> &[M8AdmissionEvidence] {
        &self.evidence
    }

    pub const fn uses_structural_equality(&self) -> bool {
        true
    }

    pub const fn uses_hash_fingerprint(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8LocalPatchLifecycleState {
    Placeholder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalPatchLifecycle {
    state: M8LocalPatchLifecycleState,
    rows: Vec<String>,
}

impl Default for M8LocalPatchLifecycle {
    fn default() -> Self {
        Self {
            state: M8LocalPatchLifecycleState::Placeholder,
            rows: Vec::new(),
        }
    }
}

impl M8LocalPatchLifecycle {
    pub const fn state(&self) -> M8LocalPatchLifecycleState {
        self.state
    }

    pub fn rows(&self) -> &[String] {
        &self.rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8LocalTraceKind {
    OwnerEnqueued,
    OwnerAuthorityValidated,
    OwnerRead,
    OwnerWrite,
    RelationPrimaryInvalidated,
    RelationOptionAdvanced,
    RelationFallbackFrozen,
    RelationPrimaryReturnIgnored,
    RelationFreshLineageReacquired,
    DesignatedAuthorityValidated,
    DesignatedInputReceiptValidated,
    DesignatedValuePublished,
    DesignatedConsumerAuthorityValidated,
    DesignatedValueConsumed,
    PatchStateInitialized,
    LocalCutSaved,
    RestoreRejected,
    OwnerEnqueueRejected,
    OwnerServeRejected,
    RelationTransitionRejected,
    DesignatedEvaluationRejected,
    DesignatedConsumptionRejected,
    EntityPresenceSynchronized,
    EntityPresenceControlApplied,
}

/// Local-only references retained in K8/H.  They never cross the runtime
/// crate boundary; observer export receives a separate redacted projection.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct M8LocalAuthorityRefs {
    principal: Option<String>,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    witness_ref: Option<String>,
}

impl M8LocalAuthorityRefs {
    fn from_owner(authority: &M8AuthorityUse) -> Self {
        Self {
            principal: Some(authority.principal().to_string()),
            membership_ref: authority.membership_ref().map(ToOwned::to_owned),
            capability_ref: authority.capability_ref().map(ToOwned::to_owned),
            witness_ref: authority.witness_ref().map(ToOwned::to_owned),
        }
    }

    fn from_relation(authority: &M8RelationAuthorityUse) -> Self {
        Self {
            principal: authority.principal().map(ToOwned::to_owned),
            membership_ref: authority.membership_ref().map(ToOwned::to_owned),
            capability_ref: authority.capability_ref().map(ToOwned::to_owned),
            witness_ref: authority.witness_ref().map(ToOwned::to_owned),
        }
    }

    fn from_designated(authority: Option<&M8DesignatedAuthorityUse>) -> Self {
        Self {
            principal: None,
            membership_ref: authority
                .and_then(M8DesignatedAuthorityUse::membership_ref)
                .map(ToOwned::to_owned),
            capability_ref: authority
                .and_then(M8DesignatedAuthorityUse::capability_ref)
                .map(ToOwned::to_owned),
            witness_ref: authority
                .and_then(M8DesignatedAuthorityUse::witness_ref)
                .map(ToOwned::to_owned),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum M8LocalFailure {
    OwnerEnqueue(M8EnqueueDiagnosticKind),
    OwnerServe(M8ServeDiagnosticKind),
    RelationTransition(M8RelationDiagnosticKind),
    DesignatedEvaluation(M8DesignatedDiagnosticKind),
    DesignatedConsumption(M8DesignatedDiagnosticKind),
}

/// Observer-safe structural failure row.  It retains no raw authority or
/// witness reference from H.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverFailureRow {
    failure: M8LocalFailure,
    node_id: String,
    dependencies: Vec<String>,
    source_ref: SourceRef,
    occurrence_id: Option<String>,
    label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
}

impl M8ObserverFailureRow {
    pub fn failure_family(&self) -> &str {
        self.failure.family()
    }

    pub fn failure_kind(&self) -> String {
        self.failure.kind_name()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn operation_occurrence_id(&self) -> Option<&str> {
        self.occurrence_id.as_deref()
    }

    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    pub fn label(&self) -> &EvidenceSecurityLabel {
        &self.label
    }

    pub fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }

    pub const fn authority_refs_are_redacted(&self) -> bool {
        true
    }

    pub fn has_actual_earlier_dependencies(&self, trace: &M8LocalTrace) -> bool {
        !self.dependencies.is_empty()
            && self
                .dependencies
                .iter()
                .all(|dependency| trace.contains_edge(dependency, &self.node_id))
    }
}

/// Typed observer-safe failure rows derived from H without copying authority
/// or fresh-witness references.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8ObserverFailureRows {
    rows: Vec<M8ObserverFailureRow>,
}

impl M8ObserverFailureRows {
    pub fn owner_enqueue(&self, kind: M8EnqueueDiagnosticKind) -> Option<&M8ObserverFailureRow> {
        self.rows
            .iter()
            .find(|row| row.failure == M8LocalFailure::OwnerEnqueue(kind))
    }

    pub fn owner_serve(&self, kind: M8ServeDiagnosticKind) -> Option<&M8ObserverFailureRow> {
        self.rows
            .iter()
            .find(|row| row.failure == M8LocalFailure::OwnerServe(kind))
    }

    pub fn relation_transition(
        &self,
        kind: M8RelationDiagnosticKind,
    ) -> Option<&M8ObserverFailureRow> {
        self.rows
            .iter()
            .find(|row| row.failure == M8LocalFailure::RelationTransition(kind))
    }

    pub fn designated_evaluation(
        &self,
        kind: M8DesignatedDiagnosticKind,
    ) -> Option<&M8ObserverFailureRow> {
        self.rows
            .iter()
            .find(|row| row.failure == M8LocalFailure::DesignatedEvaluation(kind))
    }

    pub fn designated_consumption(
        &self,
        kind: M8DesignatedDiagnosticKind,
    ) -> Option<&M8ObserverFailureRow> {
        self.rows
            .iter()
            .find(|row| row.failure == M8LocalFailure::DesignatedConsumption(kind))
    }

    pub fn to_redacted_structural_json(&self) -> String {
        serde_json::Value::Array(
            self.rows
                .iter()
                .map(|row| {
                    serde_json::json!({
                        "failure_family": row.failure_family(),
                        "failure_kind": row.failure_kind(),
                        "label": row.label().as_str(),
                        "security_class": format!("{:?}", row.label().security_class()),
                        "redaction": row.redaction().as_str(),
                        "source_ref": {
                            "path": row.source_ref().path.as_str(),
                            "start_line": row.source_ref().start_line,
                            "start_column": row.source_ref().start_column,
                            "end_line": row.source_ref().end_line,
                            "end_column": row.source_ref().end_column,
                        },
                        "operation_occurrence_id": row.operation_occurrence_id(),
                        "dependencies": row.dependencies(),
                    })
                })
                .collect(),
        )
        .to_string()
    }
}

impl M8LocalFailure {
    fn family(&self) -> &'static str {
        match self {
            Self::OwnerEnqueue(_) => "owner_enqueue",
            Self::OwnerServe(_) => "owner_serve",
            Self::RelationTransition(_) => "relation_transition",
            Self::DesignatedEvaluation(_) => "designated_evaluation",
            Self::DesignatedConsumption(_) => "designated_consumption",
        }
    }

    fn kind_name(&self) -> String {
        match self {
            Self::OwnerServe(M8ServeDiagnosticKind::DeclaredFailure(failure)) => {
                format!("DeclaredFailure::{failure:?}")
            }
            Self::OwnerEnqueue(kind) => format!("{kind:?}"),
            Self::OwnerServe(kind) => format!("{kind:?}"),
            Self::RelationTransition(kind) => format!("{kind:?}"),
            Self::DesignatedEvaluation(kind) => format!("{kind:?}"),
            Self::DesignatedConsumption(kind) => format!("{kind:?}"),
        }
    }

    fn observer_policy(&self) -> (EvidenceSecurityLabel, EvidenceRedaction) {
        (
            EvidenceSecurityLabel::new(format!("m8:observer:failure:{}", self.family()))
                .with_class(M8SecurityClass::Private),
            EvidenceRedaction::new("m8:redact:failure-structure"),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M8LocalTraceEntry {
    kind: M8LocalTraceKind,
    node_id: String,
    node_index: u64,
    dependencies: BTreeSet<String>,
    source_ref: SourceRef,
    occurrence_id: Option<String>,
    restore_diagnostic_kind: Option<M8LocalRestoreDiagnosticKind>,
    outside_saved_payload: bool,
    authority: M8LocalAuthorityRefs,
    failure: Option<M8LocalFailure>,
    fresh_witness_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8LocalTraceObservation {
    pub(crate) node_id: String,
    pub(crate) dependencies: Vec<String>,
    pub(crate) source_ref: SourceRef,
    pub(crate) occurrence_id: Option<String>,
}

/// Sealed M9-to-M8 entity-presence synchronization evidence. It carries only
/// opaque M9 references; raw membership provenance remains inside M9.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8EntityPresenceSynchronization {
    pub(crate) before_status: String,
    pub(crate) after_status: String,
    pub(crate) source_ref: SourceRef,
    pub(crate) occurrence_id: String,
    pub(crate) occurrence_trace_id: String,
    pub(crate) control_id: String,
    pub(crate) control_trace_id: String,
    pub(crate) sealed_membership_ref: String,
    pub(crate) m9_snapshot_ref: String,
    pub(crate) m8_authority_use_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8LocalTrace {
    entries: Vec<M8LocalTraceEntry>,
    next_node_index: u64,
}

impl M8LocalTrace {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn prefix(&self, len: usize) -> Self {
        let entries = self.entries[..len.min(self.entries.len())].to_vec();
        let next_node_index = entries
            .last()
            .map_or(0, |entry| entry.node_index.saturating_add(1));
        Self {
            entries,
            next_node_index,
        }
    }

    pub fn suffix_from(&self, start: usize) -> Self {
        let entries = self.entries[start.min(self.entries.len())..].to_vec();
        let next_node_index = entries
            .last()
            .map_or(0, |entry| entry.node_index.saturating_add(1));
        Self {
            entries,
            next_node_index,
        }
    }

    pub fn kinds(&self) -> Vec<M8LocalTraceKind> {
        self.entries.iter().map(|entry| entry.kind).collect()
    }

    pub fn restore_diagnostic_kinds(&self) -> Vec<M8LocalRestoreDiagnosticKind> {
        self.entries
            .iter()
            .filter_map(|entry| entry.restore_diagnostic_kind)
            .collect()
    }

    pub fn observer_failure_rows(&self) -> M8ObserverFailureRows {
        M8ObserverFailureRows {
            rows: self
                .entries
                .iter()
                .filter_map(|entry| {
                    entry.failure.clone().map(|failure| {
                        let (label, redaction) = failure.observer_policy();
                        M8ObserverFailureRow {
                            failure,
                            node_id: entry.node_id.clone(),
                            dependencies: entry.dependencies.iter().cloned().collect(),
                            source_ref: entry.source_ref.clone(),
                            occurrence_id: entry.occurrence_id.clone(),
                            label,
                            redaction,
                        }
                    })
                })
                .collect(),
        }
    }

    pub fn all_entries_are_outside_saved_payload(&self) -> bool {
        self.entries.iter().all(|entry| entry.outside_saved_payload)
    }

    pub fn starts_with(&self, prefix: Self) -> bool {
        self.entries.starts_with(&prefix.entries)
    }

    pub fn node_indexes_are_monotone(&self) -> bool {
        self.entries
            .windows(2)
            .all(|pair| pair[0].node_index < pair[1].node_index)
    }

    pub fn node_ids_are_unique(&self) -> bool {
        let mut ids = BTreeSet::new();
        self.entries.iter().all(|entry| ids.insert(&entry.node_id))
    }

    pub fn contains_node_id(&self, node_id: &str) -> bool {
        self.entries.iter().any(|entry| entry.node_id == node_id)
    }

    pub fn contains_edge(&self, dependency_id: &str, node_id: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.node_id == node_id && entry.dependencies.contains(dependency_id))
    }

    pub fn source_ref_for_node_id(&self, node_id: &str) -> Option<&SourceRef> {
        self.entries
            .iter()
            .find(|entry| entry.node_id == node_id)
            .map(|entry| &entry.source_ref)
    }

    pub fn has_self_edges(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.dependencies.contains(&entry.node_id))
    }

    pub fn dependencies_only_name_earlier_nodes(&self) -> bool {
        let mut earlier = BTreeSet::new();
        self.entries.iter().all(|entry| {
            let valid = entry
                .dependencies
                .iter()
                .all(|dependency| earlier.contains(dependency));
            earlier.insert(entry.node_id.clone());
            valid
        })
    }

    fn append(
        &mut self,
        kind: M8LocalTraceKind,
        source_ref: SourceRef,
        occurrence_id: Option<String>,
        restore_diagnostic_kind: Option<M8LocalRestoreDiagnosticKind>,
        outside_saved_payload: bool,
    ) {
        self.append_fact(
            kind,
            source_ref,
            occurrence_id,
            restore_diagnostic_kind,
            outside_saved_payload,
            M8LocalAuthorityRefs::default(),
            None,
            None,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn append_fact(
        &mut self,
        kind: M8LocalTraceKind,
        source_ref: SourceRef,
        occurrence_id: Option<String>,
        restore_diagnostic_kind: Option<M8LocalRestoreDiagnosticKind>,
        outside_saved_payload: bool,
        authority: M8LocalAuthorityRefs,
        failure: Option<M8LocalFailure>,
        fresh_witness_ref: Option<String>,
    ) {
        let node_index = self.next_node_index;
        self.next_node_index += 1;
        let node_id = format!("m8-local-trace-{node_index:020}");
        let dependencies = self
            .entries
            .last()
            .map(|entry| BTreeSet::from([entry.node_id.clone()]))
            .unwrap_or_default();
        self.entries.push(M8LocalTraceEntry {
            kind,
            node_id,
            node_index,
            dependencies,
            source_ref,
            occurrence_id,
            restore_diagnostic_kind,
            outside_saved_payload,
            authority,
            failure,
            fresh_witness_ref,
        });
    }

    pub(crate) fn latest_observation(
        &self,
        kind: M8LocalTraceKind,
    ) -> Option<M8LocalTraceObservation> {
        self.entries
            .iter()
            .rev()
            .find(|entry| entry.kind == kind)
            .map(|entry| M8LocalTraceObservation {
                node_id: entry.node_id.clone(),
                dependencies: entry.dependencies.iter().cloned().collect(),
                source_ref: entry.source_ref.clone(),
                occurrence_id: entry.occurrence_id.clone(),
            })
    }

    pub(crate) fn latest_observation_for_occurrence(
        &self,
        kind: M8LocalTraceKind,
        occurrence_id: &str,
    ) -> Option<M8LocalTraceObservation> {
        self.entries
            .iter()
            .rev()
            .find(|entry| {
                entry.kind == kind && entry.occurrence_id.as_deref() == Some(occurrence_id)
            })
            .map(|entry| M8LocalTraceObservation {
                node_id: entry.node_id.clone(),
                dependencies: entry.dependencies.iter().cloned().collect(),
                source_ref: entry.source_ref.clone(),
                occurrence_id: entry.occurrence_id.clone(),
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8OwnerCounters {
    next_owner_occurrence_index: u64,
    next_owner_trace_node_index: u64,
}

impl M8OwnerCounters {
    pub const fn next_owner_occurrence_index(&self) -> u64 {
        self.next_owner_occurrence_index
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M8LocalDesignatedSaveState {
    receipt_state: M8ReceiptState,
    result_store: M8DesignatedResultStore,
    version_store: M8ResultVersionStore,
    consumption_state: M8ConsumptionState,
    trace: M8DesignatedTrace,
    next_trace_node: u64,
    next_occurrence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalSavePayload {
    shared_snapshot: M8SemanticSnapshot,
    owner_execution: M8RuntimeExecution,
    relation_trace: M8RelationTrace,
    finite_fallback_chains: BTreeMap<String, M8FiniteFallbackChain>,
    designated: M8LocalDesignatedSaveState,
    lease_inventory: M8LeaseInventory,
    patch_lifecycle: M8LocalPatchLifecycle,
}

/// A cut-local causal witness carried by each saved local cut. It uses the
/// shared trace vocabulary only to prove local-cut admission consistency:
/// restoring a cut must not retain its local receive edge when its local reply
/// predecessor is absent. It does not claim a transport exchange.
#[derive(Debug, Clone, PartialEq, Eq)]
struct M8CutReceiptCausality {
    reply: Option<TraceEntry>,
    receive: TraceEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8CutReceiptCausalityEvidence {
    pub(crate) reply_occurrence_id: Option<String>,
    pub(crate) receive_occurrence_id: String,
    pub(crate) receive_predecessor_id: Option<String>,
    pub(crate) dependency_edge_id: String,
}

impl M8CutReceiptCausality {
    fn for_cut(cut_id: &str, save_sequence: usize, source_ref: SourceRef) -> Self {
        let reply_occurrence =
            OccurrenceId::new(format!("m8-local-cut-reply-{cut_id}-{save_sequence:020}"));
        let receive_occurrence =
            OccurrenceId::new(format!("m8-local-cut-receive-{cut_id}-{save_sequence:020}"));
        Self {
            reply: Some(TraceEntry {
                kind: TraceKind::ReceiptReplied,
                occurrence: reply_occurrence.clone(),
                causal_predecessor: None,
                source_ref: Some(source_ref.clone()),
            }),
            receive: TraceEntry {
                kind: TraceKind::ReceiptReceived,
                occurrence: receive_occurrence,
                causal_predecessor: Some(reply_occurrence),
                source_ref: Some(source_ref),
            },
        }
    }

    fn is_consistent(&self) -> bool {
        let Some(reply) = self.reply.as_ref() else {
            return false;
        };
        reply.kind == TraceKind::ReceiptReplied
            && self.receive.kind == TraceKind::ReceiptReceived
            && self.receive.causal_predecessor.as_ref() == Some(&reply.occurrence)
    }

    fn evidence(&self) -> M8CutReceiptCausalityEvidence {
        let receive_occurrence_id = self.receive.occurrence.as_str().to_string();
        let receive_predecessor_id = self
            .receive
            .causal_predecessor
            .as_ref()
            .map(|occurrence| occurrence.as_str().to_string());
        M8CutReceiptCausalityEvidence {
            reply_occurrence_id: self
                .reply
                .as_ref()
                .map(|entry| entry.occurrence.as_str().to_string()),
            receive_occurrence_id: receive_occurrence_id.clone(),
            dependency_edge_id: format!(
                "m8-cut-receive-dependency|{}|{}",
                receive_predecessor_id.as_deref().unwrap_or("missing"),
                receive_occurrence_id,
            ),
            receive_predecessor_id,
        }
    }

    fn without_reply(mut self) -> Self {
        self.reply = None;
        self
    }

    fn canonical_projection(&self) -> String {
        format!(
            "reply|{}|{}\nreceive|{}|{}",
            self.reply
                .as_ref()
                .map(|entry| entry.occurrence.as_str())
                .unwrap_or("missing"),
            self.reply
                .as_ref()
                .map(|entry| format!("{:?}", entry.kind))
                .unwrap_or_else(|| "missing".to_string()),
            self.receive.occurrence.as_str(),
            self.receive
                .causal_predecessor
                .as_ref()
                .map(|occurrence| occurrence.as_str())
                .unwrap_or("missing"),
        )
    }
}

/// The saved M8 semantic state with patch lifecycle rows omitted.  This is
/// internal support for checking that an activation cut changes no owner,
/// relation, designated, lease, or authority state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8LocalSemanticPayload {
    shared_snapshot: M8SemanticSnapshot,
    owner_execution: M8RuntimeExecution,
    relation_trace: M8RelationTrace,
    designated: M8LocalDesignatedSaveState,
    lease_inventory: M8LeaseInventory,
}

impl M8LocalSemanticPayload {
    pub(crate) fn equivalent_for_activation(&self, other: &Self) -> bool {
        self.shared_snapshot == other.shared_snapshot
            && self
                .owner_execution
                .equivalent_without_plans(&other.owner_execution)
            && self.relation_trace == other.relation_trace
            && self.designated == other.designated
            && self.lease_inventory == other.lease_inventory
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalCut {
    cut_id: String,
    admission_provenance: M8LocalAdmissionProvenance,
    admitted: M8RuntimeInstance,
    payload: M8LocalSavePayload,
    cut_receipt_causality: M8CutReceiptCausality,
    trace_prefix: M8LocalTrace,
}

impl M8LocalCut {
    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        self.admission_provenance.program_identity()
    }

    pub(crate) fn cut_id(&self) -> &str {
        &self.cut_id
    }

    pub fn admission_provenance(&self) -> &M8LocalAdmissionProvenance {
        &self.admission_provenance
    }

    pub fn owner_state(&self) -> &M8SemanticSnapshot {
        &self.payload.shared_snapshot
    }

    pub fn pending_owner_fifo(&self, owner_locus: &str) -> Vec<String> {
        self.payload
            .owner_execution
            .owner_queue(owner_locus)
            .occurrence_ids()
            .to_vec()
    }

    pub fn owner_counters(&self) -> M8OwnerCounters {
        M8OwnerCounters {
            next_owner_occurrence_index: self.payload.owner_execution.next_occurrence,
            next_owner_trace_node_index: self.payload.owner_execution.next_trace_node,
        }
    }

    pub fn authority_inventory(&self) -> &M8AuthorityState {
        self.payload.shared_snapshot.authority_state()
    }

    pub fn lease_inventory(&self) -> &M8LeaseInventory {
        &self.payload.lease_inventory
    }

    pub fn relation_state(&self, relation: &str) -> Option<&M8SemanticRelation> {
        self.payload.shared_snapshot.relations.get(relation)
    }

    pub fn designated_receipt_state(&self) -> &M8ReceiptState {
        &self.payload.designated.receipt_state
    }

    pub fn designated_result_store(&self) -> &M8DesignatedResultStore {
        &self.payload.designated.result_store
    }

    pub fn designated_version_store(&self) -> &M8ResultVersionStore {
        &self.payload.designated.version_store
    }

    pub fn designated_consumption_state(&self) -> &M8ConsumptionState {
        &self.payload.designated.consumption_state
    }

    pub fn patch_lifecycle(&self) -> &M8LocalPatchLifecycle {
        &self.payload.patch_lifecycle
    }

    pub fn trace_prefix(&self) -> M8LocalTrace {
        self.trace_prefix.clone()
    }

    pub fn contains_presentation_contexts(&self) -> bool {
        false
    }

    pub fn contains_presentation_policies(&self) -> bool {
        false
    }

    pub fn contains_presentation_interpolations(&self) -> bool {
        false
    }

    pub fn save_relevant_payload(&self) -> M8LocalSavePayload {
        self.payload.clone()
    }

    pub(crate) fn cut_receipt_causality(&self) -> M8CutReceiptCausalityEvidence {
        self.cut_receipt_causality.evidence()
    }

    /// Receipt-only projection of concrete owner/designated store data.
    pub(crate) fn canonical_store_projection(&self) -> String {
        format!(
            "snapshot|{}\nowner|{}\ndesignated_receipts|{}\ndesignated_results|{}\ndesignated_versions|{}\ndesignated_consumption|{}\ndesignated_trace|{}",
            self.payload.shared_snapshot.canonical_store_projection(),
            self.payload.owner_execution.canonical_store_projection(),
            self.payload
                .designated
                .receipt_state
                .canonical_store_projection(),
            self.payload
                .designated
                .result_store
                .canonical_store_projection(),
            self.payload
                .designated
                .version_store
                .canonical_store_projection(),
            self.payload
                .designated
                .consumption_state
                .canonical_store_projection(),
            self.payload.designated.trace.canonical_store_projection(),
        )
    }

    /// Receipt-only projection of maintained relations together with the
    /// leases that determine their admissible frontier.
    pub(crate) fn canonical_relation_projection(&self) -> String {
        format!(
            "relations|{}\nleases|{}\nfallback_chain|{}",
            self.payload.shared_snapshot.canonical_relation_projection(),
            self.payload.lease_inventory.canonical_projection(),
            self.payload
                .finite_fallback_chains
                .values()
                .map(M8FiniteFallbackChain::canonical_projection)
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    /// Receipt-only checked program identity.  M9 membership/grant caches and
    /// patch lifecycle history are deliberately excluded from this M8 domain.
    pub(crate) fn canonical_configuration_projection(&self) -> String {
        format!(
            "program|{}\nsnapshot_config|{}\nfallback_chain|{}",
            self.admission_provenance.program_identity().stable_key(),
            self.payload
                .shared_snapshot
                .canonical_configuration_projection(),
            self.payload
                .finite_fallback_chains
                .values()
                .map(M8FiniteFallbackChain::canonical_projection)
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    pub(crate) fn canonical_semantic_projection(&self) -> String {
        format!(
            "cut_id|{}\nprogram|{}\nsnapshot|{}\nleases|{}\nfallback_chain|{}\npatch_rows|{}\ncut_receipt_causality|{}",
            self.cut_id,
            self.admission_provenance.program_identity().stable_key(),
            self.payload.shared_snapshot.canonical_projection(),
            self.payload.lease_inventory.canonical_projection(),
            self.payload
                .finite_fallback_chains
                .values()
                .map(M8FiniteFallbackChain::canonical_projection)
                .collect::<Vec<_>>()
                .join("\n"),
            self.payload.patch_lifecycle.rows().join(","),
            self.cut_receipt_causality.canonical_projection(),
        )
    }

    pub(crate) fn doctor_expired_lease_as_live(&self, lease_ref: &str) -> Option<Self> {
        let mut doctored = self.clone();
        if !doctored
            .payload
            .lease_inventory
            .doctor_expired_lease_as_live(lease_ref)
        {
            return None;
        }
        doctored.cut_id = format!("{}:doctor-expired-lease-live", self.cut_id);
        Some(doctored)
    }

    pub(crate) fn doctor_receive_without_send(&self) -> Self {
        let mut doctored = self.clone();
        doctored.cut_id = format!("{}:doctor-receive-without-send", self.cut_id);
        doctored.cut_receipt_causality = doctored.cut_receipt_causality.clone().without_reply();
        doctored
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8LocalRestoreDiagnosticKind {
    AdmissionProvenanceMismatch,
    InconsistentCut,
    StaleMembership,
    RevokedCapability,
    StaleWitness,
    ExpiredLease,
    ConsumedDeliveryRollback,
    ResultVersionRollback,
    OldRelationLineage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalRestoreDiagnostic {
    kind: M8LocalRestoreDiagnosticKind,
    source_ref: SourceRef,
}

impl M8LocalRestoreDiagnostic {
    pub const fn kind(&self) -> M8LocalRestoreDiagnosticKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalRestoreDiagnostics {
    entries: Vec<M8LocalRestoreDiagnostic>,
}

impl M8LocalRestoreDiagnostics {
    fn one(kind: M8LocalRestoreDiagnosticKind) -> Self {
        Self {
            entries: vec![M8LocalRestoreDiagnostic {
                kind,
                source_ref: SourceRef::new("<m8-local-cut>", 1, 1, 1, 1),
            }],
        }
    }

    pub fn primary(&self) -> &M8LocalRestoreDiagnostic {
        self.entries
            .first()
            .expect("M8 local restore diagnostics have a primary entry")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LiveFloor {
    authority_inventory: M8AuthorityState,
    entity_presence: M8EntityPresenceRegistry,
    lease_inventory: M8LeaseInventory,
    consumption_floor: M8ConsumptionState,
    version_floor: M8ResultVersionStore,
    relation_floor: BTreeMap<String, M8SemanticRelation>,
    stale_memberships: BTreeSet<String>,
    revoked_capabilities: BTreeSet<String>,
    stale_witnesses: BTreeSet<String>,
    expired_leases: BTreeSet<String>,
}

impl M8LiveFloor {
    pub fn same_current(cut: &M8LocalCut) -> Self {
        Self {
            authority_inventory: cut.authority_inventory().clone(),
            entity_presence: cut
                .payload
                .shared_snapshot
                .entity_presence_registry()
                .clone(),
            lease_inventory: cut.lease_inventory().clone(),
            consumption_floor: cut.designated_consumption_state().clone(),
            version_floor: cut.designated_version_store().clone(),
            relation_floor: cut.payload.shared_snapshot.relations.clone(),
            stale_memberships: BTreeSet::new(),
            revoked_capabilities: BTreeSet::new(),
            stale_witnesses: BTreeSet::new(),
            expired_leases: BTreeSet::new(),
        }
    }

    pub fn from_runtime(runtime: &M8LocalRuntime) -> Self {
        Self {
            authority_inventory: runtime.shared_snapshot.authority_state().clone(),
            entity_presence: runtime.shared_snapshot.entity_presence_registry().clone(),
            lease_inventory: runtime.lease_inventory.clone(),
            consumption_floor: runtime.designated.consumption_state.clone(),
            version_floor: runtime.designated.version_store.clone(),
            relation_floor: runtime.shared_snapshot.relations.clone(),
            stale_memberships: BTreeSet::new(),
            revoked_capabilities: BTreeSet::new(),
            stale_witnesses: BTreeSet::new(),
            expired_leases: BTreeSet::new(),
        }
    }

    pub fn with_stale_membership(mut self, reference: impl Into<String>) -> Self {
        self.stale_memberships.insert(reference.into());
        self
    }

    pub fn with_revoked_capability(mut self, reference: impl Into<String>) -> Self {
        self.revoked_capabilities.insert(reference.into());
        self
    }

    pub fn with_stale_witness(mut self, reference: impl Into<String>) -> Self {
        self.stale_witnesses.insert(reference.into());
        self
    }

    pub fn with_expired_lease(mut self, reference: impl Into<String>) -> Self {
        self.expired_leases.insert(reference.into());
        self
    }

    pub fn with_result_version_floor(
        mut self,
        value_name: impl Into<String>,
        result_version: ResultVersion,
    ) -> Self {
        self.version_floor.set_floor(value_name, result_version);
        self
    }
}

/// The single semantic M8 local session.  `shared_snapshot` contains the only
/// live authority inventory; the facade snapshots are deliberately empty
/// between operations and receive this value by move for their transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LocalRuntime {
    admitted: M8RuntimeInstance,
    shared_snapshot: M8SemanticSnapshot,
    owner: M8RuntimeExecution,
    relation: M8RelationProjectionRuntime,
    designated: M8DesignatedRuntime,
    lease_inventory: M8LeaseInventory,
    patch_lifecycle: M8LocalPatchLifecycle,
    trace: RefCell<M8LocalTrace>,
}

impl M8LocalRuntime {
    pub fn from_admitted(instance: M8RuntimeInstance, seed: M8LocalRuntimeSeed) -> Self {
        let M8LocalRuntimeSeed {
            owner_ints,
            authority_state,
            live_leases,
            designated_input_receipts,
        } = seed;
        let mut owner_seed = M8ExecutionSeed::new().with_authority_state(authority_state);
        for (key, value) in owner_ints {
            owner_seed = owner_seed.with_int(key, value);
        }
        let mut owner = instance.clone().into_execution(owner_seed);
        let mut relation = instance.clone().into_relation_projection(
            M8RelationProjectionSeed::new()
                .with_authority_state(M8AuthorityState::new())
                .with_live_leases(live_leases.clone()),
        );
        let designated = instance.clone().into_designated_values(
            M8DesignatedSeed::new()
                .with_authority_state(M8AuthorityState::new())
                .with_input_receipts(designated_input_receipts),
        );

        let mut shared_snapshot =
            std::mem::replace(&mut owner.snapshot, M8SemanticSnapshot::empty());
        let mut relation_snapshot =
            std::mem::replace(&mut relation.semantic_snapshot, M8SemanticSnapshot::empty());
        shared_snapshot.absorb_relations_from(&mut relation_snapshot);

        Self {
            admitted: instance,
            shared_snapshot,
            owner,
            relation,
            designated,
            lease_inventory: live_leases,
            patch_lifecycle: M8LocalPatchLifecycle::default(),
            trace: RefCell::new(M8LocalTrace::default()),
        }
    }

    pub fn enqueue_owner(
        &mut self,
        request: M8OwnerRequest,
    ) -> Result<M8Occurrence, M8EnqueueDiagnostics> {
        let trace_len = self.owner.trace().entries().len();
        let outcome = self.with_owner_snapshot(|owner| owner.try_enqueue(request));
        self.append_owner_trace_since(trace_len);
        outcome
    }

    pub fn serve_next_owner(
        &mut self,
        owner_locus: &str,
    ) -> Result<M8ServeOutcome, M8ServeDiagnostics> {
        let trace_len = self.owner.trace().entries().len();
        let outcome = self.with_owner_snapshot(|owner| owner.serve_next_owner(owner_locus));
        self.append_owner_trace_since(trace_len);
        outcome
    }

    pub fn invalidate_primary(
        &mut self,
        relation: &str,
        authority: M8RelationAuthorityUse,
        invalidation: M8BindingInvalidation,
    ) -> Result<crate::m8_runtime_relation_projection::M8RelationTransition, M8RelationDiagnostics>
    {
        let trace_len = self.relation.trace().entries().len();
        let trace_authority = authority.clone();
        let outcome = self.with_relation_snapshot(|runtime| {
            runtime.invalidate_primary(relation, authority, invalidation)
        });
        self.append_relation_trace_since(trace_len);
        if outcome.is_ok() {
            self.lease_inventory = self.relation.live_lease_inventory();
        }
        if let Err(diagnostics) = &outcome {
            self.trace.borrow_mut().append_fact(
                M8LocalTraceKind::RelationTransitionRejected,
                diagnostics.primary().source_ref().clone(),
                None,
                None,
                false,
                M8LocalAuthorityRefs::from_relation(&trace_authority),
                Some(M8LocalFailure::RelationTransition(
                    diagnostics.primary().kind(),
                )),
                None,
            );
        }
        outcome
    }

    pub fn reacquire_primary(
        &mut self,
        relation: &str,
        authority: M8RelationAuthorityUse,
        reacquire: M8RelationReacquire,
    ) -> Result<crate::m8_runtime_relation_projection::M8RelationTransition, M8RelationDiagnostics>
    {
        let trace_len = self.relation.trace().entries().len();
        let trace_authority = authority.clone();
        let fresh_witness_ref = reacquire.fresh_witness_ref().map(ToOwned::to_owned);
        let outcome = self.with_relation_snapshot(|runtime| {
            runtime.reacquire_primary(relation, authority, reacquire)
        });
        self.append_relation_trace_since(trace_len);
        if let Err(diagnostics) = &outcome {
            self.trace.borrow_mut().append_fact(
                M8LocalTraceKind::RelationTransitionRejected,
                diagnostics.primary().source_ref().clone(),
                None,
                None,
                false,
                M8LocalAuthorityRefs::from_relation(&trace_authority),
                Some(M8LocalFailure::RelationTransition(
                    diagnostics.primary().kind(),
                )),
                fresh_witness_ref,
            );
        }
        outcome
    }

    pub fn advance_anchor_to_frozen(
        &mut self,
        relation: &str,
        prior_transition: &crate::m8_runtime_relation_projection::M8RelationTransition,
    ) -> Result<crate::m8_runtime_relation_projection::M8RelationTransition, M8RelationDiagnostics>
    {
        let trace_len = self.relation.trace().entries().len();
        let outcome = self.with_relation_snapshot(|runtime| {
            runtime.advance_anchor_to_frozen(relation, prior_transition)
        });
        self.append_relation_trace_since(trace_len);
        if outcome.is_ok() {
            self.lease_inventory = self.relation.live_lease_inventory();
        }
        if let Err(diagnostics) = &outcome {
            self.trace.borrow_mut().append_fact(
                M8LocalTraceKind::RelationTransitionRejected,
                diagnostics.primary().source_ref().clone(),
                None,
                None,
                false,
                M8LocalAuthorityRefs::default(),
                Some(M8LocalFailure::RelationTransition(
                    diagnostics.primary().kind(),
                )),
                None,
            );
        }
        outcome
    }

    pub fn request_selected_option_write(
        &mut self,
        relation: &str,
    ) -> Result<(), M8RelationDiagnostics> {
        self.with_relation_snapshot(|runtime| runtime.request_selected_option_write(relation))
    }

    pub(crate) fn install_finite_fallback_chain(
        &mut self,
        chain: M8FiniteFallbackChain,
    ) -> Result<(), M8RelationDiagnostics> {
        let inventory =
            self.with_relation_snapshot(|runtime| runtime.install_finite_fallback_chain(chain))?;
        self.lease_inventory = inventory.clone();
        self.relation.replace_live_leases(inventory);
        Ok(())
    }

    pub fn note_primary_available_same_lineage(
        &mut self,
        relation: &str,
        anchor: &str,
    ) -> Result<crate::m8_runtime_relation_projection::M8RelationTransition, M8RelationDiagnostics>
    {
        let trace_len = self.relation.trace().entries().len();
        let outcome = self.with_relation_snapshot(|runtime| {
            runtime.note_primary_available_same_lineage(relation, anchor)
        });
        self.append_relation_trace_since(trace_len);
        if let Err(diagnostics) = &outcome {
            self.trace.borrow_mut().append_fact(
                M8LocalTraceKind::RelationTransitionRejected,
                diagnostics.primary().source_ref().clone(),
                None,
                None,
                false,
                M8LocalAuthorityRefs::default(),
                Some(M8LocalFailure::RelationTransition(
                    diagnostics.primary().kind(),
                )),
                None,
            );
        }
        outcome
    }

    pub fn project_relation(
        &mut self,
        relation: &str,
        context: M8PresentationContext,
    ) -> Result<M8RelationProjection, M8ProjectionDiagnostics> {
        self.with_relation_snapshot(|runtime| runtime.project_relation(relation, context))
    }

    /// Refresh only the sealed M9-derived authority inventory while retaining
    /// this M8 session's mutable execution state and identity.
    pub(crate) fn refresh_m9_authority_state(&mut self, authority_state: M8AuthorityState) {
        self.shared_snapshot
            .replace_authority_state(authority_state.clone());
        // The unified local runtime swaps the shared snapshot through the
        // owner, relation, and designated engines for each operation.  Keep
        // their parked snapshots on the same sealed M9 authority inventory;
        // otherwise an owner occurrence immediately after a bridge refresh
        // could swap a stale inventory back into the shared relation state.
        self.owner
            .snapshot
            .replace_authority_state(authority_state.clone());
        self.relation
            .semantic_snapshot
            .replace_authority_state(authority_state.clone());
        self.designated
            .semantic_snapshot
            .replace_authority_state(authority_state);
    }

    /// Synchronize entity presence only from an M9-sealed bridge. The facade
    /// snapshots remain empty between operations; this shared snapshot is the
    /// sole semantic holder and the M9 bridge has already revalidated the
    /// underlying membership lineage.
    pub(crate) fn synchronize_entity_presence(
        &mut self,
        bridge: M9M8EntityPresenceBridge,
    ) -> Result<M8EntityPresenceSynchronization, String> {
        let namespace = bridge.namespace().to_string();
        let identity = bridge.identity().to_string();
        let source_ref = bridge.source_ref().clone();
        let before_status = self
            .shared_snapshot
            .entity_presence(&namespace, &identity)
            .map(|record| record.status().as_str().to_string())
            .unwrap_or_else(|| "absent".to_string());
        match bridge.status() {
            M9M8EntityPresenceStatus::Live => {
                if before_status == "retired" {
                    return Err(
                        "M8 entity presence bridge attempted to resurrect a retired target"
                            .to_string(),
                    );
                }
                self.shared_snapshot.admit_entity_presence(
                    &namespace,
                    &identity,
                    format!("sealed-m9-m8-presence|{}", bridge.sealed_membership_ref()),
                );
            }
            M9M8EntityPresenceStatus::Retired => {
                if before_status != "live"
                    || !self
                        .shared_snapshot
                        .retire_entity_presence(&namespace, &identity)
                {
                    return Err(
                        "M8 entity presence bridge could not retire an existing live target"
                            .to_string(),
                    );
                }
            }
        }
        let occurrence_id = format!(
            "m8-entity-presence-occurrence-{:020}",
            self.trace.borrow().len()
        );
        self.trace.borrow_mut().append(
            M8LocalTraceKind::EntityPresenceSynchronized,
            source_ref.clone(),
            Some(occurrence_id.clone()),
            None,
            false,
        );
        let occurrence_trace_id = self
            .trace
            .borrow()
            .latest_observation(M8LocalTraceKind::EntityPresenceSynchronized)
            .expect("M8 presence synchronization appends an occurrence trace")
            .node_id;
        self.trace.borrow_mut().append(
            M8LocalTraceKind::EntityPresenceControlApplied,
            source_ref.clone(),
            None,
            None,
            false,
        );
        let control_trace_id = self
            .trace
            .borrow()
            .latest_observation(M8LocalTraceKind::EntityPresenceControlApplied)
            .expect("M8 presence synchronization appends a control trace")
            .node_id;
        Ok(M8EntityPresenceSynchronization {
            before_status,
            after_status: bridge.status().as_str().to_string(),
            source_ref,
            occurrence_id,
            occurrence_trace_id,
            control_id: control_trace_id.clone(),
            control_trace_id,
            sealed_membership_ref: bridge.sealed_membership_ref().to_string(),
            m9_snapshot_ref: bridge.m9_snapshot_ref().to_string(),
            m8_authority_use_ref: bridge.m8_authority_use_ref().to_string(),
        })
    }

    /// Crate-private patch seam for a checked, newly declared finite-v0 Int
    /// field.  It can only insert the default value once and always leaves an
    /// occurrence-bearing semantic trace; ordinary M10 requests cannot call
    /// this directly.
    pub(crate) fn initialize_patch_declared_int(
        &mut self,
        key: M8StateKey,
        source_ref: SourceRef,
    ) -> bool {
        let initialized = self.shared_snapshot.initialize_int_default(key);
        if initialized {
            let occurrence_id = format!("m8-patch-init-{:020}", self.trace.borrow().len());
            self.trace.borrow_mut().append(
                M8LocalTraceKind::PatchStateInitialized,
                source_ref,
                Some(occurrence_id),
                None,
                false,
            );
        }
        initialized
    }

    pub fn evaluate_designated(
        &mut self,
        request: M8DesignatedEvaluationRequest,
    ) -> Result<
        crate::m8_runtime_designated_value::M8PublishedDesignatedValue,
        M8DesignatedDiagnostics,
    > {
        let trace_len = self.designated.trace().kinds().len();
        let outcome = self.with_designated_snapshot(|runtime| runtime.evaluate_designated(request));
        self.append_designated_trace_since(trace_len);
        outcome
    }

    pub fn consume_published_value(
        &mut self,
        request: M8ConsumeRequest,
    ) -> Result<M8ConsumedDesignatedValue, M8DesignatedDiagnostics> {
        let trace_len = self.designated.trace().kinds().len();
        let outcome =
            self.with_designated_snapshot(|runtime| runtime.consume_published_value(request));
        self.append_designated_trace_since(trace_len);
        outcome
    }

    pub fn attach_presentation_interpolation(
        &mut self,
        value_name: &str,
        result_version: ResultVersion,
        interpolation: M8PresentationInterpolation,
    ) -> Result<(), M8DesignatedDiagnostics> {
        self.with_designated_snapshot(|runtime| {
            runtime.attach_presentation_interpolation(value_name, result_version, interpolation)
        })
    }

    pub fn save_local_cut(&self, cut_id: impl Into<String>) -> M8LocalCut {
        let cut_id = cut_id.into();
        let save_sequence = self.trace.borrow().len();
        let cut_receipt_causality = M8CutReceiptCausality::for_cut(
            &cut_id,
            save_sequence,
            self.admitted.program_identity().root_source_ref().clone(),
        );
        self.trace.borrow_mut().append(
            M8LocalTraceKind::LocalCutSaved,
            self.admitted.program_identity().root_source_ref().clone(),
            None,
            None,
            false,
        );
        M8LocalCut {
            cut_id,
            admission_provenance: M8LocalAdmissionProvenance::from_instance(&self.admitted),
            admitted: self.admitted.clone(),
            payload: self.save_relevant_payload(),
            cut_receipt_causality,
            trace_prefix: self.trace.borrow().clone(),
        }
    }

    pub fn try_restore_local_cut(
        &mut self,
        cut: &M8LocalCut,
        floor: &M8LiveFloor,
    ) -> Result<(), M8LocalRestoreDiagnostics> {
        let provenance = M8LocalAdmissionProvenance::from_instance(&self.admitted);
        let failure = if cut.admission_provenance != provenance {
            Some(M8LocalRestoreDiagnosticKind::AdmissionProvenanceMismatch)
        } else if !cut.cut_receipt_causality.is_consistent() {
            Some(M8LocalRestoreDiagnosticKind::InconsistentCut)
        } else if let Some(reference) = floor.stale_memberships.iter().next() {
            let _ = reference;
            Some(M8LocalRestoreDiagnosticKind::StaleMembership)
        } else if let Some(reference) = floor.revoked_capabilities.iter().next() {
            let _ = reference;
            Some(M8LocalRestoreDiagnosticKind::RevokedCapability)
        } else if let Some(reference) = floor.stale_witnesses.iter().next() {
            let _ = reference;
            Some(M8LocalRestoreDiagnosticKind::StaleWitness)
        } else if floor.expired_leases.iter().next().is_some() {
            Some(M8LocalRestoreDiagnosticKind::ExpiredLease)
        } else if cut.authority_inventory() != &floor.authority_inventory
            || cut.payload.shared_snapshot.entity_presence_registry() != &floor.entity_presence
        {
            Some(M8LocalRestoreDiagnosticKind::StaleMembership)
        } else if !cut
            .payload
            .lease_inventory
            .records
            .eq(&floor.lease_inventory.records)
        {
            Some(M8LocalRestoreDiagnosticKind::ExpiredLease)
        } else if !cut
            .designated_consumption_state()
            .covers(&floor.consumption_floor)
        {
            Some(M8LocalRestoreDiagnosticKind::ConsumedDeliveryRollback)
        } else if !cut
            .designated_version_store()
            .satisfies_floor(&floor.version_floor)
        {
            Some(M8LocalRestoreDiagnosticKind::ResultVersionRollback)
        } else if relation_option_floor_regresses_same_lineage(
            &cut.payload.shared_snapshot.relations,
            &floor.relation_floor,
        ) || cut.payload.shared_snapshot.relations != floor.relation_floor
        {
            // Same-lineage finite fallback floors are monotone; only a fresh
            // M9 reacquire starts a new lineage at option zero.
            Some(M8LocalRestoreDiagnosticKind::OldRelationLineage)
        } else {
            None
        };

        if let Some(kind) = failure {
            self.trace.borrow_mut().append(
                M8LocalTraceKind::RestoreRejected,
                self.admitted.program_identity().root_source_ref().clone(),
                None,
                Some(kind),
                true,
            );
            return Err(M8LocalRestoreDiagnostics::one(kind));
        }

        self.restore_payload(&cut.payload);
        *self.trace.borrow_mut() = cut.trace_prefix.clone();
        Ok(())
    }

    pub fn save_relevant_payload(&self) -> M8LocalSavePayload {
        M8LocalSavePayload {
            shared_snapshot: self.shared_snapshot.clone(),
            owner_execution: self.owner.clone(),
            relation_trace: self.relation.trace().clone(),
            finite_fallback_chains: self.relation.finite_fallback_chains(),
            designated: M8LocalDesignatedSaveState {
                receipt_state: self.designated.receipt_state.clone(),
                result_store: self.designated.result_store.clone(),
                version_store: self.designated.version_store.clone(),
                consumption_state: self.designated.consumption_state.clone(),
                trace: self.designated.trace.clone(),
                next_trace_node: self.designated.next_trace_node,
                next_occurrence: self.designated.next_occurrence,
            },
            lease_inventory: self.lease_inventory.clone(),
            patch_lifecycle: self.patch_lifecycle.clone(),
        }
    }

    /// Exact mutable local store, including pending owner FIFO/counters and
    /// designated receipt/version/consumption state needed by no-replay.
    pub(crate) fn canonical_store_projection(&self) -> String {
        format!(
            "snapshot|{}\nowner|{}\ndesignated_receipts|{}\ndesignated_results|{}\ndesignated_versions|{}\ndesignated_consumption|{}\ndesignated_trace|{}",
            self.shared_snapshot.canonical_store_projection(),
            self.owner.canonical_store_projection(),
            self.designated.receipt_state.canonical_store_projection(),
            self.designated.result_store.canonical_store_projection(),
            self.designated.version_store.canonical_store_projection(),
            self.designated
                .consumption_state
                .canonical_store_projection(),
            self.designated.trace.canonical_store_projection(),
        )
    }

    /// Exact maintained relation and lease frontier state.
    pub(crate) fn canonical_relation_projection(&self) -> String {
        format!(
            "relations|{}\nleases|{}\nfallback_chain|{}",
            self.shared_snapshot.canonical_relation_projection(),
            self.lease_inventory.canonical_projection(),
            self.relation.canonical_fallback_configuration_projection(),
        )
    }

    /// Checked program identity only.  Membership epoch, authority cache, and
    /// patch lifecycle history never feed M8 configuration receipts.
    pub(crate) fn canonical_configuration_projection(&self) -> String {
        format!(
            "program|{}\nsnapshot_config|{}\nfallback_chain|{}",
            self.admitted.program_identity().stable_key(),
            self.shared_snapshot.canonical_configuration_projection(),
            self.relation.canonical_fallback_configuration_projection(),
        )
    }

    pub(crate) fn canonical_semantic_projection(&self) -> String {
        format!(
            "program|{}\nsnapshot|{}\nleases|{}\npatch_rows|{}",
            self.admitted.program_identity().stable_key(),
            self.shared_snapshot.canonical_projection(),
            self.lease_inventory.canonical_projection(),
            self.patch_lifecycle.rows().join(","),
        )
    }

    pub fn trace(&self) -> M8LocalTrace {
        self.trace.borrow().clone()
    }

    pub fn pending_owner_fifo(&self, owner_locus: &str) -> Vec<String> {
        self.owner
            .owner_queue(owner_locus)
            .occurrence_ids()
            .to_vec()
    }

    pub fn owner_state(&self) -> &M8SemanticSnapshot {
        &self.shared_snapshot
    }

    pub fn relation_state(&self, relation: &str) -> Option<&M8SemanticRelation> {
        self.shared_snapshot.relations.get(relation)
    }

    pub(crate) fn finite_fallback_selection(
        &self,
        relation: &str,
    ) -> Option<M8FiniteFallbackSelection> {
        self.relation
            .finite_fallback_selection(self.shared_snapshot.relation(relation)?)
    }

    pub(crate) fn has_finite_fallback_chain(&self, relation: &str) -> bool {
        self.relation.has_finite_fallback_chain(relation)
    }

    pub(crate) fn contains_live_relation_lease(&self, reference: &str) -> bool {
        self.lease_inventory.contains_live(reference)
    }

    pub fn designated_result_store(&self) -> &M8DesignatedResultStore {
        &self.designated.result_store
    }

    pub(crate) fn active_program_identity(&self) -> &CheckedProgramIdentity {
        self.admitted.program_identity()
    }

    pub(crate) fn active_admission(&self) -> &crate::m8_runtime_admission::M8RuntimeAdmission {
        self.admitted.admission()
    }

    pub(crate) fn has_pending_owner_requests(&self) -> bool {
        self.owner.has_pending_requests()
    }

    pub(crate) fn install_admitted_patch(
        &mut self,
        instance: M8RuntimeInstance,
        input_receipts: Option<M8InputReceiptSet>,
        patch_id: &str,
    ) {
        self.with_owner_snapshot(|owner| owner.replace_admitted_plans(&instance));
        self.with_relation_snapshot(|relation| relation.replace_admitted_plans(&instance));
        self.with_designated_snapshot(|designated| {
            designated.replace_admitted_plans(&instance, input_receipts)
        });
        self.admitted = instance;
        self.record_patch_activation(patch_id);
    }

    pub(crate) fn install_admitted_from_cut(&mut self, cut: &M8LocalCut) {
        self.with_owner_snapshot(|owner| owner.replace_admitted_plans(&cut.admitted));
        self.with_relation_snapshot(|relation| relation.replace_admitted_plans(&cut.admitted));
        self.with_designated_snapshot(|designated| {
            designated.replace_admitted_plans(&cut.admitted, None)
        });
        self.admitted = cut.admitted.clone();
    }

    /// Records the sole state change made by an accepted M8 patch: its
    /// activation is retained in this session's saved lifecycle state.  Patch
    /// checking and diagnostics live at the patch boundary; this method does
    /// not create a second semantic snapshot or reconstruct source input.
    pub(crate) fn record_patch_activation(&mut self, patch_id: &str) {
        self.patch_lifecycle
            .rows
            .push(format!("activated:{patch_id}"));
    }

    pub(crate) fn semantic_payload_without_patch_lifecycle(&self) -> M8LocalSemanticPayload {
        M8LocalSemanticPayload {
            shared_snapshot: self.shared_snapshot.clone(),
            owner_execution: self.owner.clone(),
            relation_trace: self.relation.trace().clone(),
            designated: M8LocalDesignatedSaveState {
                receipt_state: self.designated.receipt_state.clone(),
                result_store: self.designated.result_store.clone(),
                version_store: self.designated.version_store.clone(),
                consumption_state: self.designated.consumption_state.clone(),
                trace: self.designated.trace.clone(),
                next_trace_node: self.designated.next_trace_node,
                next_occurrence: self.designated.next_occurrence,
            },
            lease_inventory: self.lease_inventory.clone(),
        }
    }

    pub(crate) fn patch_lifecycle_rows(&self) -> Vec<String> {
        self.patch_lifecycle.rows.clone()
    }

    pub(crate) fn last_activated_patch(&self) -> Option<&str> {
        self.patch_lifecycle
            .rows
            .iter()
            .rev()
            .find_map(|row| row.strip_prefix("activated:"))
    }

    fn with_owner_snapshot<T>(
        &mut self,
        operation: impl FnOnce(&mut M8RuntimeExecution) -> T,
    ) -> T {
        std::mem::swap(&mut self.shared_snapshot, &mut self.owner.snapshot);
        let result = operation(&mut self.owner);
        std::mem::swap(&mut self.shared_snapshot, &mut self.owner.snapshot);
        result
    }

    fn with_relation_snapshot<T>(
        &mut self,
        operation: impl FnOnce(&mut M8RelationProjectionRuntime) -> T,
    ) -> T {
        std::mem::swap(
            &mut self.shared_snapshot,
            &mut self.relation.semantic_snapshot,
        );
        let result = operation(&mut self.relation);
        std::mem::swap(
            &mut self.shared_snapshot,
            &mut self.relation.semantic_snapshot,
        );
        result
    }

    fn with_designated_snapshot<T>(
        &mut self,
        operation: impl FnOnce(&mut M8DesignatedRuntime) -> T,
    ) -> T {
        std::mem::swap(
            &mut self.shared_snapshot,
            &mut self.designated.semantic_snapshot,
        );
        let result = operation(&mut self.designated);
        std::mem::swap(
            &mut self.shared_snapshot,
            &mut self.designated.semantic_snapshot,
        );
        result
    }

    fn append_owner_trace_since(&mut self, start: usize) {
        let entries = self.owner.trace().entries()[start..].to_vec();
        for entry in entries {
            let kind = match entry.kind() {
                M8QueueTraceKind::Enqueued => Some(M8LocalTraceKind::OwnerEnqueued),
                M8QueueTraceKind::AuthorityValidated => {
                    Some(M8LocalTraceKind::OwnerAuthorityValidated)
                }
                M8QueueTraceKind::OwnerRead => Some(M8LocalTraceKind::OwnerRead),
                M8QueueTraceKind::OwnerWrite => Some(M8LocalTraceKind::OwnerWrite),
                M8QueueTraceKind::TypedEnqueueRejected => {
                    Some(M8LocalTraceKind::OwnerEnqueueRejected)
                }
                M8QueueTraceKind::DeclaredFailure => Some(M8LocalTraceKind::OwnerServeRejected),
            };
            if let Some(kind) = kind {
                let failure = entry
                    .enqueue_diagnostic_kind()
                    .map(M8LocalFailure::OwnerEnqueue)
                    .or_else(|| {
                        entry.failure().map(|failure| {
                            M8LocalFailure::OwnerServe(M8ServeDiagnosticKind::DeclaredFailure(
                                failure,
                            ))
                        })
                    });
                self.trace.borrow_mut().append_fact(
                    kind,
                    entry.source_ref().clone(),
                    entry.request_occurrence_id().map(ToOwned::to_owned),
                    None,
                    false,
                    M8LocalAuthorityRefs::from_owner(entry.authority()),
                    failure,
                    None,
                );
            }
        }
    }

    fn append_relation_trace_since(&mut self, start: usize) {
        let entries = self.relation.trace().entries()[start..].to_vec();
        for entry in entries {
            let kind = match entry.kind() {
                M8RelationTraceKind::SemanticPrimaryInvalidated => {
                    M8LocalTraceKind::RelationPrimaryInvalidated
                }
                M8RelationTraceKind::RelationOptionAdvanced => {
                    M8LocalTraceKind::RelationOptionAdvanced
                }
                M8RelationTraceKind::FallbackOptionFrozen => {
                    M8LocalTraceKind::RelationFallbackFrozen
                }
                M8RelationTraceKind::SameLineagePrimaryReturnIgnored => {
                    M8LocalTraceKind::RelationPrimaryReturnIgnored
                }
                M8RelationTraceKind::FreshRelationLineageReacquired => {
                    M8LocalTraceKind::RelationFreshLineageReacquired
                }
            };
            self.trace
                .borrow_mut()
                .append(kind, entry.source_ref().clone(), None, None, false);
        }
    }

    fn append_designated_trace_since(&mut self, start: usize) {
        let entries = self.designated.trace().observations();
        for entry in entries.into_iter().skip(start) {
            let kind = match entry.kind {
                M8DesignatedTraceKind::AuthorityValidated => {
                    Some(M8LocalTraceKind::DesignatedAuthorityValidated)
                }
                M8DesignatedTraceKind::InputReceiptValidated => {
                    Some(M8LocalTraceKind::DesignatedInputReceiptValidated)
                }
                M8DesignatedTraceKind::ValuePublished => {
                    Some(M8LocalTraceKind::DesignatedValuePublished)
                }
                M8DesignatedTraceKind::ConsumerAuthorityValidated => {
                    Some(M8LocalTraceKind::DesignatedConsumerAuthorityValidated)
                }
                M8DesignatedTraceKind::ValueConsumed => {
                    Some(M8LocalTraceKind::DesignatedValueConsumed)
                }
                M8DesignatedTraceKind::EvaluationIdempotent => None,
                M8DesignatedTraceKind::EvaluationFailed => {
                    Some(M8LocalTraceKind::DesignatedEvaluationRejected)
                }
                M8DesignatedTraceKind::ConsumptionRejected => {
                    Some(M8LocalTraceKind::DesignatedConsumptionRejected)
                }
            };
            if let Some(kind) = kind {
                let failure = match (entry.kind, entry.diagnostic_kind) {
                    (M8DesignatedTraceKind::EvaluationFailed, Some(diagnostic)) => {
                        Some(M8LocalFailure::DesignatedEvaluation(diagnostic))
                    }
                    (M8DesignatedTraceKind::ConsumptionRejected, Some(diagnostic)) => {
                        Some(M8LocalFailure::DesignatedConsumption(diagnostic))
                    }
                    _ => None,
                };
                self.trace.borrow_mut().append_fact(
                    kind,
                    entry.source_ref,
                    entry.occurrence_id,
                    None,
                    false,
                    M8LocalAuthorityRefs::from_designated(entry.authority.as_ref()),
                    failure,
                    None,
                );
            }
        }
    }

    fn restore_payload(&mut self, payload: &M8LocalSavePayload) {
        self.shared_snapshot = payload.shared_snapshot.clone();
        self.owner = payload.owner_execution.clone();
        self.relation.trace = payload.relation_trace.clone();
        self.relation
            .replace_finite_fallback_chains(payload.finite_fallback_chains.clone());
        self.designated.receipt_state = payload.designated.receipt_state.clone();
        self.designated.result_store = payload.designated.result_store.clone();
        self.designated.version_store = payload.designated.version_store.clone();
        self.designated.consumption_state = payload.designated.consumption_state.clone();
        self.designated.trace = payload.designated.trace.clone();
        self.designated.next_trace_node = payload.designated.next_trace_node;
        self.designated.next_occurrence = payload.designated.next_occurrence;
        self.relation
            .replace_live_leases(payload.lease_inventory.clone());
        self.lease_inventory = payload.lease_inventory.clone();
        self.patch_lifecycle = payload.patch_lifecycle.clone();
    }
}

fn relation_option_floor_regresses_same_lineage(
    saved: &BTreeMap<String, M8SemanticRelation>,
    current: &BTreeMap<String, M8SemanticRelation>,
) -> bool {
    saved.iter().any(|(relation, saved_state)| {
        current.get(relation).is_some_and(|current_state| {
            saved_state.binding_epoch() == current_state.binding_epoch()
                && saved_state.selected_option_index() < current_state.selected_option_index()
        })
    })
}
