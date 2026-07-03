# plan/79 - G1 E-ROW diagnostic alignment

## Purpose

This file records LAB repository memory for aligning canon E-ROW diagnostic
vocabulary with current LAB Surface elaboration failure-row evidence.

This is not a diagnostic ABI freeze, not a conformance pass, not OBL-024/025
discharge, and not G1 exit. It does not edit canon.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration contract:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic ID format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- LAB Surface elaboration evidence:
  `samples/full-system-v1-surface/elaboration/`
- LAB helper / tests:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`,
  `scripts/surface_mir_samples.py`,
  `scripts/tests/test_surface_mir_samples.py`

If this LAB alignment conflicts with canon, canon wins.

## Canon anchors

| Canon anchor | Reading for this package |
|---|---|
| BND-001 row containment | generated failure row must be contained in declared `fails`, else E-ROW-001 |
| `spec/07` E-ROW-001 | failure rows: generated failure set is not a subset of declared `fails` |
| `spec/07` E-ROW-002 | failure rows: undeclared `VisibilityDenied` |
| `theory/10` diagnostic carrier | final diagnostic should carry ID, span, rule instance, failed premise, missing evidence, suggested repair, and refs |
| OBL-024 / OBL-025 | explanation soundness/completeness remain open and are not discharged by LAB helper diagnostics |

## Current LAB diagnostic surface

Current LAB Surface elaboration uses helper-local diagnostic code:

```text
generated_failure_not_declared
```

This code appears when a generated remote request has a failure set that is not
contained in the surrounding `when ... fails ...` declaration. The current
helper also sets `failure_row_complete: false` on the generated remote request
summary.

This is evidence for row-containment checking, not the final diagnostic ID ABI.

## Alignment table

| LAB row / helper | LAB diagnostic | Canon alignment | Current evidence | Boundary |
|---|---|---|---|---|
| `ELAB-07` underdeclared generated write request | `generated_failure_not_declared` | clean E-ROW-001-shaped evidence | write request failure row incomplete; expected diagnostic row exists | no request-serving runtime proof |
| `ELAB-10` visible communication without declared `VisibilityDenied` | `generated_failure_not_declared` | E-ROW-002 pressure, implemented through same LAB helper code as E-ROW-001 | `VisibilityDenied` is the missing generated failure; expected diagnostic row exists | no final E-ROW-002-specific ABI or message split |
| `ELAB-04` underdeclared generated read request | `generated_failure_not_declared` | mixed E-ROW-shaped evidence | read request failure row incomplete; expected diagnostic row exists, but the source omits more than one generated failure | no final ID / span / repair shape |
| `surface_core_generated_failure_rows_contained` obligation row | none on success | positive support for E-ROW-001 containment premise | positive rows show generated failures contained when declared | not conformance pass |

## Interpretation

The LAB helper currently uses one diagnostic code for the failure-row
containment family. For canon alignment, read/write underdeclared generated
failures map to E-ROW-001. The `VisibilityDenied` case is canonically important
enough to have E-ROW-002, but LAB has not split the helper diagnostic string
yet.

Therefore the safe reading is:

- LAB `generated_failure_not_declared` is a helper-local row-containment
  diagnostic family.
- `ELAB-07` aligns structurally with E-ROW-001.
- `ELAB-10` aligns structurally with E-ROW-002, while still using the same
  helper-local diagnostic family.
- `ELAB-04` is useful mixed E-ROW-shaped evidence, but should not be treated as
  the clean E-ROW-002 row because it omits more than `VisibilityDenied`.
- A future package may split LAB helper diagnostics into canon-shaped IDs, but
  doing so would be an implementation/API decision and should be validated with
  tests.

## Diagnostic carrier gap

Canon `theory/10` expects a Diagnostic carrier with:

- ID;
- span;
- rule instance;
- failed premise;
- missing evidence;
- suggested repair;
- refs.

Current LAB evidence has:

- helper-local diagnostic code;
- source span metadata in surrounding elaboration evidence;
- remote request summaries with `failure_row_complete: false`;
- expected JSON rows and Python/Rust tests for code and incomplete row flag.

Current LAB evidence does not prove OBL-024 or OBL-025 and does not provide the
full final diagnostic carrier.

## Suggested later implementation split

If/when code is changed, a safe later split would be:

| Canon ID | Candidate helper-facing stable category | Repair family |
|---|---|---|
| E-ROW-001 | generated failure not declared | add missing generated failure family to the nearest `when ... fails` row |
| E-ROW-002 | undeclared VisibilityDenied | add `VisibilityDenied` or declare appropriate visibility / observe authority |

This plan does not implement that split.

## Open questions

- Should LAB helper output keep legacy code strings while adding canon ID fields
  alongside them?
- Should `VisibilityDenied` get a distinct helper code before a final
  diagnostic ABI exists?
- What minimal carrier fields are needed before OBL-024 explanation soundness
  can be stated?
- Should suggested repair rows be generated in LAB expected JSON, or deferred
  until diagnostic ABI work begins?

## Next safe packages

1. Diagnostic carrier inventory for OBL-024 / OBL-025 prerequisites, still
   without proof discharge.
2. Optional LAB helper diagnostic ID split for E-ROW-001 / E-ROW-002, with
   Rust and Python tests, if the project wants executable evidence next.
3. Focused statement-draft refinement only if review finds a real overfit in
   OBL-001/020/021.

## Non-claims

- No canon edit.
- No final diagnostic ABI.
- No final message wording.
- No localization decision.
- No OBL-024 / OBL-025 discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime MessageEnvelope dispatch claim.
- No runtime request serving correctness claim.
- No final Surface grammar/API claim.
