# 581. current-l2 explained representative problem sample bundles

## 位置づけ

- current Phase 6 / Package 107 closeout。
- 二大問題の representative sample を `samples/` 側の簡潔な日本語 guide で辿れるようにし、
  runner / Lean artifact / parser companion / guided helper の 4 本を 1 本の sample bundle 導線へ揃える。
- final public tutorial surface や exhaustive sample catalog を作る task ではない。

## この package で固定する current cut

1. `samples/problem-bundles/README.md`
   と Problem 1 / Problem 2 の per-problem guide を actualize する。
2. `scripts/current_l2_guided_samples.py bundle problem1|problem2`
   に `sample_bundle_doc` を actualize し、helper 側から `samples/` guide を辿れるようにする。
3. `samples/current-l2/README.md` と `samples/prototype/README.md`
   に explained bundle への導線を追加する。

## current recommendation

- Problem 1 は `p06`、Problem 2 は `p07 / p08` を中心とし、
  supporting / reserve / negative sample を控えめに添える bundle が current first line と整合する。
- explained bundle は existing corrected prototype / Lean artifact / parser companion / helper command を再配置せず、
  既存 evidence への readable index として置く。

## actualized evidence

- docs:
  - `samples/problem-bundles/README.md`
  - `samples/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- helper:
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
- tests:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`

## stop line

- exhaustive tutorial expansion
- exhaustive sample catalog
- final public parser / checker / runtime API
- final public verifier contract
- exhaustive shared-space catalog

## retained-later line

- bundle command の full smoke execution automation
- final public tutorial / onboarding surface
- mixed gate に残る theorem/model-check / witness-provider public contract
