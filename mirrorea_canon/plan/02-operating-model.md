---
id: plan/02-operating-model
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0012, adr/ADR-0014]
summary: 運用規約。owner-reserved boundary、delegated L2/L3 theory research、package 5 種、モラトリアム、KPI、相互レビュー。
open_items: []
---

# 02 — 運用規約

## 役割

- **人間(オーナー)**: L0/L1 の決定、core primitive / external contract、
  SCN/Gate/Phase action、ADR の発効、最終 proof / OBL discharge / public claim
  の承認。月次(以上)の「決める会」で reserved 未決を必ず一つ以上減らす。
- **研究 author agent**: existing LAB lane で可逆な L2/L3 working theory の候補
  比較、反例、文献、Lean transcription / conditional lemma、実験、暫定採択、撤回を
  行う。ADR-0014 の exact editable target があるときだけ canon update を提案する。
  reserved boundary は選ばない。
- **独立 reviewer agent**: final rebased authority/evidence cut、registry membership、
  semantic delta、evidence、non-effects、rollback、canon wording を review する。
  author の authority を拡張しない。
- **canon steward**: integration を直列化し、review 前に target base と evidence
  cut を rebase/freeze し、shared snapshots、validation、commit、push を担当する。
  same change の reviewer を兼ねず、review 後の変化には再 review を要求する。

## Package 5 種(close 条件が異なる)

| 種別 | 産物 | close 条件 |
|---|---|---|
| design-memo | owner-reserved decision request (`meta/proposals/`) | owner disposition または explicit defer |
| delegated-theory-research | target-ID scoped L2/L3 candidate | authority cut、positive/negative evidence、non-effects、rollback、LAB lifecycle record。canon update は ADR-0014 editable-target row と final-delta review が追加で必要 |
| calculus-experiment | 理論の変種検討・反例 | 結論と falsifier を LAB 台帳へ反映。current state を変えるなら delegated-theory-research 条件も満たす |
| proof | OBL の statement/証明 | Lean artifact は LAB evidence。`theory/11` の全 status / identity movement は owner action |
| spike | 使い捨て実装実験 | 学びの 1 頁メモ。main に残すコードは既存の許可済み lane にある delegated-theory-research artifact に限る |

## Delegated L2/L3 research lifecycle

LAB lifecycle は `proposed -> compared -> provisionally-selected ->`
`superseded | falsified | escalated`。canon document status と混同しない。ADR-0014
editable-target row が無い candidate は、LAB lifecycle だけを進み canon を変えない。

開始時に、governing ADR、target IDs、pinned canon/LAB revision、allowed operation、
forbidden surface、result class、expected falsifier、rollback、integration steward を
固定する。canon update を求める場合、steward は review 前に intended integration
base、cited/affected canon blobs、proposed diff、artifact digests、rollback diff を
freeze する。これらのいずれかが変われば review は失効する。

LAB candidate の `L3-open -> L2-working` provisional selection には、少なくとも status quo と
代替/falsifier、positive/negative evidence、reproducible command / tool version /
retained artifact hash、dependent IDs / frozen SCN impact read、non-effects、
independent review が必要である。canon working statement は ADR-0014 の editable
target row がある場合だけ、scope、assumptions、LAB evidence cut、review、rollback
trigger を保持する。

再現可能な falsifier は依存作業と新しい integration を LAB で即時停止できる。
canon L2-to-L3 demotion は rebase/freeze と independent review 後に行う。replacement
L2 は再 review を要する。history は消さず LAB に forward record する。

次の場合は `escalated` として owner/canon action を要求する: L0/L1 の再解釈、
primitive / external contract / SCN / conformance / Gate / Phase / any theory/11 / final proof の変更、
new lane/helper/schema/CI/Make target、又は authority ambiguity / settled invariant
conflict。

## モラトリアムと KPI

MirCore v0(T1 exit)まで、新しい evidence lane・新規 helper 群・新規 report 系列の追加は凍結する。既存の許可済み lane における scoped research artifact は、この凍結を解除しない。KPI は (a) Gate exit criteria の充足数、(b) SCN 通過数(実装期)、(c) review 済みで rollback 可能な L3→L2 working selection の質である。report 枚数・決定 ID の増加は進捗ではない。

## レビューと外部性

L0/L1 と reserved boundary の規範変更は必ず二重レビュー(起案したモデルと別系統のモデル、可能なら人間)。delegated L2/L3 working-state update は author と異なる independent reviewer を必須とする。四半期ごとに canon 全体の recut(矛盾・肥大の刈り込み)。T2 前後にワークショップ論文 1 本を蒸留の強制装置として書く。

## 記録の置き場

作業履歴・候補・実験ログ・生成物は LAB へ。canon に入るのは規範・ADR-0014 の
exact row に許された current working theory・計画・規約のみ。progress 系ダッシュ
ボードは鏡であって記憶にしない。
