# Report 0042 — review current l2 admit lease trace audit recheck

- Date: 2026-03-31T08:33:31.046384Z
- Author / agent: Codex (GPT-5)
- Scope: `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` の最新未コミット差分に限定した review
- Decision levels touched: L2 review only。spec 本文は編集していない

## 1. Objective

current L2 の `admit` miss / `lease` expiry の trace-audit semantics について、前回の唯一の finding だった D-037 / D-038 wording conflict が解消されたかを再確認し、最新未コミット差分が `no findings` かどうかを判定する。

## 2. Scope and assumptions

- review 対象は user 指定の 4 ファイルの未コミット差分に限定した。
- 読書順は repository rule に従い、`README.md`、`Documentation.md`、`specs/00`-`03`、`specs/09`、その後に関連する `specs/04`、`specs/05`、`specs/10`、`specs/12`、examples を参照した。
- user 指示に従い、spec 本文は編集せず、新しい report のみ追加した。
- review の主眼は semantic correctness、invariant 維持、trace / audit wording の整合性であり、style-only な指摘は finding に含めない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/TEMPLATE.md`

## 4. Actions taken

1. 規範文書と関連 subsystem 文書を既定順で読み、L2 の trace / audit 論点が Mir core の failure boundary と Mirrorea の audit responsibility をまたぐことを確認した。
2. 指定 4 ファイルの未コミット差分を `git diff` で抽出した。
3. D-037、D-038、`admit` miss、`lease` expiry、non-admissible reason、capability mismatch に関する現行文言を行番号付きで再確認した。
4. review 結果をこの新規 report に記録した。

Files changed

- `docs/reports/0042-review-current-l2-admit-lease-trace-audit-recheck.md` を新規追加

## 5. Evidence / outputs / test results

Review result

- No findings: D-037 は option-local `admit` の runtime reading に限定され、観測面は D-038 に委ねる形へ整理されている。これにより、前回 finding だった D-037 / D-038 の wording conflict は解消されている。
- `specs/10-open-questions.md:80-83` と `specs/12-decision-register.md:43-44` は、current L2 では `admit` miss / `lease` expiry を dedicated skip event にせず、request-level outcome を event surface に残しつつ audit / trace metadata で non-admissible reason を説明する、という同じ最小読解に揃っている。
- `specs/examples/01-current-l2-surface-syntax-candidates.md:124-133` と `specs/examples/01-current-l2-surface-syntax-candidates.md:339-345` は、`admit` miss を non-admissible skip とし、explicit failure を admitted option 実行後、dynamic `Reject` を admissible candidate 枯渇後へ保っている。runtime reading と観測面が D-037 / D-038 に一致している。
- `specs/examples/00-representative-mir-programs.md:226-232` と `specs/examples/00-representative-mir-programs.md:346-355` は、E3 variant で `admit-miss`、E6 で `lease-expired` を audit metadata として残し、event surface には request-level outcome だけを残す読みを具体例として支えている。
- capability mismatch については、E6 が narrative audit explanation に留めており、`specs/10-open-questions.md:83` の「同じ taxonomy にどう収めるかは未決定」と矛盾していない。
- `specs/04-mir-core.md` の current L2 fallback / `lease` 読解と照らしても、`lease` expiry を monotone degradation に伴う success-side choice 除外理由として扱う今回の wording は、explicit failure や dynamic `Reject` の境界を壊していない。

Commands run and exact outputs

```text
$ git status --short
 M specs/10-open-questions.md
 M specs/12-decision-register.md
 M specs/examples/00-representative-mir-programs.md
 M specs/examples/01-current-l2-surface-syntax-candidates.md
?? docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md
?? docs/reports/0040-review-admit-vs-lease-trace-audit-diff.md
?? docs/reports/0041-review-current-l2-non-admissible-skip-surface.md
?? docs/reports/0042-review-current-l2-admit-lease-trace-audit-recheck.md
```

```text
$ python3 scripts/new_report.py --slug review-current-l2-admit-lease-trace-audit-recheck
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0042-review-current-l2-admit-lease-trace-audit-recheck.md
```

Key inspection points

- `specs/12-decision-register.md:43`
- `specs/12-decision-register.md:44`
- `specs/10-open-questions.md:80`
- `specs/10-open-questions.md:83`
- `specs/examples/01-current-l2-surface-syntax-candidates.md:124`
- `specs/examples/01-current-l2-surface-syntax-candidates.md:133`
- `specs/examples/01-current-l2-surface-syntax-candidates.md:339`
- `specs/examples/01-current-l2-surface-syntax-candidates.md:345`
- `specs/examples/00-representative-mir-programs.md:226`
- `specs/examples/00-representative-mir-programs.md:232`
- `specs/examples/00-representative-mir-programs.md:346`
- `specs/examples/00-representative-mir-programs.md:355`

## 6. What changed in understanding

- 今回の差分は、新しい trace event を増やす変更ではなく、「current L2 の最小観測面では option-level miss を audit metadata に留める」という既存方針を D-038 に集約した整理として読めることが明確になった。
- D-037 を runtime reading 専用へ狭めたことで、前回問題だった「current reading 自体が二重に見える」状態は解消された。
- capability mismatch はまだ正式 taxonomy の外に置かれており、今回の差分はそこを premature に確定していない。

## 7. Open questions

- `admit-miss`、`lease-expired`、capability mismatch などを最終的にどの field 名 / reason code / serialization で固定するか。
- dedicated skip event を将来導入する必要が本当にあるか。
- capability mismatch を `non-admissible reason` taxonomy に正式編入するか、それとも audit explanation の別カテゴリに留めるか。

## 8. Suggested next prompt

current L2 の non-admissible reason taxonomy について、`admit-miss`、`lease-expired`、capability mismatch の 3 つをどう整理するかを、spec 本文はまだ固定しすぎない範囲で比較検討する report を作ってください。
