# Report 2178 — Numbered plan registration guard

- Date: 2026-07-04 13:17 JST
- Author / agent: Codex
- Scope: documentation validator drift-prevention guard
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Prevent a repeat of the `plan/97..119` required-scaffold drift by making
`scripts/validate_docs.py` reject existing numbered `plan/*.md` files that are
not registered in its explicit `REQUIRED` scaffold list.

## Scope and assumptions

Scope:

- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`
- snapshot logs that track validator maintenance

Assumptions:

- The explicit `REQUIRED` / source-hierarchy lists remain useful because they
  detect accidental deletion of plan files.
- A separate registration guard is the safer small step: it detects newly added
  numbered plan files that are not yet listed without replacing the explicit
  list with dynamic discovery.
- This guard is about repository structure and documentation maintenance, not
  about semantic correctness of plan contents.

## Start state / dirty state

Package 40 started from clean `HEAD == origin/main == f6a5449e`.

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
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Added a failing regression test with an extra unregistered
  `plan/120-unregistered-plan.md` fixture under a temporary repository root.
- Confirmed the test failed before implementation because validation proceeded
  to report-template checks instead of rejecting the unregistered plan file.
- Added `unregistered_numbered_plan_files()` to `scripts/validate_docs.py`.
- Made `validate_docs.main()` reject unregistered numbered plan files after
  required-file existence checks and before the later document/content checks.
- Ran repo-local validation and found existing historical `plan/02..38` files
  were not registered in the explicit required lists.
- Expanded both `scripts/validate_docs.py` and `scripts/check_source_hierarchy.py`
  so current numbered plan memory is covered through `plan/00..119`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md` for the expanded validator responsibility.

## Files changed

- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2178-numbered-plan-registration-guard.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_unregistered_numbered_plan_file`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs` passed: 32 tests.
- `python3 scripts/validate_docs.py` passed and found 1330 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed with required/present
  659/659.
- `git diff --check` passed.

TDD RED evidence:

- The targeted test failed because the expected
  `Numbered plan files are not registered` message was absent; validation
  instead reached the latest report heading check.

TDD GREEN evidence:

- The targeted test passed after adding the numbered plan registration guard.

Repo validation follow-up evidence:

- The first full `python3 scripts/validate_docs.py` run failed because existing
  historical `plan/02..38` files were not registered in `REQUIRED`. This showed
  the new guard was detecting a real broader registration gap, not only future
  additions.
- The explicit docs and source-hierarchy plan lists were therefore expanded to
  cover current numbered `plan/00..119`.

## What changed in understanding

The explicit `REQUIRED` / source-hierarchy lists should remain deletion guards,
but `validate_docs.py` also needs an addition guard so numbered plan files
cannot silently bypass the scaffold list. Applying that guard exposed that
older `plan/02..38` repository memory was also outside the current explicit
guard set.

## Open questions

- Whether `check_source_hierarchy.py` should eventually share the same
  registered plan list helper remains open. Current coverage is held by
  explicit lists plus `test_validate_docs` cross-checks.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`, looking for adjacent validator
or index drift that can be caught with a small focused regression.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, open-question, source-traceability, or repository
  memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for the numbered plan registration guard and updated
  the top `最終更新` timestamp.

## tasks.md update status

`tasks.md` 更新済み:

- Added the numbered plan registration guard to the current holding-state
  maintenance notes and updated the top `最終更新` timestamp.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a maintenance validation log row. Sample workflow status did not
  change.

## Reviewer findings and follow-up

No sub-agent reviewer was launched for this narrow validator-regression package.
The change is covered by TDD red/green and focused validation.

## Skipped validations and reasons

No executable sample validation is required for this docs-validator maintenance
update. Docs validator unit tests, docs validation, source-hierarchy check, and
whitespace checks are run before commit.

## Commit / push status

Committed and pushed:

- `a26aa868 Guard numbered plan scaffold registration`

This report section was then updated for commit-record accuracy.

## Sub-agent session close status

No sub-agent has been launched for this package.
