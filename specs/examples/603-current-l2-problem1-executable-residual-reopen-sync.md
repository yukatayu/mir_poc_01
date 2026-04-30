# 603. current-l2 Problem 1 executable residual reopen sync

## 位置づけ

- historical Phase 6 / Package 130 closeout memory。
- `specs/examples/597`、`600`、`601` の次段として、
  Problem 1 remaining mixed gate の reopen order を
  `check-source-sample`
  と
  `emit-theorem problem1`
  の historical helper-local reopen-order memory として記録する。
- final typed source principal、final public theorem contract、final public checker artifact、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
   は 2026-04-22 clean-sample migration 前の historical helper-local reopen-order anchor として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. Problem 1 sample bundle doc 側の
   `reopen-map problem1`
   と
   `lane problem1-final-public-seams`
   の導線は、
   `check-source-sample` → `emit-theorem problem1` → lane summary
   を historical helper-local reopen-order memory として読んでよい。
4. ただし、
   `reopen-map problem1`
   と
   `lane problem1-final-public-seams`
   自体の current-surface cooling は別 package の stale-docs maintenance に残す。

## current recommendation

- Problem 1 residual mixed gate は docs-only lane としてではなく、
  checker-adjacent executable slice と theorem-first emitted-artifact loop をつないでいた
  historical helper-local reopen-order memory として保ってよい。
- current cut は
  - checker-adjacent executable slice
  - theorem-first emitted-artifact loop
  - final-public-seam lane summary
  の連結に留める。
- historical `emit-theorem problem1` reopen order は helper-local / non-production drift suppression であり、
  current active command surface には戻さない。
- current helper update は helper-local / non-production drift suppression であり、
  final typed source principal、
  final public theorem contract、
  final public checker artifact、
  final public verifier contract
  には上げない。

## actualized evidence

- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`
  - current repo ではいずれも migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final typed source principal
- final public theorem contract
- final public checker artifact
- final public verifier contract

## retained-later line

- concrete theorem prover brand
- first settled property language
- final public parser / checker / runtime API
