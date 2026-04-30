# 604. current-l2 Problem 2 executable residual reopen sync

## 位置づけ

- historical Phase 6 / Package 131 closeout memory。
- `specs/examples/598`、`602`、`603` の次段として、
  Problem 2 remaining mixed gate の reopen order を
  `emit-scenario problem2`
  の historical helper-local reopen-order memory として記録する。
- final source-surface handoff wording、final public witness/provider/artifact contract、exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
   は 2026-04-22 clean-sample migration 前の historical helper-local reopen-order anchor として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. Problem 2 sample bundle doc 側の
   `reopen-map problem2`
   `lane problem2-final-public-seams`
   `residuals`
   の導線は、
   `emit-scenario problem2` → lane summary → residual summary
   を historical helper-local reopen-order memory として読んでよい。
4. ただし、
   `reopen-map problem2`
   `lane problem2-final-public-seams`
   `residuals`
   自体の current-surface cooling は別 package の stale-docs maintenance に残す。

## current recommendation

- Problem 2 residual mixed gate は docs-only lane としてではなく、
  authoritative-room runnable scenario loop を前段に置いていた
  historical helper-local reopen-order memory として保ってよい。
- current cut は
  - authoritative-room runnable scenario loop
  - final-public-seam lane summary
  - residual summary
  の連結に留める。
- historical `emit-scenario problem2` reopen order は helper-local / non-production drift suppression であり、
  current active command surface には戻さない。
- current helper update は helper-local / non-production drift suppression であり、
  final source-surface handoff wording、
  final public witness/provider/artifact contract、
  exhaustive shared-space catalog
  には上げない。

## actualized evidence

- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2 --format json`
  - `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json`
  - current repo ではいずれも migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final source-surface handoff wording
- final public witness/provider/artifact contract
- exhaustive shared-space catalog

## retained-later line

- stronger fairness / replay profile
- final public parser / checker / runtime API
- packaging / installed binary / FFI / engine adapter
