# Report 0100 — host plan coverage failure bundle failure artifact schema

- Date: 2026-04-03T04:59:53.661264Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC における `host_plan_coverage_failure` typed bundle failure artifact schema の docs-only refinement
- Decision levels touched: L2

## 1. Objective

current L2 parser-free PoC 基盤と 0090 / 0091 / 0092 / 0093 / 0094 / 0095 / 0096 / 0097 / 0098 / 0099 を前提に、`host_plan_coverage_failure` を将来 bundle failure artifact 側の typed carrier に昇格させるなら、その最小 schema を docs-only で切り出す。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0089`〜`0099`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. task-start dirty state を固定した。
2. `RunReport` / `BundleRunReport` / `BatchRunSummary` / `BatchBundleOutcome::Failed { host_plan_coverage_failure }` の current code anchor を再確認した。
3. 既存 docs / plan / report chain で確立済みの judgment を整理した。
4. 次の 3 案を比較した。
   - current aggregate-only を維持
   - bundle failure artifact 側に minimal typed carrier を導入
   - detached non-core / `bundle_context` に薄く混ぜる
5. bundle failure artifact 案の内部で、さらに次を比較した。
   - discriminator だけ
   - discriminator + `bundle_context` 参照
   - discriminator + short failure note
6. docs-only 正本として `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` を追加した。
7. `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md` の OPEN 節を今回の refinement 参照に合わせて補正した。
8. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` を最小更新した。

## 4. Files changed

- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md`

## 5. Commands run and exact outputs

- `git status --short --branch`
  - `## main...origin/main [ahead 3]`
  - `?? diff_investigation_01.md`
  - `?? docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
  - `?? docs/reports/0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
  - `?? 旧資料_参考_ChatGPT_01_69c5e3f6/`
  - `?? 旧資料_参考_ChatGPT_02_Mirrorea_2025/`
- `rg -n "host_plan_coverage_failure|RunReport|BundleRunReport|BatchRunSummary|BatchBundleOutcome::Failed" ...`
  - `RunReport` は `static_verdict`、`entered_evaluation`、`terminal_outcome`、`trace_audit_sink`、`steps_executed` を持つ。
  - `BundleRunReport` は `report: RunReport` だけを持つ。
  - `host_plan_coverage_failure` は `BatchBundleOutcome::Failed` と `BatchRunSummary.host_plan_coverage_failures` に現れる。
- `sed -n ... crates/mir-semantics/src/lib.rs`
  - `RunReport` 定義を確認した。
- `sed -n ... crates/mir-semantics/src/harness.rs`
  - `BundleRunReport`、`BatchBundleOutcome::Failed`、`BatchRunSummary`、`batch_summary_from_discovery()`、`run_bundle()`、`FixtureHostStub::run_fixture()` の anchor を確認した。
- `sed -n ... crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `run_directory_reports_host_plan_coverage_failures_in_summary()` が summary / batch report だけを確認していることを確認した。

## 6. Evidence / findings

- current code では `host_plan_coverage_failure` は payload core でも `TraceAuditSink` metadata でもなく、bundle 実行 failure の batch-layer classification である。
- current docs judgment は次で揃っている。
  - current detached artifact では aggregate-only
  - future typed placement は bundle failure artifact 側
- future typed schema の 3 比較では、**bundle failure artifact の typed core を `failure_kind` discriminator だけに留める**案が最小である。
- `bundle_context` 参照を typed core に混ぜる案は、current docs-only boundary に無い参照機構を先に持ち込むため過剰である。
- short failure note を typed core に混ぜる案は、note が detached non-core / explanation に属するため最小 schema としては重い。
- `payload core` は引き続き `RunReport` 由来の exact-compare core のままとし、success artifact に同名 field を持ち込まない方が helper boundary を保ちやすい。
- `fixture authoring / elaboration` は依然 independent bottleneck であり、この refinement はそれを直接解消しない。

## 7. Changes in understanding

- `host_plan_coverage_failure` の future typed placement は「bundle failure artifact 側」という判断だけでなく、**typed core 自体は discriminator-only に留める**ところまで narrow に具体化できた。
- `bundle_context` を artifact 全体が持つことと、typed carrier 自体が context reference を持つことは分けるべきだと明確になった。
- short coverage note は将来有用だが、typed core ではなく detached non-core の後段 task に回す方が current cut と整合する。

## 8. Open questions

- actual exporter API をどう置くか。
- detached artifact 保存先と path policy をどうするか。
- short failure note を detached non-core としていつ足すか。
- string detection をどの段階で typed detection に置き換えるか。
- richer host interface の typed carrier 化とどこで接続するか。
- `fixture authoring / elaboration` をどの task で独立に改善するか。

仕様本文コミット hash:

- `d20e82e` `host plan coverage failure の typed schema を整理する`

report 自身の commit hash は self-reference の都合で本文には固定しない。

## 9. Suggested next prompt

`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` を前提に、current L2 parser-free PoC で bundle failure artifact の docs-only schema をさらに narrow に進め、`failure_kind: "host-plan-coverage-failure"` を持つ failure artifact と aggregate export の接続面を比較してください。success artifact と payload core は変えず、detached non-core の short coverage note を次点案として扱ってください。
