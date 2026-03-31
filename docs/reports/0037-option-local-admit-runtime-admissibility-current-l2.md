# Report 0037 — current L2 の option-local `admit` runtime admissibility 整理

- Date: 2026-03-31T07:28:37.859106Z
- Author / agent: Codex
- Scope: current L2 companion notation における option-local `admit` と runtime admissibility の関係整理、representative examples への最小反映、mirror 文書の更新
- Decision levels touched: L2

## 1. Objective

option-local `admit` failure を、non-admissible skip、explicit failure、dynamic `Reject` のどれとして読むのが current L2 に最も整合的かを整理する。
今回は parser 実装には進まず、canonical normalization law、rejection phase、static evidence floor、underdeclared handling、statement-local `require` / `ensure`、option-local declared contract surface の既存方針を前提に、representative examples と companion notation で使える最小の runtime reading だけを定める。

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
- `docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`
- `docs/reports/0032-predicate-sublanguage-current-l2.md`
- `docs/reports/0034-option-local-declared-contract-surface-current-l2.md`
- `docs/reports/0035-review-option-local-declared-contract-surface.md`
- `docs/reports/0036-recheck-option-local-declared-contract-surface-review.md`

## 3. Actions taken

1. `code_mapper` を先に使い、task-start の worktree が clean であること、今回の主対象が `specs/examples/01`、`specs/examples/00`、`specs/10`、`specs/12` に閉じること、`admit` は admission-side metadata として既に current L2 に導入済みであることを確認した。
2. `admit` failure の読みとして、non-admissible skip、explicit failure、dynamic `Reject` の 3 案を比較した。
3. explicit failure 案は、option-local `admit` が request 実行後の failure ではなく admission gate であること、そして既存 examples が explicit failure を option を実際に試した後の outcome として使っていることと噛み合わないため退けた。
4. dynamic `Reject` 案は、1 つの option の admission miss と request 全体の失敗を同一視してしまい、E3 の比較用 variant と E6 の `Reject` の読みを不必要に重くするため退けた。
5. current L2 の最小読解として、`admit` 不成立は non-admissible skip、すなわちその option が current request の success-side candidate から外れるだけ、と整理した。
6. `specs/examples/01-current-l2-surface-syntax-candidates.md` に `admit` semantics の節を追加し、statement-local `require` / `ensure` と option-local `admit` の役割分担を runtime 側でも明示した。
7. `specs/examples/00-representative-mir-programs.md` では、E3 の比較用 variant と E6 の runtime outcome 説明に補助文を足し、option-local admission miss 自体と chain-level `Reject` を区別した。
8. mirror として `specs/10-open-questions.md` と `specs/12-decision-register.md` を更新した。
9. 最後に `reviewer` を使い、non-admissible skip 読みが既存理論と矛盾していないか、未決の観測面を決め打ちしていないかを確認する。

## 4. Files changed

- Changed:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
  - `docs/reports/0038-review-admit-runtime-reading.md`
- Checked but not changed:
  - `README.md`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/01-charter-and-decision-levels.md`
  - `specs/02-system-overview.md`
  - `specs/03-layer-model.md`
  - `specs/04-mir-core.md`
  - `specs/09-invariants-and-constraints.md`
  - `specs/11-roadmap-and-workstreams.md`
- Task-start dirty state:
  - なし。`git status --porcelain=v2 --branch` は `# branch.head main` と `# branch.ab +0 -0` のみだった。

## 5. Commands run and exact outputs

- `git status --porcelain=v2 --branch`
  - `# branch.oid d5685ea4ee7e9e53f182074256e8bdee1c67ff4a`
  - `# branch.head main`
  - `# branch.upstream origin/main`
  - `# branch.ab +0 -0`
