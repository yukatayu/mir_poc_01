# plan/117 - G1 OBL-001 / OBL-020 / OBL-021 statement guard hardening

## Purpose

This file records a LAB-only sync-test guard hardening package for the older
OBL-001, OBL-020, and OBL-021 Lean statement-shape drafts.

The package closes a narrow maintenance risk: those drafts were already
compile-checked, but the sync tests only checked manifest registration. A
future edit could weaken the semantic body while preserving compile success and
registration. The new guard keeps the key body-level links explicit and rejects
simple vacuous weakeners in guarded bodies.

This is LAB repository memory. It does not change canon, does not edit
`mirrorea_canon/theory/11-metatheory-ledger.md`, and does not claim OBL-001,
OBL-002, OBL-020, or OBL-021 completion, proof discharge, proof skeleton
completion, G1/T1/T2 exit, conformance, runtime dispatch, runtime scheduling
determinism, final equality selection, final diagnostic ABI, final runtime API,
or final step-family taxonomy.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB predecessor:
  `plan/74-g1-obl001-lean-statement-draft.md`
- LAB predecessor:
  `plan/77-g1-obl021-lean-statement-draft.md`
- LAB predecessor:
  `plan/78-g1-obl020-lean-statement-draft.md`
- LAB artifacts:
  `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`,
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`,
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`
- LAB explanations:
  `samples/lean/lab-statements/obl001/THM001StatementDraft.md`,
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.md`,
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md`
- LAB guard:
  `scripts/tests/test_current_l2_lean_sample_sync.py`
- Mapping input:
  sub-agent findings from 2026-07-04 OBL-001/020/021 statement guard-risk
  mapping.

If this LAB guard conflicts with canon, canon wins.

## Guarded OBL-001 links

The guard checks that `RequestEvidenceSound` still includes:

- `RequestForWrite`
- `OwnerDirectedRequest`
- `RequestCarriesAuthorityObligations`
- `RequestCarriesFailureContainment`
- `RequestCarriesDependencyEvidence`
- `RequestCarriesSpanEvidence`

The guard checks that `GeneratedWriteSound` still keeps each generated write
either owner-local or backed by an existential request with
`RequestEvidenceSound`.

The guard checks that `AssignmentElabSoundnessPost` still includes:

- all generated writes sound;
- all RHS reads recorded;
- generated failures contained;
- authority obligations represented;
- source spans preserved;
- visible write consequences explicit;
- nested-locus non-authority.

The guard checks that `THM001StatementDraft` still requires
`SurfaceAssignment`, `SimpleAssign`, and `ElaboratesAssignment` before the
postcondition.

## Guarded OBL-020 links

The guard checks that `PreservesWF` remains:

```text
WellFormed(before) -> Step(before, label, after) -> WellFormed(after)
```

It also checks that `FamilyStepPreservesWF` still threads `CanonStepFamily` and
`StepHasFamily` into `PreservesWF`, and that `OBL020StatementDraft` still
quantifies `before`, `label`, and `after` and returns `PreservesWF`.

The guard deliberately does not expand canon WF clauses into Lean fields. Those
clauses stay behind `WellFormed` until a later proof-interface decision.

## Guarded OBL-021 links

The guard checks that `SameElabResult` still includes all nine projected output
families:

- Core term
- type
- mode
- effect row
- failure row
- constraints
- obligations
- generated edges
- source spans

It checks that `SameDiagnostic` remains backed by `EquivalentDiagnostic`, that
`ElabDeterministicPost` still contains success/success equivalence,
reject/reject diagnostic equivalence, and success/reject mutual exclusion, and
that `OBL021StatementDraft` still gates the postcondition on `WellScopedInput`.

The guard deliberately does not choose syntactic equality, normalized equality,
definitional equality, alpha-equivalence, or a canon-specific final equality
relation.

## Status

- Sync-unit guard tests now inspect body-level definitions for OBL-001,
  OBL-020, and OBL-021 instead of only checking manifest registration.
- The guard strips Lean comments before simple vacuity checks and rejects
  obvious weakeners such as `True \/ ...`, `... \/ True`, `False -> ...`, and
  trivial proof-shaped bodies in the guarded definitions.
- Explanation and per-directory README files now state the guard boundaries.
- The Lean statement drafts remain compile-check-only `Prop` shapes.
- No canon file was edited.

## Non-claims

- No OBL-001 completion.
- No OBL-002 proof discharge.
- No OBL-020 completion.
- No OBL-021 completion.
- No canon ledger movement.
- No proof skeleton completion.
- No G1/T1/T2 exit.
- No C-static, C-runtime, or C-distributed conformance claim.
- No runtime dispatch or runtime scheduling determinism claim.
- No final equality relation or diagnostic ABI.
- No final step-family taxonomy, scheduler semantics, runtime API, Core IR JSON,
  public API, transport, projection, devtools, telemetry, provider, or product
  completion.
