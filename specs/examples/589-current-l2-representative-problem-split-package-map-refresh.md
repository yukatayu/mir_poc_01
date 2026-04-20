# 589. current-l2 representative problem split-package map refresh

## 位置づけ

- current Phase 6 / Package 115 / 116 closeout。
- `reopen-map` aggregate helper を踏まえて、
  Problem 1 / Problem 2 の remaining mixed gate を
  next split package 単位へ落とし直す helper-local map を actualize する。
- final public theorem/model-check/witness-provider contract、
  final public parser / checker / runtime API、
  exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
   は Problem 1 remaining mixed gate と
   `typed source principal split` /
   `theorem public-contract split` /
   `model-check public-contract split`
   を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
   は Problem 2 remaining mixed gate と
   `source wording / emitted schema split` /
   `witness-provider public-shape split`
   を pretty summary で返してよい。
3. `reopen-map ... --format json`
   は `split_packages` を machine-readable に返してよい。
4. Problem 1 / Problem 2 の sample bundle doc は
   `次の split package` section を持ち、
   helper 側の split-package map と同じ読みへ揃えてよい。

## current recommendation

- reopen-map aggregate で止めず、
  current mixed gate を actual next package 名まで narrow に戻す。
- Problem 1 は
  typed source principal / theorem public-contract / model-check public-contract の 3 本へ split する。
- Problem 2 は
  source wording / emitted schema と witness-provider public-shape の 2 本へ split する。
- global true user-spec residual は引き続き package split へ混ぜない。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2 --format json`
- docs:
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final public theorem/model-check/witness-provider contract
- final public CLI / tutorial surface
- final public parser / checker / runtime API
- exhaustive shared-space catalog

## retained-later line

- Problem 1 typed source principal split
- Problem 1 theorem public-contract split
- Problem 1 model-check public-contract split
- Problem 2 source wording / emitted schema split
- Problem 2 witness-provider public-shape split
