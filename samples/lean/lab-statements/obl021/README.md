# OBL-021 LAB statement drafts

This directory stores LAB-only Lean statement-shape drafts for OBL-021
elaboration determinism.

Current draft:

- `ElabDeterminismStatementDraft.lean`: compile-check-only OBL-021 shape
  whose sync guard keeps success-success result equivalence, reject-reject
  diagnostic equivalence, and success/reject mutual exclusion linked through
  `ElabDeterministicPost`.

Boundary:

- compile-check only;
- no `mirrorea_canon/theory/11-metatheory-ledger.md` status movement;
- no OBL-021 completion;
- no proof discharge;
- no G1/T1/T2 exit or conformance claim;
- no final elaboration equality relation freeze, final equality selection, or
  runtime scheduling determinism claim.
