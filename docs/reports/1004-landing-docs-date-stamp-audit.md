# 1004 landing docs date-stamp audit

## Objective

Audit the remaining front-door landing docs for stale date-stamped “current line” wording and residual `next queue` phrasing, then cool that wording without changing the underlying snapshot judgment.

This package is maintenance only. It does not change semantics, does not alter roadmap sequencing, and does not reopen any implementation line.

## Scope and assumptions

- `progress.md` / `tasks.md` remain queue authority.
- The previous roadmap-memory packages (`1002`, `1003`) already settled queue-authority wording; this package only removes residual stale landing-page phrasing.
- No `plan/` update is needed unless the landing docs expose a mismatch in repository memory itself.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/README.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/1002-plan01-point-in-time-disclaimer-audit.md`
- `docs/reports/1003-plan11-and-roadmap-mirror-point-in-time-audit.md`

## Actions taken

- Audited `README.md`, `Documentation.md`, and `docs/research_abstract/README.md` for the remaining stale `2026-04-29` “current line” phrasing.
- Updated `README.md` so the top-level repo description no longer looks like a dated snapshot and so the `current_phase_closeout_01.md` link no longer advertises a `next queue`.
- Updated `Documentation.md` so the top-level current-line sentence and the `current_phase_closeout_01.md` reference no longer carry stale point-in-time wording.
- Updated `docs/research_abstract/README.md` so its `current reading の要点` section no longer begins with a stale date stamp.
- Updated `progress.md` and `tasks.md` to record the maintenance-only closeout.

## Files changed

- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1004-landing-docs-date-stamp-audit.md`

## Evidence / outputs / test results

Commands run before edits:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree after package `1003` |
| `nl -ba README.md \| sed -n '1,28p;216,232p'` | pass | found stale `2026-04-29` wording and a `next queue` phrase in the `current_phase_closeout_01.md` link blurb |
| `nl -ba Documentation.md \| sed -n '1,24p;276,292p'` | pass | found stale `2026-04-29` wording in the intro and closeout-guide link blurb |
| `nl -ba docs/research_abstract/README.md \| sed -n '100,124p'` | pass | found stale `2026-04-29` wording in the `current reading の要点` section |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 15:32 JST` |
| `git diff --stat` | pass | only landing docs, snapshot docs, and the new report changed |

Docs-floor rerun after the edits:

| Command | Result | Output summary |
|---|---|---|
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 1002 numbered report(s).` |
| `git diff --check` | pass | no output |

Observed drift that this package corrects:

- `README.md` and `Documentation.md` still presented the repo’s current line as a dated `2026-04-29` statement even after the snapshot docs had moved through additional maintenance packages.
- `README.md` still described `docs/hands_on/current_phase_closeout_01.md` as a place to read the `next queue`, which contradicted the authority split repaired in package `1003`.
- `docs/research_abstract/README.md` still opened its current-line summary with a stale date even though the section is refreshed in place.

## What changed in understanding

- After queue-authority wording is cooled, stale explicit dates become the next major reader-facing freshness risk.
- The landing docs can stay stable if they describe the repo’s current line generically and defer exact timestamps to `progress.md` and `docs/reports/`.

## Open questions

- Should the remaining reader-facing detailed summaries be systematically searched for explicit closeout dates that are not intentionally historical?
- Should there be a short repository-wide wording rule that landing pages should avoid explicit `YYYY-MM-DD 時点` unless the file is intentionally a historical memo?

## Suggested next prompt

Continue with the next safe maintenance package: audit reader-facing detailed summaries for non-historical explicit date stamps and other stale “current line” wording, then rerun the docs-focused floor and close with a new report.

## plan/progress/tasks/samples updates

- `plan/`: 更新不要
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No full sample/Cargo rerun is planned for this package because the edits are landing-doc wording only.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the post-edit docs-floor rerun.

## Sub-agent session close status

- No sub-agent was required for this narrow landing-doc wording pass.
