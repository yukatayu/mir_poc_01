//! Bounded M8 maintained-relation semantics and consumer-local projection.
//!
//! The runtime consumes admitted checked relation Core only.  Its semantic
//! relation state is stored in the same `M8SemanticSnapshot` shape used by the
//! owner queue, while presentation contexts remain ephemeral inputs.

use std::collections::BTreeMap;

use mir_semantics::shared_model::SourceRef;
use serde::{Deserialize, Serialize};

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

    /// Deliberately corrupt one saved lease clone for the bounded M10
    /// save/load negative.  Normal relation execution has no route to this
    /// mutator; restore must reject the resulting clone against the live
    /// lease floor before it can affect a runtime.
    pub(crate) fn doctor_expired_lease_as_live(&mut self, reference: &str) -> bool {
        let Some(record) = self.records.get_mut(reference) else {
            return false;
        };
        record.live = true;
        true
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

/// Exact private snapshot of a sealed relation authority use.  It has no
/// route, issuer, or capability minting path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct M8I3PrivateRelationAuthorityUseSnapshot {
    relation: String,
    owner_locus: Option<String>,
    transition: Option<String>,
    principal: Option<String>,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
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

    pub(crate) fn i3_private_snapshot(&self) -> M8I3PrivateRelationAuthorityUseSnapshot {
        M8I3PrivateRelationAuthorityUseSnapshot {
            relation: self.relation.clone(),
            owner_locus: self.owner_locus.clone(),
            transition: self.transition.clone(),
            principal: self.principal.clone(),
            membership_ref: self.membership_ref.clone(),
            capability_ref: self.capability_ref.clone(),
            membership_epoch: self.membership_epoch.clone(),
            binding_epoch: self.binding_epoch.clone(),
            witness_ref: self.witness_ref.clone(),
            witness_epoch: self.witness_epoch.clone(),
        }
    }

    pub(crate) fn from_i3_private_snapshot(
        snapshot: M8I3PrivateRelationAuthorityUseSnapshot,
    ) -> Self {
        Self {
            relation: snapshot.relation,
            owner_locus: snapshot.owner_locus,
            transition: snapshot.transition,
            principal: snapshot.principal,
            membership_ref: snapshot.membership_ref,
            capability_ref: snapshot.capability_ref,
            membership_epoch: snapshot.membership_epoch,
            binding_epoch: snapshot.binding_epoch,
            witness_ref: snapshot.witness_ref,
            witness_epoch: snapshot.witness_epoch,
        }
    }

    /// Validate an already-issued relation use against the live, sealed M8
    /// authority inventory.  This is a checker only: it neither creates a
    /// capability nor treats a carrier route as authority.
    pub(crate) fn validates_admitted_relation_use(
        &self,
        authority_state: &M8AuthorityState,
        relation: &str,
        owner_locus: &str,
        transition: &str,
    ) -> bool {
        self.relation == relation
            && self.owner_locus.as_deref() == Some(owner_locus)
            && self.transition.as_deref() == Some(transition)
            && self.principal.is_some()
            && authority_state.validates_relation_use(M8RelationAuthorityLookup {
                relation,
                transition,
                owner_locus,
                principal: self.principal.as_deref().unwrap_or(""),
                membership_ref: self.membership_ref.as_deref(),
                capability_ref: self.capability_ref.as_deref(),
                membership_epoch: self.membership_epoch.as_deref(),
                binding_epoch: self.binding_epoch.as_deref(),
                witness_ref: self.witness_ref.as_deref(),
                witness_epoch: self.witness_epoch.as_deref(),
            })
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
    /// The owner has prepared an immutable current-state publication.  Its
    /// occurrence is later committed only after the generated consumer
    /// endpoint admits the publication.
    SemanticRelationPublished,
    /// A consumer-local session imported an owner publication after its
    /// generated endpoint and M9 target admission both validated it.
    ConsumerRelationPublicationObserved,
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

/// Immutable semantic relation publication produced only by the relation
/// owner.  It deliberately carries the selected semantic relation snapshot,
/// not anchor samples, credentials, or presentation output.  A consumer may
/// import it only through the exact relation plan it already admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8PublishedRelationState {
    program_identity: String,
    relation: String,
    owner_locus: String,
    source_ref: SourceRef,
    core_ref: String,
    publication_occurrence: u64,
    predecessor_occurrence: Option<u64>,
    owner_publish_occurrence_id: Option<String>,
    semantic: M8SemanticRelation,
}

impl M8PublishedRelationState {
    pub(crate) fn relation(&self) -> &str {
        &self.relation
    }

    pub(crate) fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub(crate) const fn publication_occurrence(&self) -> u64 {
        self.publication_occurrence
    }

    pub(crate) const fn predecessor_occurrence(&self) -> Option<u64> {
        self.predecessor_occurrence
    }

    pub(crate) fn owner_publish_occurrence_id(&self) -> Option<&str> {
        self.owner_publish_occurrence_id.as_deref()
    }

    pub(crate) fn with_owner_publish_occurrence_id(
        mut self,
        occurrence: impl Into<String>,
    ) -> Self {
        self.owner_publish_occurrence_id = Some(occurrence.into());
        self
    }

    pub(crate) fn semantic(&self) -> &M8SemanticRelation {
        &self.semantic
    }
}

/// Consumer-local shadow imported from an immutable owner publication.  This
/// is not a semantic owner replica: it exists solely to drive a consumer
/// projection after the generated endpoint receives a valid publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct M8ObservedRelationShadow {
    relation: String,
    owner_locus: String,
    consumer_locus: String,
    source_ref: SourceRef,
    core_ref: String,
    publication_occurrence: u64,
    consumer_observe_occurrence_id: Option<String>,
    semantic: M8SemanticRelation,
}

