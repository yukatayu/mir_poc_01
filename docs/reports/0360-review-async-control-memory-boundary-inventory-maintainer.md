# Report 0360 — review async control memory boundary inventory maintainer

- Date: 2026-04-09 12:38 JST
- Author / agent: Codex
- Scope: current worktree の未commit変更に対する maintainer review。重点は `atomic_cut` 境界、Phase 4 / 5 送りの wording、low-level memory-order 非採用整理、tree-like bubbling の derived-view line、tasks / plan / progress / report の整合確認
- Decision levels touched: none; review only

## 1. Objective

current worktree の async control / memory-model boundary inventory 変更を review し、semantic regression や documentation drift が無いかを findings-first で確認する。

## 2. Scope and assumptions

- 規範判断の正本は `specs/` とし、`plan/` / `tasks.md` / `progress.md` / `docs/reports/` は mirror / repository memory / status snapshot として確認した。
- review 対象は current worktree の未commit変更であり、既存の committed 文書は comparison anchor として読んだ。
- この review task 自体では normative docs を更新しない。

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
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `docs/reports/0358-async-control-memory-boundary-inventory.md`
- `docs/reports/0359-review-async-control-memory-boundary-inventory.md`

## 4. Actions taken

1. required baseline docs と relevant plan/docs を AGENTS の順序に従って再読した。
2. current worktree の未commit差分を `plan/12` / `plan/13` / `plan/16` / `progress.md` / `tasks.md` / report 群について確認した。
3. `atomic_cut` の既存 line、shared-space registry line、proof boundary、scheduler deferred line、report policy と照合した。
4. review 結果をこの新規 report に記録した。

Files changed:
- `docs/reports/0360-review-async-control-memory-boundary-inventory-maintainer.md`

Commands run and exact outputs:

```text
$ date '+%Y-%m-%d %H:%M JST'
2026-04-09 12:38 JST

$ python3 scripts/new_report.py --slug review-async-control-memory-boundary-inventory-maintainer
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0360-review-async-control-memory-boundary-inventory-maintainer.md
```

## 5. Evidence / outputs / test results

Findings:

1. `progress.md` の mirror 更新が途中で止まっており、Task C 拡張後の snapshot と fully consistent ではありません。[tasks.md:17](/home/yukatayu/dev/mir_poc_01/tasks.md#L17) [tasks.md:131](/home/yukatayu/dev/mir_poc_01/tasks.md#L131) では主線と Task C が `async-control boundary` まで拡張されていますが、[progress.md:140](/home/yukatayu/dev/mir_poc_01/progress.md#L140) [progress.md:178](/home/yukatayu/dev/mir_poc_01/progress.md#L178) [progress.md:195](/home/yukatayu/dev/mir_poc_01/progress.md#L195) はまだ旧来の `static analysis / type / theorem prover boundary` のままです。summary と bottleneck は async-control を含めて更新済みなので、priority / chapter-progress / next-task の 3 箇所だけ snapshot drift が残っています。

2. report chain がまだ close しておらず、repo の report policy と相互整合が崩れています。[docs/reports/0359-review-async-control-memory-boundary-inventory.md:1](/home/yukatayu/dev/mir_poc_01/docs/reports/0359-review-async-control-memory-boundary-inventory.md#L1) は空の scaffold のままで、commit 対象に含めると report policy 違反です。加えて [docs/reports/0358-async-control-memory-boundary-inventory.md:23](/home/yukatayu/dev/mir_poc_01/docs/reports/0358-async-control-memory-boundary-inventory.md#L23) の consulted-doc list には、この repo が non-trivial task で mandatory にしている baseline docs (`specs/00`〜`03`, `specs/09`, `plan/00-index.md`) が反映されていないため、provenance record が review可能な形で完結していません。

No findings:

- `atomic_cut` を current core の place-local finalizing cut に留める line 自体は、[specs/04-mir-core.md:96](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L96) [specs/09-invariants-and-constraints.md:8](/home/yukatayu/dev/mir_poc_01/specs/09-invariants-and-constraints.md#L8) [specs/12-decision-register.md:17](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L17) [plan/04-core-semantics-current-l2.md:80](/home/yukatayu/dev/mir_poc_01/plan/04-core-semantics-current-l2.md#L80) と整合しており、今回の wording で壊れていません。
- higher-level async-control family を Phase 4 / 5 の docs-first inventory に送る整理は、[plan/12-open-problems-and-risks.md:310](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md#L310) [plan/13-heavy-future-workstreams.md:135](/home/yukatayu/dev/mir_poc_01/plan/13-heavy-future-workstreams.md#L135) [progress.md:22](/home/yukatayu/dev/mir_poc_01/progress.md#L22) のあいだで shared-space / proof boundary / scheduler deferred line と衝突していません。
- C++ 的 low-level memory-order family を immediate candidate にしない整理は、Mir core を膨らませない既存 line と矛盾していません。[plan/04-core-semantics-current-l2.md:22](/home/yukatayu/dev/mir_poc_01/plan/04-core-semantics-current-l2.md#L22) [plan/13-heavy-future-workstreams.md:148](/home/yukatayu/dev/mir_poc_01/plan/13-heavy-future-workstreams.md#L148)
- tree-like / leaf-to-root bubbling を derived execution / debug / explanation view に留める wording は、registry source-of-truth line と整合しています。[plan/16-shared-space-membership-and-example-boundary.md:173](/home/yukatayu/dev/mir_poc_01/plan/16-shared-space-membership-and-example-boundary.md#L173) [plan/16-shared-space-membership-and-example-boundary.md:183](/home/yukatayu/dev/mir_poc_01/plan/16-shared-space-membership-and-example-boundary.md#L183)

Tests:

- docs review task のため追加テストは未実行。diff inspection と source comparison のみ実施。

Status notes:

- `plan/ 更新不要`
- `progress.md 要追補`
- `tasks.md 更新不要`

## 6. What changed in understanding

- semantic line 自体よりも、`progress.md` と report provenance の snapshot hygiene に drift が残っていることが main finding だった。
- async-control inventory の substantive wording は、shared-space registry line と proof-boundary line を崩さずに追加できている。

## 7. Open questions

- `docs/reports/0359-review-async-control-memory-boundary-inventory.md` を捨てるのか、この review 結果で埋めるのか。
- `progress.md` で async-control inventory を独立 row にするか、Phase 5 row の名称拡張に留めるか。

## 8. Suggested next prompt

`progress.md` の Phase 5 / Priority A / next-task wording を Task C と揃え、`docs/reports/0359-review-async-control-memory-boundary-inventory.md` を削除するか正式な review report として埋めるかを決めてください。
