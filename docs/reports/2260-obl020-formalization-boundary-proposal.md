# Report 2260 - OBL-020 formalization-boundary proposal

- Date: 2026-07-17
- Author / agent: Codex
- Scope: decision-ready design memo preparation after bounded LAB research
- Decision levels touched: L1 decision requested; none adopted

## Objective

Prepare the smallest canon-process decision request needed after T-RESEARCH-006
showed that the selected OBL-020 rule x well-formedness matrix has no
derivation-complete canon premise set. Do not adopt a transition, frame,
history, record, or Lean proof interface.

## Scope and assumptions

The owner controls L0/L1 decisions, ADR effectivity, Gate exits, and proof
status. A proposal is a non-self-executing design memo. The existing owner
acceptance of the abstract OBL-020 statement as a G1-supporting scope artifact
under PROPOSAL-001 remains unchanged and does not choose concrete semantics.

## Start state / dirty state

The worktree was clean at `60dcecfb Record T-RESEARCH-006 source adequacy audit`.
T-RESEARCH-006 was `research-complete` with `0 direct / 65 missing`; its five
gap groups were LAB evidence only. No OBL-020 proof-facing canon definition
existed.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `CANON.md`, and
  `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/plan/00-gates.md`, `01-phases.md`, and `02-operating-model.md`
- `mirrorea_canon/meta/agent-instructions.md` and PROPOSAL-001 / PROPOSAL-002
- `mirrorea_canon/theory/01-mircore-v0.md`, `04-ordering-and-cuts.md` through
  `08-patch-hotplug.md`, and `11-metatheory-ledger.md`
- `plan/76`, `plan/78`, `plan/126`, `plan/134`, `plan/156`, and Report 2259
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and `.docs/progress-task-axes.md`

## Actions taken

- Rechecked the existing OBL-020 abstract statement draft and its sync guard.
  It intentionally keeps concrete WF clauses and step behavior behind
  `WellFormed` / `Step`; no LAB predicate refinement was made.
- Classified the next obstacle as an L1 formalization-organization choice, not
  as a missing Lean convenience abstraction.
- Created PROPOSAL-003 with three symmetric alternatives: a shared five-heading
  LAB-derived review checklist, no required shared checklist with package-local
  organization, or defer. A is recorded only as the LAB bundle's advisory
  recommendation.
- Regenerated `mirrorea_canon/INDEX.json` from the canon root.
- Synced the decision-ready state into LAB plan and current snapshots without
  adopting any option.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-003-obl020-formalization-boundary-review.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `docs/reports/2260-obl020-formalization-boundary-proposal.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`
- `tasks.md`

## Commands run

- canon / LAB source searches and focused reads
- `python3 meta/build-index.py` from `mirrorea_canon/`
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `lean --trust=0 samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `git diff --check` and focused source-to-proposal term inspection
- focused Oracle proposal review and one permitted retry; both browser runs
  ended with a terminal browser-disconnection error, and the retry saved a
  usable transcript

## Evidence / outputs / test results

- PROPOSAL-003 is `L3-open`; it contains no owner disposition, ADR, or semantic
  equation.
- Its A/B/C question requests only organizational posture for future OBL-020
  proof-facing packages. The five headings are LAB-derived, non-exhaustive
  review categories, not canon vocabulary, theorem premises, or a fixed Lean
  interface.
- `meta/build-index.py` reported `ok: 73 files indexed` after adding the
  proposal.
- `make check` passed: source hierarchy (704/704), documentation validation,
  and `cargo check` all passed.
- The Lean sample sync suite passed (21 tests), and the existing abstract
  statement compiled under `lean --trust=0`.
- The active Surface sample corpus passed all 53 checks. Its own output still
  marks `workflow_ready: false`; this package does not change that status.
- Local source inspection removed an over-specific `explicit reacquisition`
  phrase from the chain checklist because it was not among the frozen
  source-adequacy group's stated details. The regenerated canon index records
  the corrected proposal.
- The retry's saved Oracle transcript produced concrete scope edits. Accepted
  edits make the alternatives symmetric and preserve the five groups as LAB
  review headings rather than canon or Lean-interface requirements.

## What changed in understanding

The first next step after a complete source-adequacy audit is not to encode its
gaps as new Lean fields. That would choose an unapproved history/frame/record
semantics in LAB. Nor should the audit taxonomy itself become an implicit
proof-premise or record-update architecture. The correct immediate artifact is
a narrow owner decision about whether later packages use a common LAB-derived
review index; it leaves definitionally supplied force, abstract interfaces, and
package-local proof organization available.

## Open questions

- The owner must choose A, B, C, or return PROPOSAL-003 for clarification.
- If A is accepted, a later canon proposal must still specify concrete carriers,
  equations, rule applicability, and proof obligations one at a time.
- OBL-021 and the OBL-001 bridge remain independently researchable and are not
  decided by this proposal.

## Suggested next prompt

Record an owner disposition for PROPOSAL-003: `A accepted`, `B accepted`,
`C deferred`, or `return for clarification` with the ambiguous family or scope.

## Plan update status

`plan/` updated: `plan/156` records that the formalization boundary is
decision-ready, identifies the proposal, and keeps unrelated research open.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points and source hierarchy
did not change.

## docs/project-status.md update status

更新済み: the concise control view lists the PROPOSAL-003 A/B/C decision and
states that no concrete transition or WF premise is adopted before an owner
record.

## progress.md update status

`progress.md` updated: it distinguishes the preceding research-complete audit
from the new OBL-020-only decision-ready boundary.

## tasks.md update status

`tasks.md` updated: T-RESEARCH-007 is decision-ready, and the next autonomous
selection must be independent of the pending OBL-020 choice.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no active sample, validation command,
debug surface, or runnable workflow classification changed.

## Reviewer findings and follow-up

A focused Oracle source/scope review found that the initial wording could make
the A/B/C choice asymmetric and could accidentally promote the five LAB audit
groups into required theorem premises or a fixed proof interface. The review
also found that the initial question could be read as blocking already permitted
exploratory work, and that the original wording overstated A's comparative
narrowness.

Applied findings: the proposal now presents symmetric A/B/C organizational
postures; marks the five groups as non-exhaustive LAB review headings; preserves
PROPOSAL-001 under all options; allows definitionally supplied or abstract proof
force; removes the record-update classification; and states that no option is a
prerequisite for bounded LAB research. The local review retained the exact
source-audit details, omitting `incarnation` from the state/membership heading
because it was not among T-RESEARCH-006's frozen group details.

The first browser session and its single permitted retry both ended with a
browser-disconnection error. The retry's saved transcript contained the complete
advisory review used above. Model-picker selection was not verified by the
wrapper; the advice was checked against local canon and LAB evidence before use.
No further duplicate review was started.

## Skipped validations and reasons

No runtime, distributed, conformance, or proof validation is applicable because
the package adds only a non-self-executing decision request and index entry.

## Commit / push status

Pending after validation and focused review. The package will be committed with
`--no-gpg-sign` and pushed before selecting a further research line.

## Sub-agent session close status

No local sub-agent session was available. The focused Oracle review is closed:
the retry's saved transcript supplied a complete advisory response despite the
terminal browser-disconnection error.
