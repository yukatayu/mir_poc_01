# Report 2233 — storage workdir mountpoint guard hardening

- Date: 2026-07-05 01:37 JST
- Author / agent: Codex
- Scope: Macro 0 storage / tooling guard hardening
- Decision levels touched: LAB tooling / repository memory only

## Objective

Harden the storage workdir guard so the external workdir is treated as mounted
only when it is an exact mountpoint, then record the current storage state and
non-claims without changing canon, sample status, workflow status, OBL status,
or runtime / proof claims.

## Scope and assumptions

Scope is limited to the storage environment helper, disposable cleanup helper,
the regression test that covers the mountpoint guard, and repository-memory /
reader-facing documentation that mentions the storage guard.

Assumptions:

- `/mnt/mirrorea-work` is the configured default external workdir, but at this
  checkpoint it is not mounted.
- No cleanup, mount provisioning, ownership repair, or cache move is authorized
  by this package.
- `findmnt --mountpoint "$path"` is the intended exact mountpoint check; a
  directory that merely exists on `/` must not count as mounted.

## Start state / dirty state

Start state was clean and synced at
`503032a8681483403cf4f221713bdbe7c159fea6`.

Resource snapshot captured during the package:

- `df -h .`: `/dev/sda2` 188G total, 152G used, 27G available, 86% used.
- `free -h`: 15Gi memory total, 10Gi available; 15Gi swap total, 14Gi free.
- `lsblk -f`: only root-backed `/dev/sda2` is mounted for the repo; no
  `/mnt/mirrorea-work` mountpoint is present.
- `findmnt --mountpoint /mnt/mirrorea-work`: no mountpoint found.
- `du -sk .`: about 7.0G repository footprint.

The package resumed with uncommitted P95 changes in the working tree and no
report file yet.

## Documents consulted

- `README.md`
- `CANON.md`
- `Documentation.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `mirrorea_canon/plan/01-phases.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/90-source-traceability.md`
- `scripts/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/compiler_backend_llvm_preparation_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `.agents/skills/discord-report/SKILL.md`
- `/home/codex/.codex/superpowers/skills/verification-before-completion/SKILL.md`
- `/home/codex/.codex/superpowers/skills/requesting-code-review/SKILL.md`

## Actions taken

- Reproduced the mount-detection issue with an existing unmounted temporary
  directory: `findmnt -T` succeeds for a path on `/`, while
  `findmnt --mountpoint` rejects it.
- Added `scripts/tests/test_storage_workdir_guards.py` with two regression
  tests:
  - `mirrorea_storage_env.sh --ensure-dirs` must refuse an existing unmounted
    default workdir and avoid creating heavy subdirectories.
  - `cleanup_disposable_artifacts.sh --confirm` must refuse an existing
    unmounted workdir and preserve candidate contents.
- Verified the tests failed against the prior `findmnt -T` implementation.
- Added `mirrorea_is_mountpoint()` to
  `scripts/env/mirrorea_storage_env.sh`.
- Reused the same helper in
  `scripts/storage/cleanup_disposable_artifacts.sh`.
- Added `plan/148-storage-workdir-mountpoint-guard-hardening.md`.
- Updated source-hierarchy / docs validators to register `plan/148`.
- Updated storage guard references in README, Documentation, plan, scripts,
  hands-on, research abstract, progress, and task snapshots.
- Reordered the `progress.md` recent log so the newer P95 entry remains ahead
  of P94.
- Addressed reviewer feedback by preserving cleanup's parsed
  `--allow-unmounted` flag across env-script sourcing and adding a
  temp-workdir-only regression test for the explicit override.

## Files changed

- `README.md`
- `Documentation.md`
- `docs/hands_on/compiler_backend_llvm_preparation_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/research_abstract/compiler_backend_llvm_preparation_01.md`
- `docs/reports/2233-storage-workdir-mountpoint-guard-hardening.md`
- `plan/00-index.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/90-source-traceability.md`
- `plan/148-storage-workdir-mountpoint-guard-hardening.md`
- `progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/env/mirrorea_storage_env.sh`
- `scripts/storage/cleanup_disposable_artifacts.sh`
- `scripts/tests/test_storage_workdir_guards.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `tasks.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `df -h .`
- `free -h`
- `lsblk -f`
- `findmnt --mountpoint /mnt/mirrorea-work`
- `du -sk .`
- `git status --short --branch`
- `git rev-parse HEAD`
- `rg -n 'plan/148|00\\.\\.148|39\\.\\.148|118\\.\\.148|plan/00\\.\\.147|plan/39\\.\\.147|plan/118\\.\\.147|storage workdir|MIRROREA_WORKDIR_MOUNTED|最終更新' ...`
- `python3 -m unittest scripts.tests.test_storage_workdir_guards`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
- `bash scripts/storage/detach_prepare.sh`
- `python3 -m unittest scripts.tests.test_storage_workdir_guards`
- `bash -n scripts/env/mirrorea_storage_env.sh scripts/storage/cleanup_disposable_artifacts.sh scripts/storage/detach_prepare.sh`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `set +e; bash scripts/env/mirrorea_storage_env.sh --ensure-dirs ...`
- `python3 -m unittest discover -s scripts/tests`
- `make check`
- `cargo fmt --check`
- `cargo test --workspace --all-targets`
- tracked Discord webhook secret scan using `git grep`

## Evidence / outputs / test results

Initial RED evidence:

- `python3 -m unittest scripts.tests.test_storage_workdir_guards` failed
  against the previous implementation.
