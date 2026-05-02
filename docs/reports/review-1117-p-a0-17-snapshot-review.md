# Report 1117 — P-A0-17 Snapshot Review

- Date: 2026-05-02 15:12 JST
- Author / agent: Codex
- Scope: snapshot/docs consistency review for `P-A0-17`
- Decision levels touched: none; review-only snapshot audit

## Objective

Review the requested snapshot and roadmap documents for the intended `P-A0-17` closeout shape, with emphasis on four stop lines:

- helper-local synthetic acceptance floor only
- no runnable/public/runtime promotion
- `reason_codes_scope` kept separate from any future accept-side scope
- large stage map percentages remaining explicit in snapshot docs

## Scope and assumptions

- This task is review-only; no normative or snapshot text is changed here apart from this required report.
- The current repository state is still the post-`P-A0-16` snapshot, so findings focus on drift risks and concrete inconsistency points to watch when `P-A0-17` is closed.
- Queue authority remains `progress.md` / `tasks.md`; `plan/` is treated as repository memory and `docs/reports/` as evidence.

## Start state / dirty state

- Started from a clean worktree.
- No pre-existing dirty changes were present in the repository root when this review began.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `samples/alpha/lifetime-fallback/README.md`
- `samples/alpha/contract-variance/README.md`
- `scripts/README.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `docs/reports/1116-p-a0-16-checker-widening-closeout.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`

## Actions taken

1. Read the required repository front-door, normative baseline, snapshot, sample-taxonomy, script-taxonomy, roadmap-memory, and prior closeout report documents in the repo-mandated order.
2. Traced every current `P-A0-16` / `P-A0-17` queue statement and every large-stage percentage mirror across snapshot docs.
3. Traced every current `reason_codes_scope` statement in snapshot and roadmap memory to identify where a future accept-side schema could be accidentally conflated with the negative-row checker floor.
4. Compared the alpha family READMEs against the mixed-state wording already used by `samples/alpha/README.md` and `samples/README.md`.
5. Recorded the concrete drift points that must move together when `P-A0-17` is closed.

## Files changed

- `docs/reports/1117-p-a0-17-snapshot-review.md`

## Commands run

```bash
wc -l README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/alpha/README.md samples/alpha/lifetime-fallback/README.md samples/alpha/contract-variance/README.md scripts/README.md plan/00-index.md plan/01-status-at-a-glance.md plan/39-type-system-freeze-roadmap.md plan/40-layer-compatibility-freeze-roadmap.md plan/43-alpha-e2e-roadmap.md docs/reports/1116-p-a0-16-checker-widening-closeout.md specs/00*.md specs/01*.md specs/02*.md specs/03*.md specs/09*.md
git status --short
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,240p' samples_progress.md
sed -n '1,220p' samples/README.md
sed -n '1,220p' samples/alpha/README.md
sed -n '1,220p' samples/alpha/lifetime-fallback/README.md
sed -n '1,220p' samples/alpha/contract-variance/README.md
sed -n '1,220p' scripts/README.md
sed -n '1,220p' plan/00-index.md
sed -n '1,240p' plan/01-status-at-a-glance.md
sed -n '1,220p' plan/39-type-system-freeze-roadmap.md
sed -n '1,220p' plan/40-layer-compatibility-freeze-roadmap.md
sed -n '1,260p' plan/43-alpha-e2e-roadmap.md
sed -n '1,240p' docs/reports/1116-p-a0-16-checker-widening-closeout.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
rg -n "P-A0-16|P-A0-17|reason_codes_scope|positive/accept|planned skeleton only|Current runners do not execute this family yet|active runnable root|large stage map" Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/alpha/README.md samples/alpha/lifetime-fallback/README.md samples/alpha/contract-variance/README.md scripts/README.md plan/01-status-at-a-glance.md plan/39-type-system-freeze-roadmap.md plan/40-layer-compatibility-freeze-roadmap.md plan/43-alpha-e2e-roadmap.md docs/reports/1116-p-a0-16-checker-widening-closeout.md
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- The current snapshot is internally consistent for post-`P-A0-16` state: `progress.md`, `tasks.md`, and `samples_progress.md` all mirror the same stage percentages (`B 40`, `C 35`, `D 35`, `E 60`, `F 60`) and the same queue state (`P-A0-16` closed, `P-A0-17` not promoted yet).
- `python3 scripts/check_source_hierarchy.py` passed with `required: 60`, `present: 60`, `missing: 0`.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1119 numbered report(s).`
- `git diff --check` passed.
- The strongest concrete drift risks for a future `P-A0-17` closeout are:
  - alpha family READMEs that still say `planned skeleton only` / `Current runners do not execute this family yet`
  - snapshot docs that currently hard-code `P-A0-16` as current and `P-A0-17` as not promoted
  - roadmap-memory docs that currently say positive/accept rows remain planned-only
  - scope wording that currently only defines `reason_codes_scope = alpha-static-floor`

## What changed in understanding

- The main risk is not current contradiction; it is partial update drift after `P-A0-17` if the helper-local acceptance floor is introduced without reclassifying the family READMEs from pure skeleton wording to mixed scaffold wording.
- The current documentation already has a good stop-line vocabulary for non-public/non-runnable/non-parser/runtime claims; the fragile part is keeping that vocabulary synchronized once positive rows are no longer purely planned.
- `reason_codes_scope` is consistently documented today as the negative-row checker-floor confinement key, so any accept-side carrier that reuses that label without an explicitly separate accept-side scope term would create immediate ambiguity.

## Open questions

- What exact name should the accept-side scope field use so it is visibly distinct from `reason_codes_scope` in docs and sidecars?
- Which positive rows, if any, are small enough for `P-A0-17` without forcing percentage changes beyond a helper-local floor reading?
- Should the family READMEs say `planned skeleton only` after `P-A0-17`, or should they move to an explicit `mixed scaffold + synthetic acceptance floor` wording?

## Suggested next prompt

Implement `P-A0-17` as a helper-local synthetic acceptance floor only, then update `progress.md`, `tasks.md`, `samples_progress.md`, `samples/alpha/*` READMEs, `samples/README.md`, `scripts/README.md`, and `plan/39/40/43` together so `reason_codes_scope` stays negative-floor-only, accept-side scope is named separately, and no runnable/public/runtime promotion language slips in.

## Plan update status

`plan/` 更新不要: this task only reviewed the existing repository-memory texts and identified future drift points.

## Documentation.md update status

`Documentation.md` 更新不要: this task only reviewed current snapshot wording.

## progress.md update status

`progress.md` 更新不要: this task did not change repo status.

## tasks.md update status

`tasks.md` 更新不要: this task did not change the queue.

## samples_progress.md update status

`samples_progress.md` 更新不要: this task did not change runnable sample state.

## Reviewer findings and follow-up

- No separate reviewer/sub-agent was invoked for this review-only audit.
- Follow-up is the finding list in the final response and the open questions above.

## Skipped validations and reasons

- No runtime/Cargo/sample execution floor was rerun because the task was a document review and did not modify executable behavior.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent sessions were opened.
