# plan/96 - G1 E-ROW set-insertion / bundle payload inventory

## Purpose

This file inventories LAB-only payload vocabulary for possible future E-ROW
set-insertion repairs, conjunctive repair bundles, and partial-repair guidance.

It exists before any executable widening for `ELAB-04` or `ELAB-07`. It does
not widen `suggested_repair[]`, does not add set-insertion support, does not
add bundle semantics, does not edit canon, does not freeze a Diagnostic or
repair ABI, does not prove OBL-024/025, does not claim repair ranking or
multi-edit support, does not claim conformance, and does not claim G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB E-ROW repair shape inventory:
  `plan/88-g1-erow-repair-shape-inventory.md`
- LAB singleton repair gate and prototype:
  `plan/93-g1-erow001-singleton-repair-assumption.md`
  and `plan/94-g1-erow001-singleton-repair-prototype.md`
- LAB mixed / multi decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB `ELAB-07` set-insertion gate review:
  `plan/97-g1-erow07-set-insertion-gate-review.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB `ELAB-07` executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`

If this LAB inventory conflicts with canon, canon wins.

## Current executable policy

Current repair-bearing executable classes remain singleton-only:

| Sample | Shape | Repair output |
|---|---|---|
| `ELAB-10` | `E-ROW-002` / `VisibilityDenied` singleton | one LAB-only `add-to-fails-row` item |
| `ELAB-13..16` | `E-ROW-001` non-visibility singleton | one LAB-only `add-to-fails-row` item per row |

Current mixed / multi-missing fences remain no-repair:

| Sample | Shape | Missing failures | Repair output |
|---|---|---|---|
| `ELAB-07` | non-visibility multi-missing `E-ROW-001` | `MissingWitness`, `RouteUnavailable`, `StaleMembership` | no `suggested_repair` field |
| `ELAB-04` | mixed visibility / non-visibility multi-missing | `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` | no `suggested_repair` field |

This package keeps that policy. It only defines candidate vocabulary for a
later package.

## Why singleton vocabulary is insufficient

The current singleton payload says that adding one missing failure atom to one
concrete `when ... fails` row discharges the local row-containment premise:

```text
generated_failures_subset_declared_fails
```

For `ELAB-07`, emitting one independent singleton item for each missing
failure would not have the same meaning. Applying only one item would still
leave the local premise false. Therefore a future payload must distinguish:

- a complete local repair witness;
- a bundle whose children must all be applied;
- partial guidance that does not discharge the local premise;
- no repair output.

For `ELAB-04`, the same multi-missing problem is compounded by
`VisibilityDenied`, which has `E-ROW-002`-specific meaning and possible
visibility / observe-authority alternatives.

## Candidate payload roles

The names below are inventory names, not final JSON keys.

| Role | Candidate meaning | Current status |
|---|---|---|
| `repair_shape` | high-level shape: `singleton`, `set_insertion`, `conjunctive_bundle`, `partial_guidance`, or `deferred` | candidate only |
| `repair_family` | concrete family such as `add-to-fails-row` | singleton implemented; set / bundle candidate only |
| `edit_atom` | what counts as one local source edit | OPEN for sets and bundles |
| `edit_count_class` | `single_source_edit`, `multi_edit`, or `unknown` | candidate only |
| `target_kind` | target declaration surface, currently only `when_fails_row` | singleton implemented |
| `target_context` | target row reference, locus, event name, and later span/edit-script info | LAB `target_ref` exists; final span policy OPEN |
| `missing_failures_covered` | generated failures covered by this item or group | candidate only |
| `required_failures` | generated failure set for the associated request | carried in LAB context |
| `declared_failures` | current declared failure row | carried in LAB context |
| `declared_failures_after` | row after applying the item or group | singleton implemented; set / bundle candidate only |
| `local_premise` | premise targeted by the repair | singleton implemented |
| `local_premise_after_edit` | whether the item or group discharges the local premise | candidate only |
| `repair_group_id` | stable id for a group of required child repairs | candidate only |
| `bundle_semantics` | e.g. `all_required`, not alternatives | candidate only |
| `child_repairs` | atom additions inside a bundle | candidate only |
| `partiality` | marks guidance that does not discharge the local premise | candidate only |
| `visibility_branch` | separates base-failure repairs from visibility-specific repairs | candidate only |
| `ordering_policy` | whether items are unordered, ordered, or ranked | OPEN |
| `non_goal` | denies runtime success, authority, capability / witness / route / membership availability, and whole-program acceptance | required guard |
| `repair_non_final` / `lab_non_final` | prevents public ABI reading | required guard |

