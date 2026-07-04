# Report 2235 - phase-position validator guard

- Date: 2026-07-05 02:06 JST
- Author / agent: Codex
- Scope: Macro 0 docs-validator guard hardening
- Decision levels touched: LAB repository memory only

## Objective

Keep the current phase-position reading from `plan/149` visible in
`progress.md` and `tasks.md` by adding a focused `scripts/validate_docs.py`
guard and documenting the maintenance package.

## Scope and assumptions

This task is limited to repository management and validation. It assumes
`plan/149-current-phase-position-reading.md` remains the current LAB answer:
canon lifecycle is `T0/G0 rebaseline`, human-count phase is phase 1 of 9, T0
is late pre-exit, and G0 exit remains unclaimed.

## Start state / dirty state

Start state was clean and synced on `main` at
`7e41497ff2e11dd245a7be4013db35c851e58b2c`.

At report creation, the worktree contains the intentional P97 edits to
validator tests, validator code, repository memory, snapshot docs, and this
report.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/149-current-phase-position-reading.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `docs/reports/TEMPLATE.md`

## Actions taken

- Added RED tests for missing phase-position guard phrases in `progress.md`
  and `tasks.md`.
- Confirmed the RED failure: both tests failed because `validate_docs.main()`
  returned `0` before the guard existed.
- Added `SNAPSHOT_PHASE_POSITION_GUARD_PHRASES` and
  `missing_phase_position_guard_phrases()` to `scripts/validate_docs.py`.
- Added `plan/150-phase-position-validator-guard.md`.
- Registered `plan/150` in docs/source hierarchy validators and test fixtures.
- Updated `README.md`, `Documentation.md`, `scripts/README.md`,
  `plan/00-index.md`, `plan/90-source-traceability.md`, `progress.md`, and
  `tasks.md`.

## Files changed

- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/README.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/150-phase-position-validator-guard.md`
- `docs/reports/2235-phase-position-validator-guard.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- `date '+%Y-%m-%d %H:%M %Z'`
- `rg -n "PROGRESS_REQUIRED_HEADINGS|TASKS_REQUIRED_HEADINGS|REQUIRED|stale|progress.md|tasks.md|def main" scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_progress_missing_phase_position_guard scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_tasks_missing_phase_position_guard`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `python3 -m unittest discover -s scripts/tests`
- `make check`
- `cargo fmt --check`
- `cargo test --workspace --all-targets --no-fail-fast`
- Discord webhook secret scan with the local denylist pattern omitted from this
  report.
- `git add README.md Documentation.md progress.md tasks.md scripts/README.md scripts/check_source_hierarchy.py scripts/tests/test_validate_docs.py scripts/validate_docs.py plan/00-index.md plan/90-source-traceability.md plan/150-phase-position-validator-guard.md docs/reports/2235-phase-position-validator-guard.md`
- `git commit --no-gpg-sign -m "Guard phase position snapshot wording"`
- `git push`

## Evidence / outputs / test results

RED result:

```text
FAIL: test_main_rejects_progress_missing_phase_position_guard
AssertionError: 0 != 1

FAIL: test_main_rejects_tasks_missing_phase_position_guard
AssertionError: 0 != 1
```

GREEN result:

```text
Ran 2 tests in 0.168s
OK
```

Focused and full validation:

- `python3 -m unittest scripts.tests.test_validate_docs`: 39 tests, OK.
- `python3 scripts/check_source_hierarchy.py`: required 690, present 690,
  missing 0.
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1387
  numbered reports.
- `git diff --check`: exit 0.
- `python3 -m unittest discover -s scripts/tests`: 787 tests, OK.
- `make check`: source hierarchy check, docs validation, and `cargo check`
  passed.
- `cargo fmt --check`: exit 0.
- `cargo test --workspace --all-targets --no-fail-fast`: exit 0.
- Secret scan: no Discord webhook matches in tracked files.

Primary package commit and push:

```text
[main 291a76ea] Guard phase position snapshot wording
To github.com:yukatayu/mir_poc_01.git
   7e41497f..291a76ea  main -> main
```

## What changed in understanding

`progress.md` and `tasks.md` already carried the correct `plan/149` reading,
but the docs validator only checked section shape and freshness. The new guard
keeps the concise phase-position answer from disappearing during future
snapshot maintenance.

## Open questions

None for this package. Any actual phase / gate movement still requires the
canon process, not this validator.

## Suggested next prompt

Continue the autonomous run with the next coherent Macro 0 or G1 maintenance
package after this guard is validated, reviewed, committed, and pushed.

## Plan update status

`plan/` updated: added `plan/150-phase-position-validator-guard.md`, updated
`plan/00-index.md`, and updated `plan/90-source-traceability.md`.

## Documentation.md update status

`Documentation.md` updated: added the `plan/150` guard note.

## progress.md update status

`progress.md` updated: added the phase-position validator guard note, Macro 0
row update, and 2026-07-05 02:06 JST recent log entry.

## tasks.md update status

`tasks.md` updated: added the `plan/150` guard note, maintenance refresh row,
and Macro 0 row update.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample, validation
command, debug surface, or sample blocker changed.

## Reviewer findings and follow-up

Read-only reviewer sub-agent `019f2e1b-938b-7bc1-9be5-262d4dbd0789` reported
no Critical issues.

Important finding: this report was stale after validation and still said full
validation / review were pending. Follow-up: updated this report with the
actual validation commands and reviewer status.

Minor finding: the guard is phrase-presence coverage only and will not catch a
semantic inversion that leaves the required phrases in place. Follow-up: accepted
as the intended scope for this package; `plan/150` and this report state that
the guard is management drift protection, not a semantic phase validator.

## Skipped validations and reasons

No relevant local validations were skipped for this docs-validator package.

## Commit / push status

Primary package commit `291a76ea9093a03418c72c3bc6eee32fc6b067ab`
(`Guard phase position snapshot wording`) was pushed to `origin/main`.
This report status update is being recorded in a follow-up status-only commit.

## Sub-agent session close status

Reviewer sub-agent `019f2e1b-938b-7bc1-9be5-262d4dbd0789` completed and was
closed after report update validation.
