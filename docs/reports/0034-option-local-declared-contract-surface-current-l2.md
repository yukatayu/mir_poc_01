# Report 0034 — current L2 の option-local declared contract surface 候補整理

- Date: 2026-03-31T06:14:07.850295Z
- Author / agent: Codex
- 対象範囲: current L2 companion notation における option-local declared contract surface の最小表記整理、representative examples への最小反映、関連 mirror 文書の更新
- 影響した決定レベル: L2

## 1. Objective

current L2 の representative Mir programs と companion notation において、option declaration 側に将来必要になりうる contract 情報をどこまで持ち込むべきかを整理する。
今回は parser 実装には進まず、`perform ... on` / `perform ... via`、option / chain 宣言、`try { ... } fallback { ... }`、statement-local `require` / `ensure`、predicate sublanguage の既存方針を前提に、option-local declared contract surface の最小候補だけを companion notation として与える。

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
- `docs/reports/0032-predicate-sublanguage-current-l2.md`
- `docs/reports/0033-review-uncommitted-l2-predicate-sublanguage-changes.md`

## 3. 実施内容

1. `code_mapper` を先に起動し、task-start の worktree が clean であること、今回の主対象が `specs/examples/01`、`specs/examples/00`、`specs/10`、`specs/12` に閉じること、option-local contract surface の既決 / 未決が `specs/04`、`specs/10`、`specs/12` に集約されていることを確認した。
2. current L2 の既存文脈から、option-local contract surface に関する候補を次の 3 案に絞った。
   - option 側には何も足さず、capability / `lease` だけに留める案
   - statement-local `require` / `ensure` を option declaration に流用する案
   - option 側は admission-side metadata だけを別 marker で持つ案
3. 2 案目は、`require` / `ensure` を直前の `perform` にだけ付ける既存の clause attachment policy と衝突するため除外した。
4. 1 案目は E3 / E6 のような capability-only case では十分だが、同じ target / capability / `lease` を持つ option 間の admissibility の違いを書き分けられないため、option-local 情報が本当に必要な例を支えられないと判断した。
5. そのため current L2 の最小 companion notation として、option declaration にぶら下がる admission-side metadata だけを `admit pred` / `admit:` で書く案を採った。
6. `specs/examples/01-current-l2-surface-syntax-candidates.md` に option-local declared contract surface の節を追加し、`admit` の役割、statement-local `require` / `ensure` との役割分担、predicate fragment の再利用範囲、未決事項を整理した。
7. `specs/examples/00-representative-mir-programs.md` では、E3 と E6 を capability-only case として明示しつつ、E3 配下に比較用 variant を追加して option-local `admit` が必要な場合の書き味を確認した。
8. mirror として `specs/10-open-questions.md` と `specs/12-decision-register.md` を更新した。
9. 最後に `reviewer` を使い、`admit` 候補が既存理論を壊していないか、未決の syntax を決め打ちしていないか、statement-local clause policy と衝突していないかを確認する。

## 4. 変更ファイル

- 変更したファイル:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0034-option-local-declared-contract-surface-current-l2.md`
  - `docs/reports/0035-review-option-local-declared-contract-surface.md`
  - `docs/reports/0036-recheck-option-local-declared-contract-surface-review.md`
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
  - なし。`git status --porcelain=v2 --branch` は `# branch.head main` と `# branch.ab +0 -0` のみだった。

## 5. 実行コマンドと出力

- `git status --porcelain=v2 --branch`
  - `# branch.oid 57e728f15aedefb3927af0ad48e035370cd0c6a4`
  - `# branch.head main`
  - `# branch.upstream origin/main`
  - `# branch.ab +0 -0`
