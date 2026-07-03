# plan/99 - G1 ELAB-07 set-insertion executable preflight

## Purpose

This file records the LAB-only executable-preflight design for a possible
future `ELAB-07` set-insertion repair.

Conclusion: `ELAB-07` remains no-repair in executable output. This package does
not widen `suggested_repair[]`, does not adopt set insertion as one source
edit, does not add set-insertion support, does not add bundle semantics, does
not edit canon, does not freeze a Diagnostic or repair ABI, does not prove
OBL-024/025, does not claim conformance, and does not claim G1 exit.

The preflight exists so a later implementation package cannot accidentally
turn singleton repair evidence into multi-missing coverage without an explicit
payload model, coverage relation, and test matrix.

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
- LAB mixed / multi decomposition inventory:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- LAB set-insertion / bundle vocabulary:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB `ELAB-07` no-repair gate review:
  `plan/97-g1-erow07-set-insertion-gate-review.md`
- LAB `ELAB-04` mixed visibility branch inventory:
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- LAB `ELAB-07` assumption acceptance:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `scripts/surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/`
- Advisory review:
  sub-agent inventory from 2026-07-04 and Oracle consult
  `advisory-only-for-mirrorea-lab` completed / advisory, recorded in the
  package report

If this LAB preflight conflicts with canon, canon wins.

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
| current repair output | no `suggested_repair` field |

The current Rust and Python tests assert the same facts and assert that
`suggested_repair` is absent.

## Why `ELAB-07` is the first preflight target

`ELAB-07` is narrower than `ELAB-04`:

- it is `E-ROW-001` only;
- it has no `VisibilityDenied`;
- it has one generated request;
- it has one concrete `when_fails_row` target;
- all missing failures are base remote-request failures;
- each missing failure already has singleton repair-bearing evidence in
  `ELAB-13..16`.

That makes `ELAB-07` the smallest candidate for deciding whether adding a
duplicate-free set of missing failures to one failure row can be treated as one
source edit. `ELAB-04` should not be first because it adds mixed visibility
branch ownership, association, and ordering / ranking questions.

## Current executable guard to preserve

Current code emits repair output only when:

```text
missing_failures.len() == 1
target_kind == "when_fails_row"
target_ref is non-empty
```

The current payload also has a singleton field:

```text
missing_failure: String
```

Therefore `ELAB-07` cannot be represented by the current payload without
either losing coverage information or making three child edits look like
alternatives. The current no-repair output is correct until the set payload is
explicit.

## Candidate future payload contract

If a later package promotes executable `ELAB-07` set insertion, the first
candidate should be exactly one top-level set item, not three visible
singleton-looking items.

Candidate conceptual shape:

```text
repair_shape = set_insertion
repair_family = add-to-fails-row
diagnostic_family = E-ROW-001
edit_atom = add_duplicate_free_failure_set_to_one_when_fails_row
edit_count_class = single_source_edit_candidate
applies_to = { legacy_code, canon_id, request_id }
target_kind = when_fails_row
target_context = { target_ref, locus, event_name }
missing_failures_covered = [
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
required_failures = [
  MissingCapability,
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
declared_failures = [ MissingCapability ]
declared_failures_after = [
  MissingCapability,
  MissingWitness,
  RouteUnavailable,
  StaleMembership
]
coverage_scope = whole_missing_set_for_associated_request
local_premise = generated_failures_subset_declared_fails
local_premise_after_edit = discharged_for_associated_request
single_edit_assumption = erow001_base_failure_set_insertion_one_when_fails_row_candidate
non_goal = does_not_authorize_capability_witness_route_membership_or_claim_runtime_success
repair_non_final = true
lab_non_final = true
```

The field names are candidate vocabulary only. A later executable package must
choose actual Rust / JSON field names. Reusing `missing_failure: String` for
this shape is not acceptable.

## Minimum predicates before widening

A later executable package needs predicates at least this strong:

| Predicate | Required reading |
|---|---|
| `missing_failures = required_failures - declared_failures` | computed set difference, not a hand-written list |
| `one_associated_generated_request` | the repair targets one generated request id |
| `one_concrete_failure_row_target` | target kind is `when_fails_row` and `target_ref` is non-empty |
| `base_only_missing_set` | every missing failure is in the base remote-request failure family |
| `no_visibility_component` | `VisibilityDenied` is absent |
| `multi_missing_nonempty` | the set item is only for missing set size greater than one |
| `duplicate_free_missing_set` | duplicate declarations do not create duplicate additions |
| `no_extraneous_declared_failures` | the current declaration has no non-required failure symbols on this repair path |
| `declared_after_is_stable_exact_union` | output order is deterministic and the resulting declaration equals the required failure set |
| `whole_gap_coverage` | the payload covers every missing failure for the associated request |
| `local_premise_discharged` | the row-containment premise is true after the edit |
| `single_source_edit_accepted` | set insertion has been explicitly accepted as one source edit for this gate |
| `partial_guidance_excluded` | partial guidance does not satisfy OBL-025-shaped coverage |

