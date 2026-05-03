# Report 1160 — P-A1-06 First Practical Devtools Export Floor

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-06` first practical devtools export floor over the practical alpha-1 package line
- Decision levels touched: `specs/18` practical devtools boundary, `plan/44` roadmap memory, practical devtools-export/viewer carrier split, snapshot/dashboard wording
- 日本語要約: `P-A1-06` では full devtools completion へ広げず、exact practical local-runtime / transport reports を source にする distinct devtools export bundle と non-final static HTML viewer だけを first floor として closeout した。actualize したのは `VIS-A1-01` event DAG + publication/witness/handoff export、`VIS-A1-02` observer-safe route trace export、`VIS-A1-06` auth-lane separated redacted observer view に限り、membership timeline、hot-plug lifecycle、fallback degradation、retention/on-demand trace、save/load、product prototype、final public viewer/telemetry ABI は still later に残した。

## Objective

Close `P-A1-06` honestly after the `P-A1-05` transport floor. Add the narrowest practical devtools/viewer export surface that can reuse already-actualized practical reports, keep the carrier split explicit, synchronize roadmap/snapshot/sample dashboards, and promote `P-A1-07` as the next practical package.

## Scope and assumptions

- Scope is limited to the practical alpha-1 devtools lane in `scripts/`, `samples/practical-alpha1/expected/`, and synchronized docs.
- `P-A1-06` does not widen runtime semantics, transport semantics, or the practical sample root into an active runnable root.
- The honest closeout is a first export floor only:
  `exact practical reports -> distinct devtools export bundle -> non-final viewer`.
- Actualized observables are exactly:
  - `VIS-A1-01` event DAG + publication / witness / handoff relation export
  - `VIS-A1-02` observer-safe route trace export
  - `VIS-A1-06` auth-lane separated redacted observer view
- Deferred observables remain:
  - `VIS-A1-03` membership timeline
  - `VIS-A1-04` hot-plug lifecycle
  - `VIS-A1-05` fallback degradation
  - `VIS-A1-07` retention / on-demand trace
- The package must not claim full devtools completion, local save/load command completion, product prototype completion, active runnable-root promotion, or final public viewer / telemetry / runtime-devtools ABI.

## Start state / dirty state

- Work resumed on `main` after `P-A1-05` had already been closed and pushed.
- The package resumed from a dirty worktree inside package scope:
  `scripts/practical_alpha1_export_devtools.py`, `scripts/tests/test_practical_alpha1_export_devtools.py`, practical expected bundles, and synchronized docs were already in progress.
- `git status --short` at report write showed only package-scope edits plus new package files; no unrelated user-authored edits were detected in the touched files.
- Long-running research guardrail checks were rerun before closeout:
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `604Mi` used, `148Mi` free, `362Mi` buff/cache, `1.8Gi` swap

## Documents consulted

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
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1158-p-a1-05-transport-scope-blocker.md`
- `docs/reports/1159-p-a1-05-practical-transport-e2e-closeout.md`
- `docs/reports/review-2026-05-03-pa1-05-transport-scope-review.md`

## Actions taken

1. Added `scripts/practical_alpha1_export_devtools.py` as a pure Python helper that reuses exact practical reports rather than adding new runtime semantics.
2. Fixed a distinct devtools carrier split with:
   `exact practical reports -> distinct devtools export bundle -> non-final viewer`.
3. Actualized `VIS-A1-01` by exporting event DAG plus publication / witness / handoff relation evidence from `RUN-01`.
4. Actualized `VIS-A1-02` by exporting observer-safe route trace evidence from `TR-A1-06`.
5. Actualized `VIS-A1-06` by exporting a redacted observer view with auth-lane separation from `TR-A1-07`.
6. Added a non-final static HTML renderer over the same bundle payload without claiming a final public viewer surface.
7. Added exact expected bundle files for `VIS-A1-01/02/06`.
8. Added focused Python tests for bundle drift detection, HTML rendering, and closeout non-claims.
9. Synchronized `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, practical sample READMEs, and `scripts/README.md`.

## Files changed

- Code / tests:
  - `scripts/practical_alpha1_export_devtools.py`
  - `scripts/tests/test_practical_alpha1_export_devtools.py`
- Practical expected bundles:
  - `samples/practical-alpha1/expected/vis-a1-01-event-dag-export.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-02-route-trace-export.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-06-redacted-observer-view.expected.json`
- Docs / dashboards / repository memory:
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `specs/18-practical-alpha1-scope.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `samples/practical-alpha1/expected/README.md`
  - `scripts/README.md`
  - `docs/reports/1160-p-a1-06-first-devtools-export-floor.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M JST'
df -h .
free -h
git status --short
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py list --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-01 --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-02 --format json
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-06 --format json
python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-06 --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Resource / environment checks passed:
  - `date`: `2026-05-03 22:25 JST`
  - root disk: `99G` total, `65G` used, `30G` available
  - memory: `960Mi` total, `629Mi` used, `171Mi` free, `314Mi` buff/cache, `330Mi` available, `1.8Gi` swap used
- Existing practical floors reran green:
  - `python3 scripts/practical_alpha1_check.py check-all --format json`
    - `sample_count: 10`
    - `first_checker_floor_complete: true`
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json`
    - `sample_count: 2`
    - `local_runtime_first_floor_complete: true`
    - `runtime_plan_boundary_present: true`
  - `python3 scripts/practical_alpha1_attach.py check-all --format json`
    - `sample_count: 9`
    - `package_hotplug_first_floor_complete: true`
    - `hotplug_plan_boundary_present: true`
    - `object_attach_seam_present: true`
    - `detach_minimal_contract_complete: true`
  - `python3 scripts/practical_alpha1_transport.py check-all --format json`
    - `sample_count: 7`
    - `transport_first_floor_complete: true`
    - `transport_plan_boundary_present: true`
    - `docker_row_complete: true`
    - `route_trace_complete: true`
    - `auth_lane_complete: true`
    - `stage_pa1_5_complete: true`
