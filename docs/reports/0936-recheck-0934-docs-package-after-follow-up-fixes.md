# 0936 Recheck 0934 Docs Package After Follow-Up Fixes

## Objective

`0934` cross-package sweep docs-only package を follow-up fix 後に再確認し、指定された 4 条件が満たされているかを verify する。

## Scope and assumptions

- scope は user 指定の同一 docs package file 群に限る。
- verify 対象は次の 4 点のみとする。
  - `cross-package sweep` が current active と already closed の両義記述を脱しているか
  - `FAIRY-05` が closeout guide の `remaining mixed gate` から外れたか
  - `tasks.md` の next 2 package が適切な macro phase に置かれたか
  - `0934` report の planned-family summary に network-transport planned family が入ったか
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
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0934-cross-package-sweep-and-next-queue-recut.md`

## Actions taken

1. required repository reading order を再確認した。
2. scope files で `cross-package sweep`、`FAIRY-05`、`Typed external boundary / adapter executable widening`、`Macro 6/7`、`network-transport` を grep し、fix 後 wording を点検した。
3. `tasks.md`、`samples_progress.md`、`docs/hands_on/current_phase_closeout_01.md`、`docs/reports/0934-*` の relevant line を line-number 付きで確認した。
4. remaining finding の有無だけを user-facing review に返す準備をした。

## Evidence / outputs / test results

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,260p' progress.md`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,220p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,220p' specs/02-system-overview.md`
- `sed -n '1,220p' specs/03-layer-model.md`
- `sed -n '1,220p' specs/09-invariants-and-constraints.md`
- `rg -n "cross-package sweep|current active packages|remaining mixed gate|FAIRY-05|Typed external boundary / adapter executable widening|Macro 6|Macro 7|planned family|network-transport" README.md Documentation.md progress.md tasks.md samples_progress.md docs/hands_on/current_phase_closeout_01.md docs/research_abstract/mirrorea_future_axis_01.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md specs/11-roadmap-and-workstreams.md docs/reports/0934-cross-package-sweep-and-next-queue-recut.md`
- `nl -ba tasks.md | sed -n '130,180p'`
- `nl -ba samples_progress.md | sed -n '1,20p'`
- `nl -ba docs/hands_on/current_phase_closeout_01.md | sed -n '78,104p'`
- `nl -ba docs/reports/0934-cross-package-sweep-and-next-queue-recut.md | sed -n '39,60p'`
- validation / tests は未実行。今回の recheck は docs wording review のみ。

## What changed in understanding

- follow-up fixes で targeted drift の大半は解消している。
- ただし `samples_progress.md` は header が `Current active packages` のままで、その直下に `recent close 0934` を並べているため、package status wording がまだ半歩だけ噛み合っていない。

## Open questions

- `samples_progress.md` 冒頭の label は `Current queue` へ変えるのか、あるいは `0934` をその行から完全に外すのか。

## Suggested next prompt

`samples_progress.md` 冒頭の `Current active packages` wording を queue/close status に合う形へ 1 行だけ修正し、`0934` を active package と誤読しない表現に揃えてください。
