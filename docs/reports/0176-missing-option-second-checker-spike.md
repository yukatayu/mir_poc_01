# Report 0176 — missing-option second checker spike

- Date: 2026-04-05T16:55:00Z
- Author / agent: Codex
- Scope: same-lineage first checker spike の次段として missing-option structure floor を helper-local second checker spike に切り出し、detached validation loop から smoke できる最小 helper / wrapper / docs mirror を追加する
- Decision levels touched: L2

## 1. Objective

same-lineage first checker spike が通った前提で、

- second spike を missing-option family / capability family / capability coverage 先行のどれにするかを narrow に比較し、
- missing-option family を helper-local / non-production helper として actualize し、
- detached validation loop から 1 fixture smoke で回せる最小 wrapper を追加する

ことを目的とする。

## 2. Scope and assumptions

- current L2 semantics、parser grammar、failure family、machine-check policy は変えない。
- `checked_reasons` / `checked_reason_codes` の additive coexistence は維持する。
- second checker spike は helper-local / non-production helper に留め、public checker API にはしない。
- fixed するのは missing-option family の narrow compare helper だけであり、capability family は後段に残す。

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
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/tests/test_current_l2_missing_option_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. second checker spike 候補を missing-option / capability / capability coverage 先行で比較し、missing-option family を second cut とする current judgment を `specs/examples/47...` に追加した。
2. `scripts/tests/test_current_l2_missing_option_checker.py` を追加し、missing-option row filter / matched / missing fixture rows / out-of-scope を RED/GREEN で固定した。
3. `scripts/current_l2_missing_option_checker.py` を追加し、fixture-side `checked_reason_codes` と detached static gate artifact の actual reason rows を読み、missing-option family だけを filter して exact compare する helper-local checker spike を実装した。
4. `scripts/tests/test_current_l2_static_gate_loop.py` と `scripts/current_l2_detached_loop.py` を更新し、`smoke-missing-option-checker` wrapper を追加した。
5. `e16-malformed-missing-chain-head-option` と `e17-malformed-missing-predecessor-option` に対して actual smoke を取り、missing-option family の matched case を実地確認した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/13`、`plan/15`、`plan/90`、`progress.md` を mirror 更新した。

## 5. Files changed

- 追加:
  - `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/tests/test_current_l2_missing_option_checker.py`
  - `docs/reports/0176-missing-option-second-checker-spike.md`
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

- `python3 -m unittest scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/current_l2_detached_loop.py smoke-missing-option-checker crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json --run-label missing-option-smoke --overwrite`
- `python3 scripts/current_l2_detached_loop.py smoke-missing-option-checker crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json --run-label missing-option-smoke --overwrite`

## 7. Evidence / outputs / test results

- targeted Python suite:
  - `Ran 10 tests in 0.008s`
  - `OK`
- smoke `e16-malformed-missing-chain-head-option`:
  - `cluster: missing_option_structure_floor`
  - `status: matched`
  - `missing_chain_head_option` row が fixture / actual で一致
- smoke `e17-malformed-missing-predecessor-option`:
  - `cluster: missing_option_structure_floor`
  - `status: matched`
  - `missing_predecessor_option` row が fixture / actual で一致

## 8. What changed in understanding

- second checker spike は missing-option family から始めるのが最小であり、same-lineage spike と同じ helper-local compare pattern で actualize できる。
- capability floor はまだ coverage が薄く、actual helper cut より先に evidence density を増やす判断の方が自然である。

## 9. Open questions

- `e18` まで routine smoke として常設するか
- capability floor coverage をどの fixture で増やすか
- missing-option checker spike を later task で shared support helper に寄せるか
- final checker API をどこへ置くか

## 10. Suggested next prompt

same-lineage と missing-option の checker spike が helper-local smoke で通った前提で、次は capability floor fixture を増やすべきか、それとも 2 spike を shared support helper に薄く寄せるべきかを narrow に比較してください。
