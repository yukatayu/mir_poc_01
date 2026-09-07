//! Bounded I3-3 request-lifecycle model evidence for candidate B.
//!
//! This is deliberately a dependency-free state model, not a socket, QUIC,
//! process, or end-to-end harness.  It models one owner, at most two
//! source/Core-bound semantic request identities, two transport attempts per
//! identity, one monotone authority withdrawal, and one disconnect/reconnect
//! cycle.  A changed carrier fingerprint is an adversarial binding mismatch,
//! not a new semantic request identity.
//!
//! Correspondence is intentionally narrow:
//! - `Sys5I3ProcessRuntime::reserve_inbound_owner_request` reserves the
//!   owner-local tombstone before handoff and rejects duplicate, binding
//!   mismatch, and bounded-ledger cases.
//! - `Sys5I3ProcessRuntime::accept_reserved_inbound_owner_request` is the
//!   owner-handoff/mutation point represented below by `Delivery::Complete`
//!   and `Delivery::DisconnectAfterHandoff`.
//!
//! Reconnect preservation and use-time authority revalidation are refinement
//! targets for the active I3-3 adapter/runtime work.  This model does not
//! prove that the current runtime has a pure revalidation function, prove a
//! network property, or make a production/exactly-once claim.  It instead
//! checks that the finite candidate rule has the stated safety shape and that
//! three deliberately faulty variants are detected.

use std::collections::{BTreeMap, VecDeque};

const MAX_SEMANTIC_REQUEST_IDENTITIES: usize = 2;
const MAX_TRANSPORT_ATTEMPTS_PER_IDENTITY: usize = 2;
const MAX_AUTHORITY_WITHDRAWALS: usize = 1;
const MAX_DISCONNECT_RECONNECT_CYCLES: usize = 1;

// This is an intentionally smaller abstract capacity than the current
// runtime's fixed 64-entry ledger.  It lets the two-identity model exercise
// the same fail-closed "capacity rejects without semantic mutation" rule; it
// does not assert the runtime's concrete capacity value.
const MODEL_LEDGER_CAPACITY: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Request {
    Alpha,
    Beta,
}

impl Request {
    const ALL: [Self; MAX_SEMANTIC_REQUEST_IDENTITIES] = [Self::Alpha, Self::Beta];

