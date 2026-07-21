# Report 2302 - WRK-0003 projection-extensionality countermodel evidence

- Date: 2026-07-21 20:06 JST
- Author / agent: Codex
- Scope: Create and validate the pre-registered LAB-only Lean countermodel before manifesting its evidence commit in WRK-0003.
- Decision levels touched: L3 evidence only. No L0/L1/L2, OBL status, theory ledger, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Test the pre-registered claim that per-result total/unique projections and
native component equality can still coexist with two distinct successful
`Result` values under `OBL021StatementDraft`.

## Scope and assumptions

Canon remains authoritative. The countermodel directly imports the existing
LAB statement draft and changes only the permitted `plan` and `samples/lean`
evidence lanes plus this operational report. It does not select an
extensionality law, a direct Result relation, final equality, or a diagnostic
contract.

## Start state / dirty state

Started from pushed, clean `main` at `e02f8faa`, where WRK-0003 had already
been committed. The registered red check then confirmed that the target source
was absent. Ignored local Cargo output and a small external temporary Lean
`.olean` artifact exist outside the tracked evidence set.

## Documents consulted

- `mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md`,
  `mirrorea_canon/theory/01-mircore-v0.md`, `03-elaboration.md`,
  `10-diagnostics.md`, and `11-metatheory-ledger.md`.
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` and
  the WRK-0002 countermodel.
- `plan/126`, `plan/143`, `plan/158`, `plan/159`, and the new WRK-0003 LAB
  plan.
- `AGENTS.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Actions taken

- Added the reproducible LAB plan and a direct-import Lean countermodel.
- Defined a two-constructor `Result`; every compared output type is `Unit`,
  every projection is true, and every component equivalence is native equality.
- Proved total/unique projection witnesses, equality component relations, the
  statement draft, and two distinct successes for the same unit input.
- Built the imported draft into `/tmp/wrk-0003-lean-artifact` and compiled the
  countermodel through `LEAN_PATH`, retaining no generated `.olean` in the
  repository.
- Corrected a construction-only Lean issue: the local minimal import surface
  does not provide an `ExistsUnique` identifier, so the registered total/unique
  condition is spelled out with `Exists` plus an explicit uniqueness clause.

## Files changed

- `plan/wrk-0003-projection-extensionality-countermodel.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.md`
- `docs/reports/2302-wrk-0003-countermodel-evidence.md`

## Commands run

- Registered red check: `test ! -e samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean`.
- `lean --version`.
- `lean -o /tmp/wrk-0003-lean-artifact/.../ElabDeterminismStatementDraft.olean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
- `LEAN_PATH=/tmp/wrk-0003-lean-artifact lean samples/lean/lab-statements/obl021/ElabDeterminismProjectionExtensionalityCountermodel.lean`.
- Registered Python source audit for required theorem names and forbidden
  placeholder/escape tokens.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- `python3 scripts/validate_docs.py`.
- `(cd mirrorea_canon && python3 meta/build-index.py --check)`.

## Evidence / outputs / test results

- The pre-source red check succeeded: the target countermodel did not exist.
- Lean 4.29.1 compiled both the existing statement draft and the new
  countermodel when the imported draft's `.olean` was supplied from the
  external temporary path.
- `projection_predicates_are_total_and_unique` proves that all nine compared
  projections have one witness for each Result and that each witness is unique.
- `component_equivalences_are_equality` proves each compared component
  equivalence is native equality in the concrete model.
- `statement_draft_holds` proves `OBL021StatementDraft`; at the same time,
  `distinct_results_can_elaborate` proves two unequal `DistinctResult` values
  successfully elaborate for the same input.
- The static audit found all four registered theorem names and no `sorry`,
  `admit`, `axiom`, `unsafe`, `partial`, or `implemented_by` token. The Lean
  sync test passed all 21 tests; documentation validation and Canon index check
  also passed.

## What changed in understanding

The empty-projection issue from WRK-0002 is not the sole reason the LAB draft
fails to entail Result identity. Even when every named component is present,
unique, and compared by equality, the draft permits a Result with unprojected
distinction. A future statement needs an unselected bridge: either a joint
extensionality law for the chosen output tuple or a direct Result relation.

## Open questions

- Which bridge best matches Canon's tuple/function intent remains unselected.
- Whether a joint extensionality law should be modeled as native equality,
  a named abstract relation, or a separate adequacy premise remains unresolved.
- Final Result equality, diagnostic equivalence, Diagnostic ABI, input
  identity, OBL-021 proof/discharge, and all Canon lifecycle decisions remain
  owner controlled.

## Suggested next prompt

Manifest this source evidence commit in WRK-0003, then compare a direct
abstract Result relation against a joint-extensionality premise without
choosing either as Canon.

## Plan update status

更新済み: `plan/wrk-0003-projection-extensionality-countermodel.md` records
the model, success/stop conditions, and source-evidence/manifest split.

## Documentation.md update status

更新不要: the reader route does not need an unmanifested L3 source-artifact
detail.

## docs/project-status.md update status

更新不要: WRK-0003 remains correctly shown as pre-registered until the source
evidence commit is append-only manifested in its record.

## progress.md update status

更新不要: current snapshot status remains an unmanifested L3 evidence package
until the following manifest commit.

## tasks.md update status

更新不要: the same WRK-0003 evidence package remains current; its completion
signal is the following append-only WRK manifest.

## samples_progress.md update status

更新不要: this is a LAB theorem countermodel, not an active runnable sample or
sample-dashboard workflow change.

## Reviewer findings and follow-up

The preceding temporary Oracle review and read-only Canon audit motivated the
question, but neither is used as proof. The local Lean countermodel is the sole
outcome evidence. A later review may challenge the claim boundary after the
evidence is manifested.

## Skipped validations and reasons

No broad Cargo test, runtime sample suite, clean-worktree validation, or
external source-first pipeline run was executed. They do not test this
Lean-only countermodel, the configured external workdir remains unmounted, and
local Cargo output is nontrivial. No validation required by WRK-0003 was
skipped.

## Commit / push status

This source-evidence package is committed and pushed at its task closeout,
before any separate WRK manifest commit. The manifest must cite this full
evidence commit rather than itself.

## Sub-agent session close status

The read-only Canon-audit sub-agent was already closed after the preceding
pre-registration package. No sub-agent is active for this source-evidence
package.
