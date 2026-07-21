# WRK-0001 pilot checkpoint

## Purpose

This LAB checkpoint closes the first bounded L3 pilot as a research-process
exercise. It records validation, review, terminology limits, and the next
candidate class. It is not canon, a Gate/Phase action, a theorem-ledger change,
or a promotion request.

## Evidence chain reviewed

The reviewed sequence is linear and pushed:

1. `3e263f72731a32e2ca0ed549a873da5bb33d92ad` pre-registers WRK-0001.
2. `887a0f6cd2de57443f4508c14fbadf4a88f25992` retains the exact Lean
   reproduction evidence in the permitted `plan/` root.
3. `ef5dfbbb15dc6af3b303df2fd4a45d021b9721ad` manifests that commit and its
   artifact hash in the WRK record.

A clean disposable detached worktree at `ef5dfbbb` passed
`python3 scripts/validate_docs.py --authoritative-working-annex` and `make docs`.
The ordinary worktree correctly rejected ignored local configuration and local
artifacts in authoritative mode; that rejection is expected and is not evidence
of a WRK failure.

## Cross-cut review

An independent read-only reviewer confirmed the registration/evidence/manifest
ordering, pinned input hashes, append-only evidence ownership, L3
`not-promoted` boundary, and status/dashboard consistency. It found no L0/L1,
Gate, Phase, SCN, `theory/11`, conformance, or production claim movement.

The reviewer identified one wording risk: the remote-call budget uses
unbounded `Nat`. The pilot therefore supports a finite lifetime/capture
fragment plus a simple numeric budget parameter. It does not establish finite
cardinality of every index. This clarification is recorded in the mutable
Results and review section of WRK-0001 without altering the immutable
pre-registered question.

## Validation record

At the checkpoint:

- the exact Lean compile and registered source audit had passed;
- `make check` passed, including index freshness, source hierarchy,
  documentation validation, and `cargo check`;
- `python3 -m unittest discover -s scripts/tests -p 'test_*.py'` exited 0;
- `python3 scripts/validate_docs.py --authoritative-working-annex` passed in
  the clean detached worktree; and
- the ordinary worktree's authoritative rejection was preserved as a local
  state diagnostic only.

None of these results changes the L3 `not-promoted` reliance status.

## Candidate horizon after the checkpoint

The browser-backed Oracle advisory compared several possible future targets and
recommended a narrow OBL-021 `CoreTermOf` projection-adequacy countermodel.
This is a **candidate class, not an adopted task**. It must first pass the
standing eligibility predicate, use a fresh WRK record, pin its own canon/LAB
inputs, and declare an alternative, falsifier, source scope, and non-claims.
The advisory cannot change OBL-021, select a premise, or amend WRK-0001.

No next candidate is pre-registered by this checkpoint. This is deliberate: a
new experiment must not inherit either the finite-index result or the Oracle's
advisory authority.

## Stop line

The first pilot checkpoint is complete. The next autonomous package, if
continued, is candidate eligibility and pre-registration only; it must not
begin an OBL-021 edit or outcome command until a distinct L3 record is
committed. L2 remains fail-closed pending an owner-authenticated trust anchor.
