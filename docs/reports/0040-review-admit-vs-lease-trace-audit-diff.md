# Report 0040 — review admit vs lease trace audit diff

- Date: 2026-03-31T08:29:16.533542Z
- Author / agent: Codex (GPT-5)
- Scope: 未コミット差分のうち `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に限定した review
- Decision levels touched: L2 review only。spec 本文の編集は行っていない

## 1. Objective

current L2 の `admit` miss / `lease` expiry の trace-audit semantics に関する未コミット差分を review し、次を確認する。

- non-admissible skip を dedicated event にせず audit metadata に留める方針が canonical law、runtime `Reject`、explicit failure、underdeclared handling を壊していないか
- `admit` miss と `lease` expiry を同じ大分類の non-admissible reason に入れつつ subreason で分ける整理が E3 variant と E6 に自然か
- dedicated skip event や final reason taxonomy を未決定のまま保てているか

## 2. Scope and assumptions

- review 対象は user 指定の 4 ファイルの未コミット差分に限定した。
- 規範文書の読書順は `README.md`、`Documentation.md`、`specs/00`-`03`、`specs/09`、その後に関連する `specs/04`、`specs/05`、`specs/10`、`specs/12` を参照した。
- `specs/` は編集しないという user 指示を優先し、report のみ追加した。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/TEMPLATE.md`

## 4. Actions taken

1. 規範文書と関連 subsystem 文書を既定順で確認した。
2. 指定 4 ファイルの未コミット差分を `git diff` で抽出した。
3. `admit` miss / `lease` expiry / non-admissible reason / dedicated skip event に関する箇所を行番号付きで再確認した。
4. review 結果をこの新規 report に記録した。

Files changed

- `docs/reports/0040-review-admit-vs-lease-trace-audit-diff.md` を新規追加

## 5. Evidence / outputs / test results

Review result

- Medium: [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L43) の D-037 と [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L44) の D-038 が current L2 の観測方針について競合している。D-037 は「`admit` 不成立を trace / audit に独立 event として出すか、skip reason としてだけ残すかは未決定」と書いたままだが、D-038 は current L2 の最小読解として「dedicated skip event として event surface に上げず、audit / trace 側の non-admissible reason metadata に留める」と新たに定めている。`specs/10-open-questions.md` と example 文書は D-038 側に寄っているため、decision register の内部だけが二重状態になっている。これにより current L2 の canonical reading が「metadata-only で一旦読むが将来昇格は未決定」なのか、「まだ現時点でも二案未決定」なのかを読者が一意に取れない。user の確認観点 3 に対しては、この stale な D-037 文が残る限り、未決定性のスコープが不正確である。

No findings on the other review points

- `specs/10-open-questions.md:80-83`、`specs/examples/01-current-l2-surface-syntax-candidates.md:124-133`、`specs/examples/00-representative-mir-programs.md:228-231`、`specs/examples/00-representative-mir-programs.md:348-355` を読む限り、`admit` miss と `lease` expiry は request-level outcome の前段にある non-admissible reason metadata として扱われており、explicit failure は admitted option 実行後、dynamic `Reject` は admissible candidate 枯渇後にだけ立つ。したがって canonical law、runtime `Reject`、explicit failure の境界自体は壊していない。
- underdeclared handling を dynamic `Reject` へ押し込む変更は見当たらない。static evidence floor と underdeclared static error の扱いは既存記述のままである。
- `admit-miss` と `lease-expired` を subreason で分ける整理は、E3 variant の owner/delegated writer 例と E6 の write-after-expiry 例の説明に自然に乗っている。E6 の capability mismatch については final taxonomy へ収めるかが未決定のまま維持されている。

Commands run and exact outputs

```text
$ git status --short -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md docs/reports
 M specs/10-open-questions.md
 M specs/12-decision-register.md
 M specs/examples/00-representative-mir-programs.md
 M specs/examples/01-current-l2-surface-syntax-candidates.md
?? docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md
```

```text
$ git diff --check -- specs/10-open-questions.md specs/12-decision-register.md specs/examples/00-representative-mir-programs.md specs/examples/01-current-l2-surface-syntax-candidates.md
[no output]
```

```text
$ python3 scripts/new_report.py --slug review-admit-vs-lease-trace-audit-diff
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0040-review-admit-vs-lease-trace-audit-diff.md
```

Key inspection points

- [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L43)
- [specs/12-decision-register.md](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L44)
- [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L80)
- [specs/10-open-questions.md](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L83)
- [specs/examples/01-current-l2-surface-syntax-candidates.md](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L124)
- [specs/examples/01-current-l2-surface-syntax-candidates.md](/home/yukatayu/dev/mir_poc_01/specs/examples/01-current-l2-surface-syntax-candidates.md#L133)
- [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L228)
- [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L231)
- [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L348)
- [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L355)

## 6. What changed in understanding

- この差分は canonical law や failure lattice を動かす変更ではなく、current L2 examples の trace / audit 読みを「request-level outcome を event surface に残し、option-level miss は audit metadata に留める」方向へ寄せる整理であることが明確になった。
- 問題は新方針そのものではなく、decision register に旧 wording が残って current reading を曖昧にしている点だと分かった。

## 7. Open questions

- D-037 を runtime reading のみへ狭め、観測方針は D-038 に委ねる形へ整理するか。
- capability mismatch を将来 `non-admissible reason` taxonomy の正式 subreason に入れるか、それとも narrative audit explanation に留めるか。
- final reason taxonomy と field 名をどの decision で固定するか。

## 8. Suggested next prompt

`D-037` と `D-038` の競合を解消する最小修正案を作ってください。spec の意味は変えず、current L2 では metadata-only を採りつつ、dedicated skip event への将来昇格だけを未決定として残す wording に揃えてください。
