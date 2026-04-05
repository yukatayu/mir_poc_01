# Report 0174 — same-lineage first checker spike

- Date: 2026-04-05T16:25:00Z
- Author / agent: Codex
- Scope: first checker cut の actual first spike として same-lineage static evidence floor を helper-local に切り出し、detached validation loop から smoke できる最小 helper / wrapper / docs mirror を追加する
- Decision levels touched: L2

## 1. Objective

current static-only corpus baseline が揃った前提で、

- first checker spike を same-lineage floor / missing-option floor / capability floor のどれから始めるかを narrow に比較し、
- same-lineage floor を helper-local / non-production helper として actualize し、
- detached validation loop から 1 fixture smoke で回せる最小 wrapper を追加する

ことを目的とする。

## 2. Scope and assumptions

- current L2 semantics、parser grammar、failure family、machine-check policy は変えない。
- `checked_reasons` / `checked_reason_codes` の additive coexistence は維持する。
- first checker spike は helper-local / non-production helper に留め、public checker API にはしない。
- fixed するのは same-lineage family の narrow compare helper だけであり、missing-option / capability は後段 comparison に残す。

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
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_reason_codes_assist.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/tests/test_current_l2_same_lineage_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. first checker spike 候補を same-lineage / missing-option / capability で比較し、same-lineage family を first cut とする current judgment を `specs/examples/46...` に追加した。
2. `scripts/tests/test_current_l2_same_lineage_checker.py` を追加し、same-lineage row filter / matched / missing fixture rows / out-of-scope を RED/GREEN で固定した。
3. `scripts/current_l2_same_lineage_checker.py` を追加し、fixture-side `checked_reason_codes` と detached static gate artifact の actual reason rows を読み、same-lineage family だけを filter して exact compare する helper-local checker spike を実装した。
4. `scripts/tests/test_current_l2_static_gate_loop.py` と `scripts/current_l2_detached_loop.py` を更新し、`smoke-same-lineage-checker` wrapper を追加した。
5. `e4-malformed-lineage` と `e12-underdeclared-target-missing` に対して actual smoke を取り、same-lineage family の matched case を実地確認した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/13`、`plan/15`、`plan/90`、`progress.md` を mirror 更新した。

## 5. Files changed

- 追加:
  - `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/tests/test_current_l2_same_lineage_checker.py`
  - `docs/reports/0174-same-lineage-first-checker-spike.md`
- 更新:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/90-source-traceability.md`
  - `progress.md`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_static_gate_loop.py`

## 6. Commands run

- `python3 -m unittest scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/current_l2_detached_loop.py smoke-same-lineage-checker crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --run-label same-lineage-smoke --overwrite`
- `python3 scripts/current_l2_detached_loop.py smoke-same-lineage-checker crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json --run-label same-lineage-smoke --overwrite`

## 7. Evidence / outputs / test results

- targeted Python suite:
  - `Ran 9 tests in 0.006s`
  - `OK`
- smoke `e4-malformed-lineage`:
  - `cluster: same_lineage_evidence_floor`
  - `status: matched`
  - `lineage_assertion_edge_mismatch` row が fixture / actual で一致
- smoke `e12-underdeclared-target-missing`:
  - `cluster: same_lineage_evidence_floor`
  - `status: matched`
  - `declared_target_missing` row が fixture / actual で一致

## 8. What changed in understanding

- first checker cut の actual spike は same-lineage family から始めるのが最小であり、current static-only corpus baseline と detached static gate artifact だけで narrow helper を構成できる。
- missing-option / capability は current helper-local spike の後段 comparison に残す方が自然である。
- same-lineage family は row compare だけで helper-local smoke を回せるため、public checker API を固定する前の実地 verification として扱いやすい。

## 9. Open questions

- missing-option structure floor を次点 spike として続けるか
- capability floor coverage を増やしてから second spike へ進むか
- same-lineage checker spike を later task で shared support helper に寄せるか
- final checker API をどこへ置くか

## 10. Suggested next prompt

same-lineage first checker spike が helper-local smoke で通った前提で、次の actual checker spike を missing-option structure floor にするべきか、それとも capability floor の corpus coverage を先に厚くするべきかを narrow に比較してください。
