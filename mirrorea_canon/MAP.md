---
id: root/map
status: L1-fixed
maturity: draft
depends_on: [root/readme]
summary: 知識マップ 1 枚。全域の依存図、3 種の読み筋、ID 体系。
open_items: []
---

# MAP — 知識マップ

## 全域の依存図

```text
NORTH-STAR (軸)
   │
   ▼
adr/ (決定 ADR-0001..0014) ◄──── GLOSSARY (概念 CON-###)
   │
   ├──── working/ (WRK-####: 可逆な L2/L3 research annex、既存正本は read-only)
   │
   ▼
theory/ (MirCore v0)                     mental-model/ (表と裏の対応)
  00 見取り図                                 ▲
  01 計算体系 ──► 02 型/効果/失敗              │ 03 が theory/10 と対応
  01 ──► 03 elaboration ──► THM-001           │
  01 ──► 04 順序と cut ──► 05 権限            │
  01 ──► 06 存在と fallback                   │
  01 ──► 07 観測  08 patch  09 二層時間  10 診断
  11 定理・義務台帳(THM/OBL)  12 文献
   │
   ▼
spec/ (Mir Report: 文法・静的意味・Core IR・runtime・適合・診断)
   │                                    architecture/ (層 S0-S7、
   ▼                                      契約 BND-###、toolchain、carrier)
scenarios/ (SCN-01..10 = 適合性の凍結基準)
   │
   ▼
plan/ (Gate 0-7 → Phase T0-T2, I1-I6)   meta/ (規約・正本関係・agent 規約)
```

## 3 種の読み筋

- **理論筋**(体系を理解・拡張する): NORTH-STAR → adr → theory/00 → 01 → (関心の章) → 11 → scenarios。
- **実装筋**(toolchain を作る): spec/02..07 → architecture/03..04 → scenarios → plan/01 の該当 Phase。
- **運用筋**(進め方を知る): plan/00..03 → meta/agent-instructions → adr/ADR-0012 → adr/ADR-0014。

## ID 体系

| 接頭辞 | 意味 | 台帳 |
|---|---|---|
| ADR-#### | 決定 | adr/ |
| CON-### | 概念 | GLOSSARY.md |
| THM-### | 定理(statement) | theory/11 |
| OBL-### | 証明・検証義務 | theory/11 |
| SCN-## | 適合性シナリオ | scenarios/ |
| BND-### | 層間契約 | architecture/02 |
| GATE-# / PHASE-x | 計画単位 | plan/00, 01 |
| OPEN-### | 未決 | 各ファイル open_items と INDEX.json |
| WRK-#### | 可逆な research working proposition | working/ |
| E-XXXX-### | 診断 ID | spec/07 |

相互参照は必ず ID で行う。`INDEX.json` に全 id → path → status → depends_on の索引がある(`meta/build-index.py` で再生成)。

## Current working records

| ID | Status | Bounded question |
|---|---|---|
| WRK-0001 | L3-open | theory/02 の有限 index 許容範囲と helper-local Lean 正例・拒否例の再現。`working/WRK-0001-finite-index-boundaries.md` |
| WRK-0002 | L3-open | OBL-021 LAB statement draft の projection vacuity を既存 Lean lane の countermodel で検査。`working/WRK-0002-obl021-projection-vacuity.md` |

## 「ここから推論してはいけないこと」(canon 全体)

- 図・例・シナリオの語彙(`World`, `Player` 等)は S5 のドメイン語彙であり、core primitive ではない(ADR-0001)。
- ファイルが存在すること自体は、その内容が実装済みであることを意味しない。実装状態は plan/01 のみが語る。
- LAB(旧 repo)の記述は evidence であり規範ではない(ADR-0012)。
- `working/` の記録は settled theory ではない。既存の theory/spec/scenario/plan/ledger を変更せず、ADR-0014 の範囲だけを示す。
