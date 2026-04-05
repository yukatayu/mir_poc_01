# Report 0173 — review first checker cut regression baseline

- Date: 2026-04-05T16:10:00Z
- Author / agent: Codex
- Scope: current working tree の first checker cut regression baseline diff を maintainer 観点で review し、helper-local boundary、docs / plan / progress mirror、actual corpus evidence の整合を点検する
- Decision levels touched: L2

## 1. Objective

current working tree change について、次を確認する。

- checker cluster roll-up が helper-local / display-only に留まり、machine-check semantics を変えていないこと
- docs / plan / progress が checker mainline への戻し判断と current risks を正しく mirror していること
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
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_reason_code_readiness.py`
- `scripts/tests/test_current_l2_reason_code_readiness.py`
- `scripts/current_l2_detached_loop.py`
- `docs/reports/0172-first-checker-cut-regression-baseline.md`

## 3. Actions taken

1. current session では reviewer subagent tool が利用できなかったため、task close fallback として local diff inspection を採用した。
2. required top-level docs / specs / plan files を再読し、current helper boundary と reporting / traceability policy を確認した。
3. target diff を読み、checker cluster roll-up が display-only summary に留まり、harness / checker semantics を変更していないことを点検した。
4. targeted Python suite と actual corpus readiness smoke を review evidence として採用した。
5. `plan/90-source-traceability.md` が今回追加した spec / report chain を root source として参照していることを確認した。

## 4. Files changed

- 追加:
  - `docs/reports/0173-review-first-checker-cut-regression-baseline.md`
- `plan/` 更新は task 本体側で実施済み

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness`
  - `Ran 4 tests in 0.007s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label review-checker-cut --overwrite`
  - `static-only fixtures scanned: 10`
  - `runtime fixtures skipped: 9`
  - `fixtures with checked_reasons: 8`
  - `fixtures with reason_codes suggestions: 8`
  - `fixtures with checked_reason_codes: 8`
  - `fixtures with stable coexistence anchors: 8`
  - `fixtures with checked_reason_codes but missing checked_reasons: 0`
  - `fixtures with checked_reason_codes mismatching actual suggestion: 0`
  - `checker cluster coverage:`
  - `  - capability_strengthening_floor: 1`
  - `  - missing_option_structure_floor: 3`
  - `  - same_lineage_evidence_floor: 4`
  - `fixtures needing coexistence follow-up:`
  - `  - none`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 173 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

### No substantive findings

- `scripts/current_l2_reason_code_readiness.py` は fixture JSON と detached static-gate artifacts を読み、display-only summary を出す helper に留まり、`run_bundle()` や static gate machine-check compare の public boundary を変更していない。
- checker cluster roll-up は `reason_code_kinds` からの helper-local 集計であり、新しい fixture schema field や detached artifact field を導入していない。
- docs / plan / progress の「mainline を first checker cut 側へ戻す current judgment」は、actual corpus smoke の cluster count `4 / 1 / 3` と stable coexistence zero follow-up に支えられている。
- `plan/90-source-traceability.md` は `specs/examples/45...`、`docs/reports/0172...`、`docs/reports/0173...` を根拠 chain に含めており、今回 task に関する traceability 漏れは見当たらない。

### Residual risk

- `capability_strengthening_floor` は fixture 1 本分の coverage に留まるため、first checker cut を actual helper cut に下ろす前に regression baseline の厚みをもう少し増やす余地がある。

## 7. Changes in understanding

- current 主リスクは helper boundary ではなく coverage density であり、same-lineage / missing-option は最低限の baseline が見えているが capability floor はまだ薄い。
- stable carrier coexistence が zero follow-up まで収束したことで、carrier migration の追加 refinement より checker-boundary 整理へ戻る方が全体計画に沿うことが再確認できた。

## 8. Open questions

- first checker cut を actual helper / API としてどこへ置くか
- capability floor の corpus coverage を何 fixture まで増やすか
- malformed / underdeclared split を checker cluster summary にどう反映するか

## 9. Suggested next prompt

first checker cut regression baseline を前提に、actual checker spike を same-lineage floor から始めるべきか、missing-option structure floor から始めるべきかを narrow に比較してください。
