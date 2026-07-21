# Report 2304 - WRK-0003 evidence-packaging correction

- Date: 2026-07-21 20:22 JST
- Author / agent: Codex
- Scope: Correct the aggregate theorem packaging of already manifested WRK-0003 evidence without rewriting its history or changing its registered research question.
- Decision levels touched: L3 evidence clarification only. No L0/L1/L2, OBL status, theory ledger, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Ensure one named Lean theorem packages the full advertised conjunction: all
nine total/unique projections, all component-equality conditions, the draft,
and distinct successful Results.

## Scope and assumptions

The prior evidence commit and manifest remain historical facts. A temporary
Oracle review identified that the former summary theorem packaged only the
`CoreTermOf` totality/uniqueness conjunct, although the individual theorems
already established the other facts. This correction adds new source evidence
in the already permitted `plan` and `samples/lean` lanes; it does not amend
history or alter the research question.

## Start state / dirty state

Started from pushed, clean `main` at `708dbe02`. WRK-0003 was already
manifested as L3 `not-promoted`. The source contained four valid individual
theorems and a too-narrow aggregate theorem.

## Documents consulted

- `mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md`,
  `theory/03-elaboration.md`, ADR-0014, and `plan/159`.
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean`
  and its companion explanation.
- Report 2302, Report 2303, and the temporary Oracle review output.

## Actions taken

- Inspected the exact aggregate theorem after the Oracle review identified its
  missing eight projection clauses and component-equality premise.
- Replaced it with
  `total_unique_equality_projections_still_allow_distinct_results`, which
  conjoins the four already-checked facts directly.
- Updated the LAB plan and explanation to make the aggregate theorem part of
  the reproducible evidence contract.

## Files changed

- `plan/wrk-0003-projection-extensionality-countermodel.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.md`
- `docs/reports/2304-wrk-0003-evidence-packaging-correction.md`

## Commands run

- Focused source inspection with `nl -ba`.
- External `.olean` build of the existing statement draft and
  `LEAN_PATH=/tmp/wrk-0003-lean-artifact lean ...ProjectionExtensionalityCountermodel.lean`.
- Original registered static audit plus an additional audit for
  `total_unique_equality_projections_still_allow_distinct_results`.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- `python3 scripts/validate_docs.py`.
- `(cd mirrorea_canon && python3 meta/build-index.py --check)`.

## Evidence / outputs / test results

- Lean 4.29.1 checks the corrected aggregate theorem using the externally built
  imported draft.
- The aggregate theorem now has the exact conjunction of
  `projection_predicates_are_total_and_unique`,
  `component_equivalences_are_equality`, `OBL021StatementDraft V P`, and the
  two distinct successes.
- The original required-name/forbidden-token audit, the additional aggregate
  theorem check, all 21 Lean sync tests, documentation validation, and Canon
  index check pass.

## What changed in understanding

The countermodel's logical conclusion did not change, but its retained proof
surface now states the full advertised premise set in one theorem. This removes
an avoidable ambiguity between a collection of separately checked facts and
the countermodel conclusion.

## Open questions

- Whether the current draft also permits a well-scoped input with neither a
  successful Result nor a Diagnostic is the immediate next falsifiable question.
- Which abstract Result relation and observational-adequacy bridge should be
  compared remains deferred until after that outcome-totality check.
- Final equality, Diagnostic ABI, proof/discharge, and all Canon lifecycle
  decisions remain unresolved and owner controlled.

## Suggested next prompt

Append this correction evidence to WRK-0003, record the Oracle finding as an
advisory reviewer result, then pre-register the outcome-totality countermodel
before comparing bridge shapes.

## Plan update status

更新済み: the detailed WRK-0003 plan now requires the full aggregate theorem.

## Documentation.md update status

更新不要: no high-level reader route changed.

## docs/project-status.md update status

更新不要: the source correction remains unmanifested additional L3 evidence
until the next append-only WRK update.

## progress.md update status

更新不要: workflow status remains the same manifested WRK-0003 research line
until the correction evidence is append-only manifested.

## tasks.md update status

更新不要: the correction itself does not replace the next selected package;
outcome-totality becomes current only when the following manifest synchronizes
the task snapshot.

## samples_progress.md update status

更新不要: no active runnable sample or dashboard workflow changed.

## Reviewer findings and follow-up

The temporary Oracle review was advisory but actionable: it found the aggregate
theorem's packaging omission, while accepting the model-level countermodel as
otherwise valid. The local source inspection confirmed the issue and this
correction addresses only that packaging defect. Oracle's recommendation for an
abstract Result relation plus an observational-adequacy bridge is deferred
behind the planner's more primitive outcome-totality countermodel, and is not
adopted as Canon.

## Skipped validations and reasons

No broad Cargo test, runtime suite, or clean-worktree validation was run. They
do not test this Lean theorem packaging correction, and no WRK-0003 required
validation was skipped.

## Commit / push status

This correction source-evidence package is committed and pushed at task
closeout before a separate append-only WRK manifest update.

## Sub-agent session close status

The planner sub-agent completed a read-only sequence review without edits and
was closed. It recommended the outcome-totality countermodel before any
bridge-shape comparison.
