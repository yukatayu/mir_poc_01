# Report 0178 — capability floor third checker spike

- Date: 2026-04-06T00:56:00+09:00
- Author / agent: Codex
- Scope: current L2 parser-free PoC の static-only corpus に capability floor late-edge variant を追加し、capability strengthening family の helper-local third checker spike を narrow に actualize する
- Decision levels touched: L2

## 1. Objective

- `e13` だけだった capability floor coverage を actual corpus で厚くする
- `capability_strengthens` family を same-lineage / missing-option と同じ pattern の checker spike として detached validation loop へ接続する
- public checker API、runtime semantics、fixture schema の新規拡張は行わない

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `progress.md`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_detached_loop.py`

## 3. Actions taken

- static-only malformed fixture `e20-malformed-late-capability-strengthening.json` を追加した
- Rust test corpus / selection count / static gate detached reason-code emission を `e20` に追従させた
- `scripts/current_l2_capability_checker.py` を追加し、`capability_strengthens` family だけを narrow compare する helper-local third checker spike を実装した
- `scripts/current_l2_detached_loop.py` に `smoke-capability-checker` wrapper を追加した
- Python tests を追加し、capability checker helper と wrapper の最低限の contract を固定した
- `specs/examples/48-current-l2-capability-third-checker-spike.md` を追加し、current cut を docs-only で固定した
- fixture schema / roadmap / helper stack / open problems / heavy future workstreams / progress を capability floor `2` と third spike に合わせて更新した

## 4. Files changed

- 追加:
  - `crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json`
  - `scripts/current_l2_capability_checker.py`
  - `scripts/tests/test_current_l2_capability_checker.py`
  - `specs/examples/48-current-l2-capability-third-checker-spike.md`
  - `docs/reports/0179-review-capability-floor-third-checker-spike.md`
- 変更:
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_static_gate_loop.py`
  - `specs/examples/02-current-l2-ast-fixture-schema.md`
  - `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/08-representative-programs-and-fixtures.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/90-source-traceability.md`
  - `progress.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-semantics`
  - 初回: `discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only`、`run_directory_profiled_static_only_includes_profile_name_in_summary`、`run_directory_returns_summary_for_current_l2_fixture_dir` が count mismatch で失敗
  - 修正後: `test result: ok. 44 passed; 0 failed`
- `python3 -m unittest scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_static_gate_loop`
  - 初回: `ModuleNotFoundError: No module named 'current_l2_capability_checker'` と wrapper hook 欠落で失敗
  - 修正後: `Ran 11 tests in 0.861s` / `OK`
- `python3 -m unittest scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 17 tests in 0.030s` / `OK`
- `python3 scripts/current_l2_detached_loop.py smoke-capability-checker crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json --run-label capability-spike-smoke --overwrite`
  - `cluster: capability_strengthening_floor`
  - `status: matched`
- `python3 scripts/current_l2_detached_loop.py smoke-capability-checker crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json --run-label capability-spike-smoke --overwrite`
  - `cluster: capability_strengthening_floor`
  - `status: matched`

## 6. Evidence / findings

- capability floor は `e13` 単独ではなく `e13` / `e20` の 2 fixture になり、singleton family ではなくなった
- `capability_strengthens` row は fixture-side `checked_reason_codes` と detached static gate artifact `reason_codes` の narrow compare だけで helper-local checker spike に載せられる
- same-lineage / missing-option / capability の 3 family が同じ detached static gate wrapper family で smoke 可能になった
- current cut は helper-local / non-production compare に留まり、public checker API や shared support helper にはまだ上げていない

## 7. Changes in understanding

- first checker cut baseline の capability floor は `1` ではなく `2` まで actual corpus で厚くしてよい
- capability family は same-lineage / missing-option に続く third checker spike として actualize してよい
- 次の narrow question は「3 family spike を shared support helper に寄せるか、family ごとの helper-local compare をしばらく維持するか」である

## 8. Open questions

- same-lineage / missing-option / capability の 3 spike を shared support helper へ寄せるべきか
- capability floor をさらに増やす fixture が要るか
- final checker API をどこへ置くか
- reviewer completion が得られない場合は local evidence fallback を review report にどう残すか

## 9. Suggested next prompt

`same-lineage / missing-option / capability の 3 checker spike を shared support helper へ寄せるべきか、それとも family ごとの helper-local compare を維持するべきかを narrow に比較し、必要なら non-production の shared family compare helper を追加してください。`
