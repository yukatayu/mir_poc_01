# 598. current-l2 Problem 2 final-public-seam lane helper actualization

## 位置づけ

- current Phase 6 / Package 125 closeout。
- `residuals` summary と Package 124 Problem 1 lane helper の次段として
  `scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
  を actualize し、Problem 2 mixed gate の reopen order を独立 lane として読めるようにする。
- final source-surface handoff wording や final public witness/provider/artifact contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
   は Problem 2 final-public-seam lane の
   - focus
   - entry commands
   - component order
   - stop line
   - anchor refs
   を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json`
   は `component_order` / `stop_line` / `anchor_refs`
   を machine-readable に返してよい。
3. Problem 2 sample bundle doc は `residuals` に加えて
   `lane problem2-final-public-seams`
   を案内し、Problem 2 lane と global residual lane summary を往復してよい。

## current recommendation

- Problem 2 remaining mixed gate は
  `residuals`
  だけで終わらせず、
  `lane problem2-final-public-seams`
  で source wording / emitted schema と witness-provider public-shape の reopen order まで narrow に読む。
- current component order は
  - source wording / emitted schema split
  - witness-provider public-shape split
  の順に置いてよい。
- current cut は Problem 2 lane helper に留め、
  final source-surface handoff wording、
  final public witness/provider/artifact contract、
  exhaustive shared-space catalog には上げない。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
  - `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final source-surface handoff wording
- final public witness/provider/artifact contract
- exhaustive shared-space catalog

## retained-later line

- syntax-modality final-marker lane helper
- final source-surface handoff wording
- final public witness/provider/artifact contract
- exhaustive shared-space catalog
