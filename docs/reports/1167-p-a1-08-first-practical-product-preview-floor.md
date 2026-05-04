# Report 1167 — P-A1-08 First Practical Product-Preview Floor

- Date: 2026-05-04
- Author / agent: Codex
- Scope: `P-A1-08` closeout over the practical alpha-1 first product-preview floor
- Decision levels touched: `L1`, `L2`
- 日本語要約:
  `P-A1-08` を full product prototype ではなく、preview manifest から exact practical reports / exact practical devtools bundles を束ねる first practical product-preview floor として閉じた。`PE2E-01..07` を actualize し、`PE2E-04` は `HP-A1-06` placeholder object preview companion evidence に narrow した。custom Mir avatar runtime、unsupported runtime fallback、same-session runtime attach/detach lifecycle execution、final public CLI / viewer / package-avatar / save-load / transport API は引き続き claim していない。

## Objective

Close `P-A1-08` honestly by recutting the practical product lane to a first product-preview floor over existing practical carriers, then synchronize roadmap/snapshot/sample taxonomy/docs, run relevant validation, and prepare commit/push.

## Scope and assumptions

- Scope is limited to the first practical product-preview floor.
- The implementation surface is:
  `samples/practical-alpha1/previews/`,
  `scripts/practical_alpha1_product_preview.py`,
  `scripts/tests/test_practical_alpha1_product_preview.py`,
  exact expected `pe2e-a1-*.expected.json`,
  and synchronized docs/roadmap/dashboard wording.
- The preview carrier split is:
  `preview manifest -> exact practical reports / exact practical devtools bundles -> non-final product-preview bundle`.
- `PE2E-04` is narrowed to `HP-A1-06` placeholder object preview companion evidence only.
- This package does not actualize:
  custom Mir avatar runtime,
  unsupported runtime fallback,
  same-session runtime attach execution,
  same-session detach runtime lifecycle execution,
  full practical product prototype completion,
  active runnable-root promotion,
  final public CLI / viewer / transport / save-load / package-avatar API.

## Start state / dirty state

- Start state was not clean.
- At turn start, the worktree already contained in-progress `P-A1-08` carry-over edits:
  `specs/18-practical-alpha1-scope.md` had already gained the product-preview boundary,
  and new uncommitted files already existed for
  `scripts/practical_alpha1_product_preview.py`,
  `scripts/tests/test_practical_alpha1_product_preview.py`,
  `samples/practical-alpha1/previews/`,
  and exact expected `pe2e-a1-*.expected.json`.
- `git status --short` at carry-over start showed:
  modified `specs/18-practical-alpha1-scope.md`,
  plus untracked preview manifests, expected bundles, product-preview script, and product-preview tests.
- Resource check was run before continuing long-form work:
  `df -h .`,
  `free -h`,
  and `date`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/reports/1162-p-a1-08-recut-blocker.md`
- `docs/reports/1165-p-a1-09-practical-hotplug-lifecycle-export.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `AGENTS.md`

## Actions taken

1. Confirmed the carry-over implementation shape for the preview-manifest floor:
   `PE2E-01..07`,
   exact expected bundles,
   and the new `practical_alpha1_product_preview.py` runner/test pair.
2. Kept the normative boundary in `specs/18`:
   first practical product-preview floor only,
   separate from full product prototype completion.
3. Added and/or verified the new practical preview surface:
   - `samples/practical-alpha1/previews/README.md`
   - `samples/practical-alpha1/previews/pe2e-a1-01-local-full-toolchain-preview/preview.json`
   - `samples/practical-alpha1/previews/pe2e-a1-02-docker-full-toolchain-preview/preview.json`
   - `samples/practical-alpha1/previews/pe2e-a1-03-hotplug-debug-layer-preview/preview.json`
   - `samples/practical-alpha1/previews/pe2e-a1-04-placeholder-object-preview/preview.json`
   - `samples/practical-alpha1/previews/pe2e-a1-05-local-save-load-continue/preview.json`
   - `samples/practical-alpha1/previews/pe2e-a1-06-invalid-distributed-save-rejected/preview.json`
   - `samples/practical-alpha1/previews/pe2e-a1-07-devtools-viewer-preview/preview.json`
