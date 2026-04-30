# 1003 plan11 and roadmap mirror point-in-time audit

## Objective

Audit `plan/11-roadmap-near-term.md` and the nearby reader-facing roadmap mirrors for wording that still reads like live queue authority or stale point-in-time status, then cool that wording without reopening any implementation work.

This package is maintenance only. It does not change normative semantics, does not alter the validation floor, and does not promote any new post-`U1` implementation line.

## Scope and assumptions

- `progress.md` / `tasks.md` remain queue authority; `plan/` remains repository memory.
- Reader-facing summaries may describe the current repo line, but they must not replace the snapshot docs as the active queue source.
- `NET-01` remains a reported Sugoroku parity anchor, while runnable transport canaries remain `NET-02..05`.
- The prior full validation floor in `docs/reports/1001-*` remains the latest broad execution evidence; this package reruns only the docs-focused floor.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `docs/research_abstract/README.md`
- `docs/hands_on/README.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/1001-full-validation-rerun-and-front-door-parity.md`
- `docs/reports/1002-plan01-point-in-time-disclaimer-audit.md`

## Actions taken

- Audited `plan/11` for the same document-role ambiguity that had already been fixed in `plan/01`.
- Added a short `この文書について` section to `plan/11` so the file states explicitly that it is near-term roadmap memory, not live queue authority.
- Renamed `plan/11`'s queue section to `historical closeout chain` and clarified that the actual maintenance-line authority remains in `progress.md` / `tasks.md`.
- Updated the closing note in `plan/11` so the active snapshot authority points to the actual snapshot docs instead of a vague “Mirrorea future-axis queue”.
- Updated `docs/research_abstract/mirrorea_future_axis_01.md` to:
  - state explicitly that queue authority remains in `progress.md` / `tasks.md`,
  - remove stale date-stamped “current line” wording,
  - align the transport row and transport section with the reported `NET-01` / runnable `NET-02..05` split,
-  - cool roadmap references from dated “as of 2026-04-29” phrasing to current repository-memory phrasing,
  - keep `U1` as the snapshot reading while deferring queue authority to `progress.md` / `tasks.md`.
- Updated `docs/hands_on/current_phase_closeout_01.md` to remove a stale top-level date and to demote `next open gate` wording to closeout-memory wording.
- Updated `README.md`, `Documentation.md`, `docs/research_abstract/README.md`, and `docs/hands_on/README.md` so front-door docs no longer advertise a live `next promoted line` / `next queue` outside `progress.md` / `tasks.md`.
- Updated `progress.md` and `tasks.md` to record this maintenance-only closeout.

## Files changed

- `plan/11-roadmap-near-term.md`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1003-plan11-and-roadmap-mirror-point-in-time-audit.md`

## Evidence / outputs / test results

Commands run before edits:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree after package `1002` |
| `rg -n "queue authority\\|maintenance-only\\|point-in-time\\|U1\\|next reopen point\\|next open work\\|NET-01\\|NET-02..05\\|actual commitment\\|third recommendation\\|current line" plan/11-roadmap-near-term.md docs/research_abstract/mirrorea_future_axis_01.md docs/hands_on/current_phase_closeout_01.md Documentation.md README.md \| sed -n '1,260p'` | pass | found stale `NET-01..05` wording and date-stamped current-line phrasing in the reader-facing mirror, plus missing queue-authority disclaimers in `plan/11` |
| `nl -ba plan/11-roadmap-near-term.md \| sed -n '1,220p'` | pass | confirmed `plan/11` still lacked a `この文書について` section and still named a `stabilized queue` section without saying it was memory |
| `nl -ba docs/research_abstract/mirrorea_future_axis_01.md \| sed -n '1,260p'` | pass | confirmed stale top-level date wording and `NET-01..05` wording |
| `nl -ba docs/hands_on/current_phase_closeout_01.md \| sed -n '1,240p'` | pass | confirmed the stale top-level `2026-04-29` framing |
| `date '+%Y-%m-%d %H:%M %Z'` | pass | `2026-04-30 15:29 JST` |
| `git diff --stat` | pass | nine docs plus one new report file changed; no source/sample/runtime files touched |

Docs-floor rerun after the edits:

| Command | Result | Output summary |
|---|---|---|
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 1001 numbered report(s).` |
| `git diff --check` | pass | no output |

Observed drift that this package corrects:

- `plan/11` still behaved like a roadmap snapshot without explicitly stating that queue authority belongs elsewhere.
- `docs/research_abstract/mirrorea_future_axis_01.md` still mixed current-line summary with stale date markers and the older `NET-01..05` shorthand.
- `docs/hands_on/current_phase_closeout_01.md` still opened with a stale date even though it is a current closeout guide rather than a fixed historical memo.
- The `docs_researcher` sidecar found one additional drift class after the initial patch: front-door docs (`README.md`, `Documentation.md`, `docs/research_abstract/README.md`, `docs/hands_on/README.md`) still advertised a `next promoted line` / `next queue` that conflicted with the maintenance-only queue in `progress.md` / `tasks.md`.
- Local diff inspection after the follow-up patch found no semantics change, no validation-floor change, and no reopened implementation queue.

## What changed in understanding

- The next freshness risk after `plan/01` was not more storage wording but roadmap-role ambiguity across `plan/11` and its reader-facing mirrors.
- Reader-facing summaries can remain current without becoming false queue sources if they explicitly point back to `progress.md` / `tasks.md`.
- The network transport line is now stable across snapshot, roadmap memory, and reader-facing summary when described as:
  - reported `NET-01` parity anchor,
  - runnable `NET-02..05` helper-local canaries.
- Front-door landing pages can drift independently from roadmap memory even when `progress.md` / `tasks.md` are already correct; they need explicit cooling passes of their own.

## Open questions

- Should `README.md` / `Documentation.md` also gain an explicit one-line reminder that live queue authority remains in `progress.md` / `tasks.md`, or is that unnecessary given the current front-door split?
- Should `docs/hands_on/current_phase_closeout_01.md` eventually carry an explicit “this guide is refreshed in place” note, or is removing the stale date enough?

## Suggested next prompt

Continue with the next safe maintenance package: audit the remaining reader-facing landing docs for point-in-time wording drift beyond queue authority, with emphasis on date-stamped “current line” text and any stale closeout-time vocabulary that no longer matches `progress.md` / `tasks.md`.

## plan/progress/tasks/samples updates

- `plan/`: updated (`plan/11-roadmap-near-term.md`)
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: 更新不要

## Skipped validations and reasons

- No full sample/Cargo rerun is planned for this package because the edits are docs-only and do not change executable behavior.
- No fresh storage probe was run in this package because the package only adjusts roadmap and reader-facing wording, not storage guardrail state.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the post-edit docs-floor rerun and diff review.

## Sub-agent session close status

- `docs_researcher` sidecar `019ddd0f-c16e-7660-84a9-0875cdb682e4` returned concrete stale-wording findings for `plan/11`, front-door docs, and hands-on landing pages. Those findings were incorporated into the package patch, and the session was closed after incorporation.
