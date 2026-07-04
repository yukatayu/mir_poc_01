# plan/126 - G1 OBL-020/021 boundary audit and OBL-021 guard hardening

## Purpose

This file is LAB repository memory.

It records the post-`plan/125` review of whether OBL-020
well-formedness-preservation and OBL-021 elaboration-determinism statement
boundaries must be refined before the current G1 ordinary-assignment bridge can
continue.

It also records a narrow sync-test hardening for a guard weakness in the
current OBL-021 statement-body checks.

This file does not edit canon, does not close G0 or G1, does not move
metatheory ledger status, does not prove OBL-020 or OBL-021, does not create a
proof skeleton, and does not change runtime, transport, diagnostic, repair,
Core IR, public API, or sample-status claims.

## Verdict

No Lean predicate refinement is needed for OBL-020 or OBL-021 at this
checkpoint.

The current LAB statement drafts are still appropriate as compile-check-only
`Prop` shapes:

- OBL-020 keeps concrete step-rule taxonomy, scheduler behavior, and per-step
  proof decomposition behind abstract `WellFormed`, `Step`,
  `CanonStepFamily`, and `StepHasFamily` predicates.
- OBL-021 keeps final equality selection, projection-totality mechanics,
  diagnostics ABI, and runtime scheduling determinism outside the statement
  draft. It states result equivalence, diagnostic equivalence, and
  success/reject exclusion over abstract predicates.

The only required change is test-only guard hardening: required Lean body links
must be checked after stripping comments, and the vacuity helper must reject a
bare `:= True` body.

## OBL-020 boundary reading

The current `StepWFStatementDraft.lean` boundary is sufficient for the current
G1 bridge.

The draft states a generic preservation shape:

- if `before` is well-formed;
- and a step moves `before` through `label` to `after`;
- then `after` is well-formed.

The family-level wrapper threads `CanonStepFamily` and `StepHasFamily` without
choosing a final runtime step taxonomy. This is the right level while the G1
bridge remains a static elaboration / statement-boundary package rather than a
runtime proof package.

Future OBL-020 refinement should wait until the project introduces concrete
`Config`, `StepLabel`, `StepFamily`, `WellFormed` clauses, and per-step proof
obligations. Adding them now would overfit a proof-facing obligation to current
LAB fixtures.

## OBL-021 boundary reading

The current `ElabDeterminismStatementDraft.lean` boundary is also sufficient
for the current G1 bridge.

The draft keeps the important equivalence dimensions explicit:

- Core term result equivalence;
- type, mode, effect-row, failure-row, constraint-set, obligation-set,
  generated-edge, and source-span-map equivalence;
- diagnostic equivalence;
- success/reject mutual exclusion for the same well-scoped input.

This is enough for the current ordinary-assignment bridge because the current
pressure is static elaboration consistency, not final equality selection or
runtime scheduling determinism.

Future OBL-021 refinement should wait for a proof package that chooses the
final equality relation, projection-totality statement, and diagnostic
equivalence contract.

## Guard weakness found

The pre-existing OBL-021 sync guard had a maintenance weakness.

`assert_no_vacuous_weakening` stripped Lean comments before checking several
vacuity patterns, but it did not reject a bare `:= True` body. Also, required
OBL-021 body-link checks used raw `assertIn`, so a required predicate name
could appear only in a Lean comment and still satisfy the test.

That weakness did not change the Lean draft or any current sample output. It
made the drift guard weaker than intended.

## Guard hardening applied

The test-only hardening adds:

- a red test showing `assert_no_vacuous_weakening` previously accepted
  `:= True`;
- a red test showing comment-only required body links previously passed;
- a helper that checks required body links after `lean_without_comments`;
- a helper that applies body regex assertions after `lean_without_comments`;
- OBL-001, OBL-020, and OBL-021 body-level required-link / regex checks routed
  through the uncommented-body helpers.

This intentionally hardens the older `plan/117` sync-guard family while keeping
the semantic statement drafts unchanged.

## Relation to G1

This package supports the current G1 bridge by making the statement-boundary
guards harder to satisfy accidentally.

It still does not claim:

- C-static conformance;
- G1 exit;
- OBL-020 completion;
- OBL-021 completion;
- Lean proof skeleton completion;
- proof discharge;
- final equality selection;
- runtime scheduling determinism;
- final diagnostic ABI;
- runtime dispatch.

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No OBL-020 completion.
- No OBL-021 completion.
- No proof discharge.
- No proof skeleton completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean predicate refinement.
- No final equality / Diagnostic / repair / Core IR / runtime / transport /
  projection / public API freeze.
- No sample status relabel.

## Next allowed move

The OBL-020/021 boundary review should be closed for the current G1 bridge.

The next safe package can return to G1 ordinary-assignment support work, with
OBL-020/021 statement refinement kept as reserve-only unless a future proof
package or concrete bridge blocker creates a narrower reason to reopen it.
