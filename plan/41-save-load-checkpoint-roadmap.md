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
- `P-A0-14` honest CUT widening now also exists via:
  - `crates/mir-runtime/src/alpha_local_runtime.rs`
  - example `mirrorea_alpha_local_runtime -- save-load-stale-membership`
  - `scripts/alpha_cut_save_load_samples.py`
  - `scripts/alpha_cut_save_load_checker.py`
  - `samples/alpha/cut-save-load/cut-11-zcycle_checkpoint_invalid.expected.json`
  - `samples/alpha/cut-save-load/cut-17-load_does_not_resurrect_stale_membership.expected.json`
  this proves only two additional narrow facts:
  - `CUT-17`: a restored local savepoint does not resurrect stale membership into accepted resumed dispatch after a later membership-frontier advance
  - `CUT-11`: a useless checkpoint Z-cycle is inadmissible in the checker floor
- current first cut intentionally still does **not** claim:
  communication-induced checkpoint repair, stale lease/witness load verdict split, distributed save/load runtime, or durable cut completion

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
- `CUT-17` local stale-membership rejection bridge
- consistent-cut predicate/checker first cut for `CUT-05/07/08/09/11/13/14/15`
- explicit distributed-save non-claim remains in place

### next executable cut

- invalid distributed snapshot reject widening only if a real distributed-cut carrier is introduced
- explicit channel-state/in-flight representation
- `CUT-12` communication-induced checkpoint repair once protocol state is specified
- `CUT-10/16` stale lease/witness non-resurrection rows once lease/witness stores exist in the saved carrier
- any broader membership-dependent dispatch closure only if it remains local-scope honest

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

- after `P-A0-14` honest CUT widening closeout:
  CUT-local reopen stays blocked on `CUT-10/12/16` because lease/witness store and communication-induced checkpoint protocol are not yet actualized. The next promoted package should therefore prefer Stage-E remaining-row widening unless a new save/load substrate package is explicitly opened.
