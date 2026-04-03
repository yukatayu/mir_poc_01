# Report 0101 — review host plan coverage failure bundle failure artifact schema

- Date: 2026-04-03T05:00:14.990511Z
- Author / agent: Codex
- Scope: `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`Documentation.md`、`specs/00-document-map.md`、`plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md` の narrow-scope review
- Decision levels touched: review only

## 1. Objective

`host_plan_coverage_failure` typed bundle failure artifact schema の docs-only refinement が current code と helper boundary に整合しているか、また未決事項を決まったことのように書いていないかを確認する。

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
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. reviewer を 1 回起動して completion を待つ代わりに、このセッションで利用可能な tool 境界を確認した。
2. reviewer completion を取得できなかったため、task の fallback ルールに従って local review を行った。
3. 次の観点を narrow に点検した。
   - `host_plan_coverage_failure` を payload core に上げていないか
   - success artifact に同名 field を持ち込んでいないか
   - `bundle_context` / detached non-core / explanation の cut を崩していないか
   - `failure_kind` discriminator-only という最小 schema 判断が current code anchor に沿っているか
   - `plan/90-source-traceability.md` に今回の spec / report chain の参照漏れがないか
4. `python3 scripts/validate_docs.py`、`git diff --check`、`cargo test -p mir-semantics` を再実行し、close-out evidence を取り直した。

## 4. Files changed

- `docs/reports/0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `plan/90-source-traceability.md`

## 5. Commands run and exact outputs

- reviewer
  - completion は取得できず、local review fallback を採用した

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 101 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- `cargo test -p mir-semantics`

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 6. Evidence / findings

- `RunReport` は `static_verdict`、`entered_evaluation`、`terminal_outcome`、`trace_audit_sink`、`steps_executed` のみを持ち、`host_plan_coverage_failure` を持たない。
- `BundleRunReport` は `report: RunReport` だけを持つ thin wrapper である。
- `host_plan_coverage_failure` は `batch_summary_from_discovery()` が error text から分類し、
  - `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }`
  - `BatchRunSummary.host_plan_coverage_failures`
  に materialize する。
- tests も `run_directory_reports_host_plan_coverage_failures_in_summary()` で summary / batch report だけを確認している。

finding:

- **no findings**

上の code anchor と照らすと、

- future typed carrier を bundle failure artifact 側に限る判断
- その typed core を `failure_kind` discriminator だけに留める判断
- `bundle_context` / detached non-core / explanation を別 section に維持する判断
- success artifact に同名 field を持ち込まない判断

はいずれも current code と整合していた。

## 7. Changes in understanding

- `host_plan_coverage_failure` の future-narrow-step judgment は、placement だけでなく typed core の粒度まで source-backed に narrow 化できた。
- `bundle_context` を artifact 全体が持つことと、typed carrier 自体が context reference を持つことは別問題として扱うべきだと再確認した。
- traceability には新 spec だけでなく新 report も追加した方が close-out evidence として明快だった。

## 8. Open questions

- actual exporter API
- detached artifact 保存先と path policy
- short failure note を detached non-core としていつ足すか
- string detection をどの段階で typed detection に置き換えるか
- richer host interface typed 化との接続点
- `fixture authoring / elaboration` の独立改善 task

仕様本文コミット hash:

- `d20e82e` `host plan coverage failure の typed schema を整理する`

review report 自身の commit hash は self-reference の都合で本文には固定しない。

## 9. Suggested next prompt

`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` を前提に、current L2 parser-free PoC で bundle failure artifact の typed core を `failure_kind` discriminator-only に保ったまま、aggregate export がそれをどう吸い上げるかを docs-only で比較してください。success artifact と payload core は変えず、short coverage note は detached non-core の次点案として扱ってください。
