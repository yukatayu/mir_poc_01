# Report 0163 — static reason codes authoring assist

- Date: 2026-04-05T13:19:46.854248Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC の detached static gate artifact にある helper-local / reference-only `reason_codes` を、future typed carrier 候補 row として display-only に確認する non-production assist を追加する
- Decision levels touched: L2

## 1. Objective

`checked_reasons` assist の次段として、fixture schema や machine-check core を動かさずに future typed carrier 候補 row を review しやすくする。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_checked_reasons_assist.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_checked_reasons_assist.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`

## 3. Actions taken

1. `checked_reasons` assist と static gate artifact の current shape を再確認した。
2. failing test を先に追加し、`current_l2_reason_codes_assist.py` 不在と loop wrapper 未接続で RED を確認した。
3. `scripts/current_l2_reason_codes_assist.py` を追加し、`detached_noncore.reason_codes` だけを source にする display-only assist を実装した。
4. `scripts/current_l2_detached_loop.py` に `suggest-reason-codes` subcommand を追加した。
5. `specs/examples/38-current-l2-static-reason-codes-authoring-assist.md` を追加し、current cut を docs-only で固定した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/15`、`plan/90`、`progress.md` を mirror 更新した。
7. 実 fixture を使って、stable cluster では suggestion が出て duplicate cluster では出ないことを smoke で確認した。
8. reviewer finding を受けて、fixture 側に unsupported `expected_static.reason_codes` がある場合は assist を fail-closed に止める negative case を追加した。

## 4. Files changed

- 追加:
  - `specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`
  - `scripts/current_l2_reason_codes_assist.py`
  - `scripts/tests/test_current_l2_reason_codes_assist.py`
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

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_reason_codes_assist`
  - RED: `ModuleNotFoundError: No module named 'current_l2_reason_codes_assist'`
  - REVIEW FIX RED: `AssertionError: 0 != 2`
  - GREEN: `Ran 4 tests in 0.004s` / `OK`
- `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop`
  - RED: `AttributeError: module 'current_l2_detached_loop' has no attribute 'reason_codes_assist'`
  - GREEN: `Ran 4 tests in 0.003s` / `OK`
- `python3 scripts/current_l2_detached_loop.py suggest-reason-codes crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json --run-label assist-e19 --overwrite`
  - stable cluster 側で `declared_target_mismatch` row を display-only suggestion として表示
- `python3 scripts/current_l2_detached_loop.py suggest-reason-codes crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json --run-label assist-e14 --overwrite`
  - duplicate cluster 側で `detached artifact has no reason_codes suggestion`
- `python3 -m unittest scripts.tests.test_current_l2_checked_reasons_assist scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_scaffold_fixture scripts.tests.test_current_l2_diff_static_gate_artifacts`
  - `Ran 20 tests in 0.031s`
  - `OK`
- `cargo test -p mir-semantics`
  - `test result: ok. 40 passed; 0 failed; ...`
  - `test result: ok. 8 passed; 0 failed; ...`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 165 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

- `reason_codes` assist は stable cluster では row を出し、duplicate cluster では no-suggestion になる。
- assist は current fixture-side typed carrier が absent であることを毎回明示する。
- unsupported fixture-side typed field を見つけたときは assist が fail-closed に止まる。
- suggestion source は `checker_core.reasons` ではなく `detached_noncore.reason_codes` に限定した。
- `checked_reasons` assist と `reason_codes` assist の役割を分けたまま helper stack に追加できた。

## 7. Changes in understanding

- `reason_codes` assist は typed carrier actualization の前段として有効だが、fixture-side field を仮置きしない cut が重要である。
- absence メッセージだけでは境界 violation を隠しうるため、unsupported field の explicit rejection を assist に持たせる必要がある。
- duplicate declaration cluster を current stable code inventory に上げない judgmentを、authoring assist の no-suggestion behavior でも実地確認できた。

## 8. Open questions

- future first-class typed carrier を fixture-side / detached-side のどちらへ置くか
- final typed row schema の exact serialization
- `checked_reasons` と typed carrier の migration timing
- duplicate declaration cluster をいつ stable code inventory に上げるか

## 9. Suggested next prompt

`reason_codes` display-only assist を前提に、future typed carrier actualization を急がずに fixture authoring 実地反復をもう 1〜2 回回し、どの cluster で assist が本当に効くかを source-backed に確認してください。
