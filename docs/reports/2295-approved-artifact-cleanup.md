# Report 2295 - approved local artifact cleanup

- Date: 2026-07-21 17:49 JST
- Author / agent: Codex
- Scope: Execute the user's approved cleanup of the local Rust build output and direct Mirrorea temporary artifacts only.
- Decision levels touched: Operational cleanup only. No L0/L1, theory, OBL, contract, SCN, Gate, Phase, implementation, or public-state movement.

## Objective

Recover root filesystem capacity without deleting Git-tracked source, canonical material, retained LAB evidence, or global Lean tooling.

## Scope and assumptions

The explicit approval covers the previously audited `target/` build directory and direct `/tmp/mirrorea-*` temporary directories. `~/.elan`, `.git`, source directories, non-Mirrorea `/tmp` entries, and the unmounted external-workdir candidate are excluded.

## Start state / dirty state

Started from pushed, clean `main` at `6fcce73a`, with root at about 2.4 GiB free, ignored `target/` at 7.0 GiB, and 460 direct Mirrorea temporary directories totaling about 2.33 GiB.

## Documents consulted

- `AGENTS.md` storage/build-artifact and cleanup rules.
- `docs/reports/2294-storage-followup.md`, `docs/project-status.md`, `progress.md`, and `tasks.md`.
- `scripts/storage/cleanup_disposable_artifacts.sh` and `scripts/storage/detach_prepare.sh`.

## Actions taken

- Reconfirmed that `target/` is Git-ignored, the temporary directories are direct `/tmp/mirrorea-*` paths, and no Mirrorea/Cargo/Rust/Lean process held them open.
- Ran `cargo clean` from the repository root.
- Removed only the matched direct Mirrorea temporary directories after explicit approval.
- Verified final filesystem capacity and clean Git state.

## Files changed

- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2295-approved-artifact-cleanup.md`

## Commands run

- `git check-ignore -v target target/debug`
- Process and open-file checks for the selected temporary directories.
- `cargo clean`
- An exact direct-directory cleanup over `/tmp/mirrorea-*`.
- `df -h`, `du -sh`, and Git status verification.

## Evidence / outputs / test results

- `cargo clean` reported removal of 18,248 files and 8.5 GiB.
- The direct temporary cleanup removed 460 Mirrorea directories; zero matching directories remain.
- Root free space increased from about 2.4 GiB to 12 GiB; `/tmp` is now 73 MiB and the repository is 111 MiB without the generated Rust build output.
- Before this operational record was added, the cleanup itself left the tracked worktree clean. No tracked source, `.git` content, Lean toolchain, canonical record, plan evidence, or non-Mirrorea temporary path was removed.
- The first temporary-directory deletion attempt had an invalid `find` argument order and failed before deletion; the corrected one-directory-at-a-time command removed the audited paths successfully.

## What changed in understanding

The principal local pressure was reproducible Rust output plus historical project-specific temporary test artifacts. Their removal is operationally safe after process checks and explicit approval, but future heavy work can recreate the build output quickly.

## Open questions

- The configured external workdir remains unmounted.
- Whether to prepare external storage before future LLVM, toolchain, or broad executable-suite work.
- Whether a retention policy should prune future direct Mirrorea temporary outputs automatically after validated runs.

## Suggested next prompt

Prepare or mount the external workdir, then resume the OBL-021 eligibility/pre-registration package with a fresh capacity audit before any heavy validation.

## Plan update status

`plan/` 更新不要: no roadmap, theory choice, or retained evidence changed.

## Documentation.md update status

`Documentation.md` 更新不要: the reader route did not change.

## docs/project-status.md update status

更新済み: the storage stop line now records approved recovery and the remaining external-workdir guard.

## progress.md update status

更新済み: the recent log records exact cleanup evidence and restored capacity.

## tasks.md update status

更新済み: maintenance now requires capacity checks rather than a full heavy-work pause.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample status or validation contract changed.

## Reviewer findings and follow-up

No reviewer was required for the explicitly approved, pre-audited cleanup. The previous storage audit's constraints are retained for future heavy work.

## Skipped validations and reasons

No rebuild was run after `cargo clean`; rebuilding would immediately recreate the intentionally removed output. Documentation validation is run for these status/report changes.

## Commit / push status

The cleanup baseline is pushed. The validated task-close commit records this operational update and is pushed to `origin/main`.

## Sub-agent session close status

No sub-agent was used for this bounded cleanup task.
