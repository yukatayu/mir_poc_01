# Report 0097 — review current l2 bundle first detached payload context split

- Date: 2026-04-03T04:05:00Z
- Author / agent: Codex
- Scope: report 0096 と、その companion spec / plan mirror に対する narrow-scope review の記録
- Decision levels touched: review only

## 1. Objective

bundle-first detached exporter の payload/context split が、既存の detached artifact schema、exporter entry judgment、helper boundary、traceability mirror と矛盾していないかを 1 回の reviewer で確認し、その結果を保存する。

## 2. Inputs consulted

- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0096-current-l2-bundle-first-detached-payload-context-split.md`
- `crates/mir-semantics/src/harness.rs`

## 3. Actions taken

1. reviewer を 1 回だけ起動し、completion まで待った。
2. 指摘を確認した。
3. `plan/90-source-traceability.md` の parser-free PoC stack report chain に `0096` を追記した。

## 4. Files changed

- `plan/90-source-traceability.md`
- `docs/reports/0097-review-current-l2-bundle-first-detached-payload-context-split.md`

## 5. Commands run and exact outputs

reviewer completion の要旨:

```text
Finding 1:
- plan/90 の parser-free PoC stack report-chain section に report 0096 が抜けている

No other findings.
The substantive judgment is source-backed and remains narrow.
```

## 6. Evidence / findings

- reviewer finding は 1 件だけで、内容は traceability mirror の report-chain 漏れだった。
- judgment 自体については no findings で、次の整理は source-backed だと確認された。
  - payload core は `RunReport` 由来
  - `fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` は `bundle_context`
  - `steps_executed` は detached non-core
  - `host_plan_coverage_failure` は aggregate-only

## 7. Changes in understanding

- 이번 task では設計判断そのものより、`plan/90` の report-chain mirror を最後まで揃えることが重要だった。

## 8. Open questions

- actual exporter API の形
- detached artifact 保存先と path policy
- bundle-level failure artifact の shape
- `host_plan_coverage_failure` の typed bundle-level carrier をいつ導入するか

仕様本文コミット hash:

- `13cc1f0` `bundle-first detached artifact の split を整理する`

review report 自身の commit hash は self-reference の都合でこの本文には固定しない。

## 9. Suggested next prompt

current L2 parser-free PoC を前提に、bundle-first detached artifact で aggregate-only に残した `host_plan_coverage_failure` を、将来どの layer で typed carrier に切り出すのが自然かを narrow scope で比較してください。`RunReport` payload core は変えず、bundle failure artifact と batch aggregate export の境界だけを扱ってください。