    const fn index(self) -> usize {
        match self {
            Self::Alpha => 0,
            Self::Beta => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Attempt {
    Initial,
    Retry,
}

impl Attempt {
    const ALL: [Self; MAX_TRANSPORT_ATTEMPTS_PER_IDENTITY] = [Self::Initial, Self::Retry];

    const fn index(self) -> usize {
        match self {
            Self::Initial => 0,
            Self::Retry => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Delivery {
    Complete,
    DisconnectBeforeHandoff,
    DisconnectAfterHandoff,
}

impl Delivery {
    const ALL: [Self; 3] = [
        Self::Complete,
        Self::DisconnectBeforeHandoff,
        Self::DisconnectAfterHandoff,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Fingerprint {
    Exact,
    Changed,
}

impl Fingerprint {
    const ALL: [Self; 2] = [Self::Exact, Self::Changed];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Connection {
    Connected,
    Disconnected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Authority {
    Granted,
    Withdrawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TombstonePhase {
    Reserved,
    Completed,
    Ambiguous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tombstone {
    fingerprint: Fingerprint,
    phase: TombstonePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum OwnerDecision {
    NoHandoff,
    NoSemanticMutation,
    Mutated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RequesterKnowledge {
    NotCompleted,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    connection: Connection,
    authority: Authority,
    // This remains true after a mutant reconnect incorrectly restores
    // `authority`, which lets the model identify stale post-withdrawal use.
    authority_was_withdrawn: bool,
    withdrawal_count: usize,
    disconnect_count: usize,
    reconnect_count: usize,
    attempts_used: [[bool; MAX_TRANSPORT_ATTEMPTS_PER_IDENTITY]; MAX_SEMANTIC_REQUEST_IDENTITIES],
    ledger: BTreeMap<Request, Tombstone>,
    owner_decisions: [OwnerDecision; MAX_SEMANTIC_REQUEST_IDENTITIES],
    requester_knowledge: [RequesterKnowledge; MAX_SEMANTIC_REQUEST_IDENTITIES],
    semantic_mutations: [u8; MAX_SEMANTIC_REQUEST_IDENTITIES],
}

impl State {
    fn initial() -> Self {
        Self {
            connection: Connection::Connected,
            authority: Authority::Granted,
            authority_was_withdrawn: false,
            withdrawal_count: 0,
            disconnect_count: 0,
            reconnect_count: 0,
            attempts_used: [[false; MAX_TRANSPORT_ATTEMPTS_PER_IDENTITY];
                MAX_SEMANTIC_REQUEST_IDENTITIES],
            ledger: BTreeMap::new(),
            owner_decisions: [OwnerDecision::NoHandoff; MAX_SEMANTIC_REQUEST_IDENTITIES],
            requester_knowledge: [RequesterKnowledge::NotCompleted;
                MAX_SEMANTIC_REQUEST_IDENTITIES],
            semantic_mutations: [0; MAX_SEMANTIC_REQUEST_IDENTITIES],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    WithdrawAuthority,
    Disconnect,
    Reconnect,
    Deliver {
        request: Request,
        attempt: Attempt,
        delivery: Delivery,
        fingerprint: Fingerprint,
    },
    // A checked identity beyond this finite model's ledger budget is rejected
    // before reservation.  It is a capacity fault input, not a third modeled
    // semantic identity or a claim about the runtime's 64-entry constant.
    CapacityPressure {
        request: Request,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rejection {
    MissingAuthority,
    Duplicate,
    BindingMismatch,
    Capacity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    NoSemanticDelivery,
    Served,
    AmbiguousBeforeHandoff,
    AmbiguousAfterHandoff,
    Rejected(Rejection),
}

impl Outcome {
    const fn is_rejection(self) -> bool {
        matches!(self, Self::Rejected(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelVariant {
    CandidateB,
    RemoveDedup,
    ClearLedgerOnReconnect,
    RenewGrantOnReconnect,
}

#[derive(Clone, Debug)]
struct Transition {
    after: State,
    outcome: Outcome,
}

fn admissible_actions(state: &State) -> Vec<Action> {
    let mut actions = Vec::new();
    if state.withdrawal_count < MAX_AUTHORITY_WITHDRAWALS {
        actions.push(Action::WithdrawAuthority);
    }
    if state.connection == Connection::Connected
        && state.disconnect_count < MAX_DISCONNECT_RECONNECT_CYCLES
    {
        actions.push(Action::Disconnect);
    }
    if state.connection == Connection::Disconnected
        && state.reconnect_count < MAX_DISCONNECT_RECONNECT_CYCLES
    {
        actions.push(Action::Reconnect);
    }
    if state.connection == Connection::Connected {
        for request in Request::ALL {
            for attempt in Attempt::ALL {
                if state.attempts_used[request.index()][attempt.index()] {
                    continue;
                }
                for delivery in Delivery::ALL {
                    if matches!(
                        delivery,
                        Delivery::DisconnectBeforeHandoff | Delivery::DisconnectAfterHandoff
                    ) && state.disconnect_count >= MAX_DISCONNECT_RECONNECT_CYCLES
                    {
                        continue;
                    }
                    for fingerprint in Fingerprint::ALL {
                        actions.push(Action::Deliver {
                            request,
                            attempt,
                            delivery,
                            fingerprint,
                        });
                    }
                }
            }
        }
        if state.ledger.len() >= MODEL_LEDGER_CAPACITY {
            for request in Request::ALL {
                if !state.ledger.contains_key(&request) {
                    actions.push(Action::CapacityPressure { request });
                }
            }
        }
    }
    actions
}

fn transition(state: &State, action: Action, variant: ModelVariant) -> Transition {
    let mut after = state.clone();
    let outcome = match action {
        Action::WithdrawAuthority => {
            after.authority = Authority::Withdrawn;
            after.authority_was_withdrawn = true;
            after.withdrawal_count += 1;
            // This administrative transition has no semantic request result.
            Outcome::NoSemanticDelivery
        }
        Action::Disconnect => {
            after.connection = Connection::Disconnected;
            after.disconnect_count += 1;
            Outcome::NoSemanticDelivery
        }
        Action::Reconnect => {
            after.connection = Connection::Connected;
            after.reconnect_count += 1;
            if variant == ModelVariant::ClearLedgerOnReconnect {
                after.ledger.clear();
            }
            if variant == ModelVariant::RenewGrantOnReconnect {
                after.authority = Authority::Granted;
            }
            Outcome::NoSemanticDelivery
        }
        Action::CapacityPressure { request } => {
            if after.ledger.contains_key(&request) {
                Outcome::Rejected(Rejection::Duplicate)
            } else {
                debug_assert!(after.ledger.len() >= MODEL_LEDGER_CAPACITY);
                Outcome::Rejected(Rejection::Capacity)
            }
        }
        Action::Deliver {
            request,
            attempt,
            delivery,
            fingerprint,
        } => {
            after.attempts_used[request.index()][attempt.index()] = true;

            // This model assumes the source/Core semantic identity has already
            // been checked.  A carrier fingerprint change is still denied: it
            // never becomes a new semantic request in the finite model.
            if fingerprint == Fingerprint::Changed {
                Outcome::Rejected(Rejection::BindingMismatch)
            } else if after.authority != Authority::Granted {
                Outcome::Rejected(Rejection::MissingAuthority)
            } else if let Some(existing) = after.ledger.get(&request) {
                if variant != ModelVariant::RemoveDedup {
                    debug_assert_eq!(existing.fingerprint, Fingerprint::Exact);
                    Outcome::Rejected(Rejection::Duplicate)
                } else {
                    handoff(&mut after, request, delivery)
                }
            } else if after.ledger.len() >= MODEL_LEDGER_CAPACITY {
                Outcome::Rejected(Rejection::Capacity)
            } else {
                // Candidate B's essential ordering: reserve the tombstone
                // before the owner handoff.  The disconnect-before-handoff
                // branch below makes that retention observable.
                after.ledger.insert(
                    request,
                    Tombstone {
                        fingerprint: Fingerprint::Exact,
                        phase: TombstonePhase::Reserved,
                    },
                );
                handoff(&mut after, request, delivery)
            }
        }
    };
    Transition { after, outcome }
}

fn handoff(after: &mut State, request: Request, delivery: Delivery) -> Outcome {
    let request_index = request.index();
    let tombstone = after
        .ledger
        .get_mut(&request)
        .expect("handoff is reachable only after a retained reservation");
    match delivery {
        Delivery::Complete => {
            tombstone.phase = TombstonePhase::Completed;
            after.semantic_mutations[request_index] += 1;
            after.owner_decisions[request_index] = OwnerDecision::Mutated;
            after.requester_knowledge[request_index] = RequesterKnowledge::Completed;
            Outcome::Served
        }
        Delivery::DisconnectBeforeHandoff => {
            tombstone.phase = TombstonePhase::Ambiguous;
            after.connection = Connection::Disconnected;
            after.disconnect_count += 1;
            after.owner_decisions[request_index] = OwnerDecision::NoSemanticMutation;
            // The requester has a delivery ambiguity, even though this model's
            // owner-side decision knows no mutation occurred.
            after.requester_knowledge[request_index] = RequesterKnowledge::NotCompleted;
            Outcome::AmbiguousBeforeHandoff
        }
        Delivery::DisconnectAfterHandoff => {
            tombstone.phase = TombstonePhase::Ambiguous;
            after.connection = Connection::Disconnected;
            after.disconnect_count += 1;
            after.semantic_mutations[request_index] += 1;
            after.owner_decisions[request_index] = OwnerDecision::Mutated;
            // A mutation occurred, but the requester cannot infer completion
            // from a disconnected result path.
            after.requester_knowledge[request_index] = RequesterKnowledge::NotCompleted;
            Outcome::AmbiguousAfterHandoff
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Violation {
    AtMostOneSemanticMutation(Request),
    WithdrawalDidNotDominateLaterUse(Request),
    RejectionMutatedSemanticState,
    ReconnectResetLedger,
    ReconnectResetAuthority,
    AmbiguityImpliedCompletion(Request),
    MissingPreHandoffTombstone(Request),
    FingerprintChangeBecameNewSemanticRequest(Request),
}

fn violations(before: &State, action: Action, transition: &Transition) -> Vec<Violation> {
    let after = &transition.after;
    let mut found = Vec::new();
    for request in Request::ALL {
        let index = request.index();
        if after.semantic_mutations[index] > 1 {
            found.push(Violation::AtMostOneSemanticMutation(request));
        }
        if matches!(
            after.ledger.get(&request),
            Some(Tombstone {
                phase: TombstonePhase::Ambiguous,
                ..
            })
        ) && after.requester_knowledge[index] == RequesterKnowledge::Completed
        {
            found.push(Violation::AmbiguityImpliedCompletion(request));
        }
    }
    if transition.outcome.is_rejection() && after.semantic_mutations != before.semantic_mutations {
        found.push(Violation::RejectionMutatedSemanticState);
    }
    if let Action::Reconnect = action {
        if after.ledger != before.ledger {
            found.push(Violation::ReconnectResetLedger);
        }
        if after.authority != before.authority {
            found.push(Violation::ReconnectResetAuthority);
        }
    }
    match action {
        Action::Deliver {
            request,
            delivery: Delivery::DisconnectBeforeHandoff,
            fingerprint: Fingerprint::Exact,
            ..
        } if before.authority == Authority::Granted
            && !before.ledger.contains_key(&request)
            && before.ledger.len() < MODEL_LEDGER_CAPACITY
            && !after.ledger.contains_key(&request) =>
        {
            found.push(Violation::MissingPreHandoffTombstone(request));
        }
        _ => {}
    }
    match action {
        Action::Deliver {
            request,
            fingerprint: Fingerprint::Changed,
            ..
        } if transition.outcome != Outcome::Rejected(Rejection::BindingMismatch)
            || after.ledger != before.ledger =>
        {
            found.push(Violation::FingerprintChangeBecameNewSemanticRequest(
                request,
            ));
        }
        _ => {}
    }
    if let Action::Deliver { request, .. } = action {
        let index = request.index();
        if before.authority_was_withdrawn
            && after.semantic_mutations[index] > before.semantic_mutations[index]
        {
            found.push(Violation::WithdrawalDidNotDominateLaterUse(request));
        }
    }
    found
}

#[derive(Debug)]
struct Counterexample {
    violation: Violation,
    trace: Vec<Action>,
}

#[derive(Debug, Default)]
struct ExplorationStats {
    states: usize,
    transitions: usize,
}

#[derive(Debug)]
struct Exploration {
    stats: ExplorationStats,
    states: BTreeMap<State, Vec<Action>>,
    counterexamples: Vec<Counterexample>,
}

fn explore(variant: ModelVariant) -> Exploration {
    let initial = State::initial();
    let mut states = BTreeMap::from([(initial.clone(), Vec::new())]);
    let mut work = VecDeque::from([initial]);
    let mut counterexamples = Vec::new();
    let mut transitions = 0;

    while let Some(state) = work.pop_front() {
        let trace = states
            .get(&state)
            .expect("queued states retain their shortest witness trace")
            .clone();
        for action in admissible_actions(&state) {
            let next = transition(&state, action, variant);
            transitions += 1;
            let mut next_trace = trace.clone();
            next_trace.push(action);
            counterexamples.extend(violations(&state, action, &next).into_iter().map(
                |violation| Counterexample {
                    violation,
                    trace: next_trace.clone(),
                },
            ));
            if !states.contains_key(&next.after) {
                states.insert(next.after.clone(), next_trace);
                work.push_back(next.after);
            }
        }
    }

    Exploration {
        stats: ExplorationStats {
            states: states.len(),
            transitions,
        },
        states,
        counterexamples,
    }
}

fn contains_violation(exploration: &Exploration, expected: &Violation) -> bool {
    exploration
        .counterexamples
        .iter()
        .any(|counterexample| &counterexample.violation == expected)
}

fn witness_for<'a>(exploration: &'a Exploration, expected: &Violation) -> &'a Counterexample {
    exploration
        .counterexamples
        .iter()
        .find(|counterexample| &counterexample.violation == expected)
        .unwrap_or_else(|| panic!("expected bounded explorer to find {expected:?}"))
}

#[test]
fn i3_3_candidate_b_bounded_request_lifecycle_preserves_ledger_authority_and_ambiguity_invariants()
{
    let exploration = explore(ModelVariant::CandidateB);
    assert!(
        exploration.counterexamples.is_empty(),
        "candidate B violated a bounded lifecycle invariant: {:#?}",
        exploration.counterexamples
    );
    assert!(
        exploration.stats.states > 1 && exploration.stats.transitions > exploration.stats.states,
        "the bounded explorer must traverse meaningful states and actions"
    );
    assert_eq!(
        exploration.stats.states, 432,
        "the fixed finite candidate-B bounds must explore the reviewed state set"
    );
    assert_eq!(
        exploration.stats.transitions, 2136,
        "the fixed finite candidate-B bounds must traverse the reviewed transition set"
    );
    assert!(
        exploration.states.keys().any(|state| {
            state.ledger.get(&Request::Alpha).is_some_and(|entry| {
                entry.phase == TombstonePhase::Ambiguous
                    && state.owner_decisions[Request::Alpha.index()] == OwnerDecision::Mutated
                    && state.requester_knowledge[Request::Alpha.index()]
                        == RequesterKnowledge::NotCompleted
            })
        }),
        "disconnect-after-handoff must distinguish an owner mutation from requester completion knowledge"
    );
    println!(
        "I3-3 candidate-B bounded model positive: states={}, transitions={}; bounds: one owner, two request identities, two attempts/identity, one withdrawal, one disconnect/reconnect, ledger capacity {}",
        exploration.stats.states, exploration.stats.transitions, MODEL_LEDGER_CAPACITY,
    );
}

#[test]
fn i3_3_capacity_pressure_never_reclassifies_a_retained_request_as_a_new_identity() {
    let admitted = transition(
        &State::initial(),
        Action::Deliver {
            request: Request::Alpha,
            attempt: Attempt::Initial,
            delivery: Delivery::Complete,
            fingerprint: Fingerprint::Exact,
        },
        ModelVariant::CandidateB,
    );
    assert_eq!(admitted.outcome, Outcome::Served);

    let retained_identity = transition(
        &admitted.after,
        Action::CapacityPressure {
            request: Request::Alpha,
        },
        ModelVariant::CandidateB,
    );
    assert_eq!(
        retained_identity.outcome,
        Outcome::Rejected(Rejection::Duplicate),
        "an already retained source identity takes duplicate precedence over the full-ledger capacity outcome"
    );
    assert_eq!(retained_identity.after, admitted.after);

    let absent_identity = transition(
        &admitted.after,
        Action::CapacityPressure {
            request: Request::Beta,
        },
        ModelVariant::CandidateB,
    );
    assert_eq!(
        absent_identity.outcome,
        Outcome::Rejected(Rejection::Capacity)
    );
    assert_eq!(absent_identity.after, admitted.after);
}

#[test]
fn i3_3_bounded_request_lifecycle_mutants_are_falsified_by_the_same_invariants() {
    let without_dedup = explore(ModelVariant::RemoveDedup);
    let cleared_on_reconnect = explore(ModelVariant::ClearLedgerOnReconnect);
    let renewed_on_reconnect = explore(ModelVariant::RenewGrantOnReconnect);

    let double_mutation = Violation::AtMostOneSemanticMutation(Request::Alpha);
    let stale_authority = Violation::WithdrawalDidNotDominateLaterUse(Request::Alpha);
    assert!(
        contains_violation(&without_dedup, &double_mutation),
        "removing dedup must permit a bounded double-mutation witness"
    );
    assert!(
        contains_violation(&cleared_on_reconnect, &double_mutation),
        "clearing the ledger on reconnect must permit a bounded double-mutation witness"
    );
    assert!(
        contains_violation(&renewed_on_reconnect, &stale_authority),
        "renewing a withdrawn grant on reconnect must permit stale-authority use"
    );

    println!(
        "I3-3 bounded model falsifiers: remove-dedup={:?}; clear-ledger-on-reconnect={:?}; renew-grant-on-reconnect={:?}",
        witness_for(&without_dedup, &double_mutation).trace,
        witness_for(&cleared_on_reconnect, &double_mutation).trace,
        witness_for(&renewed_on_reconnect, &stale_authority).trace,
    );
}
