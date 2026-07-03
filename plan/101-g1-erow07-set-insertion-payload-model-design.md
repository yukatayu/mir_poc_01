# plan/101 - G1 ELAB-07 set-insertion payload-model design

## Purpose

This file records the LAB-only payload-model design for a possible future
`ELAB-07` set-insertion repair item.

This is design-only repository memory. Current executable `ELAB-07` still
emits no `suggested_repair`. This package does not edit Rust, expected JSON,
sample matrices, Lean statements, or canon.

The design exists so a later implementation package cannot widen the singleton
repair path by relaxing `missing_failures.len() == 1`. A future executable
package must introduce a separate set payload shape and tests.

Non-claims:

- no current repair output widening;
- no executable set-insertion support;
- no bundle semantics support;
- no partial-guidance coverage;
- no repair ranking or visibility ranking;
- no final Diagnostic or repair payload ABI;
- no OBL-024/025 proof or completion;
- no conformance claim;
- no canon edit;
- no G1 exit.

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
- LAB OBL-025 statement draft:
  `plan/87-g1-obl025-lean-statement-draft.md`,
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- LAB singleton repair gate and prototype:
  `plan/93-g1-erow001-singleton-repair-assumption.md`,
  `plan/94-g1-erow001-singleton-repair-prototype.md`
- LAB repair shape and payload vocabulary:
  `plan/83-g1-erow-repair-payload-inventory.md`,
  `plan/88-g1-erow-repair-shape-inventory.md`,
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-07` gate / preflight / assumption:
  `plan/97-g1-erow07-set-insertion-gate-review.md`,
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`,
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `scripts/surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`
- Advisory review:
  sub-agent review from 2026-07-04 and Oracle consult
  `we-are-in-a-specificat` completed / advisory, recorded in the package report

If this LAB design conflicts with canon, canon wins.

## Decision level

This is `L3` LAB repository memory.

Canon fixes BND-001 row containment and diagnostic direction. `plan/100`
accepts a narrow LAB edit-cardinality premise for the exact `ELAB-07`
candidate gate. This file does not add canon semantics; it designs the
candidate payload roles needed before a later executable package may use that
premise.

## Current executable baseline

Current repair-bearing executable rows are singleton-only:

| Row | Family | Missing failures | Current repair output |
|---|---|---|---|
| `ELAB-10` | `E-ROW-002` | `VisibilityDenied` | one LAB-only singleton `add-to-fails-row` item |
| `ELAB-13` | `E-ROW-001` | `MissingWitness` | one LAB-only singleton `add-to-fails-row` item |
| `ELAB-14` | `E-ROW-001` | `MissingCapability` | one LAB-only singleton `add-to-fails-row` item |
| `ELAB-15` | `E-ROW-001` | `RouteUnavailable` | one LAB-only singleton `add-to-fails-row` item |
| `ELAB-16` | `E-ROW-001` | `StaleMembership` | one LAB-only singleton `add-to-fails-row` item |

Current no-repair fences remain:

| Row | Shape | Current repair output |
|---|---|---|
| `ELAB-07` | non-visibility multi-missing `E-ROW-001` | no `suggested_repair` field |
| `ELAB-04` | mixed base / `VisibilityDenied` multi-missing row | no `suggested_repair` field |

Current Rust payload baseline:

```text
SurfaceLabSuggestedRepair {
  repair_family
  diagnostic_family
  applies_to
  target_kind
  target_context
  missing_failure
  required_failures
  declared_failures
  local_effect.declared_failures_after
  local_premise
  single_edit_assumption
  non_goal
  repair_non_final
  lab_non_final
}
```

`missing_failure` is singular. A set payload must not reuse this field for
multiple inserted failures.

## Designed payload boundary

The future `ELAB-07` set item should be exactly one top-level item, not three
serialized singleton child repairs.

The names below are candidate roles, not final Rust field names, JSON keys, or
public ABI.

