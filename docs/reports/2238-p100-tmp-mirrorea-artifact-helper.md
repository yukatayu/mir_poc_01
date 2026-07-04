# Report 2238 - P100 tmp Mirrorea artifact helper

- Date: 2026-07-05 02:50 JST
- Author / agent: Codex
- Scope: Macro 0 / storage and temporary artifact maintenance
- Decision levels touched: none; LAB tooling guard only

## Objective

Add a safe, test-covered helper for auditing `/tmp/mirrorea-*` disposable
artifacts after long validation runs, without deleting real artifacts or
changing any canon, phase, gate, proof, conformance, runtime, sample, or
workflow status.

## Scope and assumptions

The package is limited to Macro 0 maintenance. It may add a helper script,
regression tests, and repository-memory notes. It must not clean real `/tmp`
entries, mount external storage, move caches, promote a Surface/G1 package,
select an OBL-020 or OBL-001 review-facing extraction, move OBL status, claim
proof / conformance, or relabel sample / workflow readiness.

The temporary artifact helper is intentionally separate from the external
workdir cleanup helper. The new helper targets immediate `mirrorea-*`
directories under a tmp root and requires explicit `--cleanup --confirm` before
deletion.

## Start state / dirty state

Start state was clean and synced on `main` at
`786db26ec69b89fa2397d5d75cc7f0378a559dc9`.

Discord task baseline was already recorded for P100 with
`python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`.

Initial P100 resource / storage audit:

- `df -h .`: `/dev/sda2` size 188G, used 156G, available 24G, use 88%.
- `free -h`: memory 15Gi total / 10Gi available; swap 15Gi total / 14Gi free.
- `lsblk -f`: root filesystem is `/dev/sda2` ext4 mounted at `/`.
- `findmnt -T .`: checkout is on `/dev/sda2`.
- `findmnt /mnt/mirrorea-work`: exit 1, no mount visible.
- `du -sk .`: 7373816.
- `du -sk target`: 7275624.
- `du -sk .git`: 54916.
- `.cargo` and `.lake` do not exist in the repo root.
- `scripts/env/mirrorea_storage_env.sh`: `MIRROREA_WORKDIR_MOUNTED=no`.
- `scripts/storage/cleanup_disposable_artifacts.sh --list`: list-only,
  mounted `no`, no deletion.
- P99 `/tmp/mirrorea-p99-*` subset: 5 entries, 378232 KiB total.
- Initial broad `/tmp/mirrorea-*` audit before helper implementation: 3446
  matching entries, 25090344 KiB total.
- Closeout helper audit: 3348 immediate directory cleanup candidates,
  25079200 KiB total.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `plan/148-storage-workdir-mountpoint-guard-hardening.md`
- `plan/149-current-phase-position-reading.md`
- `scripts/README.md`
- `scripts/tests/test_storage_workdir_guards.py`
- `scripts/storage/cleanup_disposable_artifacts.sh`
- `scripts/env/mirrorea_storage_env.sh`
- `/home/codex/.codex/superpowers/skills/using-superpowers/SKILL.md`
- `/home/codex/.codex/superpowers/skills/test-driven-development/SKILL.md`

## Actions taken

- Answered the user status question from `plan/149`, `progress.md`, and
  `tasks.md`: canon lifecycle remains `T0/G0 rebaseline`, phase 1 of 9, late
  pre-exit inside T0, with G0 exit unclaimed.
- Audited current storage, mount, repo usage, P99 tmp outputs, and total
  `/tmp/mirrorea-*` footprint.
- Wrote failing tests first for the tmp artifact helper.
- Implemented `scripts/storage/tmp_mirrorea_artifacts.sh`.
- Confirmed targeted tests pass after implementation.
- Updated `scripts/README.md`, `plan/148`, `progress.md`, and `tasks.md`.
- Added this report.

## Files changed

