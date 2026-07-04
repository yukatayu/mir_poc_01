# Report 2186 — Surface helper subprocess path portability

- Date: 2026-07-04 14:17 JST
- Author / agent: Codex
- Scope: Surface Mir helper subprocess path portability and P-SURF-99 gate refresh
- Decision levels touched: none; helper/reporting maintenance only

## Objective

Make `scripts/surface_mir_samples.py` pass repo-owned Surface source files to
nested subprocesses through repo-relative `samples/...` arguments, then rerun
the Surface helper and release-check evidence.

## Scope and assumptions

Scope:

- audit Surface helper output for repo-root absolute path drift
- add regression coverage for repo-relative subprocess input arguments
- preserve absolute arguments for paths outside the repository
- refresh the stale P-SURF-99 release-check sample-count gate
- update snapshot docs and runnable dashboard evidence

Assumptions:

- Public Surface helper JSON already having zero repo-root absolute matches
  means this package should not add raw payload rewriting.
- Passing repo-owned paths relative to `REPO_ROOT` is safe because all nested
  subprocesses already run with `cwd=REPO_ROOT`.
- Updating the P-SURF-99 gate from 46 to 52 is a release-check maintenance fix
  because the current Surface matrices contain 52 rows.

## Start state / dirty state

Package 48 started from clean `HEAD == origin/main == cdffd8aa` after the Full
System V1 helper path audit package.

## Documents consulted

