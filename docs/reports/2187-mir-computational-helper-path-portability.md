# Report 2187 — Mir computational helper path portability

- Date: 2026-07-04 14:26 JST
- Author / agent: Codex
- Scope: Mir computational helper subprocess path portability
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/mir_computational_samples.py` pass repo-owned computational sample
roots to nested `mirrorea-alpha run-local` / `check` subprocesses through
repo-relative `samples/...` arguments.

## Scope and assumptions

Scope:

- audit `scripts/mir_computational_samples.py check-all` output for repo-root
  absolute path drift
- add regression coverage for repo-relative nested product-alpha subprocess
  arguments
- preserve absolute arguments for paths outside the repository
- update script and snapshot docs

Assumptions:

- Public helper JSON already having zero repo-root absolute matches means this
  package should not rewrite emitted payloads.
- The nested `mirrorea-alpha` subprocesses run with `cwd=REPO_ROOT`, so
  repo-relative sample-root arguments are portable and executable.
- This maintenance package does not change computational semantics, sample
  status, workflow status, ABI, or canon claims.

## Start state / dirty state

Package 49 started from clean `HEAD == origin/main == d2bf8426` after the
Surface helper path portability package.

## Documents consulted

- `AGENTS.md`
- `scripts/mir_computational_samples.py`
- `scripts/tests/test_mir_computational_samples.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2186-surface-helper-subprocess-path-portability.md`

## Actions taken

- Ran `scripts/mir_computational_samples.py check-all` and scanned the output
  for repo-root absolute path matches.
- Added RED tests for repo-owned computational sample-root conversion, external
  absolute fallback, and actual `run-local` / `check` subprocess argv.
- Added `repo_cli_arg()` to `scripts/mir_computational_samples.py`.
- Replaced the nested `mirrorea-alpha run-local` and `mirrorea-alpha check`
  sample-root arguments with `repo_cli_arg(sample_root)`.
- Strengthened the new argv tests to reject repo-root prefix matches, not only
  exact repo-root list membership.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Spawned a read-only code-mapper sub-agent to map practical-alpha helper
  portability candidates for the next package.

## Files changed

- `scripts/mir_computational_samples.py`
- `scripts/tests/test_mir_computational_samples.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2187-mir-computational-helper-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/mir_computational_samples.py check-all --format json > /tmp/mirrorea-mir-computational-p49-before.json`
- JSON scan of `/tmp/mirrorea-mir-computational-p49-before.json`
- `sed -n '1,260p' scripts/mir_computational_samples.py`
- `sed -n '1,260p' scripts/tests/test_mir_computational_samples.py`
- `sed -n '260,330p' scripts/mir_computational_samples.py`
- `sed -n '260,425p' scripts/tests/test_mir_computational_samples.py`
- `python3 -m unittest scripts.tests.test_mir_computational_samples.MirComputationalSamplesTests.test_repo_cli_arg_uses_repo_relative_paths_for_sample_roots scripts.tests.test_mir_computational_samples.MirComputationalSamplesTests.test_repo_cli_arg_keeps_external_paths_absolute scripts.tests.test_mir_computational_samples.MirComputationalSamplesTests.test_product_alpha_invocations_use_repo_relative_sample_roots scripts.tests.test_mir_computational_samples.MirComputationalSamplesTests.test_product_alpha_check_uses_repo_relative_sample_roots` (RED)
- same focused unit command after implementation
- `python3 -m unittest scripts.tests.test_mir_computational_samples`
- `python3 scripts/mir_computational_samples.py check-all --format json > /tmp/mirrorea-mir-computational-p49-fixed.json`
- `python3 scripts/mir_computational_samples.py check-all --format json > /tmp/mirrorea-mir-computational-p49-final.json`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `date '+%Y-%m-%d %H:%M %Z'`

## Evidence / outputs / test results

- Initial `mir_computational_samples.py check-all` scan:
  - sample_count 15
  - passed 15
  - failed `[]`
  - repo-root absolute matches 0
- RED focused tests failed:
  - `repo_cli_arg` did not exist
  - `run-local` and `check` argv contained host absolute sample-root paths
- Focused tests passed after implementation: 4 tests.
- Full Mir computational unit coverage passed: 17 tests.
- Final `mir_computational_samples.py check-all` passed:
  - sample_count 15
  - passed 15
  - failed `[]`
  - accepted 7
  - runtime_failures 0
  - expected_check_rejections 3
  - repo-root absolute matches 0
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1338 numbered reports
  before this report was added.
- Post-report `python3 scripts/validate_docs.py` passed and found 1339
  numbered reports.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- Post-report `python3 scripts/check_source_hierarchy.py` passed:
  required/present 659/659.
- `git diff --check` passed before report creation.
- Post-report `git diff --check` passed.

## What changed in understanding

The Mir computational helper public JSON did not need output normalization; it
already emitted repo-relative sample and package fields. The non-portable piece
was narrower: direct Product Alpha subprocess invocations passed repo-owned
sample roots as host absolute paths.

## Open questions

No blocking questions for this package.

Remaining follow-up:

- practical alpha helper family path-portability audit, guided by the
  code-mapper sidecar result when it completes.

## Suggested next prompt

Continue autonomous maintenance with the practical alpha helper portability
audit.

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
- added a recent-log entry for Mir computational helper subprocess path
  portability

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- removed older computational helper surfaces from the remaining
  lower-priority portability candidate list

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- added Mir computational helper subprocess path portability to the relevant
  dashboard row and Recent Validation Log

## Reviewer findings and follow-up

Focused self-review:

- Confirmed both nested product-alpha subprocess paths now use
  `repo_cli_arg(sample_root)`.
- Confirmed tests cover repo-owned path conversion, external path fallback, and
  both product-alpha invocation paths.
- Confirmed generated helper output remains repo-root clean.

No reviewer sub-agent was opened for this narrow code change.

Code-mapper sidecar `019f2b96-1c89-7e02-b7e9-ce33a823e0b3` completed a
read-only practical-alpha helper mapping for the next package:

- highest-priority next audit: `scripts/practical_alpha1_transport.py`
- reason: it has both practical-alpha subprocess path flow and likely repo-root
  path leakage in closeout JSON (`compose_file` / `binary_path`)
- likely validation commands: `python3 scripts/practical_alpha1_transport.py
  check-all --format json`, `python3 scripts/practical_alpha1_transport.py
  closeout --format json`, `python3 -m unittest
  scripts.tests.test_practical_alpha1_transport`, plus the Cargo validation rows
  listed in that helper closeout
- other concrete path-forwarding helpers were mapped for later:
  `practical_alpha1_check.py`, `practical_alpha1_run_local.py`,
  `practical_alpha1_attach.py`, `practical_alpha1_avatar.py`,
  `practical_alpha1_save_load.py`, `practical_alpha08_session_hotplug.py`, and
  `practical_alpha09_devtools.py`
- the sidecar did not edit files or run validation commands

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, focused Python tests, and snapshot
  docs. The real Mir computational helper `check-all` reran the relevant
  product-alpha subprocess paths.

## Commit / push status

Committed and pushed:

- `3a72a5eb Use relative Mir computational helper inputs`

This report section was updated after the first push and will be captured by a
report-only follow-up commit.

## Sub-agent session close status

Code-mapper sub-agent `019f2b96-1c89-7e02-b7e9-ce33a823e0b3` completed and was
closed after its result was recorded for the next package.
