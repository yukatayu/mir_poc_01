# plan/77 - G1 OBL-021 repo-local Lean statement draft

## Purpose

This file records the first repo-local Lean-checked statement-shape draft for
OBL-021 elaboration determinism.

This is LAB repository memory. It does not change canon, does not edit
`mirrorea_canon/theory/11-metatheory-ledger.md`, and does not claim OBL-021
completion, proof discharge, proof skeleton completion, G1/T1/T2 exit,
conformance, runtime determinism, parser/checker implementation proof, or final
equality relation.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB dependency inventory:
  `plan/76-g1-obl020-021-dependency-inventory.md`
- LAB statement artifact:
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- LAB explanation:
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md`
- LAB manifest:
  `samples/lean/manifest.json`

If this LAB statement conflicts with canon, canon wins.

## What was added

`samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
introduces a LAB-only namespace:

```text
MirCore.Lab.OBL021.StatementDraft
```

The file defines abstract carriers and predicates:

- `Vocab`: abstract types for environment, context, locus, Surface item,
  elaboration result, diagnostic, Core term, type/mode/effect/failure rows,
  constraints, obligations, generated edges, and source-span map;
- `Pred`: abstract proposition fields for well-scoped inputs, successful
  elaboration, diagnostic rejection, result projections, and equivalence
  relations over every projected output component;
- `SameElabResult`: a component-wise equivalence relation over elaboration
  results, parameterized by `Pred`;
- `SameDiagnostic`: a named diagnostic equivalence wrapper, currently backed by
  `EquivalentDiagnostic`, so later work can strengthen diagnostic comparison
  without implying a final diagnostic ABI here;
- `ElabDeterministicPost`: same-input success-success equivalence,
  diagnostic-diagnostic equivalence, and success/reject mutual exclusion;
- `OBL021StatementDraft`: a `Prop` definition tying well-scoped fixed input to
  `ElabDeterministicPost`.

## Lean reading

`OBL021StatementDraft` is a `Prop` definition. It is intentionally not a
proved `theorem`.

This keeps the statement shape machine-checked while avoiding all of the
following:

- `axiom`;
- `constant`;
- `sorry`;
- a false proof claim;
- a final `MirCore.Elab.Det` namespace claim;
- an accidental canon OBL status movement.

The equality/equivalence relation is abstract. The file does not decide
whether future OBL-021 uses syntactic equality, normalized equality,
definitional equality, alpha-equivalence, or a canon-specific relation.

## Predicate boundary

| Predicate group | Current statement reading | Explicit non-claim |
|---|---|---|
| input | same `env / ctx / locus / item` under `WellScopedInput` | no parser grammar freeze |
| success result | two successful elaborations of the same input have component-wise equivalent results | no proof that the real elaborator has this property |
| diagnostics | two diagnostic outcomes for the same input satisfy `SameDiagnostic` | no final diagnostic ABI |
| mutual exclusion | the same fixed input cannot both elaborate successfully and reject | no implementation proof |
| result components | Core term, type, mode, effect row, failure row, constraints, obligations, generated edges, source spans | no final Core datatype or JSON/API freeze |

## Status

- Lean file exists and compiles locally.
- `samples/lean/manifest.json` records the new `statement_drafts` entry and
  successful verification.
- `scripts/current_l2_lean_sample_sync.py` registers the OBL-021 draft under
  `statement_drafts`.
- `scripts/tests/test_current_l2_lean_sample_sync.py` checks that the OBL-021
  LAB draft remains registered with its explanation file.

## Relation to adjacent obligations

| Adjacent item | Separation rule |
|---|---|
| OBL-001 | Assignment soundness statement draft remains in `obl001`; this OBL-021 draft does not refine THM-001 directly. |
| OBL-002 | THM-001 proof work remains later and is not advanced by this draft. |
| OBL-020 | Step-rule well-formedness preservation remains separate; a later package actualized its own LAB draft under `obl020`. |
| OBL-021 | This is compile-check-only statement-shape evidence; it is not completion. |

## Open questions

- Should OBL-020 receive a similar LAB-only statement-shape draft before any
  proof skeleton work?
- Should a later shared vocabulary file avoid duplicating abstract carriers
  between OBL-001 and OBL-021 drafts?
- Should the future OBL-021 statement split success determinism, diagnostic
  determinism, and success/reject exclusivity into separate lemmas?
- What canon-confirmed equivalence relation should replace the abstract
  predicate fields if/when OBL-021 moves toward real statement status?

## Next safe packages

1. OBL-001 statement refinement only if review finds a real missing predicate or
   overfit.
2. OBL-020 / OBL-021 statement refinement only if review finds a real
   abstraction gap or overfit.
3. E-ROW diagnostic alignment package for canon E-ROW-001 / E-ROW-002 versus
   LAB `generated_failure_not_declared`, still without diagnostic ABI freeze.

## Non-claims

- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No OBL status movement in canon.
- No Lean proof completion.
- No theorem discharge.
- No proof skeleton completion.
- No OBL-021 completion.
- No OBL-020 completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No runtime scheduling determinism claim.
- No parser/checker implementation proof.
- No final equality relation, diagnostic ABI, Core IR JSON, public API,
  runtime, transport, projection, devtools, telemetry, provider, or product
  completion.
