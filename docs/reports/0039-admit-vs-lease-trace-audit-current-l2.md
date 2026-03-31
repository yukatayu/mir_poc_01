# Report 0039 — current L2 の `admit` miss / `lease` expiry trace・audit 整理

- Date: 2026-03-31T08:24:31.977362Z
- Author / agent: Codex
- Scope: current L2 companion notation における option-local `admit` miss と `lease` expiry の trace / audit semantics 整理、representative examples への最小反映、mirror 文書の更新
- Decision levels touched: L2

## 1. Objective

current L2 で `admit` failure を non-admissible skip と読む方針を前提に、`admit` miss と `lease` expiry を trace / audit 上でどう残すのが最小で整合的かを整理する。
今回は parser 実装や dedicated trace implementation には進まず、event surface と audit surface の最小区別だけを定める。

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
- `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
- `docs/reports/0038-review-admit-runtime-reading.md`

## 3. Actions taken

1. `code_mapper` を先に使い、task-start の worktree が clean であること、今回の主対象が `specs/examples/01`、`specs/examples/00`、`specs/10`、`specs/12` に閉じること、そして未決点が「dedicated skip event を立てるか」と「`admit` miss / `lease` expiry を同一分類にまとめるか」に絞られていることを確認した。
2. 観測面の候補を次の二案で比較した。
   - non-admissible skip を dedicated event にせず、audit / trace 側の skip reason metadata に留める案
   - non-admissible skip を独立 event として event surface に上げる案
3. 前者は、explicit failure と dynamic `Reject` を request-level outcome として残しつつ、`admit` miss と `lease` expiry の説明可能性だけを audit 側に持てるため、current L2 の最小観測面として採用した。
4. 後者は、個々の option miss を event surface に持ち込み、explicit failure や final `Reject` と同じ階層に並べることで観測面を重くするため、current L2 では採らなかった。
5. そのうえで、`admit` miss と `lease` expiry は同じ大分類の non-admissible reason に入れてよいが、少なくとも subreason は区別する方針に整理した。
6. `specs/examples/01-current-l2-surface-syntax-candidates.md` に `admit` / `lease` の trace-audit 読みを追加し、representative examples 側では E3 比較用 variant と E6 の最小 trace / audit 説明を更新した。
7. mirror として `specs/10-open-questions.md` と `specs/12-decision-register.md` を更新した。
8. 最後に `reviewer` を使い、この観測面整理が canonical law、runtime `Reject` の境界、underdeclared handling を壊していないか確認する。

## 4. Files changed

- Changed:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
  - `docs/reports/0040-review-admit-vs-lease-trace-audit-diff.md`
  - `docs/reports/0041-review-current-l2-non-admissible-skip-surface.md`
  - `docs/reports/0042-review-current-l2-admit-lease-trace-audit-recheck.md`
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
  - なし。`git status --porcelain=v2 --branch` は `# branch.head main` と `# branch.ab +3 -0` を返した。

## 5. Commands run and exact outputs

- `git status --porcelain=v2 --branch`
  - `# branch.oid d5d67cdd86564fc586d1b8d29aea282705dd9ca9`
  - `# branch.head main`
  - `# branch.upstream origin/main`
  - `# branch.ab +3 -0`
- `python3 scripts/new_report.py --slug admit-vs-lease-trace-audit-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
- `git diff --check && python3 scripts/validate_docs.py`
  - `[no output from git diff --check]`
  - `Documentation scaffold looks complete.`
  - `Found 42 numbered report(s).`
- `git commit --no-gpg-sign -m "admit と lease の観測面を整理する"`
  - `[main 09ff3a8] admit と lease の観測面を整理する`
  - ` 4 files changed, 16 insertions(+), 5 deletions(-)`

## 6. Evidence / findings

### 6.1 current L2 の最小観測面

- event surface には request-level outcome を優先して残す。
- したがって current L2 では、non-admissible skip を dedicated event にする必要はない。
- `admit` miss と `lease` expiry は、audit / trace 側の skip reason metadata に留める方が、explicit failure と dynamic `Reject` の境界を壊さない。

### 6.2 `admit` miss と `lease` expiry の分類

- current L2 の最小分類では、両者を同じ大分類の non-admissible reason に入れてよい。
- ただし理由を完全に潰すと、E3 比較用 variant と E6 の説明力が落ちる。
- そのため audit 側では、少なくとも `admit-miss` と `lease-expired` を区別できる subreason を残すのが最小である。
- field 名、reason code 名、serialization、他の reason との taxonomy は **未決定** とする。

### 6.3 representative examples で確認できたこと

- E3 比較用 variant では、`owner_writer` の `admit` miss を event にしなくても、`delegated_writer` が選ばれたことと audit metadata 上の `admit-miss` だけで十分に説明できる。
- ここで dedicated skip event を立てると、admission gate と request-level outcome の境界が濃くなりすぎ、current L2 の companion notation に対して観測面が重い。
- E6 では、event surface は final `Reject` だけに留め、その手前の `writer` の `lease` expiry を `non-admissible reason = lease-expired` として audit 側に残せば十分に説明できる。
- したがって、両例を同時に最も素直に読めるのは「event surface は薄く、audit 側で non-admissible reason を持つ」案だった。

### 6.4 reviewer による確認結果

- 初回 review は `docs/reports/0040-review-admit-vs-lease-trace-audit-diff.md` に記録した。
- そこでの finding は 1 点だけで、D-037 がなお「独立 event か skip reason か未決定」と読め、D-038 の metadata-only current reading と wording conflict を起こしていた。
- そのため D-037 を runtime reading 専用に狭め、観測方針は D-038 に委ねる形へ修正した。
- recheck は `docs/reports/0041-review-current-l2-non-admissible-skip-surface.md` に記録した。
- recheck は no findings で、`admit` miss / `lease` expiry を dedicated skip event に上げず audit metadata に留める current L2 読解、`admit-miss` / `lease-expired` の最小 subreason 区別、dedicated skip event と final taxonomy を **未決定** に残す方針が一貫していることを確認した。
- 追加の独立 recheck は `docs/reports/0042-review-current-l2-admit-lease-trace-audit-recheck.md` に記録した。
- こちらも no findings で、D-037 / D-038 の wording conflict が解消され、examples と open questions / decision register の読みが揃っていることを再確認した。

### 6.5 specification-body commit

- 仕様本文の commit は `09ff3a8` `admit と lease の観測面を整理する` で記録した。
- report 自身を含む報告 commit の hash は self-reference の都合で本書には固定せず、最終の assistant response と `git log` で示す。

## 7. Changes in understanding

`admit` miss と `lease` expiry をどう観測するかで本当に必要だったのは、新しい event 種別ではなく、request-level outcome に至る前の option miss を説明できる最小 audit metadata だった。current L2 では、non-admissible reason という広い箱と、その下位 reason の区別だけあれば representative examples を崩さずに読める。

## 8. Open questions

- dedicated skip event を将来導入する必要が本当にあるか。
- audit surface で最低限残すべき metadata の key 形状や serialization をどこまで揃えるか。
- `lease` expiry と `admit` miss 以外の non-admissible reason、例えば capability mismatch を同じ taxonomy に入れるかどうか。

## 9. Suggested next prompt

current L2 の representative examples を前提に、non-admissible reason metadata の最小 shape を整理してください。特に、`option ref`、`reason kind`、`subreason`、`request ref` のどれを audit に最低限残せば E3 比較用 variant と E6 を説明できるかを、event surface を増やさずに比較してください。
