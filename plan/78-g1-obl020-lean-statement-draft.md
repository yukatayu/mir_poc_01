# plan/78 - G1 OBL-020 repo-local Lean statement draft

## Purpose

This file records the first repo-local Lean-checked statement-shape draft for
OBL-020 well-formedness preservation of step rules.

This is LAB repository memory. It does not change canon, does not edit
`mirrorea_canon/theory/11-metatheory-ledger.md`, and does not claim OBL-020
completion, proof discharge, proof skeleton completion, G1/T1/T2 exit,
conformance, runtime implementation proof, request-serving correctness,
authority soundness, observer noninterference, or final runtime step/API
taxonomy.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB dependency inventory:
  `plan/76-g1-obl020-021-dependency-inventory.md`
- LAB statement artifact:
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- LAB explanation:
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.md`
- LAB manifest:
  `samples/lean/manifest.json`
- LAB guard hardening:
  `plan/117-g1-obl001-020-021-statement-guard-hardening.md`

If this LAB statement conflicts with canon, canon wins.

## What was added

`samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` introduces a
LAB-only namespace:

```text
MirCore.Lab.OBL020.StatementDraft
```

The file defines abstract carriers and predicates:

- `Vocab`: abstract types for runtime configuration, step label, and optional
  step family;
- `Pred`: abstract proposition fields for `WellFormed`, `Step`,
  `CanonStepFamily`, and `StepHasFamily`;
- `PreservesWF`: aggregate preservation shape
  `WellFormed(before) -> Step(before, label, after) -> WellFormed(after)`;
- `FamilyStepPreservesWF`: optional family-oriented helper for later proof
  organization without enumerating canon step families;
- `OBL020StatementDraft`: a `Prop` definition quantifying over all abstract
  steps.

## Lean reading

`OBL020StatementDraft` is a `Prop` definition. It is intentionally not a
proved `theorem`.

This keeps the statement shape machine-checked while avoiding all of the
following:

- `axiom`;
- `constant`;
- `sorry`;
- a false proof claim;
- a final `MirCore.Step.WF` namespace claim;
- an accidental canon OBL status movement;
- a frozen step-family taxonomy.

## WF clause boundary

Canon `theory/01` names the following WF pressure clauses:

| Clause family | Current LAB reading | Explicit non-claim |
|---|---|---|
| acyclic occurrence DAG | should be included in the eventual meaning of `WellFormed` | no Lean proof decomposition yet |
| grant lineage | should be included in the eventual meaning of `WellFormed` | no authority soundness proof |
| observe has publish ancestor | should be included in the eventual meaning of `WellFormed` | no observer noninterference proof |
| active key or tombstone | should be included in the eventual meaning of `WellFormed` | no store-runtime implementation proof |
| monotone chain position | should be included in the eventual meaning of `WellFormed` | no G2 fallback proof |

The first Lean draft keeps these clauses behind one `WellFormed` predicate.
Turning them into Lean fields now would look like a premature proof interface
decision.

## Status

- Lean file exists and compiles locally.
- `samples/lean/manifest.json` records the new `statement_drafts` entry and
  successful verification.
- `scripts/current_l2_lean_sample_sync.py` registers the OBL-020 draft under
  `statement_drafts`.
- `scripts/tests/test_current_l2_lean_sample_sync.py` checks that the OBL-020
  LAB draft remains registered with its explanation file.
- `plan/117` now hardens the sync unit guard so `PreservesWF`,
  `FamilyStepPreservesWF`, and `OBL020StatementDraft` retain their body-level
  links. This is still compile-check-only statement evidence, not per-step
  proof decomposition or runtime implementation proof.

## Relation to adjacent obligations

| Adjacent item | Separation rule |
|---|---|
| OBL-001 | Assignment soundness statement draft remains in `obl001`; this OBL-020 draft does not prove THM-001 runtime soundness. |
| OBL-002 | THM-001 proof work remains later and is not advanced to proof status by this draft. |
| OBL-020 | This is compile-check-only statement-shape evidence; it is not completion. |
| OBL-021 | Elaboration determinism statement draft remains in `obl021`; this OBL-020 draft does not prove determinism. |

## Open questions

- Should a future OBL-020 proof split per-step lemmas and then prove an
  aggregate preservation theorem?
- What final datatype represents step labels and rule families, if any?
- When should canon WF clauses become explicit Lean predicates instead of
  remaining behind `WellFormed`?
- Which OBL-020 premises are shared with THM-001 / OBL-001, and which remain
  runtime-only?

## Next safe packages

1. Focused statement-draft refinement only if review finds a real missing
   predicate or overfit in OBL-001/020/021.
2. E-ROW diagnostic alignment package for canon E-ROW-001 / E-ROW-002 versus
   LAB `generated_failure_not_declared`, still without diagnostic ABI freeze.
3. G1 statement-draft consolidation notes if duplication across OBL-001/020/021
   starts obscuring the source hierarchy.

## Non-claims

- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No OBL status movement in canon.
- No Lean proof completion.
- No theorem discharge.
- No proof skeleton completion.
- No OBL-020 completion.
- No OBL-021 completion.
- No OBL-001 / OBL-002 completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No runtime implementation proof.
- No request-serving correctness proof.
- No authority soundness proof.
- No observer-safe noninterference proof.
- No final step-family taxonomy, runtime API, scheduler semantics, transport,
  projection, devtools, telemetry, provider, or product completion.
