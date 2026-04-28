# 0977 — R2 AttachPoint detach minimal contract closeout

## Objective

`R2` `AttachPoint` compatibility / detach minimal contract を docs-first package として close し、
helper-local `hotplug_lifecycle` / explicit detach TODO boundary の current minimal contract row、
grounding envelope / view / telemetry anchor、kept-later migration / rollback gate を
repo memory / reader-facing docs / snapshot docs に同期する。

## Scope and assumptions

- current scope は helper-local evidence floor の整理であり、runtime-crate hot-plug engine の実装ではない
- current scope は final public hot-plug ABI、rollback protocol、durable migration engine の固定ではない
- current grounding seam は accepted / rejected `MessageEnvelope` に残し、transport / auth / membership / capability / witness を collapse しない
- storage detach script と runtime detach lifecycle は同一視しない
- user-dirty の `crates/mir-ast/*` は触らない

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0968-p14-hotplug-package-surface-map.md`
- `docs/reports/0969-p14-hotplug-package-manager-closeout.md`
- `docs/reports/0975-r1-verification-layer-widening-threshold-inventory-first-cut.md`
- `docs/reports/0976-r1-verification-threshold-closeout-and-r2-promotion.md`

## Actions taken

1. helper-local hot-plug evidence floor を再確認した
   - `01_runtime_attach_game` から `compatibility` / `activation_cut`
   - `09_detach_todo` から `detach_boundary` / `migration_contract`
   - `attach_request#1` / `detach_request#1` / `detached_roll_request#1`
   - `attach_lifecycle` / `detach_lifecycle`
   - `attach_activation#1` / `detach_boundary#1`
2. package memory を `plan/30-attachpoint-detach-minimal-contract.md` として追加した
3. reader-facing summary / landing page を追加した
   - `docs/research_abstract/attachpoint_detach_minimal_contract_01.md`
   - `docs/hands_on/attachpoint_detach_minimal_contract_01.md`
4. hot-plug memory / glossary / open-question wording を同期した
   - `plan/14`
   - `plan/21`
   - `specs/10`
   - `docs/research_abstract/hotplug_attachpoint_plan_01.md`
5. front-door / snapshot / queue wording を `R2` close / `R3` promoted-next に同期した
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
- `plan/14-glossary-and-boundary-rules.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/30-attachpoint-detach-minimal-contract.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/attachpoint_detach_minimal_contract_01.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/attachpoint_detach_minimal_contract_01.md`

## Commands run

Resource baseline:

```bash
df -h .
free -h
```

Validation:

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 -m unittest scripts.tests.test_sugoroku_world_samples
python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- resource baseline
  - root disk: `/dev/vda2` 99G, 32G free, 67% used
  - memory: `960Mi` total, `209Mi` available
- `01_runtime_attach_game`
  - `hotplug_lifecycle.lifecycle_state = attached_active`
  - `compatibility.result = compatible`
  - `activation_cut.request_envelope = attach_request#1`
  - `hotplug_activation_boundary` emits `hotplug_lifecycle`
- `09_detach_todo`
  - `terminal_outcome = todo_deferred`
  - `hotplug_lifecycle.lifecycle_state = detached_todo_boundary`
  - `detach_boundary.post_detach_action.verdict = reject`
  - `migration_contract.status = deferred`
  - `detached_roll_request#1` is explicitly rejected
- closeout inventory
  - `hotplug_lifecycle_lanes = attachpoint_id / patch_id / lifecycle_state / compatibility / activation_cut / detach_boundary / migration_contract`
  - `hotplug_kept_later_gates = runtime_crate_hotplug_engine / rollback_protocol / durable_migration_engine / final_public_hotplug_abi`
- unit validation
  - `scripts.tests.test_sugoroku_world_samples`: `46/46` pass
  - `scripts.tests.test_visual_debugger_viewer_samples`: `8/8` pass
- doc / whitespace validation
  - `check_source_hierarchy.py`: pass
  - `validate_docs.py`: pass
  - `git diff --check`: pass

## What changed in understanding

- `P14` closeout inventory だけでは helper-local lifecycle lane の existence は示せても、
  current minimal contract row の最小読みはまだ front-door に出ていなかった
- `R2` の適切な narrow line は、新しい ABI を発明することではなく、
  既存 lane を `compatibility / activation_cut / detach_boundary / migration_contract`
  の 4 row と grounding anchor に整理することだった
- `hotplug_lifecycle` を public ABI と誤読させないには、
  `MessageEnvelope` grounding、view / telemetry anchor、storage detach 非同一性を同じ package で明示する必要があった

## Open questions

- `runtime_crate_hotplug_engine` をいつ、どの crate ownership で actualize するか
- rollback protocol の current smallest honest contract はどこまでか
- durable migration engine と detach replay contract をどこで split するか
- final public hot-plug ABI を current helper-local carrier からどう切り離すか
- `R3` で `FAIRY-05` visibility-return witness を timeline event / anchor-switch event / witness event / typed bundle のどこへ載せるか

## Suggested next prompt

`R3` `FAIRY-05` visibility-return carrier bundling を進め、active representative slice を保ったまま residual planned family `FAIRY-05` の carrier-choice matrix を `plan/24`、reader-facing docs、`progress.md`、`tasks.md`、`samples_progress.md` に同期してください。planning-only candidate label `state_timeline` / `anchor_switch` を current public debug mode と混同しないでください。
