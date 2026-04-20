# 593. current-l2 Problem 2 source wording / emitted schema split helper actualization

## 位置づけ

- current Phase 6 / Package 120 closeout。
- Problem 2 の source wording / emitted schema residual を witness-provider public-shape residual から切り離して読むための
  narrow helper/doc cut を actualize する。
- final source-surface handoff wording、final emitted-artifact schema、final public parser / checker / runtime API を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
   は Problem 2 source wording / emitted schema residual を、
   representative `p07 / p08`、supporting `p13 / p14`、
   kept-separate residual、stop line 付きで pretty summary に返してよい。
2. `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema --format json`
   は同じ読みを machine-readable manifest に返してよい。
3. `samples/problem-bundles/problem2-order-handoff-shared-space.md`
   は `source wording / emitted schema split の入口` section を持ち、
   split helper と同じ command / reading / stop line に揃えてよい。

## current recommendation

- Problem 2 residual を aggregate reopen map だけで読まず、
  source wording / emitted schema residual を独立 helper と doc section で narrow に読む。
- source wording line は edge-row principal / stage-block secondary / serial reserve を保ち、
  witness/provider public-shape residual と混ぜない。
- final public source wording judgment や final emitted-artifact schema judgment 自体はまだ上げず、
  split helper では current reopen point の独立可読化に留める。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
  - `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema --format json`
- docs:
  - `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final source-surface handoff wording
- final emitted-artifact schema
- final public parser / checker / runtime API

## retained-later line

- witness-provider public-shape split
- final public witness/provider/artifact contract families
- stronger fairness / replay profile
