# plan/146 - G1 OBL-001 explanation-boundary sync guard hardening

## Purpose

This file is LAB repository memory.

It records a narrow OBL-001 sync-guard hardening: the OBL-001 explanation file
now explicitly preserves four boundary facts, and the sync tests guard that
wording:

- it is LAB evidence outside `mirrorea_canon/`;
- it is not the canon `MirCore.Elab.Soundness (stmt)` artifact;
- it does not settle artifact identity or wrapper acceptance;
- OPEN-014 remains open.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-001, does not prove OBL-002, does
not create a proof skeleton, does not create a Lean wrapper file, does not
refine a Lean predicate, does not resolve OPEN-014, does not claim
conformance, and does not change runtime, transport, Core IR, public API,
grammar, Diagnostic / repair ABI, assignment taxonomy, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file records test-only
guard hardening for a LAB explanation boundary, not status authority.

## Trigger

After `plan/145`, the OBL-001 status-prep route is clear:

- `plan/137` controls the OBL-001 artifact identity / wrapper preflight;
- `plan/138` controls the OBL-001 artifact annex template;
- `plan/141` keeps the status-shell slots unresolved;
- `plan/145` prevents duplicate artifact-decision packets.

The remaining reserve risk was narrower: the OBL-001 sync test already guarded
the Lean body links and checked that the explanation says "not a proof
skeleton" and "not runtime dispatch", but it did not fail if the explanation
stopped preserving the LAB/canon hierarchy, the distinction between the LAB
artifact and the canon `MirCore.Elab.Soundness (stmt)` target, artifact /
wrapper non-acceptance, or OPEN-014 non-resolution.

That would allow explanation text to drift away from `plan/137` / `plan/138` /
`plan/145` while the sync guard still passed.

## Change

The package adds four explanation-boundary guards:

```text
LAB evidence outside `mirrorea_canon/`
not the canon `MirCore.Elab.Soundness (stmt)` artifact
does not settle artifact identity or wrapper acceptance
OPEN-014 remains open
```

and updates:

- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `samples/lean/lab-statements/obl001/THM001StatementDraft.md`

The explanation now says:

```text
This is LAB evidence outside `mirrorea_canon/`.
This is not the canon `MirCore.Elab.Soundness (stmt)` artifact and does not
settle artifact identity or wrapper acceptance.
OPEN-014 remains open.
```

This is deliberately a wording guard, not a Lean predicate change.

## TDD evidence

RED:

```bash
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl001_draft_body_keeps_assignment_soundness_boundary
```

failed because the new required phrase was absent from
`THM001StatementDraft.md`.

GREEN:

```bash
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl001_draft_body_keeps_assignment_soundness_boundary
```

passed after the explanation text was updated.

## Why this is not a status move

The guard does not say the LAB artifact is accepted, rejected, or wrapped.

It says only:

- the LAB artifact is not silently the canon ledger target;
- artifact identity remains a future human/canon review decision;
- wrapper acceptance remains unresolved;
- OBL-001 statement status remains open unless canon changes the ledger.

## Non-claims

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

## Next allowed moves

The specific OBL-001 explanation-boundary drift risk is closed.

Reasonable next packages are:

1. reopen OBL-001 sync guard hardening only if a fresh review finds another
   concrete drift path that the tests do not catch;
2. refine the OBL-001 statement draft only if review finds actual overfit or a
   missing predicate;
3. prepare a review-facing artifact decision request only if human/canon
   review is explicitly promoted;
4. otherwise move to another explicitly promoted current `tasks.md` candidate.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, `samples_progress.md`, and the
package report are synchronized.

Close condition is guard-hardening-only: no canon edit, no gate exit, no
requested status choice, no OBL status movement, no proof, no conformance
claim, no implementation change, no Lean wrapper, and no runnable sample status
change.
