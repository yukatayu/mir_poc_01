# plan/43 — alpha E2E roadmap

## purpose

この文書は、Mirrorea Spaces Alpha-0/Alpha-1 line の
stage/phase/E2E completion memory を repository-memory として置く。

ここで保持するのは current alpha-local package order と demo completion condition であり、
final public product completion ではない。

## stage A..F map

### Stage A

- existing repo-local alpha-ready floor
- clean-near-end suite
- Sugoroku
- avatar representative slice
- typed external preview
- network canary
- projection bridge evidence
- viewer prototype inventory
- hot-plug narrow runtime floor
- current imported-baseline closeout is fixed by:
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - `python3 scripts/clean_near_end_samples.py closeout`
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - `python3 scripts/avatar_follow_samples.py closeout --format json`
  - `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  - `python3 scripts/network_transport_samples.py check-all --format json`
  - `python3 scripts/projection_codegen_samples.py check-all --format json`
  - `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
  - `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- this counts as Stage A 100% for the current alpha line current scope
- this does not claim final public product, parser/runtime execution of `samples/alpha/*.mir`, active runnable-root promotion of `samples/alpha/`, or final public ABI freeze

### Stage B

- integrated local Mirrorea runtime
- first actualized subset is now a non-public Rust local-runtime floor over `mirrorea-core` substrate:
  local queue, `MessageEnvelope` dispatch, membership freshness rejection, and report-local event DAG export hook
- current-scope closeout is now fixed by:
  - `scripts/alpha_local_runtime_samples.py` for `LR-01/02`
  - `scripts/alpha_local_runtime_samples.py stage-b-closeout` reusing `CUT-04/17` as the local-only save/load subset
- this closes alpha-0.5 only for local runtime + local-only save/load subset; it does not claim distributed save/load or full CUT-family completion

### Stage C

- real transport narrow cut
- first actualized subset is now a non-public Rust Stage-C network floor over local TCP process-boundary / Docker Compose bridge:
  explicit membership freshness checks, capability/witness admission checks, observer-safe route trace, and auth-evidence lane preservation for `NET-02/03/04/05/07/09`
- current-scope closeout is now fixed by:
  - example `mirrorea_alpha_network_runtime -- closeout`
  - `scripts/alpha_network_docker_e2e.py stage-c-closeout --format json` over `NET-02/03/04/05/07/09`
- this closes alpha-0.7 only for the existing Docker/local-subprocess transport floor; it does not claim `NET-06/08/10`, WAN federation, partition completion, medium-substitution completion, or final public transport ABI

### Stage D

- hot-plug lifecycle
- first actualized subset is now a non-public Rust layer-insertion floor over the Stage-B local-runtime cut:
  attach-time contract comparison, accepted debug attach, rejected non-admin attach, explicit auth contract-update path, declared-failure rate-limit path, incompatible patch reject
- runtime-private package/avatar admission subset remains:
  `AV-01/02/06/08/09` and `HP-11/12/15`
- current-scope closeout is now fixed by:
  - example `mirrorea_alpha_layer_insertion_runtime -- closeout`
  - `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
  - `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json`
- this closes alpha-0.8 only for the existing attach-time layer subset plus runtime-private package/avatar admission subset; it does not claim detach runtime, durable migration, distributed activation ordering, native execution realization, or final public layer/package/avatar ABI

### Stage E

- visualization / devtools
- first actualized subset is now a dedicated Stage-E subset runner `scripts/alpha_visualization_samples.py` over existing alpha/helper/runtime evidence:
  `VIS-01/02/03/05/06/07/08/10/11`
- current subset is grounded in `LR-01.event_dag`, `NET-02.observer_route_trace`, `P16-VIEW-02`, `AV-08.representation_state`, `NET-07.observer_route_trace`, `LI-01/02.post_attach_trace_rows`, and `LI-01/05` retention-policy evidence
- current-scope closeout is now fixed by:
  - `python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json`
- `VIS-04/09/12` remain planned-only, but they are outside the current-scope Stage E closeout set
- `VIS-02` is limited to a report-local place-catalog projection over `LI-01`, and `VIS-05` is limited to a report-local membership epoch/incarnation timeline over `CUT-17`; neither freezes a final viewer API
- this closes alpha-0.9 only for the existing devtools subset; it does not claim `VIS-04/09/12`, final public viewer/telemetry API, or Stage F completion

### Stage F

- Mirrorea Spaces alpha demo
- first actualized subset is now a thin integrated bridge runner `scripts/alpha_e2e_samples.py`:
  `E2E-01/02/03/04/05/06/07/09/10`
- `E2E-06` is now backed by the local-only `CUT-04` save/load bridge
- `CUT-17` local stale-membership rejection and `CUT-11` checker-backed Z-cycle inadmissibility now narrow the integrated save/load non-claim surface without implying distributed repair/runtime completion
- `E2E-08` upper-layer seed remains outside the actualized subset
- current-scope closeout is now fixed by:
  - `python3 scripts/alpha_e2e_samples.py stage-f-closeout --format json`
- this closes alpha-1 only for `E2E-01/02/03/04/05/06/07/09/10` plus the current-scope Stage E closeout surface; it does not claim `E2E-08`, distributed save/load completion, active runnable-root promotion, public alpha / `U1`, or final public runtime/viewer/transport/hot-plug ABI

## phase 0..8 packages

- `Phase 0`
  theory freeze / roadmap sync / sample skeletons
- `Phase 1`
  typed IR / checker skeleton
- `Phase 2`
  executable semantics / event DAG
- `Phase 3`
  local runtime
- `Phase 4`
  layer insertion runtime
- `Phase 5`
  network / Docker E2E
- `Phase 6`
  save/load and consistent cut
- `Phase 7`
  runtime package / avatar adapter
- `Phase 8`
  integrated alpha demo

## Docker E2E path

minimum path:

1. local runtime skeleton
2. transport trait / subprocess or TCP narrow cut
3. Docker world + participant shape
4. stale membership / missing capability / missing witness negatives
5. observer-safe route trace
6. auth-evidence lane preserved separately from transport delivery

Docker E2E remains evidence of narrow alpha cut, not production WAN federation.

## Mirrorea Spaces alpha demo completion condition

current-scope Stage F closeout requires at least:

- local/runtime Place execution
- network/container E2E
- runtime package hot-plug
- layer hot-plug
- typed debug/devtools output
- placeholder/custom avatar runtime path
- local save/load or explicit checker-backed non-claim for distributed save
- negative tests for stale membership / missing capability / missing witness / invalid cut / incompatible patch / unsigned native package reject

## validation floor

Phase 0 docs/sample-skeleton floor:

- `find samples/alpha -maxdepth 3 -type f | sort`
- `python3 -m unittest scripts.tests.test_current_l2_family_snapshot_support scripts.tests.test_alpha_lifetime_fallback_snapshot`
- `python3 -m unittest scripts.tests.test_current_l2_family_anchor_handoff_support scripts.tests.test_alpha_lifetime_fallback_anchor_handoff`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

Phase 1+ runtime floor:

- focused Cargo tests for changed crates
- alpha sample-family runner checks once such runners exist
- current Phase 3 first-cut floor:
  `cargo test -p mir-runtime --test alpha_local_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership`
- current Phase 4 first-cut floor:
  `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout`
- current Phase 5 first-cut floor:
  `cargo test -p mir-runtime --test alpha_network_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout`
  `python3 scripts/alpha_network_docker_e2e.py check-all --format json`
  `python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json`
- current Phase 7 first-cut floor:
  `cargo test -p mir-runtime --test alpha_avatar_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`
  `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
  `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json`
- current Stage E subset floor:
  `python3 scripts/alpha_visualization_samples.py check-all --format json`
  `python3 scripts/alpha_visualization_samples.py closeout --format json`
  `python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json`
  `python3 -m unittest scripts.tests.test_alpha_visualization_samples`
- current Phase 6 first-cut floor:
  `cargo test -p mirrorea-core --test runtime_substrate`
  `cargo test -p mir-runtime --test alpha_local_runtime`
  `cargo test -p mir-runtime --test alpha_cut_save_load_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership`
  `python3 scripts/alpha_cut_save_load_samples.py check-all --format json`
- current Phase 8 bridge floor:
  `cargo test -p mirrorea-core --test runtime_substrate`
  `cargo test -p mir-runtime --test alpha_local_runtime --test alpha_layer_insertion_runtime --test alpha_network_runtime --test alpha_avatar_runtime`
  `cargo test -p mir-runtime --test alpha_cut_save_load_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`
  `python3 scripts/alpha_cut_save_load_samples.py check-all --format json`
  `python3 scripts/alpha_network_docker_e2e.py check-all --format json`
  `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
  `python3 scripts/alpha_e2e_samples.py run E2E-06 --format json`
  `python3 scripts/alpha_e2e_samples.py check-all --format json`
  `python3 scripts/alpha_e2e_samples.py closeout --format json`
  `python3 scripts/alpha_e2e_samples.py stage-f-closeout --format json`
  `python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_e2e_samples`

## stop lines

- do not treat current-scope Stage F closeout as public alpha / `U1` completion
- do not treat imported Stage A baseline revalidation as a new runtime semantics package
- do not call skeleton/planned sample rows runnable
- do not call Docker canary production transport
- do not call local save distributed save/load
- do not treat checker-backed invalid distributed cut rejection as distributed save/load runtime completion
- do not call `VIS-04/09/12` implemented or part of the current-scope Stage E closeout
- do not call `E2E-08` part of the current-scope Stage F closeout set
- do not call Reversed Library implemented during Spaces alpha work

## next reopen point

- after `P-A0-23`, Stage B current-scope closeout is fixed and queue authority shifts to Stage C-first completion work
- after `P-A0-24`, Stage C current-scope closeout is fixed and queue authority shifts to Stage D lifecycle closeout
- after `P-A0-25`, Stage D current-scope closeout is fixed and queue authority shifts to Stage E devtools closeout
- after `P-A0-26`, Stage E current-scope closeout is fixed and queue authority shifts to Stage F integrated alpha closeout
- after `P-A0-27`, current-scope Stage F closeout is fixed and the large-stage-first alpha line is complete for current scope; next reopen must choose a later-family blocker lane or public-boundary lane rather than widening Stage F itself
- after `P-A0-28`, Stage A imported baseline is rerun and the large-stage-first alpha line reads sequentially from Stage A through Stage F for current scope; next reopen still must choose a later-family blocker lane or public-boundary lane
- later helper-local LIF/VAR carriers remain valid and separate, but they are no longer the next promoted line while the large-stage-first sequence is active
- no safe next package is auto-promoted here; `CUT-10/12/16`, `LIF-15`, `VAR-14`, detach/migration/native, and `U1` remain separate reopen candidates
