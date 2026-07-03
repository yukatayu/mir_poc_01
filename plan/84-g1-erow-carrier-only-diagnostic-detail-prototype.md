# plan/84 - G1 E-ROW carrier-only diagnostic detail prototype

## Purpose

This file records the LAB-only implementation of a carrier-only E-ROW diagnostic
detail prototype for Surface-to-Core elaboration failure-row containment.

This is LAB repository memory. It does not edit canon, does not freeze a
Diagnostic ABI, does not emit repair rows, does not state or prove OBL-024/025,
does not claim explanation soundness or completeness, does not claim
conformance, and does not claim G1 exit.

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
- LAB diagnostic carrier inventory:
  `plan/80-g1-diagnostic-carrier-inventory.md`
- LAB OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- LAB OBL-025 relation inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`

If this LAB note conflicts with canon, canon wins.

## Implemented carrier

`crates/mir-semantics::surface_to_core_elaboration` now emits
`lab_diagnostic_details` on the elaboration report when generated remote
request failures are not contained in the surrounding `when ... fails` row.

The legacy `TextualMirDiagnostic` code remains
`generated_failure_not_declared`, and helper projections still expose
`diagnostic_codes`. The new detail is additive and LAB-only.

Current non-final detail fields:

| Field | Current meaning |
|---|---|
| `legacy_code` | existing helper / textual diagnostic code |
| `canon_id` | E-ROW-001 or E-ROW-002 candidate canon diagnostic family |
| `severity` | current fixed value `error` |
| `rule_instance` | current fixed value `BND-001.row-containment` |
| `failed_premise` | current fixed value `generated_failures_subset_declared_fails` |
| `missing_evidence` | generated failure families not declared in the failure row |
| `refs` | LAB trace refs to canon elaboration, diagnostic format, and OBL-024 theory |
| `lab_non_final` | explicit guard that this is not a public ABI |

No `suggested_repair[]` field is emitted.

## E-ROW split used by this prototype

| Case | Current carrier output | Evidence |
|---|---|---|
| General or mixed generated failure omission | `E-ROW-001` | `ELAB-04` and `ELAB-07` expected JSON plus Rust/Python tests |
| Only missing generated failure is `VisibilityDenied` | `E-ROW-002` | `ELAB-10` expected JSON plus Rust/Python tests |

`ELAB-04` remains mixed E-ROW-shaped evidence because it is a visible read whose
declared row includes only `MissingCapability`; its missing set is
`MissingWitness`, `RouteUnavailable`, `StaleMembership`, and
`VisibilityDenied`.

`ELAB-07` is the clean non-visibility write omission case. Its missing set is
`MissingWitness`, `RouteUnavailable`, and `StaleMembership`.

`ELAB-10` is the clean visibility-only omission case. Its missing set is only
`VisibilityDenied`.

## Code and sample surfaces

- Rust report carrier:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- Example JSON boundary:
  `crates/mir-semantics/examples/surface_to_core_elaborate.rs`
- Helper projection:
  `scripts/surface_mir_samples.py`
- Rust tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- Python helper tests:
  `scripts/tests/test_surface_mir_samples.py`
- Expected JSON evidence:
  `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
  `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`

## What this changes

- E-ROW diagnostics now have a repo-local carrier with enough information to
  inspect canon ID, rule instance, failed premise, and missing generated
  failures.
- The example binary no longer drops the LAB carrier when converting the Rust
  report into JSON.
- The helper projection includes `lab_diagnostic_details` only when the carrier
  is non-empty, so positive rows do not gain empty ABI-looking fields.
- Expected JSON for `ELAB-04`, `ELAB-07`, and `ELAB-10` records the carrier
  evidence.

## What this does not change

- No final Diagnostic ABI.
- No repair payload ABI.
- No `suggested_repair[]`.
- No OBL-024 statement or proof.
- No OBL-025 statement or proof.
- No explanation soundness or completeness claim.
- No C-static conformance claim.
- No G0 exit.
- No G1 exit.
- No T1/T2 transition.
- No runtime MessageEnvelope dispatch claim.
- No final Surface runtime / transport / viewer claim.

## Follow-up

The next implementation step, if promoted, should be repair-bearing only after
tests can reject placeholder `suggested_repair[]` items and every repair item
can identify the target failure row, missing failure, local premise, and
single-edit assumption.

OBL-024/025 Lean statement drafts should remain reserve work until replay,
repair, and payload vocabularies are stable enough to avoid freezing the wrong
interface.