- `scripts/storage/tmp_mirrorea_artifacts.sh`
- `scripts/tests/test_tmp_mirrorea_artifacts.py`
- `scripts/README.md`
- `plan/148-storage-workdir-mountpoint-guard-hardening.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2238-p100-tmp-mirrorea-artifact-helper.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `date '+%Y-%m-%d %H:%M %Z'`
- `git status --short --branch`
- `df -h .`
- `free -h`
- `lsblk -f`
- `findmnt -T .`
- `findmnt /mnt/mirrorea-work`
- `du -sk .`
- `du -sk target`
- `du -sk .git`
- `du -sk .cargo`
- `du -sk .lake`
- `bash scripts/env/mirrorea_storage_env.sh`
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
- `bash scripts/storage/detach_prepare.sh`
- `find /tmp -maxdepth 1 -type d -name 'mirrorea-p99-*' -print | wc -l`
- `du -skc /tmp/mirrorea-p99-* | tail -n 1`
- `find /tmp -maxdepth 1 -type d -name 'mirrorea-*' -print | wc -l`
- `du -skc /tmp/mirrorea-* | tail -n 1`
- `python3 -m unittest scripts.tests.test_tmp_mirrorea_artifacts` (RED)
- `chmod +x scripts/storage/tmp_mirrorea_artifacts.sh`
- `python3 -m unittest scripts.tests.test_tmp_mirrorea_artifacts` (GREEN)
- `bash scripts/storage/tmp_mirrorea_artifacts.sh --list | tail -n 4`
- `find /tmp -maxdepth 1 -type d -name 'mirrorea-*' -print | wc -l`
- `du -skc /tmp/mirrorea-* 2>/dev/null | tail -n 1`
- `python3 -m unittest scripts.tests.test_storage_workdir_guards`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `python3 -m unittest discover -s scripts/tests`
- `rg -n --hidden --glob '!/.git/**' --glob '!/.codex-discord/**' 'https://discord(?:app)?\\.com/api/webhooks/[0-9]+/[A-Za-z0-9_-]+' .`
- `bash -n scripts/storage/tmp_mirrorea_artifacts.sh`
- `make check`
- `cargo test --workspace --all-targets --no-fail-fast`

## Evidence / outputs / test results

TDD evidence:

- RED: `python3 -m unittest scripts.tests.test_tmp_mirrorea_artifacts` failed
  with three failures because `scripts/storage/tmp_mirrorea_artifacts.sh` did
  not exist yet.
- GREEN: after implementing the helper, the same command ran 3 tests in
  0.027s and returned OK.

Helper behavior covered by tests:

- `--list` reports only immediate `mirrorea-*` directories under the tmp root,
  includes candidate count / total KiB, and does not delete candidates.
- `--cleanup` without `--confirm` exits 2 and preserves candidates.
- `--cleanup --confirm` deletes only immediate `mirrorea-*` directories in a
  temporary fixture root, preserving unrelated directories and `mirrorea-*`
  files.

Live audit evidence:

- `/mnt/mirrorea-work` was not mounted.
- P99-specific tmp outputs occupied 378232 KiB.
- The helper closeout run reported `candidate_count=3348` and
  `total_kib=25079200` for immediate directory cleanup candidates.
- A broad glob `du -skc /tmp/mirrorea-*` reported 25090344 KiB in the same
  window; the helper intentionally targets only immediate directories.
- No real `/tmp/mirrorea-*` cleanup was run.

Closeout validation:

- `bash scripts/storage/tmp_mirrorea_artifacts.sh --list | tail -n 4`: exit 0;
  reported `candidate_count=3348` and `total_kib=25079200`.
- `python3 -m unittest scripts.tests.test_tmp_mirrorea_artifacts`: 3 tests,
  OK.
- `python3 -m unittest scripts.tests.test_storage_workdir_guards`: 3 tests,
  OK.
- `python3 -m unittest discover -s scripts/tests`: 793 tests, OK.
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1390
  numbered reports.
- `python3 scripts/check_source_hierarchy.py`: required 691, present 691,
  missing 0.
- `git diff --check`: exit 0.
- Workspace concrete Discord webhook URL scan excluding `.git/` and
  `.codex-discord/`: no matches.
- `bash -n scripts/storage/tmp_mirrorea_artifacts.sh`: exit 0.
- `make check`: source hierarchy check, docs validation, and `cargo check`
  passed.
- `cargo test --workspace --all-targets --no-fail-fast`: exit 0.

## What changed in understanding

The root disk pressure is not only repo-local `target/`. Long validation runs
have also accumulated a large `/tmp/mirrorea-*` footprint. The repo now has a
minimal safety helper to inspect that footprint and to require an explicit
confirmation path before cleanup.

This does not change the phase reading: canon remains `T0/G0 rebaseline`,
human-count phase 1 of 9, late pre-exit inside T0, with G0 exit unclaimed.

## Open questions

No new project-theory open question was created.

Operationally, actual cleanup of the existing `/tmp/mirrorea-*` entries remains
an explicit action outside this package. The helper is ready to list them, but
P100 intentionally did not delete them.

## Suggested next prompt

Continue autonomous Macro 0 maintenance or explicitly choose a G1 review-facing
extraction line, such as OBL-020 scope or OBL-001 artifact identity, if the next
work should move toward human/canon review preparation.

## Plan update status

`plan/148-storage-workdir-mountpoint-guard-hardening.md` 更新済み: added the
P100 tmp artifact follow-up, live `/tmp/mirrorea-*` footprint reading, helper
contract, non-claims, and next-use note.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing project overview, canon hierarchy,
roadmap, or source-hierarchy summary changed.

## progress.md update status

`progress.md` 更新済み: added the P100 tmp artifact helper note, Macro 0 row
update, and recent log entry.

## tasks.md update status

`tasks.md` 更新済み: added the P100 holding-state note and storage / tmp
artifact maintenance rows.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, workflow readiness, sample
validation command, or sample blocker changed.

## Reviewer findings and follow-up

No separate reviewer or sub-agent was used for P100. The package is a small
storage helper with test-first coverage and no real cleanup side effect.

Local diff inspection found no broad refactor or unrelated source changes.

## Skipped validations and reasons

Real `/tmp/mirrorea-*` cleanup was intentionally skipped. This package adds a
guarded helper and tests; deletion requires a separate explicit cleanup action.

No planned P100 validation was skipped. Full workspace Cargo tests were rerun
after report creation and passed.

## Commit / push status

Not yet committed at initial report creation. This section will be updated
after validation, commit, and push.

## Sub-agent session close status

No P100 sub-agent session was opened. There is no P100 sub-agent to close.
