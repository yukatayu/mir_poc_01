# plan/112 - G1 OBL-024 replay vocabulary preflight

## Purpose

This file records a docs-first vocabulary preflight for OBL-024 replay
semantics. It separates the current executable LAB `trace_local_replay` anchor
from a future proof-level replay relation.

The current `diagnostic_soundness_projection.trace_local_replay` object is
useful evidence for the E-ROW pressure case, but it is not a replay engine, not
a final replay ABI, and not proof that OBL-024 is discharged.

This package does not edit production code, expected JSON, Lean statement
files, canon, or repair semantics.

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
- OBL-024 executable projection carrier:
  `plan/110-g1-obl024-executable-projection-carrier.md`
- OBL-024 Rust fixture guards:
  `plan/111-g1-obl024-projection-rust-fixture-guards.md`

If this LAB vocabulary conflicts with canon, canon wins.

## Vocabulary split

Use qualified terms when discussing replay. Do not use bare "replay" when the
intended meaning is one of the narrower roles below.

| Level | Role | Current status | Non-claims |
|---|---|---|---|
| carrier projection | serialized LAB diagnostic detail fields that report the diagnostic id, association key, rule, premise, bindings, and replay anchor | implemented for `ELAB-04/07/10/13..16` by `plan/110` and guarded by `plan/111` | not final Diagnostic JSON / ABI |
| report-local replay anchor | helper-local record saying which request / target / premise / missing evidence the current report recomputed | implemented as `trace_local_replay` inside `diagnostic_soundness_projection` | not a replay engine, not a proof-level relation, not global root-cause uniqueness |
| proof-level replay relation | future formal relation saying the reported bindings reconstruct a judgment slice and replay of that slice fails trace-locally at the reported premise | OPEN | not implemented, not ledger-moved, not final theorem vocabulary |

Preferred names in new LAB prose:

- `report-local replay anchor`: current serialized LAB evidence under
  `diagnostic_soundness_projection.trace_local_replay`;
- `report-local projection`: the whole non-final
  `diagnostic_soundness_projection` object;
- `report-local association key`: the serialized LAB-only
  `lab_association_key`;
- `fixture projection guard`: Rust / Python / sample validation that the
  projection-bearing fixtures remain internally consistent;
- `proof-level replay relation`: future formal relation for OBL-024 proof work.

Boundary sentence for future docs:

```text
In this document, report-local replay anchor means serialized LAB evidence that
the current diagnostic detail names the local failed premise and local
missing-evidence explanation. It is not a proof-level replay relation, replay
engine, replay API, or final Diagnostic ABI.
```

## Current report-local replay anchor

Current executable LAB evidence stores:

```text
trace_local_replay = {
  replay_scope,
  replayed_request_id,
  replayed_target_ref,
  fails_exactly_at,
  failure_reason,
  expected_missing_evidence,
  replay_non_final
}
```

For current E-ROW evidence, this means:

- replay scope is the helper-local Surface-to-Core elaboration report;
- replayed request is the generated remote request in the same LAB diagnostic
  detail;
- replayed target is the `when ... fails` row target reference;
- the failing premise is `generated_failures_subset_declared_fails`;
- the failure reason is `missing_generated_failures`;
- expected missing evidence is the same set as the diagnostic's
  `missing_evidence`.

This is a report consistency anchor. It does not by itself define a replay
algorithm.

## Future proof-level replay vocabulary

A future OBL-024 proof-level replay relation needs vocabulary beyond current
JSON fields. Candidate abstract roles:

