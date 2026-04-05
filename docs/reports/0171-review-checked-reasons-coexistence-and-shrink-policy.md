# Report 0171 — review checked reasons coexistence and shrink policy

- Date: 2026-04-05T15:50:00Z
- Author / agent: Codex
- Scope: current working tree の `checked_reasons` / `checked_reason_codes` coexistence-and-shrink-policy diff を maintainer 観点で review し、helper-local boundary、docs/plan/progress mirror、actual corpus evidence の整合を点検する
- Decision levels touched: L2

## 1. Objective

current working tree change について、次を確認する。

- readiness / coexistence scan が helper-local に留まり、machine-check semantics を変えていないこと
- docs / plan / progress が additive coexistence 維持、shrink defer、duplicate cluster non-promotion を正しく mirror していること
- actual corpus evidence に関する claim が source-backed であること

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_reason_code_readiness.py`
- `scripts/current_l2_reason_codes_assist.py`
- `scripts/current_l2_checked_reasons_assist.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_reason_code_readiness.py`
- `docs/reports/0170-checked-reasons-coexistence-and-shrink-policy.md`
- `docs/reports/0171-review-checked-reasons-coexistence-and-shrink-policy.md`
- current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. reviewer `Huygens` を 1 回起動し、`180000ms` を 2 回待った。
2. reviewer から返った finding 1 件を読み、placeholder の `0171` を evidence-bearing review record に差し替えた。
3. required top-level docs / specs / plan files を再読し、current helper boundary と reporting / traceability policy を確認した。
4. target diff を読み、`scripts/current_l2_reason_code_readiness.py` の追加 behavior が `run_bundle()` や checker core に越境していないことを点検した。
5. `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness` と actual corpus readiness smoke を review evidence として採用した。

## 4. Files changed

- 追加:
  - `docs/reports/0171-review-checked-reasons-coexistence-and-shrink-policy.md`
- `plan/` 更新は task 本体側で実施済み

## 5. Commands run and exact outputs

- `wait_agent(reviewer, 180000)` -> timeout
- `wait_agent(reviewer, 180000)` -> completion
- `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness`
  - `Ran 3 tests in 0.015s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label review-coexistence --overwrite`
  - `static-only fixtures scanned: 10`
  - `runtime fixtures skipped: 9`
  - `fixtures with checked_reasons: 8`
  - `fixtures with reason_codes suggestions: 8`
  - `fixtures with checked_reason_codes: 8`
  - `fixtures with stable coexistence anchors: 8`
  - `fixtures with checked_reason_codes but missing checked_reasons: 0`
  - `fixtures with checked_reason_codes mismatching actual suggestion: 0`
  - `fixtures needing coexistence follow-up:`
  - `  - none`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 171 numbered report(s).`

## 6. Evidence / findings

### Finding 1

- Severity: medium
- Location: `plan/90-source-traceability.md:145`, `docs/reports/0171-review-checked-reasons-coexistence-and-shrink-policy.md`
- reviewer は、`plan/90` が placeholder の `0171` を root source として指している traceability drift を指摘した。
- 対応として、placeholder の `0171` を evidence-bearing review record に差し替え、traceability root が空参照にならないようにした。

### No other substantive findings

- `scripts/current_l2_reason_code_readiness.py` は helper-local のままで、fixture JSON と detached static-gate artifacts を読む display-only summary に留まり、harness / checker semantics を変更していない。
- docs / plan / progress の additive coexistence 維持、shrink defer、duplicate cluster non-promotion に関する claim は actual corpus scan と整合している。
- stable cluster 8 fixture は `checked_reasons` / `checked_reason_codes` / detached suggestion の 3 者で aligned し、duplicate cluster 2 fixture は unpromoted のままである。

## 7. Changes in understanding

- coexistence scan 自体は十分 narrow で、current task の主リスクは semantics ではなく traceability completeness だった。
- stable cluster 8 fixture が zero follow-up に収束していることは current judgment を支えるが、shrink defer を覆すほどの evidence ではない。

## 8. Open questions

- `checked_reasons` deprecation を later task に送る条件
- first checker cut と typed source-of-truth の接続

## 9. Suggested next prompt

current additive coexistence を維持したまま、parser boundary / first checker cut へ主線を戻す順序を narrow に比較してください。
