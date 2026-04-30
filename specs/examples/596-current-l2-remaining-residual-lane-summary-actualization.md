# 596. current-l2 remaining residual lane summary actualization

## 位置づけ

- historical Phase 6 / Package 123 closeout memory。
- split-package closeout 後に残る mixed gate と true user-spec residual を
  `scripts/current_l2_guided_samples.py residuals`
  で圧縮していた historical helper-local summary memory として記録する。
- final public theorem/model-check/witness-provider contract、
  final parser / checker / runtime API、
  exhaustive shared-space catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py residuals`
   は 2026-04-22 clean-sample migration 前の historical residual-summary entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. archived representative sample bundle doc memory は
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
   `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
   に残し、historical `residuals` entrypoint を案内してよい。

## current recommendation

- split-package closeout 後の global helper surface は
  `reopen-map`
  と
  `residuals`
  を分けていた historical memory に留める。
- `reopen-map` は problem-local reopen point と split package closeout を読む historical入口に留める。
- `residuals` は remaining mixed gate lane と true user-spec residual の圧縮 summary に集中していた historical reading に留める。
- historical closeout queue memory では next self-driven reopen order を
  Problem 1 final public seam、
  Problem 2 final public seam、
  syntax/modality final marker
  の順で narrow に並べていた。current queue authority は `progress.md` / `tasks.md` に残す。
- historical `residuals` helper は helper-local / non-production memory であり、
  current active command surface には戻さない。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py residuals`
  - `python3 scripts/current_l2_guided_samples.py residuals --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

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
