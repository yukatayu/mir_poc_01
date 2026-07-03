# DiagnosticSoundnessStatementDraft.lean

## Summary

- LAB-only Lean statement-shape draft for OBL-024 explanation soundness.
- The primary artifact is a `Prop` definition named `OBL024StatementDraft`.
- It is intentionally not a theorem, proof, axiom, final namespace, final
  Diagnostic ABI, replay engine, conformance statement, or canon ledger update.

## Why this file exists

- `plan/81` identified the OBL-024 relation shape: an emitted diagnostic must
  report an actual rule instance, an actual failed premise, enough bindings to
  reconstruct the local judgment slice, and replay must fail exactly there.
- `plan/84` and `plan/85` added LAB-only E-ROW diagnostic detail and
  precondition context, making an abstract compile-check-only draft useful
  without freezing the helper JSON shape.
- OBL-024 is separate from OBL-025. This draft does not mention repairs,
  repair witnesses, suggested repairs, or repair ranking.

## Shape

`OBL024StatementDraft` states an abstract soundness shape:

- a well-scoped judgment input rejects;
- a diagnostic is associated with that rejection;
- the diagnostic lies inside the current LAB evidence boundary;
- the diagnostic and rejection share an abstract association key;
- the diagnostic reports a diagnostic id, rule instance, failed premise,
  bindings, diagnostic family, missing evidence, and primary span;
- the reported rule instance is actual for the same judgment input;
- the reported failed premise belongs to that rule instance under the reported
  bindings;
- the reported bindings are sufficient to reconstruct the local premise;
- a replay witness for the same judgment slice fails trace-locally at the
  reported premise;
- diagnostic id, diagnostic family, missing evidence, and blame span match
  that premise;
- mixed-row diagnostics keep branch classification separate from the whole
  failed premise: every branch of the diagnostic gap must classify some missing
  evidence, remain in the diagnostic partition, and stay non-independent from
  the whole failed premise; branches do not introduce repair coverage.

The `TraceLocalReplayFailsExactlyAt` predicate is deliberately trace-local. It
does not claim global root-cause uniqueness, diagnostic ranking, minimality, or
all possible failing premises.

`CurrentEvidenceBoundary` and `CoveredDiagnosticSoundnessCase` prevent the LAB
draft from claiming all diagnostic families. Current executable evidence is
E-ROW-shaped, but the Lean vocabulary is not hard-coded to E-ROW.

`AssociationKey` and `DiagnosticBranch` are abstract proof-shape carriers. They
are not final request IDs, branch IDs, JSON keys, diagnostic fields, or ABI.
`DiagnosticBranch` is deliberately non-repair vocabulary and is separate from
the OBL-025 `RepairBranch` vocabulary.

## E-ROW reading

For current E-ROW evidence, the intended instantiation is:

- rule instance: row containment for generated remote requests;
- failed premise: `generated_failures_subset_declared_fails`;
- bindings: generated request context plus failure-row context;
- missing evidence: missing generated failure families;
- replay witness: recomputation of the generated request failures and the
  surrounding `when ... fails` row.

This is a LAB reading of current evidence only. `generated_failure_not_declared`
and `lab_diagnostic_details` remain helper-local / non-final carrier evidence.
For mixed rows such as `ELAB-04`, the top-level diagnostic owns the failed
premise while every branch of the diagnostic gap remains classification /
partition evidence for base / visibility pressure without turning into an
independent premise.

## Boundary

- This is LAB evidence outside `mirrorea_canon/`.
- This does not edit `mirrorea_canon/theory/11-metatheory-ledger.md`.
- This does not claim OBL-024 completion, proof discharge, G1/T1/T2 exit,
  conformance, final Diagnostic ABI, final JSON field names, final replay
  semantics, diagnostic ordering, diagnostic equality, root-cause uniqueness,
  request IDs, branch IDs, association-key ABI, branch JSON, or repair
  completeness.
- This does not specialize OBL-024 to E-ROW as the whole theorem.
- This does not alter OBL-025 repair completeness or any executable repair
  payload.

## Validation anchor

```bash
lean samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean
python3 scripts/current_l2_lean_sample_sync.py
```
