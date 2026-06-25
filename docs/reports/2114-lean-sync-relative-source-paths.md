# Report 2114 — Lean sync relative source paths

- Date: 2026-06-25T08:49:44.364333Z
- Author / agent: Codex
- Scope: make Lean clean-near-end generated source references repo-relative, regenerate committed Lean bundle/readme artifacts, and validate portability
- Decision levels touched: no normative `specs/` decision changed; generated artifact portability and helper behavior only

## Objective

Change `current_l2_lean_sample_sync.py` so generated clean-near-end Lean bundle/readme `source_path` values are repo-relative instead of machine-local absolute paths, then regenerate and commit the affected artifacts.

## Scope and assumptions

- Scope is limited to the Lean sync helper and its generated `samples/lean/clean-near-end/` bundle/readme outputs.
- Active `.lean` proof stubs and foundations are not semantically changed.
- Runtime helper `list` still returns absolute source paths; the portability boundary is the Lean sync helper's committed artifact output.
- Paths under the repo are normalized to `samples/...` POSIX-style relative paths.
- Paths outside the repo are preserved as originally supplied.

## Start state / dirty state

- Branch: `main`.
- Start tracked state: clean after commit `f6a2a5df`.
- Ignored/local-only entries present: `.codex-discord/`, `Cargo.lock`, `target/`, and Python `__pycache__/`.
- Lean 4.29.1 / Lake 5.0.0 were already installed via `elan` from report 2113.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `samples/README.md`
- `scripts/README.md`
- `samples_progress.md`
- `docs/reports/2113-lean-toolchain-install-and-validation.md`

## Actions taken

- Recorded Discord task baseline before edits.
- Confirmed root cause: `sync_clean_stubs` wrote `row["source_path"]` from runtime output directly to bundle/readme artifacts; runtime output is checkout-absolute.
- Added TDD regression tests for:
  - normalizing repo-contained absolute paths with `..` segments to repo-relative `samples/...`;
  - preserving external absolute paths.
- Confirmed RED: the new tests failed because `repo_relative_source_path` did not exist.
- Implemented `repo_relative_source_path`.
- Routed bundle JSON and README rendering through the normalized source path.
- Ran `current_l2_lean_sample_sync.py` and kept the generated `samples/lean/clean-near-end/` artifact updates.
- Updated `progress.md` and `samples_progress.md` with the portability validation.
- Created this report.

## Files changed

- Helper / tests:
  - `scripts/current_l2_lean_sample_sync.py`
  - `scripts/tests/test_current_l2_lean_sample_sync.py`
- Generated Lean bridge artifacts:
  - `samples/lean/clean-near-end/*/*.bundle.json`
  - `samples/lean/clean-near-end/*/README.md`
- Snapshot/report:
  - `progress.md`
  - `samples_progress.md`
  - `docs/reports/2114-lean-sync-relative-source-paths.md`

## Commands run

- TDD RED:
  - `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- Sync / generation:
  - `python3 scripts/current_l2_lean_sample_sync.py`
- Verification:
  - `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - `find samples/lean/foundations samples/lean/clean-near-end -name "*.lean" -type f | sort | while read file; do lean "$file"; done`
  - `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe -- --nocapture`
  - `make check`
  - `git diff --check`
  - `rg` scan ensuring no `/home/...` source paths remain under `samples/lean/clean-near-end`
- Final closeout after report/reviewer:
  - `make check`
  - `git diff --check`
  - targeted absolute path scan for `samples/lean/clean-near-end`
  - redacted tracked-diff webhook marker scan

## Evidence / outputs / test results

- TDD RED:
  - `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` failed with `AttributeError: module 'current_l2_lean_sample_sync' has no attribute 'repo_relative_source_path'`.
- GREEN / validation logs under `/tmp/mir-relative-lean-sync-20260625-174840/validation/`:
  - `python_lean_sync_tests.log`: pass, 5 tests OK.
  - `current_l2_lean_sample_sync_idempotence.log`: pass.
  - `active_lean_files.log`: pass.
  - `cargo_lean_actual_probe.log`: pass.
  - `make_check.log`: pass.
  - `git_diff_check.log`: pass.
  - `no_absolute_lean_source_paths.log`: pass.
