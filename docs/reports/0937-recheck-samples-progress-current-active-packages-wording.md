# 0937 Recheck Samples Progress Current Active Packages Wording

## Objective

`samples_progress.md` 冒頭の `Current active packages` wording を再確認し、closed な `0934` cross-package sweep が active package として残っていないことを verify する。

## Scope and assumptions

- scope は `samples_progress.md` 冒頭の package-status wording と、その判断に必要な最小限の snapshot / spec context に限る。
- unrelated dirty Rust files と untracked handoff は対象外。
- 本 task は review-only。
- `plan/` 更新不要。
- `progress.md` 更新不要。
- `tasks.md` 更新不要。
- `samples_progress.md` 更新不要。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `samples_progress.md`

## Actions taken

1. required repository reading order を brief に再確認した。
2. `samples_progress.md` 冒頭 12 行を line-number 付きで確認した。
3. `Current active packages` field が promoted / reopen queue のみを列挙しているかを点検した。

## Evidence / outputs / test results

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `sed -n '1,140p' README.md`
- `sed -n '1,180p' Documentation.md`
- `sed -n '1,120p' progress.md`
- `sed -n '1,160p' specs/00-document-map.md`
- `sed -n '1,160p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,160p' specs/02-system-overview.md`
- `sed -n '1,160p' specs/03-layer-model.md`
- `sed -n '1,160p' specs/09-invariants-and-constraints.md`
- `nl -ba samples_progress.md | sed -n '1,12p'`
- validation / tests は未実行。今回の task は docs wording review のみ。

## What changed in understanding

- `samples_progress.md` 冒頭は `promoted` と `reopen` の queue のみを示す形に揃い、closed package を active slot へ残す drift は解消した。

## Open questions

- なし。

## Suggested next prompt

必要なら `0934` docs package 全体の final consistency pass を行い、report / snapshot / hands-on / plan の queue wording が同じ二語彙 (`promoted`, `reopen`) で揃っているかを横断確認してください。
