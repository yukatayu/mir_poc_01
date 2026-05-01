# plan/41 — save-load checkpoint roadmap

## purpose

この文書は、Alpha-0 line における
`specs/15-cut-save-load-checkpoint.md`
の repository-memory roadmap を置く。

ここで保持するのは `atomic_cut` retained boundary と local-save/consistent-cut first line であり、
distributed durable save/load completion ではない。

## current repo state

- existing repo already fixes:
  - `atomic_cut` is place-local
  - order / witness / handoff / hot-plug boundaries are explicit
  - hot-plug activation cut is separate from durable/distributed ordering
- existing executable floor already covers:
  - `try` / rollback / `atomic_cut`
  - place-local runtime examples
- Alpha-0 save/load sample family is now present under `samples/alpha/cut-save-load/`
- cut-validity checker naming and executable predicate surface are still pending

## decisions mirrored from specs/15

- `atomic_cut` does not become global/durable/distributed commit
- distributed save/load requires consistent cut
- in-flight message / publish-observe / witness / hot-plug closure are part of validity
- load does not resurrect stale lease / witness / membership
- Z-cycle/useless-checkpoint risk must stay explicit

## local save/load roadmap

### first safe cut

- sample skeletons for local save/load valid / invalid distributed cut / `durable_cut` deferred
- docs-first boundary for save-state content inventory
- no runtime claim beyond local scope

### next executable cut

- consistent-cut predicate/checker
- local save/load skeleton
- invalid distributed snapshot reject
- explicit channel-state/in-flight representation
- capability-grant / auth-evidence closure checks
- stale witness / stale membership non-resurrection checks

## Docker / distributed negative sample roadmap

- keep negative samples separate from actual distributed snapshot completion
- highlight:
  - orphan receive
  - orphan observe
  - orphan witness use
  - orphan capability use
  - orphan auth-evidence use
  - orphan hot-plug activation
  - Z-cycle invalid checkpoint

## deferred durable/distributed protocol

- coordinated snapshot algorithm
- communication-induced checkpointing
- durable replay service
- storage backend
- consensus
- distributed migration / durable activation ordering

## next package

- after checker first cut:
  add consistent-cut checker skeleton or explicit planned status for CUT family rows
