# 1036 — tasks maintenance wording reconciliation

## Objective

`tasks.md` に残っていた `current_l2_guided_samples.py` emit-family historical-memory closeout の obsolete residual note を外し、same-day 20:50 JST の reopen/split/lane/residual cluster closeout と current task map を再同期する。

## Scope and assumptions

- scope は snapshot-docs only の maintenance closeout に限る。
- 規範正本、`specs/`、`plan/`、sample taxonomy、runtime behavior は変更しない。
- current queue authority は `tasks.md` / `progress.md` を正とし、historical closeout memory と current self-driven ordering を混同しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `docs/reports/1029-current-l2-guided-bundle-cluster-and-check-source-sample-cooling.md`
- `progress.md` recent log entry for 2026-04-30 20:50 JST

## Actions taken

1. Verified that the stale note existed only in `tasks.md` and not as an unresolved current package elsewhere in active snapshot docs.
2. Rewrote the `emit-family historical-memory cooling` bullet in `tasks.md` so it now states that the same-day `reopen/split/lane/residual` residual line is already closed.
3. Added a short `progress.md` recent-log entry so the snapshot records why `tasks.md` changed.
4. Added this report for traceability.

## Commands run

- `rg -n 'next stale-docs maintenance package|remaining .*reopen-map|remaining .*lane|remaining .*residuals|next maintenance line' tasks.md progress.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md plan`
- `git status --short`
- `git branch --show-current`
- `git log -1 --oneline`
- `date '+%Y-%m-%d %H:%M JST'`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Files changed

- `tasks.md`
- `progress.md`
- `docs/reports/1036-tasks-maintenance-wording-reconciliation.md`

## Evidence / outputs / test results

- `rg -n 'next stale-docs maintenance package|remaining .*reopen-map|remaining .*lane|remaining .*residuals|next maintenance line' ...`
  - active snapshot drift was limited to `tasks.md:23`
- `git status --short`
  - before edits: clean after commit `11db069`
- `git branch --show-current`
  - `main`
- `git log -1 --oneline`
  - `11db069 Cool authority witness and RNG wording`
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- residual queue drift was no longer in `specs/` or repository-memory docs; it had collapsed to a single stale sentence in `tasks.md`.
- current snapshot maintenance ordering benefits from explicit same-day reconciliation entries in `progress.md` when a task-map-only stale note is removed.

## Open questions

- none for this narrow package. Wider stale-docs maintenance can continue opportunistically if another active snapshot drift is found.

## Suggested next prompt

`U1` 待ちのまま自走を続けるなら、active snapshot docs に残る単発の stale maintenance or queue wording を同じ narrow maintenance package で順次回収する。

## plan/ 更新の有無

- 更新不要

## progress.md 更新の有無

- 更新した

## tasks.md 更新の有無

- 更新した

## samples_progress.md 更新の有無

- 更新不要

## skipped validations and reasons

- sample / cargo / full validation floor は未実行。今回は `tasks.md` / `progress.md` / report だけの snapshot-docs maintenance closeout であり、source hierarchy / docs scaffold / diff check を focused validation とした。

## commit / push status

- report authoring時点では未実行。same-package closeout で commit / push を行う。

## sub-agent session close status

- この package では sub-agent 未使用。
