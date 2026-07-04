# plan/114 - G1 OBL-024 Lean association vocabulary refinement

## Purpose

This file records a LAB-only Lean statement-shape refinement for OBL-024. It
separates the current report-local association key from future proof-level
diagnostic association vocabulary inside
`samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`.

This package keeps OBL-024 compile-check-only. It does not edit canon, move the
canon proof ledger, prove OBL-024, freeze Diagnostic / association / replay ABI,
change runtime JSON, change repair output, or claim conformance / G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- OBL-024 Lean statement draft:
  `plan/109-g1-obl024-lean-statement-draft.md`
- OBL-024 replay vocabulary preflight:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- OBL-024 Lean replay vocabulary refinement:
  `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- LAB Lean artifact:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- LAB Lean explanation:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- LAB OBL-024 Lean directory note:
  `samples/lean/lab-statements/obl024/README.md`
- LAB Lean sync guard:
  `scripts/tests/test_current_l2_lean_sample_sync.py`
- Follow-up static guard hardening:
  `plan/115-g1-obl024-association-guard-hardening.md`

If this LAB refinement conflicts with canon, canon wins.

## What changed

The previous OBL-024 draft used one abstract `AssociationKey` carrier, a
lower-scope `AssociatedEmittedDiagnostic` predicate, and predicates named
`RejectionAssociationKey` / `DiagnosticAssociationKey`. That was
compile-checkable, but it could be read as too close to a final association-key
ABI or as treating key sharing as the semantic association relation.

The Lean draft now uses:

- `ReportLocalAssociationKey`: the current LAB role represented by helper-local
  diagnostic projection association evidence;
- `DiagnosticAssociatedToRejection`: scoped semantic association predicate over
  `env`, `ctx`, `locus`, input, rejection, and diagnostic;
- `DiagnosticReportsReportLocalAssociationKey`: relation saying the diagnostic
  reports the non-final report-local key;
- `ReportLocalAssociationKeyFor`: relation saying the report-local key is
  compatible with the scoped rejected judgment and diagnostic;
- `ReportLocalAssociationKeyNonFinal`: explicit non-final guard for that key;
- `ReportLocalAssociationKeyCompatible`: helper relation keeping the current
  diagnostic / rejection / key evidence together without defining semantic
  association as key equality;
- `ProofLevelAssociationWitness`: future proof-level association witness
  vocabulary;
- `ProofLevelAssociationWitnessFor`: relation connecting that future witness to
  the environment, context, locus, rejected judgment, and diagnostic;
- `ProofLevelAssociationRelation`: future proof-level relation vocabulary for
  the association witness;
- `DiagnosticAssociationCompatible`: helper relation requiring the scoped
  semantic association, report-local key compatibility, and future proof-level
  witness / relation.

The previous lower-scope `AssociatedEmittedDiagnostic` predicate was replaced
by `DiagnosticAssociatedToRejection` so the candidate relation is scoped to the
same environment / context / locus as the rejecting judgment. It is not a final
emitted diagnostic API.

## Test guard

`scripts/tests/test_current_l2_lean_sample_sync.py` now includes a narrow
static guard that the OBL-024 Lean draft and explanation continue to name:

- `ReportLocalAssociationKey`;
- `ProofLevelAssociationWitness`;
- `DiagnosticAssociatedToRejection`;
- `DiagnosticReportsReportLocalAssociationKey`;
- `ReportLocalAssociationKeyFor`;
- `ReportLocalAssociationKeyCompatible`;
- `DiagnosticAssociationCompatible`;
- `ProofLevelAssociationRelation`;
- `report-local association key`;
- `proof-level association relation`.

The guard also rejects reintroducing standalone `AssociationKey : Type u` or
`DiagnosticAssociationKey`, and rejects stale `AssociatedEmittedDiagnostic`.
`plan/115` further hardens this by guarding against final-looking request /
branch / ABI names, association-key comparability or uniqueness pressure,
direct branch-local association-key carrier shapes, and explanation wording
that omits the non-equality / non-branch-local boundary.

This test is intentionally vocabulary-oriented. It is not a proof test and does
not validate final Diagnostic ABI, request IDs, branch IDs, association-key ABI,
or runtime JSON shape.

## Validation anchors

```bash
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_lean_sample_sync.py
```

The sync script rewrites generated clean-near-end Lean stubs as part of its
normal workflow, so this package checks the resulting Git diff and commits only
intentional source / explanation / test changes.

## Boundary

- The current executable `lab_association_key` remains report-local LAB
  evidence.
- `ReportLocalAssociationKey` is not final request identity, branch identity,
  Diagnostic JSON, or association-key ABI.
- `ProofLevelAssociationWitness` is future proof vocabulary, not a proof.
- `ProofLevelAssociationRelation` is future proof relation vocabulary, not a
  final theorem or proof discharge.
- The bridge from report-local association key to proof-level diagnostic
  association remains an OPEN proof design boundary.
- This association vocabulary does not change report-local replay anchor
  vocabulary or OBL-025 repair completeness.

## Open questions

- Whether final diagnostic association is keyed by judgment attempt, emitted
  diagnostic event, rule instance, branch, or a combination.
- Whether final OBL-024 association should be stated before replay soundness or
  as part of replay compatibility.
- How diagnostic ordering / equality should interact with OBL-021 determinism.
- Whether final proof vocabulary should keep the names used in this LAB draft.

## Suggested next packages

1. Keep OBL-024 compile-check-only until proof-level association and replay
   relation vocabulary are stable enough for theorem work.
2. If continuing OBL-024, refine only theorem-shape helpers or guard tests that
   preserve the report-local / proof-level split.
3. If switching to OBL-025, avoid importing association-key vocabulary into
   repair completeness except where a repair cites the already associated
   diagnostic / rejected gap.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-024 proof.
- No OBL-024 completion.
- No final Diagnostic ABI.
- No final association-key ABI.
- No final request ID, branch ID, diagnostic event ID, or association ID
  semantics.
- No final replay ABI or replay engine.
- No diagnostic ordering or root-cause uniqueness claim.
- No runtime JSON change.
- No repair output change.
- No OBL-025 completion claim.
- No conformance or G1 exit claim.
