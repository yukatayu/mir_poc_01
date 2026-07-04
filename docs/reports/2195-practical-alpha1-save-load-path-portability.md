# Report 2195 — Practical alpha-1 save/load path portability

- Date: 2026-07-04 15:15 JST
- Author / agent: Codex
- Scope: Practical alpha-1 save/load helper runtime branch subprocess path
  portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha1_save_load.py` pass repo-owned runtime-backed
save/load package roots to its nested save-load Cargo example through
repo-relative `samples/...` arguments, while preserving the checker-backed
preflight branch delegation.

## Scope and assumptions

Scope:

- audit `practical_alpha1_save_load.py check-all` and `closeout` output for
  repo-root absolute path drift
- add regression coverage for repo-relative runtime save/load subprocess
  package argv
- preserve absolute arguments for paths outside the repository
- confirm the `SL-A1-03` checker preflight branch continues to delegate through
  `practical_alpha1_check.py`
- rerun save/load helper, focused tests, validation-floor runner commands, and
  relevant save/load Cargo tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested save/load Cargo example runs with `cwd=REPO_ROOT`, so
  repo-relative package-root arguments are portable and executable.
- The checker-backed preflight branch is already covered by
  `practical_alpha1_check.py` path hardening and should not be routed through
  the runtime save/load subprocess.
- This package does not change save/load semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 57 started from clean `HEAD == origin/main == ddb69148` after the
practical alpha-1 avatar helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha1_save_load.py`
- `scripts/tests/test_practical_alpha1_save_load.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2194-practical-alpha1-avatar-path-portability.md`
- read-only code-mapper result from `Halley`, completed during Package 53

## Actions taken

- Ran `practical_alpha1_save_load.py check-all` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Ran `practical_alpha1_save_load.py closeout` and confirmed emitted JSON had
  zero repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - runtime save/load Cargo example package argv
  - checker preflight branch delegation without runtime subprocess execution
- Added `repo_cli_arg()` to `scripts/practical_alpha1_save_load.py`.
- Converted the runtime save/load Cargo example package argv and JSON decode
  failure message to use `repo_cli_arg()`.
- Left the checker-backed `SL-A1-03` preflight branch delegated through
  `practical_alpha1_check.py`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha1_save_load.py`
- `scripts/tests/test_practical_alpha1_save_load.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2195-practical-alpha1-save-load-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha1_save_load.py check-all --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-save-load-p57-before.json`
- `python3 scripts/practical_alpha1_save_load.py closeout --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-closeout-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-save-load-p57-closeout-before.json`
- `sed -n '1,360p' scripts/practical_alpha1_save_load.py`
- `sed -n '1,300p' scripts/tests/test_practical_alpha1_save_load.py`
- `rg -n "subprocess\\.run|str\\(|REPO_ROOT|repo_cli_arg|package_path|package_dir|_build_runtime_save_load_report|_checker_preflight_report|check_path|cargo|closeout" scripts/practical_alpha1_save_load.py scripts/tests/test_practical_alpha1_save_load.py`
- `python3 -m unittest scripts.tests.test_practical_alpha1_save_load` (RED)
- same unit command after implementation
- `python3 scripts/practical_alpha1_save_load.py check-all --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-after.json`
- `python3 scripts/practical_alpha1_save_load.py closeout --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-closeout-after.json`
- `python3 scripts/practical_alpha1_check.py run CHK-CUT-01 --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-chk-cut.json`
- `python3 scripts/practical_alpha1_save_load.py run SL-A1-01 --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-sl01.json`
- `python3 scripts/practical_alpha1_save_load.py run SL-A1-02 --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-sl02.json`
- `python3 scripts/practical_alpha1_save_load.py run SL-A1-03 --format json > /tmp/mirrorea-practical-alpha1-save-load-p57-sl03.json`
- `cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_practical_alpha1_save_load`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- endpoint scan over changed files

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 3
  - passed `["SL-A1-01", "SL-A1-02", "SL-A1-03"]`
  - failed `[]`
  - local_save_load_first_floor_complete `True`
  - repo-root absolute matches 0
- Initial `closeout` scan:
  - implemented_rows `["SL-A1-01", "SL-A1-02", "SL-A1-03"]`
  - local_save_load_first_floor_complete `True`
  - repo-root absolute matches 0
- RED unit run failed as expected:
  - `repo_cli_arg` did not exist
  - runtime save/load Cargo example argv contained a host absolute package path
  - checker preflight branch delegation test already passed
- `python3 -m unittest scripts.tests.test_practical_alpha1_save_load` passed
  after implementation: 8 tests.
- Final `check-all` scan:
  - sample_count 3
  - passed `["SL-A1-01", "SL-A1-02", "SL-A1-03"]`
  - failed `[]`
  - local_save_load_first_floor_complete `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - implemented_rows `["SL-A1-01", "SL-A1-02", "SL-A1-03"]`
  - local_save_load_first_floor_complete `True`
  - repo-root absolute matches 0
- Validation floor runner commands:
  - `CHK-CUT-01` reported verdict `rejected`, repo-root absolute matches 0
  - `SL-A1-01` reported terminal_outcome `accepted`, repo-root absolute matches 0
  - `SL-A1-02` reported terminal_outcome `rejected`, repo-root absolute matches 0
  - `SL-A1-03` reported terminal_outcome
    `rejected_invalid_distributed_cut_preflight`, repo-root absolute matches 0
- `cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture`
  passed: 4 tests.
- `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture`
  passed: 4 tests.
- Final focused unit rerun passed: 8 tests.
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1347 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed.
- Endpoint scan over changed files found no webhook endpoint matches.

## What changed in understanding

The save/load helper emitted clean success-path public JSON already. The
non-portable piece was the internal runtime save/load Cargo example invocation
for runtime-backed `SL-A1-01/02`. The checker-backed `SL-A1-03` branch already
delegates through the hardened checker helper and does not call the runtime
save/load subprocess.

The focused practical helper portability candidate list from the code-mapper
audit is now closed.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Run a broader repo/path-portability audit if continuing this maintenance line,
  because the focused practical helper candidate list is now exhausted.

## Suggested next prompt

Continue autonomous maintenance with a broader path-portability audit or the
next current task-map package.

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
- added a recent-log entry for practical alpha-1 save/load helper path
  portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-1 save/load helper path portability is now
  hardened
- recorded that the focused practical helper candidate list is now closed

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the `SL-A1-*` row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all`, `closeout`, and validation floor runner JSON
  have zero repo-root absolute matches after the change.
- Confirmed tests cover path helper behavior, runtime save/load Cargo example
  argv, and checker preflight delegation.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 53.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical save/load helper commands and Cargo save/load
  tests were rerun.
- Oracle was not used because the package was a narrow mechanical portability
  hardening step with direct local evidence.

## Commit / push status

Pending at report creation time.

## Sub-agent session close status

No new sub-agent session was opened for this package. `Halley` had already
completed and was closed during Package 53.
