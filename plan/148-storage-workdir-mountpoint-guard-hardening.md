# plan/148 - storage workdir mountpoint guard hardening

## status

LAB repository-memory / tooling guard hardening.

This package does not edit canon, does not promote a Surface or G1 package,
does not choose OBL-020 or OBL-001 review-facing extraction, and does not claim
any sample, runtime, proof, conformance, or G1 exit status.

## purpose

Long autonomous validation runs can create large disposable build artifacts.
The repository already routes heavy disposable artifacts toward an external
workdir, normally `/mnt/mirrorea-work`, and deliberately refuses to create or
delete heavy paths under an unmounted default root unless explicitly allowed.

This package hardens that guard by requiring the configured workdir itself to
be a mountpoint. A directory that merely exists on the root filesystem is not
enough.

## bug found

The storage helpers used `findmnt -T "$MIRROREA_WORKDIR"` as the mount check.
That command answers "which mounted filesystem contains this path." It
therefore succeeds for an ordinary directory on `/`.

Consequence before this package:

- `scripts/env/mirrorea_storage_env.sh --ensure-dirs` could create the heavy
  external-workdir directory family under a normal root-disk directory.
- `scripts/storage/cleanup_disposable_artifacts.sh --confirm` could treat that
  same ordinary directory as mounted and delete candidate paths under it.

Both behaviors contradicted the documented "unmounted default root" guard.

## fix

- `scripts/env/mirrorea_storage_env.sh` now exposes
  `mirrorea_is_mountpoint()`.
- That helper uses `findmnt --mountpoint "$path"`, so only an exact mountpoint
  is accepted as mounted.
- `scripts/storage/cleanup_disposable_artifacts.sh` reuses the same helper
  after sourcing the env script.

## regression evidence

`scripts/tests/test_storage_workdir_guards.py` covers both refusal paths with a
temporary directory that exists but is not a mountpoint.

The tests first failed against the previous implementation:

- `mirrorea_storage_env.sh --ensure-dirs` returned 0 and created candidate
  directories.
- `cleanup_disposable_artifacts.sh --confirm` returned 0 and removed a
  candidate under the temporary directory.

After the fix, both tests pass and the marker file is preserved.

A follow-up reviewer check found that `cleanup_disposable_artifacts.sh` parsed
`--allow-unmounted` before sourcing the env helper, but the env helper's own
top-level `allow_unmounted=0` reset that flag. The cleanup script now preserves
the parsed cleanup flag across sourcing, and a temp-workdir-only regression
test verifies that explicit `--confirm --allow-unmounted` still works without
touching the real configured workdir.

## current environment reading

At this checkpoint:

- `/mnt/mirrorea-work` is not mounted.
- `scripts/env/mirrorea_storage_env.sh` reports `MIRROREA_WORKDIR_MOUNTED=no`.
- `scripts/storage/cleanup_disposable_artifacts.sh --list` reports
  `mounted: no`.
- repository-local `target/` is about 7.0G after the recent validation sweep.

No cleanup was run. Any deletion still requires explicit `--confirm`, and
without `--allow-unmounted` cleanup refuses an unmounted workdir.

## tmp artifact follow-up

P100 audited the disposable `/tmp/mirrorea-*` footprint after the P99 validation
sweep. A closeout helper run reported 3,348 immediate directory cleanup
candidates totaling 25,079,200 KiB. A broad glob `du` audit in the same window
reported about 25,090,344 KiB; the helper intentionally counts only immediate
directories under the tmp root. The P99-specific subset had 5 entries totaling
378,232 KiB.

`scripts/storage/tmp_mirrorea_artifacts.sh` now provides a small safety helper
for this separate temporary-artifact class:

- `--list` reports immediate `mirrorea-*` directories under the configured tmp
  root and prints candidate count / total KiB without deleting anything.
- `--cleanup --confirm` deletes only immediate `mirrorea-*` directories under
  that tmp root.
- `--cleanup` without `--confirm` exits nonzero and preserves candidates.
- `--tmp-root DIR` exists for tests and controlled audits; the default root is
  `${TMPDIR:-/tmp}`.

Regression tests use only temporary fixture directories. P100 did not delete
the real `/tmp/mirrorea-*` entries, did not mount external storage, and did not
move repo-local `target/`.

## non-claims

This package does not:

- mount or provision `/mnt/mirrorea-work`;
- delete `target/` or any build cache;
- move Cargo / Lean / LLVM artifacts;
- change sample status;
- change Product Alpha, Full System V1, or Surface workflow status;
- claim final packaging, backend, LLVM, or public distribution readiness;
- edit canon;
- affect OBL status, proof status, conformance, runtime readiness, or G1 exit.

## next use

Before future heavy validation, LLVM, backend, or generated-artifact work:

1. Run `bash scripts/env/mirrorea_storage_env.sh`.
2. Confirm `MIRROREA_WORKDIR_MOUNTED=yes` before using
   `--ensure-dirs` without `--allow-unmounted`.
3. Use `bash scripts/storage/detach_prepare.sh` for non-destructive audit.
4. Use `bash scripts/storage/cleanup_disposable_artifacts.sh --list` before any
   explicit cleanup request.
5. Use `bash scripts/storage/tmp_mirrorea_artifacts.sh --list` to audit
   disposable `/tmp/mirrorea-*` helper outputs before any explicit temporary
   artifact cleanup request.

If `/mnt/mirrorea-work` remains unmounted, heavy commands should either keep
their current repo-local target behavior intentionally documented, use `/tmp`
for disposable summaries, or wait for an explicit external-workdir setup path.
