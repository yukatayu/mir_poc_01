---
id: arch/readme
status: L1-fixed
maturity: draft
depends_on: [theory/00-overview]
summary: semantic strata、PL責任層、層間契約、toolchain、carrier、衛星系、Browser/Host trust boundaryの読み方。
open_items: []
---

# architecture/ — 階層と契約

01 semantic strata S0--S6 → 02 層間契約 BND-001..016 → 03 toolchain の責務 →
04 runtime carrier 正本 → 05 衛星系(PrismCascade / TEWP) → 06 separate
project/product responsibility layers PL-0--PL-6 と三軸map → 07 Browser/Host
package/View/provider/raw FFI/resource trust edge → 08 共通security invariant。

ここから推論してはいけないこと: semantic S番号、project/product PL番号、lifecycle
phase番号は同一軸でも一対一対応でもない。契約や carrier の存在は実装の存在を
意味しない。carrier のフィールド名は L2(凍結は PHASE-I1 出口)。
