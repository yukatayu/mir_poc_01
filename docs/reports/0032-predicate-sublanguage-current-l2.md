# Report 0032 — current L2 の predicate sublanguage 候補整理

- Date: 2026-03-31T05:38:52.710711Z
- Author / agent: Codex
- 対象範囲: current L2 companion notation における `require` / `ensure` の predicate sublanguage 候補の整理、representative examples への最小反映、関連 mirror 文書の更新
- 影響した決定レベル: L2

## 1. 目的

current L2 の representative Mir programs と companion notation において、`contract` を semantic role に留め、statement-local `require` / `ensure` を使う既存方針を前提に、predicate の最小断片を整理する。
今回は parser 実装には進まず、current L2 companion notation として十分な predicate fragment を定め、既存 examples を必要最小限だけ書き換える。

## 2. 参照文書

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`
- `docs/reports/0026-try-fallback-surface-syntax-candidates.md`
- `docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md`
- `docs/reports/0029-contract-surface-policy-current-l2.md`
- `docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`
- `docs/reports/0031-review-uncommitted-l2-clause-punctuation-changes.md`

## 3. 実施内容

1. `code_mapper` を先に起動し、task-start の worktree が clean であること、predicate 記法の主要な更新点が `specs/examples/01`、`specs/examples/00`、`specs/10`、`specs/12` に閉じることを確認した。
2. current L2 の clause / punctuation / `contract` policy を再確認し、predicate fragment だけを companion notation として追加する方針に絞った。
3. `specs/examples/01-current-l2-surface-syntax-candidates.md` に predicate sublanguage の節を追加し、atomic predicate、explicit `and`、括弧 grouping、改行継続、blank line 禁止を current L2 の最小候補として整理した。
4. `specs/examples/00-representative-mir-programs.md` を最小更新し、E1 を application-like form の単独 predicate、E2 を explicit `and` と grouping を使う multi-line predicate に整え、single-line bare predicate と multi-line predicate block の両方を representative examples 上で確認できるようにした。
5. policy の mirror として `specs/10-open-questions.md` と `specs/12-decision-register.md` を更新した。
6. 最後に `reviewer` を使い、predicate sublanguage 候補が既存理論を壊していないか、未決の grammar を決め打ちしていないかを確認した。結果は `no findings` で、レビュー証跡は `docs/reports/0033-review-uncommitted-l2-predicate-sublanguage-changes.md` に残した。

## 4. 変更ファイル

- 変更したファイル:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0032-predicate-sublanguage-current-l2.md`
  - `docs/reports/0033-review-uncommitted-l2-predicate-sublanguage-changes.md`
- 確認したが変更しなかったファイル:
  - `README.md`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/01-charter-and-decision-levels.md`
  - `specs/02-system-overview.md`
  - `specs/03-layer-model.md`
  - `specs/04-mir-core.md`
  - `specs/09-invariants-and-constraints.md`
  - `specs/11-roadmap-and-workstreams.md`
- task-start 時点の既存 dirty state:
  - なし。`git status --porcelain=v2 --branch` は `# branch.head main` と `# branch.ab +4 -0` のみだった。

## 5. 実行コマンドと出力

- `git status --porcelain=v2 --branch`
  - `# branch.oid ...`
  - `# branch.head main`
  - `# branch.upstream origin/main`
  - `# branch.ab +4 -0`
- `python3 scripts/new_report.py --slug predicate-sublanguage-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0032-predicate-sublanguage-current-l2.md`
- `git diff --check`
  - 出力なし
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 33 numbered report(s).`
- `git commit --no-gpg-sign -m "predicate sublanguage の候補を整理する"`
  - `[main a8ee38f] predicate sublanguage の候補を整理する`
  - ` 4 files changed, 37 insertions(+), 13 deletions(-)`

## 6. 根拠と確認結果

### 6.1 current L2 の最小 predicate fragment

- atomic predicate は bare atom か application-like form で十分とした。
- 複数 predicate を 1 つの clause にまとめる必要がある場合、current L2 companion notation では explicit `and` だけを最小 connective とした。
- grouping は括弧で十分とし、追加の block predicate syntax は導入しない。
- multi-line predicate block 内の改行は continuation であり、implicit conjunction は導入しない。
- blank line は predicate block 内では許さない。
- representative examples では、E1 を application-like form の単独 predicate、E2 を explicit `and` と grouping を使う multi-line predicate に整え、E3 / E6 は bare atom clause のまま読めることを確認した。

### 6.2 review と仕様本文 commit

- `reviewer` は `no findings` を返した。
- 残留リスクとしては、`or` / `not` / precedence / final parser grammar が未決のままであることと、predicate block の newline-only continuation / blank line 禁止が、将来例が長くなったときに可読性と再トレードオフになる可能性があることが挙がった。
- 仕様本文 commit は `a8ee38f predicate sublanguage の候補を整理する` で記録した。

### 6.3 未解決のまま残した点

- `or` / `not` を current L2 の admitted predicate fragment に含めるか
- precedence table を companion notation で固定するか
- predicate block 内の richer な expression grammar
- final parser syntax としての punctuation

## 7. 理解の更新

current L2 の representative examples に必要だったのは「predicate を書ける完全言語」ではなく、長い precondition / postcondition を誤読せずに読める最小断片だった。bare atom、predicate application、explicit `and`、括弧 grouping までに絞ると、`contract` を surface keyword に上げずに clause attachment を保ったまま examples をかなり安定して書ける。

## 8. 未解決事項

- `or` / `not` を current L2 companion notation に含める必要が本当にあるか。
- precedence table を companion notation で先に固定する必要があるか。
- predicate block で line-leading `and` 以外の継続 style を許すか。
- richer な predicate surface が option-local contract surface とどこまで共有されるべきか。

## 9. Suggested next prompt

current L2 の companion notation を前提に、`option-local declared contract surface` の最小表記だけを整理してください。特に `perform` に付く statement-local `require` / `ensure` と、option declaration 側に将来必要になりうる contract 情報を、どこまで同じ predicate fragment で共有してよいかを比較してください。

注記:
- report 自身と review report (`0033`) の commit hash は self-reference の都合でこの本文には固定していない。
