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
        M8SemanticRelation, M8SemanticRelationInitialState, M8SemanticSnapshot,
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
}

impl M8BindingInvalidation {
    pub fn anchor_unavailable(anchor: impl Into<String>) -> Self {
        Self {
            anchor: anchor.into(),
            frontier: None,
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
    SameLineagePrimaryReturnIgnored,
    FreshRelationLineageReacquired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationTraceEntry {
    kind: M8RelationTraceKind,
    relation: String,
    source_ref: SourceRef,
}

impl M8RelationTraceEntry {
    pub const fn kind(&self) -> M8RelationTraceKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8RelationProjection {
    relation: String,
    consumer_locus: String,
    kind: M8ProjectionKind,
    subject: String,
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
        let selected_anchor = semantic.selected_anchor();
        let expected_epoch = if semantic.selected_option_index() == 0 {
            semantic.primary_epoch()
        } else {
            plan.core().fallback().epoch()
        };
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
        let relation_state = self
            .semantic_snapshot
            .relation_mut(relation)
            .expect("admitted relation semantic state exists");
        relation_state.selected_option_index = 1;
        relation_state.selected_anchor = plan.core().fallback().anchor().to_string();
        relation_state.activation_frontier = next_frontier;
        relation_state.lineage.push(format!(
            "{relation}:advance:{}",
            relation_state.lineage.len()
        ));
        self.append_trace(M8RelationTraceKind::SemanticPrimaryInvalidated, &plan);
        self.append_trace(M8RelationTraceKind::RelationOptionAdvanced, &plan);
        Ok(M8RelationTransition {
            previous_option_index,
            current_option_index: 1,
            authority,
            fresh_reacquire_witness: None,
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
        self.append_trace(M8RelationTraceKind::SameLineagePrimaryReturnIgnored, &plan);
        Ok(M8RelationTransition {
            previous_option_index: current.selected_option_index(),
            current_option_index: current.selected_option_index(),
            authority: M8RelationAuthorityUse::for_relation(relation),
            fresh_reacquire_witness: None,
        })
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
        relation_state.selected_anchor = plan.core().primary().anchor().to_string();
        relation_state.primary_epoch = anchor_epoch.to_string();
        relation_state.binding_epoch = binding_epoch.to_string();
        relation_state.binding_frontier = binding_frontier.to_string();
        relation_state.active_lease_ref = fresh_lease_ref.to_string();
        relation_state.activation_frontier = binding_frontier.to_string();
        relation_state.lineage = vec![format!("{relation}:lineage:{binding_epoch}")];
        self.append_trace(M8RelationTraceKind::FreshRelationLineageReacquired, &plan);
        Ok(M8RelationTransition {
            previous_option_index,
            current_option_index: 0,
            authority,
            fresh_reacquire_witness: reacquire.fresh_witness,
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

    fn append_trace(&mut self, kind: M8RelationTraceKind, plan: &M8RelationExecutionPlan) {
        self.trace.entries.push(M8RelationTraceEntry {
            kind,
            relation: plan.name().to_string(),
            source_ref: plan.source_ref().clone(),
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
