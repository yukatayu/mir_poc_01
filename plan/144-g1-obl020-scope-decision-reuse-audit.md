# plan/144 - G1 OBL-020 scope decision reuse / unresolved-slot audit

## Purpose

This file is LAB repository memory.

It records that `plan/134-g1-obl020-scope-clarification-packet.md` is already
the controlling LAB packet for the OBL-020 full-row vs G1-supporting scope
question. This file prevents the candidate package named "OBL-020 full-row vs
G1-supporting scope decision packet" from being run a second time as a duplicate
scope matrix.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-020, does not prove OBL-020, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, does not select concrete `Config`, `StepLabel`, `StepFamily`,
`WellFormed`, `Step`, scheduler semantics, per-step lemmas, runtime behavior,
Core IR, public API, grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is an
anti-duplication audit and unresolved-slot routing note, not a status
authority.

## Inputs

Canon authority:

- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`

LAB OBL-020 status-prep memory:

- `plan/78-g1-obl020-lean-statement-draft.md`
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/134-g1-obl020-scope-clarification-packet.md`
- `plan/135-g1-obl020-artifact-identity-wrapper-preflight.md`
- `plan/136-g1-obl020-artifact-annex-template.md`
- `plan/141-g1-status-packet-shell-unresolved-slots.md`
- `plan/142-g1-status-packet-shell-evidence-dry-run.md`
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`

Advisory inputs:

- read-only sidecar mapper `019f2da2-5c01-7f53-89e1-20e80d3e33fe`
- Oracle consult `mirrorea-obl020-scope-decision`

## Audit status

This audit is not a second OBL-020 scope clarification packet.

It says:

1. `plan/134` already contains the OBL-020 scope decision surface.
2. `plan/135` and `plan/136` move downstream into artifact identity and
   annex-template work; they do not reopen scope.
3. `plan/141` and `plan/142` correctly keep OBL-020 requested status, ledger
   delta, artifact identity, wrapper need, and scope unresolved.
4. A future autonomous package should not repeat the scope matrix unless the
   user explicitly promotes a human/canon-facing decision request.

## Controlling OBL-020 scope packet

The controlling LAB scope packet is:

```text
plan/134-g1-obl020-scope-clarification-packet.md
```

Its scope options remain the current LAB reading:

| Option | Current LAB reading |
|---|---|
| Full-row OBL-020 status movement | Deferred. The current abstract LAB draft is too weak to imply full step-rule WF coverage, concrete WF clauses, scheduler boundaries, or per-step proof obligations. |
| G1-supporting statement scope | Advisory recommendation. The current abstract statement shape may be reviewed as G1-supporting statement-scope evidence only. |
| Proof-package fallback | Fallback if human/canon review rejects scoped statement identity or requires concrete proof-package binding first. |

This file does not change those readings.

## Why not write a second scope matrix

A new full scope matrix would mostly duplicate `plan/134` and create drift
risk:

- two files could appear to own the same decision;
- a later status packet might cite the newer duplicate and ignore the older
  exclusions;
- readers could infer that OBL-020 scope has become more accepted merely
  because the same recommendation was repeated after `plan/143`;
- a duplicate matrix could accidentally weaken the distinction between
  G1-supporting statement scope and full-row `MirCore.Step.WF` status movement.

Therefore this file only records reuse and unresolved-slot routing.

## Current unresolved slots

These slots remain unresolved and must be carried by any later packet.

| Slot | Current reading |
|---|---|
| OBL-020 scope | `plan/134` recommends G1-supporting scope for review, keeps full-row status movement deferred, and keeps proof-package fallback available. Human/canon acceptance remains unresolved. |
| Requested status | No `stated`, `lean-stated`, `lean-proved`, `external`, or other requested status is chosen here. |
| Ledger delta | No ledger delta text exists here; the canon ledger remains unchanged. |
| Artifact identity | `plan/135` / `plan/136` keep direct LAB artifact citation, wrapper requirement, and deferral as unresolved review choices. |
| Abstract WF vocabulary | No decision here says abstract `WellFormed`, `Step`, and `PreservesWF` are sufficient for ledger movement. |
| Concrete runtime model | Concrete `Config`, `StepLabel`, `StepFamily`, `WellFormed` clauses, `Step` relation, scheduler semantics, and per-step preservation lemmas remain unchosen. |
| Proof state | No proof skeleton completion, proof discharge, or `lean-proved` claim. |
| Conformance / runtime | No C-static, C-runtime, C-distributed, dispatch, store mutation, request serving, occurrence ordering, admission lifecycle, stale-membership runtime failure, transport, or runtime scheduling claim. |
| Gate state | No G1 exit and no T0 -> T1 transition. |

## Decision routing

Use this routing after `plan/144`:

| Future need | Route |
|---|---|
| Human/canon wants to decide OBL-020 scope | Extract a review-facing decision request from `plan/134`; do not rewrite the scope matrix. |
| Human/canon wants to decide OBL-020 artifact identity | Use `plan/135` / `plan/136`; keep scope label and non-claims from `plan/134`. |
| A status packet is explicitly promoted | Use `plan/141` shell slots and `plan/136` annex slots; fill requested status / evidence only in that later package. |
| Concrete proof boundary is needed | Open a proof-package or statement-refinement package, not a docs-only scope duplicate. |
| No human/canon review is promoted | Remove "OBL-020 scope decision packet" from default candidate next packages and keep OBL-020 scope as already clarified by `plan/134`. |

## Current LAB recommendation

The current LAB recommendation is:

- Do not create another OBL-020 full-row vs G1-supporting scope packet.
- Treat `plan/134` as the current scope clarification.
- Keep `plan/135` / `plan/136` as artifact-identity / annex-template memory.
- Keep `plan/141` status-shell slots unresolved.
- Replace the stale candidate-next-package entry with a narrower review-facing
  extraction option that only runs if human/canon review is explicitly
  promoted.

This recommendation is advisory. It is not canon acceptance.

## Overclaims to avoid

- Do not say OBL-020 is `stated`, `lean-stated`, `lean-proved`, complete, or
  accepted.
- Do not say the LAB constant
  `MirCore.Lab.OBL020.StatementDraft.OBL020StatementDraft` is the canon
  `MirCore.Step.WF` target.
- Do not treat `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
  as proof, conformance, runtime determinism, or G1 exit evidence.
