# OBL-024 LAB statement drafts

This directory stores LAB-only Lean statement-shape drafts for OBL-024
explanation soundness.

Current draft:

- `DiagnosticSoundnessStatementDraft.lean`: compile-check-only OBL-024 shape
  with abstract diagnostic projection, reported rule / failed premise /
  bindings, association key, trace-local replay, diagnostic-family
  compatibility, missing evidence matching, span-blame predicates, and
  non-repair mixed diagnostic branch boundary predicates for every branch of a
  mixed diagnostic gap.

Boundary:

- compile-check only;
- no `mirrorea_canon/theory/11-metatheory-ledger.md` status movement;
- no OBL-024 completion;
- no proof discharge;
- no G1/T1/T2 exit or conformance claim;
- no final Diagnostic ABI, JSON field names, replay engine, diagnostic
  equality / ordering, root-cause uniqueness, request ID / branch ID /
  association-key ABI, repair payload, or OBL-025 repair-completeness claim.