| Role | Meaning | Current status |
|---|---|---|
| `ReplayInput` | the judgment input plus reported rule / premise / bindings selected for replay | OPEN |
| `JudgmentSlice` | the local derivation slice reconstructed from the reported bindings | OPEN |
| `BindingSubstitution` | relation applying reported bindings to a rule premise | OPEN |
| `ReplayDerivationPath` | the named trace-local derivation path used for replay | OPEN |
| `PremiseEvaluation` | evaluation result of the reported premise in that reconstructed slice | OPEN |
| `TraceLocalFailure` | relation that the replay path fails at the reported premise | OPEN |
| `NoEarlierTraceLocalFailure` | optional relation excluding earlier failures on the same replay path | OPEN |
| `NoGlobalUniquenessClaim` | explicit guard that trace-local failure is not global root-cause uniqueness | OPEN guard |
| `DiagnosticReplayCompatible` | relation connecting diagnostic projection, replay input, and replay failure | OPEN |
| `MixedBranchReplayBoundary` | relation ensuring mixed diagnostic branches remain classification / partition evidence, not independent replay premises | OPEN |

These are vocabulary roles, not final Lean names.

Bridge rule:

```text
A report-local replay anchor may be used as LAB evidence when designing a
future proof-level replay relation, but it is not itself that relation. Any
future proof must pass through an explicit carrier projection and
diagnostic-to-rejection association relation.
```

## Exactness boundary

"Fails exactly at the reported premise" should remain trace-local unless a
future canon decision says otherwise.

Trace-local exactness may assert:

- the selected replay path is the one named by the diagnostic projection;
- the reported premise belongs to the reported rule instance under the reported
  bindings;
- replay of that selected slice fails at that premise;
- diagnostic id / family / missing evidence / span are compatible with that
  premise.

Trace-local exactness must not silently assert:

- no other premise in the whole program could also fail;
- this diagnostic is the globally minimal root cause;
- diagnostic ordering or ranking is final;
- the emitted JSON association key is final ABI;
- mixed branch classification creates separate failed premises;
- OBL-025 repair coverage follows from OBL-024 replay soundness.

## E-ROW instantiation reading

For current E-ROW rows:

| LAB row | Replay reading | Boundary |
|---|---|---|
| `ELAB-04` | mixed E-ROW-001 diagnostic: replay anchor names the row-containment premise for the top-level diagnostic | no repair payload; mixed base / visibility branches remain classification pressure |
| `ELAB-07` | clean E-ROW-001 write request: replay anchor names missing base generated failures for the exact request / row | exact `set_insertion` repair remains OBL-025-adjacent evidence, not OBL-024 proof |
| `ELAB-10` | clean E-ROW-002 visibility pressure: replay anchor names `VisibilityDenied` missing from the failure row | singleton visibility repair remains non-final repair evidence |
| `ELAB-13..16` | clean E-ROW-001 singleton base missing failures | singleton repair evidence remains separate from replay soundness |

## UNRESOLVED

- Final Diagnostic JSON / ABI field names.
- Final request id, branch id, association key, and replay id semantics.
- Whether proof-level replay should be whole-judgment, rule-local, or both.
- Whether `NoEarlierTraceLocalFailure` is required for OBL-024 or belongs to a
  later diagnostic-ordering theorem.
- How diagnostic ordering / equality should interact with OBL-021 determinism.
- Multi-span declaration-site / use-site blame semantics.
- Whether the first proof target is an E-ROW fragment or all diagnostic
  families.
- How parse diagnostics and elaboration diagnostics share or do not share one
  replay theorem shape.
- Whether mixed-row branches should ever get branch-local replay witnesses or
  stay diagnostic classification only.

## Suggested next packages

1. If continuing OBL-024, refine the Lean statement draft with separate abstract
   roles for report-local replay anchor and proof-level replay relation, still
   without proof or ABI freeze.
2. If continuing executable evidence, add only guard tests that preserve the
   current non-final projection shape; do not add public replay fields before
   the proof vocabulary is clearer.
3. If switching to OBL-025, keep replay vocabulary out of repair completeness
   except where repair suggestions cite the same failed premise.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-024 proof.
- No OBL-024 completion.
- No final Diagnostic ABI.
- No final replay ABI or replay engine.
- No final request ID, branch ID, association-key, or replay ID semantics.
- No diagnostic ordering or root-cause uniqueness claim.
- No repair output change.
- No OBL-025 completion claim.
- No conformance or G1 exit claim.
