# WRK-0012 direct-world sidecars (R-2346)

- Date: 2026-07-22 12:30 JST
- Author / agent: Codex, using the pre-registered WRK-0012 boundary.
- Scope: Add only the two committed direct-world source manifests before the registered outcome sequence.
- Decision levels touched: L3 evidence preparation only; no Canon theory, OBL, Gate, Phase, runtime, schema, helper, CLI, or public decision.

## Objective

Create the exact two WRK-0012 source leaves needed for the later direct-carrier command without treating construction as a successful execution result.

## Scope and assumptions

WRK-0012 at 12e6b458bc6fe82953cab813b71012b5e35bbdae permits only the two declared direct-world leaves and the later plan/171 result artifact. Both manifests use the existing Product Alpha `world` schema, disabled native policy, `RunComputationalRow`, typed read/write host I/O contract, and closed registry identifiers. The negative manifest's expected output is required schema shape, not a claim that host output is reached.

## Start state / dirty state

main and origin/main resolved to 12e6b458bc6fe82953cab813b71012b5e35bbdae with a clean worktree. Neither direct-world leaf nor any WRK-0012 outcome artifact existed.

## Documents consulted

mirrorea_canon/README.md, mirrorea_canon/MAP.md, ADR-0014, working/README.md, WRK-0012, plan/170-post-wrk0011-candidate-selection.md, Product Alpha computational direct-package samples, mir-ast Product Alpha schema validation, mir-runtime Product Alpha session tests, Documentation.md, docs/project-status.md, progress.md, tasks.md, and samples_progress.md were consulted. Canon remains normative; LAB code and samples are execution evidence only.

## Actions taken

Created the positive world package for Computational.ControlFlow.Positive.sum_to with input 5 and expected output 15, and the negative world package for Computational.Scope.NegativeUseBeforeDeclare.clamp_zero with input 3 and schema-required expected output 0. No helper, schema, runtime, CLI, script, Make target, or existing fixture was changed.

## Files changed

- samples/product-alpha1/computational/control-flow/positive/direct-world/package.mir.json
- samples/product-alpha1/computational/variables-scope/negative/direct-world/package.mir.json
- docs/reports/2346-wrk0012-direct-world-sidecars.md

## Commands run

Read-only schema, registry, and existing runtime-test inspection ran before construction. The registered matrix, regression, CLI check, and run-local commands were deliberately deferred until these exact source leaves are committed and pushed.

## Evidence / outputs / test results

No execution outcome exists. The manifests are unvalidated source preparation until the registered command sequence runs after this commit; they do not establish check acceptance, runtime success, runtime rejection, or source/registry equivalence.

## What changed in understanding

The existing runtime-test constructor maps directly to a complete serializable `world` package shape. The selected positive and negative module signatures fit the same closed schema route, while their runtime classifications remain empirical within WRK-0012.

## Open questions

Whether the two concrete checked-in manifests pass the existing checker and produce the pre-registered positive and negative runtime classifications remains open until the next package executes the exact command sequence.

## Suggested next prompt

Review the two manifests, commit and push only these leaves plus this report, then execute the registered WRK-0012 command sequence with disposable output and record its results in plan/171.

## Plan update status

plan/ 更新不要: plan/171 is the registered future result artifact and must reflect observed output, not source construction alone.

## Documentation.md update status

Documentation.md 更新不要: no user-facing capability or validated workflow readiness changed.

## docs/project-status.md update status

更新不要: no workflow result, Gate, Phase, or status claim exists before execution.

## progress.md update status

progress.md 更新不要: this package prepares source only and does not change evidence classification or macro-phase position.

## tasks.md update status

tasks.md 更新不要: the next self-driven package remains the registered outcome sequence.

## samples_progress.md update status

samples_progress.md 更新不要: no sidecar has completed its documented validation command yet.

## Reviewer findings and follow-up

No independent review is required for this L3 source-preparation package before the registered commands. Focused manifest reviewer Ohm found no issue: both leaves match the registered paths, complete `world` schema, fixed module/function/input/output values, and internal contract/capability/failure consistency; this report has all required sections and no premature outcome claim. No sub-agent edited files.

## Skipped validations and reasons

All outcome-bearing matrix, regression, CLI check, and run-local commands are deferred until this source preparation commit is at HEAD and pushed. No standalone schema check was substituted for the pre-registered sequence.

## Commit / push status

Not committed at report write. The next action is focused manifest review, exact-path audit, commit with --no-gpg-sign, make check, push, and remote-head verification.

## Sub-agent session close status

Reviewer Ohm completed read-only work and is closed. No sub-agent changed files.
