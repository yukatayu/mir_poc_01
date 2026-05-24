# plan/65 — indexed state roadmap

## purpose

This document is repository memory for `specs/40-indexed-state-semantics.md`.

It keeps the Surface Mir indexed-state line focused on storage owner / keyspace /
authority separation.

## current decision

Decided:

- `S { state player[p: Participant]: Player }` declares an S-owned partial map.
- `p` is a key, not owner authority.
- write authority comes from owner locus or explicit capability.
- join/leave create active / retired / tombstoned key states.
- compaction is blocked by in-flight messages, live witnesses, live leases /
  fallback refs, retained savepoints, or retention policy.

Not decided:

- arbitrary unconstrained storage maps.
- distributed durable compaction.
- final storage backend.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-SURF-00B` | docs/spec rebaseline | indexed-state semantics and roadmap exist |
| `P-SURF-02` | indexed state AST / checker | closed: owner, keyspace, value type, visible fields, key-not-authority rejection, stale-key rejection, retained-savepoint compaction rejection, owner-scoped state names, and nested-place ambient-authority rejection are represented |
| `P-SURF-03` | elaboration integration | cross-locus indexed reads/writes generate Core obligations |
| `P-SURF-07` | source operational rows | WorldCore / MembershipChat / Sugoroku source roots exercise indexed state |
| `P-SURF-08` | devtools | closed: semantic-checker-backed indexed-state map and source-span summary are visible; active/tombstoned key timeline remains later |

## planned rows

- `IDX-01` server-owned participant-indexed state accepted.
- `IDX-02` key write without capability rejected.
- `IDX-03` stale key access after leave rejected.
- `IDX-04` compaction blocked by retained savepoint evidence rejected.
- `IDX-05` nested place block ambient-authority bypass rejected.

## validation anchors

Current anchors:

```bash
python3 scripts/surface_mir_samples.py run IDX-01 --format json
python3 scripts/surface_mir_samples.py run IDX-02 --format json
python3 scripts/surface_mir_samples.py run IDX-03 --format json
python3 scripts/surface_mir_samples.py run IDX-04 --format json
python3 scripts/surface_mir_samples.py run IDX-05 --format json
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
```

## stop lines

- do not treat key as authority.
- do not resurrect stale membership / witness / lease / indexed entry on load.
- do not mark conceptual rows workflow-ready before helper/runtime evidence exists.
