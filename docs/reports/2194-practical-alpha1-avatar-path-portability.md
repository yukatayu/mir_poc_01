# Report 2194 — Practical alpha-1 avatar path portability

- Date: 2026-07-04 15:09 JST
- Author / agent: Codex
- Scope: Practical alpha-1 avatar helper subprocess path portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha1_avatar.py` pass repo-owned practical avatar
package roots to its nested avatar Cargo example through repo-relative
`samples/...` arguments.

## Scope and assumptions

Scope:

- audit `practical_alpha1_avatar.py check-all` and `closeout` output for
  repo-root absolute path drift
- add regression coverage for repo-relative avatar subprocess package argv
- preserve absolute arguments for paths outside the repository
- rerun avatar helper, focused tests, and relevant practical avatar Cargo tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested avatar Cargo example runs with `cwd=REPO_ROOT`, so repo-relative
  package-root arguments are portable and executable.
- This package does not change avatar preview semantics, sample status,
  workflow status, ABI, or canon claims.

## Start state / dirty state

Package 56 started from clean `HEAD == origin/main == 4cb8b0c6` after the
practical alpha-0.8 session hot-plug helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha1_avatar.py`
- `scripts/tests/test_practical_alpha1_avatar.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2193-practical-alpha08-session-hotplug-path-portability.md`
- read-only code-mapper result from `Halley`, completed during Package 53

## Actions taken

- Ran `practical_alpha1_avatar.py check-all` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Ran `practical_alpha1_avatar.py closeout` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - avatar Cargo example package argv
- Added `repo_cli_arg()` to `scripts/practical_alpha1_avatar.py`.
- Converted the avatar Cargo example package argv and JSON decode failure
  message to use `repo_cli_arg()`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha1_avatar.py`
- `scripts/tests/test_practical_alpha1_avatar.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2194-practical-alpha1-avatar-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha1_avatar.py check-all --format json > /tmp/mirrorea-practical-alpha1-avatar-p56-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-avatar-p56-before.json`
- `python3 scripts/practical_alpha1_avatar.py closeout --format json > /tmp/mirrorea-practical-alpha1-avatar-p56-closeout-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-avatar-p56-closeout-before.json`
- `sed -n '1,300p' scripts/practical_alpha1_avatar.py`
- `sed -n '1,260p' scripts/tests/test_practical_alpha1_avatar.py`
- `rg -n "subprocess\\.run|str\\(|REPO_ROOT|repo_cli_arg|package_path|package_dir|_build_avatar_report|cargo|closeout" scripts/practical_alpha1_avatar.py scripts/tests/test_practical_alpha1_avatar.py`
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar` (RED)
- same unit command after implementation
- `python3 scripts/practical_alpha1_avatar.py check-all --format json > /tmp/mirrorea-practical-alpha1-avatar-p56-after.json`
- `python3 scripts/practical_alpha1_avatar.py closeout --format json > /tmp/mirrorea-practical-alpha1-avatar-p56-closeout-after.json`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_avatar -- --nocapture`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- endpoint scan over changed files

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 3
  - passed `["AV-A1-01", "AV-A1-02", "AV-A1-03"]`
  - failed `[]`
  - avatar_preview_first_floor_complete `True`
  - repo-root absolute matches 0
- Initial `closeout` scan:
  - implemented_rows `["AV-A1-01", "AV-A1-02", "AV-A1-03"]`
  - avatar_preview_first_floor_complete `True`
  - repo-root absolute matches 0
- RED unit run failed as expected:
  - `repo_cli_arg` did not exist
  - avatar Cargo example argv contained a host absolute package path
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar` passed
  after implementation: 9 tests.
- Final `check-all` scan:
  - sample_count 3
  - passed `["AV-A1-01", "AV-A1-02", "AV-A1-03"]`
  - failed `[]`
  - avatar_preview_first_floor_complete `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - implemented_rows `["AV-A1-01", "AV-A1-02", "AV-A1-03"]`
  - avatar_preview_first_floor_complete `True`
  - repo-root absolute matches 0
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
  passed: 10 tests.
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  passed: 17 tests.
- `cargo test -p mir-runtime --test practical_alpha1_avatar -- --nocapture`
  passed: 3 tests.
- Final focused unit rerun passed: 9 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1346 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed.
- Endpoint scan over changed files found no webhook endpoint matches.

## What changed in understanding

The avatar helper emitted clean success-path public JSON already. The
non-portable piece was the internal avatar Cargo example invocation, which
passed repo-owned package roots as host absolute paths.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audit remains for
  `practical_alpha1_save_load.py`, specifically the runtime save/load branch.

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
- added a recent-log entry for practical alpha-1 avatar helper path
  portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-1 avatar helper path portability is now
  hardened
- updated the remaining practical alpha helper portability candidate order

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the `AV-A1-*` row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all` and `closeout` JSON have zero repo-root
  absolute matches after the change.
- Confirmed tests cover path helper behavior and the avatar Cargo example argv.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 53.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical avatar validation floor was rerun.
- Oracle was not used because the package was a narrow mechanical portability
  hardening step with direct local evidence.

## Commit / push status

Pending at report creation time.

## Sub-agent session close status

No new sub-agent session was opened for this package. `Halley` had already
completed and was closed during Package 53.
