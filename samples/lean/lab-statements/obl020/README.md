# OBL-020 LAB statement drafts

This directory stores LAB-only Lean statement-shape drafts for OBL-020
well-formedness preservation of step rules.

Current draft:

- `StepWFStatementDraft.lean`: compile-check-only OBL-020 shape whose sync
  guard keeps `PreservesWF` as
  `WellFormed(before) -> Step(before, label, after) -> WellFormed(after)`,
  while WF clauses stay behind `WellFormed`.

Boundary:

- compile-check only;
- no `mirrorea_canon/theory/11-metatheory-ledger.md` status movement;
- no OBL-020 completion;
- no proof discharge;
- no proof skeleton completion;
- no G1/T1/T2 exit or conformance claim;
- no runtime implementation proof, per-step proof decomposition, scheduler
  semantics, or step-rule family completion claim.
