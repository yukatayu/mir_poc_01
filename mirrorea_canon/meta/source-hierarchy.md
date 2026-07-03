---
id: meta/source-hierarchy
status: L0-frozen
maturity: draft
depends_on: [adr/ADR-0012]
summary: canon > LAB の正本関係、LAB 格下げの実施手順、引用形式、旧語彙の移行。
open_items: []
---

# source-hierarchy — 正本関係と LAB 格下げ手順

## 関係

`canon(規範) > LAB(evidence)`。衝突は常に canon 勝ち。LAB の specs/plan/report/samples は「歴史的 evidence・実験場」であり、`LAB:specs/39` `LAB:plan/69` `LAB:D-024` の形式で引用する。LAB の decision register のうち理論決定は本 canon(adr/ + theory/)に蒸留済み、process 判断は plan/02 が代替する。

## LAB 側で実施する格下げ手順(T0 の作業)

1. LAB ルートに `CANON.md` を追加: 「規範は mirrorea-canon にある。本 repo は実験場である」と正本宣言し、canon の場所と INDEX を指す。
2. LAB README / AGENTS.md の冒頭に同旨の 3 行を追記(既存本文は改変しない — evidence の凍結保存)。
3. `mir_hilight.html` の KEYWORDS から `world` を除去、または「歴史的表示」と注記。
4. clean-near-end 系サンプルに「旧語彙(world 等)を含む歴史的 suite」の注記 1 行。
5. 以後、LAB への新規追加は spike 産物と evidence のみ(plan/02 の記録の置き場)。

## 移行規則

- canon が LAB の概念を再定義した場合、GLOSSARY の旧語彙対応表に行を足す。
- LAB の evidence を canon の根拠に使うときは「evidence であって規範でない」ことを本文で明示する。
- LAB の削除・改変は原則しない(歴史の保存)。容量問題は archive 移動で解く。
