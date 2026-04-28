# Report 0971 — P16 visual debugger / viewer first public prototype closeout

- Date: 2026-04-28 18:44 JST
- Author / agent: Codex (GPT-5)
- Scope: Close `P16` as a typed public prototype inventory over existing helper/runtime visualization surfaces. Add the smallest honest viewer-prototype helper, sync front-door docs and repository memory, and preserve the stop line against claiming a final public viewer API or telemetry backend.
- Decision levels touched: none; current package closeout and repository-memory synchronization only

## 1. Objective

Close the current first cut of `P16` by:

- introducing a helper-local viewer-prototype validation surface over already-existing typed visualization inputs
- normalizing helper/runtime view and telemetry lanes into a shared typed public prototype inventory
- synchronizing front-door docs, roadmap memory, hands-on docs, and runnable sample dashboard
- recording an evidence-backed stop line that keeps final public viewer API, final visualization/telemetry schema, Event DAG / graph views, retention policy, and telemetry backend deferred

## 2. Scope and assumptions

- This task does **not** implement a final public viewer API.
- This task does **not** implement a final public visualization schema or telemetry schema.
- This task does **not** implement Event DAG, place graph, effect route graph, proof obligation graph, witness timeline, performance telemetry service, or IDE embedding.
- The current honest scope is typed public prototype inventory over already-existing helper/runtime surfaces.
- `mir_hilight.html` remains a sample readability helper and is not relabeled as the `P16` viewer prototype.
- Unrelated user modifications in `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs` and `crates/mir-ast/src/current_l2.rs` were left untouched.

## 3. Documents consulted

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
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `plan/26-visual-debugger-viewer-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/hands_on/typed_external_boundary_canaries_01.md`
- `docs/hands_on/visual_debugger_viewer_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_08_visualization_protocol.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `docs/research_abstract/visual_debugger_viewer_plan_01.md`
- `scripts/README.md`
- `scripts/visual_debugger_viewer_samples.py`
- `scripts/tests/test_visual_debugger_viewer_samples.py`
- `docs/reports/0970-p15-projection-codegen-generated-bridge-first-cut-closeout.md`

## 4. Actions taken

- Added `scripts/visual_debugger_viewer_samples.py` as the package-local validation helper for:
  - `list`
  - `run <bundle_id>`
  - `check-all`
  - `closeout`
- Added regression tests in `scripts/tests/test_visual_debugger_viewer_samples.py` for:
  - bundle listing
  - helper/runtime panel normalization
  - typed telemetry normalization
  - duplicate helper panel-id canonicalization
  - drift detection
  - closeout inventory
  - pretty formatting
- Hardened the viewer prototype helper against silent shadowing by canonicalizing repeated helper `panel_id` values with explicit `@N` suffixes.
- Switched `actualized_panel_kinds` / `actualized_telemetry_kinds` in closeout output from rule-union inference to live normalized bundle aggregation.
- Added `plan/26-visual-debugger-viewer-roadmap.md` as repository memory for the current typed viewer-prototype boundary.
- Added `docs/hands_on/visual_debugger_viewer_01.md` and `docs/research_abstract/visual_debugger_viewer_plan_01.md` as hands-on and reader-facing entry points.
- Synced front-door docs, hands-on docs, roadmap memory, snapshot docs, and sample dashboard to:
  - `P16` close (current first cut only)
  - `P17` promoted next
  - `P18` reopen next
- Preserved the stop line that keeps helper-local pretty print, runtime canonical inventory, projection preview, host-boundary preview, route-trace canary, and `mir_hilight.html` separate from the final public viewer contract.

## 5. Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/26-visual-debugger-viewer-roadmap.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/hands_on/typed_external_boundary_canaries_01.md`
- `docs/hands_on/visual_debugger_viewer_01.md`
- `docs/research_abstract/hands_on_sugoroku_08_visualization_protocol.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`
- `docs/research_abstract/visual_debugger_viewer_plan_01.md`
- `scripts/README.md`
- `scripts/tests/test_visual_debugger_viewer_samples.py`
- `scripts/visual_debugger_viewer_samples.py`
- `docs/reports/0971-p16-visual-debugger-viewer-first-public-prototype-closeout.md`

## 6. Evidence / outputs / test results

Fresh package-close validation was rerun after the doc/report sync.

- `df -h .`
  - pass; repo root filesystem had ample free space before the package-close run
- `free -h`
  - pass; low-memory VPS profile was still workable for helper/runtime closeout commands
- `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples`
  - pass; `8/8` tests green
- `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-02 --format json`
  - pass; helper closeout catalog now returns `verification_summary@2` instead of silently shadowing the second verification panel
- `python3 scripts/visual_debugger_viewer_samples.py list --format json`
  - pass; typed public prototype bundle inventory lists `P16-VIEW-01..05`
- `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json`
  - pass; Sugoroku helper `turn_timeline` / `message_route` / `verification_summary` / `projection_view` normalize into typed panel lanes
- `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json`
  - pass; `NET-05` observer route trace stays redacted and typed
- `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json`
  - pass; runtime canonical inventory keeps `report_local_inventory` retention boundary
- `python3 scripts/visual_debugger_viewer_samples.py check-all --format json`
  - pass; `passed = [P16-VIEW-01, P16-VIEW-02, P16-VIEW-03, P16-VIEW-04, P16-VIEW-05]`
- `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
  - pass; returns `viewer_panel_lanes`, `viewer_telemetry_lanes`, live `actualized_panel_kinds`, live `actualized_telemetry_kinds`, `kept_later_gates`, and the prototype boundary wording
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - pass; helper visualization bundle remains typed and evidence-oriented
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass; Sugoroku closeout catalog still exposes `membership_snapshot` and hot-plug backing inventory for the viewer prototype
- `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json`
  - pass; fail-closed route trace remains observer-safe
- `python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json`
  - pass; host-boundary preview stays a typed viewer-pressure anchor without freezing host schema
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass; runtime canonical inventory continues to expose `authority_trace_redacted_view`, `provider_boundary_redacted_flow`, and `cross_place_projection`
- `python3 scripts/check_source_hierarchy.py`
  - pass; required hierarchy intact after report `0971`
- `python3 scripts/validate_docs.py`
  - pass; documentation scaffold complete with numbered report count `969`
- `git diff --check`
  - pass; package-close diff is whitespace-clean

## 7. What changed in understanding

- The smallest honest `P16` move was not a browser UI; it was a typed public prototype inventory that normalizes already-existing helper/runtime evidence surfaces.
- The main technical gap was schema normalization between helper-local and report-local inventories, not the lack of raw visualization signals.
- The helper closeout catalog already contained repeated `view_id` values, so the public-prototype helper needed explicit ID canonicalization to preserve the repo’s no-silent-shadowing rule.
- Projection preview, host-boundary preview, route-trace canary, and runtime canonical inventory can contribute to a viewer prototype without being mislabeled as a final public viewer API.

## 8. Open questions

- Which later package should own final public visualization schema and telemetry schema widening: a `P18` freeze gate only, or a narrower pre-freeze contract package?
- Should later Event DAG / place-graph / proof-obligation-graph surfaces share the same typed panel lanes, or require a separate graph-carrier family?
- How much of runtime canonical inventory should later become viewer-public, and how much should stay report-local only?

## 9. Suggested next prompt

Close `P17` storage / LLVM / backend preparation as an implementation-ready staging package: verify external workdir bindings and cleanup safety, tighten backend-prep stop lines, and avoid any device rewrite or destructive cleanup.