impl M8ObservedRelationShadow {
    pub(crate) fn relation(&self) -> &str {
        &self.relation
    }

    pub(crate) fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub(crate) fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub(crate) const fn publication_occurrence(&self) -> u64 {
        self.publication_occurrence
    }

    pub(crate) fn consumer_observe_occurrence_id(&self) -> Option<&str> {
        self.consumer_observe_occurrence_id.as_deref()
    }

    pub(crate) fn with_consumer_observe_occurrence_id(
        mut self,
        occurrence: impl Into<String>,
    ) -> Self {
        self.consumer_observe_occurrence_id = Some(occurrence.into());
        self
    }

    pub(crate) fn semantic(&self) -> &M8SemanticRelation {
        &self.semantic
    }

    pub(crate) fn semantic_digest(&self) -> String {
        relation_semantic_digest(&self.semantic)
    }
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
    program_identity: String,
    relation_plans: Vec<M8RelationExecutionPlan>,
    finite_fallback_chains: BTreeMap<String, M8FiniteFallbackChain>,
    presentation_policies: M8PresentationPolicies,
    pub(crate) semantic_snapshot: M8SemanticSnapshot,
    pub(crate) trace: M8RelationTrace,
    live_leases: M8LeaseInventory,
    /// Owner-only monotone relation-publication sequence.  This is distinct
    /// from the semantic binding epoch: it detects duplicate/stale endpoint
    /// deliveries without changing the relation's meaning.
    published_occurrences: BTreeMap<String, u64>,
    /// Imported observer state is intentionally separate from
    /// `semantic_snapshot`, which contains the admitted plan's boot-time
    /// relation material.  Consumers must render this imported shadow, never
    /// their local clone of the global semantic snapshot.
    observed_shadows: BTreeMap<String, M8ObservedRelationShadow>,
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
            program_identity: instance.program_identity().stable_key(),
            relation_plans,
            finite_fallback_chains: BTreeMap::new(),
            presentation_policies,
            semantic_snapshot,
            trace: M8RelationTrace::default(),
            live_leases,
            published_occurrences: BTreeMap::new(),
            observed_shadows: BTreeMap::new(),
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

