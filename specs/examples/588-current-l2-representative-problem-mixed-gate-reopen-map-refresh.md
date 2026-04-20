# 588. current-l2 representative problem mixed-gate reopen map refresh

## 位置づけ

- current Phase 6 / Package 114 closeout。
- representative sample の quickstart / matrix / bundle / smoke floor を踏まえて、
  Problem 1 / Problem 2 の remaining mixed gate と true user-spec residual を
  entry command 付きで読み直す helper-local reopen map を actualize する。
- final public tutorial surface、final public parser / checker / runtime API、
  final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reopen-map`
   は Problem 1 / Problem 2 の current runnable floor、remaining mixed gate、
   global true user-spec residual を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py reopen-map --format json`
   は `problem_rows` と `true_user_spec_residuals` を machine-readable に返してよい。
3. `samples/problem-bundles/problem1-typed-theorem-model-check.md`
   と `samples/problem-bundles/problem2-order-handoff-shared-space.md`
   は `現在の mixed gate 再開点` section を持ち、
   bundle doc 単体でも reopen point を読めるようにしてよい。
4. `samples/problem-bundles/README.md`
   は `reopen-map` command を sample bundle index から案内してよい。

## current recommendation

- representative sample floor が固まった後は、
  「まだ mixed gate だから later」と散らすのではなく、
  どの command から再開するかを helper / sample bundle / snapshot docs で揃える。
- Problem 1 では
  typed source principal、theorem public contract、model-check public contract を分けて読む。
- Problem 2 では
  source wording / emitted schema と witness/provider public shape を分けて読む。
- packaging / host integration / upper-layer target のような
  true user-spec residual は、problem-local mixed gate と混ぜず global residual として別に出す。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py reopen-map`
  - `python3 scripts/current_l2_guided_samples.py reopen-map --format json`
- docs:
  - `samples/problem-bundles/README.md`
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

- Problem 1 theorem/model-check mixed-gate split refresh
- Problem 2 order-handoff/public-shape mixed-gate split refresh
- final modal foundation / source marker
