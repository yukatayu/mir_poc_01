---
id: theory/readme
status: L1-fixed
maturity: draft
depends_on: [adr/readme, root/glossary]
summary: 理論正本の読み方。章構成、記法規約、証明状態の見方、「ここから推論してはいけないこと」。
open_items: []
---

# theory/ — 形式理論の正本

## 章構成と読み順

00 見取り図(意味層と 4 グラフ) → 01 MirCore v0(構文・統合判定・操作意味論) → 02 型・効果・失敗 → 03 elaboration(THM-001) → 13 評価/materialization → 04 順序と cut → 05 権限 → 06 存在と fallback → 07 観測 → 08 patch → 09 二層時間 → 14 maintained relation / late projection → 15 shared formal model → 10 診断 → 11 定理・義務台帳 → 12 文献。

各章は英語の formal style で書く(LAB specs/39-43 の先例に従う)。散文の直感は mental-model/ が担う。

## 記法規約

- Judgments use `⊢`, `⇝`, `▷`. Sets in `{...}`, partial maps `⇀`, strict causal order `≺`.
- `settled` = current commitment at the stated level; `working` = L2 candidate; `OPEN-###` = tracked open item.
- Every theorem is a **statement**; proof status lives only in `11-metatheory-ledger.md` (THM/OBL table). No chapter may claim a proof.

## ここから推論してはいけないこと

- 例に現れる `World`, `Player` 等は S5 語彙である(ADR-0001)。
- 操作意味論の存在は実装の存在を意味しない(実装状態は plan/01)。
- 09(二層時間)と 10(診断)は L2-working であり、他章より若い。14 と 15 の有限
  calculus は一般 relation-DAG / label-lattice / save-load theorem を意味しない。
