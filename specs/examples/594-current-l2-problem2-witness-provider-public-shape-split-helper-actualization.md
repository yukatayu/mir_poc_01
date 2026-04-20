# 594. current-l2 Problem 2 witness-provider public-shape split helper actualization

## 位置づけ

- current Phase 6 / Package 121 closeout。
- Problem 2 の witness/provider public-shape residual を source wording / emitted schema residual から切り離して読むための
  narrow helper/doc cut を actualize する。
- final public witness/provider/artifact contract、stronger fairness / replay profile、exhaustive shared-space catalog を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape`
   は Problem 2 witness/provider public-shape residual を、
   representative `p07 / p08`、supporting `p09 / p13 / p14`、
   kept-separate residual、stop line 付きで pretty summary に返してよい。
2. `python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape --format json`
   は同じ読みを machine-readable manifest に返してよい。
3. `samples/problem-bundles/problem2-order-handoff-shared-space.md`
   は `witness-provider public-shape split の入口` section を持ち、
   split helper と同じ command / reading / stop line に揃えてよい。

## current recommendation

- Problem 2 residual を aggregate reopen map だけで読まず、
  witness/provider public-shape residual を独立 helper と doc section で narrow に読む。
- witness/provider line は claim/payload split first / route-schema split first を保ち、
  source wording / emitted schema residual と混ぜない。
- final public witness/provider/artifact contract judgment 自体はまだ上げず、
  split helper では current reopen point の独立可読化に留める。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape`
  - `python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape --format json`
- docs:
  - `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final public witness/provider/artifact contract
- stronger fairness / replay profile
- exhaustive shared-space catalog

## retained-later line

- source wording / emitted schema split
- final public witness/provider/artifact contract families
- stronger fairness / replay profile strengthening
