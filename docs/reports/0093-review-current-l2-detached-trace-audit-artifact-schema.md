# Report 0093 — review current l2 detached trace audit artifact schema

- Date: 2026-04-03T01:57:11.335906Z
- Author / agent: Codex
- Scope: `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`plan/07-parser-free-poc-stack.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`docs/reports/0092-current-l2-detached-trace-audit-artifact-schema.md`
- Decision levels touched: review only

## 1. Objective

current L2 detached trace / audit artifact schema task について、detached artifact の core / non-core / human-facing explanation 分離が既存 machine-check surface と helper boundary を壊していないか確認する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md`
- `docs/reports/0092-current-l2-detached-trace-audit-artifact-schema.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`

## 3. Actions taken

1. detached artifact schema の spec / plan / report 差分を読み、current machine-check surface の整理と照合した。
2. `ExpectedTraceAudit`、`TraceAuditSink`、`RunReport`、`BatchRunSummary` の carrier と bundle / batch exact compare 面を確認した。
3. reviewer finding を 2 件抽出した。
4. main task 側で finding を反映した後、残件がないことを確認した。

## 4. Files changed

- 新規: `docs/reports/0093-review-current-l2-detached-trace-audit-artifact-schema.md`

## 5. Commands run and exact outputs

review は subagent で実施したため、shell command の追加実行はない。

## 6. Evidence / findings

### finding 1

`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` では `relative fixture path / host-plan path` を detached non-core と分類していたが、bundle schema sketch に `fixture_path` を top-level で置いていた。  
これは exact-compare core と detached non-core の cut をぼかすため、`detached_noncore` へ移動する修正が必要だった。

### finding 2

`plan/07-parser-free-poc-stack.md` では detached non-core の列挙に `host-plan explanation` が入っていたが、後半の docs-only boundary summary にそれが抜けていた。  
spec を正本とする以上、mirror drift を避けるため summary 側にも追記が必要だった。

### post-fix status

上記 2 件は反映済みで、残りは no findings である。特に次は整合している。

- `must_explain` は machine-check に上げていない
- `host_plan_coverage_failure` は batch-layer core としつつ typed carrier は未決に留めている
- `steps_executed` は detached non-core であり、exact-compare core にはしていない
- richer host interface は後段 task に残している

## 7. Changes in understanding

review を通すと、detached artifact task の主要リスクは semantics 追加ではなく、core / non-core の cut を schema sketch と plan mirror で一貫させることだと再確認できた。

## 8. Open questions

- production schema version をどう切るか
- typed `host_plan_coverage_failure` carrier をいつ導入するか
- detached artifact exporter prototype をどの layer から始めるか

## 9. Suggested next prompt

`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` を前提に、current L2 detached artifact exporter の docs-aligned prototype をどの layer から切り始めるべきかを比較してください。`RunReport` 直列 export、bundle export、batch export の 3 案を narrow scope で比較し、richer host interface には進まないでください。`
