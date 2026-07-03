# plan/110 - G1 OBL-024 executable diagnostic-soundness projection carrier

## Purpose

This file records a LAB-only executable evidence package for OBL-024 diagnostic
soundness projection inside current Surface-to-Core E-ROW diagnostics.

The package adds a non-final `diagnostic_soundness_projection` object under
`lab_diagnostic_details` for current underdeclared generated failure-row
diagnostics. It ties the helper-local diagnostic detail to:

- a LAB-only diagnostic id;
- a LAB-only association key;
- reported rule instance and failed premise;
- reported bindings reconstructed from generated-request and failure-row
  context;
- report-local trace replay that fails at the local failed premise.

This package does not edit canon, does not move
`mirrorea_canon/theory/11-metatheory-ledger.md`, does not prove OBL-024, does
not claim OBL-024 completion, does not freeze Diagnostic ABI / JSON field names
/ request IDs / branch IDs / association-key ABI / replay semantics, does not
claim root-cause uniqueness, does not widen OBL-025 repair coverage, does not
add an `ELAB-04` repair payload, and does not claim conformance or G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- OBL-024 Lean statement draft:
  `plan/109-g1-obl024-lean-statement-draft.md`
- OBL-024 replay vocabulary preflight:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- E-ROW carrier prototype:
  `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- E-ROW request / failure-row context:
  `plan/85-g1-erow-carrier-precondition-hardening.md`
- ELAB-07 repair association guard:
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- ELAB-04 mixed no-repair preflight:
  `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- Implementation:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- Rust evidence:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- Helper evidence:
  `scripts/tests/test_surface_mir_samples.py`
- Expected JSON evidence:
  `samples/full-system-v1-surface/elaboration/elab-04-*/expected/elaboration.json`
  and `ELAB-07/10/13..16` expected JSON files.

If this LAB evidence conflicts with canon, canon wins.

## What was added

`SurfaceLabDiagnosticDetail` now carries:

```text
diagnostic_soundness_projection
```

The projection is a LAB-only helper-local object. It currently contains:

- `diagnostic_id`: derived from legacy code, canon diagnostic family, and the
  current report-local request id;
- `lab_association_key`: derived from the failure-row target reference and
  request id;
- `reported_rule_instance`: `BND-001.row-containment`;
- `reported_failed_premise`: `generated_failures_subset_declared_fails`;
- `reported_bindings`: generated request context plus failure-row context;
- `trace_local_replay`: report-local replay anchor naming the request, target
  row, local failed premise, failure reason, and expected missing evidence;
- `projection_non_final` / `lab_non_final`: explicit non-final markers.

The projection is emitted for the current E-ROW LAB details:

- `ELAB-04`
- `ELAB-07`
- `ELAB-10`
- `ELAB-13`
- `ELAB-14`
- `ELAB-15`
- `ELAB-16`

The helper summary confirmed 7 projection-bearing E-ROW details.

## Association split

There are two association notions and they must remain distinct:

- `diagnostic_soundness_projection.lab_association_key` is serialized LAB
  evidence for OBL-024 projection and replay shape. It is not final ABI.
- `SurfaceLabDiagnosticFailureRowContext.association_key` remains
  `#[serde(skip)]` module-private implementation state for `ELAB-07`
  set-insertion repair suppression across generated requests in the same source
  row.

This package intentionally does not expose the internal span-based
implementation key as public JSON.

## Relation to OBL-024

OBL-024 asks whether an emitted diagnostic reports an actual failed premise and
whether replay of the relevant local judgment slice fails there. The projection
adds executable LAB evidence for the current E-ROW instantiation:

- the reported rule and failed premise are stored in the detail;
- bindings are reconstructed from the request and failure-row context;
- trace replay is represented as report-local evidence, not a replay engine;
- missing evidence is kept aligned with the failure-row context.

This is evidence-carrier hardening only. It is not proof discharge.

## Relation to OBL-025

The package does not change `suggested_repair[]` semantics.

- `ELAB-04` remains no-repair.
- `ELAB-07` retains the exact `set_insertion` item from `plan/102`.
- `ELAB-10` retains the singleton `E-ROW-002` repair item.
- `ELAB-13..16` retain singleton `E-ROW-001` repair items.

No branch-local guidance, mixed wrapper payload, repair ranking, bundle
semantics, or whole-program repair success claim is added.

## Validation anchors

```bash
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all
```

The package also keeps the usual docs/source/fmt validators in the closeout
report.

## Suggested next packages

1. Keep the projection as helper-local LAB evidence. `plan/112` now separates
   the current report-local replay anchor from future proof-level replay
   vocabulary.
2. If a future `ELAB-04` executable payload is desired, first choose a mixed
   wrapper / associated diagnostic model; do not infer it from this projection.
3. If OBL-024 proof work resumes, use this projection as evidence for the E-ROW
   instantiation, not as the final theorem statement.

## Non-claims

- No canon edit.
- No proof-status movement.
- No OBL-024 proof.
- No OBL-024 completion.
- No final Diagnostic ABI.
- No final JSON field names.
- No final request ID.
- No final branch ID.
- No final association-key ABI.
- No final replay engine.
- No final diagnostic equality / ordering.
- No root-cause uniqueness.
- No repair output widening.
- No `ELAB-04` executable payload.
- No OBL-025 proof.
- No OBL-025 completion.
- No conformance claim.
- No G1 exit.
