# Report 1109 — P-A0-11 sample taxonomy review

- Date: 2026-05-02 09:51 JST
- Author / agent: Codex
- Scope: Review `P-A0-11` alpha sample coverage/taxonomy wording for planned-vs-runnable consistency, focusing on `samples/README.md`, `samples_progress.md`, `samples/alpha/README.md`, `samples/alpha/e2e/README.md`, and `sub-agent-pro/alpha-0/08-sample-matrix.md`
- Decision levels touched: none; review only

## Objective

Identify taxonomy inconsistencies and required README/dashboard updates if `P-A0-11` adds an integrated demo runner or a closeout helper for the Alpha-0 E2E family.

## Scope and assumptions

- This task is a documentation/taxonomy review, not an implementation package.
- Normative judgments remain in `specs/`; this report records review findings only.
- The review checks current wording against two possible additions:
  - an executable integrated demo runner
  - an inventory-style closeout helper

## Start state / dirty state

- Initial `git status --short` capture returned no visible output in the tool transcript.
- Later `git status --short --branch` showed additional untracked `P-A0-11` review reports already present in the tree.
- Because the first capture was inconclusive and concurrent report files appeared during the task, whether those untracked files predated this task is `UNRESOLVED`.

## Documents consulted

- `sub-agent-pro/alpha-0/08-sample-matrix.md`
- `README.md`
- `Documentation.md`
- `specs/00-vision.md`
- `specs/01-core.md`
- `specs/02-mir.md`
- `specs/03-portal-and-place.md`
- `specs/09-user-experience.md`
- `samples/README.md`
- `samples_progress.md`
- `samples/alpha/README.md`
- `samples/alpha/e2e/README.md`
- `tasks.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/43-alpha-e2e-roadmap.md`

## Actions taken

- Read the required repository context in the order mandated by `AGENTS.md`.
- Compared current alpha taxonomy wording against the current `P-A0-11` package description and Stage F completion memory.
- Checked whether the existing docs distinguish:
  - scaffold-only families
  - runner-backed but non-public alpha subfamilies
  - executable validation commands versus inventory/closeout helpers
- Wrote review findings and required follow-up updates in this report.

## Files changed