- `P-A1-06` devtools floor validation passed:
  - `python3 scripts/practical_alpha1_export_devtools.py list --format json`
    - listed exactly `VIS-A1-01`, `VIS-A1-02`, `VIS-A1-06`
  - `python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-01 --format json`
    - matched exact event DAG + publication / witness / handoff export bundle
  - `python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-02 --format json`
    - matched exact observer-safe route trace export bundle
  - `python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-06 --format json`
    - matched exact auth-lane separated redacted observer bundle
  - `python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-06 --format json`
    - emitted a non-final static HTML viewer file under `/tmp/practical-a1-devtools-*.html`
  - `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
    - `sample_count: 3`
    - `passed: ["VIS-A1-01", "VIS-A1-02", "VIS-A1-06"]`
    - `devtools_export_first_floor_complete: true`
    - `stage_pa1_6_complete: false`
    - `deferred_observables: ["VIS-A1-03", "VIS-A1-04", "VIS-A1-05", "VIS-A1-07"]`
  - `python3 scripts/practical_alpha1_export_devtools.py closeout --format json`
    - `implemented_rows: ["VIS-A1-01", "VIS-A1-02", "VIS-A1-06"]`
    - `devtools_export_first_floor_complete: true`
    - `stage_pa1_6_complete: false`
    - `viewer_mode: "static_html_bundle"`
- Python unit tests passed:
  - `python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs`
    - `Ran 37 tests in 0.050s`
    - `OK`
- Docs / scaffold floor passed:
  - `python3 scripts/check_source_hierarchy.py`
    - `required: 73`
    - `present: 73`
    - `missing: 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1161 numbered report(s).`
  - `cargo fmt --check`: passed
  - `git diff --check`: passed

## What changed in understanding

- The honest `P-A1-06` solution is not a full viewer stage and not a new runtime bridge. It is a first practical devtools export floor over already-actualized practical reports.
- `VIS-A1-01/02/06` are safe because each one already has an exact practical source report with a narrow authority boundary.
- `VIS-A1-03/04/05/07` remain later because they need additional practical evidence rather than synthetic widening.
- `scripts/practical_alpha1_check.py closeout()` is not the authority for this package; the new devtools helper must carry its own closeout semantics.

## Open questions

- What is the narrowest honest shape for `P-A1-07` local save/load so it reuses the practical package line without collapsing into alpha-0 `CUT-*` evidence language?
- When `VIS-A1-03/04/05/07` are revisited later, should they widen this same export bundle schema or require one more distinct practical devtools carrier split?

## Suggested next prompt

Close `P-A1-07` by adding the narrowest honest practical local save/load command over the practical alpha-1 package line, with explicit stale-state rejection and no distributed durable save/load claim.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` に `P-A1-06` first practical devtools export floor と `P-A1-07` promoted next package を反映した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 line の current snapshot に `P-A1-06` first practical devtools export floor と non-claim boundary を反映した。

## progress.md update status

`progress.md` 更新済み: large stage、current status、validation floor、readiness rows、recent log を `P-A1-06` first floor に同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-06` closeout を current task-level status と package map に反映し、`P-A1-07` を promoted next package として固定した。

## samples_progress.md update status

`samples_progress.md` 更新済み: practical alpha-1 stage summary と `VIS-A1-01/02/06` devtools export row を反映した。

## Reviewer findings and follow-up

- `Gauss`:
  full `PA1-6` closeoutは unsafe であり、safe package は first practical devtools export floor に narrow すべき、という finding を採用した。
- `Nietzsche`:
  pure Python export helper + exact bundle files + static HTML viewer を推奨し、`practical_alpha1_check.py closeout()` を authority にしないことを確認した。
- `Ampere`:
  `specs/18`、`plan/44`、snapshot docs、sample README、`scripts/README.md` の non-claim wording synchronization を要求し、その follow-up を反映した。

## Skipped validations and reasons

- No Rust crate sources were changed in `P-A1-06`, so Rust integration tests beyond the already-actualized practical floors were not widened here.
- No Docker / transport runtime command beyond `P-A1-05`'s existing practical floor was needed, because this package reuses exact practical reports rather than adding new runtime/transport execution semantics.

## Commit / push status

Main package commit `8c3297d` (`mirrorea: close p-a1-06 first devtools export floor`) was pushed to `origin/main`. Report metadata and sub-agent close status were finalized in a docs-only follow-up after the package closeout.

## Sub-agent session close status

- `Gauss` completed the semantics/spec review and is now closed
- `Nietzsche` completed the implementation/test review and is now closed
- `Ampere` completed the docs/snapshot review and is now closed
