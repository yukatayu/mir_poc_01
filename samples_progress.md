# samples_progress

Last updated: 2026-05-05 16:35 JST
Current repo-local focus: current-L2 base source corpus, clean near-end runnable floor, Lean foundations / generated theorem stubs, practical alpha-1 first-floor toolchain, bounded operational α-0.5 / α-0.8 / α-0.9 runtime lines, the bounded practical α-1 integrated workflow carrier, and the product alpha-1 local same-session runtime/save/transport/viewer/native-bundle first cut. `samples/alpha/` remains the alpha-0 evidence root; `samples/practical-alpha1/` remains the practical first-floor fixture root; `samples/product-alpha1/` now has product alpha schema/check, local same-session run/session/attach, local R0/R2 save evidence, local/Docker transport, non-final devtools/viewer evidence, and native host launch bundle evidence but is not release-ready.

## Legend

Primary metric:

- `workflow-ready`: an external developer can reproduce the named layer workflow end-to-end from repo commands.
- `evidence-closed`: helper / sidecar / report / expected JSON / first-floor runner evidence is synchronized and validated, but the row is not operational workflow completion.
- `boundary-fixed`: normative specs / roadmap define the boundary, but no reproducible workflow is present yet.
- `entrypoint/schema-ready`: alpha CLI and package schema checks are reproducible, but the product workflow is not yet end-to-end.
- `product-carrier-first-cut`: alpha CLI can run a local same-session product demo carrier through save/load, transport, non-final viewer export, and native host launch bundle generation, but release validation and CLI `demo` remain incomplete.
- `planned`: source or roadmap exists, but no reproducible workflow is present.

Notes:

- `100%` is not used for helper / sidecar / report / expected JSON / first-floor runner rows.
- Use `100%` only when a layer is externally usable as a reproducible operational workflow or product/public layer.
- helper-local preview, report-local inventory, and generated bridge evidence are not final public API.

## Workflow and Product-Boundary Snapshot

