# 592. current-l2 Problem 1 model-check public-contract split helper actualization

## 位置づけ

- current Phase 6 / Package 119 closeout。
- Problem 1 の model-check public-contract residual を typed residual / theorem residual から切り離して読むための
  narrow helper/doc cut を actualize する。
- first settled property language、final public checker artifact、final public verifier contract を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract`
   は Problem 1 model-check public-contract residual を、
   representative `p06`、supporting `p10 / p11 / p12 / p15 / p16`、
   kept-separate residual、stop line 付きで pretty summary に返してよい。
2. `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract --format json`
   は同じ読みを machine-readable manifest に返してよい。
3. `samples/problem-bundles/problem1-typed-theorem-model-check.md`
   は `model-check public-contract split の入口` section を持ち、
   split helper と同じ command / reading / stop line に揃えてよい。

## current recommendation

- Problem 1 residual を aggregate reopen map だけで読まず、
  model-check public-contract residual を独立 helper と doc section で narrow に読む。
- model-check line は row-local property route first / checker-artifact route first を保ち、
  typed source principal residual や theorem public-contract residual と混ぜない。
- final public checker contract judgment 自体はまだ上げず、
  split helper では current reopen point の独立可読化に留める。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract`
  - `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract --format json`
- docs:
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- first settled property language
- final public checker artifact
- final public verifier contract

## retained-later line

- typed source principal split
- theorem public-contract split
- final public checker / verifier contract families
