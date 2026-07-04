# Report 2189 — Practical alpha-1 checker path portability

- Date: 2026-07-04 14:39 JST
- Author / agent: Codex
- Scope: Practical alpha-1 checker helper subprocess path portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha1_check.py` pass repo-owned practical package roots
to its nested checker Cargo example through repo-relative `samples/...`
arguments.

## Scope and assumptions

Scope:

- audit `practical_alpha1_check.py check-all` output for repo-root absolute path
  drift
- add regression coverage for repo-relative checker subprocess package argv
- preserve absolute arguments for paths outside the repository
- rerun checker helper, focused tests, and checker Cargo tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested checker Cargo example runs with `cwd=REPO_ROOT`, so repo-relative
  package-root arguments are portable and executable.
- This package does not change checker semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 51 started from clean `HEAD == origin/main == e03a6fd2` after the
practical alpha-1 transport helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha1_check.py`
- `scripts/tests/test_practical_alpha1_check.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2188-practical-alpha1-transport-path-portability.md`

## Actions taken

- Ran `practical_alpha1_check.py check-all` and confirmed emitted JSON had zero
  repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - checker Cargo example package argv
- Added `repo_cli_arg()` to `scripts/practical_alpha1_check.py`.
- Converted the checker Cargo example package argv and JSON decode failure
  message to use `repo_cli_arg()`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha1_check.py`
- `scripts/tests/test_practical_alpha1_check.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2189-practical-alpha1-checker-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha1_check.py check-all --format json > /tmp/mirrorea-practical-alpha1-check-p51-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-check-p51-before.json`
- `sed -n '1,240p' scripts/practical_alpha1_check.py`
- `sed -n '1,220p' scripts/tests/test_practical_alpha1_check.py`
- `python3 -m unittest scripts.tests.test_practical_alpha1_check.PracticalAlpha1CheckTests.test_repo_cli_arg_uses_repo_relative_paths_for_package_dirs scripts.tests.test_practical_alpha1_check.PracticalAlpha1CheckTests.test_repo_cli_arg_keeps_external_paths_absolute scripts.tests.test_practical_alpha1_check.PracticalAlpha1CheckTests.test_build_check_report_uses_repo_relative_package_path` (RED)
- same focused unit command after implementation
- `python3 -m unittest scripts.tests.test_practical_alpha1_check`
- `python3 scripts/practical_alpha1_check.py check-all --format json > /tmp/mirrorea-practical-alpha1-check-p51-final.json`
- `python3 scripts/practical_alpha1_check.py closeout --format json > /tmp/mirrorea-practical-alpha1-check-p51-closeout.json`
- `cargo test -p mir-ast practical_alpha1_checker -- --nocapture`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `date '+%Y-%m-%d %H:%M %Z'`

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 10
  - passed 10
  - failed `[]`
  - repo-root absolute matches 0
- RED focused tests failed as expected:
  - `repo_cli_arg` did not exist
  - checker Cargo example argv contained host absolute package path
- Focused tests passed after implementation: 3 tests.
- `python3 -m unittest scripts.tests.test_practical_alpha1_check` passed:
  10 tests.
- Final `check-all` scan:
  - sample_count 10
  - passed 10
  - failed `[]`
  - first_checker_floor_complete `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - first_checker_floor_complete `True`
  - repo-root absolute matches 0
- `cargo test -p mir-ast practical_alpha1_checker -- --nocapture` passed:
  the focused `practical_alpha1_checker` tests ran 3 tests and passed.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1341 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed after report creation.

## What changed in understanding

The checker helper emitted clean JSON already. The non-portable piece was the
internal checker Cargo example invocation, which passed repo-owned package roots
as host absolute paths.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audits remain for
  `practical_alpha1_run_local.py`, `practical_alpha1_attach.py`,
  `practical_alpha1_avatar.py`, `practical_alpha1_save_load.py`,
  `practical_alpha08_session_hotplug.py`, and `practical_alpha09_devtools.py`.

## Suggested next prompt

Continue autonomous maintenance with the next practical alpha helper
path-portability audit.

## Plan update status

`plan/` 更新不要:

- This package did not change roadmap, semantics, source-traceability,
  open-question, or repository-memory decisions.

## Documentation.md update status

`Documentation.md` 更新不要:

- No top-level reader-facing status or source hierarchy changed.

## progress.md update status

Updated:

- advanced the top `最終更新` timestamp
- added a recent-log entry for practical alpha-1 checker helper path portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-1 checker helper path portability is now
  hardened

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the `CHK-*` row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all` and `closeout` JSON have zero repo-root absolute
  matches after the change.
- Confirmed tests cover path helper behavior and the checker Cargo example argv.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 49.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical alpha-1 checker helper and Cargo checker tests
  were rerun.

## Commit / push status

Committed and pushed:

- `af4a5066 Use relative practical checker helper paths`

This report section was updated after the first push and will be captured by a
report-only follow-up commit.

## Sub-agent session close status

No new sub-agent session was opened for this package.
