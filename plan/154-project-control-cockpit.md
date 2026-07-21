# plan/154 - Project control cockpit and reporting protocol

## Purpose

This LAB plan defines the two-document reporting route requested by the owner:

- `docs/project-status.md` is the concise, continuously updated human-facing
  control view.
- This file is the detailed LAB execution, stop, decision, and maintenance
  protocol behind that view.

Neither document is normative. `mirrorea_canon/` remains the only normative
source. Canon lifecycle, Gate, Phase, OBL, proof, and conformance displays may
change only after an effective canonical record changes through the canon
process. An owner-recorded LAB decision may only change a separate
"decision recorded / canon reflection pending" reading until that record exists.

## Current baseline

- Canonical lifecycle position: `T0/G0 rebaseline`.
- No G0 exit, T1 entry, OBL completion, proof discharge, conformance result,
  or runtime/product readiness is created by this plan.
- The current delegated-research policy is ADR-0014 plus
  `plan/157-delegated-theory-research-governance.md`; `plan/156` remains the
  pre-delegation evidence record.
- No Gate/Phase or implementation package is promoted by this baseline.

## Design

### Document roles

| Document | Role | May state | Must not do |
| --- | --- | --- | --- |
| `docs/project-status.md` | Human-facing derived control view | Whole-plan checklist, current position, research lifecycle, active owner decisions, and exact source links | Create a decision, promote a package, move a Gate/Phase, or duplicate detailed evidence |
| `plan/154-project-control-cockpit.md` | LAB execution and maintenance protocol | Update transaction, source hierarchy, detailed decision routing, validator contract, and non-claims | Override canon or replace `progress.md` / `tasks.md` / `samples_progress.md` |
| `progress.md` | Concise LAB status snapshot | Workflow readiness, evidence classification, macro phase, and recent log | Serve as a canon decision record |
| `tasks.md` | Current LAB task map | Promoted package, bounded work, and decision gates | Grant autonomous roadmap authority |
| `samples_progress.md` | Runnable-sample dashboard | Reproducible workflow status and validation anchors | Claim product or conformance completion from helper evidence |
| `docs/reports/` | Immutable execution evidence | What a completed package changed and validated | Be overwritten to represent current state |

### Human-facing control view

`docs/project-status.md` stays at or below 180 lines. Its fixed validator
headings present four reader panels rather than a historical task log:

1. **canonical state**: current lifecycle and the Gate/Phase checklist;
2. **runnable LAB evidence**: what can actually be reproduced, with its
   non-claim boundary;
3. **research lifecycle and decision queue**: current candidate state plus
   unresolved owner decisions, with deferred items marked dormant rather than
   repeatedly presented as active prompts; and
4. **roadmap**: the next evidence-first research order, with the exact sources
   that constrain it.

The checklist is an orientation aid, not a percentage or an exit claim. A box
is checked only when a cited canonical record establishes the corresponding
exit. The report contains no historical log, command output, copied Gate
criteria, sample matrix, Oracle transcript, or recommendation that has not
been recorded elsewhere.

### Current decision routing

`plan/155` records G0-D1/D2/D4. G0-D3 is deferred and dormant: it re-enters
the queue only through an owner reopen, not because an agent needs a next task.
`plan/156` records the pre-delegation research envelope and its bounded evidence.
ADR-0014 plus `plan/157` now govern autonomous LAB research and the exact-row,
review-gated route for a canon L2/L3 update. They do not grant Gate/Phase
movement, ADR effectivity, any proof-ledger status, or OBL discharge authority.

## Update transaction

For every non-trivial package that changes the questions answered by the
control view, update in this order:

1. change the authority-bearing canon source or bounded LAB evidence;
2. update `progress.md`, `tasks.md`, and `samples_progress.md` when their
   respective status changes;
3. update `docs/project-status.md` last among mutable status views;
4. run documentation and source-hierarchy validators;
5. finalize a new `docs/reports/` report with exact commands, results, update
   statuses, and non-claims; it becomes immutable at this closeout point.

Update the control view when a canon lifecycle reference, promoted-package
state, controlling stop packet, owner decision, runnable classification, or
load-bearing report path changes. Do not update its timestamp merely for an
unrelated commit or validation rerun.

If a required source cannot be resolved, state `STALE - source reconciliation
required` in the control view and do not infer a replacement state.

## Validator contract

The existing documentation validator is the enforcement point. It must:

- require `docs/project-status.md` and this numbered plan file;
- require the control-view headings in order;
- require its non-normative / canon-precedence notice;
- require a source path in each load-bearing current-state, stop, decision,
  and evidence section; inspect every path-like code span there and verify
  that each resolves to a repository file without whitespace ambiguity,
  traversal, directory, or external-symlink escape;
- require checked Gate / Phase boxes to carry a same-line existing canonical
  record path, without evaluating that record's semantics;
- include the report in source-hierarchy and host-path reader-facing lints;
- require every future latest task report to state whether
  `docs/project-status.md` was updated or did not need an update, using one
  exact-path, single-line `## Files changed` bullet as update evidence.

This is a structural drift guard only. It does not derive `mir-conform` JSON,
compare proof semantics, create an owner decision, or validate a Gate exit.

## P110 implementation plan

### Task 1: Establish the detailed protocol and indexes

Files:

- Create: `plan/154-project-control-cockpit.md`
- Modify: `plan/00-index.md`
- Modify: `scripts/validate_docs.py`
- Modify: `scripts/check_source_hierarchy.py`

Steps:

- [x] Write the authority boundary, document roles, update transaction, and
  P109 decision routing in this plan.
- [x] Register this numbered plan in the plan index and both documentation
  scaffold registries.
- [x] Add the control-view validator constants and fail-closed checks.
- [x] Run the focused validator tests and the full documentation validator.

### Task 2: Publish the concise human-facing control view

Files:

- Create: `docs/project-status.md`
- Modify: `Documentation.md`
- Modify: `progress.md`
- Modify: `tasks.md`

Steps:

- [x] Create the concise checklist-oriented report using only cited sources.
- [x] Add one reader-facing entry point from `Documentation.md`.
- [x] Record the new reporting route in current LAB snapshots without changing
  canon lifecycle, runnable classification, or package authority.
- [x] Validate source hierarchy, structural current-position references, and
  exact links.

### Task 3: Make future report closeouts account for the control view

Files:

- Modify: `AGENTS.md`
- Modify: `docs/reports/TEMPLATE.md`
- Modify: `scripts/validate_docs.py`
- Modify: `scripts/tests/test_validate_docs.py`

Steps:

- [x] Add `docs/project-status.md update status` immediately after the
  `Documentation.md` status in the latest-report template contract.
- [x] Add a focused failing validator test, then the smallest implementation
  that makes it pass.
- [x] Keep historical reports untouched; only the latest-report contract is
  extended.
- [x] Run focused tests, documentation checks, and a diff review.

### Task 4: Close P110 with evidence

Files:

- Create: `docs/reports/2248-p110-project-control-cockpit.md`
- Modify: `progress.md`
- Modify: `tasks.md`

Steps:

- [x] Record the consulted sources, Oracle advisory result, tests, non-claims,
  and all dashboard update statuses in the required report structure.
- [x] Close the bounded P110 package while leaving no autonomous successor
  promoted.
- [x] Commit with `--no-gpg-sign`, push, and confirm a clean branch.

## Non-claims

This reporting route does not change canon, Gate or Phase state, the OBL
ledger, `mir-conform` coverage, executable sample status, implementation
readiness, product readiness, or the authority held by the owner.
