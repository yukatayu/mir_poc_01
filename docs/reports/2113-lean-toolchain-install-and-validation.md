# Report 2113 — Lean toolchain install and validation

- Date: 2026-06-25T08:38:29.612145Z
- Author / agent: Codex
- Scope: global Lean / Lake / elan install for the Codex user, Lean validation rerun, disk-use measurement, and snapshot report
- Decision levels touched: no normative `specs/` decision changed; environment/toolchain and validation-evidence update only

## Objective

Install the Lean toolchain needed by this repository, rerun Lean-specific validations that were skipped in report 2112, and report the new disk usage introduced by the install.

## Scope and assumptions

- User explicitly allowed global installation of Lean and other required tools.
- Installation target was the Codex user's home directory via `elan`, not repo source and not `/usr/local`.
- The repository `lean-toolchain` file pins `leanprover/lean4:v4.29.1`; this version was installed.
- Disk usage is reported two ways:
  - filesystem used-byte delta from before/after `df -B1`;
  - direct `du -sb` size of the newly created `~/.elan` directory.
- This task intentionally does not rewrite committed Lean bundle absolute paths with local `/home/codex/...` paths.

## Start state / dirty state

- Branch: `main`, with `HEAD == origin/main` at the previous pushed commit `a7b5441d`.
- Tracked dirty state before report/progress edits: clean.
- Ignored/local-only entries present: `.codex-discord/`, `Cargo.lock`, `target/`, and Python `__pycache__/`.
- Baseline at `2026-06-25 17:35:41 JST`:
  - `df -B1 .`: used `86,396,616,704` bytes, available `105,247,682,560` bytes.
  - `~/.elan`: absent.
  - `~/.profile`: `1,349` bytes.
  - `~/.local/bin`: `59,620,672` bytes.

## Documents consulted

- `lean-toolchain`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/current_l2_theorem_lean_stub_pipeline.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py`
- `scripts/tests/test_current_l2_theorem_toolchain_probe.py`
- `crates/mir-semantics/tests/current_l2_lean_theorem_stub_support.rs`
- `crates/mir-semantics/tests/current_l2_lean_theorem_stub_actual_probe.rs`
- Existing report context: `docs/reports/2112-broad-build-execution-validation-audit.md`

## Actions taken

- Recorded a Discord task baseline with `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.
- Recorded before-install disk baseline under `/tmp/mir-lean-install-baseline-20260625-173541`.
- Inspected `lean-toolchain` and confirmed target toolchain `leanprover/lean4:v4.29.1`.
- Installed `elan` and Lean globally for the Codex user:
  - `elan 4.2.3`
  - `Lean 4.29.1`
  - `Lake 5.0.0-src+f72c35b`
- Confirmed a fresh login shell resolves:
  - `/home/codex/.elan/bin/lean`
  - `/home/codex/.elan/bin/lake`
  - `/home/codex/.elan/bin/elan`
- Ran active and historical Lean files with `lean`.
- Ran `current_l2_lean_sample_sync.py`; it passed but rewrote clean-near-end Lean bundle/readme source paths from `/home/yukatayu/...` to `/home/codex/...`. These were inspected as path-only churn and restored.
- Reran Lean theorem-stub pipeline and Lean-related Python / Rust tests.
- Updated `progress.md` and `samples_progress.md` recent validation rows.
- Created this report.

## Files changed

- Repository files changed:
  - `progress.md`
  - `samples_progress.md`
  - `docs/reports/2113-lean-toolchain-install-and-validation.md`
- Global / non-repo files changed:
  - created `/home/codex/.elan/`
  - appended `export PATH="$HOME/.elan/bin:$PATH"` to `/home/codex/.profile`
- Restored / not committed:
  - `samples/lean/clean-near-end/*/*.bundle.json`
  - `samples/lean/clean-near-end/*/README.md`
  These were changed only by local absolute path churn after sync.

## Commands run

- Baseline / disk:
  - `df -h . "$HOME" /tmp`
  - `df -B1 . "$HOME" /tmp`
  - `du -sb "$HOME/.elan" "$HOME/.cache/elan" "$HOME/.local/bin" "$HOME/.profile" "$HOME/.bashrc"`
- Install:
  - `curl -fsSL https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh | sh -s -- -y --default-toolchain leanprover/lean4:v4.29.1`
- Version / path:
  - `lean --version`
  - `lake --version`
  - `elan --version`
  - `bash -lc 'command -v lean; lean --version; command -v lake; command -v elan'`
- Lean validations:
  - direct `lean` over `samples/lean/foundations` and `samples/lean/clean-near-end`
  - direct `lean` over all `samples/lean/**/*.lean`, including historical `samples/lean/old`
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e1-place-atomic-cut --artifact-root /tmp/mir-lean-install-baseline-20260625-173541/lean-validation/theorem-artifacts --run-label theorem-lean-stub-e1-after-install`
  - `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync scripts.tests.test_current_l2_theorem_lean_stub_pipeline scripts.tests.test_current_l2_theorem_toolchain_probe`
  - `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_support -- --nocapture`
  - `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe -- --nocapture`
- Cleanup / diff:
  - restored local `/home/codex/...` path churn in generated Lean clean-near-end bundle/readme files with a mechanical replacement back to committed `/home/yukatayu/...`.
- Final closeout:
  - `make check`
  - `git diff --check`
  - `bash -lc 'command -v lean && lean --version && command -v lake && command -v elan'`
  - redacted tracked-diff webhook marker scan

## Evidence / outputs / test results

- Install log: `/tmp/mir-lean-install-baseline-20260625-173541/elan_install.log`
- Version log: `/tmp/mir-lean-install-baseline-20260625-173541/lean_versions_after_install.log`
- Active Lean files:
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/active_lean_files.log`
  - result: pass for 20 active files.