| Role | Candidate value / meaning |
|---|---|
| `repair_shape` | `set_insertion` |
| `repair_family` | `add-to-fails-row` |
| `diagnostic_family` | `E-ROW-001` |
| `edit_atom` | `complete_missing_base_failure_set_into_one_existing_when_fails_row` |
| `source_locus_edit_count` | `1` |
| `element_insert_count` | `3` |
| `applies_to` | `{ legacy_code, canon_id, request_id }` |
| `target_kind` | `when_fails_row` |
| `target_context` | `{ target_ref, locus, event_name }` |
| `declared_failures_before` | `[ MissingCapability ]` |
| `insert_failures` | `[ MissingWitness, RouteUnavailable, StaleMembership ]` |
| `required_failures` | `[ MissingCapability, MissingWitness, RouteUnavailable, StaleMembership ]` |
| `declared_failures_after` | stable duplicate-free union, exactly equal to `required_failures` |
| `coverage_scope` | `complete_missing_set_for_associated_request` |
| `local_premise` | `generated_failures_subset_declared_fails` |
| `local_premise_after_edit` | `discharged_for_associated_request` |
| `single_edit_assumption` | `erow001_elab07_complete_base_failure_set_source_locus_edit` |
| `non_goal` | denies capability / witness / route / membership availability and runtime success |
| `repair_non_final` | `true` |
| `lab_non_final` | `true` |

Do not include these roles in the first `ELAB-07` set item:

- `repair_group_id`;
- `bundle_semantics`;
- `child_repairs`;
- `partiality`;
- `visibility_branch`;
- `ordering_policy`;
- repair ranking or visibility ranking fields.

Those roles belong to conjunctive bundles, partial guidance, or `ELAB-04`
mixed-branch design, not to this first `ELAB-07` set item.

## Eligibility guards

A later executable package may emit the future set item only when all guards
below hold:

| Guard | Required reading |
|---|---|
| `sample_scope_elab07_only` | exact current `ELAB-07` fact pattern only |
| `diagnostic_family_erow001` | `canon_id == "E-ROW-001"` |
| `write_side_non_visibility` | request kind is write; no visibility failure participates |
| `one_associated_generated_request` | exactly one generated request is attached |
| `one_existing_concrete_row` | exactly one existing concrete `when_fails_row` target |
| `non_empty_target_ref` | `target_ref` is non-empty and stable |
| `base_failure_set_only` | only `MissingCapability`, `MissingWitness`, `RouteUnavailable`, `StaleMembership` appear |
| `visibility_denied_absent` | `VisibilityDenied` absent from required, declared, missing, and inserted sets |
| `missing_is_computed_difference` | `missing == required - declared_before` |
| `whole_gap_inserted` | `insert_failures == missing` |
| `proper_subset_rejected` | one- or two-element subsets are not complete repairs |
| `stable_duplicate_free_union` | deterministic order and no duplicates |
| `declared_after_exact` | `declared_failures_after == required_failures` |
| `no_extraneous_declared_failures` | insertion path rejects padded declarations |
| `source_locus_edit_count_one` | one existing row-field / source-locus edit |
| `element_insert_count_three` | three inserted failure identifiers |
| `local_premise_discharged` | BND-001 row containment is true for the associated request after the edit |
| `non_goal_preserved` | payload does not claim authority, evidence, runtime success, or whole-program success |
| `current_output_no_repair_until_implementation` | current executable `ELAB-07` still omits `suggested_repair` |

This guard set is stronger than BND-001 subset containment because it requires
exact `declared_after == required`. That exactness prevents padded
over-declaration from being counted as this candidate local witness.

## Set arithmetic

The only accepted set arithmetic for this design is:

```text
declared_before =
  { MissingCapability }

required =
  { MissingCapability, MissingWitness, RouteUnavailable, StaleMembership }

missing =
  required - declared_before
  = { MissingWitness, RouteUnavailable, StaleMembership }

insert_failures =
  missing

declared_failures_after =
  stable_duplicate_free_union(declared_before, insert_failures)
  = { MissingCapability, MissingWitness, RouteUnavailable, StaleMembership }
```

The intended future local effect is limited to the declaration row. It does not
create a row, delete a row, split a row, move a row, retarget a request, change
generated-failure computation, or supply capability / witness / route /
membership evidence.

## Future positive tests

The future implementation package must specify and then implement tests for at
least:

| ID | Case | Required future result |
|---|---|---|
| `P1` | exact current `ELAB-07` fact pattern | exactly one top-level `set_insertion` item |
| `P2` | inserted failures | exactly `MissingWitness`, `RouteUnavailable`, `StaleMembership` |
| `P3` | no singleton field reuse | set item does not use singular `missing_failure` for multi-failure payload |
| `P4` | declared-after arithmetic | stable duplicate-free `declared_failures_after == required_failures` |
| `P5` | coverage | whole missing set for the associated request is covered |
| `P6` | local premise | local row-containment premise is discharged for the associated request |
| `P7` | count fields | `source_locus_edit_count = 1`, `element_insert_count = 3` |
| `P8` | non-final markers | `repair_non_final == true`, `lab_non_final == true` |
| `P9` | no-placeholder values | all payload strings are concrete local witnesses |