`plan/100` now accepts `single_source_edit_accepted` only for the exact
`ELAB-07` candidate gate under a source-locus edit model:
`source_locus_edit_count = 1` and `element_insert_count = 3`. Executable
`ELAB-07` output still remains no-repair until a later set payload package is
implemented and validated.

The safe set arithmetic for this case is exact:

```text
required = { MissingCapability, MissingWitness, RouteUnavailable, StaleMembership }
declared = { MissingCapability }
missing = required - declared
insert = missing
declared_after = declared union insert
declared_after == required
```

If the declared row already contains an extraneous non-required failure symbol,
this repair path is not applicable; insertion alone cannot make the declaration
exact.

## Minimum tests before widening

A future implementation package must add or update tests proving:

- `ELAB-07` emits exactly one set-insertion item if and only if the set
  insertion assumption is adopted;
- the item covers exactly `MissingWitness`, `RouteUnavailable`, and
  `StaleMembership`;
- `declared_failures_after` is the stable duplicate-free union of declared and
  missing failures;
- `local_premise_after_edit` or its chosen equivalent states whole local
  premise discharge;
- child repairs, if present internally, are not serialized as alternatives;
- textual guidance such as "also add the other failures" is not counted as
  executable repair coverage;
- payload values are non-placeholder local witnesses;
- `ELAB-04` remains no-repair and does not inherit the base set-insertion
  rule;
- `ELAB-10` remains singleton `E-ROW-002` repair evidence;
- `ELAB-13..16` remain singleton `E-ROW-001` repair evidence;
- no-repair rows still omit `suggested_repair` unless empty-list semantics are
  explicitly standardized;
- OBL-025-related tests do not count partial guidance or any-overlap coverage
  as complete coverage.

The future package should keep Rust tests, Python helper tests, expected JSON,
sample README / matrix wording, `plan/` memory, snapshot docs, and report in
the same commit series.

## Implementation sequencing constraint

Do not start by changing `erow_singleton_row_addition_suggested_repair` to
accept `missing_failures.len() > 1`.

The safer implementation order for a later executable package is:

1. introduce a separate set-insertion payload model;
2. add focused tests for the candidate predicate gates;
3. add `ELAB-07` expected JSON only after the tests can distinguish whole-gap
   set coverage from child alternatives;
4. preserve the singleton path unchanged for `ELAB-10` and `ELAB-13..16`;
5. preserve no-repair output for `ELAB-04`;
6. then wire emission for base-only multi-missing rows.

This package does not perform those steps.

## Hidden failure modes

- Three singleton-looking items can be misread as alternatives even though all
  are required.
- One child item can be misread as OBL-025 coverage.
- A set item can be counted as a single edit before edit granularity is
  actually decided.
- A non-required declared failure can be ignored, causing insertion to look
  complete even though `declared_after` is not exactly the generated required
  set.
- `missing_failure: String` can freeze the singleton ABI and make set coverage
  unrepresentable.
- `declared_failures_after` can become order-dependent or duplicate-sensitive.
- A payload can cover the missing set but fail to tie itself to one generated
  request and one target row.
- Any-overlap coverage can satisfy a test even when the whole rejected gap is
  not covered.
- Set insertion for `ELAB-07` can accidentally enable mixed visibility repair
  for `ELAB-04`.
- A row repair can be overread as supplying capability, witness, route, or
  membership evidence.
- A row repair can be overread as whole-program acceptance or runtime success.
- A future `suggested_repair: []` field can standardize empty-list semantics
  accidentally.
- Textual guidance can be mistaken for executable repair coverage.

## Relation to OBL-025

OBL-025 remains Line-1 explanation completeness for single-edit repairs. The
current Lean draft can name set insertion only when it also satisfies the
single-edit witness relation and covers the whole rejected gap.

`ELAB-07` is not current OBL-025 coverage evidence. It becomes candidate
coverage evidence only if a later package explicitly accepts the set insertion
as one source edit, emits a complete local repair suggestion, and validates
whole rejected-gap coverage.

Conjunctive bundles and partial guidance remain outside current OBL-025
coverage unless a later obligation or relation admits them explicitly.

## Status classification

Use separate statuses for `ELAB-07`:

| Status | Meaning | Current reading |
|---|---|---|
| `no_repair_executable` | current diagnostics omit `suggested_repair` | yes |
| `set_insertion_payload_preflight` | candidate payload and tests are specified before code widening | yes |
| `single_source_edit_accepted` | set insertion has been accepted as one source edit | yes, but only by `plan/100` for the exact `ELAB-07` source-locus candidate gate |
| `repair_bearing_evidence` | expected JSON emits a complete set repair item | no |
| `obl025_coverage_evidence` | row counts as current OBL-025 coverage | no |

## Suggested next packages

1. Keep executable output unchanged and validate that `ELAB-07` still omits
   `suggested_repair`.
2. `plan/100` has now accepted the narrow source-locus edit assumption for
   `ELAB-07` only.
3. If executable widening is promoted, implement a separate set-insertion
   payload model and tests.
4. Keep `ELAB-04` out of the first executable widening package.
5. Refine OBL-025 only if the set-insertion predicate needs a more precise
   statement boundary.

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
