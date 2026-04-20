# Report 0889 — Package 136 theorem-first external pilot summary index actualization

- Date: 2026-04-20T11:56:07Z
- Author / agent: Codex
- Scope: reserve package として theorem-first external pilot の output dir summary index を actualize する
- Decision levels touched: L2

## 1. Objective

- `emit-theorem problem1` が出力する representative theorem bundle JSON に加えて、notebook-first theorem pilot をまとめて読み直せる summary index を output dir に materialize する。
- theorem-first external pilot を reserve package として close し、next reopen line を `auditable_authority_witness` / `delegated_rng_service` / model-check second-line へ進める。
- final public theorem contract、proof object public schema、concrete theorem prover production binding、final public verifier contract は still later に残す。

## 2. Scope and assumptions

- 対象は theorem-first external pilot の repo-local emitted artifact loop であり、concrete theorem prover brand や public verifier contract ではない。
- representative sample は `p06`、support pair は `p07 / p08` に固定する。
- output dir 内の summary index は helper-local / non-production artifact に留める。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/601-current-l2-theorem-first-emitted-artifact-loop-hardening.md`
- `specs/examples/603-current-l2-problem1-executable-residual-reopen-sync.md`
- `specs/examples/606-current-l2-reserve-integration-entrypoint-summary-sync.md`
- `specs/examples/608-current-l2-true-user-spec-hold-line-freeze-sync.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`

## 4. Actions taken

- `scripts/current_l2_guided_samples.py` の `emit-theorem problem1` manifest に `pilot-summary.md` / `pilot-summary.json` を追加した。
- emitted summary index を output dir に書き出すようにし、pretty output / JSON manifest から path を辿れるようにした。
- Problem 1 sample bundle doc に summary index の見方を追記した。
- reserve integration lane の theorem-first external pilot anchor に current actualization note を追加した。
- snapshot docs を theorem-first external pilot close 後の current queue に同期した。

## 5. Files changed

- `docs/reports/0889-package136-theorem-first-external-pilot-summary-index-actualization.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/609-current-l2-theorem-first-external-pilot-summary-index-actualization.md`

## 6. Commands run

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_problem1_theorem_emit_text_mentions_output_dir_command_and_samples scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_problem1_theorem_emit_manifest_writes_pilot_index_files scripts.tests.test_problem_sample_bundles.ProblemSampleBundleDocsTests.test_problem1_bundle_doc_mentions_representative_paths_and_commands`
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`
- `cat target/current-l2-guided/problem1-theorem-pilot/pilot-summary.md`
- `cat target/current-l2-guided/problem1-theorem-pilot/pilot-summary.json`

## 7. Evidence / outputs / test results

- targeted unittest は RED で `pilot-summary.md` / manifest keys 未定義を確認した後、GREEN で `Ran 3 tests` / `OK` を確認した。
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  では output dir と `pilot-summary.md` / `pilot-summary.json` path が pretty output に現れた。
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`
  では `pilot_notebook_index_markdown` / `pilot_notebook_index_json` が manifest に入った。
- `pilot-summary.md` / `pilot-summary.json`
  の両方が `target/current-l2-guided/problem1-theorem-pilot/` に生成され、representative `p06` と support pair `p07 / p08` の summary rows を確認した。

## 8. What changed in understanding

- theorem-first external pilot は reserve summary の entry command だけでは少し弱く、output dir 自体に notebook-style index を置くと external pilot としての再利用性が上がる。
- public theorem contract を決めなくても、representative theorem line の emitted artifact loop を external-consumer-adjacent index まで actualize できる。

## 9. Open questions

- `auditable_authority_witness` と `delegated_rng_service` をどちらから先に reopen するか。
- model-check second-line reserve を theorem-first external pilot close 後のどこで差し込むか。
- theorem-first pilot summary index を future mixed gate の final public theorem contract reopen とどう切り分け続けるか。

## 10. Suggested next prompt

- `Package 136 は閉じたので、次は auditable_authority_witness reserve package を narrow actualization してください。`
