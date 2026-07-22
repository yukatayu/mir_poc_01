# WRK-0012 direct-carrier execution (R-2347)

## Title and identifier

R-2347 preserves the single pre-registered WRK-0012 execution as historical
metadata after the later retention-boundary falsifier froze the record.

- Date: 2026-07-22 12:47 JST
- Author / agent: Codex, executing the committed WRK-0012 sequence.
- Scope: Run and record only the pre-registered two-sidecar command sequence.
- Decision levels touched: L3 LAB evidence only; no Canon theory, OBL, Gate, Phase, runtime, schema, helper, CLI, or public decision.

## Objective

Execute the exact WRK-0012 command sequence after its source leaves were committed and pushed, then retain a bounded result record without broadening its claims.

## Scope and assumptions

The registered working record is 12e6b458bc6fe82953cab813b71012b5e35bbdae. The executed source state is 2242901a44d3feb7708f82ff535d91bff4fbe143, which adds only the two declared direct-world leaves and its direct report. The authority/LAB snapshot remains 66229addaa1044c4759a2759b5ef41f355f25d11. `/tmp` output is disposable. The pre-registered numbered plan artifact was deliberately not retained after validator admission would have required an excluded script change.

## Start state / dirty state

main and origin/main resolved to 2242901a44d3feb7708f82ff535d91bff4fbe143 with a clean worktree. The two sidecars were committed and pushed, while no outcome JSON or retained result artifact existed.

## Documents consulted

mirrorea_canon/README.md, mirrorea_canon/MAP.md, ADR-0014, working/README.md, WRK-0012, plan/170, Product Alpha computational matrix/helper documentation, the two sidecars, existing Product Alpha session tests, Documentation.md, docs/project-status.md, progress.md, tasks.md, and samples_progress.md were consulted. Canon remains normative; command output is LAB evidence only.

## Actions taken

Read the registered command text directly from WRK-0012 and executed it once with `bash -x` to discover the disposable output directory. The command completed successfully: matrix and check-all accepted their existing expected sets, both selected Rust tests passed, the positive sidecar checked and ran, and the negative sidecar checked then exited 2 with the registered error classification. A draft plan/171 then exposed that numbered-plan admission requires an out-of-scope validator/source-hierarchy maintenance change. The draft was not retained; this report preserves the run as historical metadata for the frozen record.

## Files changed

- docs/reports/2347-wrk0012-direct-carrier-execution.md

## Commands run

The exact `Commands:` value in WRK-0012 ran under `bash -x`. It ran the two computational Python commands, two exact Rust tests, positive `check` and `run-local`, negative `check` and expected-exit-2 `run-local`, then the registered JSON assertions. Read-only JSON inspection and SHA-256 calculation then summarized the generated artifacts.

## Evidence / outputs / test results

The command returned exit 0. Matrix: 15 rows, 7 accepted, 5 expected runtime rejections, 3 expected check rejections, no validation errors. Check-all: all 15 passed, no failures. Positive: check accepted with zero diagnostics and run-local recorded `sum_to`, `Int(5)`, `Int(15)`. Negative: check accepted with zero diagnostics; run-local exited 2 with `status: error`, `command: run-local`, `diagnostic_code: MirCompute`, and message "UnboundVariable: unbound variable `y`". Disposable JSON SHA-256 values are retained below as report history, not an evidence-artifact replacement: matrix `4cb3739d7752374146046df30f30235e62dc55241f0dd3d6fdfde94a88f70347`; check-all `5cbd19b9abd6a3bc0d1802e2278757c7733da8e5a006c8bc3326579e36bec020`; positive check/run `b6c8f9ef48c6ff085dc70bdb0b0bd40247233c8475a0caf219297d2ae30288a3` / `c447a6e65e8c799c25d29774742bf0f00de695c4c22b2124fd7a9b2e629eca02`; negative check/run `54b81849e874254516207902a9ade658e0ec23fb867f9ff6d57c8655519e8537` / `85d91e999b3222a689b6631535a2ff56461f365f058c7b5c2791891c52f9d2a9`.

## What changed in understanding

The existing Product Alpha world-package carrier produced the registered observations for this one fixed accepted request and this one fixed rejected request. The attempt also established a separate operational boundary: the pre-registered numbered result artifact cannot enter the repository without an excluded validator/source-hierarchy change. This narrows neither the language design nor the helper's general classification.

## Open questions

No reliance-worthy conclusion is available from WRK-0012 because the retention falsifier freezes it. No conclusion has been established for direct textual `.mir`, the other P-COMP-03 rows, helper/sidecar equivalence, rejection phase across carriers, general runtime completeness, any repair, or validator policy. Further scope requires a new forward registered question or escalation.

## Suggested next prompt

Freeze WRK-0012 without validator repair, attribute only the already committed two sidecars, and synchronize current snapshots. A later successor may pre-register an admissible retention path; this historic run cannot be retroactively rescued.

## Plan update status

plan/ 更新不要: the pre-registered numbered plan artifact was not admitted. plan/171 and its index entry are intentionally absent because admission would require a prohibited script/source-hierarchy change.

## Documentation.md update status

Documentation.md 更新不要: the following frozen-manifest/status package will replace stale pre-registration wording without claiming product readiness.

## docs/project-status.md update status

更新不要: the following frozen-manifest package will update the current snapshot; this report alone is historical metadata, not a manifested artifact.

## progress.md update status

progress.md 更新不要: current status will update with the frozen WRK manifest, not before it.

## tasks.md update status

tasks.md 更新不要: the task map will move from execution to a next research cut only after the frozen state is manifested.

## samples_progress.md update status

samples_progress.md 更新不要: sample dashboard status will update with the frozen L3 record, not before it.

## Reviewer findings and follow-up

Focused reviewer Mill verified the output/source hashes and bounded claims, and found one exact-message transcription issue, corrected here. Planner Anscombe and a temporary Oracle advisory consultation independently concluded that the required validator/source-hierarchy change is a registered operational falsifier: freeze rather than repair, do not treat report metadata as an evidence artifact, and attribute only the sidecars. No sub-agent edited files.

## Skipped validations and reasons

No broader sample suite, direct textual `.mir` command, helper modification, schema/runtime/CLI repair, validator/source-hierarchy change, or public workflow was attempted because each exceeds WRK-0012's registered boundary. The plan/171 draft was not committed.

## Commit / push status

Not committed at report write. The next action is a frozen WRK manifest with exact metadata, current-snapshot synchronization, clean authoritative validation, make check, push, and remote-head verification.

## Sub-agent session close status

Reviewer Mill and planner Anscombe completed read-only work and are closed. No sub-agent changed files.
