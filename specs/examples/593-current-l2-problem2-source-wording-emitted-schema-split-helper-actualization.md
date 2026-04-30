# 593. current-l2 Problem 2 source wording / emitted schema split helper actualization

## 位置づけ

- historical Phase 6 / Package 120 closeout memory。
- Problem 2 の source wording / emitted schema residual を witness-provider public-shape residual から切り離して読むための
  narrow helper/doc cut を historical helper/doc memory として記録する。
- final source-surface handoff wording、final emitted-artifact schema、final public parser / checker / runtime API を固定する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
   は 2026-04-22 clean-sample migration 前の historical source-wording/emitted-schema split entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. archived Problem 2 sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
   の `source wording / emitted schema split の入口` section で保持してよい。

## current recommendation

- Problem 2 residual を aggregate reopen map だけで読まず、
  source wording / emitted schema residual を独立 helper と archived doc section で narrow に読んでいた historical memory として保つ。
- source wording line は edge-row principal / stage-block secondary / serial reserve を保ち、
  witness/provider public-shape residual と混ぜない。
- final public source wording judgment や final emitted-artifact schema judgment 自体はまだ上げず、
  split helper では historical reopen point の独立可読化に留める。
- historical `split problem2 source-wording-emitted-schema` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
  - `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final source-surface handoff wording
- final emitted-artifact schema
- final public parser / checker / runtime API

## retained-later line

- witness-provider public-shape split
- final public witness/provider/artifact contract families
- stronger fairness / replay profile
