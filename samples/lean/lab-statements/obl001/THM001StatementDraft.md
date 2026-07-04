# THM001StatementDraft.lean

## Summary

- LAB-only Lean statement-shape draft for THM-001 / OBL-001 ordinary assignment
  elaboration soundness.
- The primary artifact is a `Prop` definition named `THM001StatementDraft`.
- It is intentionally not a theorem, proof, axiom, final namespace, or canon
  ledger update.

## Why this file exists

- `plan/73` identified the minimum predicate split needed before proof-oriented
  OBL-001 work.
- This file checks that the postcondition can be represented in Lean while all
  semantic carriers remain abstract through `Vocab` and `Pred`.
- The sync guard keeps request evidence, generated-write coverage, RHS
  dependency recording, generated-failure containment, authority obligations,
  source-span evidence, visible consequences, and nested-locus non-authority
  linked through `AssignmentElabSoundnessPost`.
- Keeping the file under `lab-statements/` prevents it from being confused with
  current proof fragments under `foundations/` or generated stubs under
  `clean-near-end/`.

## Boundary

- This is LAB evidence outside `mirrorea_canon/`.
- This does not edit `mirrorea_canon/theory/11-metatheory-ledger.md`.
- This does not claim OBL-001 completion, OBL-002 proof discharge, OBL-020
  well-formedness preservation, OBL-021 determinism, G1 exit, conformance, or
  runtime dispatch.
- It is not a proof skeleton and not runtime dispatch.
- OPEN-014 remains open. Dependency recording is stated abstractly and does not
  freeze a cache, reply, observe, projection, or transport policy.

## Validation anchor

```bash
lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
```
