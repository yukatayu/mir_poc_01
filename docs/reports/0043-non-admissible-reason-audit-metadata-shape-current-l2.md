# Report 0043 — non admissible reason audit metadata shape current l2

- Date: 2026-03-31T09:18:45.556169Z
- Author / agent: Codex
- Scope: current L2 companion notation における non-admissible reason audit metadata の最小 shape 整理、representative examples への最小反映、mirror 文書の更新
- Decision levels touched: L2

## 1. Objective

option-local `admit` miss と `lease` expiry を current L2 では dedicated skip event にせず audit / trace metadata に留める方針を前提に、E3 比較用 variant と E6 を説明できる最小 metadata shape を整理する。
今回は parser 実装や dedicated trace implementation には進まず、event surface を増やさない前提で conceptual shape だけを定める。

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
- `docs/reports/0034-option-local-declared-contract-surface-current-l2.md`
- `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
- `docs/reports/0038-review-admit-runtime-reading.md`
- `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
- `docs/reports/0040-review-admit-vs-lease-trace-audit-diff.md`
- `docs/reports/0041-review-current-l2-non-admissible-skip-surface.md`
- `docs/reports/0042-review-current-l2-admit-lease-trace-audit-recheck.md`

## 3. Actions taken

1. `code_mapper` を先に起動し、task-start state と主対象ファイルの要約取得を試みた。返答待ちの間に、ローカルでも branch と worktree を確認し、今回の開始時点で worktree は clean、`main` は `origin/main` に対して ahead 7 であることを確認した。
2. current L2 の既存方針を読み直し、`admit` miss / `lease` expiry は dedicated skip event ではなく audit metadata に留め、event surface には request-level outcome を残すこと、`admit-miss` / `lease-expired` の区別は current L2 ですでに要求されていることを再確認した。
3. field 候補として `option ref`、`request ref`、`reason kind`、`subreason` を比較した。
4. E3 比較用 variant と E6 を使って、どの field 候補がないと説明不能になるかを比較した。
5. その結果、current L2 の最小 conceptual shape は「current request evaluation に結び付いた audit metadata」として、少なくとも `option ref` と `subreason` を持つ形で足りると整理した。broad family として non-admissible reason であることは metadata channel 自体から読めればよく、独立した `reason kind` field は current L2 の必須要件に含めない。
6. explicit な `request ref` field も current L2 の最小 shape には含めず、request-local attachment だけを要件として残した。detached serialization や cross-trace correlation の都合で将来必要になる可能性は **未決定** とした。
7. この方針を `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に最小反映した。
8. 最後に `reviewer` を使い、今回の metadata shape が既存理論と矛盾しないかを確認した。
9. reviewer は別 report `docs/reports/0044-review-non-admissible-reason-audit-metadata-shape.md` を返し、findings なしであることを確認した。

## 4. Files changed

- Changed:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
  - `docs/reports/0044-review-non-admissible-reason-audit-metadata-shape.md`
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
  - なし。`git status --porcelain=v2 --branch` は `# branch.head main` と `# branch.ab +7 -0` を返した。

## 5. Commands run and exact outputs

- `git status --porcelain=v2 --branch`
  - `# branch.oid 9cbfafe2477ff2189634047af5a02e79cc9eaf5d`
  - `# branch.head main`
  - `# branch.upstream origin/main`
  - `# branch.ab +7 -0`
- `python3 scripts/new_report.py --slug non-admissible-reason-audit-metadata-shape-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
- `git diff --check && python3 scripts/validate_docs.py`
  - `[no output from git diff --check]`
  - `Documentation scaffold looks complete.`
  - `Found 43 numbered report(s).`
- `git commit --no-gpg-sign -m "non-admissible reason metadata の最小 shape を整理する"`
  - `[main 87c2765] non-admissible reason metadata の最小 shape を整理する`
  - ` 4 files changed, 22 insertions(+), 2 deletions(-)`
- `git diff --check HEAD~4 HEAD && python3 scripts/validate_docs.py && git status --short --branch`
  - `[no output from git diff --check]`
  - `Documentation scaffold looks complete.`
  - `Found 44 numbered report(s).`
  - `## main...origin/main [ahead 11]`
  - `?? docs/reports/0044-review-non-admissible-reason-audit-metadata-shape.md`

## 6. Evidence / findings

