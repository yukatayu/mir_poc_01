# 604. current-l2 Problem 2 executable residual reopen sync

## 位置づけ

- current Phase 6 / Package 131 closeout。
- `specs/examples/598`、`602`、`603` の次段として、
  Problem 2 remaining mixed gate の reopen order を
  `emit-scenario problem2`
  の executable scenario loop 側へ再同期する。
- final source-surface handoff wording、final public witness/provider/artifact contract、exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
   は Problem 2 current floor として
   `emit-scenario problem2`
   を含んでよい。
2. `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
   は entry command として
   `emit-scenario problem2`
   `reopen-map problem2`
   `residuals`
   を示してよい。
3. Problem 2 sample bundle doc は
   `reopen-map problem2`
   から
   `emit-scenario problem2` → `lane problem2-final-public-seams` → `residuals`
   の current reopen order を案内してよい。

## current recommendation

- Problem 2 residual mixed gate は docs-only lane としてではなく、
  executable scenario evidence を前段に置いた reopen order として保ってよい。
- current cut は
  - authoritative-room runnable scenario loop
  - final-public-seam lane summary
  - residual summary
  の連結に留める。
- current helper update は helper-local / non-production drift suppression であり、
  final source-surface handoff wording、
  final public witness/provider/artifact contract、
  exhaustive shared-space catalog
  には上げない。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
- helper commands:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2 --format json`
  - `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json`

## stop line

- final source-surface handoff wording
- final public witness/provider/artifact contract
- exhaustive shared-space catalog

## retained-later line

- stronger fairness / replay profile
- final public parser / checker / runtime API
- packaging / installed binary / FFI / engine adapter
