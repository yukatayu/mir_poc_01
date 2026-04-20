# 601. current-l2 theorem-first emitted-artifact loop hardening

## 位置づけ

- current Phase 6 / Package 128 closeout。
- `specs/examples/508`、`509`、`575` と Package 127 の typed-checker executable slice の次段として、
  Problem 1 theorem-first pilot を repo-local output dir へ materialize する
  `emit-theorem problem1`
  helper を actualize する。
- final public theorem contract、concrete theorem prover brand、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
   は `current_l2_emit_theorem_lean_bundle` example を
   prototype sample path + adjacent host-plan path 付きで呼び、
   `target/current-l2-guided/problem1-theorem-pilot`
   へ bundle JSON を materialize してよい。
2. representative theorem line は
   `p06-typed-proof-owner-handoff`
   を primary とし、
   `p07-dice-late-join-visible-history`
   と
   `p08-dice-stale-reconnect-refresh`
   を theorem-reached support pair として同じ emitted-artifact loop に載せてよい。
3. Problem 1 sample bundle doc は
   `check-source-sample`
   に加えて
   `emit-theorem problem1`
   を execution step として案内してよい。

## current recommendation

- theorem-first pilot は docs / Lean corpus / anchor refs だけでなく、
  repo-local output dir へ bundle JSON を出す emitted-artifact loop として保ってよい。
- current cut は
  - notebook-first theorem line
  - brand-neutral emitter example
  - representative theorem line `p06 / p07 / p08`
  - repo-local output dir
  に留める。
- `emit-theorem problem1` は helper-local / non-production command であり、
  final public theorem contract、
  concrete theorem prover brand、
  final public verifier contract
  には上げない。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- runtime commands:
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`

## stop line

- final public theorem contract
- concrete theorem prover brand
- final public verifier contract

## retained-later line

- proof object public schema
- concrete theorem/model-check production binding
- final public parser / checker / runtime API
