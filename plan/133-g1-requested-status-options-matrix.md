# plan/133 - G1 requested-status options matrix

## Purpose

This file is LAB repository memory.

It compares `stated` and `lean-stated` as future requested-status options for
OBL-001 / OBL-020 / OBL-021 after the criteria inventory (`plan/130`), packet
outline (`plan/131`), and evidence-readiness dry-run (`plan/132`).

This file does not edit canon, does not close G0 or G1, does not submit a
status proposal, does not move metatheory ledger status, does not complete
OBL-001 / OBL-020 / OBL-021, does not prove OBL-002, does not claim
conformance, does not add an executable row, does not refine a Lean predicate,
and does not change runtime, transport, diagnostic, repair, Core IR, public
API, grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is an advisory
options matrix only; the canon metatheory ledger remains the only proof/status
authority.

## Status vocabulary boundary

`mirrorea_canon/theory/11-metatheory-ledger.md` defines the allowed status
vocabulary:

- `open`
- `stated`
- `lean-stated`
- `lean-proved`
- `external`

This file uses `requested status` to mean "what a later proposal packet might
ask the human/canon process to accept." It does not mean accepted status.

## Local evidence input

| Input | Reading used here |
|---|---|
| `mirrorea_canon/plan/00-gates.md` | G1 exit requires ordinary-assignment theory, OBL-001 Lean statement completion, and OBL-020/021 completion; gate exit requires human decision plus ADR / ledger update. |
| `mirrorea_canon/plan/01-phases.md` | Current canon implementation position remains T0; T1 is paper and Lean statement work after G1 exit conditions are satisfied. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | The ledger is the only proof/status authority; all entries remain open unless canon changes. |
| `plan/130` | Status proposal criteria exist, but no status has been chosen. |
| `plan/131` | Proposal packet slots exist, but requested-status slots are deliberately unfilled. |
| `plan/132` | OBL-001/020/021 Lean drafts compile directly, sync guards pass, and a targeted admitted-stub / placeholder scan passes; this is current LAB support only. |

## Recommendation summary

| OBL | Advisory requested-status recommendation | Reason | Blocking decision before any proposal |
|---|---|---|---|
| OBL-001 | `lean-stated` is the most natural future request, if a canon-facing artifact identity is accepted. | The obligation is explicitly the THM-001 Lean statement, and P79 evidence supports a compile-checked LAB statement artifact with non-vacuity guards. | Human/canon must accept the current LAB artifact or require a canon-facing wrapper; OPEN-014 must be deferred or scoped away without claiming runtime materialization. |
| OBL-020 | Defer full-row status; conditional `lean-stated` only after scope acceptance. | The canon obligation is full well-formedness preservation of step rules, while the current Lean draft is an abstract statement shape that avoids concrete `Config`, `StepLabel`, `StepFamily`, WF clauses, scheduler, and per-step proof boundaries. A pure `stated` fallback would underuse the Lean evidence unless it names an exact statement identity. | Human/canon must choose full OBL-020 vs G1-supporting statement scope and decide whether the abstract vocabulary is acceptable for ledger movement. |
| OBL-021 | Conditional `lean-stated` if the abstraction boundary is accepted; otherwise defer. | The current Lean draft is the right artifact class if canon accepts abstract result / diagnostic equivalence as the statement boundary. The blocker is semantic acceptance, not compile quality. | Human/canon must accept the abstraction boundary or require a narrower/finalized equality and diagnostic statement. |

This summary is intentionally asymmetric. Treating all three obligations as
equally ready for unconditional `lean-stated` would overstate OBL-020/021
readiness; treating all three as only `stated` would underuse the Lean evidence
and create paper/Lean divergence risk.

## Per-OBL option matrix

| OBL | `stated` option | `lean-stated` option | Current advisory reading |
|---|---|---|---|
| OBL-001 | Accepts a precise mathematical THM-001 statement while leaving Lean artifact identity open. This is conservative but weaker than the canon wording "Lean statement". | Accepts a Lean statement artifact as the THM-001 statement identity. Current LAB support is strongest here because the draft directly names the assignment-elaboration postcondition and P79 evidence passed. | Prefer future `lean-stated` request only after artifact identity / wrapper and OPEN-014 deferral are explicit. |
| OBL-020 | Accepts the abstract WF preservation statement shape as a paper/canon statement while leaving concrete Lean target binding to a later package. This still needs exact statement identity and does not avoid the full-vs-G1 scope question. | Accepts the current Lean draft as the ledger statement target. This is plausible only after human/canon accepts the abstract WF vocabulary and full-row vs G1-supporting scope. | Defer full-row status request. Keep conditional `lean-stated` available only after scope acceptance; do not use vague `stated` as a shortcut. |
| OBL-021 | Accepts the abstract determinism/equivalence contract as a paper/canon statement while leaving final equality and diagnostic relations open. This risks paper/Lean divergence if the Lean draft stays separate. | Accepts the current Lean draft as the ledger statement target. This is plausible if human/canon accepts abstract result / diagnostic equivalence as the statement boundary. | Prefer conditional `lean-stated` pending abstraction-boundary acceptance; otherwise defer. |

## Hidden failure modes

