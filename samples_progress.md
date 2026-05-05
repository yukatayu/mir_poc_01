# samples_progress

Last updated: 2026-05-05 11:33 JST
Current repo-local focus: current-L2 base source corpus, clean near-end runnable floor, Lean foundations / generated theorem stubs, practical alpha-1 first-floor toolchain, bounded operational α-0.5 / α-0.8 / α-0.9 runtime lines, and the bounded practical α-1 integrated workflow carrier. `samples/alpha/` remains the alpha-0 evidence root; `samples/practical-alpha1/` remains the practical first-floor fixture root; the α-0.5 session carrier now consumes that root through minimal typed host-I/O, same-session hot-plug lanes, session-bound devtools export, and a non-final integrated workflow summary.

## Legend

Progress:

- 0%: not scheduled
- 10%: theory/sample skeleton exists
- 25%: carrier boundary exists
- 50%: minimal positive path exists
- 65%: negative/rejection path exists
- 75%: debug/observability output exists
- 90%: closeout validation passes
- 100%: operational-layer-ready or product/public-ready for the named line

Notes:

- `100% current-scope evidence closeout` and `100% first-floor closeout` are explicit categories and do **not** imply operational readiness.
- conceptual-only rows must stay at or below `25%`.
- helper-local preview, report-local inventory, and generated bridge evidence are not final public API.

## Operational readiness snapshot

| Line | Progress | Status | Current evidence | Missing actualization |
|---|---:|---|---|---|
| α-0.5 local observable runtime | 100 | bounded operational α-0.5 ready | `RUN-01..04`, `SL-A1-01/02/03`, `VIS-A1-01/03/05/06`, `OA05-01..07`, `specs/19..24`, `plan/45/48/49` | none within the bounded α-0.5 line |
| α-0.8 same-session hot-plug runtime | 100 | bounded operational α-0.8 ready | `HP-A1-01..07`, `AV-A1-03`, `VIS-A1-04/05`, `OA08-01..10`, `crates/mir-runtime::practical_alpha08_hotplug_session`, `specs/21/22/24`, `plan/46/48/49` | none within the bounded α-0.8 line |
| α-0.9 session-bound devtools | 100 | bounded operational α-0.9 ready | `OA09-01..09`, `crates/mir-runtime::practical_alpha09_devtools`, example `export-devtools`, `scripts/practical_alpha09_devtools.py`, `specs/22/24`, `plan/47` | none within the bounded α-0.9 line |
| practical α-1 integrated workflow | 100 | bounded practical workflow ready | `PA1W-01..08`, `scripts/practical_alpha1_integrated_workflow.py`, exact `VIS-A1-01` / `PE2E-01/02/07` evidence, `OA05/OA08/OA09` carriers | product/public-ready α-1, final public viewer/telemetry ABI, distributed durable save/load |

## Practical alpha-1 first-floor map

| Family | Progress | Category | Validation anchor | Current reading |
|---|---:|---|---|---|
| `SRC-01..05` | 100 | first-floor closeout | `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` | limited `package.mir.json` front-door。final grammar ではない |
| `CHK-LIF/VAR/CUT/PKG-01/02` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_check.py check-all --format json` | distinct lowered checker IR + explicit accepted/rejected obligations |
| `RUN-01..04` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_run_local.py check-all --format json` | accepted local dispatch、stale-membership reject、missing capability reject、missing witness reject の first local-runtime floor |
| `HP-A1-01..07` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_attach.py check-all --format json` | attach accept/reject、object preview seam、deferred detach minimal contract |
| `TR-A1-01..07` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_transport.py check-all --format json` | local TCP / Docker Compose TCP、observer-safe route trace、auth-lane separation |
| `VIS-A1-01..07` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_export_devtools.py check-all --format json` | export-side event DAG / route trace / membership timeline / hot-plug lifecycle / redacted view / retention query |
| `SL-A1-01..03` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_save_load.py check-all --format json` | local-only roundtrip、stale-membership non-resurrection、checker-backed invalid distributed-cut preflight reject |
| `AV-A1-01..03` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_avatar.py check-all --format json` | placeholder / custom preview / unsupported-runtime visible fallback companion floor |
| `PE2E-01..09` | 100 | first-floor closeout | `python3 scripts/practical_alpha1_product_preview.py check-all --format json` | thin exact-evidence product-preview bundles。same-session runtime ではない |
| `PA1W-01..08` | 100 | bounded practical workflow ready | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json` | front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 workflow に束ねる。product/public-ready α-1 ではない |

## Alpha-0 evidence reference

