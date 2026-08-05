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
| ADR-0008 | pre-M6 Surface profile の保持(S{ } 系、== 等価、chain 宣言) | L1 |
| ADR-0009 | .mir ソースが意味の正本。package 成果物は生成物 | L1 |
| ADR-0010 | 検証は三線分離 + 明示的義務 carrier。Lean-first | L1 |
| ADR-0011 | stdio は core builtin にしない。外界は typed adapter 境界 | L1 |
| ADR-0012 | canon が正本、旧 repo は LAB。改定は一方向手続き | 運用 L0 |
| ADR-0013 | T0/G0 v1/v2 governance history と G0 substantive evidence acceptance。current profileはADR-0017へ移行 | L1 |
| ADR-0014 | standing L3 pre-registration と review-gated L2 working theory research を委任。reserved boundary は owner に留保 | 運用 L0 |
| ADR-0015 | Mir Theory v0 / I1+ の owner-approved program に evidence-gated bounded autonomy と milestone 直列 integration を委任 | 運用 L0 |
| ADR-0016 | Design Constitution を採用し、M1 の RMW/fallback/Surface alignment を記録 | L1 |
| ADR-0017 | T0/G0 semantic-assertion profile v3を採用し、reproduced pass digestからG0 exit/T1 entryを受理 | L1 |
| ADR-0018 | M3のfinite evaluation/materialization calculus、owner RMW、explicit receipt boundaryを採用 | L1 |
| ADR-0019 | M4のowner-held maintained relation、late projection、semantic/presentation fallback分離を採用 | L1 |
| ADR-0020 | M5のconcrete shared formal modelを採用し、M6--M8のsemantic sourceを固定 | L1 |
| ADR-0021 | M6 bounded ordinary Surface と M5 fragment への分類を採用 | L1 |
| ADR-0022 | M6 fixed input の finite checked elaboration と typed residual obligation を採用 | L1 |
| ADR-0023 | M7 checked artifactからM8 runtimeへのfinite admission・lowering・state boundaryを採用 | L1 |
| ADR-0024 | M8 deferred residualのsource-bound M9 auth/verification seamを採用 | L1 |
| ADR-0025 | 凍結SCN-01..10のordinary-source有限M10 conformance profileとM6/M7 direct-consumer seamを採用 | L1 |
