# 597. current-l2 Problem 1 final-public-seam lane helper actualization

## 位置づけ

- historical Phase 6 / Package 124 closeout memory。
- `residuals` summary の次段として
  `scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
  を historical helper-local lane memory として記録し、Problem 1 mixed gate の reopen order を独立 lane として読んでいたことを残す。
- final public theorem/model-check/verifier contract や final typed source principal を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
   は 2026-04-22 clean-sample migration 前の historical Problem 1 lane entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. archived Problem 1 sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   の lane entrypoint で保持してよい。

## current recommendation

- Problem 1 remaining mixed gate は
  `residuals`
  だけで終わらせず、
  `lane problem1-final-public-seams`
  で typed / theorem / model-check reopen order まで narrow に読んでいた historical memory に留める。
- current component order は
  - typed source principal split
  - theorem public-contract split
  - model-check public-contract split
  の順に置いてよい。
- historical `lane problem1-final-public-seams` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。
- current cut は Problem 1 lane helper memory に留め、
  final public theorem/model-check/verifier contract には上げない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
  - `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final typed source principal
- final public theorem contract
- final public checker artifact
- final public verifier contract

## retained-later line

- Problem 2 final-public-seam lane helper
- syntax-modality final-marker lane helper
- final public theorem/model-check/verifier contract
