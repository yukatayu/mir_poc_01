# 591. current-l2 Problem 1 theorem public-contract split helper actualization

## 位置づけ

- historical Phase 6 / Package 118 closeout memory。
- Problem 1 の theorem public-contract residual を typed residual / model-check residual から切り離して読むための
  narrow helper/doc cut を historical helper/doc memory として記録する。
- final public theorem contract、concrete theorem prover brand、final public verifier contract を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract`
   は 2026-04-22 clean-sample migration 前の historical theorem split entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. archived Problem 1 sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   の `theorem public-contract split の入口` section で保持してよい。

## current recommendation

- Problem 1 residual を aggregate reopen map だけで読まず、
  theorem public-contract residual を独立 helper と archived doc section で narrow に読んでいた historical memory として保つ。
- theorem line は review-unit transport first / notebook-consumer object first を保ち、
  typed source principal residual や model-check residual と混ぜない。
- final public theorem contract judgment 自体はまだ上げず、
  split helper では historical reopen point の独立可読化に留める。
- historical `split problem1 theorem-public-contract` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract`
  - `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public theorem contract
- concrete theorem prover brand
- final public verifier contract

## retained-later line

- typed source principal split
- model-check public-contract split
- final public theorem/model-check/verifier contract families