- All Lean files:
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/all_lean_files_including_old.log`
  - result: pass for 32 files including historical `samples/lean/old`.
- Lean sync helper:
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/current_l2_lean_sample_sync.log`
  - result: pass; path-only repo diffs restored.
- Lean theorem-stub pipeline:
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/current_l2_theorem_lean_stub_pipeline_after_install.log`
  - result: pass.
- Lean tests:
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/tests/python_lean_tests.log`
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/tests/cargo_lean_stub_support.log`
  - `/tmp/mir-lean-install-baseline-20260625-173541/lean-validation/tests/cargo_lean_actual_probe.log`
  - result: pass.
- Disk after install at `2026-06-25 17:38:09 JST`:
  - `df -B1 .`: used `89,248,079,872` bytes, available `102,396,219,392` bytes.
  - filesystem used delta: `2,851,463,168` bytes = about `2.66 GiB`.
  - `/home/codex/.elan`: `2,819,461,716` bytes = about `2.63 GiB`.
  - `/home/codex/.profile`: increased by `37` bytes.
  - validation temp output root: `17,323` bytes, excluding toolchain install.
- Final closeout evidence:
  - `/tmp/mir-lean-install-baseline-20260625-173541/final/make_check.log`: pass.
  - `/tmp/mir-lean-install-baseline-20260625-173541/final/git_diff_check.log`: pass.
  - `/tmp/mir-lean-install-baseline-20260625-173541/final/lean_login_shell.log`: pass.
  - `/tmp/mir-lean-install-baseline-20260625-173541/final/secret_diff_scan.log`: pass.

## What changed in understanding

- The repository's active and historical Lean files all compile under the pinned Lean 4.29.1 toolchain.
- `current_l2_lean_sample_sync.py` is now runnable, but it is not path-stable: it rewrites committed clean-near-end Lean bundle/readme files with the local checkout absolute path.
- `current_l2_lean_theorem_stub_actual_probe` now exercises actual Lean execution instead of skipping due to missing `lean`.
- The large disk cost is mostly the Lean toolchain `lib` directory under `~/.elan/toolchains/leanprover--lean4---v4.29.1/lib`.

## Open questions

- Should Lean clean-near-end bundle/readme source paths be changed to repo-relative paths so `current_l2_lean_sample_sync.py` can run without machine-local churn?
- Should `elan` installation be documented as a recommended local setup step, including the expected about-2.7-GiB disk cost?

## Suggested next prompt

Make `current_l2_lean_sample_sync.py` path-stable by emitting repo-relative source paths, then regenerate and validate the Lean manifest intentionally.

## Plan update status

`plan/` 更新不要: no long-term repository memory or semantic design decision changed. The environment now supports Lean validation, and the status was recorded in `progress.md` / `samples_progress.md`.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing project entrypoint changed in this task. A future setup-doc task may document `elan` installation.

## progress.md update status

`progress.md` 更新済み: added the 2026-06-25 Lean install / mechanization probe log and updated the top-level timestamp.

## tasks.md update status

`tasks.md` 更新不要: no task-map blocker or self-driven package changed. The path-stability follow-up is recorded here as an open question / suggested prompt.

## samples_progress.md update status

`samples_progress.md` 更新済み: added the Lean toolchain install / mechanization probe validation row and updated the top-level timestamp.

## Reviewer findings and follow-up

Local closeout review only; no separate sub-agent was used for this focused environment/toolchain follow-up.

- Finding: `current_l2_lean_sample_sync.py` produced path-only local absolute path churn in Lean clean-near-end bundle/readme files.
  Follow-up: inspected the diff and restored the committed `/home/yukatayu/...` path strings so no machine-local churn is committed.
- Finding: disk figure needed both filesystem delta and direct toolchain directory size.
  Follow-up: recorded both `df -B1` delta and `du -sb ~/.elan` values in this report, `progress.md`, and `samples_progress.md`.
- Final local validation passed: `make check`, `git diff --check`, login-shell Lean resolution, and redacted tracked-diff webhook marker scan.

## Skipped validations and reasons

- No `lake build` was run because this repo has a `lean-toolchain` file and standalone Lean files, but no tracked `lakefile.*` project root.
- The local path churn from `current_l2_lean_sample_sync.py` was not kept because it only replaced committed `/home/yukatayu/...` absolute paths with this checkout's `/home/codex/...` paths.

## Commit / push status

Pre-commit status at report close: final local validation has passed, and this report is intentionally included before the commit. Commit will use `git commit --no-gpg-sign`; push and post-push commit漏れ verification will be recorded in the final response.

## Sub-agent session close status

No sub-agent was used for this focused follow-up.
