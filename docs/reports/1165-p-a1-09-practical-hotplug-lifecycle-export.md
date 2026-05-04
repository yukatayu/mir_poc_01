# Report 1165 — P-A1-09 Practical Hot-Plug Lifecycle Export Widening

- Date: 2026-05-04
- Author / agent: Codex
- Scope: `P-A1-09` widening package on the practical alpha-1 lane for `VIS-A1-04` only
- Decision levels touched: `specs/18` practical devtools boundary wording, `plan/44` practical package queue memory, snapshot/dashboard package status
- 日本語要約: `P-A1-09` では practical avatar/product lane に飛ばず、`PA1-6` を narrow に reopen して `VIS-A1-04` だけを actualize した。source は exact practical hot-plug reports `HP-A1-01` accepted attach と `HP-A1-07` deferred detach boundary に限り、`scripts/practical_alpha1_export_devtools.py` と exact expected `vis-a1-04-hotplug-lifecycle.expected.json` により、attach accepted boundary・membership snapshot・deferred detach boundary を distinct devtools export bundle + non-final viewer surface へ下ろした。これは detach runtime lifecycle execution、final object package attach、product prototype completion、final public viewer / telemetry ABI を意味しない。`P-A1-08` は引き続き recut-required blocker のままである。

## Objective

Close a safe practical hot-plug lifecycle observability widening without overclaiming avatar/product semantics. Reopen `PA1-6` only far enough to actualize `VIS-A1-04` as an exact-report export bundle, keep the carrier split explicit, synchronize queue/status docs, and stop with the existing `P-A1-08` blocker still intact.

## Scope and assumptions

- Scope is limited to the practical alpha-1 devtools/export lane:
  - `scripts/practical_alpha1_export_devtools.py`
  - `scripts/tests/test_practical_alpha1_export_devtools.py`
  - exact expected `VIS-A1-04` bundle
  - synchronized practical roadmap/snapshot docs
- Actualized row is exactly:
  - `VIS-A1-04` hot-plug lifecycle export over exact reports
- Exact source reports are exactly:
  - `HP-A1-01` accepted attach
  - `HP-A1-07` deferred detach minimal contract boundary
- This package does not add:
  - new runtime semantics
  - detach runtime lifecycle execution
  - object attach completion
  - product prototype completion
  - final public viewer/telemetry ABI
  - practical avatar/runtime carrier actualization
- `P-A1-08` remains blocked. This package does not silently recut or unblock it.

## Start state / dirty state

- Work resumed on `main` after `P-A1-07` had already been closed and pushed, and after `P-A1-08` had already been recorded as a recut-required blocker.
- The package resumed from a dirty worktree inside package scope:
  devtools export helper/tests, exact expected `VIS-A1-04`, and synchronized docs were already in progress when this closeout started.
- No unrelated user-authored edits were found in the touched files.
- Long-running research guardrail checks were rerun before closeout:
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `638Mi` used, `89Mi` free, `390Mi` buff/cache, `321Mi` available

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/alpha/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1160-p-a1-06-first-devtools-export-floor.md`
- `docs/reports/1161-p-a1-07-local-save-load-command.md`
- `docs/reports/1162-p-a1-08-recut-blocker.md`
- `docs/reports/1164-review-vis-a1-04-practical-hotplug-lifecycle-export.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`

## Actions taken

1. Re-read the practical alpha-1 queue, stage map, sample matrix, and blocker state to verify that a safe next move exists only as a `PA1-6` widening rather than as `P-A1-08`.
2. Ran resource checks and three reviews:
   - semantics/spec/queue review,
   - helper/test review,
   - docs/snapshot review.
3. Confirmed that `HP-A1-01` and `HP-A1-07` already expose sufficient exact practical hot-plug report lanes to ground `VIS-A1-04` without importing helper-local lifecycle state as runtime truth.
4. Added `VIS-A1-04` to `scripts/practical_alpha1_export_devtools.py` as `hotplug_lifecycle_export`.
5. Implemented `_hotplug_lifecycle_bundle()` so the export:
   - consumes only exact practical hot-plug reports,
   - keeps attach accepted boundary, membership snapshot, and deferred detach boundary separate,
   - carries explicit non-claims for detach execution, rollback/migration, fallback degradation, retention/on-demand, and product completion.
6. Changed `render_html()` to go through `run_sample()` rather than bypass exact-report parity via `build_bundle()`.
7. Added test coverage for:
   - `VIS-A1-04` exact source report family/scope/surface checks,
   - attach/detach split panels and refs,
   - explicit non-claim for detach runtime lifecycle execution,
   - HTML rendering parity through `run_sample()`,
   - widened closeout set `VIS-A1-01/02/04/06`.
8. Added exact expected `samples/practical-alpha1/expected/vis-a1-04-hotplug-lifecycle.expected.json`.
9. Synchronized `README.md`, `Documentation.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, `samples/practical-alpha1/README.md`, `samples/practical-alpha1/expected/README.md`, `scripts/README.md`, `progress.md`, `tasks.md`, and `samples_progress.md`.
10. Ran the practical prerequisite floors, widened devtools export floor, docs/source-hierarchy validators, formatting/diff checks, and prepared the repo to stop with `P-A1-08` still blocked.

