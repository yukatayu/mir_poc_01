# Report 0894 — model check second line reserve package summary index actualization

- Date: 2026-04-20T14:31:19.682821Z
- Author / agent: Codex (GPT-5)
- Scope: Problem 1 reserve integration lane のうち `model-check-second-line` package を repo-local summary index まで actualize し、sample docs / snapshot / roadmap / traceability を同期する
- Decision levels touched: L1 / L2 / L3 を含むが、final public checker artifact / final public verifier contract は触らない

## 1. Objective

`specs/examples/478` と `568` で source-backed に残っていた model-check second-line を、

- `emit-reserve model-check-second-line`
- `target/current-l2-guided/reserve-packages/model-check-second-line/package-summary.md|json`
- `p06 / p10 / p11 / p12 / p15 / p16` run-source-sample JSON

まで materialize し、Problem 1 の representative bridge / positive carrier / bad pattern rejection pair を 1 本の repo-local reopen entrypoint に圧縮する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/prototype/current-l2-typed-proof-model-check/README.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `specs/examples/606-current-l2-reserve-integration-entrypoint-summary-sync.md`
- `specs/examples/610-current-l2-auditable-authority-witness-reserve-package-summary-index-actualization.md`
- `specs/examples/611-current-l2-delegated-rng-service-reserve-package-summary-index-actualization.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. `scripts/tests/test_current_l2_guided_samples.py` に `model-check-second-line` reserve package 用の RED tests を追加し、missing implementation failure を確認した。
2. `scripts/current_l2_guided_samples.py` に
   - `ModelCheckSecondLineEmitRow`
   - `emit-reserve model-check-second-line`
   - `package-summary.md|json` 書き出し
   - reserve summary entrypoint への統合
   を追加した。
3. `scripts/tests/test_problem_sample_bundles.py` を広げ、Problem 1 bundle doc が新しい reserve package command と output dir を含むことを固定した。
4. `samples/problem-bundles/problem1-typed-theorem-model-check.md` と `samples/prototype/current-l2-typed-proof-model-check/README.md` に、`emit-reserve model-check-second-line` の practical walkthrough を追加した。
5. `Documentation.md`、`progress.md`、`tasks.md`、`plan/11`、`plan/18`、`plan/90` を、reserve package summary index 4 本が揃った current reading に更新した。
6. `specs/examples/612` を追加し、今回の actualization cut / retained alternatives / stop line を規範側から辿れるようにした。

## 4. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/prototype/current-l2-typed-proof-model-check/README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/612-current-l2-model-check-second-line-reserve-package-summary-index-actualization.md`
- `docs/reports/0894-model-check-second-line-reserve-package-summary-index-actualization.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_supported_emit_reserve_package_ids_include_model_check_second_line ...`
  - 初回は `ModelCheckSecondLineEmitRow` / renderer / manifest 未実装で failure / error を確認した。
- 同じ targeted unittest を再実行
  - `Ran 5 tests ... OK`
- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `Ran 88 tests in 0.200s`
  - `OK`
- `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line --format pretty`
  - `model-check second-line reserve package`
  - `p06-typed-proof-owner-handoff: representative theorem-model-check bridge`
  - `p11-typed-unauthorized-fingerprint-release: authority miss rejection`
  - `p12-typed-classified-fingerprint-publication-block: label-flow rejection`
  - `p15-typed-capture-escape-rejected: capture/lifetime rejection`
  - `p16-typed-remote-call-budget-exceeded: simple cost rejection`
- `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line --format json`
  - `package_id = model-check-second-line`
  - `rows = p06 / p10 / p11 / p12 / p15 / p16`
- `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - `Ran 91 tests in 0.177s`
  - `OK`
- `python3 scripts/current_l2_guided_samples.py reserve`
  - reserve package 4 本の entry command 群に `emit-reserve model-check-second-line` が加わったことを確認した。
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - no output

## 6. Evidence / findings

- `emit-reserve model-check-second-line` は実行可能で、`target/current-l2-guided/reserve-packages/model-check-second-line/` に summary index と sample JSON 群を materialize する。
- `p06` は representative theorem-model-check bridge、`p10` は positive carrier、`p11 / p12 / p15 / p16` は rejection pair として 1 本の helper-local package に収まる。
- `typed_hint_status` / `theorem_preview_status` / `model_check_preview_status` / `model_check_reopen_status` を並べることで、checker-adjacent / theorem-first / model-check second-line を collapse せずに読める。
- reserve package summary index はこれで
  - theorem-first external pilot
  - auditable authority witness
  - delegated RNG service
  - model-check second line
  の 4 本が揃った。

## 7. Changes in understanding

- Problem 1 の practical sample floor は、`matrix problem1` と `bundle problem1` だけではなく、`emit-reserve model-check-second-line` を持つことで「実際に通る例と落ちる例を output dir ごと materialize する」読みへ一段上げられる。
- bad pattern rejection の可視化は、new sample を増やすより、既存 corrected prototype を reserve package summary index に束ねる方が歪みが少ない。
- reserve integration lane は、current-default の near-end closeout line として十分に実行可能であり、remaining work は mixed gate / true user-spec residual へより明確に寄せられる。

## 8. Open questions

- first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、final public verifier contract は引き続き mixed gate / later reopen に残る。
- practical sample floor はかなり固まったが、final public language implementation complete を主張できる段階ではまだない。

## 9. Suggested next prompt

remaining mixed gate lane を reopen する場合は、`python3 scripts/current_l2_guided_samples.py residuals` と `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams` を入口に、final public seam 側の narrow reopen を進めてください。
