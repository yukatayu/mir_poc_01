# 601. current-l2 theorem-first emitted-artifact loop hardening

## 位置づけ

- historical Phase 6 / Package 128 closeout memory。
- `specs/examples/508`、`509`、`575` と Package 127 の typed-checker executable slice の次段として、
  Problem 1 theorem-first pilot を repo-local output dir へ materialize していた
  historical `emit-theorem problem1`
  helper memory を記録する。
- final public theorem contract、concrete theorem prover brand、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
   は 2026-04-22 clean-sample migration 前の historical helper-local emitted-artifact loop entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. historical emitted output dir は
   `target/current-l2-guided/problem1-theorem-pilot`
   に固定する。
4. representative theorem line は
   `p06-typed-proof-owner-handoff`
   を primary とし、
   `p07-dice-late-join-visible-history`
   と
   `p08-dice-stale-reconnect-refresh`
   を theorem-reached support pair として同じ historical emitted-artifact loop memory に載せてよい。
5. Problem 1 sample bundle doc 側の `emit-theorem problem1` step は
   historical helper memory として読む。

## current recommendation

- theorem-first pilot は docs / Lean corpus / anchor refs だけでなく、
  repo-local output dir へ bundle JSON を出していた historical emitted-artifact loop memory として保ってよい。
- current cut は
  - notebook-first theorem line
  - brand-neutral emitter example
  - representative theorem line `p06 / p07 / p08`
  - repo-local output dir
  に留める。
- historical `emit-theorem problem1` は helper-local / non-production memory であり、
  final public theorem contract、
  concrete theorem prover brand、
  final public verifier contract
  には上げない。

## actualized evidence

- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- historical helper commands:
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public theorem contract
- concrete theorem prover brand
- final public verifier contract

## retained-later line

- proof object public schema
- concrete theorem/model-check production binding
- final public parser / checker / runtime API