| Failure mode | Trigger | Avoidance rule |
|---|---|---|
| Silent ledger movement | A plan/report says an OBL "is lean-stated" instead of "could request lean-stated later." | Always write `advisory requested-status recommendation`, not accepted status. |
| LAB namespace promotion | The packet cites `MirCore.Lab...` constants as if canon already accepted them. | Require artifact-identity acceptance or a canon-facing wrapper decision before any proposal. |
| OBL-020 scope overclaim | `lean-stated` is requested for current abstract WF draft while readers infer full step-family coverage. | Keep OBL-020 full-row status deferred until full-vs-G1-supporting scope is explicitly accepted. |
| OBL-021 equality freeze | `lean-stated` is requested before equality / diagnostic equivalence boundaries are accepted. | Make OBL-021 `lean-stated` conditional on abstraction-boundary acceptance; otherwise defer. |
| OBL-001 underclaim | The future packet requests only `stated` for OBL-001 even though G1 gate text asks for a Lean statement. | Prefer `lean-stated` for OBL-001 once wrapper/artifact identity is accepted. |
| Weak `stated` fallback | A future packet uses `stated` as a vague English summary rather than an exact mathematical statement location. | Require a concrete paper/canon section, statement text, or named statement identity for any `stated` request. |
| Paper/Lean divergence | Canon accepts a paper `stated` form while the Lean statement artifact evolves separately. | Prefer a single accepted statement identity when Lean evidence is already available, or require an explicit paper-to-Lean reconciliation plan. |
| OPEN-014 leakage | OBL-001 status wording is read as deciding transparent read materialization, cache, freshness, or transport policy. | State that OPEN-014 is deferred for static statement/status and not resolved by any requested status. |
| Proof/conformance collapse | `lean-stated` is read as proof discharge or C-static conformance. | Repeat that statement status is not `lean-proved`, proof skeleton completion, runtime behavior, or conformance. |
| G1 exit by implication | A future packet fills status recommendations and readers infer G1 exit. | Keep gate movement separate: human decision plus ADR / ledger update plus acceptance of all G1 criteria is still required. |

## Combined packet strategies

| Strategy | Contents | Benefit | Risk | Advisory result |
|---|---|---|---|---|
| Conservative all-`stated` | Request `stated` for OBL-001/020/021. | Avoids LAB namespace and Lean artifact acceptance risk. | Underclaims OBL-001 and may not satisfy "Lean statement" expectations for G1. | Not preferred. |
| Uniform all-`lean-stated` | Request `lean-stated` for all three. | Fastest path if accepted. | Overclaims OBL-020/021 readiness and may freeze abstract placeholders. | Not safe now. |
| Staged asymmetric request | OBL-001 `lean-stated` candidate; OBL-020 full-row defer with conditional `lean-stated` after scope acceptance; OBL-021 conditional `lean-stated` after abstraction-boundary acceptance. | Matches evidence strength and keeps blockers visible without discarding Lean evidence. | Does not by itself close all G1 statement/status blockers. | Preferred advisory posture. |
| No status request yet | Keep all three open until a canon-facing wrapper or acceptance policy exists. | Safest against accidental canon movement. | Leaves no concrete next proposal path and underuses P79 evidence. | Use only if human/canon rejects advisory status vocabulary. |

## Required wording for a future proposal packet

A later proposal packet should say:

- "This packet requests review of status movement; it does not move status by
  itself."
- "The current canon ledger remains unchanged until human/canon acceptance."
- "OBL-001 `lean-stated` is a candidate only if the Lean artifact identity or
  canon-facing wrapper is accepted."
- "OBL-020 is not claimed as full step-rule WF coverage unless the full-vs-G1
  scope decision is accepted; a conditional `lean-stated` path remains blocked
  until then."
- "OBL-021 conditional `lean-stated` depends on accepting abstract result /
  diagnostic equivalence as the statement boundary; it does not select final
  equality or final Diagnostic ABI."
- "Any `stated` request must name an exact mathematical statement identity; it
  cannot rely on an English summary alone."
- "OPEN-014 is deferred or explicitly scoped away; no runtime read
  materialization policy is selected here."
- "`lean-stated` is not `lean-proved`, proof skeleton completion, C-static
  conformance, runtime readiness, or G1 exit."

## Snapshot update guidance

This matrix changes the task map and roadmap reading, so future closeout should
update:

- `progress.md`: mention `plan/133` as advisory requested-status matrix only.
- `tasks.md`: move the next candidate away from "requested-status options
  matrix" and toward the next selected blocker.

It should not update `samples_progress.md` unless runnable sample or Lean
validation status changes. P80 is docs-only and does not rerun or change sample
status beyond P79's existing evidence-readiness entry.

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
- No exact executable negative evidence claim for SCN-02 negative (b).
- No OPEN-014 resolution.
- No G3 / THM-004 authority proof or production auth claim.

## Next allowed move

The next autonomous package should stay docs-only and choose one of these
follow-ups:

1. prepare an OBL-001 canon-facing wrapper preflight for a later `lean-stated`
   proposal, without editing canon or moving status;
2. prepare an OBL-020 full-vs-G1-supporting scope decision packet;
3. prepare an OBL-021 equality / diagnostic abstraction decision packet.

The safest default is OBL-020 scope clarification, because it is the most likely
to block a coherent three-OBL proposal packet.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, and the package report are
synchronized.

Close condition is advisory-options-only: no canon edit, no gate exit, no
status proposal submission, no requested status acceptance, no OBL status
movement, no proof, no conformance claim, no implementation change, and no
runnable sample status change.