## Candidate shape: set insertion

A set-insertion item would be one repair item whose local effect adds all
missing failures for one request to one concrete `when ... fails` row.

Candidate conceptual shape:

```text
repair_shape = set_insertion
repair_family = add-to-fails-row
edit_atom = add_missing_failure_set_to_one_when_fails_row
edit_count_class = single_source_edit
target_kind = when_fails_row
missing_failures_covered = missing_failures
declared_failures_after = declared_failures union missing_failures
local_premise = generated_failures_subset_declared_fails
local_premise_after_edit = discharged_for_associated_request
non_goal = does_not_authorize_capability_witness_route_membership_visibility_or_claim_runtime_success
repair_non_final = true
lab_non_final = true
```

Admissibility conditions for a future `ELAB-07` set-insertion prototype would
need at least:

1. one associated generated request;
2. `target_kind == "when_fails_row"`;
3. a non-empty concrete target row reference or later target span;
4. all missing failures are base remote-request failures;
5. no `VisibilityDenied`;
6. set insertion is explicitly treated as one source edit in the declared
   fragment;
7. `declared_failures_after` includes every required failure for the associated
   request;
8. the payload still denies runtime success and authority availability.

Until condition 6 is decided, this package does not classify `ELAB-07` as a
single-edit OBL-025 coverage case.

## Candidate shape: conjunctive bundle

A conjunctive bundle would represent several atom-addition edits that must be
applied together. The group, not the individual child item, would be the local
premise-discharging witness.

Candidate conceptual shape:

```text
repair_shape = conjunctive_bundle
repair_group_id = erow-row-containment-<request-id>
bundle_semantics = all_required
edit_count_class = multi_edit
child_repairs = [
  { repair_shape = singleton, missing_failure = MissingWitness, ... },
  { repair_shape = singleton, missing_failure = RouteUnavailable, ... },
  { repair_shape = singleton, missing_failure = StaleMembership, ... }
]
declared_failures_after = declared_failures union all child missing_failures
local_premise_after_edit = discharged_by_group
non_goal = does_not_authorize_capability_witness_route_membership_visibility_or_claim_runtime_success
repair_non_final = true
lab_non_final = true
```

Important guard:

- child repairs in such a bundle are not alternatives;
- applying one child alone is not a complete local repair witness;
- OBL-025 single-edit coverage should not count a conjunctive bundle unless a
  later statement explicitly admits grouped multi-edit witnesses.

## Candidate shape: partial guidance

Partial guidance may be useful for humans, but it is not a repair witness for
OBL-025-shaped coverage unless the payload says so explicitly.

Candidate conceptual shape:

```text
repair_shape = partial_guidance
partiality = does_not_discharge_local_premise
local_premise_after_edit = still_not_discharged
```

Partial guidance should not be emitted under the same semantics as complete
repair suggestions. A later package must decide whether partial guidance
belongs in `suggested_repair[]` with an explicit marker, or in a separate
field. Until then, `ELAB-04/07` should keep omitting `suggested_repair`.

## `ELAB-07` future reading

The safest future widening for `ELAB-07`, if any, is a single grouped item:

- either a set-insertion item, if set insertion is accepted as one source edit;
- or a conjunctive bundle item with `all_required` semantics.

Do not emit one ordinary singleton item per missing failure as if those items
were alternatives or complete local repair witnesses.

If the project cannot decide whether set insertion is one edit, `ELAB-07`
should remain no-repair.

## `ELAB-04` future reading

