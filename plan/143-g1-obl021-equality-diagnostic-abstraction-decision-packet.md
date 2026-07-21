# plan/143 - G1 OBL-021 equality / diagnostic abstraction decision packet

## Purpose

This file is LAB repository memory.

It defines a narrow, non-applied decision packet for the OBL-021 abstraction
boundary that blocks any later OBL-021 requested-status drafting. The packet
asks what human/canon review would need to accept before the current LAB
`ElabDeterminismStatementDraft.lean` artifact could be used in a conditional
status request.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not choose or accept a requested status, does not move
the metatheory ledger, does not complete OBL-021, does not prove OBL-021, does
not create a proof skeleton, does not create a Lean wrapper file, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, and does not choose final result equality, final diagnostic
equivalence, final Diagnostic ABI, projection-totality, parser/checker
implementation proof, runtime scheduling determinism, runtime behavior, Core
IR, public API, grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is a decision
packet template and local recommendation, not a status authority.

## Inputs

Canon authority:

- `mirrorea_canon/plan/00-gates.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`

LAB OBL-021 memory:

- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md`
- `plan/133-g1-requested-status-options-matrix.md`
- `plan/139-g1-obl021-artifact-identity-wrapper-preflight.md`
- `plan/140-g1-obl021-artifact-annex-template.md`
- `plan/141-g1-status-packet-shell-unresolved-slots.md`
- `plan/142-g1-status-packet-shell-evidence-dry-run.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`

## Packet status

This packet is a decision-prep artifact.

It is narrower than the `plan/141` status packet shell: it concerns only the
OBL-021 abstraction-boundary blocker. It is also narrower than `plan/140`: it
does not fill an artifact annex. It asks which OBL-021 boundary shape a later
annex or status packet is allowed to use.

| State | Meaning | Current file status |
|---|---|---|
| Artifact annex template | Names the OBL-021 artifact and unresolved slots. | Already supplied by `plan/140`. |
| Boundary decision packet | Asks whether the current abstract comparison-predicate boundary is acceptable before status drafting. | This file. |
| Filled annex | Provides fresh validation and selected decision values. | Not created here. |
| Draft status proposal | Chooses requested status and ledger delta proposal text. | Not created here. |
| Submitted / accepted status movement | Human/canon process decides and canon ledger changes. | Not created here. |

## Current OBL-021 boundary

The current LAB artifact is:

```text
samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft
```

The canon ledger target is:

```text
OBL-021 / Elaboration determinism / MirCore.Elab.Det
```

The current statement shape says that for a fixed well-scoped input:

- two successful elaborations have `SameElabResult`;
- two rejections have `SameDiagnostic`;
- the same input cannot both elaborate successfully and reject.

`SameElabResult` is component-wise and abstract. It names comparison predicates
for Core term, type, mode, effect row, failure row, constraints, obligations,
generated edges, and source spans. The LAB draft assumes no reflexivity,
symmetry, transitivity, adequacy, or extensionality law for those predicates.
`SameDiagnostic` likewise delegates only to the abstract
`EquivalentDiagnostic` predicate.

WRK-0002 through WRK-0005 add bounded LAB evidence about this shape. The draft
does not entail result identity, projection non-vacuity, or outcome existence.
For one fixed well-scoped input it does entail a possibly-vacuous all-pairs
`SameOutcome` relation on actual outcomes; a separate experimental totality
premise makes that fiber nonempty. This is partial relational coherence, not a
selected equality/equivalence, total function contract, or Canon statement.

This is statement-shape evidence. It is not a proof that the implementation is
deterministic.

## Central question

The smallest useful human/canon question is:

```text
For OBL-021 requested-status drafting, is the current abstract comparison-predicate
boundary acceptable as statement-status vocabulary, or must the project first
choose stronger concrete equality / diagnostic / joint-adequacy relations?
```

This question must stay separate from:

- whether the LAB path / namespace / constant is accepted as the requested
  artifact;
- whether a canon-facing wrapper is required;
- what requested status, if any, would be requested;
- whether the ledger moves;
- whether OBL-021 is proved or complete.

## Decision axes

The packet should ask for decisions on four axes.

### Axis A - result equality boundary

| Option | Meaning | Consequence |
|---|---|---|
| A1 Abstract component comparisons accepted | `SameElabResult` and its component predicates are acceptable statement-status vocabulary for this checkpoint, without asserting relation laws. | A later packet may request conditional OBL-021 status using the current abstract result-comparison boundary, still without proof or conformance. |
| A2 Concrete equality required first | The packet must choose concrete equality / equivalence relations for result components before any status request. | Open a statement-refinement package before OBL-021 status drafting. |
| A3 Joint adequacy or direct Result relation required first | Projection totality / uniqueness alone is insufficient; the packet must state or prove a joint extensionality/adequacy bridge, or select a direct Result relation. | Defer status drafting or refine the statement shape to expose the selected bridge. |
| A4 Defer | Human/canon review does not accept or reject the result boundary yet. | Keep OBL-021 requested-status work deferred. |

### Axis B - diagnostic comparison boundary

| Option | Meaning | Consequence |
|---|---|---|
| B1 Abstract diagnostic comparison accepted | `SameDiagnostic` / `EquivalentDiagnostic` is acceptable statement-status vocabulary for reject/reject determinism, without asserting relation laws or theory/10 adequacy. | A later packet may keep Diagnostic ABI details outside OBL-021 status drafting, with explicit non-claims. |
| B2 Final Diagnostic ABI required first | A final or canon-facing Diagnostic carrier/equality contract must exist before OBL-021 status request. | Defer OBL-021 status drafting to a diagnostic-boundary package. |
| B3 Diagnostic soundness link required first | OBL-021 status must wait until the relation to OBL-024 diagnostic soundness is stated more concretely. | Open a cross-OBL boundary package before status drafting. |
| B4 Defer | Human/canon review leaves diagnostic comparison unresolved. | Keep OBL-021 requested-status work deferred. |

### Axis C - artifact identity / wrapper boundary

| Option | Meaning | Consequence |
|---|---|---|
| C1 Direct LAB artifact acceptable for requested artifact | The LAB path / namespace / constant may be named as the requested OBL-021 artifact. | A later packet may cite it directly, while still marking it LAB until canon accepts status. |
| C2 Wrapper required | A non-applied canon-facing wrapper or renamed statement target is required before status request. | Open a wrapper package under `plan/139` constraints before status drafting. |
| C3 Artifact identity deferred | Artifact identity should wait for equality / diagnostic / projection decisions. | Keep OBL-021 requested-status work deferred. |

### Axis D - fixed-input identity / non-vacuity boundary

| Option | Meaning | Consequence |
|---|---|---|
| D1 Current fixed-input abstraction accepted | `env`, `ctx`, `locus`, `item`, plus `WellScopedInput`, are acceptable as the fixed-input boundary for statement-status vocabulary. | A later packet may keep canonical input snapshot / context equality outside OBL-021 status drafting. |
| D2 Canonical input snapshot required first | Status drafting must define equality or canonicalization for environment, context, locus, and Surface item. | Open a statement-refinement or input-snapshot boundary package before OBL-021 status drafting. |
| D3 Projection non-vacuity plus joint adequacy required first | Status drafting must state that successful results expose enough projections for non-vacuous comparison and select a joint adequacy / direct-Result bridge. | Refine OBL-021 statement shape before status drafting. |
| D4 Defer | Human/canon review leaves fixed-input identity unresolved. | Keep OBL-021 requested-status work deferred. |

## Current LAB recommendation

The current LAB recommendation is:

- Ask Axis A and Axis B before any OBL-021 status draft.
- Ask Axis D in the same packet, because abstract result comparison can look
  stronger than it is if fixed-input identity or projection non-vacuity remains
  implicit.
- Treat A1 and B1 as plausible scoped acceptances for **statement-status
  vocabulary only**, because OBL-021 is a statement target and proof is later.
- Keep C1 / C2 / C3 unresolved until the same human/canon review decides
  whether `MirCore.Lab...` may be cited directly.
- Do not create a wrapper now.
- Do not request `lean-stated` now.

This recommendation is advisory. It is not canon acceptance.

## Relation to earlier LAB memory

`plan/77` originally asked what canon-confirmed equivalence relation should
replace the abstract predicate fields if OBL-021 moved toward real statement
status. Later memory has narrowed that into a choice rather than a requirement:
human/canon review may accept the abstract boundary for statement-status
vocabulary, require concrete equality / Diagnostic / projection-totality
relations first, or defer. This file records that later reading and does not
rewrite the old draft into an accepted decision.

## What each outcome would unblock

| Outcome | Unblocks | Does not unblock |
|---|---|---|
| A1 + B1 accepted, C unresolved | A later OBL-021 annex can say the comparison-predicate boundary is accepted for statement-status drafting. | Artifact identity, requested status, ledger movement, proof, conformance, G1 exit. |
| A1 + B1 + C1 accepted | A later status proposal draft may cite the current LAB artifact as the requested artifact, if fresh validation is rerun. | Canon status movement until submitted and accepted; proof / final ABI claims. |
| A1 + B1 + C2 accepted | A wrapper package may be opened with strict non-applied wording. | Status request until wrapper and validation package close. |
| A2 or A3 selected | A statement-refinement or joint-adequacy/direct-Result package should happen before status drafting. | Direct status proposal. |
| B2 or B3 selected | A diagnostic-boundary or OBL-024 relation package should happen before status drafting. | Direct status proposal. |
| D2 or D3 selected | A fixed-input identity or projection non-vacuity refinement should happen before status drafting. | Direct status proposal. |
| Any axis deferred | OBL-021 status remains deferred. | Silent assumption that the current draft is accepted. |

## Packet text for later human/canon review

A later human/canon-facing packet should ask:

```text
OBL-021 boundary question.

