# 0980 — R4 docs-first package review

## Objective

未コミットの `R4` docs-first package について、`R4` closeout と `R5` promotion の wording / evidence / source hierarchy / overclaim を review し、maintainer 観点の finding を残す。

## Scope and assumptions

- 対象は user 指定の docs-first scope に限る
- `crates/mir-ast/*` の user-dirty files は review 対象外とする
- この task は review であり、package 本体の修正は行わない
- 規範判断の正本は `specs/`、長期 memory は `plan/`、snapshot は `progress.md` / `tasks.md` / `samples_progress.md` として読む

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`

## Actions taken

1. 指定 scope の未コミット差分を確認した
2. AGENTS 指定順に front-door / snapshot / base specs / relevant plan / reader-facing docs を読んだ
3. `R4` closeout と `R5` promoted-next wording を横断検索し、stale wording と dangling reference を探した
4. `samples_progress.md` の progress rule と current uncommitted state を照合した
5. `0979` report の evidence / next prompt / source-hierarchy consistency を確認した

## Files changed

- `docs/reports/0980-r4-docs-first-package-review.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short -- README.md Documentation.md progress.md tasks.md samples_progress.md plan/00-index.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/21-hotplug-attachpoint-roadmap.md plan/30-attachpoint-detach-minimal-contract.md plan/32-hotplug-real-migration-rollback-boundary.md specs/10-open-questions.md docs/research_abstract/README.md docs/research_abstract/mirrorea_future_axis_01.md docs/research_abstract/hotplug_attachpoint_plan_01.md docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md docs/hands_on/README.md docs/hands_on/current_phase_closeout_01.md docs/hands_on/hotplug_real_migration_rollback_boundary_01.md docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`
- `git diff --stat -- README.md Documentation.md progress.md tasks.md samples_progress.md plan/00-index.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/21-hotplug-attachpoint-roadmap.md plan/30-attachpoint-detach-minimal-contract.md plan/32-hotplug-real-migration-rollback-boundary.md specs/10-open-questions.md docs/research_abstract/README.md docs/research_abstract/mirrorea_future_axis_01.md docs/research_abstract/hotplug_attachpoint_plan_01.md docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md docs/hands_on/README.md docs/hands_on/current_phase_closeout_01.md docs/hands_on/hotplug_real_migration_rollback_boundary_01.md docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`
- `wc -l README.md Documentation.md progress.md tasks.md samples_progress.md plan/00-index.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/21-hotplug-attachpoint-roadmap.md plan/30-attachpoint-detach-minimal-contract.md plan/32-hotplug-real-migration-rollback-boundary.md specs/00-document-map.md specs/01-charter-and-decision-levels.md specs/02-system-overview.md specs/03-layer-model.md specs/09-invariants-and-constraints.md specs/10-open-questions.md docs/research_abstract/README.md docs/research_abstract/mirrorea_future_axis_01.md docs/research_abstract/hotplug_attachpoint_plan_01.md docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md docs/hands_on/README.md docs/hands_on/current_phase_closeout_01.md docs/hands_on/hotplug_real_migration_rollback_boundary_01.md docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`
- `rg -n '^#|^##|R4|R5|hotplug|attachpoint|migration|rollback|research_abstract|hands_on|progress|tasks|samples_progress|plan/' specs/00-document-map.md`
- `rg -n "plan/33|33-hotplug|R5 runtime-crate hot-plug engine ownership cut|runtime-crate hot-plug engine ownership cut" -S .`
- `ls plan | sort | tail -n 10`
- `rg -n "hotplug_kept_later_gates|distributed activation ordering|distributed_activation_ordering|kept-later boundary" README.md Documentation.md progress.md tasks.md samples_progress.md plan/00-index.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/21-hotplug-attachpoint-roadmap.md plan/30-attachpoint-detach-minimal-contract.md plan/32-hotplug-real-migration-rollback-boundary.md specs/10-open-questions.md docs/research_abstract/README.md docs/research_abstract/mirrorea_future_axis_01.md docs/research_abstract/hotplug_attachpoint_plan_01.md docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md docs/hands_on/README.md docs/hands_on/current_phase_closeout_01.md docs/hands_on/hotplug_real_migration_rollback_boundary_01.md docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`

## Evidence / outputs / test results

- `git status --short`:
  target scope では `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `plan/`、relevant reader-facing docs、`0979` report が未コミットだった
- `ls plan | sort | tail -n 10`:
  `plan/32-hotplug-real-migration-rollback-boundary.md` までしか存在せず、`plan/33` は存在しなかった
- `rg -n "plan/33|runtime-crate hot-plug engine ownership cut" -S .`:
  `tasks.md` の `R5` docs/report requirement と `docs/reports/0979-...` の suggested next prompt が `plan/33` を参照していた
- `samples_progress.md`:
  `Hot-plug package` overall row が 100%、`PH14` row も 100% とされていた一方で、package 自体は未コミットで review 中だった
- `samples_progress.md`:
  `SUG-01` / `SUG-09` は 2026-04-28 22:32 JST へ更新されているが、`PH14` row の `Last validation` は 2026-04-28 22:05 JST のままだった

## What changed in understanding

- `R4` closeout 自体の主張は概ね一貫しているが、`R5` promotion 側に dangling repository-memory anchor が混入していた
- source-hierarchy / roadmap discipline の観点では、next package を promoted に上げるなら対応する `plan/` anchor の有無を snapshot / report / next prompt で一致させる必要がある
- `samples_progress.md` の 100% rule は closeout wording より厳しく、未コミット review state の package を 100% にするのは overclaim になる

## Open questions

- `R5` 用の repository-memory anchor を `plan/33-...` として新設するのか、既存 `plan/32` / `plan/21` の範囲で扱うのか
- `samples_progress.md` の `PH14` を review 中は 90% 相当に戻すのか、commit/push 完了後にのみ 100% へ上げる運用にするのか

## Suggested next prompt

`R4` package を修正してください。少なくとも `tasks.md` と `docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md` の `plan/33` 参照を実在する repository-memory anchor に揃え、`samples_progress.md` の hot-plug rows が uncommitted review state で 100% を主張しないように調整してください。可能なら `PH14` の last-validation timestamp も latest rerun に揃えてください。
