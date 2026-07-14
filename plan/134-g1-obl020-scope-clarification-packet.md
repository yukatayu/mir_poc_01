# plan/134 - G1 OBL-020 scope clarification packet

## Purpose

This file is LAB repository memory.

It clarifies the scope boundary for any future OBL-020 status proposal after
the requested-status options matrix in `plan/133`.

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not move metatheory ledger status, does not complete
OBL-020, does not prove OBL-020, does not create a proof skeleton, does not
claim conformance, does not add an executable row, does not refine a Lean
predicate, and does not change runtime, transport, Core IR, public API,
grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is a scope
clarification packet only; the canon metatheory ledger remains the only
proof/status authority.

## Problem statement

`mirrorea_canon/plan/00-gates.md` says G1 requires OBL-020 completion.
`mirrorea_canon/theory/11-metatheory-ledger.md` names OBL-020 as
"Well-formedness preservation of step rules" with Lean target
`MirCore.Step.WF`; the ledger remains open.

The current LAB Lean draft,
`samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`, states an
abstract proposition over `WellFormed`, `Step`, `CanonStepFamily`,
`StepHasFamily`, and `PreservesWF`. P79 evidence says it compile-checks and
passes drift / vacuity guards. That is useful statement-shape evidence, but it
does not by itself choose the concrete runtime configuration, step taxonomy,
well-formedness clauses, scheduler semantics, or per-step preservation proof
structure required for full OBL-020 coverage.

Therefore a later packet must not blur:

- full-row OBL-020 completion;
- a G1-supporting statement-scope acceptance;
- Lean statement artifact identity;
- proof discharge;
- runtime or conformance evidence.

## Canon and LAB anchors

| Anchor | Reading for this packet |
|---|---|
| `mirrorea_canon/plan/00-gates.md` | G1 requires OBL-020 completion, but gate exit needs human decision plus ADR / ledger update. |
| `mirrorea_canon/plan/01-phases.md` | Current canon phase remains T0; T2 later expects OBL-020 proof skeleton work. |
| `mirrorea_canon/theory/01-mircore-v0.md` | Defines runtime configuration `H / Q / S / M / G / W / L / P`, selected step rules, and says WF is preserved by every step rule as OBL-020. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | OBL-020 target is `MirCore.Step.WF`; status authority remains the ledger. |
| `plan/76` | Inventories the full step-rule family and warns that the G1 slice must not silently narrow OBL-020 itself. |
| `plan/126` | Says the current abstract Lean statement is sufficient for the current G1 bridge, not for full status movement. |
| `plan/130` | Requires explicit full-vs-G1 scope choice before any OBL-020 status movement proposal. |
| `plan/132` | Records current LAB evidence readiness: OBL-020 Lean draft compile-check and guard checks pass. |
| `plan/133` | Recommends deferring full-row OBL-020 status and keeping conditional `lean-stated` only after scope acceptance. |

## Scope options

| Option | Meaning | Current reading |
|---|---|---|
| Full-row OBL-020 status movement | Ask canon to accept OBL-020 itself as complete or `lean-stated` for the full step-rule WF preservation obligation. | Not safe now. Current LAB artifact is too abstract to imply full step-family coverage, concrete WF clauses, scheduler boundaries, or per-step proof obligations. |
| G1-supporting statement scope | Ask canon/human review to accept that the current abstract WF preservation statement is an acceptable G1-supporting statement shape while leaving full OBL-020 completion open. | Safest near-term posture. It preserves P79 Lean evidence without laundering it into full ledger movement. |
| Defer until proof package | Make no status/scope request until concrete runtime datatypes and proof skeleton work start. | Safe but too conservative as the default; it discards useful statement-shape evidence that can be reviewed without proof discharge. |

## Recommended posture

The next proposal-facing packet should use a **scope-first, no-ledger-movement**
posture:

1. Do not request full-row OBL-020 completion.
2. Do not claim the current `OBL020StatementDraft` is already accepted as
   `MirCore.Step.WF`.
