# 591. current-l2 Problem 1 theorem public-contract split helper actualization

## 位置づけ

- current Phase 6 / Package 118 closeout。
- Problem 1 の theorem public-contract residual を typed residual / model-check residual から切り離して読むための
  narrow helper/doc cut を actualize する。
- final public theorem contract、concrete theorem prover brand、final public verifier contract を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract`
   は Problem 1 theorem public-contract residual を、
   representative `p06`、Lean artifact / theorem-first pilot bundle 導線、
   kept-separate residual、stop line 付きで pretty summary に返してよい。
2. `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract --format json`
   は同じ読みを machine-readable manifest に返してよい。
3. `samples/problem-bundles/problem1-typed-theorem-model-check.md`
   は `theorem public-contract split の入口` section を持ち、
   split helper と同じ command / reading / stop line に揃えてよい。

## current recommendation

- Problem 1 residual を aggregate reopen map だけで読まず、
  theorem public-contract residual を独立 helper と doc section で narrow に読む。
- theorem line は review-unit transport first / notebook-consumer object first を保ち、
  typed source principal residual や model-check residual と混ぜない。
- final public theorem contract judgment 自体はまだ上げず、
  split helper では current reopen point の独立可読化に留める。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract`
  - `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract --format json`
- docs:
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final public theorem contract
- concrete theorem prover brand
- final public verifier contract

## retained-later line

- typed source principal split
- model-check public-contract split
- final public theorem/model-check/verifier contract families
