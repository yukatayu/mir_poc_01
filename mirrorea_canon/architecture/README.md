---
id: arch/readme
status: L1-fixed
maturity: draft
depends_on: [theory/00-overview, adr/ADR-0039, arch/10-i3-multi-process-runtime]
summary: semantic strata、PL責任層、層間契約、toolchain、carrier、衛星系、Browser/Host trust boundary、I3 private adapter/process runtimeの読み方。
open_items: []
---

# architecture/ — 階層と契約

01 semantic strata S0--S6 → 02 層間契約 BND-001..016 → 03 toolchain の責務 →
04 runtime carrier 正本 → 09 I3-1 private adapter mapping → 10 I3-2 multi-process runtime mapping → 05 衛星系(PrismCascade / TEWP) → 06 separate
project/product responsibility layers PL-0--PL-6 と三軸map → 07 Browser/Host
package/View/provider/raw FFI/resource trust edge → 08 共通security invariant。

ここから推論してはいけないこと: semantic S番号、project/product PL番号、lifecycle
phase番号は同一軸でも一対一対応でもない。契約や carrier の存在は実装の存在を
意味しない。carrier のフィールド名は L2(凍結は PHASE-I1 出口)。
