# 580. current-l2 parser-side representative mapping matrix actualization

## 位置づけ

- historical Phase 6 / Package 106 closeout memory。
- `mapping` helper と parser-companion README table によって
  representative slice `p06 / p07 / p08` の
  original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report 対応を見せていた導線を、
  2026-04-22 clean-sample migration 前の historical mapping memory として記録する。
- final public tutorial surface、
  final public parser / checker / runtime API、
  exhaustive sample catalog を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py mapping`
   は historical helper-local mapping memory として扱い、
   current active compatibility front door には戻さない。
2. representative mapping memory は
   archived parser-companion README と archived problem-bundle docs に残してよい。
3. current parser-side compare-floor command は
   `specs/examples/579` の archived parser-companion inspection に留める。

## current recommendation

- `mapping` helper は、
  representative archived compare floor の cross-anchor 対応を
  どう読んでいたかの historical memory として保ってよい。
- helper command 自体は current active command surface に戻さず、
  archived parser-companion inspection と snapshot docs の current authority を分けて扱う。

## actualized evidence

- retired helper commands today:
  - `python3 scripts/current_l2_guided_samples.py mapping`
  - `python3 scripts/current_l2_guided_samples.py mapping --format json`
  - current repo では migration note + `supported compatibility commands: list, smoke-all, closeout` を返して exit 2 になる
- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/README.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
  - `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`
- current parser-side compare-floor command:
  - `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt --format json`

## stop line

- final public tutorial surface
- final public parser / checker / runtime API
- exhaustive sample catalog

## retained-later line

- archived problem-bundle memory
- final parser grammar / public API
