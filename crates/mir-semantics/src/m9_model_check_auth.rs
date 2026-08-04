//! Deterministic bounded-state M9 authorization model.
//!
//! This is a finite operational exploration, not a proof.  It starts from a
//! pending authority state and explores `admit`, `grant`, `revoke`, `use`, and
//! `reacquire` transitions through the requested bound.  The two fault-injector
//! inputs deliberately alter transition rules; the checker reports a concrete
//! trace only when a reachable edge violates the property being checked.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::surface_v0_pipeline::CheckedSurfaceV0;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum M9AuthModelEvidenceRef {
    Fixture(String),
    RuntimeTrace(String),
    AuthorityGraph(String),
}

impl M9AuthModelEvidenceRef {
    pub fn fixture(value: impl Into<String>) -> Self {
        Self::Fixture(value.into())
    }

    pub fn runtime_trace(value: impl Into<String>) -> Self {
        Self::RuntimeTrace(value.into())
    }

    pub fn authority_graph(value: impl Into<String>) -> Self {
        Self::AuthorityGraph(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum M9AuthModelProperty {
    MonotoneRevocation,
    RejectedUseDoesNotMutateM8Payload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9AuthModelCounterexampleKind {
    RevokedGrantReplay,
    HiddenM8PayloadMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M9AuthModelResultKind {
    Holds,
    Counterexample,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelCounterexample {
    kind: M9AuthModelCounterexampleKind,
    action_trace: Vec<String>,
    state_trace: Vec<M9AuthModelState>,
    violating_edge: M9AuthModelViolatingEdge,
}

impl M9AuthModelCounterexample {
    pub const fn kind(&self) -> M9AuthModelCounterexampleKind {
        self.kind
    }

    /// Concrete action trace leading to the first deterministic violation.
    pub fn action_trace(&self) -> &[String] {
        &self.action_trace
    }

    pub fn state_trace(&self) -> &[M9AuthModelState] {
        &self.state_trace
    }

    pub fn violating_edge(&self) -> Option<&M9AuthModelViolatingEdge> {
        Some(&self.violating_edge)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelState {
    epoch: u8,
    membership_live: bool,
    capability_live: bool,
    capability_revoked: bool,
    witness_valid: bool,
    m8_payload_fingerprint: String,
    revocations: BTreeSet<String>,
    accepted_capability_use: Option<String>,
    fresh_reacquire_evidence_epoch: Option<String>,
    membership_ref: String,
    capability_ref: String,
    witness_ref: String,
}

impl M9AuthModelState {
    pub const fn epoch(&self) -> u8 {
        self.epoch
    }

    pub const fn membership_live(&self) -> bool {
        self.membership_live
    }

    pub const fn capability_live(&self) -> bool {
        self.capability_live
    }

    pub const fn witness_valid(&self) -> bool {
        self.witness_valid
    }

    pub fn epoch_label(&self) -> String {
        format!("epoch{}", self.epoch)
    }

    pub fn live_capability(&self, capability_ref: &str) -> bool {
        self.capability_live && self.capability_ref == capability_ref
    }

    pub fn live_lineage(
        &self,
        membership_ref: &str,
        capability_ref: &str,
        witness_ref: &str,
    ) -> bool {
        self.membership_live
            && self.capability_live
            && self.witness_valid
            && self.membership_ref == membership_ref
            && self.capability_ref == capability_ref
            && self.witness_ref == witness_ref
    }

    pub fn m8_payload_fingerprint(&self) -> &str {
        &self.m8_payload_fingerprint
    }

    pub fn contains_revocation(&self, revocation: &str) -> bool {
        self.revocations.contains(revocation)
    }

    pub fn accepted_capability_use(&self, capability: &str) -> bool {
        self.accepted_capability_use.as_deref() == Some(capability)
    }

    pub fn fresh_reacquire_evidence_epoch(&self) -> Option<&str> {
        self.fresh_reacquire_evidence_epoch.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelViolatingEdge {
    kind: M9AuthModelCounterexampleKind,
    action_label: String,
    capability_ref: String,
    pre_state: M9AuthModelState,
    post_state: M9AuthModelState,
}

impl M9AuthModelViolatingEdge {
    pub const fn kind(&self) -> M9AuthModelCounterexampleKind {
        self.kind
    }

    pub fn action_label(&self) -> &str {
        &self.action_label
    }

    pub fn capability_ref(&self) -> &str {
        &self.capability_ref
    }

    pub fn pre_state(&self) -> &M9AuthModelState {
        &self.pre_state
    }

    pub fn post_state(&self) -> &M9AuthModelState {
        &self.post_state
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelProvenance {
    evidence_refs: Vec<M9AuthModelEvidenceRef>,
    bound: usize,
    visited_states: usize,
    explored_transitions: usize,
}

impl M9AuthModelProvenance {
    pub fn contains_ref(&self, evidence: &M9AuthModelEvidenceRef) -> bool {
        self.evidence_refs.contains(evidence)
    }

    /// Any source or evidence replacement invalidates this bounded result.
    pub fn is_invalidated_by(&self, evidence: &M9AuthModelEvidenceRef) -> bool {
        self.contains_ref(evidence)
    }

    pub const fn bound(&self) -> usize {
        self.bound
    }

    pub const fn visited_states(&self) -> usize {
        self.visited_states
    }

    pub const fn explored_transitions(&self) -> usize {
        self.explored_transitions
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelArtifact {
    property: M9AuthModelProperty,
    provenance: M9AuthModelProvenance,
    counterexample_trace: Option<Vec<String>>,
}

impl M9AuthModelArtifact {
    pub const fn property(&self) -> M9AuthModelProperty {
        self.property
    }

    pub fn provenance(&self) -> &M9AuthModelProvenance {
        &self.provenance
    }

    pub fn counterexample_trace(&self) -> Option<&[String]> {
        self.counterexample_trace.as_deref()
    }

    pub const fn claims_static_check(&self) -> bool {
        false
    }

    pub const fn claims_lean_proof(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelResult {
    kind: M9AuthModelResultKind,
    counterexample: Option<M9AuthModelCounterexample>,
    covered_properties: Vec<M9AuthModelProperty>,
    evidence_refs: Vec<M9AuthModelEvidenceRef>,
    bound: usize,
    visited_states: usize,
    explored_transitions: usize,
    max_explored_depth: usize,
    bounded_state_graph_complete: bool,
    states_by_action_trace: BTreeMap<Vec<String>, M9AuthModelState>,
    accepted_capability_uses: BTreeSet<(String, String)>,
}

impl M9AuthModelResult {
    pub const fn kind(&self) -> M9AuthModelResultKind {
        self.kind
    }

    pub fn counterexample(&self) -> Option<&M9AuthModelCounterexample> {
        self.counterexample.as_ref()
    }

    pub fn covered_properties(&self) -> Vec<M9AuthModelProperty> {
        self.covered_properties.clone()
    }

    pub fn evidence_refs(&self) -> Vec<M9AuthModelEvidenceRef> {
        self.evidence_refs.clone()
    }

    pub const fn bound(&self) -> usize {
        self.bound
    }

    pub const fn visited_states(&self) -> usize {
        self.visited_states
    }

    pub const fn explored_transitions(&self) -> usize {
        self.explored_transitions
    }

    pub const fn max_explored_depth(&self) -> usize {
        self.max_explored_depth
    }

    pub const fn explored_state_count(&self) -> usize {
        self.visited_states
    }

    pub const fn transition_count(&self) -> usize {
        self.explored_transitions
    }

    /// The bounded reachable-state graph was fully explored with state
    /// equivalence merging. This does not claim enumeration of every action
    /// sequence leading to an equivalent state.
    pub const fn exhaustively_explored_bounded_state_graph(&self) -> bool {
        self.bounded_state_graph_complete
    }

    pub fn state_after_action_trace(&self, action_trace: &[&str]) -> Option<&M9AuthModelState> {
        let key = action_trace
            .iter()
            .map(|action| (*action).to_string())
            .collect::<Vec<_>>();
        self.states_by_action_trace.get(&key)
    }

    pub fn accepts_capability_use_with_witness(
        &self,
        capability_ref: &str,
        witness_ref: &str,
    ) -> bool {
        self.accepted_capability_uses
            .contains(&(capability_ref.to_string(), witness_ref.to_string()))
    }

    pub const fn claims_proof_discharge(&self) -> bool {
        false
    }

    pub fn into_artifact(self) -> M9AuthModelArtifact {
        M9AuthModelArtifact {
            property: self
                .covered_properties
                .first()
                .copied()
                .unwrap_or(M9AuthModelProperty::MonotoneRevocation),
            provenance: M9AuthModelProvenance {
                evidence_refs: self.evidence_refs,
                bound: self.bound,
                visited_states: self.visited_states,
                explored_transitions: self.explored_transitions,
            },
            counterexample_trace: self
                .counterexample
                .map(|counterexample| counterexample.action_trace),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelCase {
    _case_id: String,
    checked_identity: Option<String>,
    properties: Vec<M9AuthModelProperty>,
    memberships: BTreeSet<String>,
    capabilities: BTreeSet<String>,
    witnesses: BTreeMap<String, String>,
    revocations: BTreeSet<String>,
    attempted_use_after_revocation: Option<String>,
    rejected_use: Option<String>,
    reacquire_attempt: Option<String>,
    expected_new_lineage: Option<(String, String, String)>,
    replay_allowed: Option<String>,
    hidden_m8_payload_mutation: bool,
    fresh_reacquire_evidence: Option<(String, String)>,
    evidence_refs: Vec<M9AuthModelEvidenceRef>,
}

impl M9AuthModelCase {
    pub fn new(case_id: impl Into<String>) -> Self {
        Self {
            _case_id: case_id.into(),
            checked_identity: None,
            properties: Vec::new(),
            memberships: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            witnesses: BTreeMap::new(),
            revocations: BTreeSet::new(),
            attempted_use_after_revocation: None,
            rejected_use: None,
            reacquire_attempt: None,
            expected_new_lineage: None,
            replay_allowed: None,
            hidden_m8_payload_mutation: false,
            fresh_reacquire_evidence: None,
            evidence_refs: Vec::new(),
        }
    }

    pub fn with_checked_surface(mut self, checked: CheckedSurfaceV0) -> Self {
        self.checked_identity = Some(checked.program_identity().stable_key());
        self
    }

    pub fn with_property(mut self, property: M9AuthModelProperty) -> Self {
        if !self.properties.contains(&property) {
            self.properties.push(property);
        }
        self
    }

    pub fn with_membership(mut self, membership: impl Into<String>) -> Self {
        self.memberships.insert(membership.into());
        self
    }

    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.insert(capability.into());
        self
    }

    pub fn with_witness(
        mut self,
        witness: impl Into<String>,
        capability: impl Into<String>,
    ) -> Self {
        self.witnesses.insert(witness.into(), capability.into());
        self
    }

    pub fn with_revocation(mut self, revocation: impl Into<String>) -> Self {
        self.revocations.insert(revocation.into());
        self
    }

    pub fn with_attempted_use_after_revocation(mut self, capability: impl Into<String>) -> Self {
        self.attempted_use_after_revocation = Some(capability.into());
        self
    }

    pub fn with_rejected_use(mut self, capability: impl Into<String>) -> Self {
        self.rejected_use = Some(capability.into());
        self
    }

    pub fn with_reacquire_attempt(mut self, capability: impl Into<String>) -> Self {
        self.reacquire_attempt = Some(capability.into());
        self
    }

    pub fn without_fresh_epoch_evidence(mut self) -> Self {
        self.fresh_reacquire_evidence = None;
        self
    }

    pub fn with_fresh_epoch_evidence(
        mut self,
        epoch: impl Into<String>,
        proof_ref: impl Into<String>,
    ) -> Self {
        self.fresh_reacquire_evidence = Some((epoch.into(), proof_ref.into()));
        self
    }

    pub fn with_expected_new_lineage(
        mut self,
        membership: impl Into<String>,
        capability: impl Into<String>,
        witness: impl Into<String>,
    ) -> Self {
        self.expected_new_lineage = Some((membership.into(), capability.into(), witness.into()));
        self
    }

    /// Fault injection: changes the revoke/grant/use transition rule only.
    pub fn allow_replay_after_revocation(mut self, capability: impl Into<String>) -> Self {
        self.replay_allowed = Some(capability.into());
        self
    }

    /// Fault injection: changes only the rejected-use transition rule.
    pub fn allow_hidden_m8_payload_mutation(mut self) -> Self {
        self.hidden_m8_payload_mutation = true;
        self
    }

    /// Supplies the fresh, epoch-bound evidence required by `reacquire`.
    /// Without it, a revoked capability cannot be reactivated in the model.
    pub fn with_fresh_reacquire_evidence_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.fresh_reacquire_evidence = Some((epoch.into(), "implicit-fresh-proof".to_string()));
        self
    }

    pub fn with_evidence_ref(mut self, evidence: M9AuthModelEvidenceRef) -> Self {
        if !self.evidence_refs.contains(&evidence) {
            self.evidence_refs.push(evidence);
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M9AuthModelDiagnostics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M9AuthModelChecker {
    bound: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MembershipState {
    Pending,
    Live,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CapabilityState {
    Absent,
    Live,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum WitnessState {
    Invalid,
    Valid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct M9FiniteState {
    epoch: u8,
    membership: MembershipState,
    capability: CapabilityState,
    witness: WitnessState,
    m8_payload_version: u8,
    fresh_reacquire_evidence_epoch: Option<u8>,
}

impl M9FiniteState {
    const fn initial() -> Self {
        Self {
            epoch: 1,
            membership: MembershipState::Pending,
            capability: CapabilityState::Absent,
            witness: WitnessState::Invalid,
            m8_payload_version: 0,
            fresh_reacquire_evidence_epoch: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M9FiniteAction {
    Admit,
    Grant,
    Revoke,
    Use,
    Reacquire,
}

impl M9FiniteAction {
    const ALL: [Self; 5] = [
        Self::Admit,
        Self::Grant,
        Self::Revoke,
        Self::Use,
        Self::Reacquire,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct M9Transition {
    next: M9FiniteState,
    accepted_use: bool,
}

#[derive(Debug)]
struct M9ModelExploration {
    bound: usize,
    visited_states: usize,
    explored_transitions: usize,
    max_explored_depth: usize,
    states_by_action_trace: BTreeMap<Vec<String>, M9AuthModelState>,
    accepted_capability_uses: BTreeSet<(String, String)>,
}

#[derive(Debug, Clone)]
struct M9FinitePolicy {
    has_membership_certificate: bool,
    has_capability_certificate: bool,
    has_witness_certificate: bool,
    replay_fault: bool,
    hidden_m8_mutation_fault: bool,
    revocation_requested: bool,
    attempted_use_after_revocation: bool,
    rejected_use_requested: bool,
    reacquire_requested: bool,
    revocation_ref: Option<String>,
    max_epoch: u8,
    fresh_reacquire_evidence_epoch: Option<u8>,
    membership_template: String,
    capability_template: String,
    witness_template: String,
    expected_new_lineage: Option<(String, String, String)>,
}

impl M9AuthModelChecker {
    pub const fn bounded(bound: usize) -> Self {
        Self { bound }
    }

    pub fn check(
        &self,
        case: M9AuthModelCase,
    ) -> Result<M9AuthModelResult, M9AuthModelDiagnostics> {
        if self.bound == 0 || case.checked_identity.is_none() {
            return Err(M9AuthModelDiagnostics);
        }
        let target = model_target_capability(&case).ok_or(M9AuthModelDiagnostics)?;
        let membership_template = case
            .memberships
            .iter()
            .next()
            .cloned()
            .ok_or(M9AuthModelDiagnostics)?;
        let witness_template = case
            .witnesses
            .iter()
            .find_map(|(witness, capability)| (capability == target).then(|| witness.clone()))
            .ok_or(M9AuthModelDiagnostics)?;
        let revocation_ref = format!("revocation:{target}");
        let revocation_requested = match case.revocations.len() {
            0 => false,
            1 if case.revocations.contains(&revocation_ref) => true,
            _ => return Err(M9AuthModelDiagnostics),
        };
        let attempted_use_after_revocation = match case.attempted_use_after_revocation.as_deref() {
            None => true,
            Some(capability) if capability == target => true,
            Some(_) => return Err(M9AuthModelDiagnostics),
        };
        let rejected_use_requested = matches!(
            case.rejected_use.as_deref(),
            Some(capability) if capability == target
        );
        if case.rejected_use.is_some() && !rejected_use_requested {
            return Err(M9AuthModelDiagnostics);
        }
        if case.hidden_m8_payload_mutation && !rejected_use_requested {
            return Err(M9AuthModelDiagnostics);
        }
        if case
            .replay_allowed
            .as_deref()
            .is_some_and(|capability| capability != target)
        {
            return Err(M9AuthModelDiagnostics);
        }
        let fresh_reacquire_evidence_epoch = case
            .fresh_reacquire_evidence
            .as_ref()
            .and_then(|(epoch, proof_ref)| (!proof_ref.is_empty()).then_some(epoch.as_str()))
            .and_then(parse_finite_epoch);
        let expected_reacquire_capability = fresh_reacquire_evidence_epoch
            .map(|epoch| {
                case.expected_new_lineage
                    .as_ref()
                    .map(|(_, capability, _)| capability.clone())
                    .unwrap_or_else(|| rebind_epoch(target, epoch))
            })
            .unwrap_or_else(|| target.to_string());
        let reacquire_requested = matches!(
            case.reacquire_attempt.as_deref(),
            Some(capability) if capability == expected_reacquire_capability
        );
        if case.reacquire_attempt.is_some() && !reacquire_requested {
            return Err(M9AuthModelDiagnostics);
        }
        let policy = M9FinitePolicy {
            has_membership_certificate: !case.memberships.is_empty(),
            has_capability_certificate: case.capabilities.contains(target),
            has_witness_certificate: case
                .witnesses
                .values()
                .any(|capability| capability == target),
            replay_fault: case.replay_allowed.as_deref() == Some(target),
            hidden_m8_mutation_fault: case.hidden_m8_payload_mutation,
            revocation_requested,
            attempted_use_after_revocation,
            rejected_use_requested,
            reacquire_requested,
            revocation_ref: revocation_requested.then_some(revocation_ref),
            max_epoch: (self.bound.min((u8::MAX - 1) as usize) as u8).saturating_add(1),
            fresh_reacquire_evidence_epoch,
            membership_template,
            capability_template: target.to_string(),
            witness_template,
            expected_new_lineage: case.expected_new_lineage.clone(),
        };
        let initial_state = M9FiniteState::initial();
        let initial_trace = vec![snapshot(initial_state, &policy, false)];
        let mut frontier = VecDeque::from([(initial_state, Vec::new(), initial_trace)]);
        let mut visited = BTreeSet::from([M9FiniteState::initial()]);
        let mut explored_transitions = 0;
        let mut max_explored_depth = 0;
        let mut states_by_action_trace = BTreeMap::new();
        states_by_action_trace.insert(Vec::new(), snapshot(initial_state, &policy, false));
        let mut accepted_capability_uses = BTreeSet::new();

        while let Some((state, action_trace, state_trace)) = frontier.pop_front() {
            if action_trace.len() == self.bound {
                continue;
            }
            for action in M9FiniteAction::ALL {
                let transition = step(state, action, &policy);
                explored_transitions += 1;
                let action_label = action.label(transition.accepted_use);
                let mut next_action_trace = action_trace.clone();
                next_action_trace.push(action_label.to_string());
                let mut next_state_trace = state_trace.clone();
                next_state_trace.push(snapshot(transition.next, &policy, transition.accepted_use));
                max_explored_depth = max_explored_depth.max(next_action_trace.len());
                let post_state = next_state_trace
                    .last()
                    .cloned()
                    .expect("transition trace records its post-state");
                if transition.accepted_use {
                    accepted_capability_uses.insert((
                        post_state.capability_ref.clone(),
                        post_state.witness_ref.clone(),
                    ));
                }
                states_by_action_trace.insert(next_action_trace.clone(), post_state.clone());

                if let Some(kind) = violated_property(&case.properties, state, transition, action) {
                    let pre_state = state_trace
                        .last()
                        .cloned()
                        .expect("model traces retain their initial state");
                    return Ok(model_result(
                        &case,
                        M9ModelExploration {
                            bound: self.bound,
                            visited_states: visited.len(),
                            explored_transitions,
                            max_explored_depth,
                            states_by_action_trace,
                            accepted_capability_uses,
                        },
                        Some(M9AuthModelCounterexample {
                            kind,
                            action_trace: next_action_trace,
                            state_trace: next_state_trace,
                            violating_edge: M9AuthModelViolatingEdge {
                                kind,
                                action_label: action_label.to_string(),
                                capability_ref: policy.capability_template.clone(),
                                pre_state,
                                post_state,
                            },
                        }),
                    ));
                }

                if visited.insert(transition.next) {
                    frontier.push_back((transition.next, next_action_trace, next_state_trace));
                }
            }
        }

        Ok(model_result(
            &case,
            M9ModelExploration {
                bound: self.bound,
                visited_states: visited.len(),
                explored_transitions,
                max_explored_depth,
                states_by_action_trace,
                accepted_capability_uses,
            },
            None,
        ))
    }
}

fn model_target_capability(case: &M9AuthModelCase) -> Option<&str> {
    case.capabilities.iter().next().map(String::as_str)
}

fn step(state: M9FiniteState, action: M9FiniteAction, policy: &M9FinitePolicy) -> M9Transition {
    let mut next = state;
    let mut accepted_use = false;
    match action {
        M9FiniteAction::Admit => {
            if policy.has_membership_certificate {
                next.membership = MembershipState::Live;
            }
        }
        M9FiniteAction::Grant => {
            if next.membership == MembershipState::Live && policy.has_capability_certificate {
                match next.capability {
                    CapabilityState::Revoked => {}
                    _ => {
                        next.capability = CapabilityState::Live;
                        next.witness = if policy.has_witness_certificate {
                            WitnessState::Valid
                        } else {
                            WitnessState::Invalid
                        };
                    }
                }
            }
        }
        M9FiniteAction::Revoke => {
            if policy.revocation_requested && next.capability == CapabilityState::Live {
                next.capability = CapabilityState::Revoked;
                next.witness = WitnessState::Invalid;
            }
        }
        M9FiniteAction::Use => {
            accepted_use = next.membership == MembershipState::Live
                && next.capability == CapabilityState::Live
                && next.witness == WitnessState::Valid;
            if next.capability == CapabilityState::Revoked
                && policy.attempted_use_after_revocation
                && policy.replay_fault
            {
                accepted_use = true;
            }
            if !accepted_use && policy.rejected_use_requested && policy.hidden_m8_mutation_fault {
                next.m8_payload_version = next.m8_payload_version.saturating_add(1);
            }
        }
        M9FiniteAction::Reacquire => {
            if policy.reacquire_requested
                && next.capability == CapabilityState::Revoked
                && policy.has_membership_certificate
                && policy.has_capability_certificate
                && policy
                    .fresh_reacquire_evidence_epoch
                    .is_some_and(|fresh_epoch| {
                        fresh_epoch > next.epoch && fresh_epoch <= policy.max_epoch
                    })
            {
                let fresh_epoch = policy
                    .fresh_reacquire_evidence_epoch
                    .expect("guard retains fresh epoch evidence");
                next.epoch = fresh_epoch;
                next.fresh_reacquire_evidence_epoch = Some(fresh_epoch);
                next.membership = MembershipState::Live;
                next.capability = CapabilityState::Live;
                next.witness = if policy.has_witness_certificate {
                    WitnessState::Valid
                } else {
                    WitnessState::Invalid
                };
            }
        }
    }
    M9Transition { next, accepted_use }
}

impl M9FiniteAction {
    const fn label(self, accepted_use: bool) -> &'static str {
        match self {
            Self::Admit => "admit_membership",
            Self::Grant => "grant_capability",
            Self::Revoke => "revoke_capability",
            Self::Use if accepted_use => "use_capability",
            Self::Use => "reject_use",
            Self::Reacquire => "reacquire_capability",
        }
    }
}

fn snapshot(state: M9FiniteState, policy: &M9FinitePolicy, accepted_use: bool) -> M9AuthModelState {
    let (membership_ref, capability_ref, witness_ref) =
        if state.fresh_reacquire_evidence_epoch.is_some() {
            policy.expected_new_lineage.clone().unwrap_or_else(|| {
                (
                    rebind_epoch(&policy.membership_template, state.epoch),
                    rebind_epoch(&policy.capability_template, state.epoch),
                    rebind_epoch(&policy.witness_template, state.epoch),
                )
            })
        } else {
            (
                rebind_epoch(&policy.membership_template, state.epoch),
                rebind_epoch(&policy.capability_template, state.epoch),
                rebind_epoch(&policy.witness_template, state.epoch),
            )
        };
    let mut revocations = BTreeSet::new();
    if state.capability == CapabilityState::Revoked
        && let Some(revocation_ref) = &policy.revocation_ref
    {
        revocations.insert(revocation_ref.clone());
    }
    M9AuthModelState {
        epoch: state.epoch,
        membership_live: state.membership == MembershipState::Live,
        capability_live: state.capability == CapabilityState::Live,
        capability_revoked: state.capability == CapabilityState::Revoked,
        witness_valid: state.witness == WitnessState::Valid,
        m8_payload_fingerprint: format!("m8-payload:v{}", state.m8_payload_version),
        revocations,
        accepted_capability_use: accepted_use.then(|| capability_ref.clone()),
        fresh_reacquire_evidence_epoch: state
            .fresh_reacquire_evidence_epoch
            .map(|epoch| format!("epoch{epoch}")),
        membership_ref,
        capability_ref,
        witness_ref,
    }
}

fn rebind_epoch(reference: &str, epoch: u8) -> String {
    let Some((prefix, _old_epoch)) = reference.rsplit_once(':') else {
        return reference.to_string();
    };
    format!("{prefix}:epoch{epoch}")
}

fn parse_finite_epoch(value: &str) -> Option<u8> {
    let digits = value
        .strip_prefix("epoch")
        .unwrap_or(value)
        .parse::<u8>()
        .ok()?;
    (digits > 0).then_some(digits)
}

fn violated_property(
    properties: &[M9AuthModelProperty],
    before: M9FiniteState,
    transition: M9Transition,
    action: M9FiniteAction,
) -> Option<M9AuthModelCounterexampleKind> {
    if properties.contains(&M9AuthModelProperty::MonotoneRevocation)
        && before.capability == CapabilityState::Revoked
        && ((action == M9FiniteAction::Use && transition.accepted_use)
            || (transition.next.capability == CapabilityState::Live
                && transition.next.epoch == before.epoch))
    {
        return Some(M9AuthModelCounterexampleKind::RevokedGrantReplay);
    }
    if properties.contains(&M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload)
        && action == M9FiniteAction::Use
        && !transition.accepted_use
        && transition.next.m8_payload_version != before.m8_payload_version
    {
        return Some(M9AuthModelCounterexampleKind::HiddenM8PayloadMutation);
    }
    None
}

fn model_result(
    case: &M9AuthModelCase,
    exploration: M9ModelExploration,
    counterexample: Option<M9AuthModelCounterexample>,
) -> M9AuthModelResult {
    const REQUIRED_PROPERTY_DEPTH: usize = 4;
    let bounded_state_graph_complete =
        exploration.bound >= REQUIRED_PROPERTY_DEPTH && counterexample.is_none();
    let covers_requested_properties = bounded_state_graph_complete;
    M9AuthModelResult {
        kind: if counterexample.is_some() {
            M9AuthModelResultKind::Counterexample
        } else {
            M9AuthModelResultKind::Holds
        },
        counterexample,
        covered_properties: if covers_requested_properties {
            case.properties.clone()
        } else {
            Vec::new()
        },
        evidence_refs: case.evidence_refs.clone(),
        bound: exploration.bound,
        visited_states: exploration.visited_states,
        explored_transitions: exploration.explored_transitions,
        max_explored_depth: exploration.max_explored_depth,
        bounded_state_graph_complete,
        states_by_action_trace: exploration.states_by_action_trace,
        accepted_capability_uses: exploration.accepted_capability_uses,
    }
}
