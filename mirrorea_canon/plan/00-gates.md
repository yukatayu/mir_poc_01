---
id: plan/00-gates
status: L1-fixed
maturity: draft
depends_on: [theory/11-metatheory-ledger, scenarios/readme]
summary: 理論凍結ゲート G0-G7。各 exit criteria と「宣言してはならないこと」。
open_items: []
---

# 00 — 理論凍結ゲート

各 Gate は順不同に着手してよいが、**exit は原則この順**(後段は前段に依存)。exit は人間の決定+ADR/台帳更新で成立する。

| Gate | 目標 | Exit criteria | 宣言してはならないこと |
|---|---|---|---|
| G0 軸と語彙 | 軸/非軸、core/library 語彙、正本関係 | ADR-0001/0002/0005/0009/0012 発効、GLOSSARY 整備、LAB 格下げ完了 | 公開製品方針の確定 |
| G1 普通の代入 | 代入の elaboration 理論 | theory/01/03 が SCN-01/02 を完全に説明、OBL-001 (Lean statement) 完了、OBL-020/021 完了 | 分散 runtime の完成 |
| G2 存在と fallback | 存在 DAG・lease・単調劣化 | theory/06 が SCN-08 を説明、OBL-005..008 statement、chain 表記の確定(OPEN-005 解消) | 完全な依存寿命理論 |
| G3 権限 | grant 系譜・admission | theory/05 が SCN-03/04 を説明、OBL-015 statement | 本番 identity/認証スタック |
| G4 効果と観測 | typed effect・redaction・retention | theory/07 が SCN-07 を説明、OBL-017 statement、label 語彙初版 | 最終 viewer/telemetry ABI |
| G5 cut と保存 | consistent cut・save/load・Z-cycle | theory/04 が SCN-10 を説明、OBL-009..014 statement | R3/R4 分散永続の完成 |
| G6 射影 | 意味保存射影・通信境界導出 | BND-006 の保存リスト確定、SCN-01/02 の射影期待の明文化 | 最適配置・最終 codegen/ABI |
| G7 hot-plug | patch pipeline・frontier 束縛 | theory/08 が SCN-09 を説明、OBL-019 statement | 最終 hot-plug ABI・移行エンジン |

## Current G0 status

G0-D3, G0 exit, and T1 entry were accepted once by the M2 record in
`plan/04-t0-g0-semantic-assertion-profile` and ADR-0017, using the reproduced
v3 pass artifact `LAB:plan/248`. This is phase-governance acceptance only; no
other gate or implementation/conformance/proof claim is implied.

横断規則: どの Gate も、対応する SCN の期待を変えるなら ADR を要する。proof(OBL の discharge)は Gate exit の必須条件ではない(statement 化まで)が、PHASE-T2 の exit には主要 proof の骨格が要る(plan/01)。
