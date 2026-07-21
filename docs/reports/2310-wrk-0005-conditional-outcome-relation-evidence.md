# Report 2310 - WRK-0005 conditional outcome-relation evidence

- Date: 2026-07-21 20:43 JST
- Author / agent: Codex
- Scope: Create and validate the pre-registered LAB-only conditional relation before manifesting its evidence commit in WRK-0005.
- Decision levels touched: L3 evidence only. No L0/L1/L2, OBL status, theory ledger, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Test whether the existing `OBL021StatementDraft`, plus an explicit
experiment-local outcome-existence premise, relates every two tagged outcomes
without choosing final Result equality, Diagnostic equality, relation laws, or
quotient semantics.

## Scope and assumptions

Canon remains authoritative. The source directly imports the existing LAB
statement draft and changes only the permitted `plan` and `samples/lean`
evidence lanes plus this operational report. `Outcome`, `OutcomeOf`,
`SameOutcome`, and `OutcomeTotal` are local to this experiment. The result does
not decide whether outcome totality belongs in OBL-021, OBL-003, a different
Canon obligation, or any public interface.

## Start state / dirty state

Started from pushed `main` at `aa9dd804`, where WRK-0005 was committed as
L3 `not-promoted` pre-registration. The registered red check confirmed that
the target source did not exist. Ignored local Cargo output and prior external
temporary Lean artifacts were outside the tracked evidence set.

## Documents consulted

- `mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md`,
  ADR-0014, `theory/03-elaboration.md`, `theory/10-diagnostics.md`, and
  `theory/11-metatheory-ledger.md`.
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
- WRK-0002 through WRK-0004, `plan/143`, `plan/158`, and `plan/159`.
- `AGENTS.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Actions taken

- Added a reproducible LAB plan, an experiment-local outcome carrier, and a
  direct-import Lean theorem.
- Defined `SameOutcome` by delegating success pairs to `SameElabResult`,
  rejection pairs to `SameDiagnostic`, and mixed pairs to the draft's existing
  incompatibility clause.
- Kept existence outside the draft: `OutcomeTotal` is an explicit hypothesis
  from which the theorem returns one witness and the all-pairs relation.
- Compiled the imported statement draft into a new external temporary workdir
  and checked the new source through `LEAN_PATH`.

## Files changed

- `plan/wrk-0005-conditional-outcome-relation.md`
- `samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean`
- `samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.md`
- `docs/reports/2310-wrk-0005-conditional-outcome-relation-evidence.md`

## Commands run

- Registered red check: `test ! -e samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean`.
- `lean --version`.
- `lean -o /tmp/wrk-0005-lean-artifact.*/.../ElabDeterminismStatementDraft.olean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
- `LEAN_PATH=/tmp/wrk-0005-lean-artifact.* lean samples/lean/lab-statements/obl021/ElabDeterminismConditionalOutcomeRelation.lean`.
- Registered Python source audit for required names and forbidden
  placeholder/escape tokens.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- `python3 scripts/validate_docs.py`.
- `(cd mirrorea_canon && python3 meta/build-index.py --check)`.

## Evidence / outputs / test results

- The pre-source red check succeeded: the target conditional-relation source
  did not exist before the registered evidence run.
- Lean 4.29.1 compiled both the existing statement draft and the new source
  when the imported draft's `.olean` was supplied from the external temporary
  path.
- `statement_draft_implies_outcomes_related` covers all four tagged pairs. The
  success/success and reject/reject branches use the draft's first two clauses;
  the two mixed branches use its explicit incompatibility clause.
- `outcome_totality_and_draft_imply_unique_relation` produces a witness only
  from the stated `OutcomeTotal` premise, then retains the derived all-pairs
  relation. It does not prove existence from the draft.
- The static audit found every registered required name and no `sorry`,
  `admit`, `axiom`, `unsafe`, `partial`, or `implemented_by` token. The Lean
  synchronization test passed all 21 tests; documentation validation and the
  Canon index check also passed.

## What changed in understanding

The current LAB draft plus explicit existence yields a small abstract
all-outcome relation. This is sufficient as a conditional bridge-free reading
of the three existing branches, but it is still not function-like determinism
under native equality: the relation has no established laws, no quotient, and
no Canon placement. The preceding no-outcome countermodel remains essential,
because this theorem does not remove the existence gap.

## Open questions

- The exact Canon home, form, and proof obligations of an eventual
  outcome-totality statement remain unresolved.
- Whether an abstract Result relation should be given reflexive, symmetric,
  transitive, or observational-adequacy conditions remains deliberately open.
- Final equality, Diagnostic ABI, proof/discharge, and all Canon lifecycle
  decisions remain owner controlled.

## Suggested next prompt

Manifest this source evidence in WRK-0005, then close the current OBL-021
statement-shape research checkpoint without promoting the conditional relation
to Canon or a public interface.

## Plan update status

更新済み: `plan/wrk-0005-conditional-outcome-relation.md` records the
experiment, its boundaries, reproducible method, and evidence/manifest split.

## Documentation.md update status

更新不要: the reader route does not need an unmanifested L3 source-artifact
detail.

## docs/project-status.md update status

更新不要: WRK-0005 remains correctly shown as pre-registered until the source
evidence commit is append-only manifested in its record.

## progress.md update status

更新不要: the current snapshot status remains an unmanifested L3 evidence
package until the following manifest commit.

## tasks.md update status

更新不要: the same WRK-0005 evidence package remains current; its completion
signal is the following append-only WRK manifest.

## samples_progress.md update status

更新不要: this is a LAB theorem experiment, not an active runnable sample or
sample-dashboard workflow change.

## Reviewer findings and follow-up

Prior Oracle and planner conclusions motivated the explicit-totality boundary,
but neither substitutes for proof. The local Lean theorem is the sole positive
evidence. A future review should assess a candidate relation law only after a
separate pre-registration; no such law was introduced here.

## Skipped validations and reasons

No broad Cargo test, runtime sample suite, clean-worktree validation, or
external source-first pipeline run was executed. They do not test this
Lean-only conditional lemma, the configured external workdir remains unmounted,
and local Cargo output is nontrivial. No validation required by WRK-0005 was
skipped.

## Commit / push status

This source-evidence package is committed and pushed at its task closeout,
before any separate WRK manifest commit. The manifest must cite this full
evidence commit rather than itself.

## Sub-agent session close status

No sub-agent is active for this source-evidence package.
