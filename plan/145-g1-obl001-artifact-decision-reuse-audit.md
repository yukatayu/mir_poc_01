# plan/145 - G1 OBL-001 artifact decision reuse / unresolved-slot audit

## Purpose

This file is LAB repository memory.

It records that `plan/137-g1-obl001-artifact-identity-wrapper-preflight.md`
and `plan/138-g1-obl001-artifact-annex-template.md` already define the current
LAB decision surface for OBL-001 artifact identity, wrapper need, OPEN-014
deferral, and simple-assignment scope. This file prevents the candidate
"OBL-001 artifact identity / wrapper acceptance review" from being run as a
second preflight or a premature wrapper package.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-001, does not prove OBL-002, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, does not resolve OPEN-014, and does not change runtime, transport,
Core IR, public API, grammar, Diagnostic / repair ABI, assignment taxonomy, or
sample status.

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
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`

LAB OBL-001 status-prep memory:

- `plan/73-g1-obl001-lean-statement-inventory.md`
- `plan/74-g1-obl001-lean-statement-draft.md`
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md`
- `plan/124-g1-obl001-boundary-audit.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/137-g1-obl001-artifact-identity-wrapper-preflight.md`
- `plan/138-g1-obl001-artifact-annex-template.md`
- `plan/141-g1-status-packet-shell-unresolved-slots.md`
- `plan/142-g1-status-packet-shell-evidence-dry-run.md`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

Advisory inputs:

- read-only sidecar mapper `019f2db8-171e-7602-9e07-f405c1001fd9`
- Oracle consult `mirrorea-repo-advisory-check-source`

## Audit status

This audit is not a second OBL-001 artifact-identity preflight.

It says:

1. `plan/137` already contains the OBL-001 artifact identity / wrapper decision
   boundary.
2. `plan/138` already contains the non-applied OBL-001 artifact annex template
   and the later review slots.
3. `plan/141` and `plan/142` correctly keep OBL-001 requested status, ledger
   delta, artifact identity, wrapper need, OPEN-014, and assignment-scope
   decisions unresolved.
4. The `[UNRESOLVED]` slots in `plan/141` mean not accepted / not filled; they
   do not mean that OBL-001 lacks a controlling LAB routing surface.
5. A future autonomous package should not create a wrapper, duplicate the
   preflight, or fill the status shell unless the user or human/canon process
   explicitly promotes that review.

## Controlling OBL-001 artifact decision surface

The controlling LAB files are:

```text
plan/137-g1-obl001-artifact-identity-wrapper-preflight.md
plan/138-g1-obl001-artifact-annex-template.md
```

Their current decision options remain:

| Option | Current LAB reading |
|---|---|
| Direct LAB artifact accepted | A later proposal may ask review to cite `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` / `MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft` as the requested-status artifact. This is not accepted now. |
| Wrapper required | A later non-applied wrapper package may be opened only after review says the LAB namespace cannot be cited directly. |
| Artifact identity deferred | OBL-001 requested-status work waits for OPEN-014, simple-assignment scope, assignment taxonomy, or proof-boundary decisions. |

This file does not choose among those options.

## Why not write a second decision packet now

A new OBL-001 artifact decision packet would mostly duplicate `plan/137` and
`plan/138` and could create drift risk:

- two files could appear to own the same artifact-identity decision;
- a later packet might cite the newer duplicate and skip `plan/137` wrapper
  constraints;
- repeating that OBL-001 is the strongest later `lean-stated` candidate could
  look like status momentum or review acceptance;
- a wrapper-oriented packet could make a canon-facing Lean name look accepted
  before the ledger target mapping is accepted;
- a direct-citation packet could silently launder the `MirCore.Lab...`
  namespace into `MirCore.Elab.Soundness (stmt)`.

Therefore this file records reuse and routing only.

## Current unresolved slots

These slots remain unresolved and must be carried by any later packet.

| Slot | Current reading |
|---|---|
| Requested status | `lean-stated` is an advisory candidate from `plan/133`, not a chosen or accepted status. |
| Ledger delta | No ledger delta text exists here; the canon ledger remains unchanged. |
| Artifact identity | `plan/137` / `plan/138` keep direct LAB artifact, wrapper required, and deferral as unresolved review choices. |
| Wrapper need | No wrapper is created or required here. |
| OPEN-014 | Read materialization, cache, freshness, transport, projection, and observe-vs-read-request policy remain unresolved / deferred. |
| Assignment scope | Simple ordinary assignment is the current LAB scope; compound assignment and final assignment taxonomy remain unresolved. |
| Proof state | No OBL-002 proof skeleton completion, proof discharge, or `lean-proved` claim. |
| Conformance / runtime | No C-static, C-runtime, C-distributed, dispatch, store mutation, request serving, occurrence ordering, admission lifecycle, stale-membership runtime failure, transport, or runtime scheduling claim. |
| Gate state | No G1 exit and no T0 -> T1 transition. |

