# Report 2478 - V1/R1 presentation-refinement selection

**Identifier:** `LAB-REPORT-2478`
**Date:** 2026-07-28 12:07 JST
**Status:** completed selection package; no model registered or executed

## Objective

Re-screen the post-WRK-0032 frontier without assuming that C3/C4/C5 proper can
be autonomously designed. Select at most one existing-lane L3 candidate that
preserves the recorded V1/R1 direction and keeps explicit semantic facts
separate from later ergonomic inference.

## Scope and assumptions

Canon in `mirrorea_canon/` is normative. `plan/`, snapshots, and this report
are LAB evidence. The package is a candidate selection only: it does not
create a working record, run a Lean model, alter a Canon rule, select a shared
carrier, or advance any lifecycle state. ADR-0014's pre-registration remains a
hard prerequisite for outcome evidence.

## Start state / dirty state

The start point was clean at `f98f81c3` on `main`, equal to `origin/main`.
WRK-0032 had retained only P012's conditional-A2 direction and four named
ordinary-admission literal non-matches. No V1/R1 presentation-comparison
working record or retained model was present.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and `CANON.md`
- `mirrorea_canon/adr/ADR-0014.md`
- `mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md`
- `mirrorea_canon/working/WRK-0030-c2a-source-tagged-anti-collapse-vocabulary.md`
- `mirrorea_canon/working/WRK-0032-c5pre-ordinary-admission-issuance-guard.md`
- `plan/187-mircore-value-flow-and-occurrence-decision-packet.md`
- `plan/193-post-admission-validation-context-literature-and-counterexample-memo.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `plan/201-c5-a2-issuance-guard-candidate-selection.md`
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
- `.docs/oracle-chatgpt-pro-operations.md` and the local Oracle manual

## Actions taken

1. Re-read the source hierarchy, ADR-0014 standing predicate, P012's V1/R1
   wording, and C3's deferred boundaries.
2. Compared three possible frontier directions: a V1/R1 presentation
   refinement, M1 information-loss re-exploration, and SW1 interleaving.
3. Confirmed that Plan 193 already retains the M1 adversarial inventory and
   that SW1 would select C4's validation/mutation boundary.
4. Recorded Plan 202: a finite administrative-binding versus one-slot-machine
   presentation comparison, bounded by explicit matching, single-use, and
   failure assumptions.
5. Kept potential ergonomic inference conditional: only uniquely determined
   facts with a reconstructible elaborated basis may later be omitted.

## Files changed

- `plan/202-v1-r1-presentation-refinement-candidate-selection.md`
- `plan/00-index.md`
- `plan/199-selected-semantic-composition-and-inference-boundary.md`
- `plan/200-reanchored-semantic-composition-research-plan.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- this report

## Commands run

- `git status --short`, `git diff --stat`, and branch comparison
- source and frontier `rg` queries over Canon, `plan/`, reports, and active
  evidence roots
- focused reads of P012, ADR-0014, Plans 187/193/199/200/201, WRK-0030, and
  WRK-0032
- one temporary Oracle review plus one retry after the first session's concrete
  zombie error
- documentation validation commands recorded below

## Evidence / outputs / test results

Local evidence supports a limited result: P012 explicitly permits an
evaluation-frame or machine-state presentation only as an equivalent
presentation of V1, while R1 requires a matching typed receipt before
resumption. Plan 187 records the required equivalence obligations. No existing
working record was found for the exact finite presentation comparison.

The first Oracle temporary session ended after one hour with the documented
zombie error and produced no answer. A single reduced retry completed. Its
advisory result agreed that V1/R1 presentation refinement is the least
committal remaining candidate, and identified M1 duplication and SW1 semantic
selection risks. The advice was checked against the cited local evidence; no
raw external transcript is committed.

No Lean source, Lean command, sample, helper, schema, or model output was
created or executed. The next model action is intentionally blocked until a
separate WRK-0033 pre-registration is committed and pushed.

`git diff --check`, `python3 scripts/validate_docs.py`, and
`python3 scripts/check_source_hierarchy.py` passed on the selection diff.
`make docs` also passed: Canon index check reported 119 indexed files and the
source-hierarchy check found all 752 required paths.

## What changed in understanding

The apparent C3 block is not all-or-nothing. C3 proper still requires an
unselected pending unit, correlation relation, held linear context,
success/failure transition, persistence rule, and source-elaboration relation.
However, P012 permits a much smaller presentation-level comparison that treats
all of those names as opaque LAB assumptions. Its useful negative result would
be precise: dropping matching, one-shot consumption, or failure exclusion
changes observable classification, so those facts cannot later disappear as
unjustified source inference.

## Open questions

- Whether the WRK-0033 standing predicate remains satisfied after exact source
  and LAB digests are pinned.
- Whether the finite model can express the two presentations and each required
  counterexample without smuggling in a semantic carrier.
- C3 proper's correlation, pending-control, failure, persistence, and
  source-elaboration design remain unresolved.
- C7 inference/desugaring remains deferred; this package only supplies a
  possible reconstruction criterion.

## Suggested next prompt

Continue with the standalone WRK-0033 pre-registration, commit and push it
before materializing or checking the finite Lean model, then retain the exact
conditional result or frozen falsifier in the existing LAB lane.

## plan/ update status

Updated: Plan 202 records the selection; Plans 199/200 point to the narrow
candidate without claiming C3 proper is unblocked; the index registers Plan
202.

## Documentation.md update status

Updated to link the new LAB selection and preserve the Canon/LAB distinction.

## docs/project-status.md update status

Updated to identify the selected V1/R1 comparison and the mandatory WRK-0033
registration boundary.

## progress.md update status

Updated as a LAB snapshot: selected research candidate, no executed model, and
the current blocker are explicit.

## tasks.md update status

Updated: the next autonomous package is WRK-0033 registration; C3 proper
remains a later Canon-design boundary.

## samples_progress.md update status

Update unnecessary. No runnable sample, active root, command, debug surface,
or sample blocker changed.

## Reviewer findings and follow-up

Oracle advisory review was used once successfully after one concrete failed
session. The successful review's narrow recommendation was adopted only where
local source evidence supports it. No callable sub-agent interface was
available in this task environment. Unrelated global Oracle sessions were not
inspected or used.

## Skipped validations and reasons

No Lean validation was run by design: ADR-0014 requires the standalone
pre-registration to be committed and pushed before outcome commands. No build
or sample run is relevant because no executable surface changed. The
documentation build was run; the skipped category is executable build/sample
validation only.

## Commit / push status

Pending at report creation. The package will be committed with `--no-gpg-sign`,
pushed, and checked against `origin/main` after validation.

## Sub-agent session close status

No callable sub-agent session was available or opened. The temporary Oracle
retry completed and has no required follow-up session.
