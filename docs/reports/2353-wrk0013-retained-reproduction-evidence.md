# WRK-0013 retained-reproduction evidence (R-2353)

- Date: 2026-07-22 13:53 JST
- Author / agent: Codex
- Scope: Run the pushed registered W13 command once in a fresh checkout and
  retain only its declared unnumbered evidence delta.
- Decision levels touched: L3 LAB evidence only; no Canon theory, OBL, Gate,
  Phase, implementation, conformance, or workflow-status change.

## Objective

Reproduce W13's fixed positive/negative classifications from fresh output and
test whether the exact unnumbered plan retention path can be prepared without
changing its excluded machinery.

## Scope and assumptions

The W13 registration is pushed at
`3043140e6111de902826031ed520c3371993b8ad`; its reader-guide-only successor
is `ac8e1f3b90e5d33baf025a66b415ce09fa103713`. W12 stays frozen, and its old
R-2347 output is not used as W13 evidence. The two sidecars are immutable
inputs only. This package changes exactly the memo, its plan index entry, and
this direct report.

## Start state / dirty state

`main` and `origin/main` were clean and equal at
`ac8e1f3b90e5d33baf025a66b415ce09fa103713`. No W13 result memo, result-index
entry, or evidence commit existed. A clean detached checkout was created after
the registration and reader-guide synchronization were pushed.

## Documents consulted

Canon README/MAP, ADR-0014, boundary contracts, theory ledger, working README,
frozen WRK-0012, registered WRK-0013, W13 selection/R-2351/R-2352,
`plan/00-index.md`, the two pinned sidecars, validators, `Documentation.md`,
`docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
were consulted. Canon remains normative.

## Actions taken

Ran the exact `Commands:` value from W13 under `bash -x` in a fresh detached
checkout, with the existing disposable Cargo target cache only as build-output
location. Rechecked both pinned input digests, captured fresh JSON output,
prepared the declared unnumbered plan memo and index entry, and changed no
sidecar, validator, helper, schema, script, CI/Make surface, Rust crate,
runtime, CLI, interface, or unrelated plan file.

## Files changed

- `plan/wrk-0013-pcomp03-retained-reproduction.md`
- `plan/00-index.md`
- `docs/reports/2353-wrk0013-retained-reproduction-evidence.md`

## Commands run

Created a clean detached worktree at `ac8e1f3b`, extracted the committed W13
`Commands:` value unchanged, and ran it under `bash -x`. It executed the two
input digest checks; positive check/run-local; negative check/run-local with
the registered expected exit 2; and registered JSON assertions. Read-only JSON
inspection and SHA-256 calculation followed. Retention validation and commit
remain pending at report write.

## Evidence / outputs / test results

The exact command returned exit 0. Positive input digest matched; `check`
returned `accepted`; `run-local` recorded `mir_computation_claimed: true` and
one `sum_to` history item with `Int(5)` to `Int(15)`. Negative input digest
matched; `check` returned `accepted`; `run-local` exited 2 with `status: error`,
`command: run-local`, `diagnostic_code: MirCompute`, and
`UnboundVariable: unbound variable \`y\``. Fresh output hashes and the exact
retention boundary are in the new plan memo. This is fresh W13 output, not
reused W12/R-2347 output.

## What changed in understanding

The fresh execution portion of W13 reproduces both registered classifications.
The remaining question is now only whether this exact memo/index/report delta
passes unchanged validators and can be append-only manifested; a run pass alone
is not W13 completion or a broader carrier claim.

## Open questions

Whether the exact retention delta passes unchanged documentation and source
hierarchy validation remains pending until its focused validation and commit.
After that, W13 still requires a separate append-only manifest to attach this
commit and memo digest. Any retention failure freezes W13.

## Suggested next prompt

Validate the three-file evidence delta, commit and push it, then append its
exact commit and memo digest to W13 in a separate manifest/snapshot package.

## Plan update status

`plan/` 更新済み: the fresh reproduction, output hashes, exact retention delta,
non-claims, and reopen condition are retained in the declared unnumbered memo
and its `plan/00-index.md` entry.

## Documentation.md update status

`Documentation.md` 更新不要: reader-facing results wait for the later W13
append-only manifest; this evidence package is intentionally exact and narrow.

## docs/project-status.md update status

更新不要: current status is synchronized only when W13 manifests the evidence
commit, not while this unmanifested evidence delta is staged.

## progress.md update status

`progress.md` 更新不要: macro/feature status waits for the manifested W13
result; this package does not change Gates, Phases, or workflow readiness.

## tasks.md update status

`tasks.md` 更新不要: task 33 remains the registered run plus retention route
until this evidence is committed and manifested.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, command, debug surface, or
workflow classification changed.

## Reviewer findings and follow-up

The prior planner and temporary Oracle advisory required an independent fresh
run, old sidecars as inputs only, a three-file evidence delta, and a later
manifest. This package follows those guardrails. A focused final review should
inspect this exact delta before the evidence commit; no sub-agent changed files.

## Skipped validations and reasons

No helper/schema/runtime/CLI modification, direct textual `.mir` command,
broader matrix, numbered-plan policy change, or public workflow test was run
because each exceeds W13's registered question. Full retention validation,
commit, push, and manifest are deliberately separate next steps.

## Commit / push status

Pending focused retention validation, `git commit --no-gpg-sign`, push, and
remote-head verification.

## Sub-agent session close status

No new sub-agent was opened. The prior selection planner is closed; focused
evidence-delta review is pending.
