# Report 2188 — Practical alpha-1 transport path portability

- Date: 2026-07-04 14:34 JST
- Author / agent: Codex
- Scope: Practical alpha-1 transport helper path portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Remove repo-root absolute path leakage from `scripts/practical_alpha1_transport.py`
closeout JSON and make repo-owned local / Docker transport command paths
repo-relative where executable.

## Scope and assumptions

Scope:

- reproduce `practical_alpha1_transport.py` `check-all` / `closeout` path output
- add regression coverage for repo-relative closeout fields and command argv
- preserve Docker bind mount host-path env values as internal execution state
- rerun practical alpha-1 transport helper, focused tests, and relevant Cargo
  transport tests
- update snapshot docs and report the outcome

Assumptions:

- Docker Compose bind mounts need host filesystem paths, so
  `MIRROREA_PRACTICAL_ALPHA1_BINARY`, `MIRROREA_PRACTICAL_ALPHA1_OUTPUT_DIR`,
  and `MIRROREA_PRACTICAL_ALPHA1_PACKAGE_DIR` remain absolute internal env
  values.
- Repo-owned values that are emitted in JSON or passed as executable argv from
  `cwd=REPO_ROOT` should be repo-relative.
- This package does not change transport semantics, sample status, workflow
  status, ABI, or canon claims.

## Start state / dirty state

Package 50 started from clean `HEAD == origin/main == 275d67ec` after the Mir
computational helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/practical_alpha1_transport.py`
- `scripts/tests/test_practical_alpha1_transport.py`
- `samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2187-mir-computational-helper-path-portability.md`

## Actions taken

- Ran `practical_alpha1_transport.py check-all` and confirmed the helper summary
  already had zero repo-root absolute matches.
- Ran `practical_alpha1_transport.py closeout` and reproduced two repo-root
  absolute path fields: `compose_file` and `binary_path`.
- Added RED tests for:
  - repo-owned `COMPOSE_FILE` / `BINARY_PATH` relative display
  - external path fallback
  - closeout JSON not containing repo-root paths
  - local transport cargo example package argv
  - Docker Compose `-f` argv while preserving Docker bind mount host-path env
- Added `repo_cli_arg()` to `scripts/practical_alpha1_transport.py`.
- Converted local transport package argv, Docker Compose `-f` argv, closeout
  `compose_file`, closeout `binary_path`, and selected failure messages to use
  `repo_cli_arg()`.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.

## Files changed

- `scripts/practical_alpha1_transport.py`
- `scripts/tests/test_practical_alpha1_transport.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2188-practical-alpha1-transport-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/practical_alpha1_transport.py check-all --format json > /tmp/mirrorea-practical-alpha1-transport-p50-check-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-transport-p50-check-before.json`
- `python3 scripts/practical_alpha1_transport.py closeout --format json > /tmp/mirrorea-practical-alpha1-transport-p50-closeout-before.json`
- JSON scan of `/tmp/mirrorea-practical-alpha1-transport-p50-closeout-before.json`
- `sed -n '1,260p' scripts/practical_alpha1_transport.py`
- `sed -n '260,430p' scripts/practical_alpha1_transport.py`
- `sed -n '1,240p' scripts/tests/test_practical_alpha1_transport.py`
- `sed -n '1,240p' samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml`
- `python3 -m unittest scripts.tests.test_practical_alpha1_transport.PracticalAlpha1TransportTests.test_repo_cli_arg_uses_repo_relative_paths_for_transport_files scripts.tests.test_practical_alpha1_transport.PracticalAlpha1TransportTests.test_repo_cli_arg_keeps_external_paths_absolute scripts.tests.test_practical_alpha1_transport.PracticalAlpha1TransportTests.test_closeout_marks_stage_pa1_5_complete_once_all_rows_pass scripts.tests.test_practical_alpha1_transport.PracticalAlpha1TransportTests.test_local_transport_invocation_uses_repo_relative_package_path scripts.tests.test_practical_alpha1_transport.PracticalAlpha1TransportTests.test_docker_transport_invocation_uses_repo_relative_compose_file` (RED)
- same focused unit command after implementation
- `python3 -m unittest scripts.tests.test_practical_alpha1_transport`
- `cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture`
- `python3 scripts/practical_alpha1_transport.py check-all --format json > /tmp/mirrorea-practical-alpha1-transport-p50-check-final.json`
- `python3 scripts/practical_alpha1_transport.py closeout --format json > /tmp/mirrorea-practical-alpha1-transport-p50-closeout-final.json`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `date '+%Y-%m-%d %H:%M %Z'`

## Evidence / outputs / test results

- Initial `check-all` scan:
  - sample_count 7
  - passed 7
  - failed `[]`
  - repo-root absolute matches 0
- Initial `closeout` scan:
  - repo-root absolute matches 2
  - leaking fields: `compose_file`, `binary_path`
- RED focused tests failed as expected:
  - `repo_cli_arg` did not exist
  - closeout emitted absolute `compose_file`
  - local transport argv contained absolute package path
  - Docker Compose `-f` argv contained absolute compose-file path
- Focused tests passed after implementation: 5 tests.
- `python3 -m unittest scripts.tests.test_practical_alpha1_transport` passed:
  10 tests.
- `cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture`
  passed: 5 tests.
- `cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture`
  passed: 8 tests.
- Final `check-all` scan:
  - sample_count 7
  - passed 7
  - failed `[]`
  - stage_pa1_5_complete `True`
  - repo-root absolute matches 0
- Final `closeout` scan:
  - stage_pa1_5_complete `True`
  - compose_file `samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml`
  - binary_path `target/debug/examples/mir_practical_alpha1_transport`
  - repo-root absolute matches 0
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1340 numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed after report creation.

## What changed in understanding

The transport `check-all` summary was already repo-root clean. The current leak
was limited to `closeout` display fields plus nested command argv. Docker bind
mount env values are intentionally still host paths because they are consumed by
Docker Compose volume bindings and are not emitted in committed or helper JSON.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- Practical alpha helper family audits remain for
  `practical_alpha1_check.py`, `practical_alpha1_run_local.py`,
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
- added a recent-log entry for practical alpha-1 transport helper path
  portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- recorded that practical alpha-1 transport helper path portability is now
  hardened

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- updated the `TR-A1-*` row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed emitted `check-all` and `closeout` JSON have zero repo-root absolute
  matches after the change.
- Confirmed Docker bind mount host-path env values remain internal-only and are
  not serialized into closeout evidence.
- Confirmed the focused tests cover path helper behavior, closeout fields, local
  cargo example argv, and Docker Compose `-f` argv.

No new sub-agent was opened for this package; it follows the completed
code-mapper recommendation from Package 49.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The relevant practical alpha-1 transport Python helper and Cargo
  transport tests were rerun.

## Commit / push status

Not yet committed at report draft time.

This section will be updated after commit and push.

## Sub-agent session close status

No new sub-agent session was opened for this package.
