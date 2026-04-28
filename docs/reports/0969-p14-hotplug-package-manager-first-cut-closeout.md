# Report 0969 — P14 hot-plug package-manager first-cut closeout

- Date: 2026-04-28 16:53 JST
- Author / agent: Codex (GPT-5)
- Scope: Close `P14` as helper-local package-manager preview hardening only. Tighten hot-plug closeout inventory, sync docs/snapshots, and keep runtime-crate migration / rollback / final public ABI as deferred gates.
- Decision levels touched: none; current package closeout and repository-memory synchronization only

## 1. Objective

Close the current first cut of `P14` by:

- tightening Sugoroku helper closeout inventory for hot-plug package-manager concerns
- adding regression tests for the new inventory fields
- removing wording drift across front-door docs, hands-on docs, roadmap memory, and sample dashboard
- recording an evidence-backed stop line that does not overclaim runtime-crate hot-plug ownership

## 2. Scope and assumptions

- This task does **not** implement a runtime-crate hot-plug engine.
- This task does **not** claim completed detach migration, rollback protocol, durable upgrade machinery, or final public hot-plug ABI.
- The current honest scope is helper/test/docs closeout hardening around existing attach / detach lifecycle evidence.
- `docs/reports/0968-p14-hotplug-package-surface-map.md` is treated as the analysis precursor, not the closeout itself.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `docs/reports/0968-p14-hotplug-package-surface-map.md`

## 4. Actions taken

- Added closeout-level hot-plug inventory fields to `scripts/sugoroku_world_samples.py`:
  - `hotplug_scope`
  - `hotplug_anchor_samples`
  - `hotplug_package_concerns`
  - `hotplug_lifecycle_lanes`
  - `hotplug_anchor_envelopes`
  - `hotplug_view_ids`
  - `hotplug_telemetry_row_ids`
  - `hotplug_kept_later_gates`
  - `hotplug_validation_floor`
- Added regression tests in `scripts/tests/test_sugoroku_world_samples.py` for:
  - package-manager inventory presence
  - lifecycle lane inventory
  - anchor envelope inventory
  - view / telemetry row inventory
- Synced front-door docs, roadmap memory, hands-on landing, sample README, snapshot docs, and dashboard wording to the new helper closeout inventory.
- Removed the confusing P14 validation reference to `03_roll_publish_handoff --debug hotplug`, which deliberately prints no hot-plug lifecycle surface.
- Promoted the queue reading to `P15` next / `P16` reopen next while preserving the stop line between helper-local preview and kept-later runtime/public gates.

## 5. Files changed

- `README.md`
- `Documentation.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `progress.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples_progress.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `specs/11-roadmap-and-workstreams.md`
- `tasks.md`
- `docs/reports/0968-p14-hotplug-package-surface-map.md`
- `docs/reports/0969-p14-hotplug-package-manager-first-cut-closeout.md`

## 6. RED/GREEN sequence

First RED:

- Added the initial package-manager closeout regression expecting `hotplug_scope`.
- Observed failure:
  - `KeyError: 'hotplug_scope'`

First GREEN:

- Implemented `hotplug_scope`, `hotplug_anchor_samples`, `hotplug_package_concerns`, `hotplug_kept_later_gates`, and `hotplug_validation_floor`.
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - `Ran 43 tests`
  - `OK`

Second RED:

- Strengthened the suite to require:
  - `hotplug_lifecycle_lanes`
  - `hotplug_anchor_envelopes`
  - `hotplug_view_ids`
  - `hotplug_telemetry_row_ids`
- Observed missing-key failures against the closeout JSON.

Second GREEN:

- Implemented the remaining lane / envelope / view / telemetry inventory fields.
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - `Ran 46 tests`
  - `OK`

## 7. Evidence / outputs / test results

Final closeout validation for this package was rerun after the doc/report sync. The exact command set and results are:

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - result: pass
  - `Ran 46 tests in 0.134s`
  - `OK`
- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
  - result: pass
  - attach lifecycle JSON keeps `AttachPoint[SugorokuGame#1]`, `attach_request#1`, `attach_lifecycle`, `attach_activation#1`, and `migration_contract.status = not_started`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
  - result: pass
  - detach lifecycle JSON keeps `detach_request#1`, `detached_roll_request#1`, `detach_lifecycle`, `detach_boundary#1`, post-detach reject, and `migration_contract.status = deferred`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - result: pass
  - closeout JSON returns `hotplug_scope`, `hotplug_anchor_samples`, `hotplug_package_concerns`, `hotplug_lifecycle_lanes`, `hotplug_anchor_envelopes`, `hotplug_view_ids`, `hotplug_telemetry_row_ids`, `hotplug_kept_later_gates`, and `hotplug_validation_floor`
- `python3 scripts/check_source_hierarchy.py`
  - result: pass
  - `required: 23`, `present: 23`, `missing: 0`
- `python3 scripts/validate_docs.py`
  - result: pass
  - `Documentation scaffold looks complete.`
  - `Found 967 numbered report(s).`
- `git diff --check`
  - result: pass

## 8. What changed in understanding

- The honest `P14` move was not new runtime behavior; it was making the existing helper-local hot-plug preview explicit and auditable at closeout level.
- The attach-side and detach-side lifecycle surfaces were already present; the missing piece was aggregate inventory wording that makes package-manager concerns visible without pretending migration or rollback are done.
- The biggest risk was documentation drift, not missing raw evidence.

## 9. Open questions

- Should a later tranche replace `patch_id` with a more explicitly package-manager-oriented identifier, or keep `patch_id` as the stable low-level lane and layer naming above it?
- When `P16` widens viewer/public visualization, should hot-plug view identifiers remain helper-oriented (`attach_lifecycle`, `detach_lifecycle`) or be normalized through a broader typed visualization catalog?
- Should runtime-crate hot-plug ownership begin from lifecycle replay inventory first, or from compatibility / activation / detach rejection law carriers first?

## 10. Suggested next prompt

Close `P15` projection/codegen first emitted place-specific programs as a helper/report/generated-reserve first cut: introduce the smallest honest emitted-artifact inventory, concrete place-specific generated reserve samples or manifests, and an equivalence review surface, while keeping optimizer / deployment planner / final public emitted-program ABI deferred.
