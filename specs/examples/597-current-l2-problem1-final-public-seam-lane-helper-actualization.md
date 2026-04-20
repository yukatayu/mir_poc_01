# 597. current-l2 Problem 1 final-public-seam lane helper actualization

## 位置づけ

- current Phase 6 / Package 124 closeout。
- `residuals` summary の次段として
  `scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
  を actualize し、Problem 1 mixed gate の reopen order を独立 lane として読めるようにする。
- final public theorem/model-check/verifier contract や final typed source principal を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
   は Problem 1 final-public-seam lane の
   - focus
   - entry commands
   - component order
   - stop line
   - anchor refs
   を pretty summary で返してよい。
2. `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`
   は `component_order` / `stop_line` / `anchor_refs`
   を machine-readable に返してよい。
3. Problem 1 sample bundle doc は `residuals` に加えて
   `lane problem1-final-public-seams`
   を案内し、Problem 1 lane と global residual lane summary を往復してよい。

## current recommendation

- Problem 1 remaining mixed gate は
  `residuals`
  だけで終わらせず、
  `lane problem1-final-public-seams`
  で typed / theorem / model-check reopen order まで narrow に読む。
- current component order は
  - typed source principal split
  - theorem public-contract split
  - model-check public-contract split
  の順に置いてよい。
- current cut は Problem 1 lane helper に留め、
  final public theorem/model-check/verifier contract には上げない。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
  - `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final typed source principal
- final public theorem contract
- final public checker artifact
- final public verifier contract

## retained-later line

- Problem 2 final-public-seam lane helper
- syntax-modality final-marker lane helper
- final public theorem/model-check/verifier contract
