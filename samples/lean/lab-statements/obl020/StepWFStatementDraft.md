# StepWFStatementDraft.lean

## Summary

- LAB-only Lean statement-shape draft for OBL-020 well-formedness preservation.
- The primary artifact is a `Prop` definition named `OBL020StatementDraft`.
- It is intentionally not a theorem, proof, axiom, final namespace, or canon
  ledger update.

## Why this file exists

- `plan/76` identified OBL-020 as well-formedness preservation over runtime
  step rules, separate from OBL-001 assignment soundness and OBL-021
  elaboration determinism.
- This file checks that the aggregate preservation statement can be expressed
  without importing final MirCore runtime datatypes or proving any step rule.
- It keeps step-family membership and well-formedness clause evidence abstract.
- The canon WF pressure clauses are: acyclic occurrence DAG, grant lineage,
  observe/publish ancestry, active-or-tombstoned keys, and monotone chain
  positions. The Lean draft keeps these behind `WellFormed` rather than turning
  them into a first-draft proof interface.
- The sync guard keeps `PreservesWF` as
  `WellFormed(before) -> Step(before, label, after) -> WellFormed(after)`.
  WF clauses stay behind `WellFormed`.

## Boundary

- This is LAB evidence outside `mirrorea_canon/`.
- This does not edit `mirrorea_canon/theory/11-metatheory-ledger.md`.
- This does not claim OBL-020 completion, proof discharge, proof skeleton
  completion, G1/T1/T2 exit, C-runtime or C-static conformance, runtime
  implementation proof, final scheduler semantics, or final runtime API.
- It is not per-step proof decomposition, scheduler semantics, or step-family
  completion.
- It does not decide whether future OBL-020 is proved as per-step lemmas plus
  an aggregate theorem or only as an aggregate statement.

## Validation anchor

```bash
lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
```
