# 592. current-l2 Problem 1 model-check public-contract split helper actualization

## 位置づけ

- historical Phase 6 / Package 119 closeout memory。
- Problem 1 の model-check public-contract residual を typed residual / theorem residual から切り離して読むための
  narrow helper/doc cut を historical helper/doc memory として記録する。
- first settled property language、final public checker artifact、final public verifier contract を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract`
   は 2026-04-22 clean-sample migration 前の historical model-check split entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. archived Problem 1 sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   の `model-check public-contract split の入口` section で保持してよい。

## current recommendation

- Problem 1 residual を aggregate reopen map だけで読まず、
  model-check public-contract residual を独立 helper と archived doc section で narrow に読んでいた historical memory として保つ。
- model-check line は row-local property route first / checker-artifact route first を保ち、
  typed source principal residual や theorem public-contract residual と混ぜない。
- final public checker contract judgment 自体はまだ上げず、
  split helper では historical reopen point の独立可読化に留める。
- historical `split problem1 model-check-public-contract` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract`
  - `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract --format json`
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

- first settled property language
- final public checker artifact
- final public verifier contract

## retained-later line

- typed source principal split
- theorem public-contract split
- final public checker / verifier contract families
