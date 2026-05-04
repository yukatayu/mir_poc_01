# Report 1164 — VIS-A1-04 practical hot-plug lifecycle export review

- Date: 2026-05-04 11:16:51 JST
- Author / agent: Codex
- Scope: Review the proposed practical package shape for adding `VIS-A1-04` hot-plug lifecycle export by consuming exact practical hot-plug reports only, with focus on overclaim risk and safe-next-package status.
- Decision levels touched: none; review only

## Objective

Review whether a practical `VIS-A1-04` package is semantically safe if it exports hot-plug lifecycle information only from exact practical hot-plug reports, likely anchored to `HP-A1-01` accepted attach and `HP-A1-07` deferred detach boundary.

## Scope and assumptions

- Review only; no normative text edits were requested.
- Assumed target shape is export/viewer work only, not new runtime or hot-plug semantics.
- Focused on `specs/18`, `plan/44`, `Documentation.md`, `progress.md`, `tasks.md`, and practical sample/readme evidence.

## Start state / dirty state

- `git status --short` was clean at review start.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/hp-a1-01-debug-layer-attach.expected.json`
- `samples/practical-alpha1/expected/hp-a1-07-detach-minimal-contract.expected.json`

## Actions taken

- Read required repo front-door and baseline specs in the mandated order.
- Read practical alpha-1 normative and roadmap documents plus hot-plug boundary memory.
- Searched practical-alpha and snapshot docs for `VIS-A1-04`, `HP-A1-01`, `HP-A1-07`, detach/lifecycle wording, and product-preview wording.
- Inspected exact practical hot-plug expected reports to confirm available lifecycle lanes and stop-line fields.
- Wrote this review report.

## Files changed

- `docs/reports/1164-review-vis-a1-04-practical-hotplug-lifecycle-export.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `rg --files -g 'README.md' -g 'Documentation.md' -g 'progress.md' -g 'tasks.md' -g 'samples_progress.md' -g 'specs/*' -g 'plan/*'`
- `sed -n ...` over the consulted docs listed above
- `rg -n "VIS-A1-04|HP-A1-01|HP-A1-07|deferred detach|detach runtime lifecycle|object_attach_claimed|product prototype preview|product prototype|hot-plug lifecycle" ...`
- `git status --short`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `python3 -m unittest scripts.tests.test_validate_docs`

## Evidence / outputs / test results

- Snapshot/roadmap docs currently state that practical progress stops at `P-A1-07` and that the next autonomous package is blocked pending `P-A1-08` recut or prior practical avatar carriers.
- `specs/18` and `plan/44` both permit future `VIS-A1-04` devtools work, but current wording requires that it remain exact-report export only and not imply detach runtime lifecycle or product completion.
- Exact expected practical hot-plug reports already contain the lanes needed for a narrow export:
  - `HP-A1-01` carries accepted attach evidence including `operation_kind = attach`, verdict lanes, and `activation_cut_ref`.
  - `HP-A1-07` carries deferred detach evidence including `operation_kind = detach`, `detach_boundary_ref`, `terminal_outcome = deferred_detach_minimal_contract`, explicit stop lines, and `object_attach_claimed = false`.
- Docs/report guardrail validation passed:
  - `python3 scripts/validate_docs.py`
  - `python3 scripts/check_source_hierarchy.py`
  - `git diff --check`
  - `python3 -m unittest scripts.tests.test_validate_docs`
- Practical runtime/Cargo/package matrices were not rerun because this task was a wording/spec review rather than an implementation change.

## What changed in understanding

- The proposed `VIS-A1-04` shape is technically supportable from existing exact practical reports.
- The main risk is not missing carrier data but overclaim drift in package naming and queue status.
- As written today, repo snapshots still present `P-A1-08` recut as the next gate, so promoting `VIS-A1-04` as the next autonomous package would conflict with current queue authority unless `plan/44`, `progress.md`, and `tasks.md` are intentionally reopened and synchronized.

## Open questions

- Should this be introduced as a reopened `PA1-6` widening package before `P-A1-08`, or kept subordinate to the `P-A1-08` recut decision?
- Should the row/package wording say `hot-plug lifecycle export preview` or `attach + deferred-detach boundary export` to avoid reading as full lifecycle completion?

## Suggested next prompt

Review and, if approved, recut the queue so `PA1-6` reopens as a narrow `VIS-A1-04` exact-report export package before `P-A1-08`, with wording constrained to accepted attach plus deferred detach boundary only and with explicit non-claims for detach execution, object attach completion, and product prototype readiness.

## Plan update status

`plan/` 更新不要: review-only task; no roadmap authority was changed.

## Documentation.md update status

`Documentation.md` 更新不要: reviewed for wording risk only.

## progress.md update status

`progress.md` 更新不要: reviewed for queue-authority conflict only.

## tasks.md update status

`tasks.md` 更新不要: reviewed for queue-authority conflict only.

## samples_progress.md update status

`samples_progress.md` 更新不要: sample dashboard status was not changed.

## Reviewer findings and follow-up

- Finding: a narrow `VIS-A1-04` export package can be grounded in exact practical hot-plug reports without adding runtime semantics.
- Finding: generic `hot-plug lifecycle` wording is still risky unless the package explicitly says it exports accepted attach plus deferred detach boundary only.
- Finding: current queue authority still says the next autonomous move is blocked on `P-A1-08` recut or prior `AV-A1-*`, so a `VIS-A1-04` package is not a safe next autonomous package unless those snapshot docs are deliberately recut first.

## Skipped validations and reasons

- Did not rerun practical runtime/Cargo/package validation suites such as `practical_alpha1_attach.py`, `practical_alpha1_export_devtools.py`, or Cargo practical hot-plug tests, because no implementation or normative/snapshot package semantics were changed in this review-only task.
- Did not commit or push. User requested review findings, not a repository update package.

## Commit / push status

Not committed. Not pushed.

## Sub-agent session close status

No sub-agent sessions were opened for this task.
