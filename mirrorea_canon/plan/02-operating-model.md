---
id: plan/02-operating-model
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0012, meta/agent-instructions]
summary: 運用規約。エージェント役割の反転、package 4 種、モラトリアム、KPI、決定の周期、相互レビュー。
open_items: []
---

# 02 — 運用規約

## 役割

- **人間(オーナー)**: L0/L1 の決定、Gate exit の承認、ADR の発効。月次(以上)の「決める会」で未決を必ず一つ以上減らす。
- **AI エージェント**: (i) 反例探索・敵対的レビュー、(ii) 決定済み体系の Lean への写経と補題探し、(iii) 文献調査と対比表更新、(iv) 決定の帰結の機械的展開(サンプル再表現・整合検証・INDEX 再生成)。**ロードマップの自律推進はしない。** 詳細権限は meta/agent-instructions。

## Package 4 種(close 条件が異なる)

| 種別 | 産物 | close 条件 |
|---|---|---|
| design-memo | 提案文書(meta/proposals/) | 人間の採否決定 |
| calculus-experiment | 理論の変種検討・反例 | 結論の台帳反映(採否問わず) |
| proof | OBL の statement/証明 | theory/11 の状態更新 |
| spike | 使い捨て実装実験 | 学びの 1 頁メモ。**コードは main に入れない** |

## モラトリアムと KPI

MirCore v0(T1 exit)まで、新しい evidence lane・新規 helper 群・新規 report 系列の追加は凍結。KPI は (a) Gate exit criteria の充足数、(b) SCN 通過数(実装期)、(c) L3→L2/L1 昇格数(OPEN の解消数)。report 枚数・決定 ID の増加は進捗ではない。

## レビューと外部性

規範変更は必ず二重レビュー(起案したモデルと別系統のモデル、可能なら人間)。四半期ごとに canon 全体の recut(矛盾・肥大の刈り込み)。T2 前後にワークショップ論文 1 本を蒸留の強制装置として書く。

## 記録の置き場

作業履歴・実験ログ・生成物は LAB へ。canon に入るのは規範・理論・計画・規約のみ。progress 系ダッシュボードは鏡であって記憶にしない。
