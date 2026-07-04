# plan/115 - G1 OBL-024 association guard hardening

## Purpose

This file records a LAB-only static-guard hardening package for the OBL-024
Lean statement draft. It follows `plan/114` and prevents the report-local
association key split from drifting back toward final ABI, key-equality
semantics, or branch-local association-key vocabulary.

This package does not edit canon, move the canon proof ledger, prove OBL-024,
freeze Diagnostic / association / replay ABI, change runtime JSON, change
repair output, or claim conformance / G1 exit.

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
- OBL-024 replay vocabulary preflight / refinement:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md`
  and `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- OBL-024 association vocabulary refinement:
  `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`
- LAB Lean artifact:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- LAB Lean explanation:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- LAB Lean sync guard:
  `scripts/tests/test_current_l2_lean_sample_sync.py`

If this LAB guard conflicts with canon, canon wins.

## What changed

The OBL-024 Lean sync test now guards additional association-boundary risks:

- no final-looking `BranchId`, `DiagnosticBranchAssociationKey`,
  `AssociationKeyABI`, or comparable final association-key names in the Lean
  draft;
- no `BEq`, `Hashable`, `Function.Injective`, or `Function.Surjective`
  pressure on `ReportLocalAssociationKey`;
- no direct `DiagnosticBranch -> ReportLocalAssociationKey` or
  `ReportLocalAssociationKey -> DiagnosticBranch` carrier shape;
- structural anchors remain present: `CurrentEvidenceBoundary`,
  `CoveredDiagnosticSoundnessCase`, and `Rejects`;
- the explanation explicitly says the report-local association key is
  **not semantic association by key equality** and **not a branch-local
  association key**.

## Why this matters

`plan/114` split the vocabulary into scoped semantic association,
report-local key compatibility, and future proof-level association relation.
This package makes the split harder to regress accidentally. The key point is:
the current helper-local `lab_association_key` may be useful projection
evidence, but it must not become the definition of diagnostic association, a
stable public key, or a branch-local failed-premise selector.

## Validation anchors

```bash
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_obl024_draft_names_association_vocabulary_boundary
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync
python3 scripts/current_l2_lean_sample_sync.py
```

## Open questions

- Whether final proof-level association should use a witness object, relation
  over elaboration events, judgment-attempt identity, or another structure.
- Whether branch classification will ever participate in association proof
  vocabulary. Current LAB guard says it is not a report-local association key.
- How diagnostic equality / ordering interacts with future OBL-021
  determinism proof vocabulary.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-024 proof.
- No OBL-024 completion.
- No final Diagnostic ABI.
- No final association-key ABI.
- No final request ID, branch ID, diagnostic event ID, or association ID
  semantics.
- No branch-local association-key semantics.
- No key uniqueness, comparability, injectivity, surjectivity, or stability
  claim.
- No final replay ABI or replay engine.
- No runtime JSON change.
- No repair output change.
- No OBL-025 completion claim.
- No conformance or G1 exit claim.
