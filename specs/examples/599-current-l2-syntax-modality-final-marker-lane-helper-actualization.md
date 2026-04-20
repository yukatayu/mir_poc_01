# 599. current-l2 syntax-modality final-marker lane helper actualization

## 位置づけ

- current Phase 6 / Package 126 closeout。
- `residuals` summary と Package 124 / 125 の final-public-seam lane helper の次段として
  `scripts/current_l2_guided_samples.py lane syntax-modality-final-marker`
  を actualize し、syntax / modality mixed gate を Problem 1 / Problem 2 lane と true user-spec residual から切り分けて読めるようにする。
- final modal foundation adoption、final source marker adoption、final parser grammar を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py lane syntax-modality-final-marker`
   は syntax / modality final-marker lane の
   - focus
   - entry commands
   - current recommendation
   - retained families
   - separation boundary
   - component order
   - stop line
   - anchor refs
   を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py lane syntax-modality-final-marker --format json`
   は `current_recommendation`、`retained_families`、`separation_boundary`、
   `component_order`、`stop_line`、`anchor_refs`
   を machine-readable に返してよい。
3. `samples/problem-bundles/README.md`
   は `residuals` に加えて
   `lane syntax-modality-final-marker`
   を案内し、二大問題 sample bundle 入口から syntax / modality residual lane を見直してよい。

## current recommendation

- syntax / modality remaining mixed gate は
  `residuals`
  だけで終わらせず、
  `lane syntax-modality-final-marker`
  で partial basis / stronger family / readable source marker keep を narrow に読む。
- current recommendation は
  `partial basis keep + stronger family keep + readable source marker keep`
  に置いてよい。
- retained families は
  - `lambda_circle_box` partial basis keep
  - guarded / MDTT / MTT / Fitch-style stronger family keep
  として helper-local lane に保持してよい。
- separation boundary は
  - problem-local seam separation
  - true user-spec residual separation
  に置き、Problem 1 / Problem 2 final-public-seam lane と混ぜない。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py lane syntax-modality-final-marker`
  - `python3 scripts/current_l2_guided_samples.py lane syntax-modality-final-marker --format json`
- docs:
  - `samples/problem-bundles/README.md`
  - `plan/06-surface-notation-status.md`
  - `specs/10-open-questions.md`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`

## stop line

- final modal foundation adoption
- final source marker adoption
- final parser grammar

## retained-later line

- final modal foundation adoption
- final source marker adoption
- final parser grammar
- final public parser / checker / runtime API
