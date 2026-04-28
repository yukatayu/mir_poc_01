# Report 0968 — P14 hot-plug package surface map

- Date: 2026-04-28 16:38 JST
- Author / agent: Codex (GPT-5)
- Scope: Map the current P14 hot-plug helper/test/docs surface around Sugoroku helper closeout, debug lanes, roadmap wording, and package-manager-first-cut gaps.
- Decision levels touched: none; analysis-only

## 1. Objective

Map the current hot-plug-related helper/test/docs surface for the requested files, identify the smallest honest package-manager-first-cut additions, list concrete failing test candidates, and note stale wording risks.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## 3. Actions taken

- Read the mandated repo front-door and spec sequence before narrowing to the hot-plug surface.
- Inspected helper code, unit tests, roadmap docs, hands-on docs, sample README, and snapshot docs with line-numbered reads.
- Executed the current hot-plug pretty debug views for attach and detach.
- Executed the current Sugoroku helper unittest suite to confirm the present regression floor.
- Executed Sugoroku helper closeout JSON to confirm current closeout fields and aggregate inventory.

## 4. Files changed

- Added this report:
  `docs/reports/0968-p14-hotplug-package-surface-map.md`
- No helper / test / snapshot / plan / spec files were modified in this task.
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug`
  - printed attach lifecycle summary with `AttachPoint[SugorokuGame#1]`, `SugorokuGamePackage@runtime`, `state=attached_active`, `activation_cut: request_envelope=attach_request#1`, `migration_status: not_started`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug`
  - printed detach lifecycle summary with `state=detached_todo_boundary`, `request_envelope=detach_request#1`, `post_detach_action: reject (detached_game_rejects_domain_action)`, `migration_status: deferred`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug hotplug`
  - `HOT-PLUG LIFECYCLE`
  - `  - none`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples -v`
  - `Ran 43 tests in 0.127s`
  - `OK`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - returned current helper closeout JSON including `hotplug_scope`, `hotplug_anchor_samples`, `hotplug_package_concerns`, `hotplug_kept_later_gates`, `hotplug_validation_floor`, `hotplug_stop_line`, hot-plug views, hot-plug telemetry rows, and hot-plug layer signatures

## 6. Evidence / findings

- Helper already exposes per-sample hot-plug lifecycle evidence:
  - attach lifecycle with `attachpoint_id`, `patch_id`, `compatibility`, `activation_cut`, `migration_contract`
  - detach lifecycle with `detach_boundary` and post-detach rejection
- Helper already exposes closeout-level package-manager-preview inventory:
  - `hotplug_scope = helper_local_package_manager_preview`
  - `hotplug_anchor_samples = [01_runtime_attach_game, 09_detach_todo]`
  - `hotplug_package_concerns = attachpoint_compatibility / activation_cut / detach_boundary / migration_stop_line / rollback_protocol`
  - `hotplug_kept_later_gates` and `hotplug_stop_line`
- Helper already exposes debug / telemetry / visualization / layer surfaces for hot-plug:
  - debug mode `--debug hotplug`
  - telemetry kinds `hotplug_activation` and `hotplug_detach`
  - view kind `hotplug_lifecycle` with `attach_lifecycle` and `detach_lifecycle`
  - layer signatures `hotplug_activation_boundary` and `hotplug_detach_boundary`
- Current regression floor is stronger than one snapshot claims:
  - live unittest run is 43/43 green
  - `samples_progress.md` still says `39/39 green`
- Current P14 docs still have a few drift points:
  - `tasks.md` includes `03_roll_publish_handoff --debug hotplug --format json` as a P14 validation command even though sample `03` has no hot-plug lifecycle surface and pretty hotplug output is `none`
  - `samples/clean-near-end/sugoroku-world/README.md` focused-run list omits the attach/detach hot-plug commands even though the PH14 section says `SUG-01` / `SUG-09` should be read through `--debug hotplug`
  - same README still says `layers` shows only verification/runtime_trace/membership layers, but current helper layer inventory also has `hotplug_activation_boundary` and `hotplug_detach_boundary`

## 7. Changes in understanding

- The repo is not waiting to invent a package-manager surface from scratch; a narrow preview already exists in helper closeout and tests.
- The smallest honest P14 move is inventory tightening, not new runtime behavior:
  helper closeout lacks an explicit hot-plug lane catalog and lacks an explicit aggregate anchor list for request envelopes / view ids / telemetry row ids.
- The main risk is wording drift across docs and validation commands, not absence of basic hot-plug evidence carriers.

## 8. Open questions

- Should package-manager first cut add only aggregate closeout metadata such as `hotplug_lifecycle_lanes` and `hotplug_anchor_envelopes`, or also add a new closeout-level `hotplug_inventory` object?
- Should `SUG-03` remain in the P14 validation list as a deliberate “no hot-plug surface here” negative control, or be removed to avoid confusion?
- Should the package identifier stay `patch_id` or be renamed/aliased to a more explicitly package-manager-oriented field in a later tranche?

## 9. Suggested next prompt

Implement the minimal P14 package-manager-first-cut tightening: add explicit `hotplug_lifecycle_lanes` and `hotplug_anchor_envelopes` to `scripts/sugoroku_world_samples.py` closeout, add failing tests first in `scripts/tests/test_sugoroku_world_samples.py`, and update `samples/clean-near-end/sugoroku-world/README.md`, `docs/hands_on/current_phase_closeout_01.md`, `progress.md`, `tasks.md`, and `samples_progress.md` to remove the current wording drift.
