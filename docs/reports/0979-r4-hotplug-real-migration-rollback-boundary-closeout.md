# 0979 — R4 hot-plug real migration / rollback boundary closeout

## Objective

`R4` hot-plug real migration / rollback boundary を docs-first package として close し、
helper-local hot-plug evidence floor を widening せずに、
current minimal contract row の先に残る kept-later hot-plug boundary を
repo memory / reader-facing docs / snapshot docs に同期する。

## Scope and assumptions

- current scope は helper closeout / runtime crate / public ABI の actual implementation ではない
- current scope は `hotplug_lifecycle` field 追加や helper schema widening ではない
- current scope は rollback protocol、durable migration engine、distributed activation ordering、final public hot-plug ABI の fixed implementation ではない
- helper-local package-manager inventory は current evidence floor として維持する
- storage detach、network replay、runtime detach lifecycle は別 concern として保つ
- user-dirty の `crates/mir-ast/*` は触らない

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
- `specs/10-open-questions.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0977-r2-attachpoint-detach-minimal-contract-closeout.md`
- `docs/reports/0978-r3-fairy05-visibility-return-carrier-bundling-closeout.md`

## Actions taken

1. current hot-plug evidence floor を再確認した
   - `01_runtime_attach_game`
   - `09_detach_todo`
   - `attach_request#1 / detach_request#1 / detached_roll_request#1`
   - `hotplug_lifecycle`
   - helper closeout `hotplug_scope` / `hotplug_kept_later_gates`
2. kept-later hot-plug boundary matrix を `plan/32-hotplug-real-migration-rollback-boundary.md` として追加した
3. reader-facing summary / landing page を追加した
   - `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
   - `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
4. hot-plug memory / open-question wording を同期した
   - `plan/21`
   - `plan/30`
   - `specs/10`
   - `docs/research_abstract/hotplug_attachpoint_plan_01.md`
5. front-door / snapshot / queue wording を `R4` close / `R5` promoted-next に同期した
   - `README.md`
   - `Documentation.md`
   - `progress.md`
   - `tasks.md`
   - `samples_progress.md`
   - `plan/01`
   - `plan/11`
   - `docs/research_abstract/README.md`
   - `docs/research_abstract/mirrorea_future_axis_01.md`
   - `docs/hands_on/README.md`
   - `docs/hands_on/current_phase_closeout_01.md`

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `plan/32-hotplug-real-migration-rollback-boundary.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `docs/reports/0979-r4-hotplug-real-migration-rollback-boundary-closeout.md`

## Commands run

- `df -h .`
- `free -h`
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
- `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- resource check
  - `df -h .`: `/dev/vda2` 99G total / 63G used / 32G avail / 67% use
  - `free -h`: 960Mi total / 812Mi used / 79Mi free / 19Gi swap with 2.2Gi used
- `01_runtime_attach_game --debug hotplug`
  - `hotplug_lifecycle.lifecycle_state = attached_active`
  - `compatibility.result = compatible`
  - `activation_cut.request_envelope = attach_request#1`
  - `migration_contract.status = not_started`
- `09_detach_todo --debug hotplug`
  - `terminal_outcome = todo_deferred`
  - `hotplug_lifecycle.lifecycle_state = detached_todo_boundary`
  - `detach_boundary.post_detach_action.verdict = reject`
  - `migration_contract.status = deferred`
  - `detached_roll_request#1` is rejected
- `sugoroku_world_samples.py closeout --format json`
  - `hotplug_scope = helper_local_package_manager_preview`
  - `hotplug_kept_later_gates = [runtime_crate_hotplug_engine, rollback_protocol, durable_migration_engine, final_public_hotplug_abi]`
  - `hotplug_validation_floor = helper-local attach/detach lifecycle evidence only; not completed migration/rollback/runtime-crate ownership`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - `Ran 46 tests ... OK`
- `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples`
  - `Ran 8 tests ... OK`
- `python3 scripts/check_source_hierarchy.py`
  - required 23 / present 23 / missing 0
- `python3 scripts/validate_docs.py`
  - documentation scaffold complete
  - numbered reports: 977
- `git diff --check`
  - no whitespace / conflict-marker issues

## What changed in understanding

- `R2` current minimal contract row だけでは helper-local evidence の honest stop line は読めても、
  distributed activation ordering / rollback / durable migration / runtime-crate ownership の
  **未証明境界** は front-door に十分出ていなかった
- `R4` の適切な narrow line は helper schema を広げることではなく、
  `activation_cut` と `migration_contract` が何をまだ証明していないかを
  kept-later boundary matrix として固定することだった
- `R5` を次 line に送るには、rollback / migration actualization の前に
  helper-local preview と crate-side ownership split を先に切る必要がある

## Open questions

- runtime-crate hot-plug engine の exact ownership split をどう切るか
- actual rollback protocol / durable migration engine / distributed activation ordering の exact contract をどこで固めるか
- final public hot-plug ABI を helper-local evidence carrier からどう切り離すか
- `R5` で `mirrorea-core` と `mir-runtime` の owner split をどう reader-facing に固定するか

## Suggested next prompt

`R5` runtime-crate hot-plug engine ownership cut を進め、helper-local hot-plug evidence、crate-side carrier/runtime substrate、later runtime orchestration の owner split を `plan/21` と新設する `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`、front-door docs、`progress.md`、`tasks.md` に同期してください。hot-plug engine actualization、rollback / migration / distributed activation ordering、final public ABI を同じ package で claim しないでください。