- The env helper returned success and created candidate directories for an
  existing unmounted workdir.
- The cleanup helper returned success and removed a marker under a candidate
  directory for an existing unmounted workdir.

Implementation evidence after the fix:

- `scripts/env/mirrorea_storage_env.sh` reports
  `MIRROREA_WORKDIR_MOUNTED=no` for the current default workdir.
- `scripts/storage/cleanup_disposable_artifacts.sh --list` reports
  `mounted: no`.
- `bash -n scripts/env/mirrorea_storage_env.sh scripts/storage/cleanup_disposable_artifacts.sh scripts/storage/detach_prepare.sh`
  exited 0.
- `python3 -m unittest scripts.tests.test_storage_workdir_guards`: initially
  2 tests OK; after reviewer follow-up, 3 tests OK, including a
  temp-workdir-only `--confirm --allow-unmounted` override regression.
- `python3 -m unittest scripts.tests.test_validate_docs`: 37 tests OK.
- `python3 scripts/check_source_hierarchy.py`: required 688, present 688,
  missing 0.
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1385
  numbered reports found.
- `git diff --check`: exited 0.
- `bash scripts/env/mirrorea_storage_env.sh`: reports
  `MIRROREA_WORKDIR_MOUNTED=no` for `/mnt/mirrorea-work`.
- `bash scripts/env/mirrorea_storage_env.sh --ensure-dirs`: exited 2 with the
  expected refusal message for unmounted default root.
- `test -d /mnt/mirrorea-work ...`: `/mnt/mirrorea-work directory absent`.
- `scripts/storage/cleanup_disposable_artifacts.sh --list` reports
  `mounted: no`.
- `scripts/storage/detach_prepare.sh` ran as a non-destructive audit and did
  not delete files; it reported repo usage about 7.1G, repo-local `target/`
  about 7.0G, `.git` about 52M, and missing external workdir.
- `python3 -m unittest discover -s scripts/tests`: initially 784 tests OK;
  after reviewer follow-up, 785 tests OK.
- `make check`: source hierarchy, docs validation, and `cargo check` exited 0.
- `cargo fmt --check`: exited 0.
- `cargo test --workspace --all-targets`: exited 0.
- Tracked webhook secret scan passed.

## What changed in understanding

The old check confused "path is on some mounted filesystem" with "the workdir
itself is a mountpoint." On a root-backed directory, `findmnt -T` therefore
weakened both creation and cleanup guards. Exact mountpoint detection is the
right boundary for the existing external-workdir policy.

This is a tooling safety correction only. It does not move any semantic,
runtime, proof, conformance, sample, workflow, or canon status.

## Open questions

- Whether and when `/mnt/mirrorea-work` should be mounted or provisioned for
  future heavy validation remains open.
- Whether to clean or move the existing repo-local `target/` cache remains
  open and requires explicit cleanup / storage direction.

## Suggested next prompt

If storage setup becomes the next desired line, ask for an explicit external
workdir setup / cleanup package. Otherwise, continue G1-safe maintenance or a
promoted OBL-020 / OBL-001 review-facing extraction line.

## Plan update status

更新済み:

- Added `plan/148-storage-workdir-mountpoint-guard-hardening.md`.
- Updated `plan/00-index.md`.
- Updated `plan/23-compiler-backend-llvm-guardrail-roadmap.md`.
- Updated `plan/90-source-traceability.md`.

## Documentation.md update status

更新済み: `Documentation.md` now mirrors the storage workdir mountpoint guard
hardening and its non-claims.

## progress.md update status

更新済み: `progress.md` records the P95 storage guard note and recent log entry.

## tasks.md update status

更新済み: `tasks.md` records the storage guard note and keeps the package as
Macro 0 maintenance, not a promoted OBL extraction or G1 exit.

## samples_progress.md update status

`samples_progress.md` 更新不要: this package did not change runnable sample,
workflow, debug-surface, or sample validation status.

## Reviewer findings and follow-up

Focused reviewer `019f2e00-bbc2-7c22-9262-59277d8ef30d` found:

- Medium: cleanup's `--allow-unmounted` flag was parsed before sourcing the env
  helper but clobbered by the env helper's top-level `allow_unmounted=0`.
- Medium: the report still had pending reviewer / commit placeholders.
- Low: the report's consulted-doc list omitted canon entry files.

Follow-up:

- Fixed cleanup flag preservation by saving and restoring the parsed cleanup
  flag across env-helper sourcing.
- Added a temp-workdir-only regression test proving
  `--confirm --allow-unmounted` removes only the temp candidate when explicitly
  allowed.
- Added canon entry files to the consulted-doc list.
- Re-ran focused storage tests, docs validation, source-hierarchy validation,
  diff check, all scripts unit tests, and `make check`; all exited 0.

## Skipped validations and reasons

Skipped destructive / state-changing storage actions by design:

- Did not run cleanup with a real confirmed deletion target.
- Did not run `--allow-unmounted`.
- Did not mount or provision `/mnt/mirrorea-work`.
- Did not move Cargo / Lean / LLVM caches.

## Commit / push status

Primary package commit pushed:

- `e30bd969` — `Harden storage workdir mountpoint guard`

This report-status line is recorded by a follow-up status-only commit.

## Sub-agent session close status

Read-only sidecar `019f2df3-ee93-7df1-ac08-18589140dc5a` completed and was
closed. It recommended keeping the package scoped as a Macro 0 operational
guardrail, avoiding cleanup / mount / ownership repair, and not treating this
as OBL promotion, sample relabel, runtime readiness, conformance, or G1 exit.