3. Ask only whether the current abstract statement shape is acceptable as a
   **G1-supporting OBL-020 statement scope** for later proposal preparation.
4. Keep full OBL-020 completion, proof skeleton, concrete step taxonomy,
   concrete WF clauses, scheduler semantics, runtime behavior, and conformance
   explicitly open.

If canon/human review rejects scoped statement acceptance, the fallback is
deferral until a proof-package or concrete runtime-semantics package chooses
the missing objects. The fallback is not to broaden LAB evidence silently.

Decision posture:

- B is the advisory recommendation: clarify a narrower G1-supporting OBL-020
  statement-scope candidate for later human/canon review.
- A is deferred: do not request or imply full-row `MirCore.Step.WF` status
  movement from the current LAB abstract draft.
- C is a fallback only if canon rejects scoped statement identity and requires
  concrete proof-package binding first.

Conditional `lean-stated` remains blocked despite the recorded narrow
G1-supporting scope acceptance. A later human/canon process still must decide
whether the abstract `WellFormed` / `Step` / `PreservesWF` vocabulary is
acceptable as statement identity, along with requested status and artifact
identity / wrapper handling.

## Owner disposition recorded in canon

`mirrorea_canon/meta/proposals/PROPOSAL-001-obl020-g1-statement-scope-review.md`
records the project owner's 2026-07-14 answer: `yes` to the narrow
G1-supporting statement-scope question, while full OBL-020 completion remains
open.

This converts the advisory scope posture into owner-accepted proposal
preparation scope only. It does not make the current LAB artifact the canon
`MirCore.Step.WF` target, select `stated` or `lean-stated`, move the ledger,
resolve artifact identity or wrapper need, fill `plan/141` slots, or affect
G0/G1 or T0/T1 state.

## G1-supporting scope content

The G1-supporting scope may cite the current abstract Lean draft only for this
limited claim:

```text
For a chosen abstract vocabulary V and predicates P, if a configuration is
well-formed before a step and the step relation holds, then the resulting
configuration is well-formed after the step.
```

This scope can support the ordinary-assignment bridge because SCN-01 / SCN-02
pressure already needs:

- owner-local write preservation of active-key store discipline;
- request emission preserving explicit request / queue evidence;
- owner-side successful serve and fail-closed serve as later step-family
  obligations;
- publish / observe ancestry preservation for visible write consequences;
- authority / witness carrier presence as a premise, not G3 proof;
- failure-row explicitness, not C-runtime conformance;
- source-span and generated-edge metadata as LAB evidence, not runtime WF.

The scope does not assert that every listed family is proved, concrete, or
complete.

## Exclusions that must stay visible

Any future packet using this scope must explicitly exclude:

- full OBL-020 completion;
- `lean-proved` or proof skeleton completion;
- concrete `Config`, `StepLabel`, `StepFamily`, and `WellFormed` definitions;
- final step-rule taxonomy;
- per-step preservation lemmas;
- scheduler semantics;
- runtime implementation proof;
- C-static, C-runtime, or C-distributed conformance;
- request serving, store mutation, occurrence ordering, admission lifecycle,
  stale-membership runtime failure, or distributed transport claims;
- final Core IR / public API / grammar / runtime ABI freeze;
- G1 exit by implication.

## Hidden failure modes

