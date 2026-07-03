---
id: plan/01-phases
status: L1-fixed
maturity: draft
depends_on: [plan/00-gates, spec/06-conformance]
summary: 実装フェーズ T0-T2 / I1-I6。各段階の正確なゴール、何が動き、どの程度実用的か、非宣言。
open_items: [OPEN-032]
---

# 01 — フェーズ計画

**唯一の実装状態の正本。** 他のどのファイルの存在も実装を意味しない。現在位置: **T0**。

## 理論フェーズ(実装凍結。例外は使い捨て spike のみ、main 不合流)

| Phase | ゴール(exit) | 動くもの / 実用性 |
|---|---|---|
| T0 語彙と決定 | G0 exit。canon 発効、LAB 格下げ、旧語彙注記 | 何も動かない。判断の基準器が立つ |
| T1 計算体系 | G1 exit + G2/G3 の statement 群。SCN 期待の最終化 | 紙と Lean statement。以後の全実装の仕様が確定 |
| T2 骨格証明 | OBL-020/021/002 の証明骨格、G5 statement 群 | Lean 上で核が回る。理論の破綻はここまでに露見する |

## 実装フェーズ

| Phase | ゴール(exit criteria) | 動くもの | 実用性 | 非宣言 |
|---|---|---|---|---|
| I1 参照実装 | mir-parse/check/elab/run が C-static+C-runtime 10/10、carrier 凍結(arch/04) | 単一プロセスで全 SCN | 教育・検証用。言語に触れる | 性能・分散・永続 |
| I2 多 locus | プロセス内 multi-place、生成通信の実 dispatch、devtools 最小 panel | ローカル toy world | 一人で遊べる箱庭 | 実網・耐障害 |
| I3 実 transport | 2 OS プロセス+実 socket、C-distributed(SCN-01/02/03/06) | LAN で双六が二人で遊べる | 最初の「本物が動く」点 | WAN・セキュリティ強度 |
| I4 永続と patch | save/load(local durable)、ライブ patch(SCN-09 を実セッションで) | 落として上げ直せる world | 継続世界の試作 | 分散 durable(R3/R4) |
| I5 射影と View | ブラウザ client への projection、View FFI(pose 契約)、viewer devtools | 人に見せられる仮想空間デモ | デモ可能な α | 最終 ABI・複数エンジン |
| I6 分散永続と連合 | R3/R4、複数サーバ、federation 入口、限定公開 | 招待制の常設小世界 | 限定公開 α | 一般公開・スケール保証 |

各 Phase の exit は mir-conform の JSON 合否+人間の受理で成立。**Phase を跨ぐ最適化の先取りは禁止**(BND-006 の意味保存を先に)。OPEN-032: I3 の transport 選定(候補: QUIC/WebTransport 系)は I2 exit 時に ADR で決定。
