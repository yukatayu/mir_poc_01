---
id: adr/readme
status: L1-fixed
maturity: draft
depends_on: [root/north-star]
summary: 決定記録の一覧と読み方。LAB の決定台帳(specs/12)との関係。
open_items: []
---

# ADR — 決定記録

各 ADR は「決定・レベル・理由・帰結・却下した代替案・再考条件」を持つ。LAB の specs/12(D-001..D-099+)のうち理論決定(概ね D-001〜D-050)は本 ADR 群と theory/ に蒸留済みであり、process 判断(D-051 以降)は canon に持ち込まない(plan/02 の運用規約が代替する)。

| ID | 決定(1 行) | Level |
|---|---|---|
| ADR-0001 | World / Room / Avatar は core primitive でない | L0 |
| ADR-0002 | Event の三分類(occurrence / request・publication / domain event) | L0 |
| ADR-0003 | 並行書き込み = 単一権威 + owner-directed request + 明示 handoff | L0 |
| ADR-0004 | fallback は単調劣化。復帰は明示的再取得 | L0 |
| ADR-0005 | 名前・場所・transport は権限でない。権限は grant の系譜 | L0 |
| ADR-0006 | patch は eval でない。pipeline + activation cut のみ | L0 |
| ADR-0007 | 順序の source principal は高水準関係族 | L1 |
| ADR-0008 | Surface 文法 v0 の採用(S{ } 系、== 等価、chain 宣言) | L1 |
| ADR-0009 | .mir ソースが意味の正本。package 成果物は生成物 | L1 |
| ADR-0010 | 検証は三線分離 + 明示的義務 carrier。Lean-first | L1 |
| ADR-0011 | stdio は core builtin にしない。外界は typed adapter 境界 | L1 |
| ADR-0012 | canon が正本、旧 repo は LAB。改定は一方向手続き | 運用 L0 |