| Failure mode | Trigger | Avoidance rule |
|---|---|---|
| Full-row laundering | A docs packet says "OBL-020 is lean-stated" from the current abstract LAB draft. | Say "G1-supporting statement-scope review" unless canon explicitly accepts full-row status. |
| Scope split invisibility | The packet talks about OBL-020 support without naming what is outside scope. | Include the exclusions section in any proposal-facing packet. |
| Abstract predicate vacuity | Readers treat abstract `WellFormed` / `Step` predicates as proof of actual runtime preservation. | Cite compile-check and guard evidence only as statement-shape support, not semantic proof. |
| Underclaiming by total deferral | The project waits for T2 proof before asking whether the statement shape is acceptable. | Allow scope review now, while keeping ledger movement blocked. |
| Status vocabulary confusion | `lean-stated` is read as `lean-proved` or as conformance. | Repeat that statement status, even if later accepted, is not proof, conformance, runtime readiness, or G1 exit. |
| Ledger target mismatch | A LAB namespace is cited as though it were already the canon `MirCore.Step.WF` target. | Require artifact-identity or wrapper acceptance before any status request. |
| G1 exit shortcut | OBL-020 scope acceptance is treated as satisfying all G1 conditions. | Keep G1 gate movement separate: human decision plus ADR / ledger update plus all G1 criteria. |

## Required wording for a future proposal packet

A later OBL-020 proposal-facing packet should say:

- "This packet requests review of an OBL-020 scope boundary; it does not move
  ledger status."
- "Full-row OBL-020 completion remains open."
- "The current LAB Lean draft may be considered only as a G1-supporting
  statement-shape candidate unless canon accepts broader status."
- "Concrete runtime configuration, step-family taxonomy, well-formedness
  clauses, scheduler semantics, and per-step proof decomposition remain later
  work."
- "`lean-stated`, if later requested, would be a statement-artifact request,
  not `lean-proved`, proof skeleton completion, runtime readiness,
  conformance, or G1 exit."
- "If canon requires full-row statement identity before any status movement,
  this packet records deferral rather than narrowing OBL-020 silently."

## Decision boundary for later work

This packet makes the next human/canon-facing decision smaller:

```text
Is the current abstract OBL-020 Lean statement shape acceptable as a
G1-supporting scope artifact for proposal preparation, while full OBL-020
completion remains open?
```

Answers and consequences:

| Answer | Consequence |
|---|---|
| Yes | Recorded in `PROPOSAL-001`: a later proposal draft can cite the abstract artifact as accepted G1-supporting scope support, still without applying ledger movement. |
| Yes, but require wrapper | A later package should prepare a canon-facing wrapper or artifact-identity annex before any requested-status draft. |
| No, needs concrete definitions | Defer OBL-020 status work until concrete `Config` / `Step` / WF clauses are chosen. |
| No, needs proof package | Defer OBL-020 status work to T2 proof-skeleton preparation. |

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status accepted.
- No status proposal submission.
- No metatheory ledger movement.
- No OBL completion.
- No proof skeleton completion.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new executable row.
- No Lean predicate refinement.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, or step-family taxonomy freeze.
- No sample status relabel.
- No G1 exit by implication from scope review.

## Next allowed move

The next autonomous package can choose one of these follow-ups:

1. prepare an OBL-020 artifact-identity / wrapper preflight, if a
   canon-facing wrapper is judged necessary;
2. prepare an OBL-001 canon-facing wrapper preflight, because OBL-001 remains
   the strongest later `lean-stated` candidate;
3. prepare an OBL-021 equality / diagnostic abstraction decision packet;
4. draft a non-applied G1 status proposal skeleton only if the scope /
   artifact-identity blockers are explicitly carried as unresolved slots.

The default should not be full OBL-020 ledger movement.

Later-state note, 2026-07-05: `plan/135` / `plan/136` now supply the OBL-020
artifact preflight and annex template, `plan/137` / `plan/138` now supply the
OBL-001 artifact preflight and annex template, `plan/143` now supplies the
OBL-021 abstraction decision packet, `plan/144` keeps this file as the
controlling OBL-020 scope packet, and `plan/145` keeps `plan/137` / `plan/138`
as the controlling OBL-001 artifact decision surface. Do not read the list
above as current permission to duplicate those packets.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is scope-clarification-only: no canon edit, no gate exit, no
status proposal submission, no requested status acceptance, no OBL status
movement, no proof, no conformance claim, no implementation change, and no
runnable sample status change.
