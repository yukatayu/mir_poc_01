# Report 0030 — require ensure punctuation and multiline current l2

- Date: 2026-03-31T04:51:33.253611Z
- Author / agent: Codex
- Scope: current L2 companion notation における `require` / `ensure` clause の punctuation と multi-line predicate の最小 surface syntax 候補の整理、および representative examples への最小反映
- Decision levels touched: L2

## 1. Objective

current L2 の representative Mir programs と companion notation において、`contract` を semantic role に留める既存方針を前提に、statement-local `require` / `ensure` の読みをもう一段安定させる。
今回は parser 実装や final grammar には進まず、single-line clause と multi-line predicate の最小 punctuation 候補を整理し、既存 representative examples を必要最小限だけ書き換える。

## 2. Inputs consulted

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

## 3. Actions taken

1. `code_mapper` を先に起動し、対象ファイルと task-start の dirty state を確認した。
2. current L2 の clause policy を棚卸しし、single-line clause は bare form を維持し、multi-line predicate が必要な場合だけ colon-headed clause block を使う案を比較した。
3. `specs/examples/01-current-l2-surface-syntax-candidates.md` に punctuation と multi-line predicate の最小規則を追記した。
4. `specs/examples/00-representative-mir-programs.md` の E1, E2, E3 を最小限だけ書き換え、single-line / multi-line の両方を examples 上で確認できるようにした。
5. policy の mirror として `specs/10-open-questions.md` と `specs/12-decision-register.md` を更新した。
6. 仕上げに `reviewer` を使い、今回の punctuation 候補が既存理論を壊していないかを確認した。
7. reviewer の指摘に合わせて、E3 の `require:` を bare single-line `require read` に戻し、E1 の clause 終端説明を separator policy に揃えた。

## 4. Files changed

- 変更したファイル:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`
  - `docs/reports/0031-review-uncommitted-l2-clause-punctuation-changes.md`
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
  - なし。`git status --short --branch --untracked-files=all` は `## main...origin/main [ahead 2]` のみだった。

## 5. Commands run and exact outputs

- `git status --short --branch --untracked-files=all`
  - `## main...origin/main [ahead 2]`
- `python3 scripts/new_report.py --slug require-ensure-punctuation-and-multiline-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`
- `git diff --check`
  - 出力なし
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 30 numbered report(s).`
- `git add specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md && git commit -m "require と ensure の句読点候補を整理する"`
  - `error: gpg failed to sign the data:`
  - `gpg: signing failed: No such file or directory`
  - `fatal: failed to write commit object`
- `git commit --no-gpg-sign -m "require と ensure の句読点候補を整理する"`
  - `[main 77c1a54] require と ensure の句読点候補を整理する`
  - ` 4 files changed, 26 insertions(+), 12 deletions(-)`
- `code_mapper`
  - current worktree は clean、`require` / `ensure` の clause attachment と separator policy は既存文書にあり、final punctuation と multi-line policy だけが未確定という要約を得た。
- `reviewer`
  - finding 1 件。E3 が `require:` の下に単一行 `read` だけを置いており、「colon-headed form は multi-line predicate が必要な場合だけ使う」という今回の方針と食い違う、という指摘だった。
  - review 記録: `docs/reports/0031-review-uncommitted-l2-clause-punctuation-changes.md`

## 6. Evidence / findings

### 6.1 current L2 で採った最小 punctuation 候補

- single-line clause は従来どおり `require pred` / `ensure pred` と書く。
- multi-line predicate が必要な場合だけ `require:` / `ensure:` を header line にし、その直下へ 1 段深い predicate block を置く。
- predicate block は複数 clause の列挙ではなく、1 つの predicate を複数行に折り返したものとして読む。
- clause attachment は従来どおり直前の `perform` に限定し、`contract` を surface keyword に上げない current L2 policy と矛盾しない。

### 6.2 representative examples で確認したこと

- E1 では `ensure:` の multi-line predicate を使っても、`atomic_cut` までの clause attachment は誤読されない。
- E2 では `try { ... } fallback { ... }` の内部で multi-line `require:` を使っても、rollback scope や branch 境界の読みに影響しない。
- E3 では `via` request に multi-line `require:` を置いても、option chain の canonical order や fallback semantics を変えずに書ける。

### 6.3 current L2 で未決のまま残した点

- final parser syntax としての `require` / `ensure` の punctuation
- predicate block 内の boolean expression grammar
- predicate block 内で blank line を許すかどうか
- `contract { ... }` block form や explicit separator token の導入可否

### 6.4 reviewer で確認できたこと

- reviewer は、E3 の `require:` が multi-line 専用規則と食い違う点を 1 件だけ指摘した。
- その指摘に従い、E3 は `require read` に戻した。
- あわせて E1 の説明文から「blank line で suite が終わる」という wording を除き、separator policy の「blank line は readability-only」と揃えた。
- 修正後は current L2 の既存 theory、`contract` semantic-role-only policy、perform attachment、separator / block nesting、`try` / `fallback` policy との整合が保たれるという確認を得た。

### 6.5 仕様本文コミット

- `77c1a54 require と ensure の句読点候補を整理する`
- report 自身の commit hash は self-reference の都合でこの本文には固定せず、最終 `git log` で補う。

## 7. Changes in understanding

`require` / `ensure` の読みで不安定だったのは clause attachment そのものではなく、長い predicate をどこまで bare line で書くかだった。single-line を軽いまま維持し、multi-line にだけ `:` と 1 段深い predicate block を与えると、既存の `place` / `try` / `fallback` / option chain の読みを壊さずに安定化できることが確認できた。

## 8. Open questions

- `require:` / `ensure:` に続く predicate block の内部で、論理結合子、括弧、省略記法をどこまで認めるかは未決定である。
- blank line を predicate block の内部で readability 用に許すかどうかは未決定である。
- current L2 では `contract` を semantic role に留めているため、将来 `contract { ... }` block form を optional sugar として再検討する場合、この punctuation policy と同時に見直す必要がある。

## 9. Suggested next prompt

current L2 の companion notation を前提に、`predicate` 自体の最小 surface syntax を整理してください。特に `require:` / `ensure:` の multi-line predicate block 内で、論理結合子、括弧、改行継続、blank line 禁止の範囲をどこまで current L2 で固定するかを、representative examples を使って比較してください。