| Stage | Progress | Category | Validation anchor | Current reading |
|---|---:|---|---|---|
| A | 100 | current-scope evidence closeout | imported baseline rerun floor | imported alpha-ready baseline |
| B | 100 | current-scope evidence closeout | `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json` | local runtime + local-only save/load supporting subset |
| C | 100 | current-scope evidence closeout | `python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json` | transport narrow cut |
| D | 100 | current-scope evidence closeout | `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json` | hot-plug lifecycle closeout |
| E | 100 | current-scope evidence closeout | `python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json` | devtools closeout subset |
| F | 100 | current-scope evidence closeout | `python3 scripts/alpha_e2e_samples.py stage-f-closeout --format json` | integrated alpha evidence closeout |

## Required operational sample matrix status

| Required family | Current closest evidence | Gap |
|---|---|---|
| α-0.5 accepted local dispatch / stale membership reject / save-load resume / save-load stale-membership reject | `OA05-01/02/05`, `RUN-01/02`, `SL-A1-01/02` | bounded α-0.5 line では gap なし |
| α-0.5 missing capability / missing witness / fallback degradation / observer-safe export | `OA05-03/04/06`, `RUN-03/04`, `VIS-A1-05/06` | bounded α-0.5 line では gap なし |
| α-0.8 debug/auth/rate-limit/object attach / incompatible patch / deferred detach / lifecycle export | `OA08-01..10`, `HP-A1-01..07`, `VIS-A1-04/05` | bounded α-0.8 line では gap なし |
| α-0.9 event DAG / route trace / membership timeline / witness relation / save-load timeline / redacted observer view / retention trace | `OA09-01..09`, `VIS-A1-01..07` | bounded α-0.9 line では gap なし |
| typed host-I/O minimal demo | `OA05-07`, `crates/mir-runtime::practical_alpha05_host_io` | bounded α-0.5 line では gap なし |

## Validation anchors for this package

- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
- `python3 scripts/practical_alpha09_devtools.py check-all --format json`
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
- `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`
- `python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow`
- `cargo fmt --check`
- `git diff --check`

## Recent validation log

| Timestamp | Scope | Status | Notes |
|---|---|---|---|
| 2026-05-05 11:33 JST | `P-A1-23` practical α-1 integrated workflow carrier | pass | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow`、`python3 scripts/practical_alpha1_export_devtools.py check-all --format json`、`python3 scripts/practical_alpha1_product_preview.py check-all --format json` が pass |
| 2026-05-05 11:17 JST | `P-A1-22` α-0.9 session-bound devtools export | pass | `cargo test -p mir-runtime --test practical_alpha09_devtools`、`python3 scripts/practical_alpha09_devtools.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha09_devtools` が pass |
| 2026-05-05 10:18 JST | `P-A1-21` α-0.8 same-session hot-plug runtime | pass | `cargo test -p mir-runtime --test practical_alpha08_session_hotplug`、`python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug` が pass |
| 2026-05-05 09:47 JST | `P-A1-20` typed host-I/O direct execution lane | pass | `cargo test -p mir-runtime --test practical_alpha05_host_io`、`cargo test -p mir-runtime --test practical_alpha05_session`、`cargo test -p mir-runtime --test practical_alpha1_local_runtime`、`python3 scripts/practical_alpha1_run_local.py check-all --format json`、`python3 scripts/practical_alpha05_session.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session` が pass |
| 2026-05-05 09:26 JST | `P-A1-19` session runtime carrier | pass | `cargo test -p mir-runtime --test practical_alpha1_local_runtime`、`cargo test -p mir-runtime --test practical_alpha05_session`、`python3 scripts/practical_alpha1_run_local.py check-all --format json`、`python3 scripts/practical_alpha05_session.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session` が pass |
| 2026-05-05 08:32 JST | `P-A1-18` theory freeze docs/package sync | pass | `python3 -m unittest scripts.tests.test_validate_docs`、`python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`cargo fmt --check`、`git diff --check` が pass。Rust runtime behavior は未変更のため focused Cargo behavior tests は不要 |
| 2026-05-04 17:28 JST | `P-A1-17` save-load preview carrier alignment | pass | `SL-A1-03` を exact save-load preflight evidence として `PE2E-06` に realign |
| 2026-05-04 16:25 JST | practical devtools export widened floor | pass | `VIS-A1-03/04/05/07` を含む export-side first floors を維持 |
| 2026-05-04 14:15 JST | practical avatar preview first floor | pass | `AV-A1-01/02/03` companion floor を維持 |
| 2026-05-04 01:05 JST | alpha-0 Stage B freshness rerun | pass | Stage B は `100% current-scope evidence closeout` reference であり、operational α-0.5 runtime ではないと再確認 |
