# plan/92 - G1 E-ROW-001 base singleton fixture closure

## Purpose

This file records the LAB-only closure of the original no-repair fixture set
for non-visibility singleton `E-ROW-001` row-containment omissions. `plan/94`
later intentionally widened this closed fixture set into repair-bearing
singleton evidence.

`ELAB-13` already covered `MissingWitness`. This package adds `ELAB-14..16`
for the remaining base remote-request failures:

- `MissingCapability`
- `RouteUnavailable`
- `StaleMembership`

This is executable LAB evidence only. It does not edit canon, does not freeze
a Diagnostic or repair ABI, does not prove OBL-024/025, does not claim
explanation soundness or completeness, does not claim conformance, and does
not claim G1 exit. The original closure package did not widen
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
- First singleton fixture:
  `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
- Advisory review:
  ChatGPT Pro Oracle consult `we-are-working-in-the-2`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`

If this LAB fixture closure conflicts with canon, canon wins.

## Fixture set

| Sample | Missing singleton | Declared failures | Current repair output |
|---|---|---|---|
| `ELAB-13` | `MissingWitness` | `MissingCapability`, `RouteUnavailable`, `StaleMembership` | one LAB-only `add-to-fails-row` item after `plan/94` |
| `ELAB-14` | `MissingCapability` | `MissingWitness`, `RouteUnavailable`, `StaleMembership` | one LAB-only `add-to-fails-row` item after `plan/94` |
| `ELAB-15` | `RouteUnavailable` | `MissingCapability`, `MissingWitness`, `StaleMembership` | one LAB-only `add-to-fails-row` item after `plan/94` |
| `ELAB-16` | `StaleMembership` | `MissingCapability`, `MissingWitness`, `RouteUnavailable` | one LAB-only `add-to-fails-row` item after `plan/94` |

All four rows share:

- `legacy_code = generated_failure_not_declared`
- `canon_id = E-ROW-001`
- `rule_instance = BND-001.row-containment`
- `failed_premise = generated_failures_subset_declared_fails`
- `target_kind = when_fails_row`
- `event_name = attack`
- `required_failures = [MissingCapability, MissingWitness, RouteUnavailable, StaleMembership]`
- `lab_non_final = true`
- after `plan/94`, one `suggested_repair` item with
  `single_edit_assumption =
  erow001_non_visibility_singleton_row_addition_only`

## Why this closure exists

Implementation currently handles the four base failures symmetrically as one
ordered required-failure set. However, a future repair-widening package would
likely be described as covering the class:

```text
non-visibility singleton E-ROW-001
```

For that class-wide statement, the identity of the missing atom is part of the
observable diagnostic payload: `missing_evidence`, `missing_failures`, and the
current repair item all carry it. The original no-repair fixture set therefore
kept one executable row per base failure before `plan/94` widened the class.

## Tests

The current closure and widened singleton repair evidence are covered by:

- Python helper test:
  `scripts/tests/test_surface_mir_samples.py`
- Rust sample-path regression:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- sample helper rows:
  `python3 scripts/surface_mir_samples.py run ELAB-14 --format json`
  `python3 scripts/surface_mir_samples.py run ELAB-15 --format json`
  `python3 scripts/surface_mir_samples.py run ELAB-16 --format json`
- full Surface helper:
  `python3 scripts/surface_mir_samples.py check-all --format json`

The original RED checks failed before sample creation because `ELAB-14..16`
were unknown to the helper and missing on disk for the Rust sample-path test.
The later `plan/94` RED checks failed because the closed singleton rows still
omitted `suggested_repair`.

## Relation to OBL-025

This package is not OBL-025 completion. It only improves the fixture boundary
for a later `CoveredLine1RepairCase` instantiation:

- singleton repair-bearing evidence now covers every base remote-request
  failure atom after `plan/94`;
- mixed and multi-missing rows remain no-repair until decomposition, set
  insertion atomicity, and ranking are settled.

## Next safe packages

1. Read `plan/93-g1-erow001-singleton-repair-assumption.md` and
   `plan/94-g1-erow001-singleton-repair-prototype.md` for the current
   singleton repair-bearing state.
2. Preserve `ELAB-13..16` as singleton-only repair evidence when refining
   OBL-025 wording or target/span vocabulary.
3. Keep `ELAB-04` and `ELAB-07` no-repair until multi-missing semantics are
   decided.

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
