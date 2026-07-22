# WRK-0012 P-COMP-03 direct-carrier preregistration (R-2345)

- Date: 2026-07-22 12:22 JST
- Author / agent: Codex, with read-only source exploration by sub-agent Kant.
- Scope: Register only a reversible L3 experiment before creating sidecars or relying on outcomes.
- Decision levels touched: L3 only; no L0/L1 decision, Canon theory, OBL, Gate, Phase, or implementation change.

## Objective

Commit the bounded P-COMP-03 direct-carrier question before creating either sidecar manifest or running its matrix, test, check, or runtime commands.

## Scope and assumptions

The record pins its authority and LAB-input snapshots to 66229addaa1044c4759a2759b5ef41f355f25d11, while its registration parent is the later guardrail commit 0dcc9dd38d2153c7fcc5f5c31fdc7ad48d6929d3. Its LAB inputs are plan/170 and the two selected P-COMP-03 row manifests; its permitted locations are the existing `plan/` root and the two selected row directories. Its retained future artifact names are fixed to plan/171-wrk0012-pcomp03-direct-carrier-evidence.md and the two exact non-production direct-world/package.mir.json leaves. Existing scripts and Rust crates are unmodified execution machinery; direct textual .mir remains out of scope.

## Start state / dirty state

The candidate selection was made at 66229addaa1044c4759a2759b5ef41f355f25d11. Before this registration package, main was fast-forwarded to 0dcc9dd38d2153c7fcc5f5c31fdc7ad48d6929d3, the separately committed validator guardrail. The primary worktree then contained only this uncommitted WRK-0012 registration metadata draft; no sidecar or outcome artifact existed.

## Documents consulted

mirrorea_canon/README.md, mirrorea_canon/MAP.md, ADR-0014, architecture/02-boundary-contracts.md, theory/11-metatheory-ledger.md, working/README.md, plan/170-post-wrk0011-candidate-selection.md, the Product Alpha computational README/matrix/fixed manifests/direct manifest, Documentation.md, docs/project-status.md, progress.md, tasks.md, and samples_progress.md were consulted in source-hierarchy order. Read-only sub-agent Kant traced the current CLI/schema/runtime/test path.

## Actions taken

This report and WRK-0012 pre-register exact sidecar paths, the two fixed module requests, check/runtime classifications, command sequence, falsifier, rollback, and non-effects. The registration revision narrows LAB inputs to plan/170 and the two selected rows, fixes the future plan evidence filename, separates the pinned authority/LAB snapshot from the later sidecar execution cut, and corrects command grouping so any prior required command failure propagates rather than being bypassed by the expected-negative exit handling. No registered outcome command was executed before this record was prepared.

## Files changed

- mirrorea_canon/working/WRK-0012-pcomp03-direct-carrier.md
- mirrorea_canon/MAP.md
- mirrorea_canon/INDEX.json after canonical regeneration
- docs/reports/2345-wrk0012-pcomp03-direct-carrier-preregistration.md

## Commands run

Read-only canon/LAB/source inspection, pinned digest calculation, source-hierarchy/working-validator inspection, command failure-propagation inspection, and Git status checks were run. The registered matrix, regression, check, and runtime sequence was deliberately not run before the preregistration commit.

## Evidence / outputs / test results

No outcome evidence exists at this stage. The registered experiment later checks one positive sum_to(5)=Int(15) sidecar and one negative clamp_zero(3) sidecar, with the latter expected to return the existing MirCompute error and unbound variable detail after a successful schema check.

## What changed in understanding

The existing direct carrier is not limited to the current two direct sample roots: the schema's closed module registry includes both selected P-COMP-03 module/function signatures. That makes a two-leaf, falsifiable carrier test possible without treating the helper fixtures themselves as already direct.

## Open questions

Only the registered execution may determine whether the proposed manifests actually validate and produce the stated outcomes. Any need to repair a helper, schema, script, runtime, CLI, or public surface is a stop condition, not work to perform within this record.

## Suggested next prompt

Validate this pure registration in a clean disposable worktree, push and verify its remote head, then create only the two committed-path sidecar manifests and plan/171-wrk0012-pcomp03-direct-carrier-evidence.md. Only after that registration close may the committed WRK-0012 sequence run with disposable session output.

## Plan update status

plan/ 更新不要: the registration commit contains only the new working record and exact operational metadata. A later evidence package may retain its result under plan/.

## Documentation.md update status

Documentation.md 更新不要: preregistration changes no user-facing capability or outcome status. A follow-up snapshot package will replace the temporary selected-but-unregistered wording after this commit is pushed.

## docs/project-status.md update status

更新不要: this exact registration metadata package creates no workflow, Gate, Phase, or result claim; its stale registration wording will be synchronized by the immediate evidence/snapshot package.

## progress.md update status

progress.md 更新不要: no outcome or workflow readiness changed before the registered command sequence runs.

## tasks.md update status

tasks.md 更新不要: it already identifies WRK-0012 registration as the next self-driven package; outcome/status synchronization follows execution.

## samples_progress.md update status

samples_progress.md 更新不要: no sidecar, runnable command, debug surface, or dashboard classification exists before this registration is committed.

## Reviewer findings and follow-up

Read-only explorer Kant confirmed that a directory-sidecar is necessary because the CLI requires package.mir.json, that the module/function/type tuples are in the existing closed registry, and that the negative runtime route has the registered error classification. Focused registration reviewer Arendt found four issues before commit: an unused base assignment, misleading `plan/170` authorization wording, an unnecessary external-review reference, and a make-check wording conflict. This revision separates the snapshot from execution, states the actual plan-root permission and exact intended artifact, removes the unrelated reference, and records make check as pending. Final reviewer Boole found no remaining issue and independently confirmed the six pinned SHA-256 values, registration purity, snapshot-versus-execution wording, planned artifact, absence of outcome claims, and expected-negative command grouping. No sub-agent edited files.

## Skipped validations and reasons

The registered outcome sequence is intentionally deferred until WRK-0012 is committed at HEAD, as required by the working annex. Pre-commit validation is limited to static checks; committed-HEAD validation and make check are pending and will run before push. The detached full documentation-unit wrapper is not treated as completion evidence because its outer wrapper lacks a final status.

## Commit / push status

Not committed at report write. The next action is canonical index regeneration, focused registration-diff review, committed-HEAD validation in a clean disposable worktree, make check after commit, push, and remote-head verification.

## Sub-agent session close status

Explorer Kant and reviewers Arendt/Boole completed read-only work and are closed. No sub-agent changed files.