- Do not treat `FamilyStepPreservesWF` as full final step-family coverage.
- Do not create or require a wrapper here.
- Do not freeze concrete WF clauses, scheduler semantics, step taxonomy,
  runtime API, public API, grammar, or conformance profile.

## Stale wording note

Earlier LAB files may still contain historical "future draft" or "names
undecided" wording from before the OBL-020 Lean artifact existed. When touched,
read that wording through the later state:

- `plan/78` created the OBL-020 LAB Lean statement-shape artifact;
- `plan/134` clarified scope;
- `plan/135` / `plan/136` prepared artifact identity and annex-template
  memory;
- `plan/141` / `plan/142` kept status-shell and validation evidence separate.

This note does not require a broad historical rewrite.

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status chosen or accepted.
- No status proposal submitted.
- No metatheory ledger movement.
- No OBL-020 completion.
- No OBL-020 proof skeleton completion.
- No OBL-020 proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file.
- No Lean predicate refinement.
- No new executable row.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, source-map ABI, conformance profile, or step-family taxonomy
  freeze.
- No sample status relabel.
- No G1 exit by implication from scope reuse.

## Next allowed moves

Reasonable next packages are:

1. prepare an OBL-020 review-facing decision-request extraction only if the
   user or human/canon process explicitly promotes OBL-020 scope review;
2. prepare a status proposal draft only after requested status, ledger delta,
   artifact identity, wrapper need, OPEN-014 handling, OBL-020 scope, OBL-021
   abstraction boundary, and fresh validation slots are deliberately filled;
3. otherwise move to a different non-duplicate blocker, such as OBL-001
   artifact identity / wrapper acceptance review, OBL-001 sync guard
   hardening if a real drift risk is found, or another explicitly promoted
   current `tasks.md` candidate.

## Close condition

This file is closed when it is registered in the plan/source-hierarchy
scaffolds, `plan/00-index.md`, `plan/90-source-traceability.md`,
`Documentation.md`, `progress.md`, `tasks.md`, and the package report are
synchronized.

Close condition is anti-duplication / unresolved-slot audit only: no canon
edit, no gate exit, no requested status choice, no OBL status movement, no
proof, no conformance claim, no implementation change, and no runnable sample
status change.
