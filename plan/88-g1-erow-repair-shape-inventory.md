# plan/88 - G1 E-ROW repair shape inventory before widening

## Purpose

This file inventories E-ROW repair shapes before any further widening of
LAB-only `suggested_repair[]` output beyond the current `E-ROW-002` /
`VisibilityDenied` prototype.

This is LAB repository memory. It does not edit canon, does not freeze a
Diagnostic or repair ABI, does not widen executable repair output, does not
prove OBL-024/025, does not claim explanation soundness or completeness, does
not claim conformance, and does not claim G1 exit.

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
- LAB E-ROW-002 repair carrier prototype:
  `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- LAB OBL-025 Lean statement draft:
  `plan/87-g1-obl025-lean-statement-draft.md`
- LAB implementation and evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/tests/test_surface_mir_samples.py`,
  `samples/full-system-v1-surface/elaboration/`

If this LAB inventory conflicts with canon, canon wins.

## Current executable boundary

Current Surface-to-Core elaboration computes generated remote-request failures
from:

- base remote-request failures:
  `MissingCapability`, `MissingWitness`, `RouteUnavailable`,
  `StaleMembership`;
- optional visibility failure:
  `VisibilityDenied`, only when the communication decision requires visibility
  failure evidence.

The current executable repair carrier is deliberately narrower:

| Evidence row | Required failures | Declared failures | Missing failures | Current canon family | Current repair output |
|---|---|---|---|---|---|
| `ELAB-10` | base failures + `VisibilityDenied` | all base failures | `VisibilityDenied` | `E-ROW-002` | one LAB-only `add-to-fails-row` suggestion |
| `ELAB-07` | base failures | `MissingCapability` | `MissingWitness`, `RouteUnavailable`, `StaleMembership` | `E-ROW-001` | none |
| `ELAB-04` | base failures + `VisibilityDenied` | `MissingCapability` | `MissingWitness`, `RouteUnavailable`, `StaleMembership`, `VisibilityDenied` | `E-ROW-001` in current LAB split | none |

Important reading:

- `ELAB-10` is visibility-only singleton evidence.
- `ELAB-07` is non-visibility multi-missing evidence, not singleton evidence.
- `ELAB-04` is mixed visibility/non-visibility multi-missing evidence.
- There is not yet executable evidence for a non-visibility singleton
  `E-ROW-001` repair row.

## Repair shape taxonomy

| Shape | Candidate repair family | Current status | Widening gate |
|---|---|---|---|
| Visibility singleton | add `VisibilityDenied` to the relevant `when ... fails` row | implemented only for `E-ROW-002` / `ELAB-10` | keep as-is unless target/span vocabulary changes |
| Non-visibility singleton | add the one missing generated failure family to the relevant `when ... fails` row | candidate only; no executable row yet | add focused singleton fixture, expected JSON, Rust/Python no-placeholder tests, and explicit single-edit assumption |
| Non-visibility multi-missing | add multiple missing generated failure families to one `fails` row, or emit one repair per missing family | no-repair evidence today (`ELAB-07`) | decide whether set insertion is one edit or multiple edits; do not infer from OBL-025 |
| Mixed visibility/non-visibility multi-missing | same local premise but includes both `VisibilityDenied` and other missing failures | no-repair evidence today (`ELAB-04`) | first decide decomposition, ordering, and whether visibility repair competes with add-to-fails-row |
| Alternative visibility repair | declare visibility / observe authority instead of adding `VisibilityDenied` to `fails` | OPEN; not current prototype | needs separate repair family and authority/visibility preservation wording |
| Multi-request row failures | one source item emits multiple failing request diagnostics, even if each request is singleton-shaped | no-repair by default | needs diagnostic association, ordering, and per-request target policy |
| Ambiguous target row | target row cannot be uniquely identified as the nearest relevant `when ... fails` row | no-repair by default | carrier may classify the failure, but repair target is not safe |
| Non-`when` target | local row target is not `when_fails_row` | no-repair by default | outside current repair target vocabulary |
| Coincident non-row diagnostic | E-ROW appears with independent visibility / private-field / authority diagnostics | no-repair by default | avoid suggesting that a row repair solves the whole rejection |

## Singleton definition for future tests

A future executable non-visibility singleton row should satisfy all of the
following before any `suggested_repair[]` output is widened:

