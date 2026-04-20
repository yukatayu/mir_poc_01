# 603. current-l2 Problem 1 executable residual reopen sync

## 位置づけ

- current Phase 6 / Package 130 closeout。
- `specs/examples/597`、`600`、`601` の次段として、
  Problem 1 remaining mixed gate の reopen order を
  `check-source-sample`
  と
  `emit-theorem problem1`
  の executable evidence 側へ再同期する。
- final typed source principal、final public theorem contract、final public checker artifact、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
   は Problem 1 current floor として
   `check-source-sample`
   と
   `emit-theorem problem1`
   を含んでよい。
2. `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
   は entry command として
   `check-source-sample`
   `emit-theorem problem1`
   `reopen-map problem1`
   を示してよい。
3. Problem 1 sample bundle doc は
   `reopen-map problem1`
   から
   `check-source-sample` → `emit-theorem problem1` → `lane problem1-final-public-seams`
   の current reopen order を案内してよい。

## current recommendation

- Problem 1 residual mixed gate は docs-only lane としてではなく、
  executable evidence を前段に置いた reopen order として保ってよい。
- current cut は
  - checker-adjacent executable slice
  - theorem-first emitted-artifact loop
  - final-public-seam lane summary
  の連結に留める。
- current helper update は helper-local / non-production drift suppression であり、
  final typed source principal、
  final public theorem contract、
  final public checker artifact、
  final public verifier contract
  には上げない。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
- helper commands:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`

## stop line

- final typed source principal
- final public theorem contract
- final public checker artifact
- final public verifier contract

## retained-later line

- concrete theorem prover brand
- first settled property language
- final public parser / checker / runtime API