Current LAB artifact:
  samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean
  MirCore.Lab.OBL021.StatementDraft.OBL021StatementDraft

Canon ledger target:
  OBL-021 / MirCore.Elab.Det

Decision requested:
  1. Are abstract component result comparison predicates acceptable as OBL-021
     statement-status vocabulary at this checkpoint?
  2. Is the abstract diagnostic comparison predicate acceptable as OBL-021
     statement-status vocabulary at this checkpoint?
  3. Does OBL-021 status drafting require direct LAB artifact acceptance,
     a canon-facing wrapper, or deferral?
  4. Is the current fixed-input abstraction non-vacuous enough, or must the
     project first define canonical input equality / snapshot and a joint
     projection adequacy / direct-Result bridge?

Non-claim:
  This decision does not prove OBL-021, complete OBL-021, move the ledger,
  select final equality, freeze Diagnostic ABI, prove projection-totality,
  claim conformance, claim runtime scheduling determinism, or exit G1.
```

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status chosen or accepted.
- No status proposal submitted.
- No metatheory ledger movement.
- No OBL-021 completion.
- No OBL-021 proof skeleton completion.
- No OBL-021 proof discharge.
- No OBL-001 / OBL-002 proof claim.
- No OBL-020 proof claim.
- No OBL-024 / OBL-025 diagnostic or repair proof claim.
- No final equality relation.
- No final diagnostic equivalence contract.
- No final Diagnostic ABI.
- No projection-totality proof.
- No canonical input snapshot or same-input equality relation.
- No parser/checker implementation proof.
- No runtime scheduling determinism claim.
- No C-static, C-runtime, or C-distributed conformance claim.
- No Lean wrapper file.
- No Lean predicate refinement.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, or distributed
  transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, source-map ABI, or assignment taxonomy freeze.
- No sample status relabel.

## Drift checks before reuse

Before copying this decision packet into a later review artifact, recheck:

1. `mirrorea_canon/theory/11-metatheory-ledger.md` still names
   `OBL-021 / MirCore.Elab.Det`;
2. `mirrorea_canon/theory/03-elaboration.md` still treats determinism as an
   elaboration contract clause;
3. `mirrorea_canon/theory/10-diagnostics.md` has not fixed a new final
   Diagnostic ABI that changes the diagnostic-equivalence question;
4. the LAB artifact path, namespace, and constant still exist;
5. the statement body still links `SameElabResult`, `SameDiagnostic`, and
   `ElabDeterministicPost`;
6. `plan/139` / `plan/140` / `plan/141` have not been superseded by an accepted
   wrapper, annex, or status decision;
7. no fresh canon decision has changed status vocabulary or artifact identity;
8. no later package has defined canonical input snapshot, context equality, or
   projection non-vacuity in a way that supersedes Axis D.

## Next allowed moves

Reasonable next packages are:

1. prepare an OBL-020 full-row vs G1-supporting scope decision packet, leaving
   OBL-021 unresolved;
2. if the user explicitly promotes human/canon review, prepare a review-facing
   decision request using this packet without ledger movement;
3. prepare a draft status proposal only after requested status, OBL-020 scope,
   OBL-021 boundary, artifact identity, and fresh validation slots are all
   deliberately filled.

## Close condition

This file is closed when it is registered in the plan/source-hierarchy
scaffolds, `plan/00-index.md`, `plan/90-source-traceability.md`,
`Documentation.md`, `progress.md`, `tasks.md`, and the package report are
synchronized.

Close condition is decision-packet-only: no canon edit, no gate exit, no
requested status choice, no OBL status movement, no proof, no conformance
claim, no implementation change, and no runnable sample status change.
