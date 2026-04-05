# Report 0170 — checked reasons coexistence and shrink policy

- Date: 2026-04-05T15:35:00Z
- Author / agent: Codex
- Scope: current stable static reason cluster 8 kind の actualization 後に、`checked_reasons` / `checked_reason_codes` の additive coexistence を維持するか、shrink をどこまで保留するかを docs-first で整理し、corpus 横断の follow-up scan を helper に追加する
- Decision levels touched: L2

## 1. Objective

current stable cluster 8 kind で `checked_reason_codes` が actualize 済みであることを前提に、

- `checked_reasons` を今すぐ縮退させるべきか
- additive coexistence を維持するべきか
- shrink を later task へ送る前に何を corpus 横断で見えるようにするべきか

を narrow に整理する。

## 2. Scope and assumptions

- current L2 semantics、parser grammar、failure family、machine-check policy は変えない。
- `checked_reasons` と `checked_reason_codes` の coexistence policy を決めるが、final deprecation はまだ行わない。
- detached static gate artifact の `detached_noncore.reason_codes` は helper-local / reference-only mirror に留める。
- duplicate declaration cluster は current stable typed carrier に昇格させない。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
- `specs/examples/41-current-l2-first-typed-static-reason-family-carrier-cut.md`
- `specs/examples/42-current-l2-second-typed-static-reason-family-actualization.md`
- `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_reason_code_readiness.py`
- `scripts/tests/test_current_l2_reason_code_readiness.py`
- `scripts/current_l2_detached_loop.py`
- current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. current readiness scan helper と actual corpus を再確認し、stable cluster 8 fixture が `checked_reasons` / `checked_reason_codes` / detached suggestion の 3 者で揃っているかを確認した。
2. `scripts/tests/test_current_l2_reason_code_readiness.py` に RED を追加し、coexistence anchor count と follow-up group summary を要求した。
3. `scripts/current_l2_reason_code_readiness.py` を最小拡張し、次を display-only summary として出せるようにした。
   - `fixtures with stable coexistence anchors`
   - `fixtures with checked_reason_codes but missing checked_reasons`
   - `fixtures with checked_reason_codes mismatching actual suggestion`
   - `fixtures needing coexistence follow-up`
4. actual corpus に対する smoke を取り、stable cluster 8 fixture が zero follow-up で収束していることを確認した。
5. `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md` を追加し、current judgment を additive coexistence 維持 / shrink 保留として固定した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を mirror 更新した。

## 5. Files changed

- 追加:
  - `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
  - `docs/reports/0170-checked-reasons-coexistence-and-shrink-policy.md`
- 更新:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/90-source-traceability.md`
  - `progress.md`

## 6. Commands run

- `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness`
- `python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label coexistence-readiness --overwrite`
- `python3 -m unittest scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_reason_code_readiness scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 7. Evidence / outputs / test results

- RED:
  - `test_main_reports_kind_counts_and_fixture_groups`
  - `test_main_reports_coexistence_follow_up_groups`
  が、coexistence summary line 不在で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness` は `Ran 3 tests in 0.004s` / `OK`
- actual corpus smoke:
  - `static-only fixtures scanned: 10`
  - `runtime fixtures skipped: 9`
  - `fixtures with checked_reasons: 8`
  - `fixtures with reason_codes suggestions: 8`
  - `fixtures with checked_reason_codes: 8`
  - `fixtures with stable coexistence anchors: 8`
  - `fixtures with checked_reason_codes but missing checked_reasons: 0`
  - `fixtures with checked_reason_codes mismatching actual suggestion: 0`
  - `fixtures needing coexistence follow-up: none`
- final verification:
  - targeted Python suite は `OK`
  - `python3 scripts/validate_docs.py` は `Documentation scaffold looks complete.` / `Found 170 numbered report(s).`
  - `git diff --check` は無出力

## 8. What changed in understanding

- stable cluster 8 fixture では coexistence は実際に収束しているが、それだけでは `checked_reasons` shrink の根拠として十分ではない。
- `checked_reasons` は今も actual wording compare の bridge であり、`checked_reason_codes` は typed companion である。current natural cut は replacement ではなく additive coexistence 維持である。
- shrink 判断に必要なのは family actualization の追加ではなく、coexistence follow-up が消えていることと、wording anchor をどこへ移すかの boundary 整理である。

## 9. Open questions

- `checked_reasons` を最終的に deprecated にするか
- deprecated にするとして、actual wording compare をどこへ残すか
- duplicate declaration cluster を explanation-only cluster として長期固定するか
- first checker cut / typed source-of-truth と static reason carrier migration の関係をどう切るか

## 10. Suggested next prompt

`checked_reasons` / `checked_reason_codes` coexistence が stable cluster 8 fixture で zero follow-up まで収束した前提で、`checked_reasons` を shrink しないまま parser boundary / first checker cut へ主線を戻すべきか、それとも static reason carrier migration の deprecation 条件をもう一段 narrow に比較するべきかを整理してください。
