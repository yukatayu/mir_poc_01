# Report 2176 — Scripts README source-hierarchy taxonomy refresh

- Date: 2026-07-04 13:03 JST
- Author / agent: Codex
- Scope: scripts taxonomy documentation maintenance
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Update `scripts/README.md` so the `check_source_hierarchy.py` description
matches the current structural-presence guard and no longer implies that the
plan coverage stops at `plan/39..86`.

## Scope and assumptions

Scope is documentation-only:

- `scripts/README.md`
- snapshot logs that track scripts taxonomy maintenance

Assumptions:

- `scripts/README.md` is the front-door taxonomy for script responsibilities.
- `check_source_hierarchy.py` currently checks structural presence only. It
  does not validate stale wording, normative consistency, report template
  completeness, or executable sample behavior.
- The `check_source_hierarchy.py` structural target set is intentionally
  narrower than the broader `validate_docs.py` scaffold / lint target set.

## Start state / dirty state

Package 38 started from clean `HEAD == origin/main == 757569e9`.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`

## Actions taken

- Compared `scripts/README.md` with current `scripts/check_source_hierarchy.py`
  target paths.
- Updated the `check_source_hierarchy.py` taxonomy text to name `plan/39..96`,
  Product Alpha demo entry files, `docs/hands_on/`, `docs/research_abstract/`,
  and `sub-agent-pro/operational-product-sample-001/`.
- Kept the stated responsibility limited to structural presence and did not
  merge it with `validate_docs.py` semantic-adjacent lint responsibilities.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` as maintenance
  snapshot entries.

## Files changed

- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2176-scripts-readme-source-hierarchy-taxonomy-refresh.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py` passed and found 1328 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed with required/present
  602/602.
- `git diff --check` passed.

## What changed in understanding

`check_source_hierarchy.py` and `validate_docs.py` are adjacent but distinct
front-door checks. The former remains a structural presence guard; the latter
also owns reader-facing wording, active host-path, snapshot timestamp, and
report-shape guards.

## Open questions

None for this package.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`, keeping script taxonomy
wording synchronized with the actual validator/helper boundaries when those
scripts change.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, open-question, source-traceability, or repository
  memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for the `scripts/README.md` source-hierarchy
  structural-check responsibility refresh and updated the top `最終更新`
  timestamp.

## tasks.md update status

`tasks.md` 更新済み:

- Mirrored the `check_source_hierarchy.py` responsibility refresh in the
  current holding-state maintenance notes and updated the top `最終更新`
  timestamp.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a maintenance validation log row. Sample workflow status did not
  change.

## Reviewer findings and follow-up

Reviewer not launched for this narrow docs taxonomy package unless validation
finds a non-obvious issue.

## Skipped validations and reasons

No executable sample validation is required for this docs-only taxonomy update.
Docs validators and whitespace checks are run before commit.

## Commit / push status

Commit and push pending at this report update step.

## Sub-agent session close status

No sub-agent has been launched for this package.
