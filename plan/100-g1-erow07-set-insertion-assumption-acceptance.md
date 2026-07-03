# plan/100 - G1 ELAB-07 set-insertion assumption acceptance

## Purpose

This file records the LAB-only acceptance of one narrow edit-cardinality
assumption for a possible future `ELAB-07` repair payload.

Accepted assumption:

```text
For the ELAB-07 candidate gate only, completing the one existing concrete
when_fails_row by duplicate-free insertion of the exact missing base-failure
gap {MissingWitness, RouteUnavailable, StaleMembership} into the declared set
{MissingCapability} counts as one LAB source-locus edit. The same operation
has element_insert_count = 3 and yields declared_after exactly
{MissingCapability, MissingWitness, RouteUnavailable, StaleMembership}.
```

This acceptance is docs-only. The current `ELAB-07` executable artifact still
emits no `suggested_repair`. This package does not widen executable repair
output, does not add set-insertion support, does not add bundle semantics, does
not edit canon, does not freeze a Diagnostic or repair ABI, does not prove
OBL-024/025, does not claim conformance, and does not claim G1 exit.

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
- LAB set-insertion / bundle vocabulary:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-07` gate / preflight:
  `plan/97-g1-erow07-set-insertion-gate-review.md`,
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `scripts/surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/`
- Advisory review:
  sub-agent review from 2026-07-04 and Oracle retry
  `retry-without-attachment-after-browser` completed / advisory, recorded in
  the package report

If this LAB acceptance conflicts with canon, canon wins.

## Decision level

This is `L3` LAB repository memory.

Canon fixes BND-001 row containment and diagnostic direction. Canon does not
decide whether mutating one declared-failure row by several inserted failure
identifiers counts as one source edit. This file accepts that reading only for
the `ELAB-07` candidate gate.

## Current `ELAB-07` facts

`ELAB-07` is a write-side non-visibility row-containment rejection:

```text
BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
```

Current expected diagnostic detail:

| Field | Current value |
|---|---|
| `canon_id` | `E-ROW-001` |
| `request_kind` | `write` |
| `generated_from` | `nested_place_block` |
| `target_kind` | `when_fails_row` |
| `target_ref` | `when_fails_row|locus=role:BrowserClient|event=attack` |
| `required_failures` | `MissingCapability`, `MissingWitness`, `RouteUnavailable`, `StaleMembership` |
| `declared_failures` | `MissingCapability` |
| `missing_failures` | `MissingWitness`, `RouteUnavailable`, `StaleMembership` |
| `local_premise` | `generated_failures_subset_declared_fails` |
| current executable repair output | no `suggested_repair` field |

The current Rust and Python tests assert that `suggested_repair` is absent.
This file does not change that executable fact.

## Accepted edit-cardinality model

Use two edit counts:

| Count | Value | Meaning |
|---|---:|---|
| `source_locus_edit_count` | `1` | one mutation of one existing concrete `when_fails_row` declared-failure set |
| `element_insert_count` | `3` | three failure identifiers are inserted into that set |

The accepted claim is one source-locus edit, not one inserted identifier and
not general set-insertion support.

The row-field/source-locus model is accepted here because the edit changes one
existing declaration locus:

```text
when attack(target: Participant) fails MissingCapability
```

into a declaration whose failure set is exactly the generated required set:

```text
when attack(target: Participant)
  fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership
```

This does not create a row, delete a row, split a row, move a row, retarget a
request, change route semantics, change witness semantics, change capability
semantics, or change generated-failure computation.

## Exact set arithmetic

The accepted candidate gate uses exact set arithmetic:

```text
declared_before =
  { MissingCapability }

required =
  { MissingCapability, MissingWitness, RouteUnavailable, StaleMembership }

missing =
  required - declared_before
  = { MissingWitness, RouteUnavailable, StaleMembership }

insert =
  missing

declared_after =
  stable_duplicate_free_union(declared_before, insert)
  = { MissingCapability, MissingWitness, RouteUnavailable, StaleMembership }
