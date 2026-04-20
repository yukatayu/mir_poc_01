# 586. current-l2 representative problem quickstart CLI mirror actualization

## 位置づけ

- current Phase 6 / Package 112 closeout。
- bundle doc 側に置いた representative 4-step quickstart を
  `scripts/current_l2_guided_samples.py` からも problem ごとに読めるようにする。
- exhaustive tutorial surface、final public CLI / tutorial surface、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py quickstart problem1`
   は Problem 1 の representative 4-step quickstart を helper-side summary として返してよい。
2. `python3 scripts/current_l2_guided_samples.py quickstart problem2`
   は Problem 2 の representative 4-step quickstart を helper-side summary として返してよい。
3. `--format json`
   は step title / command / expected_results を machine-readable に返してよい。
4. `samples/problem-bundles/README.md`
   は quickstart CLI mirror command を index 側から案内してよい。

## current recommendation

- problem ごとの quickstart は doc-only に閉じず、
  helper-side summary にも mirror しておく。
- `smoke-all` は cross-problem aggregate entrypoint、
  `quickstart problem1|problem2` は per-problem first-step entrypoint として使い分ける。
- current cut は repo-local helper summary に留め、
  final public CLI / tutorial surface には上げない。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py quickstart problem1`
  - `python3 scripts/current_l2_guided_samples.py quickstart problem2 --format json`
- docs:
  - `samples/problem-bundles/README.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- exhaustive tutorial surface
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- quickstart parity checks against sample bundle docs
- broader sample catalog widening
- final public onboarding flow
