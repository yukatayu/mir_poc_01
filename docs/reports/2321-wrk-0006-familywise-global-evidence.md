# Report 2321 - WRK-0006 familywise/global source evidence

- Date: 2026-07-22 01:20 JST
- Author / agent: Codex
- Scope: Create and validate the pre-registered LAB-only OBL-020 boundary source before its separate working-record manifest.
- Decision levels touched: L3 evidence only. No L0/L1/L2, OBL status, theory ledger, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Test the registered global/familywise relationship in the existing abstract
OBL-020 Lean vocabulary, including only an experiment-local coverage-conditioned
converse and a non-vacuous finite separation model.

## Scope and assumptions

Canon remains authoritative. The source directly imports the existing LAB
statement draft and changes only WRK-0006's declared `plan` and `samples/lean`
evidence lanes plus this direct report. The finite model's family, classification
relation, and coverage premise are local to this experiment. They do not select
MirCore step families, a coverage policy, a theorem interface, or a proof
architecture.

## Start state / dirty state

Started from pushed `main` at `7e41710d`, where WRK-0006 was committed as L3
`not-promoted` pre-registration. The registered red check confirmed that the
target source did not exist. Four prepared LAB snapshot edits remain uncommitted
for the following manifest package and are not evidence in this source package.

## Documents consulted

- `mirrorea_canon/working/WRK-0006-obl020-familywise-global-boundary.md`,
  ADR-0014, `working/README.md`, and `theory/01-mircore-v0.md` /
  `11-metatheory-ledger.md`.
- `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` and its README.
- `plan/156`, `plan/158`, `plan/161`, and Report 2320.
- `AGENTS.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Actions taken

1. Ran the registered pre-source absence check and compiled the existing draft.
2. Added two direct proposition-level implications in the imported abstract
   vocabulary.
3. Added one finite experiment-local model that has a canonical family and a
   classified preserving step while leaving one actual non-preserving step
   unclassified.
4. Kept the result source-only: no Canon record, current snapshot, helper,
   script, test infrastructure, or existing draft changed in this package.
5. Confirmed that an import-bearing Lean source needs the established external
   `.olean` plus `LEAN_PATH` runner: bare `lean` has no project search path and
   fails identically for the pre-existing OBL-021 imported evidence source.

## Files changed

- `plan/wrk-0006-obl020-familywise-global-boundary.md`
- `samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean`
- `samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.md`
- `docs/reports/2321-wrk-0006-familywise-global-evidence.md`

## Commands run

- Registered red check: `test ! -e samples/lean/lab-statements/obl020/FamilywiseGlobalBoundary.lean`.
- `lean --version` and direct compile of the existing statement draft.
- Fresh external `.olean` compile of the imported draft, then
  `LEAN_PATH=<external-workdir> lean` for the new boundary source.
- Required-name and forbidden-token source audits from WRK-0006.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- `python3 scripts/validate_docs.py`, Canon index check, and focused diff checks.

## Evidence / outputs / test results

The registered pre-source red check passed. Bare `lean` of the new imported
source failed before theorem elaboration because the standalone Lean process has
no repository module search path; the same command fails for the pre-existing
WRK-0005 imported source. A fresh external `.olean` for
`StepWFStatementDraft.lean`, supplied through `LEAN_PATH`, compiled the new
source under Lean 4.29.1 with no warnings.

`global_implies_familywise` derives the wrapper by ignoring its two additional
antecedents. `coverage_and_familywise_imply_global` uses only the explicitly
written coverage premise. The finite model proves that a canonical family and a
classified preserving step are present, while an unclassified actual step can
still take `good` to `bad`; it proves familywise preservation and disproves the
aggregate draft. Required-name and forbidden-token audits passed. The Lean
synchronization suite passed 21 tests, documentation validation found 1,475
numbered reports, and the Canon index check found 85 indexed files.

## What changed in understanding

The aggregate and familywise forms have a precise abstract relationship: global
preservation entails every family-qualified instance; a familywise proof has no
route back to the aggregate form without a stated bridge covering actual steps.
The experiment-local coverage premise is one sufficient bridge, not a selected
Canon requirement. The finite model prevents treating the separation as merely
an empty-family or no-step artifact. None of this binds the abstract predicates
to MirCore or proves OBL-020.

## Open questions

1. Whether any later Canon proof should use family-local reasoning remains
   unselected.
2. Concrete rule coverage, transition preservation premises, and all 65
   selected-rule adequacy cells remain open.
3. The final OBL-020 theorem interface and its placement remain owner-reserved.

## Suggested next prompt

Commit and push this source as the sole evidence package, then create a separate
append-only WRK manifest that pins its commit and artifact hashes. Record the
established external-`.olean` runner as a method clarification without changing
the pre-registered question or any Canon boundary.

## Plan update status

更新済み: `plan/wrk-0006-obl020-familywise-global-boundary.md` records the
experiment boundary, method, stop condition, and manifest split.

## Documentation.md update status

更新不要: the top-level reader route does not need an unmanifested L3 source
artifact detail.

## docs/project-status.md update status

更新不要: WRK-0006 remains correctly shown as pre-registered until the source
evidence is append-only manifested in its record.

## progress.md update status

更新不要: the current snapshot remains an unmanifested L3 source-evidence
package until the following manifest commit.

## tasks.md update status

更新不要: the same WRK-0006 evidence package remains current; the following
append-only WRK manifest will close it.

## samples_progress.md update status

更新不要: this is a LAB theorem experiment, not an active runnable sample or
sample-dashboard workflow change.

## Reviewer findings and follow-up

The pre-registration records two temporary Oracle reviews and one independent
read-only review. Their advisory findings constrain the source to an abstract
boundary: one recommends deferral for low impact, while the other identifies
this exact experiment as ADR-0014 standing-eligible. The local Lean result is
the only source evidence; no new review is needed before the L3 manifest.

## Skipped validations and reasons

No broad Cargo suite, runtime sample suite, clean-worktree authoritative
validation, or distributed execution applies to this Lean-only source. The
direct bare-`lean` command is not a valid import runner in this repository; its
failure is retained above, and the existing external-`.olean` runner was used
instead. No theorem/source audit is skipped.

## Commit / push status

Pending at report write. This source-evidence package will be committed with
`--no-gpg-sign`, validated, and pushed before the separate WRK manifest cites
its full evidence commit.

## Sub-agent session close status

No sub-agent is active for this source-evidence package. The completed advisory
Oracle sessions and independent reviewer remain closed.