- Final closeout logs under `/tmp/mir-relative-lean-sync-20260625-174840/final/`:
  - `make_check.log`: pass.
  - `git_diff_check.log`: pass.
  - `no_absolute_lean_source_paths.log`: pass.
  - `secret_diff_scan.log`: pass.
- Post-review final closeout logs under `/tmp/mir-relative-lean-sync-20260625-174840/final-close-2/`:
  - `make_check.log`: pass.
  - `git_diff_check.log`: pass.
  - `no_absolute_lean_source_paths.log`: pass.
  - `secret_diff_scan.log`: pass.
  - Note: an earlier absolute-path scan command under `final-close/` had a shell quoting error and was rerun corrected; it is not used as passing evidence.
- Representative generated diff:
  - before: `/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/typing/01_authorized_declassification.mir`
  - after: `samples/clean-near-end/typing/01_authorized_declassification.mir`

## What changed in understanding

- The Lean sync helper was the right portability boundary: runtime reports can stay absolute for local execution evidence, while committed Lean bundle/readme artifacts should not encode workstation paths.
- `Path.resolve()` is needed because the runtime path contains `crates/mir-runtime/../../samples/...`; simple prefix stripping would keep unstable `..` segments.
- The generated Lean manifest itself did not need a content change for this portability fix.

## Open questions

- Should the runtime-side `clean_near_end` sample reports also expose a repo-relative companion path, while preserving the current absolute path for local diagnostics?
- Should other generated evidence families that still embed `/home/yukatayu/...` be normalized in a broader portability pass?
- Should future outside-repo sample rows be rejected by Lean sync instead of preserving external absolute paths?

## Suggested next prompt

Audit generated evidence under `samples/` for committed absolute workstation paths and normalize the ones that are intended to be portable source references.

## Plan update status

`plan/` 更新不要: no long-term repository memory or semantic design changed.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing project overview changed. The helper behavior and validation evidence are captured in this report and the progress dashboards.

## progress.md update status

`progress.md` 更新済み: added the 2026-06-25 Lean sync relative-path portability log and updated the top-level timestamp.

## tasks.md update status

`tasks.md` 更新不要: no task-map blocker or promoted package changed.

## samples_progress.md update status

`samples_progress.md` 更新済み: added a Recent Validation Log row for Lean clean-near-end source path portability and updated the top-level timestamp.

## Reviewer findings and follow-up

- Final read-only reviewer: `019efdf9-a7d5-7372-88f7-41d1d8ef81ba`.
- Finding: no blocking findings; reviewer checked 16 bundle JSON files, matching README `Source:` lines, target source file existence, and absence of webhook/token material.
- Residual risk: `repo_relative_source_path` preserves outside-repo paths unchanged, so a future external sample row would still emit an absolute path.
  Follow-up: recorded as an open question above; current runtime sample rows are repo-local and affected generated artifacts are clean.
- Residual report status: reviewer noted pending closeout fields.
  Follow-up: this report section and sub-agent close status were completed before commit.

## Skipped validations and reasons

- No `lake build` was run because the repository has standalone Lean files and a `lean-toolchain`, but no tracked `lakefile.*` project root.
- Historical `samples/lean/old` was not rerun in this task because the change only touches active clean-near-end generated artifacts; report 2113 already confirmed all 32 Lean files, including historical files, compile under Lean 4.29.1.

## Commit / push status

Pre-commit status at report close: final local validation has passed, and this report is intentionally included before commit. Commit will use `git commit --no-gpg-sign`; push and post-push commit漏れ verification will be recorded in the final response.

## Sub-agent session close status

Reviewer `019efdf9-a7d5-7372-88f7-41d1d8ef81ba`: completed final read-only review; no blocking findings; closed.
