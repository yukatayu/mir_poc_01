# samples_progress

Last updated: 2026-05-07 09:03 JST
Current repo-local focus: current-L2 base source corpus, clean near-end runnable floor, Lean foundations / generated theorem stubs, practical alpha-1 first-floor toolchain, bounded operational α-0.5 / α-0.8 / α-0.9 runtime lines, the bounded practical α-1 integrated workflow carrier, the product alpha-1 release-candidate workflow, and the canonical operational product sample suite. `samples/alpha/` remains the alpha-0 evidence root; `samples/practical-alpha1/` remains the practical first-floor fixture root; `samples/product-alpha1/` now has product alpha schema/check, local same-session run/session/attach, local R0/R2 save evidence, local/Docker transport, non-final devtools/viewer evidence, native host launch bundle evidence, CLI `demo`, release check evidence, and the `operational/` suite for `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` with bounded room-oriented `MembershipChat` `ChatText("hello room") -> "room#lobby message accepted: hello room"` evidence, `SugorokuWorld` bounded runtime evidence, schema-backed projection inventory, bounded portal discrete handoff evidence, bounded two-shard hard-authority evidence, bounded observer-only gradient runtime evidence, a `template_only` `world-core` / `membership-chat` / `sugoroku-world` starter catalog plus guide, the docs-first boundary that keeps portal/shard authoring on active roots rather than starter duplicates, a docs-first backend comparison inventory, and a non-executable `gradient-observation.profile.json` future profile paired with the separate runtime root. Docker skip paths are partial local probes, not release-candidate evidence.

## Legend

Primary metric:

