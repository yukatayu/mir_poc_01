# 40 — Indexed State Semantics

## role

This document fixes the Surface Mir alpha semantics for indexed state such as:

```mir
S {
  state player[p: Participant]: Player
    init Player { hp: 100, atk: 10 }
}
```

The key correction is that this declaration creates an `S`-owned indexed state
map. The key is not the owner and is not authority.

## decision level

- `L1`
  - indexed state is owned by its declaring place / locus.
  - a key such as `p: Participant` does not grant read or write authority.
  - membership freshness, incarnation, witness, and save/load cut constraints
    apply to indexed entries.
- `L2`
  - alpha keyspace restrictions.
  - tombstone / compaction conditions.
  - dependency-cycle and devtools carrier shape.

## mathematical reading

For:

```mir
S {
  state player[p: Participant]: Player
}
```

the mathematical reading is:

```text
player_S : Active(Participant, epoch) ⇀ Player
```

Components:

```text
owner_locus = S
state_name = player
keyspace = Participant
key = p
value_type = Player
```

`player[Alice]` is stored at `S`. Alice is the key, not the owner.

## access rules

Read:

```text
requires active key or declared historical-read mode
requires local owner locus or observe / visibility authority
requires membership epoch / incarnation freshness unless historical mode says otherwise
```

Write:

```text
requires current_locus == owner_locus OR explicit write capability for an
owner-directed generated request/effect
requires active key
requires failure row covering stale key / missing authority cases
```

Allowed local owner update:

```mir
S {
  player[target].hp -= player[self].atk
}
```

Remote-capability write:

```text
Participant[self] requests update of S-owned player[target]
  -> generated owner-directed request/effect at S
  -> capability/freshness/witness/failure-row checks
  -> S performs or rejects the state mutation
```

A capability authorizes the request path. It does not permit a direct remote
store into S-owned indexed state.

Rejected authority confusion:

```text
Participant Alice is key of player[Alice]
therefore Alice may write player[Alice]
```

That inference is invalid.

## lifecycle

Join:

```text
join(p):
  membership_epoch += 1
  incarnation[p] becomes current
  active[p] = true
  allocate or initialize indexed entries for p
```

Leave:

```text
leave(p):
  membership_epoch += 1
  active[p] = false
  mark indexed entries retired / tombstoned
```

Leave does not immediately drop entries.

Compaction may drop an entry only if all are true:

```text
no in-flight message references the key
no live witness references the key
no live lease/fallback references the key
no retained savepoint references the key
audit retention policy permits deletion
```

## save/load relation

Indexed state lives inside the existing `SaveObject` / consistent cut model from
`specs/20-cut-save-load-semantics.md`.

Load must not resurrect:

- stale membership.
- stale participant incarnation.
- stale witness.
- expired lease.
- retired indexed entry as active state.
- hidden fallback or compaction state.

If reacquisition is needed, the runtime must produce a new event / epoch /
witness rather than silently repairing an old key.

## alpha keyspace restrictions

Alpha supports:

```text
role keyspace: Participant
object keyspace: Object, later
avatar keyspace: Avatar, later
```

Alpha does not support arbitrary unconstrained maps as storage-owner indexed
state. Ordinary maps may exist as value-level data, but they do not gain the
membership / authority / lifecycle semantics of indexed state unless declared
through this state form.

## dependency graph

Indexed state dependencies are schema-level graph edges.

Allowed:

```text
score[p] depends on player[p]
ranking depends on score[*]
```

Danger:

```text
score[p] depends on ranking
ranking depends on score[*]
```

The latter can create a cycle. It must be rejected or converted into an explicit
residual proof / model-check obligation. It must not become an implicit
back-edge in the semantic event graph.

## devtools requirements

Observer-safe devtools must expose:

- owner locus.
- keyspace.
- active keys.
- retired / tombstoned keys.
- access source spans.
- generated communication for cross-locus access.
- rejected stale-key or missing-capability attempts.

Observer-safe views must not expose raw witness payloads, raw auth evidence, or
private capability grants.

## required alpha sample rows

- `IDX-01`: server-owned participant-indexed state accepted.
- `IDX-02`: key write without owner locus or capability rejected.
- `IDX-03`: stale key access after leave rejected at runtime.
- `IDX-04`: compaction blocked by retained savepoint evidence is rejected in the
  P-SURF-02 checker floor; witness / in-flight reference blockers remain
  lifecycle obligations for later runtime carriers.
- `IDX-05`: nested `S { ... }` from a non-owner locus does not become ambient
  owner authority; it must elaborate to an owner-directed request before any
  indexed-state write can be admitted.

## non-claims

This document does not claim:

- arbitrary dependent map semantics.
- distributed durable compaction protocol.
- final public state grammar.
- final storage backend.
