# Report 2556 — P017 X1 K0 typed terminal owner-service failure basis

- Date: 2026-07-30 09:34 JST
- Author / agent: Codex
- Scope: One Plan 233 terminal-failure fact-role comparison.
- Decision levels touched: LAB ordinary design; no Canon/OBL/Gate/Phase decision.

## Objective

Compare the smallest positive basis for typed terminal owner-service failure.

## Scope and assumptions

K0 V1/R1 only; no candidate basis is adopted. Plans 208--210/220 retain the
full failure/receipt/linearity/causality/load integration obligations.

## Start state / dirty state

`HEAD == origin/main == efce080a`; clean.

## Documents consulted

Canon P012/P013/P017, theory/02/04/05, ADR-0014; LAB Plans 208--210, 220,
233--237; temporary Oracle review `p017-final-b-role-preflight`.

## Actions taken

1. Preflighted the two remaining roles with an independent Oracle review.
2. Selected terminal failure because P017 items 2/3 and theory/02 supply an
   abstract, failure-specific consumer without choosing a row.
3. Added Plan 238: A direct typed terminal-failure view, B typed failure plus
   erasable terminality, C `OPEN`; A/A is advisory only.

## Files changed

- `plan/238-p017-x1-k0-terminal-failure-positive-basis-and-failure-nonconflation-card.md`
- `plan/00-index.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2556-p017-x1-k0-terminal-failure-positive-basis.md`

## Commands run

Source reads, clean-status inspection, one temporary Oracle review, `make
docs`, whitespace check, and a concrete Discord-webhook scan. Authoritative
validation and focused documentation unit tests remain for the clean worktree.

## Evidence / outputs / test results

P017 separates a terminal owner failure from outstanding/success and separately
requires a typed failure fact. theory/02 allows abstract row containment but
not a selected member/row. Hence typed terminal failure is smaller than a new
consulted-provenance linkage; all Plan 233 rows remain `OPEN`.

Documentation validation passed with Canon index `132`, source hierarchy
`788/788`, and `1710` numbered reports. Whitespace and concrete-webhook scans
had no findings.

## What changed in understanding

Failure typing and terminality can be compared without deciding failure delivery
or validation, but neither implies no mutation, provenance, or branch exclusion.

## Open questions

Consulted validation provenance, all failure semantics, rows, validation,
receipt, linearity, causality, and persistence remain open.

## Suggested next prompt

Preflight consulted validation provenance without duplicating P013/Plan 236.

## Plan update status

`plan/` updated: Plan 238 and index added.

## Documentation.md update status

`Documentation.md` updated: Plan 238 is described as an unadopted failure
basis comparison, not a failure-semantics selection.

## docs/project-status.md update status

更新済み: current status separates the advisory A/A comparison from the
unchanged all-`OPEN` ledger and K1 failure-row gap.

## progress.md update status

`progress.md` updated: the next boundary is consulted validation provenance;
terminal failure remains an unadopted advisory basis.

## tasks.md update status

`tasks.md` updated: Macro 1 now records Plan 238 and preserves its excluded
failure-row and integration surfaces.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable evidence changed.

## Reviewer findings and follow-up

Oracle recommends ordinary-card classification, A/A advisory, and C operative;
it forbids concrete rows, lifecycle, validation, receipt, and persistence.
No callable sub-agent interface is available.

## Skipped validations and reasons

No executable source changed; Lean/runtime/sample runs do not apply. The
authoritative and focused documentation tests remain for the committed clean
worktree.

## Commit / push status

Content is ready for the first commit with `--no-gpg-sign`; then run clean
worktree validation, record evidence, push, and verify remote equality.

## Sub-agent session close status

No sub-agent session exists; the temporary Oracle transcript remains external.