    /// Publish the current owner semantic state.  Only the owner session may
    /// call this; source/Core provenance is read from the admitted relation
    /// plan and cannot be supplied by a caller.
    pub(crate) fn publish_semantic_relation(
        &mut self,
        relation: &str,
        owner_locus: &str,
        authority: M8RelationAuthorityUse,
    ) -> Result<M8PublishedRelationState, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let semantic = self.semantic_relation(relation).clone();
        if semantic.owner_locus() != owner_locus || plan.core().owner_locus() != owner_locus {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        if !self.authority_matches(&plan, &semantic, &authority, "publish_relation") {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::MissingRelationAuthority,
                relation,
                plan.source_ref().clone(),
            ));
        }
        if !self.has_live_lease(&plan, &semantic) {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        let next = match self.published_occurrences.get(relation).copied() {
            Some(previous) => previous.checked_add(1).ok_or_else(|| {
                M8RelationDiagnostics::one(
                    M8RelationDiagnosticKind::InvalidRelationTransition,
                    relation,
                    plan.source_ref().clone(),
                )
            })?,
            None => 0,
        };
        let predecessor = self.published_occurrences.get(relation).copied();
        self.append_trace(M8RelationTraceKind::SemanticRelationPublished, &plan, None);
        Ok(M8PublishedRelationState {
            program_identity: self.program_identity.clone(),
            relation: relation.to_string(),
            owner_locus: owner_locus.to_string(),
            source_ref: plan.source_ref().clone(),
            core_ref: relation_core_ref(&plan),
            publication_occurrence: next,
            predecessor_occurrence: predecessor,
            owner_publish_occurrence_id: None,
            semantic,
        })
    }

    /// Commit a publication sequence only after the generated consumer
    /// endpoint imported the exact immutable state.  A route, inbox, or
    /// target-admission failure therefore leaves the next occurrence
    /// reusable for a retry rather than creating a permanent sequence gap.
    pub(crate) fn commit_semantic_relation_publication(
        &mut self,
        publication: &M8PublishedRelationState,
    ) -> Result<(), M8RelationDiagnostics> {
        let relation = publication.relation();
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let current = self.semantic_relation(relation);
        let expected = match self.published_occurrences.get(relation).copied() {
            Some(previous) => previous.checked_add(1).ok_or_else(|| {
                M8RelationDiagnostics::one(
                    M8RelationDiagnosticKind::InvalidRelationTransition,
                    relation,
                    plan.source_ref().clone(),
                )
            })?,
            None => 0,
        };
        let predecessor = self.published_occurrences.get(relation).copied();
        if publication.program_identity != self.program_identity
            || publication.owner_locus() != plan.core().owner_locus()
            || publication.source_ref() != plan.source_ref()
            || publication.core_ref() != relation_core_ref(&plan)
            || publication.publication_occurrence() != expected
            || publication.predecessor_occurrence() != predecessor
            || publication.semantic() != current
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        self.published_occurrences
            .insert(relation.to_string(), expected);
        Ok(())
    }

    /// Import an immutable relation publication at the consumer endpoint.
    /// It rejects a foreign program/plan, provenance mismatch, duplicate,
    /// stale, and out-of-order occurrence before replacing the local shadow.
    pub(crate) fn import_semantic_relation_shadow(
        &mut self,
        consumer_locus: &str,
        publication: M8PublishedRelationState,
    ) -> Result<M8ObservedRelationShadow, M8RelationDiagnostics> {
        let relation = publication.relation().to_string();
        let plan = self.relation_plan_or_diagnostic(&relation)?.clone();
        let expected_consumer = plan.core().consumer_projection_locus().unwrap_or("");
        let provenance_matches = publication.program_identity == self.program_identity
            && publication.owner_locus() == plan.core().owner_locus()
            && publication.source_ref() == plan.source_ref()
            && publication.core_ref() == relation_core_ref(&plan)
            && publication.semantic().name == relation
            && publication.semantic().owner_locus() == plan.core().owner_locus();
        if consumer_locus != expected_consumer || !provenance_matches {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                &relation,
                plan.source_ref().clone(),
            ));
        }
        if let Some(previous) = self.observed_shadows.get(&relation) {
            let expected = previous
                .publication_occurrence()
                .checked_add(1)
                .ok_or_else(|| {
                    M8RelationDiagnostics::one(
                        M8RelationDiagnosticKind::InvalidRelationTransition,
                        &relation,
                        plan.source_ref().clone(),
                    )
                })?;
            if publication.publication_occurrence() != expected
                || publication.predecessor_occurrence() != Some(previous.publication_occurrence)
            {
                return Err(M8RelationDiagnostics::one(
                    M8RelationDiagnosticKind::InvalidRelationTransition,
                    &relation,
                    plan.source_ref().clone(),
                ));
            }
        } else if publication.publication_occurrence() != 0
            || publication.predecessor_occurrence().is_some()
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                &relation,
                plan.source_ref().clone(),
            ));
        }
        let shadow = M8ObservedRelationShadow {
            relation: relation.clone(),
            owner_locus: publication.owner_locus().to_string(),
            consumer_locus: consumer_locus.to_string(),
            source_ref: publication.source_ref().clone(),
            core_ref: publication.core_ref().to_string(),
            publication_occurrence: publication.publication_occurrence(),
            consumer_observe_occurrence_id: None,
            semantic: publication.semantic().clone(),
        };
        self.observed_shadows.insert(relation, shadow.clone());
        self.append_trace(
            M8RelationTraceKind::ConsumerRelationPublicationObserved,
            &plan,
            None,
        );
        Ok(shadow)
    }

    /// Replace the consumer-local raw M8 observe occurrence with the exact
    /// fabric-qualified occurrence once SYS-4 has bound it to the generated
    /// endpoint.  The update is constrained to the shadow just imported by
    /// that M8 session; an arbitrary carrier cannot annotate another shadow.
    pub(crate) fn qualify_observed_relation_shadow_occurrence(
        &mut self,
        shadow: &M8ObservedRelationShadow,
        qualified_occurrence: &str,
    ) -> Result<M8ObservedRelationShadow, M8RelationDiagnostics> {
        let relation = shadow.relation();
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let stored = self.observed_shadows.get_mut(relation).ok_or_else(|| {
            M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            )
        })?;
        if stored.relation != shadow.relation
            || stored.owner_locus != shadow.owner_locus
            || stored.consumer_locus != shadow.consumer_locus
            || stored.source_ref != shadow.source_ref
            || stored.core_ref != shadow.core_ref
            || stored.publication_occurrence != shadow.publication_occurrence
            || stored.semantic != shadow.semantic
            || (stored.consumer_observe_occurrence_id.is_some()
                && stored.consumer_observe_occurrence_id != shadow.consumer_observe_occurrence_id)
            || shadow.consumer_observe_occurrence_id.is_none()
            || qualified_occurrence.is_empty()
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        stored.consumer_observe_occurrence_id = Some(qualified_occurrence.to_string());
        Ok(stored.clone())
    }

    pub(crate) fn observed_relation_shadow(
        &self,
        relation: &str,
        consumer_locus: &str,
    ) -> Option<&M8ObservedRelationShadow> {
        self.observed_shadows
            .get(relation)
            .filter(|shadow| shadow.consumer_locus() == consumer_locus)
    }

    pub(crate) fn publication_state(
        &self,
    ) -> (
        BTreeMap<String, u64>,
        BTreeMap<String, M8ObservedRelationShadow>,
    ) {
        (
            self.published_occurrences.clone(),
            self.observed_shadows.clone(),
        )
    }

    pub(crate) fn replace_publication_state(
        &mut self,
        published_occurrences: BTreeMap<String, u64>,
        observed_shadows: BTreeMap<String, M8ObservedRelationShadow>,
    ) {
        self.published_occurrences = published_occurrences;
        self.observed_shadows = observed_shadows;
    }

    /// Fresh reacquisition is allowed only after the primary binding has
    /// been semantically invalidated to a fallback/frozen floor.
    pub(crate) fn requires_fresh_reacquire(
        &self,
        relation: &str,
    ) -> Result<bool, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?;
        let semantic = self.semantic_relation(relation);
        Ok(semantic.selected_floor() != M8RelationFloor::Live
            && semantic.selected_anchor() != plan.core().primary().anchor())
    }

    /// Project only an imported consumer shadow.  The live owner semantic
    /// snapshot is deliberately not consulted, so a consumer cannot render a
    /// boot-time clone when the endpoint has not delivered the current owner
    /// publication.  This path is used for the local presentation-gap case.
    pub(crate) fn project_observed_relation_shadow(
        &self,
        relation: &str,
        context: M8PresentationContext,
    ) -> Result<M8RelationProjection, M8ProjectionDiagnostics> {
        let plan = self.plan(relation).ok_or_else(|| {
            M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::UnknownRelation,
                SourceRef::new("<m8-relation>", 1, 1, 1, 1),
            )
        })?;
        let shadow = self
            .observed_relation_shadow(relation, &context.consumer_locus)
            .ok_or_else(|| {
                M8ProjectionDiagnostics::one(
                    M8ProjectionDiagnosticKind::UnknownRelation,
                    plan.source_ref().clone(),
                )
            })?;
        if context.consumer_locus != plan.core().consumer_projection_locus().unwrap_or("")
            || context.frontier() != Some(shadow.semantic().activation_frontier())
        {
            return Err(M8ProjectionDiagnostics::one(
                M8ProjectionDiagnosticKind::SplitFramePresentationContext,
                plan.source_ref().clone(),
            ));
        }
        self.local_fallback_or_missing(plan, shadow.semantic(), context)
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

    /// Install the accepted finite local fallback chain entirely from the
    /// admitted relation plan and its M9-derived bootstrap lease.  No SYS-5
    /// schedule value supplies an anchor, epoch, lease, or capability here.
    pub(crate) fn install_finite_local_bootstrap_chain(
        &mut self,
        relation: &str,
    ) -> Result<M8LeaseInventory, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        let bootstrap = plan.live_lease_ref().to_string();
        let chain = M8FiniteFallbackChain::live_anchor_frozen(
            relation,
            M8FiniteFallbackOption::new(
                plan.core().primary().anchor(),
                bootstrap.clone(),
                format!("{bootstrap}:primary-capability"),
                plan.core().primary().epoch(),
            ),
            M8FiniteFallbackOption::new(
                plan.core().fallback().anchor(),
                format!("{bootstrap}:fallback-lease"),
                format!("{bootstrap}:fallback-capability"),
                plan.core().fallback().epoch(),
            ),
            M8FiniteFallbackOption::new(
                plan.core().fallback().anchor(),
                format!("{bootstrap}:frozen-lease"),
                format!("{bootstrap}:frozen-capability"),
                plan.core().fallback().epoch(),
            ),
        );
        self.install_finite_fallback_chain(chain)
    }

    /// Add the already M9-sealed fresh lease required by a single
    /// reacquisition.  The caller receives this `M8LeaseRecord` only from an
    /// opaque M9 binding, so this method cannot issue a relation lease.
    pub(crate) fn install_sealed_fresh_relation_lease(
        &mut self,
        relation: &str,
        lease: M8LeaseRecord,
    ) -> Result<M8LeaseInventory, M8RelationDiagnostics> {
        let plan = self.relation_plan_or_diagnostic(relation)?.clone();
        if lease.relation.as_deref() != Some(relation)
            || lease.owner_locus.as_deref() != Some(plan.core().owner_locus())
            || lease.binding_frontier.as_deref() != Some(plan.binding_frontier())
            || lease.epoch.as_deref() != Some("binding_epoch:2")
            || lease.anchor_epoch.as_deref() == Some(plan.core().primary().epoch())
            || !lease.live
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
                relation,
                plan.source_ref().clone(),
            ));
        }
        self.live_leases
            .records
            .insert(lease.reference.clone(), lease);
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
        if current.selected_floor() != M8RelationFloor::Live
            || current.selected_anchor() != plan.core().primary().anchor()
        {
            return Err(M8RelationDiagnostics::one(
                M8RelationDiagnosticKind::InvalidRelationTransition,
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
        authority.validates_admitted_relation_use(
            self.semantic_snapshot.authority_state(),
            plan.name(),
            plan.core().owner_locus(),
            transition,
        ) && match transition {
            // Invalidation changes the current binding, so it must name
            // the exact binding being invalidated.
            "invalidate_primary" => {
                authority.binding_epoch.as_deref() == Some(relation.binding_epoch())
            }
            // Fresh reacquisition separately proves that it changes to a
            // new binding/lease below.  Publication makes no semantic
            // transition: its independent, admitted M9 publish use is
            // checked above and remains valid for the current relation
            // lineage (including a freshly reacquired one).
            "reacquire_primary" | "publish_relation" => true,
            _ => false,
        }
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

fn relation_core_ref(plan: &M8RelationExecutionPlan) -> String {
    let core = plan.core();
    format!(
        "m8-relation-core:{}:{}:{}:{}",
        plan.name(),
        core.owner_locus(),
        core.primary().anchor(),
        core.fallback().anchor(),
    )
}

fn relation_semantic_digest(relation: &M8SemanticRelation) -> String {
    format!(
        "relation:{}:{}:{}:{}:{}:{}:{}:{}",
        relation.name,
        relation.owner_locus,
        relation.selected_option_index,
        relation.selected_floor.as_str(),
        relation.selected_anchor,
        relation.selected_option_epoch,
        relation.binding_epoch,
        relation.lineage.join(","),
    )
}
