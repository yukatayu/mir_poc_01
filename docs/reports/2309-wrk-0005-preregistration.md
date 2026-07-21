# Report 2309 - WRK-0005 conditional outcome-relation pre-registration

- Date: 2026-07-21 20:36 JST
- Author / agent: Codex
- Scope: Select and commit a bounded L3 conditional lemma candidate before running its outcome checks.
- Decision levels touched: L3 only. No L0/L1, theory/11, contract, SCN, Gate, Phase, proof, implementation, or public-state movement.

## Objective

Pre-register a small Lean conditional lemma that makes outcome existence an
explicit experiment-local premise and tests the pairwise relation already
provided by the LAB OBL-021 draft.

## Scope and assumptions

Canon is authoritative. WRK-0004 established that the LAB draft itself does not
entail outcome existence. This candidate introduces only an experiment-local
tagged `Outcome`, `OutcomeOf`, `SameOutcome`, and totality premise in an
existing Lean evidence lane. It selects no final equality, relation law,
quotient, Diagnostic ABI, or Canon placement.

## Start state / dirty state

Started from pushed, clean `main` at `2f33576a`. WRK-0002 through WRK-0004 are
manifested L3 `not-promoted` evidence. No source for the WRK-0005 conditional
lemma exists and no outcome command has run.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014,
  `working/README.md`, and `plan/01-phases.md` / `02-operating-model.md`.
- `mirrorea_canon/theory/03-elaboration.md`, `10-diagnostics.md`, and
  `11-metatheory-ledger.md`.
- WRK-0002 through WRK-0004, `plan/143`, `plan/158`, and `plan/159`.
- The existing OBL-021 statement draft and the completed Oracle/planner review
  conclusions recorded in Reports 2304--2308.

## Actions taken

- Used the outcome-totality countermodel to require an explicit premise rather
  than treating existence as implicit.
- Selected a tagged experimental relation whose three branches correspond
  exactly to the existing draft's success-success, reject-reject, and mixed
  clauses.
- Created WRK-0005 with pinned inputs, alternative, falsifier, rollback, exact
  commands, and explicit non-claims.

## Files changed

- `mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2309-wrk-0005-preregistration.md`

## Commands run

- Read-only Canon/LAB inspection with `sed`, `rg`, pinned SHA-256 snapshots,
  and `git rev-parse HEAD`.
- No WRK-0005 Lean compile, missing-file red check, source audit, or outcome
  command.

## Evidence / outputs / test results

- The current LAB draft has the three pairwise branches needed for the tagged
  relation, but does not provide outcome existence; WRK-0004 is the negative
  control for that premise.
- WRK-0005 is only pre-registered. Its target Lean source does not yet exist
  and no positive or negative outcome is claimed.

## What changed in understanding

The strongest immediately testable bridge-free reading is conditional: existence
is a stated premise, while pairwise compatibility remains a derived abstract
relation. This remains strictly weaker than native equality, an equivalence
law, quotient semantics, or a Canon function contract.

## Open questions

- Whether the registered conditional lemma compiles without additional premises
  remains untested until after this commit.
- Whether the experimental `SameOutcome` has suitable relation laws, or any
  Canon relevance, remains unresolved and out of scope.

## Suggested next prompt

Run WRK-0005's registered red/green Lean commands, retain the conditional lemma
only if its totality premise and all outcome-pair cases are explicit, and keep
all relation-law and Canon-placement questions open.

## Plan update status

更新不要: registration changes only Canon working metadata and current task
selection; the detailed LAB evidence plan belongs after the registration commit.

## Documentation.md update status

更新不要: the high-level reader route did not change.

## docs/project-status.md update status

更新済み: the reader status now distinguishes WRK-0005 pre-registration from
unrun evidence.

## progress.md update status

更新済み: the recent log records the L3 registration and explicit-premise
boundary.

## tasks.md update status

更新済み: conditional-relation pre-registration is closed and WRK-0005 evidence
is the current package.

## samples_progress.md update status

更新不要: no active sample status, validation command, or dashboard row changed
before evidence execution.

## Reviewer findings and follow-up

The prior Oracle review suggested an abstract Result relation plus an adequacy
bridge; the planner placed outcome-totality first. This candidate is their
bounded intersection: it checks only the draft's pairwise relation under an
explicit experimental totality premise. No review result changes Canon.

## Skipped validations and reasons

The registered Lean outcome commands are intentionally deferred until after
this commit. Running them first would violate the committed outcome order.
Documentation/index/source-hierarchy validation is run for the registration
itself.

## Commit / push status

This pre-registration package is committed and pushed after documentation and
Canon-index validation, before any outcome evidence is generated.

## Sub-agent session close status

No sub-agent is active.
