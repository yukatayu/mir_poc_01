# 602. current-l2 authoritative-room runnable scenario loop hardening

## 位置づけ

- current Phase 6 / Package 129 closeout。
- `specs/examples/570`、`571`、`576` と Package 128 の emitted-artifact loop hardening の次段として、
  Problem 2 authoritative-room first scenario を repo-local output dir へ materialize する
  `emit-scenario problem2`
  helper を actualize する。
- final source wording、final public witness/provider/artifact contract、exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
   は representative pair `p07 / p08`、reserve route `p09`、negative static-stop pair `p13 / p14`
   の `run-source-sample --format json` output を
   `target/current-l2-guided/problem2-scenario-bundle`
   へ materialize してよい。
2. `p07` と `p08` は first-line representative、
   `p09` は reserve practical route、
   `p13` と `p14` は negative static-stop
   のまま同じ scenario loop に載せてよい。
3. Problem 2 sample bundle doc は
   `smoke problem2`
   に加えて
   `emit-scenario problem2`
   を execution step として案内してよい。

## current recommendation

- authoritative-room first scenario は docs / matrix / bundle だけでなく、
  repo-local output dir へ JSON を出す runnable scenario loop として保ってよい。
- current cut は
  - authoritative-room first default profile
  - representative pair `p07 / p08`
  - reserve route `p09`
  - negative pair `p13 / p14`
  - repo-local output dir
  に留める。
- `emit-scenario problem2` は helper-local / non-production command であり、
  final source wording、
  final public witness/provider/artifact contract、
  exhaustive shared-space catalog
  には上げない。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- runtime commands:
  - `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  - `python3 scripts/current_l2_guided_samples.py emit-scenario problem2 --format json`

## stop line

- final source-surface handoff wording
- final public witness/provider/artifact contract
- stronger fairness / replay profile
- exhaustive shared-space catalog

## retained-later line

- delegated RNG provider contract finalization
- auditable authority witness strengthening
- final public parser / checker / runtime API
