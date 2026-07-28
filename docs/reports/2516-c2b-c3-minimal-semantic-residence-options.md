# 2516: C2-B/C3 Minimal Semantic-Residence Options

- Date: 2026-07-29
- Author / agent: Codex
- Scope: LAB ordinary-design decision preparation for the C2-B/C3 carrier gap.
- Decision levels touched: none; Canon remains unchanged.

## Objective

Reduce the first-card `CARRIER-GAP` to the smallest owner-facing alternatives
without silently selecting a Core, configuration, persistence, or source-level
contract.

## Scope and assumptions

This is a LAB comparison and recommendation. The request occurrence already
exists in the current history, but no source-visible, wire-visible, or
cross-load global identity is assumed. Canon and all implementation, OBL, Gate,
Phase, and public status remain unchanged.

## Start state / dirty state

Started from clean, remote-equal commit
`86c40f952ca74a82f7f339432549a011dd66a2a9`.

## Documents consulted

`AGENTS.md`; Canon README/MAP; ADR-0014; theory/01--05; P012; P013;
`OPEN-010`; `OPEN-011`; Plans 199, 200, and 214--218; `Documentation.md`;
`docs/project-status.md`; `progress.md`; `tasks.md`; the reporting template;
and the Oracle operating notes.

## Actions taken

Compared three semantic-residence designs: explicit relation-valued
configuration state anchored by an in-history request occurrence; history-only
projection; and a fresh nominal exchange identity. Recorded the first as the
LAB recommendation, the second as insufficient without hidden state, and the
third as a reserve that adds an unneeded identity contract. Defined a future
ergonomic-elaboration boundary: bookkeeping may be omitted only after a
complete unique elaboration can preserve every selected semantic fact.

## Files changed

- `plan/219-c2b-c3-minimal-semantic-residence-options.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `docs/reports/2516-c2b-c3-minimal-semantic-residence-options.md`

## Commands run

Read targeted Canon and LAB sources; inspected completed Oracle evidence and
Oracle session status; ran `df -h .`, `free -h`, `git diff --check`,
`make docs`, focused diff inspection, commit/push, and remote-equality checks.

## Evidence / outputs / test results

The prior completed carrier-neutral Oracle review remains advisory evidence at
SHA-256 `d496ba61d986013e25177e065cc1444365884de50421a20156adc2ad6967d502`.
A new temporary Oracle review was attempted, but all three browser invocations
failed before prompt submission (one Chrome disconnection and two missing
attach-metadata errors); it supplied no conclusion.

`git diff --check` passed. The first documentation run exposed two repository
discipline defects: the `progress.md` header lagged its new log timestamp, and
the preceding latest report lacked the validator's required Japanese
`docs/project-status.md` update declaration. The header was synchronized and
this new report supplies the declaration. Final `make docs` is run after this
report is present and passed: Canon index checked 126 files, source hierarchy
found all 761 required paths, and documentation validation completed.

## What changed in understanding

The gap is not solved by naming a request or by using queue/transport state.
The least additional semantic structure is an explicit relation-valued state
with a current-history request anchor, distinct owner outcome and requester
receipt observations, and restore correspondence. A future compact surface is
compatible with this, but only as a later proof-backed elaboration convenience.

## Open questions

Owner/Canon must decide whether to adopt the recommended relation state, its
minimal requester receipt transition, and its within-admissible-load one-shot
scope. Exact failure-receipt policy, M1 representation, provenance layout,
`Gamma`/`Delta` disposition, fallback interaction, and any source grammar stay
unresolved until that proposal supplies only the details needed by the choice.

## Suggested next prompt

Review the normal Canon proposal based on Plan 219 option A, keeping the
history-only and nominal alternatives as explicit rejected/reserve cases.

## Plan update status

`plan/` 更新済み: Added Plan 219 and synchronized Plans 199/200 plus the plan
index with the current owner-facing decision boundary.

## Documentation.md update status

`Documentation.md` 更新済み: Added the Plan 219 entry and its concise role.

## docs/project-status.md update status

更新済み: Replaced the stale C2-B/C3 gap-only snapshot with the current LAB
recommendation while keeping Canon selection explicitly 未選択.

## progress.md update status

`progress.md` 更新済み: Recorded the new logical-specification boundary,
owner decision, macro reading, and timestamped work log.

## tasks.md update status

`tasks.md` 更新済み: Replaced the generic C2-B/C3 carrier choices with the
three compared alternatives and the recommended normal Canon decision.

## samples_progress.md update status

`samples_progress.md` 更新不要: No runnable sample, validation command, debug
surface, or sample blocker changed.

## Reviewer findings and follow-up

The completed earlier Oracle review supports only the carrier-neutral method,
not this selection. The current Oracle request produced no answer because it
failed before submission. Local source review supplies the recommendation; a
normal Canon review remains required before any semantic change.

## Skipped validations and reasons

Lean and runtime suites were not applicable because no executable semantics or
sample changed. No applicable validation was intentionally skipped.

## Commit / push status

The design-evidence commit is
`1411f6b7e243a75b6d1765c874dff5506f4a8684`
(`docs: compare minimal C2-B/C3 semantic residence`). It was pushed to
`origin/main`; after `git fetch origin main`, `HEAD` and `origin/main` both
equaled that commit. This report is versioned and pushed in its task-closeout
Git history; its containing commit is the closeout identity.

## Sub-agent session close status

No callable sub-agent facility was available in this environment; no sub-agent
session was opened or left active.
