# OBL-025 LAB statement drafts

This directory stores LAB-only Lean statement-shape drafts for OBL-025
explanation completeness / repair coverage.

Current draft:

- `RepairCompletenessStatementDraft.lean`: compile-check-only OBL-025 shape
  with abstract whole-rejected-gap, set-insertion, grouped multi-edit,
  complete-local-repair, partial-guidance, and branch-local non-coverage
  predicates. The sync guard checks that completeness still goes through an
  eligible single-edit witness, `SuggestedRepairOf`, and
  `SuggestionCoversWitness`, and that grouped multi-edit, partial guidance, and
  branch-local guidance remain outside current whole-gap coverage.

Boundary:

- compile-check only;
- no `mirrorea_canon/theory/11-metatheory-ledger.md` status movement;
- no OBL-025 completion;
- no proof discharge;
- no G1/T1/T2 exit or conformance claim;
- no final Diagnostic / repair ABI, repair ranking, multi-edit repair support,
  branch ID semantics, placeholder repair-array sufficiency, all-repairs /
  minimality claim, or whole-program success claim.
