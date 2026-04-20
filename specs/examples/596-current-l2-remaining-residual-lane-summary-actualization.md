# 596. current-l2 remaining residual lane summary actualization

## 位置づけ

- current Phase 6 / Package 123 closeout。
- split-package closeout 後に残る mixed gate と true user-spec residual を
  `scripts/current_l2_guided_samples.py residuals`
  で圧縮し、next reopen order を helper / docs / snapshot から同じ読みで辿れるようにする。
- final public theorem/model-check/witness-provider contract、
  final parser / checker / runtime API、
  exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py residuals`
   は remaining mixed gate を
   - `problem1-final-public-seams`
   - `problem2-final-public-seams`
   - `syntax-modality-final-marker`
   の 3 lane と true user-spec residual に圧縮して返してよい。
2. `python3 scripts/current_l2_guided_samples.py residuals --format json`
   は `mixed_gate_lanes` / `true_user_spec_residuals` / `recommended_order`
   を machine-readable に返してよい。
3. representative sample bundle doc は `reopen-map` に加えて `residuals` entrypoint を案内し、
   problem-local reopen と global residual lane summary を往復して読めるようにしてよい。

## current recommendation

- split-package closeout 後の global helper surface は
  `reopen-map`
  と
  `residuals`
  を分ける。
- `reopen-map` は problem-local reopen point と split package closeout を読む入口に留める。
- `residuals` は remaining mixed gate lane と true user-spec residual の圧縮 summary に集中させる。
- next self-driven reopen order は
  Problem 1 final public seam、
  Problem 2 final public seam、
  syntax/modality final marker
  の順で narrow に並べてよい。

## actualized evidence

- helper:
  - `python3 scripts/current_l2_guided_samples.py residuals`
  - `python3 scripts/current_l2_guided_samples.py residuals --format json`
- tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final public theorem/model-check/witness-provider contract
- final public parser / checker / runtime API
- exhaustive shared-space catalog
- packaging / FFI / engine adapter / host integration target

## retained-later line

- problem-local `reopen-map`
- split helper command family
- Problem 1 / Problem 2 / syntax-modality residual lane follow-up packages
- true user-spec residuals
