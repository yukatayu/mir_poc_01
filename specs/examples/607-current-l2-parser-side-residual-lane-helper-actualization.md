# 607. current-l2 parser-side residual lane helper actualization

## 位置づけ

- historical Phase 6 / Package 134 closeout memory。
- `specs/examples/564`、`565`、`577`、`578`、`579`、`580`、`606` の次段として、
  parser companion surface / parser-side tranche / final parser-checker-runtime API residual を
  historical `lane parser-side-residual`
  helper へ再同期した package memory を記録する。
- final parser grammar、final public parser / checker / runtime API、public tutorial surface adoption、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py lane parser-side-residual`
   は 2026-04-22 clean-sample migration 前の historical helper-local residual lane entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置き、historical `residuals` / `reserve` lane memory は snapshot docs に残す。
3. historical closeout queue memory では
   `135 true user-spec residual freeze sync`
   `later mixed / user-spec residual`
   に進んでいた。current queue authority は `progress.md` / `tasks.md` に残す。

## current recommendation

- parser-side residual は new public API proposal に広げず、
  representative slice と non-production parser-side carrier を読む historical helper-local lane / snapshot memory として保持してよい。
- current cut は
  - parser companion surface bundle
  - bundle-to-helper bridge
  - request-head / clause-bundle inspector
  - representative mapping matrix
  を 1 本の mixed-gate lane に圧縮するところに留める。
- final parser grammar、final public parser / checker / runtime API、public tutorial surface adoption には上げない。

## actualized evidence

- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_remaining_residual_lane_summary_json_separates_mixed_gate_lanes_and_user_spec_residuals scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_once_through_closeout_summary_json_contains_next_packages scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_reserve_integration_summary_json_contains_package_ids_and_next_queue scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_parser_side_residual_lane_text_mentions_component_order_and_stop_line scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_parser_side_residual_lane_json_contains_component_order`
- historical helper commands:
  - `python3 scripts/current_l2_guided_samples.py lane parser-side-residual`
  - `python3 scripts/current_l2_guided_samples.py lane parser-side-residual --format json`
  - `python3 scripts/current_l2_guided_samples.py residuals --format json`
  - `python3 scripts/current_l2_guided_samples.py reserve --format json`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final parser grammar
- final public parser / checker / runtime API
- public tutorial surface adoption
- final public verifier contract

## retained-later line

- true user-spec residual freeze sync
- parser-side residual beyond representative slice
- later mixed / user-spec residual reopen
