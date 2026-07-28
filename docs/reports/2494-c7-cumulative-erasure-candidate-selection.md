# Report 2494 - C7 cumulative-erasure countermodel candidate selection

**Identifier:** `LAB-REPORT-2494`
**Date:** 2026-07-28 15:22 JST
**Status:** selection package prepared; commit/push pending

## Objective

Determine whether a post-WRK-0035 C7 candidate can remain inside ADR-0014's
existing-LAB-lane L3 route without turning a generic factorization result into
a Mir source rule or a semantic design decision.

## Scope and assumptions

The selected `C7-CUM-PRE` candidate is a fixed finite, artifact-local
countermodel. It may establish only that individually safe erasures do not
generally justify their common coarsening for a paired observation. Local types
and functions have no Mir meaning. No Lean evidence source is written or run in
this selection package.

## Start state / dirty state

Start point was clean `main` at `91912c18a8065310e427c1bcf3200fafbc0b7b75`,
equal to `origin/main`, after WRK-0035 evidence, Canon metadata, and LAB
snapshots were committed and pushed.

## Documents consulted

- `AGENTS.md`, Canon README/MAP, ADR-0014, and `working/README.md`
- theory/03, P012, Plans 199, 200, 204, and 205; WRK-0035
- current `Documentation.md`, project status, progress, tasks, and samples dashboard
- current-cut duplicate searches and the temporary Oracle advisory review

## Actions taken

1. Re-read the C7 acceptance/matrix consumer and the C0--C6 stop boundaries.
2. Searched for multi-erasure, common-coarsening, mutual-omission, and
   composition-equivalent retained results at the current cut.
3. Compared a fixed cumulative-erasure countermodel against no-candidate,
   full-codomain reconstruction nonexistence, grounds restatements, and the
   reserved semantic frontiers.
4. Recorded the smallest candidate with an explicit consumer, falsifier,
   rollback, and one-file existing-lane evidence route.

## Files changed

- `plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `plan/204-wrk0034-semantic-composition-no-candidate-disposition.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- Current-cut authority/digest capture and resource audit
- Focused English/Japanese multi-erasure duplicate searches
- Canon/LAB C7 consumer and stop-boundary reads
- Temporary GPT-5.6 Sol Pro Oracle review with pinned source attachments
- `git diff --check` and `make docs`

## Evidence / outputs / test results

The English multi-erasure/common-coarsening search reported no retained match
outside existing single-erasure C7 records. The Japanese broad search hit only
an unrelated historical optional-notation discussion. Plan 199 explicitly
names a future C7 inference/desugaring matrix and source-to-elaborated evidence
equivalence; this is the candidate's only intended consumer.

The advisory Oracle output, checked locally rather than adopted as authority,
recommended the same finite countermodel but identified its decisive risk: it
is theorem churn if the future matrix already requires direct checking of every
cumulative representation. Plan 206 makes that risk a pre-registration stop
line and retains no Lean result yet.

## What changed in understanding

WRK-0035's single-erasure theorem is not enough to justify composing separate
ergonomic omissions. A finite negative example can make this missing guard
explicit without deciding what a Mir omission is. The project still lacks the
grounds, artifact, and source-contract work required for practical ergonomics.

## Open questions

- Whether the registered finite model compiles without imports, classical
  machinery, quotient, or a hidden generalization.
- Whether its common-coarsening equations and matrix consumer remain distinct
  from WRK-0035's single collision at evidence time.
- Whether any subsequent frontier remains after this bounded result; C0-D,
  C1, C2-B, C3/C4/C5 proper, and C6 remain reserved or duplicate-prone.

## Suggested next prompt

Create and push only the C7-CUM-PRE L3 pre-registration, then execute its fixed
finite countermodel exactly as registered. Freeze instead of repairing it if a
registered falsifier occurs.

## Plan update status

更新済み: Plan 206 records the selection; Plans 199, 200, and 204 identify it
as a C7 matrix guard rather than semantic composition or source authorization.

## Documentation.md update status

更新済み: reader-facing navigation now distinguishes the selected cumulative
countermodel from the already retained WRK-0035 evidence.

## docs/project-status.md update status

更新済み: current status names C7-CUM-PRE as a pre-registration candidate only
and keeps official T0 / OBL / Gate / Phase status unchanged.

## progress.md update status

更新済み: logical and research rows plus the timestamped log record the new
selection, its countermodel class, and its non-effects.

## tasks.md update status

更新済み: package 5 identifies C7-CUM-PRE registration as the next autonomous
task and retains the C3/C4/C5 ordinary-design stop line.

## samples_progress.md update status

更新不要: no active sample root, runnable command, debug surface, or workflow
readiness changed.

## Reviewer findings and follow-up

The temporary Oracle review was advisory and recommended only the finite
common-coarsening countermodel, conditioned on a current duplicate search and
the Plan 199 matrix consumer. Its strongest objection, possible theorem churn,
is a registered stop line. No callable sub-agent session was available.

## Skipped validations and reasons

No Lean source may be created or run before the selected L3 record is committed
and pushed. No sample/parser/checker/runtime validation applies because no
runnable workflow or implementation surface changed.

## Commit / push status

Pending selection commit, push, fetch, and `HEAD == origin/main` verification.

## Sub-agent session close status

No callable sub-agent session was opened.
