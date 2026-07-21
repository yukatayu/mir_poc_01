# WRK-0001 finite-index reproduction evidence

## Purpose

This document is retained **LAB evidence** for the committed L3 working record
`mirrorea_canon/working/WRK-0001-finite-index-boundaries.md`. It records one
reproduction of an existing helper-local Lean fragment. It is not canon, a
language-design decision, a proof-ledger update, a conformance result, or a
promotion request.

## Pre-registered boundary

The registration commit is `3e263f72731a32e2ca0ed549a873da5bb33d92ad`.
It pins `theory/02-types-effects-failures` and the following existing LAB
inputs at parent `032a0ac22964b053aa0af29b0ab1a928de88d423`:

- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- `samples/lean/manifest.json`

The working question is limited to the fragment's three already-permitted
finite shapes: a two-point lifetime preorder, finite capture-set inclusion,
and a natural-number remote-call bound. The associated non-claims, alternative,
and falsifier remain exactly those in WRK-0001.

## Commands and result

Executed on 2026-07-21 17:17 JST:

```text
lean --version
lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
python3 -c "from pathlib import Path; text = Path('samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean').read_text(); required = ('step_does_not_outlive_session', 'ephemeral_only_not_subset_of_empty', 'room_history_only_not_subset_of_ephemeral_only', 'zero_budget_rejects_remote_call'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"
```

Observed result:

- Lean was `4.29.1` (release commit
  `f72c35b3f637c8c6571d353742168ab66cc22c00`).
- The direct file check exited `0` with no stdout or stderr.
- The source audit exited `0`: it found all four named lemmas and none of the
  six prohibited placeholder or implementation-escape tokens.

The positive part is compilation of the existing fragment. The rejecting part
is limited to the named local lemmas: step does not outlive session,
`ephemeralOnly` is not a subset of empty capture, `roomHistoryOnly` is not a
subset of `ephemeralOnly`, and zero call budget is rejected. The audit is
structural support for the registered falsifier; it is not a proof that all
future elaborators or runtimes reject analogous programs.

## Interpretation and limits

The evidence supports only a reproducible L3 statement: this existing
helper-local Lean file currently checks and contains the specific positive and
negative facts named above. It does **not** select a Mir primitive, establish a
final typed calculus, prove `theory/02`, validate runtime behavior, change
`theory/11`, discharge an OBL, establish conformance, or permit L2 promotion.

The pre-registered falsifier did not occur for these commands. The record's
reliance status remains `not-promoted`; a later failed re-run, provenance
failure, or cross-cut conflict freezes reliance rather than revising this
evidence into a stronger claim.

## Advisory and follow-up

A browser-backed Oracle candidate-selection consultation remains advisory and
was still running when these commands completed. This evidence does not depend
on that consultation. Its result, if relevant, must be separately distilled
into a later LAB report or plan entry and cannot rewrite the pre-registration.

The next required operation is to add this exact evidence commit to WRK-0001's
append-only `Evidence commits:` field in a separate manifest commit, then
perform the clean-worktree authoritative validation before the pilot
checkpoint.