- `python3 scripts/new_report.py --slug option-local-declared-contract-surface-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0034-option-local-declared-contract-surface-current-l2.md`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
- `git diff --check`
  - 出力なし
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 36 numbered report(s).`
- `git commit --no-gpg-sign -m "option-local contract surface の候補を整理する"`
  - `[main 2e12262] option-local contract surface の候補を整理する`
  - ` 4 files changed, 85 insertions(+), 11 deletions(-)`

## 6. 根拠と確認結果

### 6.1 current L2 の最小 option-local contract surface

- current L2 では、option declaration の head は従来どおり `option name on target capability cap lease guard` に留める。
- option-local contract 情報を持ち込むのは、capability と `lease` だけでは option 間の admissibility の差を書けない場合に限る。
- その最小表記は admission-side metadata のみであり、`admit pred` または `admit:` に続く predicate block を option declaration にぶら下げる companion notation とする。
- `admit` に書く predicate は、statement-local `require` / `ensure` と同じ最小 predicate fragment、すなわち bare atom、application-like form、explicit `and`、括弧 grouping を再利用してよい。
- option-local `ensure`、option-local `invariant`、その他の outcome-side guarantee は current L2 では導入しない。

### 6.2 statement-local clause との役割分担

- `perform` に付く `require` / `ensure` は request-local contract を表す。
- option declaration に付く `admit` は、その option 自身が success-side choice になれる条件を表す。
- したがって current L2 では、同じ predicate fragment を使っても clause attachment policy は共有しない。`require` / `ensure` は直前の `perform` にだけ付き、`admit` は直前の `option` declaration にだけ付く。

### 6.3 representative examples で比較できたこと

- E3 は capability-only case として保てた。`write -> write -> read` という capability の単調劣化だけで successor compatibility を説明できるので、option-local `admit` は不要だった。
- E6 も capability-only case として保てた。runtime `Reject` は request-local `require write` と option の capability / `lease` だけで十分に読める。
- E3 配下の比較用 variant では、target / capability / `lease` が同じ 2 option を並べ、option-local `admit` がないと admissibility の差を書き分けにくいことを示せた。

### 6.4 current L2 で未決のまま残した点

- `admit` の最終 keyword / punctuation
- option-local outcome guarantee / invariant を separate marker で持つかどうか
- option-local contract surface の final parser syntax
- option-local contract surface と richer な capability / mixed case 比較規則の関係

### 6.5 reviewer による確認結果

- 初回 review は `docs/reports/0035-review-option-local-declared-contract-surface.md` に記録した。
- そこで見つかった指摘は次の 2 点だった。
  - D-033 がなお「surface では statement-local `require` / `ensure` だけを使う」と読め、今回追加の D-036 と wording conflict を起こしていた。
  - `admit` が必要な case で omission を許すように読め、D-028 の underdeclared static error policy と接続が弱かった。
- そのため、D-033 を request-local surface の話に狭め、D-036 で option-local admission metadata を別扱いに整理した。
- あわせて、`specs/10-open-questions.md`、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md` に「capability と `lease` だけで admissibility 差を書けない case では `admit` を明示しなければならず、省略した branch は underdeclared 側に残る」と追記した。
- 再 review は `docs/reports/0036-recheck-option-local-declared-contract-surface-review.md` に記録され、結論は `no findings` だった。

## 7. 理解の更新

option-local declared contract surface について、current L2 で本当に必要だったのは「full contract language」ではなく、capability-only case では足りない例を壊さずに書ける最小 admission-side metadata だった。`admit` を option declaration 専用の companion marker に限定すると、statement-local `require` / `ensure` の読みを壊さずに option-local 情報だけを追加できる。

## 8. 未解決事項

- `admit` を current L2 companion notation のまま維持するか、別 token にするか。
- option-local outcome guarantee / invariant を examples に導入する必要が本当にあるか。
- option-local `admit` と `declared capability surface` の比較を、read / write より細かい mixed case へどう拡張するか。
- option-local contract surface と future parser grammar の関係を、どの段階で固定するか。

## 9. 仕様本文コミット

- `2e12262` `option-local contract surface の候補を整理する`
- report 自身を含む後続コミットの hash は self-reference の都合で本 report 本文には固定しない。

## 10. Suggested next prompt

current L2 の companion notation を前提に、option-local `admit` と runtime admissibility の関係を representative examples 上でさらに絞ってください。特に、option-local `admit` failure を explicit failure・dynamic `Reject`・単なる non-admissible skip のどれとして読むのが current L2 に最も整合的かを、E3 の比較用 variant を使って整理してください。
