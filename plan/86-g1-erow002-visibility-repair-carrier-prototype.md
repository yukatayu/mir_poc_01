# plan/86 - G1 E-ROW-002 visibility repair carrier prototype

## Purpose

This file records a LAB-only repair-bearing prototype for the narrow
`E-ROW-002` shape where the only missing generated failure is
`VisibilityDenied`.

This is LAB repository memory. It does not edit canon, does not freeze a
Diagnostic or repair ABI, does not prove OBL-024/025, does not claim
explanation soundness or completeness, does not claim conformance, and does not
claim G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- LAB E-ROW alignment:
  `plan/79-g1-erow-diagnostic-alignment.md`
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB carrier-only prototype:
  `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- LAB precondition hardening:
  `plan/85-g1-erow-carrier-precondition-hardening.md`

If this LAB note conflicts with canon, canon wins.

## Implemented prototype

`SurfaceLabDiagnosticDetail` now has an optional `suggested_repair` field.
The field is omitted unless the diagnostic is the `E-ROW-002` /
`VisibilityDenied`-only row-containment failure shape represented by
`ELAB-10`.

Current actual evidence is `ELAB-10`. The prototype emits one non-final local
repair candidate:

| Field | Current meaning |
|---|---|
| `repair_family` | `add-to-fails-row` |
| `diagnostic_family` | candidate E-ROW canon id, currently `E-ROW-002` |
| `applies_to` | legacy code, candidate canon id, and local request id |
| `target_kind` | `when_fails_row` |
| `target_context` | LAB-local target reference, local locus, and event name of the relevant `when` row |
| `missing_failure` | `VisibilityDenied` |
| `required_failures` | generated failures for the request |
| `declared_failures` | surrounding declared `fails` row |
| `local_effect` | `declared_failures_after` with the missing failure appended |
| `local_premise` | `generated_failures_subset_declared_fails` |
| `single_edit_assumption` | `erow002_visibility_single_row_addition_only` |
| `non_goal` | `does_not_authorize_visibility_or_claim_runtime_success` |
| `repair_non_final` | explicit non-final repair marker |
| `lab_non_final` | explicit LAB / non-public-ABI guard |

The existing carrier fields remain present: `legacy_code`, `canon_id`,
`severity`, `rule_instance`, `failed_premise`, `missing_evidence`, refs,
`request_context`, `failure_row_context`, and `lab_non_final`. The
`failure_row_context` also carries the same LAB-local `target_ref` used by the
repair item.

## Case split

| Row | Missing set | Repair output |
|---|---|---|
| `ELAB-04` | `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` | no `suggested_repair` |
| `ELAB-07` | `MissingWitness`, `RouteUnavailable`, `StaleMembership` | no `suggested_repair` |
| `ELAB-10` | `VisibilityDenied` | one LAB-only `suggested_repair` item |

This keeps mixed, multi-missing, and non-`VisibilityDenied` singleton cases out
of the repair-bearing prototype. Singleton alone is not a sufficient condition
for repair emission.

## Deferred

- Final diagnostic / repair JSON key names.
- Target span and use span.
- Declaration-site / use-site multi-span policy.
- General singleton missing-failure repairs.
- Multi-missing repairs.
- Repair ranking.
- Repair application semantics.
- OBL-025 proof or status movement.
- Any claim that applying the repair makes runtime execution safe or
  successful.

## Code and sample surfaces

- Rust report carrier:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- Rust tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- Python helper tests:
  `scripts/tests/test_surface_mir_samples.py`
- Expected JSON evidence:
  `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`

`ELAB-04` remains explicit no-repair evidence for mixed visibility /
non-visibility cases. `ELAB-07` later gained a separate exact `E-ROW-001`
set path under `plan/102`; it is not part of this `E-ROW-002` visibility
carrier. The current visibility code also gates the repair on `canon_id ==
E-ROW-002`, `missing_failures == ["VisibilityDenied"]`, `target_kind ==
"when_fails_row"`, and a non-empty `target_ref`.

## What this changes

- The `ELAB-10` E-ROW-002 / `VisibilityDenied`-only diagnostic can now carry a
  LAB-only, machine-readable add-to-fails-row repair candidate.
- Tests reject placeholder repairs by checking `applies_to`, target reference
  and target context, missing failure, required / declared failures, local
  effect, local premise, single-edit assumption, non-goal, `repair_non_final`,
  and `lab_non_final`.

## What this does not change

- No final Diagnostic ABI.
- No final repair payload ABI.
- No OBL-024 statement or proof.
- No OBL-025 statement or proof.
- No explanation soundness or completeness claim.
- No C-static conformance claim.
- No G0 exit.
- No G1 exit.
- No T1/T2 transition.
- No runtime MessageEnvelope dispatch claim.
- No final Surface runtime / transport / viewer claim.

## Follow-up

Before widening repair output, inventory non-`VisibilityDenied` singleton cases
and mixed / multi-missing cases separately. OBL-025 statement work should
mention that this prototype is `E-ROW-002` / `VisibilityDenied`-only and
LAB-local.