1. `canon_id == "E-ROW-001"`.
2. `failure_row_context.target_kind == "when_fails_row"`.
3. `failure_row_context.target_ref` is non-empty.
4. `failure_row_context.missing_failures.len() == 1`.
5. The single missing failure is one of the base remote-request failures, not
   `VisibilityDenied`.
6. The suggested repair records:
   - legacy code and candidate canon id;
   - local request id;
   - target row reference and event context;
   - required / declared / missing failure sets;
   - `declared_failures_after`;
   - local premise `generated_failures_subset_declared_fails`;
   - a non-final single-edit assumption specific to non-visibility singleton
     row addition;
   - a non-goal that the repair does not prove runtime success or capability
     availability;
   - `repair_non_final` and `lab_non_final`.

This singleton definition is a LAB test gate, not final ABI.

## No-repair cases to preserve

The following cases should continue to omit `suggested_repair[]` until a later
package explicitly changes their status with tests and repository memory:

- more than one missing generated failure;
- any mixed missing set that includes `VisibilityDenied` plus other failures;
- any source item with multiple failing generated requests until per-request
  diagnostic association and ordering are explicit;
- any case where the target failure row cannot be identified;
- any case where the target is not a `when_fails_row`;
- any case where the local premise is not row containment;
- any case where an independent non-row diagnostic is part of the same
  rejection and the row repair could be read as solving the whole rejection;
- any case where the suggested item would be a placeholder rather than a
  witness-compatible local repair.

For current no-repair rows, the safer JSON shape is to omit
`suggested_repair` rather than emit an empty array. Empty-array semantics are
not standardized in this LAB carrier.

## Relation to OBL-025

`plan/87` states OBL-025 as a LAB compile-check-only existential coverage
shape. This inventory should be read as an evidence-boundary refinement for
future instantiations of `CoveredLine1RepairCase`.

Safe reading:

- current executable coverage evidence is only `E-ROW-002` /
  `VisibilityDenied` singleton;
- non-visibility singleton is a plausible next covered repair case, but only
  after a fixture and tests exist;
- mixed and multi-missing cases are not covered single-edit repair evidence
  until set-insertion atomicity or decomposition is decided;
- non-empty `suggested_repair[]` is meaningful only when the item realizes a
  local witness and targets the reported premise.

## Future code-widening checklist

If a later package widens executable repair output, it should update all of the
following in the same task:

- Rust carrier logic in
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`;
- Rust elaboration tests for the new positive and no-repair negative boundary;
- Python helper tests under `scripts/tests/test_surface_mir_samples.py`;
- expected JSON under `samples/full-system-v1-surface/elaboration/`;
- `samples/full-system-v1-surface/elaboration/README.md`;
- `samples_progress.md`;
- relevant `plan/` memory and a new `docs/reports/` report.

Do not widen by changing only the emitted JSON shape; the tests must reject
placeholder repair rows and must preserve no-repair cases. Inventory-only rows
should continue to omit `suggested_repair` unless a later package explicitly
standardizes empty repair-list semantics.

## Open questions

- Which non-visibility base failure should be the first singleton fixture:
  `MissingCapability`, `MissingWitness`, `RouteUnavailable`, or
  `StaleMembership`?
- Should the later prototype test one representative non-visibility singleton
  or one row per base failure?
- Is adding multiple missing failures to one `fails` row one edit, multiple
  edits, or a separate set-insertion repair family?
- Should mixed visibility/non-visibility omissions eventually decompose into
  independent repair witnesses, or remain no-repair until ranking exists?
- How should alternative visibility repairs be represented without treating
  `VisibilityDenied` as merely another base failure?
- What final target-span / declaration-span representation should replace the
  LAB-local `target_ref`?

## Next safe packages

1. Add a non-visibility singleton fixture and keep it no-repair first, if the
   goal is to expose executable pressure without widening repair output.
2. Widen `suggested_repair[]` to the non-visibility singleton only after the
   fixture, tests, and single-edit assumption are explicit.
3. Keep mixed and multi-missing cases no-repair until set-insertion atomicity,
   decomposition, and ranking are separately addressed.
4. Refine the OBL-025 Lean statement only if this inventory reveals a missing
   abstract predicate.

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