- `docs/reports/1109-p-a0-11-sample-taxonomy-review.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `nl -ba sub-agent-pro/alpha-0/08-sample-matrix.md | sed -n '1,260p'`
- `nl -ba README.md | sed -n '1,260p'`
- `nl -ba Documentation.md | sed -n '1,260p'`
- `nl -ba specs/00-vision.md | sed -n '1,260p'`
- `nl -ba specs/01-core.md | sed -n '1,260p'`
- `nl -ba specs/02-mir.md | sed -n '1,260p'`
- `nl -ba specs/03-portal-and-place.md | sed -n '1,260p'`
- `nl -ba specs/09-user-experience.md | sed -n '1,260p'`
- `nl -ba samples/README.md | sed -n '1,260p'`
- `nl -ba samples_progress.md | sed -n '1,260p'`
- `nl -ba samples/alpha/README.md | sed -n '1,260p'`
- `nl -ba samples/alpha/e2e/README.md | sed -n '1,260p'`
- `rg -n "P-A0-11|alpha|planned skeleton|runnable|closeout|integrated demo|demo runner|runner" samples_progress.md tasks.md progress.md Documentation.md plan sub-agent-pro/alpha-0 -g '*.md'`
- `nl -ba plan/43-alpha-e2e-roadmap.md | sed -n '1,220p'`
- `nl -ba plan/19-repository-map-and-taxonomy.md | sed -n '90,170p'`
- `nl -ba tasks.md | sed -n '65,150p'`
- `rg -n "samples/alpha|alpha-local|runnable root|closeout|runner|Stage F|integrated alpha demo" README.md Documentation.md`
- `nl -ba README.md | sed -n '44,62p'`
- `nl -ba Documentation.md | sed -n '1,220p'`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `git status --short --branch`
- `ls docs/reports | sort | tail -n 10`

## Evidence / outputs / test results

- `samples/alpha/README.md` currently leads with `active runnable root ではない` and `current package では planned skeleton / expected-verdict sidecar を置く`, but the same file later says `local-runtime/`, `layer-insertion/`, `network-docker/`, and `avatar-runtime/` already have non-public runner-backed floors.
- `samples/alpha/e2e/README.md` still states `planned skeleton only`, `Current runners do not execute this family yet`, and `Validation for this package is filesystem/docs integrity only`.
- `samples_progress.md` still classifies `A0-E2E` as `scaffold family`, `10`, and `target matrix only`.
- `sub-agent-pro/alpha-0/08-sample-matrix.md` already anticipates a future executable family runner for E2E via `scripts/alpha_e2e_samples.py check-all --format json`.
- `plan/43-alpha-e2e-roadmap.md` defines Stage F as an integrated alpha demo with explicit runtime/network/hot-plug/devtools evidence and stop-lines against marking skeleton rows runnable.
- Post-report docs guardrails passed:
  `python3 scripts/check_source_hierarchy.py` reported required/present/missing = `60/60/0`,
  `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1111 numbered report(s).`,
  and `git diff --check` was clean.

## What changed in understanding

- The main issue is not whether `samples/alpha/` becomes an active root; current docs consistently reject that.
- The real gap is finer-grained taxonomy inside `samples/alpha/`: current wording does not cleanly separate scaffold-only families from runner-backed but still non-public families.
- `P-A0-11` therefore needs a command-status taxonomy, not a root-promotion taxonomy.

## Open questions

- If `P-A0-11` lands only a closeout helper, should that helper be treated as inventory-only evidence like other repo helpers, or is executable integrated row coverage required before any `A0-E2E` progress increase beyond scaffold status?
- Should the integrated alpha demo command live as an E2E-specific runner (matching the handoff expectation) or as an aggregate closeout command that orchestrates existing Stage B/C/D runners plus any new Stage E/F checks?

## Suggested next prompt

Implement the taxonomy/doc updates for `P-A0-11`: add explicit alpha family states for `scaffold-only`, `runner-backed non-public floor`, and `inventory-only closeout`; update `samples/README.md`, `samples/alpha/README.md`, `samples/alpha/e2e/README.md`, and `samples_progress.md` to classify `A0-E2E` correctly for either an executable integrated runner or an inventory-only helper.

## Plan update status

`plan/` 更新不要:
review only; repository-memory inconsistency was identified, but this task did not resolve package design or taxonomy policy text.

## Documentation.md update status

`Documentation.md` 更新不要:
review only; no snapshot claim was changed in this task.

## progress.md update status

`progress.md` 更新不要:
review only; no package state or validation result changed.

## tasks.md update status

`tasks.md` 更新不要:
review only; `P-A0-11` remains the next package and no task-map decision changed.

## samples_progress.md update status

`samples_progress.md` 更新不要:
this task identified required future updates for `A0-E2E`, but did not implement a runner/helper or change validation evidence.

## Reviewer findings and follow-up

- Local review finding 1:
  `samples/alpha/README.md` overstates the root as a planned-skeleton package even though several alpha subfamilies are already runner-backed. Follow-up: change the lead bullets so the root is described as a mixed alpha scaffold containing both scaffold-only families and non-public runner-backed families.
- Local review finding 2:
  `samples/alpha/e2e/README.md` has no taxonomy for `inventory-only closeout` versus `executable integrated demo`. Follow-up: if `P-A0-11` adds only a closeout helper, say so explicitly and keep the family non-runnable; if it adds executable Stage-F coverage, replace the current filesystem-only wording with exact validation commands and stop-lines.
- Local review finding 3:
  `samples_progress.md` needs command-classification updates before `A0-E2E` can move off scaffold status. Follow-up: keep `A0-E2E` at `<=25%` for an inventory-only helper, or update progress/last-validation/notes to the new executable command if Stage-F rows are actually run.

## Skipped validations and reasons

- Sample/runtime/Cargo validation was not run because this task made no implementation changes to alpha runners.
- Docs guardrail validation was deferred until after writing this report, because the report file itself must exist before `validate_docs.py` can check the latest-report headings.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent session was opened.
