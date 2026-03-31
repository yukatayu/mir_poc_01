# Report 0045 — capability mismatch non admissible taxonomy current l2

- Date: 2026-03-31T23:34:24.427007Z
- Author / agent: Codex
- Scope: current L2 companion notation と representative examples における capability mismatch の non-admissible taxonomy 上の位置づけ整理、関連 mirror 文書の最小更新、review / verification / commit
- Decision levels touched: L2

## 1. Objective

current L2 の non-admissible reason family において、capability mismatch を `admit-miss` / `lease-expired` と並ぶ formal subreason に入れるべきか、それとも E6 の説明を支える narrative audit explanation に留めるべきかを比較し、event surface を増やさない最小方針を定める。

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
- `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
- `docs/reports/0044-review-non-admissible-reason-audit-metadata-shape.md`

## 3. Actions taken

1. `code_mapper` を最初に起動し、今回触るべきファイルと task-start dirty state の要約を取得した。
2. 指定順に入口文書と core spec を読み、current L2 では `admit-miss` / `lease-expired` だけが formal subreason として最小化され、capability mismatch は未決定のまま残っていることを再確認した。
3. `specs/examples/00-representative-mir-programs.md` の E6 を中心に、次の二案を比較した。
   - capability mismatch を formal subreason に上げる
   - capability mismatch を narrative audit explanation に留める
4. 比較の結果、current L2 では capability mismatch を formal subreason に上げず、request-local `require` と declared capability surface の比較から読む narrative explanation に留める方が、event surface と metadata shape を増やさず、E6 の読みも小さく保てると判断した。
5. この判断を `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に最小反映した。
6. 最後に `reviewer` を起動し、今回の taxonomy 整理が existing current L2 semantics を壊していないかを確認しようとした。
7. reviewer は timeout 後も completion を返さなかったため、agent を shutdown し、review 未返却であることを report に明記した。
8. local verification を取り直したうえで、specification-body だけを先に commit した。

## 4. Files changed

- Changed:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
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
  - なし。`git status --porcelain=v2 --branch` は `# branch.head main` と `# branch.ab +12 -0` を返した。

## 5. Commands run and exact outputs

- `git status --porcelain=v2 --branch`
  - `# branch.oid f68fa0dbf4830a21875dfe3db84691984d937085`
  - `# branch.head main`
  - `# branch.upstream origin/main`
  - `# branch.ab +12 -0`
- `python3 scripts/new_report.py --slug capability-mismatch-non-admissible-taxonomy-current-l2`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
- `git diff --check`
  - `[no output]`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 45 numbered report(s).`
- `git commit --no-gpg-sign -m "capability mismatch の taxonomy 位置づけを整理する"`
  - `[main a5e51f5] capability mismatch の taxonomy 位置づけを整理する`
  - ` 4 files changed, 7 insertions(+), 4 deletions(-)`

## 6. Evidence / findings

### 6.1 結論

- current L2 では、`admit-miss` と `lease-expired` だけを formal subreason に残し、capability mismatch は formal subreason に上げない。
- capability mismatch は、request-local `require` と declared capability surface の比較から読む narrative audit explanation に留める。
- これにより event surface は request-level outcome のまま保たれ、D-038 / D-039 の最小 metadata shape も広げずに済む。

### 6.2 formal subreason 化しない理由

- `admit-miss` と `lease-expired` は、どちらも「ある option が success-side candidate から外れた option-local reason」として `option ref` と `subreason` だけで説明できる。
- これに対して capability mismatch は、option-local metadata 単独ではなく、request-local `require` と option の declared capability surface の比較から読まれる。
- したがって capability mismatch を同列の formal subreason に上げると、non-admissible taxonomy が option-local miss record から request / option comparison taxonomy へ広がり、current L2 の最小性を崩しやすい。

### 6.3 E6 に対する読み

- E6 では `writer` の `lease-expired` だけを formal subreason として残せば、write-admissible option が尽きて final `Reject` に至る説明は足りる。
- `readonly` は「read-only で write request を満たせない」という narrative explanation のままで十分であり、formal subreason を追加しなくても readability は落ちない。
- むしろ E6 で capability mismatch まで formal subreason にすると、`writer` の option-local miss と `readonly` の request/capability comparison が同じレベルの taxonomy に見え、current L2 の最小 shape より重くなる。

### 6.4 current L2 に残す最小 taxonomy

- broad family: non-admissible reason
- formal subreason:
  - `admit-miss`
  - `lease-expired`
- narrative explanation に留めるもの:
  - capability mismatch

### 6.5 reviewer

- `reviewer` は起動したが、この session では completion を返さなかった。
- そのため reviewer finding を捏造せず、review 未返却として扱う。
- local diff inspection の範囲では、event surface を増やさず、`option ref` / `subreason` という最小 shape を保ったまま capability mismatch だけを narrative explanation に留めていることを確認した。

### 6.6 specification-body commit

- 仕様本文の commit は `a5e51f5` `capability mismatch の taxonomy 位置づけを整理する` で記録した。
- report 自身を含む報告 commit の hash は self-reference の都合で本書には固定せず、最終の assistant response と `git log` で示す。

## 7. Changes in understanding

current L2 の non-admissible taxonomy で本当に固定したいのは、「option-local miss をどこまで formal に残すか」であり、request-local clause と option capability surface の比較まで同じ taxonomy に押し込むことではなかった。E6 は、その境界を narrative explanation で保った方がむしろ読みやすい。

## 8. Open questions

- capability mismatch を将来 formal subreason に昇格させる必要が本当にあるか。
- もし昇格させるなら、`option ref` だけでなく request-local clause carrier を最小 shape に入れる必要があるか。
- field 名、reason code 名、serialization をどの decision で固定するか。
- parser 実装前に trace / audit surface の taxonomy をもう一段詰める必要があるか。

## 9. Suggested next prompt

current L2 の representative examples を前提に、capability mismatch を将来 formal subreason に昇格させる場合でも、`option ref` / `subreason` という最小 metadata shape を壊さずに済むかを比較してください。特に request-local `require` を別 carrier に残す案と、taxonomy 自体を拡張する案のどちらが小さいかだけに絞って整理してください。
