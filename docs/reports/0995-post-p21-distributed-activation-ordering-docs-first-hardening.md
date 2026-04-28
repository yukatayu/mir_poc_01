# 0995 post-P21 distributed activation ordering docs-first hardening

## Objective

`P21` close 後の later family の second recommendation を
`distributed activation ordering` として docs-first に harden し、
first recommendation close 済みの `rollback / durable migration` family、
remaining third recommendation の final public hot-plug ABI、
front-door / snapshot / roadmap memory の stale grouped-later wording を同期する。

## Scope and assumptions

- 規範判断の正本は `specs/`
- `sub-agent-pro/` handoff は working directive / handoff として参照し、
  規範化や repository memory 化は `specs/` / `plan/` / docs 側で行う
- `P21` の narrow runtime-side engine-state progression は close 済み前提とする
- helper-local attach sample と runtime-side engine state を
  distributed ordering completion や final public ABI と混同しない

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `docs/reports/0993-p21-runtime-crate-hotplug-completed-engine-narrow-cut.md`
- `docs/reports/0994-post-p21-rollback-durable-migration-family-first-recommendation.md`

## Actions taken

1. `distributed activation ordering` の current references と stale grouped-later wording を audit した。
2. `plan/37-post-p21-distributed-activation-ordering-family.md` を追加し、
   second recommendation family の role / scope / validation floor / stop line を repository memory に固定した。
3. reader-facing summary / landing page
   `docs/research_abstract/post_p21_distributed_activation_ordering_family_01.md`
   と
   `docs/hands_on/post_p21_distributed_activation_ordering_family_01.md`
   を追加した。
4. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、
   related `plan/` / `specs/` / reader-facing docs を
   first recommendation close 済み / second recommendation close 済み / third recommendation remaining
   の current line に同期した。
5. `plan/90-source-traceability.md` に `plan/37` traceability row を追加し、
   `plan/00-index.md` の index も更新した。

## Evidence / outputs / test results

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - `activation_cut.request_envelope = attach_request#1`
  - helper-local attach evidence は distributed activation ordering proof ではない current line を再確認
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - rejected `detached_roll_request#1`
  - `migration_contract.status = deferred`
  - rollback / durable migration / final public hot-plug ABI を premature に collapse していないことを再確認
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime --test hotplug_runtime_skeleton'`
  - pass
  - 8/8 green
  - warning only: `/mnt/mirrorea-work/llvm` parent is not writable by the current user
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
  - documentation scaffold is complete
  - numbered reports: 993
- `git diff --check`
  - pass
- reviewer finding
  - `0995` report evidence 欠落と historical/current wording ambiguity を指摘
  - この package 内で修正を吸収した
- narrow re-review after fix
  - no findings

## What changed in understanding

- post-`P21` later family の grouped inventory は historical memory として保てるが、
  current repo snapshot では
  first recommendation `rollback / durable migration`
  と
  second recommendation `distributed activation ordering`
  を別々の docs-first family として明示した方が
  stale reading を減らせる。
- `activation_cut` widening criteria は
  `migration_contract` honesty row の widening criteria と distinct であり、
  second recommendation family として独立させるのが妥当である。

## Open questions

- distributed activation ordering family を
  durable activation commit と 1 family のまま保つか、
  network / consensus witness family で再分割するか
- final public hot-plug ABI family を
  `plan/27` / `plan/28` mixed gate memory の上にどう narrowing するか

## Suggested next prompt

post-`P21` final public hot-plug ABI family を docs-first に harden し、`plan/27` / `plan/28` / `specs/11` / `progress.md` / `tasks.md` / `samples_progress.md` / front-door docs に remaining third recommendation と freeze prerequisite を同期してください。helper-local anchor naming や runtime-private state naming を public ABI に昇格させないでください。
