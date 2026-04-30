# 590. current-l2 Problem 1 typed source principal split helper actualization

## 位置づけ

- historical Phase 6 / Package 117 closeout memory。
- Problem 1 の typed residual を theorem/model-check public-contract residual から切り離して読むための
  narrow helper/doc cut を historical helper/doc memory として記録する。
- stronger typed source principal 自体、final typed calculus、final public verifier contract を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
   は 2026-04-22 clean-sample migration 前の historical typed split entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. archived Problem 1 sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   の `typed source principal split の入口` section で保持してよい。

## current recommendation

- Problem 1 residual を aggregate reopen map だけで読まず、
  typed residual を独立 helper と archived doc section で narrow に読んでいた historical memory として保つ。
- `p06` representative と `p10 / p11 / p12 / p15 / p16` supporting set の役割差を保ち、
  theorem/model-check residual は kept-separate のまま残す。
- stronger typed source principal の actual adoption judgment はまだ上げず、
  split helper では historical reopen point の独立可読化に留める。
- historical `split problem1 typed-source-principal` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
  - `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal --format json`
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

- stronger typed source principal actual adoption
- final typed calculus
- final public verifier contract

## retained-later line

- theorem public-contract split
- model-check public-contract split
- final public theorem/model-check contract families
