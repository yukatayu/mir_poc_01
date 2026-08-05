//! Bounded M8 maintained-relation semantics and consumer-local projection.
//!
//! The runtime consumes admitted checked relation Core only.  Its semantic
//! relation state is stored in the same `M8SemanticSnapshot` shape used by the
//! owner queue, while presentation contexts remain ephemeral inputs.

use std::collections::BTreeMap;

use mir_semantics::shared_model::SourceRef;

use crate::{
    m8_runtime_admission::{M8RelationExecutionPlan, M8RuntimeInstance, M8SecurityClass},
    m8_runtime_authority::{M8AuthorityState, M8RelationAuthorityLookup},
    m8_runtime_owner_queue::{
        M8RelationFloor, M8SemanticRelation, M8SemanticRelationInitialState, M8SemanticSnapshot,
    },
};

/// Finite already-admitted relation lease record.  This lives with relation
/// execution so both the standalone facade and the unified local session can
/// apply the same live-lease gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8LeaseRecord {
    pub(crate) reference: String,
    live: bool,
    relation: Option<String>,
    owner_locus: Option<String>,
    binding_frontier: Option<String>,
    epoch: Option<String>,
    anchor_epoch: Option<String>,
}

impl M8LeaseRecord {
    pub fn live(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            live: true,
            relation: None,
            owner_locus: None,
            binding_frontier: None,
            epoch: None,
            anchor_epoch: None,
        }
    }

    pub fn for_relation(mut self, relation: impl Into<String>) -> Self {
        self.relation = Some(relation.into());
        self
    }

    pub fn with_owner_locus(mut self, owner_locus: impl Into<String>) -> Self {
        self.owner_locus = Some(owner_locus.into());
        self
    }

    pub fn with_binding_frontier(mut self, binding_frontier: impl Into<String>) -> Self {
        self.binding_frontier = Some(binding_frontier.into());
        self
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }

    pub fn with_anchor_epoch(mut self, anchor_epoch: impl Into<String>) -> Self {
        self.anchor_epoch = Some(anchor_epoch.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8LeaseInventory {
    pub(crate) records: BTreeMap<String, M8LeaseRecord>,
}

impl M8LeaseInventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_live_lease(mut self, lease: M8LeaseRecord) -> Self {
        self.records.insert(lease.reference.clone(), lease);
        self
    }

    pub fn contains_live(&self, reference: &str) -> bool {
        self.records
            .get(reference)
            .is_some_and(|record| record.live)
    }

    pub(crate) fn canonical_projection(&self) -> String {
        self.records
            .values()
            .map(|record| {
                format!(
                    "lease|{}|{}|{}|{}|{}|{}|{}",
                    record.reference,
                    record.live,
                    record.relation.as_deref().unwrap_or(""),
                    record.owner_locus.as_deref().unwrap_or(""),
                    record.binding_frontier.as_deref().unwrap_or(""),
                    record.epoch.as_deref().unwrap_or(""),
                    record.anchor_epoch.as_deref().unwrap_or(""),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn contains_live_exact_binding(
        &self,
        reference: &str,
        relation: &str,
        owner_locus: &str,
        binding_frontier: &str,
        binding_epoch: &str,
    ) -> bool {
        self.records.get(reference).is_some_and(|record| {
            record.live
                && record.relation.as_deref() == Some(relation)
                && record.owner_locus.as_deref() == Some(owner_locus)
                && record.binding_frontier.as_deref() == Some(binding_frontier)
                && record.epoch.as_deref() == Some(binding_epoch)
        })
    }

    fn contains_live_fresh_reacquire(
        &self,
        reference: &str,
        relation: &str,
        owner_locus: &str,
        binding_frontier: &str,
        binding_epoch: &str,
        anchor_epoch: &str,
    ) -> bool {
        self.contains_live_exact_binding(
            reference,
            relation,
            owner_locus,
            binding_frontier,
            binding_epoch,
        ) && self
            .records
            .get(reference)
            .is_some_and(|record| record.anchor_epoch.as_deref() == Some(anchor_epoch))
    }
}

/// A finite, already-validated three-option relation chain retained by M8.
/// M10 may seed this carrier only after validating its typed fallback input;
/// subsequent option selection is owned by this runtime rather than an M10
/// cursor or report-local state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8FiniteFallbackOption {
    target: String,
    lease_ref: String,
    required_capability: String,
    epoch: String,
}

impl M8FiniteFallbackOption {
    pub(crate) fn new(
        target: impl Into<String>,
        lease_ref: impl Into<String>,
        required_capability: impl Into<String>,
        epoch: impl Into<String>,
    ) -> Self {
        Self {
            target: target.into(),
            lease_ref: lease_ref.into(),
            required_capability: required_capability.into(),
            epoch: epoch.into(),
        }
    }

    pub(crate) fn canonical_projection(&self) -> String {
        format!(
            "target|{}|lease|{}|capability|{}|epoch|{}",
            self.target, self.lease_ref, self.required_capability, self.epoch
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8FiniteFallbackChain {
    relation: String,
    options: [M8FiniteFallbackOption; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8FiniteFallbackSelection {
    option_index: usize,
    floor: M8RelationFloor,
    target: String,
    lease_ref: String,
    required_capability: String,
    epoch: String,
}

impl M8FiniteFallbackSelection {
    pub(crate) const fn option_index(&self) -> usize {
        self.option_index
    }

    pub(crate) const fn floor(&self) -> M8RelationFloor {
        self.floor
    }

    pub(crate) fn target(&self) -> &str {
        &self.target
    }

    pub(crate) fn lease_ref(&self) -> &str {
        &self.lease_ref
    }

    pub(crate) fn required_capability(&self) -> &str {
        &self.required_capability
    }

    pub(crate) fn epoch(&self) -> &str {
        &self.epoch
    }
}

impl M8FiniteFallbackChain {
    pub(crate) fn live_anchor_frozen(
        relation: impl Into<String>,
        live: M8FiniteFallbackOption,
        anchor: M8FiniteFallbackOption,
        frozen: M8FiniteFallbackOption,
    ) -> Self {
        Self {
            relation: relation.into(),
            options: [live, anchor, frozen],
        }
    }

    fn relation(&self) -> &str {
        &self.relation
    }

    fn option(&self, index: usize) -> Option<&M8FiniteFallbackOption> {
        self.options.get(index)
    }

    pub(crate) fn canonical_projection(&self) -> String {
        format!(
            "relation|{}|0:{}|1:{}|2:{}",
            self.relation,
            self.options[0].canonical_projection(),
            self.options[1].canonical_projection(),
            self.options[2].canonical_projection(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum M8RestrictionPolicy {
    Public,
    Restricted,
    Private,
}

impl M8RestrictionPolicy {
    pub fn is_at_least(self, other: Self) -> bool {
        self >= other
    }

    fn join(self, other: Self) -> Self {
        self.max(other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8RelationProjectionSeed {
    relation_policies: BTreeMap<String, M8RestrictionPolicy>,
    subject_policies: BTreeMap<String, M8RestrictionPolicy>,
    anchor_policies: BTreeMap<String, M8RestrictionPolicy>,
    authority_state: M8AuthorityState,
    live_leases: M8LeaseInventory,
}

impl M8RelationProjectionSeed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_authority_state(mut self, authority_state: M8AuthorityState) -> Self {
        self.authority_state = authority_state;
        self
    }

    pub fn with_live_leases(mut self, live_leases: M8LeaseInventory) -> Self {
        self.live_leases = live_leases;
        self
    }

    pub fn with_relation_policy(
        mut self,
        relation: impl Into<String>,
        policy: M8RestrictionPolicy,
    ) -> Self {
        self.relation_policies.insert(relation.into(), policy);
        self
    }

    pub fn with_subject_policy(
        mut self,
        subject: impl Into<String>,
        policy: M8RestrictionPolicy,
    ) -> Self {
        self.subject_policies.insert(subject.into(), policy);
        self
    }

    pub fn with_anchor_policy(
        mut self,
        anchor: impl Into<String>,
        policy: M8RestrictionPolicy,
    ) -> Self {
        self.anchor_policies.insert(anchor.into(), policy);
        self
    }

    fn into_parts(self) -> (M8PresentationPolicies, M8AuthorityState, M8LeaseInventory) {
        (
            M8PresentationPolicies {
                relation_policies: self.relation_policies,
                subject_policies: self.subject_policies,
                anchor_policies: self.anchor_policies,
            },
            self.authority_state,
            self.live_leases,
        )
    }
}

/// Consumer-local configuration retained separately from semantic state.
#[derive(Debug, Clone, PartialEq, Eq)]
struct M8PresentationPolicies {
    relation_policies: BTreeMap<String, M8RestrictionPolicy>,
    subject_policies: BTreeMap<String, M8RestrictionPolicy>,
    anchor_policies: BTreeMap<String, M8RestrictionPolicy>,
}

impl M8PresentationPolicies {
    fn relation_policy(&self, relation: &str) -> M8RestrictionPolicy {
        self.relation_policies
            .get(relation)
            .copied()
            .unwrap_or(M8RestrictionPolicy::Restricted)
    }

    fn subject_policy(&self, subject: &str) -> M8RestrictionPolicy {
        self.subject_policies
            .get(subject)
            .copied()
            .unwrap_or(M8RestrictionPolicy::Restricted)
    }

    fn anchor_policy(&self, anchor: &str) -> M8RestrictionPolicy {
        self.anchor_policies
            .get(anchor)
            .copied()
            .unwrap_or(M8RestrictionPolicy::Restricted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M8Point {
    x: i64,
    y: i64,
}

impl M8Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> i64 {
        self.x
    }

    pub const fn y(&self) -> i64 {
        self.y
    }

    fn translated(self, transform: &M8Transform2) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(transform.x)?,
            y: self.y.checked_add(transform.y)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8Transform2 {
    x: i64,
    y: i64,
}

impl M8Transform2 {
    pub const fn translate(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn identity() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8AnchorSample {
    anchor: String,
    epoch: Option<String>,
    frontier: Option<String>,
    pose: Option<M8Point>,
    policy: Option<M8RestrictionPolicy>,
}

impl M8AnchorSample {
    pub fn new(anchor: impl Into<String>) -> Self {
        Self {
            anchor: anchor.into(),
            epoch: None,
            frontier: None,
            pose: None,
            policy: None,
        }
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }

    pub fn with_frontier(mut self, frontier: impl Into<String>) -> Self {
        self.frontier = Some(frontier.into());
        self
    }

    pub fn with_pose(mut self, pose: M8Point) -> Self {
        self.pose = Some(pose);
        self
    }

    pub fn with_policy(mut self, policy: M8RestrictionPolicy) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn anchor(&self) -> &str {
        &self.anchor
    }

    pub fn frontier(&self) -> &str {
        self.frontier.as_deref().unwrap_or("")
    }

    fn epoch(&self) -> Option<&str> {
        self.epoch.as_deref()
    }

    fn pose(&self) -> Option<M8Point> {
        self.pose
    }

    fn policy(&self) -> Option<M8RestrictionPolicy> {
        self.policy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PresentationFallback {
    subject: String,
    pose: M8Point,
    policy: Option<M8RestrictionPolicy>,
}

impl M8PresentationFallback {
    pub fn hold_last_local(subject: impl Into<String>, pose: M8Point) -> Self {
        Self {
            subject: subject.into(),
            pose,
            policy: None,
        }
    }

    pub fn with_policy(mut self, policy: M8RestrictionPolicy) -> Self {
        self.policy = Some(policy);
        self
    }
}

/// Ephemeral consumer-local input.  It is never installed in semantic state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PresentationContext {
    consumer_locus: String,
    frontier: Option<String>,
    anchor_samples: Vec<M8AnchorSample>,
    fallback: Option<M8PresentationFallback>,
}

impl M8PresentationContext {
    pub fn for_consumer(consumer_locus: impl Into<String>) -> Self {
        Self {
            consumer_locus: consumer_locus.into(),
            frontier: None,
            anchor_samples: Vec::new(),
            fallback: None,
        }
    }

    pub fn with_frontier(mut self, frontier: impl Into<String>) -> Self {
        self.frontier = Some(frontier.into());
        self
    }

    pub fn with_anchor_sample(mut self, sample: M8AnchorSample) -> Self {
        self.anchor_samples.push(sample);
        self
    }

    pub fn with_presentation_fallback(mut self, fallback: M8PresentationFallback) -> Self {
        self.fallback = Some(fallback);
        self
    }

    fn frontier(&self) -> Option<&str> {
        self.frontier.as_deref()
    }

    fn sample(&self, anchor: &str) -> Option<&M8AnchorSample> {
        self.anchor_samples
            .iter()
            .find(|sample| sample.anchor() == anchor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationAuthorityUse {
    relation: String,
    owner_locus: Option<String>,
    transition: Option<String>,
    principal: Option<String>,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    // The authority membership epoch and the relation binding epoch are
    // distinct clocks.  Older local callers omit this and retain the legacy
    // binding-epoch interpretation; the sealed M9 bridge supplies it.
    membership_epoch: Option<String>,
    binding_epoch: Option<String>,
    witness_ref: Option<String>,
    witness_epoch: Option<String>,
}

impl M8RelationAuthorityUse {
    pub fn for_relation(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
            owner_locus: None,
            transition: None,
            principal: None,
            membership_ref: None,
            capability_ref: None,
            membership_epoch: None,
            binding_epoch: None,
            witness_ref: None,
            witness_epoch: None,
        }
    }

    pub fn with_owner_locus(mut self, owner_locus: impl Into<String>) -> Self {
        self.owner_locus = Some(owner_locus.into());
        self
    }

    pub fn with_transition(mut self, transition: impl Into<String>) -> Self {
        self.transition = Some(transition.into());
        self
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

    pub(crate) fn with_membership_epoch(mut self, membership_epoch: impl Into<String>) -> Self {
        self.membership_epoch = Some(membership_epoch.into());
        self
    }

    pub fn with_binding_epoch(mut self, binding_epoch: impl Into<String>) -> Self {
        self.binding_epoch = Some(binding_epoch.into());
        self
    }

    pub fn with_witness_ref(mut self, witness_ref: impl Into<String>) -> Self {
        self.witness_ref = Some(witness_ref.into());
        self
    }

    pub fn with_witness_epoch(mut self, witness_epoch: impl Into<String>) -> Self {
        self.witness_epoch = Some(witness_epoch.into());
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

    pub(crate) fn principal(&self) -> Option<&str> {
        self.principal.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8BindingInvalidation {
    anchor: String,
    frontier: Option<String>,
    cause: M8BindingInvalidationCause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8BindingInvalidationCause {
    AnchorUnavailable,
    LeaseExpired,
}

impl M8BindingInvalidationCause {
    pub const fn audit_subreason(self) -> &'static str {
        match self {
            Self::AnchorUnavailable => "anchor-unavailable",
            Self::LeaseExpired => "lease-expired",
        }
    }
}

impl M8BindingInvalidation {
    pub fn anchor_unavailable(anchor: impl Into<String>) -> Self {
        Self {
            anchor: anchor.into(),
            frontier: None,
            cause: M8BindingInvalidationCause::AnchorUnavailable,
        }
    }

    pub fn lease_expired(anchor: impl Into<String>) -> Self {
        Self {
            anchor: anchor.into(),
            frontier: None,
            cause: M8BindingInvalidationCause::LeaseExpired,
        }
    }

    pub fn with_frontier(mut self, frontier: impl Into<String>) -> Self {
        self.frontier = Some(frontier.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationReacquire {
    anchor: String,
    anchor_epoch: Option<String>,
    binding_epoch: Option<String>,
    fresh_witness: Option<String>,
    fresh_lease_ref: Option<String>,
    frontier: Option<String>,
}

impl M8RelationReacquire {
    pub fn new(anchor: impl Into<String>) -> Self {
        Self {
            anchor: anchor.into(),
            anchor_epoch: None,
            binding_epoch: None,
            fresh_witness: None,
            fresh_lease_ref: None,
            frontier: None,
        }
    }

    pub fn with_anchor_epoch(mut self, anchor_epoch: impl Into<String>) -> Self {
        self.anchor_epoch = Some(anchor_epoch.into());
        self
    }

    pub fn with_binding_epoch(mut self, binding_epoch: impl Into<String>) -> Self {
        self.binding_epoch = Some(binding_epoch.into());
        self
    }

    pub fn with_fresh_witness(mut self, fresh_witness: impl Into<String>) -> Self {
        self.fresh_witness = Some(fresh_witness.into());
        self
    }

    pub fn with_fresh_lease_ref(mut self, fresh_lease_ref: impl Into<String>) -> Self {
        self.fresh_lease_ref = Some(fresh_lease_ref.into());
        self
    }

    pub fn with_frontier(mut self, frontier: impl Into<String>) -> Self {
        self.frontier = Some(frontier.into());
        self
    }

    pub(crate) fn fresh_witness_ref(&self) -> Option<&str> {
        self.fresh_witness.as_deref()
    }

    pub(crate) fn fresh_lease_ref(&self) -> Option<&str> {
        self.fresh_lease_ref.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8ProjectionKind {
    ConsumerLocalPresentation,
    ConsumerLocalFallback,
    OpaqueSemanticTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8ProjectionDiagnosticKind {
    SplitFramePresentationContext,
    StaleAnchorSample,
    MissingAnchorSample,
    UnknownRelation,
    MissingLiveRelationLease,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ProjectionDiagnostic {
    kind: M8ProjectionDiagnosticKind,
    source_ref: SourceRef,
}

impl M8ProjectionDiagnostic {
    pub const fn kind(&self) -> M8ProjectionDiagnosticKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ProjectionDiagnostics {
    entries: Vec<M8ProjectionDiagnostic>,
}

impl M8ProjectionDiagnostics {
    fn one(kind: M8ProjectionDiagnosticKind, source_ref: SourceRef) -> Self {
        Self {
            entries: vec![M8ProjectionDiagnostic { kind, source_ref }],
        }
    }

    pub fn primary(&self) -> &M8ProjectionDiagnostic {
        self.entries
            .first()
            .expect("M8 projection diagnostics always have a primary entry")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8RelationDiagnosticKind {
    MissingRelationAuthority,
    InvalidRelationTransition,
    MissingLiveRelationLease,
    WriteCapabilityUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationDiagnostic {
    kind: M8RelationDiagnosticKind,
    relation: String,
    source_ref: SourceRef,
}

impl M8RelationDiagnostic {
    pub const fn kind(&self) -> M8RelationDiagnosticKind {
        self.kind
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationDiagnostics {
    entries: Vec<M8RelationDiagnostic>,
}

impl M8RelationDiagnostics {
    fn one(kind: M8RelationDiagnosticKind, relation: &str, source_ref: SourceRef) -> Self {
        Self {
            entries: vec![M8RelationDiagnostic {
                kind,
                relation: relation.to_string(),
                source_ref,
            }],
        }
    }

    pub fn primary(&self) -> &M8RelationDiagnostic {
        self.entries
            .first()
            .expect("M8 relation diagnostics always have a primary entry")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8RelationTraceKind {
    SemanticPrimaryInvalidated,
    RelationOptionAdvanced,
    FallbackOptionFrozen,
    SameLineagePrimaryReturnIgnored,
    FreshRelationLineageReacquired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationTraceEntry {
    kind: M8RelationTraceKind,
    relation: String,
    source_ref: SourceRef,
    invalidation_cause: Option<M8BindingInvalidationCause>,
}

impl M8RelationTraceEntry {
    pub const fn kind(&self) -> M8RelationTraceKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub const fn invalidation_cause(&self) -> Option<M8BindingInvalidationCause> {
        self.invalidation_cause
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8RelationTrace {
    entries: Vec<M8RelationTraceEntry>,
}

impl M8RelationTrace {
    pub fn entries(&self) -> &[M8RelationTraceEntry] {
        &self.entries
    }

    pub fn kinds(&self) -> Vec<M8RelationTraceKind> {
        self.entries
            .iter()
            .map(M8RelationTraceEntry::kind)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationTransition {
    previous_option_index: usize,
    current_option_index: usize,
    authority: M8RelationAuthorityUse,
    fresh_reacquire_witness: Option<String>,
    invalidation_cause: Option<M8BindingInvalidationCause>,
}

impl M8RelationTransition {
    pub const fn previous_option_index(&self) -> usize {
        self.previous_option_index
    }

    pub const fn current_option_index(&self) -> usize {
        self.current_option_index
    }

    pub fn authority(&self) -> &M8RelationAuthorityUse {
        &self.authority
    }

    pub fn fresh_reacquire_witness(&self) -> &str {
        self.fresh_reacquire_witness.as_deref().unwrap_or("")
    }

    pub const fn invalidation_cause(&self) -> Option<M8BindingInvalidationCause> {
        self.invalidation_cause
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationProjection {
    relation: String,
    consumer_locus: String,
    kind: M8ProjectionKind,
    subject: String,
    selected_floor: M8RelationFloor,
    selected_anchor: String,
    context_frontier: String,
    anchor_samples: Vec<M8AnchorSample>,
    relative_transform: M8Transform2,
    anchor_pose: Option<M8Point>,
    derived_pose: Option<M8Point>,
    fallback_pose: Option<M8Point>,
    derived_visibility: M8RestrictionPolicy,
    redaction_policy: String,
    absolute_value_stream: Vec<M8Point>,
}

impl M8RelationProjection {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub const fn kind(&self) -> M8ProjectionKind {
        self.kind
    }

    pub const fn consumer_is_semantic_owner(&self) -> bool {
        false
    }

    pub const fn publishes_value(&self) -> bool {
        false
    }

    pub fn absolute_value_stream(&self) -> &[M8Point] {
        &self.absolute_value_stream
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn selected_anchor(&self) -> &str {
        &self.selected_anchor
    }

    pub const fn selected_floor(&self) -> M8RelationFloor {
        self.selected_floor
    }

    pub fn context_frontier(&self) -> &str {
        &self.context_frontier
    }

    pub const fn uses_single_presentation_frame(&self) -> bool {
        true
    }

    pub fn anchor_sample(&self, anchor: &str) -> Option<&M8AnchorSample> {
        self.anchor_samples
            .iter()
            .find(|sample| sample.anchor() == anchor)
    }

    pub fn relative_transform(&self) -> &M8Transform2 {
        &self.relative_transform
    }

    pub const fn anchor_pose(&self) -> Option<M8Point> {
        self.anchor_pose
    }

    pub const fn derived_pose(&self) -> Option<M8Point> {
        self.derived_pose
    }

    pub const fn is_consumer_local_fallback(&self) -> bool {
        matches!(self.kind, M8ProjectionKind::ConsumerLocalFallback)
    }

    pub const fn fallback_pose(&self) -> Option<M8Point> {
        self.fallback_pose
    }

    pub const fn derived_visibility(&self) -> M8RestrictionPolicy {
        self.derived_visibility
    }

    pub fn redaction_policy(&self) -> &str {
        &self.redaction_policy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationProjectionRuntime {
    relation_plans: Vec<M8RelationExecutionPlan>,
    finite_fallback_chains: BTreeMap<String, M8FiniteFallbackChain>,
    presentation_policies: M8PresentationPolicies,
    pub(crate) semantic_snapshot: M8SemanticSnapshot,
    pub(crate) trace: M8RelationTrace,
    live_leases: M8LeaseInventory,
}

impl M8RelationProjectionRuntime {
    pub fn from_admitted(instance: M8RuntimeInstance, seed: M8RelationProjectionSeed) -> Self {
        let relation_plans = instance.relation_execution_plans().to_vec();
        let (presentation_policies, authority_state, live_leases) = seed.into_parts();
        let mut semantic_snapshot = M8SemanticSnapshot::empty_with_authority_state(authority_state);
        for plan in &relation_plans {
            if !plan.has_exact_admission_evidence() {
                continue;
            }
            semantic_snapshot.insert_relation(initial_relation(plan));
        }
        Self {
            relation_plans,
            finite_fallback_chains: BTreeMap::new(),
            presentation_policies,
            semantic_snapshot,
            trace: M8RelationTrace::default(),
            live_leases,
        }
    }

    pub fn semantic_snapshot(&self) -> M8SemanticSnapshot {
        self.semantic_snapshot.clone()
    }

    pub fn semantic_relation(&self, relation: &str) -> &M8SemanticRelation {
        self.semantic_snapshot
            .relation(relation)
            .expect("M8 relation runtime only exposes admitted relation plans")
    }

    pub fn trace(&self) -> &M8RelationTrace {
        &self.trace
    }

    pub(crate) fn replace_admitted_plans(&mut self, instance: &M8RuntimeInstance) {
        self.relation_plans = instance.relation_execution_plans().to_vec();
        for plan in &self.relation_plans {
            if self.semantic_snapshot.relation(plan.name()).is_some() {
                continue;
            }
            self.semantic_snapshot
                .insert_relation(initial_relation(plan));
        }
    }

    pub(crate) fn replace_live_leases(&mut self, live_leases: M8LeaseInventory) {
        self.live_leases = live_leases;
    }

    pub(crate) fn live_lease_inventory(&self) -> M8LeaseInventory {
        self.live_leases.clone()
    }

    pub(crate) fn finite_fallback_chains(&self) -> BTreeMap<String, M8FiniteFallbackChain> {
        self.finite_fallback_chains.clone()
    }

    pub(crate) fn has_finite_fallback_chain(&self, relation: &str) -> bool {
        self.finite_fallback_chains.contains_key(relation)
    }

    pub(crate) fn replace_finite_fallback_chains(
        &mut self,
        chains: BTreeMap<String, M8FiniteFallbackChain>,
    ) {
        self.finite_fallback_chains = chains;
    }

    pub(crate) fn canonical_fallback_configuration_projection(&self) -> String {
        self.finite_fallback_chains
            .values()
            .map(M8FiniteFallbackChain::canonical_projection)
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) fn finite_fallback_selection(
        &self,
        relation: &M8SemanticRelation,
    ) -> Option<M8FiniteFallbackSelection> {
        let option = self
            .finite_fallback_chains
            .get(&relation.name)
            .and_then(|chain| chain.option(relation.selected_option_index()))?;
        if option.target != relation.selected_anchor()
            || option.lease_ref != relation.active_lease_ref()
            || option.epoch != relation.selected_option_epoch()
        {
            return None;
        }
        Some(M8FiniteFallbackSelection {
            option_index: relation.selected_option_index(),
            floor: relation.selected_floor(),
            target: option.target.clone(),
            lease_ref: option.lease_ref.clone(),
            required_capability: option.required_capability.clone(),
            epoch: option.epoch.clone(),
        })
    }

    pub(crate) fn install_finite_fallback_chain(
        &mut self,
        chain: M8FiniteFallbackChain,
    ) -> Result<M8LeaseInventory, M8RelationDiagnostics> {
        let relation = chain.relation().to_string();
        let plan = self.relation_plan_or_diagnostic(&relation)?.clone();
        let current = self.semantic_relation(&relation).clone();
        let Some(live) = chain.option(0) else {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                &relation,
                plan.source_ref().clone(),
            ));
        };
        let Some(anchor) = chain.option(1) else {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                &relation,
                plan.source_ref().clone(),
            ));
        };
        if chain.options.iter().any(|option| {
            option.target.is_empty()
                || option.lease_ref.is_empty()
                || option.required_capability.is_empty()
                || option.epoch.is_empty()
        }) {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                &relation,
                plan.source_ref().clone(),
            ));
        }
        if live.target != plan.core().primary().anchor()
            || live.epoch != plan.core().primary().epoch()
            || anchor.target != plan.core().fallback().anchor()
            || anchor.epoch != plan.core().fallback().epoch()
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                &relation,
                plan.source_ref().clone(),
            ));
        }

        for option in &chain.options {
            self.live_leases.records.insert(
                option.lease_ref.clone(),
                M8LeaseRecord::live(&option.lease_ref)
                    .for_relation(&relation)
                    .with_owner_locus(current.owner_locus())
                    .with_binding_frontier(current.binding_frontier())
                    .with_epoch(current.binding_epoch())
                    .with_anchor_epoch(&option.epoch),
            );
        }
        let relation_state = self
            .semantic_snapshot
            .relation_mut(&relation)
            .expect("admitted relation semantic state exists");
        relation_state.selected_option_index = 0;
        relation_state.selected_floor = M8RelationFloor::Live;
        relation_state.selected_anchor = live.target.clone();
        relation_state.selected_option_epoch = live.epoch.clone();
        relation_state.active_lease_ref = live.lease_ref.clone();
        self.finite_fallback_chains.insert(relation, chain);
        Ok(self.live_leases.clone())
    }

    pub fn project_relation(
        &mut self,
        relation: &str,
        context: M8PresentationContext,
    ) -> Result<M8RelationProjection, M8ProjectionDiagnostics> {
        let plan = self.plan(relation).ok_or_else(|| {
            M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::UnknownRelation,
                SourceRef::new("<m8-relation>", 1, 1, 1, 1),
            )
        })?;
        let semantic = self.semantic_relation(relation);
        if !self.has_live_lease(plan, semantic) {
            return Err(M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::MissingLiveRelationLease,
                plan.source_ref().clone(),
            ));
        }
        let expected_consumer = plan.core().consumer_projection_locus().unwrap_or("");
        if context.consumer_locus != expected_consumer
            || context.frontier() != Some(semantic.activation_frontier())
        {
            return Err(M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::SplitFramePresentationContext,
                plan.source_ref().clone(),
            ));
        }
        if semantic.selected_option_index() == 2 {
            return Ok(M8RelationProjection {
                relation: relation.to_string(),
                consumer_locus: context.consumer_locus,
                kind: M8ProjectionKind::OpaqueSemanticTarget,
                subject: plan.core().subject().to_string(),
                selected_floor: semantic.selected_floor(),
                selected_anchor: semantic.selected_anchor().to_string(),
                context_frontier: semantic.activation_frontier().to_string(),
                anchor_samples: context.anchor_samples,
                relative_transform: M8Transform2::identity(),
                anchor_pose: None,
                derived_pose: None,
                fallback_pose: None,
                derived_visibility: self
                    .presentation_policies
                    .relation_policy(relation)
                    .join(restriction_from_admitted_label(
                        plan.visibility_label().security_class(),
                    ))
                    .join(
                        self.presentation_policies
                            .subject_policy(plan.core().subject()),
                    ),
                redaction_policy: plan.redaction().as_str().to_string(),
                absolute_value_stream: Vec::new(),
            });
        }
        let selected_anchor = semantic.selected_anchor();
        let expected_epoch = semantic.selected_option_epoch();
        let transform = if semantic.selected_option_index() == 0 {
            transform_from_plan(plan.core().primary().transform())
        } else {
            transform_from_plan(plan.core().fallback().transform())
        };

        let Some(sample) = context.sample(selected_anchor) else {
            return self.local_fallback_or_missing(plan, semantic, context);
        };
        if sample.frontier() != semantic.activation_frontier() {
            return Err(M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::SplitFramePresentationContext,
                plan.source_ref().clone(),
            ));
        }
        if sample.epoch() != Some(expected_epoch) {
            return Err(M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::StaleAnchorSample,
                plan.source_ref().clone(),
            ));
        }
        let anchor_pose = sample.pose();
        let derived_pose = anchor_pose.and_then(|pose| pose.translated(&transform));
        let derived_visibility = self
            .presentation_policies
            .relation_policy(relation)
            .join(restriction_from_admitted_label(
                plan.visibility_label().security_class(),
            ))
            .join(
                self.presentation_policies
                    .subject_policy(plan.core().subject()),
            )
            .join(
                sample
                    .policy()
                    .unwrap_or_else(|| self.presentation_policies.anchor_policy(selected_anchor)),
            );
        Ok(M8RelationProjection {
            relation: relation.to_string(),
            consumer_locus: context.consumer_locus,
            kind: M8ProjectionKind::ConsumerLocalPresentation,
            subject: plan.core().subject().to_string(),
            selected_floor: semantic.selected_floor(),
            selected_anchor: selected_anchor.to_string(),
            context_frontier: semantic.activation_frontier().to_string(),
            anchor_samples: context.anchor_samples,
            relative_transform: transform,
            anchor_pose,
            derived_pose,
            fallback_pose: None,
            derived_visibility,
            redaction_policy: plan.redaction().as_str().to_string(),
            absolute_value_stream: Vec::new(),
        })
    }

    pub fn invalidate_primary(
        &mut self,
        relation: &str,
        authority: M8RelationAuthorityUse,
        invalidation: M8BindingInvalidation,
    ) -> Result<M8RelationTransition, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let current = self.semantic_relation(relation).clone();
        if !self.has_live_lease(&plan, &current) {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingLiveRelationLease,
                relation,
                plan.source_ref().clone(),
            ));
        }
        if !self.authority_matches(&plan, &current, &authority, "invalidate_primary") {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingRelationAuthority,
                relation,
                plan.source_ref().clone(),
            ));
        }
        if current.selected_option_index() != 0
            || invalidation.anchor != plan.core().primary().anchor()
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        let next_frontier = invalidation
            .frontier
            .unwrap_or_else(|| current.activation_frontier().to_string());
        let previous_option_index = current.selected_option_index();
        let chain_option = self
            .finite_fallback_chains
            .get(relation)
            .and_then(|chain| chain.option(1))
            .cloned();
        let relation_state = self
            .semantic_snapshot
            .relation_mut(relation)
            .expect("admitted relation semantic state exists");
        relation_state.selected_option_index = 1;
        relation_state.selected_floor = M8RelationFloor::Anchor;
        relation_state.selected_anchor = chain_option
            .as_ref()
            .map(|option| option.target.clone())
            .unwrap_or_else(|| plan.core().fallback().anchor().to_string());
        relation_state.selected_option_epoch = chain_option
            .as_ref()
            .map(|option| option.epoch.clone())
            .unwrap_or_else(|| plan.core().fallback().epoch().to_string());
        if let Some(option) = chain_option {
            relation_state.active_lease_ref = option.lease_ref;
        }
        relation_state.activation_frontier = next_frontier;
        relation_state.lineage.push(format!(
            "{relation}:advance:{}",
            relation_state.lineage.len()
        ));
        if invalidation.cause == M8BindingInvalidationCause::LeaseExpired
            && let Some(lease) = self.live_leases.records.get_mut(current.active_lease_ref())
        {
            lease.live = false;
        }
        self.append_trace(
            M8RelationTraceKind::SemanticPrimaryInvalidated,
            &plan,
            Some(invalidation.cause),
        );
        self.append_trace(M8RelationTraceKind::RelationOptionAdvanced, &plan, None);
        Ok(M8RelationTransition {
            previous_option_index,
            current_option_index: 1,
            authority,
            fresh_reacquire_witness: None,
            invalidation_cause: Some(invalidation.cause),
        })
    }

    /// Freeze the currently selected fallback inside the M8-owned relation
    /// snapshot.  This is monotone: a fresh M8 reacquire is the only path
    /// that can return the relation to its live primary floor.
    pub fn advance_anchor_to_frozen(
        &mut self,
        relation: &str,
        prior_transition: &M8RelationTransition,
    ) -> Result<M8RelationTransition, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let current = self.semantic_relation(relation).clone();
        if !self.has_live_lease(&plan, &current) {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingLiveRelationLease,
                relation,
                plan.source_ref().clone(),
            ));
        }
        if current.selected_option_index() != 1
            || current.selected_floor() != M8RelationFloor::Anchor
            || prior_transition.previous_option_index() != 0
            || prior_transition.current_option_index() != 1
            || prior_transition.invalidation_cause()
                != Some(M8BindingInvalidationCause::LeaseExpired)
            || !self.authority_matches(
                &plan,
                &current,
                prior_transition.authority(),
                "invalidate_primary",
            )
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        let frozen = self
            .finite_fallback_chains
            .get(relation)
            .and_then(|chain| chain.option(2))
            .cloned()
            .ok_or_else(|| {
                M8RelationDiagnostics::one(
                    M8RelationDiagnosticKind::InvalidRelationTransition,
                    relation,
                    plan.source_ref().clone(),
                )
            })?;
        let relation_state = self
            .semantic_snapshot
            .relation_mut(relation)
            .expect("admitted relation semantic state exists");
        relation_state.selected_option_index = 2;
        relation_state.selected_floor = M8RelationFloor::Frozen;
        relation_state.selected_anchor = frozen.target;
        relation_state.selected_option_epoch = frozen.epoch;
        relation_state.active_lease_ref = frozen.lease_ref;
        relation_state.lineage.push(format!(
            "{relation}:freeze:{}",
            relation_state.lineage.len()
        ));
        if let Some(lease) = self.live_leases.records.get_mut(current.active_lease_ref()) {
            lease.live = false;
        }
        self.append_trace(
            M8RelationTraceKind::FallbackOptionFrozen,
            &plan,
            Some(M8BindingInvalidationCause::LeaseExpired),
        );
        Ok(M8RelationTransition {
            previous_option_index: current.selected_option_index(),
            current_option_index: 2,
            authority: M8RelationAuthorityUse::for_relation(relation),
            fresh_reacquire_witness: None,
            invalidation_cause: Some(M8BindingInvalidationCause::LeaseExpired),
        })
    }

    pub fn note_primary_available_same_lineage(
        &mut self,
        relation: &str,
        anchor: &str,
    ) -> Result<M8RelationTransition, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let current = self.semantic_relation(relation).clone();
        if !self.has_live_lease(&plan, &current) {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingLiveRelationLease,
                relation,
                plan.source_ref().clone(),
            ));
        }
        if anchor != plan.core().primary().anchor() {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        self.append_trace(
            M8RelationTraceKind::SameLineagePrimaryReturnIgnored,
            &plan,
            None,
        );
        Ok(M8RelationTransition {
            previous_option_index: current.selected_option_index(),
            current_option_index: current.selected_option_index(),
            authority: M8RelationAuthorityUse::for_relation(relation),
            fresh_reacquire_witness: None,
            invalidation_cause: None,
        })
    }

    /// The finite fallback carrier is read-only.  A post-selection write must
    /// be rejected from the selected M8 option rather than inferred by an
    /// outer schedule layer.
    pub fn request_selected_option_write(
        &mut self,
        relation: &str,
    ) -> Result<(), M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let current = self.semantic_relation(relation).clone();
        if self.finite_fallback_selection(&current).is_none() {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        Err(M8RelationDiagnostics::one(
            M8RelationDiagnosticKind::WriteCapabilityUnavailable,
            relation,
            plan.source_ref().clone(),
        ))
    }

    pub fn reacquire_primary(
        &mut self,
        relation: &str,
        authority: M8RelationAuthorityUse,
        reacquire: M8RelationReacquire,
    ) -> Result<M8RelationTransition, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let current = self.semantic_relation(relation).clone();
        if !self.has_live_lease(&plan, &current) {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingLiveRelationLease,
                relation,
                plan.source_ref().clone(),
            ));
        }
        let Some(binding_epoch) = reacquire.binding_epoch.as_deref() else {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        };
        let Some(anchor_epoch) = reacquire.anchor_epoch.as_deref() else {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingRelationAuthority,
                relation,
                plan.source_ref().clone(),
            ));
        };
        let Some(binding_frontier) = reacquire.frontier.as_deref() else {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingRelationAuthority,
                relation,
                plan.source_ref().clone(),
            ));
        };
        let Some(fresh_lease_ref) = reacquire.fresh_lease_ref() else {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingRelationAuthority,
                relation,
                plan.source_ref().clone(),
            ));
        };
        let valid_reacquire = reacquire.anchor == plan.core().primary().anchor()
            && reacquire
                .fresh_witness
                .as_deref()
                .is_some_and(|witness| !witness.is_empty())
            && anchor_epoch != current.primary_epoch()
            && binding_epoch != current.binding_epoch()
            && self.authority_matches(&plan, &current, &authority, "reacquire_primary")
            && self.fresh_witness_matches(&authority, reacquire.fresh_witness.as_deref())
            && authority.binding_epoch.as_deref() == Some(binding_epoch)
            && self.live_leases.contains_live_fresh_reacquire(
                fresh_lease_ref,
                plan.name(),
                current.owner_locus(),
                binding_frontier,
                binding_epoch,
                anchor_epoch,
            );
        if !valid_reacquire {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingRelationAuthority,
                relation,
                plan.source_ref().clone(),
            ));
        }
        let previous_option_index = current.selected_option_index();
        let relation_state = self
            .semantic_snapshot
            .relation_mut(relation)
            .expect("admitted relation semantic state exists");
        relation_state.selected_option_index = 0;
        relation_state.selected_floor = M8RelationFloor::Live;
        relation_state.selected_anchor = plan.core().primary().anchor().to_string();
        relation_state.selected_option_epoch = anchor_epoch.to_string();
        relation_state.primary_epoch = anchor_epoch.to_string();
        relation_state.binding_epoch = binding_epoch.to_string();
        relation_state.binding_frontier = binding_frontier.to_string();
        relation_state.active_lease_ref = fresh_lease_ref.to_string();
        relation_state.activation_frontier = binding_frontier.to_string();
        relation_state.lineage = vec![format!("{relation}:lineage:{binding_epoch}")];
        self.append_trace(
            M8RelationTraceKind::FreshRelationLineageReacquired,
            &plan,
            None,
        );
        Ok(M8RelationTransition {
            previous_option_index,
            current_option_index: 0,
            authority,
            fresh_reacquire_witness: reacquire.fresh_witness,
            invalidation_cause: None,
        })
    }

    fn local_fallback_or_missing(
        &self,
        plan: &M8RelationExecutionPlan,
        semantic: &M8SemanticRelation,
        context: M8PresentationContext,
    ) -> Result<M8RelationProjection, M8ProjectionDiagnostics> {
        let Some(fallback) = context.fallback else {
            return Err(M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::MissingAnchorSample,
                plan.source_ref().clone(),
            ));
        };
        let derived_visibility = self
            .presentation_policies
            .relation_policy(plan.name())
            .join(restriction_from_admitted_label(
                plan.visibility_label().security_class(),
            ))
            .join(
                self.presentation_policies
                    .subject_policy(plan.core().subject()),
            )
            .join(fallback.policy.unwrap_or(M8RestrictionPolicy::Restricted));
        Ok(M8RelationProjection {
            relation: plan.name().to_string(),
            consumer_locus: context.consumer_locus,
            kind: M8ProjectionKind::ConsumerLocalFallback,
            subject: fallback.subject,
            selected_floor: semantic.selected_floor(),
            selected_anchor: semantic.selected_anchor().to_string(),
            context_frontier: semantic.activation_frontier().to_string(),
            anchor_samples: context.anchor_samples,
            relative_transform: M8Transform2::identity(),
            anchor_pose: None,
            derived_pose: None,
            fallback_pose: Some(fallback.pose),
            derived_visibility,
            redaction_policy: plan.redaction().as_str().to_string(),
            absolute_value_stream: Vec::new(),
        })
    }

    fn plan(&self, relation: &str) -> Option<&M8RelationExecutionPlan> {
        self.relation_plans
            .iter()
            .find(|plan| plan.name() == relation && plan.has_exact_admission_evidence())
    }

    fn relation_plan_or_diagnostic(
        &self,
        relation: &str,
    ) -> Result<&M8RelationExecutionPlan, M8RelationDiagnostics> {
        self.plan(relation).ok_or_else(|| {
            M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                SourceRef::new("<m8-relation>", 1, 1, 1, 1),
            )
        })
    }

    fn authority_matches(
        &self,
        plan: &M8RelationExecutionPlan,
        relation: &M8SemanticRelation,
        authority: &M8RelationAuthorityUse,
        transition: &str,
    ) -> bool {
        authority.relation == plan.name()
            && authority.owner_locus.as_deref() == Some(plan.core().owner_locus())
            && authority.transition.as_deref() == Some(transition)
            && authority.principal.is_some()
            && self
                .semantic_snapshot
                .authority_state()
                .validates_relation_use(M8RelationAuthorityLookup {
                    relation: plan.name(),
                    transition,
                    owner_locus: plan.core().owner_locus(),
                    principal: authority.principal.as_deref().unwrap_or(""),
                    membership_ref: authority.membership_ref.as_deref(),
                    capability_ref: authority.capability_ref.as_deref(),
                    membership_epoch: authority.membership_epoch.as_deref(),
                    binding_epoch: authority.binding_epoch.as_deref(),
                    witness_ref: authority.witness_ref.as_deref(),
                    witness_epoch: authority.witness_epoch.as_deref(),
                })
            && ((transition == "invalidate_primary"
                && authority.binding_epoch.as_deref() == Some(relation.binding_epoch()))
                || transition == "reacquire_primary")
    }

    fn fresh_witness_matches(
        &self,
        authority: &M8RelationAuthorityUse,
        fresh_witness: Option<&str>,
    ) -> bool {
        fresh_witness.is_some()
            && fresh_witness == authority.witness_ref()
            && self
                .semantic_snapshot
                .authority_state()
                .contains_witness(fresh_witness.unwrap_or(""))
    }

    fn has_live_lease(
        &self,
        plan: &M8RelationExecutionPlan,
        relation: &M8SemanticRelation,
    ) -> bool {
        self.live_leases.contains_live_exact_binding(
            relation.active_lease_ref(),
            plan.name(),
            relation.owner_locus(),
            relation.binding_frontier(),
            relation.binding_epoch(),
        )
    }

    fn append_trace(
        &mut self,
        kind: M8RelationTraceKind,
        plan: &M8RelationExecutionPlan,
        invalidation_cause: Option<M8BindingInvalidationCause>,
    ) {
        self.trace.entries.push(M8RelationTraceEntry {
            kind,
            relation: plan.name().to_string(),
            source_ref: plan.source_ref().clone(),
            invalidation_cause,
        });
    }
}

fn initial_relation(plan: &M8RelationExecutionPlan) -> M8SemanticRelation {
    let core = plan.core();
    let frontier = core
        .binding_frontier()
        .as_slice()
        .first()
        .expect("M7 relation binding frontier is finite and nonempty")
        .as_str();
    M8SemanticRelation::from_initial_state(M8SemanticRelationInitialState {
        name: plan.name().to_string(),
        owner_locus: core.owner_locus().to_string(),
        selected_option_index: 0,
        selected_anchor: core.primary().anchor().to_string(),
        primary_epoch: core.primary().epoch().to_string(),
        binding_epoch: "binding_epoch:1".to_string(),
        binding_frontier: plan.binding_frontier().to_string(),
        active_lease_ref: plan.live_lease_ref().to_string(),
        activation_frontier: frontier.to_string(),
        lineage: vec![format!("{}:lineage:binding_epoch:1", plan.name())],
    })
}

fn restriction_from_admitted_label(label: M8SecurityClass) -> M8RestrictionPolicy {
    match label {
        M8SecurityClass::Public => M8RestrictionPolicy::Public,
        M8SecurityClass::Restricted => M8RestrictionPolicy::Restricted,
        M8SecurityClass::Private => M8RestrictionPolicy::Private,
    }
}

fn transform_from_plan(
    transform: &mir_semantics::surface_v0_pipeline::RelationTransformCore,
) -> M8Transform2 {
    match transform.translation() {
        Some((x, y)) => M8Transform2::translate(x, y),
        None => M8Transform2::identity(),
    }
}