- `python3 scripts/new_report.py --slug option-local-admit-runtime-admissibility-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
- `git diff --check`
  - 出力なし
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 38 numbered report(s).`
- `git commit --no-gpg-sign -m "admit の runtime reading を整理する"`
  - `[main b32875f] admit の runtime reading を整理する`
  - ` 4 files changed, 31 insertions(+), 4 deletions(-)`

## 6. Evidence / findings

### 6.1 三案比較

- non-admissible skip
  - `admit` を option admission gate として読む既存方針に最も素直に合う。
  - statement-local `require` / `ensure` と役割が競合しない。
  - `Reject` は chain 全体で admissible candidate が尽きたときにだけ残せる。
- explicit failure
  - option を実際に試した後の failure と、success-side choice に入る前の admission miss を混同する。
  - E3 比較用 variant では、単に `owner_writer` が使えないだけの場面に過剰な failure event を立てることになる。
- dynamic `Reject`
  - 1 つの option の `admit` miss で request 全体を落としやすくなり、後続 fallback successor を素直に読む canonical chain と噛み合いにくい。
  - E6 の `Reject` が持つ「chain 全体で admissible candidate が尽きた結果」という位置づけも曖昧になる。

### 6.2 current L2 の採用方針

- current L2 では、`admit` failure は non-admissible skip と読む。
- その option は current request の success-side candidate から外れるが、その時点では explicit failure event は立てない。
- その option miss だけで request 全体を dynamic `Reject` にもしない。
- dynamic `Reject` は、後続 option も含めて current request を満たす admissible candidate が尽きた場合にだけ立つ。
- explicit failure は、admitted option を試した後に operation 側で失敗した場合へ残す。

### 6.3 representative examples で確認できたこと

- E3 の比較用 variant では、`owner_writer` の `admit` miss を skip と読めば、`delegated_writer` を次の success-side candidate として自然に扱える。
- 同じ場面を explicit failure と読むと、「実行していない option が failure した」ように見え、option-local admission metadata という語感から外れる。
- 同じ場面を dynamic `Reject` と読むと、fallback successor が存在するのに request 全体が早期終了する読みになり、canonical chain の left-to-right reading と相性が悪い。
- E6 では `Reject` を chain exhaustion の outcome に残せるため、admission miss と request-level failure の境界が保てる。

### 6.4 reviewer による確認結果

- review は `docs/reports/0038-review-admit-runtime-reading.md` に記録した。
- 結論は `no findings` だった。
- reviewer は、今回の wording が canonical law / rejection phase / static evidence floor / underdeclared handling / request-local `require`-`ensure` policy を壊していないこと、explicit failure と dynamic `Reject` の境界も既存例と矛盾していないこと、観測面も「独立 event にするか skip reason に留めるか未決定」のまま保てていることを確認した。

## 7. Changes in understanding

option-local `admit` に必要だったのは、新しい failure class ではなく「option admission miss を request-level outcome から切り分ける最小読解」だった。non-admissible skip を採ると、statement-local `require` / `ensure`、option-local admission metadata、explicit failure、dynamic `Reject` がそれぞれ別の責務を保てる。

## 8. Open questions

- `admit` 不成立を trace / audit 上で独立 event として出すか、skip reason としてだけ残すか。
- `admit` predicate の evaluation を explicit boolean event として観測する必要があるか。
- option-local `admit` miss と `lease` expiry を trace 上で同じ kind の non-admissible reason としてまとめるか、別理由として分けるか。
- `admit` の最終 keyword / punctuation と final parser syntax。

## 9. 仕様本文コミット

- `b32875f` `admit の runtime reading を整理する`
- report 自身を含む後続コミットの hash は self-reference の都合で本 report 本文には固定しない。

## 10. Suggested next prompt

current L2 の representative examples を前提に、option-local `admit` miss と `lease` expiry を trace / audit 上でどう区別して残すべきかを整理してください。特に、non-admissible skip を独立 event にする必要があるか、それとも skip reason metadata に留めるべきかを、E3 の比較用 variant と E6 を使って比較してください。
