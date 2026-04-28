# 0981 — R4 follow-up narrow re-review

## Objective

前回 review の 3 finding に対する follow-up 修正を、指定 4 点の current worktree diff に限定して narrow re-review し、残 finding の有無を確認する。

## Scope and assumptions

- review scope は user 指定の 4 点に限る
- `crates/mir-ast/*` の user-dirty files は対象外
- current worktree diff のみを見る
- この task は narrow re-review であり、package 本体の追加修正は行わない

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
- `docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`
- `docs/reports/0980-r4-docs-first-package-review.md`

## Actions taken

1. front-door / snapshot / base spec の relevant section を再読した
2. `progress.md`、`tasks.md`、`samples_progress.md`、`0979` report の current diff を確認した
3. 前回 finding 3 点について、修正後 wording と evidence timestamp を行番号付きで再確認した

## Files changed

- `docs/reports/0981-r4-followup-narrow-rereview-no-findings.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git diff -- progress.md samples_progress.md tasks.md docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md docs/reports/0980-r4-docs-first-package-review.md`
- `git diff --check -- docs/reports/0981-r4-followup-narrow-rereview-no-findings.md`

## Evidence / outputs / test results

- `samples_progress.md`
  - hot-plug summary row は `90` に戻り、`100%` 条件を commit/push 後に限定する注記が追加されていた
  - `PH14` row は `90`、`Last validation = 2026-04-28 22:32 JST`、`0979` 反映済みへ更新されていた
- `tasks.md`
  - `R5` docs/report requirement は dangling `plan/33` 参照ではなく、「新設する `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`」を明示していた
- `docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`
  - suggested next prompt は同じく「新設する `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`」へ修正されていた
- `progress.md`
  - recent log に 2026-04-28 22:40 JST の reviewer follow-up が追加され、`0980` 追加後の `check_source_hierarchy.py` / `validate_docs.py` / `git diff --check` rerun が記録されていた
- current scope では新しい finding は見つからなかった

## What changed in understanding

- follow-up 修正により、前回 3 finding は current diff 上では解消されていた
- next-package wording は「未作成 anchor を参照する」状態から、「次 task で新設する anchor を要求する」状態へ変わっており、source-hierarchy drift はこの narrow scope では解消された

## Open questions

- なし

## Suggested next prompt

`R5` package 本体に進み、新設する `plan/33-runtime-crate-hotplug-engine-ownership-cut.md` を実際に追加して、front-door docs / snapshot docs / report を同期してください。current narrow re-review では追加 finding はありませんでした。