`ELAB-04` should remain no-repair after this package.

A future mixed-row payload would need to preserve two branches:

| Branch | Missing failures | Candidate repair boundary |
|---|---|---|
| base remote-request branch | `MissingWitness`, `RouteUnavailable`, `StaleMembership` | set insertion or conjunctive bundle, as for `ELAB-07` |
| visibility branch | `VisibilityDenied` | `E-ROW-002`-like add-to-fails-row or a separate visibility / observe-authority repair family |

The mixed row needs an ordering / ranking policy if both branches can produce
repairs. Without that policy, a payload could either hide the visibility branch
inside `E-ROW-001` or emit conflicting suggestions. The current safe policy is
no repair output.

## Empty-list policy

No-repair rows should continue to omit `suggested_repair`.

This package does not standardize any meaning for:

```json
{ "suggested_repair": [] }
```

An empty array might later mean "repair analysis ran and found none", "repair
analysis is unavailable", or "field present for ABI shape only". Until the
project chooses, omission is safer for LAB no-repair rows.

## Relation to OBL-025

Current singleton evidence can be read as a narrow candidate
`CoveredLine1RepairCase` class because the emitted item is local
witness-compatible.

Future set-insertion evidence could enter the same class only if set insertion
is explicitly a single source edit and the item discharges the local premise.

Future conjunctive bundles are not single-edit evidence unless OBL-025 is
refined to admit grouped multi-edit witnesses, or unless a separate relation
covers bundle completeness. Partial guidance is not OBL-025 coverage.

This package does not change the LAB Lean statement draft and does not move
canon proof status.

## Future executable widening checklist

A later package that widens `ELAB-07` or `ELAB-04` must update all of the
following in one task:

- Rust repair emission logic;
- Rust tests for positive and no-repair boundaries;
- Python helper tests;
- expected JSON for affected rows;
- sample READMEs and matrix stage labels;
- `plan/88`, `plan/93`, `plan/94`, `plan/95`, and this file if assumptions
  change;
- `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md`;
- a new report under `docs/reports/`.

Tests must prove:

- no placeholder repair rows;
- partial items are not marked as complete local repair witnesses;
- grouped items expose `all_required` semantics when all children are required;
- `ELAB-04` does not collapse the visibility branch into base failures;
- no-repair rows still omit `suggested_repair` unless empty-list semantics are
  explicitly standardized.

## Current recommendation

Recommended next ordering:

1. Keep `ELAB-04/07` no-repair in executable output.
   `plan/97` confirms that `ELAB-07` should stay no-repair until set insertion
   is explicitly accepted as one source edit or bundle semantics are defined.
   `plan/98` confirms that `ELAB-04` should stay no-repair until base and
   visibility branches have explicit diagnostic ownership, association, and
   ordering / ranking semantics.
2. If proof-side work comes first, refine OBL-025 around:
   - single source edit;
   - set insertion;
   - grouped multi-edit witness;
   - partial guidance non-coverage.
3. If code widening comes first, start with `ELAB-07` only and only as a single
   grouped / set item, not independent singleton alternatives.
   `plan/99` now records the docs-only executable preflight for that route:
   one atomic set insertion, one target, whole-gap coverage, exact
   declared-after set, and no current output widening.
4. Leave `ELAB-04` no-repair until visibility alternatives and ranking are
   explicit.

## Open questions

- Is adding several failures to one `fails` row one source edit in the declared
  fragment?
- If a bundle has multiple child edits, is the bundle itself a repair witness
  or only a repair plan?
- Should partial guidance live in `suggested_repair[]` or a separate guidance
  field?
- What final span / edit-script representation should replace the current
  LAB-local `target_ref`?
- How should mixed rows associate `E-ROW-001` and `E-ROW-002` branches without
  duplicate or conflicting diagnostics?
- Does OBL-025 stay single-edit only for G1, or should a later obligation cover
  grouped multi-edit repair completeness?

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No set-insertion support.
- No bundle semantics support.
- No partial-guidance output support.
- No repair ranking.
- No multi-edit support.
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
