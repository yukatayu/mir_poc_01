# Report 2466 — Semantic-composition next-candidate screen

- Date: 2026-07-28 09:47 JST
- Author / agent: Codex
- Scope: Re-screen C0-C/C0-D, C1, C2-B, and C6 after WRK-0030, then select at
  most one non-duplicate L3 pre-registration candidate.
- Decision levels touched: LAB planning and current snapshots only; no Canon
  semantic selection.

## Objective

Choose the smallest next research package that can produce reversible evidence
without choosing request identity, snapshot semantics, scalar representation,
or totality/Diagnostic semantics.

## Scope and assumptions

`mirrorea_canon/` remains normative. The selection uses current source reading,
historical evidence only at its own cut, ADR-0014's standing predicate, and a
temporary independent Oracle review. The result is a planning disposition, not
a working-record result or implementation authorization.

## Start state / dirty state

Started clean at `1fe85acacaf2457a82c44a5faee6d66ecd657f83`, equal to
`origin/main`, after WRK-0030 metadata linkage and full `make docs` validation.

## Documents consulted

- Canon README/MAP, ADR-0014, working annex, theory/03, theory/10, specs
  01/02/03/07, P008/P012/P013/P015, SCN-02, and SCN-08.
- WRK-0024, WRK-0027, WRK-0028, WRK-0029, WRK-0030, Plans 199/200, current
  snapshots, and a temporary GPT-5.6 Sol Pro review.

## Actions taken

1. Compared each candidate against the exact bounded result already retained.
2. Checked whether a new L3 question can avoid a reserved semantic choice.
3. Selected only C0-C, narrowed to source-local Diagnostic references.
4. Deferred C0-D, C1, C2-B, and C6 for the reasons recorded below.

## Files changed

- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2466-semantic-composition-next-candidate-screen.md`

## Commands run

- Ordered Canon/LAB source reads, historical evidence inspection, source-cut
  provenance checks, and Git parity checks.
- One temporary Oracle consultation `c2a-next-candidate-screen-20260728` with
  the relevant Plan, WRK, and proposal sources attached.

## Evidence / outputs / test results

Both local reading and the independent review find C0-C conditionally eligible:
only source-local terminal/reject/`Diagnostic` wording and explicit references
may be recorded. C0-D repeats P008/WRK-0004-style separation unless it chooses
domain/equality/OBL identity. C1 moves beyond WRK-0024 only by selecting
snapshot/evaluation/fusion/pending semantics. C6 moves beyond WRK-0027 only by
selecting scalar/terminal representation. C2-B needs an unselected identity
ontology.

## What changed in understanding

The prior plan's phrase “each claimed stage” was too strong: stage membership
and reject domain are themselves open. C0-C is safe only as a source-reference
audit. R0 did not place C1 and C6 historical evidence on the same current cut.

## Open questions

- Does the current cut leave at least one C0-C observation after removing
  WRK-0028's already retained source-local rows?
- Can the C0-C record pin all inputs and remain within an existing LAB lane
  without referring to coverage or Diagnostic assignment?

## Suggested next prompt

Pre-register C0-C as a literal source-reference audit, then retain it only if
its registered source checks leave an independent non-substitution result.

## Plan update status

更新済み: Plans 199/200 now distinguish selected C0-C from deferred C0-D/C1/
C2-B/C6 and state the exact nonsemantic stop boundary.

## Documentation.md update status

更新不要: top-level reader navigation is unchanged.

## docs/project-status.md update status

更新済み: the human-facing current view now identifies C0-C as the immediate
research action and the other candidate families as deferred.

## progress.md update status

更新済み: logical-specification status, research task, timestamp, and recent
log now record the selected C0-C boundary.

## tasks.md update status

更新済み: the current task map now schedules C0-C pre-registration rather than
a generic re-screen.

## samples_progress.md update status

更新不要: no runnable sample, command, or dashboard evidence changed.

## Reviewer findings and follow-up

The temporary Oracle review ranked C0-C first but only as a source-local
reference audit; it explicitly rejected combining it with C0-D. It found C1,
C6, and C2-B to require semantic selection for a nontrivial next result. Local
source reading independently confirms the R0 overlap and the narrower C0-C
boundary. The advisory answer is distilled here; its raw transcript remains
outside the repository.

## Skipped validations and reasons

No Lean, parser, runtime, or sample execution is appropriate before a new C0-C
record is pre-registered. Documentation/index validation runs after this
selection package is committed and pushed.

## Commit / push status

Pending at report write. This selection package will be self-reviewed,
committed with `--no-gpg-sign`, pushed, and compared with `origin/main` before
C0-C pre-registration.

## Sub-agent session close status

No callable sub-agent session is available. The temporary Oracle review
completed and is closed; its source-bounded recommendation was checked locally.
