# OBL-001 LAB statement drafts

This directory stores LAB-only Lean statement-shape drafts for THM-001 /
OBL-001 assignment elaboration soundness.

Current draft:

- `THM001StatementDraft.lean`: compile-check-only THM-001 / OBL-001 shape
  whose sync guard keeps request evidence, generated-write coverage, RHS
  dependency, generated-failure containment, authority obligations,
  source-span evidence, visible consequences, and nested-locus non-authority
  linked through the postcondition.

Boundary:

- compile-check only;
- no `mirrorea_canon/theory/11-metatheory-ledger.md` status movement;
- no OBL-001 completion;
- no proof discharge;
- no proof skeleton, runtime dispatch, G1 exit, or conformance claim.

## Adjacent bounded evidence

`ResultWriteCoverageCountermodel.lean` imports this draft and checks a distinct
LAB-only question: the draft's `GeneratedWrite` predicate need not enumerate an
experiment-local Result/write membership relation. It does not alter this draft
or define Canon Core. Its governing L3 record is `WRK-0007`.
