# 06 — Theory freeze: atomic cut, save/load, consistent cuts, rollback, and Z-cycle

Mirror normative content into `specs/15-cut-save-load-checkpoint.md`.

## 1. Non-negotiable boundary

`atomic_cut` remains place-local. It finalizes the rollback frontier of the current Place only. It is not:

- a global synchronization point
- a process-wide transaction commit
- a distributed consensus point
- a durable persistence guarantee
- a save/load checkpoint

This must remain unchanged.

## 2. Event DAG and causal order

Define a semantic event graph with a causal/happens-before relation. At minimum, the relation includes:

```text
program_order
state_dependency_order
send_envelope -> receive_envelope
publish -> observe
witness_create -> witness_use
handoff_predecessor -> handoff_successor
hotplug_request -> hotplug_verdict -> activation_cut
membership_update -> dispatch depending on membership
capability_grant -> effect using capability
auth_evidence_create -> auth_evidence_use
```

The event graph must remain acyclic.

## 3. Consistent cut predicate

A cut `K` is consistent if it is prefix-closed under causal order:

```text
if e ∈ K and e' ≺ e, then e' ∈ K
```

Operational examples:

- If a receive is included, the corresponding send must be included or the message must be represented as in-flight channel state.
- If an observe is included, the corresponding publish must be included.
- If a witness use is included, witness creation must be included.
- If a hot-plug activation is included, request and verdict must be included.
- If a state update depending on membership is included, relevant membership update/frontier must be included.

## 4. Save state contents

A distributed or multi-Place save is not just local memory. It must account for:

- Place local states at frontiers
- queues and in-flight MessageEnvelopes
- channel/transport state or equivalent
- MembershipRegistry frontier
- PlaceCatalog frontier
- capability/authority state
- witness store
- lease/freshness frontier
- fallback degradation positions
- hot-plug lifecycle state
- runtime package versions
- adapter states
- external irreversible effect obligations
- randomness/provider evidence where relevant

## 5. Load/rollback constraints

Load must satisfy:

1. The saved state is a consistent cut.
2. Rollback does not cross `atomic_cut` within a Place.
3. Durable/finalized prefixes are not reverted unless a future explicit durable protocol permits a new epoch.
4. Expired leases are not resurrected.
5. Stale witnesses and membership epochs are not resurrected.
6. In-flight messages are restored, discarded, replayed, or compensated explicitly.
7. Irreversible external effects require compensation or isolation.
8. Runtime package versions and hot-plug activations are consistent with the cut.

## 6. Z-cycle / useless checkpoint boundary

A checkpoint dependency graph can create Z-cycles. A checkpoint participating in such a cycle may be useless: it may not belong to any consistent global checkpoint. Mirrorea must not claim distributed save/load unless it has a strategy for avoiding, detecting, or handling such checkpoints.

Alpha options:

### Option A — Local-only save/load

Implement only local Place save/load. Do not claim distributed consistency.

### Option B — Consistent-cut checker only

Allow users/tests to propose distributed cuts, but reject inconsistent or Z-cycle-affected cuts. No production distributed snapshot algorithm.

### Option C — Coordinated snapshot narrow cut

Implement a Chandy-Lamport-style marker/channel-state algorithm for a narrow transport model.

### Option D — Communication-induced checkpointing

Use MessageEnvelope metadata and forced checkpoints to prevent Z-cycles.

Recommendation for Alpha-0/Alpha-1:

- Implement local save/load and a consistent-cut predicate/checker.
- Add negative samples for inconsistent cuts and Z-cycle checkpoints.
- Do not implement full distributed durable save/load yet.

## 7. `durable_cut` boundary

`durable_cut` is not in Mir-0. It may become a Mir-1 vocabulary item meaning that a successful pre-cut prefix will not return to an uncommitted state after local rollback, process restart, or route rebinding. The storage/replication/consensus mechanism remains Mirrorea/runtime realization, not Mir-0 semantics.

Do not use `durable_cut` in alpha samples as if it were implemented.

## 8. External effects and compensation

Classify effects by rollback behavior:

```text
pure/local reversible
local state reversible before atomic_cut
external reversible with explicit compensation
external irreversible
external isolated/provider-owned
```

Rollback region may not contain irreversible external effects unless isolated or compensated.

## 9. Required proof obligations

1. `atomic_cut` rollback boundary.
2. Consistent cut prefix closure.
3. No orphan receive.
4. No orphan observe.
5. No orphan witness use.
6. Hot-plug activation closure.
7. Load preserves consistency.
8. No lease/witness/membership resurrection.
9. Irreversible effect compensation/isolation.
10. Z-cycle checkpoint inadmissibility.

## 10. Required samples

Minimum:

- local try rollback before cut
- rollback across atomic_cut reject
- irreversible effect in rollback region reject
- local save/load valid
- inconsistent distributed snapshot reject
- in-flight message saved as channel state
- publish/observe consistency
- witness consistency
- hot-plug cut consistency
- expired lease not resurrected by load
- Z-cycle invalid checkpoint
- forced checkpoint or coordinated snapshot valid example if implemented
- `durable_cut` rejected/deferred in Mir-0

## 11. Extension points

Deferred:

- production distributed snapshot algorithm
- durable cut implementation
- durable migration/reattach
- distributed activation ordering
- storage backend
- replay service
- consensus mechanism
- exact external effect compensation algebra
