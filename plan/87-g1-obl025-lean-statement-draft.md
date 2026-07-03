# plan/87 - G1 OBL-025 repo-local Lean statement draft

## Purpose

This file records a LAB-only Lean-checked statement-shape draft for OBL-025
explanation completeness / repair coverage.

This is LAB repository memory. It does not change canon, does not edit
`mirrorea_canon/theory/11-metatheory-ledger.md`, and does not claim OBL-025
completion, proof discharge, G1/T1/T2 exit, conformance, final Diagnostic ABI,
final repair payload ABI, repair ranking, multi-edit repair support, or
whole-program success after repair.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- LAB OBL-025 inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB E-ROW-002 repair carrier prototype:
  `plan/86-g1-erow002-visibility-repair-carrier-prototype.md`
- LAB statement artifact:
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- LAB explanation:
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md`
- LAB manifest:
  `samples/lean/manifest.json`

If this LAB statement conflicts with canon, canon wins.

## What was added

`samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
introduces a LAB-only namespace:

```text
MirCore.Lab.OBL025.StatementDraft
```

The file defines abstract carriers and predicates:

- `Vocab`: abstract types for environment, context, locus, Line-1 input,
  rejection, diagnostic, statement fragment, repair witness, suggested repair,
  diagnostic family, missing evidence kind, failed premise, and blame target;
- `Pred`: abstract proposition fields for well-scoped Line-1 inputs, covered
  repair cases, current evidence boundary, rejection, statement-fragment
  membership, diagnostic family, missing evidence kind, failed premise, blame
  target, associated diagnostics, eligible single-edit repair witnesses,
  suggested repairs, and suggestion realization;
- `EligibleSingleEditRepair`: an existential local witness that a repair is
  single-edit, stays in the statement fragment, matches the diagnostic family
  and missing evidence, discharges the local failed premise, and targets the
  blame site;
- `SuggestionCoversWitness`: an existential local witness that an emitted
  suggested repair realizes a compatible repair witness and carries the same
  diagnostic-family / missing-evidence / premise / target alignment;
- `RepairCompletenessForRejection`: if at least one eligible single-edit
  witness exists for the covered rejection, then some associated diagnostic has
  at least one realized suggested repair;
- `OBL025StatementDraft`: a `Prop` definition tying the coverage shape to a
  well-scoped covered Line-1 rejection and its declared fragment.

## Lean reading

`OBL025StatementDraft` is a `Prop` definition. It is intentionally not a proved
`theorem`.

This keeps the statement shape machine-checked while avoiding all of the
following:

- `axiom`;
- `constant`;
- `sorry`;
- a false proof claim;
- a final `MirCore.Diagnostics.Completeness` namespace claim;
- an accidental canon OBL status movement;
- a frozen Diagnostic / repair payload ABI.

The Line-1 coverage relation, repair edit vocabulary, diagnostic association,
and repair-suggestion realization relation are abstract. The file does not
define final Surface-to-Core diagnostic semantics or edit application
semantics.

## Predicate boundary

| Predicate group | Current statement reading | Explicit non-claim |
|---|---|---|
| Line-1 coverage | `CurrentEvidenceBoundary` and `CoveredLine1RepairCase` mark cases to which the draft applies | no claim that every Line-1 rejection family is currently covered |
| rejection context | diagnostic family, missing evidence kind, failed premise, and blame target are abstract relations | no final diagnostic payload field names or span ABI |
| statement fragment | repair search is constrained by an abstract statement fragment | no final edit-script or source-patch semantics |
| single edit | eligible repair witness is abstract and existential | no multi-edit support, all-repairs coverage, or set-insertion atomicity decision |
| emitted diagnostic | a diagnostic is associated with the rejection | no diagnostic ordering or equality ABI |
| suggested repair | at least one suggestion realizes a compatible witness when some single-edit witness exists | no repair ranking, global minimality, all-combinations coverage, or all-repairs-listed claim |
| local effect | suggestion targets the local failed premise | no whole-program acceptance or runtime success claim |

## Relation to current E-ROW evidence

The current executable LAB repair-carrier evidence is narrower than the abstract
draft:

- `ELAB-10` carries one LAB-only `suggested_repair[]` item for diagnostic
  family `E-ROW-002` and missing evidence `VisibilityDenied`.
- `ELAB-04` and `ELAB-07` remain no-repair evidence for mixed and
  multi-missing shapes.
- The Lean draft's `CoveredLine1RepairCase` predicate is the guard that keeps
  this from being read as all Line-1 families or all singleton repairs.

## Status

- Lean file exists and compiles locally.
- `samples/lean/manifest.json` records the new `statement_drafts` entry and
  successful verification.
- `scripts/current_l2_lean_sample_sync.py` registers the OBL-025 draft under
  `statement_drafts`.
- `scripts/tests/test_current_l2_lean_sample_sync.py` checks that the OBL-025
  LAB draft remains registered with its explanation file.

## Relation to adjacent obligations

| Adjacent item | Separation rule |
|---|---|
| OBL-024 | Explanation soundness remains separate; OBL-025 does not prove replay accuracy. |
| OBL-025 | This is compile-check-only statement-shape evidence; it is not completion. |
| E-ROW repair carrier | `plan/86` is current executable evidence for one narrow repair shape, not a proof of this statement. |
| THM-001 / OBL-001 | Assignment soundness remains separate from diagnostic repair coverage. |

## Open questions

- Which Line-1 families should instantiate `CoveredLine1RepairCase` first
  beyond the current `E-ROW-002` / `VisibilityDenied` evidence?
- Should non-visibility singleton E-ROW repairs be inventoried before any
  statement refinement?
- Is adding multiple missing generated failures to one `fails` row one edit,
  multiple edits, or a separate repair family?
- What final target-span / blame-target representation should replace the
  current LAB-local `target_ref`?
- How should competing visibility repairs be ranked or represented?

## Next safe packages

1. Keep OBL-025 at compile-check-only status and use it as a guard while
   inventorying non-visibility singleton and mixed / multi-missing repair
   shapes.
2. Refine `RepairCompletenessStatementDraft.lean` only if review finds a real
   missing predicate or overfit.
3. Do not widen executable `suggested_repair[]` output before the relevant
   repair shape has no-placeholder tests and a documented local witness.

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
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
- No whole-program success after repair claim.
