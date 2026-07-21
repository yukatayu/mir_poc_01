# WRK-0011 final-store directness evidence (R-2341)

- Date: 2026-07-22 08:48 JST
- Author / agent: Codex, with planner and focused reviewer consultation.
- Scope: Execute the committed L3 assertion-provenance audit and retain only its literal matrix.
- Decision levels touched: L3 evidence only; no L0/L1 decision, Canon theory, OBL, Gate, Phase, or implementation change.

## Objective

Run WRK-0011's registered existing-lane command in an authoritative clean disposable worktree and retain only the named source-route versus direct-evaluator assertion distinction.

## Scope and assumptions

The result is limited to the four named e21/e22 source-route test bodies and two named direct fixture/evaluator test bodies at the pinned revision. Existing unmodified code/tests are execution and inspection machinery; retained LAB evidence remains confined to `plan/` and `samples/current-l2/`. No assertion-provenance result implies a semantic or test-coverage judgment.

## Start state / dirty state

Started from clean pushed `main` at `fa130a499cecca20c625663e4ad20872ef192d67`, whose only difference from the pinned source base `6297b9e6d60b8d4f02bd2efa744beb15648d9e53` is WRK-0011 operational metadata.

## Documents consulted

`mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014, `architecture/02-boundary-contracts.md`, `theory/11-metatheory-ledger.md`, `working/README.md`, WRK-0008 through WRK-0011, `plan/wrk-0008-obl027-formal-hook-attribution.md`, `plan/169-wrk0010-static-decision-attribution-selection.md`, the registered source/test bodies, `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and the report template were consulted under their source hierarchy.

## Actions taken

1. Re-read the working-annex retention rule and WRK-0008 precedent after Oracle/reviewer boundary concerns.
2. Confirmed that the evidence commit changes only a `plan/` artifact, its index entry, and this direct report; no source, test, helper, schema, fixture, or sample changes are retained.
3. Created a clean detached worktree at the committed registration and ran the exact registered `git grep`, six focused tests, and 23-command regression there.
4. Retained the literal matrix without interpreting store values, cut behavior, coverage adequacy, or defect status.

## Files changed

- `plan/wrk-0011-current-l2-final-store-directness.md`
- `plan/00-index.md`
- `docs/reports/2341-wrk0011-final-store-directness-evidence.md`

## Commands run

- `df -h . /tmp`, `free -h`, and disposable-worktree/artifact size checks.
- `git worktree add --detach` at `fa130a49`, then the exact WRK-0011 registered command sequence in that clean worktree.
- Pinned `git grep` across the three registered source-route test files.
- The six named exact Cargo test filters and `python3 scripts/current_l2_source_sample_regression.py regression` with a unique `/tmp` artifact root.
- Focused validation, evidence review, commit/push, and final checks remain pending at report write.

## Evidence / outputs / test results

The pinned source-route test-file search contained no `final_place_store` reference. Each of the six named focused tests passed. The source-sample regression passed all 23 commands. The resulting matrix distinguishes absent exact source-derived `RunReport` final-store assertions in the four named source-route bodies from present exact `evaluator.state.place_store` assertions in the two named direct-evaluator bodies. The authoritative worktree was `/tmp/mirrorea-wrk0011-clean-20260722084732-2363026`; its 1.2 GiB build worktree and the 384 KiB regression artifacts are disposable and uncommitted.

## What changed in understanding

The active named source-route tests establish route execution, selected structural/static checks, terminal outcome, and trace, but do not directly bind the route report's exact final store. The verification ladder's hook identity is separately built from fixture `run_bundle`, and the direct fixture/evaluator lane binds exact evaluator-state stores. This is an assertion-location distinction only.

## Open questions

Whether a different existing source route has a direct final-store assertion is outside this registered body set. Whether either expected store is semantically appropriate, whether source-route coverage should change, and how any future observation should be carried remain unresolved and outside WRK-0011.

## Suggested next prompt

Manifest the scoped WRK-0011 result, synchronize the LAB status snapshots, and resume candidate triage without adding a final-store carrier, test, or repair.

## Plan update status

`plan/` 更新済み: the retained assertion matrix, exact command/result boundary, and reopen condition are recorded and indexed.

## Documentation.md update status

`Documentation.md` 更新不要: reader-facing status is synchronized only with the next manifest package.

## docs/project-status.md update status

更新不要: no Gate, Phase, workflow, or user-facing capability has changed in this evidence-only package.

## progress.md update status

`progress.md` 更新不要: the next manifest package will record the scoped result in the current LAB snapshot.

## tasks.md update status

`tasks.md` 更新不要: the task map is synchronized when the working record manifests this evidence.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, or dashboard classification changed.

## Reviewer findings and follow-up

Focused reviewer `Zeno` correctly required an authoritative clean-worktree run and exact named-body qualifiers; both are incorporated. It raised a retained-root concern. Planner re-read the working annex, ADR-0014, validator behavior, and WRK-0008 precedent and concluded that unmodified pinned code/tests are permitted execution/inspection machinery while retained LAB inputs/artifacts and evidence-commit deltas stay in declared roots. The matrix follows that narrower reading. Oracle advisory initially warned that an undeclared assertion source would be a stop; this package does not retain or modify that source, and the resulting claim is limited to its literal pinned bodies. A follow-up wrapper could not continue the temporary browser session because its saved session lacked a conversation URL; no duplicate Oracle request was made.

## Skipped validations and reasons

No helper/schema/runner implementation or new test was made because the registered stop line forbids repair. The full documentation unit suite is not used as a completion signal because its outer wrapper detaches; focused documentation/source-hierarchy checks and `make check` will be run after the evidence commit.

## Commit / push status

Pending at report write. This package will use `git commit --no-gpg-sign`, push to `origin/main`, verify clean tracking, then proceed to the separate manifest package.

## Sub-agent session close status

Planner `Raman` completed the governance adjudication and will be closed after final validation. Reviewer `Zeno` completed read-only review and is closed. No sub-agent edited files.
