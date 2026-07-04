# Report 2179 — Numbered plan cross-list guard

- Date: 2026-07-04 13:23 JST
- Author / agent: Codex
- Scope: validator test maintenance
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Add focused unit-test coverage so numbered plan files cannot drift between
`scripts/validate_docs.py`, `scripts/check_source_hierarchy.py`, and the actual
`plan/` directory.

## Scope and assumptions

Scope:

- `scripts/tests/test_validate_docs.py`
- snapshot logs that track validator maintenance

Assumptions:

- The explicit plan lists in `validate_docs.REQUIRED` and
  `check_source_hierarchy.REQUIRED_PATHS["plan"]` should cover the same current
  numbered `plan/*.md` repository-memory set.
- This package adds test coverage only. It does not change validator runtime
  behavior or any project semantics.

## Start state / dirty state

Package 41 started from clean `HEAD == origin/main == a435d43d`.

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
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Checked existing `test_validate_docs` coverage for generic required-list /
  source-hierarchy plan drift.
- Added `test_numbered_plan_required_scaffold_matches_source_hierarchy`.
- Added `test_all_repo_numbered_plan_files_are_registered`.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` as maintenance
  snapshot entries.

## Files changed

- `scripts/tests/test_validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2179-numbered-plan-cross-list-guard.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs` passed: 34 tests.
- `python3 scripts/validate_docs.py` passed and found 1331 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed with required/present
  659/659.
- `git diff --check` passed.

## What changed in understanding

After Package 40, runtime validation catches unregistered numbered plan files,
but a unit test can also catch mismatch between the docs validator list,
source-hierarchy list, and actual `plan/` directory before running the full
validator command.

## Open questions

None for this package.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`, looking for adjacent required
scaffold lists that should have the same cross-list drift protection.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, open-question, source-traceability, or repository
  memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for the numbered plan cross-list guard and updated
  the top `最終更新` timestamp.

## tasks.md update status

`tasks.md` 更新済み:

- Added the numbered plan cross-list guard to the current holding-state
  maintenance notes and updated the top `最終更新` timestamp.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a maintenance validation log row. Sample workflow status did not
  change.

## Reviewer findings and follow-up

No sub-agent reviewer was launched for this narrow unit-test maintenance
package.

## Skipped validations and reasons

No executable sample validation is required for this test-only docs-maintenance
update. Validator unit tests, docs validation, source-hierarchy check, and
whitespace checks are run before commit.

## Commit / push status

Commit and push pending at this report update step.

## Sub-agent session close status

No sub-agent has been launched for this package.
