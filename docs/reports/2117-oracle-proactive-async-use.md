# Report 2117 — Oracle proactive async use

- Date: 2026-07-02 18:03 JST
- Author / agent: Codex
- Scope: Oracle consult operating policy clarification for proactive asynchronous use
- Decision levels touched: none; operational policy only

## Objective

Record the user's instruction that ChatGPT 5.5 Pro Extended Oracle should be
used proactively for theory-heavy, whole-project, roadmap, and complex review
tasks, with long-running browser consultations treated as asynchronous review
jobs where appropriate.

## Scope and assumptions

- This update only changes agent / operational policy.
- It does not change normative project semantics or promote any `specs/`
  decision.
- Oracle remains advisory review input.
- Sub-agents may operate or monitor Oracle runs, but the main agent remains
  responsible for evaluating the result against repo evidence and for any repo
  edits.

## Start state / dirty state

The tracked worktree was clean on `main...origin/main` before edits.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `AGENTS.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Updated `.docs/oracle-chatgpt-pro-operations.md` so theory-heavy and
   whole-project tasks explicitly prefer proactive Oracle consults.
2. Added an asynchronous and sub-agent coordination section to the Oracle
   operations note.
3. Updated `AGENTS.md` with the same high-level operating rule.
4. Updated `progress.md` timestamp and recent log.
5. Added this report.

## Files changed

- `.docs/oracle-chatgpt-pro-operations.md`
- `AGENTS.md`
- `progress.md`
- `docs/reports/2117-oracle-proactive-async-use.md`

## Commands run

```bash
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,280p' Documentation.md
sed -n '1,430p' progress.md
git status --short --branch
sed -n '1,260p' specs/00-document-map.md
sed -n '1,180p' specs/01-charter-and-decision-levels.md
sed -n '1,160p' specs/02-system-overview.md
sed -n '1,200p' specs/03-layer-model.md
sed -n '1,180p' specs/09-invariants-and-constraints.md
sed -n '1,240p' .docs/oracle-chatgpt-pro-operations.md
sed -n '1,80p' AGENTS.md
sed -n '1,220p' docs/reports/TEMPLATE.md
find docs/reports -maxdepth 1 -type f -name '*.md' | sort | tail -10
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
rg -n "Spt52|discord.com/api/webhooks|/home/codex/dev/tmp|5\\.5-pro\\) Mirrorea|config.local.json" .docs/oracle-chatgpt-pro-operations.md AGENTS.md progress.md docs/reports/2117-oracle-proactive-async-use.md
```

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`: all 546 required paths present.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1269
  numbered reports found.
- `python3 -m unittest scripts.tests.test_validate_docs`: 18 tests passed.
- `git diff --check`: passed.
- Secret / temporary-path scan did not find the Discord webhook URL or temporary
  consultation archive path in the touched files. It only matched the existing
  `AGENTS.md` mention of `.codex-discord/config.local.json` as the local
  uncommitted config path.

## What changed in understanding

Oracle should not be reserved only for blocked states. For theory-heavy,
whole-project, roadmap, or complex design review work, it should be used early
and proactively when it is likely to improve the decision. Long response time is
expected; the useful pattern is to run it as an asynchronous reviewer while
continuing non-overlapping local work.

## Open questions

- None for this operational clarification.

## Suggested next prompt

Create the first non-normative planning ledger for axis/non-axis, semantic
strata, ordinary assignment target obligation, and open questions for promotion.
Use Oracle proactively if the theory framing or whole-project management
questions become non-trivial.

## Plan update status

`plan/` 更新不要:

- No repository-memory or roadmap content changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- The existing pointer to `.docs/oracle-chatgpt-pro-operations.md` remains
  sufficient.

## progress.md update status

`progress.md` 更新済み:

- Updated `最終更新`.
- Added a recent log entry for the proactive asynchronous Oracle usage rule.

## tasks.md update status

`tasks.md` 更新不要:

- The current task map and promoted package status did not change.

## samples_progress.md update status

`samples_progress.md` 更新不要:

- No runnable sample, validation command, debug surface, or blocker changed.

## Reviewer findings and follow-up

Local self-review only. The change is a narrow operational policy clarification.

## Skipped validations and reasons

- No live Oracle consultation was run because this task only records when to use
  Oracle in future work.
- Cargo / Rust tests and broad build checks were not run because this task
  touched only docs and operational policy.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent was spawned for this narrow update.
