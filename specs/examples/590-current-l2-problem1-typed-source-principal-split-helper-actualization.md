# 590. current-l2 Problem 1 typed source principal split helper actualization

## 位置づけ

- current Phase 6 / Package 117 closeout。
- Problem 1 の typed residual を theorem/model-check public-contract residual から切り離して読むための
  narrow helper/doc cut を actualize する。
- stronger typed source principal 自体、final typed calculus、final public verifier contract を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
   は Problem 1 typed source principal residual を、
   representative `p06`、supporting `p10 / p11 / p12 / p15 / p16`、
   kept-separate residual、stop line 付きで pretty summary に返してよい。
2. `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal --format json`
   は同じ読みを machine-readable manifest に返してよい。
3. `samples/problem-bundles/problem1-typed-theorem-model-check.md`
   は `typed source principal split の入口` section を持ち、
   split helper と同じ command / reading / stop line に揃えてよい。

## current recommendation

- Problem 1 residual を aggregate reopen map だけで読まず、
  typed residual を独立 helper と doc section で narrow に読む。
- `p06` representative と `p10 / p11 / p12 / p15 / p16` supporting set の役割差を保ち、
  theorem/model-check residual は kept-separate のまま残す。
- stronger typed source principal の actual adoption judgment はまだ上げず、
  split helper では current reopen point の独立可読化に留める。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
  - `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal --format json`
- docs:
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- stronger typed source principal actual adoption
- final typed calculus
- final public verifier contract

## retained-later line

- theorem public-contract split
- model-check public-contract split
- final public theorem/model-check contract families
