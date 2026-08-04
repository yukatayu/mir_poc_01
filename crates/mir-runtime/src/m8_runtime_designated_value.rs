//! Bounded M8 designated-value execution over admitted M7 Core.
//!
//! Designated evaluation consumes only already-admitted, source-bound input
//! receipts.  It never reads an owner store, creates communication, or issues
//! authority.  Consumer delivery is a one-shot use of the retained value.

use std::collections::{BTreeMap, BTreeSet};

use mir_semantics::{
    evaluation_materialization::{EvaluationPolicy, InputFrontier, ObservationPolicy, PolicyStamp},
    shared_model::{ResultFrontier, ResultVersion, SourceRef},
    surface_v0_pipeline::{CheckedBinaryOperator, CheckedExpressionTree, TypedStateRead},
};

use crate::{
    m8_runtime_admission::{
        EvidenceRedaction, EvidenceSecurityLabel, M8DesignatedExecutionPlan, M8RuntimeInstance,
        M8SecurityClass,
    },
    m8_runtime_authority::{
        M8AuthorityState, M8DesignatedConsumptionAuthorityLookup,
        M8DesignatedEvaluationAuthorityLookup,
    },
    m8_runtime_owner_queue::{M8SemanticSnapshot, M8StateKey},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8InputReceipt {
    reference: String,
    live: bool,
    state_key: Option<M8StateKey>,
    source_owner_locus: Option<String>,
    evaluator: Option<String>,
    input_frontier: Option<String>,
    source_ref: Option<SourceRef>,
    label: Option<EvidenceSecurityLabel>,
    int_value: Option<i64>,
}

impl M8InputReceipt {
    pub fn live(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            live: true,
            state_key: None,
            source_owner_locus: None,
            evaluator: None,
            input_frontier: None,
            source_ref: None,
            label: None,
            int_value: None,
        }
    }

    pub fn stale(reference: impl Into<String>) -> Self {
        Self {
            live: false,
            ..Self::live(reference)
        }
    }

    pub fn for_state_read(mut self, state_key: M8StateKey) -> Self {
        self.state_key = Some(state_key);
        self
    }

    pub fn with_source_owner_locus(mut self, source_owner_locus: impl Into<String>) -> Self {
        self.source_owner_locus = Some(source_owner_locus.into());
        self
    }

    pub fn with_evaluator(mut self, evaluator: impl Into<String>) -> Self {
        self.evaluator = Some(evaluator.into());
        self
    }

    pub fn with_input_frontier(mut self, input_frontier: impl Into<String>) -> Self {
        self.input_frontier = Some(input_frontier.into());
        self
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = Some(source_ref);
        self
    }

    pub fn with_label(mut self, label: EvidenceSecurityLabel) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_int_value(mut self, int_value: i64) -> Self {
        self.int_value = Some(int_value);
        self
    }

    pub fn source_owner_locus(&self) -> &str {
        self.source_owner_locus.as_deref().unwrap_or("")
    }

    pub fn evaluator(&self) -> &str {
        self.evaluator.as_deref().unwrap_or("")
    }

    pub fn input_frontier(&self) -> &str {
        self.input_frontier.as_deref().unwrap_or("")
    }

    pub fn source_ref(&self) -> &SourceRef {
        self.source_ref
            .as_ref()
            .expect("well-formed M8 input receipts carry source provenance")
    }

    pub fn label(&self) -> &EvidenceSecurityLabel {
        self.label
            .as_ref()
            .expect("well-formed M8 input receipts carry a security label")
    }

    pub const fn int_value(&self) -> Option<i64> {
        self.int_value
    }

    pub fn generated_dependency_path(&self) -> M8GeneratedDependencyPath {
        M8GeneratedDependencyPath {
            source_owner_locus: self.source_owner_locus().to_string(),
            evaluator: self.evaluator().to_string(),
            input_frontier: self.input_frontier().to_string(),
            source_ref: self
                .source_ref
                .clone()
                .unwrap_or_else(|| SourceRef::new("<m8-input-receipt>", 1, 1, 1, 1)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8GeneratedDependencyPath {
    source_owner_locus: String,
    evaluator: String,
    input_frontier: String,
    source_ref: SourceRef,
}

impl M8GeneratedDependencyPath {
    pub fn source_owner_locus(&self) -> &str {
        &self.source_owner_locus
    }

    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    pub fn input_frontier(&self) -> &str {
        &self.input_frontier
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8InputReceiptSet {
    receipts: BTreeMap<String, M8InputReceipt>,
}

impl M8InputReceiptSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_receipt(mut self, receipt: M8InputReceipt) -> Self {
        self.receipts.insert(receipt.reference.clone(), receipt);
        self
    }

    fn for_state_key(&self, state_key: &M8StateKey) -> Option<&M8InputReceipt> {
        self.receipts
            .values()
            .find(|receipt| receipt.state_key.as_ref() == Some(state_key))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedSeed {
    authority_state: M8AuthorityState,
    input_receipts: M8InputReceiptSet,
}

impl M8DesignatedSeed {
    pub fn new() -> Self {
        Self {
            authority_state: M8AuthorityState::new(),
            input_receipts: M8InputReceiptSet::new(),
        }
    }

    pub fn with_authority_state(mut self, authority_state: M8AuthorityState) -> Self {
        self.authority_state = authority_state;
        self
    }

    pub fn with_input_receipts(mut self, input_receipts: M8InputReceiptSet) -> Self {
        self.input_receipts = input_receipts;
        self
    }
}

impl Default for M8DesignatedSeed {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum M8DesignatedAuthoritySite {
    Evaluator(String),
    Consumer(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedAuthorityUse {
    site: M8DesignatedAuthoritySite,
    principal: Option<String>,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    witness_ref: Option<String>,
}

impl M8DesignatedAuthorityUse {
    pub fn for_evaluator(evaluator: impl Into<String>) -> Self {
        Self {
            site: M8DesignatedAuthoritySite::Evaluator(evaluator.into()),
            principal: None,
            membership_ref: None,
            capability_ref: None,
            witness_ref: None,
        }
    }

    pub fn for_consumer(consumer: impl Into<String>) -> Self {
        Self {
            site: M8DesignatedAuthoritySite::Consumer(consumer.into()),
            principal: None,
            membership_ref: None,
            capability_ref: None,
            witness_ref: None,
        }
    }

    pub fn with_principal(mut self, principal: impl Into<String>) -> Self {
        self.principal = Some(principal.into());
        self
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

    pub fn capability_ref(&self) -> Option<&str> {
        self.capability_ref.as_deref()
    }

    pub fn witness_ref(&self) -> Option<&str> {
        self.witness_ref.as_deref()
    }

    pub(crate) fn membership_ref(&self) -> Option<&str> {
        self.membership_ref.as_deref()
    }

    fn principal(&self) -> Option<&str> {
        self.principal.as_deref()
    }

    fn evaluator(&self) -> Option<&str> {
        match &self.site {
            M8DesignatedAuthoritySite::Evaluator(evaluator) => Some(evaluator),
            M8DesignatedAuthoritySite::Consumer(_) => None,
        }
    }

    fn consumer(&self) -> Option<&str> {
        match &self.site {
            M8DesignatedAuthoritySite::Evaluator(_) => None,
            M8DesignatedAuthoritySite::Consumer(consumer) => Some(consumer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedTick {
    id: String,
    input_frontier: Option<String>,
}

impl M8DesignatedTick {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            input_frontier: None,
        }
    }

    pub fn with_input_frontier(mut self, input_frontier: impl Into<String>) -> Self {
        self.input_frontier = Some(input_frontier.into());
        self
    }

    pub fn input_frontier(&self) -> &str {
        self.input_frontier.as_deref().unwrap_or("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedEvaluationRequest {
    value_name: String,
    tick: Option<M8DesignatedTick>,
    authority: Option<M8DesignatedAuthorityUse>,
}

impl M8DesignatedEvaluationRequest {
    pub fn for_value(value_name: impl Into<String>) -> Self {
        Self {
            value_name: value_name.into(),
            tick: None,
            authority: None,
        }
    }

    pub fn with_tick(mut self, tick: M8DesignatedTick) -> Self {
        self.tick = Some(tick);
        self
    }

    pub fn with_authority(mut self, authority: M8DesignatedAuthorityUse) -> Self {
        self.authority = Some(authority);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ConsumeRequest {
    value_name: String,
    consumer: Option<String>,
    delivery_id: Option<String>,
    authority: Option<M8DesignatedAuthorityUse>,
}

impl M8ConsumeRequest {
    pub fn for_value(value_name: impl Into<String>) -> Self {
        Self {
            value_name: value_name.into(),
            consumer: None,
            delivery_id: None,
            authority: None,
        }
    }

    pub fn with_consumer(mut self, consumer: impl Into<String>) -> Self {
        self.consumer = Some(consumer.into());
        self
    }

    pub fn with_delivery_id(mut self, delivery_id: impl Into<String>) -> Self {
        self.delivery_id = Some(delivery_id.into());
        self
    }

    pub fn with_authority(mut self, authority: M8DesignatedAuthorityUse) -> Self {
        self.authority = Some(authority);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8DesignatedDiagnosticKind {
    UnknownDesignatedValue,
    MissingInputReceipt,
    InputReceiptSourceMismatch,
    InputReceiptFrontierMismatch,
    InputReceiptSourceRefMismatch,
    InputReceiptLabelMissing,
    InputReceiptValueMissing,
    StaleInputReceipt,
    TickFrontierMismatch,
    MissingEvaluatorAuthority,
    MissingConsumerAuthority,
    MissingPublishedValue,
    AlreadyConsumed,
    ArithmeticFailure,
    OutputVisibilityWouldWeakenInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedDiagnostic {
    kind: M8DesignatedDiagnosticKind,
    source_ref: SourceRef,
}

impl M8DesignatedDiagnostic {
    pub const fn kind(&self) -> M8DesignatedDiagnosticKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedDiagnostics {
    entries: Vec<M8DesignatedDiagnostic>,
}

impl M8DesignatedDiagnostics {
    fn one(kind: M8DesignatedDiagnosticKind, source_ref: SourceRef) -> Self {
        Self {
            entries: vec![M8DesignatedDiagnostic { kind, source_ref }],
        }
    }

    pub fn primary(&self) -> &M8DesignatedDiagnostic {
        self.entries
            .first()
            .expect("M8 designated diagnostics have a primary entry")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8DesignatedTraceKind {
    InputReceiptValidated,
    AuthorityValidated,
    ValuePublished,
    EvaluationIdempotent,
    EvaluationFailed,
    ConsumerAuthorityValidated,
    ValueConsumed,
    ConsumptionRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M8DesignatedTraceEntry {
    kind: M8DesignatedTraceKind,
    node_id: String,
    node_index: u64,
    dependencies: BTreeSet<String>,
    diagnostic_kind: Option<M8DesignatedDiagnosticKind>,
    authority: Option<M8DesignatedAuthorityUse>,
    source_ref: SourceRef,
    occurrence_id: Option<String>,
    receipt_provenance_access: bool,
    receipt_value_access: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8DesignatedTraceObservation {
    pub(crate) kind: M8DesignatedTraceKind,
    pub(crate) source_ref: SourceRef,
    pub(crate) occurrence_id: Option<String>,
    pub(crate) diagnostic_kind: Option<M8DesignatedDiagnosticKind>,
    pub(crate) authority: Option<M8DesignatedAuthorityUse>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8PublishedValueObservation {
    pub(crate) value_name: String,
    pub(crate) occurrence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8DesignatedTrace {
    entries: Vec<M8DesignatedTraceEntry>,
}

impl M8DesignatedTrace {
    pub fn kinds(&self) -> Vec<M8DesignatedTraceKind> {
        self.entries.iter().map(|entry| entry.kind).collect()
    }

    pub(crate) fn observations(&self) -> Vec<M8DesignatedTraceObservation> {
        self.entries
            .iter()
            .map(|entry| M8DesignatedTraceObservation {
                kind: entry.kind,
                source_ref: entry.source_ref.clone(),
                occurrence_id: entry.occurrence_id.clone(),
                diagnostic_kind: entry.diagnostic_kind,
                authority: entry.authority.clone(),
            })
            .collect()
    }

    pub fn success_publication_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind == M8DesignatedTraceKind::ValuePublished)
            .count()
    }

    pub fn contains_failure(
        &self,
        kind: M8DesignatedTraceKind,
        diagnostic_kind: M8DesignatedDiagnosticKind,
    ) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == kind && entry.diagnostic_kind == Some(diagnostic_kind))
    }

    pub fn authority_reference_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.authority.is_some())
            .count()
    }

    pub fn node_indexes_are_monotone(&self) -> bool {
        self.entries
            .windows(2)
            .all(|pair| pair[0].node_index < pair[1].node_index)
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

    /// Return the bounded causal chain for one successful designated-value
    /// publication.  The chain only follows dependencies owned by the same
    /// evaluation occurrence, in deterministic trace order.
    pub fn causal_chain_for(&self, occurrence_id: &str) -> Vec<M8DesignatedTraceKind> {
        let Some(publication) = self.entries.iter().find(|entry| {
            entry.kind == M8DesignatedTraceKind::ValuePublished
                && entry.occurrence_id.as_deref() == Some(occurrence_id)
        }) else {
            return Vec::new();
        };
        let mut required = BTreeSet::from([publication.node_id.clone()]);
        let mut changed = true;
        while changed {
            changed = false;
            for entry in &self.entries {
                if required.contains(&entry.node_id) {
                    for dependency in &entry.dependencies {
                        changed |= required.insert(dependency.clone());
                    }
                }
            }
        }
        self.entries
            .iter()
            .filter(|entry| {
                required.contains(&entry.node_id)
                    && entry.occurrence_id.as_deref() == Some(occurrence_id)
            })
            .map(|entry| entry.kind)
            .collect()
    }

    pub fn authority_precedes_receipt_provenance_access(&self, occurrence_id: &str) -> bool {
        self.authority_precedes_receipt_access(occurrence_id, |entry| {
            entry.receipt_provenance_access
        })
    }

    pub fn authority_precedes_receipt_value_access(&self, occurrence_id: &str) -> bool {
        self.authority_precedes_receipt_access(occurrence_id, |entry| entry.receipt_value_access)
    }

    pub fn contains_receipt_provenance_or_value_access(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.receipt_provenance_access || entry.receipt_value_access)
    }

    fn authority_precedes_receipt_access(
        &self,
        occurrence_id: &str,
        access: impl Fn(&M8DesignatedTraceEntry) -> bool,
    ) -> bool {
        let Some(authority_index) = self
            .entries
            .iter()
            .find(|entry| {
                entry.kind == M8DesignatedTraceKind::AuthorityValidated
                    && entry.occurrence_id.as_deref() == Some(occurrence_id)
            })
            .map(|entry| entry.node_index)
        else {
            return false;
        };
        let accesses: Vec<_> = self
            .entries
            .iter()
            .filter(|entry| entry.occurrence_id.as_deref() == Some(occurrence_id) && access(entry))
            .collect();
        !accesses.is_empty()
            && accesses
                .into_iter()
                .all(|entry| authority_index < entry.node_index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PublishedDesignatedValue {
    value_id: String,
    value_name: String,
    evaluator: String,
    result: String,
    logical_tick: M8DesignatedTick,
    input_frontier: InputFrontier,
    result_frontier: ResultFrontier,
    result_version: ResultVersion,
    evaluation_policy: EvaluationPolicy,
    observation_policy: ObservationPolicy,
    policy_stamp: PolicyStamp,
    int_value: Option<i64>,
    authority: M8DesignatedAuthorityUse,
    visibility_label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
    source_ref: SourceRef,
    publication_node_id: String,
    occurrence_id: String,
    input_security_class_join: M8SecurityClass,
}

impl M8PublishedDesignatedValue {
    pub fn value_id(&self) -> &str {
        &self.value_id
    }

    pub fn value_name(&self) -> &str {
        &self.value_name
    }

    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub fn logical_tick(&self) -> &M8DesignatedTick {
        &self.logical_tick
    }

    pub fn input_frontier(&self) -> &InputFrontier {
        &self.input_frontier
    }

    pub fn result_frontier(&self) -> &ResultFrontier {
        &self.result_frontier
    }

    pub const fn result_version(&self) -> ResultVersion {
        self.result_version
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

    pub const fn int_value(&self) -> Option<i64> {
        self.int_value
    }

    pub fn authority(&self) -> &M8DesignatedAuthorityUse {
        &self.authority
    }

    pub fn visibility_label(&self) -> &EvidenceSecurityLabel {
        &self.visibility_label
    }

    pub fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }

    pub fn occurrence_id(&self) -> &str {
        &self.occurrence_id
    }

    pub const fn input_security_class_join(&self) -> M8SecurityClass {
        self.input_security_class_join
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct M8PublishedValueKey {
    value_name: String,
    result_version: ResultVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8DesignatedResultStore {
    values: BTreeMap<M8PublishedValueKey, M8PublishedDesignatedValue>,
}

impl M8DesignatedResultStore {
    pub fn success_publications(&self, value_name: &str) -> Vec<&M8PublishedDesignatedValue> {
        self.values
            .iter()
            .filter(|(key, _)| key.value_name == value_name)
            .map(|(_, value)| value)
            .collect()
    }

    pub fn published_values(&self, value_name: &str) -> Vec<&M8PublishedDesignatedValue> {
        self.success_publications(value_name)
    }

    pub(crate) fn published_value_observations(&self) -> Vec<M8PublishedValueObservation> {
        self.values
            .values()
            .map(|published| M8PublishedValueObservation {
                value_name: published.value_name.clone(),
                occurrence_id: published.occurrence_id.clone(),
            })
            .collect()
    }

    pub fn published_value(
        &self,
        value_name: &str,
        result_version: ResultVersion,
    ) -> Option<&M8PublishedDesignatedValue> {
        self.values.get(&M8PublishedValueKey {
            value_name: value_name.to_string(),
            result_version,
        })
    }

    fn insert(&mut self, value: M8PublishedDesignatedValue) {
        self.values.insert(
            M8PublishedValueKey {
                value_name: value.value_name.clone(),
                result_version: value.result_version,
            },
            value,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8ReceiptState {
    receipts: M8InputReceiptSet,
}

impl M8ReceiptState {
    pub fn receipt(&self, reference: &str) -> Option<&M8InputReceipt> {
        self.receipts.receipts.get(reference)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8ConsumptionState {
    consumed: BTreeMap<(String, String), Vec<String>>,
}

impl M8ConsumptionState {
    pub fn consumed_deliveries(&self, consumer: &str, value_name: &str) -> Vec<String> {
        self.consumed
            .get(&(consumer.to_string(), value_name.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    fn already_consumed(&self, consumer: &str, value_name: &str, delivery_id: &str) -> bool {
        self.consumed
            .get(&(consumer.to_string(), value_name.to_string()))
            .is_some_and(|deliveries| deliveries.iter().any(|known| known == delivery_id))
    }

    fn record(&mut self, consumer: &str, value_name: &str, delivery_id: String) {
        self.consumed
            .entry((consumer.to_string(), value_name.to_string()))
            .or_default()
            .push(delivery_id);
    }

    pub(crate) fn covers(&self, floor: &Self) -> bool {
        floor.consumed.iter().all(|(key, deliveries)| {
            self.consumed
                .get(key)
                .is_some_and(|current| deliveries.iter().all(|delivery| current.contains(delivery)))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8ResultVersionStore {
    versions: BTreeMap<String, ResultVersion>,
}

impl M8ResultVersionStore {
    pub fn version(&self, value_name: &str) -> Option<ResultVersion> {
        self.versions.get(value_name).copied()
    }

    pub(crate) fn satisfies_floor(&self, floor: &Self) -> bool {
        floor.versions.iter().all(|(value, version)| {
            self.versions
                .get(value)
                .is_some_and(|current| current >= version)
        })
    }

    pub(crate) fn set_floor(&mut self, value_name: impl Into<String>, version: ResultVersion) {
        self.versions.insert(value_name.into(), version);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ConsumedDesignatedValue {
    consumer_locus: String,
    value_name: String,
    int_value: Option<i64>,
    result_version: ResultVersion,
    authority: M8DesignatedAuthorityUse,
}

impl M8ConsumedDesignatedValue {
    pub fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub fn value_name(&self) -> &str {
        &self.value_name
    }

    pub const fn int_value(&self) -> Option<i64> {
        self.int_value
    }

    pub const fn result_version(&self) -> ResultVersion {
        self.result_version
    }

    pub fn authority(&self) -> &M8DesignatedAuthorityUse {
        &self.authority
    }

    pub const fn reevaluated_semantics(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ConsumerApi {
    consumer_locus: String,
}

impl M8ConsumerApi {
    pub fn can_semantically_reevaluate(&self, _value_name: &str) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PresentationInterpolation {
    consumer_locus: String,
    frame: Option<String>,
    display_hint_int: Option<i64>,
}

impl M8PresentationInterpolation {
    pub fn for_consumer(consumer_locus: impl Into<String>) -> Self {
        Self {
            consumer_locus: consumer_locus.into(),
            frame: None,
            display_hint_int: None,
        }
    }

    pub fn with_frame(mut self, frame: impl Into<String>) -> Self {
        self.frame = Some(frame.into());
        self
    }

    pub fn with_display_hint_int(mut self, display_hint_int: i64) -> Self {
        self.display_hint_int = Some(display_hint_int);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedRuntime {
    plans: Vec<M8DesignatedExecutionPlan>,
    pub(crate) semantic_snapshot: M8SemanticSnapshot,
    pub(crate) receipt_state: M8ReceiptState,
    pub(crate) result_store: M8DesignatedResultStore,
    pub(crate) version_store: M8ResultVersionStore,
    pub(crate) consumption_state: M8ConsumptionState,
    presentation_interpolations:
        BTreeMap<(String, String, ResultVersion), M8PresentationInterpolation>,
    owner_store_reads: Vec<String>,
    hidden_communications: Vec<String>,
    pub(crate) trace: M8DesignatedTrace,
    pub(crate) next_trace_node: u64,
    pub(crate) next_occurrence: u64,
}

impl M8DesignatedRuntime {
    pub fn from_admitted(instance: M8RuntimeInstance, seed: M8DesignatedSeed) -> Self {
        Self {
            plans: instance.designated_execution_plans().to_vec(),
            semantic_snapshot: M8SemanticSnapshot::empty_with_authority_state(seed.authority_state),
            receipt_state: M8ReceiptState {
                receipts: seed.input_receipts,
            },
            result_store: M8DesignatedResultStore::default(),
            version_store: M8ResultVersionStore::default(),
            consumption_state: M8ConsumptionState::default(),
            presentation_interpolations: BTreeMap::new(),
            owner_store_reads: Vec::new(),
            hidden_communications: Vec::new(),
            trace: M8DesignatedTrace::default(),
            next_trace_node: 0,
            next_occurrence: 0,
        }
    }

    pub fn semantic_snapshot(&self) -> M8SemanticSnapshot {
        self.semantic_snapshot.clone()
    }

    pub fn result_store(&self) -> &M8DesignatedResultStore {
        &self.result_store
    }

    pub fn version_store(&self) -> &M8ResultVersionStore {
        &self.version_store
    }

    pub fn receipt_state(&self) -> &M8ReceiptState {
        &self.receipt_state
    }

    pub fn consumption_state(&self) -> &M8ConsumptionState {
        &self.consumption_state
    }

    pub(crate) fn replace_admitted_plans(
        &mut self,
        instance: &M8RuntimeInstance,
        input_receipts: Option<M8InputReceiptSet>,
    ) {
        self.plans = instance.designated_execution_plans().to_vec();
        if let Some(input_receipts) = input_receipts {
            self.receipt_state.receipts = input_receipts;
        }
    }

    pub fn authority_state(&self) -> &M8AuthorityState {
        self.semantic_snapshot.authority_state()
    }

    pub fn trace(&self) -> &M8DesignatedTrace {
        &self.trace
    }

    pub fn owner_store_reads(&self) -> &[String] {
        &self.owner_store_reads
    }

    pub fn hidden_communications(&self) -> &[String] {
        &self.hidden_communications
    }

    pub fn consumer_api(&self, consumer_locus: &str) -> M8ConsumerApi {
        M8ConsumerApi {
            consumer_locus: consumer_locus.to_string(),
        }
    }

    pub fn evaluate_designated(
        &mut self,
        request: M8DesignatedEvaluationRequest,
    ) -> Result<M8PublishedDesignatedValue, M8DesignatedDiagnostics> {
        let plan = self.plan(&request.value_name).cloned().ok_or_else(|| {
            M8DesignatedDiagnostics::one(
                M8DesignatedDiagnosticKind::UnknownDesignatedValue,
                SourceRef::new("<m8-designated>", 1, 1, 1, 1),
            )
        })?;
        let Some(tick) = request.tick else {
            return Err(self.record_evaluation_failure(
                &plan,
                None,
                M8DesignatedDiagnosticKind::TickFrontierMismatch,
            ));
        };
        let expected_frontier = plan.core().trigger().frontier().unwrap_or("");
        if tick.input_frontier() != expected_frontier {
            return Err(self.record_evaluation_failure(
                &plan,
                request.authority,
                M8DesignatedDiagnosticKind::TickFrontierMismatch,
            ));
        }
        let authority = request
            .authority
            .unwrap_or_else(|| M8DesignatedAuthorityUse::for_evaluator("").with_principal(""));
        let occurrence_id = self.next_occurrence_id();
        let evaluator = plan.core().evaluator();
        let result = plan.core().result();
        let authority_is_valid = authority.evaluator() == Some(evaluator)
            && authority.principal().is_some()
            && self
                .semantic_snapshot
                .authority_state()
                .validates_designated_evaluation_use(M8DesignatedEvaluationAuthorityLookup {
                    evaluator,
                    result,
                    input_frontier: tick.input_frontier(),
                    principal: authority.principal().unwrap_or(""),
                    membership_ref: authority.membership_ref.as_deref(),
                    capability_ref: authority.capability_ref.as_deref(),
                    witness_ref: authority.witness_ref.as_deref(),
                });
        if !authority_is_valid {
            return Err(self.record_evaluation_failure_with_context(
                &plan,
                Some(authority),
                M8DesignatedDiagnosticKind::MissingEvaluatorAuthority,
                BTreeSet::new(),
                Some(&occurrence_id),
            ));
        }
        let authority_node = self.append_trace_with_context(
            M8DesignatedTraceKind::AuthorityValidated,
            BTreeSet::new(),
            None,
            Some(authority.clone()),
            plan.source_ref().clone(),
            M8TraceContext::for_occurrence(&occurrence_id, false, false),
        );
        let validated_receipts = match self.validate_receipts(&plan, &tick) {
            Ok(receipts) => receipts,
            Err(kind) => {
                return Err(self.record_evaluation_failure_with_context(
                    &plan,
                    Some(authority),
                    kind,
                    BTreeSet::from([authority_node]),
                    Some(&occurrence_id),
                ));
            }
        };
        let receipt_node = self.append_trace_with_context(
            M8DesignatedTraceKind::InputReceiptValidated,
            BTreeSet::from([authority_node]),
            None,
            None,
            plan.source_ref().clone(),
            M8TraceContext::for_occurrence(&occurrence_id, true, true),
        );
        if !plan
            .visibility_label()
            .security_class()
            .is_at_least(validated_receipts.security_class_join)
        {
            return Err(self.record_evaluation_failure_with_context(
                &plan,
                Some(authority),
                M8DesignatedDiagnosticKind::OutputVisibilityWouldWeakenInput,
                BTreeSet::from([receipt_node]),
                Some(&occurrence_id),
            ));
        }
        let result_version = plan.core().result_version();
        if let Some(existing) = self
            .result_store
            .published_value(plan.name(), result_version)
        {
            let existing = existing.clone();
            self.append_trace_with_context(
                M8DesignatedTraceKind::EvaluationIdempotent,
                BTreeSet::from([receipt_node]),
                None,
                Some(authority),
                plan.source_ref().clone(),
                M8TraceContext::for_occurrence(&occurrence_id, false, false),
            );
            return Ok(existing);
        }
        let int_value = match evaluate_checked_expression(
            plan.core().expression().tree(),
            &validated_receipts.values,
        ) {
            Ok(value) => value,
            Err(()) => {
                return Err(self.record_evaluation_failure_with_context(
                    &plan,
                    Some(authority),
                    M8DesignatedDiagnosticKind::ArithmeticFailure,
                    BTreeSet::from([receipt_node]),
                    Some(&occurrence_id),
                ));
            }
        };
        let publication_node_id = self.append_trace_with_context(
            M8DesignatedTraceKind::ValuePublished,
            BTreeSet::from([receipt_node]),
            None,
            Some(authority.clone()),
            plan.source_ref().clone(),
            M8TraceContext::for_occurrence(&occurrence_id, false, false),
        );
        let publication = M8PublishedDesignatedValue {
            value_id: format!("{}:version{}", plan.name(), result_version.value()),
            value_name: plan.name().to_string(),
            evaluator: evaluator.to_string(),
            result: result.to_string(),
            logical_tick: tick,
            input_frontier: plan.core().input_frontier().clone(),
            result_frontier: plan.core().result_frontier().clone(),
            result_version,
            evaluation_policy: plan.core().evaluation_policy().clone(),
            observation_policy: plan.core().observation_policy().clone(),
            policy_stamp: plan.core().policy_stamp().clone(),
            int_value: Some(int_value),
            authority,
            visibility_label: plan.visibility_label().clone(),
            redaction: plan.redaction().clone(),
            source_ref: plan.source_ref().clone(),
            publication_node_id,
            occurrence_id,
            input_security_class_join: validated_receipts.security_class_join,
        };
        self.version_store
            .versions
            .insert(plan.name().to_string(), result_version);
        self.result_store.insert(publication.clone());
        Ok(publication)
    }

    pub fn consume_published_value(
        &mut self,
        request: M8ConsumeRequest,
    ) -> Result<M8ConsumedDesignatedValue, M8DesignatedDiagnostics> {
        let plan = self.plan(&request.value_name).cloned().ok_or_else(|| {
            M8DesignatedDiagnostics::one(
                M8DesignatedDiagnosticKind::MissingPublishedValue,
                SourceRef::new("<m8-designated>", 1, 1, 1, 1),
            )
        })?;
        let result_version = plan.core().result_version();
        let consumer = request.consumer.unwrap_or_default();
        let delivery_id = request.delivery_id.unwrap_or_default();
        let authority = request
            .authority
            .unwrap_or_else(|| M8DesignatedAuthorityUse::for_consumer("").with_principal(""));
        let authority_is_valid = authority.consumer() == Some(consumer.as_str())
            && authority.principal().is_some()
            && self
                .semantic_snapshot
                .authority_state()
                .validates_designated_consumption_use(M8DesignatedConsumptionAuthorityLookup {
                    consumer: &consumer,
                    value_name: plan.name(),
                    result_version,
                    principal: authority.principal().unwrap_or(""),
                    membership_ref: authority.membership_ref.as_deref(),
                    capability_ref: authority.capability_ref.as_deref(),
                    witness_ref: authority.witness_ref.as_deref(),
                });
        if !authority_is_valid {
            return Err(self.record_consumption_failure(
                &plan,
                Some(authority),
                M8DesignatedDiagnosticKind::MissingConsumerAuthority,
                BTreeSet::new(),
            ));
        }
        let authority_node = self.append_trace(
            M8DesignatedTraceKind::ConsumerAuthorityValidated,
            BTreeSet::new(),
            None,
            Some(authority.clone()),
            plan.source_ref().clone(),
        );
        let Some(published) = self
            .result_store
            .published_value(plan.name(), result_version)
            .cloned()
        else {
            return Err(self.record_consumption_failure(
                &plan,
                Some(authority),
                M8DesignatedDiagnosticKind::MissingPublishedValue,
                BTreeSet::from([authority_node]),
            ));
        };
        if self
            .consumption_state
            .already_consumed(&consumer, plan.name(), &delivery_id)
        {
            return Err(self.record_consumption_failure(
                &plan,
                Some(authority),
                M8DesignatedDiagnosticKind::AlreadyConsumed,
                BTreeSet::from([authority_node]),
            ));
        }
        self.consumption_state
            .record(&consumer, plan.name(), delivery_id);
        self.append_trace(
            M8DesignatedTraceKind::ValueConsumed,
            BTreeSet::from([authority_node]),
            None,
            Some(authority.clone()),
            plan.source_ref().clone(),
        );
        Ok(M8ConsumedDesignatedValue {
            consumer_locus: consumer,
            value_name: plan.name().to_string(),
            int_value: published.int_value(),
            result_version,
            authority,
        })
    }

    pub fn attach_presentation_interpolation(
        &mut self,
        value_name: &str,
        result_version: ResultVersion,
        interpolation: M8PresentationInterpolation,
    ) -> Result<(), M8DesignatedDiagnostics> {
        let Some(published) = self
            .result_store
            .published_value(value_name, result_version)
        else {
            return Err(M8DesignatedDiagnostics::one(
                M8DesignatedDiagnosticKind::MissingPublishedValue,
                SourceRef::new("<m8-designated>", 1, 1, 1, 1),
            ));
        };
        self.presentation_interpolations.insert(
            (
                interpolation.consumer_locus.clone(),
                value_name.to_string(),
                result_version,
            ),
            interpolation,
        );
        let _ = published;
        Ok(())
    }

    pub fn run_replay(mut self, log: M8DesignatedReplayLog) -> M8DesignatedReplayReport {
        for operation in log.operations {
            match operation {
                M8DesignatedReplayOperation::Evaluation(request) => {
                    let _ = self.evaluate_designated(request);
                }
                M8DesignatedReplayOperation::Consumption(request) => {
                    let _ = self.consume_published_value(request);
                }
            }
        }
        M8DesignatedReplayReport {
            result_store: self.result_store,
            version_store: self.version_store,
            receipt_state: self.receipt_state,
            consumption_state: self.consumption_state,
            trace: self.trace,
        }
    }

    fn plan(&self, value_name: &str) -> Option<&M8DesignatedExecutionPlan> {
        self.plans.iter().find(|plan| plan.name() == value_name)
    }

    fn validate_receipts(
        &self,
        plan: &M8DesignatedExecutionPlan,
        tick: &M8DesignatedTick,
    ) -> Result<M8ValidatedInputReceipts, M8DesignatedDiagnosticKind> {
        let mut values = BTreeMap::new();
        let mut security_class_join = M8SecurityClass::Public;
        for dependency in plan.core().generated_remote_input_dependencies() {
            let read = dependency.typed_state_read();
            let key =
                state_key_from_read(read).ok_or(M8DesignatedDiagnosticKind::MissingInputReceipt)?;
            let Some(receipt) = self.receipt_state.receipts.for_state_key(&key) else {
                return Err(M8DesignatedDiagnosticKind::MissingInputReceipt);
            };
            if !receipt.live {
                return Err(M8DesignatedDiagnosticKind::StaleInputReceipt);
            }
            if receipt.source_owner_locus() != dependency.source_owner_locus()
                || receipt.evaluator() != plan.core().evaluator()
            {
                return Err(M8DesignatedDiagnosticKind::InputReceiptSourceMismatch);
            }
            if receipt.input_frontier() != tick.input_frontier()
                || receipt.input_frontier() != plan.core().trigger().frontier().unwrap_or("")
            {
                return Err(M8DesignatedDiagnosticKind::InputReceiptFrontierMismatch);
            }
            if receipt.source_ref() != &read.source_ref() {
                return Err(M8DesignatedDiagnosticKind::InputReceiptSourceRefMismatch);
            }
            if receipt
                .label
                .as_ref()
                .is_none_or(|label| label.as_str().is_empty())
            {
                return Err(M8DesignatedDiagnosticKind::InputReceiptLabelMissing);
            }
            let Some(value) = receipt.int_value() else {
                return Err(M8DesignatedDiagnosticKind::InputReceiptValueMissing);
            };
            security_class_join = security_class_join.join(
                receipt
                    .label
                    .as_ref()
                    .expect("labels were checked above")
                    .security_class(),
            );
            values.insert(key, value);
        }
        Ok(M8ValidatedInputReceipts {
            values,
            security_class_join,
        })
    }

    fn record_evaluation_failure(
        &mut self,
        plan: &M8DesignatedExecutionPlan,
        authority: Option<M8DesignatedAuthorityUse>,
        kind: M8DesignatedDiagnosticKind,
    ) -> M8DesignatedDiagnostics {
        self.record_evaluation_failure_with_context(plan, authority, kind, BTreeSet::new(), None)
    }

    fn record_evaluation_failure_with_context(
        &mut self,
        plan: &M8DesignatedExecutionPlan,
        authority: Option<M8DesignatedAuthorityUse>,
        kind: M8DesignatedDiagnosticKind,
        dependencies: BTreeSet<String>,
        occurrence_id: Option<&str>,
    ) -> M8DesignatedDiagnostics {
        if let Some(occurrence_id) = occurrence_id {
            self.append_trace_with_context(
                M8DesignatedTraceKind::EvaluationFailed,
                dependencies,
                Some(kind),
                authority,
                plan.source_ref().clone(),
                M8TraceContext::for_occurrence(occurrence_id, false, false),
            );
        } else {
            self.append_trace(
                M8DesignatedTraceKind::EvaluationFailed,
                dependencies,
                Some(kind),
                authority,
                plan.source_ref().clone(),
            );
        }
        M8DesignatedDiagnostics::one(kind, plan.source_ref().clone())
    }

    fn append_trace(
        &mut self,
        kind: M8DesignatedTraceKind,
        dependencies: BTreeSet<String>,
        diagnostic_kind: Option<M8DesignatedDiagnosticKind>,
        authority: Option<M8DesignatedAuthorityUse>,
        source_ref: SourceRef,
    ) -> String {
        self.append_trace_with_context(
            kind,
            dependencies,
            diagnostic_kind,
            authority,
            source_ref,
            M8TraceContext::default(),
        )
    }

    fn append_trace_with_context(
        &mut self,
        kind: M8DesignatedTraceKind,
        dependencies: BTreeSet<String>,
        diagnostic_kind: Option<M8DesignatedDiagnosticKind>,
        authority: Option<M8DesignatedAuthorityUse>,
        source_ref: SourceRef,
        context: M8TraceContext,
    ) -> String {
        let node_index = self.next_trace_node;
        self.next_trace_node += 1;
        let node_id = format!("m8-designated-trace-{node_index:020}");
        self.trace.entries.push(M8DesignatedTraceEntry {
            kind,
            node_id: node_id.clone(),
            node_index,
            dependencies,
            diagnostic_kind,
            authority,
            source_ref,
            occurrence_id: context.occurrence_id,
            receipt_provenance_access: context.receipt_provenance_access,
            receipt_value_access: context.receipt_value_access,
        });
        node_id
    }

    fn record_consumption_failure(
        &mut self,
        plan: &M8DesignatedExecutionPlan,
        authority: Option<M8DesignatedAuthorityUse>,
        kind: M8DesignatedDiagnosticKind,
        dependencies: BTreeSet<String>,
    ) -> M8DesignatedDiagnostics {
        self.append_trace(
            M8DesignatedTraceKind::ConsumptionRejected,
            dependencies,
            Some(kind),
            authority,
            plan.source_ref().clone(),
        );
        M8DesignatedDiagnostics::one(kind, plan.source_ref().clone())
    }

    fn next_occurrence_id(&mut self) -> String {
        let occurrence = self.next_occurrence;
        self.next_occurrence += 1;
        format!("m8-designated-occurrence-{occurrence:020}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M8ValidatedInputReceipts {
    values: BTreeMap<M8StateKey, i64>,
    security_class_join: M8SecurityClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct M8TraceContext {
    occurrence_id: Option<String>,
    receipt_provenance_access: bool,
    receipt_value_access: bool,
}

impl M8TraceContext {
    fn for_occurrence(
        occurrence_id: &str,
        receipt_provenance_access: bool,
        receipt_value_access: bool,
    ) -> Self {
        Self {
            occurrence_id: Some(occurrence_id.to_string()),
            receipt_provenance_access,
            receipt_value_access,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8DesignatedReplayLog {
    operations: Vec<M8DesignatedReplayOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum M8DesignatedReplayOperation {
    Evaluation(M8DesignatedEvaluationRequest),
    Consumption(M8ConsumeRequest),
}

impl M8DesignatedReplayLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_evaluation(mut self, request: M8DesignatedEvaluationRequest) -> Self {
        self.operations
            .push(M8DesignatedReplayOperation::Evaluation(request));
        self
    }

    pub fn with_consumption(mut self, request: M8ConsumeRequest) -> Self {
        self.operations
            .push(M8DesignatedReplayOperation::Consumption(request));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8DesignatedReplayReport {
    result_store: M8DesignatedResultStore,
    version_store: M8ResultVersionStore,
    receipt_state: M8ReceiptState,
    consumption_state: M8ConsumptionState,
    trace: M8DesignatedTrace,
}

impl M8DesignatedReplayReport {
    pub fn result_store(&self) -> &M8DesignatedResultStore {
        &self.result_store
    }

    pub fn version_store(&self) -> &M8ResultVersionStore {
        &self.version_store
    }

    pub fn receipt_state(&self) -> &M8ReceiptState {
        &self.receipt_state
    }

    pub fn consumption_state(&self) -> &M8ConsumptionState {
        &self.consumption_state
    }

    pub fn trace(&self) -> &M8DesignatedTrace {
        &self.trace
    }
}

fn state_key_from_read(read: &TypedStateRead) -> Option<M8StateKey> {
    Some(M8StateKey::indexed_field(
        read.namespace(),
        read.index()?,
        read.field()?,
    ))
}

fn evaluate_checked_expression(
    tree: &CheckedExpressionTree,
    receipt_values: &BTreeMap<M8StateKey, i64>,
) -> Result<i64, ()> {
    match tree {
        CheckedExpressionTree::StateRead(read) => receipt_values
            .get(&state_key_from_read(read).ok_or(())?)
            .copied()
            .ok_or(()),
        CheckedExpressionTree::IntegerLiteral(literal) => Ok(literal.value()),
        CheckedExpressionTree::Binary {
            operator,
            left,
            right,
            ..
        } => {
            let left = evaluate_checked_expression(left, receipt_values)?;
            let right = evaluate_checked_expression(right, receipt_values)?;
            match operator {
                CheckedBinaryOperator::Add => left.checked_add(right).ok_or(()),
                CheckedBinaryOperator::Subtract => left.checked_sub(right).ok_or(()),
            }
        }
    }
}