- `AGENTS.md`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2185-full-system-v1-helper-path-audit.md`

## Actions taken

- Confirmed `surface_mir_samples.py check-all` output already had zero
  repo-root absolute matches.
- Added RED tests for repo-owned Surface source path conversion and external
  absolute path preservation.
- Added `repo_cli_arg()` to `scripts/surface_mir_samples.py`.
- Replaced five nested subprocess file arguments with `repo_cli_arg(path)`.
- Updated the P-SURF-99 release-check semantic sample-count gate to the current
  52-row matrix.
- Updated `scripts/README.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`.
- Spawned a read-only reviewer sub-agent for the current diff.

## Files changed

- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2186-surface-helper-subprocess-path-portability.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `python3 scripts/surface_mir_samples.py check-all --format json > /tmp/mirrorea-surface-mir-samples-p48-before.json`
- JSON scan of `/tmp/mirrorea-surface-mir-samples-p48-before.json`
- `sed -n '240,430p' scripts/surface_mir_samples.py`
- `sed -n '520,860p' scripts/surface_mir_samples.py`
- `sed -n '1,260p' scripts/tests/test_surface_mir_samples.py`
- `python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_repo_cli_arg_uses_repo_relative_paths_for_sample_files scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_repo_cli_arg_keeps_external_paths_absolute` (RED)
- same focused unit command after implementation
- `python3 -m unittest scripts.tests.test_surface_mir_samples`
- `df -h . /tmp`
- `free -h`
- `python3 scripts/surface_mir_samples.py check-all --format json > /tmp/mirrorea-surface-mir-samples-p48-fixed.json`
- `python3 -m unittest scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py matrix --format json`
- failed attempt: `python3 scripts/surface_mir_release_check.py --format json check-all --out-dir /tmp/mirrorea-surface-release-p48`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p48`
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py check-all --format json > /tmp/mirrorea-surface-mir-samples-p48-final.json`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p48-final > /tmp/mirrorea-surface-release-p48-final.json`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-review > /tmp/mirrorea-surface-release-review.json`

## Evidence / outputs / test results

- Initial `surface_mir_samples.py check-all` scan:
  - sample_count 52
  - passed 52
  - failed `[]`
  - repo-root absolute matches 0
- RED focused tests failed with `AttributeError` because `repo_cli_arg` did not
  exist.
- Focused tests passed after implementation: 2 tests.
- `python3 -m unittest scripts.tests.test_surface_mir_samples` passed:
  47 tests.
- Final combined Surface unit coverage passed: 55 tests.
- `python3 scripts/surface_mir_samples.py matrix --format json` reported:
  - sample_count 52
  - executable_count 52
  - validation_errors `[]`
- Final `surface_mir_samples.py check-all` passed:
  - sample_count 52
  - passed 52
  - failed `[]`
  - workflow_ready `False`
  - repo-root absolute matches 0
- `surface_mir_release_check.py --format json check-all --out
  /tmp/mirrorea-surface-release-p48-final` passed:
  - `surface_mir_release_check_ready: true`
  - failed_commands `[]`
  - results 18
  - repo-root absolute matches 1, from `check_source_hierarchy.py` stdout
    `repo_root:` display, not from helper argv or payload fields
- Reviewer-requested `surface_mir_release_check.py --format json check-all
  --out /tmp/mirrorea-surface-release-review` also passed:
  - `surface_mir_release_check_ready: true`
  - failed_commands `[]`
  - results 18
  - repo-root absolute matches 1, same `check_source_hierarchy.py` stdout
    `repo_root:` display
- `python3 -m unittest scripts.tests.test_validate_docs` passed: 36 tests.
- `python3 scripts/validate_docs.py` passed and found 1338 numbered reports
  after this report was added.
- `python3 scripts/check_source_hierarchy.py` passed: required/present 659/659.
- `git diff --check` passed.
- Resource preflight before release-check:
  - root filesystem: 188G size, 141G used, 38G available
  - memory: 15Gi total, 10Gi available

## What changed in understanding

The Surface helper public JSON did not need raw payload normalization; it was
already repo-root clean. The portability issue was narrower: the helper passed
repo-owned source files to nested Cargo examples and `mirrorea-alpha
patch-source` as host absolute paths.

The full Surface release-check also revealed a stale P-SURF-99 semantic gate:
it still expected 46 rows, while the current Surface matrices now contain 52
rows after later G1 addenda.

## Open questions

No blocking questions for this package.

Remaining follow-up is lower priority:

- audit practical alpha helper families and older computational helper surfaces
  only if a concrete repo-root leak or non-portable subprocess argv is
  reproduced.

## Suggested next prompt

Continue autonomous maintenance by auditing the lower-priority helper
portability candidates or by moving to the next current task-map package.

## Plan update status

`plan/` 更新不要:

- This package did not change roadmap, semantics, source-traceability,
  open-question, or repository-memory decisions.

## Documentation.md update status

`Documentation.md` 更新不要:

- The top-level reader-facing status already records the current 52-row Surface
  state; this package only refreshed script/snapshot operational evidence.

## progress.md update status

Updated:

- advanced the top `最終更新` timestamp
- added a recent-log entry for Surface helper subprocess path portability
- clarified that the old P-SURF-99 46-row count was then-current and that the
  current snapshot uses the later 52-row state

## tasks.md update status

Updated:

- advanced the top `最終更新` timestamp
- removed Surface helper from the high-priority path-portability candidate list
- recorded remaining lower-priority helper portability audits as conditional on
  reproduced leaks

## samples_progress.md update status

Updated:

- advanced the top `Last updated` timestamp
- added Surface helper subprocess path portability to the Surface row and
  Recent Validation Log

## Reviewer findings and follow-up

Sub-agent reviewer:

- `019f2b8f-0f4b-7063-bfb9-6bfcd8bff9de` was spawned for read-only review of
  the current diff.
- Finding: low-risk test gap. The reviewer noted that the P-SURF-99
  sample-count gate compared to `SURFACE_SAMPLE_COUNT_FOR_P_SURF_99`, while
  existing unit fixtures also used the same constant.
- Follow-up: added
  `test_p_surf_99_sample_count_gate_matches_current_matrix`, which asserts the
  release-check constant equals `surface_mir_samples.matrix()["sample_count"]`.
- The reviewer also confirmed the path portability change, the 52-row matrix
  total, and the absence of current stale status docs.
- The reviewer-requested release-check command passed before closeout.

Focused self-review:

- Confirmed all five nested Surface file subprocess arguments now use
  `repo_cli_arg(path)`.
- Confirmed external absolute path fallback is covered.
- Confirmed the 52-row gate equals the current matrix total
  9 + 5 + 16 + 4 + 4 + 2 + 12.
- Confirmed no `specs/`, `mirrorea_canon/`, or `plan/` normative statement was
  changed.

## Skipped validations and reasons

- Full workspace `cargo test --workspace --all-targets` was not rerun because
  this package changes one Python helper, one Python release-check gate, unit
  tests, and snapshot docs. The Surface release-check reran the relevant
  focused Cargo tests and compatibility anchors.

## Commit / push status

Committed and pushed:

- `9777ebf9 Use relative Surface helper inputs`

This report section was updated after the first push and will be captured by a
report-only follow-up commit.

## Sub-agent session close status

Reviewer sub-agent `019f2b8f-0f4b-7063-bfb9-6bfcd8bff9de` completed and was
closed after its finding was addressed.
