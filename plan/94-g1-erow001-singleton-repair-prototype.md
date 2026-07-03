# plan/94 - G1 E-ROW-001 singleton repair prototype

## Purpose

This file records the LAB-only widening of `suggested_repair[]` for the
non-visibility singleton `E-ROW-001` row-containment class.

It implements the `plan/93` single-edit / no-placeholder gate for the four
base remote-request failure atoms:

- `MissingCapability`
- `MissingWitness`
- `RouteUnavailable`
- `StaleMembership`

This is executable LAB evidence only. It does not edit canon, does not freeze
a Diagnostic or repair ABI, does not prove OBL-024/025, does not claim
explanation soundness or completeness, does not claim conformance, does not
claim repair ranking or multi-edit support, and does not claim G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB E-ROW repair shape inventory:
  `plan/88-g1-erow-repair-shape-inventory.md`
- LAB mixed / multi decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB singleton fixture memory:
  `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
  and `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- LAB widening gate:
  `plan/93-g1-erow001-singleton-repair-assumption.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`

If this LAB prototype conflicts with canon, canon wins.

## Widened executable state

The current repair-bearing executable evidence is now:

| Sample | Canon family | Missing failures | Current repair output |
|---|---|---|---|
| `ELAB-10` | `E-ROW-002` | `VisibilityDenied` | one LAB-only `add-to-fails-row` item |
| `ELAB-13` | `E-ROW-001` | `MissingWitness` | one LAB-only `add-to-fails-row` item |
| `ELAB-14` | `E-ROW-001` | `MissingCapability` | one LAB-only `add-to-fails-row` item |
| `ELAB-15` | `E-ROW-001` | `RouteUnavailable` | one LAB-only `add-to-fails-row` item |
| `ELAB-16` | `E-ROW-001` | `StaleMembership` | one LAB-only `add-to-fails-row` item |

The current no-repair fences remain:

| Sample | Reading | Current repair output |
|---|---|---|
| `ELAB-07` | non-visibility multi-missing | no `suggested_repair` field |
| `ELAB-04` | mixed visibility / non-visibility multi-missing | no `suggested_repair` field |

## Prototype gate

The implementation emits an `E-ROW-001` repair only when all of the following
hold:

1. `canon_id == "E-ROW-001"`;
2. `failure_row_context.target_kind == "when_fails_row"`;
3. `failure_row_context.target_ref` is non-empty;
4. `failure_row_context.missing_failures.len() == 1`;
5. the missing failure is a base remote-request failure;
6. the missing failure is not `VisibilityDenied`;
7. the emitted repair payload matches `plan/93` local-witness constraints.

The payload uses:

```text
repair_family = add-to-fails-row
single_edit_assumption = erow001_non_visibility_singleton_row_addition_only
non_goal = does_not_authorize_capability_witness_route_membership_or_claim_runtime_success
```

The local effect remains:

```text
declared_failures_after = declared_failures + [missing_failure]
```

This is intentionally local. It says the row-containment premise can be
addressed by one declaration edit. It does not say the program will execute
successfully, that capability / witness / route / membership evidence exists,
or that later runtime authority checks pass.

## Tests

The prototype is covered by:

- Python helper test:
  `test_elaboration_non_visibility_singleton_failure_row_reports_repair_payload`
- Python helper no-placeholder test:
  `test_erow_suggested_repair_payloads_are_not_placeholders`
- Rust sample-path regression:
  `sample_fixtures_cover_each_non_visibility_singleton_with_repair_payload`
- Rust no-placeholder regression:
  `suggested_repair_payloads_are_non_placeholder_local_witnesses`
- Surface helper rows:
  `python3 scripts/surface_mir_samples.py run ELAB-13 --format json`
  through `ELAB-16`
- full Surface helper:
  `python3 scripts/surface_mir_samples.py check-all --format json`

The tests still require `ELAB-04` and `ELAB-07` to omit `suggested_repair`.

## Relation to OBL-025

This prototype is not OBL-025 completion. It supplies executable LAB evidence
for one candidate `CoveredLine1RepairCase` class:

```text
E-ROW-001 non-visibility singleton row-containment omission
```

The evidence remains narrower than OBL-025:

- no proof that every Line-1 single-edit repair is covered;
- no final diagnostic association theorem;
- no repair application semantics;
- no ranking or multi-edit coverage;
- no claim that the suggested edit makes the whole program accepted.

## What remains open

- Whether set insertion can be one edit for multi-missing rows; `plan/95`
  keeps this open and no-repair.
- Whether mixed visibility / non-visibility rows decompose into several
  singleton repairs; `plan/95` keeps this open and no-repair.
- Whether target spans should replace or supplement current LAB-local
  `target_ref`.
- How repair application semantics will be represented if later edit scripts
  are validated.
- Whether the next proof-side step should refine OBL-025 or draft OBL-024.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No OBL-024 proof.
- No OBL-025 proof.
- No OBL-025 completion.
- No explanation soundness claim.
- No explanation completeness claim.
- No repair ranking.
- No multi-edit support.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
- No whole-program success after repair claim.
