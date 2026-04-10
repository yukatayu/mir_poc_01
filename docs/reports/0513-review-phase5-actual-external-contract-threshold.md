# Report 0513 — review phase5 actual external contract threshold

- Date: 2026-04-10 17:12 JST
- Author / agent: Codex
- Scope: Phase 5 docs-only actual-external-contract-threshold package の maintainer-style review。対象は `specs/examples/198...`、`docs/reports/0512...`、top-level mirrors、`plan/` roadmap/risk/traceability docs、`progress.md`、`tasks.md`、research abstract
- Decision levels touched: none。review only

## 1. Objective

current Phase 5 docs-only package が

- semantic に単調接続しているか
- mirror snapshot に stale wording を残していないか
- traceability edge を欠いていないか
- report evidence と closeout claim が一致しているか

を source-backed に確認し、actual defect だけを記録する。

## 2. Scope and assumptions

- review scope は user 指定ファイルと、その整合確認に必要な必読 chain に限る。
- normative judgment の正本は `specs/` とし、`Documentation.md` / `plan/` / `progress.md` / `tasks.md` / reports は mirror / repository memory / status snapshot として読む。
- `plan/ 更新不要`
- `progress.md 更新不要`
- `tasks.md 更新不要`

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
- `docs/reports/0512-phase5-actual-external-contract-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Actions taken

1. repo rule に従って top-level docs と foundational spec chain を再読した。
2. scoped files を line-number 付きで確認し、`198` の判断と mirror / report wording を突き合わせた。
3. `python3 scripts/validate_docs.py` と `git diff --check` を実行し、report の validation claim を実測で確認した。
4. review record として本 report を追加した。review scope 内の package 自体は変更していない。

## 5. Files changed

- `docs/reports/0513-review-phase5-actual-external-contract-threshold.md`
- `plan/ 更新不要`
- `progress.md 更新不要`
- `tasks.md 更新不要`

## 6. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 17:12 JST

$ git status --short
 M Documentation.md
 M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M tasks.md
?? docs/reports/0512-phase5-actual-external-contract-threshold.md
?? docs/reports/0513-review-phase5-actual-external-contract-threshold.md
?? specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md

$ python3 scripts/new_report.py --slug review-phase5-actual-external-contract-threshold
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0513-review-phase5-actual-external-contract-threshold.md

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 512 numbered report(s).

$ git diff --check
[no output]
```

## 7. Evidence / outputs / test results

### Findings

1. `docs/reports/0512-phase5-actual-external-contract-threshold.md:84-87`, `progress.md:126`
   report hygiene が壊れている。`0512` は `python3 scripts/validate_docs.py` と `git diff --check` を command list に入れているのに、evidence section では両方を `pending rerun after file updates` のまま残している。一方 `progress.md` は同じ package を `review / mirror sweep / validation` 済みとして close しており、closeout claim と report evidence が衝突している。今回の実測では両コマンドとも通っているため、未更新 placeholder の取り残しである。
2. `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md:288-290`, `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md:184-186`
   Phase 5 abstract の unresolved list が stale である。abstract はまだ「boundary-specific handoff artifact row の次段として external-contract-facing handoff row をどの field で切るか」と「actual handoff pair shape を retained bridge のまま維持するか boundary-specific artifact row へ actualize するか」を open question に残しているが、これらは `197` / `196` までで既に close 済みで、`198` の current next question は consumer-specific external contract payload である。Phase 5 の quick recap が current promoted line より 2 段古い。
3. `plan/11-roadmap-near-term.md:110`, `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md:127-133`
   `plan/11` の working assumption が current package と噛み合っていない。`plan/11` は theorem-line retained bridge を「actual external contract とは区別する」と書くが、`198` では `retained_payload_body_materialization_external_contract` を retained bridge の最小 field として current first choice に追加している。ここで必要なのは「actual exported/finalized contract と混同しない」という注意であって、「actual external contract 自体を retained bridge に入れない」という読みではない。現 wording のままだと current Phase 5 line を誤読させる。

### Non-findings

- `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md` 自体には、scope 内で明確な semantic inconsistency は見当たらなかった。`retained_payload_body_materialization_external_handoff_row` の次段として `retained_payload_body_materialization_external_contract` だけを足し、consumer-specific payload を後段に残す cut は `specs/09` の explicit-boundary / no-overclaim line を壊していない。
- `plan/90-source-traceability.md:1461-1466` には `198` / `0512` の addendum があり、今回 scope 内で新たな traceability omission は見つからなかった。

## 8. What changed in understanding

- この package の主問題は `198` の threshold judgment 自体ではなく、Phase 5 mirror と closeout report の maintenance 漏れだった。
- traceability addendum は入っているので、今回の欠陥は missing edge というより stale mirror と unfinished evidence record に集中している。

## 9. Open questions

- `plan/11` の line は `actual external contract` ではなく `actual exported contract` あるいは `finalized consumer-specific contract` と書き換えるのが正しいか。
- Phase 5 abstract の unresolved list を current promoted line のみへ絞るか、直前で close した threshold も historical note として残すか。

## 10. Suggested next prompt

Address the Phase 5 actual-external-contract-threshold review findings: update `docs/reports/0512...` so its validation evidence matches the commands actually run, remove stale pre-198 open questions from `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`, and tighten `plan/11-roadmap-near-term.md` so the retained bridge is distinguished from finalized/exported contract rather than from the minimal actual external contract field added in `198`.
