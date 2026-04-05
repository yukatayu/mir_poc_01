# Report 0177 — review missing-option second checker spike

- Date: 2026-04-05T17:05:00Z
- Author / agent: Codex
- Scope: current working tree の missing-option second checker spike diff を maintainer 観点で review し、helper-local boundary、detached validation loop wrapper、docs / plan / progress mirror、traceability を点検する
- Decision levels touched: L2

## 1. Objective

current working tree change について、次を確認する。

- `scripts/current_l2_missing_option_checker.py` が helper-local / non-production boundary を越えていないこと
- `smoke-missing-option-checker` wrapper が detached validation loop の convenience に留まり、public checker API と誤読されないこと
- docs / plan / progress / report / traceability が missing-option second spike judgment を正しく mirror していること

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
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_missing_option_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `docs/reports/0176-missing-option-second-checker-spike.md`

## 3. Actions taken

1. reviewer を 1 回起動し、`180000ms` を 2 回待った。
2. reviewer は completion を返さなかったため、local evidence fallback を採用した。
3. target diff を読み、missing-option checker helper と wrapper が helper-local / non-production boundary に留まるかを点検した。
4. docs / plan / progress / traceability mirror が missing-option second spike judgment と capability 未決を正しく反映しているかを確認した。
5. targeted unit test と actual smoke を review evidence として採用した。

## 4. Files changed

- 追加:
  - `docs/reports/0177-review-missing-option-second-checker-spike.md`
- `plan/` 更新は task 本体側で実施済み

## 5. Commands run and exact outputs

- `wait_agent(reviewer, 180000)` -> timeout
- `wait_agent(reviewer, 180000)` -> timeout
- `python3 -m unittest scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_static_gate_loop`
- `python3 scripts/current_l2_detached_loop.py smoke-missing-option-checker crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json --run-label review-missing-option --overwrite`
- `python3 scripts/current_l2_detached_loop.py smoke-missing-option-checker crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json --run-label review-missing-option --overwrite`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 6. Evidence / findings

### reviewer completion

- reviewer は 2 回の wait window 内では completion を返さなかった。
- current task では retry 1 回までという運用に従い、local evidence fallback へ切り替えた。

### Local finding result

- substantive finding は追加で見つからなかった。
- `scripts/current_l2_missing_option_checker.py` は fixture-side `checked_reason_codes` と detached static gate artifact の actual rows を読む helper-local compare に留まり、`lib.rs` / `harness.rs` の API や fixture schema を増やしていない。
- `smoke-missing-option-checker` は static gate artifact emit と helper 実行を束ねる convenience であり、detached validation loop の non-production wrapper を越えていない。
- docs / plan / progress / traceability は missing-option second spike judgment、capability coverage 先行という alternative、public checker API 未決で揃っている。

## 7. Changes in understanding

- same-lineage spike の次段は missing-option family から始めるのが最小であり、capability floor は current corpus coverage がまだ薄いまま別タスクに残す方が自然である。
- current diff は helper-local boundary を維持できており、missing-option second spike judgment は actual smoke と一致している。

## 8. Open questions

- capability floor coverage をどの fixture で増やすか
- `e18` まで routine smoke として常設するか
- missing-option helper を later task で shared support helper に寄せるか

## 9. Suggested next prompt

missing-option second checker spike の次段として、capability floor fixture を増やすか、same-lineage / missing-option spike を shared support helper に寄せるかを比較してください。
