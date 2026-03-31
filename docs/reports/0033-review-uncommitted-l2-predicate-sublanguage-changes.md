# Report 0033 — current L2 predicate sublanguage 未コミット変更レビュー

- Date: 2026-03-31T05:46:37.832120Z
- Author / agent: Codex
- 対象範囲: current L2 predicate sublanguage 候補に関する未コミット変更の review。対象は `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`
- 影響した決定レベル: L2 review only; no normative spec change authored in this task.

## 1. 目的

current L2 の predicate sublanguage 候補に関する未コミット変更が、

- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、`contract` semantic-role-only policy を保っているか
- companion notation を必要最小限に留め、理論を黙って増やしていないか
- 未決の `or` / `not` / precedence / final parser grammar を早まって固定していないか
- representative examples の clause attachment を曖昧にしていないか

を確認する。

## 2. 参照文書

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`（fallback evidence floor、underdeclared handling、syntax 境界に関する必要箇所）
- 対象 4 ファイルの未コミット diff と line-numbered view

## 3. 実施内容

1. `AGENTS.md` が要求する順序で repo の入口文書を読み直した。
2. review baseline として必要な Mir core / open questions / decision register の該当箇所を確認した。
3. `git status --short` で review 対象の未コミット変更範囲を確認した。
4. 対象 4 ファイルの未コミット diff だけを読んだ。
5. 依頼された 4 観点に沿って changed lines を照合した。

## 4. 変更ファイル

- 追加したファイル:
  - `docs/reports/0033-review-uncommitted-l2-predicate-sublanguage-changes.md`

## 5. 実行コマンドと出力

1. `git status --short`
   出力:
   ```text
    M specs/10-open-questions.md
    M specs/12-decision-register.md
    M specs/examples/00-representative-mir-programs.md
    M specs/examples/01-current-l2-surface-syntax-candidates.md
   ?? docs/reports/0032-predicate-sublanguage-current-l2.md
   ```
2. `python3 scripts/new_report.py --slug review-uncommitted-l2-predicate-sublanguage-changes`
   出力:
   ```text
   /home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
     text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
   /home/yukatayu/dev/mir_poc_01/docs/reports/0033-review-uncommitted-l2-predicate-sublanguage-changes.md
   ```
3. `git diff --unified=12 -- <target file>` と `nl -ba <target file> | sed -n '<range>p'`
   出力要約:
   - predicate sublanguage 追加は依頼された 4 ファイルに閉じていた。
   - 他の normative spec file の未コミット変更は review 対象に含めなかった。

## 6. 根拠と確認結果

結果: no findings.

根拠の要約:

- 新しい predicate sublanguage の記述は current L2 companion notation に明示的に限定されており、final parser syntax を固定していないことを繰り返し明記している。
- 変更は fallback / preference-chain semantics、rejection-phase boundary、static evidence floor、underdeclared handling を変更しておらず、example predicate の書き方と読み方だけを絞っている。
- `contract` semantic-role-only policy は保たれており、`contract` は mandatory な surface keyword に上がっていない。`require` / `ensure` も引き続き `perform` に付く statement-local clause のままである。
- multiline example で newline-based implicit conjunction を禁止したことで、第二の読みを増やすのではなく clause attachment ambiguity を減らしている。
- `or`、`not`、precedence、predicate grammar の完全化、final grammar における blank-line 許可、reserved keyword 集合は未決定のまま残っている。

## 7. 理解の更新

- 今回の編集は grammar を固定する判断より狭く、parser-ready な predicate language ではなく examples の読解規律を与えるものだと確認できた。
- explicit `and` と括弧 grouping を加えるだけで、既存の semantic boundary を変えずに representative examples を十分書ける。

## 8. 未解決事項

- 将来の current L2 examples が、final predicate grammar 設計前に `or` / `not` / 比較演算子を必要とするかどうか。
- example が長くなった場合でも、predicate block 内 blank line 禁止を companion notation で維持すべきかどうか。

## 9. Suggested next prompt

`0032-predicate-sublanguage-current-l2.md` の本文が、D-035 と updated examples の wording を正確に反映しているかを点検し、report 言語と normative / open-question 文言のあいだに食い違いがあれば指摘してください。
