# plan/147 - G1 next-line promotion-boundary audit

## Purpose

This file is LAB repository memory.

It records a narrow queue-boundary decision for the current G1 status-prep
line: a broad request to continue autonomously is not, by itself, a promotion
of either review-facing extraction candidate.

The current candidate rows remain candidates until the user explicitly chooses
the next line:

- `OBL-020 review-facing decision request extraction`
- `OBL-001 review-facing artifact decision request extraction`

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-001 / OBL-020 / OBL-021, does not
prove OBL-002 / OBL-020 / OBL-021, does not create a proof skeleton, does not
create a Lean wrapper file, does not extract a human/canon review request, does
not resolve OPEN-014, does not claim conformance, does not change runtime,
transport, Core IR, public API, grammar, Diagnostic / repair ABI, equality
relation, projection-totality, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file clarifies queue
routing, not canon status.

## Trigger

After `plan/144`, `plan/145`, and `plan/146`, the remaining top G1 candidate
rows in `tasks.md` are review-facing extraction rows marked `only if promoted`.

The user has also given a broad autonomous-work instruction for long-running
progress. That instruction authorizes continued work, validation, review,
reporting, commits, and pushes, but it does not name one of the two
review-facing extraction candidates.

The ambiguity is whether "continue autonomously" should count as package
promotion. It should not.

## Local evidence

- `tasks.md` says the candidate next strategy packages are candidates only and
  are not promoted until the user chooses the next line.
- `plan/144` says OBL-020 review-facing scope extraction is safe only if
  human/canon review is explicitly promoted.
- `plan/145` says OBL-001 artifact decision extraction is safe only if
  human/canon review is explicitly promoted.
- `plan/146` closes the concrete OBL-001 explanation-boundary drift risk and
  leaves review-facing artifact extraction as an explicitly promoted future
  option, not a default next package.
- `plan/141` keeps requested review, requested status, ledger delta, artifact
  identity, wrapper need, OPEN-014 handling, OBL-020 scope, OBL-021 abstraction
  boundary, proof, conformance, runtime, and G1 exit unresolved.

## Advisory review

Read-only sidecar review and ChatGPT Pro Extended Oracle review agreed on the
same narrow reading:

- broad autonomous delegation is not a specific next-line promotion;
- OBL-020 / OBL-001 review-facing extraction should not be inferred from it;
- the smallest safe next package is a queue-clarification / promotion-boundary
  audit;
- filling `plan/141` slots or extracting a review request would risk hidden
  promotion without an explicit user choice.

Those advisory inputs are evidence for the queue-boundary audit only. They do
not decide canon status.

## Post-P101 follow-up

After P100 added the `/tmp/mirrorea-*` helper and P101 registered the
storage/env helper surface in scaffold validators, a ChatGPT Pro Extended
follow-up reviewed whether another default autonomous package remained.

The advisory answer was:

- no default P102 should be manufactured only to keep the autonomous loop
  moving;
- another Macro 0 package is justified only if a fresh concrete drift trigger
  has already been found;
- otherwise the smallest safe next action is to stop package execution and ask
  the user to explicitly choose the next line:
  - `OBL-020 review-facing decision request extraction`;
  - `OBL-001 review-facing artifact decision request extraction`; or
  - a specific new Macro 0 audit trigger.

This follow-up reinforces the existing queue rule. It does not promote either
OBL extraction candidate, does not fill `plan/141` slots, and does not create a
new canon / ledger / proof / conformance / runtime / sample claim.

## Queue rule

Current rule:

```text
Broad autonomous delegation is not package promotion.

To promote a review-facing extraction package, the next user instruction must
name or unmistakably choose one of:

- OBL-020 review-facing decision request extraction
- OBL-001 review-facing artifact decision request extraction
```

If the user asks only to continue autonomously, the agent may:

- run validation and build/test front doors;
- inspect for concrete drift risks;
- update queue clarity, reports, and snapshot docs;
- prepare evidence that does not fill unresolved status-shell slots.

The agent must not infer:

- requested status selection;
- proposal submission;
- human/canon decision request extraction;
- artifact identity acceptance;
- wrapper acceptance or wrapper requirement;
- OBL scope acceptance;
- OPEN-014 resolution;
- ledger delta text;
- canon edit.

## Hidden failure modes

### Promotion laundering

Treating broad autonomous permission as a choice of OBL-020 or OBL-001 would
turn execution permission into a specific review/canon decision route.

### Review-request drift

A review-facing extraction can look like it fills the `Decision requested`
slot in `plan/141` even if the text says advisory-only.

### Unresolved-slot contamination

Extracting a question too early can make one unresolved axis look selected:
requested review scope, requested status, artifact identity, wrapper need,
OPEN-014 deferral, OBL-020 scope, or OBL-021 abstraction boundary.

### Status momentum

Repeating OBL-001 or OBL-020 decision surfaces after `plan/144` / `plan/145`
can make "candidate" or "G1-supporting" wording look increasingly accepted
without canon action.

### Wrapper pressure leak

OBL-001 and OBL-020 still leave direct LAB citation vs wrapper unresolved.
Neither "wrapper required" nor "wrapper unnecessary" should be inferred.

## Current allowed next moves

Without a new explicit user choice, reasonable autonomous work is limited to:

1. validation / build / execution sweeps that do not change status;
2. concrete guard hardening if a fresh drift path is found;
3. queue clarity / report / snapshot synchronization;
4. read-only review or Oracle consultation for difficult roadmap choices;
5. waiting for explicit user choice if the only remaining next moves are
   review-facing extraction packages.

With explicit user choice, either review-facing extraction may proceed:

1. `OBL-020 review-facing decision request extraction`, limited to extracting a
   human/canon-facing OBL-020 scope question from `plan/134`; or
2. `OBL-001 review-facing artifact decision request extraction`, limited to
   extracting a human/canon-facing OBL-001 artifact identity / wrapper /
   OPEN-014 / simple-assignment question from `plan/137` / `plan/138`.

Both routes must keep `plan/141` slots unresolved unless the same explicit
promotion says otherwise.

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No requested status chosen or accepted.
- No status proposal submitted.
- No metatheory ledger movement.
- No OBL-001 / OBL-020 / OBL-021 completion.
- No OBL-002 / OBL-020 / OBL-021 proof discharge.
- No OBL-024 / OBL-025 diagnostic or repair proof claim.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file.
- No Lean predicate refinement.
- No human/canon review request extraction.
- No OPEN-014 resolution.
- No G3 / THM-004 authority proof.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, source-map ABI, conformance profile, assignment taxonomy, or
  step-family taxonomy freeze.
- No sample status relabel.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
docs validators, current snapshot docs, `samples_progress.md` if needed, and
the package report are synchronized.

Close condition is queue-boundary-only: no canon edit, no status proposal, no
review request extraction, no requested status choice, no ledger movement, no
wrapper, no proof, no conformance claim, no implementation change, and no
runnable sample status change.
