# Report 0041 — review current l2 non admissible skip surface

- Date: 2026-03-31T08:32:03.602098Z
- Author / agent: Codex (GPT-5)
- Scope: `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` の未コミット差分 review。spec 本文は編集しない
- Decision levels touched: L2 review only

## 1. Objective

current L2 の non-admissible skip 観測面に関する未コミット差分を短く review し、次を確認する。

- `admit` miss / `lease` expiry を event surface に上げず audit metadata に留める方針が explicit failure / dynamic `Reject` 境界を壊していないか
- 同じ大分類の non-admissible reason と subreason 区別が current L2 として過剰でないか
- dedicated skip event と final taxonomy を未決定のまま保てているか

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/TEMPLATE.md`

## 3. Actions taken

1. AGENTS.md の順序に従って基準文書を確認し、failure visibility、dynamic `Reject`、monotone degradation、未決定事項の扱いを再確認した。
2. 指定 4 ファイルの未コミット差分を `git diff` で確認し、変更箇所を行番号付きで読み直した。
3. 3 つの確認観点ごとに、`specs/04` の既存 L2 reading と diff の整合性を点検した。
4. review 結果をこの新規 report に記録した。

## 4. Files changed

- `docs/reports/0041-review-current-l2-non-admissible-skip-surface.md` を新規追加

## 5. Commands run and exact outputs

- `git status --short -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md docs/reports/0041-review-current-l2-non-admissible-skip-surface.md`

```text
 M specs/10-open-questions.md
 M specs/12-decision-register.md
 M specs/examples/00-representative-mir-programs.md
 M specs/examples/01-current-l2-surface-syntax-candidates.md
?? docs/reports/0041-review-current-l2-non-admissible-skip-surface.md
```

- `git diff --stat -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md`

```text
 specs/10-open-questions.md                                | 4 +++-
 specs/12-decision-register.md                             | 3 ++-
 specs/examples/00-representative-mir-programs.md          | 7 +++++--
 specs/examples/01-current-l2-surface-syntax-candidates.md | 7 ++++++-
 4 files changed, 16 insertions(+), 5 deletions(-)
```

- `git diff --check -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md`

```text
[no output]
```

- `python3 scripts/new_report.py --slug review-current-l2-non-admissible-skip-surface`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0041-review-current-l2-non-admissible-skip-surface.md
```

## 6. Evidence / findings

no findings

- 観点 1:
  [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L80) から [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L83)、[specs/examples/01-current-l2-surface-syntax-candidates.md](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L124) から [specs/examples/01-current-l2-surface-syntax-candidates.md](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L133)、[specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L228) から [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L231)、[specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L348) から [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L355) は一貫して、option miss 自体では explicit failure も dynamic `Reject` も立てず、request-level outcome だけを event surface に残すと書いている。これは [specs/04-mir-core.md](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L152) から [specs/04-mir-core.md](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L154) の dynamic `Reject` 境界と矛盾しない。
- 観点 2:
  [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L82) と [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L44) は、大分類を `non-admissible reason` に留めつつ、`admit-miss` / `lease-expired` を識別できれば十分と書いている。field 名、reason code 名、serialization は未決定のままで、current L2 に必要な最小区別以上へ踏み込んでいない。
- 観点 3:
  [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L83) と [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L44) は、dedicated skip event への昇格要否と capability mismatch を含む final taxonomy を明示的に未決定として残している。今回の差分は current L2 の最小 reading を追加しただけで、final taxonomy を固定していない。

## 7. Changes in understanding

今回の差分は failure lattice の再設計ではなく、current L2 examples の観測面を「request-level outcome は event surface、option-level miss は audit metadata」という最小読解へそろえる整理だった。未決定として残すべき範囲も、今回の文言では維持できている。

## 8. Open questions

- capability mismatch を将来 `non-admissible reason` の正式 subreason に含めるかどうか
- dedicated skip event が将来必要になる条件をどの decision で固定するか
- audit metadata の key 形状をどの段階で明文化するか

## 9. Suggested next prompt

current L2 の `non-admissible reason` audit metadata について、`option ref`、`request ref`、`reason kind`、`subreason` のうち何を最小必須 field とみなすかだけを整理してください。event surface は増やさず、E3 variant と E6 を両方説明できる最小 shape に限定してください。
