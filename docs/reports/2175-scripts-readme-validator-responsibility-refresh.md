# Report 2175 — Scripts README validator responsibility refresh

- Date: 2026-07-04 12:57 JST
- Author / agent: Codex
- Scope: scripts taxonomy documentation maintenance
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Update `scripts/README.md` so the `validate_docs.py` description matches its
current responsibility after the recent source-hierarchy, host-path, and
snapshot timestamp guard hardening packages.

## Scope and assumptions

Scope is documentation-only:

- `scripts/README.md`
- snapshot logs that track scripts taxonomy maintenance

Assumptions:

- `scripts/README.md` is the front-door taxonomy for script responsibilities.
- The validator implementation is already covered by its own tests and reports;
  this package only corrects reader-facing script taxonomy wording.

## Start state / dirty state

Package 37 started from clean `HEAD == origin/main == 8f213f52`.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Actions taken

- Compared `scripts/README.md` with current `scripts/validate_docs.py`
  responsibilities.
- Updated `scripts/README.md` to mention:
  - source-hierarchy wording lint
  - active reader-facing host-specific repo path lint
  - `progress.md` / `tasks.md` top `最終更新` freshness guard
  - current docs/lint target families at a high level
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` as maintenance
  snapshot entries.

## Files changed

- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2175-scripts-readme-validator-responsibility-refresh.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py` passed and found 1327 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed with required/present
  602/602.
- `git diff --check` passed.

## What changed in understanding

`validate_docs.py` is no longer just a scaffold check. It is still not a
semantic validator or sample runner, but it now owns several reader-facing drift
guards that `scripts/README.md` should name explicitly.

## Open questions

- Whether `scripts/README.md` should eventually split validator responsibilities
  into a compact table remains open. The current update keeps the existing
  prose style.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`, prioritizing small
reader-facing taxonomy corrections when validator or script responsibilities
change.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, or repository-memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for the `scripts/README.md` validator responsibility
  refresh and updated the top `最終更新` timestamp.

## tasks.md update status

`tasks.md` 更新済み:

- Added `scripts/README.md` to the docs freshness audit row and mirrored the
  expanded validator responsibility in the current holding-state text.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a maintenance validation log row. Sample workflow status did not change.

## Reviewer findings and follow-up

Reviewer not launched for this narrow docs taxonomy package unless validation
finds a non-obvious issue.

## Skipped validations and reasons

No executable sample validation is required for this docs-only taxonomy update.
Docs validators and whitespace checks are run before commit.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent has been launched for this package.
