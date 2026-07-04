# Report 2190 — Practical alpha-1 run-local path portability

- Date: 2026-07-04 14:47 JST
- Author / agent: Codex
- Scope: Practical alpha-1 run-local helper subprocess path portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/practical_alpha1_run_local.py` pass repo-owned practical package
roots to its nested local-runtime Cargo example through repo-relative
`samples/...` arguments.

## Scope and assumptions

Scope:

- audit `practical_alpha1_run_local.py check-all` output for repo-root absolute
  path drift
- add regression coverage for repo-relative local-runtime subprocess package
  argv
- preserve absolute arguments for paths outside the repository
- rerun run-local helper, focused tests, and relevant practical runtime Cargo
  tests
- update snapshot docs and report the outcome

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested local-runtime Cargo example runs with `cwd=REPO_ROOT`, so
  repo-relative package-root arguments are portable and executable.
- This package does not change runtime semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 52 started from clean `HEAD == origin/main == 2e905b52` after the
practical alpha-1 checker helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha1_run_local.py`
- `scripts/tests/test_practical_alpha1_run_local.py`
- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_transport.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2189-practical-alpha1-checker-path-portability.md`

## Actions taken

- Confirmed the existing `practical_alpha1_run_local.py check-all` JSON had
  zero repo-root absolute matches.
- Added RED tests for:
  - repo-owned package-dir conversion
  - external absolute fallback
  - local-runtime Cargo example package argv
- Added `repo_cli_arg()` to `scripts/practical_alpha1_run_local.py`.
- Converted the local-runtime Cargo example package argv and JSON decode
  failure message to use `repo_cli_arg()`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha1_run_local.py`
- `scripts/tests/test_practical_alpha1_run_local.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2190-practical-alpha1-run-local-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json > /tmp/mirrorea-practical-alpha1-run-local-p52-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-run-local-p52-before.json`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json | python3 - <<'PY' ... PY` (diagnostic attempt; failed because the here-doc consumed Python stdin instead of the pipe)
- `sed -n '1,260p' scripts/practical_alpha1_run_local.py`
- `sed -n '1,260p' scripts/tests/test_practical_alpha1_run_local.py`
- `rg -n "subprocess\\.run|str\\(package|package_path|REPO_ROOT|repo_cli_arg|_build_runtime_report|cargo|closeout" scripts/practical_alpha1_run_local.py scripts/tests/test_practical_alpha1_run_local.py`
- `rg -n "def repo_cli_arg|repo_cli_arg\\(" scripts/practical_alpha1_check.py scripts/practical_alpha1_transport.py scripts/mir_computational_samples.py scripts/surface_mir_samples.py`
- `sed -n '1,160p' scripts/practical_alpha1_check.py`
- `sed -n '1,140p' scripts/practical_alpha1_transport.py`
- `python3 -m unittest scripts.tests.test_practical_alpha1_run_local` (RED)
- same unit command after implementation
- `python3 scripts/practical_alpha1_run_local.py check-all --format json > /tmp/mirrorea-practical-alpha1-run-local-p52-after.json`
- `python3 scripts/practical_alpha1_run_local.py closeout --format json > /tmp/mirrorea-practical-alpha1-run-local-p52-closeout.json`
- `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
- `cargo test -p mir-runtime --test alpha_local_runtime`
- `date '+%Y-%m-%d %H:%M %Z'`

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 4
  - passed `["RUN-01", "RUN-02", "RUN-03", "RUN-04"]`
  - failed `[]`
  - repo-root absolute matches 0
- RED unit run failed as expected:
  - `repo_cli_arg` did not exist
  - local-runtime Cargo example argv contained a host absolute package path
- `python3 -m unittest scripts.tests.test_practical_alpha1_run_local` passed
  after implementation: 12 tests.
- Final `check-all` scan:
  - sample_count 4
  - passed `["RUN-01", "RUN-02", "RUN-03", "RUN-04"]`
  - failed `[]`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - implemented_rows `["RUN-01", "RUN-02", "RUN-03", "RUN-04"]`
  - local_runtime_first_floor_complete `True`
  - runtime_plan_boundary_present `True`
  - repo-root absolute matches 0
- `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture` passed:
  the focused `practical_alpha1_runtime_plan` tests ran 5 tests and passed.
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
  passed: 6 tests.
- `cargo test -p mir-runtime --test alpha_local_runtime` passed: 3 tests.

## What changed in understanding

The run-local helper emitted clean public JSON already. The non-portable piece
was the internal local-runtime Cargo example invocation, which passed
repo-owned package roots as host absolute paths.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audits remain for
  `practical_alpha1_attach.py`, `practical_alpha1_avatar.py`,
  `practical_alpha1_save_load.py`, `practical_alpha08_session_hotplug.py`, and
  `practical_alpha09_devtools.py`.

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
- added a recent-log entry for practical alpha-1 run-local helper path
  portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-1 run-local helper path portability is now
  hardened

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the `RUN-*` row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all` and `closeout` JSON have zero repo-root
  absolute matches after the change.
- Confirmed tests cover path helper behavior and the local-runtime Cargo
  example argv.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 49.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical alpha-1 run-local helper and practical runtime
  Cargo tests were rerun.
- Oracle was not used because the package was a narrow mechanical
  portability hardening step with direct local evidence.

## Commit / push status

Pending at report creation time.

## Sub-agent session close status

No new sub-agent session was opened for this package.
