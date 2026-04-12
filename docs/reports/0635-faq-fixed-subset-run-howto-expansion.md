# Report 0635 — FAQ 002 fixed-subset run how-to expansion

- Date: 2026-04-12 16:24:28 JST
- Author / agent: Codex
- Scope: `faq_002.md` へ fixed subset 実行手順と actual output を追記
- Decision levels touched: none; documentation / evidence only

## 1. Objective

`faq_002.md` の「fixed subset についてはもう走る」という説明に対し、

- 実際にどういう command で確認するのか
- 何を入力として見るのか
- どこに結果 artifact が出るのか
- 実際に 2026-04-12 にどういう output が得られたのか

を、省略せずに追記する。

## 2. Inputs consulted

- `faq_002.md`
- `samples/current-l2/README.md`
- `samples/current-l2/e2-try-fallback.txt`
- `samples/current-l2/e4-malformed-lineage.txt`
- `samples/current-l2/e23-malformed-try-fallback-missing-fallback-body.txt`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_detached_loop.py`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`

## 3. Actions taken

1. current FAQ の sample execution 節を再読し、user に不足しているのが「実手順」と「actual output」であることを確認した。
2. current source sample 本文 3 本を読み、runtime representative と static representative を選んだ。
3. inventory / runner test / smoke-formal-hook runtime / smoke-formal-hook static / full regression bundle を repo root で実行した。
4. 生成された static gate / formal hook JSON を開き、FAQ に載せる representative output を選んだ。
5. `faq_002.md` に runbook 形式の subsection を追記した。

## 4. Files changed

- Updated: `faq_002.md`
- Added: `docs/reports/0635-faq-fixed-subset-run-howto-expansion.md`

## 5. Commands run and exact outputs

- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - output:
    - `current L2 fixed-subset first-cluster inventory`
    - `e2-try-fallback | source-authored | valid | success | runtime_try_cut_cluster | present | first authored trio runtime path`
    - `e4-malformed-lineage | source-authored | malformed | not_evaluated | fixture_static_cluster | present | first authored trio static stop`
    - `e23-malformed-try-fallback-missing-fallback-body | source-authored | malformed | not_evaluated | fixture_static_cluster | present | first authored trio static stop`
    - `e1-place-atomic-cut | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
    - `e3-option-admit-chain | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
    - `e21-try-atomic-cut-frontier | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner -- --nocapture`
  - output summary:
    - `running 5 tests`
    - `current_l2_source_sample_runner_accepts_named_e23_sample ... ok`
    - `current_l2_source_sample_runner_accepts_explicit_e4_path ... ok`
    - `current_l2_source_sample_runner_rejects_unknown_sample_stem ... ok`
    - `current_l2_source_sample_runner_rejects_existing_out_of_scope_explicit_file ... ok`
    - `current_l2_source_sample_runner_resolves_named_e2_sample_and_runs_runtime ... ok`
    - `test result: ok. 5 passed; 0 failed`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json --artifact-root target/current-l2-source-sample-regression-faq002-howto --run-label faq002-howto-e2-try-fallback --overwrite`
  - output:
    - `bundle artifact: target/current-l2-source-sample-regression-faq002-howto/bundles/faq002-howto-e2-try-fallback/e2-try-fallback.detached.json`
    - `formal hook artifact: target/current-l2-source-sample-regression-faq002-howto/formal-hooks/faq002-howto-e2-try-fallback/e2-try-fallback.formal-hook.json`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --artifact-root target/current-l2-source-sample-regression-faq002-howto --run-label faq002-howto-e4-malformed-lineage --overwrite`
  - output:
    - `static gate artifact: target/current-l2-source-sample-regression-faq002-howto/static-gates/faq002-howto-e4-malformed-lineage/e4-malformed-lineage.static-gate.json`
    - `formal hook artifact: target/current-l2-source-sample-regression-faq002-howto/formal-hooks/faq002-howto-e4-malformed-lineage/e4-malformed-lineage.formal-hook.json`
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label faq002-howto --artifact-root target/current-l2-source-sample-regression-faq002-howto`
  - output summary:
    - `[1/7] runtime lowering test ... ok`
    - `[2/7] source sample runner test ... ok`
    - `[3/7] verification ladder test ... ok`
    - `[4/7] formal hook support test ... ok`
    - `[5/7] runtime formal hook smoke for e2-try-fallback`
    - `[6/7] static formal hook smoke for e4-malformed-lineage`
    - `[7/7] static formal hook smoke for e23-malformed-try-fallback-missing-fallback-body`
    - `all regression commands passed`

## 6. Evidence / findings

- current repo で sample execution を shell command だけで end-to-end に見るときの current入口は、final public CLI ではなく `inventory` / `cargo test` / `current_l2_detached_loop.py` / generated JSON artifact である。
- runtime representative `e2` は source text、runner test、runtime formal hook artifact の 3 段で確認できる。
- static representative `e4` は source text、static gate artifact、static formal hook artifact の 3 段で確認できる。
- full regression helper を回せば、lowering / runner / ladder / formal-hook smoke を 1 command に束ねて再現できる。

## 7. Changes in understanding

- 「もう走る」の意味を、public executable があることと誤解しやすいので、current repo では helper/test/artifact route であることを明示した方がよいと再確認した。
- runtime success sample と static malformed sample を 1 本ずつ concrete に見せると、current fixed subset の reachable boundary がかなり明確になる。

## 8. Open questions

- final public CLI / exporter をどの phase で導入するか。
- source sample から runtime report を直接 human-friendly に表示する thin command を later に切るか。
- wider authored row (`e1` / `e3` / `e21`) をどの順で actualize するか。

## 9. Suggested next prompt

`faq_002.md` の runbook を踏まえて、次は `e1` / `e3` / `e21` のうちどれを先に source-authored row へ昇格させるべきかを、verification ladder と proof boundary の観点から比較してください。

## 補足

- `Documentation.md` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `plan/` 更新不要
