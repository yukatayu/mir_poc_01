---
id: arch/01-strata
status: L1-fixed
maturity: draft
depends_on: [arch/readme, root/glossary]
summary: LAB の L0-L5 と S0-S7 と theory の S0-S5 を一枚に統一した層モデル。
open_items: []
---

# 01 — 統一層モデル

LAB には三つの層番号系(グローバル L0–L5、実現 S0–S7、相談メモの意味層)が並立していた。canon は次の一枚に統一する。**意味層(theory の S0–S5)を主番号**とし、実現層はその展開として読む。

| 意味層 | 内容 | 実現層(旧 S0-S7) | 旧グローバル層 |
|---|---|---|---|
| S0 Surface | 普通のソース | — | L1 の表層 |
| S1 Core | elaboration 産物、権限・失敗・効果 | S1 計算核 / S2 効果契約核 | L1 Mir Core |
| S2 Trace | 履歴 DAG、cut、save/load | S4 の一部(観測面) | L1/L2 |
| S3 Verify | checker / model / proof、診断 | S3 検証層 | L1 横断 |
| S4 Projection | per-locus 成果物、通信境界、fabric | S4 runtime/fabric、S5 projection | L2 Mirrorea |
| S5 Domain | World/Avatar/ゲーム、共有空間 | S7 応用 | L3 共有空間 / L5 応用 |
| (S6 Host) | OS・ブラウザ・エンジン・FFI 先 | S0 基盤 / S6 host | L0 基盤 / L4 エンジン |

規則: 上の層の語彙を下の層の primitive にしない(ADR-0001)。下の層の実現詳細(transport、storage、エンジン)を上の層の意味にしない(ADR-0007、ADR-0011)。観測(devtools)は S2/S3 に足場を置き全層を横断する。置換可能性は境界が明示なところで最大になる — その明示が 02 の BND である。
