---
id: plan/readme
status: L1-fixed
maturity: draft
depends_on: [root/north-star, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026, plan/02-operating-model]
summary: Gate/Phase二軸、closed ADR-0015、active ADR-0026、program外ADR-0014の計画読解。
open_items: []
---

# plan/ — 計画

二軸で進む: **Gate 0–7** は理論の凍結順(何を先に固めるか)、**Phase T0–T2 / I1–I6** は時系列の作業段階(何がいつ動くか)。Gate は plan/00、Phase は plan/01、T0/G0 semantic-assertion profile v3 は plan/04、運用とリスクは plan/02--03。

現在の権限は三つを分けて読む。ADR-0015 / PROPOSAL-018 の M0--M10 program は closed で、LAB Plan 247 と accepted cuts は history/regression baseline であって current queue ではない。ADR-0026 / PROPOSAL-029 の SYS-0--SYS-7 Mirrorea I2 Systems Foundation が active bounded program で、sole current LAB roadmap は Plan 249 である。active program 外の research は ADR-0014 の reversible L3 route に従い、L2 promotion は trust anchor 未構成のため fail-closed である。

進捗の定義: report の枚数でも決定 ID の増加でもなく、**milestone acceptance、Gate exit criteria、SCN/conformance、正確に分類された proof evidence**である(plan/02)。