| Line | Workflow status | Reproducible command | Current evidence | Missing actualization |
|---|---|---|---|---|
| α-0.5 local observable runtime | workflow-ready: local session workflow | `python3 scripts/practical_alpha05_session.py check-all --format json` | `RUN-01..04`, `SL-A1-01/02/03`, `VIS-A1-01/03/05/06`, `OA05-01..07`, `specs/19..24`, `plan/45/48/49` | none within the bounded α-0.5 workflow |
| α-0.8 same-session hot-plug runtime | workflow-ready: same-session hot-plug workflow | `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json` | `HP-A1-01..07`, `AV-A1-03`, `VIS-A1-04/05`, `OA08-01..10`, `crates/mir-runtime::practical_alpha08_hotplug_session`, `specs/21/22/24`, `plan/46/48/49` | accepted detach execution / distributed ordering |
| α-0.9 session-bound devtools | workflow-ready: session-bound devtools workflow | `python3 scripts/practical_alpha09_devtools.py check-all --format json` | `OA09-01..09`, `crates/mir-runtime::practical_alpha09_devtools`, example `export-devtools`, `scripts/practical_alpha09_devtools.py`, `specs/22/24`, `plan/47` | final public viewer/telemetry ABI, durable audit |
| practical α-1 integrated workflow | bounded workflow-ready, not product/public-ready | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json` | `PA1W-01..08`, exact `VIS-A1-01` / `PE2E-01/02/07` evidence, `OA05/OA08/OA09` carriers | product/public-ready α-1, final public viewer/telemetry ABI, distributed durable save/load |
| product alpha-1 local same-session runtime/save/transport/viewer/native-bundle | product-carrier-first-cut, not release-ready | `cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle --format json` | `crates/mirrorea-cli`, `crates/mir-ast::product_alpha1`, `crates/mir-runtime::product_alpha1_session`, `crates/mir-runtime::product_alpha1_transport`, `crates/mir-runtime::product_alpha1_devtools`, `samples/product-alpha1/demo`, `samples/product-alpha1/docker`, local session store, bounded recovery rows, R0/R2 save, local loopback TCP, Docker Compose TCP, non-final viewer bundle, native host launch bundle manifest / run script / provenance reports | release validation, CLI `demo` |

## Practical alpha-1 first-floor map

| Family | Classification | Validation anchor | Current reading |
|---|---|---|---|
| `SRC-01..05` | first-floor evidence | `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` | limited `package.mir.json` front-door。final grammar ではない |
| `CHK-LIF/VAR/CUT/PKG-01/02` | first-floor evidence | `python3 scripts/practical_alpha1_check.py check-all --format json` | distinct lowered checker IR + explicit accepted/rejected obligations |
| `RUN-01..04` | first-floor evidence | `python3 scripts/practical_alpha1_run_local.py check-all --format json` | accepted local dispatch、stale-membership reject、missing capability reject、missing witness reject の first local-runtime floor |
| `HP-A1-01..07` | first-floor evidence | `python3 scripts/practical_alpha1_attach.py check-all --format json` | attach accept/reject、object preview seam、deferred detach minimal contract |
| `TR-A1-01..07` | first-floor evidence | `python3 scripts/practical_alpha1_transport.py check-all --format json` | local TCP / Docker Compose TCP、observer-safe route trace、auth-lane separation |
| `VIS-A1-01..07` | first-floor evidence | `python3 scripts/practical_alpha1_export_devtools.py check-all --format json` | export-side event DAG / route trace / membership timeline / hot-plug lifecycle / redacted view / retention query |
| `SL-A1-01..03` | first-floor evidence | `python3 scripts/practical_alpha1_save_load.py check-all --format json` | local-only roundtrip、stale-membership non-resurrection、checker-backed invalid distributed-cut preflight reject |
| `AV-A1-01..03` | first-floor evidence | `python3 scripts/practical_alpha1_avatar.py check-all --format json` | placeholder / custom preview / unsupported-runtime visible fallback companion floor |
| `PE2E-01..09` | first-floor evidence | `python3 scripts/practical_alpha1_product_preview.py check-all --format json` | thin exact-evidence product-preview bundles。same-session runtime ではない |
| `PA1W-01..08` | bounded workflow evidence | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json` | front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 workflow に束ねる。product/public-ready α-1 ではない |

## Alpha-0 evidence reference

