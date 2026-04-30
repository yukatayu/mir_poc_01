# 589. current-l2 representative problem split-package map refresh

## 位置づけ

- historical Phase 6 / Package 115 / 116 closeout memory。
- `reopen-map` aggregate helper を踏まえて、
  Problem 1 / Problem 2 の remaining mixed gate を
  next split package 単位へ落とし直していた historical helper-local map memory を記録する。
- final public theorem/model-check/witness-provider contract、
  final public parser / checker / runtime API、
  exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
   は 2026-04-22 clean-sample migration 前の historical split-package map entrypoint として扱い、
   current active compatibility front door には戻さない。
2. `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
   も同じ historical split-package map memory に含める。
3. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
4. archived Problem 1 / Problem 2 sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
   の `次の split package` section で保持してよい。

## current recommendation

- reopen-map aggregate で止めず、
  current mixed gate を actual next package 名まで narrow に戻していた historical memory として保持する。
- Problem 1 は
  typed source principal / theorem public-contract / model-check public-contract の 3 本へ split する。
- Problem 2 は
  source wording / emitted schema と witness-provider public-shape の 2 本へ split する。
- global true user-spec residual は引き続き package split へ混ぜない。
- historical `reopen-map problem1|problem2` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2 --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

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
