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
- `P-A0-06` first checker-floor helper actualization now exists via:
  - `scripts/alpha_cut_save_load_checker.py`
  - `samples/alpha/cut-save-load/cut-05` / `07` / `08` / `09` / `13` / `14` / `15` sidecars
  synthetic detached artifact comparison only; no distributed save/load or runtime bridge claim
- `P-A0-12` local-only save/load positive bridge now exists via:
  - `crates/mir-runtime/src/alpha_local_runtime.rs`
  - example `mirrorea_alpha_local_runtime -- save-load-resume`
  - `scripts/alpha_cut_save_load_samples.py`
  - `samples/alpha/cut-save-load/cut-04-local_save_load_valid.expected.json`
  this proves only room-local runtime savepoint -> restore -> resumed dispatch over the existing runtime floor; it does not widen to distributed/durable save/load
- current first cut intentionally does **not** claim:
  full Z-cycle graph handling, communication-induced checkpoint repair, load rejection vs stale-preserving load split, or membership-dependent dispatch closure coverage

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

### current executable cut

- `CUT-04` local-only save/load positive bridge
- consistent-cut predicate/checker first cut for `CUT-05/07/08/09/13/14/15`
- explicit distributed-save non-claim remains in place

### next executable cut

- consistent-cut predicate/checker widening beyond the first orphan/deferred slice
- invalid distributed snapshot reject
- explicit channel-state/in-flight representation
- hot-plug activation closure checks
- Z-cycle structural model
- membership-dependent dispatch closure row or equivalent sample
- stale witness / stale membership non-resurrection checks with verdict split

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

- after `P-A0-13` dedicated Stage-E subset runner closeout:
  `P-A0-14` widens the remaining CUT family carefully via Z-cycle structure, stale witness/membership load verdict split, and membership-dependent dispatch closure without claiming distributed durable save/load
