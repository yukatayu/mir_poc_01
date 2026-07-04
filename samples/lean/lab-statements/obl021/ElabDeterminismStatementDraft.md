# ElabDeterminismStatementDraft.lean

## Summary

- LAB-only Lean statement-shape draft for OBL-021 elaboration determinism.
- The primary artifact is a `Prop` definition named
  `OBL021StatementDraft`.
- It is intentionally not a theorem, proof, axiom, final namespace, or canon
  ledger update.

## Why this file exists

- `plan/76` identified OBL-021 as separate from OBL-001 assignment soundness and
  OBL-020 step-rule well-formedness preservation.
- This file checks that determinism can be expressed without choosing final
  MirCore datatypes or final equality/equivalence relations.
- It keeps result equality abstract through predicate fields such as
  `EquivalentCoreTerm`, `EquivalentFailureRow`, and `EquivalentGeneratedEdges`.
- It keeps diagnostic equivalence abstract through `SameDiagnostic`, currently
  backed by `EquivalentDiagnostic`, without freezing a final diagnostic ABI.
- The sync guard keeps success-success result equivalence, reject-reject
  diagnostic equivalence, and success/reject mutual exclusion linked through
  `ElabDeterministicPost`.

## Boundary

- This is LAB evidence outside `mirrorea_canon/`.
- This does not edit `mirrorea_canon/theory/11-metatheory-ledger.md`.
- This does not claim OBL-021 completion, proof discharge, G1/T1/T2 exit,
  C-static conformance, runtime scheduling determinism, parser/checker
  implementation proof, or final equality relation.
- It is not final equality selection and not runtime scheduling determinism.
- It does not settle whether future OBL-021 uses syntactic equality,
  normalized equality, definitional equality, alpha-equivalence, or a
  canon-specific equivalence.

## Validation anchor

```bash
lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
```
