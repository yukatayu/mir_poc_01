# 0992 — R7 post-P20 hot-plug next-package inventory

## Objective

`P20` current closeout の先に残る hot-plug kept-later lanes を docs-first に inventory し、
next promoted package を `P21` runtime-crate hot-plug completed-engine narrow cut に固定する。

## Scope and assumptions

- scope は `R7` docs-first queue stabilization に限定する。
- `P21` 自体の implementation は行わない。
- rollback protocol、durable migration / reattach semantics、distributed activation ordering、final public hot-plug ABI は kept-later family に残す。
- exact later package labels は evidence なしに fixed しない。

## Documents consulted

- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `plan/34-runtime-crate-hotplug-carrier-admission-cut.md`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`

## Actions taken

1. `plan/35-post-p20-hotplug-next-package-inventory.md` を新設し、post-`P20` kept-later lane を smallest plausible package cuts と current recommendation に整理した。
2. reader-facing summary / landing page として `docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md` と `docs/hands_on/post_p20_hotplug_next_package_inventory_01.md` を追加した。
3. front-door docs、snapshot docs、relevant `plan/` / `specs/` を `R7 close / P21 promoted next / later-family grouped unresolved` に同期した。

## Evidence / outputs / test results

- resource check before rerun:
  - `df -h .` → `/dev/vda2` `99G` total, `63G` used, `32G` available
  - `free -h` → `960Mi` total, `301Mi` available, swap `19Gi`
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - pass
  - `attach_request#1` / `attach_lifecycle` / `attach_activation#1` remain helper-local preview ownership
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - pass
  - rejected `detached_roll_request#1` and `migration_contract.status = deferred` remain explicit
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime --test hotplug_runtime_skeleton'`
  - pass
  - 3/3 green
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime'`
  - pass
  - runtime regression all green; `clean_near_end_samples` 27/27 and `hotplug_runtime_skeleton` 3/3 included
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
  - `Found 990 numbered report(s).`
- `git diff --check`
  - pass

## What changed in understanding

- post-`P20` remaining lane を single vague bundle として残すより、`P21` completed-engine narrow cut と later-family grouping に分けた方が current docs / plan / samples_progress の tense drift を抑えられる。
- `P20` は thin runtime/report assembly であり、completed engine ではないという line を queue memory でも繰り返し保つ必要がある。
- `rollback / durable migration`、`distributed activation ordering`、`final public hot-plug ABI` は `P21` と同時に昇格させると scope drift になるため、current package queue では grouped later family として残すのが妥当。

## Open questions

- `P21` close 後に `rollback` と `durable migration / reattach semantics` を 1 package に保つか、別 package に再分割するかは未決のまま残る。
- `distributed activation ordering` を standalone package にするか、durable activation commit family に吸収するかは未決のまま残る。
- final public hot-plug ABI の actual target surface は引き続き post-`P18` mixed gate / true user-spec hold の外に出さない。

## Suggested next prompt

`P21` runtime-crate hot-plug completed-engine narrow cut を start し、admitted request / verdict carrier と existing substrate の上で canonical engine-side lifecycle state progression を narrow に actualize してください。rollback / durable migration / distributed activation ordering / final public ABI は同じ tranche に混ぜないでください。