4. Kept the product-preview runner exact-report-driven:
   - no parser/runtime bridge added
   - no new runtime semantics added
   - no implicit success from missing negatives
   - `PE2E-06` explicitly grounded in `CHK-CUT-01`
   - `PE2E-07` explicitly grounded in exact devtools bundles
5. Synchronized repository memory and snapshot docs so `P-A1-08` now reads as closed first floor rather than recut blocker.
6. Added a hands-on landing page:
   `docs/hands_on/practical_alpha1_product_preview_01.md`.
7. Ran fresh Python validation, fresh practical runner checks, relevant Cargo tests, and docs/source-hierarchy checks.

## Files changed

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/01-status-at-a-glance.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/practical-alpha1/previews/README.md`
- `samples/practical-alpha1/previews/pe2e-a1-01-local-full-toolchain-preview/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-02-docker-full-toolchain-preview/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-03-hotplug-debug-layer-preview/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-04-placeholder-object-preview/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-05-local-save-load-continue/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-06-invalid-distributed-save-rejected/preview.json`
- `samples/practical-alpha1/previews/pe2e-a1-07-devtools-viewer-preview/preview.json`
- `samples/practical-alpha1/expected/pe2e-a1-01-local-full-toolchain-preview.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-02-docker-full-toolchain-preview.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-03-hotplug-debug-layer-preview.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-04-placeholder-object-preview.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-05-local-save-load-continue.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-06-invalid-distributed-save-rejected.expected.json`
- `samples/practical-alpha1/expected/pe2e-a1-07-devtools-viewer-preview.expected.json`
- `scripts/practical_alpha1_product_preview.py`
- `scripts/tests/test_practical_alpha1_product_preview.py`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/hands_on/practical_alpha1_product_preview_01.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M:%S %Z'
df -h .
free -h
git status --short
find docs/reports -maxdepth 1 -type f | sed 's#docs/reports/##' | sort | tail -n 10
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_validate_docs
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast practical_alpha1_checker -- --nocapture
cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture
python3 scripts/practical_alpha1_product_preview.py closeout --format json
git diff --stat
git status --short
```

## Evidence / outputs / test results

- `python3 -m unittest ...` ran 55 tests and passed.
- `python3 scripts/practical_alpha1_check.py check-all --format json` passed 10/10 rows and reported:
  `first_checker_floor_complete: true`.
- `python3 scripts/practical_alpha1_run_local.py check-all --format json` passed 2/2 rows and reported:
  `local_runtime_first_floor_complete: true`.
- `python3 scripts/practical_alpha1_attach.py check-all --format json` passed 9/9 rows and reported:
  `package_hotplug_first_floor_complete: true`,
  `object_attach_seam_present: true`,
  `detach_minimal_contract_complete: true`.
- `python3 scripts/practical_alpha1_transport.py check-all --format json` passed 7/7 rows and reported:
  `transport_first_floor_complete: true`,
  `stage_pa1_5_complete: true`.
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json` passed 4/4 rows and reported:
  actualized observables `VIS-A1-01/02/04/06`,
  `stage_pa1_6_complete: false`.
- `python3 scripts/practical_alpha1_save_load.py check-all --format json` passed 2/2 rows and reported:
  `local_save_load_first_floor_complete: true`,
  `invalid_distributed_cut_guard_present: true`.
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json` passed 7/7 rows and reported:
  `product_preview_first_floor_complete: true`,
  `stage_pa1_8_complete: false`,
  `deferred_avatar_semantics: ["AV-A1-02", "AV-A1-03"]`,
  `viewer_html_available: true`.
- `python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json` returned exact HTML over `VIS-A1-01/02/04/06` bundle references.
- Cargo validation passed:
  - `practical_alpha1_front_door`: 11 tests
  - `practical_alpha1_checker`: 3 tests
  - `practical_alpha1_runtime_plan`: 5 tests
  - `practical_alpha1_hotplug_plan`: 8 tests
  - `practical_alpha1_save_load_plan`: 4 tests
  - `practical_alpha1_local_runtime`: 4 tests
  - `practical_alpha1_hotplug`: 15 tests
  - `practical_alpha1_transport`: 8 tests
