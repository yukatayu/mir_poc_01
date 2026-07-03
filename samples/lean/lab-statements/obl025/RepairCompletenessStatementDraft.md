# RepairCompletenessStatementDraft.lean

## Summary

- LAB-only Lean statement-shape draft for OBL-025 explanation completeness.
- The primary artifact is a `Prop` definition named `OBL025StatementDraft`.
- It is intentionally not a theorem, proof, axiom, final namespace, final
  Diagnostic / repair ABI, or canon ledger update.

## Why this file exists

- `plan/82` identified OBL-025 as separate from OBL-024 explanation soundness.
- `plan/86` provides current LAB repair-carrier evidence only for the
  `E-ROW-002` / `VisibilityDenied` row-containment shape represented by
  `ELAB-10`.
- `plan/94` adds current LAB repair-carrier evidence for `E-ROW-001`
  non-visibility singleton rows `ELAB-13..16`.
- `plan/96` records candidate set-insertion / bundle / partial-guidance
  vocabulary while keeping `ELAB-04/07` no-repair.
- This file checks that the completeness relation can be expressed without
  choosing final Diagnostic JSON fields, edit scripts, repair ranking,
  multi-span target format, or repair-application semantics.

## Shape

`OBL025StatementDraft` states an abstract coverage shape:

- a well-scoped Line-1 input rejects;
- the rejection is in a covered repair case;
- a declared fragment is identified;
- if at least one repair witness exists that matches the diagnostic family,
  concrete missing evidence, failed local premise, and blame target in that
  declared fragment, is single-edit rather than grouped multi-edit, and covers
  the whole rejected gap, then an associated diagnostic has at least one
  suggested repair that realizes a compatible complete local repair.

The existential witness shape is deliberate. The draft does not require all
possible repairs to be emitted, ranked, or proved minimal.

The draft also keeps diagnostic family and missing evidence separate. Current
LAB evidence uses diagnostic family `E-ROW-002` and missing evidence
`VisibilityDenied`, but those are not collapsed into one predicate.

The current refinement also separates complete local repair evidence from
partial guidance. Set insertion can enter the OBL-025-shaped coverage only if a
later package admits it as `EligibleSingleEditRepair`. Complete grouped
multi-edit witnesses can be named by `CompleteGroupedMultiEditRepair`, but are
not counted by `RepairCompletenessForRejection`. Partial guidance can be named
by `PartialGuidanceNonCoverage`, but is excluded from
`SuggestionCoversWitness`.

The whole-gap predicates, `RepairWitnessCoversRejectedGap` and
`SuggestedRepairCoversRejectedGap`, prevent one child repair or one
missing-failure atom from being read as coverage for a multi-missing
row-containment rejection.

## Boundary

- This is LAB evidence outside `mirrorea_canon/`.
- This does not edit `mirrorea_canon/theory/11-metatheory-ledger.md`.
- This does not claim OBL-025 completion, proof discharge, G1/T1/T2 exit,
  conformance, final diagnostic ABI, final repair payload ABI, repair ranking,
  multi-edit repair support, or whole-program success after repair.
- `CoveredLine1RepairCase` is abstract. Current executable LAB evidence only
  supports singleton repair-carrier shapes in `ELAB-10` and `ELAB-13..16`;
  mixed and multi-missing rows remain no-repair evidence.

## Validation anchor

```bash
lean samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
```
