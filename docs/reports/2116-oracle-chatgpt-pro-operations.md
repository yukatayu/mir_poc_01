# Report 2116 — Oracle ChatGPT Pro operations note

- Date: 2026-07-02 17:52 JST
- Author / agent: Codex
- Scope: ChatGPT 5.5 Pro Extended Oracle consult operating notes and repo-local documentation links
- Decision levels touched: none; operational policy only

## Objective

Record how agents should use the installed browser-backed ChatGPT 5.5 Pro
Extended Oracle commands when a difficult judgment, review, or stuck
investigation would benefit from a second opinion.

## Scope and assumptions

- The change is operational documentation, not normative project semantics.
- Oracle output remains advisory review input and does not replace `specs/`,
  `plan/`, `progress.md`, `tasks.md`, or `docs/reports/`.
- No live Oracle consultation was needed for this narrow documentation task.
  The local Oracle manual was read and the local command wrappers were verified
  with non-live checks.
- The new note must include the user instruction that Oracle runs can take
  minutes and sometimes up to about one hour, so agents should wait patiently
  and avoid duplicate runs without concrete failure evidence.

## Start state / dirty state

`git status --short --branch` at start showed a clean tracked worktree on
`main...origin/main`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/69-consultation-synthesis-and-management-roadmap.md`
- `AGENTS.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Added `.docs/oracle-chatgpt-pro-operations.md` as the repo-local operating
   note for Oracle browser consults.
2. Updated `AGENTS.md` so future agents can find the local Oracle manual and
   repo-local operating note before using the commands.
3. Updated `Documentation.md` to include `.docs/` as operational policy and to
   point readers to the Oracle operations note.
4. Updated `plan/00-index.md` to record `.docs/oracle-chatgpt-pro-operations.md`
   as operational policy, not repository memory or normative source.
5. Updated `progress.md` with a timestamped recent log entry.
6. Verified the command wrappers exist and that `ask-chatgpt-pro --dry-run`
   resolves in browser mode for `gpt-5.5-pro`.
7. Asked a read-only sub-agent reviewer to check placement and wording risks.

## Files changed

- `.docs/oracle-chatgpt-pro-operations.md`
- `AGENTS.md`
- `Documentation.md`
- `plan/00-index.md`
- `progress.md`
- `docs/reports/2116-oracle-chatgpt-pro-operations.md`

## Commands run

```bash
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
sed -n '1,220p' /home/codex/.codex/superpowers/skills/using-superpowers/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' .docs/progress-task-axes.md
sed -n '1,260p' /home/codex/.codex/docs/oracle-chatgpt-pro.md
wc -l specs/00-*.md specs/01-*.md specs/02-*.md specs/03-*.md specs/09-*.md
sed -n '1,260p' tasks.md
sed -n '1,260p' plan/00-index.md
sed -n '1,620p' plan/69-consultation-synthesis-and-management-roadmap.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,180p' specs/09-invariants-and-constraints.md
sed -n '1,650p' specs/00-document-map.md
rg -n "Oracle|ChatGPT|ask-chatgpt|oracle|5\\.5|相談" . --glob '!target/**' --glob '!.git/**' --glob '!tmp/**'
find docs/reports -maxdepth 1 -type f -name '*.md' | sort | tail -20
ls -la .docs docs plan
git status --short --branch
sed -n '1,220p' AGENTS.md
sed -n '1,220p' docs/reports/TEMPLATE.md
sed -n '320,390p' progress.md
sed -n '1,120p' .git/info/exclude
date '+%Y-%m-%d %H:%M %Z'
command -v oracle
command -v ask-chatgpt-pro
command -v ask-chatgpt-pro-followup
command -v ask-chatgpt-pro-temp
oracle --version
ask-chatgpt-pro --dry-run summary -p "dry run"
oracle status --hours 1 --limit 10
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
```

## Evidence / outputs / test results

- `command -v oracle` returned
  `/home/codex/.nvm/versions/node/v25.9.0/bin/oracle`.
- `command -v ask-chatgpt-pro` returned `/home/codex/.local/bin/ask-chatgpt-pro`.
- `command -v ask-chatgpt-pro-followup` returned
  `/home/codex/.local/bin/ask-chatgpt-pro-followup`.
- `command -v ask-chatgpt-pro-temp` returned
  `/home/codex/.local/bin/ask-chatgpt-pro-temp`.
- `oracle --version` returned `0.15.0`.
- `ask-chatgpt-pro --dry-run summary -p "dry run"` reported Oracle `0.15.0`,
  browser mode, `gpt-5.5-pro`, and no files attached.
- `oracle status --hours 1 --limit 10` returned recent browser sessions and
  confirmed the status command works.
- `python3 scripts/check_source_hierarchy.py`: all 546 required paths present.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1268
  numbered reports found after this report was added.
- `python3 -m unittest scripts.tests.test_validate_docs`: 18 tests passed.
- `git diff --check`: passed.

## What changed in understanding

Oracle should be treated as a slow, high-quality advisory reviewer. The repo
needs an explicit waiting policy because the command can take minutes and, in
some cases, around an hour. This is compatible with the existing source
hierarchy only if useful results are distilled back into repo documents and not
treated as an external normative source.

## Open questions

- None for this operational note.
- Future high-risk theory or implementation packages can decide case-by-case
  whether to run an actual Oracle consultation.

## Suggested next prompt

Create the first non-normative planning ledger for axis/non-axis, semantic
strata, ordinary assignment target obligation, and open questions for promotion.
Do not edit `specs/` yet.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md` now points to `.docs/oracle-chatgpt-pro-operations.md` as
  operational policy.

## Documentation.md update status

`Documentation.md` 更新済み:

- Added `.docs/` as operational policy and added the Oracle consult operations
  pointer.

## progress.md update status

`progress.md` 更新済み:

- Updated `最終更新`.
- Added a recent log entry for the Oracle operations note.

## tasks.md update status

`tasks.md` 更新不要:

- The current task map and promoted package status did not change.

## samples_progress.md update status

`samples_progress.md` 更新不要:

- No runnable sample, validation command, debug surface, or blocker changed.

## Reviewer findings and follow-up

Sub-agent reviewer findings:

- The new `.docs/oracle-chatgpt-pro-operations.md` must be committed with the
  tracked references.
- A new report is required for this non-trivial operational policy change.
- Placement is sound: no `specs/` edit is needed; `.docs/` plus links from
  `AGENTS.md`, `Documentation.md`, `plan/00-index.md`, `progress.md`, and this
  report are sufficient.
- No secret leakage or temporary archive/profile path leakage was found in the
  repo-local wording.
- The advisory status and wait/no-duplicate-run policy are clear.

Follow-up:

- This report adds the required report.
- The new `.docs/` file is included in the intended commit.

## Skipped validations and reasons

- No live Oracle consultation was run; this task was the setup and operating
  note itself, and the dry-run check was sufficient to verify command routing
  without spending a long browser-run cycle.
- Cargo / Rust tests and broad build checks were not run because this task
  touched only documentation and operational policy, not code, samples, or
  validators.

## Commit / push status

Pending at report write. This report will be included in the same commit as the
operational note and pushed after final validation.

## Sub-agent session close status

The read-only reviewer sub-agent completed and was closed.
