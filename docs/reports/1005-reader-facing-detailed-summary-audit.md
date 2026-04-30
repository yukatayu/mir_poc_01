# 1005 reader-facing detailed summary audit

## Objective

Remove the last remaining explicit date-stamp and stale `actual next open work` wording from the reader-facing detailed summaries that still sit on the main path after the landing-doc refresh packages.

This package is maintenance only. It does not change semantics, does not change the queue judgment, and does not touch any executable sample or runtime code.

## Scope and assumptions

- `progress.md` / `tasks.md` remain queue authority.
- The landing pages were already cooled in package `1004`; this package only fixes the remaining detailed-summary wording hits.
- No `plan/` update is needed because repository memory itself is already aligned.

## Documents consulted

- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/reports/1003-plan11-and-roadmap-mirror-point-in-time-audit.md`
- `docs/reports/1004-landing-docs-date-stamp-audit.md`

## Actions taken

- Searched the reader-facing summaries for residual explicit date stamps and hot queue wording after the front-door cleanup packages.
- Updated `docs/research_abstract/mirrorea_future_axis_01.md` so its top hot-plug status paragraph now speaks in `current snapshot reading` terms rather than `actual next open work`.
- Updated `docs/research_abstract/projection_placement_plan_01.md` so the preview-floor introduction now reads as current repository memory rather than a stale `2026-04-28` timestamp.
- Updated `progress.md` and `tasks.md` to record the maintenance-only closeout.

## Files changed

- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1005-reader-facing-detailed-summary-audit.md`

## Evidence / outputs / test results

Commands run before edits:

| Command | Result | Output summary |
|---|---|---|
| `rg -n "2026-04-(2[8-9]|30) 時点|actual next open work|next queue" docs/research_abstract docs/hands_on README.md Documentation.md \| sed -n '1,220p'` | pass | only two residual hits remained: `mirrorea_future_axis_01.md` and `projection_placement_plan_01.md` |
| `nl -ba docs/research_abstract/mirrorea_future_axis_01.md \| sed -n '36,52p;268,304p'` | pass | confirmed the remaining `actual next open work` wording |
| `nl -ba docs/research_abstract/projection_placement_plan_01.md \| sed -n '1,80p'` | pass | confirmed the stale `2026-04-28` preview-floor wording |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 15:34 JST` |
| `git diff --stat` | pass | only two reader-facing summaries plus snapshot trail changed |
| `rg -n "2026-04-(2[8-9]|30) 時点|actual next open work|next queue" docs/research_abstract docs/hands_on README.md Documentation.md \| sed -n '1,220p'` | pass after edits | no remaining hits |

Docs-floor rerun after the edits:

| Command | Result | Output summary |
|---|---|---|
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 1003 numbered report(s).` |
| `git diff --check` | pass | no output |

Observed drift that this package corrects:

- `docs/research_abstract/mirrorea_future_axis_01.md` still used `actual next open work` even though the file had already been cooled into reader-facing closeout memory.
- `docs/research_abstract/projection_placement_plan_01.md` still carried a stale `2026-04-28` “current preview floor” date stamp despite being refreshed in place.

## What changed in understanding

- After the landing pages are cleaned, the remaining stale wording tends to survive inside detailed summaries rather than in entry docs.
- A targeted regex sweep is effective for this class of drift because the residual problems are lexical, not semantic.

## Open questions

- Should there be a maintenance script that flags explicit `YYYY-MM-DD 時点` wording in reader-facing docs outside intentionally historical reports?

## Suggested next prompt

Continue with the next safe maintenance package: scan other reader-facing docs for non-authoritative but stale lexical markers that can be machine-flagged, such as `actual next open work`, `next promoted line`, or explicit date stamps outside reports.

## plan/progress/tasks/samples updates

- `plan/`: 更新不要
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No full sample/Cargo rerun is planned for this package because the edits are wording-only and touch no executable path.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the post-edit docs-floor rerun.

## Sub-agent session close status

- No sub-agent was required for this narrow lexical cleanup pass.
