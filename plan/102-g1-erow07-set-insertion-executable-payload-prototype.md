# plan/102 - G1 ELAB-07 set-insertion executable payload prototype

## Purpose

This file records the LAB-only executable prototype for the exact `ELAB-07`
set-insertion repair payload.

The package implements one non-final `suggested_repair[]` item for the current
`ELAB-07` fact pattern only:

- `canon_id = E-ROW-001`
- write-side generated remote request
- one existing concrete `when_fails_row`
- declared failures exactly `[ MissingCapability ]`
- missing failures exactly
  `[ MissingWitness, RouteUnavailable, StaleMembership ]`
- no `VisibilityDenied`

This package does not implement general set-insertion support. It does not
implement bundle semantics, partial guidance, repair ranking, visibility
ranking, multi-edit repair, final Diagnostic / repair ABI, OBL-024/025 proof,
conformance, canon movement, or G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon entry points:
  `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon static semantics:
  `mirrorea_canon/spec/03-static-semantics.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB design gate:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- LAB negative guard hardening:
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- LAB row-identity guard hardening:
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- LAB edit-cardinality gate:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- LAB executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB `ELAB-04` exclusion fence:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB singleton repair gate and prototype:
  `plan/93-g1-erow001-singleton-repair-assumption.md`,
  `plan/94-g1-erow001-singleton-repair-prototype.md`
- LAB implementation:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- LAB tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`
- LAB sample expectation:
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`

If this LAB prototype conflicts with canon, canon wins.

## Decision level

This is `L3` LAB implementation evidence.

The prototype is intentionally narrow. It is evidence that the current helper
can emit one exact non-final set payload for `ELAB-07`; it is not evidence that
set insertion is generally designed, stable, final, or proof-complete.

## TDD evidence

RED phase:

- Rust target:
  `cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture`
  failed because current `ELAB-07` emitted no `suggested_repair`.
- Python target:
  `python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic`
  failed because helper output still lacked `suggested_repair`.

GREEN phase:

- Added a separate set payload path instead of relaxing the singleton helper.
- Updated only the `ELAB-07` expected JSON.
- The same Rust and Python targets passed after implementation.

The first Rust RED attempt briefly placed the set-payload expectation on the
mixed read / visibility `ELAB-04` shape. That was corrected before
implementation: `ELAB-04` remains an explicit no-repair fence.

## Implemented payload

The executable `ELAB-07` payload is one top-level item:

```text
repair_shape = set_insertion
repair_family = add-to-fails-row
diagnostic_family = E-ROW-001
edit_atom = complete_missing_base_failure_set_into_one_existing_when_fails_row
source_locus_edit_count = 1
element_insert_count = 3
declared_failures_before = [ MissingCapability ]
insert_failures = [ MissingWitness, RouteUnavailable, StaleMembership ]
required_failures = [
  MissingCapability,
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
local_effect.declared_failures_after = required_failures
coverage_scope = complete_missing_set_for_associated_request
local_premise = generated_failures_subset_declared_fails
local_premise_after_edit = discharged_for_associated_request
single_edit_assumption = erow001_elab07_complete_base_failure_set_source_locus_edit
```

The set payload omits singleton-only fields:

- no `missing_failure`
- no singleton `declared_failures`

It also omits later roles:

- no `repair_group_id`
- no `bundle_semantics`
- no `child_repairs`
- no `partiality`
- no `visibility_branch`
- no `ordering_policy`
- no ranking fields

## Implementation notes

`SurfaceLabSuggestedRepair` now has optional set-payload roles. Singleton rows
serialize exactly as before because those roles are skipped when absent.

The construction path is split:

- `erow_row_addition_suggested_repair`
  dispatches to the exact set path first, then falls back to singleton repair.
- `erow_set_insertion_suggested_repair`
  is guarded by the exact `ELAB-07` fact pattern.
- `erow_singleton_row_addition_suggested_repair`
  remains the singleton path for `ELAB-10` and `ELAB-13..16`.

The implementation does not start by changing
`erow_singleton_row_addition_suggested_repair` to accept
`missing_failures.len() > 1`.

After `plan/103`, the set path also requires one associated generated request
for the LAB row association. When a later request for the same association
appears, earlier `set_insertion` repairs for that association are suppressed.
After `plan/104`, that association key includes the existing `when` source span
while the public `target_ref` string remains unchanged. The associated-request
count and association key are internal and skipped during serialization, so the
LAB JSON shape is unchanged.

## Current executable status

| Row | Current executable repair output |
|---|---|
| `ELAB-04` | no `suggested_repair` |
| `ELAB-07` | one non-final `set_insertion` item |
| `ELAB-10` | one singleton `E-ROW-002` item |
| `ELAB-13` | one singleton `E-ROW-001` item |
| `ELAB-14` | one singleton `E-ROW-001` item |
| `ELAB-15` | one singleton `E-ROW-001` item |
| `ELAB-16` | one singleton `E-ROW-001` item |

Focused Rust-only negative fixtures from `plan/103` also keep the set path from
appearing for proper subset, padded declaration, duplicate declaration, and
multi generated-request variants of the current `ELAB-07` shape.

Sample row count is unchanged.

## Regression fences

This package preserves:

- `ELAB-04` as mixed base / `VisibilityDenied` no-repair evidence;
- `ELAB-10` singleton `E-ROW-002` output shape;
- `ELAB-13..16` singleton `E-ROW-001` output shape;
- no empty `suggested_repair: []` output for no-repair rows;
- no child repair bundle output;
- no partial guidance output;
- no final ABI wording.

## Remaining gaps

`plan/103` adds Rust-only negative fixtures for:

- proper two-missing subset rejection;
- padded declaration rejection;
- duplicate declaration rejection;
- multi generated-request suppression for one target reference.

Remaining gaps include:

- final row identity: the current internal count is keyed by the LAB target
  reference plus existing `when` source span, not by a final AST-row identity;
- durable same-event row identity across source-preserving edits;
- true multi-target-row policy;
- true row movement / cross-row retargeting policy;
- explicit future bundle semantics and partial-guidance policy after the
  current `plan/106` shape guard;
- broader set-insertion support beyond the exact `ELAB-07` fact pattern.

These gaps remain before treating this as broader set-insertion evidence.

## Relation to OBL-025

This prototype can be read as LAB evidence for one candidate local repair
shape. It still does not prove OBL-025 and does not move canon proof status.

OBL-025 should remain abstract until negative guard coverage is stronger and
the project decides which executable payload roles are stable enough to mention
in proof-facing vocabulary.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No general set-insertion support.
- No bundle semantics support.
- No partial-guidance output or coverage support.
- No repair ranking.
- No visibility ranking.
- No multi-edit support.
- No row creation, row splitting, row movement, or row retargeting support.
- No `VisibilityDenied` / `E-ROW-002` widening.
- No `ELAB-04` reclassification.
- No `ELAB-10` change.
- No `ELAB-13..16` change.
- No OBL-024 proof.
- No OBL-025 proof.
- No OBL-025 completion.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
- No whole-program success after repair claim.
