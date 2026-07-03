# plan/103 - G1 ELAB-07 set-insertion negative-guard hardening

## Purpose

This file records the LAB-only negative guard hardening around the exact
`ELAB-07` set-insertion payload prototype from `plan/102`.

The package keeps the positive executable set path narrow:

- `canon_id = E-ROW-001`
- write-side generated remote request
- one existing concrete `when_fails_row`
- declared failures exactly `[ MissingCapability ]`
- missing failures exactly
  `[ MissingWitness, RouteUnavailable, StaleMembership ]`
- no `VisibilityDenied`
- one associated generated request for the failure-row target

The package adds focused negative Rust fixtures so that nearby shapes do not
silently receive the `set_insertion` repair.

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
- LAB positive prototype:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- LAB payload model:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- LAB assumption gate:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- LAB executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB `ELAB-04` exclusion fence:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB set / bundle vocabulary:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB mixed / multi decomposition:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB singleton repair gate and prototype:
  `plan/93-g1-erow001-singleton-repair-assumption.md`,
  `plan/94-g1-erow001-singleton-repair-prototype.md`
- LAB implementation:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- LAB tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- LAB sample expectation:
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`

If this LAB guard conflicts with canon, canon wins.

## Decision level

This is `L3` LAB implementation evidence.

The package is negative guard evidence around one exact prototype. It is not
evidence that the row-target identity model, source edit model, or proof-facing
repair vocabulary is complete.

## TDD evidence

RED phase:

- Rust target:
  `cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted_for_multiple_generated_requests_in_one_row -- --nocapture`
  failed because two generated requests under one `when` row still received a
  `set_insertion` repair item before the guard was added.

GREEN phase:

- The same Rust target passed after adding an internal associated-request
  count guard and retrospective suppression for the same target reference.
- The focused negative target
  `cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture`
  passed for subset, padded, duplicate, and multi-request cases.
- The full `surface_to_core_elaboration` integration test file passed.
- Surface helper validation remained at 52 rows / 0 failures.

## Negative fixtures

| Fixture | Shape | Expected output |
|---|---|---|
| proper subset | declared `[ MissingCapability, MissingWitness ]`; missing two base failures | reject with LAB detail; no `suggested_repair` |
| padded declaration | declared `[ MissingCapability, ExtraFailure ]`; extra declared failure present | reject with LAB detail; no `suggested_repair` |
| duplicate declaration | declared `[ MissingCapability, MissingCapability ]`; duplicate declared failure | reject with LAB detail; no `suggested_repair` |
| multi generated requests | one `when` row produces two remote writes and two failure diagnostics | reject with two LAB details; no `suggested_repair` on either detail |

These fixtures are Rust-only. They do not add sample rows and do not change
the expected JSON for `ELAB-07`.

## Implementation notes

`SurfaceLabDiagnosticFailureRowContext` now carries an internal
`associated_request_count` field. The field is skipped during serialization, so
the LAB JSON payload shape remains unchanged.

`ElaborationContext` tracks the number of generated requests associated with
the current failure-row target reference. The exact set path requires
`associated_request_count == 1`.

When a second request is observed for the same target reference, the
implementation suppresses previously emitted `set_insertion` repairs for that
target reference. This keeps output order from making the first diagnostic look
more complete than the later diagnostic.

Singleton repair paths are not widened by this guard. The suppression only
removes repair items whose `repair_shape` is `set_insertion`.

## Current executable status

| Row / shape | Current executable repair output |
|---|---|
| exact `ELAB-07` sample row | one non-final `set_insertion` item |
| `ELAB-07` proper subset variant | no `suggested_repair` |
| `ELAB-07` padded declaration variant | no `suggested_repair` |
| `ELAB-07` duplicate declaration variant | no `suggested_repair` |
| `ELAB-07` multi-request variant | no `suggested_repair` |
| `ELAB-04` | no `suggested_repair` |
| `ELAB-10` | one singleton `E-ROW-002` item |
| `ELAB-13..16` | one singleton `E-ROW-001` item per row |

Sample row count is unchanged.

## Known limitation

The current request-count key is the LAB diagnostic target reference
`when_fails_row|locus=...|event=...`. It is intentionally narrow and
conservative, but it is not a final row identity model.

Consequences:

- two distinct `when` rows with the same event name under the same role locus
  are not distinguished by this key;
- a later source-span or AST-row identity key may be needed before broader
  repair guidance can be claimed;
- true multi-target-row policy remains unresolved because the current
  `SurfaceWhenBlock` carries one failure row.

This limitation can under-suggest a repair in ambiguous same-event cases. It is
preferable to over-emitting a set repair before row identity is explicit.

## Relation to OBL-025

This package strengthens one candidate LAB repair shape by adding negative
guard evidence. It still does not prove OBL-025 and does not move canon proof
status.

OBL-025 should remain abstract until row identity, whole-gap coverage,
multi-request / multi-target policy, and repair vocabulary stability are strong
enough to mention in proof-facing terms.

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
- No final row identity model.
- No broad same-event row disambiguation.
- No true multi-target-row policy.
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
