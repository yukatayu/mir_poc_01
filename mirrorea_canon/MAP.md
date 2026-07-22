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
| WRK-0003 | L3-open | OBL-021 LAB statement draft の total/unique projection でも残る Result extensionality gap を既存 Lean lane の countermodel で検査。`working/WRK-0003-obl021-projection-extensionality.md` |
| WRK-0004 | L3-open | OBL-021 LAB statement draft が well-scoped input の outcome 存在を要求するかを既存 Lean lane の no-outcome countermodel で検査。`working/WRK-0004-obl021-outcome-totality.md` |
| WRK-0005 | L3-open | OBL-021 LAB statement draft と明示 outcome-totality 前提が実験用 SameOutcome relation を導くかを既存 Lean lane の conditional lemma で検査。`working/WRK-0005-obl021-conditional-outcome-relation.md` |
| WRK-0006 | L3-open | OBL-020 LAB statement draft の global preservation と familywise preservation の論理的接続を既存 Lean lane で検査。coverage は experiment-local 条件に留める。`working/WRK-0006-obl020-familywise-global-boundary.md` |
| WRK-0007 | L3-open | OBL-001 LAB statement draft が実験用 Result 内の write を `GeneratedWrite` で尽くすことを要求するかを検査。Core 表現・OBL status は選ばない。`working/WRK-0007-obl001-result-write-coverage.md` |
| WRK-0008 | L3-open | current-L2 runtime try/cut formal hook が same-Place の atomic_cut frontier を根拠として区別するかを既存レーンで監査。OBL-027、carrier、helper/schema は変更しない。`working/WRK-0008-obl027-formal-hook-attribution.md` |
| WRK-0009 | L3-open | current-L2 static e5 route と Lean proof-skeleton の review-unit / emitted-stub identity tuple が literal に整合するかを既存レーンで監査。theorem 意味、OBL、carrier、helper/schema は変更しない。`working/WRK-0009-current-l2-e5-skeleton-identity.md` |
| WRK-0010 | L3-open | current-L2 static-gate decision payload が static formal-hook artifact に literal または明示的 lossless reference として残るかを既存レーンで監査。診断意味、defect、carrier、helper/schema は変更しない。`working/WRK-0010-static-formal-hook-decision-attribution.md` |
| WRK-0011 | L3-open | current-L2 e21/e22 source route が exact final store を直接アサートするか、fixture/direct-evaluator lane に限定されるかを literal に監査。状態意味、同値性、defect、coverage 要求、carrier は選ばない。`working/WRK-0011-current-l2-final-store-directness.md` |
| WRK-0012 | L3-open, frozen | P-COMP-03 の固定一正例・一負例を既存 Product Alpha `world` package の direct carrier で検証した L3 record。二つの sidecar は観測されたが、番号付き結果 artifact の登録には許可外の validator/source-hierarchy 変更が必要となり凍結。helper、schema、runtime、CLI、public carrier、全 row coverage は変更・主張しない。`working/WRK-0012-pcomp03-direct-carrier.md` |
| WRK-0013 | L3-open, not-promoted | frozen WRK-0012 の二つの direct-world sidecar を入力としてのみ pin し、登録後の fresh execution と既存 unnumbered plan artifact 経路を独立に再現・保持した L3 record。sidecar、validator、helper、schema、runtime、CLI、public carrier、全 row coverage は変更・主張しない。`working/WRK-0013-pcomp03-retention-reproduction.md` |
| WRK-0014 | L3-open, not-promoted | Canon carrier を具体化せず、同一 carrier 上の relation inclusion が safety/coherence と outcome existence に必要とする向きを既存 Lean lane で検査する登録済み L3 record。OBL、Core/Config/Step/WellFormed、outcome representation、proof interface は変更しない。`working/WRK-0014-same-carrier-variance.md` |
| WRK-0016 | L3-open, frozen | exact current-L2 Lean foundation の二 constructor `Capability` に限り、任意 `CaptureSet` 間の `captureSubset` をソース可視トップレベルの非 instance `Decidable` 値として構成できるかを検査した L3 record。Lean の `theorem` は値をその形で保持できず、禁止済みの data-valued declaration が必要になるためこの route を凍結。構成的 undecidability、OBL-003、Line-1、generic carrier、global instance、checker、Canon carrier/API は変更・主張しない。`working/WRK-0016-local-predicate-constructivity.md` |
| WRK-0017 | L3-open, frozen | exact current-L2 Lean foundation の二 constructor `Capability` に限る proposition-valued `captureSubset` excluded-middle route。局所の一時 theorem は公理なしで通ったが、登録済み opaque generic-domain control は `Classical.choice` を伴って通ったため、明示 finite interface なしに generic decision を拒むという停止条件を満たし frozen。source は復元済みで、構成性、OBL-003、Line-1、checker、generic carrier、Canon carrier/APIは変更・主張しない。`working/WRK-0017-local-predicate-proposition-decidability.md` |
| WRK-0018 | L3-open, not-promoted | exact IFC Lean foundation 内で、low-agreeing toy configurations の model export equality と high-dependent telemetry toy の固定 adverse pair を検査する登録済み L3 record。telemetry semantics、label lattice、export ABI、THM-005/OBL、BND-008、grammar、SCN、runtime は変更・主張しない。`working/WRK-0018-thm005-telemetry-effect-boundary.md` |

## 「ここから推論してはいけないこと」(canon 全体)

- 図・例・シナリオの語彙(`World`, `Player` 等)は S5 のドメイン語彙であり、core primitive ではない(ADR-0001)。
- ファイルが存在すること自体は、その内容が実装済みであることを意味しない。実装状態は plan/01 のみが語る。
- LAB(旧 repo)の記述は evidence であり規範ではない(ADR-0012)。
- `working/` の記録は settled theory ではない。既存の theory/spec/scenario/plan/ledger を変更せず、ADR-0014 の範囲だけを示す。
