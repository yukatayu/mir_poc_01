---
id: spec/readme
status: L1-fixed
maturity: draft
depends_on: [theory/readme, adr/ADR-0008, adr/ADR-0009, adr/ADR-0021, adr/ADR-0022, adr/ADR-0023, adr/ADR-0024]
summary: Mir Report(仕様の蒸留)の読み方。theory との優先関係、章構成。
open_items: []
---

# spec/ — Mir Report

言語・システム仕様の正本。LAB の specs 44 本のうち利用者・実装者が必要とする面をここに蒸留する。**theory/ と矛盾した場合は theory/ が勝ち**、本 spec を修正する。

章: 01 M6 字句とモジュール → 02 M6 bounded Surface grammar →
03 M6 静的分類 → 04 M6 CoreTemplate → 08 M7 checked elaboration → 09 M8 deterministic runtime → 10 M9 auth/verification →
05 runtime 意味(参照実装の観測可能挙動) → 06 適合性 → 07 診断形式。

ここから推論してはいけないこと: M6 grammar の存在は parser/checker/runtime
実装の存在を意味しない。ADR-0021 の M6 profile は M7 の bounded reference input
であり final public grammar ではない。ADR-0008 の `chain` profile は retained
historical compatibility evidence で、current Surface syntax ではない。
