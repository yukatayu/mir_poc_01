# Report 2193 — Practical alpha-0.8 session hot-plug path portability

- Date: 2026-07-04 15:04 JST
- Author / agent: Codex
- Scope: Practical alpha-0.8 same-session hot-plug helper nested session argv
  portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha08_session_hotplug.py` pass repo-owned base
session and attach package roots to nested `mir_practical_alpha05_session`
commands through repo-relative `samples/...` arguments.

## Scope and assumptions

Scope:

- audit `practical_alpha08_session_hotplug.py check-all` and `closeout` output
  for repo-root absolute path drift
- add regression coverage for repo-relative nested session package argv
- preserve absolute paths for temporary session files
- preserve absolute arguments for paths outside the repository
- rerun alpha08 helper, focused tests, and relevant alpha08 / alpha05 / hotplug
  Cargo tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested `mir_practical_alpha05_session` Cargo example runs with
  `cwd=REPO_ROOT`, so repo-relative package-root arguments are portable and
  executable.
- Temporary session files are local temp artifacts, not repo-owned source
  inputs, so they intentionally remain absolute temp paths.
- The companion avatar helper call used by `OA08-09` remains a separate helper
  portability candidate.
- This package does not change hot-plug semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 55 started from clean `HEAD == origin/main == d16139cf` after the
practical alpha-0.9 devtools helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha08_session_hotplug.py`
- `scripts/tests/test_practical_alpha08_session_hotplug.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2192-practical-alpha09-devtools-path-portability.md`
- read-only code-mapper result from `Halley`, completed during Package 53

## Actions taken

- Ran `practical_alpha08_session_hotplug.py check-all` and confirmed emitted
  JSON had zero repo-root absolute matches.
- Ran `practical_alpha08_session_hotplug.py closeout` and confirmed emitted
  JSON had zero repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - `_run_session_start()` package argv
  - `_run_session_attach()` package argv
  - temporary session paths staying absolute
- Added `repo_cli_arg()` to `scripts/practical_alpha08_session_hotplug.py`.
- Converted `_run_session_start()` and `_run_session_attach()` package argv to
  use `repo_cli_arg()`.
- Left temporary session file paths absolute.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha08_session_hotplug.py`
- `scripts/tests/test_practical_alpha08_session_hotplug.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2193-practical-alpha08-session-hotplug-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json > /tmp/mirrorea-practical-alpha08-session-hotplug-p55-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha08-session-hotplug-p55-before.json`
- `python3 scripts/practical_alpha08_session_hotplug.py closeout --format json > /tmp/mirrorea-practical-alpha08-session-hotplug-p55-closeout-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha08-session-hotplug-p55-closeout-before.json`
- `sed -n '1,460p' scripts/practical_alpha08_session_hotplug.py`
- `sed -n '1,360p' scripts/tests/test_practical_alpha08_session_hotplug.py`
- `rg -n "subprocess\\.run|str\\(|REPO_ROOT|repo_cli_arg|_cargo_session|BASE_SESSION_PACKAGE|operation|package_path|attach_package|session_path|closeout|cargo" scripts/practical_alpha08_session_hotplug.py scripts/tests/test_practical_alpha08_session_hotplug.py`
- `python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug` (RED)
- same unit command after implementation
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json > /tmp/mirrorea-practical-alpha08-session-hotplug-p55-after.json`
- `python3 scripts/practical_alpha08_session_hotplug.py closeout --format json > /tmp/mirrorea-practical-alpha08-session-hotplug-p55-closeout-after.json`
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- endpoint scan over changed files

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 10
  - passed `["OA08-01", "OA08-02", "OA08-03", "OA08-04", "OA08-05", "OA08-06", "OA08-07", "OA08-08", "OA08-09", "OA08-10"]`
  - failed `[]`
  - operational_alpha08_ready `True`
  - repo-root absolute matches 0
- Initial `closeout` scan:
  - implemented_rows `["OA08-01", "OA08-02", "OA08-03", "OA08-04", "OA08-05", "OA08-06", "OA08-07", "OA08-08", "OA08-09", "OA08-10"]`
  - operational_alpha08_ready `True`
  - repo-root absolute matches 0
- RED unit run failed as expected:
  - `repo_cli_arg` did not exist
  - session start and attach package argv contained host absolute package paths
- `python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug`
  passed after implementation: 7 tests.
- Final `check-all` scan:
  - sample_count 10
  - passed `["OA08-01", "OA08-02", "OA08-03", "OA08-04", "OA08-05", "OA08-06", "OA08-07", "OA08-08", "OA08-09", "OA08-10"]`
  - failed `[]`
  - operational_alpha08_ready `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - same_session_hotplug_ready `True`
  - operational_alpha08_ready `True`
  - repo-root absolute matches 0
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
  passed: 3 tests.
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
  passed: 3 tests.
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  passed: 17 tests.
- Final focused unit rerun passed: 7 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1345 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed.
- Endpoint scan over changed files found no webhook endpoint matches.

## What changed in understanding

The alpha08 helper emitted clean success-path public JSON already. The
non-portable pieces were the nested session command package inputs for base
session start and attach operations. Temporary session files are intentionally
environment-local artifacts and are not converted to repo-relative paths.

The `OA08-09` fallback companion still calls `practical_alpha1_avatar.py`;
that helper remains a separate portability candidate.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audits remain for
  `practical_alpha1_avatar.py` and `practical_alpha1_save_load.py`, in that
  recommended order unless local evidence changes it.

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
- added a recent-log entry for practical alpha-0.8 same-session hot-plug helper
  path portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-0.8 same-session hot-plug helper path
  portability is now hardened for session start / attach package argv
- updated the remaining practical alpha helper portability candidate order

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the α-0.8 workflow row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all` and `closeout` JSON have zero repo-root
  absolute matches after the change.
- Confirmed tests cover path helper behavior, session start package argv,
  session attach package argv, and absolute temp session path retention.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 53.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical alpha-0.8 / alpha-0.5 / hotplug validation floor
  was rerun.
- Oracle was not used because the package was a narrow mechanical portability
  hardening step with direct local evidence.

## Commit / push status

Committed and pushed:

- `12c1fb39 Use relative alpha08 session hotplug paths`

This report section was updated after the first push and will be captured by a
report-only follow-up commit.

## Sub-agent session close status

No new sub-agent session was opened for this package. `Halley` had already
completed and was closed during Package 53.
