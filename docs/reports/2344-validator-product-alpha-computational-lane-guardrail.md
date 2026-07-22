# Validator Product Alpha computational lane guardrail (R-2344)

- Date: 2026-07-22 12:05 JST
- Author / agent: Codex, with read-only planner Hegel, temporary Oracle advisory review, and two focused reviewer passes.
- Scope: Repair one validator false negative for an already documented LAB lane, before any WRK-0012 registration or outcome command.
- Decision levels touched: LAB validation implementation only; no Canon theory, OBL, Gate, Phase, runtime, schema, helper, CLI, or public decision.

## Objective

Align the existing working-record lane validator with ADR-0014 for the documented Product Alpha computational lane while preserving its fail-closed boundary for parent, sibling, and unrelated paths. A record may select an existing descendant to narrow its own evidence boundary.

## Scope and assumptions

ADR-0014 and the working annex are normative. The prior validator root tuple was an executable guardrail, not a Canon source. This package adds only samples/product-alpha1/computational as one recognized documented lane root. Only that root may be declared through an existing descendant; every pre-existing root remains exact-match-only. It does not implement generic lane discovery, a new registry/schema, a read/write scope split, or any P-COMP-03 evidence.

## Start state / dirty state

The isolated guardrail worktree began clean at main 66229addaa1044c4759a2759b5ef41f355f25d11. The primary worktree holds an uncommitted WRK-0012 draft that was not copied into this package. No outcome command, sidecar, Canon working record, MAP, INDEX, or Product Alpha source is changed here.

## Documents consulted

mirrorea_canon/README.md, mirrorea_canon/MAP.md, ADR-0014, working/README.md, architecture/02-boundary-contracts.md, plan/170-post-wrk0011-candidate-selection.md, samples/README.md, samples/product-alpha1/README.md, the computational README, samples_progress.md, scripts/validate_docs.py, scripts/tests/test_validate_docs.py, Documentation.md, docs/project-status.md, progress.md, tasks.md, and samples_progress.md were read. Temporary Oracle and planner reviews were advisory only.

## Actions taken

1. Reproduced the false negative: the pending record's existing Product Alpha computational permitted location was rejected solely by the four-entry static root tuple.
2. Confirmed the lane is already documented and bounded, while broad Product Alpha and sibling roots remain unsuitable.
3. In a separate worktree, added a RED test for recognized-lane and row-descendant declaration acceptance plus parent/sibling/unrelated-root rejection; it failed because the computational descendants were absent.
4. Added one exact tuple entry and an initially too-broad descendant check. The first reviewer caught that it also admitted arbitrary descendants of plan and samples/lean.
5. Added a RED regression for those unintended roots, then restricted descendant declarations to the Product Alpha computational root alone.
6. Added full working-annex evidence-path tests: a declared row admits its direct-world leaf, while its control-flow parent is rejected by the real registration/evidence/manifest history path.
7. Kept the pending WRK registration and all experiment material outside this package.

## Files changed

- scripts/validate_docs.py
- scripts/tests/test_validate_docs.py
- docs/reports/2344-validator-product-alpha-computational-lane-guardrail.md

## Commands run

Baseline validation completed before edits: python3 -m unittest scripts.tests.test_validate_docs -f -v, 83 tests passed. The first focused test failed because the computational root was absent. The first reviewer found a missing evidence-path assertion; the second found that the initial descendant predicate widened plan and samples/lean. The new focused regression failed exactly on those two unintended descendants, then passed after the explicit descendant-root restriction. The final focused set covers declaration acceptance/rejection and actual evidence history acceptance/rejection. The final full suite passed all 87 tests in 474.042 seconds. Documentation/source-hierarchy/Canon-index checks and make check run before commit.

## Evidence / outputs / test results

The new regressions prove that the computational root and its record-selected row descendants are recognized, while samples, samples/product-alpha1, samples/product-alpha1/posegraph, scripts, crates/mir-runtime, and arbitrary descendants of each legacy root (plan, samples/clean-near-end, samples/current-l2, and samples/lean) are rejected. A declared control-flow positive row admits its direct-world sidecar through the complete working-annex registration/evidence/manifest path and rejects a control-flow-parent artifact through that same path. The focused helper-path test additionally rejects a sibling row, variables/scope row, matrix, and script paths. Existing history, registration-purity, evidence-attribution, path-safety, and digest checks ran in the final 87-test suite. No P-COMP-03 outcome evidence exists.

## What changed in understanding

The static root tuple had silently reinstated an old fixed-root restriction that the Canon route does not impose. Recognizing one documented lane while allowing a record to select a narrower existing descendant is distinct from generic authorization or from granting an entire Product Alpha parent. Descendant admission must itself be scoped per documented root: applying it to the legacy roots would create an unreviewed policy expansion. The record format still mechanically conflates pinned-input containment and evidence-change containment; changing that remains out of scope.

## Open questions

A future broader lane discovery mechanism, descendant-permitted-location rule for any other lane, or separate read-only/write evidence fields would need a separately governed design. This correction does not decide those questions. WRK-0012 still requires a pure registration commit and clean authoritative validation before execution.

## Suggested next prompt

Update the primary worktree to this guardrail commit, rebase the uncommitted WRK-0012 draft on its parent, commit only the working record and exact metadata, validate it in a clean worktree, then run no outcome command until that registration is pushed.

## Plan update status

plan/ 更新不要: plan/170 already identifies this exact existing lane and stop line. The correction repairs the validator implementation without changing the candidate comparison or research direction.

## Documentation.md update status

Documentation.md 更新不要: no user-facing capability, workflow readiness, or project direction changed.

## docs/project-status.md update status

更新不要: this is a pre-registration validator guardrail correction, not a working-record result, workflow, Gate, or Phase change.

## progress.md update status

progress.md 更新不要: no outcome, evidence classification, or macro-phase position changed.

## tasks.md update status

tasks.md 更新不要: WRK-0012 remains the same next self-driven registration package; this commit removes only its validator false negative.

## samples_progress.md update status

samples_progress.md 更新不要: no sample, command, debug surface, or runnable workflow changed.

## Reviewer findings and follow-up

Planner Hegel and the temporary Oracle review agreed that a separate guardrail correction is appropriate, provided it does not create generic discovery or broaden to Product Alpha parents/siblings. Initial reviewer Laplace found that the first draft incorrectly described evidence descendants as rejected; the response added row-local evidence containment. Reviewer Tesla then found two material issues: the descendant predicate widened every legacy root, and tests did not exercise the real working-annex path. The response restricts descendant declarations to the Product Alpha computational root and adds successful and rejected real history-path tests. Final reviewer Lovelace found only P2 coverage for two untouched legacy roots; the response adds samples/clean-near-end and samples/current-l2 rejection assertions. The final focused set and full 87-test suite pass; no remaining reviewer finding is open.

## Skipped validations and reasons

No P-COMP-03 matrix, regression, CLI check, or run-local command ran because WRK-0012 is not committed at HEAD. No generic lane-discovery test was added because that would select a new policy rather than repair this exact false negative.

## Commit / push status

Not committed at report write. After final review and focused validation, this isolated package will commit with --no-gpg-sign, push to origin/main, and verify the remote head before the separate WRK registration package starts.

## Sub-agent session close status

Planner Hegel, explorer Kant, and reviewers Laplace/Tesla/Lovelace completed read-only work and are closed after their advice was recorded. No sub-agent edited this package.