| Stage | Classification | Validation anchor | Current reading |
|---|---|---|---|
| A | current-scope evidence | imported baseline rerun floor | imported alpha-ready baseline |
| B | current-scope evidence | `python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json` | local runtime + local-only save/load supporting subset |
| C | current-scope evidence | `python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json` | transport narrow cut |
| D | current-scope evidence | `python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json` | hot-plug lifecycle closeout |
| E | current-scope evidence | `python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json` | devtools closeout subset |
| F | current-scope evidence | `python3 scripts/alpha_e2e_samples.py stage-f-closeout --format json` | integrated alpha evidence closeout |

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
- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
- `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out /tmp/mirrorea-alpha1-devtools --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-alpha1-devtools --check --format json`
- `cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle --format json`
- `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`
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
| 2026-05-05 16:35 JST | `P-A1-30` product alpha native launch bundle | pass | `mirrorea-alpha build-native-bundle`、bundle `run.sh check/view`、manifest `NativeExecutionPolicy = Disabled`、provenance-only signature metadata、observer-safe generated reports を追加。CLI `demo` / release validation は `P-A1-31` scope |
| 2026-05-05 15:53 JST | `P-A1-29` product alpha transport + viewer | pass | `mirrorea-alpha transport --mode local` / `--mode docker`、Docker Compose TCP endpoint reports、`export-devtools` bundle、`view --check` を same session carrier に追加。observer-safe redaction、admin/debug `kept_later`、13 panel IDs を確認。native bundle / release validation は later |
| 2026-05-05 15:06 JST | `P-A1-28` product alpha message recovery + quiescent-save | pass | DAG-linked `MessageState` / `TransportContract` / `RecoveryPolicy` rows、`mirrorea-alpha save` / `load` / `quiescent-save`、R0 local save/load、bounded R2 quiescent-save、load-admissibility reject、duplicate event-ID guard を same session carrier に追加。transport / viewer / native bundle は later |
| 2026-05-05 14:48 JST | `P-A1-27` product alpha local same-session runtime | pass | `mirrorea-alpha run-local` / `session` / `attach` と `crates/mir-runtime::product_alpha1_session` を追加。同じ local session file に declared typed host-I/O、activation cut、auth/capability decision、hot-plug lifecycle、membership/witness/route/save-load/recovery state を保持。transport / save-load / viewer / native bundle は later |
| 2026-05-05 14:00 JST | `P-A1-26` product alpha CLI/schema | pass | `mirrorea-alpha check` と product `package.mir.json` schema first cut を追加。later command family は structured unsupported。product workflow-ready claim はまだしない |
| 2026-05-05 13:14 JST | `P-A1-25` product alpha boundary recut | pass | `specs/25` / `plan/50` を required scaffold に追加し、product alpha-1 は boundary-fixed だが not workflow-ready と分類。behavior implementation は未変更 |
| 2026-05-05 12:32 JST | root Markdown concision + operational workflow verification | pass | α-0.5 / α-0.8 / α-0.9 / bounded practical α-1 workflow `check-all`、focused Python unittest、focused Rust runtime tests、docs/source hierarchy checks が pass。`product_public_ready = false` は維持 |
| 2026-05-05 11:59 JST | `P-A1-24` workflow-readiness policy sync | pass | progress dashboard を percentage から workflow status / evidence classification へ切り替え、helper / sidecar / report / expected JSON / first-floor runner を completion ではなく evidence として分類 |
| 2026-05-05 11:33 JST | `P-A1-23` practical α-1 integrated workflow carrier | pass | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow`、`python3 scripts/practical_alpha1_export_devtools.py check-all --format json`、`python3 scripts/practical_alpha1_product_preview.py check-all --format json` が pass |
| 2026-05-05 11:17 JST | `P-A1-22` α-0.9 session-bound devtools export | pass | `cargo test -p mir-runtime --test practical_alpha09_devtools`、`python3 scripts/practical_alpha09_devtools.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha09_devtools` が pass |
| 2026-05-05 10:18 JST | `P-A1-21` α-0.8 same-session hot-plug runtime | pass | `cargo test -p mir-runtime --test practical_alpha08_session_hotplug`、`python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug` が pass |
| 2026-05-05 09:47 JST | `P-A1-20` typed host-I/O direct execution lane | pass | `cargo test -p mir-runtime --test practical_alpha05_host_io`、`cargo test -p mir-runtime --test practical_alpha05_session`、`cargo test -p mir-runtime --test practical_alpha1_local_runtime`、`python3 scripts/practical_alpha1_run_local.py check-all --format json`、`python3 scripts/practical_alpha05_session.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session` が pass |
| 2026-05-05 09:26 JST | `P-A1-19` session runtime carrier | pass | `cargo test -p mir-runtime --test practical_alpha1_local_runtime`、`cargo test -p mir-runtime --test practical_alpha05_session`、`python3 scripts/practical_alpha1_run_local.py check-all --format json`、`python3 scripts/practical_alpha05_session.py check-all --format json`、`python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session` が pass |
| 2026-05-05 08:32 JST | `P-A1-18` theory freeze docs/package sync | pass | `python3 -m unittest scripts.tests.test_validate_docs`、`python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`cargo fmt --check`、`git diff --check` が pass。Rust runtime behavior は未変更のため focused Cargo behavior tests は不要 |
| 2026-05-04 17:28 JST | `P-A1-17` save-load preview carrier alignment | pass | `SL-A1-03` を exact save-load preflight evidence として `PE2E-06` に realign |
| 2026-05-04 16:25 JST | practical devtools export widened floor | pass | `VIS-A1-03/04/05/07` を含む export-side first floors を維持 |
| 2026-05-04 14:15 JST | practical avatar preview first floor | pass | `AV-A1-01/02/03` companion floor を維持 |
| 2026-05-04 01:05 JST | alpha-0 Stage B freshness rerun | pass | Stage B は current-scope evidence reference であり、operational α-0.5 runtime ではないと再確認 |