- `python3 scripts/practical_alpha1_product_preview.py closeout --format json` confirmed:
  - `implemented_rows = PE2E-01..07`
  - `product_preview_first_floor_complete: true`
  - `stage_pa1_8_complete: false`
  - `viewer_html_available: true`
- `git diff --stat` stayed within the expected `P-A1-08` scope:
  practical preview surface, roadmap/snapshot/docs sync, and one new hands-on doc.

## What changed in understanding

- `P-A1-08` can be closed honestly without claiming a full product prototype, as long as the carrier is explicitly a preview-manifest bundle over exact existing practical evidence.
- `PE2E-04` needed to stay narrow. Treating placeholder object preview evidence as practical avatar runtime would overclaim semantics.
- The practical alpha-1 line now has six explicit carrier routes, and the product-preview route can be described independently without reopening parser/runtime bridge questions.
- After this package, the main unresolved package line is no longer a wording recut. The unresolved issue is which next narrow carrier, if any, can actualize practical avatar semantics or later devtools/save-load widenings honestly.

## Open questions

- Which narrow carrier, if any, can actualize `AV-A1-02` custom Mir avatar runtime without collapsing placeholder preview evidence into runtime completion?
- Which narrow carrier, if any, can actualize `AV-A1-03` unsupported runtime fallback without collapsing preview evidence into unsupported-runtime execution semantics?
- Are `VIS-A1-03/05/07` or broader save/load widenings promotable before practical avatar semantics, or do they depend on later semantic cuts?

## Suggested next prompt

Promote the next narrow practical alpha-1 package only if it has an equally explicit carrier boundary. Candidate lanes are: practical `AV-A1-02/03` carrier design, remaining `VIS-A1-03/05/07` exact-evidence widening, or later save/load widening with a distinct carrier.

## Plan update status

`plan/` 更新済み:
`plan/44-practical-alpha1-roadmap.md` and `plan/01-status-at-a-glance.md` now mirror the first practical product-preview floor, the `PE2E-01..07` row set, and the post-`P-A1-08` reopen point.

## Documentation.md update status

`Documentation.md` 更新済み:
the practical alpha-1 snapshot now includes the product-preview floor, the preview root, the sixth carrier split, and the non-claims around avatar/runtime/product completion.

## progress.md update status

`progress.md` 更新済み:
`PA1-8` is now tracked as a 60% first practical product-preview floor, and the current practical stage text now points at unresolved avatar semantics rather than the old recut blocker.

## tasks.md update status

`tasks.md` 更新済み:
`P-A1-08` is now closed as a first practical product-preview floor, ordered current work no longer asks for a recut, and the avatar compatibility blocker is rewritten as a later widening question.

## samples_progress.md update status

`samples_progress.md` 更新済み:
the practical package map now includes `PA1-8` as a 60% first product-preview floor and adds the `PE2E-01..07` sample-family row.

## Reviewer findings and follow-up

- No spawned sub-agent review was used in this turn.
- Reason:
  current tool policy in this session did not authorize delegation by default, and this closeout could be completed with local focused review.
- Local focused review findings:
  - preview carrier naming is kept separate from runtime/checker/devtools/save-load carriers
  - `PE2E-06` is explicit negative preview evidence rather than inferred absence-of-failure
  - `PE2E-04` remains narrowed to placeholder object companion evidence only
  - repo memory and snapshot docs now consistently describe `P-A1-08` as a first floor, not a full product prototype

## Skipped validations and reasons

- `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture` was not rerun in this package.
  Reason:
  `P-A1-08` does not modify the save/load Rust runtime itself, and fresh evidence was taken from the already-exact `scripts/practical_alpha1_save_load.py check-all --format json` plus the save-load-plan and dependent preview-floor validations.
- No Docker Compose practical transport execution command was rerun here.
  Reason:
  `P-A1-08` only bundles exact already-actualized transport evidence and does not modify the Docker transport lane itself.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent sessions were opened in this task.
