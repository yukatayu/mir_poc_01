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

### Stage B

- integrated local Mirrorea runtime
- first actualized subset is now a non-public Rust local-runtime floor over `mirrorea-core` substrate:
  local queue, `MessageEnvelope` dispatch, membership freshness rejection, and report-local event DAG export hook

### Stage C

- real transport narrow cut
- first actualized subset is now a non-public Rust Stage-C network floor over local TCP process-boundary / Docker Compose bridge:
  explicit membership freshness checks, capability/witness admission checks, observer-safe route trace, and auth-evidence lane preservation for `NET-02/03/04/05/07/09`

### Stage D

- hot-plug lifecycle
- first actualized subset is now a non-public Rust layer-insertion floor over the Stage-B local-runtime cut:
  attach-time contract comparison, accepted debug attach, rejected non-admin attach, explicit auth contract-update path, declared-failure rate-limit path, incompatible patch reject

### Stage E

- visualization / devtools
- first actualized subset is now a dedicated Stage-E subset runner `scripts/alpha_visualization_samples.py` over existing alpha/helper/runtime evidence:
  `VIS-01/02/03/05/06/07/08/10/11`
- current subset is grounded in `LR-01.event_dag`, `NET-02.observer_route_trace`, `P16-VIEW-02`, `AV-08.representation_state`, `NET-07.observer_route_trace`, `LI-01/02.post_attach_trace_rows`, and `LI-01/05` retention-policy evidence
- `VIS-04/09/12` remain planned-only, so this is still not Stage E completion
- `VIS-02` is limited to a report-local place-catalog projection over `LI-01`, and `VIS-05` is limited to a report-local membership epoch/incarnation timeline over `CUT-17`; neither freezes a final viewer API

### Stage F

- Mirrorea Spaces alpha demo
- first actualized subset is now a thin integrated bridge runner `scripts/alpha_e2e_samples.py`:
  `E2E-01/02/03/04/05/06/07/09/10`
- `E2E-06` is now backed by the local-only `CUT-04` save/load bridge
- `CUT-17` local stale-membership rejection and `CUT-11` checker-backed Z-cycle inadmissibility now narrow the integrated save/load non-claim surface without implying distributed repair/runtime completion
- `E2E-08` upper-layer seed remains outside the actualized subset
- current reading is `Stage F bridge + local save/load path actualized, Stage F completion still blocked on dedicated alpha devtools`

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
- current Phase 7 first-cut floor:
  `cargo test -p mir-runtime --test alpha_avatar_runtime`
  `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`
  `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
- current Stage E subset floor:
  `python3 scripts/alpha_visualization_samples.py check-all --format json`
  `python3 scripts/alpha_visualization_samples.py closeout --format json`
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
  `python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_e2e_samples`

## stop lines

- do not mark Stage F complete without integrated runtime/network/hot-plug/devtools evidence
- do not call skeleton/planned sample rows runnable
- do not call Docker canary production transport
- do not call local save distributed save/load
- do not treat checker-backed invalid distributed cut rejection as distributed save/load runtime completion
- do not call the current Stage-E subset runner Stage E completion
- do not call the current Stage-F bridge runner dedicated alpha visualization/devtools completion
- do not call Reversed Library implemented during Spaces alpha work

## next package

- after `P-A0-10` runtime package / avatar first cut, `P-A0-11` actualizes the thin integrated demo bridge
- after `P-A0-15`, the next reopen point is:
  - `P-A0-16` selected LIF/VAR checker widening after the widened Stage-E subset sync
    - widen selected positive/static lifetime rows through the existing synthetic checker floor when the spec text is already settled
    - widen selected positive/static contract rows through the existing synthetic checker floor when the overlay law is already settled
    - keep parser/runtime/theorem-backed completion explicitly later
