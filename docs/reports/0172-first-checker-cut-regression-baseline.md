# Report 0172 — first checker cut regression baseline

- Date: 2026-04-05T15:58:00Z
- Author / agent: Codex
- Scope: stable static reason carrier の additive coexistence を維持したまま、current static-only corpus が first checker cut の local / structural floor をどこまで覆っているかを source-backed に整理し、mainline を checker boundary 側へ戻してよいかを narrow に判断する
- Decision levels touched: L2

## 1. Objective

current stable cluster 8 kind の `checked_reason_codes` actualization と coexistence scan が収束したことを前提に、

- current static-only corpus が first checker cut 候補 cluster をどこまで覆っているか
- carrier migration の細部より先に checker boundary 側へ主線を戻してよいか
- duplicate cluster を current cut の外に残したまま regression baseline を認めてよいか

を narrow に整理する。

## 2. Scope and assumptions

- current L2 semantics、parser grammar、failure family、machine-check policy は変えない。
- `checked_reasons` と `checked_reason_codes` の additive coexistence は維持する。
- detached static gate artifact の `reason_codes` は helper-local / reference-only mirror に留める。
- current task では final checker API を固定しない。
- duplicate declaration cluster は current first checker cut baseline の外に残す。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md`
- `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_reason_code_readiness.py`
- `scripts/tests/test_current_l2_reason_code_readiness.py`
- `scripts/current_l2_detached_loop.py`
- current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. `scripts/tests/test_current_l2_reason_code_readiness.py` に RED を追加し、reason kind を first checker cut cluster に roll-up した summary line を要求した。
2. `scripts/current_l2_reason_code_readiness.py` に helper-local な `checker_cluster_name_for_kind()` を追加し、stable kind を same-lineage / capability / missing-option の checker cluster に roll-up する display-only summary を実装した。
3. actual corpus に対して `scan-reason-code-readiness` smoke を実行し、cluster coverage を source-backed に確認した。
4. `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` を追加し、mainline を first checker cut 側へ戻してよいだけの baseline と current judgment を整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/30...`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/13`、`plan/90`、`progress.md` を mirror 更新した。

## 5. Files changed

- 追加:
  - `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
  - `docs/reports/0172-first-checker-cut-regression-baseline.md`
- 更新:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/90-source-traceability.md`
  - `progress.md`

## 6. Commands run

- `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness`
- `python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label checker-cut-readiness --overwrite`
- `python3 -m unittest scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_reason_code_readiness scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 7. Evidence / outputs / test results

- RED:
  - cluster coverage summary line が不在のため `test_main_reports_kind_counts_and_fixture_groups` 系が失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness` は `Ran 4 tests in ...` / `OK`
- actual corpus smoke:
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
- final verification:
  - targeted Python suite は `OK`
  - `python3 scripts/validate_docs.py` は `Documentation scaffold looks complete.` / `Found 173 numbered report(s).`
  - `git diff --check` は無出力

## 8. What changed in understanding

- current stable carrier migration は zero follow-up まで収束しており、mainline を checker boundary 側へ戻す妨げではなくなった。
- same-lineage / capability / missing-option の 3 cluster は current corpus で regression baseline を持つが、capability floor は 1 fixture と薄い。
- duplicate declaration cluster を typed carrier / checker core に急いで寄せない current judgment は維持してよい。

## 9. Open questions

- first checker cut の actual helper / API placement
- malformed / underdeclared split を cluster summary にどう反映するか
- capability floor fixture をもう 1 本以上増やすべきか
- parser subset inventory と checker cut inventory の handoff をどの文書で一本化するか

## 10. Suggested next prompt

first checker cut regression baseline が source-backed に見えた前提で、same-lineage / capability / missing-option の 3 cluster から actual checker spike をどの順で narrow に始めるべきかを整理してください。
