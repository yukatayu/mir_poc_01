# 587. current-l2 representative problem quickstart parity checks actualization

## 位置づけ

- current Phase 6 / Package 113 closeout。
- sample bundle doc と quickstart helper の representative 4-step 導線が揃っているかを
  focused parity check として actualize する。
- exhaustive tutorial validation、final public CLI / tutorial surface、
  final public parser / checker / runtime API を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py quickstart-parity`
   は Problem 1 / Problem 2 の quickstart parity を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py quickstart-parity --format json`
   は `problem_id` / `sample_bundle_doc` / `status` / `missing_titles` / `missing_commands`
   を machine-readable に返してよい。
3. current cut は representative 4-step quickstart の title / command parity に留め、
   exhaustive tutorial validation には広げない。
4. `samples/problem-bundles/README.md`
   は parity check command を sample bundle index から案内してよい。

## current recommendation

- quickstart hardening と CLI mirror の次は、
  drift suppression を focused parity check で支える。
- mismatch があっても最初から large tutorial framework にせず、
  representative 4-step entrypoint だけを narrow に見る。
- current helper は repo-local verification cut に留め、
  final public CLI / tutorial surface には上げない。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py quickstart-parity`
  - `python3 scripts/current_l2_guided_samples.py quickstart-parity --format json`
- docs:
  - `samples/problem-bundles/README.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- exhaustive tutorial validation
- final public CLI / tutorial surface
- final public parser / checker / runtime API

## retained-later line

- mixed-gate reopen-map refresh
- broader sample catalog widening
- final public onboarding flow
