---
id: meta/agent-instructions
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012, adr/ADR-0014, plan/02-operating-model]
summary: AI エージェントの読込プロトコルと権限境界。review-gated L2/L3 working-theory route、reserved boundary、衝突時の挙動。
open_items: []
---

# agent-instructions — AI エージェント規約

## 読込プロトコル

1. `README.md` → `MAP.md` を読む。2. タスクに応じた読み筋(MAP の三筋)で、対象ディレクトリの `README.md` → 必要ファイルを `depends_on` 順に読む。3. 全域走査はしない(INDEX.json で所在解決)。4. LAB を読むのは evidence 参照が必要なときだけ。

## できること

- design-memo(提案)の起票(`meta/proposals/PROPOSAL-###.md`。本文に: 対象 ID、現状、提案、影響範囲、代替案)。
- 決定済み事項の帰結の機械的展開: サンプル再表現、SCN 整合検証、INDEX 再生成、対比表更新、Lean への写経、反例探索、誤字・参照切れの修正(規範文の意味を変えない範囲)。
- ADR-0014 の delegated route に従う L2/L3 working theory: target ID、pinned
  authority cut、allowed operation、result class、non-effects、falsifier、rollback
  を先に記録し、existing LAB lane で candidate を比較・検証する。ADR-0014 の exact
  editable-target row があり、steward が rebased final diff を freeze し、
  independent review が通った場合に限り、その row の current L3/L2 statement を
  維持できる。
- 「分からない・矛盾がある」の報告(それ自体が価値ある産物)。

## できないこと(禁止)

- ADR effectivity、L0/L1 status / maturity、SCN 期待、conformance classification、
  Gate/Phase exit criteria / lifecycle state の新設・変更・削除。
- core / authority / ownership / effect / failure / judgment primitive、source / public /
  wire / serialization / provider / transport / artifact / compatibility contract の選択・変更。
- `theory/11` の全 entry / status / wording / Lean target / discharge、final proof
  または public completion claim への movement。
- モラトリアム中(T1 exit まで)の新 evidence lane・新 helper 系列の追加。
- canon の主張の overclaim(「完成」「証明済み」等)。LAB 文書の規範扱い。

## 衝突時の挙動

L0/L1、reserved boundary、authority cut、または settled invariant に矛盾する
candidate は **作業を止めて `escalated` bundle を報告する**(勝手に片方を選ばない)。
同じ owner の直接指示が canon 改定を求める場合は、その指示を proposal / ADR /
CHANGELOG / INDEX の通常手続きへ反映してから standing rule にする。delegated route
内の L2/L3 candidate conflict は falsifier / supersession として LAB に記録し、
replacement が必要なら review をやり直す。

## 産物の置き場

owner-reserved 提案 → `meta/proposals/`。candidate・実験・ログ・生成コード → LAB。
canon の delegated write は ADR-0014 editable-target row の exact section のみで、
その根拠と history を LAB に残す。「できること」の範囲を超える canon write はしない。
