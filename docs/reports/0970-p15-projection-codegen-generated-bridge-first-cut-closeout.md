# Report 0970 — P15 projection/codegen generated bridge first-cut closeout

- Date: 2026-04-28 17:19 JST
- Author / agent: Codex (GPT-5)
- Scope: Close `P15` as committed generated bridge evidence only. Add the smallest honest projection/codegen validation surface, sync docs/snapshots/roadmap memory, and preserve the stop line against claiming final emitted executable programs.
- Decision levels touched: none; current package closeout and repository-memory synchronization only

## 1. Objective

Close the current first cut of `P15` by:

- introducing a committed generated bridge artifact under `samples/generated/`
- adding a helper-local validation surface that aligns that artifact against live helper/runtime anchors
- synchronizing front-door docs, roadmap memory, hands-on docs, and runnable sample dashboard
- recording an evidence-backed stop line that keeps final emitted executable family, optimizer, deployment planner, and final public emitted-program ABI deferred

## 2. Scope and assumptions

- This task does **not** implement a final emitted executable place-program family.
- This task does **not** implement a final projection intermediate representation (IR), optimizer, cross-place equivalence checker, or deployment planner.
- The current honest scope is committed generated bridge evidence plus live-anchor alignment over already-existing helper/runtime preview surfaces.
- `samples/generated/projection-placement/manifest.json` is treated as generated bridge evidence, not as a source sample and not as a final emitted executable program.
- Unrelated user modifications in `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs` and `crates/mir-ast/src/current_l2.rs` were left untouched.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/generated/README.md`
- `scripts/README.md`
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
- `plan/19-repository-map-and-taxonomy.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `scripts/projection_codegen_samples.py`
- `scripts/tests/test_projection_codegen_samples.py`
- `samples/generated/projection-placement/manifest.json`
- `docs/reports/0968-p14-hotplug-package-surface-map.md`

## 4. Actions taken

- Added `scripts/projection_codegen_samples.py` as the package-local validation helper for:
  - `list`
  - `run <artifact_id>`
  - `check-all`
  - `closeout`
- Added `samples/generated/projection-placement/manifest.json` as committed generated bridge evidence with:
  - `P15-GEN-01..04`
  - `artifact_boundary`
  - `generated_bridge_artifact_inventory`
  - `generated_reserve_inventory`
  - `equivalence_review_categories`
  - `kept_later_gates`
  - `validation_floor`
- Added regression tests in `scripts/tests/test_projection_codegen_samples.py` for:
  - manifest boundary wording
  - artifact listing
  - live-anchor alignment
  - drift detection
  - closeout inventory
  - validation floor coverage
- Synced front-door docs, hands-on docs, roadmap memory, snapshot docs, and sample dashboard to:
  - `P15` close (current first cut only)
  - `P16` promoted next
  - `P17` reopen next
- Updated `samples/generated/` taxonomy to explicitly distinguish:
  - source sample
  - generated bridge evidence
  - final emitted executable family
- Preserved the stop line that keeps helper/report-local preview, committed manifest bridge evidence, and final public emitted-program contract separate.

## 5. Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/generated/README.md`
- `scripts/README.md`
- `scripts/projection_codegen_samples.py`
- `scripts/tests/test_projection_codegen_samples.py`
- `samples/generated/projection-placement/manifest.json`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/90-source-traceability.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/reports/0970-p15-projection-codegen-generated-bridge-first-cut-closeout.md`

## 6. Evidence / outputs / test results

Fresh package-close validation was rerun after the doc/report sync. The exact command set and results are:

- `python3 -m unittest scripts.tests.test_projection_codegen_samples`
  - result: pass
  - projection/codegen helper suite is green
- `python3 scripts/projection_codegen_samples.py list --format json`
  - result: pass
  - manifest inventory returns `P15-GEN-01..04`
- `python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json`
  - result: pass
  - Sugoroku helper `projection_view` anchor aligns with committed generated bridge evidence
- `python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json`
  - result: pass
  - clean near-end runtime `cross_place_projection` anchor aligns with committed generated bridge evidence
- `python3 scripts/projection_codegen_samples.py check-all --format json`
  - result: pass
  - `P15-GEN-01..04` all green
- `python3 scripts/projection_codegen_samples.py closeout --format json`
  - result: pass
  - closeout returns `generated_bridge_artifact_inventory`, `generated_reserve_inventory`, `equivalence_review_categories`, and `validation_floor`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  - result: pass
  - helper-local projection preview remains intact and preview-only
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - result: pass
  - report-local `cross_place_projection` remains intact and preview-only
- `find samples/generated -maxdepth 3 -type f | sort`
  - result: pass
  - current committed generated files are limited to `README.md` and `projection-placement/manifest.json`
- `python3 scripts/check_source_hierarchy.py`
  - result: pass
- `python3 scripts/validate_docs.py`
  - result: pass
- `git diff --check`
  - result: pass
- focused reviewer follow-up after wording/progress fixes
  - result: no findings
- git commit / push status
  - completed in this package close task; exact commit SHA is reported in the package-close response and repository history

## 7. What changed in understanding

- The smallest honest `P15` move was not a full emitted executable family; it was a committed generated bridge artifact plus alignment checks against already-existing helper/runtime anchors.
- `samples/generated/` can hold useful committed bridge evidence without collapsing into either source-sample space or final emitted executable space.
- The main risk at this package was overclaiming, not lack of preview evidence.

## 8. Open questions

- When `P16` widens viewer/public visualization, should generated bridge artifacts surface directly in the viewer, or should the viewer only consume normalized typed inventories?
- Should a later projection/codegen tranche introduce a distinct emitted-program runner before any optimizer / deployment planner work, or should those arrive together behind a new gate?
- What is the narrowest future equivalence surface that proves more than anchor alignment without prematurely freezing a final public emitted-program ABI?

## 9. Suggested next prompt

Close `P16` visual debugger / viewer first public prototype as a typed visualization public-prototype boundary: widen the current helper/report-local visualization inventory into a first public viewer surface, keep label / authority / redaction / retention explicit, and avoid collapsing helper-local debug output into a final public viewer API.
