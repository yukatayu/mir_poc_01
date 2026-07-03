---
id: arch/03-toolchain
status: L2-working
maturity: draft
depends_on: [arch/02-boundary-contracts, spec/04-core-ir]
summary: toolchain 各コンポーネントの責務・入出力・してはならないこと。LAB の前身対応。
open_items: [OPEN-029]
---

# 03 — Toolchain の責務

| Tool | 入力 → 出力 | してはならないこと | LAB 前身 |
|---|---|---|---|
| mir-parse | .mir → AST(+span) | 意味判断、糖の隠れ展開 | mir-ast (surface parser) |
| mir-check | AST → 判定結果+Diagnostics+Obligations | 未討義務の成功扱い | mir-semantics 各 checker |
| mir-elab | AST → Core IR(spec/04) | 隠れ辺、span 欠落、権限創出 | surface_to_core_elaboration |
| mir-run | Core IR+script → 実行+occurrence rows(spec/05 profile) | verdict なし実行、fail-open | mir-runtime |
| mir-project | Core IR+topology Π → per-locus 成果物+通信境界 | 意味を落とす最適化(BND-006) | projection IR 線 |
| mir-transport | Envelope 配送(I3 以降) | auth の吸収(BND-005) | network-docker canary |
| mir-devtools | occurrence rows → panels/export | redaction 破り、H 非由来行 | viewer/telemetry 線 |
| mir-prove | Obligations → Lean project 骨格+状態同期 | 証明の代筆主張 | lean/ 基盤 |
| mir-conform | SCN suite 実行 → 合否(spec/06) | expectation の暗黙緩和 | scripts/ 検証群 |

共通則: 各 tool は自分の Diagnostic family(spec/07)だけを発行し、下流の失敗を握り潰さない。全 tool は `--format json` で carrier(arch/04)を吐けること。

OPEN-029: mir-lsp(エディタ統合)は I5 で検討。CLI 名は仮称であり、LAB の `mirrorea-alpha` 系列を改称して流用してよい。