## Files changed

- Script / tests:
  - `scripts/practical_alpha1_export_devtools.py`
  - `scripts/tests/test_practical_alpha1_export_devtools.py`
- Practical expected bundle:
  - `samples/practical-alpha1/expected/vis-a1-04-hotplug-lifecycle.expected.json`
- Docs / repository memory / dashboards:
  - `README.md`
  - `Documentation.md`
  - `specs/18-practical-alpha1-scope.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `samples/practical-alpha1/expected/README.md`
  - `scripts/README.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Reports:
  - `docs/reports/1164-review-vis-a1-04-practical-hotplug-lifecycle-export.md`
  - `docs/reports/1165-p-a1-09-practical-hotplug-lifecycle-export.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M JST'
df -h .
free -h
git status --short
rg -n "VIS-A1-01/02/06|P-A1-06|最終更新|Last updated|PA1-6|VIS-A1-04|次の reopen|next reopen|P-A1-08" progress.md tasks.md samples_progress.md README.md Documentation.md specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md plan/01-status-at-a-glance.md samples/README.md samples/practical-alpha1/README.md samples/practical-alpha1/expected/README.md scripts/README.md
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py list --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-01 --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-02 --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-04 --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-06 --format json
python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-04 --format json
python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-06 --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py closeout --format json
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Resource / environment checks passed:
  - `date`: `2026-05-04 11:31 JST` and final snapshot sync at `2026-05-04 11:33 JST`
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `638Mi` used, `89Mi` free, `390Mi` buff/cache, `321Mi` available
- Practical prerequisite floors passed:
  - `python3 scripts/practical_alpha1_check.py check-all --format json`
    - `sample_count: 10`
    - `passed: ["CHK-LIF-01", "CHK-LIF-02", "CHK-LIF-03", "CHK-LIF-04", "CHK-VAR-01", "CHK-VAR-02", "CHK-VAR-03", "CHK-CUT-01", "CHK-PKG-01", "CHK-PKG-02"]`
    - `first_checker_floor_complete: true`
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json`
    - `sample_count: 2`
    - `passed: ["RUN-01", "RUN-02"]`
    - `local_runtime_first_floor_complete: true`
  - `python3 scripts/practical_alpha1_attach.py check-all --format json`
    - `sample_count: 9`
    - `passed: ["HP-A1-01", "HP-A1-02", "HP-A1-03", "HP-A1-04", "HP-A1-05", "HP-A1-04B1", "HP-A1-04B2", "HP-A1-06", "HP-A1-07"]`
    - `detach_minimal_contract_complete: true`
  - `python3 scripts/practical_alpha1_transport.py check-all --format json`
    - `sample_count: 7`
    - `passed: ["TR-A1-01", "TR-A1-02", "TR-A1-03", "TR-A1-04", "TR-A1-05", "TR-A1-06", "TR-A1-07"]`
    - `stage_pa1_5_complete: true`
- Widened devtools export floor passed:
  - `python3 scripts/practical_alpha1_export_devtools.py list --format json`
    - implemented rows: `VIS-A1-01`, `VIS-A1-02`, `VIS-A1-04`, `VIS-A1-06`
  - `python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-04 --format json`
    - `bundle_kind: "hotplug_lifecycle_export"`
    - `family: "practical-alpha1-devtools-export"`
    - `source_reports` limited to `HP-A1-01` and `HP-A1-07`
    - `panel_ids: ["attach_lifecycle", "membership_snapshot", "detach_lifecycle"]`
    - `export_sections.attach_boundary.activation_cut_ref: "activation_cut#debug_trace_layer_attach"`
    - `export_sections.detach_boundary.detach_boundary_ref: "detach_boundary#alpha_local_hotplug_minimal_contract"`
    - `what_it_does_not_prove` explicitly includes `detach runtime lifecycle execution`
  - `python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-04 --format json`
    - emitted HTML bundle with separate attach/detach boundary panels and non-claim section
  - `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
    - `sample_count: 4`
    - `passed: ["VIS-A1-01", "VIS-A1-02", "VIS-A1-04", "VIS-A1-06"]`
    - `deferred_observables: ["VIS-A1-03", "VIS-A1-05", "VIS-A1-07"]`
  - `python3 scripts/practical_alpha1_export_devtools.py closeout --format json`
    - `implemented_rows: ["VIS-A1-01", "VIS-A1-02", "VIS-A1-04", "VIS-A1-06"]`
    - `devtools_export_first_floor_complete: true`
    - `stage_pa1_6_complete: false`
- Rust prerequisites passed:
  - `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture`
    - `11` tests passed
  - `cargo test -p mir-ast practical_alpha1_checker -- --nocapture`
    - `3` checker tests passed
    - existing warnings in `crates/mir-ast/tests/support/practical_alpha1_checker_support.rs` remained non-fatal and unchanged
  - `cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture`
    - `5` runtime-plan tests passed
  - `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
    - `8` hotplug-plan tests passed
  - `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
    - `15` practical hot-plug runtime tests passed
- Python unit / docs tests passed:
  - `python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs`
    - `Ran 45 tests`
    - `OK`
- Docs / formatting / diff floor passed:
  - `python3 scripts/check_source_hierarchy.py`
    - `required: 73`
    - `present: 73`
    - `missing: 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - report count updated after adding this package report
  - `cargo fmt --check`: passed
  - `git diff --check`: passed

