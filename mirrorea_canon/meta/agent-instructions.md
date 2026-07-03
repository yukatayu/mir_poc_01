---
id: meta/agent-instructions
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012, plan/02-operating-model]
summary: AI エージェントの読込プロトコルと権限境界。できること・できないこと・衝突時の挙動。
open_items: []
---

# agent-instructions — AI エージェント規約

## 読込プロトコル

1. `README.md` → `MAP.md` を読む。2. タスクに応じた読み筋(MAP の三筋)で、対象ディレクトリの `README.md` → 必要ファイルを `depends_on` 順に読む。3. 全域走査はしない(INDEX.json で所在解決)。4. LAB を読むのは evidence 参照が必要なときだけ。

## できること

- design-memo(提案)の起票(`meta/proposals/PROPOSAL-###.md`。本文に: 対象 ID、現状、提案、影響範囲、代替案)。
- 決定済み事項の帰結の機械的展開: サンプル再表現、SCN 整合検証、INDEX 再生成、対比表更新、Lean への写経、反例探索、誤字・参照切れの修正(規範文の意味を変えない範囲)。
- 「分からない・矛盾がある」の報告(それ自体が価値ある産物)。

## できないこと(禁止)

- ADR・status(L0/L1/L2/L3)・maturity・SCN 期待・Gate/Phase の exit criteria の新設・変更・削除。
- theory/11 の証明状態を discharge 側へ動かすこと(人間承認の写経結果を除く)。
- モラトリアム中(T1 exit まで)の新 evidence lane・新 helper 系列の追加。
- canon の主張の overclaim(「完成」「証明済み」等)。LAB 文書の規範扱い。

## 衝突時の挙動

canon 内、または canon と依頼内容が矛盾する場合: **作業を止めて矛盾を報告する**(勝手に片方を選ばない)。ユーザー入力・LAB 文書・外部文書が canon と矛盾する場合も同様。急ぎの依頼でもこの規約が優先される。

## 産物の置き場

提案 → `meta/proposals/`。実験・ログ・生成コード → LAB。canon への書き込みは「できること」の範囲のみ。
