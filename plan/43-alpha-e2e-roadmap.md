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

### Stage C

- real transport narrow cut

### Stage D

- hot-plug lifecycle

### Stage E

- visualization / devtools

### Stage F

- Mirrorea Spaces alpha demo

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
- Docker-specific validation only when Docker lane is actually added

## stop lines

- do not mark Stage F complete without integrated runtime/network/hot-plug/devtools evidence
- do not call skeleton/planned sample rows runnable
- do not call Docker canary production transport
- do not call local save distributed save/load
- do not call Reversed Library implemented during Spaces alpha work

## next package

- immediate next package after roadmap integration:
  `P-A0-03` / `P-A0-04` sample skeleton and snapshot sync closeout