### 6.1 current L2 の最小 shape

- `option ref` は最小必須である。これがないと、E3 比較用 variant でどちらの option が `admit-miss` により候補から外れたか、E6 でどの option が `lease-expired` だったかを説明できない。
- `subreason` も current L2 では残すべきである。これがないと E3 比較用 variant の `admit-miss` と E6 の `lease-expired` を同じ broad bucket に潰してしまい、代表例が要求する説明力を失う。
- broad family として non-admissible reason であることは current L2 でも必要だが、metadata channel 自体から読めれば足りる。独立した `reason kind` field までは要求しない。
- explicit な `request ref` field は current L2 の最小必須 field には含めない。metadata 自体が current request evaluation に request-local にぶら下がっている、という conceptual attachment があれば E3 と E6 の説明には足りるからである。

### 6.2 field 候補の比較

- `option ref`
  - 最小必須。E3 比較用 variant では `owner_writer` が落ちたこと、E6 では `writer` が `lease-expired` だったことを指せないと説明不能になる。
- `request ref`
  - conceptual association としては必要だが、field としては必須にしない。current L2 は metadata が current request evaluation にネストされている前提で足りる。
- `reason kind`
  - broad family を表す概念としては必要だが、独立 field としては必須にしない。current L2 では metadata channel 自体が non-admissible reason family を表してよい。
- `subreason`
  - 最小必須。E3 比較用 variant の `admit-miss` と E6 の `lease-expired` を区別するために必要である。

### 6.3 `admit` miss と `lease` expiry の最小分類

- current L2 では、両者を同じ大分類 `non-admissible reason` にまとめてよい。
- ただし subreason を捨ててはならない。E3 比較用 variant では「option-local `admit` が不成立だった」ことが重要であり、E6 では「`lease` expiry により write-admissible 候補が消えた」ことが重要だからである。
- したがって最小分類は「metadata channel が broad family を表し、その内部で distinguishable subreason を残す形」である。

### 6.4 representative examples への影響

- E3 比較用 variant では、`owner_writer` に対して request-local audit metadata として `option ref = owner_writer` と `subreason = admit-miss` があれば、skip を dedicated event にしなくても十分に説明できる。broad family は metadata channel から読めれば足りる。
- E6 では、`writer` に対して同じ shape で `subreason = lease-expired` を残せば、event surface を final `Reject` だけに保ったまま説明できる。
- `readonly` の request / capability mismatch については、current L2 では narrative explanation で十分であり、この task では同じ metadata shape に正式編入しない。

### 6.5 reviewer

- `reviewer` は起動し、今回の 5 ファイルだけに絞った review を依頼した。
- reviewer report `0044-review-non-admissible-reason-audit-metadata-shape.md` は findings なしだった。
- reviewer は、`option ref` / `subreason` の最小化、event surface 非拡張、capability mismatch の premature な taxonomy 編入なし、の 3 点で current L2 の整理と矛盾しないことを確認した。
- local verification と diff inspection の範囲でも、event surface を増やさず、`request ref` を explicit field として固定せず、field naming / serialization を未決定に残していることを確認した。

### 6.6 specification-body commit

- 仕様本文の commit は `87c2765` `non-admissible reason metadata の最小 shape を整理する` で記録した。
- report 自身を含む報告 commit の hash は self-reference の都合で本書には固定せず、最終の assistant response と `git log` で示す。

## 7. Changes in understanding

current L2 で本当に必要なのは、skip を event 化することではなく、「どの option が、どの subreason によって success-side candidate から外れたか」を request-local に説明できることだった。broad family は metadata channel から読めればよく、`request ref` も概念的には必要でも field としてはまだ固定しなくてよい。

## 8. Open questions

- field 名、reason code 名、serialization をどこまで固定するか。
- detached serialization や cross-trace correlation の都合で、`request ref` 相当を field として要求する必要があるか。
- capability mismatch を `non-admissible reason` taxonomy の正式 subreason に入れるかどうか。
- dedicated skip event を将来導入する必要があるか。

## 9. Suggested next prompt

current L2 の representative examples を前提に、capability mismatch を `non-admissible reason` taxonomy の正式 subreason に入れるべきか、それとも narrative audit explanation に留めるべきかを比較してください。event surface は増やさず、E6 の readability と taxonomy の最小性だけに焦点を絞って整理してください。