## Decision routing

Use this routing after `plan/145`:

| Future need | Route |
|---|---|
| Human/canon wants to decide OBL-001 artifact identity | Extract a review-facing decision request from `plan/137` / `plan/138`; do not rewrite the preflight or annex. |
| Human/canon requires a wrapper | Open a wrapper package with the shape-preserving constraints from `plan/137`; keep it non-applied until canon accepts it. |
| A status packet is explicitly promoted | Use `plan/141` shell slots and `plan/138` annex slots; fill requested status / evidence only in that later package. |
| OPEN-014 or assignment scope becomes a blocker | Open a separate OPEN-014 / assignment-scope package; do not hide it inside artifact identity. |
| No human/canon review is promoted | Remove "OBL-001 artifact identity / wrapper acceptance review" from default autonomous package flow and keep OBL-001 artifact identity as already preflighted by `plan/137` / `plan/138`. |

## Current LAB recommendation

The current LAB recommendation is:

- Do not create another OBL-001 artifact identity / wrapper preflight.
- Do not create a Lean wrapper file now.
- Treat `plan/137` as the current OBL-001 artifact identity / wrapper
  preflight.
- Treat `plan/138` as the current non-applied OBL-001 artifact annex template.
- Keep `plan/141` status-shell slots unresolved.
- Use a narrower review-facing extraction only if human/canon review is
  explicitly promoted.

This recommendation is advisory. It is not canon acceptance.

## Overclaims to avoid

- Do not say OBL-001 is `stated`, `lean-stated`, `lean-proved`, complete, or
  accepted.
- Do not say the LAB constant
  `MirCore.Lab.OBL001.StatementDraft.THM001StatementDraft` is the canon
  `MirCore.Elab.Soundness (stmt)` target.
- Do not treat
  `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean` as proof,
  conformance, runtime behavior, or G1 exit evidence.
- Do not infer that a wrapper is unnecessary or required merely because the
  annex template exists.
- Do not resolve OPEN-014, compound assignment, final assignment taxonomy,
  authority proof, diagnostic / repair ABI, public API, grammar, runtime, or
  conformance profile inside artifact identity wording.
- Do not treat the current sync guards as proof that implementation satisfies
  the abstract predicates.

## Stale wording note

Earlier LAB files may still contain historical "future draft" or "names
undecided" wording from before the OBL-001 Lean artifact, artifact preflight,
and annex template existed. When touched, read that wording through the later
state:

- `plan/74` created the OBL-001 LAB Lean statement-shape artifact;
- `plan/124` audited the OBL-001 boundary and found no predicate refinement
  needed for the then-current G1 bridge;
- `plan/137` clarified artifact identity / wrapper risk;
- `plan/138` prepared the non-applied artifact annex template;
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
- No OBL-001 completion.
- No OBL-002 proof skeleton completion.
- No OBL-002 proof discharge.
- No OBL-020 / OBL-021 completion.
- No OBL-024 / OBL-025 diagnostic or repair proof claim.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file.
- No Lean predicate refinement.
- No new executable row.
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
- No G1 exit by implication from artifact decision reuse.

## Next allowed moves

Reasonable next packages are:

1. prepare an OBL-001 review-facing artifact decision-request extraction only
   if the user or human/canon process explicitly promotes OBL-001 artifact
   review;
2. prepare a status proposal draft only after requested status, ledger delta,
   artifact identity, wrapper need, OPEN-014 handling, OBL-020 scope, OBL-021
   abstraction boundary, and fresh validation slots are deliberately filled;
3. otherwise move to a different non-duplicate blocker, such as a real
   OBL-001 sync-guard drift risk, OBL-020 / OBL-021 statement refinement only
   if review finds concrete overfit, or another explicitly promoted current
   `tasks.md` candidate.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is reuse-audit-only: no canon edit, no gate exit, no requested
status choice, no OBL status movement, no proof, no conformance claim, no
implementation change, no Lean wrapper, and no runnable sample status change.
