# Report 2243 — P105 Discord notification file inputs

- Date: 2026-07-05 03:39 JST
- Author / agent: Codex
- Scope: Macro 0 notification operation hardening for the repo-scoped Discord report skill.
- Decision levels touched: LAB repository-memory / operational helper only; no canon decision movement.

## Objective

Close a concrete operations trigger from P104 closeout: inline Discord
notification text containing a backtick-wrapped commit hash was interpreted by
the shell before `discord_notify.py` received the summary. Add a safer
file-based input path for notification text and document when to use it.

## Scope and assumptions

Scope is limited to the repo-scoped `discord-report` helper, its instructions,
tests, and repository memory. The fix should preserve existing inline
`--summary` / `--next-step` behavior for short plain text and add
`--summary-file` / `--next-step-file` for shell-sensitive text.

This does not change webhook storage, delivery guarantees, rate limiting,
begin/progress/complete semantics, phase/gate status, proof/conformance,
runtime readiness, or sample status.

## Start state / dirty state

Start state was clean and synced with `origin/main`:
`## main...origin/main`.

## Documents consulted

- `AGENTS.md`
- `.agents/skills/discord-report/SKILL.md`
- `.agents/skills/discord-report/scripts/discord_notify.py`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/151-discord-webhook-secret-validator-guard.md`

## Actions taken

- Investigated the P104 notification issue and identified shell command
  substitution at the inline `--summary` boundary as the root cause.
- Added RED tests for `--summary-file` and `--next-step-file`.
- Added UTF-8 file input support to `discord_notify.py`.
- Updated `.agents/skills/discord-report/SKILL.md` and `AGENTS.md` so future
  notifications with backticks, quotes, newlines, or long commit lists use
  file inputs instead of inline shell text.
- Added `plan/152-discord-notification-file-inputs.md`.
- Registered `plan/152` in `scripts/validate_docs.py`,
  `scripts/check_source_hierarchy.py`, `scripts/tests/test_validate_docs.py`,
  and `plan/00-index.md`.
- Updated `Documentation.md`, `progress.md`, `tasks.md`, and `scripts/README.md`.

## Files changed

- `.agents/skills/discord-report/SKILL.md`
- `.agents/skills/discord-report/scripts/discord_notify.py`
- `scripts/tests/test_discord_notify_skill.py`
- `AGENTS.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `scripts/README.md`
- `scripts/validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `plan/00-index.md`
- `plan/152-discord-notification-file-inputs.md`
- `docs/reports/2243-p105-discord-notification-file-inputs.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch && git log -5 --oneline`
- `rg -n 'discord_notify|discord-report|summary|next-step|backtick|shell|quote|quoting|complete|progress' .agents scripts docs/reports/2242-p104-phase-position-late-pre-exit-guard.md AGENTS.md`
- `python3 -m unittest scripts.tests.test_discord_notify_skill`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `python3 -m unittest discover scripts/tests`
- `if git grep -l -E 'discord\\.com/api/webhooks/[0-9]+/[A-Za-z0-9_-]{40,}' -- . ':!.codex-discord'; then echo 'tracked concrete Discord webhook URL candidate found'; exit 1; else echo 'no tracked concrete Discord webhook URLs found'; fi`
- `make check`
- `cargo test`
- `find samples/lean -path 'samples/lean/old' -prune -o -name '*.lean' -print0 | xargs -0 -n1 lean`

## Evidence / outputs / test results

- RED run failed before implementation:
  argparse reported `unrecognized arguments: --summary-file ...` and
  `unrecognized arguments: --summary-file ... --next-step-file ...`.
- GREEN focused run passed after implementation:
  `Ran 2 tests in 0.005s` / `OK`.
- `python3 -m unittest scripts.tests.test_validate_docs` passed:
  `Ran 45 tests in 3.381s` / `OK`.
- `python3 scripts/validate_docs.py` passed:
  `Documentation scaffold looks complete.` / `Found 1395 numbered report(s).`
- `python3 scripts/check_source_hierarchy.py` passed:
  required `699`, present `699`, missing `0`.
- `git diff --check` passed with no output.
- `python3 -m unittest discover scripts/tests` passed:
  `Ran 798 tests in 25.589s` / `OK`.
- Tracked concrete Discord webhook URL scan passed:
  `no tracked concrete Discord webhook URLs found`.
- `make check` passed, including source hierarchy, docs validation, and
  `cargo check`.
- `cargo test` passed for Rust unit/integration/doc tests.
- Active Lean files under `samples/lean/` excluding `samples/lean/old/`
  compiled with `lean` and produced no errors.

## What changed in understanding

The notifier payload builder already preserves literal backticks once text
reaches Python. The unsafe boundary was constructing shell commands with inline
message text. File-based inputs are a narrow way to avoid shell metacharacter
interpretation without changing Discord payload semantics.

## Open questions

- No new project-level open question was introduced.
- The broader next-work boundary remains unchanged: OBL-020 / OBL-001
  review-facing extraction still requires explicit user selection, and Macro 0
  work still requires a fresh concrete trigger.

## Suggested next prompt

Choose `OBL-020 review-facing decision request extraction`,
`OBL-001 review-facing artifact decision request extraction`, or name a
specific Macro 0 audit surface if another repository-management package should
be promoted.

## Plan update status

`plan/` 更新済み: added
`plan/152-discord-notification-file-inputs.md` and registered it in
`plan/00-index.md`.

## Documentation.md update status

`Documentation.md` 更新済み: added a short `plan/152` note.

## progress.md update status

`progress.md` 更新済み: updated `最終更新`, current note, Macro 0 row, and recent
log with P105.

## tasks.md update status

`tasks.md` 更新済み: updated `最終更新`, current holding state, and Macro 0 row
with P105.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample status, sample path,
validation command, debug surface, or sample blocker changed.

## Reviewer findings and follow-up

No separate reviewer, Oracle, or sub-agent was used. This package is a narrow
operations helper fix with local root-cause evidence and RED/GREEN tests.

## Skipped validations and reasons

No planned validation was intentionally skipped. Sub-agent / Oracle review was
not run because the change is a narrow operations helper fix with local
root-cause evidence and RED/GREEN tests.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent was opened for P105. The scope was narrow and did not have an
independent review surface worth splitting out.
