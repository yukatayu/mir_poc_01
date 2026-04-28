# 0997 post-P21 final public hot-plug ABI docs-first hardening

## Objective

post-`P21` later family の最後の self-driven docs-first package として、
`final public hot-plug ABI` family を
`freeze prerequisite fixed; public ABI still unfrozen`
の line で repository memory / snapshot / front-door docs / reader-facing docs に同期する。

## Scope and assumptions

- 規範判断の正本は `specs/` とする。
- `plan/` は repository memory、`progress.md` / `tasks.md` / `samples_progress.md` は snapshot として扱う。
- current scope は docs-first hardening であり、actual final public hot-plug ABI freeze や `U1` commitment は行わない。
- worktree には task 開始前から Rust code の未コミット変更があるため、本 task では触らない。
- `0996` audit report は pre-work evidence として読み、report 自体も同じ commit scope に含める。

## Documents consulted

- `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/37-post-p21-distributed-activation-ordering-family.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/post_p21_distributed_activation_ordering_family_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/post_p21_distributed_activation_ordering_family_01.md`
- `docs/hands_on/post_p21_rollback_durable_migration_family_01.md`
- `docs/reports/0996-post-p21-final-public-hotplug-abi-audit.md`

## Actions taken

1. `0996` audit の stale wording / malformed traceability / minimum sync set を確認した。
2. `plan/38-post-p21-final-public-hotplug-abi-family.md` を追加し、third recommendation family の role、preconditions、scope、keep-as-one-family criteria、validation floor、stop line を固定した。
3. `plan/27` / `plan/28` に hot-plug mixed-gate bridge を追加し、helper-local preview naming、runtime-private carrier / engine-state naming、public candidate naming の non-equivalence を明文化した。
4. `plan/35` / `plan/36` / `plan/37` / `plan/00` / `plan/01` / `plan/11` / `plan/17` / `plan/90` を更新し、third recommendation close、next open work = actual `U1` commitment、traceability table repair を同期した。
5. `specs/11`、`README.md`、`Documentation.md`、`docs/research_abstract/README.md`、`docs/research_abstract/mirrorea_future_axis_01.md`、`docs/hands_on/current_phase_closeout_01.md` に third recommendation close の current reading を反映した。
6. `progress.md`、`tasks.md`、`samples_progress.md` を更新し、`current remaining third recommendation` 読みを除去して actual next gate を `U1` commitment に揃えた。
7. reviewer follow-up で `plan/90-source-traceability.md` の不足 row、`samples_progress.md` の blocker ownership wording、および stale blocker label を actual `U1` hot-plug public-freeze commitment に揃え、派生 stale wording と reader docs の current phrasing も再同期した。
8. required validation を rerun し、narrow re-review の前提 evidence を fresh に取り直した。

## Files changed

- added:
  - `plan/38-post-p21-final-public-hotplug-abi-family.md`
  - `docs/research_abstract/post_p21_final_public_hotplug_abi_family_01.md`
  - `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`
  - `docs/reports/0996-post-p21-final-public-hotplug-abi-audit.md`
  - `docs/reports/0997-post-p21-final-public-hotplug-abi-docs-first-hardening.md`
- updated:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/27-public-api-parser-gate-roadmap.md`
  - `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
  - `plan/32-hotplug-real-migration-rollback-boundary.md`
  - `plan/35-post-p20-hotplug-next-package-inventory.md`
  - `plan/36-post-p21-rollback-durable-migration-family.md`
  - `plan/37-post-p21-distributed-activation-ordering-family.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `docs/research_abstract/post_p21_distributed_activation_ordering_family_01.md`
  - `docs/hands_on/current_phase_closeout_01.md`
  - `docs/hands_on/post_p21_distributed_activation_ordering_family_01.md`
  - `docs/hands_on/post_p21_rollback_durable_migration_family_01.md`

## Evidence / outputs / test results

### Resource check

- `df -h .`
  - `/dev/vda2` 99G total, 32G available
- `free -h`
  - 960Mi total RAM, 257Mi available, swap 17Gi available

### Validation commands run

- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
- `bash -lc 'source scripts/env/mirrorea_storage_env.sh >/dev/null && cargo test -p mir-runtime --test hotplug_runtime_skeleton'`
  - pass
  - warning only: `/mnt/mirrorea-work/llvm` parent non-writable
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

### Validation notes

- Sugoroku closeout は helper-local `hotplug_scope` / `hotplug_validation_floor` を維持し、helper-local preview naming と runtime-private naming を public ABI naming に上げていない current line を再確認した。
- `hotplug_runtime_skeleton` test は `HotPlugRuntimeSkeletonReport` / `HotPlugRuntimeEngineReport` regression 8/8 green で、runtime-private anchor を維持した。
- source hierarchy / docs scaffold / whitespace-clean diff は current sync 後も維持された。
- review follow-up 後にも `plan/90` traceability row、`samples_progress.md` blocker ownership wording / blocker label を含む docs-only tail が fresh validation を通ることを確認した。

### Skipped validations

- なし

## What changed in understanding

- post-`P21` later family の self-driven docs-first package は third recommendation までで打ち止めであり、この後に残るのは actual `U1` commitment だけである。
- third recommendation package の主眼は public ABI freeze ではなく、helper-local preview naming、runtime-private naming、public candidate naming を混ぜない freeze-prerequisite bridge を `plan/27` と `plan/28` の間で明文化することにある。
- `plan/90` の traceability table は `0997` で repair しないと current source trace として破綻したままだった。

## Open questions

- actual shipped public hot-plug ABI surface を first shipped public surface scope のどこに置くか。
- `AttachPoint` / `Patch` package catalog naming を host target / engine adapter choice と同じ `U1` track で決めるか、別 track に split するか。
- packaging shape / installed binary target、host integration target、final shared-space operational catalog breadth をどの順序で actual commitment するか。

## Suggested next prompt

`U1` actual commitment を進めてください。`plan/28-post-p18-true-user-spec-hold-option-matrix.md` を入口に、1) packaging shape / installed binary target、2) host integration target、3) first shipped public surface scope、4) final shared-space operational catalog breadth を順に actual choice してください。`final public hot-plug ABI` family では helper-local preview naming と runtime-private naming を public ABI に上げないまま止めてあるので、その前提を維持して narrow に決めてください。