- `workflow-ready`: an external developer can reproduce the named layer workflow end-to-end from repo commands.
- `evidence-closed`: helper / sidecar / report / expected JSON / first-floor runner evidence is synchronized and validated, but the row is not operational workflow completion.
- `boundary-fixed`: normative specs / roadmap define the boundary, but no reproducible workflow is present yet.
- `entrypoint/schema-ready`: alpha CLI and package schema checks are reproducible, but the product workflow is not yet end-to-end.
- `product-release-candidate`: alpha CLI can reproduce the product alpha command family through `demo` and release check, but final public product claims remain out of scope.
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
| product alpha-1 release candidate | product-release-candidate, not final product | `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release` | `crates/mirrorea-cli`, `crates/mir-ast::product_alpha1`, `crates/mir-runtime::product_alpha1_session`, `crates/mir-runtime::product_alpha1_transport`, `crates/mir-runtime::product_alpha1_devtools`, `samples/product-alpha1/demo`, `samples/product-alpha1/docker`, source-backed debug/auth/rate-limit attach, deferred object/avatar-preview boundary, local admin session store, observer-safe session artifact, bounded recovery rows, R0/R2 save, local loopback TCP, Docker Compose TCP, concrete non-final viewer bundle, native host launch bundle, CLI `demo`, release check script with validation floor / payload semantics | final public grammar / ABI / WAN / distributed durable save-load |
| operational product sample suite | workflow-ready canonical suite, not final product | `python3 scripts/operational_product_samples.py check-all --format json` | `samples/product-alpha1/operational/world-core`, `membership-chat`, `sugoroku-world`, `portal-worldlink`, `two-shard-hard-boundary`, `two-shard-gradient-observation`, shared debug/auth/rate-limit/object/avatar packages, local/Docker deployment profiles, schema-backed projection profile, retained portal/shard blueprints, non-executable `samples/product-alpha1/operational/future/gradient-observation.profile.json`, `scripts/operational_product_samples.py`, `docs/hands_on/operational_product_sample_01.md`, `docs/research_abstract/operational_product_sample_01.md`, `samples/product-alpha1/operational/templates/world-core-starter`, `samples/product-alpha1/operational/templates/membership-chat-starter`, `samples/product-alpha1/operational/templates/sugoroku-world-starter`, `docs/hands_on/operational_package_authoring_01.md`, `docs/research_abstract/operational_package_authoring_01.md`, `docs/hands_on/operational_backend_inventory_01.md`, `docs/research_abstract/operational_backend_inventory_01.md`, `docs/hands_on/operational_gradient_observation_profile_01.md`, `docs/research_abstract/operational_gradient_observation_profile_01.md`, `docs/hands_on/operational_portal_shard_starter_boundary_01.md`, `docs/research_abstract/operational_portal_shard_starter_boundary_01.md`, `membership-chat` bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` lane, `sugoroku-world` bounded roll / publish / witness / handoff / stale membership reject scenario, `portal-worldlink` bounded resolve / handoff offer / witness emit / destination admit scenario, `two-shard-hard-boundary` bounded offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject scenario, `two-shard-gradient-observation` bounded observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject scenario, observer-safe devtools bundle with source/dependency/projection/portal/shard panels, helper semantic checks for projection inventory, portal runtime evidence, hard-boundary runtime evidence, and gradient runtime evidence | final-public gate scoping |

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
- `python3 -m unittest scripts.tests.test_operational_product_samples`
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
- `cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-demo --format json`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-gradient-observation --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/world-core-starter --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/membership-chat-starter --format json`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/sugoroku-world-starter --format json`
- `python3 -m json.tool samples/product-alpha1/operational/future/spatial-shard-future.profile.json`
- `python3 -m json.tool samples/product-alpha1/operational/future/gradient-observation.profile.json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/membership-chat --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/world-core-starter --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/membership-chat-starter --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/sugoroku-world-starter --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- session 'session#operational-world-core-starter' --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-world-core-starter' --out /tmp/mirrorea-ops-authoring-viewer --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-authoring-viewer --check --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-membership-chat' --out /tmp/mirrorea-ops-chat-viewer --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-chat-viewer --check --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json`
- `python3 scripts/operational_product_samples.py run-sugoroku --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/portal-worldlink --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-portal-worldlink' --out /tmp/mirrorea-ops-portal-viewer --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-portal-viewer --check --format json`
- `python3 scripts/operational_product_samples.py run-portal-worldlink --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-hard-boundary --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-hard-boundary' --out /tmp/mirrorea-ops-shard-viewer --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-shard-viewer --check --format json`
- `python3 scripts/operational_product_samples.py run-two-shard-hard-boundary --format json`
- `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-gradient-observation --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-gradient-observation' --out /tmp/mirrorea-ops-gradient-viewer --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-gradient-viewer --check --format json`
- `python3 scripts/operational_product_samples.py run-two-shard-gradient-observation --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/debug-layer --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/auth-layer --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/rate-limit-layer --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r0' --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- quiescent-save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r2' --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode local --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode docker --format json`
- `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-sugoroku' --out /tmp/mirrorea-ops-viewer --format json`
- `python3 scripts/operational_product_samples.py export-devtools --format json`
- `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-viewer --check --format json`
- `cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/operational/sugoroku-world --out /tmp/mirrorea-ops-bundle --format json`
- `python3 scripts/operational_product_samples.py list --format json`
- `python3 scripts/operational_product_samples.py check-all --format json`
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
| 2026-05-07 09:03 JST | `P-OPS-15` gradient observation runtime first cut | pass | separate runnable `two-shard-gradient-observation/` root、freshness-carrying observer-only route evidence、gradient devtools panel、profile/runtime boundary wordingを同期し、next reopen point を final-public gate scoping に進めた |
| 2026-05-07 08:28 JST | `P-OPS-14` maintenance / dashboard freshness | pass | queue / validator / roadmap / dashboard wording を current state に同期し、next reopen point を gradient observation runtime first cut に進めた |
| 2026-05-07 01:45 JST | `P-OPS-13` broader room-chat lane widening | pass | `MembershipChat` の current lane を bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` に widen し、sample root / starter / schema/runtime / helper / guide / dashboard を同期した。next reopen point は maintenance / dashboard freshness |
| 2026-05-07 01:32 JST | `P-OPS-12` portal/shard starter boundary | pass | portal/shard authoring を active roots に留め、starter catalog を `SugorokuWorld` で止める current decision を docs/roadmap/dashboard に同期した。next reopen point は broader room-chat lane widening |
| 2026-05-07 01:10 JST | `P-OPS-11` gradient observation profile inventory | pass | `future/gradient-observation.profile.json` と guide を追加し、observer-only shard overlap reading を `planned_only` inventory として固定した。next reopen point は portal/shard starter decision |
| 2026-05-07 00:56 JST | `P-OPS-10` broader operational template catalog first cut | pass | `templates/membership-chat-starter/` と `templates/sugoroku-world-starter/` を追加し、validated starter catalog を `WorldCore -> MembershipChat -> SugorokuWorld` まで広げた。next reopen point は gradient observation profile |
| 2026-05-07 00:39 JST | `P-OPS-08` backend feasibility inventory | pass | host launch bundle / WASM / LLVM comparison inventory を docs-first に追加し、current actualized path が `native host launch bundle` のみであることと backend non-claims を固定した。next reopen point は broader operational template catalog |
| 2026-05-07 00:25 JST | `P-OPS-09` developer package authoring guide | pass | `templates/world-core-starter/` と bounded authoring guide を追加し、template-only starter の `check` / `run-local` / `session` / `export-devtools` / `view --check` 入口を固定した。next reopen point は `P-OPS-08` |
| 2026-05-07 00:00 JST | `P-OPS-07` two-shard hard-boundary first cut | pass | `two-shard-hard-boundary/` root を追加し、bounded same-session offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidence を `run-local` / observer-safe devtools export / helper semantic checks に接続した。next reopen point は `P-OPS-09` |
| 2026-05-06 23:32 JST | `P-OPS-06` portal / world-link first cut | pass | `portal-worldlink/` root を追加し、bounded same-session discrete handoff evidence を `run-local` / observer-safe devtools export / helper semantic checks に接続した。next reopen point は `P-OPS-07` |
| 2026-05-06 23:01 JST | `P-OPS-05` projection manifest / packet / FFI schema | pass | `projection.profile.json` を schema-backed inventory に formalize し、`check` / runtime plan / devtools projection panel / helper semantic checks を同期した。next reopen point は `P-OPS-06` |
| 2026-05-06 22:42 JST | `P-OPS-04` Sugoroku behavior widening | pass | `SugorokuWorld` に bounded same-session roll / publish / witness / handoff / stale membership reject scenario を追加し、`run-local` / `session` / devtools export / helper semantic checks まで同期した。next reopen point は projection / packet / FFI schema |
| 2026-05-06 22:27 JST | `P-OPS-03` operational direct text host boundary | pass | `MembershipChat` に bounded `EchoText("Taro") -> "Hello, Taro!"` lane を追加し、`run-local` / session export / devtools export / helper semantic checks まで同期した。broader room-chat lane は still later |
| 2026-05-06 21:12 JST | `P-OPS-01` canonical operational suite closeout hardening | pass | operational Docker compose selection、bundle attach package preservation、deferred object/avatar attach rows、helper `check-all`、route/config panel boundedness wordingを補修後、suite `accepted` を再確認。portal/shard runtime は planned-only |
| 2026-05-05 17:48 JST | `P-A1-31` review hardening | pass | Docker skip now reports partial/non-release, demo verifies attach matrix and same-session reopen, viewer renders concrete observer-safe panel JSON, release check runs validation floor and payload semantics |
| 2026-05-05 17:14 JST | `P-A1-31` product alpha release candidate | pass | `mirrorea-alpha demo`、`product_alpha1_release_check.py check-all`、debug/auth/rate-limit/object/avatar-preview package breadth、hands-on / research docs を追加。final public product ではない |
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
