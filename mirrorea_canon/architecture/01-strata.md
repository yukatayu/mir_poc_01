---
id: arch/01-strata
status: L1-fixed
maturity: draft
depends_on: [arch/readme, root/glossary]
summary: current semantic S0--S6とlegacy LAB layer labelsを区別する意味層モデル。
open_items: []
---

# 01 — Semantic strata

LAB にはグローバル L0–L5、実現 S0–S7、theory S0–S5が並立していた。
current Canonのsemantic axisは次の **S0–S6** である。旧実現 `S0–S7` は
historical mappingであり、current semantic stratum又はmaturity scaleではない。
project/product responsibilityは`arch/06-project-product-layers`の`PL-0--PL-6`、
lifecycleは`plan/01-phases`の`T0--T2` / `I1--I6`として別に読む。

| Semantic stratum | 内容 | 旧実現層(LAB S0-S7) | 旧グローバル層 |
|---|---|---|---|
| S0 Surface | 普通のソース | — | L1 の表層 |
| S1 Core | elaboration 産物、権限・失敗・効果 | S1 計算核 / S2 効果契約核 | L1 Mir Core |
| S2 Trace | 履歴 DAG、cut、save/load | S4 の一部(観測面) | L1/L2 |
| S3 Verify | checker / model / proof、診断 | S3 検証層 | L1 横断 |
| S4 Projection | per-locus 成果物、通信境界、fabric | S4 runtime/fabric、S5 projection | L2 Mirrorea |
| S5 Domain | World/Avatar/ゲーム、共有空間 | S7 応用 | L3 共有空間 / L5 応用 |
| S6 Host | OS・ブラウザ・エンジン・FFI先との非authoritativeな実現境界 | S0 基盤 / S6 host | L0 基盤 / L4 エンジン |

formal theory chaptersが主にS0–S5を扱うことは、S6をoptionalにせず、Hostを
domain semanticsへ昇格させない。旧`S7 Application`はcurrent stratumではなく、
application/domain responsibilityはS5とPL-5/PL-6の交差として読む。

規則: 上の層の語彙を下の層の primitive にしない(ADR-0001)。下の層の実現詳細(transport、storage、エンジン)を上の層の意味にしない(ADR-0007、ADR-0011)。観測(devtools)は S2/S3 に足場を置き全層を横断する。置換可能性は境界が明示なところで最大になる — その明示が 02 の BND である。S番号、PL番号、phase番号から別軸のowner又は成熟度を推論しない。
