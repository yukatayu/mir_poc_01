# Report 0169 — review first typed static reason family selection

- Date: 2026-04-05T15:05:00Z
- Author / agent: Codex
- Scope: Report 0168 と対応 diff に対する最終 review 記録
- Decision levels touched: L2

## 1. Objective

stable static reason `checked_reason_codes` tranche actualization が current helper boundary、duplicate cluster の non-promotion、docs / code / plan / progress mirror の整合を壊していないか確認する。

## 2. Inputs consulted

- `docs/reports/0168-first-typed-static-reason-family-selection.md`
- task diff
- local verification outputs
- reviewer evidence

## 3. Actions taken

1. reviewer `Linnaeus` を 1 回起動した。
2. reviewer から返った 2 finding を読んだ。
3. `static_reason_code_rows` / `is_supported_checked_reason_code` を public API から外し、detached static gate helper 側には helper-local duplicate transform を戻した。
4. `plan/07`、`plan/15`、`plan/90` の drift を修正し、stable 8 kind actualization と traceability anchor を mirror した。
5. code / tests / docs / plan / progress diff を再読し、stable cluster 8 kind actualization と duplicate cluster non-promotion の cut を再点検した。

## 4. Files changed

- 追加:
  - `docs/reports/0169-review-first-typed-static-reason-family-selection.md`
- `plan/` 更新は task 本体側で実施済み

## 5. Commands run and exact outputs

- `wait_agent(reviewer, 180000)` -> completion
- `python3 -m unittest scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_reason_code_readiness scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 12 tests in 0.017s`
  - `OK`
- `cargo test -p mir-semantics`
  - unit 2、support integration 2 + 2 + 8、main integration 44、doc-tests 0 が pass
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 169 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

- reviewer findings:
  1. `static_reason_code_rows` と `is_supported_checked_reason_code` が `lib.rs` public API に露出しており、helper-boundary leak になっていた。
  2. `plan/07`、`plan/15`、`plan/90` に stable 8 kind actualization 後の drift が残っていた。
- 対応:
  - `crates/mir-semantics/src/lib.rs` の両 helper は `pub(crate)` に縮めた。
  - `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs` に helper-local transform を戻し、detached static gate mirror の wording-derived transform を non-production support layer に留めた。
  - repo-memory drift は mirror 更新で解消した。
- reviewer は duplicate declaration cluster の accidental promotion や stable-8 matching path の semantic regression は見つけなかった。

## 7. Changes in understanding

- stable typed carrier actualization を進めても、wording-derived detached mirror transform は `lib.rs` public API に出さない方が boundary としてきれいである。
- current open issue は stable cluster actualization そのものではなく、`checked_reasons` / `checked_reason_codes` coexistence と shrink policy へ移ったことがさらに明確になった。

## 8. Open questions

- `checked_reasons` と `checked_reason_codes` の coexistence / shrink policy
- duplicate declaration cluster を explanation-only として固定するかの長期判断

## 9. Suggested next prompt

current stable cluster 8 kind が `checked_reason_codes` に actualize 済みである前提で、`checked_reasons` との coexistence / shrink / deprecation policy を docs-first で比較してください。
