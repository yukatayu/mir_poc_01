# Report 0166 — static reason code readiness scan

- Date: 2026-04-05T14:20:00Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC の static-only fixture corpus に対して、`checked_reasons` adoption と helper-local / reference-only `reason_codes` suggestion availability を batch で display-only 要約する non-production readiness scan を追加する
- Decision levels touched: L2

## 1. Objective

`suggest-reason-codes` を 1 fixture assist のまま維持しつつ、future typed carrier actualization の着手順を corpus 横断で source-backed に比較できるようにする。

## 2. Scope and assumptions

- current L2 semantics、fixture schema、machine-check policy は変えない。
- `detached_noncore.reason_codes` は helper-local / reference-only mirror に留める。
- runtime fixture は readiness scan の current target に含めず、skipped count にだけ入れる。
- `expected_static.reason_codes` のような fixture-side typed field は未導入のままとし、見つけたら fail-closed に止める。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `progress.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_reason_codes_assist.py`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. static-only fixture corpus と current `reason_codes` assist の boundary を再確認した。
2. `scripts/tests/test_current_l2_reason_code_readiness.py` と `scripts/tests/test_current_l2_static_gate_loop.py` に RED を追加し、helper 不在と wrapper 未接続を確認した。
3. `scripts/current_l2_reason_code_readiness.py` を追加し、fixture directory と emitted static gate artifact directory を読んで readiness summary を出す helper を実装した。
4. `scripts/current_l2_detached_loop.py` に `scan-reason-code-readiness` subcommand を追加し、static-only fixture にだけ artifact emit を行ってから readiness helper を呼ぶようにした。
5. `specs/examples/39-current-l2-static-reason-code-readiness-scan.md` を追加し、display-only / static-only / fail-closed の current cut を docs-only で固定した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/15`、`progress.md` を mirror 更新した。
7. current fixture corpus に対する real smoke を取り、stable cluster 8 件 / duplicate cluster 2 件の split を確認した。

## 5. Files changed

- 追加:
  - `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
- 更新:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/90-source-traceability.md`
  - `progress.md`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_static_gate_loop.py`

## 6. Commands run

- `python3 -m unittest scripts.tests.test_current_l2_reason_code_readiness`
- `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/current_l2_detached_loop.py scan-reason-code-readiness crates/mir-ast/tests/fixtures/current-l2 --run-label readiness-scan --overwrite`
- `python3 -m unittest scripts.tests.test_current_l2_checked_reasons_assist scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_reason_code_readiness scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_scaffold_fixture scripts.tests.test_current_l2_diff_static_gate_artifacts`
- `cargo test -p mir-semantics`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 7. Evidence / outputs / test results

- targeted helper tests は GREEN。
- final targeted Python suite は `Ran 23 tests in 0.068s` / `OK`。
- `cargo test -p mir-semantics` は unit 2 件、support integration 2 + 2 + 8 件、main integration 40 件、doc-tests 0 件が pass した。
- `python3 scripts/validate_docs.py` は `Documentation scaffold looks complete.` / `Found 167 numbered report(s).`
- `git diff --check` は無出力。
- review 記録は `docs/reports/0167-review-static-reason-code-readiness-scan.md` に残した。reviewer completion retrieval はこの session の tool surface では取れなかったため、180 秒の wait window を守ったうえで local evidence fallback を明記した。
- current corpus に対する smoke では次を確認した。
  - `static-only fixtures scanned: 10`
  - `runtime fixtures skipped: 9`
  - `fixtures with checked_reasons: 8`
  - `fixtures with reason_codes suggestions: 8`
  - suggestion kinds は `missing_lineage_assertion`、`lineage_assertion_edge_mismatch`、`declared_target_missing`、`declared_target_mismatch`、`capability_strengthens`、`missing_chain_head_option`、`missing_predecessor_option`、`missing_successor_option`
  - no-suggestion fixture は `e14` / `e15`

## 8. What changed in understanding

- `reason_codes` assist は 1 fixture authoring には十分だが、typed carrier actualization の着手順を決めるには corpus 横断の readiness summary があると判断がかなり楽になる。
- current corpus では stable cluster と duplicate cluster の split がすでに実装 / docs / detached artifact mirror で揃っており、scan helper はその split を display-only に可視化するだけで十分有益である。

## 9. Open questions

- future first-class typed carrier をどの stable cluster から actualize するか
- readiness scan の結果を detached aggregate 側へ寄せる価値があるか
- runtime fixture 側の static gate readiness を別 helper に切る必要があるか

## 10. Suggested next prompt

static-only corpus 横断の readiness scan を前提に、future typed carrier actualization をどの stable cluster から始めるのが最も narrow で安全かを docs / tests / fixture corpus から比較してください。
