# plan/89 - G1 E-ROW-001 non-visibility singleton fixture

## Purpose

This file records the LAB-only addition of `ELAB-13`, the first
non-visibility singleton E-ROW-001 fixture. It was introduced as no-repair
boundary evidence, and `plan/94` later intentionally widened it into
repair-bearing singleton evidence.

This is LAB repository memory. It does not edit canon, does not freeze a
Diagnostic or repair ABI, does not prove OBL-024/025, does not claim
explanation soundness or completeness, does not claim conformance, and does
not claim G1 exit. The original fixture package did not widen executable
`suggested_repair[]`; the later `plan/94` prototype does.

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
- LAB E-ROW singleton repair prototype:
  `plan/94-g1-erow001-singleton-repair-prototype.md`
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
| current repair output | one LAB-only `add-to-fails-row` item after `plan/94` |

The original package made the future widening question concrete without
changing repair output. `plan/92-g1-erow001-base-singleton-fixture-closure.md`
later completed the same no-repair fixture set for the other three base
remote-request failures, and `plan/94` then flipped all four singleton rows
into LAB-only repair-bearing evidence.

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

The current tests require `ELAB-13` to stay `E-ROW-001`, to identify
`MissingWitness` as the only missing evidence, to emit one local
`add-to-fails-row` repair payload, and to satisfy the no-placeholder
constraints inherited from `plan/93`.

## Relation to OBL-025

`ELAB-13` is not OBL-025 completion. It began as a no-repair boundary fixture
that made the later `plan/94` covered-case prototype safer:

- the later widening intentionally changed this fixture rather than silently
  changing the meaning of older evidence;
- even after widening, non-visibility singleton repair is executable LAB
  evidence, not proof of OBL-025 or whole-program repair success.

## What remains open

- Whether mixed / multi-missing rows decompose into several singleton repairs
  or remain no-repair until ranking / atomicity is settled.
- Final target-span / declaration-span representation beyond LAB-local
  `target_ref`.

## Next safe packages

1. Read `ELAB-13` together with the `ELAB-14..16` closure in `plan/92` and
   the current widening in `plan/94`.
2. Preserve the `plan/93` no-placeholder payload tests if target/span
   vocabulary changes.
3. Keep `ELAB-04` no-repair; keep `ELAB-07` on the later exact set path rather
   than treating it as three singleton alternatives.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair ranking or multi-edit widening.
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
