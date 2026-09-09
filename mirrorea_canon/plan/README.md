---
id: plan/readme
status: L1-fixed
maturity: draft
depends_on: [root/north-star, adr/ADR-0014, adr/ADR-0015, adr/ADR-0026, adr/ADR-0033, adr/ADR-0034, adr/ADR-0035, adr/ADR-0036, adr/ADR-0037, adr/ADR-0038, adr/ADR-0039, adr/ADR-0040, meta/proposal-042, meta/proposal-043, plan/02-operating-model, adr/ADR-0043]
summary: Gate/Phase、ADR-0040で再開したADR-0034 program、三軸/trust/private adapter/two-process runtime acceptance、program外ADR-0014 routeの計画読解。
open_items: []
---

# plan/ — 計画

二軸で進む: **Gate 0–7** は理論の凍結順(何を先に固めるか)、**Phase T0–T2 / I1–I6** は時系列の作業段階(何がいつ動くか)。Gate は plan/00、Phase は plan/01、T0/G0 semantic-assertion profile v3 は plan/04、ADR-0034 programがconsumeするI3 entry contractはplan/05、運用とリスクは plan/02--03。

現在の権限は分けて読む。ADR-0015のM0--M10とADR-0026のSYS-0--SYS-7はclosedで、LAB Plan 247/249はhistory/regression baselineである。PROPOSAL-037 / ADR-0034のI3 bounded programはPlan 250をsole current roadmapとする。ADR-0035--0039はALIGN-1/2とI3-0/1/2、PROPOSAL-046 / ADR-0043はI3-3を受理した。ALIGN-0/1/2とI3-0/1/2/3はcompleted。現在はowner pauseでactive semantic milestoneなし、I3-4はexplicit resumeまでinactiveである。official I3 lifecycleは未entry、program外はADR-0014のreversible L3 route、L2 promotionはtrust anchor未構成のためfail-closedである。

進捗の定義: report の枚数でも決定 ID の増加でもなく、**milestone acceptance、Gate exit criteria、SCN/conformance、正確に分類された proof evidence**である(plan/02)。
