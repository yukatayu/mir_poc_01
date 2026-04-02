# Report 0082 — review current l2 fallback compact syntax comparison

- Date: 2026-04-02T13:08:09.933660Z
- Author / agent: Codex
- Scope: `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md` の maintainer-style review
- Decision levels touched: review only。規範意味と companion notation 自体は未変更

## 1. Objective

current L2 parser-free PoC 基盤における fallback / preference chain の compact syntax comparison が、既存 semantics と docs mirror を壊していないかを確認し、report 0081 の不足または不正確さがあれば特定する。

## 2. Inputs consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/04-mir-core.md`
9. `specs/09-invariants-and-constraints.md`
10. `specs/10-open-questions.md`
11. `specs/12-decision-register.md`
12. `specs/examples/01-current-l2-surface-syntax-candidates.md`
13. `specs/examples/04-current-l2-step-semantics.md`
14. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
15. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
16. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
17. `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`

## 3. Actions taken

1. AGENTS.md の順序に従って入口文書と current L2 の基礎仕様を再読した。
2. `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`specs/examples/04-current-l2-step-semantics.md` を基準に、guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` / rollback-cut non-interference を確認した。
3. `git diff` で `specs/examples/01...` と `specs/examples/15...` の未コミット差分を確認し、current 変更が comparison wording だけかを切り分けた。
4. `docs/reports/0081...` を report template と照合し、構造上の不足と内容上の不正確さを確認した。

## 4. Files changed

- `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`

## 5. Commands run and exact outputs

```bash
git status --short
```

```text
 M specs/examples/01-current-l2-surface-syntax-candidates.md
 M specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
?? docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md
```

```bash
git diff -- specs/examples/01-current-l2-surface-syntax-candidates.md
```

```text
@@ -166,7 +166,7 @@
- compact syntax candidate の比較は `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` にまとめる。current L2 では、より短い案を比較対象として残しつつも、この explicit edge-row form を暫定 companion notation として維持する。
+ compact syntax candidate の比較は `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` にまとめる。current L2 では、line-leading `>` ladder のようなより短い案を比較対象として残しつつも、edge-local lineage と request-evaluation 境界を最も誤読させにくいこの explicit edge-row form を暫定 companion notation として維持する。
```

```bash
git diff -- specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
```

```text
主要差分:
- Candidate A / B の直接比較表を追加
- 「主要比較は A/B、C は補助比較対象」と明記
- A を current L2 companion notation として維持する理由に examples mirror 安定性を追記
- B を見送る理由に edge-local lineage の付属情報化リスクを追記
```

```bash
nl -ba docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md | tail -n 20
```

```text
101	## 9. Suggested next prompt
102	
103	`current L2 parser-free PoC 基盤を前提に、fallback / preference chain の compact syntax candidate として line-leading \`>\` ladder をもし examples の一部に試験導入するなら、どの representative examples に限定して A/B 比較を行うのが最も誤読差分を観察しやすいかを整理してください。`
104	## 2. Inputs consulted
105	
106	## 3. Actions taken
107	
108	## 4. Files changed
109	
110	## 5. Commands run and exact outputs
111	
112	## 6. Evidence / findings
113	
114	## 7. Changes in understanding
115	
116	## 8. Open questions
117	
118	## 9. Suggested next prompt
```

## 6. Evidence / findings

1. Low: `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md:104` 以降に template の空セクションが重複して残っており、report が完結していないように見える。AGENTS.md と `docs/reports/TEMPLATE.md` の report structure に照らすと、これは単純な取り込みミスで、現行 report としては削るべきである。
2. Low: `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md:68`-`75` は `Commands run and exact outputs` という見出しの下でコマンド列だけを置いており、template が要求する exact outputs を実際には記録していない。review trace としては不十分で、少なくとも pass/fail の要約か主要出力を残した方がよい。
3. No findings: `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` と `specs/examples/01-current-l2-surface-syntax-candidates.md` の今回の差分は current L2 semantics を拡張していない。guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` / rollback-cut non-interference は、`specs/04-mir-core.md:118`-`143`、`specs/10-open-questions.md:8`-`27`、`specs/12-decision-register.md:30`-`31`、`specs/examples/04-current-l2-step-semantics.md:147`-`176` と整合していた。
4. No findings: Candidate A vs Candidate B の比較は fair な範囲に留まっており、current L2 companion notation judgment も「暫定維持」に留められている。final parser syntax の既成事実化や B 排除の言い過ぎは見当たらなかった。
5. No findings: `>` ladder を比較対象に留める理由は `Documentation.md:89`、`specs/00-document-map.md:67`-`69`、`specs/examples/01-current-l2-surface-syntax-candidates.md:169`、`specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:97`-`100` と整合していた。docs mirror は「compact syntax candidate comparison を行い、current L2 では explicit edge-row form を暫定維持する」という粒度で揃っている。

## 7. Changes in understanding

current 変更の実体は syntax comparison の tightening であり、semantics 側は既存 L2 をそのまま参照していた。問題は spec 側ではなく、新規 report 0081 の構造取り込みが最後まで掃除されていない点に集約された。

## 8. Open questions

- report 0081 の evidence section に exact outputs をどこまで残すか。
- `>` ladder を comparison 対象から外す閾値を、将来どの mirror 文書で明示するか。

## 9. Suggested next prompt

`docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md` の末尾に残っている重複 template を除去し、Commands run セクションに主要出力を追加してください。spec 側は今回はそのままでよいです。
