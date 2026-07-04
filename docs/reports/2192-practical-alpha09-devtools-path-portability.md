# Report 2192 — Practical alpha-0.9 devtools path portability

- Date: 2026-07-04 14:59 JST
- Author / agent: Codex
- Scope: Practical alpha-0.9 session-bound devtools helper nested session argv
  portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha09_devtools.py` pass repo-owned base session,
host-I/O, and attach package roots to nested `mir_practical_alpha05_session`
commands through repo-relative `samples/...` arguments.

## Scope and assumptions

Scope:

- audit `practical_alpha09_devtools.py check-all`, `closeout`, and
  `render-html` output for repo-root absolute path drift
- add regression coverage for repo-relative nested session package argv
- preserve absolute paths for temporary session files and rendered temp HTML
  output
- preserve absolute arguments for paths outside the repository
- rerun alpha09 helper, focused tests, and relevant alpha09 / alpha08 Cargo
  tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested `mir_practical_alpha05_session` Cargo example runs with
  `cwd=REPO_ROOT`, so repo-relative package-root arguments are portable and
  executable.
- Temporary session files and the default `render-html` output are local temp
  artifacts, not repo-owned source inputs, so they intentionally remain
  absolute temp paths.
- This package does not change devtools semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 54 started from clean `HEAD == origin/main == 0a372991` after the
practical alpha-1 attach helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha09_devtools.py`
- `scripts/tests/test_practical_alpha09_devtools.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2191-practical-alpha1-attach-path-portability.md`
- read-only code-mapper result from `Halley`, completed during Package 53

## Actions taken

- Ran `practical_alpha09_devtools.py check-all` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Ran `practical_alpha09_devtools.py closeout` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - all package args sent from `build_session_devtools_payload()` to
    `_cargo_session()`
  - temporary session paths staying absolute
- Added `repo_cli_arg()` to `scripts/practical_alpha09_devtools.py`.
- Converted the base session package, host-I/O package, and attach sequence
  package argv to use `repo_cli_arg()`.
- Left temporary session file paths and `render-html` temp output paths
  absolute.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha09_devtools.py`
- `scripts/tests/test_practical_alpha09_devtools.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2192-practical-alpha09-devtools-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha09_devtools.py check-all --format json > /tmp/mirrorea-practical-alpha09-devtools-p54-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha09-devtools-p54-before.json`
- `python3 scripts/practical_alpha09_devtools.py closeout --format json > /tmp/mirrorea-practical-alpha09-devtools-p54-closeout-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha09-devtools-p54-closeout-before.json`
- `sed -n '1,440p' scripts/practical_alpha09_devtools.py`
- `sed -n '1,340p' scripts/tests/test_practical_alpha09_devtools.py`
- `rg -n "subprocess\\.run|str\\(|REPO_ROOT|repo_cli_arg|_cargo_session|BASE_SESSION_PACKAGE|HOST_IO_PACKAGE|ATTACH_SEQUENCE|render-html|html_path|session_path|package" scripts/practical_alpha09_devtools.py scripts/tests/test_practical_alpha09_devtools.py`
- `python3 -m unittest scripts.tests.test_practical_alpha09_devtools` (RED)
- same unit command after implementation
- `python3 scripts/practical_alpha09_devtools.py check-all --format json > /tmp/mirrorea-practical-alpha09-devtools-p54-after.json`
- `python3 scripts/practical_alpha09_devtools.py closeout --format json > /tmp/mirrorea-practical-alpha09-devtools-p54-closeout-after.json`
- `python3 scripts/practical_alpha09_devtools.py render-html --format json > /tmp/mirrorea-practical-alpha09-devtools-p54-render-html.json`
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_practical_alpha09_devtools`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- endpoint scan over changed files

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 9
  - passed `["OA09-01", "OA09-02", "OA09-03", "OA09-04", "OA09-05", "OA09-06", "OA09-07", "OA09-08", "OA09-09"]`
  - failed `[]`
  - repo-root absolute matches 0
- Initial `closeout` scan:
  - implemented_rows `["OA09-01", "OA09-02", "OA09-03", "OA09-04", "OA09-05", "OA09-06", "OA09-07", "OA09-08", "OA09-09"]`
  - repo-root absolute matches 0
- RED unit run failed as expected:
  - `repo_cli_arg` did not exist
  - base session package argv contained a host absolute package path
- `python3 -m unittest scripts.tests.test_practical_alpha09_devtools` passed
  after implementation: 7 tests.
- Final `check-all` scan:
  - sample_count 9
  - passed `["OA09-01", "OA09-02", "OA09-03", "OA09-04", "OA09-05", "OA09-06", "OA09-07", "OA09-08", "OA09-09"]`
  - failed `[]`
  - operational_alpha09_ready `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - session_bound_devtools_ready `True`
  - operational_alpha09_ready `True`
  - repo-root absolute matches 0
- `render-html` scan:
  - sample_id `OA09-09`
  - viewer_mode `nonfinal_static_html_session_viewer`
  - html_path was an absolute `/tmp/...` file path
  - rendered HTML included `retention_on_demand_trace`
  - repo-root absolute matches 0
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
  passed: 3 tests.
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
  passed: 3 tests.
- Final focused unit rerun passed: 7 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1344 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed.
- Endpoint scan over changed files found no webhook endpoint matches.

## What changed in understanding

The alpha09 helper emitted clean success-path public JSON already. The
non-portable pieces were the nested session command package inputs for the base
session, host-I/O row, and attach sequence. Temporary session files and rendered
HTML output are intentionally environment-local artifacts and are not converted
to repo-relative paths.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audits remain for
  `practical_alpha08_session_hotplug.py`, `practical_alpha1_avatar.py`, and
  `practical_alpha1_save_load.py`, in that recommended order unless local
  evidence changes it.

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
- added a recent-log entry for practical alpha-0.9 devtools helper path
  portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-0.9 devtools helper path portability is now
  hardened
- updated the remaining practical alpha helper portability candidate order

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the α-0.9 workflow row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all`, `closeout`, and `render-html` JSON have zero
  repo-root absolute matches after the change.
- Confirmed tests cover path helper behavior, all nested package argv families,
  and absolute temp session path retention.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 53.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical alpha-0.9 / alpha-0.8 devtools validation floor
  was rerun.
- Oracle was not used because the package was a narrow mechanical portability
  hardening step with direct local evidence.

## Commit / push status

Pending at report creation time.

## Sub-agent session close status

No new sub-agent session was opened for this package. `Halley` had already
completed and was closed during Package 53.