## What changed in understanding

- A safe practical next move existed even with `P-A1-08` blocked, but only as a reopened `PA1-6` widening over already-actualized exact hot-plug reports.
- The critical honesty boundary is not “hot-plug lifecycle” as a generic label, but “accepted attach plus deferred detach boundary exported without claiming detach execution.”
- `render_html()` parity mattered: allowing HTML rendering to bypass `run_sample()` would have weakened the exact-report-evidence claim.

## Open questions

- Should `P-A1-08` be recut into a thin product-preview bundle over current practical carriers, with avatar scope narrowed to placeholder companion evidence?
- Or should practical `AV-A1-*` carriers be added first so product wording can remain literal?
- After `VIS-A1-04`, should the next devtools widening target be `VIS-A1-03`, `VIS-A1-05`, or `VIS-A1-07`, and what exact practical evidence would ground it without synthetic overclaim?

## Suggested next prompt

Recut `P-A1-08` to a first practical product-preview bundle over existing practical carriers, or explicitly choose the prior practical `AV-A1-*` carrier path before reopening the product lane.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` に `P-A1-09` closeout、`PA1-6` widened floor、`P-A1-08` blocked status を同期した。

## Documentation.md update status

`Documentation.md` 更新済み: current practical closeout が `P-A1-07` に加えて `P-A1-09` まで進んだこと、`VIS-A1-04` が export-side widening only であること、`P-A1-08` が blocked のままであることを反映した。

## progress.md update status

`progress.md` 更新済み: top timestamp、current practical status、validation freshness、blockers、3-axis row、`PA1-6` row、recent log を `P-A1-09` closeout 状態へ同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-09` closeout、`PA1-6` widened-floor state、`P-A1-08` blocked status、no promoted next package の current queue authority を同期した。

## samples_progress.md update status

`samples_progress.md` 更新済み: top timestamp、`PH0` row、practical stage header、`VIS-A1-01/02/04/06` devtools row、`PA1-6` widened-floor row を `P-A1-09` closeout 状態へ同期した。

## Reviewer findings and follow-up

- `Helmholtz`:
  - finding: `VIS-A1-04` is safe only if introduced as a reopened `PA1-6` widening rather than as a silent replacement for `P-A1-08`
  - follow-up applied: `plan/44` / `progress.md` / `tasks.md` now treat `P-A1-09` as the last closed package and keep `P-A1-08` blocked
- `Carver`:
  - finding: `render_html()` must not bypass exact-report parity, and `VIS-A1-04` must remain attach/detach boundary slices rather than a synthesized executed lifecycle
  - follow-up applied: `render_html()` now calls `run_sample()`, tests assert exact source family/scope/surface, separate panel ids, and explicit non-claim for detach runtime lifecycle execution
- `Mencius`:
  - finding: snapshot docs must raise `PA1-6` to `60%`, carry `P-A1-09` as the last closed package, and keep `P-A1-08` blocked without overclaiming practical/product readiness
  - follow-up applied: `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`specs/18`、`plan/01`、`plan/44`、sample READMEs、`scripts/README.md` were synchronized

## Skipped validations and reasons

- Did not rerun `scripts/practical_alpha1_save_load.py` or its Rust save/load crates in this package because save/load implementation was not changed; `P-A1-07` remains referenced as the already-closed first practical local save/load floor.
- Did not run Docker-local transport environment probes beyond `python3 scripts/practical_alpha1_transport.py check-all --format json`, because `P-A1-09` consumes exact transport/hotplug/local-runtime reports rather than modifying those runtime environments.
- Did not widen any practical avatar/runtime carrier; `P-A1-08` and practical `AV-A1-*` remain separate open questions rather than silently accepted scope.

## Commit / push status

Package closeout is prepared in the worktree. Commit/push status will be updated after validation-final diff review and the actual `git commit --no-gpg-sign` / `git push`.

## Sub-agent session close status

- `Helmholtz` completed the semantics/spec/queue review and should be closed after package commit
- `Carver` completed the helper/test review and should be closed after package commit
- `Mencius` completed the docs/snapshot review and should be closed after package commit
