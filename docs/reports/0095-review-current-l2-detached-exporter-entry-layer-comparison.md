# Report 0095 — review current l2 detached exporter entry layer comparison

- Date: 2026-04-03T03:45:00Z
- Author / agent: Codex
- Scope: report 0094 とその companion spec / plan mirror に対する narrow-scope review の記録
- Decision levels touched: L2

## 1. Objective

`RunReport` / `BundleRunReport` / `BatchRunSummary` の detached exporter entry comparison が、既存理論、helper boundary、docs mirror の traceability を壊していないかを 1 回の reviewer で確認し、その結果を保存する。

## 2. Inputs consulted

- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0094-current-l2-detached-exporter-entry-layer-comparison.md`
- `crates/mir-semantics/src/harness.rs`

## 3. Actions taken

1. reviewer を 1 回だけ起動し、completion まで待った。
2. 指摘を確認した。
3. `plan/90-source-traceability.md` に `plan/09` の bundle-first exporter judgment の根拠を追記した。

## 4. Files changed

- `plan/90-source-traceability.md`
- `docs/reports/0095-review-current-l2-detached-exporter-entry-layer-comparison.md`

## 5. Commands run and exact outputs

reviewer completion の要旨:

```text
Finding 1 (Medium):
- plan/09 に追加した detached-exporter boundary judgment の source traceability が incomplete
- plan/90 の plan/09 row に specs/examples/17 と docs/reports/0094 を追加すべき

No other findings.
The underlying judgment itself is consistent with specs/examples/16, specs/examples/17,
and current helper boundaries in harness.rs.
```

## 6. Evidence / findings

- reviewer finding は 1 件だけで、内容は traceability 漏れだった。
- judgment 自体には finding がなく、
  - payload core は `RunReport`
  - first exporter entry は `run_bundle` / `BundleRunReport`
  - `BatchRunSummary` は後段
  という整理は source-backed だと確認された。

## 7. Changes in understanding

- detached exporter entry comparison 自体よりも、`plan/09` に入れた新しい判断を `plan/90` へ追跡可能に残すことが重要だと明確になった。

## 8. Open questions

- actual exporter API の形
- detached artifact 保存先と path policy
- `BatchRunSummary` を aggregate export として切る閾値

仕様本文コミット hash:

- `2dc8bf8` `detached exporter の entry 境界を整理する`

review report 自身の commit hash は self-reference の都合でこの本文には固定しない。

## 9. Suggested next prompt

current L2 parser-free PoC を前提に、`run_bundle` を first exporter entry とする detached artifact exporter の docs-only payload/context split をさらに狭く比較し、fixture path・host-plan path・coverage failure を detached non-core にどう配置するかを整理してください。
