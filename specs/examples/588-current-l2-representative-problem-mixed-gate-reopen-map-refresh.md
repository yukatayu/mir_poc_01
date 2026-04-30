# 588. current-l2 representative problem mixed-gate reopen map refresh

## 位置づけ

- historical Phase 6 / Package 114 closeout memory。
- representative sample の quickstart / matrix / bundle / smoke floor を踏まえて、
  Problem 1 / Problem 2 の remaining mixed gate と true user-spec residual を
  entry command 付きで読み直していた historical helper-local reopen-map memory を記録する。
- final public tutorial surface、final public parser / checker / runtime API、
  final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map`
   は 2026-04-22 clean-sample migration 前の historical helper-local reopen-map entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. historical sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
   に残し、
   `現在の mixed gate 再開点` section を historical doc memory として読んでよい。

## current recommendation

- representative sample floor が固まった後は、
  「まだ mixed gate だから later」と散らすのではなく、
  どの command から再開していたかを helper / archived sample bundle / snapshot docs の historical memory で揃える。
- Problem 1 では
  typed source principal、theorem public contract、model-check public contract を分けて読む。
- Problem 2 では
  source wording / emitted schema と witness/provider public shape を分けて読む。
- packaging / host integration / upper-layer target のような
  true user-spec residual は、problem-local mixed gate と混ぜず global residual として別に出す。
- historical `reopen-map` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py reopen-map`
  - `python3 scripts/current_l2_guided_samples.py reopen-map --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
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

- Problem 1 theorem/model-check mixed-gate split refresh
- Problem 2 order-handoff/public-shape mixed-gate split refresh
- final modal foundation / source marker
