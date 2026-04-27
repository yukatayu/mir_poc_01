# 0935 Review Cross-Package Sweep Docs Package

## Objective

`0934` cross-package sweep docs-only package を review し、queue taxonomy consistency、package close wording、active / planned / mixed-gate の切り分け、next promoted package / reopen point の整合性を確認する。

## Scope and assumptions

- scope は user 指定の 12 file に限る。
- unrelated dirty Rust files と untracked handoff は review 対象外とする。
- 本 task は review-only であり、snapshot docs の修正は行わない。
- `plan/` 更新不要。
- `progress.md` 更新不要。
- `tasks.md` 更新不要。
- `samples_progress.md` 更新不要。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0934-cross-package-sweep-and-next-queue-recut.md`

## Actions taken

1. required repository reading order を満たした。
2. review scope の各文書で `FAIRY-05`、`Typed external boundary / adapter executable widening`、`cross-package sweep`、`mixed gate`、`Macro 6/7` の記述を突き合わせた。
3. `tasks.md`、`progress.md`、`samples_progress.md`、`plan/17` の queue / phase taxonomy を比較した。
4. `docs/hands_on/current_phase_closeout_01.md` と `docs/research_abstract/mirrorea_future_axis_01.md` が next queue と mixed-gate wording をどう reader-facing に見せるかを確認した。
5. findings は user-facing review にまとめ、repo にはこの report のみ追加した。

## Evidence / outputs / test results

- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,260p' progress.md`
- `sed -n '1,260p' .docs/progress-task-axes.md`
- `sed -n '1,260p' specs/00-document-map.md`
- `sed -n '1,260p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `sed -n '1,320p' specs/11-roadmap-and-workstreams.md`
- `sed -n '1,320p' tasks.md`
- `sed -n '1,320p' samples_progress.md`
- `sed -n '1,320p' docs/hands_on/current_phase_closeout_01.md`
- `sed -n '1,320p' docs/research_abstract/mirrorea_future_axis_01.md`
- `sed -n '1,320p' plan/01-status-at-a-glance.md`
- `sed -n '1,320p' plan/11-roadmap-near-term.md`
- `sed -n '1,320p' plan/17-research-phases-and-autonomy-gates.md`
- `sed -n '1,360p' docs/reports/0934-cross-package-sweep-and-next-queue-recut.md`
- `rg -n "cross-package sweep|FAIRY-05|Typed external boundary / adapter executable widening|remaining mixed gate|next promoted|next reopen point|Macro 7|Macro 6|current active packages" README.md Documentation.md progress.md tasks.md samples_progress.md docs/hands_on/current_phase_closeout_01.md docs/research_abstract/mirrorea_future_axis_01.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/17-research-phases-and-autonomy-gates.md specs/11-roadmap-and-workstreams.md docs/reports/0934-cross-package-sweep-and-next-queue-recut.md`
- `nl -ba tasks.md | sed -n '1,260p'`
- `nl -ba progress.md | sed -n '1,220p'`
- `nl -ba samples_progress.md | sed -n '1,220p'`
- `nl -ba docs/hands_on/current_phase_closeout_01.md | sed -n '1,220p'`
- `nl -ba docs/research_abstract/mirrorea_future_axis_01.md | sed -n '1,260p'`
- `nl -ba plan/01-status-at-a-glance.md | sed -n '1,260p'`
- `nl -ba plan/11-roadmap-near-term.md | sed -n '1,220p'`
- `nl -ba plan/17-research-phases-and-autonomy-gates.md | sed -n '1,220p'`
- `nl -ba docs/reports/0934-cross-package-sweep-and-next-queue-recut.md | sed -n '1,240p'`
- validation は review-only task のため未実行。sample / helper / Rust test を再実行していないことを final review でも明記する。

## What changed in understanding

- current drift の主因は「何が close 済み package なのか」と「何が next promoted queue / mixed gate / reopen point なのか」を文書ごとに slightly different に切っていることだった。
- `FAIRY-05` は planned family のまま next promoted package でもありうるが、その場合でも mixed gate と current active package の wording を雑に重ねると current reading が崩れる。
- `Macro 6` と `Macro 7` の境界は `.docs/progress-task-axes.md` と `plan/17` でかなり明確なので、`tasks.md` package label がそこから外れると taxonomy drift が目立つ。

## Open questions

- `cross-package sweep` を current active package と書きたいのか、already-closed package と書きたいのかを snapshot docs 全体でどちらに揃えるか。
- `FAIRY-05` を self-driven promoted package と mixed-gate residual のどちらで主に読むべきか。
- `FAIRY-05` の package label を `Macro 6` と読むのか、`Macro 8` application realization 寄りの reserve line と読むのか。

## Suggested next prompt

`0934` docs package の review findings を反映してください。特に `samples_progress.md` の current package wording、`docs/hands_on/current_phase_closeout_01.md` の mixed-gate vs next-queue wording、`tasks.md` の `FAIRY-05` macro phase label、`docs/reports/0934-*` の planned-family enumeration を整合させてください。
