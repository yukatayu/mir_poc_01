# Report 2299 - WRK-0002 OBL-021 countermodel evidence

- Date: 2026-07-21 19:54 JST
- Author / agent: Codex
- Scope: Create and validate the pre-registered LAB-only Lean countermodel before manifesting its evidence commit in WRK-0002.
- Decision levels touched: L3 evidence only. No L0/L1/L2, OBL status, theory ledger, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Test the narrowly pre-registered claim that `OBL021StatementDraft` can hold for
two distinct successful results when its result-projection predicates are all
empty.

## Scope and assumptions

Canon remains authoritative. The countermodel directly imports the existing
LAB statement draft and changes only the permitted `plan` and `samples/lean`
evidence lanes plus this operational report. It is not a choice of final
result equality, diagnostic equivalence, or projection-totality law.

## Start state / dirty state

Started from pushed, clean `main` at `7745a44e`, where WRK-0002 had already
been committed. The registered red check then confirmed that the target source
was absent. Ignored local Cargo output and a small external temporary Lean
`.olean` artifact exist outside the tracked evidence set.

## Documents consulted

- `mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md`,
  `mirrorea_canon/theory/03-elaboration.md`, `10-diagnostics.md`, and
  `11-metatheory-ledger.md`.
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` and
  its companion explanation.
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md`,
  `plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`,
  `plan/158-standing-bounded-autonomy.md`, and
  `plan/159-wrk-evidence-commit-integrity-recut.md`.
- `AGENTS.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- The local Oracle operating manual and `.docs/oracle-chatgpt-pro-operations.md`.

## Actions taken

- Added a reproducible LAB plan and index entry for the registered method.
- Added the countermodel and companion explanation under the existing OBL-021
  Lean statement-draft directory.
- Built the imported draft into `/tmp/wrk-0002-lean-artifact`, then compiled
  the countermodel through `LEAN_PATH` so no generated `.olean` entered the
  repository.
- Corrected three Lean surface issues encountered during construction: import
  commands must precede a module doc comment, proposition syntax must use
  `¬`/`≠` rather than Bool syntax, and the distinct-constructor proof must not
  rely on typeclass reduction through opaque `V.Result`.
- Started one temporary Oracle advisory review of the claim boundary. Its
  answer is pending and is not relied on by this source-evidence commit.

## Files changed

- `plan/00-index.md`
- `plan/wrk-0002-projection-vacuity-countermodel.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.md`
- `docs/reports/2299-wrk-0002-countermodel-evidence.md`

## Commands run

- Registered red check: `test ! -e samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean`.
- `lean --version`.
- `lean -o /tmp/wrk-0002-lean-artifact/.../ElabDeterminismStatementDraft.olean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
- `LEAN_PATH=/tmp/wrk-0002-lean-artifact lean samples/lean/lab-statements/obl021/ElabDeterminismProjectionVacuityCountermodel.lean`.
- Registered Python source audit for required theorem names and forbidden
  placeholder/escape tokens.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- `python3 scripts/validate_docs.py`.
- `(cd mirrorea_canon && python3 meta/build-index.py --check)`.
- One `ask-chatgpt-pro-temp` advisory review with the WRK record, draft,
  countermodel, and LAB plan attached.

## Evidence / outputs / test results

- The pre-source red check succeeded: the target countermodel did not exist.
- Lean 4.29.1 compiled both the existing statement draft and the new
  countermodel when the imported draft's `.olean` was supplied from the
  external temporary path.
- `projection_predicates_are_empty` proves that all nine compared result
  projections have no witness in the concrete model.
- `statement_draft_holds` proves the existing `OBL021StatementDraft` in that
  model; `distinct_results_can_elaborate` proves two unequal `DistinctResult`
  values both elaborate for the same unit input; their conjunction is recorded
  by `statement_draft_allows_distinct_successes`.
- The static audit found all three registered theorem names and no
  `sorry`, `admit`, `axiom`, `unsafe`, `partial`, or `implemented_by` token.
- The Lean sync test passed all 21 tests. Documentation validation completed
  and the Canon index check reported 81 indexed files.

## What changed in understanding

The current LAB statement draft can be satisfied without identifying results
or requiring any projection witness. Therefore it is insufficient on its own
to establish the canonical phrase "elaboration is a function of its inputs" in
the result-identity or projection-non-vacuity sense. The countermodel does not
show which additional premise is best, whether equality should be syntactic or
semantic, or whether the final obligation should use projections at all.

## Open questions

- Oracle's independent boundary review is pending.
- Whether the next bounded candidate should compare the smallest possible
  non-vacuity premise, a direct result relation, or an explicit uniqueness law
  remains a research question; no option is selected here.
- Final result equality, diagnostic equivalence, projection-totality,
  projection uniqueness, and all OBL-021 ledger/status decisions remain
  unresolved and owner/canon controlled.

## Suggested next prompt

Manifest this source evidence commit in WRK-0002, incorporate the advisory
review only if it survives local comparison, then choose the next L3 candidate
from the resulting explicit premise gap.

## Plan update status

更新済み: `plan/wrk-0002-projection-vacuity-countermodel.md` records the
method, success/stop conditions, and evidence-commit/manifest split.

## Documentation.md update status

更新不要: the concise reader route does not need a new L3 source artifact
detail before WRK manifestation.

## docs/project-status.md update status

更新不要: WRK-0002 remains correctly shown as pre-registered until the source
evidence commit is append-only manifested in its record.

## progress.md update status

更新不要: current snapshot status remains an unmanifested L3 evidence package
until the following manifest commit.

## tasks.md update status

更新不要: the same OBL-021 evidence package remains current; its completion
signal is the following append-only WRK manifest.

## samples_progress.md update status

更新不要: this is a LAB theorem countermodel, not an active runnable sample or
sample-dashboard workflow change.

## Reviewer findings and follow-up

The temporary Oracle review was launched after local Lean validation to
challenge interpretation, hidden assumptions, non-claims, and the smallest
next question. It is still running at this package boundary, so no advisory
finding is treated as evidence or mirrored here. A later manifest/checkpoint
report will record the result or an explicit timeout/failure.

## Skipped validations and reasons

No broad Cargo test, runtime sample suite, clean-worktree validation, or
external source-first pipeline run was executed. They neither validate this
Lean-only countermodel nor fit the current WRK lane, and local Cargo output is
already nontrivial while the configured external workdir is unmounted. No
validation required by WRK-0002 was skipped.

## Commit / push status

This source-evidence package is committed and pushed at its task closeout,
before any separate WRK manifest commit. The manifest must cite this full
evidence commit rather than itself.

## Sub-agent session close status

No sub-agent was used. One temporary Oracle session is active and will be
waited for without starting a duplicate request.
