# WRK-0013 retained-reproduction registration (R-2351)

- Date: 2026-07-22 13:41 JST
- Author / agent: Codex
- Scope: Register one forward L3 retained-reproduction question; do not execute it.
- Decision levels touched: L3 working-record registration only; no L0/L1 decision,
  Canon theory, OBL, Gate, Phase, implementation, or workflow-status change.

## Objective

Commit WRK-0013 before any new outcome command, with exact input snapshots,
expected classifications, retention artifact path, and freeze conditions.

## Scope and assumptions

WRK-0012 remains frozen and its R-2347 run remains historical metadata. The
two sidecars owned by `2242901a44d3feb7708f82ff535d91bff4fbe143` are inputs
only. This package creates no result memo, plan-index entry, execution output,
or evidence commit.

## Start state / dirty state

`main` and `origin/main` were clean and equal at
`d8bfbc38bab7a20cfd0574b9f987319944998a12`. The source-selection record had
chosen the unnumbered retention route, but WRK-0013 did not yet exist.

## Documents consulted

Canon README/MAP, ADR-0014, boundary contracts, theory ledger, working README,
frozen WRK-0012, `plan/00-index.md`, the WRK-0013 selection, prior WRK-0011
registration/evidence/manifest commits, validators, `Documentation.md`,
`docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
were consulted. Canon remains normative.

## Actions taken

Registered WRK-0013 as a fresh retained reproduction. It pins the two old
sidecars, declares the new exact result-memo/index route and full falsifier,
and requires execution only after this registration is committed and pushed.
MAP, Canon index, and current LAB snapshots now distinguish registration from
fresh output. No outcome command or plan change ran.

## Files changed

- `mirrorea_canon/working/WRK-0013-pcomp03-retention-reproduction.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2351-wrk0013-retained-reproduction-registration.md`

## Commands run

Read-only Git/status/hash checks, source and validator inspection, Canon index
regeneration, focused diff review, and committed-state validation are run for
this registration. The registered P-COMP command is intentionally deferred.

## Evidence / outputs / test results

The authority, selection, plan-index, and two sidecar SHA-256 values were
recomputed from the pinned commits. Registration alone produces no outcome,
evidence artifact, validation of the sidecars, runtime claim, or workflow
readiness change.

## What changed in understanding

The next valid question is now mechanically pre-registered: it tests fresh
provenance and retention through an established unnumbered plan convention,
not whether the old carrier observation can be reused or repaired.

## Open questions

Whether the exact fresh run reproduces both classifications and whether its
result memo/index delta passes unchanged validators remain open. Any failure
freezes WRK-0013; numbered-plan policy remains a separate escalation.

## Suggested next prompt

Validate and push this registration in a clean detached worktree, synchronize
the separate reader guide, then run only WRK-0013's registered command and
attempt its exact evidence delta.

## Plan update status

`plan/` 更新不要: registration deliberately creates neither the future result
memo nor its `plan/00-index.md` entry.

## Documentation.md update status

`Documentation.md` 更新不要: it is not permitted WRK registration metadata.
A separate status-only package will replace its selected-but-unregistered
wording before execution.

## docs/project-status.md update status

更新済み: current LAB view now states that WRK-0013 is registered but has no
fresh execution, result memo, or evidence commit.

## progress.md update status

`progress.md` 更新済み: milestone, macro phase, feature boundary, and dated log
now require the pushed registered command rather than another preregistration.

## tasks.md update status

`tasks.md` 更新済み: registration is closed and task 33 is the fresh execution
and retention check.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or workflow classification changed.

## Reviewer findings and follow-up

The preceding planner and temporary Oracle advisory both required this exact
scope: a forward reproduction, old sidecars as inputs only, no reuse of
R-2347 output, fresh execution after registration, and freezing rather than
repair if the unnumbered retention route fails. No new sub-agent changed files.

## Skipped validations and reasons

No P-COMP execution, result memo/index edit, sidecar modification, validator
change, helper/schema/runtime/CLI change, numbered-plan admission, or public
workflow test was attempted because registration must precede all outcome work.

## Commit / push status

Pending focused validation, `git commit --no-gpg-sign`, push, and remote-head
verification.

## Sub-agent session close status

Planner Euler completed the prior selection package and is closed. No sub-agent
was opened for this registration package.
