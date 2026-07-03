---
id: spec/readme
status: L1-fixed
maturity: draft
depends_on: [theory/readme, adr/ADR-0008, adr/ADR-0009]
summary: Mir Report(仕様の蒸留)の読み方。theory との優先関係、章構成。
open_items: []
---

# spec/ — Mir Report

言語・システム仕様の正本。LAB の specs 44 本のうち利用者・実装者が必要とする面をここに蒸留する。**theory/ と矛盾した場合は theory/ が勝ち**、本 spec を修正する。

章: 01 字句とモジュール → 02 Surface 文法(EBNF) → 03 静的意味 → 04 Core IR → 05 runtime 意味(参照実装の観測可能挙動) → 06 適合性 → 07 診断形式。

ここから推論してはいけないこと: 文法の存在は parser 実装の存在を意味しない(plan/01)。`chain` 表記は OPEN-005(最も調整が入りやすい箇所)。
