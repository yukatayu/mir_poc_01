# plan/93 - G1 E-ROW-001 singleton repair assumption gate

## Purpose

This file defines the LAB-only gate for a possible future
non-visibility singleton `E-ROW-001` repair prototype.

It records:

- the single-edit assumption for adding one base remote-request failure to a
  `when ... fails` row;
- the payload constraints that prevent placeholder `suggested_repair[]` rows;
- the current executable guard tests that keep `ELAB-13..16` as no-repair
  fences until a later widening package intentionally changes them.

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
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB E-ROW repair shape inventory:
  `plan/88-g1-erow-repair-shape-inventory.md`
- LAB singleton fixture memory:
  `plan/89-g1-erow001-non-visibility-singleton-fixture.md`
  and `plan/92-g1-erow001-base-singleton-fixture-closure.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`

If this LAB gate conflicts with canon, canon wins.

## Current executable state

Current repair-bearing executable evidence remains exactly:

| Sample | Canon family | Missing failures | Current repair output |
|---|---|---|---|
| `ELAB-10` | `E-ROW-002` | `VisibilityDenied` | one LAB-only `add-to-fails-row` item |
| `ELAB-13` | `E-ROW-001` | `MissingWitness` | no `suggested_repair` field |
| `ELAB-14` | `E-ROW-001` | `MissingCapability` | no `suggested_repair` field |
| `ELAB-15` | `E-ROW-001` | `RouteUnavailable` | no `suggested_repair` field |
| `ELAB-16` | `E-ROW-001` | `StaleMembership` | no `suggested_repair` field |
| `ELAB-07` | `E-ROW-001` | non-visibility multi-missing | no `suggested_repair` field |
| `ELAB-04` | current LAB `E-ROW-001` split | mixed visibility / non-visibility multi-missing | no `suggested_repair` field |

`ELAB-13..16` are now gate-ready inputs for a later non-visibility singleton
repair prototype, not repair-bearing evidence today.

## LAB single-edit assumption

For a future non-visibility singleton `E-ROW-001` prototype, the only
single-edit case currently admitted by this gate is:

1. the diagnostic is a row-containment failure with
   `canon_id == "E-ROW-001"`;
2. `failure_row_context.target_kind == "when_fails_row"`;
3. `failure_row_context.target_ref` is non-empty and identifies the local row;
4. there is exactly one generated remote request associated with the
   diagnostic item;
5. `failure_row_context.missing_failures.len() == 1`;
6. the missing failure is one of the four base remote-request failures:
   `MissingCapability`, `MissingWitness`, `RouteUnavailable`, or
   `StaleMembership`;
7. the missing failure is not `VisibilityDenied`;
8. the local edit is adding that one missing failure atom to the same
   `when ... fails` row;
9. the local effect of the edit is only:
   `declared_failures_after = declared_failures + [missing_failure]`;
10. the repair targets only the reported local premise
    `generated_failures_subset_declared_fails`;
11. the repair does not claim runtime success, capability availability,
    route availability, witness availability, authorization, or whole-program
    acceptance.

This assumption treats adding one failure atom to one existing row as a single
local declaration edit. It does not decide whether adding a set of failures is
one edit, whether edits can be ranked, or whether several singleton repairs
can be emitted for a multi-missing row.

## No-placeholder payload constraints

Any future `suggested_repair[]` item admitted by this gate must be non-empty
and must be a local witness-compatible payload. In LAB vocabulary, that means
at least:

| Payload role | Constraint |
|---|---|
| `repair_family` | `add-to-fails-row` for this gate |
| `diagnostic_family` | equals the enclosing diagnostic `canon_id` |
| `applies_to.legacy_code` | equals the enclosing `legacy_code` |
| `applies_to.canon_id` | equals the enclosing `canon_id` |
| `applies_to.request_id` | equals the enclosing `request_context.request_id` |
| `target_kind` | equals `failure_row_context.target_kind` |
| `target_context.target_ref` | equals `failure_row_context.target_ref` and is non-empty |
| `target_context.locus` | equals `failure_row_context.target_locus` |
| `target_context.event_name` | equals `failure_row_context.event_name` |
| `missing_failure` | equals the singleton `missing_evidence` and `missing_failures` value |
| `required_failures` | equals `failure_row_context.required_failures` |
| `declared_failures` | equals `failure_row_context.declared_failures` |
| `local_effect.declared_failures_after` | equals `declared_failures + [missing_failure]` |
| `local_premise` | equals `failure_row_context.local_premise` |
| `single_edit_assumption` | specific to this gate, not a generic placeholder |
| `non_goal` | explicitly denies runtime / authority / whole-program success claims |
| `repair_non_final` | `true` |
| `lab_non_final` | `true` |

The payload must not use placeholder string values such as empty strings,
`todo`, `tbd`, `fixme`, `unknown`, `unresolved`, or `placeholder`.

This does not standardize final JSON semantics. It is a LAB guard against
satisfying OBL-025-shaped tests with a non-empty but meaningless repair array.

## Executable guards added with this gate

This package adds regression checks, but intentionally does not widen repair
output:

- Python helper test:
  `test_erow_suggested_repair_payloads_are_not_placeholders`
  verifies that the existing `ELAB-10` repair item is a local
  witness-compatible payload and contains no placeholder strings.
- Python helper test:
  `test_elaboration_non_visibility_singleton_failure_row_stays_no_repair`
  now also checks that the `ELAB-13..16` singleton gate inputs have non-empty
  target/request context and remain no-repair.
- Rust regression test:
  `suggested_repair_payloads_are_non_placeholder_local_witnesses`
  verifies the same local-witness alignment on serialized
  Surface-to-Core elaboration output.
- Rust sample-path regression:
  `sample_fixtures_cover_each_non_visibility_singleton_without_repair`
  now also checks non-empty target/request context for `ELAB-13..16`.

## Future widening rule

A later package may widen `suggested_repair[]` for non-visibility singleton
`E-ROW-001` only if it intentionally changes the executable expectation for
`ELAB-13..16`.

The recommended class-wide path is to flip all four singleton rows together,
because the current implementation and helper evidence treat the base
remote-request failures symmetrically. A staged path is still allowed if the
report explicitly says which singleton row is now repair-bearing and why the
remaining singleton rows stay no-repair fences.

## Cases still excluded

These remain no-repair unless a later package defines a separate assumption
and tests:

- non-visibility multi-missing rows such as `ELAB-07`;
- mixed visibility / non-visibility multi-missing rows such as `ELAB-04`;
- any repair that would need ranking or decomposition;
- any row whose target cannot be identified as a concrete `when_fails_row`;
- any row with multiple generated failing requests until request/diagnostic
  association and ordering are explicit;
- any payload that cannot state the missing failure, target row, local premise,
  request id, and non-goal.

## What remains open

- Whether the first repair-bearing non-visibility singleton prototype should
  flip all four singleton rows at once or stage one row first.
- Whether set insertion is ever a single edit.
- Whether mixed rows should decompose into several singleton repairs.
- Whether target spans should replace or supplement the current LAB-local
  `target_ref`.
- How repair application semantics will be represented if the project later
  validates an edit script.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No non-visibility repair output widening.
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
