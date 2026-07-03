# plan/89 - G1 E-ROW-001 non-visibility singleton fixture

## Purpose

This file records the LAB-only addition of `ELAB-13`, the first
non-visibility singleton E-ROW-001 fixture that remains no-repair evidence.

This is LAB repository memory. It does not edit canon, does not freeze a
Diagnostic or repair ABI, does not widen executable `suggested_repair[]`, does
not prove OBL-024/025, does not claim explanation soundness or completeness,
does not claim conformance, and does not claim G1 exit.

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
- LAB E-ROW repair shape inventory:
  `plan/88-g1-erow-repair-shape-inventory.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`

If this LAB fixture conflicts with canon, canon wins.

## Added fixture

`ELAB-13` lives at:

```text
samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/
```

The source declares a generated remote write request whose required base
failures are:

```text
MissingCapability, MissingWitness, RouteUnavailable, StaleMembership
```

The surrounding `when ... fails` row declares every base failure except
`MissingWitness`:

```text
MissingCapability, RouteUnavailable, StaleMembership
```

Therefore the fixture is a non-visibility singleton omission:

```text
missing_failures = [MissingWitness]
```

## Expected reading

| Field | Expected value |
|---|---|
| `sample_id` | `ELAB-13` |
| candidate canon family | `E-ROW-001` |
| missing-set shape | non-visibility singleton |
| missing evidence | `MissingWitness` |
| target kind | `when_fails_row` |
| target event | `attack` |
| current repair output | no `suggested_repair` field |

This made the future widening question concrete without changing repair output.
`plan/92-g1-erow001-base-singleton-fixture-closure.md` later completes the
same no-repair fixture set for the other three base remote-request failures.

## Tests

The fixture is covered by:

- Python helper test:
  `scripts/tests/test_surface_mir_samples.py`
- Rust regression test:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- sample helper row:
  `python3 scripts/surface_mir_samples.py run ELAB-13 --format json`
- full Surface helper:
  `python3 scripts/surface_mir_samples.py check-all --format json`

The tests require `ELAB-13` to stay `E-ROW-001`, to identify
`MissingWitness` as the only missing evidence, and to omit
`suggested_repair`.

## Relation to OBL-025

`ELAB-13` is not OBL-025 completion. It is a no-repair boundary fixture that
makes a future OBL-025 covered-case decision safer:

- if a later package widens `suggested_repair[]` for non-visibility singleton
  E-ROW-001, it must change this fixture intentionally;
- until then, non-visibility singleton is executable pressure evidence, not
  repair-coverage evidence.

## What remains open

- What single-edit assumption should be used for non-visibility singleton
  repairs.
- Whether a later repair-bearing prototype should use one parametric rule or
  one row per missing base failure.
- Whether mixed / multi-missing rows decompose into several singleton repairs
  or remain no-repair until ranking / atomicity is settled.
- Final target-span / declaration-span representation beyond LAB-local
  `target_ref`.

## Next safe packages

1. Read `ELAB-13` together with the `ELAB-14..16` closure in `plan/92`.
2. Widen non-visibility singleton `suggested_repair[]` only after a separate
   package defines the single-edit assumption and no-placeholder payload tests.
3. Keep `ELAB-04` and `ELAB-07` no-repair until set-insertion atomicity,
   decomposition, and ranking are resolved.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
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
