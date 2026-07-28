# 2514: C2-B/C3 Carrier-Neutral Conditional Comparison

## Objective

Define a bounded LAB comparison method for C2-B/C3 candidates without selecting
or accidentally introducing a semantic carrier.

## Scope and assumptions

This corrects LAB research framing only. Canon, Gates, Phases, OBL status,
implementation, grammar, runtime, API, and public behavior are unchanged.

## Start state / dirty state

Started from clean, remote-equal commit `373fd66925172a9ebed5e5699446759c180e7dee`.

## Documents consulted

`AGENTS.md`, Canon entry points and theory/01--06, `spec/04`, P008, P012,
P013, `OPEN-010`, `OPEN-011`, Plans 209, 215, 216, `Documentation.md`,
`progress.md`, and `tasks.md`.

## Actions taken

Ran an independent Oracle review, corrected structural bias in Plans 215/216,
and created Plan 217's candidate-native conditional comparison method.

## Files changed

- `plan/215-c2b-c3-ordinary-design-decision-packet.md`
- `plan/216-c2b-c3-cross-boundary-compatibility-audit.md`
- `plan/217-c2b-c3-carrier-neutral-conditional-comparison.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `docs/reports/2514-c2b-c3-carrier-neutral-comparison.md`

## Commands run

Oracle temporary-review command, source reads, `sha256sum`, `git diff --check`,
`make docs`, focused diff review, and commit/push/remote-equality checks.

## Evidence / outputs / test results

Oracle review SHA-256: `d496ba61d986013e25177e065cc1444365884de50421a20156adc2ad6967d502`.
`git diff --check` passed. `make docs` passed: Canon index checked 126 files;
source hierarchy found all 761 required paths; documentation validation passed.

## What changed in understanding

A common matrix is possible only as candidate-native observations and erased
definitions. A common request/pending/reply/receipt signature would already
select a structural architecture.

## Open questions

Exact candidate hypotheses, ADR-0014 treatment of hypothetical carriers,
reply/receipt occurrence carriers, and post-load identity remain open.

## Suggested next prompt

Prepare one source-ledgered candidate card only when its hypothesis delta does
not require a reserved semantic extension; otherwise record `CARRIER-GAP`.

## Plan update status

Updated Plans 215/216, added Plan 217, and updated indexes/cross-references.

## Documentation.md update status

Updated in this task with the carrier-neutral comparison entry.

## docs/project-status.md update status

Updated in this task with the current comparison boundary.

## progress.md update status

Updated in this task with the new research method and recent log.

## tasks.md update status

Updated in this task so the next package is a conditional card or gap report.

## samples_progress.md update status

Update not needed: no runnable sample, validation command, debug surface, or
sample blocker changed.

## Reviewer findings and follow-up

The first Oracle attempt failed before submission because attachments did not
finish uploading. The reduced retry completed and identified common-carrier,
forced-factorization, functional-projection, and at-most-one-receipt bias.

## Skipped validations and reasons

No executable semantics changed, so Lean and runtime suites are not applicable.
No required validation was skipped.

## Commit / push status

The evidence commit is `2e2d3615c184a33c7ae0c86eca5d99c44975984b`
(`docs: define carrier-neutral C2-B/C3 comparison`). It was pushed to
`origin/main`; after `git fetch origin main`, `HEAD` and `origin/main` both
equal that commit. This report is versioned and pushed in its task-closeout Git
history; its containing commit is the closeout identity.

## Sub-agent session close status

No callable sub-agent facility was available in this environment; no sub-agent
session was opened or left active.