## Future negative and regression tests

The future implementation package must reject or preserve at least:

| ID | Case | Required future result |
|---|---|---|
| `N1` | `{ MissingWitness }` only | no complete set repair |
| `N2` | `{ RouteUnavailable }` only | no complete set repair |
| `N3` | `{ StaleMembership }` only | no complete set repair |
| `N4` | any two-element proper subset | no complete set repair |
| `N5` | duplicate insertion | reject or normalize before payload, never emit duplicate evidence |
| `N6` | padded insertion with extra failure | reject this set payload |
| `N7` | any `VisibilityDenied` component | out of scope for `ELAB-07` set gate |
| `N8` | `ELAB-04` mixed base / visibility row | remains no-repair |
| `N9` | multi-request diagnostic | no set payload until association policy exists |
| `N10` | multi-target-row diagnostic | no set payload until target policy exists |
| `N11` | missing / inferred target row | no set payload |
| `N12` | row creation / splitting / movement / retargeting | no set payload |
| `N13` | three serialized child singleton repairs | reject as misleading alternatives |
| `N14` | one or two child singleton repairs | reject as partial coverage |
| `N15` | textual guidance only | not executable repair coverage |
| `N16` | no-repair row gains `suggested_repair: []` | reject unless empty-list semantics are separately standardized |
| `R1` | `ELAB-10` | remains singleton `E-ROW-002` repair evidence |
| `R2` | `ELAB-13..16` | remain singleton `E-ROW-001` repair evidence |
| `R3` | current `ELAB-07` before implementation | still no `suggested_repair` |

This package records the matrix only. It does not add these tests yet.

## Implementation sequencing constraint

A later executable package should use this order:

1. Introduce a separate set payload model.
2. Add focused Rust tests for positive / negative predicate gates.
3. Add focused Python helper tests for the same gates.
4. Update `ELAB-07` expected JSON only after tests distinguish whole-gap set
   coverage from singleton alternatives.
5. Preserve the singleton path unchanged for `ELAB-10` and `ELAB-13..16`.
6. Preserve no-repair output for `ELAB-04`.
7. Wire emission only for base-only multi-missing rows that satisfy all
   `ELAB-07` guards.

Do not start by modifying `erow_singleton_row_addition_suggested_repair` to
accept `missing_failures.len() > 1`.

## Relation to OBL-025

OBL-025 remains Line-1 explanation completeness for single-edit repairs.

This design only prepares a future payload that could instantiate the abstract
`EligibleSetInsertionRepair` shape already present in the LAB OBL-025
statement draft, after executable emission and tests exist.

It does not prove OBL-025, does not move canon proof status, and does not make
current `ELAB-07` executable output OBL-025 coverage evidence.

## Hidden failure modes

- "One edit" is misread as one inserted failure identifier instead of one
  source-locus row-field edit.
- The exact `ELAB-07` assumption is misread as general set-insertion support.
- Three required additions are serialized as alternative singleton repairs.
- One or two additions are counted as whole-gap coverage.
- BND-001 subset containment hides padded over-declaration unless
  `declared_after == required`.
- `VisibilityDenied` leaks this gate into `ELAB-04` or `E-ROW-002`.
- A non-concrete or inferred target row makes source-locus identity unstable.
- Wording contradicts current no-repair executable output.
- List ordering makes payload evidence nondeterministic.
- Empty-list repair output standardizes an accidental ABI.
- A row repair is overread as capability, witness, route, membership,
  authorization, runtime success, or whole-program acceptance.
- OBL-025 is treated as complete merely because a payload design exists.

## Suggested next packages

1. Keep executable output unchanged and validate that `ELAB-07` still omits
   `suggested_repair`.
2. If implementation is promoted, build the exact `ELAB-07` set-insertion
   payload prototype using this design and test matrix.
3. Keep `ELAB-04` out of the first executable widening package.
4. Keep OBL-025 abstract until the executable payload exists and reveals
   whether the statement vocabulary needs refinement.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No current executable `ELAB-07` `suggested_repair` output.
- No executable set-insertion support.
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
