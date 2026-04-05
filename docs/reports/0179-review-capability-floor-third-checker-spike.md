# Report 0179 — review capability floor third checker spike

- Date: 2026-04-06T01:12:00+09:00
- Author / agent: Codex
- Scope: capability floor third checker spike diff を maintainer 観点で review し、helper-local boundary、corpus drift、docs / plan / progress mirror、direct regression coverage を点検する
- Decision levels touched: L2

## 1. Objective

current working tree change について、次を確認する。

- `scripts/current_l2_capability_checker.py` が helper-local / non-production boundary を越えていないこと
- capability floor coverage `2` への更新が docs / plan / progress / traceability に正しく mirror されていること
- `e20-malformed-late-capability-strengthening` が detached static gate だけでなく direct `harness.run_fixture()` regression でも抜けずに通っていること

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
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `scripts/current_l2_capability_checker.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_capability_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `docs/reports/0178-capability-floor-third-checker-spike.md`

## 3. Actions taken

1. reviewer を 1 回起動し、`180000ms` を 1 回待った。
2. reviewer は timeout したため、許容された retry 1 回として再度 `180000ms` 待った。
3. 2 回目の wait で reviewer completion を受け取り、3 件の finding を確認した。
4. baseline doc の capability coverage 文言、`plan/12` の singleton wording、`current_l2_minimal_interpreter.rs` の direct malformed loop を修正した。
5. full cargo test、Python targeted tests、family smoke、docs validation、`git diff --check` を reviewer fix 後に再実行した。

## 4. Files changed

- 追加:
  - `docs/reports/0179-review-capability-floor-third-checker-spike.md`
- 変更:
  - `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
  - `plan/12-open-problems-and-risks.md`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 5. Commands run and exact outputs

- `wait_agent(reviewer, 180000)` -> timeout
- `wait_agent(reviewer, 180000)` -> completed with 3 findings
- `cargo test -p mir-semantics`
  - `test result: ok. 44 passed; 0 failed`
- `python3 -m unittest scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 17 tests in 0.030s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py smoke-capability-checker crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json --run-label capability-spike-smoke --overwrite`
  - `cluster: capability_strengthening_floor`
  - `status: matched`
- `python3 scripts/current_l2_detached_loop.py smoke-capability-checker crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json --run-label capability-spike-smoke --overwrite`
  - `cluster: capability_strengthening_floor`
  - `status: matched`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 179 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

### reviewer findings

1. baseline doc の open question が capability coverage をまだ `1 fixture` 前提で書いており、同一文書内の baseline `2` と矛盾していた。
2. `plan/12` が capability family を singleton と呼んだままで、`e13` / `e20` の 2 fixture current state と drift していた。
3. `e20` が detached static gate support には入っていたが、direct `harness.run_fixture()` malformed / underdeclared loop には入っていなかった。

### fix result

- finding 1 は `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` で「coverage `2` が十分か、それともさらに厚くするか」という open question に補正した。
- finding 2 は `plan/12-open-problems-and-risks.md` で capability family wording を `singleton` から `family` に補正した。
- finding 3 は `current_l2_minimal_interpreter.rs` の malformed / underdeclared loop に `malformed_late_strengthening` を追加して塞いだ。

### local review result

- 上記 3 件以外に substantive finding は残らなかった。
- `scripts/current_l2_capability_checker.py` は fixture-side `checked_reason_codes` と detached static gate artifact `reason_codes` の narrow compare に留まり、`lib.rs` / `harness.rs` の public behavior や schema を拡張していない。
- capability third checker spike は same-lineage / missing-option と同じ helper-local pattern に留まり、shared helper / final checker API を既成事実化していない。

## 7. Changes in understanding

- capability floor は current baseline で `2` fixture family として扱うべきであり、singleton wording を repo memory に残すべきではない。
- third checker spike を actualize した以上、direct `harness.run_fixture()` path も detached path と同程度に押さえる必要がある。
- 次の narrow question は capability まで含めた 3 family spike を helper-local のまま維持するか、non-production shared support helper に薄く寄せるかである。

## 8. Open questions

- same-lineage / missing-option / capability の 3 checker spike を shared support helper へ寄せるべきか
- capability floor fixture を `2` でいったん止めるか、さらに variant を増やすか
- shared helper を切る場合、family script を残した facade として実装するか、checker-side compare helper を別 entry にするか

## 9. Suggested next prompt

same-lineage / missing-option / capability の 3 checker spike を shared support helper へ寄せるべきか、それとも family ごとの helper-local compare を維持するべきかを narrow に比較し、必要なら non-production の shared family compare helper を追加してください。