```

Required facts:

```text
declared_before subset required
declared_before != required
missing != empty
missing == required - declared_before
declared_after == required
declared_after contains no duplicates
declared_after contains no failures outside required
```

The `declared_after == required` condition is stricter than BND-001 subset
containment. It is intentional for this candidate gate because the future
repair payload must prove exact gap closure rather than permissive padding.

## Eligibility predicates

All predicates below must hold before a later executable package may emit a
set-insertion repair for `ELAB-07`:

| Predicate | Required reading |
|---|---|
| `sample_scope_elab07_only` | the assumption applies only to the current `ELAB-07` fact pattern |
| `diagnostic_family_erow001` | `canon_id == "E-ROW-001"` |
| `write_side_non_visibility` | request kind is write and no visibility failure participates |
| `generated_request_count_one` | exactly one generated request is associated with the diagnostic |
| `target_when_fails_row_count_one` | exactly one target `when_fails_row` is implicated |
| `target_row_concrete_and_existing` | target row already exists and has a concrete `target_ref` |
| `base_failure_set_only` | required / declared / missing sets use only `MissingCapability`, `MissingWitness`, `RouteUnavailable`, `StaleMembership` |
| `visibility_denied_absent` | `VisibilityDenied` is absent from required, declared, and missing sets |
| `missing_is_computed_difference` | missing failures are computed as `required - declared_before` |
| `whole_gap_inserted` | insertion set is exactly the full missing set |
| `stable_duplicate_free_union` | `declared_after` is deterministic and duplicate-free |
| `no_extraneous_declared_failures` | `declared_after == required`, not merely `required subset declared_after` |
| `source_locus_edit_count_one` | one existing declared-failure set is mutated |
| `element_insert_count_three` | the element delta remains three identifiers |
| `local_premise_after_edit` | BND-001 row containment is discharged for the associated request after the hypothetical edit |
| `current_output_no_repair` | current executable artifact still omits `suggested_repair` |
| `later_payload_required` | executable support needs a later separate payload implementation package |

## Disallowed near misses

Partial insertions are not accepted:

```text
{MissingWitness}
{RouteUnavailable}
{StaleMembership}
{MissingWitness, RouteUnavailable}
{MissingWitness, StaleMembership}
{RouteUnavailable, StaleMembership}
```

Duplicate or padded results are not accepted:

```text
{MissingCapability, MissingWitness, MissingWitness, RouteUnavailable, StaleMembership}
{MissingCapability, MissingWitness, RouteUnavailable, StaleMembership, VisibilityDenied}
```

Shape changes are not accepted:

- creating a missing `when_fails_row`;
- splitting one row into several rows;
- moving the handler or request target;
- changing generated request association;
- changing required failure computation;
- adding capability, witness, route, membership, or visibility evidence.

## Future payload expectation

A later executable widening package may use this assumption, but only if it
adds a separate set payload. Reusing the singleton field
`missing_failure: String` is not acceptable.

Candidate future payload roles:

```text
repair_shape = set_insertion
repair_family = add-to-fails-row
diagnostic_family = E-ROW-001
edit_atom = complete_missing_base_failure_set_into_one_existing_when_fails_row
source_locus_edit_count = 1
element_insert_count = 3
applies_to = { legacy_code, canon_id, request_id }
target_kind = when_fails_row
target_context = { target_ref, locus, event_name }
insert_failures = [
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
declared_failures_before = [ MissingCapability ]
required_failures = [
  MissingCapability,
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
declared_failures_after = [
  MissingCapability,
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
coverage_scope = complete_missing_set_for_associated_request
local_premise = generated_failures_subset_declared_fails
local_premise_after_edit = discharged_for_associated_request
single_edit_assumption = erow001_elab07_complete_base_failure_set_source_locus_edit
non_goal = does_not_authorize_capability_witness_route_membership_or_claim_runtime_success
repair_non_final = true
lab_non_final = true
```

These are candidate roles, not final JSON field names and not current output.

## Relation to current executable rows

| Row | Current status after this package |
|---|---|
| `ELAB-07` | current executable output still no-repair; candidate assumption accepted for a later set payload |
| `ELAB-04` | remains mixed base / `VisibilityDenied` no-repair; this assumption does not apply |
| `ELAB-10` | remains singleton `E-ROW-002` / `VisibilityDenied` repair evidence |
| `ELAB-13..16` | remain singleton `E-ROW-001` base-failure repair evidence |

## Relation to OBL-025

OBL-025 remains Line-1 explanation completeness for single-edit repairs.

This package accepts only the LAB edit-cardinality premise that would let a
future `ELAB-07` set-insertion payload instantiate the abstract
`SingleEditRepairWitness` / `SetInsertionRepairWitness` shape already present
in the LAB OBL-025 statement draft.

It does not prove OBL-025, does not move canon proof status, and does not make
current `ELAB-07` executable output OBL-025 coverage evidence. `ELAB-07` can
become candidate coverage evidence only after a later package emits and tests
a complete local repair suggestion.

## Required future tests before executable widening

A later executable package must add positive and negative tests for at least:

- positive `ELAB-07` classification under this exact fact pattern;
- partial-insertion rejection for every proper non-empty subset of the missing
  set;
- duplicate failure rejection;
- extraneous failure rejection;
- `VisibilityDenied` out-of-scope rejection;
- multi-request rejection;
- multi-target-row rejection;
- missing-target-row / row-creation rejection;
- stable duplicate-free ordering;
- `ELAB-04` still no-repair;
- `ELAB-10` still singleton `E-ROW-002`;
- `ELAB-13..16` still singleton `E-ROW-001`;
- no no-repair row gains `suggested_repair: []` unless empty-list semantics
  are separately standardized.

## Hidden failure modes

- "One edit" can be misread as one inserted identifier instead of one
  source-locus row-field edit.
- A complete set insertion can be misread as general set-insertion support.
- Three required child additions can be serialized as alternatives.
- One or two child additions can be counted as whole-gap coverage.
- BND-001 subset containment can hide padded over-declaration unless the gate
  requires exact `declared_after == required`.
- `VisibilityDenied` can leak this gate into `ELAB-04` / `E-ROW-002`.
- A non-concrete or inferred target row can make source-locus edit identity
  unstable.
- The current no-repair executable output can be contradicted by wording that
  implies current `suggested_repair` coverage.
- Unstable list ordering can create nondeterministic payload evidence.

## Suggested next packages

1. Keep executable output unchanged and validate that `ELAB-07` still omits
   `suggested_repair`.
2. If executable widening is promoted, implement a separate set-insertion
   payload model and tests for `ELAB-07` only.
3. Keep `ELAB-04` out of the first executable widening package.
4. Refine OBL-025 only if the later payload package needs more precise
   statement vocabulary.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation widening.
- No current executable `ELAB-07` `suggested_repair` output.
- No general set-insertion support.
- No bundle semantics support.
- No partial-guidance output support.
- No repair ranking.
- No multi-edit support.
- No row creation, row splitting, row movement, or row retargeting support.
- No `VisibilityDenied` / `E-ROW-002` widening.
- No `ELAB-04` reclassification.
- No `ELAB-10` change.
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
